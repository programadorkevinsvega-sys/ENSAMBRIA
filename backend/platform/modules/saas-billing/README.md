# Billing SaaS

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/saas-billing/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** Fase 7 para obligaciones de billing e integraciones.

## Propósito y responsabilidad

Calcular y registrar obligaciones SaaS conforme a una suscripción y políticas comerciales aprobadas/versionadas, y solicitar el efecto financiero al backend propietario.

## Operaciones previstas

Calcular/cerrar una obligación, solicitar a CORE una operación financiera autorizada y consultar el estado resultante. No se inventan importes, moneda, periodicidad, tributos ni condiciones.

## Límites e integraciones

- Consulta [`saas-subscriptions`](../saas-subscriptions/README.md) y [`commercial-policies-monetization`](../commercial-policies-monetization/README.md) por contratos.
- Solicita pagos a CORE; CORE conserva procesamiento de pagos, cargos y ledger.
- No posee el documento de factura SaaS, cuya gestión pertenece a [`saas-invoices`](../saas-invoices/README.md).
- No consulta tablas CORE/BUSINESS ni mantiene un ledger paralelo.

## Seguridad e integridad

La autorización debe validar actor, tenant y operación; los datos tenant-scoped requieren RLS y contexto transaccional conforme a [ADR-0001](../../../../docs/adr/ADR-0001-base-postgresql-compartida.md). La obligación y la solicitud financiera deben ser idempotentes y reconciliables; PLATFORM no registra movimientos de dinero.

La aceptación futura cubre política ausente/no aprobada, importe o moneda inválidos, pago rechazado/timeout, duplicados, replay, tenant ajeno y ausencia de efectos financieros parciales.

## Implementación y pruebas

Solo existe la estructura de capas. No hay cálculo, cobro ni integración ejecutables; no se documentan comandos de runtime.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [ADR-0007: separación de contextos SaaS](../../../../docs/adr/ADR-0007-separacion-contextos-facturacion-saas.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
