# ENSAMBRIA — Regla de implementación: Transactional Outbox e integración fiable entre backends

**Estado:** Regla arquitectónica obligatoria
**Aplicación:** CORE, BUSINESS, PLATFORM, PUBLIC y workers
**Stack previsto:** Rust, Tokio, SQLx y PostgreSQL. ENSAMBRIA define NATS JetStream para eventos durables; antes de implementar, verifica su configuración real en el repositorio y entorno autorizado. Si falta o no está disponible, registra el bloqueo y no sustituyas la integración por mocks ni por un broker no aprobado.
**Objetivo:** Evitar la pérdida silenciosa de eventos cuando una operación de negocio debe persistirse en PostgreSQL y notificarse a otros procesos, módulos o backends.

---

# 1. Objetivo y regla fundamental

Cuando una operación de negocio modifica datos persistentes y debe generar un evento para otros módulos, backends o consumidores asíncronos, ENSAMBRIA debe utilizar obligatoriamente el patrón **Transactional Outbox**, salvo que exista una decisión arquitectónica documentada que justifique otro mecanismo con garantías equivalentes.

La escritura del estado de negocio y la inserción del evento en la tabla Outbox deben ocurrir dentro de **la misma transacción PostgreSQL y en la base de datos propiedad del backend que realiza la operación**.

Está prohibido implementar el flujo crítico de estas dos maneras:

**Alternativa incorrecta 1:**

1. Guardar los datos de negocio en PostgreSQL.
2. Confirmar la transacción.
3. Publicar directamente al broker como única forma de registrar el evento.

Problema: si el proceso se cae después del commit y antes de publicar, los datos quedan guardados, pero los demás backends nunca reciben el evento.

**Alternativa incorrecta 2:**

1. Publicar el evento al broker.
2. Esperar confirmación.
3. Guardar los datos en PostgreSQL.

Problema: si el broker acepta el mensaje y PostgreSQL falla, se publica un evento de una operación que nunca quedó confirmada.

La regla obligatoria de ENSAMBRIA es:

```text
             OPERACIÓN DE NEGOCIO
                      |
                      v
              TRANSACCIÓN SQL
                      |
             +--------+--------+
             |                 |
             v                 v
       Datos de negocio    Evento Outbox
             |                 |
             +--------+--------+
                      |
                      v
                   COMMIT
                      |
                      v
              Worker Outbox
                      |
                      v
                Message Broker
                      |
             +--------+--------+
             |        |        |
             v        v        v
          CORE    BUSINESS   PUBLIC
             |
             v
       Consumidores
       idempotentes
```

Si la transacción falla, ni el cambio de negocio ni el evento quedan confirmados.

Si la transacción confirma, el evento permanece almacenado y puede publicarse posteriormente, aunque el proceso original se detenga, reinicie o pierda conexión con el broker.

# 2. Alcance en los cuatro backends de ENSAMBRIA

Cada backend es responsable de registrar y publicar los eventos que nacen de sus propias operaciones y de los datos que posee.

| Backend | Ejemplos de eventos que puede originar |
|---|---|
| CORE | `IdentityCreated`, `TenantCreated`, `BranchRegistered`, `BookingConfirmed`, `PaymentAuthorized`, `PaymentSettled`, `EntitlementChanged` |
| BUSINESS | `ServiceUpdated`, `EmployeeAssigned`, `CustomerRegistered`, `MembershipPurchased`, `MembershipConsumed`, `OrderCreated`, `InventoryAdjusted`, `FranchiseBranchChanged` |
| PLATFORM | `SaasSubscriptionActivated`, `SaasInvoiceIssued`, `PlanChanged`, `TenantCommercialStatusChanged`, `CommercialPolicyPublished` |
| PUBLIC | `PublicProfilePublished`, `ReviewPublished`, `ListingVisibilityChanged`, `MarketplaceAttributionRecorded` |

Los nombres son ejemplos, no una autorización para inventar eventos o contratos sin revisar el dominio y sus responsabilidades.

## 2.1 Ownership obligatorio

- El backend propietario del agregado o dato escribe el cambio y su Outbox.
- Un backend no inserta eventos directamente en la tabla Outbox de otro backend.
- Un backend no modifica directamente las tablas de negocio de otro backend.
- La comunicación entre backends se realiza mediante contratos versionados y canales autorizados.
- Un evento comunica un hecho que ya ocurrió; no debe utilizarse como una orden ambigua para modificar datos ajenos.
- Los consumidores actualizan sus propios modelos y datos mediante sus casos de uso y sus propias transacciones.
- Cada backend mantiene autonomía sobre sus datos y procesos.

