# PROMPT MAESTRO — EVOLUCIÓN ARQUITECTÓNICA DE ENSAMBRIA HACIA UNA PLATAFORMA MULTIPRODUCTO SAAS MULTIINDUSTRIA

## 1. OBJETIVO Y AUTORIDAD

Debes analizar e incorporar formalmente en ENSAMBRIA una decisión arquitectónica estratégica:

**ENSAMBRIA debe evolucionar de una plataforma SaaS inicialmente orientada a belleza y bienestar hacia una plataforma tecnológica multiproducto, multindustria y multimarca, capaz de operar múltiples aplicaciones SaaS comerciales independientes sobre una infraestructura tecnológica compartida.**

La intención es que cada industria pueda disponer de un producto comercial propio, con:

- Marca e identidad visual independientes.
- Dominio y subdominios independientes.
- Frontend especializado.
- BFF especializado.
- Navegación y experiencia de usuario propias.
- Funcionalidades y capacidades específicas.
- Planes, precios y configuración comercial independientes.
- SEO, Open Graph, páginas públicas y posicionamiento propios.
- Configuración de despliegue y ciclo de versiones independientes.
- Posibilidad de disponer de aplicaciones móviles propias.
- Aislamiento estricto de datos, tenants, usuarios, configuraciones y secretos.

Desde la perspectiva del cliente, cada producto debe poder percibirse y comercializarse como una aplicación SaaS diferente, sin que sea necesario conocer que comparte tecnología con los demás productos.

Desde la perspectiva de ingeniería, todos deben construirse sobre una plataforma común, con reutilización real, ownership explícito y límites arquitectónicos verificables.

Esta decisión es de carácter estructural y debe quedar incorporada en la documentación normativa del proyecto.

No es una sugerencia, una posibilidad futura opcional ni una ampliación que pueda dejarse para cuando existan más industrias.

## 2. FUENTES DE VERDAD Y PRECEDENCIA

Antes de modificar cualquier documento:

1. Inspecciona el repositorio completo y su estado actual.
2. Lee íntegramente `docs/requerimiento.md`, que es la fuente principal de requisitos de producto, arquitectura y roadmap.
3. Lee `ARCHITECTURE_RULES.md`.
4. Lee `docs/arquitectura-dominios.md`.
5. Lee los ADR existentes.
6. Lee el registro de descubrimiento y arquitectura de Fase 0.
7. Inspecciona los documentos de reglas, agentes y skills que intervienen en el trabajo arquitectónico.
8. Identifica decisiones ya ratificadas y restricciones que no deben reabrirse.

La nueva decisión debe integrarse en la arquitectura vigente, sin crear contradicciones con los requisitos existentes.

`docs/requerimiento.md` continúa siendo la fuente principal de verdad.

Los ADR deben documentar decisiones concretas, sus motivos, consecuencias y alternativas descartadas. No deben sustituir el requerimiento.

Si existe una contradicción, identifícala y resuélvela documentalmente respetando la jerarquía normativa. No ocultes ni elimines requisitos anteriores sin justificación.

## 3. MODELO DE PLATAFORMA MULTIPRODUCTO

La arquitectura debe distinguir claramente cuatro conceptos:

### 3.1. Plataforma tecnológica

Es la infraestructura de software compartida que proporciona:

- Identidad y autenticación.
- Autorización y permisos.
- Tenancy y aislamiento.
- Organizaciones y sucursales.
- Contexto de seguridad.
- Entitlements.
- Reservas y capacidades transversales.
- CRM reutilizable.
- Pagos.
- Ledger.
- Liquidaciones y conciliación.
- Workforce.
- Catálogo e inventario.
- Comercio.
- Membresías y fidelización.
- Facturación.
- Marketing.
- Notificaciones.
- Auditoría.
- Observabilidad.
- Integraciones.
- Infraestructura de eventos.
- Otras capacidades transversales justificadas.

La plataforma no debe depender de una marca, dominio, industria o producto comercial específico.

### 3.2. Producto SaaS

Cada producto es una aplicación comercial independiente que utiliza la plataforma tecnológica.

Ejemplos:

- Producto Beauty.
- Producto Automotive.
- Producto Veterinary.
- Producto Fitness.
- Producto Dentistry.
- Producto Hospitality.
- Otros productos que puedan incorporarse posteriormente.

Cada producto debe tener identidad propia y definir explícitamente:

- Identificador estable del producto.
- Nombre comercial.
- Marca.
- Dominios y dominios alternativos autorizados.
- Industria y vertical.
- Capacidades habilitadas.
- Frontends disponibles.
- BFF correspondiente.
- Configuración de navegación.
- Sistema de diseño.
- Configuración regional y mercados.
- Idiomas y monedas admitidos.
- Planes y funcionalidades comerciales.
- Configuración SEO y Open Graph.
- Integraciones autorizadas.
- Políticas y documentos legales aplicables.
- Configuración de despliegue.
- Estado del producto y ciclo de vida.

No asumir que todos los productos tendrán las mismas características.

### 3.3. Industria y vertical

Una industria representa el ámbito empresarial y sus reglas específicas.

Cada vertical debe definir:

- Lenguaje ubicuo.
- Entidades y agregados propios.
- Reglas de negocio.
- Casos de uso.
- Estados y transiciones.
- Políticas y restricciones.
- Eventos de dominio.
- Contratos.
- Integraciones específicas.
- Requisitos de seguridad.
- Auditoría.
- Pruebas.
- Reglas de composición con capacidades compartidas.

No convertir las diferencias de cada industria en condicionales dispersos por todo el código.

Tampoco crear una entidad universal que intente representar artificialmente todos los negocios.

### 3.4. Marca comercial

La marca es independiente del producto tecnológico y de la infraestructura.

Un producto puede tener su propia marca, dominio, logotipo, favicon, identidad visual, contenido, navegación y posicionamiento comercial.

No acoplar la lógica de negocio a nombres comerciales, dominios, colores, logos o textos de una marca.

## 4. ARQUITECTURA DE FRONTENDS POR PRODUCTO

Cada producto debe poder disponer de su propio frontend.

Ejemplos:

- Beauty Frontend.
- Automotive Frontend.
- Veterinary Frontend.
- Fitness Frontend.
- Dentistry Frontend.

Cada frontend debe disponer de:

