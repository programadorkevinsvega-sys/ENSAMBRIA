# ADR-0002: Hecho durable de alta de tenant de CORE a PLATFORM

- **Estado:** flujo base de onboarding ratificado; contexto de producto documentado según ADR-0004; implementación pendiente
- **Fecha:** 2026-11-10
- **Ámbito:** creación de tenant en CORE y recepción de la solicitud de onboarding por PLATFORM
- **Requisito:** flujo durable CORE→PLATFORM para cada alta operativa de tenant, desde el primer corte ejecutable de Fase 1
- **Fuentes normativas:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [reglas de Outbox/Inbox](../rules/ENSAMBRIA_Transactional_Outbox_Inbox.md)

## Contexto y caso de negocio ratificado

CORE es dueño de la identidad global, tenants, membresías y contexto de seguridad. PLATFORM es dueño de la relación comercial con el tenant, sus planes, suscripciones y facturación SaaS. El propietario ratificó el flujo durable para cada alta operativa de tenant: CORE crea la identidad técnica del tenant y PLATFORM registra ese tenant y abre su onboarding comercial pendiente.

El alta operativa inicia en el flujo de autoservicio del operador y su creación pertenece a CORE. No es una creación manual de PLATFORM. La administración global de PLATFORM gestiona la relación comercial resultante, pero no registra manualmente tenants ni las cuentas de sus clientes finales. Las páginas de registro e inicio de sesión de esos clientes son otro recorrido: CORE autentica la identidad y BUSINESS posee su ficha CRM, sin acceso concedido por la identidad por sí sola.

Esta comunicación no transfiere ownership de la identidad ni autoriza a PLATFORM a crear o alterar registros de CORE. PLATFORM necesita un hecho persistido y recuperable, no una llamada de consulta: si CORE confirma la creación y PLATFORM no está disponible, perder la notificación dejaría sin registrar la relación comercial pendiente. Una llamada síncrona después del commit puede fallar entre la persistencia de CORE y el envío, mientras que emitir antes del commit puede anunciar un tenant que finalmente no se creó. La Outbox guarda el hecho en la misma transacción local que la mutación de CORE; un relay lo publica fuera de la transacción y PLATFORM registra la recepción idempotentemente mediante Inbox.

El flujo aprobado cubre la creación de un tenant operativo, conforme al requerimiento. Este ADR no introduce una taxonomía, excepciones ni transiciones de tenant. No se infiere finalidad a partir del ID, correo, dominio u otros datos del tenant. Cualquier otro propósito o transición queda fuera del alcance de esta decisión y requiere un requisito o una decisión de producto explícita antes de diseñarse.

El efecto inicial en PLATFORM es registrar el tenant referido por CORE y abrir su onboarding en estado `onboarding_pending`. No asigna automáticamente plan, precio, periodo de prueba, factura, cobro, entitlement ni activación comercial.

La referencia `product_id` del evento es la extensión técnica requerida por el diseño multiproducto de [ADR-0004](ADR-0004-plataforma-multiproducto.md). La ratificación del 10 de noviembre cubre el flujo durable y su estado inicial, no una política comercial ni la asignación automática de producto, plan u oferta.

**Ratificación del propietario:** el 10 de noviembre de 2026, el propietario confirmó que cada alta operativa de tenant en CORE debe iniciar este flujo durable hacia PLATFORM. Esta constancia registra su decisión en la conversación; no es una review de GitHub ni una aprobación legal.

## Flujo y responsabilidades acordados para el diseño

### CORE — owner del hecho