La infraestructura puede comenzar con un mismo clúster PostgreSQL físico, pero el ownership lógico, las credenciales, permisos, migraciones y responsabilidades deben permanecer separados.

## 2.2 Los verticales también deben respetar esta regla

Los módulos especializados de barberías, peluquerías, salones de belleza, centros de uñas, spas y estética no deben crear mecanismos de mensajería independientes que eviten las reglas de su backend propietario.

Por ejemplo:

- Los módulos verticales de BUSINESS utilizan la Outbox de BUSINESS para los eventos originados en sus operaciones.
- Los eventos de reservas comunes utilizan la Outbox del backend que sea propietario del agregado de reservas, conforme al diseño definitivo del dominio.
- Los eventos de pagos utilizan la Outbox del backend propietario de las operaciones financieras correspondientes.

No crear una Outbox diferente para cada vertical si pertenece al mismo backend y a la misma infraestructura de persistencia.

# 3. Transacción de negocio + Outbox

Ejemplo conceptual: creación de un pedido propiedad de BUSINESS.

```sql
BEGIN;

INSERT INTO orders (
    id,
    tenant_id,
    status,
    created_at
)
VALUES (
    $1,
    $2,
    'created',
    now()
);

INSERT INTO outbox_events (
    event_id,
    aggregate_type,
    aggregate_id,
    event_type,
    schema_version,
    tenant_id,
    payload,
    occurred_at
)
VALUES (
    $3,
    'Order',
    $1,
    'OrderCreated',
    1,
    $2,
    $4,
    now()
);

COMMIT;
```

El ejemplo es ilustrativo. El esquema real debe ajustarse al modelo de datos, los identificadores, las políticas RLS y los contratos del backend.

La operación de negocio y la inserción Outbox deben compartir la misma conexión y transacción SQLx. No abrir una segunda conexión para insertar el evento.

## 3.1 Reglas transaccionales obligatorias

1. La escritura de negocio y la inserción Outbox pertenecen a la misma transacción.
2. El commit confirma ambas escrituras o ninguna.
3. No publicar al broker dentro de la transacción de negocio.
4. No mantener la transacción abierta mientras se comunica con NATS u otro broker.
5. Si el evento no puede construirse o validarse, la operación debe fallar antes del commit cuando ese evento sea obligatorio para la consistencia del dominio.
6. Si una operación no requiere integración asíncrona, no crear eventos artificiales.
7. La generación de identificadores y correlation IDs debe ser estable durante el flujo y persistirse cuando corresponda.
8. La inserción Outbox debe utilizar los mismos límites de atomicidad que la operación que representa.

# 4. Modelo de datos Outbox

Cada backend que utilice Outbox debe tener una tabla de propiedad propia, con migraciones controladas.

Modelo conceptual:

```sql
CREATE TABLE outbox_events (
    event_id UUID PRIMARY KEY,
    aggregate_type TEXT NOT NULL,
    aggregate_id UUID NOT NULL,
    aggregate_version BIGINT,
    event_type TEXT NOT NULL,
    schema_version INTEGER NOT NULL,
    tenant_id UUID,
    payload JSONB NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    occurred_at TIMESTAMPTZ NOT NULL,
    available_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    published_at TIMESTAMPTZ,
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error_code TEXT,
    locked_by TEXT,
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX outbox_events_pending_idx
    ON outbox_events (available_at, occurred_at)
    WHERE published_at IS NULL;
```

Este DDL es una base conceptual. No debe ejecutarse como migración definitiva sin revisar tipos, constraints, índices, particionamiento, retención, RLS y ownership reales.

## 4.1 Propósito de los campos

| Campo | Propósito |
|---|---|
| `event_id` | Identificador global y estable del evento |
| `aggregate_type` | Tipo de agregado que originó el hecho |
| `aggregate_id` | Identificador del agregado |
| `aggregate_version` | Versión o secuencia del agregado, cuando aplique |
| `event_type` | Nombre estable del evento |
| `schema_version` | Versión del contrato del payload |
| `tenant_id` | Contexto tenant cuando corresponda |
| `payload` | Datos mínimos necesarios para los consumidores |
| `metadata` | Correlation ID, causation ID, productor y metadatos permitidos |
| `occurred_at` | Momento del hecho de negocio |
| `available_at` | Momento a partir del cual puede intentarse publicar |
| `published_at` | Momento en que se confirmó la publicación según el broker |
| `attempts` | Número de intentos |
| `last_error_code` | Último error clasificado, sin secretos ni payload sensible |
| `locked_by` | Identificador del worker que reclamó el evento |
| `locked_until` | Vencimiento del lease del worker |
| `created_at` | Momento de creación del registro Outbox |