- Aplicación y estructura de navegación propias.
- Rutas especializadas.
- Componentes específicos de la industria.
- Experiencia de usuario especializada.
- Diseño visual independiente.
- Accesibilidad.
- Seguridad en frontend.
- Observabilidad.
- Integración con Sentry.
- SEO y Open Graph cuando corresponda.
- Internacionalización.
- Gestión de sesión y contexto de producto.
- Integración exclusiva con los contratos autorizados de su BFF.

La arquitectura debe permitir que cada frontend tenga su propio ciclo de desarrollo, pruebas, compilación y despliegue.

### 4.1. Reutilización de frontend

La reutilización debe realizarse mediante componentes, librerías y contratos compartidos.

Separar:

- Design system común.
- Componentes genéricos.
- Componentes de negocio reutilizables.
- Componentes especializados por industria.
- Temas y tokens de diseño por producto.
- Navegación y composición por producto.

No construir un frontend gigantesco lleno de condicionales de industria.

No obligar a que todas las aplicaciones tengan la misma experiencia visual.

No duplicar innecesariamente componentes que realmente sean transversales.

## 5. ARQUITECTURA DE BFF POR PRODUCTO

Cada producto debe tener un BFF especializado.

Ejemplos:

- Beauty BFF.
- Automotive BFF.
- Veterinary BFF.
- Fitness BFF.
- Dentistry BFF.

Cada BFF debe adaptar las necesidades de su frontend a los contratos de los backends correspondientes.

Debe poder definir:

- Contratos REST/OpenAPI propios.
- DTO especializados.
- Validaciones de entrada.
- Composición de respuestas.
- Autorización contextual.
- Adaptación de errores.
- Políticas de rate limiting.
- Versionado.
- Observabilidad.
- Trazabilidad.
- Pruebas unitarias, integración y aceptación.
- Configuración de despliegue.

### 5.1. Límites obligatorios de los BFF

Un BFF no es propietario de las reglas de negocio de los dominios.

Un BFF no puede:

- Consultar directamente las tablas de otros módulos.
- Acceder a repositorios internos de otros dominios.
- Invocar implementaciones internas de otros módulos.
- Duplicar reglas de negocio.
- Convertirse en un backend monolítico paralelo.
- Evitar autenticación o autorización de los servicios propietarios.
- Introducir dependencias circulares.

Los BFF deben comunicarse mediante contratos explícitos y mecanismos autorizados.

### 5.2. Compatibilidad con los cuatro backends existentes

La incorporación de BFF por producto NO autoriza a destruir, duplicar ni rediseñar arbitrariamente la arquitectura de cuatro backends:

1. CORE.
2. BUSINESS.
3. PLATFORM.
4. PUBLIC.

Se deben conservar sus responsabilidades y ownership.

Los BFF de producto son una capa de adaptación y exposición de contratos, no una excusa para crear un backend de dominio por industria.

Analiza cómo integrar los BFF especializados con la topología vigente.

La solución debe respetar que:

- CORE no se expone directamente a los frontends.
- BUSINESS conserva sus capacidades y verticales empresariales.
- PLATFORM conserva la administración global y la gestión comercial SaaS.
- PUBLIC conserva las responsabilidades de discovery y proyecciones públicas.
- El BFF público existente y los BFF de producto deben tener responsabilidades y contratos claramente definidos.
- Los procesos de BFF pueden desplegarse independientemente cuando corresponda.
- Ningún BFF puede convertirse en propietario accidental de información de otro dominio.

Si es necesario distinguir BFF de producto, BFF público y BFF de administración, documenta sus diferencias y relaciones sin crear duplicaciones funcionales.

## 6. ARQUITECTURA MULTIVERTICAL

La plataforma debe permitir incorporar nuevas industrias sin tener que reconstruir los cuatro backends ni duplicar las capacidades compartidas.

Industrias contempladas como horizonte de reutilización:

1. Beauty & Wellness.
2. Automotive.
3. Veterinary.
4. Fitness & Gyms.
5. Dentistry.
6. Healthcare & Clinics.
7. Physiotherapy & Rehabilitation.
8. Alternative Wellness.
9. Hotels & Accommodation.
10. Restaurants & Gastronomy.
11. Cleaning Services.
12. Technical Repairs.
13. Real Estate.
14. Pet Services.
15. Fashion & Apparel.
16. Photography & Events.
17. Education & Academies.
18. Coworking & Shared Spaces.
19. Construction & Maintenance.
20. Tourism & Experiences.

Esta lista representa oportunidades de reutilización y expansión arquitectónica, no una autorización para implementar inmediatamente los veinte verticales.

### 6.1. Implementación inicial

Beauty & Wellness continúa siendo el único vertical funcional inicial de ENSAMBRIA.

Los demás verticales deben quedar contemplados en el diseño, ownership, contratos de extensión y documentación de arquitectura, pero no deben recibir implementaciones ficticias.

No crear carpetas vacías, módulos vacíos, APIs falsas, contratos sin semántica, entidades genéricas artificiales ni implementaciones placeholder para aparentar que las industrias están soportadas.

Cada nueva industria necesitará su propio descubrimiento de dominio, diseño, implementación y pruebas antes de habilitarse comercialmente.

### 6.2. Capacidades transversales

Analiza y formaliza la separación entre capacidades reutilizables y verticales.

Capacidades candidatas:

- CRM.
- Workforce.
- Catalog.
- Inventory.
- Commerce.
- Memberships & Loyalty.
- Invoicing.
- Marketing.
- Scheduling.
- Payments.
- Notifications.

No asumir que todas las capacidades son necesarias en todos los productos.

Una capacidad solo será compartida cuando sus reglas, ownership y contratos sean realmente transversales.

Cuando una industria necesite reglas propias, deben permanecer en su vertical.

## 7. COMPOSICIÓN DE PRODUCTOS

Diseña un mecanismo explícito para componer productos SaaS mediante:

- Plataforma común.
- Capacidades compartidas.
- Vertical especializado.
- Frontend.
- BFF.
- Marca.
- Configuración comercial.
- Integraciones.
- Mercados habilitados.

Cada producto debe declarar qué capacidades utiliza y qué versión de sus contratos consume.

La composición debe ser explícita, validable y reproducible.

No utilizar flags arbitrarios dispersos por el código como sustituto de una arquitectura de composición.