1. Recibe una solicitud de autoservicio autorizada de creación de tenant operativo y valida actor y permisos conforme al contrato de identidad; la solicitud entra por el BFF autorizado, nunca por acceso directo del frontend a CORE.
2. Valida el producto seleccionado contra la configuración vigente de PLATFORM mediante contrato; la selección o el dominio recibido del cliente no son por sí solos autorización.
3. Persiste el tenant, su asociación técnica al producto y un registro Outbox `core.tenant.created.v1` en una única transacción local.
4. El evento identifica tenant y producto mediante sus UUIDv7 canónicos y contiene `event_id`, versión del contrato, fecha del hecho, versión del agregado y contexto de correlación necesarios. El payload no incluye credenciales, datos de contacto ni decisiones comerciales.
5. El relay lee y publica el evento fuera de la transacción PostgreSQL con confirmación de JetStream. Una caída del relay no revierte la creación del tenant; deja el evento recuperable.
6. CORE conserva la autoridad para corregir/invalidar el tenant y su asociación técnica. Una eventual cancelación o corrección que PLATFORM necesite conocer requiere otro hecho versionado, definido y aprobado antes de implementarse.

### PLATFORM — consumidor del hecho

1. Consume `core.tenant.created.v1` de JetStream con consumer durable y entrega at-least-once.
2. En una transacción local, registra el `event_id` en Inbox y crea o enlaza idempotentemente la referencia comercial del `tenant_id` y `product_id`, en estado inicial `onboarding_pending`.
3. La Inbox evita duplicar el efecto ante replay o redelivery. El ACK de JetStream se envía solo después del commit local.
4. PLATFORM no crea ni modifica el tenant de CORE, no infiere autorización de los datos del evento y no activa capacidades comerciales por la sola recepción.
5. Las decisiones de elegibilidad, plan, precio, trial, contrato, suscripción, facturación, entitlement y activación quedan fuera de este flujo de alta. El ciclo contractual completo de suscripción se implementa en una operación separada y explícita de Fase 1.4 conforme a [ADR-0007](ADR-0007-separacion-contextos-facturacion-saas.md), una vez aprobadas sus reglas comerciales; este evento no crea, activa ni cambia una suscripción.
6. La consola de PLATFORM no ofrece un alta manual equivalente que eluda el flujo de autoservicio en CORE.

El hecho conceptual y su versión `core.tenant.created.v1` quedan aceptados para este flujo. Subject físico, stream, retención, particionado y SLA se fijarán en el contrato de implementación según las reglas de eventos aprobadas; esos detalles no reabren la ratificación del onboarding.

## Estados y recuperación

No se propone una máquina de estados comercial de tenant. Se separan los estados técnicos de entrega del mensaje de los estados que Product/Legal/Finanzas deben definir.

| Frontera | Estados técnicos propuestos | Recuperación |
|---|---|---|
| Outbox de CORE | `pending` → `leased` → `published`; error transitorio → `retry_wait`; error que excede política → `quarantined` | Relay recupera leases vencidos y reintenta con backoff acotado; reenvío es seguro por `event_id`; cuarentena genera diagnóstico/alerta y requiere resolución explícita. |
| JetStream | Persistido/pendiente de ACK, entregado, redeliverable, ACK | Consumer durable recupera pendientes tras reinicio; no se confirma antes del commit de Inbox/efecto. |
| Inbox de PLATFORM | `received`/procesamiento transaccional → `applied`; error recuperable → retry; error no recuperable → `quarantined` | Reentrega con el mismo `event_id` no repite el efecto; errores de contrato/esquema se registran de forma segura para reparación y replay controlado. |
| Onboarding comercial | Al crear la relación en PLATFORM: `onboarding_pending`; el ciclo contractual posterior pertenece a `saas-subscriptions` y se implementa por operaciones explícitas en Fase 1.4, sujeto a reglas de producto/comerciales aprobadas. | No se infiere aceptación contractual, suscripción, plan, cobro ni activación del evento. Los casos huérfanos o inconsistentes requieren reconciliación operativa autorizada. |

Un fallo de JetStream no elimina el Outbox de CORE. Un fallo de PLATFORM no bloquea la escritura ya confirmada por CORE. Un fallo al aplicar el mensaje revierte tanto Inbox como efecto de dominio de PLATFORM; solo después de persistencia exitosa se ACKea. Los reintentos no deben crear estados comerciales artificiales ni saltarse autorización. El reprocesamiento manual registra actor, motivo, evento y resultado.

