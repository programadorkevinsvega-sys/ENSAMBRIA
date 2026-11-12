# Tenancy, membresía, organización, sucursal y asociación de producto

## Identificación y estado

- **Tipo:** bounded context de CORE.
- **Ubicación:** `backend/core/modules/tenancy-membership-organization-branch-product/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** base técnica y asociación inicial con el primer alta de tenant de Fase 1.

## Propósito y responsabilidad

Mantener el aislamiento técnico y las relaciones de pertenencia que determinan quién puede actuar, en qué tenant, sucursal y producto validado. CORE es propietario de tenants, membresías, organizaciones, sucursales y de la asociación técnica tenant–producto.

La gestión comercial de tenants, sus planes y suscripciones pertenece a PLATFORM.

## Operaciones previstas

Crear un tenant asociado a un producto registrado y activo; asignar o revocar membresías; y consultar el producto, scope y estructura autorizados. El alta de tenant no implica plan, precio, cobro, trial ni entitlement automático.

## Integraciones y límites

- Valida catálogo/estado de producto de PLATFORM mediante contrato, sin consultar sus tablas.
- El onboarding comercial iniciado por `core.tenant.created.v1` sigue el flujo ratificado en [ADR-0002](../../../../docs/adr/ADR-0002-outbox-alta-tenant-core-platform.md).
- No realiza joins ni lecturas/escrituras SQL en otros backends y no administra reglas SaaS.

## Seguridad e integridad

Las filas tenant-scoped requieren RLS y contexto de seguridad aplicado transaccionalmente. La autorización considera actor, membresía, producto, acción y sucursal; nunca se concede por ID, Host o header aislado. Las operaciones deben ser idempotentes y evitar efectos parciales ante errores o concurrencia.

La aceptación futura cubre producto inexistente/inactivo, asociación cruzada, scope falsificado, carreras, duplicados, rollback, revocación y reutilización segura de conexiones.

## Implementación y pruebas

El directorio contiene solo estructura de capas. La matriz requiere contratos reales y pruebas con dependencias reales; todavía no se declara implementación ni validación.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [ADR-0002](../../../../docs/adr/ADR-0002-outbox-alta-tenant-core-platform.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
