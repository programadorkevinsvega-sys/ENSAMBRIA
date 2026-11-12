# Facturación electrónica SaaS

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/electronic-invoicing/`.
- **Estado:** diseño documentado; no hay implementación funcional ni proveedor configurado.
- **Fase prevista:** Fase 7.

## Propósito y responsabilidad

Transmitir y dar seguimiento técnico a documentos SaaS autorizados mediante un adapter de proveedor aislado. No es propietario de la factura comercial ni de las obligaciones fiscales.

## Operaciones previstas

Recibir por contrato una solicitud referenciada a [`saas-invoices`](../saas-invoices/README.md), transmitirla cuando el proveedor y el emisor estén validados y devolver el resultado técnico correspondiente. La documentación oficial vigente de Factus y los requisitos aplicables se verifican durante la implementación.

## Límites e integraciones

- Trabaja únicamente con documentos y configuración del emisor SaaS de PLATFORM.
- No comparte credenciales, callbacks, documentos o historial con la integración Factus de BUSINESS.
- No procesa pagos ni modifica el ledger de CORE.
- No declara soporte de proveedor, endpoints, estados o conformidad DIAN antes de su verificación.

## Seguridad e integridad

La autorización valida workload, tenant y emisor. Las credenciales se obtienen mediante la custodia aprobada y no se almacenan en README, eventos o logs. Solicitudes, callbacks y reintentos requieren correlación e idempotencia; estados confirmados no se reemplazan por respuestas obsoletas.

La aceptación futura cubre rechazo/timeout, callback inválido, duplicado o fuera de orden, documento de otro emisor, redacción y recuperación.

## Implementación y pruebas

Solo existe la estructura de capas. No hay adapter, credenciales, callback o emisión implementados; no se documentan comandos de runtime.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [ADR-0007: separación de contextos SaaS](../../../../docs/adr/ADR-0007-separacion-contextos-facturacion-saas.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
