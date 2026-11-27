#!/usr/bin/env bash
set -euo pipefail
umask 077

die() { printf 'nats-native: %s\n' "$1" >&2; exit 1; }
is_private_or_tailscale_ipv4() {
  awk -F. '
    NF == 4 &&
    $1 >= 0 && $1 <= 255 && $2 >= 0 && $2 <= 255 &&
    $3 >= 0 && $3 <= 255 && $4 >= 1 && $4 <= 254 &&
    ($1 == 10 ||
      ($1 == 172 && $2 >= 16 && $2 <= 31) ||
      ($1 == 192 && $2 == 168) ||
      ($1 == 100 && $2 >= 64 && $2 <= 127))
  ' <<< "$1"
}
ROOT=$(cd -- "$(dirname -- "$(realpath -- "$0")")" && pwd)
ENV_FILE=${NATS_ENV_FILE:-$ROOT/.env}
[[ $(id -u) -eq 0 ]] || exec sudo -A env NATS_ENV_FILE="$ENV_FILE" bash "$0" "$@"
[[ $# -eq 0 ]] || die 'no arguments are accepted'
[[ -f "$ENV_FILE" && $(stat -c '%u:%a' "$ENV_FILE") == 0:600 ]] || die 'root-owned .env mode 0600 is required'
set -a
. "$ENV_FILE"
set +a

[[ ${NATS_LISTEN_ADDRESS:-} =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}$ ]] || die 'NATS_LISTEN_ADDRESS must be a private IPv4 address'
[[ ${NATS_EXPORTER_BIND_ADDRESS:-} =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}$ ]] || die 'NATS_EXPORTER_BIND_ADDRESS must be a private IPv4 address'
is_private_or_tailscale_ipv4 "$NATS_LISTEN_ADDRESS" || die 'NATS_LISTEN_ADDRESS must be RFC1918 or Tailscale IPv4'
is_private_or_tailscale_ipv4 "$NATS_EXPORTER_BIND_ADDRESS" || die 'NATS_EXPORTER_BIND_ADDRESS must be RFC1918 or Tailscale IPv4'
[[ ${NATS_DNS_NAME:-} =~ ^[A-Za-z0-9.-]+$ && "$NATS_DNS_NAME" != *REPLACE* ]] || die 'NATS_DNS_NAME must be the issued server certificate DNS SAN'
[[ ${NATS_EXPORTER_DNS_NAME:-} =~ ^[A-Za-z0-9.-]+$ && "$NATS_EXPORTER_DNS_NAME" != *REPLACE* ]] || die 'NATS_EXPORTER_DNS_NAME must be the issued exporter certificate DNS SAN'
[[ ${NATS_PROMETHEUS_SERVER_NAME:-} =~ ^[A-Za-z0-9.-]+$ && "$NATS_PROMETHEUS_SERVER_NAME" != *REPLACE* ]] || die 'NATS_PROMETHEUS_SERVER_NAME must match the exporter TLS certificate'
[[ ${NATS_ENVIRONMENT:-} =~ ^[a-z0-9_-]+$ && "$NATS_ENVIRONMENT" != *REPLACE* ]] || die 'NATS_ENVIRONMENT must be a stable low-cardinality label'
[[ ${NATS_HOST_LABEL:-} =~ ^[A-Za-z0-9._-]+$ && "$NATS_HOST_LABEL" != *REPLACE* ]] || die 'NATS_HOST_LABEL must be a stable host label'
[[ ${NATS_PROMETHEUS_CA_FILE:-} =~ ^/[A-Za-z0-9._/-]+$ && "$NATS_PROMETHEUS_CA_FILE" != *REPLACE* ]] || die 'NATS_PROMETHEUS_CA_FILE must be an absolute path on the Prometheus host'
[[ ${NATS_EXPORTER_HTTP_USER:-} =~ ^[A-Za-z0-9._-]{1,64}$ ]] || die 'NATS_EXPORTER_HTTP_USER must be a bounded basic-auth username'
[[ ${NATS_PROMETHEUS_PASSWORD_FILE:-} =~ ^/[A-Za-z0-9._/-]+$ && "$NATS_PROMETHEUS_PASSWORD_FILE" != *REPLACE* ]] || die 'NATS_PROMETHEUS_PASSWORD_FILE must be an absolute path on the Prometheus host'
[[ ${NATS_NODE_EXPORTER_HTTP_USER:-} =~ ^[A-Za-z0-9._-]{1,64}$ ]] || die 'NATS_NODE_EXPORTER_HTTP_USER must be a bounded basic-auth username'
[[ ${NATS_NODE_PROMETHEUS_PASSWORD_FILE:-} =~ ^/[A-Za-z0-9._/-]+$ && "$NATS_NODE_PROMETHEUS_PASSWORD_FILE" != *REPLACE* ]] || die 'NATS_NODE_PROMETHEUS_PASSWORD_FILE must be an absolute path on the Prometheus host'
[[ ${NATS_PROCESS_EXPORTER_HTTP_USER:-} =~ ^[A-Za-z0-9._-]{1,64}$ ]] || die 'NATS_PROCESS_EXPORTER_HTTP_USER must be a bounded basic-auth username'
[[ ${NATS_PROCESS_PROMETHEUS_PASSWORD_FILE:-} =~ ^/[A-Za-z0-9._/-]+$ && "$NATS_PROCESS_PROMETHEUS_PASSWORD_FILE" != *REPLACE* ]] || die 'NATS_PROCESS_PROMETHEUS_PASSWORD_FILE must be an absolute path on the Prometheus host'
[[ "$NATS_PROMETHEUS_SERVER_NAME" == "$NATS_EXPORTER_DNS_NAME" ]] || die 'Prometheus TLS server name must match the exporter certificate DNS SAN'
NATS_AUTHORIZATION_FILE=${NATS_AUTHORIZATION_FILE:-/etc/ensambria/nats/authorization.conf}
EXPORTER_AUTH_FILE=/etc/ensambria/credentials/nats/exporter-auth.env
NODE_EXPORTER_AUTH_FILE=/etc/ensambria/credentials/nats/node-exporter-auth.env
PROCESS_EXPORTER_AUTH_FILE=/etc/ensambria/credentials/nats/process-exporter-auth.env
[[ "$NATS_AUTHORIZATION_FILE" == /* && "$NATS_AUTHORIZATION_FILE" != *REPLACE* ]] || die 'NATS_AUTHORIZATION_FILE must be an absolute path'

command -v curl >/dev/null 2>&1 || die 'curl is required'
command -v tar >/dev/null 2>&1 || die 'tar is required'
command -v sha256sum >/dev/null 2>&1 || die 'sha256sum is required'
command -v openssl >/dev/null 2>&1 || die 'openssl is required'
command -v systemctl >/dev/null 2>&1 || die 'systemd is required'
command -v ip >/dev/null 2>&1 || die 'iproute2 is required'
command -v df >/dev/null 2>&1 || die 'df is required'

if systemctl is-active --quiet nats-native.service || systemctl is-active --quiet nats.service ||
  systemctl is-active --quiet nats-prometheus-exporter.service ||
  systemctl is-active --quiet nats-node-exporter.service ||
  systemctl is-active --quiet nats-process-exporter.service; then
  die 'NATS or a monitoring exporter is active; refusing to replace a running installation'
fi

ip -4 -o address show | awk '{sub(/\/.*/,"",$4);print $4}' | grep -Fxq "$NATS_LISTEN_ADDRESS" ||
  die 'NATS_LISTEN_ADDRESS is not assigned to this VM'
