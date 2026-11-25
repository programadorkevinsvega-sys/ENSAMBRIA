#!/usr/bin/env bash
set -Eeuo pipefail

export PATH=/usr/sbin:/usr/bin:/sbin:/bin

readonly service_name=ensambria-core.service
readonly migration_service_name=ensambria-core-migrate.service
readonly email_worker_service_name=ensambria-core-email-worker.service
readonly business_service_name=ensambria-business.service
readonly business_migration_service_name=ensambria-business-migrate.service
readonly env_reload_path_name=ensambria-env-reload.path
readonly env_reload_service_name=ensambria-env-reload.service
readonly business_service_user=ensambria-business
readonly business_service_group=ensambria-business
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

temporary_directory=""
release_directory=""
previous_release=""
temporary_link="${install_root}/.current-$"
release_activated=false
rollback_available=false

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

report_migration_failure() {
    local invocation_id

    printf 'CORE migration service status:\n' >&2
    if ! systemctl show "$migration_service_name" --no-pager \
        --property=Result \
        --property=LoadState \
        --property=ActiveState \
        --property=SubState \
        --property=ExecMainCode \
        --property=ExecMainStatus >&2
    then
        printf 'Could not retrieve CORE migration service status.\n' >&2
    fi

    if ! invocation_id="$(systemctl show "$migration_service_name" --property=InvocationID --value)" \
        || [[ ! "$invocation_id" =~ ^[[:xdigit:]]{32}$ ]]
    then
        printf 'Could not identify the failed CORE migration service invocation; journal entries were not forwarded.\n' >&2
        return 0
    fi

    printf 'Sanitized CORE migration diagnostics:\n' >&2
    if ! journalctl \
        "_SYSTEMD_INVOCATION_ID=${invocation_id}" \
        --lines 100 \
        --output json \
        --no-pager \
        | python3 -c '
import json
import re
import sys

allowed_fields = (
    "event",
    "component",
    "operation",
    "dependency",
    "result",
    "failure_class",
    "error_code",
    "sqlstate",
    "missing_roles",
    "failure_reason",
    "duration_ms",
    "correlation_id",
    "trace_id",
)
safe_error_messages = {
    "RUST_LOG is invalid",
    "the tracing subscriber is already initialized",
    "core-migrate requires a migrations directory",
    "unexpected core-migrate argument",
    "MIGRATIONS_DATABASE_URL is missing or invalid",
    "MIGRATIONS_DATABASE_URL is invalid",
    "CORE migration database connection failed",
    "CORE migration identity could not be verified",
    "CORE migration identity does not have the required restricted privileges",
    "CORE migration identity cannot assume the schema owner role",
    "CORE module schema-owner privileges could not be verified",
    "CORE module schema-owner role has invalid privileges",
    "CORE module schema ownership could not be verified",
    "CORE module schemas must be owned by their dedicated non-login roles",
    "CORE migration role memberships could not be verified",
    "CORE migrator must assume only the two module schema-owner roles",
    "CORE module schemas must be owned by their dedicated non-login roles",
    "CORE required database roles are missing",
    "CORE required database roles could not be verified",
    "CORE migration files could not be loaded",
    "CORE database migration failed",
    "CORE migration database connection did not close cleanly",
}
matched = False

for line in sys.stdin:
    try:
        journal_record = json.loads(line)
        message = journal_record.get("MESSAGE", "") if isinstance(journal_record, dict) else ""
    except (json.JSONDecodeError, TypeError):
        continue

    if isinstance(message, str):
        safe_message = message[7:] if message.startswith("Error: ") else message
        if safe_message in safe_error_messages:
            print(f"message={safe_message}", file=sys.stderr)
            matched = True
            continue

    try:
        event_record = json.loads(message)
    except (json.JSONDecodeError, TypeError):
        continue

    if not isinstance(event_record, dict):
        continue
    fields = event_record.get("fields")
    if not isinstance(fields, dict) or fields.get("event") != "database.migrations.failed":
        continue

    spans = event_record.get("spans")
    if not isinstance(spans, list):
        spans = []
    span_fields = next(
        (
            span.get("fields", {})
            for span in spans
            if isinstance(span, dict) and span.get("name") == "database.migrations"
        ),
        {},
    )
    if not isinstance(span_fields, dict):
        span_fields = {}

    diagnostic_fields = []
    for field in allowed_fields:
        value = fields.get(field, span_fields.get(field, ""))
        if type(value) in (str, int) and re.fullmatch(r"[A-Za-z0-9._:-]{1,128}", str(value)):
            diagnostic_fields.append(f"{field}={value}")

    if diagnostic_fields:
        print(" ".join(diagnostic_fields), file=sys.stderr)
        matched = True

if not matched:
    print(
        "No structured migration failure event was found; raw journal entries were not forwarded.",
        file=sys.stderr,
    )
'; then
        printf 'Could not retrieve or parse sanitized CORE migration diagnostics.\n' >&2
    fi
}


