# Registro de Fase 2: primeros verticales de Beauty & Wellness

## Estado vigente

**Fase 2: ABIERTA, SIN IMPLEMENTAR y NO ACEPTADA.** El código, las migraciones y las pruebas de producto descritos en el registro anterior fueron retirados durante el reinicio del repositorio. No hay evidencia vigente de capacidades funcionales para esta fase.

Las descripciones anteriores de objetivos alcanzados, rutas de implementación, DDL, resultados de pruebas y conteos dejan de ser evidencia vigente y se retiran de este registro. No deben utilizarse para afirmar que un vertical está implementado, integrado o aceptado. El estado común y los criterios de aceptación están en [ROADMAP.md](../ROADMAP.md) y [docs/requerimiento.md, §§22–23](requerimiento.md).

## Alcance requerido

La Fase 2 implementará en BUSINESS, conforme al roadmap y al ownership aprobado:

- Catálogo operativo de servicios, modalidades y recursos comunes.
- Workforce y cualificaciones requeridas para la operación.
- Verticales de barbería y peluquería/tratamientos capilares.
- Salones de belleza, uñas, pestañas, cejas, maquillaje, depilación y paquetes.
- Spas, estética y bienestar, con límites de tratamiento de datos aplicables.
- Persistencia, autorización, aislamiento tenant, eventos durables y APIs de los recorridos que los requieran.

Los módulos verticales deben consumir capacidades compartidas mediante contratos autorizados; no se crean backends adicionales ni se presentan carpetas o contratos sin implementación como soporte funcional.

Este resumen no sustituye los criterios normativos del [requerimiento](requerimiento.md), el [contrato arquitectónico](../ARCHITECTURE_RULES.md), el [modelo de dominios](arquitectura-dominios.md), los ADR ni las reglas especializadas.

## Aceptación pendiente

La aceptación requiere completar los recorridos de cada subfase, verificar integración con dependencias reales, ejecutar los escenarios positivos, negativos, límites, autorización, concurrencia, fallos e integridad que apliquen, y conservar evidencia reproducible conforme a §23 del requerimiento. No se ejecutaron compilaciones ni pruebas para este registro.
