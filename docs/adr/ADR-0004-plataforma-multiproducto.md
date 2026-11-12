# ADR-0004: Plataforma multiproducto con cuatro backends de dominio

- **Estado:** decisión expresa aceptada por el propietario el 2026-11-11; implementación futura
- **Fecha:** 2026-11-11
- **Ámbito:** composición de productos SaaS, contexto tenant–producto, BFFs y superficies frontend
- **Fuente de producto:** [requerimiento ENSAMBRIA](../requerimiento.md), sección “Evolución multiproducto y multindustria”
- **Fuentes arquitectónicas:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [modelo de dominios](../arquitectura-dominios.md)

## Contexto

ENSAMBRIA se concibió inicialmente como un producto funcional orientado a belleza y bienestar, pero el requerimiento vigente establece una plataforma tecnológica compartida desde la cual pueden evolucionar productos SaaS comerciales independientes. Beauty & Wellness es el producto funcional inicial. Las demás industrias descritas como horizonte no constituyen productos implementados ni autorizados para su lanzamiento.

La estrategia multiproducto no crea backends adicionales ni cambia el ownership de los cuatro backends. Sí requiere separar el registro comercial del producto, su contexto técnico en el tenant, sus superficies de presentación y sus reglas verticales. Esta decisión también sustituye la interpretación anterior en la que las superficies tenant/business de Beauty se vinculaban al BFF público o a un BFF BUSINESS integrado.

## Decisión

1. ENSAMBRIA mantiene una plataforma tecnológica común y cuatro backends de dominio desplegables: CORE, BUSINESS, PLATFORM y PUBLIC. Un producto o vertical no implica un nuevo backend, microservicio, crate o esquema por defecto.
2. PLATFORM posee el catálogo/configuración comercial versionada de producto: identificador estable, estado, marcas, dominios autorizados, mercados, superficies y capacidades declaradas. El catálogo no contiene datos operativos tenant ni concede permisos.
3. CORE valida el contexto de producto y posee la asociación técnica tenant–producto junto con identidad, tenancy y autorización. Cada alta autoservicio debe indicar un producto registrado/activo por un contexto confiable; Host, query, header o `product_id` aislado no autorizan acceso.
4. BUSINESS conserva los dominios operativos y sus reglas verticales. PUBLIC conserva Discovery/Marketplace, atribución y proyecciones públicas explícitamente autorizadas; no accede a las tablas privadas fuente. El flujo base de publicación BUSINESS→PUBLIC para páginas tenant se implementa en Fase 5.3; el directorio y Discovery/Marketplace se implementan en Fase 9.1 sobre esas proyecciones.
5. Cada producto implementado dispone de BFF REST/OpenAPI especializado, independiente como aplicación/despliegue, y de las superficies tenant/business/móvil que requiera. El BFF de producto consume por gRPC los contratos autorizados de los backends propietarios. El BFF público independiente sirve Discovery desde Fase 9.1. El BFF de administración permanece inicialmente integrado en PLATFORM. Los BFF no poseen reglas de negocio ni acceden directamente a PostgreSQL, Redis o NATS.
6. La oferta inicial funcional es Beauty & Wellness. La lista de otras industrias es horizonte de arquitectura y no las aprueba como productos comerciales, alcance de lanzamiento ni trabajo de fase.
7. El alta operativa sigue siendo autoservicio en el producto. CORE crea tenant y asociación tenant–producto; el hecho CORE→PLATFORM de [ADR-0002](ADR-0002-outbox-alta-tenant-core-platform.md) incluye la referencia canónica del producto para que PLATFORM cree la relación en `onboarding_pending`. No asigna automáticamente plan, precio, periodo de prueba, factura, cobro, suscripción o entitlement. El administrador PLATFORM no registra manualmente tenants ni clientes finales.
8. Fase 0 registra este diseño y sus contratos/criterios, pero no crea catálogo runtime, apps, carpetas vacías, crates ficticios, migraciones o servicios ejecutables. El catálogo y el contexto mínimos que requiere el alta autoservicio deben implementarse en la primera fase que acepta esos registros, conforme al roadmap.
9. La secuencia de superficies públicas evita dependencias circulares: Fase 5.3 entrega páginas públicas tenant, la proyección/publicación base autorizada de BUSINESS hacia PUBLIC y Corporate institucional; Fase 9.1 construye directorio y Discovery/Marketplace como consumidores de las proyecciones aceptadas en 5.3. La publicación base no implementa por anticipado la búsqueda/directorio de Marketplace.

