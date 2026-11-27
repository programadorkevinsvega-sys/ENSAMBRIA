# NATS nativo con JetStream

Esta unidad instala NATS Server como servicio `systemd`, sin Docker, clúster,
rutas ni réplica. JetStream queda activo en una sola VM de 2 vCPU, 4 GiB RAM y
100 GB de almacenamiento.

## Recursos y disponibilidad

- JetStream memory store: máximo `512MB`.
- JetStream file store: máximo `65GB`.
- Servicio NATS: `MemoryMax=2G`.
- El límite de 65 GB deja margen para sistema, logs y operación; el instalador
  exige al menos 80 GiB libres en el filesystem de `/var`.
- El máximo global no determina la retención del negocio. Cada stream debe
  establecer límites/retención compatibles con su owner y volumen.
- Al ser un nodo único, `replicas=1` no ofrece alta disponibilidad. Una caída
  de VM detiene la mensajería hasta recuperar el nodo. El volumen JetStream
  debe protegerse con respaldos operativos probados; no hay failover de nodo.

## Seguridad

- Clientes usan TLS mutuo y NATS mapea la identidad del certificado a un usuario.
- El puerto cliente `4222` escucha solo en una IP privada/Tailscale configurada.
- El endpoint de monitorización NATS `8222` solo escucha en loopback.
- El exporter sirve HTTPS con autenticación Basic por usuario y contraseña
  bcrypt. El broker conserva solo el hash; la contraseña de scraping reside en
  un `password_file` protegido en Prometheus. Restringir además el puerto
  `7777` en firewall/ACL de red al Prometheus autorizado.
- `node_exporter` (`9100`) y `process-exporter` (`9256`) también sirven HTTPS
  con Basic Auth y certificados separados. Sus puertos solo deben aceptar
  conexiones desde el Prometheus autorizado.
- `process-exporter` publica únicamente grupos de procesos incluidos en su
  allowlist; no exporta argumentos, PID ni procesos no configurados.
- El instalador exige una `authorization.conf` administrada en la VM con
  identidades mTLS emitidas y scopes publish/subscribe reales. El ejemplo del
  repositorio es intencionalmente inactivo; no se inventan usuarios, subjects
  ni permisos de aplicación.
- No habilita credenciales compartidas, acceso anónimo ni autorización global.

## Instalación

1. Preparar certificados separados bajo `/etc/ensambria/credentials/nats/`:
   `server.crt`/`server.key`, `exporter.crt`/`exporter.key`,
   `node-exporter.crt`/`node-exporter.key`,
   `process-exporter.crt`/`process-exporter.key` y `ca-bundle.crt`.
   Los certificados de los tres exporters deben incluir el SAN
   `NATS_EXPORTER_DNS_NAME` y la IP privada configurada para los exporters.
2. Provisionar estos archivos con usuario y hash bcrypt `$2a$`, propietario
   root y modo `0640`:
   `/etc/ensambria/credentials/nats/exporter-auth.env`,
   `/etc/ensambria/credentials/nats/node-exporter-auth.env` y
   `/etc/ensambria/credentials/nats/process-exporter-auth.env`. El original
   aleatorio de cada contraseña se mantiene en el `password_file` configurado
   en Prometheus, nunca en este repositorio. Consultar
   [exporter-auth.env.example](./exporter-auth.env.example),
   [node-exporter-auth.env.example](./node-exporter-auth.env.example) y
   [process-exporter-auth.env.example](./process-exporter-auth.env.example)
   solo como formatos.
3. Crear `/etc/ensambria/nats/authorization.conf` con usuarios y permisos
   estrictos. Revisar [authorization.conf.example](./authorization.conf.example)
   como formato; sustituir todos los marcadores por identidades/subjects
   aprobados y mantener la política fail-closed.
4. Copiar `.env.example` a `.env`, reemplazar todos los marcadores por valores
   reales y protegerlo: propietario root, modo `0600`.
5. Ejecutar:

   ```bash
   sudo -A infra/nats/nats-nativo/install-native.sh
   ```

