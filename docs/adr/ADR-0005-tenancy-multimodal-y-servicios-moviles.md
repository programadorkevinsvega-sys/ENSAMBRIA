# ADR-0005: Tenancy multimodal y servicios móviles

- **Estado:** decisión expresa aceptada por el propietario el 2026-11-11; implementación futura
- **Fecha:** 2026-11-11
- **Ámbito:** modelo de tenant/organización/ubicación, incorporación empresarial y servicios móviles
- **Fuente de producto:** [requerimiento ENSAMBRIA](../requerimiento.md), sección 2.2
- **Fuentes arquitectónicas:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [modelo de dominios](../arquitectura-dominios.md)

## Contexto

El modelo previo describía tenancy y multi-sucursal, pero no garantizaba que una persona natural pudiera operar sin constituir una persona jurídica o registrar un local físico. Tampoco distinguía con suficiente precisión la sucursal, el establecimiento, la ubicación de atención al cliente y una zona de cobertura. La modalidad empresarial debe poder cambiar sin cambiar la identidad técnica del tenant ni su aislamiento.

Esta decisión complementa los cuatro backends y el flujo de alta CORE→PLATFORM de [ADR-0002](ADR-0002-outbox-alta-tenant-core-platform.md). No crea un backend, producto comercial, plan, regla legal o implementación.

## Decisión

1. **Tenant estable y común:** el tenant es el scope técnico de aislamiento, no una persona jurídica, establecimiento, sucursal, forma de negocio o modalidad. Puede ser titularizado/operado por persona natural o jurídica. La modalidad de incorporación es mutable configuración operativa, no tipo de tenant.
2. **Persona y organización distintas:** CORE posee identidad global humana, titularidad/membresías explícitas, tenant y relaciones estructurales. La organización operativa no implica personalidad jurídica. Los datos legales de personas jurídicas y representantes se recogen únicamente cuando el caso/mercado lo requiera y cuente con controles aplicables.
3. **Ubicaciones distintas y opcionales:** una sucursal organizativa no equivale a establecimiento físico; ambos son opcionales para un profesional móvil. El domicilio del cliente, la ubicación temporal acordada, cobertura, ubicación del profesional y atención remota son conceptos distintos. No se crea sucursal/establecimiento ficticio para habilitar operación.
4. **Ownership sin cambios de backend:** CORE conserva identidad, seguridad, tenancy, organización/sucursal central y estado común de reserva; BUSINESS conserva perfil operativo, catálogo/modos de servicio, establecimientos, CRM, workforce, cobertura, factibilidad del desplazamiento y estado de prestación; PLATFORM conserva onboarding comercial/planes; PUBLIC publica solo proyecciones autorizadas. Ningún backend consulta tablas ajenas ni transfiere ownership por integración.
5. **Reserva y geografía:** BUSINESS evalúa modalidad, cobertura, elegibilidad, asignación y compatibilidad temporal/geográfica por contrato. CORE revalida y conserva atómicamente la reserva, con snapshot mínimo y protegido de la ubicación acordada que sea necesaria para preservar su historia. Una dirección privada no es publicación ni prueba suficiente de autorización. No hay seguimiento permanente del profesional por defecto.
6. **Incorporación y oferta:** las rutas iniciales son negocio con establecimiento, profesional independiente y negocio con varias sucursales. Son rutas de configuración progresiva, no clases rígidas ni comandos para crear recursos ficticios. El alta técnica CORE→PLATFORM mantiene sin cambios ADR-0002: onboarding inicia `onboarding_pending`; no asigna automáticamente plan, precio, trial, factura, cobro o entitlement.
7. **Reglas no presumidas:** importes, cargos de desplazamiento, impuestos, zonas/distancias, buffers, asignación automática, proveedores geográficos, criterios laborales y requisitos de identidad/fiscalidad se definen por sus owners y mercados. No se crean defaults ni interpretaciones legales. Una validación previa bloquea solo el tratamiento/operación dependiente.
8. **Fases:** diseño y aceptación documental en Fase 0; identidad/tenancy/reserva común en Fase 1; modalidades de catálogo en Fase 2; CRM en Fase 3; cobertura, workforce y operación móvil completa en Fase 4; superficies de onboarding en Fase 5; páginas públicas tenant y proyección base autorizada BUSINESS→PUBLIC en Fase 5.3; directorio/Discovery/Marketplace en Fase 9.1 sobre dichas proyecciones. Las fases existentes no se reordenan ni dependen de una interfaz Marketplace anticipada.

