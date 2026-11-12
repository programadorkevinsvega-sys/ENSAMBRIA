# ADR-0006: Arquitectura multiplataforma y separación de aplicaciones del ecosistema ENSAMBRIA

- **Estado:** decisión expresa aceptada por el propietario el 2026-11-11; implementación futura
- **Fecha:** 2026-11-11
- **Ámbito:** seis proyectos frontend, sus responsabilidades, plataformas objetivo y acceso a los BFF autorizados
- **Fuentes de producto:** [requerimiento ENSAMBRIA](../requerimiento.md), §§8, 13, 14 y 22
- **Fuentes arquitectónicas:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [modelo de dominios](../arquitectura-dominios.md), [ADR-0004](ADR-0004-plataforma-multiproducto.md) y [ADR-0005](ADR-0005-tenancy-multimodal-y-servicios-moviles.md)

## Contexto

El diseño previo agrupaba cinco tipos de experiencia y definía React Native para móvil. No diferenciaba con claridad el cliente empresarial instalado, el portal empresarial web, el cliente final, el Marketplace público, la administración global y el sitio institucional.

El alcance aprobado requiere acceso a la operación empresarial tanto desde escritorio Windows/Linux/macOS como desde navegadores, aplicaciones móviles para negocio y clientes, páginas públicas descubribles y una consola institucional privada. La separación de interfaces no debe duplicar las reglas de los cuatro backends ni crear nuevos dominios, autoridades de identidad o backends.

No hay proyectos frontend, dependencias, contratos API ejecutables ni despliegues en el repositorio que materialicen esta arquitectura. Este ADR establece el diseño; no afirma implementación o validación runtime.

## Problema

Definir tecnologías y responsabilidades de las aplicaciones sin:

- separar artificialmente la lógica empresarial entre clientes;
- limitar el trabajo en escritorio a ampliar una UI móvil;
- mezclar la operación privada de tenants con la experiencia pública;
- convertir Marketplace o Corporate en autoridades de datos;
- inferir autorización a partir del cliente, tenant, producto, dominio o interfaz;
- agregar backends, APIs o contratos no requeridos.

## Requisitos empresariales

1. Negocios y personal pueden operar desde Windows, Linux, macOS, móviles y tablets.
2. Los tenants pueden administrar el negocio desde navegador sin instalar una aplicación.
3. Los clientes finales tienen una experiencia móvil separada de las herramientas administrativas.
4. El descubrimiento público y el contenido institucional son experiencias web independientes, con necesidades de SEO distintas.
5. La administración global de ENSAMBRIA es privada y distinta de la administración de un tenant.
6. La plataforma conserva cuatro backends Rust, los límites de dominio y las políticas centrales de identidad y autorización.
7. El contenido, datos y operaciones se consumen mediante contratos tipados y superficies autorizadas; no se comparten entidades internas de dominio.

## Decisión

El ecosistema consta de seis proyectos de interfaz distintos:

| Proyecto | Tecnología | Responsabilidad | Plataformas |
|---|---|---|---|
| **ENSAMBRIA Business** | Flutter + Dart | Administración y operación empresarial por usuarios autorizados del tenant. | Android, iOS, tablets Android/iPad, Windows, Linux y macOS. |
| **ENSAMBRIA Tenant Portal** | React + TypeScript + Next.js | Administración web de los mismos tenants y capacidades empresariales accesibles desde Business. | Navegadores modernos en Windows, Linux, macOS, ChromeOS y sistemas compatibles. |
| **ENSAMBRIA App** | Flutter + Dart | Experiencia móvil de clientes finales, separada de la administración empresarial. | Android, iOS y tablets Android/iPad. |
| **ENSAMBRIA Marketplace** | React + TypeScript + Next.js | Descubrimiento público de establecimientos, profesionales, servicios y contenido autorizado. | Web pública. |
| **ENSAMBRIA Platform Admin** | React + TypeScript + Next.js | Administración global y soporte por operadores autorizados. | Web empresarial privada. |
| **ENSAMBRIA Corporate** | React + TypeScript + Next.js | Presencia institucional y comercial pública de ENSAMBRIA. | Web pública. |

### Decisiones de composición y contratos

