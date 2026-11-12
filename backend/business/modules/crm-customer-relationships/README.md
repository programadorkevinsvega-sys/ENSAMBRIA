# CRM y relación con clientes

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/crm-customer-relationships/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 3.

## Propósito y alcance

Mantener la relación operativa de cada establecimiento con sus clientes, incluyendo el perfil CRM, historial y preferencias específicas dentro de su tenant. Este contexto evita que reservas, marketing y perfiles públicos creen fuentes privadas inconsistentes.

No es dueño de la identidad global ni de la autenticación (CORE), tampoco de la publicación pública ni de la autorización para usar datos con fines promocionales.

## Operaciones previstas

Crear y actualizar perfiles; asociar interacciones autorizadas; buscar con criterios permitidos; consultar historial y preferencias; y gestionar exportaciones conforme a la política aplicable. Las respuestas deben ser paginadas o basadas en cursor y limitar los campos según permisos y finalidad.

## Límites e integraciones

- Consulta identidad o reservas a CORE solo mediante contratos y cuando sea necesario.
- Se integra con Marketing mediante contratos para preferencias y audiencias autorizadas.
- No comparte automáticamente perfiles o contactos con PUBLIC ni habilita su uso promocional.
- No combina clientes entre tenants ni consulta directamente datos de otros módulos.

## Seguridad e integridad

El acceso requiere tenant, finalidad, permisos y, cuando aplique, scope de sucursal. Deben minimizarse los datos, limitarse los campos potencialmente sensibles y respetarse derechos y preferencias. Las reglas de deduplicación aún deben definirse; no se presume que un correo o teléfono sea una clave única global.

La aceptación futura debe cubrir duplicados conforme a reglas aprobadas, búsquedas entre tenants, exportación no autorizada, revocación, actualizaciones concurrentes, campos opcionales y redacción de datos sensibles.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni suite de pruebas; no se documentan comandos ni APIs inexistentes.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