## Contratos y dependencias

- BFF de producto presenta y persiste el progreso mediante contratos de los owners; no ejecuta una transacción distribuida ni posee reglas.
- CORE consulta a PLATFORM el contexto comercial/operativo autorizado y usa el flujo Outbox/JetStream/Inbox de ADR-0002 para altas confirmadas.
- BUSINESS consulta contexto y reserva con CORE mediante contrato autenticado. La verificación de cobertura y la selección de candidato pertenecen a BUSINESS; CORE mantiene la exclusividad de la reserva.
- BUSINESS entrega a PUBLIC solo campos publicados de forma explícita. PUBLIC no confirma reservas ni sirve disponibilidad obsoleta como autoridad.
- Los hechos visibles y persistidos siguen los requisitos Outbox/realtime del owner; Fase 0 no crea eventos, endpoints, tablas ni contratos ejecutables.

## Alternativas descartadas

- Exigir que cada tenant sea persona jurídica, tenga un local o cree una sucursal predeterminada: excluye actividad profesional individual y genera datos ficticios.
- Crear un backend o tenancy aparte para profesionales móviles: duplica aislamiento e identidad y contradice la topología de cuatro backends.
- Tratar tenant, organización, sucursal y establecimiento como sinónimos: confunde aislamiento, estructura operativa y localización física.
- Convertir modalidades de registro en tipos inmutables: impide la evolución progresiva sin duplicación o migración artificial.
- Usar PUBLIC como autoridad de cobertura/disponibilidad o compartir domicilio del cliente: expone datos privados y permite decisiones con proyecciones obsoletas.
- Asignar automáticamente planes, precios, viajes, impuestos o comisión: inventa políticas que pertenecen a owners y decisiones comerciales/legal.

## Consecuencias y aceptación

- La matriz de dominios conserva datos, transacciones, contratos, eventos, dependencias, autorización, idempotencia, pruebas y fase para CORE, BUSINESS, PLATFORM y PUBLIC.
- Los casos futuros prueban alta de persona natural sin local/sucursal; estabilidad de tenant e historial al agregar colaboradores/establecimientos/sucursales; modalidades permitidas/no permitidas; cobertura insuficiente; dos reservas incompatibles por desplazamiento; asignación/transiciones no autorizadas; fallo, duplicación y recuperación entre owners; aislamiento CRM; privacidad de direcciones y proyecciones PUBLIC.
- Las pruebas de implementación requieren dependencias reales en sus fases propietarias. La lista de escenarios de Fase 0 es diseño, no tests ejecutados ni evidencia de runtime.
- Los cambios de datos, si fueran necesarios, requieren migración versionada, transaccional y reversible cuando sea viable, con validación y recuperación en la fase correspondiente.
- Fase 0 no crea aplicaciones, carpetas vacías, crates, migraciones, APIs, eventos ni infraestructura ejecutable, y no declara el modelo implementado o aprobado comercial/jurídicamente.

## Decisión del propietario y aprobación

La instrucción expresa del propietario requiere un modelo multimodal con tenant estable, persona natural sin establecimiento obligatorio, modalidades de registro progresivas y prestación móvil, conservando el ownership y topología establecidos. El propietario aceptó expresamente este diseño documental el 2026-11-11. Esto no se presenta como aprobación independiente, legal ni evidencia de implementación; los parámetros comerciales, operativos o legales se resuelven antes de habilitar solo el comportamiento que depende de ellos.

## Referencias

- [Requerimiento ENSAMBRIA, §2.2](../requerimiento.md#22-modelo-empresarial-multimodal-y-servicios-móviles)
- [Contrato arquitectónico central](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios y estructura objetivo](../arquitectura-dominios.md#26-modelo-multimodal-de-tenancy-y-servicios-móviles)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
- [ADR-0002: onboarding durable CORE→PLATFORM](ADR-0002-outbox-alta-tenant-core-platform.md)