No incluir contraseñas, tokens, secretos, datos de tarjetas, credenciales ni información personal innecesaria.

Aplicar minimización de datos y clasificación de la información.

## 4.2 Estados de publicación

El estado puede derivarse de los campos o representarse explícitamente. Debe distinguir al menos:

- Pendiente.
- Reclamado o en proceso.
- Publicado.
- Reintento programado.
- Fallo permanente o cuarentena, cuando la política lo defina.

Nunca marcar un evento como publicado antes de obtener confirmación del broker según la garantía configurada.

# 5. Worker publicador de eventos

Cada backend que tenga Outbox debe contar con un worker publicador.

Puede ejecutarse como proceso independiente o como componente desplegable separado. La decisión debe responder a las necesidades reales de operación, aislamiento y escalabilidad.

El worker es responsable de:

1. Buscar eventos pendientes cuyo `available_at` haya llegado.
2. Reclamar un lote de forma concurrente y segura.
3. Evitar que varios workers procesen simultáneamente el mismo claim válido.
4. Publicar al broker utilizando el `event_id` como identificador estable.
5. Esperar confirmación de publicación según las garantías del broker.
6. Marcar el evento como publicado únicamente después de recibir la confirmación.
7. Si la publicación falla, incrementar intentos y programar un nuevo intento.
8. Recuperar eventos cuyos leases hayan expirado.
9. Registrar métricas y errores clasificados.
10. Respetar límites de concurrencia, conexiones y carga de PostgreSQL.

## 5.1 Claim concurrente de eventos

En PostgreSQL se puede utilizar `FOR UPDATE SKIP LOCKED` dentro de una transacción corta para reclamar lotes sin que varios workers seleccionen las mismas filas.

Patrón conceptual:

```sql
BEGIN;

SELECT event_id
FROM outbox_events
WHERE published_at IS NULL
  AND available_at <= now()
  AND (
      locked_until IS NULL
      OR locked_until < now()
  )
ORDER BY occurred_at, event_id
FOR UPDATE SKIP LOCKED
LIMIT $1;

-- Actualizar lease/claim de las filas seleccionadas.

COMMIT;
```

La implementación real debe reclamar y actualizar las filas de forma atómica.

No mantener los locks de PostgreSQL durante la llamada al broker.

Utilizar leases con vencimiento para permitir la recuperación después de una caída del worker.

## 5.2 Flujo de publicación

1. Reclamar eventos y persistir el lease.
2. Liberar la transacción PostgreSQL.
3. Publicar cada evento o lote al broker.
4. Esperar confirmación durable conforme a la configuración seleccionada.
5. Actualizar `published_at` y liberar el lease.
6. Ante error o timeout, programar un nuevo intento.

**Ventana inevitable de duplicación:**

El broker puede aceptar el mensaje y el worker puede caerse antes de marcar el registro como publicado en PostgreSQL.

Cuando el worker se recupere, el evento puede volver a publicarse.

Por tanto, el patrón ofrece **entrega al menos una vez (at-least-once)**, no entrega exactamente una vez de extremo a extremo.

La duplicación es esperable y debe resolverse mediante idempotencia en los consumidores.

# 6. Reintentos y gestión de fallos

Reglas obligatorias:

- Reintentar errores transitorios, como indisponibilidad del broker, timeout y desconexión.
- Utilizar backoff exponencial con jitter y límites configurables.
- No realizar loops de reintento inmediato.
- Clasificar errores transitorios y permanentes.
- No descartar silenciosamente eventos.
- Definir política de cuarentena y Dead Letter Queue (DLQ), cuando corresponda.
- Conservar evidencia de los fallos.
- Permitir reintento y replay autorizado.
- No borrar un evento pendiente únicamente por superar un número de intentos.
- Alertar cuando un evento permanece pendiente demasiado tiempo o acumula fallos.
- No registrar payloads sensibles en los logs.

