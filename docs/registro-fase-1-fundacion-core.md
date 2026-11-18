# Registro de Fase 1: Fundación de CORE

## Estado vigente

**Fase 1: ABIERTA, SIN IMPLEMENTAR y NO ACEPTADA.** El código y los artefactos de implementación asociados a registros anteriores fueron retirados durante el reinicio del repositorio. No hay evidencia vigente de servicios ejecutables, persistencia, integraciones o pruebas de producto para esta fase.

Esto no revoca las decisiones arquitectónicas que fueron aceptadas en Fase 0 o ratificadas para el diseño de Fase 1. Los ADR conservan el estado expresado en cada documento y siguen siendo normativos dentro de su alcance hasta que una decisión aprobada los sustituya. La aceptación de esas decisiones es distinta de la aceptación de Fase 1 y no demuestra que sus comportamientos hayan sido implementados.

Las descripciones de objetivos alcanzados, rutas de código, migraciones, conteos de pruebas y comandos de ejecución que aparecían en versiones anteriores de este registro quedan retiradas como evidencia vigente. No deben utilizarse para afirmar que una subfase está implementada, validada o aceptada. El estado común y los criterios de aceptación están en [ROADMAP.md](../ROADMAP.md) y [docs/requerimiento.md, §§22–23](requerimiento.md).

## Alcance requerido

La Fase 1 implementará, según la secuencia del roadmap y en los owners correspondientes:

- La base técnica y composición legítima del workspace Cargo al incorporar código funcional.
- Persistencia PostgreSQL compartida, ownership de schemas y migraciones, credenciales de mínimo privilegio y aislamiento tenant-scoped.
- Identidad, autenticación, autorización, tenancy y contexto de producto.
- El flujo durable CORE→PLATFORM para el alta operativa del tenant, con Outbox/Inbox cuando corresponda.
- Suscripciones SaaS y evaluación de entitlements conforme a los límites de CORE y PLATFORM.
- Reservas y disponibilidad común bajo el ownership establecido.
- APIs/BFF, observabilidad, privacidad, seguridad y pruebas de aceptación aplicables a cada recorrido introducido.

Este resumen no sustituye los criterios normativos del [requerimiento](requerimiento.md), el [contrato arquitectónico](../ARCHITECTURE_RULES.md), los ADR ni las reglas especializadas.

## Aceptación pendiente

La aceptación requiere implementación real por recorrido, pruebas aplicables ejecutadas contra las dependencias reales, validación local y CI según las fuentes vigentes, documentación sincronizada y evidencia trazable conforme a §23 del requerimiento. Una decisión arquitectónica o una descripción en este registro no prueba implementación.

No se ejecutaron compilaciones ni pruebas para este registro.