ip -4 -o address show | awk '{sub(/\/.*/,"",$4);print $4}' | grep -Fxq "$NATS_EXPORTER_BIND_ADDRESS" ||
  die 'NATS_EXPORTER_BIND_ADDRESS is not assigned to this VM'

SERVER_CERT=/etc/ensambria/credentials/nats/server.crt
SERVER_KEY=/etc/ensambria/credentials/nats/server.key
EXPORTER_CERT=/etc/ensambria/credentials/nats/exporter.crt
EXPORTER_KEY=/etc/ensambria/credentials/nats/exporter.key
NODE_EXPORTER_CERT=/etc/ensambria/credentials/nats/node-exporter.crt
NODE_EXPORTER_KEY=/etc/ensambria/credentials/nats/node-exporter.key
PROCESS_EXPORTER_CERT=/etc/ensambria/credentials/nats/process-exporter.crt
PROCESS_EXPORTER_KEY=/etc/ensambria/credentials/nats/process-exporter.key
CA_BUNDLE=/etc/ensambria/credentials/nats/ca-bundle.crt
for certfile in "$SERVER_CERT" "$EXPORTER_CERT" "$NODE_EXPORTER_CERT" "$PROCESS_EXPORTER_CERT" "$CA_BUNDLE"; do
  [[ -r "$certfile" ]] || die "required public certificate file is unreadable: $certfile"
