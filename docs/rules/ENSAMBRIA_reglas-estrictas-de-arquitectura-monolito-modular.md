# Reglas Estrictas de Arquitectura — Cuatro Backends con Monolito Modular (Rust)
## DDD + Clean Architecture + Hexagonal + SOLID

> **Propósito de este documento:** este manual complementa el contrato central [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md) con reglas técnicas de implementación Rust para los cuatro backends modulares. No es un segundo contrato de topología ni de ownership de producto. Cada backend conserva internamente arquitectura de monolito modular y estas reglas técnicas aplican al crear o modificar su implementación.
>
> Las reglas técnicas dentro de su ámbito son obligatorias. La topología, responsabilidades, ownership de producto y workspace único se determinan en el contrato central; una descripción repetida aquí es un resumen y no puede modificarlos. Si una petición contradice una regla aplicable, el agente debe señalarlo antes de generar código que la viole.
>
> Los principios técnicos de capas y límites modulares son complementarios; la topología y reparto de responsabilidades específicos de ENSAMBRIA pertenecen a [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md). Los nombres de módulos, entidades, agregados, casos de uso, eventos y recursos deben reflejar el dominio real y respetar los límites del contrato central y las fuentes especializadas propietarias.

---

# 0. Objetivo estratégico (léelo primero)

## 0.0 Documentos normativos obligatorios desde el día uno

Los documentos normativos del repositorio son de cumplimiento obligatorio para ENSAMBRIA. Sus requisitos se aplican desde el diseño inicial y el primer cambio a todos los módulos, capas, workers, APIs, frontends, infraestructura, fases y subfases, dentro de su alcance y responsabilidad. La arquitectura específica, topología, ownership y límites del producto se rigen por [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md); este manual precisa su implementación técnica modular en Rust. La ingeniería de calidad y sus evidencias se rigen por [ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md](ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md); la observabilidad, instrumentación, monitoreo, alertas, SLI/SLO y operación por [ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md](ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md).

No son material de consulta opcional ni una fase posterior: gobiernan planificación, diseño, implementación, pruebas, despliegue y operación desde el día uno. Cada módulo debe cumplir los requisitos aplicables completos; la calidad, seguridad, observabilidad, resiliencia, contratos y pruebas no se posponen hasta una fase transversal posterior. Un requisito solo puede declararse no aplicable con una justificación técnica concreta, verificable y documentada antes de implementar la fase/subfase afectada.

Cuando una regla técnica Rust se solape con el contrato central, este fija la decisión de proyecto y esta guía conserva el detalle de implementación que no la contradiga. Las fuentes especializadas prevalecen en sus respectivas materias. Ante una incompatibilidad real no resuelta por esta delimitación, se detiene únicamente el trabajo afectado, se documenta el conflicto y se resuelve mediante decisión explícita antes de continuar; no se omite ni debilita silenciosamente un requisito.

ENSAMBRIA contiene cuatro backends de dominio desplegables e independientes, miembros del **workspace Cargo único definido por el `Cargo.toml` de la raíz del repositorio**. Compartir workspace, resolución de dependencias y `Cargo.lock` no los convierte en una sola aplicación ni elimina sus límites de runtime, composición, configuración, permisos, datos o despliegue:

1. **CORE (`backend-core`):** identidad/autenticación, tenancy y contexto de seguridad, organizaciones/sucursales centrales, evaluación de entitlements, reservas, procesamiento de pagos, cargos transaccionales aprobados, ledger, liquidaciones y conciliación. Los adapters de proveedores de pago y las credenciales/cuentas necesarias para procesar esos pagos pertenecen al contexto de pagos de CORE, aislados por titularidad y propósito; no forman un quinto backend.
2. **BUSINESS (`backend-business`):** operación de establecimientos, workforce, catálogo, clientes/CRM, membresías, inventario, Commerce/pedidos, marketing comercial, facturación de establecimientos, franquicias/regalías y comisiones de empleados. Es dueño de las reglas y registros comerciales de sus ventas; CORE procesa los pagos y registra su efecto financiero.
3. **PLATFORM (`backend-platform`):** administración global y relación comercial de tenants, planes/suscripciones/facturación SaaS y políticas comerciales de ENSAMBRIA, incluidas las tarifas aprobadas para pagos/Marketplace. Mantiene sus propias obligaciones y configuración SaaS; CORE procesa los pagos y registra su efecto financiero.
4. **PUBLIC (`backend-public`):** directorio, Discovery/Marketplace, páginas y proyecciones públicas autorizadas, reseñas y atribución del canal. No posee operaciones privadas, catálogo operativo, inventario ni pedidos del establecimiento.

La topología, ownership y superficies se rigen por [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md). CORE posee identidad/contexto de seguridad y pagos; PLATFORM administra el catálogo de productos y la relación comercial SaaS; BUSINESS ejecuta la operación y posee Commerce y reglas verticales; PUBLIC publica solo proyecciones autorizadas y atribución. El plan comercial Franquicia pertenece a PLATFORM; las franquicias/regalías operativas pertenecen a BUSINESS. El BFF público y los BFF especializados de productos implementados son aplicaciones independientes; el BFF de administración se integra inicialmente en PLATFORM. CORE no se expone directamente a frontends.

Cada uno de los cuatro backends es un límite de despliegue y contiene un monolito modular. Dentro de cada uno:

- Cada módulo/bounded context conserva ownership de su dominio y de sus datos dentro del backend dueño. Los cuatro backends conservan composition roots, configuración, permisos, runtime y despliegues separados; un backend no importa la implementación interna de otro.
- El bounded context es un límite lógico de ownership, no una unidad Cargo obligatoria. Varios contextos pueden residir en un paquete de backend si mantienen APIs, dependencias, persistencia y pruebas aisladas. Crear un crate separado requiere necesidad verificable y código funcional; no se crean crates placeholder ni un crate por dominio automáticamente.
- **Ningún crate de módulo depende ni importa código interno de otro módulo directamente.** La única comunicación permitida entre módulos es a través de:
  - (a) el contrato público del módulo (`module_public_api`, expuesto técnicamente como servicio **gRPC** vía `tonic`), o
  - (b) eventos de integración publicados en **NATS JetStream** (`async-nats`).
- Los backends solo se comunican cuando un caso de uso lo requiere, mediante contratos autenticados/versionados; nunca mediante imports internos o acceso cruzado SQL:
  - Todo canal gRPC entre backends usa TLS con identidad criptográfica de servicio y mTLS; los certificados y credenciales tienen rotación planificada con periodo de solapamiento. Una IP o un header declarativo no autentican un servicio.
  - Las consultas/comandos síncronos se dirigen al backend dueño del contrato y de la operación. Por ejemplo, BUSINESS o PLATFORM pueden solicitar una operación de pago a CORE mediante el contrato aprobado; ningún frontend ni BFF invoca directamente módulos internos de CORE.
  - El backend dueño verifica autenticidad, vigencia, autorización por método/scope, tenant, propósito, entorno y correspondencia del comando antes de cualquier efecto. El rechazo ante fallo de identidad o autorización es cerrado.
  - Cada request valida versión, campos/tipos, tamaño y rangos; fija deadlines/timeouts y cuotas por caller/método. Reintentos son acotados con backoff+jitter, solo para errores recuperables y con idempotencia durable; no reintentan decisiones de autorización ni operaciones de resultado incierto sin reconciliación.
  - El alta/rotación de credenciales se gestiona por el backend dueño del contexto y la cuenta correspondientes; se protege en tránsito y en reposo, no se envía por NATS. Eventos durables posteriores contienen solo referencias opacas, versión y estado cuando sea necesario.
  - gRPC expresa consultas/comandos request-response que necesitan resultado inmediato, incluso si el módulo dueño persiste su resultado. JetStream/Outbox distribuye hechos persistidos que requieren procesamiento durable, replay o desacoplamiento; no sustituye RPC ni se usa solo por llamar una operación “delicada”. Los eventos nunca llevan secretos, grants completos ni payload crudo de pago.
  - Los webhooks de proveedores se reciben y verifican en el backend propietario de la integración/dominio afectado. CORE valida los callbacks de pagos; PLATFORM y BUSINESS conservan, respectivamente, sus contextos de facturación SaaS y de establecimientos. Los hechos persistidos se publican por Outbox y las transiciones de cada dominio las decide su owner.
  - Rate limits, deadlines, cancelación, límites de concurrencia, claves idempotentes y auditoría protegen el canal. Ambos lados registran resultado y errores tipados con `correlation_id`/`trace_id`, sin secretos, tokens, certificados, firmas, PII innecesaria o contenido sensible.
- Los cuatro backends pueden usar la infraestructura PostgreSQL física compartida definida por el proyecto. Cada módulo es dueño exclusivo de sus schemas/tablas/migraciones coordinadas; no hay JOINs, consultas ni escrituras cruzadas entre dominios. El acceso a PostgreSQL se limita al módulo/backend dueño mediante credenciales y grants mínimos.
- La estrategia de pools PostgreSQL se define desde Fase 0 según [ENSAMBRIA_Connection_Pooling_PostgreSQL.md](ENSAMBRIA_Connection_Pooling_PostgreSQL.md): pool acotado por backend/proceso, identidad y grants propios, presupuesto para réplicas/workers, reserva operativa y timeouts. No crear pools por petición o tenant. El contexto RLS debe quedar limitado a la transacción y no filtrarse por reutilización. Dimensionamiento y PgBouncer se deciden con mediciones y pruebas; los valores de ejemplo no son defaults.
- Si en el futuro un módulo interno se extrae a un proceso independiente, el cambio debe limitarse a:
  - mover la carpeta/crate,
  - desplegar su servicio gRPC en su propio proceso/host/binario,
  - ajustar el `docker-compose`/deployment/configuración.
- **La lógica de dominio y aplicación NO debe tocarse**, porque los contratos gRPC y eventos NATS ya se comportan como remotos desde el día uno.
- Por eso: trata cada carpeta bajo `modules/<nombre>` como un "servicio en potencia" con su propio dominio, aplicación, infraestructura y puertos de entrada/salida.

La frontera de despliegue de ENSAMBRIA está formada por cuatro backends independientes, cada uno como monolito modular. Los módulos internos no se despliegan por separado salvo que una extracción futura sea justificada y aprobada.

## Capas de la arquitectura completa (de afuera hacia adentro)

1. **Frontend** (web/mobile/desktop cuando corresponda):
   - sin lógica de negocio;
   - consume únicamente la superficie HTTP/REST/WebSocket autorizada para su caso de uso;
   - las superficies tenant/business/móvil de cada producto usan su BFF de producto independiente; Discovery usa el BFF público independiente; la consola global usa el BFF de administración integrado en PLATFORM;
   - CORE nunca se expone directamente a un frontend;
   - maneja presentación, estado de UI y validaciones superficiales de entrada.

2. **BFF (REST API Gateway)**:
   - sin lógica de negocio;
   - implementado con un framework HTTP de alto rendimiento (ej. `axum` o `actix-web`);
   - adapta y compone respuestas REST;
   - se comunica con los módulos exclusivamente mediante **gRPC** (`tonic`);
   - ENSAMBRIA mantiene un BFF público independiente, un BFF independiente por producto implementado y el BFF de administración inicialmente integrado en PLATFORM; cada superficie usa solo su BFF autorizado y ninguno accede directamente a stores o expone CORE.

3. **Módulos de cada monolito modular (Crates)**:
   - dueños de las reglas y casos de uso de su bounded context;
   - CORE posee las reglas y transiciones de procesamiento de pagos, el ledger y los adapters de proveedor de pago;
   - BUSINESS posee las reglas comerciales y transiciones de ventas/pedidos y su facturación; PLATFORM posee planes, suscripciones, políticas y facturación SaaS; PUBLIC posee descubrimiento, proyecciones autorizadas y atribución;
   - dueños de sus schemas, tablas y migraciones;
   - dueños de su persistencia;
   - exponen su `module_public_api` como servicio **gRPC** (`tonic`);
   - se comunican entre sí de forma asíncrona mediante **NATS JetStream** (`async-nats`) cuando corresponda.

4. **Infraestructura transversal**:
   - bootstrap;
   - configuración centralizada (`platform::config`);
   - logging estructurado y tracing (`tracing` + `tracing-subscriber` + `tracing-opentelemetry`);
   - métricas;
   - componentes técnicos que realmente sean transversales.

El detalle completo de BFF/REST/Frontend está en la sección 9. Los cuatro backends pertenecen al workspace Cargo raíz, pero no se crean dependencias de implementación entre sus módulos. Solo pueden compartir bibliotecas técnicas neutrales y contratos/tipos aprobados explícitamente; la comunicación de runtime sigue siendo gRPC o eventos autenticados según el caso de uso.

### Regla de nombres

Los ejemplos abstractos de los principios siguientes usan nombres genéricos. La topología ENSAMBRIA definida en la sección 0 fija el ownership de los cuatro backends y los contratos entre ellos.

Ejemplos:

```text
module-a
module-b
module-c
```

En el proyecto real:

```text
modules/<nombre-real-del-contexto>
```

El nombre real debe representar un **bounded context** del dominio, no una capa técnica.

Por ejemplo, no usar:

```text
modules/database
modules/services
modules/controllers
modules/utils
```

si esos nombres no representan bounded contexts reales.

Dentro del workspace, los nombres deben representar bounded contexts reales del backend dueño. Los adapters de proveedores se ubican dentro del contexto propietario de la capacidad; no constituyen backends adicionales ni mezclan credenciales o reglas de dominios distintos.

---

## 0.1 Diagrama canónico de la arquitectura

Este es el diagrama de referencia obligatorio. Cualquier código generado debe ser consistente con esta forma. Los cuatro backends comparten el workspace de build, no el runtime ni el despliegue. Las llamadas/eventos entre backends aparecen solo donde existe una necesidad funcional:

```text
Discovery ──────────── REST/OpenAPI/WebSocket ──► BFF público independiente ──gRPC──► PUBLIC
Tenant / Business / Mobile UI ────────────────► BFF independiente por producto ───► CORE/BUSINESS/PLATFORM/PUBLIC
Admin UI ───────────── REST/OpenAPI/WebSocket ──► BFF PLATFORM integrado ─────────► PLATFORM

CORE ◄──────────── gRPC síncrono si aplica ────────────► BUSINESS / PLATFORM / PUBLIC
CORE / BUSINESS / PLATFORM / PUBLIC ── Outbox ── NATS JetStream ── consumidores autorizados

CORE ── adapters de pagos ──► proveedores de pago
PLATFORM ── facturación SaaS / proveedores propios
BUSINESS ── facturación y proveedores del establecimiento

PostgreSQL puede ser físicamente compartido; cada módulo conserva ownership,
migraciones y permisos propios. Ningún frontend/BFF accede directamente a stores.
```

