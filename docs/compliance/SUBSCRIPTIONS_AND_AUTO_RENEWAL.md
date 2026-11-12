# Diseño inicial de suscripciones, renovaciones y cancelación

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** borrador no contractual ni flujo de cobro implementado. El propietario aprobó que el catálogo y el ciclo contractual completo de suscripción se implementen en Fase 1; esta aprobación de alcance no aprueba planes concretos, precios, moneda, período de prueba, renovación automática, reembolso ni otros términos. Cada regla debe validarse con Producto, Finanzas y asesoría legal del mercado antes de activarse.

## Ownership

- **PLATFORM** es dueño de planes, relación comercial SaaS, suscripción, factura SaaS, renovación/cancelación y política comercial de ENSAMBRIA.
- **BUSINESS** es dueño de membresías, paquetes, pedidos, ventas y suscripciones comerciales de cada establecimiento.
- **CORE** posee validación/ejecución autorizada del efecto financiero, pagos, ledger, liquidación y conciliación; no fija unilateralmente precio ni política comercial.
- El proveedor de pago no pasa a ser owner de reglas; se conecta mediante adapter y contrato aprobado.

## Información previa a contratar

La superficie del owner debe presentar de forma legible producto/plan, precio y moneda, periodicidad, fecha del próximo cargo, impuestos o cargos cuando se hayan determinado, condiciones de renovación, cancelación, elegibilidad, limitaciones y canal de soporte. Conservar evidencia versionada de oferta, condiciones, actor autorizado y acción de confirmación. No inferir aceptación desde una casilla genérica.

## Renovación y baja

- La renovación debe tener estado y próxima fecha explícitos, no inferidos por la interfaz.
- Las obligaciones de aviso previo, consentimiento, confirmación, revocación del medio de pago y facilidad/canal de cancelación se determinan por ley y contrato antes de su configuración.
- La cancelación debe ser una transición autorizada e idempotente; confirmar el resultado, tratar carreras con renovación/cargo, y no ocultar una vía de baja equivalente a la de contratación cuando sea aplicable.
- Reembolsos, períodos de gracia, precios, trial y política de mora requieren decisiones de negocio y no se inventan en esta ADR/matriz.
- CORE no permite cobros duplicados ante request repetido; los efectos de proveedor usan idempotencia y conciliación, con Inbox/Outbox según necesidad durable.
- Separar documentos fiscales y comerciales de los datos de clientes/CRM de BUSINESS; no compartir tablas entre backends.

## Aceptación futura

Probar precio/moneda ausente o inválida, oferta vencida, usuario sin autoridad, duplicado/concurrencia, cancelación y renovación simultáneas, cargo rechazado/timeout, confirmación duplicada, refund/reverso, factura pendiente, cambio de precio, baja confirmada y datos tenant ajenos. Las pruebas financieras usan sandbox oficial, no transferencias reales; deben revisar estado y ledger postoperación, no solo el HTTP status.

Mercado, avisos, formularios, consentimiento, términos, reembolsos e impuestos se aprueban por separado en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md). Hasta entonces no activar auto-renewal ni cobros comerciales.
