# Evaluación de entitlements

## Identificación y estado

- **Tipo:** bounded context de CORE.
- **Ubicación:** `backend/core/modules/entitlement-evaluation/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** Fase 1, con integración de cada capacidad en su fase propietaria.

## Propósito y responsabilidad

Evaluar en runtime si una capacidad está efectivamente permitida para un tenant y scope. La interfaz cliente no decide acceso. CORE es dueño de la evaluación técnica; PLATFORM conserva los planes y las políticas comerciales.

## Operaciones previstas

Evaluar una capacidad según identidad, tenant/scope, acción y versión; consultar o invalidar resultados conforme al contrato. La respuesta debe ser una decisión tipada con motivo seguro y referencia de versión.

## Límites e integraciones

- Se integra con tenancy de CORE y consume de PLATFORM el estado autorizado mediante contrato.
- No consulta tablas de planes ni determina precios o políticas comerciales.
- Toda caché, si se diseña, debe respetar expiración e invalidación; no puede mantener permisos revocados activos más allá de la política aprobada.

## Seguridad e integridad

Requiere contexto autenticado y autorización en el backend propietario. Versionado y deduplicación deben prevenir decisiones obsoletas ante cambios concurrentes o eventos repetidos.

La aceptación futura cubre entitlement ausente, vencido o revocado, cambio concurrente, evento duplicado y permisos de otro tenant; el backend debe rechazar aunque la UI ofrezca la capacidad.

## Implementación y pruebas

Solo existe la estructura de capas. No hay contrato ejecutable, política comercial implementada ni comandos de prueba; no se afirma validación.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