Distinguir claramente:

- Capacidad disponible en la plataforma.
- Capacidad habilitada para un producto.
- Capacidad contratada por un tenant.
- Funcionalidad autorizada para un usuario.
- Funcionalidad realmente implementada y operativa.

Los entitlements deben seguir teniendo un ownership centralizado y respetar las reglas de CORE y PLATFORM definidas en el requerimiento.

No permitir que un cambio de configuración comercial habilite una funcionalidad que todavía no está implementada o que no cumple sus requisitos de seguridad.

## 8. DOMINIOS, MARCAS Y MULTITENANCY

Cada producto puede utilizar:

- Un dominio independiente.
- Un conjunto de subdominios.
- Dominios personalizados de clientes.
- Dominios para portales públicos.
- Dominios para aplicaciones de administración.

Diseña una resolución segura del contexto:

`domain → product → tenant → organization → branch → user context`

Esta secuencia es conceptual: define los mecanismos y validaciones exactos sin asumir que todos los elementos deben resolverse en una sola consulta.

Requisitos obligatorios:

- No confiar en el dominio enviado por el navegador como prueba de autorización.
- Validar dominios y asociaciones en backend.
- Impedir suplantación de producto o tenant mediante headers.
- Mantener autorización independiente del enrutamiento.
- Validar que el tenant pertenece al producto.
- Mantener aislamiento de datos.
- Aplicar RLS donde corresponda.
- No permitir acceso cruzado entre productos.
- No compartir secretos entre productos por conveniencia.
- No filtrar datos de un producto en otro mediante cachés.
- Incorporar el contexto correcto en logs, métricas, trazas y auditoría.
- Definir invalidación y aislamiento de caché.
- Mantener trazabilidad de solicitudes entre BFF y backends.

Un tenant no debe acceder a información de otro producto solo porque ambos utilizan la misma infraestructura física.

## 9. PERSISTENCIA Y WORKSPACE RUST

Conservar las decisiones de arquitectura ya establecidas:

- Un único Cargo workspace en la raíz.
- Los cuatro backends como unidades de ejecución independientes.
- Ownership de datos separado por dominio y backend.
- PostgreSQL física compartida conforme al modelo aprobado.
- Un pool compartido por proceso/backend, no un pool arbitrario por módulo.
- RLS en las tablas que requieren aislamiento por tenant.
- Contexto de seguridad transaccional.
- UUID v7 como identificador canónico cuando así está establecido.
- Redis para datos efímeros con TTL.
- Outbox/Inbox y JetStream para hechos durables.
- NATS Core para fan-out efímero cuando corresponda.
- gRPC y Protobuf para contratos internos autorizados.

Los BFF independientes que sean procesos separados tendrán su propio ciclo de vida y recursos de conexión. No deben crear pools por módulo interno.

No crear una base de datos independiente por industria como requisito obligatorio.

Tampoco permitir que la base física compartida se convierta en una base de datos sin límites de ownership.

Documentar las reglas de separación, migraciones, permisos, schemas y acceso a datos.

## 10. INDEPENDENCIA COMERCIAL Y OPERATIVA

Cada producto debe poder comercializarse como un SaaS distinto.

Diseñar las reglas para que cada producto pueda definir:

- Planes y precios.
- Límites y cuotas.
- Capacidades contratables.
- Monedas.
- Mercados.
- Idiomas.
- Ciclos de facturación.
- Políticas comerciales.
- Promociones.
- Contratos.
- Documentos legales.
- Condiciones de servicio.
- Identidad comercial.
- Canales de adquisición.
- Configuración de soporte.

PLATFORM conserva la administración comercial central, pero debe poder administrar diferentes catálogos y configuraciones de productos.

No confundir la facturación del SaaS al tenant con los cobros que realiza un tenant a sus clientes finales.

No compartir automáticamente contratos, suscripciones, pagos, usuarios o datos comerciales entre productos.

## 11. INDEPENDENCIA DE DESPLIEGUE

La arquitectura debe permitir desplegar y actualizar un producto sin obligar a desplegar todos los demás.

Analizar y documentar:

- Unidades de compilación.
- Unidades de ejecución.
- Artefactos.
- Dependencias.
- Configuración.
- Secretos.
- Migraciones.
- Compatibilidad de contratos.
- Versionado.
- Rollback.
- Health checks.
- Observabilidad.
- Estrategias de release.
- Compatibilidad entre versiones de frontend, BFF y backend.

Un cambio exclusivo de Automotive no debería exigir reconstruir o desplegar Beauty si no existe una dependencia real.

La infraestructura compartida debe mantener compatibilidad hacia atrás y políticas de evolución de contratos.

No asumir que independencia de despliegue significa duplicar todos los recursos o bases de datos.

## 12. SEGURIDAD Y OBSERVABILIDAD MULTIPRODUCTO

Diseñar seguridad desde la plataforma, no como una responsabilidad opcional de cada frontend.

Definir:

- Aislamiento de tenant y producto.
- Autenticación y autorización.
- Contexto de seguridad.
- Protección de sesiones.
- Gestión de secretos.
- CORS.
- CSRF cuando aplique.
- Rate limiting.
- Validación de entradas.
- Protección de APIs.
- Auditoría.
- Trazabilidad.
- Políticas de retención.
- Gestión de errores.
- Redacción de información sensible.

La observabilidad debe permitir distinguir:

- Producto.
- Vertical.
- Backend.
- BFF.
- Frontend.
- Tenant.
- Organización.
- Sucursal.
- Operación.
- Correlation ID.

Sin exponer información sensible en métricas, trazas o logs.

Cada producto debe poder tener sus propias vistas de observabilidad sin perder la capacidad de observación central de la plataforma.

## 13. FRONTENDS, SEO Y EXPERIENCIA INDEPENDIENTE

Cada producto debe tener identidad pública independiente.

Incorporar al diseño arquitectónico:

- SEO técnico por producto.
- Metadatos y Open Graph.
- Canonical URLs.
- Sitemap.
- Robots.
- Favicons.
- Manifest.
- Branding.
- Páginas legales.
- Páginas de error.
- Localización.
- Experiencias responsive.
- Accesibilidad.
- Analítica.
- Sentry.
- Consentimiento y preferencias de privacidad.

No permitir que el SEO, los metadatos o el branding de ENSAMBRIA se filtren automáticamente a los demás productos.