done
for keyfile in "$SERVER_KEY" "$EXPORTER_KEY" "$NODE_EXPORTER_KEY" "$PROCESS_EXPORTER_KEY"; do
  [[ -r "$keyfile" ]] || die "required private key file is unreadable: $keyfile"
done
[[ -f "$NATS_AUTHORIZATION_FILE" && ! -L "$NATS_AUTHORIZATION_FILE" && $(stat -c '%u' "$NATS_AUTHORIZATION_FILE") == 0 ]] ||
  die 'a root-owned authorization.conf with provisioned mTLS users is required'
[[ -f "$EXPORTER_AUTH_FILE" && ! -L "$EXPORTER_AUTH_FILE" && $(stat -c '%u:%a' "$EXPORTER_AUTH_FILE") == 0:640 ]] ||
  die 'root-owned exporter-auth.env mode 0640 is required'
[[ -f "$NODE_EXPORTER_AUTH_FILE" && ! -L "$NODE_EXPORTER_AUTH_FILE" && $(stat -c '%u:%a' "$NODE_EXPORTER_AUTH_FILE") == 0:640 ]] ||
  die 'root-owned node-exporter-auth.env mode 0640 is required'
[[ -f "$PROCESS_EXPORTER_AUTH_FILE" && ! -L "$PROCESS_EXPORTER_AUTH_FILE" && $(stat -c '%u:%a' "$PROCESS_EXPORTER_AUTH_FILE") == 0:640 ]] ||
  die 'root-owned process-exporter-auth.env mode 0640 is required'
