# Reseñas

## Identificación y estado

- **Tipo:** bounded context de PUBLIC.
- **Ubicación:** `backend/public/modules/reviews/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** Fase 9, junto con Marketplace/Discovery y reseñas verificadas conforme al roadmap.

## Propósito y responsabilidad

Gestionar reseñas públicas moderadas y asociadas a una operación cuando el producto lo autorice. PUBLIC es dueño de la reseña, referencia o seudónimo del autor, moderación y estado público.

No se afirma que una reseña esté verificada sin evidencia. El módulo no accede a detalles privados de reservas o clientes.

## Operaciones previstas

Crear, editar, retirar o reportar reseñas; moderar y consultar únicamente las reseñas visibles. La elegibilidad y cualquier asociación a una operación requieren contrato autorizado.

## Límites e integraciones

- Usa identidad/referencias autorizadas de CORE sin obtener datos privados innecesarios.
- PUBLIC posee el canal y las reglas de moderación; no se apropia de datos de CRM o reservas.
- Contenido oculto o retirado no debe publicarse ni indexarse.

## Seguridad e integridad

Se requiere autenticación cuando el caso lo exija, scope de tenant y autorización de moderación. Ediciones concurrentes usan versión; reintentos y moderación duplicada deben ser idempotentes y las retiradas propagarse.

La aceptación futura cubre reseña no elegible, límites de texto, abuso, tenant ajeno, reportes concurrentes, moderación/rechazo y eliminación de proyecciones.

## Implementación y pruebas

Solo existe estructura de capas. No hay API, política de elegibilidad ni flujo de moderación ejecutable; no se declara contenido verificado.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
