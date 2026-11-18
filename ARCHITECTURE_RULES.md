# Contrato de Arquitectura — ENSAMBRIA con Cuatro Backends Modulares
## Topología, ownership y políticas generales del proyecto

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

- [AGENTS.md](AGENTS.md) establece el flujo operativo de consulta: dirigir al agente a este contrato y exigir, antes de planificar o cambiar, la lectura de [docs/requerimiento.md](docs/requerimiento.md), [docs/arquitectura-dominios.md](docs/arquitectura-dominios.md), los ADR y todos los documentos aplicables de [`docs/rules/`](docs/rules/). Todos los `.md` de `docs/rules/` son normativos y obligatorios dentro de su materia.
- Este archivo consolida la topología, las fronteras, las políticas generales específicas del proyecto y la asignación de responsabilidades. Es fuente central para esas áreas; no reemplaza los requisitos del producto, el detalle de dominios ni las reglas especializadas.
- [docs/requerimiento.md](docs/requerimiento.md) define la visión, el alcance funcional, la secuencia de fases y sus criterios de aceptación. [docs/arquitectura-dominios.md](docs/arquitectura-dominios.md) detalla dominios, ownership, contextos y escenarios. El roadmap organiza la secuencia y no sustituye esas fuentes.
- Los documentos de [`docs/rules/`](docs/rules/) mantienen requisitos obligatorios en sus materias especializadas y se aplican conjuntamente con este contrato y el requerimiento. En particular, [las reglas técnicas Rust](docs/rules/ENSAMBRIA_reglas-estrictas-de-arquitectura-monolito-modular.md) detallan la implementación modular Rust y no redefinen topología, ownership de producto, cantidad de workspaces ni límites de backends; para esas decisiones, este contrato es la fuente arquitectónica. Este alcance específico no establece una precedencia global entre documentos.
- Estas funciones describen autoridad por materia y flujo de consulta, no una jerarquía general de prevalencia. Si fuentes aplicables se contradicen y el alcance no puede resolverse respetando sus límites, se documenta el conflicto, se detiene solo el trabajo afectado y se solicita decisión explícita del propietario; después se actualizan coordinadamente las fuentes que corresponda. Ninguna regla obligatoria se omite ni debilita silenciosamente.

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