EXPORTER_AUTH_USER=$(sed -n 's/^NATS_EXPORTER_HTTP_USER=//p' "$EXPORTER_AUTH_FILE")
EXPORTER_AUTH_HASH=$(sed -n 's/^NATS_EXPORTER_HTTP_BCRYPT_PASSWORD=//p' "$EXPORTER_AUTH_FILE")
[[ "$EXPORTER_AUTH_USER" == "$NATS_EXPORTER_HTTP_USER" ]] || die 'exporter-auth.env user does not match NATS_EXPORTER_HTTP_USER'
[[ "$EXPORTER_AUTH_HASH" =~ ^\$2a\$[0-9]{2}\$[./A-Za-z0-9]{53}$ ]] || die 'exporter-auth.env must contain a valid $2a$ bcrypt password hash'
NODE_EXPORTER_AUTH_USER=$(sed -n 's/^NATS_NODE_EXPORTER_HTTP_USER=//p' "$NODE_EXPORTER_AUTH_FILE")
NODE_EXPORTER_AUTH_HASH=$(sed -n 's/^NATS_NODE_EXPORTER_HTTP_BCRYPT_PASSWORD=//p' "$NODE_EXPORTER_AUTH_FILE")
[[ "$NODE_EXPORTER_AUTH_USER" == "$NATS_NODE_EXPORTER_HTTP_USER" ]] || die 'node-exporter-auth.env user does not match NATS_NODE_EXPORTER_HTTP_USER'
[[ "$NODE_EXPORTER_AUTH_HASH" =~ ^\$2a\$[0-9]{2}\$[./A-Za-z0-9]{53}$ ]] || die 'node-exporter-auth.env must contain a valid $2a$ bcrypt password hash'
PROCESS_EXPORTER_AUTH_USER=$(sed -n 's/^NATS_PROCESS_EXPORTER_HTTP_USER=//p' "$PROCESS_EXPORTER_AUTH_FILE")
PROCESS_EXPORTER_AUTH_HASH=$(sed -n 's/^NATS_PROCESS_EXPORTER_HTTP_BCRYPT_PASSWORD=//p' "$PROCESS_EXPORTER_AUTH_FILE")
[[ "$PROCESS_EXPORTER_AUTH_USER" == "$NATS_PROCESS_EXPORTER_HTTP_USER" ]] || die 'process-exporter-auth.env user does not match NATS_PROCESS_EXPORTER_HTTP_USER'
[[ "$PROCESS_EXPORTER_AUTH_HASH" =~ ^\$2a\$[0-9]{2}\$[./A-Za-z0-9]{53}$ ]] || die 'process-exporter-auth.env must contain a valid $2a$ bcrypt password hash'
grep -Eq '^[[:space:]]*user:' "$NATS_AUTHORIZATION_FILE" || die 'authorization.conf has no provisioned mTLS-mapped users'
grep -Eq '^[[:space:]]*permissions:' "$NATS_AUTHORIZATION_FILE" || die 'authorization.conf has no scoped user permissions'
grep -Eq '^[[:space:]]*publish:' "$NATS_AUTHORIZATION_FILE" || die 'authorization.conf has no publish scope'
grep -Eq '^[[:space:]]*subscribe:' "$NATS_AUTHORIZATION_FILE" || die 'authorization.conf has no subscribe scope'
if grep -Eiq '^[[:space:]]*password:' "$NATS_AUTHORIZATION_FILE"; then
  die 'password authentication is not accepted; use issued mTLS identities'
fi
if grep -Eiq '^[[:space:]]*allow:[[:space:]]*\[[[:space:]]*">"[[:space:]]*\]' "$NATS_AUTHORIZATION_FILE"; then
  die 'unrestricted publish/subscribe scope is not accepted'
fi

openssl verify -CAfile "$CA_BUNDLE" -purpose sslserver "$SERVER_CERT" >/dev/null ||
  die 'NATS server certificate chain or purpose is invalid'
openssl verify -CAfile "$CA_BUNDLE" -purpose sslserver "$EXPORTER_CERT" >/dev/null ||
  die 'NATS exporter certificate chain or purpose is invalid'
openssl verify -CAfile "$CA_BUNDLE" -purpose sslserver "$NODE_EXPORTER_CERT" >/dev/null ||
  die 'node_exporter certificate chain or purpose is invalid'
openssl verify -CAfile "$CA_BUNDLE" -purpose sslserver "$PROCESS_EXPORTER_CERT" >/dev/null ||
  die 'process-exporter certificate chain or purpose is invalid'
openssl x509 -in "$SERVER_CERT" -noout -checkhost "$NATS_DNS_NAME" >/dev/null 2>&1 ||
  die 'NATS server certificate DNS SAN mismatch'
openssl x509 -in "$SERVER_CERT" -noout -checkip "$NATS_LISTEN_ADDRESS" >/dev/null 2>&1 ||
  die 'NATS server certificate private IP SAN mismatch'
openssl x509 -in "$EXPORTER_CERT" -noout -checkhost "$NATS_EXPORTER_DNS_NAME" >/dev/null 2>&1 ||
  die 'NATS exporter certificate DNS SAN mismatch'
openssl x509 -in "$EXPORTER_CERT" -noout -checkip "$NATS_EXPORTER_BIND_ADDRESS" >/dev/null 2>&1 ||
  die 'NATS exporter certificate private IP SAN mismatch'
for certificate in "$NODE_EXPORTER_CERT" "$PROCESS_EXPORTER_CERT"; do
  openssl x509 -in "$certificate" -noout -checkhost "$NATS_EXPORTER_DNS_NAME" >/dev/null 2>&1 ||
    die "exporter certificate DNS SAN mismatch: $certificate"
  openssl x509 -in "$certificate" -noout -checkip "$NATS_EXPORTER_BIND_ADDRESS" >/dev/null 2>&1 ||
    die "exporter certificate private IP SAN mismatch: $certificate"
