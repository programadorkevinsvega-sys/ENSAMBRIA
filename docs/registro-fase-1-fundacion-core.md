# Registro de Fase 1: Fundación de CORE

## Estado vigente

**Fase 1: ABIERTA, SIN IMPLEMENTAR y NO ACEPTADA.** El código y los artefactos de implementación asociados a registros anteriores fueron retirados durante el reinicio del repositorio. No hay evidencia vigente de servicios ejecutables, persistencia, integraciones o pruebas de producto para esta fase.

**Subfase 1.1:** el propietario autorizó iniciar su trabajo el 2026-11-18 mientras Fase 0 continúa abierta. Esta autorización es acotada a 1.1: no cierra ni acepta Fase 0, no habilita 1.2, no autoriza integrar cambios sin los controles/revisiones aplicables y no autoriza acceder, modificar o desplegar en staging. El alcance aprobado para 1.1 es fundamento técnico sin recorridos de producto; la matriz previa de integraciones y sus N/A concretos aparece más adelante. No hay implementación ni validación de 1.1 todavía.

Esto no revoca las decisiones arquitectónicas que fueron aceptadas en Fase 0 o ratificadas para el diseño de Fase 1. Los ADR conservan el estado expresado en cada documento y siguen siendo normativos dentro de su alcance hasta que una decisión aprobada los sustituya. La aceptación de esas decisiones es distinta de la aceptación de Fase 1 y no demuestra que sus comportamientos hayan sido implementados.

Las descripciones de objetivos alcanzados, rutas de código, migraciones, conteos de pruebas y comandos de ejecución que aparecían en versiones anteriores de este registro quedan retiradas como evidencia vigente. No deben utilizarse para afirmar que una subfase está implementada, validada o aceptada. El estado común y los criterios de aceptación están en [ROADMAP.md](../ROADMAP.md) y [docs/requerimiento.md, §§22–23](requerimiento.md).

## Alcance requerido

La Fase 1 implementará, según la secuencia del roadmap y en los owners correspondientes:

- La base técnica y composición legítima del workspace Cargo al incorporar código funcional.
- Persistencia PostgreSQL compartida, ownership de schemas y migraciones, credenciales de mínimo privilegio y aislamiento tenant-scoped.
- Identidad, autenticación, autorización, tenancy y contexto de producto.
- El flujo durable CORE→PLATFORM para el alta operativa del tenant, con Outbox/Inbox cuando corresponda.
- Suscripciones SaaS y evaluación de entitlements conforme a los límites de CORE y PLATFORM.
- Reservas y disponibilidad común bajo el ownership establecido.
- APIs/BFF, observabilidad, privacidad, seguridad y pruebas de aceptación aplicables a cada recorrido introducido.

Este resumen no sustituye los criterios normativos del [requerimiento](requerimiento.md), el [contrato arquitectónico](../ARCHITECTURE_RULES.md), los ADR ni las reglas especializadas.

## Subfase 1.1 — matriz previa de integraciones y aceptación

**Alcance aprobado por el propietario el 2026-11-18:** fundamento técnico de CORE y PostgreSQL; no introducir rutas de producto, alta operativa de tenants, cambios de estado de negocio ni eventos artificiales. Esta matriz antecede a cualquier código. Se conserva el único workspace Cargo en la raíz y no se crean paquetes/targets vacíos. Los controles que se vuelven aplicables por un runtime real —errores tipados, logs seguros, métricas/health pertinentes y manejo no bloqueante de telemetría— sí se implementan y prueban en el mismo corte.

