# ADR-0014: Coordinación idempotente de ventas, pagos e inventario

- **Estado:** diseño aceptado expresamente por el propietario el 2026-11-18; implementación futura
- **Fecha:** 2026-11-18
- **Ámbito:** ventas/POS, solicitud y resultado de cobro, inventario y vínculo de documentos fiscales
- **Requisitos:** [requerimiento §§11, 14 y 16; Fases 4, 7 y 9](../requerimiento.md)
- **Fuentes:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [ADR-0001](ADR-0001-base-postgresql-compartida.md), [ADR-0002](ADR-0002-outbox-alta-tenant-core-platform.md), [ADR-0009](ADR-0009-proveedores-iniciales-y-reventa-factus.md), [reglas PostgreSQL](../rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md), [reglas Outbox/Inbox](../rules/ENSAMBRIA_Transactional_Outbox_Inbox.md)

## Contexto

Una venta puede requerir inventario, solicitud de cobro y eventualmente un documento fiscal, pero estos efectos tienen owners distintos. BUSINESS posee ventas/Commerce, proveedores y movimientos de inventario; CORE posee solicitudes de pago, transacciones financieras y ledger; cada owner conserva sus propios registros y transacciones. La coordinación debe sobrevivir a duplicados, reintentos, concurrencia, caída de proveedor y recuperación sin acceso cruzado a tablas ni una transacción SQL distribuida.

## Decisión

1. Una venta/POS se registra bajo BUSINESS con tenant, producto validado, actor, scopes aplicables y clave idempotente única por operación. El mismo reintento conserva la clave y no crea una venta/cobro duplicado; una clave usada para una intención incompatible se rechaza.
2. Cuando el caso requiere cobro, BUSINESS registra su estado/solicitud local y el Outbox en una sola transacción. CORE recibe el comando/evento conforme al contrato aprobado, registra Inbox y una única operación financiera idempotente en su propia transacción. BUSINESS no escribe ni duplica el ledger.
3. CORE invoca al proveedor desde su adapter **fuera de cualquier transacción SQL abierta**. Persiste el resultado validado bajo su owner y publica un hecho versionado por Outbox si el resultado debe propagarse duraderamente. BUSINESS procesa mediante Inbox idempotente y actualiza la venta sin crear una segunda operación financiera.
4. Los cambios de venta e inventario se coordinan en BUSINESS mediante los contratos públicos de sus módulos propietarios; no hay acceso SQL entre módulos. Si requieren propagación durable, usan Outbox/Inbox local conforme a la guía. Cada movimiento de inventario es trazable, tenant-scoped, idempotente y queda bajo BUSINESS.
5. Inventario, venta y pago mantienen estados explícitos. Las reglas de reserva/consumo/reversión, el instante de compromiso, compensaciones y reconciliación se detallan por caso de uso antes de la implementación; no se supone una atomicidad global ni se ocultan estados inciertos. Un fallo de un owner revierte su transacción local y recupera/compensa efectos ya confirmados mediante operaciones idempotentes autorizadas.
6. Las referencias monetarias conservan moneda explícita. PostgreSQL almacena cantidades como `NUMERIC(p,s)` con precisión/escala definida; contratos pueden transportar unidades menores como entero solo si la conversión es exacta conforme a moneda/escala. Nunca se usa punto flotante.
7. Los documentos fiscales se vinculan al registro de venta/obligación validado y conservan el snapshot del emisor y datos efectivamente usados en el momento de emisión. Cambios posteriores de emisor no reescriben documentos emitidos. La emisión y sus requisitos permanecen en Fase 7 y bajo su owner.
8. Cada owner registra auditoría transaccional pertinente. Las políticas de retención, conservación o supresión siguen PDLM y requisitos aplicables; los estados/eliminación lógica no reemplazan esas políticas.

### Flujo y responsabilidades

| Paso | Owner | Efecto local | Integración y recuperación |
|---|---|---|---|
| Crear venta/POS | BUSINESS | Valida permiso, alcance, precio/estado y clave idempotente; registra venta y, si aplica, solicitud de cobro con Outbox en una transacción local. | Publica el comando durable solo si otro owner debe procesarlo; replay no duplica la venta ni la solicitud. |
| Registrar solicitud de pago | CORE | Inbox deduplicado y registro de una operación financiera bajo idempotency key estable. | El proveedor se llama fuera de la transacción SQL; timeout/resultado incierto mantiene estado recuperable y reconciliable. |
| Registrar resultado de pago | CORE | Valida callback/respuesta del adapter, actualiza el estado financiero y ledger según reglas autorizadas. | Outbox versionado propaga el resultado confirmado; consumidores deduplican y confirman ACK tras commit. |
| Aplicar el resultado a la venta | BUSINESS | Inbox deduplicado aplica la transición permitida al registro de venta. | Duplicado, fuera de orden o evento incompatible se maneja mediante versión/estado y reconciliación, sin segundo cargo. |
| Coordinar inventario | BUSINESS | Registra movimiento trazable en su módulo propietario con idempotencia y límites de scope. | Coordinación por contrato interno del monolito; Outbox/Inbox cuando el flujo requiere efecto durable desacoplado. |
| Emitir documento fiscal del establecimiento | BUSINESS | Crea el documento desde la obligación/venta validada y conserva el emisor original. | Adapter y credenciales aislados por owner/emisor; aplica requisitos de Fase 7 y ADR-0009/0012. PLATFORM conserva por separado los documentos fiscales propios de ENSAMBRIA SaaS; no participa en el flujo fiscal de ventas del establecimiento. |