La política de máximo de intentos, retención, cuarentena y replay debe documentarse según el entorno y la criticidad del dominio.

Un límite de intentos no debe convertirse en pérdida definitiva de un evento financiero o crítico.

# 7. Consumidores idempotentes: patrón Inbox

El Outbox evita perder el evento entre la base de datos de origen y el broker.

Sin embargo, no impide que el broker entregue el mismo mensaje varias veces.

Todo consumidor que realice efectos persistentes debe implementar idempotencia.

Para consumidores importantes se recomienda una tabla Inbox propiedad del backend consumidor.

Modelo conceptual:

```sql
CREATE TABLE inbox_messages (
    consumer_name TEXT NOT NULL,
    event_id UUID NOT NULL,
    event_type TEXT NOT NULL,
    schema_version INTEGER NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at TIMESTAMPTZ,
    processing_status TEXT NOT NULL,
    PRIMARY KEY (consumer_name, event_id)
);
```

## 7.1 Flujo obligatorio del consumidor

El procesamiento debe realizarse en una única transacción local:

1. Intentar registrar el `event_id` en Inbox.
2. Si ya existe como procesado, no repetir el efecto.
3. Si es nuevo, validar el contrato y ejecutar la modificación de datos propiedad del consumidor.
4. Marcar el mensaje como procesado.
5. Confirmar la transacción.
6. Confirmar el mensaje al broker después del commit.

Si la transacción falla, no debe quedar una Inbox marcada como procesada sin que el efecto de negocio haya quedado confirmado.

**La Inbox y la modificación del estado del consumidor deben compartir la misma transacción PostgreSQL.**

Para efectos externos, utilizar otra Outbox o un mecanismo idempotente equivalente.

## 7.2 Clave de idempotencia

- Utilizar el `event_id` estable del evento.
- La unicidad debe incluir el consumidor lógico cuando varios consumidores procesan el mismo evento.
- No utilizar sólo el nombre del evento o el agregado como clave.
- No asumir que el broker elimina todos los duplicados.
- Mantener la retención de Inbox acorde con la ventana máxima de replay y las obligaciones del dominio.

# 8. Contratos de eventos y versionado

Los eventos deben tener contratos versionados, explícitos y documentados.

Cuando ENSAMBRIA utilice Protobuf para comunicación interna, los contratos de eventos deben seguir las convenciones de versionado y compatibilidad definidas para Protobuf.

JSON puede utilizarse donde exista una razón técnica y un contrato validado.

Envelope conceptual:

```json
{
  "event_id": "UUID",
  "event_type": "OrderCreated",
  "schema_version": 1,
  "occurred_at": "2026-11-10T12:00:00Z",
  "producer": "business",
  "tenant_id": "UUID",
  "aggregate": {
    "type": "Order",
    "id": "UUID",
    "version": 1
  },
  "correlation_id": "UUID",
  "causation_id": "UUID",
  "payload": {}
}
```

## 8.1 Reglas del contrato

- `event_id` permanece estable durante todos los reintentos.
- `event_type` identifica un hecho pasado.
- `schema_version` permite evolucionar el contrato.
- `producer` identifica el backend propietario.
- `tenant_id` se incluye únicamente cuando corresponde y no sustituye la autorización.
- `correlation_id` permite seguir una operación entre procesos.
- `causation_id` identifica el comando o evento que originó el hecho, cuando aplique.
- El payload contiene solamente los datos necesarios.
- Los cambios incompatibles requieren una estrategia de versionado y migración.
- No reutilizar un nombre de evento con semántica diferente.
- La validación del contrato debe ejecutarse antes de publicar.
- La compatibilidad de contratos debe verificarse automáticamente en CI.

Los consumidores deben tolerar las evoluciones compatibles conforme a la política de contratos definida.

# 9. Orden de eventos

No asumir que el broker garantiza orden global entre todos los eventos.

Cuando el orden sea relevante:

1. Definir el alcance del orden: agregado, tenant, partición u otro límite.
2. Asignar secuencia o versión al agregado dentro de la transacción de negocio.
3. Publicar con una clave de partición estable, por ejemplo `aggregate_id`, si el broker lo soporta.
4. Evitar que workers publiquen versiones posteriores antes de las anteriores cuando el dominio lo requiera.
5. Hacer que los consumidores detecten duplicados, eventos antiguos y saltos de secuencia.
6. Definir mecanismos para recuperar eventos faltantes o reconstruir proyecciones.

