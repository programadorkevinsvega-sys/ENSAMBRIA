# ADR-0007: Separación de contextos comerciales y fiscales SaaS en PLATFORM

- **Estado:** diseño aceptado expresamente por el propietario el 2026-11-11; implementación reservada a las fases del roadmap
- **Fecha:** 2026-11-11
- **Ámbito:** bounded contexts de PLATFORM relacionados con oferta, suscripción, cobro SaaS y documentos electrónicos
- **Requisitos:** [requerimiento §§2.1, 5, 11.1–11.3 y Fases 1, 5 y 7](../requerimiento.md)
- **Fuentes arquitectónicas:** [contrato de arquitectura](../../ARCHITECTURE_RULES.md), [modelo de dominios](../arquitectura-dominios.md#23-platform--backend-platform)

## Contexto

El diseño previo agrupaba planes, suscripciones y facturación SaaS en un contexto y describía otra carpeta para facturación, créditos y revendedores. El requerimiento distingue la gestión comercial de PLATFORM, el procesamiento financiero de CORE y la facturación de cada establecimiento en BUSINESS; también exige dos contextos Factus aislados: uno SaaS de PLATFORM y otro de cada establecimiento en BUSINESS. Mantener los conceptos agrupados oscurecía sus límites de datos, contratos y adaptación fiscal.

Esta decisión desglosa el diseño en cuatro bounded contexts de PLATFORM, sin crear un quinto backend, crates ficticios, procesos independientes ni reglas comerciales o fiscales no definidas por las fuentes. Los contextos son límites de ownership dentro del monolito modular de PLATFORM, no una indicación de implementación actual.

## Decisión

1. **Políticas comerciales y monetización** conserva el catálogo/versionado de ofertas, planes y precios SaaS, además de las políticas de monetización de Marketplace y tarifas comerciales. Los demás contextos consumen identificadores/versiones por contrato; no copian ni inventan las reglas de oferta.
2. **`saas-subscriptions`** posee el vínculo contractual de un tenant con una oferta aprobada, sus referencias/versiones y su ciclo de vida completo. Aplica reglas comerciales aprobadas, pero no decide planes, trials, precios, renovación o elegibilidad por sí mismo.
3. **`saas-billing`** posee la obligación/periodo de facturación y sus cálculos versionados, y solicita a CORE el efecto financiero mediante contrato autorizado. No posee procesamiento de pago, movimiento de fondos ni ledger.
4. **`saas-invoices`** posee el documento comercial de factura asociado a una obligación y su ciclo de vida documental. Es distinto de la transmisión fiscal electrónica; las notas crédito/débito cuando correspondan, correcciones y anulaciones se implementan únicamente con reglas y autorizaciones definidas.
5. **`electronic-invoicing`** posee la coordinación técnica de transmisión y seguimiento de documentos SaaS mediante un adapter de proveedor aislado. Consume y devuelve referencias/estados por contrato con `saas-invoices`; no redefine la factura comercial, la obligación ni los requisitos tributarios.
6. CORE conserva la ejecución de pagos/cargos autorizados y el ledger, conforme a su ownership. BUSINESS conserva las facturas, credenciales, callbacks e historial fiscal de cada establecimiento. La integración SaaS de PLATFORM no comparte configuración, credenciales, callbacks, documentos ni historial con BUSINESS.
7. Los cuatro contextos son módulos lógicos de PLATFORM en `backend/platform/modules/saas-subscriptions`, `backend/platform/modules/saas-billing`, `backend/platform/modules/saas-invoices` y `backend/platform/modules/electronic-invoicing`. Cada uno mantiene ownership de sus modelos/datos y consume otros por APIs públicas internas/contratos; no se permiten imports de implementación ni acceso cruzado a tablas.
8. La implementación separa las fases: catálogo versionado de ofertas/planes y ciclo contractual completo de suscripciones en Fase 1; consola administrativa global en Fase 5; billing, cobros SaaS, facturas e integración electrónica en Fase 7. CORE conserva los pagos y el ledger. Esta secuencia no autoriza la asignación automática de oferta, suscripción, cobro o entitlement durante onboarding.

**Decisión expresa del propietario — 2026-11-11:** el alcance funcional completo del ciclo contractual de suscripción se implementa en Fase 1, no solo su diseño inicial. La decisión cubre la fase y capacidad; no aprueba por sí sola precios, moneda, periodicidad, pruebas, reglas de renovación/cancelación, elegibilidad, textos contractuales ni obligaciones legales. Esos términos requieren aprobación de Producto/Finanzas y revisión legal aplicable antes de habilitar cada comportamiento.

El diseño y la matriz completa de casos de uso, datos, agregados, actores, contratos, eventos, dependencias, autorización, integridad, pruebas y fases están en [§2.3 del modelo de dominios](../arquitectura-dominios.md#23-platform--backend-platform).

## Límites aún no definidos

- Product/Finanzas debe definir y aprobar ofertas, precios, moneda, periodicidad, pruebas, renovaciones, elegibilidad y efectos contractuales antes de activar el comportamiento correspondiente. La capacidad del ciclo completo se implementa en Fase 1; las reglas no aprobadas permanecen deshabilitadas.
- Los requisitos fiscales, responsabilidades del emisor, datos mínimos y reglas de documentos se verifican para la jurisdicción y el tratamiento aplicables; no se interpreta ni declara cumplimiento legal con este ADR.
- La documentación oficial de Factus y sus capacidades se verifican al implementar el adapter. No se declaran endpoints, webhooks, estados ni garantías de proveedor que no estén confirmados.
- Contratos, esquemas, eventos, estados, retención, recuperación y pruebas ejecutables se especifican con la implementación de cada fase. No se inventan nombres de eventos ni se publica un evento sin consumidor durable identificado.
- Créditos comerciales/financieros no se incorporan a estos cuatro contextos por inferencia. Son distintos de las notas crédito/débito documentales, que el requerimiento contempla cuando correspondan. La decisión posterior [ADR-0009](ADR-0009-proveedores-iniciales-y-reventa-factus.md) aprueba comercializar capacidad de facturación Factus con ownership de PLATFORM y preservando la facturación por emisor de BUSINESS; no introduce créditos ni saldos prepagados por inferencia.

Estas pendientes no bloquean la separación arquitectónica ni las actividades independientes. Solo el comportamiento que dependa de una regla comercial, requisito fiscal o aprobación profesional previa queda deshabilitado hasta resolver esa dependencia concreta.

## Alternativas consideradas

1. Mantener un solo contexto para planes, suscripciones, obligaciones y facturas: descartada porque mezcla ciclos de vida, transacciones y contratos distintos.
2. Agrupar Factus con las facturas SaaS: descartada porque la transmisión fiscal y el documento comercial son responsabilidades separables; el proveedor debe quedar detrás de un adapter y no ser dueño de las reglas de factura.
3. Crear un módulo de planes separado: no elegido para esta división de cuatro contextos; las ofertas/planes y precios permanecen en el contexto ya definido de políticas comerciales y monetización.
4. Mover pagos o ledger a PLATFORM junto con billing: descartada; contradice el ownership de CORE establecido por el requerimiento.
5. Compartir la integración Factus entre PLATFORM y BUSINESS: descartada; contradice la separación de emisores, credenciales, callbacks, documentos e historial.
6. Crear un backend o crate por cada contexto: descartada; el diseño conserva cuatro backends modulares y no requiere un paquete separado sin necesidad demostrable y código funcional.

## Consecuencias y criterios de aceptación

- La matriz de dominios y los README de módulos distinguen claramente oferta/política, contrato de suscripción, obligación, factura y transmisión electrónica.
- Las obligaciones de facturación y facturas tenant-scoped deben aplicar RLS, autorización, grants mínimos y contexto transaccional conforme a ADR-0001; tablas globales de catálogo requieren justificación explícita de su alcance.
- Los importes solo se procesan con moneda/política aprobadas y versionadas; la petición a CORE es idempotente y PLATFORM no puede escribir el ledger.
- El flujo electrónico solo utiliza credenciales/callbacks/datos del emisor de PLATFORM. Las pruebas deben rechazar referencias de BUSINESS y viceversa, además de repetición, replay, callbacks inválidos/fuera de orden, fallo/reintento y efectos parciales.
- La aceptación Fase 7 requiere PostgreSQL, JetStream si aplica a los hechos durables, y el entorno oficial de prueba del proveedor cuando corresponda; documentos, interfaces, mocks o matrices por sí solos no son evidencia de integración.
- La aceptación Fase 1 del ciclo de suscripción requiere probar con dependencias reales el flujo HTTP/BFF y las operaciones/transiciones aprobadas, autorización/aislamiento, versionado, concurrencia, idempotencia, ausencia de asignación automática al alta, propagación autorizada de entitlements a CORE y sincronización realtime con recuperación por cursor/snapshot. No incluye cobros ni facturación SaaS, que se aceptan en Fase 7.
- El diseño no declara runtime, proveedor configurado, emisión fiscal aceptada ni aprobación jurídica/comercial.

## Impacto

- **Seguridad y privacidad:** autorización por tenant/actor/acción, aislamiento RLS donde corresponda, mínimo privilegio y separación de datos/configuración por emisor. No registrar credenciales, payloads fiscales ni datos personales innecesarios.
- **Rendimiento y resiliencia:** los contratos evitan consultas cruzadas; la implementación debe definir límites, reintentos, idempotencia y recuperación según las guías de persistencia y mensajería aplicables.
- **Operación:** PLATFORM requiere observabilidad y auditoría de sus estados; callbacks y credenciales se mantienen por integración/uso. Un fallo del proveedor no debe alterar el ledger ni crear documentos duplicados.
- **Componentes afectados:** `backend/platform/modules`, `backend/core` como owner de pagos, `backend/business` para su facturación independiente, la matriz de dominios y las pruebas/contratos de Fase 7.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios y estructura objetivo](../arquitectura-dominios.md)
- [ADR-0001: base PostgreSQL compartida](ADR-0001-base-postgresql-compartida.md)
- [ADR-0002: onboarding CORE→PLATFORM](ADR-0002-outbox-alta-tenant-core-platform.md)
- [Guía de Outbox/Inbox](../rules/ENSAMBRIA_Transactional_Outbox_Inbox.md)
- [Guía de pooling PostgreSQL](../rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md)