done

AVAILABLE_GB=$(df -BG --output=avail /var | awk 'NR == 2 {gsub(/G/,"",$1); print $1}')
[[ "$AVAILABLE_GB" =~ ^[0-9]+$ ]] && (( AVAILABLE_GB >= 80 )) ||
  die 'at least 80 GiB must be free on the filesystem for the 65 GiB JetStream limit and operating headroom'

case "$(uname -m)" in
  x86_64)
    NATS_ARCH=amd64
    EXPORTER_ARCH=x86_64
    NODE_EXPORTER_ARCH=amd64
    PROCESS_EXPORTER_ARCH=amd64
    NATS_SHA256=32e18c1e78d7ef4f86e483070eef3abb2647c3d4d67e29668077c7ad81da0797
    EXPORTER_SHA256=be2eab36ebc5f57fdd9ad1c61d4022f4fa436d6339336bea8d5b7587b879bb47
    NODE_EXPORTER_SHA256=5c00c5234f1ee45900b25539a5a0ab82146a098c15c16a4ec3bc855b597a6477
    PROCESS_EXPORTER_SHA256=6d274cca5e94c6a25e55ec05762a472561859ce0a05b984aaedb67dd857ceee2
    ;;
  aarch64)
    NATS_ARCH=arm64
    EXPORTER_ARCH=arm64
    NODE_EXPORTER_ARCH=arm64
    PROCESS_EXPORTER_ARCH=arm64
    NATS_SHA256=4a2088b3ec7103bf79d6484f55e80ab5d10204811d9ae6f7a33e8544d98dd45d
    EXPORTER_SHA256=7d82dc00a7eb1b14ea7f18ab917dcc350a45dda60ff3af877264aee7b721db58
    NODE_EXPORTER_SHA256=96d82b3b261803f89815b325601cbbc58f49cd8265617bfde1527211934bdd48
    PROCESS_EXPORTER_SHA256=4a2502f290323e57eeeb070fc10e64047ad0cd838ae5a1b347868f75667b5ab0
    ;;
  *) die "unsupported architecture: $(uname -m); supported architectures are x86_64 and aarch64" ;;
esac

WORK_DIR=$(mktemp -d /tmp/ensambria-nats-native.XXXXXX)
trap 'rm -rf "$WORK_DIR"' EXIT
NATS_ARCHIVE="nats-server-v2.15.3-linux-${NATS_ARCH}.tar.gz"
EXPORTER_ARCHIVE="prometheus-nats-exporter-v0.20.3-linux-${EXPORTER_ARCH}.tar.gz"
NODE_EXPORTER_ARCHIVE="node_exporter-1.13.1.linux-${NODE_EXPORTER_ARCH}.tar.gz"
PROCESS_EXPORTER_ARCHIVE="process-exporter-0.8.7.linux-${PROCESS_EXPORTER_ARCH}.tar.gz"
curl --fail --silent --show-error --location \
  "https://github.com/nats-io/nats-server/releases/download/v2.15.3/$NATS_ARCHIVE" \
  --output "$WORK_DIR/$NATS_ARCHIVE"
printf '%s  %s\n' "$NATS_SHA256" "$WORK_DIR/$NATS_ARCHIVE" | sha256sum --check --status ||
  die 'NATS server archive checksum verification failed'
curl --fail --silent --show-error --location \
  "https://github.com/nats-io/prometheus-nats-exporter/releases/download/v0.20.3/$EXPORTER_ARCHIVE" \
  --output "$WORK_DIR/$EXPORTER_ARCHIVE"
printf '%s  %s\n' "$EXPORTER_SHA256" "$WORK_DIR/$EXPORTER_ARCHIVE" | sha256sum --check --status ||
  die 'NATS Prometheus exporter archive checksum verification failed'
curl --fail --silent --show-error --location \
  "https://github.com/prometheus/node_exporter/releases/download/v1.13.1/$NODE_EXPORTER_ARCHIVE" \
  --output "$WORK_DIR/$NODE_EXPORTER_ARCHIVE"