1. Business y Tenant Portal son clientes independientes de la misma plataforma empresarial. Consumen el mismo BFF/API de producto, operaciones y contratos tipados, y se sujetan a los mismos controles backend. No se crean endpoints separados solo para distinguir interfaces.
2. La paridad funcional se mide por capacidad habilitada, actor/rol, operación, permisos, invariantes y resultado. No requiere idénticos componentes, navegación o presentación. La adaptación por plataforma es permitida y no cambia autorización ni reglas de dominio.
3. App usa el BFF/API de producto para las operaciones de cliente autorizadas y puede usar el BFF público independiente para Discovery cuando esa capacidad se entregue en Fase 9.1. Marketplace usa el BFF público, que consume contratos de PUBLIC. No se concede a ningún cliente acceso directo a gRPC interno o a stores.
4. Platform Admin consume el BFF administrativo integrado inicialmente en PLATFORM. Cada backend dueño aplica autorización para sus datos y operaciones; el panel no recibe autoridad global implícita sobre otros owners.
5. Corporate publica contenido institucional estático o servido mediante un contrato público autorizado. No posee reglas de negocio ni accede directamente a servicios, tablas o repositorios. Cualquier tratamiento de datos que se agregue a formularios o integraciones requiere diseño, owner, contrato y controles específicos en su fase.
6. Los frontends pueden compartir contratos de transporte tipados o componentes de presentación neutrales cuando exista necesidad justificada. No comparten entidades/modelos internos de dominio, sesiones, permisos ni cachés con scope implícito.
7. Rust y los cuatro backends CORE, BUSINESS, PLATFORM y PUBLIC permanecen sin cambios. No se crea un backend por aplicación, una API paralela ni una base de datos por frontend.
8. Las aplicaciones pueden residir en el mismo repositorio, pero conservan compilación, configuración, seguridad, ciclo de release y despliegue apropiados a cada proyecto. Esta decisión no requiere seis repositorios.
9. Business debe ofrecer una experiencia de escritorio diseñada para teclado, ratón, navegación eficiente, tablas y formularios, no una UI móvil simplemente escalada. Mobile y tablet deben conservar interacciones táctiles y accesibilidad adaptativa.
10. Android TV, Google TV, Apple TV, Samsung Tizen, LG webOS y Amazon Fire TV quedan fuera del alcance obligatorio. Cada plataforma requiere evaluación y decisión específicas.

### Seguridad, calidad y operación

- CORE mantiene identidad/autenticación y contexto central; cada backend propietario autoriza cada operación y valida tenant, sucursal, actor y permisos. Los IDs y valores enviados por un frontend no son prueba de autorización.
- La seguridad no depende de ocultar controles en UI. Se prueban accesos insuficientes, sesiones expiradas, tenant/sucursal ajenos, solicitudes manipuladas, concurrencia, errores y efectos parciales en backend.
- Webs públicas (Marketplace y Corporate, y el contenido tenant publicado que se habilite) aplican SEO/Open Graph al HTML inicial y solo a contenido autorizado. Tenant Portal y Platform Admin son privados/noindex; Flutter comparte enlaces canónicos aprobados, no sustituye la web pública.
- Todos los proyectos aplican las reglas de accesibilidad, privacidad, seguridad, rendimiento, correlación, redacción y pruebas de las guías especializadas. La instrumentación Sentry/SDK y el almacenamiento seguro se configuran por plataforma cuando se implemente; este ADR no crea proveedores, proyectos ni configuración.
- Las pruebas y releases son propios de cada frontend y validan la frontera real frontend→BFF→backend owner. Las pruebas visuales o de widgets no sustituyen autorización ni aceptación de API.

### Distribución y ciclo de release

La distribución es una intención arquitectónica, no configuración de tiendas o pipeline existente:

| Proyecto/plataforma | Canal previsto |
|---|---|
| ENSAMBRIA Business / Android | Google Play y distribución empresarial autorizada cuando corresponda. |
| ENSAMBRIA Business / iOS | App Store y mecanismos de distribución permitidos por Apple. |
| ENSAMBRIA Business / Windows | Instaladores/distribución de escritorio definidos al implementar. |
| ENSAMBRIA Business / Linux | Paquetes compatibles con los destinos/distribuciones que se aprueben. |
| ENSAMBRIA Business / macOS | Distribución de escritorio conforme a requisitos de Apple. |
| ENSAMBRIA App / Android e iOS | Google Play y App Store; versiones de tablets adaptativas. |
| Tenant Portal, Marketplace, Platform Admin y Corporate | Despliegues web independientes por propósito; no se presume proveedor, entorno ni pipeline. |

Cada aplicación conserva release, configuración y despliegue acordes a su audiencia, datos, plataforma y exposición. No se afirma que canales de tienda, firmas, instaladores, dominio, CI/CD o infraestructura estén configurados.

### Secuencia de implementación

