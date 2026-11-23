#!/usr/bin/env bash
# Run from a local ENSAMBRIA checkout. This tool is intentionally manual and is
# not referenced by CI/CD. It uploads this checkout to a temporary directory
# on the selected staging host and runs the remote E2E harness there.
set -Eeuo pipefail

usage() {
  cat <<'USAGE'
Usage:
  ENSAMBRIA_STAGING_SSH_TARGET=user@staging-host \
  ENSAMBRIA_STAGING_ENV_FILE=/etc/apps/ensambria/e2e-staging.env \
    scripts/manual-staging-core-platform-e2e.sh

Required local settings:
  ENSAMBRIA_STAGING_SSH_TARGET  SSH destination (user@host or SSH config alias)
  ENSAMBRIA_STAGING_ENV_FILE    Shell-compatible env file already on the VM

Optional:
  ENSAMBRIA_STAGING_SSH_OPTS    Extra SSH options (default: none)
  ENSAMBRIA_STAGING_EXPECTED_HOSTNAME  Exact remote hostname guard

The remote env file must explicitly set ENSAMBRIA_ENVIRONMENT=staging and
provide CORE_TENANT_DATABASE_URL, CORE_OUTBOX_DATABASE_URL,
PLATFORM_EVENT_DATABASE_URL, PLATFORM_VERIFY_DATABASE_URL and
ENSAMBRIA_NATS_URL. No CI/CD workflow invokes this script.
USAGE
}

fail() { printf 'ERROR: %s\n' "$*" >&2; exit 2; }
[[ "${1:-}" != "--help" && "${1:-}" != "-h" ]] || { usage; exit 0; }
[[ $# -eq 0 ]] || fail "unexpected arguments"
: "${ENSAMBRIA_STAGING_SSH_TARGET:?Set ENSAMBRIA_STAGING_SSH_TARGET}"
: "${ENSAMBRIA_STAGING_ENV_FILE:?Set ENSAMBRIA_STAGING_ENV_FILE}"
command -v ssh >/dev/null || fail "ssh is required"
command -v scp >/dev/null || fail "scp is required"
command -v tar >/dev/null || fail "tar is required"
[[ -f Cargo.toml && -d backend/core && -d backend/platform ]] || fail "run from the ENSAMBRIA repository root"
[[ -f "${ENSAMBRIA_STAGING_ENV_FILE}" ]] || fail "remote env-file path is not configured (it must exist on the VM)"
[[ -f scripts/manual-staging-core-platform-e2e-remote.sh ]] || fail "remote runner is missing"

ssh_opts=(-o BatchMode=yes -o StrictHostKeyChecking=yes)
if [[ -n "${ENSAMBRIA_STAGING_SSH_OPTS:-}" ]]; then
  # Deliberately split SSH options; do not use this variable for secrets.
  read -r -a extra_opts <<< "${ENSAMBRIA_STAGING_SSH_OPTS}"
  ssh_opts+=("${extra_opts[@]}")
fi

archive="$(mktemp "${TMPDIR:-/tmp}/ensambria-staging-e2e.XXXXXX.tar.gz")"
remote_dir="/tmp/ensambria-manual-e2e-$RANDOM-$$"
cleanup() { rm -f "$archive"; }
trap cleanup EXIT

tar -czf "$archive" \
  --exclude=.git --exclude=target --exclude=node_modules --exclude=.env \
  --exclude='**/.env' --exclude='**/secrets*' .
ssh "${ssh_opts[@]}" "$ENSAMBRIA_STAGING_SSH_TARGET" \
  "umask 077 && mkdir -p '$remote_dir' && chmod 700 '$remote_dir'"
scp "${ssh_opts[@]}" "$archive" "$ENSAMBRIA_STAGING_SSH_TARGET:$remote_dir/source.tar.gz"
printf 'Uploading source snapshot to staging host; no service or infrastructure is restarted.\n'
ssh -tt "${ssh_opts[@]}" "$ENSAMBRIA_STAGING_SSH_TARGET" \
  "tar -xzf '$remote_dir/source.tar.gz' -C '$remote_dir' && ENSAMBRIA_STAGING_EXPECTED_HOSTNAME='${ENSAMBRIA_STAGING_EXPECTED_HOSTNAME:-}' bash '$remote_dir/scripts/manual-staging-core-platform-e2e-remote.sh' '$remote_dir' '$ENSAMBRIA_STAGING_ENV_FILE'; status=\$?; rm -rf '$remote_dir'; exit \$status"