CORE es el único propietario de pagos/ledger. PLATFORM conserva sus obligaciones SaaS; BUSINESS conserva ventas y facturación del establecimiento; PUBLIC mantiene las proyecciones expresamente autorizadas y la atribución del canal. Los callbacks se enrutan al backend y módulo propietario del contexto que los valida, sin cruzar credenciales, cuentas, eventos ni registros entre ENSAMBRIA y tenants.

---

## 0.2 Responsabilidad de cada componente

| Componente | Responsabilidad única | Tecnología Rust estándar |
|---|---|---|
| **Frontend** | UI/presentación. Sin lógica de negocio. | Framework cliente (Web/Mobile/Desktop) |
| **BFF (REST Gateway)** | Adaptación y composición REST para la superficie autorizada. BFF público y BFF de producto independientes; BFF de administración integrado inicialmente en PLATFORM. Sin lógica de negocio ni acceso directo a stores. | `axum` / `actix-web` + `utoipa` (OpenAPI) |
| **gRPC** | Contratos síncronos entre BFF y módulo dueño y entre backends cuando una consulta/comando requiere respuesta inmediata. CORE no se expone a frontends. | `tonic` + `prost` |
| **Módulo / bounded context** | Dueño exclusivo de su lógica de negocio: dominio, casos de uso y persistencia. Puede residir en un paquete de backend; un crate separado requiere necesidad verificable y código funcional. | Límite lógico de ownership; crate no obligatorio |
| **DB** | Fuente de verdad compartida físicamente; cada módulo es dueño exclusivo de su schema/tablas y migraciones. | `sqlx` (PostgreSQL) / Migraciones versionadas por módulo |
| **Redis** | Exclusivamente cache efímera con TTL; no se usa para fan-out, coordinación ni como autoridad o almacenamiento durable. | `redis` + `deadpool-redis` / `bb8-redis` |
| **NATS Core** | Fan-out realtime efímero entre réplicas; sus señales no tienen replay durable y se reparan mediante cursor/snapshot. | `async-nats` (Core NATS) |
| **NATS JetStream** | Comunicación asíncrona entre módulos — eventos de dominio/integración. | `async-nats` (JetStream context) |
| **Adapters y credenciales de pago** | CORE posee el procesamiento de pagos y las cuentas/credenciales necesarias para ese propósito; aislamiento, rotación, autorización y secretos se rigen por el módulo dueño. No se publican secretos ni se transfieren reglas comerciales. | Módulos/adapters CORE + custodia aprobada + API oficial/sandbox |
| **Facturación SaaS** | PLATFORM posee obligaciones y documentos de ENSAMBRIA; CORE procesa pagos asociados. | Módulos/adapters PLATFORM para su contexto; contratos de pago CORE |
| **Facturación comercial tenant** | BUSINESS posee obligaciones y documentos de los establecimientos; CORE procesa pagos asociados. | Módulos/adapters BUSINESS para su contexto; contratos de pago CORE |
| **Integraciones externas** | Cada backend dueño conserva sus adapters, credenciales, callbacks, auditoría e historial separados por propósito/caso de uso. | Puertos neutrales del dominio + adapters en el backend propietario |
| **Realtime** | Los backends implementan los módulos/canales realtime que requieran sus dominios; cada BFF atiende únicamente audiencias autorizadas y no se convierte en fuente de verdad. | Outbox/JetStream durable + NATS Core efímero + gRPC/WebSocket autorizado |
| **OpenTelemetry** | Tracing y métricas distribuidas de extremo a extremo, en todas las capas. | `tracing` + `opentelemetry` + `opentelemetry-otlp` |
| **k6 & criterion** | Pruebas de rendimiento y carga sobre operaciones críticas y benchmarks internos. | `k6` (externo) + `criterion` (micro-benchmarks) |
| **Chaos Engineering** | Validación activa de resiliencia del sistema ante fallos reales. | Inyección controlada (`toxiproxy-rs`, Chaos scripts) |

Ninguna de estas responsabilidades se invierte ni se mezcla:

- el Frontend nunca decide reglas de negocio;
- el BFF nunca implementa reglas de negocio;
- un módulo nunca delega su lógica de negocio a otra capa;
- una base de datos nunca se convierte en mecanismo de comunicación entre módulos o backends;
- Redis nunca sustituye la fuente de verdad;
- NATS nunca se utiliza como sustituto arbitrario de una llamada síncrona;
- gRPC nunca debe convertirse en un bypass de las reglas de dominio.

## 0.3 Integración obligatoria desde cada fase/subfase

Toda fase y subfase que implemente software debe diseñarse e implementarse de extremo a extremo desde su primer cambio. Antes de escribir código, su documentación debe registrar el recorrido externo (Frontend → HTTP/REST → BFF → gRPC → módulo dueño), la persistencia/migraciones del módulo, el uso requerido de Redis, los eventos NATS JetStream mediante Outbox y las obligaciones aplicables de privacidad/cumplimiento conforme a [ENSAMBRIA_Privacy_Legal_Compliance.md](ENSAMBRIA_Privacy_Legal_Compliance.md) y [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](ENSAMBRIA_Privacy_Data_Lifecycle_Management.md). La implementación y sus pruebas de aceptación pertenecen a esa misma fase/subfase; no se permite diferir estas fronteras ni controles críticos a una refactorización futura. La Fase 0 de auditoría, decisiones y workspace es documental: no requiere runtime ni aceptación funcional y se cierra con los artefactos y comprobaciones estáticas definidos en el roadmap local, incluyendo registro de jurisdicciones/obligaciones, revisión legal y pendientes bloqueantes. La primera fase de implementación aplica íntegramente las fronteras y salvaguardas pertinentes desde el primer corte ejecutable.

En cada subfase se evalúan explícitamente Redis, NATS Core y NATS JetStream. Redis se reserva exclusivamente para caché con TTL. NATS Core distribuye señales realtime efímeras entre réplicas, sin persistencia ni replay; JetStream/Outbox transporta cambios persistidos que requieren entrega durable. Si alguno no aplica a una operación, se registra una justificación técnica concreta y verificable en la documentación de esa fase. No se introducen usos artificiales: se prohíbe omitir su evaluación. Ningún bus sustituye gRPC en operaciones síncronas.

Cuando un dominio use Redis o NATS, la recuperación tras reinicios/interrupciones es parte obligatoria de la implementación: Redis reconecta con backoff para reanudar la caché; los suscriptores NATS Core se reconectan y re-suscriben, aceptando que sus señales no son durables; los consumidores JetStream reconstruyen su consumer durable y recuperan pendientes de forma idempotente. PostgreSQL/Outbox y JetStream conservan la recuperación durable. La fase añade pruebas con dependencias reales para reconexión, fan-out, replay/reentrega y continuidad después del reinicio. Las pruebas destructivas solo se ejecutan contra los servicios de desarrollo identificados explícitamente en `.env`; nunca crean contenedores ni reinician recursos de producción.

Los frontends consumen exclusivamente HTTP/REST/OpenAPI/WebSocket de su BFF autorizado: las superficies tenant/business/móvil usan el BFF independiente del producto; Discovery usa el BFF público independiente; la consola global usa el BFF de administración integrado en PLATFORM. CORE no se expone directamente a frontends. Ningún frontend o BFF accede directamente a PostgreSQL, Redis, NATS o módulos internos. Los BFF se comunican con sus módulos por contratos aprobados; los backends solo invocan otro backend si un caso de uso requiere una respuesta inmediata o distribución durable. Las pruebas de cada fase verifican los recorridos y fronteras reales aplicables de los cuatro backends, incluyendo gRPC, Outbox/JetStream y Redis cuando corresponda.

## 0.4 Seguridad entre backends y credenciales de proveedores

Cada llamada entre backends es una frontera de confianza explícita y solo existe cuando hay necesidad funcional. La autenticación acredita qué servicio llama; la autorización determina qué método y operación puede ejecutar. Se usan certificados de cliente/servidor con mTLS y validación de identidad SAN/issuer conforme a la PKI configurada. Rotación de certificado/CA admite solapamiento, comprobación de expiración y revocación, y no depende de claves permanentes sin rotación. TLS protege confidencialidad e integridad en tránsito; grants firmados y controles de aplicación protegen audiencia, scope y vínculo con el comando cuando el riesgo y contrato lo requieran.

Cada método gRPC tiene política de caller, scope, límites y timeout. Los mensajes validan campos requeridos, versión, enums conocidos, longitud, tamaño total y rangos antes de tocar stores o proveedores. `correlation_id`, `trace_id`, `deadline`, `tenant_id`, `environment` y `idempotency_key` se propagan como contexto tipado; jamás se confía en claims aislados sin verificar autenticidad y correspondencia. Se limitan frecuencia y concurrencia por servicio/operación/tenant, se aplica backpressure y se protegen operaciones no idempotentes de replay. Fallos de certificado, identidad, autorización, integridad o contrato producen rechazo cerrado y evento de diagnóstico redactado.

Los comandos síncronos gRPC sirven para respuestas inmediatas y pueden ejecutar una escritura durable en el bounded context dueño; elegir gRPC no implica saltarse persistencia ni Outbox requeridos por el dominio. NATS JetStream/Outbox sirve para propagar hechos persistidos, trabajos durables, replay y consumidores desacoplados. Las credenciales y otros secretos nunca se publican en eventos, aunque el workflow sea complejo o delicado. Reintentos tienen límite, backoff+jitter, clasificación retryable y protección idempotente; ante timeout con resultado incierto se consulta/reconcilia antes de volver a ejecutar.

Las credenciales de proveedores pertenecen al backend dueño de la integración y al propósito de la cuenta: CORE custodia las necesarias para procesamiento de pagos; PLATFORM y BUSINESS mantienen separadas las de sus propios contextos de facturación y operación. El ingreso inicia en la superficie/BFF autorizada, el backend propietario valida permisos y administra almacenamiento, cifrado, rotación, acceso en memoria y auditoría conforme a la política de secretos del proyecto. Ningún secreto se envía por NATS, se devuelve a un frontend ni se comparte entre ENSAMBRIA y las cuentas tenant. Los adapters de proveedores no almacenan secretos fuera del mecanismo aprobado por su owner.

Las pruebas de aceptación incluyen caller permitido/denegado, certificado inválido/expirado, rotación de certificados y claves de cifrado, límite de tasa, request fuera de tamaño/rango, timeout/desconexión, replay, idempotencia y redacción entre ambas fronteras. Debe probarse recuperación tras desconexión sin enviar duplicados ni perder una configuración confirmada. Logs correlacionados registran evento, servicio, método, operación, clasificación/código, dependencia, duración, resultado y contexto seguro; nunca token, concesión completa, API key, secret, certificado privado, firma completa o payload sensible.

## 0.5 Contrato normalizado de método de pago

El webhook saliente conserva una forma común de evento y usa `payment_method` como unión discriminada y versionada. La `type` normalizada define el esquema de `details`; el adapter convierte el protocolo de su proveedor y descarta campos desconocidos/no permitidos. No se devuelve el payload crudo del proveedor. Wompi es el proveedor inicial previsto para pagos, no exclusivo, conforme a [ADR-0009](../adr/ADR-0009-proveedores-iniciales-y-reventa-factus.md). Solo se exponen métodos que el proveedor seleccionado documente y que ENSAMBRIA haya habilitado:

| `payment_method.type` | `details` normalizado |
|---|---|
| `card` | Si el proveedor seleccionado documenta este método: `brand`, `last4`; titular solo si existe necesidad contractual y autorización expresa. No PAN/CVV. |
| `pse` | Si el proveedor seleccionado documenta este método: `bank_code`, `bank_name`, `payment_reference` si está disponible y permitido. |
| `crypto` | Extensión futura, no habilitada por la selección inicial de Wompi: `asset`, `settlement_currency`, `settlement_amount` decimal string, `network` opcional, `provider_order_id`, `provider_transaction_id` y `network_transaction_id` solo cuando un proveedor seleccionado los entregue. |

`transaction.amount`/`transaction.currency` siguen expresando el valor comercial facturado en unidades menores enteras. Un importe de liquidación cripto vive en `payment_method.details` como string decimal junto con su activo, porque cada activo puede tener precisión distinta. Nunca usar `f32`/`f64` para importes ni asumir que un proveedor entrega campos que su contrato no documenta. `gateway.name` identifica el adapter real (por ejemplo `wompi`) y `receipt_id` referencia el recibo oficial disponible.

