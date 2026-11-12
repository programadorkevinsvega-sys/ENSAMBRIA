# Servicios compartidos

## Identificación y estado

- **Tipo:** agrupación de servicios compartidos de infraestructura.
- **Ubicación:** `infrastructure/shared/`.
- **Estado:** directorios organizativos; no hay servicios configurados o ejecutándose declarados por este README.

## Componentes previstos

- `postgresql/`: base PostgreSQL compartida, con schemas y permisos separados por owner.
- `redis/`: caché con TTL; no es fuente de verdad ni canal de integración.
- `nats-core/`: fan-out realtime efímero cuando aplique.
- `nats-jetstream/`: distribución durable y replay para flujos que lo requieran.
- `vault/`: gestión de secretos y certificados de workloads según la política del proyecto.
- `observability/`: componentes compartidos de telemetría y operación.

## Límites

Estos servicios son infraestructura compartida, no backends ENSAMBRIA. Cada aplicación mantiene su proceso, identidad, credenciales y despliegue; compartir infraestructura no concede acceso cruzado a datos.

La aplicabilidad y configuración de cada componente se documentan e implementan con el caso de uso y fase correspondientes. No se agregan servicios a un Compose de producción ni se asumen conexiones disponibles.

## Operación y validación

No hay configuración, endpoints ni comandos operativos disponibles en este directorio. Las pruebas futuras deben usar dependencias reales y entornos autorizados, de acuerdo con las guías especializadas.

## Fuentes

- [Reglas de arquitectura](../../ARCHITECTURE_RULES.md)
- [Reglas de PostgreSQL y pools](../../docs/rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md)
- [Reglas de Outbox e Inbox](../../docs/rules/ENSAMBRIA_Transactional_Outbox_Inbox.md)
- [Reglas de observabilidad](../../docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md)
