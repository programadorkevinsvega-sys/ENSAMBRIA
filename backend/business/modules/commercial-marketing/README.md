# Marketing comercial

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/commercial-marketing/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 6; el alcance depende de validación jurídica, selección de proveedor y controles por canal.

## Propósito y alcance

Gestionar campañas comerciales del establecimiento, incluida su audiencia autorizada, plantilla y estado de envío o preferencia según el propósito. No convierte los datos del CRM ni los avisos transaccionales en promoción automática.

Este módulo no es dueño del marketing SaaS de ENSAMBRIA ni de los datos fuente del CRM. No hay proveedores de envío seleccionados en este diseño.

## Operaciones previstas

Crear o pausar campañas, calcular audiencias permitidas, solicitar envíos y registrar bajas; consultar el estado de entrega mínimo necesario. La integración con canales externos se realizará mediante adapters desacoplados cuando hayan sido aprobados.

## Límites e integraciones

- Usa CRM y preferencias mediante contratos autorizados.
- Consume callbacks mínimos del proveedor únicamente después de seleccionar y validar dicho proveedor.
- No realiza marketing SaaS de PLATFORM ni amplía la audiencia en el frontend.
- Los hechos de campaña se publican solo cuando haya una necesidad durable y consumidores documentados.

## Seguridad e integridad

La finalidad, preferencia y canal deben validarse en el backend propietario; la baja/supresión prevalece sobre envíos pendientes. Se prohíbe incluir contenido sensible o credenciales en logs. Reintentos y callbacks deben deduplicarse para no duplicar campañas.

La aceptación futura debe cubrir opt-out, ausencia de preferencia, acceso entre tenants, envíos duplicados, caída del proveedor, distinción entre mensajes transaccionales y promocionales, y redacción.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay proveedor, contrato ejecutable, manifiesto ni suite de pruebas. El tratamiento o canal que requiera validación previa permanece bloqueado hasta cumplirla.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
