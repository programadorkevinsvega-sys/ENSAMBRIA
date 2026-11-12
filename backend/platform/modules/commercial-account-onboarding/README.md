# Cuenta comercial y onboarding SaaS

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/commercial-account-onboarding/`.
- **Estado:** diseño ratificado; no hay implementación funcional.
- **Fase prevista:** primer corte ejecutable de Fase 1, según ADR-0002.

## Propósito y responsabilidad

Mantener la referencia comercial de PLATFORM a un tenant creado por CORE e iniciar un estado pendiente de onboarding sin activar obligaciones comerciales. El estado inicial es `onboarding_pending`.

CORE conserva el alta operativa y la identidad/tenancy; PLATFORM no crea tenants ni modifica datos CORE.

## Flujo previsto

Consumir `core.tenant.created.v1`, validar su versión y referencia de tenant, registrar el efecto idempotentemente y permitir consultar el onboarding pendiente. La integración usa Outbox en CORE, JetStream e Inbox en PLATFORM.

## Límites y controles

- No se permite polling SQL de CORE ni acceso a sus tablas.
- El evento no concede permisos ni eleva confianza por sí mismo.
- No se asignan automáticamente planes, precios, cobros, trials ni entitlements.
- La Inbox y el efecto de dominio deben confirmarse en una transacción local; el ACK ocurre después del commit.

La aceptación futura cubre atomicidad de CORE+Outbox, caída de PLATFORM/JetStream, evento duplicado o repetido, versión inválida y exactamente un estado pendiente por alta.

## Implementación y pruebas

El directorio contiene solo estructura de capas; no hay consumer, migración, contrato ejecutable ni prueba de integración real. El flujo diseñado no se presenta como implementado.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [ADR-0002](../../../../docs/adr/ADR-0002-outbox-alta-tenant-core-platform.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