No imponer que todos los productos compartan un dominio o una identidad visual.

## 14. DOCUMENTACIÓN Y DECISIONES OBLIGATORIAS

Integra esta arquitectura en las fuentes normativas existentes.

Debes evaluar y actualizar, cuando corresponda:

- `docs/requerimiento.md`.
- `ARCHITECTURE_RULES.md`.
- `docs/arquitectura-dominios.md`.
- Registro de descubrimiento y arquitectura de Fase 0.
- Matrices de ownership.
- Matriz de productos, capacidades y verticales.
- Matriz de frontends y BFF.
- Matriz de dependencias.
- Matriz de contratos.
- Matriz de fases.
- Matriz de aceptación.
- ADR existentes o nuevos ADR justificados.
- Reglas de agentes y skills que necesiten reflejar la decisión.

Documentar como mínimo:

1. Definición de plataforma multiproducto.
2. Separación producto, marca, industria y vertical.
3. Ownership de cada capacidad.
4. BFF por producto.
5. Frontend por producto.
6. Integración con los cuatro backends.
7. Composición de capacidades.
8. Contexto producto/tenant.
9. Dominios y resolución segura.
10. Aislamiento de datos.
11. Seguridad.
12. Observabilidad.
13. Independencia de despliegue.
14. Versionado de contratos.
15. Incorporación de nuevas industrias.
16. Reglas para evitar duplicación de lógica.
17. Criterios de habilitación comercial.
18. Evolución futura de productos.

No crear documentos duplicados que compitan con las fuentes de verdad.

No crear ADR por cada detalle trivial. Crear ADR cuando exista una decisión arquitectónica relevante que requiera registro formal.

## 15. ROADMAP Y FASES

La nueva arquitectura no debe desordenar el roadmap vigente.

Revisa las fases para asegurar que la arquitectura multiproducto quede preparada desde Fase 0, mientras que la implementación funcional se realiza en las fases correspondientes.

Fase 0:

- Descubrimiento multiproducto.
- Modelo de productos.
- Matriz de industrias y verticales.
- Ownership.
- Composición.
- Límites arquitectónicos.
- BFF y frontend por producto.
- Dominios.
- Seguridad.
- Aislamiento.
- Contratos.
- Estrategia de despliegue.
- Criterios de aceptación.

Fase 1:

- Implementación de la infraestructura base y mecanismos comunes necesarios según el requerimiento vigente.
- No implementar automáticamente los veinte productos.

Fases posteriores:

- Implementar las capacidades y verticales de acuerdo con su ownership y prioridades.
- Mantener Beauty & Wellness como vertical funcional inicial.
- Incorporar otras industrias únicamente cuando exista alcance aprobado y una fase de implementación definida.

No mover funcionalidades entre fases arbitrariamente.

No marcar como implementado algo que únicamente está documentado.

No adelantar código runtime a Fase 0.

## 16. REGLAS CONTRA SOBREDISEÑO Y FALSAS IMPLEMENTACIONES

Quedan prohibidos:

- Crear veinte aplicaciones ficticias.
- Crear veinte backends de dominio.
- Duplicar CORE por industria.
- Duplicar CRM por producto sin justificación.
- Crear un modelo universal de negocio artificial.
- Acoplar dominios a marcas.
- Acoplar lógica de negocio a dominios web.
- Condicionales de industria dispersos por todo el código.
- Frontends gigantes con múltiples productos mezclados.
- BFF con lógica de dominio duplicada.
- Acceso directo a bases de datos ajenas.
- Módulos vacíos que aparenten funcionalidad.
- APIs placeholder.
- Contratos sin definición semántica.
- Mocks o fakes para aparentar integraciones.
- TODOs que sustituyan implementación.
- Flags de configuración que permitan saltarse seguridad.
- Cambios de arquitectura sin actualizar documentación y pruebas.

La extensibilidad debe ser real, no una abstracción anticipada sin justificación.

## 17. PROCEDIMIENTO DE TRABAJO DEL AGENTE

Trabajas en VS Code utilizando GitHub Copilot como entorno de ejecución.

Antes de actuar:

1. Identifica el estado real del repositorio.
2. Comprueba la rama actual.
3. Verifica el estado de Git y los cambios existentes.
4. Lee las instrucciones aplicables.
5. Identifica agentes y skills realmente disponibles en la sesión.
6. Comprueba las fuentes normativas.
7. Construye una matriz de contradicciones y documentos afectados.

No afirmes que has delegado trabajo si no has invocado realmente a otro agente y no tienes evidencia de su resultado.

No confundas tener perfiles de agentes registrados con haberlos ejecutado.

No inventes herramientas, capacidades ni resultados de GitHub Copilot.

Si una tarea requiere delegación, utiliza únicamente los mecanismos de delegación realmente disponibles en la sesión.

### Restricciones Git

- No trabajar directamente sobre `main`.
- No hacer push directo a `main`.
- No sobrescribir cambios del usuario.
- No ejecutar operaciones destructivas.
- No utilizar `git add .` indiscriminadamente.
- No eliminar archivos sin justificación.
- No modificar scripts de CI salvo que el alcance lo requiera expresamente.
- Mantener los cambios organizados y verificables.
- Utilizar ramas y PR según las reglas vigentes del proyecto.

Si no existe autorización para modificar el repositorio, presenta el diagnóstico y el plan, pero no realices cambios.

## 18. CRITERIOS DE ACEPTACIÓN

El trabajo no se considera terminado hasta demostrar:

### Arquitectura

- [ ] ENSAMBRIA está definido normativamente como plataforma multiproducto.
- [ ] Se distingue producto, marca, industria, vertical y capacidad.
- [ ] Los cuatro backends conservan sus responsabilidades.
- [ ] Se define formalmente el modelo de BFF por producto.
- [ ] Se define formalmente el frontend por producto.
- [ ] Los límites de dominio y contratos están documentados.
- [ ] No existe duplicación injustificada de capacidades.
- [ ] Se contempla independencia de despliegue.

### Multiverticalidad

- [ ] Beauty & Wellness permanece como vertical inicial.
- [ ] Las demás industrias están contempladas sin implementaciones ficticias.
- [ ] Cada vertical tiene criterios de incorporación.
- [ ] Las capacidades transversales tienen ownership.
- [ ] Las reglas específicas permanecen en su vertical.