No imponer un orden global si el dominio sólo requiere orden por agregado, porque puede reducir innecesariamente el rendimiento.

# 10. Broker de mensajes y NATS JetStream

NATS JetStream es el mecanismo arquitectónico de ENSAMBRIA para eventos durables y replay. La implementación debe verificar y configurar conforme a las decisiones de infraestructura:

- Streams y subjects por dominio y contrato, conforme a convenciones aprobadas.
- Persistencia y replicación según el nivel de disponibilidad requerido.
- Confirmaciones de publicación.
- Durable consumers para consumidores persistentes.
- Políticas de acknowledgement.
- Políticas de redelivery.
- Límites de retención y almacenamiento.
- Límites de tamaño de mensajes.
- Autenticación y autorización.
- TLS.
- Backpressure y límites de consumidores.
- Monitorización de mensajes pendientes.
- Monitorización de redeliveries y consumer lag.
- Procedimientos de recuperación y replay.

No asumir que una llamada de publicación exitosa significa persistencia durable sin validar la configuración y la confirmación correspondiente de JetStream.

El broker no sustituye Outbox ni Inbox.

Cada uno resuelve un problema distinto:

- **Outbox:** coordina la transacción de negocio con la intención persistente de publicar.
- **Broker:** conserva y distribuye mensajes conforme a su configuración.
- **Inbox:** permite controlar duplicados y efectos persistentes en los consumidores.

# 11. Ejemplo de integración entre los cuatro backends

Ejemplo conceptual de una reserva:

```text
                    CLIENTE
                       |
                       v
                      CORE
                       |
              +--------+--------+
              |                 |
              v                 v
        Guarda reserva     CORE Outbox
              |                 |
              +--------+--------+
                       |
                       v
                    COMMIT
                       |
                       v
               CORE Outbox Worker
                       |
                       v
                  Message Broker
                       |
              +--------+--------+
              |                 |
              v                 v
         BUSINESS Inbox     PUBLIC Inbox
              |                 |
              v                 v
     Actualiza su vista    Actualiza su
     operativa autorizada  proyección pública
```

Este diagrama es ilustrativo. El evento debe emitirse únicamente cuando la reserva alcance el estado de negocio que justifica ese hecho.

## 11.1 Reglas de ownership del ejemplo

- CORE es propietario de la reserva común si así se definió el dominio.
- BUSINESS no modifica directamente las tablas de reservas de CORE.
- BUSINESS consume el contrato y actualiza sus propios datos o proyecciones.
- PUBLIC consume únicamente los datos autorizados para exposición pública.
- Cada consumidor implementa Inbox e idempotencia.
- Si un consumidor necesita provocar otro efecto asíncrono, registra su propio Outbox en su transacción local.
- Un evento no concede permisos: cada backend valida la identidad del productor, el contrato, la autorización del servicio y el contexto de tenant.

# 12. Seguridad y aislamiento multi-tenant

Reglas obligatorias:

- Autenticar productores y consumidores mediante comunicación servicio-a-servicio.
- Autorizar subjects o topics por identidad de servicio.
- No confiar únicamente en el campo `producer` del payload.
- Validar esquema, versión, tamaño y campos obligatorios.
- Validar tenant y ownership en el backend consumidor.
- No aceptar comandos o eventos internos desde clientes públicos sin un flujo autorizado.
- Aplicar TLS y gestión segura de credenciales.
- Obtener secretos desde Vault o el gestor autorizado.
- No incluir tokens, contraseñas, datos de tarjetas ni secretos en los eventos.
- Minimizar datos personales.
- Enviar identificadores y datos estrictamente necesarios cuando sea posible.
- Auditar operaciones administrativas de replay, purga y redrive.
- Proteger tablas Outbox e Inbox con permisos mínimos.
- Aplicar RLS cuando corresponda, teniendo en cuenta procesos internos y acceso cross-tenant autorizado.

El hecho de que un evento contenga un `tenant_id` válido no demuestra que el productor esté autorizado a operar sobre ese tenant.

# 13. Retención, limpieza y recuperación

La Outbox no debe crecer indefinidamente.

Definir una política de retención que considere:

