# Catálogo y configuración comercial de productos

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/product-catalog-configuration/`.
- **Estado:** diseño documentado; implementación aún no iniciada.
- **Fase prevista:** configuración mínima antes del alta autoservicio de tenants de Fase 1; gestión comercial completa en la fase PLATFORM correspondiente.

## Propósito y responsabilidad

Mantener el catálogo canónico de productos y verticales registrados, sus versiones de configuración comercial, referencias de marca/dominios, mercados y capacidades declaradas. PLATFORM no almacena datos operativos de tenants ni secretos.

## Operaciones previstas

Registrar, actualizar o desactivar productos; consultar versión, dominio asociado, verticales/capacidades declarados y locales/mercados autorizados. Las respuestas son versionadas y minimizadas para consumidores autorizados.

## Límites e integraciones

- CORE valida el producto y mantiene su asociación técnica con el tenant.
- PUBLIC recibe solo proyecciones expresamente autorizadas.
- BFFs consultan mediante contratos autorizados; no acceden directamente a stores.
- No administra CRM, pagos/ledger ni reglas operativas de verticales.
- Los cambios persistidos solo generan eventos durables si hay consumidores que los requieran.

## Seguridad e integridad

Las acciones administrativas requieren rol privilegiado, auditoría y MFA según la política aplicable. Un dominio no autentica ni autoriza. Configuración versionada controla concurrencia; un producto inactivo, dominio duplicado/no verificado o capacidad no implementada no debe habilitarse.

La aceptación futura cubre ausencia/inactividad, dominio duplicado, versión obsoleta, cambio concurrente/replay y ausencia de secretos o PII en la salida. El catálogo no crea tenants ni contratos SaaS por sí mismo.

## Implementación y pruebas

Solo existe la estructura de capas; no hay catálogo, contratos ejecutables ni pruebas. No se declara disponible para runtime.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