report_business_migration_failure() {
    local invocation_id

    printf 'BUSINESS migration service status:\n' >&2
    if ! systemctl show "$business_migration_service_name" --no-pager \
        --property=Result \
        --property=LoadState \
        --property=ActiveState \
        --property=SubState \
        --property=ExecMainCode \
        --property=ExecMainStatus >&2
    then
        printf 'Could not retrieve BUSINESS migration service status.\n' >&2
    fi

    if ! invocation_id="$(systemctl show "$business_migration_service_name" --property=InvocationID --value)" \
        || [[ ! "$invocation_id" =~ ^[[:xdigit:]]{32}$ ]]
    then
        printf 'Could not identify the failed BUSINESS migration service invocation; journal entries were not forwarded.\n' >&2
        return 0
    fi

    printf 'Sanitized BUSINESS migration diagnostics:\n' >&2
    if ! journalctl \
        "_SYSTEMD_INVOCATION_ID=${invocation_id}" \
        --lines 100 --output json --no-pager 2>/dev/null |
        python3 -c '
import json, re, sys
allowed = ("event", "component", "operation", "failure_class", "error_code", "sqlstate", "missing_roles")
found = False
for line in sys.stdin:
    try:
        record = json.loads(line)
        message = record.get("MESSAGE", "")
        event = json.loads(message) if isinstance(message, str) else {}
    except (ValueError, TypeError):
        continue
    fields = event.get("fields", {}) if isinstance(event, dict) else {}
    if not isinstance(fields, dict):
        continue
    safe = []
    for key in allowed:
        value = fields.get(key)
        if type(value) in (str, int) and re.fullmatch(r"[A-Za-z0-9._:-]{1,128}", str(value)):
            safe.append(f"{key}={value}")
    if safe:
        print(" ".join(safe), file=sys.stderr)
        found = True
if not found:
    print("No structured BUSINESS migration failure event; raw journal withheld.", file=sys.stderr)
'
    then
        printf 'Could not retrieve or parse sanitized BUSINESS migration diagnostics.\n' >&2
    fi
}