printf '%s  %s\n' "$NODE_EXPORTER_SHA256" "$WORK_DIR/$NODE_EXPORTER_ARCHIVE" | sha256sum --check --status ||
  die 'node_exporter archive checksum verification failed'
curl --fail --silent --show-error --location \
  "https://github.com/ncabatoff/process-exporter/releases/download/v0.8.7/$PROCESS_EXPORTER_ARCHIVE" \
  --output "$WORK_DIR/$PROCESS_EXPORTER_ARCHIVE"
printf '%s  %s\n' "$PROCESS_EXPORTER_SHA256" "$WORK_DIR/$PROCESS_EXPORTER_ARCHIVE" | sha256sum --check --status ||
  die 'process-exporter archive checksum verification failed'
mkdir "$WORK_DIR/nats" "$WORK_DIR/exporter" "$WORK_DIR/node-exporter" "$WORK_DIR/process-exporter"
tar -xzf "$WORK_DIR/$NATS_ARCHIVE" -C "$WORK_DIR/nats"
tar -xzf "$WORK_DIR/$EXPORTER_ARCHIVE" -C "$WORK_DIR/exporter"
tar -xzf "$WORK_DIR/$NODE_EXPORTER_ARCHIVE" -C "$WORK_DIR/node-exporter"
tar -xzf "$WORK_DIR/$PROCESS_EXPORTER_ARCHIVE" -C "$WORK_DIR/process-exporter"
NATS_BINARY=$(find "$WORK_DIR/nats" -type f -name nats-server -print -quit)
EXPORTER_BINARY=$(find "$WORK_DIR/exporter" -type f -name prometheus-nats-exporter -print -quit)
NODE_EXPORTER_BINARY=$(find "$WORK_DIR/node-exporter" -type f -name node_exporter -print -quit)
PROCESS_EXPORTER_BINARY=$(find "$WORK_DIR/process-exporter" -type f -name process-exporter -print -quit)
[[ -n "$NATS_BINARY" && -n "$EXPORTER_BINARY" && -n "$NODE_EXPORTER_BINARY" && -n "$PROCESS_EXPORTER_BINARY" ]] ||
  die 'a release archive did not contain the expected binary'

getent group nats >/dev/null || groupadd --system nats
getent passwd nats >/dev/null || useradd --system --gid nats --home-dir /var/lib/nats --shell /usr/sbin/nologin nats
getent group nats-exporter >/dev/null || groupadd --system nats-exporter
getent passwd nats-exporter >/dev/null || useradd --system --gid nats-exporter --home-dir /var/lib/nats-exporter --shell /usr/sbin/nologin nats-exporter
getent group node-exporter >/dev/null || groupadd --system node-exporter
getent passwd node-exporter >/dev/null || useradd --system --gid node-exporter --home-dir /var/lib/node-exporter --shell /usr/sbin/nologin node-exporter
getent group process-exporter >/dev/null || groupadd --system process-exporter
getent passwd process-exporter >/dev/null || useradd --system --gid process-exporter --home-dir /var/lib/process-exporter --shell /usr/sbin/nologin process-exporter
getent group nats-tls >/dev/null || groupadd --system nats-tls
usermod -a -G nats-tls nats
usermod -a -G nats-tls nats-exporter
usermod -a -G nats-tls node-exporter
usermod -a -G nats-tls process-exporter