El instalador descarga versiones fijadas de los releases oficiales, verifica
SHA-256 antes de instalar NATS Server, el NATS exporter, `node_exporter` y
`process-exporter`. Valida la configuración NATS e inicia los cuatro servicios
`systemd`; no despliega Prometheus ni Grafana. El scrape TLS y Basic Auth se
valida desde el Prometheus autorizado, no desde el instalador.

## Métricas

El servicio `nats-prometheus-exporter` consulta `/varz`, `/connz`, `/subsz`,
`/jsz?consumers=true` y la salud de JetStream, y publica `/metrics` sobre TLS.
Incluye métricas del servidor, tráfico/conexiones, slow consumers, streams,
consumers, almacenamiento, memoria JetStream, mensajes pendientes,
ack-pending, redeliveries y progreso de ACK. `node_exporter` entrega CPU,
memoria, load average, disco, I/O, red e identidad del host. `process-exporter`
entrega métricas CPU/RSS de los grupos de proceso permitidos en
[process-exporter.yml](./process-exporter.yml); el Top 10 solo compara esos
grupos y no afirma ser un inventario de todos los procesos del sistema. Los tres
endpoints requieren Basic Auth además de TLS; las credenciales se validan como
hashes bcrypt `$2a$`.

Instalar los artefactos generados/versionados en el Prometheus/Grafana
autorizado:

- `monitoring/prometheus-nats-scrape.yml`
- `monitoring/prometheus-nats.rules.yml`
- `monitoring/grafana-nats-dashboard.json`

Los tres jobs incluyen labels `app`, `service`, `environment` y `host`; deben
mantenerse estables y de baja cardinalidad. La configuración Prometheus
referencia un `password_file` protegido por exporter que contiene la contraseña
original de scraping. Los puertos `7777`, `9100` y `9256` deben limitarse por
firewall/ACL al Prometheus autorizado.

NATS/JetStream es un broker que no genera `http_requests_total` ni
`http_request_duration_seconds_bucket` de una aplicación de negocio. No se
simulan ni se atribuyen al broker: cada servicio HTTP instrumenta y publica sus
propias métricas con labels `app` y `service`. El dashboard incluye paneles
genéricos para esas series si ya llegan al mismo Prometheus; esta instalación
no inventa sus targets porque cada servicio HTTP debe configurar su scrape en
su despliegue propietario.

Las etiquetas de recursos (nombres/descripciones de streams y consumers) deben
mantenerse libres de información personal o sensible: el exporter puede
exponerlas como labels de métricas.

Los umbrales de 80/90% de almacenamiento y 90% de memory store son alertas
iniciales de capacidad, no SLO/SLA. Deben revisarse con evidencia de carga
antes de producción. No se crean alertas por cada redelivery.

Para inspección local, desde la VM:

```bash
curl --fail http://127.0.0.1:8222/healthz?js-enabled-only=true
curl --fail http://127.0.0.1:8222/varz
curl --fail 'http://127.0.0.1:8222/jsz?streams=true&consumers=true'
```

La consola Prometheus comprueba los targets `ensambria-nats`,
`ensambria-nats-node` y `ensambria-nats-processes`; el dashboard de Grafana
requiere datasource Prometheus. El endpoint de monitorización NATS no debe
publicarse en interfaces externas. Los logs de systemd permanecen en journald;
el envío a Loki no está configurado en esta unidad.

## Validación y estado

Las pruebas estáticas están junto al despliegue. La aceptación en runtime
requiere ejecutar en la VM autorizada: validación `nats-server -t`, health
JetStream, métricas de los tres exporters, scrape Prometheus `UP`, carga de
reglas y dashboard, TLS y Basic Auth positivos/negativos, mTLS NATS
positivo/negativo, autorización por subject, reinicio y recuperación del store.
Las métricas/alertas no se consideran activas hasta comprobar los scrapes
TLS/Basic Auth en Prometheus/Grafana reales.