En PLATFORM, el catálogo de ofertas/planes y precios permanece en `commercial-policies-monetization`; los contextos `saas-subscriptions`, `saas-billing`, `saas-invoices` y `electronic-invoicing` son módulos independientes. El catálogo versionado y el ciclo contractual completo de suscripción se implementan en Fase 1; la consola administrativa global corresponde a Fase 5, y el billing, las facturas SaaS y la transmisión electrónica a Fase 7. La suscripción nace solo de una operación explícita autorizada, nunca del alta técnica del tenant. La factura SaaS y su transmisión fiscal no son el ledger ni el procesamiento del pago: CORE conserva pagos/cargos y ledger. Factus es el proveedor inicial previsto para las integraciones fiscales separadas de PLATFORM y BUSINESS, y Wompi el proveedor inicial previsto para pagos en CORE; ninguno es exclusivo ni está implementado por esta decisión. PLATFORM puede comercializar capacidad Factus sin apropiarse de la emisión/documentos de los establecimientos, cuyo owner sigue siendo BUSINESS. Los límites, contratos, proveedores y fases se detallan en [ADR-0007](docs/adr/ADR-0007-separacion-contextos-facturacion-saas.md), [ADR-0009](docs/adr/ADR-0009-proveedores-iniciales-y-reventa-factus.md) y el [modelo de dominios](docs/arquitectura-dominios.md#23-platform--backend-platform).

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

Los detalles técnicos de BFF/REST/Frontend están en la guía estricta de arquitectura Rust y en las reglas frontend especializadas aplicables. Los cuatro backends pertenecen al workspace raíz, pero no se crea una dependencia de implementación entre sus crates de dominio. Pueden depender de bibliotecas técnicas neutrales aprobadas y de contratos Protobuf versionados; esas dependencias no contienen lógica de dominio. La comunicación de runtime entre backends usa gRPC síncrono, NATS Core para fan-out efímero o eventos durables con Outbox/JetStream según la semántica.

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
| **Pagos y ledger** | CORE ejecuta pagos/reembolsos, aplica comisiones aprobadas y mantiene ledger, liquidaciones y conciliación; Wompi es el proveedor inicial no exclusivo y los adapters no definen política comercial. | Módulos/adapters CORE + cuentas/credenciales aisladas por ámbito + API oficial/sandbox |
| **Facturación SaaS** | PLATFORM mantiene suscripciones y obligaciones/documentos de ENSAMBRIA; CORE procesa pagos asociados. Factus es el proveedor inicial no exclusivo. | Módulo/adapters PLATFORM para Factus; contratos de pago CORE |
| **Facturación comercial tenant** | BUSINESS mantiene pedidos y documentos comerciales de cada establecimiento; CORE procesa los pagos asociados. Factus es el proveedor inicial no exclusivo; PLATFORM puede comercializar esa capacidad sin apropiarse del emisor ni sus documentos. | Módulo/adapters BUSINESS para Factus; contratos de pago CORE |
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

# 1. Reglas técnicas de implementación Rust

Este contrato establece la topología, ownership y políticas generales específicas de ENSAMBRIA. Los requisitos técnicos detallados para implementar sus backends Rust —incluidos SOLID, límites de archivos y capas, DDD, persistencia, Outbox/Inbox, Redis, NATS, BFF, observabilidad, resiliencia, pruebas y rendimiento— son normativos en la [guía estricta de arquitectura del monolito modular Rust](docs/rules/ENSAMBRIA_reglas-estrictas-de-arquitectura-monolito-modular.md). Deben consultarse y aplicarse junto con las demás reglas especializadas de [`docs/rules/`](docs/rules/); esta remisión no vuelve opcionales sus requisitos.

La guía Rust complementa este contrato: no redefine la cantidad de backends, sus límites, ownership de producto, topología de despliegue ni superficies autorizadas. Si una regla técnica y una decisión específica de ENSAMBRIA parecen incompatibles, se identifica el alcance afectado y se resuelve explícitamente antes de implementarlo; ninguna fuente se debilita ni se omite de manera silenciosa.

# 2. Composición específica del workspace y módulos

El workspace Cargo tiene un único manifiesto `[workspace]` en la raíz. La siguiente estructura documenta la composición prevista y el ownership de los módulos; no autoriza crear runtime, crates funcionales ni despliegues durante la Fase 0:

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

El `Cargo.toml` raíz registra paquetes de los cuatro backends, bibliotecas técnicas neutrales, BFF público y BFFs de producto implementados. Cada backend conserva su composición de paquetes/binarios y configuración de despliegue, sin workspaces anidados; Cargo comparte resolución y `Cargo.lock` en la raíz. Los cuatro backends producen despliegues independientes; cada BFF de producto funcional y el BFF público tienen artefacto/proceso y despliegue propios. El BFF de administración permanece inicialmente integrado en PLATFORM. La estructura de `crates/` es ilustrativa: no exige crear un crate por responsabilidad desde el inicio. La Fase 0 audita y documenta esta composición, contratos y dependencias permitidas; la infraestructura común ejecutable se construye y acepta durante la Fase 1.

El BFF público consume contratos gRPC de PUBLIC. ENSAMBRIA Business y Tenant Portal consumen el mismo BFF/API de producto y los mismos contratos empresariales; no duplican reglas de negocio, permisos ni APIs por tener tecnologías distintas. ENSAMBRIA App consume la superficie de producto para operaciones de cliente y puede usar el BFF público para Discovery. Marketplace consume el BFF público; Platform Admin usa el BFF integrado en PLATFORM. Corporate publica contenido institucional estático o mediante un contrato público autorizado, sin acceso directo a servicios de dominio. Cada backend contiene solo módulos, workers, callbacks, proyecciones y realtime de los dominios que posee; cada BFF integrado conserva límites de módulo y contratos, sin acceso directo a stores.

`modules/<bounded-context>/` identifica ownership y ubicación lógica, no exige un paquete Cargo por contexto. La composición puede alojar varios contextos en un paquete de backend si conserva sus límites; solo se crea un crate separado por necesidad demostrable de compilación, pruebas, despliegue o aislamiento. Cada crate adicional debe tener código funcional, owner, dependencias explícitas y pruebas reales; no se crean paquetes, directorios ejecutables ni manifests placeholder en Fase 0.

El módulo dueño del ciclo de vida y autorización de cada archivo depende de un puerto neutral de almacenamiento de objetos; el adapter inicial usa la API compatible con S3 de Cloudflare R2. Las credenciales y configuración del proveedor quedan fuera del dominio. Los contratos no exponen SDKs, tipos ni capacidades exclusivas de R2; el navegador transfiere bytes directamente al almacenamiento mediante autorizaciones temporales emitidas tras validar permisos.

# 3. Autoridad documental para ENSAMBRIA

Este archivo consolida arquitectura, políticas del proyecto y responsabilidades. Mantén aquí las decisiones y restricciones compartidas por esas áreas, sin duplicar el roadmap ni los requisitos detallados de estándares especializados. Cada cambio debe actualizar la sección dueña y mantener consistentes las referencias relacionadas. Un estándar universal reutilizable permanece separado y no reemplaza este contrato específico de ENSAMBRIA.

---

# 4. Regla de prioridad arquitectónica

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

# 5. Principio final

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

# 6. Gobierno del proyecto, documentación y entrega

## 6.1 Fuentes normativas

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

Cada requisito se mantiene en su fuente propietaria; todos los documentos aplicables se consultan y aplican conjuntamente. La guía especializada es normativa en su materia, el requerimiento define el producto y sus fases, el modelo de dominios desarrolla ownership y contextos, y este contrato fija topología y políticas generales de ENSAMBRIA. Ninguna de estas atribuciones crea precedencia global ni permite que una fuente anule otra fuera de su ámbito. Ante una contradicción real que no pueda resolverse por alcance, documenta el conflicto, detén únicamente el trabajo afectado, solicita decisión explícita del propietario y armoniza mediante cambios revisados todas las fuentes afectadas; no debilites reglas ni conviertas propuestas en decisiones aprobadas.

## 6.2 Política de documentación

- No crear archivos o documentos Markdown innecesarios, duplicados, vacíos o sin propósito verificable. Antes de crear uno, comprobar si corresponde actualizar una fuente existente.
- Mantener documentación, contratos y código sincronizados. Marcar con claridad lo implementado, parcial, planificado, bloqueado o retirado; no afirmar funcionalidades sin evidencia.
- Mantener enlaces locales válidos y proteger secretos, credenciales, datos personales y detalles sensibles.
- Revisar los cambios documentales en el PR correspondiente.
- El README raíz presenta ENSAMBRIA, su estado real, arquitectura resumida, requisitos e instrucciones verificadas, y enlaza las guías. No debe acumular contratos, decisiones o detalles que pertenecen a documentos especializados. Para sus criterios específicos, seguir [ENSAMBRIA_README_Documentation_Quality_Security_Standard.md](docs/rules/ENSAMBRIA_README_Documentation_Quality_Security_Standard.md).

## 6.3 Gobierno de GitHub y Pull Requests

GitHub es el plano de control de calidad, seguridad, colaboración, trazabilidad y entrega desde Fase 0. [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md) es la fuente especializada para los controles operativos de repositorio; esta sección conserva las obligaciones de arquitectura y proyecto propias de ENSAMBRIA y debe aplicarse junto con ese estándar.

- `main` debe tener Ruleset/Branch Protection efectivo. Todo cambio, incluidos los documentales, se integra mediante PR; no se permiten push directo, force-push ni eliminación de `main`.
- Fase 0 define y, donde existan permisos, establece CODEOWNERS, plantillas, CI base, scripts reproducibles y seguridad de workflows. Verificar los controles realmente activos; limitaciones de permisos o plataforma se documentan como bloqueos, no como protección cumplida.
- Usar ramas breves y descriptivas por unidad coherente (`feat/`, `fix/`, `refactor/`, `test/`, `docs/`, `security/`, `infra/`). No usar una rama por archivo ni mezclar objetivos sin relación.
- El PR declara objetivo, alcance, requisitos, criterios de aceptación, riesgos y evidencia; revisa diff y secretos, exige resolución de conversaciones y rama actualizada con `main` o merge queue equivalente.
- Con una sola persona mantenedora, permitir PR con auto-revisión documentada sin exigir un segundo CODEOWNER ni aprobación independiente para cambios ordinarios. Mantener obligatorios PR, checks y protección de `main`; la auto-revisión no se presenta como aprobación independiente. Aplicar [§8.1 del estándar GitHub](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md) y reevaluar al incorporar colaboradores.
- Los cambios críticos de seguridad, permisos, autenticación, privacidad, infraestructura o workflows que requieran revisión independiente no se autoaprueban; si no hay revisor autorizado, detener la integración de ese alcance. Nunca simular aprobación humana.
- Aplicar mínimo privilegio a `GITHUB_TOKEN`, fijar Actions externas a SHA completo, establecer límites de tiempo y concurrencia segura. Separar CI de PR, CI de `main`, análisis programados, releases y despliegues. Un PR no despliega a producción.
- Un check requerido fallido, pendiente, cancelado u omitido no es aprobación. Tras integrar, verificar CI del commit exacto de `main`.

## 6.4 Paridad local y CI

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

## 6.5 PR, integración y cierre de fase

Antes de fusionar deben pasar todos los checks requeridos, completarse las revisiones, resolverse las conversaciones y satisfacerse los criterios de aceptación. Cada requisito conserva trazabilidad `requisito → caso → prueba → comando local/CI → ejecución/evidencia`. No se cierra una fase por el número de PRs integrados ni se inicia la siguiente con criterios pendientes. Una dependencia externa que impida protección o validación es un bloqueo explícito, no una aprobación. Los criterios de fase específicos los define el roadmap.

# 7. Responsabilidades y evidencia

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