### Multi-producto

- [ ] Cada producto puede tener marca propia.
- [ ] Cada producto puede tener dominios propios.
- [ ] Cada producto puede tener frontend propio.
- [ ] Cada producto puede tener BFF propio.
- [ ] Cada producto puede tener planes y capacidades propios.
- [ ] Cada producto puede desplegarse independientemente cuando corresponda.
- [ ] No existe contaminación de datos o configuración entre productos.

### Seguridad

- [ ] El contexto de producto y tenant está definido.
- [ ] La resolución de dominios es segura.
- [ ] Los BFF no evitan autorización.
- [ ] RLS y ownership siguen vigentes.
- [ ] Cachés, secretos, logs y trazas respetan el aislamiento.
- [ ] No existen accesos cruzados injustificados.

### Documentación

- [ ] Las fuentes normativas están actualizadas.
- [ ] Los ADR están alineados.
- [ ] Las matrices están completas.
- [ ] El roadmap conserva su secuencia.
- [ ] No existen contradicciones entre documentos.
- [ ] Las decisiones pendientes están identificadas con su responsable y alcance.

### Evidencia

- [ ] Cada requisito tiene documento, decisión y criterio de aceptación.
- [ ] Las verificaciones ejecutadas tienen evidencia reproducible.
- [ ] Se distingue lo documentado de lo implementado.
- [ ] No se declaran funcionalidades implementadas sin pruebas.
- [ ] No se declara cerrada una fase por haber actualizado documentos.

## 19. RESULTADO ESPERADO

Al terminar, presenta un informe que incluya:

1. Estado arquitectónico encontrado.
2. Decisiones ya existentes que se conservaron.
3. Contradicciones encontradas.
4. Decisiones nuevas incorporadas.
5. Documentos modificados y motivo de cada modificación.
6. Matriz final de productos, industrias, verticales y capacidades.
7. Modelo final de frontend y BFF por producto.
8. Relación con los cuatro backends.
9. Estrategia de aislamiento y seguridad.
10. Estrategia de despliegue.
11. Cambios del roadmap.
12. Validaciones ejecutadas y resultados reales.
13. Criterios de aceptación cumplidos.
14. Criterios pendientes y sus bloqueos reales.
15. Riesgos y decisiones que requieren aprobación.

No declares éxito total si existen criterios sin cumplir.

No confundas una arquitectura documentada con una plataforma ya implementada.

---

# DIRECTIVA FINAL

La visión que debe guiar todas las decisiones es la siguiente:

**UNA PLATAFORMA TECNOLÓGICA. UN MONOREPO. CUATRO BACKENDS CENTRALES. MÚLTIPLES PRODUCTOS SAAS. MÚLTIPLES INDUSTRIAS. MÚLTIPLES MARCAS. UN FRONTEND Y UN BFF ESPECIALIZADOS POR PRODUCTO. CAPACIDADES COMPARTIDAS CON OWNERSHIP EXPLÍCITO. DOMINIOS Y EXPERIENCIAS COMERCIALES INDEPENDIENTES. AISLAMIENTO ESTRICTO DE DATOS Y SEGURIDAD.**

ENSAMBRIA no debe convertirse en una aplicación gigantesca que intenta atender todas las industrias mediante condicionales.

Debe convertirse en una plataforma desde la que se puedan construir, operar, evolucionar y comercializar aplicaciones SaaS independientes, especializadas y escalables.

La reutilización tecnológica debe reducir duplicación sin sacrificar independencia comercial, especialización de dominio, seguridad ni calidad.


## 20. ARQUITECTURA FÍSICA DE CARPETAS Y ORGANIZACIÓN DEL MONOREPO

### 20.1. Objetivo obligatorio

Además de definir la arquitectura lógica y funcional de ENSAMBRIA, debes definir formalmente su arquitectura física de carpetas.

La organización del repositorio debe reflejar los límites de dominio, ownership, unidades de ejecución, contratos, productos, capacidades y verticales.

La estructura debe permitir:

- Mantener un único monorepo.
- Mantener un único Cargo workspace en la raíz.
- Mantener exclusivamente cuatro backends centrales: CORE, BUSINESS, PLATFORM y PUBLIC.
- Mantener módulos de dominio separados por ownership.
- Incorporar nuevos productos sin duplicar los backends.
- Incorporar nuevas industrias sin crear backends por industria.
- Mantener un frontend especializado por producto.
- Mantener un BFF especializado por producto.
- Reutilizar librerías y componentes sin mezclar responsabilidades.
- Compilar, probar y desplegar unidades independientes cuando corresponda.
- Aplicar límites arquitectónicos verificables mediante dependencias y pruebas.

La estructura de carpetas no debe convertirse en una simple clasificación estética. Cada directorio debe tener una responsabilidad concreta y verificable.

### 20.2. Estructura objetivo del repositorio

La siguiente estructura representa la arquitectura física objetivo de ENSAMBRIA. Debe validarse contra el estado real del repositorio antes de aprobar cualquier reorganización.