| Tipo de registro | Política |
|---|---|
| Eventos pendientes | Nunca eliminar automáticamente por antigüedad |
| Eventos publicados | Conservar durante una ventana suficiente para auditoría, diagnóstico y replay permitido |
| Eventos en cuarentena | Conservar hasta su resolución conforme a la política |
| Inbox | Conservar durante la ventana máxima de duplicación y replay que se deba cubrir |
| Eventos sujetos a obligaciones legales | Aplicar las políticas de conservación correspondientes |

La limpieza debe ejecutarse mediante workers con lotes, índices, límites de carga y transacciones cortas.

Evaluar particionamiento únicamente cuando el volumen y las mediciones lo justifiquen.

## 13.1 Recuperación de backups

La restauración de backups debe contemplar la coherencia entre PostgreSQL y el broker.

Después de restaurar una copia pueden existir:

- Mensajes que se publiquen nuevamente.
- Eventos que necesiten republicación.
- Mensajes ya procesados que vuelvan a llegar.
- Diferencias entre el estado restaurado de PostgreSQL y el estado actual del broker.

Por eso:

- Los consumidores deben seguir siendo idempotentes.
- Debe existir un procedimiento de reconciliación.
- Los eventos deben conservar identificadores estables.
- El replay debe ser controlado y auditable.
- No se deben borrar indiscriminadamente eventos para intentar sincronizar ambos sistemas.

# 14. Observabilidad y operación

Métricas mínimas por backend:

- Cantidad de eventos pendientes.
- Edad del evento pendiente más antiguo.
- Eventos publicados por unidad de tiempo.
- Fallos de publicación por clasificación.
- Número de reintentos.
- Eventos en cuarentena.
- Duración de publicación.
- Tamaño de lotes.
- Leases vencidos.
- Mensajes pendientes por consumidor.
- Redeliveries.
- Consumer lag.
- Duplicados detectados por Inbox.
- Errores de esquema y versionado.
- Tiempo desde `occurred_at` hasta el procesamiento efectivo.

Los logs deben ser estructurados y permitir rastrear:

- `event_id`.
- `event_type`.
- `producer`.
- `consumer`.
- `tenant_id`, cuando sea seguro.
- `correlation_id`.
- `causation_id`.

No registrar payloads sensibles.

Definir umbrales de alerta con base en los SLO y el volumen medido.

Como mínimo, alertar por:

- Crecimiento sostenido de eventos pendientes.
- Antigüedad excesiva de eventos.
- Errores repetidos de publicación.
- Eventos en cuarentena.
- Consumidores detenidos.
- Aumento anormal de redeliveries.
- Consumer lag fuera de los límites operativos.

## 14.1 Runbooks obligatorios

Debe existir un procedimiento operativo documentado para:

1. Inspeccionar un evento.
2. Diagnosticar un fallo.
3. Reintentar de forma segura.
4. Reprocesar desde cuarentena.
5. Resolver incompatibilidad de contrato.
6. Evitar duplicar efectos.
7. Recuperarse tras una caída del broker o PostgreSQL.
8. Reconciliar eventos después de restaurar un backup.

# 15. Polling Outbox y CDC

## 15.1 Polling Outbox

Es la estrategia inicial recomendada por su simplicidad operativa.

Un worker consulta y reclama eventos pendientes en PostgreSQL.

Debe utilizar:

- Índices adecuados.
- Lotes limitados.
- `FOR UPDATE SKIP LOCKED`.
- Leases recuperables.
- Transacciones cortas.
- Pools de conexiones limitados.
- Backpressure.
- Métricas de operación.

## 15.2 Change Data Capture (CDC)

CDC puede sustituir el polling como mecanismo de captura y publicación cuando el volumen, la latencia o la carga de consultas lo justifiquen.

Sin embargo, CDC no elimina la necesidad de insertar el evento Outbox dentro de la misma transacción de negocio ni elimina la idempotencia del consumidor.

Antes de adoptar CDC, evaluar:

- Complejidad operativa.
- Replication slots y retención WAL.
- Backpressure.
- Recuperación y offsets.
- Compatibilidad con migraciones.
- Observabilidad.
- Coste de infraestructura.
- Capacidad del equipo para operarlo.

No implementar polling y CDC simultáneamente para el mismo flujo sin una estrategia explícita que evite publicaciones duplicadas no controladas.

# 16. Pruebas obligatorias

Todas las pruebas de integración deben utilizar PostgreSQL real y el broker real de prueba o una instancia efímera equivalente.

Los mocks pueden utilizarse en pruebas unitarias de lógica aislada, pero no sustituyen las pruebas de garantías transaccionales y mensajería.

