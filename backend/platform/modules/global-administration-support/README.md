# Administración global y soporte

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/global-administration-support/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** consola global y herramientas administrativas en Fase 5; cada operación se implementa en la fase de su owner.

## Propósito y responsabilidad

Operar el control global del producto y atender solicitudes de soporte sin transferir a PLATFORM la propiedad de los datos o reglas de negocio de los tenants. PLATFORM conserva sus operaciones administrativas, solicitudes y auditoría.

## Operaciones previstas

Buscar relaciones comerciales, consultar o actualizar datos que pertenezcan a PLATFORM, consultar evidencia e iniciar acciones delegadas al owner correspondiente mediante contrato.

## Límites e integraciones

- Consume contratos de CORE, BUSINESS o PUBLIC según el caso.
- El BFF de administración está integrado inicialmente en PLATFORM.
- Los owners ejecutan sus propios cambios; PLATFORM no muta datos de dominio ajenos.
- El acceso de soporte no concede acceso global a datos privados ni permite crear tenants/clientes manualmente.

## Seguridad e integridad

El acceso privilegiado requiere MFA, mínimo privilegio, scope explícito y auditoría. Se separan lectura y escritura; acciones críticas deben ser idempotentes y controlar concurrencia.

La aceptación futura cubre operador insuficiente, tenant equivocado, acceso de soporte, cambio parcial, repetición, sesión expirada y trazabilidad de auditoría.

## Implementación y pruebas

Solo existe la estructura de capas. No hay consola, BFF ni operaciones administrativas implementadas; no se documentan comandos disponibles.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