```text
ENSAMBRIA/
│
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── rustfmt.toml
├── clippy.toml
│
├── .cargo/
├── .github/
│   ├── agents/
│   ├── workflows/
│   └── instructions/
│
├── apps/
│   │
│   ├── backends/
│   │   ├── core/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── bootstrap.rs
│   │   │       └── transport/
│   │   │
│   │   ├── business/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── bootstrap.rs
│   │   │       └── transport/
│   │   │
│   │   ├── platform/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── bootstrap.rs
│   │   │       └── transport/
│   │   │
│   │   └── public/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── main.rs
│   │           ├── bootstrap.rs
│   │           └── transport/
│   │
│   ├── bffs/
│   │   └── products/
│   │       └── beauty/
│   │           ├── Cargo.toml
│   │           └── src/
│   │               ├── main.rs
│   │               ├── bootstrap.rs
│   │               ├── api/
│   │               ├── routes/
│   │               ├── dto/
│   │               ├── clients/
│   │               ├── composition/
│   │               ├── middleware/
│   │               └── observability/
│   │
│   └── frontends/
│       │
│       ├── products/
│       │   └── beauty/
│       │       ├── src/
│       │       │   ├── app/
│       │       │   ├── routes/
│       │       │   ├── features/
│       │       │   ├── components/
│       │       │   ├── layouts/
│       │       │   ├── theme/
│       │       │   ├── branding/
│       │       │   └── localization/
│       │       ├── public/
│       │       └── tests/
│       │
│       ├── discovery/
│       └── platform-admin/
│
├── crates/
│   │
│   ├── core/
│   │   ├── identity/
│   │   ├── authentication/
│   │   ├── authorization/
│   │   ├── tenancy/
│   │   ├── organizations/
│   │   ├── entitlements/
│   │   ├── reservations/
│   │   ├── payments/
│   │   ├── ledger/
│   │   └── settlements/
│   │
│   ├── business/
│   │   ├── crm/
│   │   ├── workforce/
│   │   ├── scheduling/
│   │   ├── services/
│   │   ├── catalog/
│   │   ├── inventory/
│   │   ├── commerce/
│   │   ├── memberships/
│   │   └── verticals/
│   │       └── beauty_wellness/
│   │
│   ├── platform/
│   │   ├── product_registry/
│   │   ├── plans/
│   │   ├── subscriptions/
│   │   ├── saas_billing/
│   │   ├── commercial_policies/
│   │   └── product_configuration/
│   │
│   ├── public/
│   │   ├── discovery/
│   │   ├── directory/
│   │   ├── public_profiles/
│   │   └── projections/
│   │
│   ├── shared/
│   │   ├── identifiers/
│   │   ├── errors/
│   │   ├── observability/
│   │   ├── telemetry/
│   │   └── primitives/
│   │
│   └── infrastructure/
│       ├── postgres/
│       ├── redis/
│       ├── messaging/
│       ├── grpc/
│       ├── storage/
│       └── security/
│
├── contracts/
│   ├── protobuf/
│   │   ├── core/
│   │   ├── business/
│   │   ├── platform/
│   │   └── public/
│   │
│   ├── openapi/
│   │   └── products/
│   │       └── beauty/
│   │
│   └── schemas/
│
├── packages/
│   └── frontend/
│       ├── design-system/
│       ├── ui/
│       ├── accessibility/
│       ├── api-clients/
│       ├── localization/
│       └── tooling/
│
├── database/
│   └── migrations/
│       ├── core/
│       ├── business/
│       ├── platform/
│       └── public/
│
├── config/
│   ├── products/
│   │   └── beauty/
│   ├── environments/
│   └── observability/
│
├── deploy/
│   ├── backends/
│   │   ├── core/
│   │   ├── business/
│   │   ├── platform/
│   │   └── public/
│   │
│   ├── bffs/
│   │   └── products/
│   │       └── beauty/
│   │
│   └── frontends/
│       └── products/
│           └── beauty/
│
├── tests/
│   ├── architecture/
│   ├── contracts/
│   ├── integration/
│   ├── acceptance/
│   ├── e2e/
│   ├── security/
│   └── isolation/
│
├── docs/
│   ├── architecture/
│   ├── adr/
│   ├── products/
│   ├── domains/
│   ├── verticals/
│   ├── contracts/
│   ├── security/
│   ├── deployment/
│   ├── testing/
│   └── roadmap/
│
└── tools/
```

Esta estructura es una propuesta arquitectónica objetivo, no una orden de crear inmediatamente todos los directorios enumerados. Los módulos y productos deben existir físicamente cuando tengan responsabilidad, alcance e implementación aprobados.

### 20.3. Responsabilidad de `apps/`

`apps/` contiene las aplicaciones ejecutables y las unidades de presentación.

No debe utilizarse para almacenar indiscriminadamente todos los módulos internos del negocio.

#### `apps/backends/`

Contiene exactamente los cuatro backends ejecutables de ENSAMBRIA:

- `core/`
- `business/`
- `platform/`
- `public/`

Cada uno debe tener su propio manifiesto de aplicación y composición de dependencias.

Su función es actuar como punto de entrada, inicialización y composición de los módulos que pertenecen a su backend.

Los cuatro backends son unidades independientes de ejecución y despliegue. No deben duplicarse por producto o industria.

#### `apps/bffs/`

Contiene los BFF independientes por producto cuando corresponda.

Cada BFF debe adaptar los contratos de los backends a las necesidades concretas de su frontend.

Los BFF no pueden contener agregados, reglas de dominio ni persistencia que pertenecen a los módulos propietarios.

La incorporación de un BFF por producto no autoriza a crear otro backend de dominio.

Los BFF público, de administración o integrados en los backends existentes deben conservar las responsabilidades ya ratificadas. Antes de crear una nueva unidad, debes determinar si representa una aplicación realmente independiente o si duplicaría una responsabilidad existente.

#### `apps/frontends/`

Contiene las aplicaciones de interfaz.

Cada producto debe tener su propia aplicación y ciclo de compilación.

No se debe construir una aplicación universal que mezcle todas las industrias mediante condicionales.

Discovery y Platform Admin deben conservar sus responsabilidades diferenciadas respecto a las experiencias comerciales de cada producto.

### 20.4. Responsabilidad de `crates/`

`crates/` contiene los paquetes Rust reutilizables, módulos de dominio y capacidades internas.

Los paquetes deben organizarse por ownership, no solamente por tipo técnico.

La distribución entre `core/`, `business/`, `platform/` y `public/` debe corresponder con la matriz de ownership normativa.

Los ejemplos del árbol anterior no autorizan a cambiar responsabilidades ya ratificadas. Si un módulo existente tiene otro propietario normativo, prevalece el requerimiento y el ADR correspondiente.

Cada módulo de dominio que justifique aislamiento como paquete Rust debe mantener sus propias capas:

```text
nombre-del-modulo/
├── Cargo.toml
└── src/
    ├── domain/
    │   ├── aggregates/
    │   ├── entities/
    │   ├── value_objects/
    │   ├── events/
    │   ├── policies/
    │   └── services/
    │
    ├── application/
    │   ├── commands/
    │   ├── queries/
    │   ├── use_cases/
    │   └── handlers/
    │
    ├── ports/
    │   ├── inbound/
    │   └── outbound/
    │
    └── adapters/
        ├── persistence/
        ├── messaging/
        └── integrations/
```

Esta separación debe aplicarse cuando corresponda al tamaño y responsabilidad del módulo. No debes crear capas, paquetes ni abstracciones vacías para aparentar cumplimiento.

Reglas:

- `domain/` no depende de infraestructura.
- `application/` coordina casos de uso y políticas, sin apropiarse de adaptadores.
- `ports/` define los contratos necesarios.
- `adapters/` implementa las integraciones autorizadas.
- Un módulo no puede importar implementaciones internas de otro módulo.
- La comunicación entre módulos y backends debe respetar sus contratos autorizados.
- No crear un paquete por cada archivo o estructura trivial.
- No compartir entidades de dominio entre productos o backends mediante un supuesto modelo universal.

### 20.5. Separación por ownership de los cuatro backends

La estructura física debe reflejar estos límites:

**CORE**

Contiene los módulos cuya autoridad corresponde al núcleo de la plataforma: identidad, autenticación, autorización central, tenancy, contexto de seguridad, organizaciones y demás capacidades centrales ratificadas.

**BUSINESS**

Contiene CRM, operaciones empresariales, workforce, servicios, agenda y capacidades verticales según el ownership vigente. Beauty & Wellness debe mantenerse como primer vertical funcional.

**PLATFORM**

Contiene administración global, catálogo de productos SaaS, planes, suscripciones, facturación SaaS, políticas comerciales y configuración central de productos.

**PUBLIC**

Contiene discovery, directorios, perfiles públicos autorizados y proyecciones públicas.

Ninguna industria debe recibir un backend propio.

### 20.6. BFF especializado y frontend independiente

La estructura debe reflejar una relación explícita entre producto, frontend y BFF.

Ejemplo inicial:

```text
apps/
├── bffs/
│   └── products/
│       └── beauty/
│
└── frontends/
    └── products/
        └── beauty/
```

Ambas aplicaciones representan el mismo producto comercial, pero tienen responsabilidades distintas.

Cuando un nuevo producto sea aprobado, su frontend y BFF deben poder incorporarse sin modificar innecesariamente las aplicaciones de otros productos ni duplicar los cuatro backends.

La estructura no debe obligar a crear veinte productos ni carpetas vacías para industrias todavía no implementadas.

### 20.7. Capacidades y verticales dentro de BUSINESS

Las capacidades compartidas deben organizarse en módulos con ownership explícito.

Los verticales deben agruparse bajo `crates/business/verticals/`.

Ejemplo:

```text
crates/business/
├── crm/
├── workforce/
├── scheduling/
├── services/
└── verticals/
    └── beauty_wellness/
```

Beauty & Wellness será el primer vertical funcional.

Cuando una nueva industria sea aprobada, su implementación deberá incorporarse bajo su propio directorio de vertical, con dominio, casos de uso, políticas, eventos, contratos y pruebas propios.

No crear carpetas para Automotive, Veterinary, Fitness o cualquier otro vertical hasta que exista un alcance real de implementación.

Las capacidades compartidas no deben contener reglas específicas de una industria.

### 20.8. Contratos y generación de código

`contracts/` será la ubicación de los contratos fuente compartidos y versionados.

- Protobuf y gRPC para los contratos internos autorizados.
- OpenAPI para los contratos HTTP expuestos por los BFF.
- Schemas para los formatos adicionales que estén justificados.

El código generado debe distinguirse de los contratos fuente.

No copiar manualmente el mismo DTO en diferentes backends, BFF y frontends cuando pueda generarse desde un contrato autorizado.

No permitir que los contratos HTTP de un BFF se conviertan en contratos internos de dominio por accidente.

### 20.9. Persistencia y migraciones

`database/migrations/` debe reflejar el ownership de cada backend y módulo.

```text
database/migrations/
├── core/
├── business/
├── platform/
└── public/
```

Cada migración debe tener un propietario identificable.

La estructura debe permitir rastrear qué módulo es responsable de cada tabla, índice, restricción, política RLS y cambio de esquema.

La existencia de una PostgreSQL física compartida no autoriza consultas cruzadas ni elimina la separación lógica.

No crear una carpeta de migraciones por producto cuando el esquema pertenece a un módulo central compartido.

### 20.10. Reutilización de frontend

`packages/frontend/` debe contener exclusivamente paquetes de presentación y herramientas de frontend reutilizables.

Por ejemplo:

- Design system.
- Componentes UI genéricos.
- Accesibilidad.
- Clientes generados desde contratos.
- Localización.
- Herramientas compartidas.

No debe contener reglas de negocio autoritativas, políticas comerciales ni lógica de dominio que pertenezca a un backend.

Cada producto debe poder consumir y personalizar los paquetes compartidos sin perder su identidad visual.

Conservar el framework, gestor de paquetes y convenciones del frontend que estén ratificados en el repositorio. No introducir tecnologías nuevas solamente para adaptarse a este árbol.

### 20.11. Configuración de productos

`config/products/` contendrá únicamente configuración versionable que corresponda mantener como artefacto de configuración.

La configuración debe distinguir:

- Identidad del producto.
- Marca.
- Dominios autorizados.
- Capacidades declaradas.
- Mercados habilitados.
- Parámetros técnicos.
- Configuración de compilación y despliegue.

Los secretos, contraseñas, tokens y credenciales nunca deben almacenarse en estos directorios.

La configuración versionada no sustituye el ownership comercial y la administración dinámica que corresponden a PLATFORM.

### 20.12. Despliegue independiente

`deploy/` debe organizar los artefactos y configuraciones de despliegue por unidad ejecutable.

Los cuatro backends deben poder desplegarse independientemente.

Cada BFF y frontend de producto debe disponer de su configuración de compilación y despliegue independiente cuando su estrategia operativa lo requiera.

No agrupar obligatoriamente todos los productos en un solo artefacto de despliegue.

No duplicar configuraciones compartidas sin justificación.

### 20.13. Un único Cargo workspace

Debe existir un único `Cargo.toml` de workspace en la raíz del monorepo.

Los paquetes Rust ejecutables y reutilizables se incorporarán a ese workspace.

Los backends y BFF implementados en Rust no deben crear workspaces independientes por producto, backend o módulo.

Los frontends pueden mantener su propio sistema de workspace si el stack existente lo requiere, sin crear un segundo Cargo workspace.

El agente debe verificar que:

- No existan workspaces Rust anidados accidentalmente.
- Las dependencias entre paquetes respeten ownership.
- Las aplicaciones ejecutables no importen implementaciones internas de módulos ajenos.
- Las dependencias compartidas no se conviertan en un mecanismo para evadir los límites arquitectónicos.
- Los paquetes y aplicaciones puedan compilarse y probarse de manera reproducible.

