# Registro de integraciones, API y extensiones

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/integrations-api-extensions/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fases previstas:** API externa en Fase 7; Marketplace de extensiones en Fase 11.

## Propósito y responsabilidad

Gobernar el registro y ciclo de revisión/publicación de integradores y extensiones. PLATFORM conserva catálogo, publicador, versión, revisión, publicación y referencias de elegibilidad/entitlement. Los dominios propietarios conservan sus recursos, permisos y datos.

## Operaciones previstas

Registrar una versión, solicitar revisión, aprobar o rechazar publicación, suspender o revocar una extensión y consultar elegibilidad. Los detalles de contrato y políticas se definirán en la fase correspondiente.

## Límites e integraciones

- Se integra con identidad/contexto de CORE y con APIs/contratos de cada owner.
- No accede a tablas de otros módulos ni incorpora su lógica de dominio.
- No ejecuta código arbitrario ni concede permisos comodín.
- El registro en Marketplace no autoriza automáticamente el acceso a datos o recursos de un tenant.

## Seguridad e integridad

Separa publicador, revisor e instalador; aplica mínimo scope, revocación y auditoría. Compatibilidad versionada, instalación idempotente y deduplicación de eventos son criterios del diseño.

La aceptación futura cubre scope excesivo, versión incompatible, publicador no autorizado, tenant cruzado, revocación en curso y aislamiento del adapter.

## Implementación y pruebas

Este directorio contiene solo estructura de capas. No hay API externa ni Marketplace de extensiones ejecutable; no se declaran integraciones aceptadas.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
