#!/usr/bin/env bash
set -Eeuo pipefail

export PATH=/usr/sbin:/usr/bin:/sbin:/bin

readonly service_name=ensambria-core.service
readonly migration_service_name=ensambria-core-migrate.service
readonly service_user=ensambria-core
readonly service_group=ensambria-core
readonly migrator_user=ensambria-core-migrator
readonly migrator_group=ensambria-core-migrator
readonly artifact_group=ensambria-core-artifacts
readonly install_root=/opt/apps/ensambria
readonly release_root=/opt/apps/ensambria/releases
readonly current_link=/opt/apps/ensambria/current
readonly environment_directory=/etc/apps/ensambria/core
readonly environment_file=/etc/apps/ensambria/core/.env

fail() {
    printf 'CORE staging deployment failed: %s\n' "$1" >&2
    exit 1
}

ensure_group() {
    getent group "$1" >/dev/null || groupadd --system "$1"
}

ensure_user() {
    local user="$1"
    local group="$2"
    id "$user" >/dev/null 2>&1 \
        || useradd --system --gid "$group" --groups "$artifact_group" \
            --home-dir /nonexistent --no-create-home --shell /usr/sbin/nologin "$user"
    [[ "$(id -u "$user")" -ne 0 && "$(id -gn "$user")" == "$group" ]] \
        || fail "unexpected service account configuration for ${user}"
    id -Gn "$user" | tr ' ' '\n' | grep -Fxq "$artifact_group" \
        || usermod -aG "$artifact_group" "$user"
}

ensure_directory() {
    local directory="$1"
    local group="$2"
    local mode="$3"
    if [[ -e "$directory" ]]; then
        [[ -d "$directory" && ! -L "$directory" ]] \
            || fail "invalid directory at ${directory}"
        [[ "$(stat -c '%U:%G:%a' "$directory")" == "root:${group}:${mode}" ]] \
            || fail "unexpected owner or permissions at ${directory}"
    else
        install -d -o root -g "$group" -m "$mode" "$directory"
    fi
}

wait_until_healthy() {
    local address="$1"
    local attempt
    for ((attempt = 1; attempt <= 30; attempt++)); do
        if systemctl is-active --quiet "$service_name" \
            && runuser -u "$service_user" -- "$current_link/core" --healthcheck "$address" \
                >/dev/null 2>&1
        then
            return 0
        fi
        sleep 2
    done
    return 1
}

rollback() {
    if [[ -n "$previous_release" ]]; then
        ln -s "$previous_release" "$temporary_link"
        mv -Tf "$temporary_link" "$current_link"
        systemctl restart "$service_name" || return 1
        wait_until_healthy "$health_address" || return 1
    else
        rm -f -- "$current_link"
        systemctl stop "$service_name" || return 1
    fi
}

cleanup() {
    local status="$?"
    trap - EXIT INT TERM
    if [[ "$status" -ne 0 && "$release_activated" == true ]]; then
        if rollback; then
            printf 'CORE runtime restored; applied database migrations were not reversed.\n' >&2
        else
            printf 'CORE runtime rollback failed; inspect the service and database before retrying.\n' >&2
            status=1
        fi
    fi
    [[ -z "$temporary_directory" ]] || rm -rf -- "$temporary_directory"
    exit "$status"
}

[[ "$#" -eq 1 && "$(id -u)" -eq 0 ]] || fail "run through sudo with the staging directory"
readonly staging_directory="$1"
[[ -d "$staging_directory" && ! -L "$staging_directory" ]] \
    || fail "staging directory is missing or invalid"
for required_file in \
    "${staging_directory}/ensambria-core.service" \
    "${staging_directory}/ensambria-core-migrate.service" \
    "${staging_directory}/core-staging.tar.gz"
do
    [[ -s "$required_file" ]] || fail "required deployment file is missing"
done
grep -Fqx 'UnsetEnvironment=MIGRATIONS_DATABASE_URL' \
    "${staging_directory}/ensambria-core.service" \
    || fail "CORE systemd service must exclude the migration credential"
grep -Fqx 'UnsetEnvironment=DATABASE_URL' \
    "${staging_directory}/ensambria-core-migrate.service" \
    || fail "migration systemd service must exclude the runtime credential"