### 20.14. Reglas de dependencias

La dirección de dependencias debe respetar DDD y Clean Architecture.

No se permitirá:

- Dependencias circulares entre módulos.
- Dependencia de dominio hacia infraestructura.
- Acceso directo desde un BFF a repositorios de dominio.
- Acceso directo de un backend al esquema privado de otro backend.
- Importaciones de código de un vertical desde otro vertical sin contrato y justificación.
- Dependencias de dominio hacia frontends.
- Acoplamiento de paquetes compartidos a una marca concreta.
- Acoplamiento de módulos de dominio a un producto comercial.

Las restricciones deben verificarse mediante mecanismos automatizados cuando sea técnicamente posible, además de la revisión de código.

### 20.15. Reglas contra la fragmentación excesiva

No conviertas cada carpeta en un crate de Rust automáticamente.

No conviertas cada caso de uso en un paquete independiente.

No crees un paquete compartido solo porque dos módulos utilizan una estructura con el mismo nombre.

La separación debe responder a ownership, límites de dominio, dependencias, ciclo de vida y necesidad real de compilación independiente.

El objetivo no es maximizar la cantidad de paquetes. El objetivo es conseguir límites arquitectónicos claros, comprobables y sostenibles.

### 20.16. Procedimiento obligatorio de análisis de la estructura actual

Antes de proponer cambios físicos:

1. Inspecciona el árbol completo del repositorio.
2. Identifica todos los Cargo.toml existentes.
3. Identifica los workspaces actuales.
4. Identifica los cuatro backends y sus unidades ejecutables.
5. Identifica los módulos, paquetes y librerías existentes.
6. Identifica frontends y BFF existentes.
7. Identifica contratos, migraciones, configuración, despliegue y pruebas.
8. Identifica código duplicado y dependencias que atraviesan límites.
9. Compara la estructura actual con la estructura objetivo.
10. Construye una matriz de correspondencia entre carpetas actuales y carpetas objetivo.

No renombres, muevas ni elimines archivos sin conocer sus dependencias y consecuencias.

Si una decisión de ownership todavía está pendiente, debes identificarla en lugar de inventar una ubicación definitiva.

### 20.17. Restricción de Fase 0

Si el trabajo se ejecuta dentro de Fase 0, el resultado requerido es una arquitectura física objetivo documentada y aprobable.

En esta fase:

- No reorganices código productivo.
- No muevas aplicaciones.
- No crees paquetes Rust ejecutables.
- No crees carpetas vacías para industrias futuras.
- No introduzcas migraciones.
- No implementes módulos runtime.
- No alteres compilación o despliegue.
- No declares que el monorepo ya adoptó físicamente la nueva estructura.

Debes documentar el árbol objetivo, la responsabilidad de cada directorio, las dependencias permitidas, la correspondencia con el estado actual y el plan de transición para la fase de implementación correspondiente.

### 20.18. Criterios de aceptación de la arquitectura física

La arquitectura de carpetas se considerará definida cuando:

1. Se haya presentado el árbol objetivo completo del monorepo.
2. Se haya explicado la responsabilidad de cada directorio principal.
3. Los cuatro backends tengan límites físicos explícitos.
4. Los módulos Rust estén organizados por ownership.
5. Se haya definido la ubicación de los BFF por producto.
6. Se haya definido la ubicación de los frontends por producto.
7. Se haya separado el código compartido de la lógica de dominio.
8. Se haya definido dónde viven los contratos fuente y el código generado.
9. Se haya definido el ownership de las migraciones.
10. Se haya definido la ubicación de configuración y despliegue.
11. Se haya establecido la política del único Cargo workspace.
12. Se hayan documentado las dependencias permitidas y prohibidas.
13. Se haya comparado la estructura objetivo con el repositorio real.
14. Se haya identificado qué elementos existen, cuáles deben reorganizarse y cuáles no deben crearse todavía.
15. No se hayan inventado directorios, paquetes ni aplicaciones para aparentar soporte de productos futuros.

El informe debe incluir una tabla con:

| Elemento | Ruta objetivo | Responsabilidad | Propietario | Estado actual |
|---|---|---|---|---|
| Backend CORE | `apps/backends/core/` | Ejecución y composición de CORE | CORE | Verificar |
| Backend BUSINESS | `apps/backends/business/` | Ejecución y composición de BUSINESS | BUSINESS | Verificar |
| Backend PLATFORM | `apps/backends/platform/` | Ejecución y composición de PLATFORM | PLATFORM | Verificar |
| Backend PUBLIC | `apps/backends/public/` | Ejecución y composición de PUBLIC | PUBLIC | Verificar |
| BFF Beauty | `apps/bffs/products/beauty/` | Adaptación de contratos para Beauty | Producto Beauty | Verificar |
| Frontend Beauty | `apps/frontends/products/beauty/` | Experiencia de usuario Beauty | Producto Beauty | Verificar |
| Módulos compartidos | `crates/shared/` | Primitivas técnicas justificadas | Compartido | Verificar |
| Contratos | `contracts/` | Contratos versionados | Según contrato | Verificar |

El estado debe completarse con evidencia del repositorio. No se deben conservar las etiquetas genéricas «Verificar» en el informe final.

Una distinción importante
Así quedaría organizada la arquitectura conceptual y física:
Capa de aplicaciones
apps/

Los cuatro backends ejecutables, los BFF de productos y los frontends.


Capa de dominio
crates/

Los módulos Rust organizados por ownership: CORE, BUSINESS, PLATFORM y PUBLIC.


Contratos e integración
contracts/ y database/

Protobuf, OpenAPI, schemas y migraciones con propietarios definidos.


Reutilización y operación
packages/, config/, deploy/ y tests/

Librerías técnicas, configuración, despliegues y verificaciones.


**La estructura física debe ser una consecuencia directa de la arquitectura aprobada. No debes adaptar la arquitectura de dominio a la comodidad de las carpetas existentes. Tampoco debes reorganizar todo el código sin una justificación, una matriz de impacto y una fase autorizada.**

**Esta arquitectura debe quedar definida desde la Fase 0 y respetarse obligatoriamente en todas las fases posteriores.**