install -d -o root -g nats -m 0750 /etc/ensambria/nats
install -d -o root -g root -m 0750 /etc/ensambria/nats/monitoring
install -d -o root -g root -m 0755 /etc/ensambria/credentials/nats
install -d -o nats -g nats -m 0750 /var/lib/nats/jetstream
install -d -o nats-exporter -g nats-exporter -m 0750 /var/lib/nats-exporter
chown root:nats-tls "$SERVER_KEY" "$EXPORTER_KEY" "$NODE_EXPORTER_KEY" "$PROCESS_EXPORTER_KEY"
chmod 0640 "$SERVER_KEY" "$EXPORTER_KEY" "$NODE_EXPORTER_KEY" "$PROCESS_EXPORTER_KEY"
chown root:nats-tls "$SERVER_CERT" "$EXPORTER_CERT" "$NODE_EXPORTER_CERT" "$PROCESS_EXPORTER_CERT" "$CA_BUNDLE"
chmod 0644 "$SERVER_CERT" "$EXPORTER_CERT" "$NODE_EXPORTER_CERT" "$PROCESS_EXPORTER_CERT" "$CA_BUNDLE"
chown root:nats-exporter "$EXPORTER_AUTH_FILE"
chmod 0640 "$EXPORTER_AUTH_FILE"
chown root:node-exporter "$NODE_EXPORTER_AUTH_FILE"
chmod 0640 "$NODE_EXPORTER_AUTH_FILE"
chown root:process-exporter "$PROCESS_EXPORTER_AUTH_FILE"
chmod 0640 "$PROCESS_EXPORTER_AUTH_FILE"
if [[ "$NATS_AUTHORIZATION_FILE" != /etc/ensambria/nats/authorization.conf ]]; then
  install -o root -g nats -m 0640 "$NATS_AUTHORIZATION_FILE" /etc/ensambria/nats/authorization.conf
else
  chown root:nats /etc/ensambria/nats/authorization.conf
  chmod 0640 /etc/ensambria/nats/authorization.conf
fi
install -o root -g root -m 0755 "$NATS_BINARY" /usr/local/bin/nats-server
install -o root -g root -m 0755 "$EXPORTER_BINARY" /usr/local/bin/prometheus-nats-exporter
install -o root -g root -m 0755 "$NODE_EXPORTER_BINARY" /usr/local/bin/node_exporter
install -o root -g root -m 0755 "$PROCESS_EXPORTER_BINARY" /usr/local/bin/process-exporter

sed "s|NATS_LISTEN_ADDRESS|$NATS_LISTEN_ADDRESS|g" \
  "$ROOT/nats.conf.tmpl" > /etc/ensambria/nats/nats.conf
chown root:nats /etc/ensambria/nats/nats.conf
chmod 0640 /etc/ensambria/nats/nats.conf
sed "s|NATS_EXPORTER_BIND_ADDRESS|$NATS_EXPORTER_BIND_ADDRESS|g" \
  "$ROOT/prometheus-nats-exporter.service.tmpl" \
  > /etc/systemd/system/nats-prometheus-exporter.service
chown root:root /etc/systemd/system/nats-prometheus-exporter.service
chmod 0644 /etc/systemd/system/nats-prometheus-exporter.service
for exporter in node process; do
  sed "s|NATS_EXPORTER_BIND_ADDRESS|$NATS_EXPORTER_BIND_ADDRESS|g" \
    "$ROOT/${exporter}-exporter.service" \
    > "/etc/systemd/system/nats-${exporter}-exporter.service"
  chown root:root "/etc/systemd/system/nats-${exporter}-exporter.service"
  chmod 0644 "/etc/systemd/system/nats-${exporter}-exporter.service"
done
install -d -o root -g root -m 0755 /usr/local/libexec
sed "s|NATS_EXPORTER_BIND_ADDRESS|$NATS_EXPORTER_BIND_ADDRESS|g" \
  "$ROOT/prometheus-nats-exporter.sh.tmpl" \
  > /usr/local/libexec/ensambria-nats-prometheus-exporter
chown root:root /usr/local/libexec/ensambria-nats-prometheus-exporter
chmod 0755 /usr/local/libexec/ensambria-nats-prometheus-exporter
for exporter in node process; do
  if [[ "$exporter" == node ]]; then
    AUTH_USER=$NODE_EXPORTER_AUTH_USER
    AUTH_HASH=$NODE_EXPORTER_AUTH_HASH
  else
    AUTH_USER=$PROCESS_EXPORTER_AUTH_USER
    AUTH_HASH=$PROCESS_EXPORTER_AUTH_HASH
  fi
  sed -e "s|EXPORTER_HTTP_USER|$AUTH_USER|g" \
    -e "s|EXPORTER_HTTP_BCRYPT_PASSWORD|$AUTH_HASH|g" \
    -e "s|EXPORTER_CERT_NAME|${exporter}-exporter|g" \
    "$ROOT/exporter-web.yml.tmpl" > "/etc/ensambria/nats/${exporter}-exporter-web.yml"
  chown "root:${exporter}-exporter" "/etc/ensambria/nats/${exporter}-exporter-web.yml"
  chmod 0640 "/etc/ensambria/nats/${exporter}-exporter-web.yml"