ensure_group "$service_group"
ensure_group "$migrator_group"
ensure_group "$artifact_group"
ensure_user "$service_user" "$service_group"
ensure_user "$migrator_user" "$migrator_group"
ensure_directory /opt/apps root 755
ensure_directory "$install_root" "$artifact_group" 750
ensure_directory "$release_root" "$artifact_group" 750
ensure_directory /etc/apps ubuntu 750
ensure_directory /etc/apps/ensambria root 755
ensure_directory "$environment_directory" root 755

[[ -e "$environment_file" ]] \
    || install -o root -g root -m 0600 /dev/null "$environment_file"
[[ -f "$environment_file" && ! -L "$environment_file" ]] \
    || fail "CORE environment file is invalid"
[[ "$(stat -c '%U:%G:%a' "$environment_file")" == "root:root:600" ]] \
    || fail "CORE environment file must be owned by root with mode 0600"
[[ -s "$environment_file" ]] || fail "CORE environment file must be populated"
for required_variable in \
    DATABASE_URL \
    MIGRATIONS_DATABASE_URL \
    CORE_DATABASE_MAX_CONNECTIONS \
    CORE_DATABASE_ACQUIRE_TIMEOUT_MS \
    CORE_HEALTH_LISTEN_ADDRESS \
    CORE_METRICS_LISTEN_ADDRESS \
    CORE_HEALTH_CHECK_INTERVAL_MS \
    CORE_HEALTH_CHECK_TIMEOUT_MS
do
    grep -q "^${required_variable}=." "$environment_file" \
        || fail "required CORE environment setting is missing"
done

health_address="$(sed -n 's/^CORE_HEALTH_LISTEN_ADDRESS=//p' "$environment_file")"
[[ "$health_address" =~ ^127\.0\.0\.1:([1-9][0-9]{0,4})$ ]] \
    || fail "health listener must use 127.0.0.1 and a valid port"
(( BASH_REMATCH[1] <= 65535 )) || fail "health listener port is invalid"

install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-core.service" \
    /etc/systemd/system/ensambria-core.service
install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-core-migrate.service" \
    /etc/systemd/system/ensambria-core-migrate.service
systemctl daemon-reload
systemctl enable "$service_name"

temporary_directory="$(mktemp -d "${release_root}/.incoming.XXXXXX")"
archive="${staging_directory}/core-staging.tar.gz"
payload="${temporary_directory}/payload"
mkdir "$payload"
tar -tzf "$archive" | while IFS= read -r entry; do
    [[ "$entry" == core || "$entry" == core-migrate || "$entry" == migrations/ ]] && continue
    [[ "$entry" =~ ^migrations/[0-9]{14}_[a-z0-9_]+\.sql$ ]] \
        || fail "deployment package contains an unexpected path"
done || fail "deployment package contains unexpected files"
tar -xzf "$archive" -C "$payload" --no-same-owner --no-same-permissions \
    || fail "could not extract deployment package"
[[ -s "$payload/core" && -s "$payload/core-migrate" ]] \
    || fail "deployment package is missing a binary"
find "$payload/migrations" -maxdepth 1 -type f -name '*.sql' -print -quit | grep -q . \
    || fail "deployment package is missing SQL migrations"

release_directory="${release_root}/$(date -u +%Y%m%dT%H%M%SZ)-$$"
mkdir "$release_directory"
mv "$payload/core" "$payload/core-migrate" "$payload/migrations" "$release_directory/"
chown -R "root:${artifact_group}" "$release_directory"
chmod 0750 "$release_directory" "$release_directory/core" "$release_directory/core-migrate"
chmod 0750 "$release_directory/migrations"
chmod 0640 "$release_directory"/migrations/*.sql

previous_release=""
if [[ -L "$current_link" ]]; then
    previous_release="$(readlink -f "$current_link")"
    [[ "$previous_release" == "$release_root/"* ]] || fail "current release link is invalid"
elif [[ -e "$current_link" ]]; then
    fail "current release path is not a symbolic link"
fi

temporary_link="${install_root}/.current-$$"
release_activated=false
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
ln -s "$release_directory" "$temporary_link"
mv -Tf "$temporary_link" "$current_link"
release_activated=true
systemctl start "$migration_service_name" || fail "automatic staging migrations failed"
systemctl restart "$service_name" || fail "CORE service failed to restart"
wait_until_healthy "$health_address" || fail "CORE readiness check failed"

release_activated=false
printf 'CORE deployed, migrations applied, and readiness verified.\n'