## Pruebas y criterios de aceptación requeridos

Antes de aceptar la implementación del flujo:

1. PostgreSQL y JetStream reales: creación de tenant y Outbox atómicos; rollback de ambos si la transacción falla; Outbox permanece pendiente si JetStream está caído.
2. Reinicio del relay/consumer: recuperación de la Outbox y del consumer durable con mensajes pendientes, sin pérdida del hecho.
3. Entrega duplicada, concurrencia de redelivery y replay: una sola Inbox/efecto por `event_id`, sin duplicar onboarding.
4. PLATFORM caído o con timeout: CORE conserva su resultado; el hecho se publica y procesa al recuperarse.
5. Error transitorio y error no recuperable: retry acotado, clasificación observable, cuarentena/replay controlado; no ACK antes del commit.
6. Evento inválido, versión desconocida, tenant inexistente/cancelado o scope no permitido: rechazo seguro, diagnóstico correlacionado y ningún efecto parcial o activación comercial.
7. Mensaje del tenant A no genera escrituras para tenant B; Inbox y permisos del consumer se prueban con roles reales.
8. Prueba contractual del payload, compatibilidad de versión y redacción de identificadores/datos no necesarios en logs.
9. Cada alta operativa confirmada genera un único efecto idempotente en PLATFORM: una referencia al tenant y al producto asociado con estado inicial `onboarding_pending`.

Estas pruebas no se ejecutaron: aún no hay flujo, runtime, schemas, JetStream ni tests de aceptación.

## Alternativas consideradas

1. CORE llama a PLATFORM síncronamente tras crear el tenant: no resuelve la ventana de fallo entre commit y llamada ni la recuperación durable.
2. PLATFORM consulta directamente tablas de CORE: viola ownership y acceso entre dominios.
3. Evento sin Outbox: puede perderse entre commit de CORE y publicación.
4. Outbox/Inbox CORE→PLATFORM: flujo ratificado para altas operativas porque preserva el hecho confirmado y permite recuperación si PLATFORM no está disponible.
5. Crear el tenant de CORE y el onboarding comercial como una transacción distribuida: rechazado; los owners tienen bases/limites de servicio distintos y no deben compartir transacción.

## Consecuencias y límites

- Introduciría un contrato de evento versionado, Outbox CORE, stream durable y consumer/Inbox PLATFORM, además de observabilidad, recuperación y reconciliación.
- El procesamiento es at-least-once; idempotencia por `event_id` es obligatoria. No se promete exactly-once.
- La disponibilidad comercial puede ser eventualmente consistente respecto del alta de identidad; las pantallas y APIs deben distinguir “tenant creado” de “onboarding comercial revisado/activado”.
- La publicación/consumo produce trazabilidad durable, pero no resuelve reglas comerciales ni una operación fuera de línea indefinida.
- El flujo se implementa en el primer corte ejecutable de Fase 1 que crea tenants operativos. No se debe retrasar ni reemplazar por un evento artificial. El alcance no determina cómo tratar otros propósitos de creación de tenant.

## Decisiones y límites restantes

1. Definir los pasos, owners humanos y estados posteriores de onboarding sin inferir políticas comerciales.
2. Definir el contrato técnico de validación del producto activo y su versión, y reflejar en la creación de CORE el contexto confiable de producto sin aceptar Host/header/ID del cliente como autorización.
3. Ratificar campos opcionales adicionales solo si PLATFORM demuestra que los necesita; el evento conserva el `product_id` canónico requerido para relacionar el onboarding, sin transportar PII comercial.

Estos detalles no reabren la decisión de que el flujo durable CORE→PLATFORM existe para cada alta operativa. La ratificación del propietario quedó registrada arriba; no representa aprobación de planes, precios, periodos de prueba, facturación, cobros o entitlements.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Guía de Outbox/Inbox](../rules/ENSAMBRIA_Transactional_Outbox_Inbox.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
