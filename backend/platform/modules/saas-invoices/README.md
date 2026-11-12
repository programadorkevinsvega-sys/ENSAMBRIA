# Facturas SaaS

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/saas-invoices/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** Fase 7.

## Propósito y responsabilidad

Mantener el documento comercial de factura SaaS asociado a una obligación válida y su ciclo de vida, separado de la transmisión fiscal electrónica.

## Operaciones previstas

Registrar y consultar una factura y solicitar su envío o corrección autorizada mediante un contrato con [`electronic-invoicing`](../electronic-invoicing/README.md). Las notas crédito/débito cuando correspondan, correcciones, anulaciones y datos fiscales requieren reglas aplicables definidas antes de implementar.

## Límites e integraciones

- Consume referencias de obligaciones de [`saas-billing`](../saas-billing/README.md).
- Mantiene los documentos SaaS de PLATFORM; las facturas de cada establecimiento pertenecen a BUSINESS.
- No ejecuta pagos, escribe ledger ni accede directamente al proveedor fiscal.
- No presume requisitos, estados ni cumplimiento tributario.

## Seguridad e integridad

Cada operación valida tenant y rol financiero. Las filas tenant-scoped requieren RLS y contexto transaccional conforme a [ADR-0001](../../../../docs/adr/ADR-0001-base-postgresql-compartida.md). La creación y las correcciones deben ser idempotentes, auditables y evitar duplicar documentos por reintentos.

La aceptación futura cubre obligación inexistente, factura duplicada, datos del emisor incompletos, tenant cruzado, corrección concurrente y estados externos retrasados.

## Implementación y pruebas

Solo existe la estructura de capas. No hay documentos, emisión o integración implementados; no se documentan comandos de runtime.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [ADR-0007: separación de contextos SaaS](../../../../docs/adr/ADR-0007-separacion-contextos-facturacion-saas.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