La integración entre backends es CORE↔BUSINESS y usa contratos versionados; gRPC resuelve consultas/comandos que requieren resultado inmediato y Outbox + NATS JetStream distribuye hechos ya persistidos que requieren entrega durable. No se usa un evento como RPC, ni gRPC como sustituto de recuperación durable. El BFF no accede a PostgreSQL, Redis o NATS.

## Alternativas consideradas

1. Una transacción distribuida o two-phase commit entre CORE y BUSINESS: descartada por el límite de propiedad y por la exigencia de transacciones locales.
2. Llamar al proveedor de pago dentro de una transacción SQL: descartada porque retendría conexiones/locks durante una dependencia externa y no resuelve resultado incierto.
3. Hacer que BUSINESS escriba el ledger o CORE sea dueño de las ventas/inventario: descartada porque cruza ownership.
4. Tratar un cobro como exactamente una vez o confiar solo en la UI para evitar duplicados: descartada; se asume entrega al menos una vez y cada owner aplica idempotencia.
5. Coordinar cambios solo con una llamada síncrona sin registro recuperable: descartada para cualquier hecho persistido que requiera procesamiento durable.

## Consecuencias, fases y aceptación

- Fase 4.1 entrega POS/caja/pagos y operaciones locales con autorización, idempotencia, conciliación de estados y trazabilidad según el owner.
- Fase 4.2 entrega inventario/compras/proveedores y movimientos trazables; BUSINESS coordina el efecto sobre venta conforme a las reglas definidas.
- Fase 7.1 integra Wompi inicialmente para pagos con sandbox/documentación oficial, callbacks seguros y reconciliación; una integración no se considera validada sin dependencias reales apropiadas.
- Fase 7.2 vincula facturación electrónica al emisor; los datos mínimos fiscales pendientes bloquean solo la emisión que depende de ellos.
- Fase 9.3 aplica la misma coordinación a Commerce/órdenes; Marketplace no duplica lógica ni ownership.
- Los escenarios de aceptación futura incluyen importes inválidos/cero/límites según el caso, clave duplicada con el mismo payload, reutilización incompatible, solicitudes concurrentes, permiso/scope insuficiente, sesión expirada, fallo/timeout/callback inválido del proveedor, reinicio/replay/doble entrega, rollback local, inventario insuficiente/concurrente, evento fuera de orden, resultado tardío y ausencia de efectos financieros o de stock duplicados.
- Las pruebas de integración requieren PostgreSQL, JetStream y proveedor sandbox/entorno oficial según la frontera afectada; mocks no sustituyen esa evidencia. Para Redis, solo aplica si el caso lo usa como caché TTL, nunca como autoridad de coordinación. Los tests destructivos de reinicio quedan limitados a servicios de desarrollo permitidos por las reglas del proyecto.
- Este ADR documenta diseño y aceptación futura; no afirma código, contratos ejecutables, migraciones, eventos, integración ni pruebas implementadas.

## Relación con otras decisiones

- **ADR-0001:** gobierna persistencia, precisión, RLS, credenciales, transacciones y ownership; esta ADR concreta las invariantes del recorrido sin cambiar la topología.
- **ADR-0002:** conserva el patrón Outbox/Inbox y garantías de entrega; no sustituye el flujo de alta CORE→PLATFORM.
- **ADR-0009:** CORE procesa pagos; BUSINESS y PLATFORM conservan facturación por emisor; Wompi/Factus son proveedores iniciales previstos, no integraciones validadas.
- **ADR-0012:** la jerarquía y la resolución fiscal permanecen; los documentos conservan el emisor usado y los scopes son verificados por owner.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios](../arquitectura-dominios.md)
- [ADR-0001](ADR-0001-base-postgresql-compartida.md)
- [ADR-0002](ADR-0002-outbox-alta-tenant-core-platform.md)
- [ADR-0009](ADR-0009-proveedores-iniciales-y-reventa-factus.md)
- [ADR-0012](ADR-0012-jerarquia-empresarial-contexto-operativo.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
