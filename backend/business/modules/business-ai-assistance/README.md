# Asistencia empresarial con IA

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/business-ai-assistance/`.
- **Estado:** diseño documentado; no hay implementación funcional ni proveedor seleccionado.
- **Fase prevista:** Fase 11.

## Propósito y alcance

Ofrecer una experiencia de asistencia para consultas y análisis basados en datos autorizados del establecimiento, y preparar borradores de contenido. BUSINESS es dueño de la experiencia tenant-facing y de su estado/configuración mínima.

El módulo no es dueño de los datos de CRM, ventas, reservas, inventario o BI. Una recomendación no ejecuta por sí misma cobros, cambios financieros, campañas ni publicaciones.

## Operaciones previstas

Recibir una solicitud tipada con actor, tenant, sucursal, propósito y herramientas acotadas; devolver una respuesta que identifique datos y período autorizados, definiciones e incertidumbre. Cualquier acción posterior debe pasar por el comando autorizado del módulo propietario y requerir confirmación cuando corresponda.

## Límites e integraciones

- Consulta BI y dominios fuente únicamente mediante contratos y permisos vigentes.
- El proveedor de IA debe permanecer desacoplado; no hay proveedor elegido.
- No permite SQL directo, no expone secretos en prompts y no envía datos personales a un proveedor no aprobado.
- La salida del modelo nunca concede autorización ni produce efectos de dominio directamente.

## Seguridad e integridad

La autorización y el scope se revalidan para cada herramienta. Deben aplicarse límites de uso/costo y protección frente a instrucciones maliciosas, datos obsoletos o contradictorios y respuestas no verificables. Las acciones sensibles requieren confirmación humana y el proveedor o tratamiento sujeto a validación previa permanece bloqueado hasta cumplir sus condiciones.

La aceptación futura debe cubrir actor o scope insuficiente, prompt injection, fuentes vacías o contradictorias, información obsoleta, caída del proveedor y rechazo de acciones sensibles sin confirmación.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay proveedor, contrato ejecutable, manifiesto ni suite de pruebas; no se afirma que exista una capacidad de IA operativa.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
