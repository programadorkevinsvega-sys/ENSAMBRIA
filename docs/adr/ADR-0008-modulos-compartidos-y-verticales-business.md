# ADR-0008: Capacidades compartidas neutrales y verticales por industria en BUSINESS

- **Estado:** decisión expresa aceptada por el propietario el 2026-11-11; implementación futura
- **Fecha:** 2026-11-11
- **Ámbito:** organización de las capacidades operativas compartidas y reglas específicas de industria en BUSINESS
- **Fuentes de producto:** [requerimiento ENSAMBRIA](../requerimiento.md), §§3.5, 4 y 14
- **Fuentes arquitectónicas:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [modelo de dominios](../arquitectura-dominios.md), [ADR-0004](ADR-0004-plataforma-multiproducto.md)

## Contexto

El diseño inicial agrupaba capacidades comunes y reglas verticales en módulos de BUSINESS, sin indicar una organización física objetivo que permitiera añadir industrias sin acoplar los módulos compartidos a Beauty & Wellness. La arquitectura debe conservar la propiedad única de datos y reglas, reutilizar capacidades genuinamente comunes y permitir que cada industria evolucione con comportamiento especializado.

Esta decisión aclara esa separación sin cambiar la topología de cuatro backends ni afirmar que el nuevo árbol o sus capacidades ya estén implementados.

## Decisión

1. BUSINESS conserva una sola autoridad para las capacidades operativas de los establecimientos y se organiza en dos grupos conceptuales:
   - `backend/business/modules/` para bounded contexts con capacidades cuya semántica sea común a más de una industria.
   - `backend/business/verticals/<industria>/` para agrupar reglas, datos y casos de uso especializados de una industria.
2. `backend/business/verticals/beauty-wellness/` es la carpeta documental del vertical inicial. Contiene su carta arquitectónica `README.md`; no contiene runtime ni configura un módulo ejecutable. La estructura física documenta la carpeta presente sin confundirla con implementación.
3. La carpeta por industria es una agrupación arquitectónica: no crea automáticamente un backend, despliegue, crate o bounded context único. Una industria puede contener los contextos que sus límites de dominio justifiquen; los subtipos de negocio pueden permanecer dentro del mismo vertical si comparten lenguaje y reglas.
4. Los módulos compartidos deben conservar contratos y reglas neutrales respecto a industria, sin condicionales de negocio específicos de Beauty & Wellness, farmacia u otra industria. No todas las industrias tienen que usar todas las capacidades compartidas.
5. El owner del módulo común conserva sus datos y reglas generales. Un vertical conserva sus reglas y atributos exclusivos e integra el módulo mediante su contrato publicado; no importa su implementación ni modifica sus tablas, migraciones o repositorios. Las capacidades que cruzan backends siguen los contratos y canales definidos por su semántica: gRPC para respuesta síncrona y Outbox/JetStream para hechos durables.
6. Si una necesidad especializada se demuestra común a varias industrias, el owner del módulo puede ampliar su contrato y comportamiento generalizado, con compatibilidad explícita y pruebas de regresión para verticales habilitados afectados. La decisión no introduce campos arbitrarios, EAV, un esquema universal, un motor central de reglas ni una plataforma genérica de extensiones.
7. `establishment-catalog-verticals` es dueño del registro común de servicios operativos de BUSINESS: identidad estable del servicio, establecimiento asociado, nombre, estado de ciclo de vida y versión. No impone un modelo universal para los detalles del servicio. El vertical dueño conserva las definiciones, atributos, parámetros y reglas que solo tienen significado para su industria, y se relaciona con el registro común mediante su identificador y contratos publicados; no accede directamente a sus tablas. Esta decisión define ownership, no un contrato runtime ni implementación.
8. Una industria futura requiere descubrimiento, ownership, decisión de producto, datos/contratos, jurisdicciones, pruebas y aceptación propias. La carpeta objetivo no la declara soportada ni comercialmente aprobada.

## Límites y composición

Cada bounded context mantiene sus agregados, invariantes, autorización, persistencia, contratos y pruebas. Los módulos co-residentes no comparten tablas ni importan implementaciones de otros módulos. Los contextos de CORE, PLATFORM y PUBLIC conservan sus ownership: un vertical BUSINESS puede consumirlos únicamente por contratos autorizados; no mueve a PLATFORM la operación de la industria ni permite que PUBLIC acceda a datos privados.

Ejemplo conceptual: inventario puede conservar la autoridad de stock, lotes y movimientos; Beauty & Wellness conserva fórmulas o reglas de servicio que sean propias de ese sector. El vertical solicita operaciones de inventario por su contrato. Si un atributo solo tiene significado para un vertical, no se agrega como regla específica al módulo general; antes de persistirlo se asigna ownership y contrato a su dueño.

## Alternativas consideradas

- Hacer que todos los módulos comunes incorporen condicionales y campos específicos por industria: descartado porque los acopla a cada vertical y amplía el riesgo de regresión.
- Duplicar inventario, workforce, Commerce u otras capacidades para cada industria: descartado porque divide la autoridad y duplica reglas/datos compartidos.
- Crear un backend o crate por industria o por vertical: descartado porque contradice la topología modular de cuatro backends y no se justifica por una carpeta.
- Usar esquema universal o campos dinámicos para cualquier atributo: descartado porque no se han definido necesidades, validaciones, consultas, integridad ni ownership que lo justifiquen.
- Mantener todo el comportamiento específico únicamente dentro de módulos compartidos: descartado porque no da un límite reconocible para evolución independiente de cada industria.

## Consecuencias y criterios de aceptación

- La vista física y la vista objetivo se distinguen; la carpeta de Beauty & Wellness contiene documentación arquitectónica, no código funcional.
- Los módulos compartidos no dependen de reglas de una industria ni se habilitan solo por configuración si su comportamiento no está implementado.
- Cada interacción vertical–capacidad común tiene contrato, owner, autorización, integridad y pruebas de fallos/concurrencia acordes al caso de uso; no hay acceso cruzado a tablas.
- Un cambio exclusivo de vertical no modifica el comportamiento o el almacenamiento de otros verticales. Una ampliación de capacidad común conserva compatibilidad y ejecuta regresión de cada vertical afectado.
- El split de ownership para el registro común y los detalles especializados queda decidido en este ADR. Antes de implementar se especifican los contratos, entidades y pruebas concretos dentro de la Fase 2; ese trabajo no reabre la decisión de ownership.
- Una industria futura no se marca soportada hasta que sus requisitos, implementación y aceptación se hayan completado en el roadmap correspondiente.

## Impacto

- **Arquitectura:** hace explícita la organización modular de BUSINESS sin modificar sus fronteras ni agregar servicios.
- **Seguridad y datos:** mantiene autorización, RLS/grants aplicables, propiedad de persistencia y acceso solo por contrato.
- **Evolución y pruebas:** cada cambio distingue regresión propia de vertical de cambios compatibles a una capacidad común.
- **Estado:** el diseño documental, incluido el límite entre el registro común de servicios y los datos/reglas verticales, fue aceptado por el propietario el 2026-11-11. Existe la carpeta `verticals/beauty-wellness/` con su README, pero no hay runtime ni pruebas funcionales como resultado de este ADR.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato arquitectónico central](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios y estructura](../arquitectura-dominios.md)
- [ADR-0004: plataforma multiproducto](ADR-0004-plataforma-multiproducto.md)