La decisión es documental en Fase 0 y no implementa ni inicia ahora ningún proyecto. Las aplicaciones se incorporan incrementalmente en las subfases de Fase 5 y en las fases propietarias de su funcionalidad. Onboarding y paneles privados corresponden a 5.1–5.2; Tenant Portal web se entrega en 5.2 y ENSAMBRIA Business Flutter en 5.4, compartiendo contratos y paridad por capacidad sin duplicar la app ni sus reglas; las páginas públicas tenant, el flujo base de publicación y Corporate institucional corresponden a 5.3; ENSAMBRIA Marketplace, el directorio y Discovery se entregan en Fase 9.1 sobre las proyecciones tenant de 5.3. Cada recorrido debe completar contratos, seguridad, pruebas y despliegue antes de declararse aceptado.

## Alternativas evaluadas

1. **React Native para clientes móviles:** descartada para el ecosistema oficial por la decisión expresa de usar Flutter + Dart en Business y App.
2. **Flutter Web en lugar de Next.js:** descartada para las cuatro experiencias web por la necesidad de navegación web, renderizado y SEO propios de Next.js.
3. **Una sola aplicación empresarial genérica:** descartada porque escritorio instalado y navegador tienen distribución y UX distintas, aunque compartan capacidades y contratos.
4. **Separar Business y Tenant Portal como sistemas de dominio:** descartada; duplicaría autorización y reglas e introduciría APIs divergentes.
5. **Convertir cada interfaz en un backend:** descartada; contradice la topología de cuatro backends y el ownership existente.
6. **Añadir Smart TV a Flutter sin evaluación:** descartada; compatibilidad, SDK, navegación y distribución deben decidirse por plataforma.

## Consecuencias y riesgos

- Hay seis proyectos de interfaz que requieren ciclos de calidad, seguridad, observabilidad y distribución diferenciados; compartir plataforma no elimina ese costo.
- La paridad entre Business y Tenant Portal debe mantenerse por matriz de capacidades y contratos, no mediante copia de implementación o promesas de igualdad total.
- La compatibilidad Flutter por sistema operativo, distribución, capacidades nativas y soporte deberá validarse antes de cada release. Este ADR no demuestra que Flutter esté instalado o que un artefacto se construya.
- Web pública, paneles autenticados y clientes móviles tienen modelos de amenaza y reglas de publicación distintos; no se comparten políticas de caché/indexación indiscriminadamente. Las páginas públicas tenant y Corporate de 5.3 completan su publicación y SEO en esa fase; Marketplace/Discovery de Fase 9.1 consume la proyección base ya aceptada y no es una dependencia de 5.3.
- La ausencia de contrato de producto, permisos o pruebas reales mantiene bloqueado únicamente el recorrido dependiente.

## Criterios de aceptación documental

- Las seis aplicaciones, tecnología, usuarios, responsabilidades, plataformas y BFF/superficies están trazadas en el requerimiento y en la arquitectura.
- Business y Tenant Portal se describen como clientes de los mismos casos de uso empresariales, contratos y autorización, con paridad funcional medible.
- App se limita a clientes; Marketplace es web pública; Platform Admin es privada y global; Corporate es institucional y no opera tenants.
- Rust, cuatro backends, owners y canales aprobados permanecen intactos.
- Reglas de seguridad, accesibilidad, SEO, privacidad, observabilidad y pruebas distinguen adecuadamente web, móvil y escritorio.
- Roadmap y registro de Fase 0 enlazan este ADR sin marcar fases, aplicaciones o controles runtime como implementados.
- No se modifican contratos, código fuente, scripts, workflows o despliegues en esta decisión.

## Decisión del propietario y estado

El propietario instruyó formalmente adoptar las seis aplicaciones, Flutter para ENSAMBRIA Business y ENSAMBRIA App, y React + TypeScript + Next.js para las cuatro experiencias web. La decisión se registra como aceptada el 2026-11-11. La aceptación es arquitectónica/documental; no es revisión independiente, aprobación legal, aprobación de lanzamiento ni evidencia de implementación.

## Referencias

- [Requerimiento ENSAMBRIA, §8](../requerimiento.md#8-aplicación-para-clientes), [§14](../requerimiento.md#14-arquitectura-y-tecnologías) y [§22](../requerimiento.md#22-roadmap-de-implementación)
- [Contrato arquitectónico central](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios y estructura objetivo](../arquitectura-dominios.md)
- [ADR-0004: plataforma multiproducto](ADR-0004-plataforma-multiproducto.md)
- [ADR-0005: tenancy multimodal y servicios móviles](ADR-0005-tenancy-multimodal-y-servicios-moviles.md)
- [Calidad web, SEO, rendimiento y accesibilidad](../rules/ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md)
- [Seguridad, accesibilidad, UX/UI, rendimiento y observabilidad frontend](../rules/ENSAMBRIA_frontend_security_accessibility_ux_ui_performance_observability_rules.md)