done
install -o root -g process-exporter -m 0640 "$ROOT/process-exporter.yml" /etc/ensambria/nats/process-exporter.yml
sed -e "s|NATS_EXPORTER_BIND_ADDRESS|$NATS_EXPORTER_BIND_ADDRESS|g" \
  -e "s|NATS_PROMETHEUS_SERVER_NAME|$NATS_PROMETHEUS_SERVER_NAME|g" \
  -e "s|NATS_PROMETHEUS_CA_FILE|$NATS_PROMETHEUS_CA_FILE|g" \
  -e "s|NATS_EXPORTER_HTTP_USER|$NATS_EXPORTER_HTTP_USER|g" \
  -e "s|NATS_PROMETHEUS_PASSWORD_FILE|$NATS_PROMETHEUS_PASSWORD_FILE|g" \
  -e "s|NATS_NODE_EXPORTER_HTTP_USER|$NATS_NODE_EXPORTER_HTTP_USER|g" \
  -e "s|NATS_NODE_PROMETHEUS_PASSWORD_FILE|$NATS_NODE_PROMETHEUS_PASSWORD_FILE|g" \
  -e "s|NATS_PROCESS_EXPORTER_HTTP_USER|$NATS_PROCESS_EXPORTER_HTTP_USER|g" \
  -e "s|NATS_PROCESS_PROMETHEUS_PASSWORD_FILE|$NATS_PROCESS_PROMETHEUS_PASSWORD_FILE|g" \
  -e "s|NATS_ENVIRONMENT|$NATS_ENVIRONMENT|g" \
  -e "s|NATS_HOST_LABEL|$NATS_HOST_LABEL|g" \
  "$ROOT/prometheus-nats-scrape.yml.tmpl" \
  > /etc/ensambria/nats/monitoring/prometheus-nats-scrape.yml
chown root:root /etc/ensambria/nats/monitoring/prometheus-nats-scrape.yml
chmod 0644 /etc/ensambria/nats/monitoring/prometheus-nats-scrape.yml
install -o root -g root -m 0644 "$ROOT/prometheus-nats.rules.yml" /etc/ensambria/nats/monitoring/prometheus-nats.rules.yml
install -o root -g root -m 0644 "$ROOT/grafana-nats-dashboard.json" /etc/ensambria/nats/monitoring/grafana-nats-dashboard.json
install -o root -g root -m 0644 "$ROOT/nats-native.service" /etc/systemd/system/nats-native.service

systemctl daemon-reload
/usr/local/bin/nats-server -t -c /etc/ensambria/nats/nats.conf ||
  die 'NATS configuration validation failed'
systemctl enable --now nats-native.service
systemctl enable --now nats-prometheus-exporter.service
systemctl enable --now nats-node-exporter.service
systemctl enable --now nats-process-exporter.service
systemctl is-active --quiet nats-native.service || die 'NATS server did not become active'
systemctl is-active --quiet nats-prometheus-exporter.service || die 'NATS exporter did not become active'
systemctl is-active --quiet nats-node-exporter.service || die 'node_exporter did not become active'
systemctl is-active --quiet nats-process-exporter.service || die 'process-exporter did not become active'
curl --fail --silent --show-error --max-time 5 http://127.0.0.1:8222/healthz?js-enabled-only=true >/dev/null ||
  die 'NATS JetStream health endpoint did not become healthy'

printf 'NATS native installed: server=v2.15.3 exporter=v0.20.3 node_exporter=v1.13.1 process-exporter=v0.8.7 listen=%s:4222 JetStream max_mem=512MB max_file=65GB.\n' "$NATS_LISTEN_ADDRESS"