## 16.1 Atomicidad de la transacción

- Si falla la escritura de negocio, no queda evento Outbox.
- Si falla la inserción Outbox, no queda confirmada la operación de negocio.
- Si ocurre rollback, ambas escrituras se revierten.
- Si el proceso cae después del commit, el evento permanece pendiente.
- El evento contiene el identificador y la versión correctos del agregado.
- Los datos del evento corresponden al estado confirmado de negocio.

## 16.2 Worker publicador

- Publicación normal y confirmación.
- Caída antes de publicar.
- Caída después de publicar y antes de marcar `published_at`.
- Timeout del broker.
- Broker no disponible y recuperación.
- Lease vencido y recuperación por otro worker.
- Varios workers concurrentes sin perder eventos.
- Reintentos con backoff.
- Evento en cuarentena y replay autorizado.
- No marcar como publicado sin la confirmación configurada del broker.
- Recuperación después de reiniciar el proceso.

## 16.3 Consumidores

- Mensaje procesado una sola vez a nivel de efecto persistente aunque llegue duplicado.
- Caída antes del commit de Inbox y del efecto.
- Caída después del commit y antes del acknowledgement.
- Redelivery del broker.
- Evento fuera de orden.
- Evento antiguo o de versión incompatible.
- Reintento después de fallo transitorio.
- Validación de tenant y productor.
- Efecto externo coordinado mediante Outbox o idempotencia equivalente.

## 16.4 Contratos y seguridad

- Compatibilidad de esquemas en CI.
- Rechazo de payload inválido o excesivo.
- Autenticación y autorización servicio-a-servicio.
- Aislamiento entre tenants.
- Ausencia de secretos y datos sensibles en logs y payloads.
- Permisos mínimos sobre Outbox e Inbox.
- Replay administrativo auditado.
- Rechazo de eventos de productores no autorizados.

## 16.5 Carga y recuperación

- Publicación de lotes bajo concurrencia.
- Crecimiento sostenido del Outbox.
- Recuperación de backlog.
- Impacto del worker sobre el pool PostgreSQL.
- Caída y restauración del broker.
- Reinicio de procesos durante publicación.
- Restauración de backup y reconciliación de eventos.
- Recuperación ante consumidores lentos.
- Protección de los backends críticos ante saturación de workers.

# 17. Integración en GitHub Actions y desarrollo local

Las mismas pruebas y criterios deben poder ejecutarse localmente y en GitHub Actions.

La CI debe validar, según los scripts reales del repositorio:

- Formato, lint y compilación.
- Pruebas unitarias.
- Pruebas de integración con PostgreSQL.
- Pruebas de integración con el broker.
- Compatibilidad de contratos.
- Pruebas de Inbox e idempotencia.
- Pruebas de concurrencia y fallos.
- Seguridad y análisis de dependencias.
- Migraciones.
- Documentación.

No crear scripts duplicados si ya existen.

No modificar workflows o scripts fuera del alcance autorizado.

Los fallos de CI deben corregirse, no ignorarse ni solucionarse rebajando los criterios.

# 18. Criterios de aceptación

La implementación no se considera completa hasta que se cumplan todos los criterios aplicables:

- [ ] Cada backend registra en su propia Outbox los eventos de sus operaciones.
- [ ] La modificación de negocio y el evento se confirman en una sola transacción local.
- [ ] Ningún flujo crítico depende de publicar directamente al broker después del commit.
- [ ] Existe worker publicador con claim concurrente y leases recuperables.
- [ ] El worker marca publicado sólo después de la confirmación configurada del broker.
- [ ] La solución asume y maneja entrega al menos una vez.
- [ ] Los consumidores con efectos persistentes implementan Inbox e idempotencia.
- [ ] Inbox y efecto local se confirman en una sola transacción.
- [ ] Los contratos están versionados y validados en CI.
- [ ] Los eventos incluyen metadatos de trazabilidad sin filtrar información sensible.
- [ ] Se valida la identidad y autorización del productor y consumidor.
- [ ] Existe política de retries, backoff, cuarentena y replay.
- [ ] Hay límites de concurrencia, lotes y conexiones.
- [ ] Existe retención y limpieza segura.
- [ ] Se prueban caídas en cada ventana crítica del flujo.
- [ ] Se prueban duplicados, orden, aislamiento y recuperación.
- [ ] Hay métricas, alertas y procedimientos operativos.
- [ ] La documentación identifica ownership y consumidores de cada evento.
- [ ] La implementación funciona con los contratos y servicios reales del repositorio.
- [ ] No quedan mocks como sustitución de integraciones productivas requeridas.
- [ ] Las migraciones son reproducibles.
- [ ] Los resultados de CI están completos y verificados.