## Resolución de contexto y límites de confianza

La resolución es secuencial y revalidada: dominio/configuración registrada → producto configurado → tenant asociado a ese producto → actor/organización/sucursal autenticados → autorización del caso de uso. Un BFF puede resolver configuración para enrutar; CORE confirma la asociación y cada owner valida el scope de sus operaciones. Los cambios persistidos de configuración que requieran distribución durable usan el contrato Outbox/JetStream e Inbox idempotente; las consultas inmediatas usan gRPC. No se infiere autorización de una marca, dominio, identidad global, ID o configuración frontend.

La descripción de cinco tipos de experiencia de este ADR corresponde al diseño previo a la decisión de interfaces de [ADR-0006](ADR-0006-arquitectura-multiplataforma-aplicaciones.md). ADR-0006 sustituye únicamente el conteo, nombres y tecnologías de los frontends; este ADR mantiene la decisión multiproducto, los owners, los BFF y las reglas de producto/tenant. No se crea una aplicación universal con reglas de dominio condicionadas por marca o industria.

## Alternativas descartadas

- Crear un backend o microservicio por producto/industria: duplica capacidad común y contradice la topología de cuatro backends.
- Mantener todo frontend tenant/business en un BFF genérico o en el BFF público: mezcla fronteras/API de productos y Discovery y no satisface la especialización/despliegue independiente del producto.
- Resolver tenant/producto únicamente por Host, encabezado o selección del frontend: convierte un dato manipulable en autoridad y no valida la asociación ni los permisos en CORE.
- Permitir a PLATFORM crear manualmente tenants/clientes o asignar una oferta durante el evento de alta: contradice el autoservicio y el flujo de onboarding ratificado en ADR-0002.
- Tratar todas las industrias candidatas como productos aprobados: confunde horizonte arquitectónico con decisiones comerciales y alcance de implementación.

## Consecuencias y criterios de aceptación

- El catálogo comercial de PLATFORM y la asociación tenant–producto de CORE tienen ownership, datos y contratos distintos; no se integran mediante SQL compartido.
- Antes de aceptar el primer registro de tenant, debe existir un producto vigente consultable por contrato, una asociación tenant–producto validada por CORE y el evento de onboarding con ambas referencias.
- Las pruebas de implementación deben cubrir producto inexistente/inactivo, dominio no asociado o reasignado, asociación y sesión de otro producto, selección/header falsificado, carreras de activación/desactivación, duplicados/replay del evento, permiso insuficiente y ausencia de efectos parciales o asignaciones comerciales implícitas.
- Cada recorrido web prueba frontend→BFF de producto→owners reales, aislamiento de producto/tenant, errores y recuperación; BFF público y consola PLATFORM conservan sus propios contratos.
- Fase 0 no declara ninguna de estas capacidades implementada o validada. El propietario aceptó el diseño documental el 2026-11-11; el cierre de Fase 0 sigue sujeto a sus criterios de salida.

## Decisión del propietario y aprobación

El propietario dio instrucción expresa de adoptar la arquitectura de plataforma multiproducto, con Beauty & Wellness como producto inicial y manteniendo Fase 0 documental. El 2026-11-11 aceptó expresamente el diseño documentado. Esta aceptación no es review de GitHub, aprobación legal ni evidencia de runtime. Las políticas comerciales no definidas y la aprobación de productos futuros se resolverán en las fases/alcances que las introduzcan y no bloquean el trabajo documental independiente.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato arquitectónico central](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios y estructura objetivo](../arquitectura-dominios.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
- [ADR-0002: onboarding durable CORE→PLATFORM](ADR-0002-outbox-alta-tenant-core-platform.md)
