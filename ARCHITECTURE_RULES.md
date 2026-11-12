# Reglas Estrictas de Arquitectura — ENSAMBRIA con Cuatro Backends Modulares (Rust)
## DDD + Clean Architecture + Hexagonal + SOLID

> **Propósito de este documento:** este archivo es el contrato arquitectónico de ENSAMBRIA para su solución modular. El producto mantiene internamente una arquitectura de monolito modular con dominios y módulos claramente separados, y estas reglas deben respetarse al crear o modificar código.
>
> Ninguna regla aquí es "sugerencia": son restricciones obligatorias. Si una petición del usuario contradice una regla, el agente debe señalarlo antes de generar código que la viole.
>
> Los principios de capas y ownership son reutilizables; la topología y reparto de responsabilidades aquí definidos son específicos de ENSAMBRIA. Los nombres de módulos, entidades, agregados, casos de uso, eventos y recursos deben reflejar el dominio real, manteniendo los límites y salvaguardas de este contrato.

---

# 0. Objetivo estratégico (léelo primero)

## 0.0 Documentos normativos obligatorios desde el día uno

La documentación normativa del repositorio y la documentación del requisito del producto son de cumplimiento obligatorio para ENSAMBRIA. Sus requisitos se aplican desde el diseño inicial y el primer cambio a **todos los módulos, capas, APIs, frontends, infraestructura, fases y subfases** del proyecto, dentro de su alcance y responsabilidad.

No son material de consulta opcional ni una fase posterior: gobiernan planificación, diseño, implementación, pruebas, despliegue y operación desde el día uno. Cada módulo debe cumplir los requisitos aplicables completos; la calidad, seguridad, observabilidad, resiliencia, contratos y pruebas no se posponen hasta una fase transversal posterior. Un requisito solo puede declararse no aplicable con una justificación técnica concreta, verificable y documentada antes de implementar la fase/subfase afectada.

Los documentos del repositorio deben conservarse sincronizados. Si hay solapamiento, se aplican conjuntamente: estas reglas fijan la topología y los límites de ENSAMBRIA; la documentación de calidad rige el método y la evidencia de verificación; la documentación de observabilidad rige instrumentación, diagnóstico y monitoreo. Ante una incompatibilidad real, se detiene únicamente el trabajo afectado, se documenta el conflicto y se resuelve mediante decisión explícita antes de continuar; no se omite ni debilita silenciosamente un requisito.

### Autoridad y límites entre documentos

- Este archivo consolida el contrato arquitectónico, las políticas generales específicas del proyecto y la asignación de responsabilidades. Es la fuente central para esas tres áreas, sin reemplazar los documentos especializados.
- [docs/requerimiento.md](docs/requerimiento.md) determina el alcance del producto, la secuencia de fases y sus criterios de salida.
- Las guías especializadas bajo `docs/rules/` mantienen la autoridad normativa de su materia. En particular, [las reglas técnicas Rust](docs/rules/ENSAMBRIA_reglas-estrictas-de-arquitectura-monolito-modular.md) complementan este contrato con detalle de implementación modular; no son una segunda fuente para redefinir la topología, ownership de producto, cantidad de workspaces ni límites de los backends. Si un resumen allí diverge de este contrato, prevalece este archivo para esas decisiones. Las reglas técnicas exclusivas de la guía Rust se conservan dentro de ese alcance, sujetas a las fuentes especializadas correspondientes.
- [AGENTS.md](AGENTS.md) contiene instrucciones operativas para agentes y debe remitir a este contrato, no mantener una política paralela.

### Capacidades transversales obligatorias desde Fase 0

Las diez capacidades tecnológicas de [docs/requerimiento.md, sección 14](docs/requerimiento.md) forman parte del diseño arquitectónico inicial: aislamiento multi-tenant, operación multi-sucursal, mercado/moneda, idioma/localización, reglas de dominio, eventos, auditoría, API pública, webhooks y Marketplace de extensiones. Su alcance, ownership, límites de confianza, contratos, dependencias, secuencia funcional, decisiones pendientes y criterios de aceptación deben quedar definidos documentalmente en Fase 0 antes de implementar los comportamientos dependientes. Fase 0 no habilita runtime, integraciones externas, mercados, monedas, APIs, webhooks ni extensiones.

### Tenancy y operación empresarial multimodal

El modelo de [requerimiento §2.2](docs/requerimiento.md) conserva un tenant estable como ámbito técnico de aislamiento, sin exigir persona jurídica, sucursal, establecimiento físico o personal. CORE posee identidad global, titularidad/membresías explícitas, tenant y relaciones estructurales de organizaciones/sucursales; organización operativa no equivale a persona jurídica y sucursal no equivale a establecimiento. BUSINESS posee el perfil y las reglas de la actividad, catálogo de servicios/modalidades, establecimientos operativos, workforce, cobertura y desplazamiento. CORE conserva el estado común de reserva y su ubicación histórica mínima; BUSINESS revalida elegibilidad y restricciones operativas por contrato. PLATFORM conserva relación comercial/planes y PUBLIC sirve únicamente proyecciones expresamente autorizadas. Ninguna identidad, titularidad, jerarquía, modalidad, ID o dirección concede permisos fuera del scope autenticado. La modalidad de registro es mutable configuración operativa, no tipo de tenant, backend o límite de autorización. Fase 0 diseña estos límites sin runtime; su detalle de contratos, pruebas y fases está en el [modelo de dominios](docs/arquitectura-dominios.md).

### Plataforma tecnológica multiproducto

ENSAMBRIA es una plataforma tecnológica compartida para productos SaaS comerciales independientes. Beauty & Wellness es el producto/vertical funcional inicial; los otros productos y las veinte industrias enumeradas en el [modelo de dominios](docs/arquitectura-dominios.md) son horizonte de diseño, no capacidades implementadas ni autorización de lanzamiento. La estrategia multiproducto es una decisión arquitectónica distinta de las diez capacidades tecnológicas C01–C10: se traza separadamente y no altera su número.

Un **producto** compone capacidades compartidas, verticales aprobados, marca/configuración, dominios, frontend(s), BFF/API, mercados, planes/entitlements, contratos e integraciones autorizadas. **Industria**, **vertical** y **marca** son conceptos separados: un vertical mantiene su lenguaje y reglas en su módulo propietario; una marca no es regla de negocio ni límite de autorización; varios tipos de establecimiento dentro de un mismo producto no generan automáticamente productos nuevos. No se crean backends por producto/industria ni una entidad universal que mezcle dominios.

Ownership y límites:

- **PLATFORM** es dueño del registro/catálogo comercial de productos, identidad/configuración de producto y marca versionadas, dominios autorizados, capacidades declaradas y configuración de mercado/planes/oferta comercial. No posee la identidad ni el aislamiento técnico de tenants.
- **CORE** es dueño del contexto de producto validado, la asociación técnica tenant–producto y sus efectos en identidad, tenancy y autorización. No posee el catálogo, precios, planes ni branding comercial. Un dominio, header, `product_id` o identidad aislados nunca autorizan acceso.
- **BUSINESS** conserva CRM, workforce, catálogo operativo, inventario, Commerce, membresías, marketing y reglas/datos verticales. Las capacidades operativas permanecen en `backend/business/modules/`; la lógica especializada se agrupa por industria en `backend/business/verticals/<industria>/`. `establishment-catalog-verticals` es dueño del registro común del servicio (identidad estable, establecimiento asociado, nombre, estado de ciclo de vida y versión); cada vertical posee sus definiciones, atributos, parámetros y reglas especializados, relacionados con ese registro mediante contratos publicados. `backend/business/verticals/beauty-wellness/README.md` documenta el vertical inicial, sin runtime. Cada vertical conserva sus bounded contexts y consume módulos comunes mediante contratos, sin distribuir condicionales de industria por módulos compartidos ni duplicar capacidades comunes.
- **PUBLIC** posee Discovery/Marketplace, atribución y proyecciones públicas autorizadas con scope de producto; no lee datos privados fuente ni opera la configuración comercial o reglas verticales.
- **Cada producto** tiene contrato/API de BFF y frontend especializados. El BFF es una unidad de presentación/API, no un quinto backend de dominio; no contiene reglas de negocio ni accede a stores. El BFF de producto invoca por gRPC los contratos autorizados de CORE, BUSINESS, PLATFORM y PUBLIC. El BFF público conserva Discovery y proyecciones; la consola global conserva el BFF integrado de PLATFORM.

El contexto sigue una secuencia validada —dominio registrado → producto → asociación tenant–producto en CORE → organización/sucursal/actor autorizados— sin confiar en el Host ni resolver toda la cadena en una consulta única. El propietario del dato revalida el scope en cada operación y RLS se aplica a sus filas tenant-scoped. Identidades globales pueden ser comunes, pero no comparten membresías, sesiones de producto, datos, credenciales ni permisos implícitos: la autenticación no habilita otro producto. Cachés, objetos, índices, eventos, jobs, secretos, configuración, auditoría y telemetría mantienen el aislamiento requerido.

PLATFORM entrega la configuración autoritativa por contratos autenticados y versionados; los dominios públicos solo se habilitan tras validar propiedad/asociación. Los contratos de producto y su versión se fijan en la composición/build del BFF; las actualizaciones persistidas que necesiten distribución durable usan Outbox/JetStream e Inbox idempotente, no RPC/eventos usados como sustitutos. Los cambios de marca/configuración nunca alteran autorización. Un producto futuro requiere descubrimiento, owner, contratos compatibles, capacidades implementadas, pruebas de aislamiento, seguridad, SEO/privacidad, soporte operativo y aprobación comercial antes de habilitarse.

Fase 0 define estas relaciones, matrices, contratos y criterios sin crear catálogo runtime, apps, carpetas vacías, credenciales o infraestructura de producto. La implementación del contexto/producto inicial debe ocurrir antes de aceptar registros autoservicio que dependan de él; las superficies de cada producto se implementan en la fase existente que introduce sus recorridos. No se adelantan productos futuros ni se declara independencia de despliegue como implementación por existir en el diseño.

- CORE conserva identidad/contexto, membresías tenant-scoped, autorización/aislamiento y estado de reservas/pagos; BUSINESS es propietario de la operación, reglas, datos y configuración por establecimiento/sucursal; PLATFORM es propietario de la relación comercial SaaS, planes, entitlements y políticas comerciales ENSAMBRIA; PUBLIC solo expone proyecciones expresamente publicadas y es propietario de la atribución del canal. No se transfieren estas responsabilidades por una integración.
- Toda capacidad transversal se implementa en el owner del comportamiento: no crear un motor de reglas que apropie decisiones de varios dominios, no permitir acceso directo a tablas, no crear otro backend por la capacidad, no ejecutar código de extensión arbitrario dentro de backends y no tratar el evento o webhook como RPC.
- La jurisdicción, moneda, idioma, permisos y configuración se propagan como contexto validado; no implican aprobación de obligaciones legales, conversión monetaria, precio, tarifa ni regla comercial. Las obligaciones legales se investigan y documentan desde el desarrollo; la revisión profesional no es requisito general para redactar o integrar documentación ni para avanzar trabajo independiente. No se declaran validadas sin evidencia, y solo se bloquea la operación o tratamiento que dependa de una validación previa concreta. Las políticas comerciales requieren aprobación del owner de producto correspondiente.
- Los eventos de negocio representan hechos persistidos y siguen el contrato Outbox/Inbox; la auditoría de dominio es append-only y distinta de los logs operativos; las APIs/webhooks y extensiones tienen autenticación, scopes, minimización, límites, versionado, revocación y pruebas de compatibilidad/abuso según la fuente propietaria.
- Ninguna fase se acepta solo por documentar esta arquitectura. Desde Fase 1 se implementan los cimientos compartidos requeridos por los casos reales; las capacidades completas se entregan por dominio y en la fase indicada en la sección 14 del requerimiento. Los controles aplicables se prueban en el mismo corte funcional; una decisión pendiente bloquea únicamente el alcance que depende de ella.

La guía especializada [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](docs/rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md) es obligatoria para requisitos y diseño de privacidad/ciclo de vida de datos, junto con las reglas de seguridad y calidad. Su Fase 0 es una línea de trabajo PDLM, no una excepción a la Fase 0 documental de ENSAMBRIA: esta última aprueba responsables, inventario, jurisdicciones, contratos, amenazas y ADRs; no implementa esquemas, migraciones ni runtime. Las recomendaciones marcadas como propuestas requieren ADR aprobado antes de implementar el alcance dependiente. La investigación y redacción legal continúan sin aprobación profesional; la revisión por jurisdicción se registra por separado y solo bloquea un tratamiento cuando una obligación concreta exija validación previa.

La política [ENSAMBRIA_Privacy_Legal_Compliance.md](docs/rules/ENSAMBRIA_Privacy_Legal_Compliance.md) también es obligatoria desde Fase 0 y complementa PDLM con el registro de cumplimiento por jurisdicción, roles de tratamiento, consumidores/menores, consentimiento y cookies, marketing, proveedores/transferencias, incidentes, consumidores y habilitación de mercados. Fase 0 documenta alcance, fuentes oficiales, aplicabilidad pendiente, responsables, decisiones, controles y trazabilidad; no implementa runtime. Desde la primera fase de implementación se incorporan y prueban los controles aplicables al comportamiento introducido, junto con los documentos y el aviso visible de revisión profesional pendiente cuando corresponda. Ninguna guía equivale a asesoría, aprobación legal o certificación: no se afirma validación sin evidencia y solo se bloquea el tratamiento o mercado dependiente de requisitos críticos pendientes.

La política [ENSAMBRIA_Geographic_Access_Policies.md](docs/rules/ENSAMBRIA_Geographic_Access_Policies.md) rige las decisiones de disponibilidad por mercado y ubicación. Fase 0 define superficies, capacidades, evidencias, propietarios y pruebas sin runtime; las restricciones se implementan con la capacidad propietaria y solo sobre la operación afectada. No bloquear globalmente páginas informativas ni solicitudes de privacidad por país; la IP no basta para una restricción estricta por ciudad, que exige verificación adicional proporcional y aprobada.

La guía especializada [ENSAMBRIA_Connection_Pooling_PostgreSQL.md](docs/rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md) rige el diseño, dimensionamiento, seguridad, operación y pruebas de conexiones PostgreSQL. ENSAMBRIA utiliza una única base de datos PostgreSQL compartida por los cuatro backends; cada módulo conserva schema, ownership, migraciones y roles mínimos independientes. Fase 0 documenta y valida la topología y el presupuesto sin introducir runtime de producto. Cada proceso mantiene un pool acotado y credenciales propias; los workers tienen límites separados; se dimensiona el presupuesto para el máximo de réplicas con margen operativo; se usan timeouts y el contexto RLS se limita a la transacción. No se crean pools por petición ni por tenant. Los valores concretos requieren medición de la infraestructura real; PgBouncer es opcional y solo se adopta con evidencia y pruebas de compatibilidad. Compartir la base no concede acceso SQL cruzado ni cambia el ownership de schemas.

La guía [ENSAMBRIA_Transactional_Outbox_Inbox.md](docs/rules/ENSAMBRIA_Transactional_Outbox_Inbox.md) define la integración durable de hechos persistidos. Fase 0 documenta owners, contratos/versiones, datos mínimos y privacidad, orden, idempotencia, retención/replay, recuperación y pruebas, pero no crea runtime. **La implementación de Outbox/Inbox comienza obligatoriamente en Fase 1, desde su primer corte ejecutable**, integrada en al menos un flujo real que requiera integración durable; no se deja solo diseñada ni se difiere a una fase posterior. Para cada evento aplicable, el módulo/backend propietario confirma el cambio de negocio y su Outbox en una transacción PostgreSQL local; un publicador recuperable lo transmite con confirmación del broker, y los consumidores con efectos persistentes deduplican con Inbox en la misma transacción que el efecto. Se asume entrega al menos una vez, no exactamente una vez. No se publica al broker dentro de la transacción ni se crean eventos artificiales. Cada fase posterior aplica estas garantías desde el primer cambio que requiera entrega durable.

La guía especializada [ENSAMBRIA_Open_Graph_SEO_Metadata.md](docs/rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md) es obligatoria para páginas públicas y contenido publicable. Su diseño, superficies, ownership, estados de publicación, requisitos de renderizado y pruebas se definen en Fase 0; la capacidad técnica base de SEO/metadata se implementa en Fase 1. Desde Fase 1 en adelante, toda fase que introduzca o modifique una página o contenido público debe entregar en la misma fase sus metadatos completos y pruebas de rastreo/preview; no se aplaza a las fases 5 o 9 cuando la superficie ya se expuso antes. No se indexan rutas autenticadas, borradores ni datos privados.

El estándar [ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md](docs/rules/ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md) gobierna la calidad web general y la preparación de lanzamiento de todos los frontends. [ENSAMBRIA_frontend_security_accessibility_ux_ui_performance_observability_rules.md](docs/rules/ENSAMBRIA_frontend_security_accessibility_ux_ui_performance_observability_rules.md) complementa los controles frontend de seguridad, accesibilidad, UX/UI, rendimiento, observabilidad, pruebas y aplicaciones móviles. Las pruebas de capacidad se rigen además por [ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md](docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md). Estas fuentes no reemplazan [ENSAMBRIA_Privacy_Legal_Compliance.md](docs/rules/ENSAMBRIA_Privacy_Legal_Compliance.md), fuente de verdad para privacidad/cumplimiento legal, [ENSAMBRIA_Open_Graph_SEO_Metadata.md](docs/rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md), fuente de verdad para SEO técnico/Open Graph, ni [ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md](docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md), fuente de verdad de observabilidad del sistema. Fase 0 registra alcance, superficies, responsables y criterios; las pruebas y requisitos aplicables se cumplen con cada entrega desde la fase de implementación correspondiente.

La guía [ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md](docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md) es la fuente normativa de observabilidad, monitoreo, errores, alertas, SLI/SLO y operación. Fase 0 define su arquitectura y criterios sin runtime; la instrumentación aplicable se implementa y prueba desde el primer corte ejecutable de Fase 1 y con cada fase posterior que introduzca componentes o comportamientos. Esta guía también define el procedimiento de implementación y verificación; no existe un prompt separado de observabilidad.

El estándar [ENSAMBRIA_README_Documentation_Quality_Security_Standard.md](docs/rules/ENSAMBRIA_README_Documentation_Quality_Security_Standard.md) es la fuente normativa para la presentación, contenido y seguridad del README raíz. Aplícalo al mantener ese documento alineado con el estado real y las guías especializadas, sin trasladar al README contratos o detalles internos.

ENSAMBRIA es una **plataforma tecnológica compartida para productos SaaS comerciales independientes**, con **cuatro backends de dominio**, coordinados desde el mismo repositorio y workspace Cargo raíz. Compartir workspace, resolución de dependencias y `Cargo.lock` solo facilita desarrollo y build; no comparte runtime, composición, configuración, secretos, permisos, datos ni despliegue. Cada capacidad tiene un único backend propietario de sus reglas de negocio y datos autoritativos:

1. **CORE (`backend-core`):** identidad/autenticación, tenancy y contexto de seguridad, organizaciones/sucursales centrales, evaluación runtime de entitlements, reservas, procesamiento de pagos, comisiones transaccionales aprobadas, ledger inmutable, liquidaciones y conciliación.
2. **BUSINESS (`backend-business`):** operaciones de establecimientos, workforce, catálogo de servicios, clientes/CRM, membresías/fidelización, inventario, Commerce/carritos/pedidos, marketing comercial, facturación comercial tenant, franquicias/regalías, comisiones de empleados y analítica operativa.
3. **PLATFORM (`backend-platform`):** administración global, catálogo y configuración comercial de productos SaaS, gestión comercial de tenants, planes/contratos/suscripciones y facturación SaaS, configuración de capacidades, políticas comerciales de ENSAMBRIA y tarifas de pagos/Marketplace, soporte y auditoría.
4. **PUBLIC (`backend-public`):** directorio, Discovery/Marketplace, páginas y proyecciones públicas autorizadas, reseñas, promociones publicadas y atribución del canal. No posee servicios, empleados, inventario, pedidos ni operación privada de BUSINESS.

