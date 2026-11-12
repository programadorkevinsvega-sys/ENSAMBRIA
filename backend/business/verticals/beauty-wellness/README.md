# Vertical Beauty & Wellness

## Identificación y estado

- **Tipo:** vertical de industria de BUSINESS.
- **Ubicación:** `backend/business/verticals/beauty-wellness/`.
- **Estado:** `PLANNED`; esta carpeta contiene su carta arquitectónica. No hay implementación funcional.
- **Producto:** vertical funcional inicial definido por el roadmap ENSAMBRIA.
- **Fase:** capacidades iniciales de catálogo y servicios en Fase 2; funciones de negocio posteriores se entregan en la fase asignada a cada módulo propietario.

## Propósito y alcance

Agrupa las reglas, datos y casos de uso especializados de belleza y bienestar, incluyendo barberías, peluquerías, salones y uñas, spas y estética. No crea una aplicación, backend, despliegue, crate o bounded context único por cada subtipo.

Las particularidades de los servicios, fórmulas, recursos, tratamientos y flujos pertenecen a los contextos dueños definidos para BUSINESS. Esta carpeta no traslada automáticamente todos esos datos a un módulo nuevo; las entidades, agregados y persistencia de cada contexto se delimitan antes de implementar el caso correspondiente.

## Capacidades comunes y límites

Las capacidades reutilizables de operación permanecen bajo `backend/business/modules/` y conservan sus propios datos, reglas y contratos. Para el catálogo operativo, `establishment-catalog-verticals` posee el registro común del servicio: identidad estable, establecimiento asociado, nombre, estado de ciclo de vida y versión. Beauty & Wellness posee sus definiciones, atributos, parámetros y reglas que solo tienen significado en esta industria, relacionados con el registro común mediante su identificador y contratos publicados. El registro compartido no absorbe fórmulas, pasos de tratamientos ni reglas exclusivas de belleza. El vertical no importa implementaciones ni consulta o modifica tablas y migraciones de módulos comunes.

Como ejemplo de límite, `inventory-purchasing-costs` conserva la autoridad sobre inventario común y sus movimientos; Beauty & Wellness conserva fórmulas y reglas propias del servicio cuando sean específicas de esta industria. Este límite de ownership está decidido; los contratos y casos de uso ejecutables se detallarán antes de la implementación de Fase 2.

El vertical tampoco sustituye:

- CORE para identidad, tenancy, autorización, reservas comunes y pagos.
- PLATFORM para catálogo comercial, planes y relación SaaS.
- PUBLIC para proyecciones públicas autorizadas y atribución de Marketplace.
- Los módulos compartidos de BUSINESS para CRM, membresías, inventario, workforce, Commerce, marketing, facturación del establecimiento, comisiones laborales, franquicias y analítica, conforme al ownership de cada capacidad.

No se añaden condicionales de industria a módulos compartidos para resolver reglas exclusivas de Beauty & Wellness. Una necesidad demostrablemente común se propone al owner del módulo compartido con compatibilidad y pruebas de regresión para los verticales afectados.

## Salvaguardas del dominio

- Fórmulas, historial capilar y otros datos especializados requieren ownership, permisos, minimización y ciclo de vida definidos antes de su persistencia.
- Datos de salud o sensibles permanecen deshabilitados cuando falte la validación profesional o el control previo requerido para el tratamiento.
- Fotografías de resultados o evolución requieren autorización específica y controles de acceso, retención y eliminación aplicables.
- El vertical de spa/estética no constituye una historia clínica ni habilita por sí solo procedimientos regulados.
- Un vertical, su activación por tenant o un producto configurado no concede identidad, permisos ni acceso a datos.

## Integración y aceptación futura

Antes de implementar cada caso de uso se documentan su contexto propietario, actores, invariantes, contrato con módulos compartidos, autorización, persistencia, eventos aplicables, escenarios positivos/negativos/concurrentes y criterios de aceptación. Las integraciones entre backends usan contratos autorizados; no hay acceso cruzado a tablas.

La aceptación debe demostrar que un cambio exclusivo de Beauty & Wellness no altera el comportamiento de otros verticales. Cuando se amplíe una capacidad compartida, se prueban compatibilidad y regresiones en los verticales habilitados que la consumen. La existencia de esta carpeta o su README no prueba implementación, habilitación ni soporte comercial.

## Documentación relacionada

- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Modelo de dominios y estructura](../../../../docs/arquitectura-dominios.md)
- [Contrato arquitectónico](../../../../ARCHITECTURE_RULES.md)
- [ADR-0008: capacidades compartidas neutrales y verticales por industria](../../../../docs/adr/ADR-0008-modulos-compartidos-y-verticales-business.md)
