# Suscripciones SaaS

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/saas-subscriptions/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fases previstas:** diseño/estructura inicial en Fase 1; administración en Fase 5.

## Propósito y responsabilidad

Mantener el vínculo contractual de un tenant con una oferta aprobada, sus referencias/versiones y su ciclo de vida. La definición de ofertas, planes y precios pertenece a [`commercial-policies-monetization`](../commercial-policies-monetization/README.md).

## Operaciones previstas

Consultar una oferta y una suscripción, e iniciar cambios o terminaciones únicamente según reglas comerciales aprobadas. El alta operativa de un tenant no crea automáticamente una suscripción.

## Límites e integraciones

- Consume identificadores y versiones de políticas comerciales por contrato.
- Expone referencias y estado contractual a [`saas-billing`](../saas-billing/README.md); no calcula obligaciones ni procesa pagos.
- No administra identidad, tenancy ni entitlements técnicos de CORE.
- No consulta tablas de otros módulos ni define precios, trials, renovaciones o elegibilidad.

## Seguridad e integridad

Cada acción valida actor, tenant y permisos; las filas tenant-scoped requieren RLS y contexto transaccional conforme a [ADR-0001](../../../../docs/adr/ADR-0001-base-postgresql-compartida.md). Las transiciones deben ser versionadas e idempotentes, sin asignar una oferta por reintento o evento de alta.

La aceptación futura cubre oferta inexistente/no aprobada, tenant cruzado, transición inválida, cambio concurrente, repetición y ausencia de asignación automática.

## Implementación y pruebas

Solo existe la estructura de capas. No hay suscripciones, contratos, eventos o reglas comerciales implementados; no se documentan comandos de runtime.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [ADR-0002: onboarding CORE→PLATFORM](../../../../docs/adr/ADR-0002-outbox-alta-tenant-core-platform.md)
- [ADR-0007: separación de contextos SaaS](../../../../docs/adr/ADR-0007-separacion-contextos-facturacion-saas.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