Binance Pay no es el proveedor inicial seleccionado. Si se selecciona en el futuro, su adapter debe reflejar el contrato vigente de Binance y no extrapolar el HMAC-SHA256 de los webhooks salientes de ENSAMBRIA. Binance documenta firma API HMAC-SHA512 y órdenes con `currency`, importe decimal `totalFee`, `transactionId` y estados propios; antes de implementarlo, verificar su [regla oficial de firma](https://developers.binance.com/en/docs/products/binance-pay-merchant/api-common), la [consulta oficial de órdenes](https://developers.binance.com/en/docs/products/binance-pay-merchant/api-order-query) y el sandbox disponible. No se presume su selección ni se limita a ese proveedor la posibilidad de añadir otros adapters.

### Sincronización realtime obligatoria en todas las fases

Esta obligación aplica a **todas las fases y subfases de implementación**, no solo a notificaciones, colaboración, trabajos o a una pantalla concreta. Cada fase/subfase que cree o modifique datos persistidos visibles en una aplicación debe incluir en su alcance el evento Outbox del módulo dueño, su envelope/versionado y distribución durable JetStream; NATS Core distribuye señales realtime efímeras entre réplicas cuando corresponda. Se incluyen autorización de audiencia, señal WebSocket, invalidación/refetch de TanStack Query y mecanismo REST de recuperación. Si una operación no produce una transición observable, documenta la justificación en la matriz de la fase; no omitas la evaluación. La Fase 0 documental registra que no introduce mutaciones persistidas ni requiere runtime realtime.

Cada backend propietario implementa el módulo o contrato realtime que necesiten sus dominios, con su schema, permisos, configuración y observabilidad. No se comparte ownership, sesión, cookies, caché ni autorización entre backends. El BFF correspondiente autentica y puentea WebSocket público con los contratos internos autorizados; no conecta directamente a NATS, Redis o PostgreSQL. El gateway realtime valida identidad, sesión, audiencia y permisos al abrir la conexión y durante su vida útil. Los webhooks públicos de proveedores entran por el backend propietario de la integración, nunca por el frontend. El frontend no se conecta a infraestructura ni recibe tokens en URL.

Cada mutación persiste el cambio de su dominio y su Outbox en la misma transacción PostgreSQL local del módulo dueño. Un evento producido por un backend y su efecto posterior en otro pertenecen a transacciones distintas, coordinadas mediante Outbox/JetStream y consumidores idempotentes; no se simula una transacción distribuida. JetStream lleva eventos de integración durables, versionados y al menos una vez a consumidores durables idempotentes; ACK solo después de persistir el efecto/proyección. Se usan `event_id` único, `stream_cursor` reanudable, `aggregate_id`, `aggregate_version` monotónica, tipo/versión, audiencia/scope, tiempo y correlation ID. PII, secretos y contenido no requerido no se colocan en envelopes de fan-out. Los contratos conservan compatibilidad durante despliegues progresivos con versiones de servicio coexistentes.

El módulo `realtime` mantiene un journal/cursor de recuperación con permisos y retención definidos, y publica por gRPC a BFF autorizados. NATS Core se usa para fan-out efímero entre réplicas WebSocket/gateway; no conserva señales durante desconexiones. Redis se usa exclusivamente para caché con TTL, nunca como canal de distribución ni para cuotas. Si una señal Core se pierde, JetStream/Outbox conserva el cambio y el cliente repara con REST desde el cursor/snapshot del journal. PostgreSQL primario es autoridad de escritura; réplicas sirven lecturas, pero cursor y read-your-writes respetan posición de réplica o consultan al primario tras una mutación. El failover no crea cursores duplicados ni oculta cambios confirmados.

Tenant y administración tienen coordinadores realtime TanStack Query centrales y separados por app/sesión/audiencia; nunca se comparten cookies, caché, query keys, canales ni cursores entre ellas. El coordinador deduplica `event_id`, ignora aggregate versions viejas, detecta gaps/eventos desconocidos, invalida familias de query keys y obtiene el estado real por REST con permisos vigentes. Reconecta con backoff+jitter y cursor; WebSocket no transporta snapshots de autoridad ni reemplaza REST. Siempre se conserva la sincronización manual como mecanismo de recuperación/rollback de presentación. Las vistas no implementan sockets, polling ni mapeos de eventos particulares.

La topología de cuatro backends soporta desde el primer cambio varias réplicas de cada backend, BFF, workers, Redis, NATS Core, JetStream, PostgreSQL y frontend: no depende de memoria local, sticky sessions ni de entrega exactly-once. Se documentan límites de conexión, heartbeat, rate/size, backpressure, cierre/drain, retención/replay durable, DLQ, lag de réplica, rotación de credenciales y despliegue compatible. Cada fase debe verificar el flujo multiinstancia relevante o registrar la prueba de resiliencia pendiente; una prueba solo con una instancia no demuestra escalabilidad horizontal.

---

# 1. Principios SOLID — aplicación obligatoria en Rust

| Principio | Regla obligatoria en Rust |
|---|---|
| **S**RP | Una struct/enum/función = una razón para cambiar. Prohibido `utils.rs` o `helpers.rs` genéricos que mezclen responsabilidades. Casos de uso (`struct XxxUseCase`) hacen UNA sola acción de negocio. |
| **O**CP | Extender comportamiento implementando nuevos `impl Trait` sobre los puertos existentes, nunca modificando código con `match`/`if` de tipos concretos dentro de dominio/aplicación. Usar patrones Strategy/Polymorphism con traits. |
| **L**SP | Toda implementación de un `trait` de puerto debe ser sustituible sin romper el contrato, manteniendo las mismas precondiciones y postcondiciones. Prohibido hacer `panic!` o lanzar errores no tipados donde el trait define un `Result<T, E>`. |
| **I**SP | Traits pequeños y específicos por caso de uso (`ResourceReader`, `ResourceWriter`) en vez de un trait gordo (`ResourceRepository` con 20 métodos) cuando los consumidores no usan todo. Componer traits con `+` (`TraitA + TraitB`). |
| **D**IP | Las capas de dominio y aplicación jamás dependen de crates de infraestructura concreta (`sqlx`, `redis`, `async-nats`, `tonic`). Dependen de `trait`s definidos en dominio/aplicación. La infraestructura implementa esos traits e inyecta `Arc<dyn Trait + Send + Sync>` desde el composition root (`module_config.rs`). |

---

# 1.bis Límite de tamaño de archivo y responsabilidad por archivo

Esta regla es obligatoria y se verifica en cada archivo Rust:

- **Máximo ordinario: 200 líneas con código por archivo `.rs`.**
- Se permite superar 200 y llegar como máximo a 300 líneas con código únicamente con una excepción técnicamente justificada y documentada antes de implementar o aceptar el cambio.
- **Máximo absoluto: 300 líneas con código.** No se permite excederlo por ningún motivo.
- Para el conteo, se cuentan las líneas no vacías que contienen código; se excluyen líneas en blanco y líneas que contienen únicamente comentarios. Una línea con código y comentario inline cuenta como línea de código.
- El tamaño es una señal para revisar SRP, no un sustituto del criterio de diseño: se priorizan cohesión, límites claros y ausencia de archivos "bolsa". La excepción no debe usarse para evitar una división cohesiva.

### Un archivo = una responsabilidad pública principal

Un archivo debe contener:

- una struct pública con su `impl`;
- un enum público;
- un trait público;
- un caso de uso;
- un handler REST/gRPC;

según corresponda.

No se permiten archivos "bolsa" con múltiples structs/funciones no relacionadas.

Ejemplo prohibido:

```text
utils.rs
 ├── calculate_a()
 ├── calculate_b()
 ├── parse_c()
 ├── validate_d()
 └── transform_e()
```

### Excepciones válidas

Las únicas excepciones permitidas son:

- tipos/enums de error estrechamente acoplados al tipo principal (ej. `enum ExecuteActionError` derivando `thiserror::Error` en el mismo archivo del caso de uso);
- DTOs de `Input`/`Output` exclusivos de ese caso de uso;
- constantes privadas usadas solamente por esa struct/función;
- funciones privadas auxiliares (`fn` sin `pub`) que existen únicamente para descomponer la función principal y no son utilizadas en otro lugar.

Si una función auxiliar podría reutilizarse en más de un archivo:

**se extrae a su propio archivo/módulo con su propio nombre.**

No se deja "flotando" dentro de otro archivo ni se duplica.

### Nombre de archivo

**Nombre de archivo = nombre de lo que exporta (en `snake_case`).**

Ejemplo:

```text
execute_action_use_case.rs
```

exporta:

```rust
pub struct ExecuteActionUseCase { ... }
```

Otro ejemplo:

```text
email.rs
```

exporta:

```rust
pub struct Email(String);
```

Evitar `mod.rs` (o el archivo con el nombre del módulo) como contenedor de lógica.

`mod.rs` o `lib.rs` solamente puede utilizarse como barrel de `pub mod` y `pub use`, sin lógica propia.

### Cuando un archivo se acerca o supera 200 líneas con código

Las estrategias de división correctas, en orden de preferencia:

1. Extraer funciones privadas reutilizables o con responsabilidad propia a submódulos cohesivos en la misma carpeta.
2. Extraer subtipos/DTOs a su propio archivo dentro de `dto/` o `value_objects/` según corresponda.
3. Si una función o servicio mezcla responsabilidades, dividirlo en casos de uso, servicios o handlers distintos.
4. Si una función larga representa una única operación coherente y no puede dividirse sin perder cohesión, se puede solicitar una excepción documentada, sin superar 300 líneas con código.

El criterio principal es SRP y cohesión. El límite ordinario es 200 líneas con código; el máximo absoluto, incluso con excepción aprobada, es 300.

Esta regla aplica a TODAS las capas:

- dominio;
- aplicación;
- infraestructura;
- handlers/controladores;
- mappers;
- tests.

---

# 2. Regla de dependencias — Clean Architecture / Hexagonal

```text
        ┌─────────────────────────────────────────┐
        │       INFRASTRUCTURE (adapters)          │
        │       sqlx, redis, async-nats, axum,    │
        │       tonic, CLI                         │
        │  ┌─────────────────────────────────────┐ │
        │  │       APPLICATION (use cases)       │ │
        │  │       structs, traits inbound/out   │ │
        │  │  ┌───────────────────────────────┐  │ │
        │  │  │        DOMAIN (core)          │  │ │
        │  │  │ structs, enums, VOs,          │  │ │
        │  │  │ aggregates, pure business rules│ │
        │  │  └───────────────────────────────┘  │ │
        │  └─────────────────────────────────────┘ │
        └─────────────────────────────────────────┘
```

### Regla de oro

Las flechas de dependencia (`use` y dependencias en `Cargo.toml`) **SIEMPRE apuntan hacia adentro**.

El dominio:

- no depende de aplicación;
- no depende de infraestructura;
- no depende de frameworks;
- no depende de crates de base de datos (`sqlx`, `diesel`, `sea-orm`);
- no depende de Redis (`redis`, `deadpool-redis`);
- no depende de NATS (`async-nats`);
- no depende de clientes HTTP ni servidores (`reqwest`, `axum`, `actix-web`);
- no depende de detalles de persistencia ni wire-formats (no atributos `serde` de transporte ni `sqlx::FromRow` en dominio).

La aplicación:

- no depende de infraestructura concreta;
- no depende de implementaciones concretas de DB/Redis/NATS/gRPC;
- depende únicamente del dominio y de las abstracciones (`traits`) de puertos de entrada y salida.

La infraestructura:

- depende de dominio y aplicación;
- implementa los `traits` definidos como puertos.

### Prohibiciones en Rust

Prohibido:

```rust
use sqlx::FromRow;
#[derive(FromRow)]
```

o:

```rust
use serde::{Serialize, Deserialize};
```

con anotaciones de wire-format o persistencia dentro de:

```text
domain/
```

El dominio debe estar compuesto por structs y enums puros de Rust (a lo sumo con derives estándar como `#[derive(Debug, Clone, PartialEq, Eq)]`), sin acoplamiento a librerías de infraestructura.

Prohibido poner lógica de negocio dentro de:

- handlers REST/gRPC;
- suscriptores/listeners de eventos NATS;
- repositorios de persistencia.

Esas capas solamente:

- traducen;
- adaptan;
- orquestan.

Prohibido el uso de `unwrap()`, `expect()` o `panic!` en rutas de ejecución que procesan datos de entrada o llamadas externas. Usar siempre `Result<T, DomainError>` con `thiserror`.

### Mapeo

El mapeo entre:

```text
modelo de persistencia (FromRow struct)
        ↓
entidad de dominio
```

se hace SIEMPRE en infraestructura (`infrastructure/persistence/sqlx/mappers`).

Nunca al revés.

---

# 3. Estructura de carpetas obligatoria (workspace Cargo raíz)

La estructura interna se aplica módulo por módulo. El único manifiesto raíz de workspace es `Cargo.toml`; los cuatro backends son miembros de ese workspace, y los paquetes conservan manifiestos propios donde Cargo los requiere sin crear workspaces anidados. Los entrypoints, identidades, configuración y artefactos de despliegue de cada backend siguen separados. El siguiente esquema es conceptual: los nombres de aplicaciones y módulos se concretan según los contratos y decisiones aprobados.

```text
Cargo.toml                         # único [workspace] del repositorio
Cargo.lock                         # resolución reproducible compartida
backend-core/
  apps/<core-runtime>/
  modules/<core-context>/
backend-business/
  apps/<business-runtime>/
  modules/<business-context>/
backend-platform/
  apps/<platform-runtime>/
  modules/<platform-context>/
backend-public/
  apps/<public-runtime>/
  modules/<public-context>/
<public-bff>/                      # aplicación independiente para Discovery
<product-bff>/                     # aplicación BFF independiente por producto implementado
frontends/
  ensambria-business/              # Flutter/Dart, móvil y escritorio
  tenant-portal/                   # Next.js, administración web de tenant
  ensambria-app/                   # Flutter/Dart, clientes
  marketplace/                     # Next.js, descubrimiento público
  platform-admin/                  # Next.js, administración global
  corporate/                       # Next.js, contenido institucional
```

El `Cargo.toml` raíz es el único manifiesto `[workspace]` y registra los cuatro backends como miembros. Cada backend conserva sus paquetes/binarios y configuración de despliegue, pero no declara un workspace propio; Cargo comparte resolución y `Cargo.lock` en la raíz. Los cuatro producen artefactos y despliegues independientes. El BFF público y cada uno de los seis proyectos de interfaz también conservan artefactos, configuraciones y ciclos de release/despliegue apropiados. La Fase 0 audita y documenta la composición prevista del workspace; crates/apps funcionales y límites de compilación ejecutables se incorporan en sus fases de implementación, sin dependencias cruzadas de implementación.

CORE contiene los módulos de identidad/contexto de seguridad, reservas y pagos/ledger, incluidos los adapters de pago que se aprueben. BUSINESS contiene los contextos operativos, Commerce y facturación de establecimientos; PLATFORM contiene catálogo comercial de productos, administración SaaS, planes, suscripciones y facturación de ENSAMBRIA; PUBLIC contiene Discovery, proyecciones públicas autorizadas y atribución. Cada owner decide sus workers, adapters y módulos realtime. El BFF público independiente consume PUBLIC por gRPC; cada BFF de producto consume por contratos a los owners correspondientes y el BFF de administración se integra inicialmente en PLATFORM. Los callbacks se validan en el owner de la integración correspondiente; los proveedores de pago no constituyen un backend adicional.

```text
Cargo.toml                             # manifiesto [workspace] único, en la raíz del repo

apps/
  bff/                                  # API Gateway REST — binario axum SIN lógica de negocio
    Cargo.toml
    src/
      modules/
        <domain>/
          handlers/                     # handlers REST (axum)
          dto/                          # DTOs REST y schemas de request/response
      common/
        grpc/                           # clientes tonic gRPC tipados
      main.rs

  api/                                  # (opcional) proceso monolito que hostea los módulos
    Cargo.toml
    src/
      main.rs

modules/
  <bounded-context>/                    # ej: module-a, module-b, etc. (Crate independiente)
    Cargo.toml
    src/
      domain/
        entities/                       # structs con identidad y métodos de negocio
        value_objects/                  # newtypes inmutables y auto-validados
        aggregates/                     # raíces de agregado + invariantes
        events/                         # domain events internos (enum)
        repositories/                   # TRAITS / puertos de salida (async_trait)
        services/                       # domain services
        errors/                         # enums de error de dominio (thiserror)

      application/
        use_cases/                      # 1 archivo = 1 caso de uso (struct + impl)
        ports/
          inbound/                      # traits de entrada (driving ports)
          outbound/                     # traits de salida: cache, publisher, repo...
        dto/                            # DTOs application
        mappers/                        # domain <-> DTO

      infrastructure/
        persistence/
          sqlx/                         # implementación PostgreSQL concreta
            entities/                   # structs FromRow (modelo de persistencia)
            repositories/               # implementan domain::repositories (traits)
            mappers/                    # persistence <-> domain
            migrations/                 # migraciones SQL versionadas del módulo

        cache/
          redis/                        # implementación CachePort (deadpool-redis)

        messaging/
          nats/
            publishers/                 # implementación EventPublisher (async-nats)
            subscribers/                # consumidores de eventos (tareas tokio)
            schemas/                    # contratos versionados (serde)

        grpc/
          proto/                        # contrato gRPC versionado (.proto)
          server/                       # implementación del servicio tonic
          dto/                          # proto <-> application DTO

        http/
          handlers/                     # SOLO casos externos al BFF (webhooks, health)
          dto/                          # DTOs HTTP
          validators/                   # validadores HTTP

      lib.rs
      module_config.rs                  # composition root / wiring DI (Arc<dyn Trait>)
      module_public_api.rs              # contrato lógico público del crate

shared-kernel/                          # Crate: SOLO elementos verdaderamente universales
  Cargo.toml
  src/
    lib.rs

platform/                               # Crate: infraestructura transversal
  Cargo.toml
  src/
    telemetry.rs                        # init de tracing + OpenTelemetry + OTLP gRPC exporter
    config.rs                           # configuración transversal
    health.rs                           # endpoints de health checks

main.rs                                 # punto de entrada principal si aplica
```

### Reglas de esta estructura

Todo módulo expone su comunicación entrante ÚNICAMENTE vía:

```text
module_public_api.rs
```

técnicamente materializado como servicio:

```text
infrastructure/grpc
```

Ningún otro crate de módulo ni el BFF puede agregar dependencias internas en su `Cargo.toml` apuntando a:

```text
modules/otro-modulo/src/domain/...
modules/otro-modulo/src/application/...
modules/otro-modulo/src/infrastructure/...
```

Reforzar mediante reglas de visibilidad estricta de Rust:

- Solo `module_public_api.rs` o los contratos gRPC/DTOs de exportación son `pub`.
- El resto del árbol interno del crate debe ser `pub(crate)` o privado.
- Usar herramientas de análisis estático como `cargo deny`, `cargo-modules` o `cargo-machete`.

### Comunicación módulo → módulo

Si módulo A necesita datos de módulo B de forma síncrona:

```text
module-a
   │
   │ gRPC (tonic client)
   ▼
module-b (tonic server)
```

Debe invocar el mismo servicio gRPC de B que utiliza el BFF.

Debe existir un único contrato público por módulo.

Si la necesidad es asíncrona:

```text
module-b
   │
   │ event (async-nats)
   ▼
NATS JetStream
   │
   ▼
module-a
```

Nunca:

```text
module-a → DB de module-b
module-a → Redis de module-b
```

---

# 4. Reglas de DDD

## 4.1 Bounded Context primero

Antes de escribir código:

1. define el lenguaje ubicuo;
2. define los límites del contexto;
3. define qué conceptos pertenecen al contexto;
4. define qué conceptos NO pertenecen al contexto.

Un mismo término puede significar cosas distintas en diferentes módulos.

Esto está permitido y es deseable.

No unificar artificialmente conceptos que realmente pertenecen a diferentes contextos.

---

## 4.2 Entidades

Las entidades tienen:

- identidad estable (`id: ResourceId`);
- comportamiento;
- invariantes protegidas.

Ejemplo conceptual en Rust:

```rust
pub struct Resource {
    id: ResourceId,
    state: ResourceState,
}

impl Resource {
    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        if self.state == ResourceState::Inactive {
            return Err(DomainError::AlreadyInactive);
        }
        self.state = ResourceState::Inactive;
        Ok(())
    }
}
```

Preferir:

```rust
resource.deactivate()?;
```

en lugar de:

```rust
resource.set_active(false); // Prohibido setter genérico
```

Prohibidos los setters genéricos públicos que permitan modificar arbitrariamente el estado de una entidad sin validar invariantes.

---

## 4.3 Value Objects

Los Value Objects deben ser:

- inmutables;
- usando el patrón **Newtype** (`struct Email(String)`);
- comparables por valor (`#[derive(PartialEq, Eq, Clone, Debug)]`);
- auto-validados en constructores (`Email::try_from(raw)` o `Email::new(raw) -> Result<Email, DomainError>`);
- independientes de infraestructura.

Ejemplos genéricos:

```text
Email
Money
DocumentId
Address
Percentage
DateRange
Quantity
Currency
```

No representar Value Objects como `String`, `i64` o `f64` sueltos dentro de las firmas del dominio cuando el concepto tenga reglas propias.

---

## 4.4 Agregados

Una raíz de agregado (`AggregateRoot`) es el único punto de entrada público para modificar su árbol de objetos internos.

### Regla por defecto

Un caso de uso debe modificar **un solo agregado por transacción**.

Esto ayuda a:

- mantener límites de consistencia fuertes;
- reducir acoplamiento;
- evitar transacciones `sqlx::Transaction` excesivamente grandes;
- evitar dependencias ocultas entre agregados.

Cuando el negocio lo permita, si es necesario coordinar múltiples agregados, priorizar:

```text
Domain Event
      ↓
Consistencia eventual
```

en lugar de:

```text
Transacción gigante
```

### Excepción

Existen casos legítimos donde una misma transacción debe involucrar más de un agregado dentro del mismo límite transaccional.

Está permitido solamente cuando:

1. ambos agregados pertenecen al mismo módulo/crate;
2. nunca cruzan un límite de módulo;
3. la decisión está documentada explícitamente en código o ADR;
4. se explica por qué la consistencia eventual no es aceptable;
5. primero se evalúa si el diseño del agregado está mal delimitado.

Los agregados deben proteger sus invariantes.

Debe ser imposible dejar el agregado en un estado inválido desde fuera.

---

## 4.5 Domain Events

Toda mutación de negocio relevante emite un Domain Event.

Ejemplos genéricos en Rust:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum DomainEvent {
    ResourceCreated { id: ResourceId, occurred_at: DateTime<Utc> },
    ResourceUpdated { id: ResourceId, occurred_at: DateTime<Utc> },
    ResourceDeleted { id: ResourceId, occurred_at: DateTime<Utc> },
    ProcessStarted { id: ProcessId, occurred_at: DateTime<Utc> },
    ProcessCompleted { id: ProcessId, occurred_at: DateTime<Utc> },
    ProcessFailed { id: ProcessId, reason: String, occurred_at: DateTime<Utc> },
}
```

Los eventos son recolectados por el agregado y despachados por la aplicación después de confirmar la persistencia.

Esto se coordina con el Outbox Pattern de la sección 6.

### Importante

Los:

```text
Domain Events internos (enum de dominio)
```

NO son exactamente lo mismo que:

```text
Integration Events (structs con serde Serialize/Deserialize para NATS)
```

Los eventos de integración publicados a NATS deben ser un mapeo:

```text
Domain Event
      ↓
Integration Event DTO
      ↓
Versioned Contract (serde JSON/Protobuf)
      ↓
NATS JetStream
```

Nunca serializar directamente el enum interno del dominio como contrato externo.

---

## 4.6 Repositorios

La interfaz vive en:

```text
domain/repositories/
```

como un `trait` (utilizando `#[async_trait]` para compatibilidad con trait objects `dyn Trait`).

Los métodos deben estar orientados a agregados completos.

Ejemplos:

```rust
#[async_trait]
pub trait ResourceRepository: Send + Sync {
    async fn save(&self, aggregate: &Resource) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &ResourceId) -> Result<Option<Resource>, RepositoryError>;
    async fn next_identity(&self) -> ResourceId;
}
```

Prohibido exponer desde el trait de dominio métodos de tipo:

```text
find_where
query_raw
query_builder
execute_sql
```

Esos mecanismos pertenecen a adaptadores de infraestructura o adaptadores específicos de lectura.

---

## 4.7 Domain Services

Usar Domain Services solamente cuando una regla de negocio:

- involucra más de una entidad/agregado;
- no pertenece naturalmente a ninguna entidad.

Si la regla puede vivir dentro de una entidad:

**debe vivir dentro de la entidad.**

No crear:

```text
GenericBusinessService
BusinessManager
DomainHelper
```

sin una responsabilidad clara y acotada.

---

## 4.8 Anti-Corruption Layer (ACL)

Cuando un módulo consume:

- otro módulo;
- otro servicio;
- un sistema externo;
- eventos externos;

debe traducir el modelo externo al lenguaje ubicuo propio mediante un mapper dedicado en infraestructura.

Nunca permitir que el modelo de otro contexto se filtre dentro del dominio propio.

Ejemplo:

```text
External Prost/JSON Model
            ↓
       ACL Mapper
            ↓
    Own Application DTO
            ↓
        Own Domain
```

---

# 5. Reglas de Casos de Uso — Application Layer

### Regla principal

```text
1 struct con su impl
=
1 caso de uso
=
1 archivo
```

Ejemplo:

```text
execute_action_use_case.rs
revert_action_use_case.rs
create_resource_use_case.rs
delete_resource_use_case.rs
```

### Firma estándar

Un caso de uso:

- recibe un `Input DTO` por valor o referencia;
- devuelve `Result<OutputDTO, UseCaseError>`.

Nunca expone directamente:

- entidades de dominio;
- modelos `FromRow` de base de datos;
- modelos de persistencia.

### Responsabilidad

El caso de uso orquesta:

```text
validar input DTO
       ↓
recuperar agregado vía puerto (Arc<dyn Repository>)
       ↓
invocar métodos de negocio del dominio
       ↓
persistir vía puerto
       ↓
registrar/despachar eventos
```

El caso de uso **NO contiene reglas de negocio propias**.

Su responsabilidad es orquestar.

### Errores

Utilizar enums de error tipados derivados con `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}
```

Prohibido el uso de `panic!`, `unwrap()` o strings de error genéricos para representar errores de negocio.

### Inyección de Dependencias

Rust favorece la composición explícita sobre magia de frameworks:

```rust
pub struct ExecuteActionUseCase {
    repo: Arc<dyn ResourceRepository>,
    publisher: Arc<dyn EventPublisher>,
}

impl ExecuteActionUseCase {
    pub fn new(repo: Arc<dyn ResourceRepository>, publisher: Arc<dyn EventPublisher>) -> Self {
        Self { repo, publisher }
    }
}
```

El cableado se realiza en `module_config.rs`.

### Idempotencia

Todo caso de uso que reacciona a un evento externo debe ser idempotente.

Usar:

```text
event_id
idempotency_key
```

o mecanismo equivalente para deduplicar.

---

# 6. Base de datos

## 6.1 Propiedad de los datos

**Todos los módulos se conectan a una única base de datos PostgreSQL. Cada módulo es dueño exclusivo de su schema lógico, sus tablas y sus migraciones.** La base física es compartida; la propiedad lógica de los datos no.

Usa un namespace/schema de PostgreSQL por módulo. No crees una base de datos independiente por módulo.

Prohibido hacer JOINs SQL entre tablas pertenecientes a módulos diferentes.

Si necesitas datos de otro módulo:

```text
module_public_api / gRPC (tonic)
```

o:

```text
evento NATS → proyección local de lectura
```

---

## 6.2 Migraciones

Las migraciones deben estar versionadas por módulo.

Ejemplo:

```text
infrastructure/
  persistence/
    sqlx/
      migrations/
        20260906000000_create_resources.sql
```

Cada módulo mantiene su propia evolución de esquema gestionada con herramientas como `sqlx migrate` o `refinery`.
Las migraciones se almacenan y ejecutan por módulo contra la misma base PostgreSQL, con identificadores/versionado y ownership inequívocos. El arranque o despliegue coordina su aplicación; ningún módulo modifica el schema o las tablas de otro.

### Identificadores canónicos

Para ENSAMBRIA, UUID v7 es el identificador canónico predeterminado de entidades distribuidas. El backend dueño lo conserva sin traducción en persistencia, contratos y eventos. La identidad global de una persona y sus membresías por tenant son entidades/relaciones distintas bajo CORE. `BIGINT` solo puede añadirse como clave o índice local con evidencia medida; no se expone ni sustituye autorización. No dupliques IDs públicos sin necesidad documentada. Cada entidad tiene una única autoridad de generación (por defecto, la aplicación Rust propietaria); generación PostgreSQL solo tras verificar la versión exacta desplegada. UUIDs no son secretos ni capabilities. Las excepciones de formato requieren justificación explícita y pruebas de compatibilidad/autorización.

---

## 6.3 Transacciones

La transacción tiene como límite por defecto el agregado.

Nunca abrir una transacción `sqlx::Transaction` que cruce dos módulos.

Cruzar dos agregados del mismo módulo solamente se permite bajo las excepciones documentadas en la sección de agregados.

---

## 6.4 Outbox Pattern obligatorio

Para ENSAMBRIA, aplica [ENSAMBRIA_Transactional_Outbox_Inbox.md](ENSAMBRIA_Transactional_Outbox_Inbox.md) como guía especializada de ownership, contratos, publicación, consumo, retención, recuperación y aceptación. NATS JetStream es el mecanismo durable definido por la arquitectura del producto; NATS Core queda limitado a fan-out efímero. Mantén Outbox y el cambio de dominio en una única transacción local y la Inbox junto con el efecto local del consumidor; entrega al menos una vez, con confirmación durable antes de marcar como publicado y ACK posterior al commit del efecto. Los detalles operativos deben validarse contra la configuración real; no afirmes que existe infraestructura configurada solo porque esté definida.

Para publicar eventos consistentemente con la escritura de DB:

### Paso 1

Dentro de la MISMA transacción `sqlx::Transaction` que persiste el cambio de estado:

```text
UPDATE / INSERT / DELETE en tablas de dominio
        +
INSERT en outbox_events
```

### Paso 2

Un proceso / worker separado (tarea `tokio::spawn` dedicada o servicio relay):

```text
outbox_events
      ↓
Outbox Relay
      ↓
NATS JetStream (async-nats)
```

marca los eventos como enviados después de publicarlos correctamente.

### Paso 3

Esto evita:

```text
DB guardada + evento perdido
```

o:

```text
evento publicado + DB no guardada
```

---

## 6.5 Repositorios

Los repositorios de infraestructura devuelven SIEMPRE:

```text
Domain Entities
```

reconstituidas mediante mappers.

Nunca devolver:

```text
struct FromRow / modelos SQLX crudos
```

directamente a aplicación o dominio.

Prohibido usar `sqlx::query!` o query builders directamente dentro de:

```text
domain/
application/
use_cases/
```

Solo dentro de implementaciones concretas de repositorio en:

```text
infrastructure/persistence/sqlx/repositories/
```

---

# 7. Redis — cache y lecturas

Redis se accede SIEMPRE mediante un trait de puerto de salida definido en:

```text
application/ports/outbound/cache_port.rs
```

Ejemplo:

```rust
#[async_trait]
pub trait CachePort: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError>;
    async fn set(&self, key: &str, value: &str, ttl_secs: u64) -> Result<(), CacheError>;
    async fn delete(&self, key: &str) -> Result<(), CacheError>;
}
```

Implementado en:

```text
infrastructure/cache/redis/
```

utilizando `redis` + `deadpool-redis` o `bb8-redis`.

Nunca importar el cliente Redis directamente en:

```text
domain/
application/
```

## Usos permitidos

### Cache de lectura

Usar cache-aside:

```text
PostgreSQL
    ↓
  Redis
```

Siempre con:

- TTL explícito;
- estrategia de invalidación documentada.

Preferentemente la invalidación se relaciona con eventos de dominio cuando corresponda.

No depender únicamente de:

```text
TTL y ya
```

---

## Modelos de lectura desnormalizados

Cuando una consulta necesita combinar información de varios agregados:

No convertir el dominio en un sistema de JOINs.

Se puede utilizar:

```text
Redis
```

o:

```text
tabla de lectura SQL
```

actualizada mediante eventos de integración.

Esto corresponde a un enfoque de CQRS ligero.

---

## Locks distribuidos

Permitidos para:

- idempotencia;
- evitar dobles ejecuciones;
- coordinación distribuida.

Utilizar librerías probadas en Rust, por ejemplo crates basados en Redlock (`rslock` o `redlock`).

No implementar un mecanismo casero basado únicamente en `SETNX` sin expiración y sin estrategia de recuperación.

---

## Rate limiting

Redis puede utilizarse para:

- rate limiting;
- sesiones;
- colas de trabajo ligeras;
- estado técnico.

---

## Prohibición

Redis nunca debe ser la única fuente de verdad de datos críticos de negocio.

Todo dato crítico debe poder reconstruirse desde:

```text
DB (PostgreSQL)
```

y/o:

```text
eventos NATS
```

según el diseño.

---

## Namespacing

Las keys deben estar namespaced por módulo.

Formato:

```text
<module>:<entity>:<id>:<subresource>
```

Ejemplo genérico:

```text
module-a:resource:123:metadata
```

---

# 8. Eventos con NATS JetStream

## 8.1 Contrato versionado

Cada evento de integración debe tener un schema explícito con `#[derive(Serialize, Deserialize)]`.

Ejemplo:

```text
infrastructure/messaging/nats/schemas/resource_updated_v1.rs
```

Debe contener como mínimo:

```text
event_id
event_type
event_version
occurred_at
producer_module
payload
trace_id
```

Los cambios incompatibles requieren una nueva versión:

```text
resource_updated_v2.rs
```

Nunca modificar silenciosamente el contrato existente.

---

## 8.2 Naming

Subjects:

```text
<context>.<aggregate>.<event>
```

en minúsculas y utilizando eventos en pasado.

Ejemplos genéricos:

```text
module-a.resource.updated
module-b.resource.reserved
module-c.process.completed
```

Los streams de JetStream deben estar organizados por bounded context cuando corresponda.

Ejemplo:

```text
STREAM: MODULE_A_EVENTS
```

---

## 8.3 Publisher

El publisher se define como trait de puerto en:

```text
application/ports/outbound/event_publisher.rs
```

y se implementa en:

```text
infrastructure/messaging/nats/publishers/
```

utilizando el crate `async-nats`.

Los casos de uso NUNCA importan directamente el cliente de NATS (`async_nats::Client`).

---

## 8.4 Subscriber

Cada handler entrante vive en:

```text
infrastructure/messaging/nats/subscribers/
```

y se ejecuta típicamente en una tarea asíncrona dedicada (`tokio::spawn`).

El subscriber:

1. recibe el mensaje;
2. valida y deserializa el payload con `serde_json`;
3. utiliza la capa ACL para mapear a DTO propio;
4. llama a un caso de uso;
5. hace `msg.ack().await` únicamente si el caso de uso tuvo éxito.

Nunca ejecuta lógica de negocio directamente.

---

## Idempotencia

Todo consumidor debe ser idempotente.

Deduplicar mediante:

```text
event_id
```

o mecanismo equivalente en DB (ej. tabla `processed_events`) antes de ejecutar acciones de negocio.

---

## Ack manual

No utilizar auto-ack para operaciones críticas.

El mensaje solamente se confirma después de que el procesamiento haya sido persistido correctamente:

```text
receive
   ↓
process (Use Case)
   ↓
persist (DB)
   ↓
msg.ack().await
```

No:

```text
receive
   ↓
msg.ack().await
   ↓
process
```

---

## Retry y Dead Letter

Cada consumidor debe definir:

- política de retries con exponential backoff;
- límites de reintentos;
- dead-letter subject/stream;
- estrategia de recuperación.

---

## At-least-once

La arquitectura debe asumir:

```text
AT-LEAST-ONCE DELIVERY
```

Nunca asumir:

```text
EXACTLY-ONCE
```

Por lo tanto, todos los consumidores deben diseñarse para tolerar mensajes duplicados.

---

## JetStream vs Core NATS

Para eventos que deben:

- sobrevivir reinicios;
- permitir replay;
- garantizar persistencia;

utilizar:

```text
NATS JetStream (async-nats)
```

Core NATS solamente es aceptable para señales efímeras no críticas cuando corresponda, por ejemplo:

```text
invalidación de cache en tiempo real
```

---

## Comunicación síncrona

La comunicación síncrona:

```text
BFF → módulo
```

o:

```text
módulo → módulo
```

NO debe hacerse mediante NATS request-reply si puede evitarse.

Para comunicación síncrona utilizar:

```text
gRPC (tonic)
```

Para comunicación asíncrona utilizar:

```text
NATS JetStream (async-nats)
```

---

# 9. BFF, REST y Frontend

## Frontend

El Frontend:

- no contiene lógica de negocio;
- consume únicamente endpoints REST del BFF cuando exista BFF;
- maneja presentación;
- maneja estado de UI;
- puede realizar validaciones superficiales de formularios.

No debe duplicar reglas de negocio del backend.

---

## REST / OpenAPI

REST/OpenAPI es el contrato de entrada del BFF hacia el frontend.

Los endpoints REST y schemas OpenAPI (ej. generados con `utoipa`) definen:

- forma de los datos;
- operaciones disponibles;
- validación de transporte.

Los handlers REST (en `axum` o `actix-web`) son adaptadores puros:

```text
HTTP Request (axum)
        ↓
Handler REST
        ↓
DTO / gRPC Request
        ↓
Tonic Client (gRPC)
        ↓
Módulo de Backend
```

No contienen lógica de negocio.

---

# BFF

El BFF es una capa de:

- adaptación;
- composición;
- transformación;
- routing;
- validación de transporte HTTP.

Puede combinar respuestas de varios módulos.

### Prohibido

Nunca implementar reglas de negocio dentro del BFF.

Si un endpoint empieza a tener:

- condicionales de negocio;
- cálculos de dominio;
- decisiones basadas en estado de agregados;
- reglas de permisos de dominio;
- invariantes;

esa lógica pertenece al módulo correspondiente.

---

## Responsabilidades técnicas permitidas

El BFF puede realizar:

- routing HTTP (`axum::Router`);
- validación de payload y OpenAPI (`utoipa`);
- verificación de sesión/token JWT;
- agregación de respuestas concurrentes con `tokio::join!`;
- mapeo de errores gRPC (`tonic::Status`) a códigos de estado HTTP (`StatusCode`);
- cache de presentación de corta duración.

El cache del BFF nunca sustituye el cache de dominio del módulo.

---

## Prohibiciones del BFF

El BFF NO puede acceder directamente a:

```text
DB (sqlx)
Redis del módulo
entities
aggregates
repositories
domain/
application/
infrastructure/persistence/
```

El BFF solamente conoce:

```text
contrato gRPC público (.proto)
```

---

# Comunicación BFF → módulos

Cada módulo expone:

```text
module_public_api.rs
```

como servicio:

```text
infrastructure/grpc/
```

definido mediante:

```text
.proto versionado (compilado con tonic-build / prost)
```

El BFF actúa como cliente gRPC (`tonic::transport::Channel` + cliente tipado generado).

Nunca importa código interno del módulo.

Nunca conoce su implementación.

Solo conoce el contrato.

El contrato gRPC no debe cambiar porque el módulo pase de:

```text
módulo en monolito
```

a:

```text
microservicio independiente
```

Solamente cambia:

```text
dirección de red (endpoint gRPC)
```

---

## Diagrama

```text
Frontend
   │
   │ HTTP fetch / REST
   ▼
 BFF (axum)
   │
   │ gRPC (tonic)
   ▼
 module-a   module-b   module-c
     │          │          │
     ▼          ▼          ▼
  DB (sqlx)  DB (sqlx)  DB (sqlx)
```

---

## Comunicación entre módulos

La comunicación entre módulos:

**NO pasa por el BFF.**

El BFF nunca es intermediario de una llamada:

```text
module-a → module-b
```

La comunicación módulo → módulo sigue:

### Síncrona

```text
module-a → gRPC (tonic) → module-b
```

### Asíncrona

```text
module-a
   ↓
NATS JetStream (async-nats)
   ↓
module-b
```

---

# 10. Preparación explícita para extracción a microservicios

Los cuatro límites de backend ya son explícitos. Dentro de cada backend, una extracción futura de módulo debe seguir siendo selectiva, justificada y aproximadamente:

```text
mover crate a su propio repositorio / binario
+
cambiar adapter de transporte y deployment
```

y no:

```text
reescribir dominio
+
reescribir aplicación
+
reescribir contratos
```

Para ello:

## 1. Cero imports internos cruzados

Nunca:

```rust
// PROHIBIDO en module-a/Cargo.toml:
// path = "../module-b/src/domain"
```

---

## 2. Comunicación remota desde el día uno

La comunicación utiliza:

```text
gRPC (tonic)
NATS JetStream (async-nats)
```

desde el principio.

Extraer un módulo solamente cambia:

```text
network address
```

No cambia:

```text
contract (.proto / event schemas)
domain
application
```

---

## 3. Datos propios

Todos los módulos se conectan a la misma base PostgreSQL, y cada uno posee sus propios datos y schema:

```text
module-a → PostgreSQL schema A
module-b → PostgreSQL schema B
module-c → PostgreSQL schema C
```

Nunca:

```text
shared tables / tables owned by another module
```

---

## 4. Eventos versionados

Los eventos de integración ya deben estar:

- desacoplados;
- versionados;
- documentados con schemas serde/Protobuf.

El microservicio extraído simplemente se conecta al mismo sistema de eventos.

---

## 5. Configuración por módulo

Variables de entorno, feature flags y configuración deben estar aisladas por crate.

Ejemplo:

```text
module_config.rs
```

La configuración debe poder trasladarse con el módulo.

---

## 6. Observabilidad por módulo

Cada módulo debe tener:

- logs estructurados (`tracing`);
- atributo `module` / `service.name`;
- `trace_id` propagado;
- métricas;
- tracing distribuido con OpenTelemetry.

El `trace_id` debe propagarse mediante:

```text
gRPC metadata (tonic)
NATS headers / payload
HTTP headers (axum)
```

según corresponda.

---

## 7. Checklist de extracción

Antes de extraer un módulo:

```text
¿Depende del código interno de otro módulo?
¿Comparte transacción sqlx con otro módulo?
¿Comparte estado en memoria (OnceCell, static, Mutex global) con otro módulo?
```

Las tres respuestas deben ser:

```text
NO
NO
NO
```

Si es así, la extracción es arquitectónicamente segura, sujeto a validar configuración, contratos, infraestructura y despliegue.

---

# 11. Shared Kernel — uso muy restringido

El crate `shared-kernel` SOLO contiene elementos verdaderamente universales y compartidos.

Ejemplos:

```rust
Result<T, E> // Helpers comunes si aplica
Uuid         // Re-export o Newtype universal
Clock        // Trait de reloj para testabilidad
IdGenerator  // Trait para IDs
errores base
tipos primitivos universales
```

Un `Money`, `Currency`, `Address` u otro Value Object solo debe estar allí si realmente es un concepto compartido por contrato y con semántica idéntica.

No debe agregarse algo simplemente porque:

```text
"dos módulos lo necesitan"
```

Si dos módulos necesitan conceptos similares pero tienen reglas distintas:

**se duplican.**

Es preferible:

```text
duplicación
```

antes que:

```text
acoplamiento incorrecto.
```

---

# 12. Observabilidad — logs, métricas y tracing

La instrumentación es obligatoria en TODA capa relevante:

```text
módulos
BFF (axum)
REST
gRPC (tonic)
NATS (async-nats)
DB (sqlx)
Redis
```

Debe existir:

- logging estructurado con el crate `tracing`;
- métricas (`opentelemetry::metrics`);
- distributed tracing (`tracing-opentelemetry`);
- correlation ID;
- trace ID.

### Errores estructurados y diagnosticables

Esta regla se aplica a **todos** los dominios, módulos, workers, BFF, integraciones y frontends y a **todas las fases y subfases**, desde su inicio. No se difiere a una fase transversal posterior. Un mensaje genérico como `server error`, `temporarily unavailable` o un tipo Rust sin contexto no es suficiente como diagnóstico interno.

- Cada error debe conservar su clase/código estable, bounded context, operación, dependencia y resultado; propagar `correlation_id` y `trace_id` en HTTP, gRPC, base de datos y eventos cuando existan.
- Los logs deben registrar metadatos diagnósticos seguros: código SQLSTATE/gRPC/HTTP o del proveedor, clase de error, componente/operación y, cuando aplique, constraint/tabla/columna o estado de reintento. Distingue fallos de validación, autorización, conflicto, timeout, pool/conectividad, persistencia, serialización y proveedor.
- No reduzcas un error de infraestructura a un `error_type` genérico cuando el código de error o metadatos seguros estén disponibles. No registres contraseñas, cookies, tokens, URL firmadas, credenciales, cuerpos de contenido ni valores de entrada sensibles. El mensaje/detail original de una dependencia solo se registra si se revisó que no puede contener dichos valores; en caso contrario registra campos seguros y una clase explícita.
- REST devuelve un envelope estable con `code`, `message` y `details` seguros, más correlation/request ID; `details` describe categoría, operación segura, posibilidad/tiempo de reintento o campos que el usuario puede corregir. Los errores de campo incluyen rutas/códigos de validación sin repetir valores sensibles. No devuelve SQL, stack traces, nombres internos sensibles ni secretos.
- El frontend traduce el envelope a una explicación clara y accionable, conserva la referencia de solicitud para soporte y permite recuperación cuando sea posible. No presenta `No se pudo contactar con el servidor` si el backend devolvió un error tipado más específico; tampoco muestra errores internos de DB o proveedor al usuario final.
- Agrega pruebas de contrato para el mapeo error de dominio → gRPC → HTTP → interfaz y verifica que los metadatos útiles se registren sin PII/secretos.
- Un evento de finalización que solo contiene método, ruta, status y duración es un resumen HTTP, no un diagnóstico. Para cada solicitud con error, correlaciona eventos de entrada, llamada a dependencia, fallo causal y traducción final; agrega duración por dependencia separada de duración total. En cada RPC, un span servidor extrae `traceparent` y `x-correlation-id`, registra método/código/duración y hace que el contexto esté disponible a los handlers. Si no puede confirmarse causa/dependencia/reintento, se registra explícitamente como no clasificado; está prohibido inferirlos a partir solo del status.
- Cada fase y subfase incluye pruebas unitarias y pruebas de integración/aceptación automatizadas en las fronteras reales aplicables. La aceptación debe emparejar el fallo específico de operación con el cierre HTTP por `correlation_id` y `trace_id`, asegurar que el cierre conserva su código/categoría y comprobar respuesta/log seguros sin secretos o PII. No se acepta como única evidencia compilar, comprobar el status, ni exigir un recorrido manual del navegador para demostrar el backend/BFF.

---

## Propagación de contexto

La propagación debe cubrir toda la cadena:

```text
HTTP/REST (axum)
   ↓
BFF
   ↓
gRPC metadata (tonic)
   ↓
Module
   ↓
Database (sqlx) / Redis
   ↓
NATS JetStream (async-nats)
   ↓
otro Module
```

---

## OpenTelemetry

**OpenTelemetry** es el estándar de instrumentación.

Debe utilizarse como mecanismo común de:

- tracing;
- métricas;
- propagación de contexto vía OTLP gRPC.

Stack estándar:

- Spans y logs emitidos vía `tracing` y `tracing-opentelemetry`.
- Exportados vía OTLP gRPC (`opentelemetry-otlp`) al **OpenTelemetry Collector**.
- Collector enruta trazas a **Tempo**, métricas a **Prometheus** (con **Alertmanager**) y logs a **Loki**.
- Profiling continuo con **Pyroscope** (`pyroscope_pprofrs`).
- Visualización unificada en **Grafana**.

---

## Métricas segmentadas

Las métricas deben segmentarse por:

- módulo/bounded context (`service.name`);
- endpoint REST;
- método gRPC;
- consumidor NATS;
- productor NATS.

Nunca depender únicamente de una métrica global del proceso.

---

## Logging

Prohibido utilizar:

```rust
println!
eprintln!
dbg!
```

como mecanismo de observabilidad de producción.

Todo log de producción debe ser:

```text
estructurado (JSON)
```

emitido a través de `tracing::info!`, `tracing::warn!`, `tracing::error!`.

---

## Campos mínimos

Cada log/span debe incluir, cuando corresponda:

```text
timestamp
service.name
module
environment
level
trace_id
span_id
request_id
correlation_id
operation
error
duration
```

---

## Datos prohibidos

Nunca registrar:

- secretos;
- tokens;
- contraseñas;
- credenciales;
- información sensible;
- PII.

Los datos deben:

```text
sanitizarse
enmascararse
```

antes de ser registrados (no usar `#[derive(Debug)]` ingenuo sobre structs que contengan credenciales — implementar `Debug` personalizado que oculte el valor sensible).

---

# 12.1 Métricas mínimas obligatorias por capa

| Capa | Métricas mínimas |
|---|---|
| HTTP/REST (BFF) | `requests_total`, `requests_failed_total`, `request_duration`, `request_rate`, `status_codes`, `active_requests` |
| gRPC (tonic) | `calls_total`, `calls_failed_total`, `call_duration`, `status_code`, `active_calls` |
| Base de datos (sqlx) | `query_duration`, `query_errors`, `pool_size`, `active_connections`, `idle_connections`, `pool_wait_time` |
| Redis | `operations_total`, `operation_duration`, `errors`, `hit_rate`, `miss_rate`, `connection_errors` |
| NATS/JetStream | `messages_published`, `messages_consumed`, `processing_duration`, `ack_failures`, `redeliveries`, `consumer_lag`, `pending_messages`, `dlq_messages` |
| Sistema | CPU, RAM, disco, red, file descriptors, tareas `tokio` activas |

---

# 12.2 Latencia — nunca solo el promedio

Medir siempre:

```text
p50
p90
p95
p99
p99.9
```

cuando sea relevante para el SLO.

El promedio **NUNCA** debe ser la única métrica de latencia.

El promedio puede ocultar solicitudes extremadamente lentas.

Toda operación crítica debe tener objetivos de latencia documentados y parametrizables.

---

# 13. SLO / SLA / SLI y Health Checks

## 13.1 SLI / SLO / Error Budget

Para toda operación crítica definir:

### SLI

Qué se mide.

### SLO

Cuál es el objetivo.

### Error Budget

Cuánto margen de error se permite derivado del SLO.

Formato de referencia:

```text
Availability SLO: 99.9%
Latency SLO: p99 < X ms
Error rate: < X%
```

Los valores anteriores son solamente ejemplos.

### Prohibido inventar valores arbitrarios

Si el proyecto todavía no tiene objetivos definidos:

```text
dejar valores parametrizables
```

y documentarlos, por ejemplo:

```text
slo.md
```

por módulo o bounded context.

Nunca hardcodear `"99.9%"` simplemente porque parece razonable.

---

# 13.2 Health Checks

Implementar cuando corresponda:

```text
liveness
readiness
startup
```

## Liveness

Determina únicamente si el proceso está vivo.

No debe depender de servicios externos.

---

## Readiness

Determina si el servicio puede recibir tráfico.

Puede depender de dependencias críticas (PostgreSQL, Redis, NATS).

---

## Startup

Determina si el servicio ha terminado correctamente su proceso inicial de arranque (ejecución de migraciones, calentamiento de pools).

---

## Prohibición

No crear un health check que simplemente devuelva `200 OK` sin verificar el estado real necesario.

También evitar que una dependencia externa no crítica haga que todo el servicio aparezca como muerto.

Diferenciar:

```text
dependencias críticas
dependencias no críticas
```

---

# 14. Testing — obligatorio, no opcional

Mantener los siguientes niveles:

```text
Unit
Integration
Contract
Component
End-to-End
Performance
Chaos
```

Cada nivel tiene un propósito diferente.

Ningún nivel sustituye completamente a otro.

---

# 14.1 Unit Testing — Domain

El dominio debe poder probarse con `cargo test`:

```text
sin Docker
sin DB
sin Redis
sin NATS
sin red
sin infraestructura externa
```

Los tests deben ser:

- rápidos;
- deterministas;
- aislados (módulos `#[cfg(test)]`).

Probar obligatoriamente:

- invariantes;
- reglas de negocio;
- Value Objects;
- Aggregates;
- Domain Services;
- Domain Events.

---

# 14.2 Application Testing — Use Cases

Los casos de uso deben probarse usando la implementación real y sus adapters reales contra infraestructura aislada; no sustituyas puertos, repositorios o servicios por mocks, fakes, stubs o adapters en memoria.

Para reglas puras del dominio, usa pruebas unitarias sin dependencias externas. Para casos de uso con persistencia o mensajería, ejecuta PostgreSQL, Redis o NATS reales en infraestructura aislada y aplica las migraciones reales. No precargues fixtures ni datasets de ejemplo: crea registros transitorios válidos mediante los flujos reales y límpialos al finalizar.

No se permite:

```text
mock implementations
in-memory fake adapters
stubbed services
```

Los tests deben verificar:

- flujo correcto;
- manejo de errores (`Result::Err`);
- idempotencia;
- autorización cuando corresponda;
- publicación de eventos;
- comportamiento ante fallos de dependencias.

---

# 14.3 Integration Testing

Los adapters reales deben probarse contra infraestructura real o equivalente controlada.

Preferiblemente mediante:

```text
testcontainers-rs
```

para tecnologías como:

```text
PostgreSQL
Redis
NATS JetStream
```

cuando correspondan al proyecto.

No sustituyas integraciones externas por servidores HTTP simulados. Usa el sandbox oficial del proveedor y el adapter real. Si no existe sandbox o faltan credenciales, informa que la integración no pudo verificarse y no la declares completa.

Verificar:

- migrations (`sqlx migrate`);
- repositories;
- transactions (`sqlx::Transaction`);
- serialización/deserialización con `serde`;
- Redis pooling y comandos;
- publishing NATS;
- consuming NATS;
- acknowledgements (`msg.ack().await`);
- retries;
- idempotencia;
- Outbox relay;
- deduplicación/Inbox.

---

# 14.4 Contract Testing

Todo contrato externo debe tener contract tests.

Incluye, cuando exista:

```text
REST / OpenAPI (validación de schema / snapshots)
protobuf / gRPC (compatibilidad con buf breaking o tests de schemas)
NATS event schemas (tests de serde contra payloads versionados)
```

Un cambio incompatible debe detectarse automáticamente en CI antes del deployment.

Los contratos versionados no deben romper consumidores existentes.

---

# 14.5 End-to-End Testing

Los flujos críticos deben probarse siguiendo la cadena completa:

```text
Frontend / Client
       ↓
REST (axum)
       ↓
      BFF
       ↓
  gRPC (tonic)
       ↓
    Module
       ↓
Database (sqlx)
       ↓
NATS JetStream (async-nats)
```

cuando corresponda.

Los E2E deben cubrir únicamente:

```text
flujos críticos
```

No deben sustituir:

```text
unit tests
integration tests
contract tests
```

---

# 15. Rendimiento y Pruebas de Carga

Herramienta estándar para pruebas externas:

```text
k6
```

para:

- load testing;
- stress testing;
- spike testing;
- soak testing;
- performance regression testing.

Herramienta para micro-benchmarks internos de Rust:

```text
criterion
```

para funciones y algoritmos críticos del dominio.

Medir como mínimo con k6:

```text
requests/sec
throughput
latency p50
latency p95
latency p99
error rate
concurrent users/connections
```

Ejecutar siempre contra ambientes:

```text
controlados
reproducibles
```

Prohibido utilizar producción para pruebas destructivas de carga.

---

# 15.1 Perfiles de prueba de carga

## Load Test

Carga esperada normal.

## Stress Test

Incrementar progresivamente la carga hasta encontrar el límite del sistema.

## Spike Test

Incrementos bruscos de tráfico.

## Soak Test

Carga sostenida durante un período prolongado para detectar:

- memory leaks;
- connection leaks;
- degradación progresiva;
- acumulación de mensajes en NATS;
- crecimiento de latencia.

## Breakpoint Test

Determinar el punto donde:

- throughput deja de crecer;
- p99 aumenta significativamente;
- aparecen errores;
- aparecen timeouts;
- aparece saturación.

---

# 15.2 Performance Budgets

Definir budgets explícitos para:

- latencia;
- error rate;
- CPU;
- memoria;
- conexiones DB (`sqlx::Pool`);
- latencia Redis;
- consumer lag NATS.

Una modificación que empeore significativamente una métrica crítica constituye una:

```text
regresión de rendimiento
```

y debe investigarse antes de aprobarse.

Idealmente debe bloquearse el merge/deploy cuando el performance budget esté integrado al CI.

---

# 16. Resiliencia

Implementar cuando corresponda:

```text
timeouts
retries
exponential backoff
jitter
circuit breakers
bulkheads
rate limiting
backpressure
load shedding
idempotencia
dead-letter queues/streams
graceful degradation
```

## Prohibiciones

Nunca:

```text
retries infinitos
```

Nunca:

```text
retry agresivo sin backoff
```

Nunca:

```text
retry sin jitter cuando pueda producir retry storm
```

Nunca reintentar automáticamente errores no recuperables.

Por ejemplo:

```text
validation error / domain invariant error
```

no debe tratarse como un error transitorio.

---

# 16.1 Timeouts

Toda comunicación externa debe tener timeout explícito utilizando `tokio::time::timeout`.

Incluye:

```text
HTTP (axum / reqwest)
gRPC (tonic timeout)
PostgreSQL (sqlx query timeout)
Redis (connection / command timeout)
NATS request/reply
```

cuando corresponda.

Nunca depender de un timeout implícito indefinido.

Los timeouts deben ser:

```text
configurables por entorno (platform::config)
```

---

# 16.2 Retries

Los retries deben tener:

- crate recomendado: `backoff` o `tokio-retry`;
- límite máximo;
- exponential backoff;
- jitter;
- aplicación únicamente a errores recuperables (I/O transitorio, timeout de red);
- compatibilidad con idempotencia.

Evitar:

```text
retry storms
```

Un retry no debe convertir una dependencia degradada en una cascada de fallos.

---

# 16.3 Circuit Breaker

Usar circuit breaker cuando una dependencia externa pueda provocar:

```text
cascading failures
```

Crates recomendados: `failsafe` o `circuitbreaker-rs`.

Estados:

```text
CLOSED
   ↓
 OPEN
   ↓
HALF_OPEN
   ↓
CLOSED
```

Debe existir observabilidad de:

- cambios de estado;
- errores;
- tiempo abierto;
- recuperaciones.

---

# 16.4 Bulkhead

Aislar recursos críticos:

- `tokio::sync::Semaphore` para limitar concurrencia por operación;
- connection pools dimensionados por caso de uso (`sqlx::PoolOptions`, `deadpool-redis`);
- límites de tareas asíncronas.

Una operación saturada no debe consumir todos los recursos disponibles.

Una operación lenta no debe bloquear necesariamente todas las demás.

---

# 16.5 Graceful Shutdown

Todo servicio debe implementar graceful shutdown mediante `tokio::signal` (`SIGTERM`/`SIGINT`) y tokens de cancelación (`tokio_util::sync::CancellationToken`):

1. dejar de aceptar nuevo tráfico (cerrar listener axum / tonic);
2. marcar readiness como no disponible;
3. permitir terminar solicitudes activas dentro de un límite de tiempo (`tokio::time::timeout`);
4. detener consumidores NATS cancelando sus tareas `tokio::spawn`;
5. procesar/confirmar correctamente mensajes pendientes según la estrategia definida;
6. cerrar pools de conexión (`sqlx::Pool::close().await`, Redis) y liberar recursos;
7. finalizar el proceso.

Nunca terminar abruptamente una operación crítica si puede evitarse.

---

# 16.6 NATS JetStream Resilience

Los consumidores deben asumir:

```text
at-least-once delivery
```

Nunca:

```text
exactly-once
```

Los consumidores deben ser idempotentes.

Implementar:

- acknowledgements explícitos (`msg.ack().await`);
- retry policy;
- backoff;
- manejo de redelivery;
- deduplicación;
- dead-letter strategy;
- monitoreo de consumer lag expuesto a Prometheus.

Un mensaje nunca debe perderse silenciosamente.

---

# 17. Chaos Engineering y Fault Injection

Implementar pruebas de resiliencia progresivamente.

Ejemplos:

```text
detener/reiniciar un módulo o contenedor
introducir latencia con toxiproxy
introducir errores simulados
cortar Redis
cortar PostgreSQL
cortar NATS
provocar pérdida de conexiones
aumentar latencia DB
saturar CPU
saturar memoria
aumentar temporalmente tráfico con k6
```

Cada experimento de Chaos debe documentar:

```text
Hypothesis
Blast Radius
Experiment
Expected Result
Observed Result
Recovery
Conclusion
```

Nunca ejecutar experimentos destructivos sin limitar previamente:

```text
Blast Radius
```

---

## Fault Injection

Las pruebas deben validar que el sistema se comporta correctamente cuando una dependencia falla.

Ejemplos genéricos:

```text
Module → Redis DOWN
Module → Database SLOW
Module → NATS UNAVAILABLE
Module → External Service TIMEOUT
```

Ante una dependencia degradada debe existir, cuando corresponda:

```text
timeout
backpressure
concurrency limits (Semaphore)
observability (Grafana / Tempo)
recovery strategy
```

---

# 18. Alerting y Observabilidad de Extremo a Extremo

Crear alertas sobre **síntomas reales de negocio/usuario**, no únicamente sobre utilización de recursos.

Reglas definidas en **Prometheus** y enrutadas por **Alertmanager**.

Priorizar:

- error rate;
- latencia p95/p99;
- saturación;
- consumer lag de NATS;
- queue depth;
- agotamiento de conexiones DB (`sqlx::Pool`);
- fallos Redis;
- redeliveries NATS;
- OOM;
- crash loops;
- disponibilidad.

No crear alertas excesivamente sensibles que produzcan:

```text
alert fatigue
```

---

## Trace de extremo a extremo

Toda solicitud debe poder rastrearse mediante:

```text
trace_id
```

y propagación de contexto OTel a través de:

```text
Frontend
   ↓
REST (axum)
   ↓
BFF
   ↓
gRPC metadata (tonic)
   ↓
Module
   ↓
DB (sqlx) / Redis
   ↓
NATS JetStream (async-nats)
   ↓
otro Module
```

Debe ser posible investigar una operación completa en Grafana (Tempo + Loki + Prometheus) sin depender únicamente de logs manuales.

---

# 19. Definition of Done

El agente **NO debe considerar una implementación "completa" únicamente porque el código compile con `cargo build` o los tests unitarios pasen.**

Antes de dar por terminada una funcionalidad, verificar:

## Arquitectura

- [ ] El bounded context está correctamente identificado como Crate en el workspace.
- [ ] La funcionalidad vive en el módulo/crate correcto.
- [ ] No existen dependencias internas de crate entre módulos (`Cargo.toml`).
- [ ] Las dependencias apuntan hacia adentro.
- [ ] El dominio no depende de infraestructura ni crates externos de persistencia/red.
- [ ] La aplicación no depende de infraestructura concreta.
- [ ] Los adapters están en infraestructura implementando traits.
- [ ] Los contratos públicos están definidos correctamente en `module_public_api.rs`.

## SOLID

- [ ] SRP respetado: structs y funciones con una sola responsabilidad.
- [ ] OCP respetado: extensión mediante `impl Trait`.
- [ ] LSP respetado: implementaciones de traits sustituibles sin `panic!`.
- [ ] ISP respetado: traits pequeños y específicos.
- [ ] DIP respetado: inversión de dependencias con `Arc<dyn Trait>`.

## Archivos

- [ ] Ningún archivo `.rs` supera 200 líneas con código, salvo excepción técnicamente justificada y documentada; ningún archivo supera el máximo absoluto de 300 líneas con código.
- [ ] No existen archivos "bolsa" (`utils.rs`).
- [ ] Una responsabilidad pública principal por archivo.
- [ ] El nombre del archivo coincide con su export principal en `snake_case`.
- [ ] `mod.rs` o `lib.rs` solo contienen re-exports (`pub mod`, `pub use`), sin lógica.

## DDD

- [ ] Bounded Context definido.
- [ ] Lenguaje ubicuo definido.
- [ ] Entidades correctamente delimitadas con métodos de mutación explícitos.
- [ ] Value Objects implementados con Newtypes y validación en constructores.
- [ ] Agregados protegen sus invariantes.
- [ ] Domain Services utilizados solamente cuando corresponde.
- [ ] Domain Events definidos en enums para mutaciones relevantes.
- [ ] ACL aplicada cuando se consume otro contexto o contratos gRPC externos.

## Application

- [ ] Un caso de uso (struct + impl) por archivo.
- [ ] Los casos de uso solamente orquestan.
- [ ] No contienen reglas de negocio duplicadas.
- [ ] Utilizan DTOs de entrada y salida.
- [ ] No exponen entidades de dominio directamente.
- [ ] Los errores están tipados con enums y `thiserror`.
- [ ] Los consumidores externos son idempotentes.

## Database

- [ ] Todos los módulos usan la misma base física PostgreSQL.
- [ ] Cada módulo posee sus propios datos/schema.
- [ ] No existen JOINs entre módulos.
- [ ] Migraciones independientes por módulo (`sqlx migrate`) aplicadas de forma coordinada a la base compartida.
- [ ] Transacciones `sqlx::Transaction` respetan los límites del agregado.
- [ ] Outbox Pattern implementado para publicación de eventos.
- [ ] Repositorios devuelven entidades de dominio reconstituidas.
- [ ] `sqlx` y queries aisladas en infraestructura.

## Redis

- [ ] Redis está detrás de un trait `CachePort`.
- [ ] TTL definido.
- [ ] Invalidación documentada.
- [ ] Keys namespaced: `<module>:<entity>:<id>:<subresource>`.
- [ ] Redis no es fuente única de verdad.
- [ ] Locks distribuidos implementados mediante mecanismos probados (Redlock).

## NATS

- [ ] Eventos versionados con structs `Serialize`/`Deserialize`.
- [ ] Subjects correctamente nombrados en minúsculas y tiempo pasado.
- [ ] Publisher detrás de trait `EventPublisher`.
- [ ] Subscribers separados en tareas `tokio::spawn`.
- [ ] ACL aplicada al recibir mensajes externos.
- [ ] Consumers idempotentes.
- [ ] Ack manual (`msg.ack().await`) tras confirmación en DB.
- [ ] Retry policy con backoff/jitter.
- [ ] Dead-letter strategy configurada.
- [ ] Consumer lag observable en Prometheus.

## gRPC

- [ ] `.proto` versionado compilado con `tonic-build`.
- [ ] Contrato público expuesto en `infrastructure/grpc`.
- [ ] No existen imports ni dependencias internas entre crates.
- [ ] BFF conoce únicamente el contrato gRPC.
- [ ] Comunicación síncrona módulo → módulo usa gRPC vía cliente `tonic`.

## BFF

- [ ] Sin lógica de negocio.
- [ ] Sin acceso directo a DB (`sqlx`).
- [ ] Sin acceso directo a Redis del módulo.
- [ ] Sin acceso a entidades internas de dominio.
- [ ] Sin acceso a repositorios internos.
- [ ] Endpoints REST / OpenAPI (`axum` + `utoipa`) correctamente definidos.
- [ ] Errores gRPC (`tonic::Status`) correctamente adaptados a `StatusCode` HTTP.

## Observabilidad

- [ ] Logs estructurados con `tracing` (formato JSON en producción).
- [ ] No existen secretos ni PII en logs (implementación manual de `Debug` cuando corresponda).
- [ ] `trace_id` propagado a lo largo de toda la cadena.
- [ ] `correlation_id` disponible.
- [ ] Métricas por módulo emitidas a Prometheus.
- [ ] Métricas REST, gRPC, DB, Redis y NATS instrumentadas.
- [ ] Latencias p50/p90/p95/p99 medidas.

## Health

- [ ] Liveness implementado (sin dependencias externas).
- [ ] Readiness implementado (con verificación de DB, Redis, NATS).
- [ ] Startup check implementado cuando corresponde.
- [ ] Dependencias críticas diferenciadas de las no críticas.

## Testing

- [ ] Unit tests en `#[cfg(test)]` ejecutables con `cargo test`.
- [ ] Pruebas de casos de uso con adapters reales e infraestructura aislada; sin mocks, fakes, stubs ni fixtures.
- [ ] Integration tests con `testcontainers-rs` (PostgreSQL, Redis, NATS).
- [ ] Contract tests para OpenAPI y Protobuf.
- [ ] Component tests cuando corresponda.
- [ ] E2E de flujos críticos.
- [ ] Performance tests con `k6` y benchmarks con `criterion`.
- [ ] Chaos tests cuando corresponda.

## Calidad y Estilo Rust

- [ ] Pasa `cargo check` sin errores.
- [ ] Pasa `cargo clippy --all-targets -- -D warnings` sin warnings.
- [ ] Pasa `cargo fmt --check`.
- [ ] No contiene `unwrap()`, `expect()` o `panic!` en rutas de producción.
- [ ] No contiene bloques `unsafe` sin documentación justificativa.

## Performance

- [ ] Load test con k6.
- [ ] Stress test.
- [ ] Spike test.
- [ ] Soak test.
- [ ] Breakpoint test cuando corresponda.
- [ ] Performance budgets definidos.
- [ ] No se ejecutaron pruebas destructivas en producción.

## Resiliencia

- [ ] Timeouts explícitos con `tokio::time::timeout`.
- [ ] Retries limitados con backoff y jitter.
- [ ] Circuit breaker cuando corresponda.
- [ ] Bulkhead (`Semaphore` y pools dimensionados).
- [ ] Rate limiting cuando corresponda.
- [ ] Backpressure cuando corresponda.
- [ ] Idempotencia asegurada.
- [ ] Dead-letter strategy.
- [ ] Graceful shutdown con `tokio::signal` y `CancellationToken`.

---

# 19.1 Matriz de Verificación de Cobertura por Bounded Context

Cada proyecto debe mantener una matriz equivalente adaptada a sus bounded contexts/módulos; indicar el crate propietario solo cuando estén separados.

**No utilizar nombres de dominio específicos en este documento de referencia.**

Plantilla:

| Bounded Context / Módulo (crate si está separado) | Entidades de Dominio | Command Handlers | Query Handlers | Suites de Prueba |
|---|---|---|---|:---:|
| `module-a` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `module-b` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `module-c` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `module-d` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `infra/cross` | `<CrossCuttingEntity>` | `<CrossCuttingCommand>` | `<CrossCuttingQuery>` | 🟢 / 🟡 / 🔴 |

La matriz real debe contener exclusivamente los bounded contexts del proyecto actual.

No inventar módulos simplemente para llenar la tabla.

---

# 20. Prohibiciones explícitas

Antes de entregar código Rust, verifica que NO hiciste ninguna de estas cosas:

- [ ] Depender de `sqlx`, `diesel`, `sea-orm`, cliente Redis o cliente NATS dentro de `domain/` o `application/`.
- [ ] Poner lógica de negocio en un handler REST, handler gRPC, listener de evento o repositorio.
- [ ] Hacer `use` o referencia directa al crate o carpeta interna de otro módulo.
- [ ] Modificar dos agregados de módulos distintos dentro de la misma transacción síncrona.
- [ ] Modificar dos agregados del mismo módulo dentro de una misma transacción sin la justificación documentada requerida.
- [ ] Exponer una struct de entidad de dominio directamente como respuesta HTTP/REST o payload gRPC.
- [ ] Exponer una entidad de dominio directamente como payload de evento sin DTO/mapper.
- [ ] Publicar un evento sin Outbox Pattern cuando la consistencia DB/evento lo requiera.
- [ ] Crear un consumidor NATS que no sea idempotente.
- [ ] Utilizar auto-ack antes de procesar y persistir correctamente un mensaje.
- [ ] Usar Redis como única fuente de verdad de un dato crítico.
- [ ] Añadir al crate `shared-kernel` algo que en realidad pertenece a un solo módulo.
- [ ] Nombrar structs o traits genéricos como `Manager`, `Helper`, `Util` o `Service` sin responsabilidad clara y acotada.
- [ ] Generar un archivo `.rs` de más de 200 líneas con código sin excepción técnicamente justificada y documentada.
- [ ] Superar 300 líneas con código en cualquier archivo `.rs`.
- [ ] Crear un archivo "bolsa" con múltiples structs/funciones no relacionadas.
- [ ] Usar `mod.rs` o `lib.rs` para escribir lógica en lugar de re-exports limpios.
- [ ] Escribir lógica de negocio dentro de handlers REST del BFF.
- [ ] Escribir lógica de negocio dentro de cualquier archivo del BFF (`apps/bff`).
- [ ] Hacer que el BFF acceda directamente a una base de datos PostgreSQL.
- [ ] Hacer que el BFF acceda directamente a Redis de un módulo.
- [ ] Hacer que el BFF acceda directamente a entidades, agregados o repositorios internos de un módulo.
- [ ] Hacer que el BFF dependa del crate interno de un módulo (solo debe conocer el `.proto` / gRPC client).
- [ ] Comunicar el BFF con un módulo mediante un mecanismo distinto de gRPC para comunicación síncrona.
- [ ] Comunicar dos módulos de forma síncrona mediante un mecanismo distinto al servicio gRPC del módulo destino.
- [ ] Hacer que un módulo lea o modifique directamente schemas/tablas propiedad de otro módulo.
- [ ] Hacer JOINs SQL entre datos pertenecientes a módulos diferentes.
- [ ] Hacer que un módulo acceda directamente al Redis de otro módulo.
- [ ] Usar NATS como sustituto arbitrario de comunicación síncrona cuando gRPC es el contrato adecuado.
- [ ] Utilizar Core NATS para eventos que requieren persistencia y replay.
- [ ] Utilizar un evento interno de dominio directamente como contrato externo sin versionarlo con serde.
- [ ] Permitir que el modelo de otro bounded context o tipos de `prost` se filtren dentro del dominio propio.
- [ ] Usar `println!`, `eprintln!` o `dbg!` como mecanismo de observabilidad de producción.
- [ ] Registrar secretos, credenciales, tokens o PII en logs, métricas o trazas.
- [ ] Usar `unwrap()`, `expect()` o `panic!` en código de producción que maneja datos de entrada o llamadas externas.
- [ ] Usar `unsafe` sin un comentario formal que explique por qué es seguro y por qué no es posible código seguro.
- [ ] Desactivar lints de clippy con `#[allow(...)]` sin justificación explícita documentada.
- [ ] Implementar un health check que solamente devuelva `200 OK` sin verificar el estado real necesario.
- [ ] Mezclar liveness con readiness.
- [ ] Implementar retries infinitos.
- [ ] Implementar retries sin backoff/jitter cuando correspondan.
- [ ] Reintentar errores no recuperables.
- [ ] Reportar latencia usando únicamente el promedio sin percentiles (p95/p99).
- [ ] Omitir p95/p99 en operaciones donde sean relevantes.
- [ ] Ejecutar pruebas de carga destructivas directamente en producción.
- [ ] Ejecutar experimentos de Chaos sin Blast Radius definido.
- [ ] Inventar SLOs arbitrarios sin documentarlos.
- [ ] Introducir una dependencia de framework en el dominio sin justificación arquitectónica explícita.
- [ ] Permitir que una implementación de infraestructura cambie las reglas de negocio del dominio.
- [ ] Duplicar reglas de negocio entre Frontend, BFF y backend.
- [ ] Crear un módulo únicamente por conveniencia técnica si no representa un límite de negocio o responsabilidad coherente.
- [ ] Fusionar bounded contexts solamente para evitar duplicación de código.
- [ ] Crear abstracciones compartidas únicamente para evitar una pequeña duplicación.
- [ ] Utilizar `shared-kernel` como depósito general de código reutilizable.

---

# 21. Instrucción final para el agente — Claude / Codex / Copilot / cualquier agente

> Cuando generes código Rust para un proyecto que adopte esta arquitectura, aplica TODAS las reglas anteriores sin excepción.
>
> Si el requerimiento del usuario es ambiguo respecto a en qué módulo/capa debe vivir algo, pregunta o indica explícitamente tu suposición antes de generar código.
>
> Si detectas que cumplir el requerimiento tal cual lo pide el usuario violaría alguna regla de esta guía —por ejemplo:
>
> - acceder a datos de otro módulo directamente;
> - importar código interno de otro crate;
> - poner lógica de negocio en el BFF;
> - omitir observabilidad;
> - usar `unwrap()` sobre input externo o I/O;
> - omitir resiliencia en una operación crítica;
> - compartir tablas o acceder directamente a datos propiedad de otro módulo;
> - publicar eventos sin el mecanismo de consistencia requerido;
> - inventar un SLO sin base;
> - introducir dependencias de infraestructura en el dominio;
>
> debes avisarlo y proponer la alternativa arquitectónicamente correcta en vez de generar el atajo que rompe la arquitectura.
>
> El objetivo permanente es que cada módulo conserve límites y contratos que permitan una extracción futura si existe una necesidad demostrable; esa posibilidad no exige un crate ni microservicio por módulo desde el inicio.
>
> Todo el sistema debe ser:
>
> - modular (Cargo workspace);
> - desacoplado;
> - observable (OpenTelemetry OTLP gRPC → Tempo / Prometheus / Loki / Pyroscope → Grafana / Alertmanager);
> - medible (k6 + criterion);
> - testeable (`cargo test` + infraestructura aislada real mediante `testcontainers-rs`);
> - resiliente (`tokio::time::timeout`, `backoff`, `failsafe`, graceful shutdown);
> - preparado para escalar;
> - preparado para evolucionar;
> - preparado para extracción progresiva a microservicios.
>
> Estas propiedades deben existir **por diseño desde el principio**, no como una capa añadida después.

---

# 22. Regla de adaptación al proyecto real

Este documento combina principios de arquitectura reutilizables para Rust con una topología y un reparto de ownership **específicos de ENSAMBRIA**. DDD, Clean Architecture, Hexagonal, SOLID y las prácticas de límites modulares pueden servir como referencia para otros proyectos; los cuatro backends, sus dominios, contratos, tecnologías y decisiones de ENSAMBRIA no son defaults universales ni deben copiarse a otro sistema sin análisis y aprobación propios.

En ENSAMBRIA, el contrato arquitectónico, las políticas generales del proyecto y las responsabilidades están consolidados en un solo archivo raíz:

```text
ARCHITECTURE_RULES.md
```

Ese archivo es la fuente de verdad de esas tres áreas; el roadmap y los estándares especializados conservan sus respectivos ámbitos normativos. Este manual Rust conserva detalle técnico complementario, pero no es autoridad paralela para decisiones de producto/topología. No mantengas copias paralelas en `PROJECT_RULES.md` o `ARCHITECTURE_ROLES.md`.

El gobierno operativo de GitHub para ENSAMBRIA se define en [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](ENSAMBRIA_github_repository_governance_and_engineering_rules.md); las reglas de ramas, PR, revisiones, workflows y releases deben consultarse allí, junto con las obligaciones arquitectónicas específicas del contrato raíz.

Para otros proyectos que decidan mantener documentos separados:

### `ARCHITECTURE_RULES.md`

Puede contener las reglas arquitectónicas generales que el proyecto decida adoptar y sus límites compartidos:

```text
DDD
Clean Architecture
Hexagonal
SOLID
Module boundaries (crates)
gRPC (tonic)
NATS (async-nats)
Outbox Pattern
Redis (deadpool-redis)
Observability (tracing + OpenTelemetry)
SLO
Testing (cargo test + testcontainers-rs)
Performance (k6 + criterion)
Resilience (tokio timeouts, retries, circuit breakers)
Chaos Engineering
```

### `PROJECT_RULES.md`

Contiene únicamente las decisiones específicas del proyecto:

```text
Nombre del proyecto
Stack tecnológico exacto
Crates de frameworks (axum/actix-web)
ORM / DB client (sqlx)
Base de datos y motor
Frontend utilizado
Bounded contexts reales
Entidades reales
Casos de uso reales
Eventos reales
REST endpoints y schemas utoipa
gRPC contracts (.proto)
NATS subjects
SLOs reales
Performance budgets reales
Convenciones específicas
```

No mezcles en una norma reusable decisiones específicas de ENSAMBRIA ni presentes la topología de este documento como universal. Cada proyecto define y aprueba su propia topología, ownership y criterios.

---

# 23. Regla de prioridad arquitectónica

Cuando exista conflicto entre:

```text
velocidad de implementación
```

y:

```text
integridad arquitectónica
```

la integridad arquitectónica tiene prioridad.

Cuando exista una razón legítima para romper una regla:

1. identificar la regla;
2. explicar por qué no puede cumplirse;
3. documentar la excepción;
4. registrar la decisión mediante ADR (Architecture Decision Record) cuando corresponda;
5. definir el alcance de la excepción;
6. evitar que la excepción se convierta en patrón general.

Nunca convertir una excepción puntual en una nueva regla implícita.

---

# 24. Principio final

ENSAMBRIA tiene cuatro backends independientes; cada uno conserva los límites de un monolito modular:

```text
Discovery ──────────── REST/OpenAPI/WebSocket ──► BFF público independiente ──gRPC──► PUBLIC
Tenant / Business / Mobile UI ────────────────► BFF independiente por producto ───► CORE/BUSINESS/PLATFORM/PUBLIC
Admin UI ───────────── REST/OpenAPI/WebSocket ──► BFF PLATFORM integrado ─────────► PLATFORM

CORE ◄──────────── gRPC síncrono si aplica ────────────► BUSINESS / PLATFORM / PUBLIC
CORE / BUSINESS / PLATFORM / PUBLIC ── Outbox ── NATS JetStream ── consumidores autorizados

┌───────────────────┐  ┌────────────────────┐
│ CORE               │  │ BUSINESS            │
│ identidad · pagos │  │ Commerce · operación │
│ reservas · ledger  │  │ workforce · factura │
└───────────────────┘  └────────────────────┘

┌───────────────────┐  ┌────────────────────┐
│ PLATFORM           │  │ PUBLIC               │
│ tenants SaaS       │  │ Discovery            │
│ planes · políticas │  │ proyecciones · canal │
└───────────────────┘  └────────────────────┘

CORE ── adapters de pagos ──► proveedores aprobados
PLATFORM / BUSINESS ── sus respectivas integraciones y facturación

Workspace Cargo raíz compartido para build; cuatro artefactos/configuraciones/
despliegues de backend separados. PostgreSQL puede ser físicamente compartido;
ownership, schemas, migraciones y permisos permanecen separados.
```

CORE posee procesamiento de pagos y ledger; BUSINESS posee las ventas y la operación de los establecimientos; PLATFORM posee la relación comercial SaaS y sus políticas; PUBLIC posee el canal de descubrimiento y atribución, y solo publica datos expresamente autorizados. Las cuentas, credenciales, callbacks, auditoría e historial de cada integración se separan por owner y propósito. No se asignan reglas comerciales de un backend a otro.

El repositorio mantiene un Cargo workspace único; cada backend conserva composition root, configuración, permisos, CI/deploy targets y despliegue independientes. Los módulos internos se comunican mediante contratos públicos gRPC para consultas/comandos síncronos y Outbox + NATS JetStream para hechos persistidos que requieren entrega durable. NATS Core se reserva al fan-out realtime efímero; Redis, a caché con TTL. No se extraen módulos a procesos independientes sin necesidad demostrada y decisión arquitectónica. Los frontends son aplicaciones independientes y consumen solo el BFF/superficie autorizados; CORE nunca se expone directamente a frontends.

La arquitectura permite evolucionar contratos y despliegues sin compartir código interno ni propiedad de datos entre backends. PostgreSQL puede ser infraestructura física compartida conforme a la decisión ENSAMBRIA, pero cada módulo conserva propiedad exclusiva de sus schemas, tablas y migraciones.

**El objetivo no es fusionar los cuatro backends ni desplegar cada módulo por separado.** El objetivo es mantener cuatro límites de backend claros, con un monolito modular dentro de cada límite y contratos seguros entre ellos.

---

# FIN DEL CONTRATO DE ARQUITECTURA

## Higiene documental del repositorio

Está prohibido crear archivos o documentos Markdown innecesarios, duplicados o sin un propósito verificable dentro de la arquitectura, un requisito o un entregable. Reutiliza y actualiza la documentación existente cuando corresponda. El `README.md` principal debe limitarse a presentar ENSAMBRIA, comunicar su estado real, resumir la arquitectura y las capacidades pertinentes, ofrecer requisitos/instrucciones que hayan sido verificadas y enlazar la documentación detallada. No debe acumular contratos extensos, detalles internos que pertenecen a otros documentos, contenido especulativo ni información que no sea necesaria para orientar sobre el proyecto.

## Flujo de ramas y revisión

Desde Fase 0, GitHub es el centro de calidad, seguridad, colaboración, trazabilidad y entrega. `main` debe protegerse con Ruleset/Branch Protection; todo cambio entra por Pull Request y no se permiten push directo, force-push ni eliminación. Se requieren checks obligatorios verdes, resolución de conversaciones y rama actualizada con `main` o merge queue equivalente. Las limitaciones de permisos o del plan de GitHub se registran como bloqueos, no como protección cumplida.

Usa ramas cortas por unidad coherente (`feat/`, `fix/`, `refactor/`, `test/`, `docs/`, `security/`, `infra/`), sin dividir por archivo ni mezclar objetivos ajenos. Cuando haya varios colaboradores, exige aprobación independiente; cambios críticos de seguridad, permisos, infraestructura o workflows requieren revisión de una persona distinta al autor. No simules revisión humana con aprobación automatizada.

Todo check obligatorio de Actions se ejecuta localmente con el mismo comando, configuración y criterio; CI invoca el mismo punto de entrada versionado. No ocultes fallos ni omitas pruebas. La Fase 0 establece CODEOWNERS, plantillas, CI base, scripts reproducibles, mínimo privilegio de workflows, Actions fijadas a SHA, timeouts, concurrencia segura y separación de PR, `main`, escaneos programados, releases y despliegues. Un PR no despliega producción. La matriz de validaciones y comandos debe corresponder a manifests reales; una suite fallida, pendiente, cancelada, omitida o no ejecutada no es aprobación. Tras integrar, verifica CI en el commit exacto y exige evidencia requisito→caso→prueba→ejecución para cerrar una fase.