| Integración/capacidad | Aplicabilidad y diseño de 1.1 | Aceptación y escenarios requeridos |
|---|---|---|
| Workspace Cargo y composición CORE | Aplicable. Un solo `Cargo.toml` raíz; incorporar únicamente composición y targets funcionales reales de CORE. El workspace no fusiona despliegues ni autoriza crates placeholder. La selección de forma virtual o paquete raíz se resuelve conforme a ADR-0003 al definir el primer target real. | `cargo metadata` debe mostrar el workspace y miembros reales. Formato, compilación, lint y tests se ejecutan localmente y en CI con los comandos efectivos del repositorio. |
| PostgreSQL y migraciones | Aplicable. Una sola base compartida; schema/migraciones propiedad de CORE, credenciales de runtime separadas de migración, grants mínimos y un pool acotado por proceso con límites/tiempos justificados por la configuración disponible. El pool no concede permisos cruzados. | Con PostgreSQL real y entorno aislado: conexión/reconexión, timeout de adquisición, límites, rollback, permisos y ausencia de acceso a schemas ajenos. Nunca ejecutar migraciones destructivas en staging sin confirmar identidad, destino, autorización y plan de reversión. |
| Contexto tenant y políticas RLS | El mecanismo técnico de contexto transaccional es parte de la infraestructura 1.1. No hay tablas ni filas tenant-scoped en esta subfase y no se crean tablas de ejemplo para simular una aceptación de producto. Las políticas RLS concretas y su autorización se integran con las primeras tablas tenant-scoped de CORE en 1.2. Esta secuencia no elimina el requisito de ADR-0001. | Con PostgreSQL real, 1.1 prueba que el contexto es local a la transacción y no persiste al reutilizar conexión. En 1.2 se prueba además contexto ausente/ajeno, denegación de lectura y escritura, rollback/cancelación y autorización. 1.1 no puede declarar RLS de datos tenant validado. |
| Frontend → HTTP/REST/OpenAPI → BFF | No aplica: 1.1 no introduce frontend ni recorrido de producto. Un health/readiness operativo, si el proceso real lo requiere, no es una API de producto ni reemplaza el recorrido autorizado. | N/A limitado a 1.1; al introducir un flujo de producto deberá completarse y probarse la cadena real en su subfase propietaria. |
| BFF → módulo/backend por gRPC | No aplica: no hay BFF ni llamada síncrona entre owners en 1.1. No se crea un método gRPC sin caso de uso. | N/A limitado a 1.1; cada RPC futuro requiere contrato, autenticación/autorización, límites, correlación y pruebas reales. |
| Redis | No aplica: 1.1 no introduce una necesidad funcional de caché. Redis sigue reservado a caché con TTL, nunca como fuente de verdad. | N/A limitado a 1.1; si una subfase futura introduce caché, debe definir claves/scope, TTL, invalidación y recuperación antes de implementarla. |
| NATS JetStream y Outbox/Inbox | No aplica en 1.1: no se crea un hecho de negocio que requiera entrega durable. No se emiten eventos artificiales. El primer flujo durable ya ratificado es el alta operativa CORE→PLATFORM de 1.3; 1.2 no habilitará esa alta antes de que exista el flujo completo. Si se introduce antes otro caso real durable, el mismo corte debe incluir Outbox/Inbox completo. | El flujo de 1.3 requiere PostgreSQL y JetStream reales: atomicidad, confirmación, duplicados, ACK tras commit, recuperación/replay y redacción. 1.1 no declara mensajería integrada o validada. |
| NATS Core y realtime | No aplica: 1.1 no persiste cambios de producto visibles para usuarios u operadores ni ofrece audiencias conectadas. | N/A limitado a 1.1; cada cambio de estado persistido visible se evaluará en la subfase que lo introduzca y completará Outbox→JetStream→gateway/NATS Core→BFF/WebSocket y recuperación autorizada si corresponde. |
| SEO y metadata pública | No aplica a 1.1: no se introduce una ruta o contenido web público, conforme al alcance infra-only confirmado por el propietario. | Esta N/A no acredita la capacidad SEO base exigida por las fuentes para Fase 1 ni autoriza publicar rutas sin ella. El conflicto de secuencia permanece abierto: antes de aceptar Fase 1 o introducir una ruta pública se debe armonizar documentalmente el requisito de capacidad base con la secuencia de interfaces, e implementar/pruebar SEO antes o junto con la primera ruta pública. |
| Observabilidad | Aplicable a todo runtime real que se introduzca: diagnóstico tipado, logs locales seguros, correlación, métricas/health pertinentes, redacción y comportamiento no bloqueante si falla un destino. No se configuran destinos ni credenciales externos no verificados. | Probar configuración inválida, PostgreSQL indisponible/timeout y campos/redacción de los eventos operativos. No afirmar recepción de Sentry/exporter o SLO activo sin evidencia. |
| Secretos, privacidad y seguridad | Aplican mínimo privilegio y no exposición de credenciales. 1.1 no introduce datos personales ni trata datos de tenants. El diseño de Vault existe, pero no se accede ni modifica el Vault compartido; el operador debe verificar el entorno/identidad de staging antes de conectar un runtime. | Inspeccionar que secretos no estén en archivos, logs, trazas o respuestas. Las pruebas no leen datos productivos. El rol de aplicación no puede asumir el rol dueño/migración ni acceder a schemas ajenos. |
| Staging y dependencias reales | Staging fue seleccionado como destino de integración, pero su provisión, configuración y disponibilidad no están verificadas. No hay `backend/.env`, `compose.yaml` ni PostgreSQL local disponible; no se ha verificado acceso autorizado a PostgreSQL/Vault/NATS de staging. | No ejecutar migraciones, reinicios destructivos ni pruebas contra staging hasta verificar autorización, identidad, endpoints y alcance. Mientras falte PostgreSQL real, las pruebas de integración y la aceptación de la persistencia quedan bloqueadas; no se sustituyen con mocks. |

### Criterios de aceptación de 1.1

1. Workspace raíz y composición funcional sin paquetes, targets o tablas placeholder.
2. Pool y credenciales implementados conforme a ADR-0001 y a la guía PostgreSQL, con errores seguros y configuración no secreta validable.
3. Pruebas reales de PostgreSQL para los comportamientos implementados; permisos mínimos, timeout, rollback y recuperación según aplicabilidad.
4. N/A de HTTP/BFF, gRPC, Redis, NATS, Outbox/Inbox, realtime y rutas SEO limitados explícitamente al alcance 1.1; ningún N/A cuenta como capacidad implementada.
5. Pruebas unitarias, compilación, formato, lints y CI aplicables ejecutados; revisión del diff/secrets y PR conforme al gobierno vigente.
6. El contexto transaccional se implementa y valida con PostgreSQL real; la ausencia de tablas tenant-scoped en 1.1 no se presenta como aceptación de RLS. Las políticas y pruebas negativas de datos tenant pertenecen a la subfase que introduce esas filas.
7. El cierre global de Fase 1 permanece pendiente de requisitos que no pertenecen a 1.1, incluidos el flujo durable de 1.3 y la capacidad SEO base que aún requiere armonización documental.

## Aceptación pendiente

La aceptación requiere implementación real por recorrido, pruebas aplicables ejecutadas contra las dependencias reales, validación local y CI según las fuentes vigentes, documentación sincronizada y evidencia trazable conforme a §23 del requerimiento. Una decisión arquitectónica o una descripción en este registro no prueba implementación.

No se ejecutaron compilaciones ni pruebas para este registro.