En PLATFORM, el catálogo de ofertas/planes y precios permanece en `commercial-policies-monetization`; los contextos `saas-subscriptions`, `saas-billing`, `saas-invoices` y `electronic-invoicing` son módulos independientes. La factura SaaS y su transmisión fiscal no son el ledger ni el procesamiento del pago: CORE conserva pagos/cargos y ledger. Factus SaaS pertenece a PLATFORM; Factus de cada establecimiento sigue en BUSINESS. Los límites, contratos y fases se detallan en [ADR-0007](docs/adr/ADR-0007-separacion-contextos-facturacion-saas.md) y el [modelo de dominios](docs/arquitectura-dominios.md#23-platform--backend-platform).

CORE posee la identidad/contexto/aislamiento del tenant; PLATFORM su relación comercial; BUSINESS ejecuta la operación; PUBLIC consume solo datos expresamente autorizados para publicar. BUSINESS es dueño de Commerce y de las reglas de las ventas; PUBLIC del descubrimiento y atribución. PLATFORM define las políticas comerciales; CORE valida y registra su efecto transaccional. Comisiones de empleados y regalías pertenecen a BUSINESS.

El registro e inicio de sesión de operadores de tenant y clientes finales son autoservicio en las superficies del producto correspondiente, mediante su BFF especializado. El BFF valida/resuelve la configuración de producto registrada con PLATFORM y usa contratos autorizados de CORE/BUSINESS para identidad, tenancy y ficha CRM, sin apropiarse del ownership. CORE autentica la identidad y crea el tenant operativo asociado al producto validado; BUSINESS conserva la ficha CRM del cliente y su relación autorizada con el tenant. Una sesión autenticada no crea por sí sola membresía, autorización ni acceso CRM. Los administradores de PLATFORM no crean manualmente tenants, cuentas de operadores ni cuentas de clientes; su relación comercial de tenant se inicia automáticamente por el hecho durable ratificado en ADR-0002, que relaciona el tenant con su producto. Ninguna página accede directamente a CORE ni a stores de infraestructura.

### Identificadores y separación de identidad/membresía

UUID v7 es el identificador canónico predeterminado para entidades distribuidas o referenciadas entre dominios, incluidas las identidades globales, tenants, organizaciones y entidades de negocio centrales. CORE, BUSINESS, PLATFORM y PUBLIC conservan el mismo UUID al propagar una referencia por contratos/eventos; no la sustituyen por IDs locales. La política no obliga a agregar columnas duplicadas: cada entidad debe definir su clave una vez según su ownership y relaciones.

La identidad autenticada de una persona es global y pertenece a CORE. Su pertenencia a cada tenant es una relación propia y acotada al tenant, con estado, roles/permisos y ciclo de vida independientes; no se duplica la identidad por trabajar en varios negocios ni se confunde una ficha CRM/cliente de BUSINESS con una cuenta autenticada. Los IDs de usuario y de tenant en esa relación conservan sus UUID canónicos. Cualquier asociación de registros de dominio a una persona debe usar contratos/referencias autorizados, no inferir identidad por datos personales.

El UUID canónico se usa también en contratos externos salvo que exista una necesidad de producto documentada para un identificador de presentación distinto (por ejemplo, un slug público estable). No se crea otro ID público por costumbre. Un `BIGINT` secuencial solo se permite como clave/índice local cuando una ventaja de rendimiento, almacenamiento o compatibilidad haya sido medida y documentada; nunca sustituye el UUID canónico en APIs, eventos o referencias entre backends, y nunca se expone como protección de un recurso. IDs públicos, UUID incluidos, no son secretos ni capacidades de autorización: cada operación valida identidad, permisos, tenant y alcance.

Cada entidad tiene una única autoridad de generación de UUID. Por defecto, lo genera la aplicación Rust del backend propietario con UUID v7. La generación en PostgreSQL solo se habilita cuando la versión exacta del servidor y su soporte se hayan verificado y documentado, y se mantenga una política uniforme para esa entidad en todos los entornos. No se debe depender de `uuidv7()` sin validar la versión desplegada. UUID v4 u otro formato requiere una justificación concreta, por ejemplo ocultar la temporalidad en una superficie específica; si se necesita esa propiedad, se prefiere un identificador de presentación separado en lugar de reemplazar IDs canónicos distribuidos sin motivo.

Aplicación por backend (sin trasladar ownership ni duplicar IDs de entidades ajenas):

| Backend | Ejemplos de entidades propias con UUID v7 canónico |
|---|---|
| CORE | Usuarios, tenants, organizaciones, sucursales centrales y membresías de tenant |
| BUSINESS | Reservas, registros CRM de clientes, empleados, pedidos y membresías comerciales |
| PLATFORM | Suscripciones, planes y registros administrativos |
| PUBLIC | Perfiles públicos, publicaciones y reseñas; las proyecciones conservan además las referencias canónicas de sus fuentes |

El **BFF público** es una aplicación independiente, con proceso y despliegue propios, que adapta REST/OpenAPI/WebSocket y consume PUBLIC por gRPC; no es un backend de dominio adicional. Cada producto implementado tiene un BFF especializado independiente, sin ownership de dominio ni acceso a stores, que consume por gRPC contratos autorizados de sus backends propietarios. El BFF de administración permanece inicialmente integrado en PLATFORM. CORE no se expone directamente a navegadores ni aplicaciones. Las superficies tenant/business/móvil de cada producto usan su BFF de producto; Discovery usa el BFF público y la consola global usa PLATFORM.

### SEO obligatorio en todas las superficies frontend

Todo frontend debe tener una estrategia SEO/descubribilidad definida, implementada y probada desde su primera entrega. En las superficies web públicas e indexables, esto exige renderizado rastreable (SSR/prerender cuando aplique), title/description por ruta y locale, URL estable, canonical, hreflang cuando aplique, Open Graph/social, sitemap, robots, enlaces y datos estructurados acordes con el contenido publicado. Las fichas públicas de establecimientos, servicios y productos deben poseer metadatos editoriales completos, imágenes autorizadas con alt y datos estructurados del tipo correcto, sin inventar precios, disponibilidad, reseñas ni otra información de dominio. El renderizado debe ser mobile-friendly y evitar problemas de rastreo, enlaces rotos y contenido duplicado.

Los límites de ownership se mantienen: BUSINESS posee catálogo, contenido y campos SEO de sus servicios/productos tenant; PUBLIC solo proyecta y publica lo autorizado por BUSINESS; PLATFORM gestiona SEO del contenido institucional o catálogos globales que hayan sido aprobados como propiedad de ENSAMBRIA. Un administrador global que modera o ayuda a editar una ficha tenant actúa con permiso delegado y auditoría, sin apropiarse del catálogo. Los administradores pueden editar o previsualizar contenido publicable dentro de sus permisos, pero borradores no se exponen. PUBLIC no deriva ni amplía la autorización de publicación.

Paneles administrativos, interfaces autenticadas, portales de empleados y otros datos privados no se indexan: deben establecer `noindex`/`nofollow` cuando corresponda, excluirse de sitemaps y evitar contenido sensible en HTML, metadata, previews y JSON-LD. Las aplicaciones móviles nativas aplican metadatos de tienda y deep/universal links (ASO), no SEO web rastreable. La decisión de indexación es por ruta/contenido, no por el nombre del frontend; una app puede incluir áreas públicas indexables y áreas privadas no indexables.

La Fase 0 inventaría superficies, rutas públicas/privadas, dominios tenant, idiomas, ownership editorial, estados de publicación, controles de privacidad y contratos/casos de prueba conforme a [ENSAMBRIA_Open_Graph_SEO_Metadata.md](docs/rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md). La capacidad técnica base (renderizado rastreable y metadata/Open Graph a partir de contratos autorizados) se implementa desde Fase 1. Cada página pública recibe SEO completo en la misma fase que la introduce; sitios tenant previstos en Fase 5 y directory/Marketplace/productos/servicios públicos previstos en Fase 9 se cubren en esas fases solo si no fueron expuestos antes. Si una fase anterior expone cualquiera de esas superficies, no se difiere su SEO. La aceptación automatiza metadatos únicos/localizados, canonical/host tenant, hreflang cuando corresponda, Open Graph/Twitter Cards, sitemap/robots, renderizado rastreable, schema validado, estados 404/410 y exclusión de borradores y datos privados; no se fija una puntuación SEO o rendimiento no medida.

Cada backend es un monolito modular y un límite de despliegue independiente. Dentro de cada backend:

- Cada módulo/bounded context conserva ownership, contratos, persistencia y pruebas propias dentro del backend dueño. El límite de módulo no exige un crate Cargo independiente; los paquetes se dividen solo ante una necesidad demostrable de composición, prueba, despliegue o aislamiento, sin crear placeholders.
- Cada módulo contiene sus modelos de dominio/persistencia, casos de uso, puertos, adapters y pruebas correspondientes a su responsabilidad. Su API/contrato público es explícito, esté implementado dentro de un paquete compartido o separado; sus repositorios/SQL solo acceden a datos que posee y no consultan directamente tablas internas de otro módulo.
- **Ningún crate de módulo depende ni importa código interno de otro módulo directamente.** La comunicación entre módulos se hace mediante el contrato público del módulo, con gRPC para una respuesta síncrona inmediata, o mediante eventos de integración durables con Outbox + NATS JetStream cuando se distribuyen hechos persistidos.
- Los crates técnicos neutrales del workspace (por ejemplo PostgreSQL, gRPC, seguridad, Vault, mensajería y observabilidad) sí pueden ser dependencias comunes explícitamente aprobadas. Reutilizan mecanismos técnicos, no casos de uso, entidades, políticas ni lógica de dominio. No se crea un crate genérico que termine siendo dueño de todos los servicios o reglas.
- Entre backends no hay imports de implementación ni acceso cruzado a tablas. La comunicación de runtime se limita a:
  - **gRPC** para consultas/comandos que necesitan resultado inmediato. Se autentica con identidad criptográfica de servicio y mTLS, contratos versionados, autorización de mínimo privilegio, deadlines, límites y reintentos seguros. La autorización del usuario/tenant se valida en el backend dueño; cualquier contexto delegado se verifica y no se toma como confiable por estar en el request.
  - **NATS JetStream + Outbox** para hechos de negocio ya persistidos que deban procesarse de manera durable y desacoplada. El Outbox se escribe en la misma transacción local que el cambio del backend dueño; los consumidores son idempotentes, validan el evento y confirman el ACK después de persistir sus efectos.
- La selección del transporte depende de la semántica: gRPC no reemplaza eventos durables y JetStream no sustituye una consulta/comando síncrono. No se publica un evento cuando no existe una necesidad real de integración; los backends permanecen independientes cuando no se requieren coordinación ni transferencia de información.
- Eventos y RPC no transportan secretos ni datos innecesarios. Se protegen con identidad/autorización de servicio, contratos versionados, límites y correlación de `correlation_id`/`trace_id`. Los errores son tipados y se registran sin credenciales, tokens ni PII innecesaria.
- Los cuatro backends usan una única base de datos PostgreSQL compartida. Cada backend y módulo tiene schemas, roles de base de datos, migraciones y ownership separados. No se permiten JOINs, consultas, escrituras ni comunicación cruzada por SQL. PostgreSQL compartido no es un canal de integración.
- Cada proceso/backend mantiene pools acotados, propios de su identidad y presupuesto; los workers tienen límites de concurrencia/conexiones separados para no agotar recursos críticos. El presupuesto agregado debe incluir máximas réplicas, workers, migraciones, administración y margen operativo. Toda configuración real, límites, timeouts, comportamiento de saturación, TLS y RLS se valida según [ENSAMBRIA_Connection_Pooling_PostgreSQL.md](docs/rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md) con dependencias reales; ningún valor ilustrativo de esa guía se toma como configuración aprobada.
- Cuando un hecho persistido deba distribuirse de forma asíncrona, el cambio de dominio y su Outbox se confirman atómicamente; el publicador no retiene locks/transacciones durante llamadas al broker. JetStream/Outbox sirve para hechos durables y replay; NATS Core solo para fan-out efímero realtime donde corresponda. Cada consumidor con efecto persistente usa Inbox/idempotencia y confirma el mensaje después del commit local. Los envelopes son versionados, mínimos y sin secretos/PII innecesaria; no se admite acceso a Outbox/Inbox ajenas ni transacción SQL entre módulos/backends.
- El workspace Cargo raíz puede compartirse; cada backend conserva su composición, binario, configuración, secretos, CI, artefactos, despliegue y operación. Ninguna imagen o unidad de despliegue arranca más de un backend.

Hay cuatro backends de dominio desplegables. El BFF público y cada BFF de producto implementado son aplicaciones independientes adicionales; el BFF de administración se ejecuta inicialmente dentro de PLATFORM. Los módulos internos de los cuatro backends no se despliegan por separado.

### Aclaraciones de topología, persistencia y evolución

- Los cuatro backends usan una única base de datos PostgreSQL compartida; no se crea una base independiente por backend o módulo. Esta decisión no exige cuatro servidores ni cuatro clústeres: la infraestructura que aloja esa base se dimensiona según capacidad y aislamiento operativo, sin alterar la topología lógica obligatoria. La separación de seguridad es obligatoria desde Fase 0: cada módulo posee su schema/tablas/migraciones; existen roles de runtime/migración y grants mínimos por backend/dominio; cada proceso usa credenciales limitadas a los datos que necesita; ningún backend recibe acceso irrestricto al conjunto de tablas. Una misma infraestructura de conexión o biblioteca común no concede permisos SQL. Los límites de módulos del mismo backend se refuerzan mediante contratos públicos, ownership de repositorios/SQL y controles que impidan consultas directas a tablas ajenas.
- Los seis proyectos frontend no significan backends adicionales. ENSAMBRIA Business y Tenant Portal usan el mismo BFF/API de producto; ENSAMBRIA App usa la superficie de producto y puede consumir Discovery mediante el BFF público; Marketplace usa el BFF público hacia PUBLIC; Platform Admin usa el BFF integrado en PLATFORM; Corporate publica contenido institucional estático o mediante una superficie pública autorizada. Los BFF adaptan HTTP/OpenAPI/WebSocket a contratos de los backends propietarios; no poseen reglas ni datos de dominio ni acceden directamente a stores. CORE no se expone directamente a frontends.
- No se crea un microservicio, binario o despliegue por cada módulo. El objetivo inicial son cuatro monolitos modulares con límites y despliegues independientes. La extracción selectiva de un módulo se considera posteriormente solo ante una necesidad demostrable de escalado, aislamiento, disponibilidad, autonomía de equipo o despliegue, documentada y aprobada; no cambia su ownership de dominio.
- Cada módulo de dominio definido conserva sus modelos y reglas propios, casos de uso, puertos, adapters, persistencia/migraciones, contratos y pruebas. Los módulos no consultan directamente tablas internas de otros dominios, aunque compartan proceso o servidor PostgreSQL; coordinan mediante el contrato público apropiado.
- La Fase 0 fija la arquitectura objetivo de los cuatro backends, sus responsabilidades, dependencias, contratos y secuencia de fases. Es una fase de definición/auditoría, no una instrucción para implementar los cuatro backends completos en paralelo. Las capacidades se implementan gradualmente conforme al roadmap, sin alterar los límites aprobados ni omitir los criterios integrales de la fase activa.

## Capas de la arquitectura completa (de afuera hacia adentro)

1. **Frontend** (web/mobile/desktop cuando corresponda):
   - sin lógica de negocio;
   - consume los endpoints **REST/HTTP** del BFF cuando exista BFF;
   - maneja presentación, estado de UI y validaciones superficiales de entrada.

2. **BFF (REST API Gateway)**:
   - sin lógica de negocio;
   - implementado con un framework HTTP de alto rendimiento (ej. `axum` o `actix-web`);
   - adapta y compone respuestas REST;
   - el BFF público independiente se comunica con PUBLIC mediante contratos **gRPC** (`tonic`);
   - cada BFF de producto independiente consume los contratos autorizados de los backends propietarios; el BFF de administración integrado en PLATFORM usa contratos públicos de aplicación sin acceso directo a stores ni implementación de otros módulos;
   - CORE no se expone directamente a frontends; los consumidores invocan únicamente las superficies HTTP autorizadas.

3. **Módulos de cada monolito modular (Crates)**:
   - dueños de las reglas y casos de uso de su bounded context;
   - CORE posee ejecución de pagos, comisiones transaccionales aprobadas, ledger, liquidaciones y conciliación;
   - PLATFORM posee suscripciones y facturación SaaS; BUSINESS posee pedidos y facturación comercial tenant; sus pagos se ejecutan mediante contratos de CORE;
   - BUSINESS posee Commerce, operación de establecimientos, comisiones de empleados y regalías; PUBLIC posee descubrimiento, proyecciones públicas autorizadas y atribución Marketplace;
   - dueños de sus schemas, tablas y migraciones;
   - dueños de su persistencia;
   - exponen su `module_public_api` como servicio **gRPC** (`tonic`);
   - se comunican entre sí de forma asíncrona mediante **NATS JetStream** (`async-nats`) cuando corresponda.

4. **Infraestructura transversal compartida**:
   - bibliotecas técnicas neutrales del workspace para PostgreSQL/SQLx, transacciones/contexto de tenant, gRPC/Protobuf, TLS/seguridad, errores técnicos, Redis, NATS Core/JetStream/Outbox, Vault y observabilidad;
   - bootstrap y configuración tipada por proceso (`platform::config`);
   - componentes comunes reutilizables, sin ownership de reglas, estados o datos de dominio.

El detalle completo de BFF/REST/Frontend está en la sección 9. Los cuatro backends pertenecen al workspace raíz, pero no se crea una dependencia de implementación entre sus crates de dominio. Pueden depender de bibliotecas técnicas neutrales aprobadas y de contratos Protobuf versionados; esas dependencias no contienen lógica de dominio. La comunicación de runtime entre backends usa gRPC síncrono, NATS Core para fan-out efímero o eventos durables con Outbox/JetStream según la semántica.

### Infraestructura compartida del workspace

ENSAMBRIA implementa la infraestructura técnica reutilizable como crates neutrales del workspace, sin convertirla en un monolito de utilidades ni en un backend de dominio. Los componentes pueden comenzar como módulos de uno o varios crates técnicos acotados y separarse en crates propios cuando lo justifiquen el ownership, dependencias, versionado, pruebas o reutilización real; la selección de granularidad no puede producir duplicación de mecanismos entre CORE, módulos verticales y backends.

La arquitectura permite extensiones controladas, no ejecución de código arbitrario ni plugins que evadan seguridad/ownership. Cada extensión debe registrarse explícitamente en el composition root, declarar contrato y versión, capacidades verificadas, permisos, compatibilidad, límites de recursos y auditoría; su activación por tenant queda sujeta a entitlement y autorización del backend propietario. No se debe modificar el CORE para cada proveedor/extensión compatible ni delegar a una extensión la propiedad de datos de otro módulo.

| Componente técnico | Responsabilidad compartida | Límite que debe conservar |
|---|---|---|
| `database` | Configuración validada de SQLx, creación del pool, health/timeout y acceso tipado a transacciones | Un pool por proceso de aplicación; módulos dueños de sus repositorios, schemas, SQL y migraciones |
| `transactions` | Helpers para comenzar/confirmar/revertir transacciones y componer operaciones atómicas | No decide reglas, límites ni efectos de dominio; cada caso de uso y módulo define la unidad de trabajo necesaria |
| `tenant_context` | Contexto autenticado tipado de actor, tenant, sucursal y correlación; helpers para establecerlo transaccionalmente en PostgreSQL/RLS y propagarlo en gRPC | Se deriva de identidad verificada, nunca de un `tenant_id` libre del cliente; cada módulo sigue siendo responsable de autorizar su operación |
| `grpc` | Bootstrap Tonic, canal, límites, deadlines, interceptores comunes y propagación W3C | No contiene lógica de aplicación ni decisiones de autorización del dominio |
| `proto` / contratos | Convenciones y tooling de generación, metadata común no de dominio | Cada bounded context es dueño de su contrato `.proto`, package/version y compatibilidad; no hay un proto monolítico ni entidades compartidas |
| `grpc_security` / `tls` | Validación de identidad de servicio, mTLS, carga/renovación de certificados emitidos por PKI y hooks de autenticación/autorización comunes | Los permisos por método y contexto se definen explícitamente por servicio; ningún interceptor confía en claims sin validar |
| `errors` | Errores técnicos tipados y utilidades seguras para clasificarlos | El módulo dueño conserva errores de dominio y define su mapeo gRPC/HTTP; nunca se colapsan detalles diagnósticos seguros en un error genérico |
| `observability` | Logs, métricas, tracing, propagación de contexto y redacción común | No es dueño de datos ni reglas de dominio; la cardinalidad y privacidad se controlan en cada frontera |
| `outbox` / `messaging` | Primitivas de escritura/publicación confiable, relay y clientes NATS Core/JetStream | El módulo dueño conserva su tabla/schema Outbox, transacción, envelope/versionado, audiencia, idempotencia y política de ACK/replay |
| `secrets` | Cliente/adaptador técnico para Vault Agent/API, renovación controlada, certificados y manejo de leases | No administra el Vault compartido ni lee secretos de otro workload; autorización y secretos se limitan a rutas ENSAMBRIA asignadas |

La infraestructura se registra en composition roots distintos por proceso. Los procesos de los cuatro backends, el BFF público y workers separados tienen pools, configuración, identidad, credenciales y ciclo de vida propios. Si varios módulos se ejecutan dentro del mismo proceso de backend, reciben el mismo `PgPool` de ese proceso por inyección; no crean pools por módulo. El BFF público, al ser un proceso independiente sin acceso a PostgreSQL, no obtiene ese pool. Al extraer un módulo o BFF a otro proceso, el nuevo proceso crea su propio pool desde la misma biblioteca común; nunca comparte memoria ni conexiones de pool con el proceso original.

RLS se aplica en transacciones PostgreSQL: el contexto de tenant autenticado se fija como configuración local de transacción antes de consultas tenant-scoped y se limpia automáticamente al terminar la transacción, incluso con conexiones pooled. No se establece un tenant persistente en la sesión reutilizable del pool. Las políticas, columnas, índices, schemas y repositorios continúan bajo ownership del módulo; la capa común solo proporciona el mecanismo y su contrato probado.

Los contratos Protobuf generados se publican desde crates/versiones propiedad de sus bounded contexts. Se pueden compartir tipos puramente técnicos para health, metadata/correlación y errores de transporte cuando exista compatibilidad explícita; no se comparten entidades, agregados, DTOs internos ni errores de negocio entre módulos. Los interceptores comunes proporcionan autenticación criptográfica, TLS, límites y propagación, mientras que la autorización concreta y la validación del contexto/tenant permanecen en el servicio/módulo dueño.

La Fase 0 documenta y valida estáticamente el mapa de crates, ownership, dependencias permitidas, pools por proceso, contexto/RLS, contratos, security, Vault y mensajería; no crea crates ejecutables, configura servicios ni implementa runtime. La Fase 1 implementa el mínimo común reutilizable requerido por los primeros casos de uso CORE y los recorridos aplicables: PostgreSQL/SQLx, transacciones/contexto de tenant y RLS, gRPC/Protobuf y seguridad TLS, errores, observabilidad, Vault/secrets, Redis como cache, NATS Core, JetStream/Outbox. Cada capacidad se prueba con dependencias reales en infraestructura de desarrollo disponible; si Vault u otra dependencia real no está accesible, el alcance que depende de ella no se considera validado ni terminado.

### Regla de nombres

Los ejemplos abstractos de los principios siguientes usan nombres genéricos. La topología ENSAMBRIA definida en la sección 0 sí nombra sus bounded contexts para fijar el ownership y los contratos entre los cuatro backends.

Ejemplos:

```text
module-a
module-b
module-c
```

En el proyecto real:

```text
modules/<nombre-real-del-contexto>
```

El nombre real debe representar un **bounded context** del dominio, no una capa técnica.

Por ejemplo, no usar:

```text
modules/database
modules/services
modules/controllers
modules/utils
```

si esos nombres no representan bounded contexts reales.

Cuando un bounded context requiera un crate propio, el nombre del crate debe identificar ese contexto y su pertenencia a uno de los cuatro backends. La ubicación lógica `modules/<bounded-context>/` no obliga a crear un crate; los adapters de proveedores externos permanecen separados por integración y dominio propietario.

---

## 0.1 Diagrama canónico de la arquitectura

Este es el diagrama de referencia obligatorio. Cualquier código generado debe ser consistente con esta forma. Los cuatro backends comparten el workspace de build, pero tienen runtime y despliegues separados:

```text
Producto Beauty: tenant/business/mobile UI ─ REST/OpenAPI/WebSocket ─► BFF Beauty
                                                                          │ gRPC
                                  ┌───────────────────────────────────────┼───────────────────────┐
                                  ▼                                       ▼                       ▼
                              CORE (auth/tenant-product)             BUSINESS (verticales)    PUBLIC (solo si aplica)

Discovery UI ── REST/OpenAPI/WebSocket ─► BFF público independiente ──gRPC──► Backend PUBLIC
Admin UI ────── REST/OpenAPI/WebSocket ─► BFF PLATFORM integrado ───────────► Backend PLATFORM

PLATFORM ── contrato autenticado/versionado ─► BFFs/CORE: catálogo/configuración de producto
CORE ── tenant.created + product_id por Outbox/JetStream ─► PLATFORM Inbox

Backend CORE ◄──── gRPC síncrono si aplica ────► BUSINESS / PLATFORM / PUBLIC
  └────────── Outbox ── NATS JetStream ── Outbox ──────────┘
                      (hechos persistidos; solo cuando se requiere)

Cada backend usa integraciones externas necesarias para sus dominios,
con credenciales, callbacks y auditoría separados por propietario.
Una única base de datos PostgreSQL compartida; schemas y roles separados
separados y sin acceso cruzado. No es canal de comunicación.
```

CORE es el único propietario de pagos/ledger; PLATFORM mantiene políticas y obligaciones SaaS; BUSINESS mantiene pedidos y facturación de establecimientos; PUBLIC mantiene atribución del Marketplace. Los callbacks se enrutan al backend propietario del contexto que los valida, nunca se mezclan entre ENSAMBRIA y tenants.

---

## 0.2 Responsabilidad de cada componente

| Componente | Responsabilidad única | Tecnología Rust estándar |
|---|---|---|
| **Frontend** | UI/presentación. Sin lógica de negocio. | Framework cliente (Web/Mobile/Desktop) |
| **BFF (REST Gateway)** | Adaptación y composición de respuestas REST para el frontend. Sin lógica de negocio. | `axum` / `actix-web` + `utoipa` (OpenAPI) |
| **gRPC** | Contrato síncrono entre el BFF público independiente y PUBLIC, entre BFFs integrados y sus módulos, y entre backends cuando una consulta/comando necesita respuesta inmediata. | `tonic` + `prost` |
| **Crates técnicos compartidos** | Clientes, pools, transacciones, contexto/RLS, transporte, seguridad, mensajería, secretos y observabilidad reutilizables por los procesos y módulos que los necesiten. | Bibliotecas neutrales Cargo inyectadas por composition roots |
| **Módulo / bounded context** | Dueño exclusivo de su lógica de negocio: dominio, casos de uso y persistencia. Puede residir en un paquete de backend; un crate separado requiere necesidad verificable y código funcional. | Límite lógico de ownership; crate no obligatorio |
| **DB** | Fuente de verdad compartida físicamente; cada módulo es dueño exclusivo de su schema/tablas y migraciones. | `sqlx` (PostgreSQL) / Migraciones versionadas por módulo |
| **PostgreSQL/transactions** | Construcción del pool por proceso, lifecycle, timeouts, transacciones y establecimiento del contexto autenticado requerido por RLS. El pool/biblioteca común no amplía los privilegios de la identidad SQL. | `database` + `transactions` comunes; credenciales/grants de mínimo privilegio y ownership por dominio |
| **Redis** | Exclusivamente caché efímera con TTL; nunca fuente de verdad ni almacén durable de negocio. | `redis` + `deadpool-redis` / `bb8-redis` |
| **NATS Core** | Fan-out realtime efímero entre réplicas; no ofrece replay durable y las señales perdidas se reparan desde JetStream y el journal/cursor REST. | `async-nats` (Core NATS) |
| **NATS JetStream** | Eventos persistidos durables con Outbox, consumidores idempotentes, ACK y replay. | `async-nats` (JetStream context) |
| **Almacenamiento de objetos** | Puerto neutral del módulo propietario del ciclo de vida/autorización del archivo; adapter inicial para Cloudflare R2 usando su API compatible con S3. Los bytes se transfieren navegador↔almacenamiento mediante URLs temporales autorizadas. | API de dominio neutral + adapter S3 compatible |
| **`gateway_credentials` / adapters de integración** | Custodia tenant de credenciales de proveedores y entrega en memoria solo al adapter autorizado. El backend propietario define autorización y ciclo de vida; los secretos no se publican en eventos. | Módulo dueño + almacenamiento protegido/custodia tenant + secret manager cuando corresponda |
| **Pagos y ledger** | CORE ejecuta pagos/reembolsos, aplica comisiones aprobadas y mantiene ledger, liquidaciones y conciliación; los adapters de pago no definen política comercial. | Módulos/adapters CORE + cuentas/credenciales aisladas por ámbito + API oficial/sandbox |
| **Facturación SaaS** | PLATFORM mantiene suscripciones y obligaciones/documentos de ENSAMBRIA; CORE procesa pagos asociados. | Módulo/adapters PLATFORM para Factus; contratos de pago CORE |
| **Facturación comercial tenant** | BUSINESS mantiene pedidos y documentos comerciales de cada establecimiento; CORE procesa los pagos asociados. | Módulo/adapters BUSINESS para Factus; contratos de pago CORE |
| **BFF público** | Adaptación REST/OpenAPI/WebSocket de Discovery y páginas/proyecciones públicas; despliegue independiente, sin lógica de negocio ni acceso directo a stores; invoca PUBLIC por gRPC. | App HTTP independiente + clientes gRPC tipados |
| **BFF de producto** | Superficie REST/OpenAPI/WebSocket especializada por producto para las experiencias tenant/business/móvil que correspondan; adaptador sin reglas de dominio ni persistencia, que propaga producto/tenant autenticados y llama contratos autorizados de los cuatro backends. Unidad de aplicación/despliegue propia por producto cuando exista el producto. | App/BFF por producto dentro del workspace Cargo único si es Rust; artefacto, configuración, secretos y release separados; sin pool PostgreSQL |
| **BFF de administración PLATFORM** | Adaptación de la consola global dentro de PLATFORM, mediante contratos de aplicación; no administra identidades/tenants manualmente ni reemplaza el autoservicio. | Módulo HTTP integrado en PLATFORM |
| **Realtime** | Cada backend emite cambios de sus dominios y sirve solo a las audiencias autorizadas; el BFF correspondiente brinda WebSocket y recuperación REST/cursor. | Outbox/JetStream durable + NATS Core efímero |
| **OpenTelemetry** | Tracing y métricas distribuidas de extremo a extremo, en todas las capas. | `tracing` + `opentelemetry` + `opentelemetry-otlp` |
| **k6 & criterion** | Pruebas de rendimiento y carga sobre operaciones críticas y benchmarks internos. | `k6` (externo) + `criterion` (micro-benchmarks) |
| **Chaos Engineering** | Validación activa de resiliencia del sistema ante fallos reales. | Inyección controlada (`toxiproxy-rs`, Chaos scripts) |

Ninguna de estas responsabilidades se invierte ni se mezcla:

- el Frontend nunca decide reglas de negocio;
- el BFF nunca implementa reglas de negocio;
- la infraestructura compartida ejecuta mecanismos técnicos y nunca decide reglas de negocio;
- un módulo nunca delega su lógica de negocio a otra capa;
- una base de datos nunca se convierte en mecanismo de comunicación entre módulos o backends;
- Redis nunca sustituye la fuente de verdad;
- NATS JetStream/Outbox se reserva para hechos persistidos que requieren procesamiento durable/asíncrono y nunca sustituye una llamada síncrona;
- gRPC nunca debe convertirse en un bypass de las reglas de dominio.

### Ownership de capacidades multi-vertical

CORE posee identidad/contexto tenant, reservas, pagos y registros transaccionales. BUSINESS posee workforce, horarios/turnos, operación vertical, clientes/CRM, membresías, Commerce, marketing comercial, comisiones laborales y regalías. PLATFORM define políticas de monetización y obligaciones SaaS. PUBLIC administra directorio/Marketplace, proyecciones autorizadas y atribución. La tarifa comercial no se decide en CORE, pero CORE valida su versión/aplicabilidad y persiste el efecto financiero; la comisión de empleado no es comisión de pago y permanece propiedad de BUSINESS.

En BUSINESS, las capacidades reutilizables se mantienen neutrales a la industria en `modules/`; las reglas y datos especializados se agrupan bajo `verticals/<industria>/`. La carpeta de Beauty & Wellness ya contiene una carta documental, no runtime. Esta organización no añade backends ni crea obligatoriamente crates o bounded contexts por carpeta. El módulo dueño expone el contrato de la capacidad común (por ejemplo, inventario conserva autoridad sobre stock, lotes y movimientos); en el catálogo, `establishment-catalog-verticals` posee el registro común del servicio (identidad estable, establecimiento asociado, nombre, estado de ciclo de vida y versión), mientras que cada vertical posee sus definiciones, atributos, parámetros y reglas especializados por contrato. Un vertical mantiene sus reglas/atributos exclusivos y no escribe directamente tablas compartidas. Si una necesidad nueva es exclusiva de una industria, se implementa en su vertical; si se demuestra común, el owner amplía la capacidad general y verifica compatibilidad y regresiones. No se introducen condicionales de industria en módulos compartidos, campos dinámicos o esquemas universales por anticipación. Los contratos y casos ejecutables se especifican en la Fase 2 antes de implementar. Véase [ADR-0008](docs/adr/ADR-0008-modulos-compartidos-y-verticales-business.md) y el [modelo de dominios](docs/arquitectura-dominios.md).

Las capacidades de IA solo consultan información real mediante contratos y permisos tenant-scoped; no reciben acceso libre a PostgreSQL ni pueden ejecutar acciones financieras/operativas críticas sin autorización. Fotografías de resultados requieren consentimiento y controles específicos; los datos de salud o sensibles no se incorporan al CRM general ni se convierten en historia clínica por defecto. Los límites de retención y uso dependen de finalidad, consentimiento y jurisdicción aplicable.

### Capacidades transversales diseñadas desde la Fase 1

La Fase 1 establece el diseño, contratos, ownership y criterios de evolución de las capacidades siguientes. No adelanta reglas de negocio ni obliga a implementar todos sus productos en CORE: el runtime funcional se entrega en la fase del dominio propietario. La Fase 1 sí implementa y prueba con dependencias reales el aislamiento multi-tenant, autorización y contexto tenant/sucursal que forman parte de la fundación CORE.

| Capacidad | Ownership y límites arquitectónicos |
|---|---|
| Multi-tenant avanzado | CORE autentica y valida tenant/actor/permisos y contexto. BUSINESS posee configuración operativa del negocio y overrides por sucursal. RLS y autorización de dominio se aplican además de aislar índices, cachés, objetos, búsquedas, jobs y eventos. |
| Multi-sucursal | CORE posee identidades y relaciones estructurales organización/sucursal y propaga scopes verificados. BUSINESS posee recursos, horarios, personal e inventario operativos por sucursal; no se asume herencia/compartición sin regla explícita. |
| Multi-país y multi-moneda | PLATFORM configura países/monedas comercialmente disponibles; CORE mantiene contexto regional verificado y ejecuta pagos con monto entero y código de moneda. Se usan ISO 3166-1, ISO 4217 y zonas IANA; el dominio dueño conserva la moneda original. País, moneda, zona horaria y políticas regionales se versionan; no hay conversión implícita ni inferencia de obligaciones legales. |
| Multi-idioma | Locale normalizado como BCP 47; español, inglés y portugués son los idiomas iniciales. APIs/eventos transportan códigos y datos estables, no texto de UI localizado. Cada dueño localiza sus presentaciones; se documentan resolución de locale y fallback. |
| Motor de reglas | No existe un dueño central de reglas multi-dominio ni un lenguaje de código arbitrario. CORE posee el estado/transiciones de reservas y cancelaciones y aplica políticas autorizadas; BUSINESS configura condiciones operativas/comerciales de establecimientos y posee membresías/comisiones laborales; PLATFORM posee políticas de monetización ENSAMBRIA. Cada módulo conserva reglas tipadas, deterministas, versionadas, auditables y limitadas por tenant/sucursal. |
| Eventos | El módulo que persiste el hecho es dueño de su evento y Outbox: CORE reservas/pagos; BUSINESS membresías/renovaciones e inventario. Entrega durable por JetStream, contratos versionados, idempotencia y replay; las decisiones/envíos de notificación no transfieren ownership del hecho de origen. |
| Auditoría | Cada dominio conserva auditoría append-only de sus cambios y decisiones con actor autenticado, acción, instante, recurso/scope y origen técnico seguro. CORE posee auditoría de identidad/seguridad; PLATFORM la administración global. Auditoría de negocio no se sustituye por logs técnicos; retención, acceso e IP/PII se minimizan según política aprobada. |
| API pública | El módulo propietario mantiene contrato y reglas de sus operaciones. PLATFORM administra el registro/relación comercial de integradores y entitlements; CORE proporciona identidad/autorización de servicios y credenciales. La exposición HTTP se hace por una superficie gateway aprobada, con scopes, versionado, rate limits, idempotencia y sandbox; nunca se expone acceso a tablas o gRPC interno. |
| Webhooks | El dominio emisor conserva ownership de eventos, autorización de suscripciones y datos publicados. La entrega técnica durable puede reutilizar infraestructura compartida con firma/rotación, deduplicación, reintentos, redacción y protección SSRF/egress; webhooks entrantes siguen el contrato oficial del proveedor y su anti-replay. |
| Marketplace de extensiones | PLATFORM posee catálogo, revisión/publicación, compatibilidad, suscripción y entitlement. El dominio que expone una capacidad mantiene sus APIs, autorización y datos. Integraciones de terceros son externas y aisladas; no se ejecuta código arbitrario dentro del backend ni se concede acceso SQL directo. |

El mapa anterior es objetivo arquitectónico, no evidencia de implementación. Fase 1 documenta la matriz de integración, fases propietarias y pruebas de aceptación. Las fases posteriores implementan cada comportamiento junto con su recorrido completo y dependencias reales. En particular, la API/webhooks externos corresponden a Fase 7, internacionalización funcional a Fase 10 y operación del Marketplace de extensiones a Fase 11; estas dependencias no permiten posponer aislamiento, versionado, privacidad ni pruebas del comportamiento que sí se implemente en una fase anterior.

## 0.3 Integración obligatoria desde cada fase/subfase

Toda fase y subfase que implemente software debe diseñarse e implementarse de extremo a extremo desde su primer cambio. Antes de escribir código, su documentación debe registrar el recorrido externo (Frontend → HTTP/REST → BFF → backend/módulo dueño; gRPC desde el BFF público independiente y contratos internos desde los BFF integrados de BUSINESS/PLATFORM), la persistencia/migraciones del módulo, las bibliotecas de infraestructura compartida que consume cada proceso, el pool/transacción/contexto de tenant, el uso requerido de Redis y la publicación/consumo requerido de eventos NATS Core o JetStream/Outbox. La implementación y sus pruebas de aceptación pertenecen a esa misma fase/subfase; no se permite diferir estas fronteras a una refactorización futura. La Fase 0 de descubrimiento y arquitectura es documental: define ownership, composición y aceptación, sin crear runtime ni ejecutar pruebas funcionales. La Fase 1 construye la infraestructura común requerida junto con los recorridos completos de los backends y superficies aplicables desde el primer corte ejecutable.

En cada subfase se evalúan explícitamente Redis, NATS Core y NATS JetStream. Redis se usa exclusivamente para caché con TTL. NATS Core distribuye señales realtime efímeras entre réplicas, sin garantías de persistencia o replay; JetStream/Outbox transporta cambios persistidos que requieren entrega durable. Si una pieza no aplica a una operación, se registra una justificación técnica concreta y verificable en la documentación de esa fase. No se introducen usos artificiales: se prohíbe omitir su evaluación. Ningún bus sustituye gRPC para operaciones síncronas.

Cuando un dominio use Redis o NATS, la recuperación tras reinicios/interrupciones es parte obligatoria de la implementación: Redis reconecta con backoff para reanudar la caché; las suscripciones NATS Core se reestablecen y sus señales efímeras perdidas se reparan desde cursor/snapshot; los consumidores JetStream reconstruyen sus consumers durables y recuperan pendientes de forma idempotente. PostgreSQL/Outbox y JetStream conservan la recuperación durable. La fase añade pruebas con dependencias reales para reconexión, fan-out, replay/reentrega y continuidad después de reinicios aplicables. Las pruebas destructivas solo se ejecutan contra los servicios de desarrollo identificados explícitamente en `.env`; nunca crean contenedores ni reinician recursos de producción.

Las superficies tenant, business y móvil de cada producto consumen exclusivamente HTTP/REST/OpenAPI/WebSocket de su BFF de producto independiente; Discovery usa el BFF público y administración global usa el BFF integrado en PLATFORM. El BFF de producto enruta a los backends dueños mediante contratos autorizados; no contiene reglas de negocio. CORE no se expone directamente a frontends. Ningún frontend se conecta directamente a módulos, gRPC interno, base de datos, Redis, NATS ni proveedores. Las pruebas de cada fase verifican el recorrido real frontend→BFF→módulo dueño y, cuando aplique, contratos entre backends por gRPC o JetStream/Outbox; no sustituyen integración real con mocks.

## 0.4 Seguridad de servicio a servicio e integraciones

Cada llamada entre backends es una frontera de confianza explícita. La autenticación acredita qué servicio llama; la autorización determina qué método y operación puede ejecutar. Se usan certificados de cliente/servidor con mTLS y validación de identidad SAN/issuer conforme a la PKI configurada. La rotación de certificado/CA admite solapamiento, comprobación de expiración y revocación, y no depende de claves permanentes sin rotación. TLS protege confidencialidad e integridad en tránsito; los controles de aplicación protegen audiencia, scope y contexto delegado cuando corresponda.

Cada método gRPC tiene política de caller, scope, límites y timeout. Los mensajes validan campos requeridos, versión, enums conocidos, longitud, tamaño total y rangos antes de procesar. `correlation_id`, `trace_id`, deadline y contexto de tenant/actor autorizado se propagan de forma tipada; nunca se confía en claims aislados sin verificar autenticidad y correspondencia. Se limitan frecuencia y concurrencia por servicio/operación/tenant, se aplica backpressure y se protegen operaciones no idempotentes de replay. Fallos de certificado, identidad, autorización, integridad o contrato producen rechazo cerrado y evento de diagnóstico redactado.

Los comandos/consultas gRPC entre backends se usan solo cuando el solicitante necesita una respuesta inmediata. Pueden ejecutar una escritura durable en el backend dueño; elegir gRPC no implica saltarse persistencia ni la publicación de un evento que el dominio requiera. NATS JetStream/Outbox se usa para propagar hechos persistidos y trabajos que requieren entrega durable, replay o consumidores desacoplados; no se usa como RPC encubierto. Si la operación no requiere coordinación, no se invoca el otro backend ni se emite un evento. Los secretos nunca se publican en eventos. Los reintentos gRPC tienen límite, backoff+jitter, clasificación retryable y protección idempotente; ante timeout con resultado incierto se consulta/reconcilia antes de volver a ejecutar.

Cada módulo propietario conserva y autoriza las credenciales de su integración. PLATFORM administra las conexiones de servicios propios de ENSAMBRIA; BUSINESS administra integraciones configuradas por establecimientos; CORE solo recibe la referencia/contexto mínimo necesario para ejecutar pagos, sin reutilizar las credenciales de otro ámbito. Ningún backend comparte secretos ni los publica por gRPC/JetStream; las APIs nunca devuelven el secreto original.

Las pruebas de aceptación incluyen caller permitido/denegado, certificado inválido/expirado, rotación de certificados y claves de cifrado, límite de tasa, request fuera de tamaño/rango, timeout/desconexión, replay, idempotencia y redacción entre ambas fronteras. Las pruebas de JetStream verifican persistencia del Outbox, entrega/reentrega, ACK posterior al efecto persistido, deduplicación y recuperación del consumer durable. Logs correlacionados registran evento, servicio, método, operación, clasificación/código, dependencia, duración, resultado y contexto seguro; nunca token, claves, certificados privados, firma completa ni payload sensible.

### Gestión de secretos y certificados con HashiCorp Vault

Este es el diseño objetivo de ENSAMBRIA; **no afirma que Vault ya esté configurado**. ENSAMBRIA consumirá la VM de Vault existente como servicio privado compartido, pero su configuración será aditiva y aislada por proyecto/entorno. Esta decisión no autoriza cambiar rutas, motores, identidades, políticas, listeners, sellado, auditoría, backups ni secretos de otros proyectos. Antes de aprovisionar se inventarían versión y configuración existentes con el operador de Vault; se detiene si una ruta/mount ya está ocupado o si no se puede garantizar el aislamiento.

Vault se alcanza únicamente por la red privada Tailscale y con TLS validado por cadena y nombre. Tailscale/ACL limita la conectividad de red, pero no sustituye TLS ni las ACL de Vault. No se expone el API administrativo a Internet. Los mounts y nombres reservados a continuación son propuestas exclusivas de ENSAMBRIA; se crean solo después de comprobar colisiones y capacidad, sin alterar los recursos compartidos existentes.

| Propósito | Motor/ruta propuesta | Uso y límites |
|---|---|---|
| Secretos estáticos | KV v2 en `ENSAMBRIA-kv/`; ruta lógica `ENSAMBRIA/<entorno>/<componente>/<workload>/<propósito>` | Valores versionados de desarrollo, staging y producción, con ACL por ruta exacta. Ejemplos lógicos: `ENSAMBRIA/dev/backend-core/api/postgresql`, `ENSAMBRIA/production/backend-business/api/storage/r2`, `ENSAMBRIA/production/backend-platform/worker/integrations/factus`. En las políticas/API KV v2 agrega el prefijo del mount y `/data/`, por ejemplo `ENSAMBRIA-kv/data/ENSAMBRIA/dev/backend-core/api/postgresql`. No se incluyen valores reales en documentación ni repositorio. |
| PKI interno por entorno | Motores separados `ENSAMBRIA-dev-pki/`, `ENSAMBRIA-staging-pki/` y `ENSAMBRIA-production-pki/` | Intermedias distintas por entorno; roles de emisión acotados por servicio, SAN/nombres DNS privados y usos cliente/servidor necesarios. La raíz permanece fuera de línea y fuera de la VM de ejecución de Vault. Certificados cortos y renovables; el cliente confía solo en la cadena ENSAMBRIA correspondiente. |
| Autenticación de workloads | Método AppRole dedicado `ENSAMBRIA-approle/`; nombres de rol con formato `ENSAMBRIA-<entorno>-<componente>-<workload>` | Identidad independiente por proceso y entorno. Por ejemplo, `ENSAMBRIA-dev-backend-core-api` y `ENSAMBRIA-production-backend-platform-worker`. El BFF público y los procesos de los cuatro backends tienen identidades propias; los BFF integrados heredan solo la identidad del proceso dueño mientras compartan su lifecycle. |
| Credenciales dinámicas de base de datos | Engine PostgreSQL dedicado `ENSAMBRIA-database/`, solo si se aprueba e implementa la emisión dinámica de roles; rutas `creds/ENSAMBRIA-<entorno>-<componente>-<workload>` | Evaluar antes de producción. Hasta entonces, credenciales estáticas por workload en KV v2, rol SQL de mínimo privilegio y rotación coordinada; nunca una cuenta superuser compartida. |

Las rutas de KV mantienen **un prefijo por entorno** (`dev`, `staging`, `production`) y por workload. Las identidades de dev y staging no reciben ninguna capacidad sobre producción; ninguna identidad de un backend lee rutas del otro. El diseño no depende de Vault Enterprise namespaces. Se reservan mounts exclusivos con prefijo `ENSAMBRIA-` para evitar colisiones con motores de otros proyectos.

Las rutas de proveedor se separan por backend propietario y propósito: CORE usa rutas de pago acotadas al contexto/cuenta autorizados; PLATFORM mantiene conexiones para facturación SaaS; BUSINESS mantiene conexiones de facturación e integraciones de los establecimientos. Factus para SaaS y Factus tenant, así como credenciales Wompi por titular, usan identidades/rutas distintas. PUBLIC y el BFF público no reciben acceso a credenciales de proveedores, bases de datos u objetos; el BFF recibe solo el certificado/configuración técnica que requiere su conexión gRPC. Las credenciales de pago introducidas por tenants pertenecen al contexto de pagos de CORE; las demás integraciones del establecimiento pertenecen al módulo que las utiliza en BUSINESS. No se crean rutas KV por tenant ni se mezclan credenciales de ENSAMBRIA con credenciales tenant; el cifrado, rotación y entrega siguen el contrato del propietario.

Las políticas ACL son de mínimo privilegio: cada AppRole recibe una política exclusiva, capacidad `read` solo sobre sus rutas exactas de KV v2 y `update` solo sobre el endpoint de emisión PKI de su propio rol. La lectura de CA/CRL se limita a las rutas requeridas. Los procesos no obtienen `list` salvo necesidad demostrada, ni permisos de escritura/borrado en KV, administración de mounts, auth, políticas, auditoría, sellado o tokens ajenos. No se usan comodines amplios como `ENSAMBRIA/*` para identidades de aplicación. Ejemplo esquemático de política dev del API general:

```hcl
path "ENSAMBRIA-kv/data/ENSAMBRIA/dev/backend-core/api/postgresql" {
  capabilities = ["read"]
}

path "ENSAMBRIA-dev-pki/issue/backend-core-api" {
  capabilities = ["update"]
}
```

La política real añade únicamente otras rutas y lectura de CA/CRL que el workload requiera y se verifiquen. Nunca alcanza otros entornos ni workloads de los cuatro backends; no incluye valores secretos.

AppRole es solo para workloads, no para operadores humanos. `role_id` identifica el rol; `secret_id` se trata como secreto y se provisiona fuera del repositorio, mediante respuesta envuelta de corta duración y un canal de bootstrap protegido para Vault Agent. Su TTL/usos se ajustan para permitir recuperación y reautenticación segura; no se configura un secreto de un solo uso si no existe un proceso probado para reaprovisionarlo tras reinicios. Vault Agent Auto-Auth gestiona el token renovable por workload. Su API local queda enlazada a loopback, no a la interfaz Tailscale ni a una dirección pública. Los operadores usan una identidad administrativa separada con MFA y privilegio excepcional; el root token queda fuera de las aplicaciones y reservado a bootstrap/emergencias con custodia controlada.

Cada workload aislado usa su propia instancia/identidad de Vault Agent; no comparte un token de Agent entre backends, BFF o workers. El acceso al endpoint local se restringe además mediante aislamiento de procesos del host. Si varios workloads comparten VM y no se puede garantizar esa separación, no se comparte el proxy Agent: se utiliza un mecanismo de acceso individual autorizado y revisado antes de desplegar.

La integración Rust objetivo es una capa técnica compartida (`crates/secrets`) consumida desde el composition root, nunca desde reglas de dominio. Obtiene solo las rutas autorizadas durante el arranque y la renovación programada, conserva valores en memoria con redacción de `Debug`, renueva leases/certificados con margen, valida y carga reemplazos atómicamente y publica métricas de disponibilidad sin valores secretos. No consulta Vault en cada request ni persiste tokens o secretos en PostgreSQL, eventos, logs, trazas, variables de entorno durables, imágenes o archivos de configuración. Si se usa un archivo de intercambio con Vault Agent, reside en almacenamiento temporal restringido y se elimina/reemplaza de forma segura.

La rotación de secretos estáticos crea una nueva versión KV v2 y coordina el cambio con el proveedor y el workload; se revoca la versión anterior solo después de verificar la actualización, respetando cualquier ventana de solapamiento necesaria. Certificados se renuevan antes de expirar y cada proceso recarga la nueva cadena sin interrupción; las claves privadas nunca se comparten entre instancias. Los secretos dinámicos, si se incorporan, se renuevan o reemplazan antes de vencer y los pools de conexiones se actualizan de forma controlada. Las pruebas incluyen revocación, expiración, renovación fallida, reinicio y rotación sin revelar valores.

Vault fuera de servicio no provoca fallback a valores inseguros ni a secretos de otro entorno. Un proceso puede continuar con material ya cargado solo mientras siga válido; readiness informa degradación y las operaciones que necesiten una credencial expirada fallan cerradas, sin impedir innecesariamente operaciones independientes. Se aplican backoff acotado y renovación antes del vencimiento, sin tormentas de reinicios.

La auditoría de Vault debe estar habilitada y protegida conforme a la operación existente. Los audit devices son configuración del clúster compartido: antes de cualquier cambio se requiere autorización del operador y evaluación de impacto para todos los proyectos; esta especificación no habilita, deshabilita ni reconfigura audit devices. Los logs de auditoría tienen acceso y retención restringidos y no se copian a ENSAMBRIA como si fueran solo sus registros.

Los backups del almacenamiento de Vault pueden contener secretos de **todos** los proyectos. El operador define su método según el backend de almacenamiento realmente instalado, cifrado, retención y copia fuera de la VM; la restauración se ensaya en un Vault aislado y nunca sobrescribe el Vault compartido sin autorización explícita y un plan de recuperación. La VM dedicada a Vault es una dependencia central y un posible punto único de falla mientras no se configure y valide HA; su indisponibilidad, RPO/RTO y recuperación deben quedar documentados antes de production.

Antes de habilitar ENSAMBRIA se requiere una revisión con el propietario de Vault para confirmar: nombres/mounts libres, red privada y CA TLS, método seguro para bootstrap/rebootstrap de AppRole, roles de Vault/operador, auditoría ya operativa, capacidad de almacenamiento y procedimiento de backup/restore. Esta tarea solo documenta la arquitectura: no consulta, modifica ni reinicia la VM, no crea mounts/roles/policies y no accede a secretos existentes.

Fuentes oficiales para la implementación posterior: [AppRole](https://developer.hashicorp.com/vault/docs/auth/approle), [Vault Agent](https://developer.hashicorp.com/vault/docs/agent-and-proxy/agent), [KV v2](https://developer.hashicorp.com/vault/docs/secrets/kv/kv-v2), [PKI](https://developer.hashicorp.com/vault/docs/secrets/pki) y [audit devices](https://developer.hashicorp.com/vault/docs/audit).

## 0.5 Contratos de integraciones de pago

Las operaciones y eventos de pago usan contratos versionados y neutrales al proveedor. Cada adapter transforma solo los campos necesarios al modelo común, descarta campos no permitidos y nunca devuelve payload crudo del proveedor. Las capacidades, métodos, estados, monedas y esquemas específicos se documentan únicamente después de verificar la documentación oficial vigente y el entorno de prueba del proveedor que ENSAMBRIA decida integrar.

Los importes comerciales se representan con enteros en unidades menores de la moneda, nunca con punto flotante. Los datos de método de pago y recibo se limitan a lo requerido por el caso de uso y autorizado por el contrato. No se exponen PAN, CVV, secretos, identificadores personales innecesarios ni datos de liquidación que el proveedor no haya confirmado.

### Sincronización realtime obligatoria en todas las fases

Esta obligación aplica a **todas las fases y subfases de implementación**, no solo a notificaciones, colaboración, trabajos o a una pantalla concreta. Cada fase/subfase que cree o modifique datos persistidos visibles en una aplicación debe incluir en su alcance el evento Outbox del módulo dueño, su envelope/versionado y distribución durable JetStream; NATS Core distribuye señales realtime efímeras entre réplicas cuando corresponda. Se incluyen autorización de audiencia, señal WebSocket, invalidación/refetch de TanStack Query y mecanismo REST de recuperación. Si una operación no produce una transición observable, documenta la justificación en la matriz de la fase; no omitas la evaluación. La Fase 0 documental registra que no introduce mutaciones persistidas ni requiere runtime realtime.

Cada backend es dueño de su superficie realtime y de las audiencias correspondientes a sus dominios. No comparten sesiones, cookies, cachés, canales ni permisos. Cada BFF autentica y bridgea su WebSocket público con los contratos internos del backend autorizado; no conecta directamente a NATS, Redis o PostgreSQL. Cada gateway realtime valida identidad, sesión, audiencia y permisos al abrir la conexión y durante su vida útil. Los callbacks externos ingresan al backend propietario de la integración, nunca a través del frontend. El frontend no se conecta a infraestructura ni recibe tokens en URL.

Cada mutación persiste el cambio de su dominio y su Outbox en la misma transacción PostgreSQL local del módulo dueño. Un evento producido por un backend y su posterior efecto en el otro pertenecen a transacciones distintas, coordinadas mediante Outbox/JetStream y consumidores idempotentes; no se simula una transacción distribuida. JetStream lleva eventos de integración durables, versionados y al menos una vez a consumidores durables idempotentes; el ACK ocurre solo después de persistir el efecto/proyección. Se usan `event_id` único, `stream_cursor` reanudable, `aggregate_id`, `aggregate_version` monotónica, tipo/versión, audiencia/scope, tiempo y `correlation_id`. PII, secretos y contenido no requerido no se colocan en envelopes de fan-out. Los contratos conservan compatibilidad durante despliegues progresivos con versiones de servicio coexistentes.

Cada módulo realtime mantiene un journal/cursor de recuperación con permisos y retención definidos, y publica por gRPC a su BFF autorizado. NATS Core se usa para fan-out efímero entre réplicas WebSocket/gateway; no conserva señales durante desconexiones. Redis se usa exclusivamente para caché con TTL, nunca como canal de distribución o autoridad. Si una señal Core se pierde, JetStream/Outbox conserva el cambio y el cliente repara con REST desde el cursor/snapshot del journal. PostgreSQL primario es autoridad de escritura; réplicas sirven lecturas, pero cursor y read-your-writes respetan posición de réplica o consultan al primario tras una mutación. El failover no crea cursores duplicados ni oculta cambios confirmados.

Cada aplicación frontend mantiene coordinadores realtime TanStack Query centrales separados por app/sesión/audiencia; nunca se comparten cookies, caché, query keys, canales ni cursores entre superficies. Cada coordinador deduplica `event_id`, ignora aggregate versions viejas, detecta gaps/eventos desconocidos, invalida familias de query keys y obtiene el estado real por REST con permisos vigentes. Reconecta con backoff+jitter y cursor; WebSocket no transporta snapshots de autoridad ni reemplaza REST. Siempre se conserva la sincronización manual como mecanismo de recuperación/rollback de presentación. Las vistas no implementan sockets, polling ni mapeos de eventos particulares.

La topología de cuatro backends soporta desde el primer cambio varias réplicas de cada backend, BFF, workers, Redis, NATS Core, JetStream, PostgreSQL y frontend: no depende de memoria local, sticky sessions ni de entrega exactly-once. Se documentan límites de conexión, heartbeat, rate/size, backpressure, cierre/drain, retención/replay durable, DLQ, lag de réplica, rotación de credenciales y despliegue compatible. Cada fase debe verificar el flujo multiinstancia relevante o registrar la prueba de resiliencia pendiente; una prueba solo con una instancia no demuestra escalabilidad horizontal.

---

# 1. Principios SOLID — aplicación obligatoria en Rust

| Principio | Regla obligatoria en Rust |
|---|---|
| **S**RP | Una struct/enum/función = una razón para cambiar. Prohibido `utils.rs` o `helpers.rs` genéricos que mezclen responsabilidades. Casos de uso (`struct XxxUseCase`) hacen UNA sola acción de negocio. |
| **O**CP | Extender comportamiento implementando nuevos `impl Trait` sobre los puertos existentes, nunca modificando código con `match`/`if` de tipos concretos dentro de dominio/aplicación. Usar patrones Strategy/Polymorphism con traits. |
| **L**SP | Toda implementación de un `trait` de puerto debe ser sustituible sin romper el contrato, manteniendo las mismas precondiciones y postcondiciones. Prohibido hacer `panic!` o lanzar errores no tipados donde el trait define un `Result<T, E>`. |
| **I**SP | Traits pequeños y específicos por caso de uso (`ResourceReader`, `ResourceWriter`) en vez de un trait gordo (`ResourceRepository` con 20 métodos) cuando los consumidores no usan todo. Componer traits con `+` (`TraitA + TraitB`). |
| **D**IP | Las capas de dominio y aplicación jamás dependen de crates de infraestructura concreta (`sqlx`, `redis`, `async-nats`, `tonic`). Dependen de `trait`s definidos en dominio/aplicación. La infraestructura implementa esos traits e inyecta `Arc<dyn Trait + Send + Sync>` desde el composition root (`module_config.rs`). |

---

# 1.bis Límite de tamaño de archivo y responsabilidad por archivo

Esta regla es obligatoria y se verifica en cada archivo Rust:

- **Máximo absoluto: 400 líneas físicas por archivo `.rs`**, contando imports, comentarios y líneas en blanco.
- **Objetivo preferido: alrededor de 200 líneas físicas** cuando la separación preserve cohesión y legibilidad.
- Un archivo puede acercarse a 400 líneas cuando contiene una función, o un conjunto pequeño de funciones estrechamente relacionadas, que forman una unidad coherente. No se fragmenta código artificialmente solo para alcanzar 200 líneas.
- Si supera 400 líneas, debe dividirse por responsabilidades en módulos cohesivos; nunca se justifica exceder el máximo por conveniencia.
- El tamaño es una señal para revisar SRP, no un sustituto del criterio de diseño: se priorizan cohesión, límites claros y ausencia de archivos "bolsa".

### Un archivo = una responsabilidad pública principal

Un archivo debe contener:

- una struct pública con su `impl`;
- un enum público;
- un trait público;
- un caso de uso;
- un handler REST/gRPC;

según corresponda.

No se permiten archivos "bolsa" con múltiples structs/funciones no relacionadas.

Ejemplo prohibido:

```text
utils.rs
 ├── calculate_a()
 ├── calculate_b()
 ├── parse_c()
 ├── validate_d()
 └── transform_e()
```

### Excepciones válidas

Las únicas excepciones permitidas son:

- tipos/enums de error estrechamente acoplados al tipo principal (ej. `enum ExecuteActionError` derivando `thiserror::Error` en el mismo archivo del caso de uso);
- DTOs de `Input`/`Output` exclusivos de ese caso de uso;
- constantes privadas usadas solamente por esa struct/función;
- funciones privadas auxiliares (`fn` sin `pub`) que existen únicamente para descomponer la función principal y no son utilizadas en otro lugar.

Si una función auxiliar podría reutilizarse en más de un archivo:

**se extrae a su propio archivo/módulo con su propio nombre.**

No se deja "flotando" dentro de otro archivo ni se duplica.

### Nombre de archivo

**Nombre de archivo = nombre de lo que exporta (en `snake_case`).**

Ejemplo:

```text
execute_action_use_case.rs
```

exporta:

```rust
pub struct ExecuteActionUseCase { ... }
```

Otro ejemplo:

```text
email.rs
```

exporta:

```rust
pub struct Email(String);
```

Evitar `mod.rs` (o el archivo con el nombre del módulo) como contenedor de lógica.

`mod.rs` o `lib.rs` solamente puede utilizarse como barrel de `pub mod` y `pub use`, sin lógica propia.

### Cuando un archivo se acerca o supera 400 líneas

Las estrategias de división correctas, en orden de preferencia:

1. Extraer funciones privadas reutilizables o con responsabilidad propia a submódulos cohesivos en la misma carpeta.
2. Extraer subtipos/DTOs a su propio archivo dentro de `dto/` o `value_objects/` según corresponda.
3. Si una función o servicio mezcla responsabilidades, dividirlo en casos de uso, servicios o handlers distintos.
4. Si una función larga representa una única operación coherente, puede permanecer en un archivo de hasta 400 líneas; no se divide únicamente para cumplir una preferencia estética de 200 líneas.

El criterio principal es SRP y cohesión. Ningún archivo puede superar 400 líneas físicas.

Esta regla aplica a TODAS las capas:

- dominio;
- aplicación;
- infraestructura;
- handlers/controladores;
- mappers;
- tests.

---

# 2. Regla de dependencias — Clean Architecture / Hexagonal

```text
        ┌─────────────────────────────────────────┐
        │       INFRASTRUCTURE (adapters)          │
        │       sqlx, redis, async-nats, axum,    │
        │       tonic, CLI                         │
        │  ┌─────────────────────────────────────┐ │
        │  │       APPLICATION (use cases)       │ │
        │  │       structs, traits inbound/out   │ │
        │  │  ┌───────────────────────────────┐  │ │
        │  │  │        DOMAIN (core)          │  │ │
        │  │  │ structs, enums, VOs,          │  │ │
        │  │  │ aggregates, pure business rules│ │
        │  │  └───────────────────────────────┘  │ │
        │  └─────────────────────────────────────┘ │
        └─────────────────────────────────────────┘
```

### Regla de oro

Las flechas de dependencia (`use` y dependencias en `Cargo.toml`) **SIEMPRE apuntan hacia adentro**.

El dominio:

- no depende de aplicación;
- no depende de infraestructura;
- no depende de frameworks;
- no depende de crates de base de datos (`sqlx`, `diesel`, `sea-orm`);
- no depende de Redis (`redis`, `deadpool-redis`);
- no depende de NATS (`async-nats`);
- no depende de clientes HTTP ni servidores (`reqwest`, `axum`, `actix-web`);
- no depende de detalles de persistencia ni wire-formats (no atributos `serde` de transporte ni `sqlx::FromRow` en dominio).

La aplicación:

- no depende de infraestructura concreta;
- no depende de implementaciones concretas de DB/Redis/NATS/gRPC;
- depende únicamente del dominio y de las abstracciones (`traits`) de puertos de entrada y salida.

La infraestructura:

- depende de dominio y aplicación;
- implementa los `traits` definidos como puertos.

### Prohibiciones en Rust

Prohibido:

```rust
use sqlx::FromRow;
#[derive(FromRow)]
```

o:

```rust
use serde::{Serialize, Deserialize};
```

con anotaciones de wire-format o persistencia dentro de:

```text
domain/
```

El dominio debe estar compuesto por structs y enums puros de Rust (a lo sumo con derives estándar como `#[derive(Debug, Clone, PartialEq, Eq)]`), sin acoplamiento a librerías de infraestructura.

Prohibido poner lógica de negocio dentro de:

- handlers REST/gRPC;
- suscriptores/listeners de eventos NATS;
- repositorios de persistencia.

Esas capas solamente:

- traducen;
- adaptan;
- orquestan.

Prohibido el uso de `unwrap()`, `expect()` o `panic!` en rutas de ejecución que procesan datos de entrada o llamadas externas. Usar siempre `Result<T, DomainError>` con `thiserror`.

### Mapeo

El mapeo entre:

```text
modelo de persistencia (FromRow struct)
        ↓
entidad de dominio
```

se hace SIEMPRE en infraestructura (`infrastructure/persistence/sqlx/mappers`).

Nunca al revés.

---

# 3. Estructura de carpetas obligatoria (workspace Cargo raíz)

La estructura interna se aplica módulo por módulo. El único manifiesto raíz de workspace es `Cargo.toml`; los paquetes conservan manifiestos propios donde Cargo los requiere, pero no crean workspaces anidados. Los entrypoints, identidades y artefactos de despliegue siguen separados:

```text
.
├── apps/bffs/products/beauty-wellness/src/
│   ├── api/ clients/ composition/ dto/ middleware/ observability/ routes/
├── backend/
│   ├── core/
│   │   ├── apps/{api,worker}/ deployment/ tests/{acceptance,contract,integration}/
│   │   └── modules/
│   │       ├── entitlement-evaluation/
│   │       ├── financial-operations-payments-ledger/
│   │       ├── identity-authentication/
│   │       ├── integration-credentials-authentication/
│   │       ├── reservations-availability/
│   │       ├── settlements-reconciliation/
│   │       └── tenancy-membership-organization-branch-product/
│   ├── business/
│   │   ├── apps/{api,worker}/ deployment/ tests/{acceptance,contract,integration}/
│   │   ├── modules/                 # 12 módulos de dominio BUSINESS
│   │       ├── business-ai-assistance/ commerce-orders-sales/ commercial-marketing/
│   │       ├── crm-customer-relationships/ employee-commissions/ establishment-billing/
│   │       ├── establishment-catalog-verticals/ franchises-chains-royalties/
│   │       ├── inventory-purchasing-costs/ memberships-loyalty-benefits/
│   │       └── operational-analytics-bi/ workforce-operations/
│   │   └── verticals/
│   │       └── beauty-wellness/README.md  # carta arquitectónica; sin runtime
│   ├── platform/
│   │   ├── apps/{api,worker}/ deployment/ tests/{acceptance,contract,integration}/
│   │   └── modules/
│   │       ├── commercial-account-onboarding/ commercial-policies-monetization/
│   │       ├── electronic-invoicing/ global-administration-support/
│   │       ├── integrations-api-extensions/ product-catalog-configuration/
│   │       └── saas-billing/ saas-invoices/ saas-subscriptions/
│   └── public/
│       ├── apps/{api,worker}/ deployment/ tests/{acceptance,contract,integration}/
│       └── modules/
│           ├── directory-discovery/ marketplace-channel-attribution/
│           └── publication-projections/ reviews/
├── bff-public/{deployment,src,tests}/
├── .github/{agents,skills,workflows}/
├── config/{environments,observability,products}/
├── contracts/{openapi,protobuf,schemas}/
├── crates/{infrastructure,observability,shared}/
├── docs/{adr,compliance,rules}/
├── flutter/{ensambria-app,ensambria-business}/
├── frontends/{corporate,marketplace,platform-admin,tenant-portal}/
├── infrastructure/shared/
├── packages/frontend/
├── scripts/
├── tests/{acceptance,architecture,contracts,e2e,integration,isolation,security}/
└── tools/
```

Las cuatro aplicaciones web bajo `frontends/` contienen `public/`, `src/` y `tests/`; las dos aplicaciones Flutter bajo `flutter/` contienen `assets/`, `lib/`, `test/` e `integration_test/`. `bff-public/` contiene `deployment/`, `src/` y `tests/`. El BFF de producto Beauty & Wellness se encuentra bajo `apps/bffs/products/beauty-wellness/src/`.

Cada uno de los 32 módulos ubicados en `backend/<owner>/modules/` mantiene este árbol de carpetas:

```text
<module>/
├── domain/{aggregates,entities,errors,events,services,value_objects}/
├── application/{dto,ports/{inbound,outbound},use_cases}/
├── adapters/{inbound,outbound}/
└── infrastructure/{composition,messaging,persistence}/
```

El `Cargo.toml` raíz es el único manifiesto `[workspace]` y registra paquetes de los cuatro backends, bibliotecas técnicas neutrales, BFF público y BFFs de producto implementados. Cada backend conserva su composición de paquetes/binarios y configuración de despliegue, pero no declara un workspace anidado; Cargo comparte resolución y `Cargo.lock` en la raíz. Los cuatro backends producen despliegues independientes; cada BFF de producto funcional y el BFF público tienen artefacto/proceso y despliegue propios. El BFF de administración PLATFORM permanece integrado inicialmente en su backend. La estructura de `crates/` es ilustrativa, no exige crear un crate por responsabilidad desde el inicio: agrupa responsabilidades técnicas cohesivas y las separa al existir una razón verificable, sin crear un crate de utilidades omnisciente. Como parte de la Fase 0 se audita y documenta esta composición, sus contratos y dependencias permitidas, sin crear runtime/crates funcionales; la infraestructura común ejecutable se construye y acepta durante la Fase 1.

El BFF público consume contratos gRPC de PUBLIC. Business y Tenant Portal consumen el mismo BFF/API de producto y los mismos contratos empresariales; no duplican reglas de negocio, permisos ni APIs por tener tecnologías distintas. ENSAMBRIA App consume la superficie de producto para operaciones de cliente y puede usar el BFF público para Discovery. Marketplace consume el BFF público; Platform Admin usa el BFF integrado en PLATFORM. Corporate publica contenido institucional estático o mediante un contrato público autorizado, sin acceso directo a servicios de dominio. Cada backend contiene solo módulos, workers, callbacks, proyecciones y realtime de los dominios que posee; no duplica ni comparte sesión, credenciales o autoridad de datos. Cada BFF integrado mantiene límites de módulo y contratos que evitan acceso directo a stores.

`modules/<bounded-context>/` identifica ownership y ubicación lógica, no exige un paquete Cargo por contexto. La composición puede alojar varios contextos en un paquete de backend cuando conserve sus límites, y solo crear un crate separado por necesidad demostrable de compilación, pruebas, despliegue o aislamiento. Cada crate adicional debe tener código funcional, owner, dependencias explícitas y tests reales; no se crean paquetes, directorios ejecutables ni manifests placeholder en Fase 0.

El módulo dueño del ciclo de vida y autorización de cada archivo depende de un puerto neutral de almacenamiento de objetos; el adapter inicial usa la API compatible con S3 de Cloudflare R2. Las credenciales y configuración del proveedor quedan fuera del dominio. Los contratos no exponen SDKs, tipos ni capacidades exclusivas de R2; el navegador transfiere bytes directamente al almacenamiento mediante autorizaciones temporales emitidas tras validar permisos.

El inventario físico vigente del repositorio y la estructura compartida de los 32 módulos están en los árboles al inicio de esta sección. Se omite aquí el esquema duplicado de rutas para evitar que una segunda representación contradiga esas ubicaciones. Las reglas de composición, visibilidad y comunicación entre módulos que siguen a continuación se mantienen vigentes.

### Reglas de visibilidad y comunicación entre módulos

Todo módulo expone su comunicación entrante ÚNICAMENTE vía:

```text
module_public_api.rs
```

técnicamente materializado como servicio:

```text
infrastructure/grpc
```

Ningún otro crate de módulo ni el BFF puede agregar dependencias internas en su `Cargo.toml` apuntando a:

```text
modules/otro-modulo/src/domain/...
modules/otro-modulo/src/application/...
modules/otro-modulo/src/infrastructure/...
```

Reforzar mediante reglas de visibilidad estricta de Rust:

- Solo `module_public_api.rs` o los contratos gRPC/DTOs de exportación son `pub`.
- El resto del árbol interno del crate debe ser `pub(crate)` o privado.
- Usar herramientas de análisis estático como `cargo deny`, `cargo-modules` o `cargo-machete`.

### Comunicación módulo → módulo

Si módulo A necesita datos de módulo B de forma síncrona:

```text
module-a
   │
   │ gRPC (tonic client)
   ▼
module-b (tonic server)
```

Debe invocar el mismo servicio gRPC de B que utiliza el BFF.

Debe existir un único contrato público por módulo.

Si la necesidad es asíncrona:

```text
module-b
   │
   │ event (async-nats)
   ▼
NATS JetStream
   │
   ▼
module-a
```

Nunca:

```text
module-a → DB de module-b
module-a → Redis de module-b
```

---

# 4. Reglas de DDD

## 4.1 Bounded Context primero

Antes de escribir código:

1. define el lenguaje ubicuo;
2. define los límites del contexto;
3. define qué conceptos pertenecen al contexto;
4. define qué conceptos NO pertenecen al contexto.

Un mismo término puede significar cosas distintas en diferentes módulos.

Esto está permitido y es deseable.

No unificar artificialmente conceptos que realmente pertenecen a diferentes contextos.

---

## 4.2 Entidades

Las entidades tienen:

- identidad estable (`id: ResourceId`);
- comportamiento;
- invariantes protegidas.

Ejemplo conceptual en Rust:

```rust
pub struct Resource {
    id: ResourceId,
    state: ResourceState,
}

impl Resource {
    pub fn deactivate(&mut self) -> Result<(), DomainError> {
        if self.state == ResourceState::Inactive {
            return Err(DomainError::AlreadyInactive);
        }
        self.state = ResourceState::Inactive;
        Ok(())
    }
}
```

Preferir:

```rust
resource.deactivate()?;
```

en lugar de:

```rust
resource.set_active(false); // Prohibido setter genérico
```

Prohibidos los setters genéricos públicos que permitan modificar arbitrariamente el estado de una entidad sin validar invariantes.

---

## 4.3 Value Objects

Los Value Objects deben ser:

- inmutables;
- usando el patrón **Newtype** (`struct Email(String)`);
- comparables por valor (`#[derive(PartialEq, Eq, Clone, Debug)]`);
- auto-validados en constructores (`Email::try_from(raw)` o `Email::new(raw) -> Result<Email, DomainError>`);
- independientes de infraestructura.

Ejemplos genéricos:

```text
Email
Money
DocumentId
Address
Percentage
DateRange
Quantity
Currency
```

No representar Value Objects como `String`, `i64` o `f64` sueltos dentro de las firmas del dominio cuando el concepto tenga reglas propias.

---

## 4.4 Agregados

Una raíz de agregado (`AggregateRoot`) es el único punto de entrada público para modificar su árbol de objetos internos.

### Regla por defecto

Un caso de uso debe modificar **un solo agregado por transacción**.

Esto ayuda a:

- mantener límites de consistencia fuertes;
- reducir acoplamiento;
- evitar transacciones `sqlx::Transaction` excesivamente grandes;
- evitar dependencias ocultas entre agregados.

Cuando el negocio lo permita, si es necesario coordinar múltiples agregados, priorizar:

```text
Domain Event
      ↓
Consistencia eventual
```

en lugar de:

```text
Transacción gigante
```

### Excepción

Existen casos legítimos donde una misma transacción debe involucrar más de un agregado dentro del mismo límite transaccional.

Está permitido solamente cuando:

1. ambos agregados pertenecen al mismo módulo/crate;
2. nunca cruzan un límite de módulo;
3. la decisión está documentada explícitamente en código o ADR;
4. se explica por qué la consistencia eventual no es aceptable;
5. primero se evalúa si el diseño del agregado está mal delimitado.

Los agregados deben proteger sus invariantes.

Debe ser imposible dejar el agregado en un estado inválido desde fuera.

---

## 4.5 Domain Events

Toda mutación de negocio relevante emite un Domain Event.

Ejemplos genéricos en Rust:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum DomainEvent {
    ResourceCreated { id: ResourceId, occurred_at: DateTime<Utc> },
    ResourceUpdated { id: ResourceId, occurred_at: DateTime<Utc> },
    ResourceDeleted { id: ResourceId, occurred_at: DateTime<Utc> },
    ProcessStarted { id: ProcessId, occurred_at: DateTime<Utc> },
    ProcessCompleted { id: ProcessId, occurred_at: DateTime<Utc> },
    ProcessFailed { id: ProcessId, reason: String, occurred_at: DateTime<Utc> },
}
```

Los eventos son recolectados por el agregado y despachados por la aplicación después de confirmar la persistencia.

Esto se coordina con el Outbox Pattern de la sección 6.

### Importante

Los:

```text
Domain Events internos (enum de dominio)
```

NO son exactamente lo mismo que:

```text
Integration Events (structs con serde Serialize/Deserialize para NATS)
```

Los eventos de integración publicados a NATS deben ser un mapeo:

```text
Domain Event
      ↓
Integration Event DTO
      ↓
Versioned Contract (serde JSON/Protobuf)
      ↓
NATS JetStream
```

Nunca serializar directamente el enum interno del dominio como contrato externo.

---

## 4.6 Repositorios

La interfaz vive en:

```text
domain/repositories/
```

como un `trait` (utilizando `#[async_trait]` para compatibilidad con trait objects `dyn Trait`).

Los métodos deben estar orientados a agregados completos.

Ejemplos:

```rust
#[async_trait]
pub trait ResourceRepository: Send + Sync {
    async fn save(&self, aggregate: &Resource) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &ResourceId) -> Result<Option<Resource>, RepositoryError>;
    async fn next_identity(&self) -> ResourceId;
}
```

Prohibido exponer desde el trait de dominio métodos de tipo:

```text
find_where
query_raw
query_builder
execute_sql
```

Esos mecanismos pertenecen a adaptadores de infraestructura o adaptadores específicos de lectura.

---

## 4.7 Domain Services

Usar Domain Services solamente cuando una regla de negocio:

- involucra más de una entidad/agregado;
- no pertenece naturalmente a ninguna entidad.

Si la regla puede vivir dentro de una entidad:

**debe vivir dentro de la entidad.**

No crear:

```text
GenericBusinessService
BusinessManager
DomainHelper
```

sin una responsabilidad clara y acotada.

---

## 4.8 Anti-Corruption Layer (ACL)

Cuando un módulo consume:

- otro módulo;
- otro servicio;
- un sistema externo;
- eventos externos;

debe traducir el modelo externo al lenguaje ubicuo propio mediante un mapper dedicado en infraestructura.

Nunca permitir que el modelo de otro contexto se filtre dentro del dominio propio.

Ejemplo:

```text
External Prost/JSON Model
            ↓
       ACL Mapper
            ↓
    Own Application DTO
            ↓
        Own Domain
```

---

# 5. Reglas de Casos de Uso — Application Layer

### Regla principal

```text
1 struct con su impl
=
1 caso de uso
=
1 archivo
```

Ejemplo:

```text
execute_action_use_case.rs
revert_action_use_case.rs
create_resource_use_case.rs
delete_resource_use_case.rs
```

### Firma estándar

Un caso de uso:

- recibe un `Input DTO` por valor o referencia;
- devuelve `Result<OutputDTO, UseCaseError>`.

Nunca expone directamente:

- entidades de dominio;
- modelos `FromRow` de base de datos;
- modelos de persistencia.

### Responsabilidad

El caso de uso orquesta:

```text
validar input DTO
       ↓
recuperar agregado vía puerto (Arc<dyn Repository>)
       ↓
invocar métodos de negocio del dominio
       ↓
persistir vía puerto
       ↓
registrar/despachar eventos
```

El caso de uso **NO contiene reglas de negocio propias**.

Su responsabilidad es orquestar.

### Errores

Utilizar enums de error tipados derivados con `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}
```

Prohibido el uso de `panic!`, `unwrap()` o strings de error genéricos para representar errores de negocio.

### Inyección de Dependencias

Rust favorece la composición explícita sobre magia de frameworks:

```rust
pub struct ExecuteActionUseCase {
    repo: Arc<dyn ResourceRepository>,
    publisher: Arc<dyn EventPublisher>,
}

impl ExecuteActionUseCase {
    pub fn new(repo: Arc<dyn ResourceRepository>, publisher: Arc<dyn EventPublisher>) -> Self {
        Self { repo, publisher }
    }
}
```

El cableado se realiza en `module_config.rs`.

### Idempotencia

Todo caso de uso que reacciona a un evento externo debe ser idempotente.

Usar:

```text
event_id
idempotency_key
```

o mecanismo equivalente para deduplicar.

---

# 6. Base de datos

## 6.1 Propiedad de los datos

**Todos los módulos se conectan a una única base de datos PostgreSQL. Cada módulo es dueño exclusivo de su schema lógico, sus tablas y sus migraciones.** La base física es compartida; la propiedad lógica de los datos no.

Usa un namespace/schema de PostgreSQL por módulo. No crees una base de datos independiente por módulo.

Los cuatro backends no requieren cuatro instancias ni cuatro bases físicas: la topología definida comparte una única base física PostgreSQL. Esto no implica acceso irrestricto: cada módulo tiene rol dueño de migraciones separado de las credenciales de runtime. Los roles de runtime y sus grants se limitan a schemas/operaciones requeridos por el backend/dominio, aplicando mínimo privilegio; no se concede acceso global por compartir servidor, base, pool o biblioteca de conexiones. Las pruebas de aceptación verifican que roles de cada backend no puedan leer ni modificar schemas ajenos y que módulos consuman otros dominios por contratos, no mediante SQL directo. Las migraciones coordinadas se aplican con credenciales propietarias y no se reutilizan como credenciales de runtime.

Prohibido hacer JOINs SQL entre tablas pertenecientes a módulos diferentes.

Si necesitas datos de otro módulo:

```text
module_public_api / gRPC (tonic)
```

o:

```text
evento NATS → proyección local de lectura
```

---

## 6.2 Migraciones

Las migraciones deben estar versionadas por módulo.

Ejemplo:

```text
infrastructure/
  persistence/
    sqlx/
      migrations/
        20260906000000_create_resources.sql
```

Cada módulo mantiene su propia evolución de esquema gestionada con herramientas como `sqlx migrate` o `refinery`.
Las migraciones se almacenan y ejecutan por módulo contra la misma base PostgreSQL, con identificadores/versionado y ownership inequívocos. El arranque o despliegue coordina su aplicación; ningún módulo modifica el schema o las tablas de otro.

---

## 6.3 Transacciones

La transacción tiene como límite por defecto el agregado.

Nunca abrir una transacción `sqlx::Transaction` que cruce dos módulos.

Cruzar dos agregados del mismo módulo solamente se permite bajo las excepciones documentadas en la sección de agregados.

---

## 6.4 Outbox Pattern obligatorio

Para publicar eventos consistentemente con la escritura de DB:

### Paso 1

Dentro de la MISMA transacción `sqlx::Transaction` que persiste el cambio de estado:

```text
UPDATE / INSERT / DELETE en tablas de dominio
        +
INSERT en outbox_events
```

### Paso 2

Un proceso / worker separado (tarea `tokio::spawn` dedicada o servicio relay):

```text
outbox_events
      ↓
Outbox Relay
      ↓
NATS JetStream (async-nats)
```

marca los eventos como enviados después de publicarlos correctamente.

### Paso 3

Esto evita:

```text
DB guardada + evento perdido
```

o:

```text
evento publicado + DB no guardada
```

---

## 6.5 Repositorios

Los repositorios de infraestructura devuelven SIEMPRE:

```text
Domain Entities
```

reconstituidas mediante mappers.

Nunca devolver:

```text
struct FromRow / modelos SQLX crudos
```

directamente a aplicación o dominio.

Prohibido usar `sqlx::query!` o query builders directamente dentro de:

```text
domain/
application/
use_cases/
```

Solo dentro de implementaciones concretas de repositorio en:

```text
infrastructure/persistence/sqlx/repositories/
```

---

# 7. Redis — cache y lecturas

Redis se accede SIEMPRE mediante un trait de puerto de salida definido en:

```text
application/ports/outbound/cache_port.rs
```

Ejemplo:

```rust
#[async_trait]
pub trait CachePort: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError>;
    async fn set(&self, key: &str, value: &str, ttl_secs: u64) -> Result<(), CacheError>;
    async fn delete(&self, key: &str) -> Result<(), CacheError>;
}
```

Implementado en:

```text
infrastructure/cache/redis/
```

utilizando `redis` + `deadpool-redis` o `bb8-redis`.

Nunca importar el cliente Redis directamente en:

```text
domain/
application/
```

## Usos permitidos

### Cache de lectura

Usar cache-aside:

```text
PostgreSQL
    ↓
  Redis
```

Siempre con:

- TTL explícito;
- estrategia de invalidación documentada.

Preferentemente la invalidación se relaciona con eventos de dominio cuando corresponda.

No depender únicamente de:

```text
TTL y ya
```

---

## Modelos de lectura desnormalizados

Cuando una consulta necesita combinar información de varios agregados:

No convertir el dominio en un sistema de JOINs.

Se puede utilizar:

```text
Redis
```

o:

```text
tabla de lectura SQL
```

actualizada mediante eventos de integración.

Esto corresponde a un enfoque de CQRS ligero.

---

## Locks distribuidos

Permitidos para:

- idempotencia;
- evitar dobles ejecuciones;
- coordinación distribuida.

Utilizar librerías probadas en Rust, por ejemplo crates basados en Redlock (`rslock` o `redlock`).

No implementar un mecanismo casero basado únicamente en `SETNX` sin expiración y sin estrategia de recuperación.

---

## Rate limiting

Redis puede utilizarse para:

- rate limiting;
- sesiones;
- colas de trabajo ligeras;
- estado técnico.

---

## Prohibición

Redis nunca debe ser la única fuente de verdad de datos críticos de negocio.

Todo dato crítico debe poder reconstruirse desde:

```text
DB (PostgreSQL)
```

y/o:

```text
eventos NATS
```

según el diseño.

---

## Namespacing

Las keys deben estar namespaced por módulo.

Formato:

```text
<module>:<entity>:<id>:<subresource>
```

Ejemplo genérico:

```text
module-a:resource:123:metadata
```

---

# 8. Eventos con NATS JetStream

## 8.1 Contrato versionado

Cada evento de integración debe tener un schema explícito con `#[derive(Serialize, Deserialize)]`.

Ejemplo:

```text
infrastructure/messaging/nats/schemas/resource_updated_v1.rs
```

Debe contener como mínimo:

```text
event_id
event_type
event_version
occurred_at
producer_module
payload
trace_id
```

Los cambios incompatibles requieren una nueva versión:

```text
resource_updated_v2.rs
```

Nunca modificar silenciosamente el contrato existente.

---

## 8.2 Naming

Subjects:

```text
<context>.<aggregate>.<event>
```

en minúsculas y utilizando eventos en pasado.

Ejemplos genéricos:

```text
module-a.resource.updated
module-b.resource.reserved
module-c.process.completed
```

Los streams de JetStream deben estar organizados por bounded context cuando corresponda.

Ejemplo:

```text
STREAM: MODULE_A_EVENTS
```

---

## 8.3 Publisher

El publisher se define como trait de puerto en:

```text
application/ports/outbound/event_publisher.rs
```

y se implementa en:

```text
infrastructure/messaging/nats/publishers/
```

utilizando el crate `async-nats`.

Los casos de uso NUNCA importan directamente el cliente de NATS (`async_nats::Client`).

---

## 8.4 Subscriber

Cada handler entrante vive en:

```text
infrastructure/messaging/nats/subscribers/
```

y se ejecuta típicamente en una tarea asíncrona dedicada (`tokio::spawn`).

El subscriber:

1. recibe el mensaje;
2. valida y deserializa el payload con `serde_json`;
3. utiliza la capa ACL para mapear a DTO propio;
4. llama a un caso de uso;
5. hace `msg.ack().await` únicamente si el caso de uso tuvo éxito.

Nunca ejecuta lógica de negocio directamente.

---

## Idempotencia

Todo consumidor debe ser idempotente.

Deduplicar mediante:

```text
event_id
```

o mecanismo equivalente en DB (ej. tabla `processed_events`) antes de ejecutar acciones de negocio.

---

## Ack manual

No utilizar auto-ack para operaciones críticas.

El mensaje solamente se confirma después de que el procesamiento haya sido persistido correctamente:

```text
receive
   ↓
process (Use Case)
   ↓
persist (DB)
   ↓
msg.ack().await
```

No:

```text
receive
   ↓
msg.ack().await
   ↓
process
```

---

## Retry y Dead Letter

Cada consumidor debe definir:

- política de retries con exponential backoff;
- límites de reintentos;
- dead-letter subject/stream;
- estrategia de recuperación.

---

## At-least-once

La arquitectura debe asumir:

```text
AT-LEAST-ONCE DELIVERY
```

Nunca asumir:

```text
EXACTLY-ONCE
```

Por lo tanto, todos los consumidores deben diseñarse para tolerar mensajes duplicados.

---

## JetStream vs Core NATS

Para eventos que deben:

- sobrevivir reinicios;
- permitir replay;
- garantizar persistencia;

utilizar:

```text
NATS JetStream (async-nats)
```

Core NATS solamente es aceptable para señales efímeras no críticas cuando corresponda, por ejemplo:

```text
invalidación de cache en tiempo real
```

---

## Comunicación síncrona

La comunicación síncrona:

```text
BFF → módulo
```

o:

```text
módulo → módulo
```

NO debe hacerse mediante NATS request-reply si puede evitarse.

Para comunicación síncrona utilizar:

```text
gRPC (tonic)
```

Para comunicación asíncrona utilizar:

```text
NATS JetStream (async-nats)
```

---

# 9. BFF, REST y Frontend

## Frontend

El Frontend:

- no contiene lógica de negocio;
- consume únicamente endpoints REST del BFF cuando exista BFF;
- maneja presentación;
- maneja estado de UI;
- puede realizar validaciones superficiales de formularios.

No debe duplicar reglas de negocio del backend.

---

## REST / OpenAPI

REST/OpenAPI es el contrato de entrada del BFF hacia el frontend.

Los endpoints REST y schemas OpenAPI (ej. generados con `utoipa`) definen:

- forma de los datos;
- operaciones disponibles;
- validación de transporte.

Los handlers REST (en `axum` o `actix-web`) son adaptadores puros:

```text
HTTP Request (axum)
        ↓
Handler REST
        ↓
DTO / gRPC Request
        ↓
Tonic Client (gRPC)
        ↓
Módulo de Backend
```

No contienen lógica de negocio.

---

# BFF

El BFF es una capa de:

- adaptación;
- composición;
- transformación;
- routing;
- validación de transporte HTTP.

Puede combinar respuestas de varios módulos.

### Prohibido

Nunca implementar reglas de negocio dentro del BFF.

Si un endpoint empieza a tener:

- condicionales de negocio;
- cálculos de dominio;
- decisiones basadas en estado de agregados;
- reglas de permisos de dominio;
- invariantes;

esa lógica pertenece al módulo correspondiente.

---

## Responsabilidades técnicas permitidas

El BFF puede realizar:

- routing HTTP (`axum::Router`);
- validación de payload y OpenAPI (`utoipa`);
- verificación de sesión/token JWT;
- agregación de respuestas concurrentes con `tokio::join!`;
- mapeo de errores gRPC (`tonic::Status`) a códigos de estado HTTP (`StatusCode`);
- cache de presentación de corta duración.

El cache del BFF nunca sustituye el cache de dominio del módulo.

---

## Prohibiciones del BFF

El BFF NO puede acceder directamente a:

```text
DB (sqlx)
Redis del módulo
entities
aggregates
repositories
domain/
application/
infrastructure/persistence/
```

El BFF solamente conoce:

```text
contrato gRPC público (.proto)
```

---

# Comunicación BFF → módulos

Cada módulo expone:

```text
module_public_api.rs
```

como servicio:

```text
infrastructure/grpc/
```

definido mediante:

```text
.proto versionado (compilado con tonic-build / prost)
```

El BFF actúa como cliente gRPC (`tonic::transport::Channel` + cliente tipado generado).

Nunca importa código interno del módulo.

Nunca conoce su implementación.

Solo conoce el contrato.

El contrato gRPC no debe cambiar porque el módulo pase de:

```text
módulo en monolito
```

a:

```text
microservicio independiente
```

Solamente cambia:

```text
dirección de red (endpoint gRPC)
```

---

## Diagrama

```text
Frontend
   │
   │ HTTP fetch / REST
   ▼
 BFF (axum)
   │
   │ gRPC (tonic)
   ▼
 module-a   module-b   module-c
     │          │          │
     ▼          ▼          ▼
  DB (sqlx)  DB (sqlx)  DB (sqlx)
```

---

## Comunicación entre módulos

La comunicación entre módulos:

**NO pasa por el BFF.**

El BFF nunca es intermediario de una llamada:

```text
module-a → module-b
```

La comunicación módulo → módulo sigue:

### Síncrona

```text
module-a → gRPC (tonic) → module-b
```

### Asíncrona

```text
module-a
   ↓
NATS JetStream (async-nats)
   ↓
module-b
```

---

# 10. Preparación explícita para extracción a microservicios

La separación de los cuatro backends ya es explícita. Dentro de cada backend, la extracción futura de uno de sus módulos debe ser aproximadamente:

```text
mover crate a su propio repositorio / binario
+
cambiar adapter de transporte y deployment
```

y no:

```text
reescribir dominio
+
reescribir aplicación
+
reescribir contratos
```

Para ello:

## 1. Cero imports internos cruzados

Nunca:

```rust
// PROHIBIDO en module-a/Cargo.toml:
// path = "../module-b/src/domain"
```

---

## 2. Comunicación remota desde el día uno

La comunicación utiliza:

```text
gRPC (tonic)
NATS JetStream (async-nats)
```

desde el principio.

Extraer un módulo solamente cambia:

```text
network address
```

No cambia:

```text
contract (.proto / event schemas)
domain
application
```

---

## 3. Datos propios

Todos los módulos se conectan a la misma base de datos PostgreSQL compartida, y cada uno posee sus propios datos y schema:

```text
module-a → PostgreSQL schema A
module-b → PostgreSQL schema B
module-c → PostgreSQL schema C
```

Nunca:

```text
shared tables / tables owned by another module
```

---

## 4. Eventos versionados

Los eventos de integración ya deben estar:

- desacoplados;
- versionados;
- documentados con schemas serde/Protobuf.

El microservicio extraído simplemente se conecta al mismo sistema de eventos.

---

## 5. Configuración por módulo

Variables de entorno, feature flags y configuración deben estar aisladas por crate.

Ejemplo:

```text
module_config.rs
```

La configuración debe poder trasladarse con el módulo.

---

## 6. Observabilidad por módulo

Cada módulo debe tener:

- logs estructurados (`tracing`);
- atributo `module` / `service.name`;
- `trace_id` propagado;
- métricas;
- tracing distribuido con OpenTelemetry.

El `trace_id` debe propagarse mediante:

```text
gRPC metadata (tonic)
NATS headers / payload
HTTP headers (axum)
```

según corresponda.

---

## 7. Checklist de extracción

Antes de extraer un módulo:

```text
¿Depende del código interno de otro módulo?
¿Comparte transacción sqlx con otro módulo?
¿Comparte estado en memoria (OnceCell, static, Mutex global) con otro módulo?
```

Las tres respuestas deben ser:

```text
NO
NO
NO
```

Si es así, la extracción es arquitectónicamente segura, sujeto a validar configuración, contratos, infraestructura y despliegue.

---

# 11. Shared Kernel — uso muy restringido

El crate `shared-kernel` SOLO contiene elementos verdaderamente universales y compartidos.

Ejemplos:

```rust
Result<T, E> // Helpers comunes si aplica
Uuid         // Re-export o Newtype universal
Clock        // Trait de reloj para testabilidad
IdGenerator  // Trait para IDs
errores base
tipos primitivos universales
```

Un `Money`, `Currency`, `Address` u otro Value Object solo debe estar allí si realmente es un concepto compartido por contrato y con semántica idéntica.

No debe agregarse algo simplemente porque:

```text
"dos módulos lo necesitan"
```

Si dos módulos necesitan conceptos similares pero tienen reglas distintas:

**se duplican.**

Es preferible:

```text
duplicación
```

antes que:

```text
acoplamiento incorrecto.
```

---

# 12. Observabilidad — logs, métricas y tracing

La instrumentación es obligatoria en TODA capa relevante:

```text
módulos
BFF (axum)
REST
gRPC (tonic)
NATS (async-nats)
DB (sqlx)
Redis
```

Debe existir:

- logging estructurado con el crate `tracing`;
- métricas (`opentelemetry::metrics`);
- distributed tracing (`tracing-opentelemetry`);
- correlation ID;
- trace ID.

### Errores estructurados y diagnosticables

Esta regla se aplica a **todos** los dominios, módulos, workers, BFF, integraciones y frontends y a **todas las fases y subfases**, desde su inicio. No se difiere a una fase transversal posterior. Un mensaje genérico como `server error`, `temporarily unavailable` o un tipo Rust sin contexto no es suficiente como diagnóstico interno.

- Cada error debe conservar su clase/código estable, bounded context, operación, dependencia y resultado; propagar `correlation_id` y `trace_id` en HTTP, gRPC, base de datos y eventos cuando existan.
- Los logs deben registrar metadatos diagnósticos seguros: código SQLSTATE/gRPC/HTTP o del proveedor, clase de error, componente/operación y, cuando aplique, constraint/tabla/columna o estado de reintento. Distingue fallos de validación, autorización, conflicto, timeout, pool/conectividad, persistencia, serialización y proveedor.
- No reduzcas un error de infraestructura a un `error_type` genérico cuando el código de error o metadatos seguros estén disponibles. No registres contraseñas, cookies, tokens, URL firmadas, credenciales, cuerpos de contenido ni valores de entrada sensibles. El mensaje/detail original de una dependencia solo se registra si se revisó que no puede contener dichos valores; en caso contrario registra campos seguros y una clase explícita.
- REST devuelve un envelope estable con `code`, `message` y `details` seguros, más correlation/request ID; `details` describe categoría, operación segura, posibilidad/tiempo de reintento o campos que el usuario puede corregir. Los errores de campo incluyen rutas/códigos de validación sin repetir valores sensibles. No devuelve SQL, stack traces, nombres internos sensibles ni secretos.
- El frontend traduce el envelope a una explicación clara y accionable, conserva la referencia de solicitud para soporte y permite recuperación cuando sea posible. No presenta `No se pudo contactar con el servidor` si el backend devolvió un error tipado más específico; tampoco muestra errores internos de DB o proveedor al usuario final.
- Agrega pruebas de contrato para el mapeo error de dominio → gRPC → HTTP → interfaz y verifica que los metadatos útiles se registren sin PII/secretos.
- Un evento de finalización que solo contiene método, ruta, status y duración es un resumen HTTP, no un diagnóstico. Para cada solicitud con error, correlaciona eventos de entrada, llamada a dependencia, fallo causal y traducción final; agrega duración por dependencia separada de duración total. En cada RPC, un span servidor extrae `traceparent` y `x-correlation-id`, registra método/código/duración y hace que el contexto esté disponible a los handlers. Si no puede confirmarse causa/dependencia/reintento, se registra explícitamente como no clasificado; está prohibido inferirlos a partir solo del status.
- Cada fase y subfase incluye pruebas unitarias y pruebas de integración/aceptación automatizadas en las fronteras reales aplicables. La aceptación debe emparejar el fallo específico de operación con el cierre HTTP por `correlation_id` y `trace_id`, asegurar que el cierre conserva su código/categoría y comprobar respuesta/log seguros sin secretos o PII. No se acepta como única evidencia compilar, comprobar el status, ni exigir un recorrido manual del navegador para demostrar el backend/BFF.

---

## Propagación de contexto

La propagación debe cubrir toda la cadena:

```text
HTTP/REST (axum)
   ↓
BFF
   ↓
gRPC metadata (tonic)
   ↓
Module
   ↓
Database (sqlx) / Redis
   ↓
NATS JetStream (async-nats)
   ↓
otro Module
```

---

## OpenTelemetry

**OpenTelemetry** es el estándar de instrumentación.

Debe utilizarse como mecanismo común de:

- tracing;
- métricas;
- propagación de contexto vía OTLP gRPC.

Stack estándar:

- Spans y logs emitidos vía `tracing` y `tracing-opentelemetry`.
- Exportados vía OTLP gRPC (`opentelemetry-otlp`) al **OpenTelemetry Collector**.
- Collector enruta trazas a **Tempo**, métricas a **Prometheus** (con **Alertmanager**) y logs a **Loki**.
- Profiling continuo con **Pyroscope** (`pyroscope_pprofrs`).
- Visualización unificada en **Grafana**.

---

## Métricas segmentadas

Las métricas deben segmentarse por:

- módulo/bounded context (`service.name`);
- endpoint REST;
- método gRPC;
- consumidor NATS;
- productor NATS.

Nunca depender únicamente de una métrica global del proceso.

---

## Logging

Prohibido utilizar:

```rust
println!
eprintln!
dbg!
```

como mecanismo de observabilidad de producción.

Todo log de producción debe ser:

```text
estructurado (JSON)
```

emitido a través de `tracing::info!`, `tracing::warn!`, `tracing::error!`.

---

## Campos mínimos

Cada log/span debe incluir, cuando corresponda:

```text
timestamp
service.name
module
environment
level
trace_id
span_id
request_id
correlation_id
operation
error
duration
```

---

## Datos prohibidos

Nunca registrar:

- secretos;
- tokens;
- contraseñas;
- credenciales;
- información sensible;
- PII.

Los datos deben:

```text
sanitizarse
enmascararse
```

antes de ser registrados (no usar `#[derive(Debug)]` ingenuo sobre structs que contengan credenciales — implementar `Debug` personalizado que oculte el valor sensible).

---

# 12.1 Métricas mínimas obligatorias por capa

| Capa | Métricas mínimas |
|---|---|
| HTTP/REST (BFF) | `requests_total`, `requests_failed_total`, `request_duration`, `request_rate`, `status_codes`, `active_requests` |
| gRPC (tonic) | `calls_total`, `calls_failed_total`, `call_duration`, `status_code`, `active_calls` |
| Base de datos (sqlx) | `query_duration`, `query_errors`, `pool_size`, `active_connections`, `idle_connections`, `pool_wait_time` |
| Redis | `operations_total`, `operation_duration`, `errors`, `hit_rate`, `miss_rate`, `connection_errors` |
| NATS/JetStream | `messages_published`, `messages_consumed`, `processing_duration`, `ack_failures`, `redeliveries`, `consumer_lag`, `pending_messages`, `dlq_messages` |
| Sistema | CPU, RAM, disco, red, file descriptors, tareas `tokio` activas |

---

# 12.2 Latencia — nunca solo el promedio

Medir siempre:

```text
p50
p90
p95
p99
p99.9
```

cuando sea relevante para el SLO.

El promedio **NUNCA** debe ser la única métrica de latencia.

El promedio puede ocultar solicitudes extremadamente lentas.

Toda operación crítica debe tener objetivos de latencia documentados y parametrizables.

---

# 13. SLO / SLA / SLI y Health Checks

## 13.1 SLI / SLO / Error Budget

Para toda operación crítica definir:

### SLI

Qué se mide.

### SLO

Cuál es el objetivo.

### Error Budget

Cuánto margen de error se permite derivado del SLO.

Formato de referencia:

```text
Availability SLO: 99.9%
Latency SLO: p99 < X ms
Error rate: < X%
```

Los valores anteriores son solamente ejemplos.

### Prohibido inventar valores arbitrarios

Si el proyecto todavía no tiene objetivos definidos:

```text
dejar valores parametrizables
```

y documentarlos, por ejemplo:

```text
slo.md
```

por módulo o bounded context.

Nunca hardcodear `"99.9%"` simplemente porque parece razonable.

---

# 13.2 Health Checks

Implementar cuando corresponda:

```text
liveness
readiness
startup
```

## Liveness

Determina únicamente si el proceso está vivo.

No debe depender de servicios externos.

---

## Readiness

Determina si el servicio puede recibir tráfico.

Puede depender de dependencias críticas (PostgreSQL, Redis, NATS).

---

## Startup

Determina si el servicio ha terminado correctamente su proceso inicial de arranque (ejecución de migraciones, calentamiento de pools).

---

## Prohibición

No crear un health check que simplemente devuelva `200 OK` sin verificar el estado real necesario.

También evitar que una dependencia externa no crítica haga que todo el servicio aparezca como muerto.

Diferenciar:

```text
dependencias críticas
dependencias no críticas
```

---

# 14. Testing — obligatorio, no opcional

Mantener los siguientes niveles:

```text
Unit
Integration
Contract
Component
End-to-End
Performance
Chaos
```

Cada nivel tiene un propósito diferente.

Ningún nivel sustituye completamente a otro.

---

# 14.1 Unit Testing — Domain

El dominio debe poder probarse con `cargo test`:

```text
sin Docker
sin DB
sin Redis
sin NATS
sin red
sin infraestructura externa
```

Los tests deben ser:

- rápidos;
- deterministas;
- aislados (módulos `#[cfg(test)]`).

Probar obligatoriamente:

- invariantes;
- reglas de negocio;
- Value Objects;
- Aggregates;
- Domain Services;
- Domain Events.

---

# 14.2 Application Testing — Use Cases

Los casos de uso deben probarse usando la implementación real y sus adapters reales contra infraestructura aislada; no sustituyas puertos, repositorios o servicios por mocks, fakes, stubs o adapters en memoria.

Para reglas puras del dominio, usa pruebas unitarias sin dependencias externas. Para casos de uso con persistencia o mensajería, ejecuta PostgreSQL, Redis o NATS reales en infraestructura aislada y aplica las migraciones reales. No precargues fixtures ni datasets de ejemplo: crea registros transitorios válidos mediante los flujos reales y límpialos al finalizar.

No se permite:

```text
mock implementations
in-memory fake adapters
stubbed services
```

Los tests deben verificar:

- flujo correcto;
- manejo de errores (`Result::Err`);
- idempotencia;
- autorización cuando corresponda;
- publicación de eventos;
- comportamiento ante fallos de dependencias.

---

# 14.3 Integration Testing

Los adapters reales deben probarse contra infraestructura real o equivalente controlada.

Preferiblemente mediante:

```text
testcontainers-rs
```

para tecnologías como:

```text
PostgreSQL
Redis
NATS JetStream
```

cuando correspondan al proyecto.

No sustituyas integraciones externas por servidores HTTP simulados. Usa el sandbox oficial del proveedor y el adapter real. Si no existe sandbox o faltan credenciales, informa que la integración no pudo verificarse y no la declares completa.

Verificar:

- migrations (`sqlx migrate`);
- repositories;
- transactions (`sqlx::Transaction`);
- serialización/deserialización con `serde`;
- Redis pooling y comandos;
- publishing NATS;
- consuming NATS;
- acknowledgements (`msg.ack().await`);
- retries;
- idempotencia;
- Outbox relay;
- deduplicación/Inbox.

---

# 14.4 Contract Testing

Todo contrato externo debe tener contract tests.

Incluye, cuando exista:

```text
REST / OpenAPI (validación de schema / snapshots)
protobuf / gRPC (compatibilidad con buf breaking o tests de schemas)
NATS event schemas (tests de serde contra payloads versionados)
```

Un cambio incompatible debe detectarse automáticamente en CI antes del deployment.

Los contratos versionados no deben romper consumidores existentes.

---

# 14.5 End-to-End Testing

Los flujos críticos deben probarse siguiendo la cadena completa:

```text
Frontend / Client
       ↓
REST (axum)
       ↓
      BFF
       ↓
  gRPC (tonic)
       ↓
    Module
       ↓
Database (sqlx)
       ↓
NATS JetStream (async-nats)
```

cuando corresponda.

Los E2E deben cubrir únicamente:

```text
flujos críticos
```

No deben sustituir:

```text
unit tests
integration tests
contract tests
```

---

# 15. Rendimiento y Pruebas de Carga

Herramienta estándar para pruebas externas:

```text
k6
```

para:

- load testing;
- stress testing;
- spike testing;
- soak testing;
- performance regression testing.

Herramienta para micro-benchmarks internos de Rust:

```text
criterion
```

para funciones y algoritmos críticos del dominio.

Medir como mínimo con k6:

```text
requests/sec
throughput
latency p50
latency p95
latency p99
error rate
concurrent users/connections
```

Ejecutar siempre contra ambientes:

```text
controlados
reproducibles
```

Prohibido utilizar producción para pruebas destructivas de carga.

---

# 15.1 Perfiles de prueba de carga

## Load Test

Carga esperada normal.

## Stress Test

Incrementar progresivamente la carga hasta encontrar el límite del sistema.

## Spike Test

Incrementos bruscos de tráfico.

## Soak Test

Carga sostenida durante un período prolongado para detectar:

- memory leaks;
- connection leaks;
- degradación progresiva;
- acumulación de mensajes en NATS;
- crecimiento de latencia.

## Breakpoint Test

Determinar el punto donde:

- throughput deja de crecer;
- p99 aumenta significativamente;
- aparecen errores;
- aparecen timeouts;
- aparece saturación.

---

# 15.2 Performance Budgets

Definir budgets explícitos para:

- latencia;
- error rate;
- CPU;
- memoria;
- conexiones DB (`sqlx::Pool`);
- latencia Redis;
- consumer lag NATS.

Una modificación que empeore significativamente una métrica crítica constituye una:

```text
regresión de rendimiento
```

y debe investigarse antes de aprobarse.

Idealmente debe bloquearse el merge/deploy cuando el performance budget esté integrado al CI.

---

# 16. Resiliencia

Implementar cuando corresponda:

```text
timeouts
retries
exponential backoff
jitter
circuit breakers
bulkheads
rate limiting
backpressure
load shedding
idempotencia
dead-letter queues/streams
graceful degradation
```

## Prohibiciones

Nunca:

```text
retries infinitos
```

Nunca:

```text
retry agresivo sin backoff
```

Nunca:

```text
retry sin jitter cuando pueda producir retry storm
```

Nunca reintentar automáticamente errores no recuperables.

Por ejemplo:

```text
validation error / domain invariant error
```

no debe tratarse como un error transitorio.

---

# 16.1 Timeouts

Toda comunicación externa debe tener timeout explícito utilizando `tokio::time::timeout`.

Incluye:

```text
HTTP (axum / reqwest)
gRPC (tonic timeout)
PostgreSQL (sqlx query timeout)
Redis (connection / command timeout)
NATS request/reply
```

cuando corresponda.

Nunca depender de un timeout implícito indefinido.

Los timeouts deben ser:

```text
configurables por entorno (platform::config)
```

---

# 16.2 Retries

Los retries deben tener:

- crate recomendado: `backoff` o `tokio-retry`;
- límite máximo;
- exponential backoff;
- jitter;
- aplicación únicamente a errores recuperables (I/O transitorio, timeout de red);
- compatibilidad con idempotencia.

Evitar:

```text
retry storms
```

Un retry no debe convertir una dependencia degradada en una cascada de fallos.

---

# 16.3 Circuit Breaker

Usar circuit breaker cuando una dependencia externa pueda provocar:

```text
cascading failures
```

Crates recomendados: `failsafe` o `circuitbreaker-rs`.

Estados:

```text
CLOSED
   ↓
 OPEN
   ↓
HALF_OPEN
   ↓
CLOSED
```

Debe existir observabilidad de:

- cambios de estado;
- errores;
- tiempo abierto;
- recuperaciones.

---

# 16.4 Bulkhead

Aislar recursos críticos:

- `tokio::sync::Semaphore` para limitar concurrencia por operación;
- connection pools dimensionados por caso de uso (`sqlx::PoolOptions`, `deadpool-redis`);
- límites de tareas asíncronas.

Una operación saturada no debe consumir todos los recursos disponibles.

Una operación lenta no debe bloquear necesariamente todas las demás.

---

# 16.5 Graceful Shutdown

Todo servicio debe implementar graceful shutdown mediante `tokio::signal` (`SIGTERM`/`SIGINT`) y tokens de cancelación (`tokio_util::sync::CancellationToken`):

1. dejar de aceptar nuevo tráfico (cerrar listener axum / tonic);
2. marcar readiness como no disponible;
3. permitir terminar solicitudes activas dentro de un límite de tiempo (`tokio::time::timeout`);
4. detener consumidores NATS cancelando sus tareas `tokio::spawn`;
5. procesar/confirmar correctamente mensajes pendientes según la estrategia definida;
6. cerrar pools de conexión (`sqlx::Pool::close().await`, Redis) y liberar recursos;
7. finalizar el proceso.

Nunca terminar abruptamente una operación crítica si puede evitarse.

---

# 16.6 NATS JetStream Resilience

Los consumidores deben asumir:

```text
at-least-once delivery
```

Nunca:

```text
exactly-once
```

Los consumidores deben ser idempotentes.

Implementar:

- acknowledgements explícitos (`msg.ack().await`);
- retry policy;
- backoff;
- manejo de redelivery;
- deduplicación;
- dead-letter strategy;
- monitoreo de consumer lag expuesto a Prometheus.

Un mensaje nunca debe perderse silenciosamente.

---

# 17. Chaos Engineering y Fault Injection

Implementar pruebas de resiliencia progresivamente.

Ejemplos:

```text
detener/reiniciar un módulo o contenedor
introducir latencia con toxiproxy
introducir errores simulados
cortar Redis
cortar PostgreSQL
cortar NATS
provocar pérdida de conexiones
aumentar latencia DB
saturar CPU
saturar memoria
aumentar temporalmente tráfico con k6
```

Cada experimento de Chaos debe documentar:

```text
Hypothesis
Blast Radius
Experiment
Expected Result
Observed Result
Recovery
Conclusion
```

Nunca ejecutar experimentos destructivos sin limitar previamente:

```text
Blast Radius
```

---

## Fault Injection

Las pruebas deben validar que el sistema se comporta correctamente cuando una dependencia falla.

Ejemplos genéricos:

```text
Module → Redis DOWN
Module → Database SLOW
Module → NATS UNAVAILABLE
Module → External Service TIMEOUT
```

Ante una dependencia degradada debe existir, cuando corresponda:

```text
timeout
backpressure
concurrency limits (Semaphore)
observability (Grafana / Tempo)
recovery strategy
```

---

# 18. Alerting y Observabilidad de Extremo a Extremo

Crear alertas sobre **síntomas reales de negocio/usuario**, no únicamente sobre utilización de recursos.

Reglas definidas en **Prometheus** y enrutadas por **Alertmanager**.

Priorizar:

- error rate;
- latencia p95/p99;
- saturación;
- consumer lag de NATS;
- queue depth;
- agotamiento de conexiones DB (`sqlx::Pool`);
- fallos Redis;
- redeliveries NATS;
- OOM;
- crash loops;
- disponibilidad.

No crear alertas excesivamente sensibles que produzcan:

```text
alert fatigue
```

---

## Trace de extremo a extremo

Toda solicitud debe poder rastrearse mediante:

```text
trace_id
```

y propagación de contexto OTel a través de:

```text
Frontend
   ↓
REST (axum)
   ↓
BFF
   ↓
gRPC metadata (tonic)
   ↓
Module
   ↓
DB (sqlx) / Redis
   ↓
NATS JetStream (async-nats)
   ↓
otro Module
```

Debe ser posible investigar una operación completa en Grafana (Tempo + Loki + Prometheus) sin depender únicamente de logs manuales.

---

# 19. Definition of Done

El agente **NO debe considerar una implementación "completa" únicamente porque el código compile con `cargo build` o los tests unitarios pasen.**

Antes de dar por terminada una funcionalidad, verificar:

## Arquitectura

- [ ] El bounded context está correctamente identificado como Crate en el workspace.
- [ ] La funcionalidad vive en el módulo/crate correcto.
- [ ] No existen dependencias internas de crate entre módulos (`Cargo.toml`).
- [ ] Las dependencias apuntan hacia adentro.
- [ ] El dominio no depende de infraestructura ni crates externos de persistencia/red.
- [ ] La aplicación no depende de infraestructura concreta.
- [ ] Los adapters están en infraestructura implementando traits.
- [ ] Los contratos públicos están definidos correctamente en `module_public_api.rs`.

## SOLID

- [ ] SRP respetado: structs y funciones con una sola responsabilidad.
- [ ] OCP respetado: extensión mediante `impl Trait`.
- [ ] LSP respetado: implementaciones de traits sustituibles sin `panic!`.
- [ ] ISP respetado: traits pequeños y específicos.
- [ ] DIP respetado: inversión de dependencias con `Arc<dyn Trait>`.

## Archivos

- [ ] Ningún archivo `.rs` supera 400 líneas físicas; se prefiere aproximadamente 200 cuando la cohesión lo permita.
- [ ] No existen archivos "bolsa" (`utils.rs`).
- [ ] Una responsabilidad pública principal por archivo.
- [ ] El nombre del archivo coincide con su export principal en `snake_case`.
- [ ] `mod.rs` o `lib.rs` solo contienen re-exports (`pub mod`, `pub use`), sin lógica.

## DDD

- [ ] Bounded Context definido.
- [ ] Lenguaje ubicuo definido.
- [ ] Entidades correctamente delimitadas con métodos de mutación explícitos.
- [ ] Value Objects implementados con Newtypes y validación en constructores.
- [ ] Agregados protegen sus invariantes.
- [ ] Domain Services utilizados solamente cuando corresponde.
- [ ] Domain Events definidos en enums para mutaciones relevantes.
- [ ] ACL aplicada cuando se consume otro contexto o contratos gRPC externos.

## Application

- [ ] Un caso de uso (struct + impl) por archivo.
- [ ] Los casos de uso solamente orquestan.
- [ ] No contienen reglas de negocio duplicadas.
- [ ] Utilizan DTOs de entrada y salida.
- [ ] No exponen entidades de dominio directamente.
- [ ] Los errores están tipados con enums y `thiserror`.
- [ ] Los consumidores externos son idempotentes.

## Database

- [ ] Todos los módulos usan la misma base física PostgreSQL.
- [ ] Cada módulo posee sus propios datos/schema.
- [ ] No existen JOINs entre módulos.
- [ ] Migraciones independientes por módulo (`sqlx migrate`) aplicadas de forma coordinada a la base compartida.
- [ ] Transacciones `sqlx::Transaction` respetan los límites del agregado.
- [ ] Outbox Pattern implementado para publicación de eventos.
- [ ] Repositorios devuelven entidades de dominio reconstituidas.
- [ ] `sqlx` y queries aisladas en infraestructura.

## Redis

- [ ] Redis está detrás de un trait `CachePort`.
- [ ] TTL definido.
- [ ] Invalidación documentada.
- [ ] Keys namespaced: `<module>:<entity>:<id>:<subresource>`.
- [ ] Redis no es fuente única de verdad.
- [ ] Locks distribuidos implementados mediante mecanismos probados (Redlock).

## NATS

- [ ] Eventos versionados con structs `Serialize`/`Deserialize`.
- [ ] Subjects correctamente nombrados en minúsculas y tiempo pasado.
- [ ] Publisher detrás de trait `EventPublisher`.
- [ ] Subscribers separados en tareas `tokio::spawn`.
- [ ] ACL aplicada al recibir mensajes externos.
- [ ] Consumers idempotentes.
- [ ] Ack manual (`msg.ack().await`) tras confirmación en DB.
- [ ] Retry policy con backoff/jitter.
- [ ] Dead-letter strategy configurada.
- [ ] Consumer lag observable en Prometheus.

## gRPC

- [ ] `.proto` versionado compilado con `tonic-build`.
- [ ] Contrato público expuesto en `infrastructure/grpc`.
- [ ] No existen imports ni dependencias internas entre crates.
- [ ] BFF conoce únicamente el contrato gRPC.
- [ ] Comunicación síncrona módulo → módulo usa gRPC vía cliente `tonic`.

## BFF

- [ ] Sin lógica de negocio.
- [ ] Sin acceso directo a DB (`sqlx`).
- [ ] Sin acceso directo a Redis del módulo.
- [ ] Sin acceso a entidades internas de dominio.
- [ ] Sin acceso a repositorios internos.
- [ ] Endpoints REST / OpenAPI (`axum` + `utoipa`) correctamente definidos.
- [ ] Errores gRPC (`tonic::Status`) correctamente adaptados a `StatusCode` HTTP.

## Observabilidad

- [ ] Logs estructurados con `tracing` (formato JSON en producción).
- [ ] No existen secretos ni PII en logs (implementación manual de `Debug` cuando corresponda).
- [ ] `trace_id` propagado a lo largo de toda la cadena.
- [ ] `correlation_id` disponible.
- [ ] Métricas por módulo emitidas a Prometheus.
- [ ] Métricas REST, gRPC, DB, Redis y NATS instrumentadas.
- [ ] Latencias p50/p90/p95/p99 medidas.

## Health

- [ ] Liveness implementado (sin dependencias externas).
- [ ] Readiness implementado (con verificación de DB, Redis, NATS).
- [ ] Startup check implementado cuando corresponde.
- [ ] Dependencias críticas diferenciadas de las no críticas.

## Testing

- [ ] Unit tests en `#[cfg(test)]` ejecutables con `cargo test`.
- [ ] Pruebas de casos de uso con adapters reales e infraestructura aislada; sin mocks, fakes, stubs ni fixtures.
- [ ] Integration tests con `testcontainers-rs` (PostgreSQL, Redis, NATS).
- [ ] Contract tests para OpenAPI y Protobuf.
- [ ] Component tests cuando corresponda.
- [ ] E2E de flujos críticos.
- [ ] Performance tests con `k6` y benchmarks con `criterion`.
- [ ] Chaos tests cuando corresponda.

## Calidad y Estilo Rust

- [ ] Pasa `cargo check` sin errores.
- [ ] Pasa `cargo clippy --all-targets -- -D warnings` sin warnings.
- [ ] Pasa `cargo fmt --check`.
- [ ] No contiene `unwrap()`, `expect()` o `panic!` en rutas de producción.
- [ ] No contiene bloques `unsafe` sin documentación justificativa.

## Performance

- [ ] Load test con k6.
- [ ] Stress test.
- [ ] Spike test.
- [ ] Soak test.
- [ ] Breakpoint test cuando corresponda.
- [ ] Performance budgets definidos.
- [ ] No se ejecutaron pruebas destructivas en producción.

## Resiliencia

- [ ] Timeouts explícitos con `tokio::time::timeout`.
- [ ] Retries limitados con backoff y jitter.
- [ ] Circuit breaker cuando corresponda.
- [ ] Bulkhead (`Semaphore` y pools dimensionados).
- [ ] Rate limiting cuando corresponda.
- [ ] Backpressure cuando corresponda.
- [ ] Idempotencia asegurada.
- [ ] Dead-letter strategy.
- [ ] Graceful shutdown con `tokio::signal` y `CancellationToken`.

---

# 19.1 Matriz de Verificación de Cobertura por Bounded Context

Cada proyecto debe mantener una matriz equivalente adaptada a sus bounded contexts/módulos; indique el crate que los contiene cuando estén separados.

**No utilizar nombres de dominio específicos en este documento de referencia.**

Plantilla:

| Bounded Context (Crate) | Entidades de Dominio | Command Handlers | Query Handlers | Suites de Prueba |
|---|---|---|---|:---:|
| `module-a` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `module-b` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `module-c` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `module-d` | `<EntityA>, <EntityB>` | `<CommandA>, <CommandB>` | `<QueryA>, <QueryB>` | 🟢 / 🟡 / 🔴 |
| `infra/cross` | `<CrossCuttingEntity>` | `<CrossCuttingCommand>` | `<CrossCuttingQuery>` | 🟢 / 🟡 / 🔴 |

La matriz real debe contener exclusivamente los bounded contexts del proyecto actual.

No inventar módulos simplemente para llenar la tabla.

---

# 20. Prohibiciones explícitas

Antes de entregar código Rust, verifica que NO hiciste ninguna de estas cosas:

- [ ] Depender de `sqlx`, `diesel`, `sea-orm`, cliente Redis o cliente NATS dentro de `domain/` o `application/`.
- [ ] Poner lógica de negocio en un handler REST, handler gRPC, listener de evento o repositorio.
- [ ] Hacer `use` o referencia directa al crate o carpeta interna de otro módulo.
- [ ] Modificar dos agregados de módulos distintos dentro de la misma transacción síncrona.
- [ ] Modificar dos agregados del mismo módulo dentro de una misma transacción sin la justificación documentada requerida.
- [ ] Exponer una struct de entidad de dominio directamente como respuesta HTTP/REST o payload gRPC.
- [ ] Exponer una entidad de dominio directamente como payload de evento sin DTO/mapper.
- [ ] Publicar un evento sin Outbox Pattern cuando la consistencia DB/evento lo requiera.
- [ ] Crear un consumidor NATS que no sea idempotente.
- [ ] Utilizar auto-ack antes de procesar y persistir correctamente un mensaje.
- [ ] Usar Redis como única fuente de verdad de un dato crítico.
- [ ] Añadir al crate `shared-kernel` algo que en realidad pertenece a un solo módulo.
- [ ] Nombrar structs o traits genéricos como `Manager`, `Helper`, `Util` o `Service` sin responsabilidad clara y acotada.
- [ ] Generar un archivo `.rs` de más de 400 líneas físicas.
- [ ] Crear un archivo "bolsa" con múltiples structs/funciones no relacionadas.
- [ ] Usar `mod.rs` o `lib.rs` para escribir lógica en lugar de re-exports limpios.
- [ ] Escribir lógica de negocio dentro de handlers REST del BFF.
- [ ] Escribir lógica de negocio dentro de cualquier archivo del BFF (`apps/bff`).
- [ ] Hacer que el BFF acceda directamente a una base de datos PostgreSQL.
- [ ] Hacer que el BFF acceda directamente a Redis de un módulo.
- [ ] Hacer que el BFF acceda directamente a entidades, agregados o repositorios internos de un módulo.
- [ ] Hacer que el BFF dependa del crate interno de un módulo (solo debe conocer el `.proto` / gRPC client).
- [ ] Comunicar el BFF con un módulo mediante un mecanismo distinto de gRPC para comunicación síncrona.
- [ ] Comunicar dos módulos de forma síncrona mediante un mecanismo distinto al servicio gRPC del módulo destino.
- [ ] Hacer que un módulo lea o modifique directamente schemas/tablas propiedad de otro módulo.
- [ ] Hacer JOINs SQL entre datos pertenecientes a módulos diferentes.
- [ ] Hacer que un módulo acceda directamente al Redis de otro módulo.
- [ ] Usar NATS como sustituto arbitrario de comunicación síncrona cuando gRPC es el contrato adecuado.
- [ ] Utilizar Core NATS para eventos que requieren persistencia y replay.
- [ ] Utilizar un evento interno de dominio directamente como contrato externo sin versionarlo con serde.
- [ ] Permitir que el modelo de otro bounded context o tipos de `prost` se filtren dentro del dominio propio.
- [ ] Usar `println!`, `eprintln!` o `dbg!` como mecanismo de observabilidad de producción.
- [ ] Registrar secretos, credenciales, tokens o PII en logs, métricas o trazas.
- [ ] Usar `unwrap()`, `expect()` o `panic!` en código de producción que maneja datos de entrada o llamadas externas.
- [ ] Usar `unsafe` sin un comentario formal que explique por qué es seguro y por qué no es posible código seguro.
- [ ] Desactivar lints de clippy con `#[allow(...)]` sin justificación explícita documentada.
- [ ] Implementar un health check que solamente devuelva `200 OK` sin verificar el estado real necesario.
- [ ] Mezclar liveness con readiness.
- [ ] Implementar retries infinitos.
- [ ] Implementar retries sin backoff/jitter cuando correspondan.
- [ ] Reintentar errores no recuperables.
- [ ] Reportar latencia usando únicamente el promedio sin percentiles (p95/p99).
- [ ] Omitir p95/p99 en operaciones donde sean relevantes.
- [ ] Ejecutar pruebas de carga destructivas directamente en producción.
- [ ] Ejecutar experimentos de Chaos sin Blast Radius definido.
- [ ] Inventar SLOs arbitrarios sin documentarlos.
- [ ] Introducir una dependencia de framework en el dominio sin justificación arquitectónica explícita.
- [ ] Permitir que una implementación de infraestructura cambie las reglas de negocio del dominio.
- [ ] Duplicar reglas de negocio entre Frontend, BFF y backend.
- [ ] Crear un módulo únicamente por conveniencia técnica si no representa un límite de negocio o responsabilidad coherente.
- [ ] Fusionar bounded contexts solamente para evitar duplicación de código.
- [ ] Crear abstracciones compartidas únicamente para evitar una pequeña duplicación.
- [ ] Utilizar `shared-kernel` como depósito general de código reutilizable.

---

# 21. Instrucción final para el agente — Claude / Codex / Copilot / cualquier agente

> Cuando generes código Rust para un proyecto que adopte esta arquitectura, aplica TODAS las reglas anteriores sin excepción.
>
> Si el requerimiento del usuario es ambiguo respecto a en qué módulo/capa debe vivir algo, pregunta o indica explícitamente tu suposición antes de generar código.
>
> Si detectas que cumplir el requerimiento tal cual lo pide el usuario violaría alguna regla de esta guía —por ejemplo:
>
> - acceder a datos de otro módulo directamente;
> - importar código interno de otro crate;
> - poner lógica de negocio en el BFF;
> - omitir observabilidad;
> - usar `unwrap()` sobre input externo o I/O;
> - omitir resiliencia en una operación crítica;
> - compartir tablas o acceder directamente a datos propiedad de otro módulo;
> - publicar eventos sin el mecanismo de consistencia requerido;
> - inventar un SLO sin base;
> - introducir dependencias de infraestructura en el dominio;
>
> debes avisarlo y proponer la alternativa arquitectónicamente correcta en vez de generar el atajo que rompe la arquitectura.
>
> El objetivo permanente es que cada módulo conserve límites y contratos que permitan una extracción futura cuando exista una necesidad demostrable; esa posibilidad no exige un crate ni microservicio por módulo desde el inicio.
>
> Todo el sistema debe ser:
>
> - modular (Cargo workspace);
> - desacoplado;
> - observable (OpenTelemetry OTLP gRPC → Tempo / Prometheus / Loki / Pyroscope → Grafana / Alertmanager);
> - medible (k6 + criterion);
> - testeable (`cargo test` + infraestructura aislada real mediante `testcontainers-rs`);
> - resiliente (`tokio::time::timeout`, `backoff`, `failsafe`, graceful shutdown);
> - preparado para escalar;
> - preparado para evolucionar;
> - preparado para extracción progresiva a microservicios.
>
> Estas propiedades deben existir **por diseño desde el principio**, no como una capa añadida después.

---

# 22. Autoridad documental para ENSAMBRIA

Este archivo consolida arquitectura, políticas del proyecto y responsabilidades. Mantén aquí las decisiones y restricciones compartidas por esas áreas, sin duplicar el roadmap ni los requisitos detallados de estándares especializados. Cada cambio debe actualizar la sección dueña y mantener consistentes las referencias relacionadas. Un estándar universal reutilizable permanece separado y no reemplaza este contrato específico de ENSAMBRIA.

---

# 23. Regla de prioridad arquitectónica

Cuando exista conflicto entre:

```text
velocidad de implementación
```

y:

```text
integridad arquitectónica
```

la integridad arquitectónica tiene prioridad.

Cuando exista una razón legítima para romper una regla:

1. identificar la regla;
2. explicar por qué no puede cumplirse;
3. documentar la excepción;
4. registrar la decisión mediante ADR (Architecture Decision Record) cuando corresponda;
5. definir el alcance de la excepción;
6. evitar que la excepción se convierta en patrón general.

Nunca convertir una excepción puntual en una nueva regla implícita.

---

# 24. Principio final

ENSAMBRIA tiene cuatro backends diferentes e independientes, cada uno con un monolito modular, identidad, configuración, secretos, permisos, CI y despliegue propios. El BFF público y los BFF especializados de productos implementados son apps independientes; el BFF de administración vive inicialmente integrado en PLATFORM:

```text
Discovery ── REST/OpenAPI ───────► BFF público independiente ──gRPC──► PUBLIC
Tenant / Business / Mobile UI ──► BFF independiente de producto ─────► CORE/BUSINESS/PLATFORM/PUBLIC
Admin Plataforma ── REST/OpenAPI ► BFF PLATFORM integrado ──────────────► PLATFORM

CORE ◄──── gRPC síncrono si aplica ────► BUSINESS / PLATFORM / PUBLIC
  └──────── Outbox ── NATS JetStream ── Outbox ──────────┘
                     (eventos durables cuando se requieren)

Workspace Cargo compartido para build; binarios/configuración/despliegues separados.
Los cuatro backends usan una única base de datos PostgreSQL compartida, con schemas y permisos separados.
```

Cada dominio tiene un solo dueño. CORE posee identidad/contexto tenant, reservas, pagos y ledger; BUSINESS posee operación de establecimientos, Commerce, órdenes, facturación comercial, comisiones laborales y franquicias; PLATFORM posee administración comercial SaaS, planes, suscripciones, políticas de comisiones y facturación ENSAMBRIA; PUBLIC posee directorio/Marketplace, datos públicos autorizados y atribución. El plan Franquicia es una oferta comercial de PLATFORM, no el módulo de franquicias de BUSINESS.

CORE procesa pagos solicitados por PLATFORM o BUSINESS, pero cuentas, credenciales, callbacks y titularidad siguen separados por propósito. Factus SaaS pertenece a PLATFORM y Factus de establecimientos a BUSINESS. La tarifa comercial la define PLATFORM; CORE valida la política aplicable y registra el efecto transaccional. PUBLIC aporta la atribución verificada del canal y no modifica las reglas de CORE o BUSINESS. Los roles, sesiones, secretos, APIs y datos se mantienen separados según su propietario.

Los backends no se invocan entre sí por defecto. Se usa **gRPC** solo para una consulta o comando que necesite respuesta inmediata; se usa **Outbox + NATS JetStream** para distribuir hechos ya persistidos que necesiten procesamiento asíncrono, durable o replay. Si no hace falta coordinación, no hay llamada ni evento. No hay imports de implementación ni acceso cruzado a tablas; compartir workspace o infraestructura PostgreSQL no cambia estos límites.

**El objetivo no es fusionar los cuatro backends ni desplegar cada módulo por separado.** El objetivo es mantener cuatro límites de backend claros, con un monolito modular dentro de cada uno, el BFF público y los BFF de producto independientes, y el BFF de administración integrado en PLATFORM; todos operan con contratos seguros, mínimos y semánticamente correctos.

---

# 25. Gobierno del proyecto, documentación y entrega

## 25.1 Fuentes normativas

Este contrato establece arquitectura y políticas generales del proyecto. Las fuentes complementarias son:

- [docs/requerimiento.md](docs/requerimiento.md): producto, roadmap, fases y aceptación.
- [ENSAMBRIA_architecture_doc_engineering_governance_standard.md](docs/rules/ENSAMBRIA_architecture_doc_engineering_governance_standard.md): gobierno técnico y documental.
- [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md): gobierno del repositorio, ramas, PR, revisiones, CI/CD, seguridad de GitHub y releases.
- [ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md](docs/rules/ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md): método de calidad y pruebas.
- [ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md](docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md): reglas especializadas para pruebas de rendimiento, carga, estrés y capacidad.
- [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](docs/rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md) y [ENSAMBRIA_Privacy_Legal_Compliance.md](docs/rules/ENSAMBRIA_Privacy_Legal_Compliance.md): privacidad, ciclo de vida y cumplimiento jurisdiccional.
- [ENSAMBRIA_Geographic_Access_Policies.md](docs/rules/ENSAMBRIA_Geographic_Access_Policies.md): acceso geográfico.
- [ENSAMBRIA_Connection_Pooling_PostgreSQL.md](docs/rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md): conexión y pools PostgreSQL.
- [ENSAMBRIA_Transactional_Outbox_Inbox.md](docs/rules/ENSAMBRIA_Transactional_Outbox_Inbox.md): eventos durables.
- [ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md](docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md): observabilidad y confiabilidad operativa.
- [ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md](docs/rules/ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md) y [ENSAMBRIA_Open_Graph_SEO_Metadata.md](docs/rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md): calidad web y SEO/Open Graph.
- [ENSAMBRIA_README_Documentation_Quality_Security_Standard.md](docs/rules/ENSAMBRIA_README_Documentation_Quality_Security_Standard.md): contenido, estructura y seguridad del README raíz.

Cada requisito se mantiene en su fuente propietaria. Si hay solapamiento, prevalece la guía especializada en su materia y este contrato mantiene únicamente las decisiones arquitectónicas/de proyecto necesarias para aplicarla. Ante contradicciones reales, detén el alcance afectado y resuelve el conflicto mediante un cambio revisado; no debilites reglas ni conviertas propuestas en decisiones aprobadas.

## 25.2 Política de documentación

- No crear archivos o documentos Markdown innecesarios, duplicados, vacíos o sin propósito verificable. Antes de crear uno, comprobar si corresponde actualizar una fuente existente.
- Mantener documentación, contratos y código sincronizados. Marcar con claridad lo implementado, parcial, planificado, bloqueado o retirado; no afirmar funcionalidades sin evidencia.
- Mantener enlaces locales válidos y proteger secretos, credenciales, datos personales y detalles sensibles.
- Revisar los cambios documentales en el PR correspondiente.
- El README raíz presenta ENSAMBRIA, su estado real, arquitectura resumida, requisitos e instrucciones verificadas, y enlaza las guías. No debe acumular contratos, decisiones o detalles que pertenecen a documentos especializados. Para sus criterios específicos, seguir [ENSAMBRIA_README_Documentation_Quality_Security_Standard.md](docs/rules/ENSAMBRIA_README_Documentation_Quality_Security_Standard.md).

## 25.3 Gobierno de GitHub y Pull Requests

GitHub es el plano de control de calidad, seguridad, colaboración, trazabilidad y entrega desde Fase 0. [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md) es la fuente especializada para los controles operativos de repositorio; esta sección conserva las obligaciones de arquitectura y proyecto propias de ENSAMBRIA y debe aplicarse junto con ese estándar.

- `main` debe tener Ruleset/Branch Protection efectivo. Todo cambio, incluidos los documentales, se integra mediante PR; no se permiten push directo, force-push ni eliminación de `main`.
- Fase 0 define y, donde existan permisos, establece CODEOWNERS, plantillas, CI base, scripts reproducibles y seguridad de workflows. Verificar los controles realmente activos; limitaciones de permisos o plataforma se documentan como bloqueos, no como protección cumplida.
- Usar ramas breves y descriptivas por unidad coherente (`feat/`, `fix/`, `refactor/`, `test/`, `docs/`, `security/`, `infra/`). No usar una rama por archivo ni mezclar objetivos sin relación.
- El PR declara objetivo, alcance, requisitos, criterios de aceptación, riesgos y evidencia; revisa diff y secretos, exige resolución de conversaciones y rama actualizada con `main` o merge queue equivalente.
- Con una sola persona mantenedora, permitir PR con auto-revisión documentada sin exigir un segundo CODEOWNER ni aprobación independiente para cambios ordinarios. Mantener obligatorios PR, checks y protección de `main`; la auto-revisión no se presenta como aprobación independiente. Aplicar [§8.1 del estándar GitHub](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md) y reevaluar al incorporar colaboradores.
- Los cambios críticos de seguridad, permisos, autenticación, privacidad, infraestructura o workflows que requieran revisión independiente no se autoaprueban; si no hay revisor autorizado, detener la integración de ese alcance. Nunca simular aprobación humana.
- Aplicar mínimo privilegio a `GITHUB_TOKEN`, fijar Actions externas a SHA completo, establecer límites de tiempo y concurrencia segura. Separar CI de PR, CI de `main`, análisis programados, releases y despliegues. Un PR no despliega a producción.
- Un check requerido fallido, pendiente, cancelado u omitido no es aprobación. Tras integrar, verificar CI del commit exacto de `main`.

## 25.4 Paridad local y CI

Cada check obligatorio de GitHub Actions debe tener una entrada local versionada que aplique el mismo comando, configuración y criterio. CI invoca ese mismo punto de entrada; no se mantienen implementaciones paralelas. Ningún script puede ocultar fallos, omitir suites o convertir un resultado fallido en éxito.

Los comandos aplicables se determinan tras inspeccionar manifests, herramientas, lockfiles y scripts reales. Los siguientes comandos Rust son candidatos, no afirmaciones de que ya sean ejecutables:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo check --locked --workspace --all-targets
cargo test --locked --workspace
cargo test --locked --workspace --doc
```

Para cada frontend se usan sus scripts y lockfile reales; no se asume que todas las aplicaciones tengan comandos idénticos. Dependencias, paquetes o suites aún no disponibles se registran como bloqueos, sin crear gates placebo. Los controles se ejecutan según los archivos afectados, sin omitir verificaciones obligatorias.

## 25.5 PR, integración y cierre de fase

Antes de fusionar deben pasar todos los checks requeridos, completarse las revisiones, resolverse las conversaciones y satisfacerse los criterios de aceptación. Cada requisito conserva trazabilidad `requisito → caso → prueba → comando local/CI → ejecución/evidencia`. No se cierra una fase por el número de PRs integrados ni se inicia la siguiente con criterios pendientes. Una dependencia externa que impida protección o validación es un bloqueo explícito, no una aprobación. Los criterios de fase específicos los define el roadmap.

# 26. Responsabilidades y evidencia

Una persona puede asumir varios roles; las responsabilidades y evidencias siguen siendo distintas. Los roles asignan rendición de cuentas, no duplican ni reemplazan requisitos de las fuentes normativas.

| Rol | Responsabilidad | Evidencia |
|---|---|---|
| Domain / Product Owner | Intención del caso de uso, actores, reglas de negocio, resultados permitidos/prohibidos y semántica de errores | Inventario de casos de uso, invariantes, criterios de aceptación y decisiones explícitas |
| Architecture Owner | Ownership de dominios, límites de confianza, contratos, datos, integraciones, IDs canónicos, topología/presupuesto PostgreSQL y eventos | ADRs, context map, contratos, mapa de amenazas/controles, política de IDs, presupuesto de pools, mapa de eventos y matriz de integración |
| Implementer | Código y pruebas del comportamiento; validación, autorización, transacciones y mapeo seguro de errores | Resultados de pruebas aplicables, evidencia de migración/API e integración con dependencias reales |
| Quality / Test Owner | Desafío independiente de escenarios, trazabilidad, ejecución y brechas | Matriz y IDs de pruebas, comandos/resultados, escenarios no cubiertos e informe de aceptación |
| Security Reviewer | Autenticación/autorización, aislamiento tenant, secretos, replay, abuso, privacidad y fallo seguro | Revisión de seguridad, pruebas, hallazgos, riesgos residuales y aprobación independiente requerida |
| Privacy / Legal Owner | Requisitos de privacidad, derechos de titulares, roles de tratamiento, jurisdicciones, retención e interpretación legal | Inventario, fuentes oficiales, decisiones de Legal, responsables y trazabilidad requisito-control-prueba-evidencia |
| Observability / SRE Owner | Arquitectura de observabilidad, controles de telemetría, SLI/SLO, alertas, runbooks y confiabilidad | Decisiones de Fase 0, controles de telemetría, base aprobada de SLI/SLO y evidencia operacional |
| CI / Release Owner | Automatización reproducible, checks, paridad local/CI, gates y evidencia | Configuración efectiva de GitHub, resultados del commit exacto, comandos locales, checklist y reversión |
| Documentation Owner | Exactitud, enlaces, descubribilidad y mantenimiento documental | Documentos sincronizados con estado real; enlaces y contenido revisados en PR |
| AI / Automation Agent | Aplicar fuentes normativas, inspeccionar estado real, ejecutar trabajo autorizado e informar con precisión | Cambios, comandos y resultados exactos, brechas y ausencia de afirmaciones no sustentadas |

El Domain / Product Owner mantiene alcance y aceptación del roadmap. Architecture Owner documenta decisiones, ownership, contratos y matrices aplicables. Implementers entregan evidencia de sus cambios; Quality / Test Owners trazan requisitos a escenarios, pruebas y ejecuciones; CI / Release Owners verifican los checks del commit exacto; Documentation Owners mantienen sincronizados los documentos afectados.

La matriz de pruebas, los niveles, las particiones positivas/negativas, límites, duplicados/concurrencia, autorización, errores, privacidad, seguridad e integración se detallan en [ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md](docs/rules/ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md). Aplicar a cada caso de uso según corresponda; los mocks no sustituyen integración real requerida. Cada N/A requiere justificación técnica previa, responsable y alcance. No se acepta una fase o subfase mientras un criterio requerido carezca de evidencia ejecutada, falle o dependa de una decisión bloqueante sin resolver. Un documento, scaffold, test escrito o build exitoso por sí solos no prueban aceptación.

# FIN DEL CONTRATO DE ARQUITECTURA