# 19. Instrucciones obligatorias para el agente de implementación

Antes de modificar código:

1. Inspeccionar la estructura real del repositorio y las cuatro fronteras de backend.
2. Identificar qué operaciones ya emiten eventos y cuáles necesitan integración asíncrona.
3. Verificar la configuración efectiva de NATS JetStream en el repositorio y entorno autorizado; ENSAMBRIA lo define como broker durable. Si no está disponible, registrar bloqueo y no declarar integración probada ni sustituirlo por mocks o tecnología no aprobada.
4. Revisar contratos Protobuf y eventos existentes.
5. Revisar migraciones, Outbox, Inbox y workers ya implementados.
6. Identificar ownership de cada agregado y tabla.
7. Identificar las herramientas y comandos de pruebas existentes.
8. Presentar un plan alineado con la arquitectura existente, sin duplicar mecanismos.

Durante la implementación:

- Implementar el patrón completo, no solamente la tabla Outbox.
- Incluir migraciones, repositorios, transacciones, publicador, confirmaciones, retries, Inbox, idempotencia, métricas, seguridad, pruebas y documentación.
- Mantener los límites entre backends.
- No introducir microservicios adicionales por defecto.
- No introducir CDC sin justificación técnica y operativa.
- No inventar políticas comerciales, eventos o contratos.
- No dejar mocks como sustituto de integraciones reales.
- No dejar TODOs ni declarar funcionalidades terminadas sin implementarlas.
- No debilitar tests, lint, validaciones o seguridad para hacer pasar CI.
- Respetar el ownership de datos.
- Respetar la arquitectura DDD y Clean Architecture.
- Mantener los contratos internos versionados.
- Evitar dependencias circulares entre módulos y backends.

Al finalizar:

1. Ejecutar las pruebas locales relevantes.
2. Ejecutar todos los checks aplicables del workspace.
3. Corregir los fallos introducidos por el cambio.
4. Documentar comandos, configuración, recuperación y operación.
5. Actualizar roadmap y criterios de aceptación únicamente con trabajo realmente completado.
6. Informar archivos modificados, migraciones, pruebas ejecutadas, resultados y limitaciones verificadas.
7. No declarar cerrada la fase si quedan criterios obligatorios pendientes.

# 20. Decisión arquitectónica final

ENSAMBRIA adopta **Transactional Outbox + publicación asíncrona + Inbox/idempotencia en consumidores** como patrón estándar para integrar cambios persistentes con mensajería entre backends.

**Inicio obligatorio:** la implementación comienza en la **Fase 1 de ENSAMBRIA, desde su primer corte ejecutable**, no en una fase posterior ni como refactor transversal. Ese primer corte incluye el patrón completo en al menos un flujo real que requiera integración durable: escritura atómica de la mutación y Outbox, publicación recuperable con confirmación de JetStream, e Inbox/efecto atómicos y ACK posterior al commit para consumidores persistentes aplicables. Fase 0 solo define ownership, contratos, decisiones y pruebas; no implementa runtime. Cada fase posterior incorpora el patrón desde el primer cambio que produzca un hecho durable aplicable. No se exige inventar eventos para casos que no requieren integración; su N/A debe justificarse por caso de uso.

La garantía buscada no es exactamente una entrega única de extremo a extremo.

La garantía práctica es:

1. La operación local y la intención de publicar quedan confirmadas atómicamente.
2. El evento persiste hasta que pueda publicarse o ser gestionado por una política operativa explícita.
3. El publicador puede repetir una publicación sin perder el evento.
4. Los consumidores toleran duplicados y evitan repetir efectos persistentes.
5. Los fallos pueden observarse, diagnosticarse y recuperarse.
6. Los límites de ownership y aislamiento entre los cuatro backends permanecen intactos.

**Regla esencial de ENSAMBRIA: ningún evento crítico debe depender de que el proceso que realizó la operación de negocio permanezca vivo después del commit para poder publicarse.**

La persistencia del evento debe estar garantizada por PostgreSQL; su publicación debe ser responsabilidad de un proceso recuperable; y su consumo debe ser idempotente.