rollback() {
    if [[ "$rollback_available" != true ]]; then
        printf 'No rollback-compatible previous release exists; keeping the validated new release selected for manual recovery.\\n' >&2
        return 1
    fi
    ln -s "$previous_release" "$temporary_link"
    mv -Tf "$temporary_link" "$current_link"
    systemctl restart "$service_name" "$email_worker_service_name" "$business_service_name" || return 1
    wait_until_healthy "$health_address" || return 1
    systemctl is-active --quiet "$email_worker_service_name" || return 1
    systemctl is-active --quiet "$business_service_name" || return 1
    curl --fail --silent --show-error --max-time 5 "$business_health_url" >/dev/null || return 1
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
    if [[ -f "/etc/systemd/system/$env_reload_path_name" ]]; then
        systemctl enable --now "$env_reload_path_name" || status=1
    fi
    if [[ "$status" -ne 0 && -n "$release_directory" && -d "$release_directory" ]]; then
        active_release="$(readlink -f "$current_link" 2>/dev/null || true)"
        if [[ "$active_release" != "$release_directory" ]]; then
            rm -rf -- "$release_directory"
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
    "${staging_directory}/ensambria-core-email-worker.service" \
    "${staging_directory}/ensambria-business.service" \
    "${staging_directory}/ensambria-business-migrate.service" \
    "${staging_directory}/ensambria-env-reload.service" \
    "${staging_directory}/ensambria-env-reload.path" \
    "${staging_directory}/validate_staging_package.py" \
    "${staging_directory}/core-staging.tar.gz"
do
    [[ -s "$required_file" ]] || fail "required deployment file is missing"
done

# Serialize deployments on the VM, including manual and out-of-band invocations.
exec 9>/run/lock/ensambria-staging-deploy.lock
flock -n 9 || fail "another ENSAMBRIA deployment is already running"

python3 "${staging_directory}/validate_staging_package.py" "${staging_directory}/core-staging.tar.gz" \
    || fail "deployment artifact validation failed before host changes"

# Check every systemd executable contract before installing any unit files.
grep -Fqx 'ExecStart=/opt/apps/ensambria/current/core' "${staging_directory}/ensambria-core.service" \
    || fail "CORE ExecStart does not match the release layout"
grep -Fqx 'ExecStart=/opt/apps/ensambria/current/core-email-worker' "${staging_directory}/ensambria-core-email-worker.service" \
    || fail "email worker ExecStart does not match the release layout"
grep -Fqx 'ExecStart=/opt/apps/ensambria/current/business' "${staging_directory}/ensambria-business.service" \
    || fail "BUSINESS ExecStart does not match the release layout"
grep -Fqx 'ExecStart=/opt/apps/ensambria/current/core-migrate /opt/apps/ensambria/current/migrations' "${staging_directory}/ensambria-core-migrate.service" \
    || fail "CORE migrator ExecStart does not match the release layout"
grep -Fqx 'ExecStart=/opt/apps/ensambria/current/business-migrate /opt/apps/ensambria/current/business-migrations' "${staging_directory}/ensambria-business-migrate.service" \
    || fail "BUSINESS migrator ExecStart does not match the release layout"

grep -Fqx 'UnsetEnvironment=MIGRATIONS_DATABASE_URL BUSINESS_DATABASE_URL BUSINESS_DATABASE_MAX_CONNECTIONS BUSINESS_HTTP_LISTEN_ADDRESS ENSAMBRIA_REDIS_REQUIRED CORE_EMAIL_WORKER_DATABASE_URL CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_IDLE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_MAX_LIFETIME_MS CORE_SMTP_HOST CORE_SMTP_PORT CORE_SMTP_USERNAME CORE_SMTP_PASSWORD CORE_SMTP_FROM' \
    "${staging_directory}/ensambria-core.service" \
    || fail "CORE systemd service must exclude migration and email-worker secrets"
grep -Fqx 'UnsetEnvironment=DATABASE_URL BUSINESS_DATABASE_URL BUSINESS_DATABASE_MAX_CONNECTIONS BUSINESS_HTTP_LISTEN_ADDRESS ENSAMBRIA_REDIS_REQUIRED CORE_EMAIL_WORKER_DATABASE_URL CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_IDLE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_MAX_LIFETIME_MS CORE_SMTP_HOST CORE_SMTP_PORT CORE_SMTP_USERNAME CORE_SMTP_PASSWORD CORE_SMTP_FROM' \
    "${staging_directory}/ensambria-core-migrate.service" \
    || fail "migration systemd service must exclude runtime and email-worker secrets"
grep -Fqx 'EnvironmentFile=/etc/apps/ensambria/core/.env' \
    "${staging_directory}/ensambria-core-email-worker.service" \
    || fail "email worker must use the shared CORE environment file"
grep -Fqx 'UnsetEnvironment=DATABASE_URL MIGRATIONS_DATABASE_URL BUSINESS_DATABASE_URL BUSINESS_DATABASE_MAX_CONNECTIONS BUSINESS_HTTP_LISTEN_ADDRESS ENSAMBRIA_REDIS_REQUIRED' \
    "${staging_directory}/ensambria-core-email-worker.service" \
    || fail "email worker must exclude runtime and migration credentials"
grep -Fqx 'EnvironmentFile=/etc/apps/ensambria/core/.env' \
    "${staging_directory}/ensambria-business.service" \
    || fail "BUSINESS must use the shared environment file"
grep -Fqx 'UnsetEnvironment=MIGRATIONS_DATABASE_URL CORE_EMAIL_WORKER_DATABASE_URL CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_IDLE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_MAX_LIFETIME_MS CORE_SMTP_HOST CORE_SMTP_PORT CORE_SMTP_USERNAME CORE_SMTP_PASSWORD CORE_SMTP_FROM' \
    "${staging_directory}/ensambria-business.service" \
    || fail "BUSINESS must exclude migration and email-worker secrets"
grep -Fqx 'EnvironmentFile=/etc/apps/ensambria/core/.env' \
    "${staging_directory}/ensambria-business-migrate.service" \
    || fail "BUSINESS migrations must use the shared environment file"
grep -Fqx 'UnsetEnvironment=DATABASE_URL BUSINESS_DATABASE_URL BUSINESS_DATABASE_MAX_CONNECTIONS CORE_EMAIL_WORKER_DATABASE_URL CORE_EMAIL_WORKER_DATABASE_MAX_CONNECTIONS CORE_EMAIL_WORKER_DATABASE_ACQUIRE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_IDLE_TIMEOUT_MS CORE_EMAIL_WORKER_DATABASE_MAX_LIFETIME_MS CORE_SMTP_HOST CORE_SMTP_PORT CORE_SMTP_USERNAME CORE_SMTP_PASSWORD CORE_SMTP_FROM' \
    "${staging_directory}/ensambria-business-migrate.service" \
    || fail "BUSINESS migrator must exclude runtime and worker credentials"

trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

ensure_group "$service_group"
ensure_group "$migrator_group"
ensure_group "$artifact_group"
ensure_group "$business_service_group"
ensure_user "$service_user" "$service_group"
ensure_user "$business_service_user" "$business_service_group"
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

# Stop the path watcher before normalizing listener addresses so it cannot
# restart services halfway through an atomic deployment.
systemctl stop "$env_reload_path_name" "$env_reload_service_name" >/dev/null 2>&1 || true
python3 - "$environment_file" <<'PY'
import os
import pathlib
import stat
import sys
import tempfile

path = pathlib.Path(sys.argv[1])
required = {
    "CORE_HEALTH_LISTEN_ADDRESS": "127.0.0.1:22080",
    "CORE_METRICS_LISTEN_ADDRESS": "127.0.0.1:22090",
    "BUSINESS_HTTP_LISTEN_ADDRESS": "127.0.0.1:22100",
}
original = path.read_text(encoding="utf-8")
lines = []
written = set()
for line in original.splitlines():
    key, separator, _ = line.partition("=")
    if separator and key in required:
        if key not in written:
            lines.append(f"{key}={required[key]}")
            written.add(key)
    else:
        lines.append(line)
for key, value in required.items():
    if key not in written:
        lines.append(f"{key}={value}")
mode = stat.S_IMODE(path.stat().st_mode)
metadata = path.stat()
fd, temporary_name = tempfile.mkstemp(prefix=".ensambria-env.", dir=str(path.parent))
try:
    with os.fdopen(fd, "w", encoding="utf-8") as temporary:
        temporary.write(chr(10).join(lines) + chr(10))
        temporary.flush()
        os.fsync(temporary.fileno())
    os.chown(temporary_name, metadata.st_uid, metadata.st_gid)
    os.chmod(temporary_name, mode)
    os.replace(temporary_name, path)
except BaseException:
    try:
        os.unlink(temporary_name)
    except FileNotFoundError:
        pass
    raise
PY

for required_variable in \
    DATABASE_URL \
    MIGRATIONS_DATABASE_URL \
    CORE_DATABASE_MAX_CONNECTIONS \
    CORE_DATABASE_ACQUIRE_TIMEOUT_MS \
    CORE_HEALTH_LISTEN_ADDRESS \
    CORE_METRICS_LISTEN_ADDRESS \
    CORE_HEALTH_CHECK_INTERVAL_MS \
    CORE_HEALTH_CHECK_TIMEOUT_MS \
    BUSINESS_DATABASE_URL \
    BUSINESS_HTTP_LISTEN_ADDRESS \
    CORE_PUBLIC_BASE_URL \
    CORE_MFA_ENCRYPTION_KEY_VERSION \
    CORE_MFA_ENCRYPTION_KEY_HEX \
    CORE_AUTH_RATE_LIMIT_SECRET_HEX
do
    grep -q "^${required_variable}=." "$environment_file" \
        || fail "required CORE environment setting is missing"
done

health_address="$(sed -n 's/^CORE_HEALTH_LISTEN_ADDRESS=//p' "$environment_file")"
[[ "$health_address" =~ ^127\.0\.0\.1:([1-9][0-9]{0,4})$ ]] \
    || fail "health listener must use 127.0.0.1 and a valid port"
(( BASH_REMATCH[1] <= 65535 )) || fail "health listener port is invalid"

business_address="$(sed -n 's/^BUSINESS_HTTP_LISTEN_ADDRESS=//p' "$environment_file")"
[[ "$business_address" =~ ^127\.0\.0\.1:([1-9][0-9]{0,4})$ ]] \
    || fail "BUSINESS listener must use 127.0.0.1 and a valid port"
(( BASH_REMATCH[1] <= 65535 )) || fail "BUSINESS listener port is invalid"
[[ "$business_address" != "$health_address" ]] || fail "CORE and BUSINESS listeners must use different ports"
business_health_url="http://127.0.0.1:${business_address##*:}/health"

temporary_directory="$(mktemp -d "${release_root}/.incoming.XXXXXX")"
archive="${staging_directory}/core-staging.tar.gz"
payload="${temporary_directory}/payload"
mkdir "$payload"
tar -tzf "$archive" | while IFS= read -r entry; do
    [[ "$entry" == core || "$entry" == core-migrate || "$entry" == core-email-worker || "$entry" == business || "$entry" == business-migrate || "$entry" == SHA256SUMS || "$entry" == migrations/ || "$entry" == business-migrations/ || "$entry" == business-migrations/bootstrap/ ]] && continue
    [[ "$entry" =~ ^migrations/[0-9]{14}_[a-z0-9_]+\.sql$ || "$entry" =~ ^business-migrations/([a-z0-9_-]+/)?[0-9]{14}_[a-z0-9_]+\.sql$ ]] \
        || fail "deployment package contains an unexpected path"
done || fail "deployment package contains unexpected files"
tar -xzf "$archive" -C "$payload" --no-same-owner --no-same-permissions \
    || fail "could not extract deployment package"
(cd "$payload" && sha256sum --check --strict SHA256SUMS) \
    || fail "deployment package checksum verification failed"
for binary in core core-migrate core-email-worker business business-migrate; do
    [[ -s "$payload/$binary" && -x "$payload/$binary" ]] \
        || fail "deployment package binary is missing or not executable: $binary"
done
find "$payload/migrations" -maxdepth 1 -type f -name '*.sql' -print -quit | grep -q . \
    || fail "deployment package is missing SQL migrations"

release_directory="${release_root}/$(date -u +%Y%m%dT%H%M%SZ)-$$"
mkdir "$release_directory"
mv "$payload/core" "$payload/core-migrate" "$payload/core-email-worker" "$payload/business" "$payload/business-migrate" "$payload/migrations" "$payload/business-migrations" "$release_directory/"
chown -R "root:${artifact_group}" "$release_directory"
chmod 0550 "$release_directory" "$release_directory/core" "$release_directory/core-migrate" "$release_directory/core-email-worker" "$release_directory/business" "$release_directory/business-migrate"
find "$release_directory/migrations" "$release_directory/business-migrations" -type d -exec chmod 0550 {} +
find "$release_directory/migrations" "$release_directory/business-migrations" -type f -name '*.sql' -exec chmod 0440 {} +

previous_release=""
if [[ -L "$current_link" ]]; then
    previous_release="$(readlink -f "$current_link")"
    [[ "$previous_release" == "$release_root/"* ]] || fail "current release link is invalid"
    if [[ -x "$previous_release/core" && -x "$previous_release/core-email-worker" && -x "$previous_release/business" ]]; then
        rollback_available=true
    else
        printf 'Previous release is not rollback-compatible; failed deployment will retain the new release for manual recovery.\\n' >&2
        previous_release=""
    fi
elif [[ -e "$current_link" ]]; then
    fail "current release path is not a symbolic link"
fi

install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-core.service" \
    /etc/systemd/system/ensambria-core.service
install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-core-migrate.service" \
    /etc/systemd/system/ensambria-core-migrate.service
install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-core-email-worker.service" \
    /etc/systemd/system/ensambria-core-email-worker.service
install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-business.service" \
    /etc/systemd/system/ensambria-business.service
install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-business-migrate.service" \
    /etc/systemd/system/ensambria-business-migrate.service
install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-env-reload.service" \
    /etc/systemd/system/ensambria-env-reload.service
install -o root -g root -m 0644 \
    "${staging_directory}/ensambria-env-reload.path" \
    /etc/systemd/system/ensambria-env-reload.path
systemctl daemon-reload
systemctl enable "$service_name" "$email_worker_service_name" "$business_service_name" "$env_reload_path_name"



ln -s "$release_directory" "$temporary_link"
mv -Tf "$temporary_link" "$current_link"
release_activated=true
if ! systemctl start "$migration_service_name"; then
    report_migration_failure
    fail "automatic CORE staging migrations failed"
fi
if ! systemctl start "$business_migration_service_name"; then
    report_business_migration_failure
    fail "automatic BUSINESS staging migrations failed"
fi
systemctl restart "$service_name" "$email_worker_service_name" "$business_service_name" || fail "one or more ENSAMBRIA services failed to restart"
for required_service in "$service_name" "$email_worker_service_name" "$business_service_name"; do
    systemctl is-active --quiet "$required_service" || fail "service did not remain active: $required_service"
done
for binary in core core-email-worker business; do
    [[ -x "$current_link/$binary" ]] || fail "active release is missing executable: $binary"
done
wait_until_healthy "$health_address" || fail "CORE readiness check failed"
for attempt in $(seq 1 30); do
    if curl --fail --silent --show-error --max-time 3 "$business_health_url" >/dev/null; then
        break
    fi
    [[ "$attempt" -lt 30 ]] || fail "BUSINESS readiness check failed"
    sleep 2
done

systemctl enable --now "$env_reload_path_name"
release_activated=false
printf 'CORE deployed, migrations applied, and readiness verified.\n'
