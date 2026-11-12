# ENSAMBRIA
## PROMPT MAESTRO — DISEÑO, ARQUITECTURA E IMPLEMENTACIÓN DE UNA PLATAFORMA SAAS MULTIPRODUCTO Y MULTIVERTICAL, CON PRODUCTO INICIAL DE BELLEZA, ESTÉTICA Y BIENESTAR

---

# 1. IDENTIDAD Y VISIÓN DEL PRODUCTO

El proyecto se llamará **ENSAMBRIA**. ENSAMBRIA es la plataforma tecnológica compartida desde la cual se construyen y operan productos SaaS comerciales independientes. El producto funcional inicial se orienta a belleza, estética y bienestar; no se afirma que la plataforma multiproducto o sus productos futuros ya estén implementados.

El producto inicial será una plataforma SaaS multi-tenant, modular, extensible y preparada para operar en distintos países, orientada a la administración, digitalización, comercialización y crecimiento de negocios de:

- Barberías.
- Peluquerías.
- Salones de belleza.
- Centros de uñas.
- Centros de estética.
- Spas.
- Centros de bienestar.
- Estudios de maquillaje.
- Centros especializados en tratamientos capilares.
- Otros negocios relacionados con belleza y cuidado personal.

No se debe construir un software limitado a la administración de barberías.

El objetivo es desarrollar un ecosistema empresarial que permita a cada establecimiento gestionar sus operaciones, clientes, trabajadores, servicios, reservas, pagos, inventario, marketing, rentabilidad y crecimiento desde una única plataforma.

ENSAMBRIA debe poder atender desde un pequeño establecimiento independiente hasta cadenas comerciales con múltiples sucursales, franquicias y operaciones internacionales.

## Visión a largo plazo

ENSAMBRIA debe evolucionar hacia:

1. Una plataforma de administración empresarial.
2. Un sistema de reservas y gestión de recursos.
3. Un CRM especializado en belleza y bienestar.
4. Una plataforma de fidelización y membresías.
5. Un sistema de pagos y monetización.
6. Un ecosistema de comercio electrónico.
7. Un marketplace de establecimientos y servicios.
8. Una plataforma de marketing automatizado.
9. Un sistema de inteligencia empresarial.
10. Un ecosistema abierto de integraciones y extensiones.

Todas las decisiones arquitectónicas iniciales deben considerar esta visión.

No se debe desarrollar una primera versión que obligue a reescribir el núcleo cuando se incorporen peluquerías, spas, centros de estética o franquicias.

### Evolución multiproducto y multindustria

La arquitectura objetivo es **una plataforma tecnológica compartida, un monorepo, cuatro backends centrales y múltiples productos SaaS comerciales**, con marcas, dominios, experiencias, configuraciones comerciales y ciclos de versión/despliegue propios cuando corresponda. Cada producto puede incorporar un vertical especializado, capacidades compartidas explícitamente habilitadas y un frontend y una superficie BFF propios. Esta decisión establece diseño y límites; no habilita runtime, lanzamiento ni la implementación anticipada de productos futuros.

Los conceptos no son intercambiables:

- **Plataforma tecnológica:** capacidades técnicas y de dominio compartidas, sin dependencia de una marca comercial.
- **Producto SaaS:** oferta y aplicación comercial identificable, compuesta explícitamente por capacidades, uno o más verticales aprobados, frontends/BFF, mercados, contratos y configuración.
- **Industria:** ámbito empresarial general, como belleza, automoción, veterinaria u hostelería.
- **Vertical:** modelo de dominio especializado dentro de una industria, con lenguaje, reglas, agregados, estados, contratos y pruebas propios. Un vertical no implica por sí mismo un backend, producto o crate.
- **Marca:** identidad comercial independiente de la tecnología, el dominio y las reglas de negocio. La marca no concede autorización ni determina ownership.

Beauty & Wellness es el producto y vertical funcional inicial. Los subtipos belleza (barberías, peluquerías, uñas, estética, spas y relacionados) no generan automáticamente productos o aplicaciones diferentes. La matriz de [dominios y productos](arquitectura-dominios.md) contempla como horizonte, no como alcance de lanzamiento: Automotive, Veterinary, Fitness & Gyms, Dentistry, Healthcare & Clinics, Physiotherapy & Rehabilitation, Alternative Wellness, Hotels & Accommodation, Restaurants & Gastronomy, Cleaning Services, Technical Repairs, Real Estate, Pet Services, Fashion & Apparel, Photography & Events, Education & Academies, Coworking & Shared Spaces, Construction & Maintenance y Tourism & Experiences. Cada candidato requiere descubrimiento, decisión de producto, ownership, implementación y aceptación propios antes de comercializarse.

El catálogo canónico de productos, sus identificadores estables, marcas/configuración comercial, dominios autorizados, capacidades declaradas y estado comercial pertenece a PLATFORM. CORE mantiene el contexto de producto validado y la asociación técnica tenant–producto junto con identidad, membresías y aislamiento; no administra catálogo, precios ni planes. El alta autoservicio asocia el tenant al producto seleccionado mediante contexto confiable y no crea suscripción, plan, cobro ni entitlement implícito. La identidad global de una persona no le otorga acceso a otro producto o tenant.

Cada producto define su propia configuración declarativa/versionada, superficies de frontend, contrato/API de BFF, branding, navegación, mercados, idiomas/monedas, SEO/Open Graph, documentos aplicables, capacidades e integraciones autorizadas. Una configuración no puede habilitar una capacidad no implementada ni omitir autorización. Los dominios resuelven una configuración de producto aprobada, pero nunca son prueba de identidad, pertenencia o permiso.

## Garantías empresariales objetivo

| Característica | Cómo se garantiza |
|---|---|
| Multi-tenancy | Aislamiento efectivo entre organizaciones, validado en cada frontera |
| Escalabilidad | Cuatro backends desplegables de forma independiente y extracción selectiva de módulos solo cuando se justifique |
| Extensibilidad | Módulos, proveedores y extensiones controladas mediante contratos desacoplados |
| Seguridad | RLS, RBAC, TLS/mTLS, Vault, auditoría y mínimo privilegio |
| Concurrencia | Transacciones, restricciones, bloqueos y control explícito de conflictos |
| Disponibilidad | Réplicas, recuperación, reconexión y tolerancia a fallos verificadas |
| Integridad financiera | Ledger inmutable, idempotencia, liquidaciones y conciliación |
| Interoperabilidad | REST/OpenAPI, gRPC/Protobuf y webhooks versionados |
| Observabilidad | Logs estructurados, métricas, trazas, alertas y correlación |
| Internacionalización | Monedas, idiomas, zonas horarias, jurisdicciones y reglas regionales |
| Automatización | Workers, eventos durables y tareas programadas idempotentes |
| Inteligencia empresarial | Analítica, indicadores con definiciones consistentes y predicciones evaluadas |

Estas son garantías de diseño que requieren implementación y evidencia en las fases correspondientes; no se consideran cumplidas por estar descritas en la arquitectura.

## Escenarios de escala ilustrativos

Para orientar decisiones futuras se pueden considerar escenarios como 1.000 establecimientos, 10.000 empleados, 100.000 clientes y miles de reservas diarias, con evolución eventual a cadenas internacionales, millones de clientes, Marketplace multi-país, múltiples proveedores de pago e integraciones/módulos adicionales. Son ejemplos de dimensionamiento, **no objetivos comprometidos ni capacidades de rendimiento demostradas**. Los límites reales requieren pruebas de carga, mediciones y dimensionamiento documentados.

---

# 2. PRINCIPIO ARQUITECTÓNICO FUNDAMENTAL

ENSAMBRIA debe diseñarse desde el principio con una arquitectura de:

**PLATAFORMA COMÚN + PRODUCTOS SAAS + CAPACIDADES COMPARTIDAS + VERTICALES ESPECIALIZADOS + CAPACIDADES CONTRATADAS.**

No se debe crear una aplicación independiente para cada subtipo de establecimiento dentro de un mismo producto. Sí se diseñan productos comerciales independientes para industrias/verticales aprobados, cada uno con frontend especializado, superficie BFF, marca y configuración propias; esta separación comercial no crea un backend de dominio por producto.

Tampoco se deben mezclar todas las funcionalidades en un único módulo genérico.

Cada dominio debe tener responsabilidades, límites, contratos, políticas y pruebas claramente definidos.
Cada funcionalidad tiene un único backend propietario de sus reglas de negocio y estado autoritativo. Otros backends pueden consumir sus contratos o eventos autorizados, pero no duplican la regla ni acceden a sus tablas.

La arquitectura objetivo se define desde la Fase 0 como cuatro monolitos modulares con contratos y despliegues independientes, pero esto no obliga a implementar todos los backends en paralelo. Se implementan las capacidades en el orden del roadmap y cada fase/subfase completa los recorridos aplicables antes de avanzar. No se divide cada módulo en un microservicio; una extracción futura debe responder a una necesidad técnica/operativa demostrable y contar con una decisión documentada.

Cada módulo de dominio posee sus modelos de dominio y persistencia, casos de uso, puertos, adapters, contratos, migraciones y pruebas. Los módulos invocan capacidades de otro dominio mediante contratos versionados; no importan implementaciones ni consultan directamente sus tablas.

Los contratos REST/OpenAPI, gRPC/Protobuf y eventos se versionan y mantienen compatibilidad explícita para permitir evolucionar backends, frontends e integraciones de manera independiente. Las extensiones/plugins se permiten únicamente como capacidades controladas: registro y contratos explícitos, permisos, aislamiento, límites, revisión/auditoría y habilitación por entitlement. No se ejecuta código arbitrario ni se permite que una extensión se apropie de datos/reglas de otro dominio.

## 2.1 ENSAMBRIA CORE

El CORE contiene las capacidades fundamentales compartidas por toda la plataforma.

Debe incluir:

### Identidad y seguridad
- Autenticación.
- Autorización.
- Gestión de sesiones.
- Usuarios.
- Roles y permisos.
- Recuperación de cuentas.
- MFA cuando corresponda.
- Gestión de credenciales.
- Auditoría de seguridad.

### Tenancy
- Identidad técnica e identificador inmutable del tenant.
- Contexto tenant autenticado, aislamiento entre organizaciones y validación de pertenencia.
- Relaciones de identidad entre tenant, organización, sucursales y usuarios cuando estén autorizadas.
- Configuración regional efectiva (idioma, zona horaria y moneda) como contexto operativo.
- Políticas de acceso del tenant y aplicación de entitlements.

CORE es propietario de la identidad y contexto de seguridad del tenant. PLATFORM crea y administra su relación comercial, planes, contratos y ciclo de vida SaaS; BUSINESS utiliza el contexto en la operación de los establecimientos; PUBLIC solo recibe los datos expresamente autorizados para publicación. La administración comercial no concede acceso directo a datos operativos.

El alta y el inicio de sesión de los operadores de un tenant son autoservicio en la superficie tenant del producto correspondiente; la identidad y el tenant pertenecen a CORE. Los clientes de cada establecimiento también disponen de páginas propias de registro e inicio de sesión asociadas a ese tenant en la superficie de cliente del producto: CORE autentica la cuenta y BUSINESS posee su ficha CRM. Ninguna identidad, por sí sola, concede membresía, acceso a otro tenant/producto ni acceso a datos CRM. Los formularios pasan por el BFF especializado de producto y contratos del backend propietario; ningún frontend accede directamente a CORE, PostgreSQL, Redis o NATS. Los administradores de PLATFORM no registran manualmente tenants, operadores o clientes finales.

### Identificadores y pertenencia a tenants

UUID v7 es el estándar canónico predeterminado de ENSAMBRIA para IDs de entidades distribuidas o referenciadas entre backends. La misma entidad conserva el mismo UUID en PostgreSQL, contratos y eventos; no se crean IDs de traducción locales en cada backend. CORE posee la identidad global de la persona y las relaciones de membresía por tenant, que contienen su propio estado y permisos. Una persona puede pertenecer a varios tenants; identidad, membresía, organización y ficha CRM son conceptos distintos.

`BIGINT` solo se admite como dato local adicional cuando mediciones demuestren una ventaja técnica concreta; no se expone en APIs/eventos ni se considera secreto o control de autorización. No se duplican IDs por convención. Un identificador público distinto (slug u otra representación) requiere necesidad y ownership documentados; el UUID público tampoco reemplaza la autorización. Cada entidad tiene una sola autoridad de generación: por defecto, la aplicación Rust del backend propietario. Generación en PostgreSQL depende de verificar la versión exacta desplegada y documentar su soporte. UUID v4 u otra excepción requiere justificación específica, como ocultar temporalidad en una presentación.

### Organización y sucursales
- Identidades centrales de organizaciones, establecimientos y sucursales, así como sus relaciones estructurales.
- Ubicaciones y jerarquía organizacional de referencia.

BUSINESS es propietario de la operación de cada establecimiento: configuración operativa, horarios, recursos físicos, personal y reglas comerciales. CORE conserva las relaciones centrales y el contexto de seguridad; no duplica las reglas operativas de BUSINESS.

### Gestión de capacidades y planes
Construir desde el principio un sistema de entitlements o capacidades contratadas.

Debe permitir:
- Activar y desactivar módulos.
- Asignar funcionalidades según el plan SaaS.
- Controlar límites de uso.
- Controlar cantidad de sucursales.
- Controlar cantidad de empleados.
- Controlar límites de reservas.
- Controlar almacenamiento.
- Controlar integraciones.
- Gestionar periodos de prueba.
- Aplicar restricciones por suscripción.
- Permitir complementos contratados.
- Gestionar excepciones comerciales autorizadas.

La activación de un módulo debe validarse en el backend. Ocultar una pantalla en el frontend nunca debe considerarse una medida de seguridad suficiente.
El catálogo y las suscripciones SaaS son propiedad de PLATFORM; CORE valida y aplica los entitlements autenticados que recibe por el contrato aprobado. El plan SaaS llamado **Franquicia** es una oferta comercial y no equivale por sí mismo al módulo funcional de gestión de franquicias/cadenas de BUSINESS.

### Motor de reservas y disponibilidad
El CORE debe proporcionar la infraestructura común para gestionar disponibilidad y reservas.

Debe soportar:
- Horarios.
- Calendarios.
- Disponibilidad.
- Bloqueos.
- Reservas.
- Estados.
- Reprogramaciones.
- Cancelaciones.
- Recursos asignables.
- Prevención de conflictos.
- Concurrencia.
- Zonas horarias.
- Reglas de anticipación.
- Capacidad de atención.
- Reservas individuales, grupales y recurrentes, cuando las reglas del servicio lo permitan.
- Listas de espera gestionadas automáticamente según reglas configurables de prioridad, disponibilidad y expiración de ofertas, con notificaciones; la reserva no se confirma ni se reasigna sin la autorización o aceptación exigida por las políticas aplicables.
- Asignación que considere profesional, especialidad, carga de trabajo, sucursal y recursos físicos.
- Reservas con varios profesionales o con varias etapas, duraciones y tiempos de preparación.
- Requerimientos simultáneos de recursos, como profesional + silla/cabina + equipo.
- Reasignación autorizada cuando hay ausencias, respetando disponibilidad y notificando a los afectados.
- Depósitos/anticipos y políticas configurables de confirmación, cancelación y sobrecupo.
- Confirmación, cancelación y reprogramación mediante canales habilitados, incluido WhatsApp cuando aplique.

Las reglas particulares de cada sector se implementarán mediante módulos verticales.
La asignación automática debe ser explicable y respetar las preferencias del cliente, permisos, disponibilidad, especialidades y restricciones de cada tenant; no puede confirmar una reserva imposible ni sobreescribir bloqueos.

### Pagos y transacciones
El CORE debe proporcionar las abstracciones compartidas de pagos y transacciones.

Debe diferenciar:
- Cobros del SaaS a sus establecimientos.
- Cobros de establecimientos a sus clientes.
- Pagos de reservas.
- Anticipos.
- Reembolsos autorizados.
- Pagos de productos.
- Pagos de membresías.
- Conciliación.
- Estados de transacciones.
- Eventos.
- Idempotencia.

No mezclar los ingresos de ENSAMBRIA con el dinero que corresponde a los establecimientos.

No asumir que una pasarela permite dividir pagos, cuentas independientes o cobros recurrentes sin verificar sus capacidades reales.

CORE es propietario del procesamiento de pagos, idempotencia, cargos transaccionales aprobados, reembolsos, ledger inmutable, liquidaciones y conciliación. PLATFORM es propietario de suscripciones/facturación SaaS y define las políticas comerciales de ENSAMBRIA; BUSINESS es propietario de ventas, caja y facturación comercial de cada establecimiento. BUSINESS y PLATFORM solicitan operaciones de pago a CORE mediante contratos autorizados; no mantienen ledgers paralelos ni alteran sus reglas. No se mezclan cuentas, credenciales, callbacks, documentos, saldos ni titularidad de ENSAMBRIA y los establecimientos.

## 2.2 Modelo empresarial multimodal y servicios móviles

ENSAMBRIA admite personas naturales y jurídicas, actividades profesionales individuales, negocios sin sede física, negocios con establecimientos y organizaciones que prestan servicios en una o varias modalidades. Todos utilizan la misma identidad, tenancy, seguridad y topología de cuatro backends. La modalidad declarada durante el registro guía la incorporación inicial; no fija el tipo del tenant ni limita su evolución.

### Persona, tenant, organización y establecimiento

- **Persona:** una persona natural es un sujeto humano cuya identidad global y autenticación pertenecen a CORE. Una persona jurídica es una entidad legal distinta de la cuenta de cualquier usuario humano; sus datos de identificación y representación solo se recogen cuando los requieran el mercado, la operación o una obligación aplicable. CORE conserva la referencia estructural y la relación de representación/autorización pertinentes; los atributos legales/fiscales los posee el dominio que los necesita (BUSINESS en la operación tenant, PLATFORM en la relación SaaS), sin duplicarlos. La persona que inicia sesión no se transforma por ello en la organización ni en el tenant.
- **Titular y participantes:** CORE registra la relación explícita de titularidad y las membresías/permisos pertinentes entre identidad, tenant y estructura autorizada. El titular, representante, administrador, empleado, colaborador o profesional obtiene únicamente los permisos concedidos para ese tenant, organización, sucursal, función y recurso. Una identidad puede participar en varios tenants con permisos distintos; ninguna relación concede roles globales de PLATFORM.
- **Tenant:** es el ámbito técnico estable de aislamiento y autorización. Puede ser operado por una persona natural o una persona jurídica; no representa necesariamente una persona jurídica, una sucursal, un local, una plantilla de empleados ni una modalidad operativa. CORE conserva su identidad, pertenencia y contexto; la relación comercial SaaS corresponde a PLATFORM. Abrir locales, incorporar personas o cambiar modalidades no sustituye ni duplica el tenant.
- **Organización:** es una estructura operativa dentro del tenant, no una afirmación de personalidad jurídica. Puede representar una actividad profesional individual, un negocio de persona natural o una empresa. CORE posee sus identificadores y relaciones estructurales; BUSINESS posee el perfil y las reglas de operación. La información de persona jurídica, titularidad o representación se modela separadamente cuando sea necesaria y autorizada. No se crea una organización diferente por sucursal automáticamente.
- **Sucursal:** es un ámbito organizativo/operativo opcional, con permisos y configuración propios. No se crea una sucursal predeterminada o ficticia para que un profesional sin local pueda operar. CORE mantiene su identidad/relación estructural y BUSINESS sus recursos, agendas, equipos y reglas.
- **Establecimiento:** es un lugar físico donde se prestan servicios, opcional y distinto de la sucursal. BUSINESS posee sus datos operativos; la relación estructural o de seguridad aplicable referencia el identificador de CORE. Una sucursal puede operar con un establecimiento cuando corresponda, pero sus identificadores y reglas no se intercambian ni se confunden. El domicilio del cliente, una zona de cobertura, una ubicación temporal acordada y una ubicación remota tampoco son establecimientos.

Ningún dato enviado por el cliente, identificador, nombre de empresa, dominio o dirección demuestra titularidad ni concede acceso. Los mecanismos de verificación de identidad, representación o datos fiscales se determinan por mercado y caso; no se exige universalmente una persona jurídica, NIT de persona jurídica, matrícula, dirección comercial, sucursal, empleado o inventario.

### Incorporación autoservicio y evolución

El inicio de la incorporación presenta las modalidades informativas:

1. **Tengo un negocio o establecimiento** — tengo un salón, spa, clínica, gimnasio u otro establecimiento físico.
2. **Soy profesional independiente** — soy persona natural y presto servicios a domicilio o me desplazo hasta mis clientes.
3. **Tengo un negocio con varias sucursales** — administro varios establecimientos y equipos de trabajo.

Estas opciones son rutas iniciales de configuración, no clases inmutables de tenant ni autorización para crear sucursales, empleados, cobros o contratos automáticamente. El onboarding solicita progresivamente los datos necesarios según modalidad, actividad, producto, mercado y obligaciones aplicables; guarda el progreso bajo el owner de cada dato, permite reanudarlo y soporta reintentos idempotentes. Su secuencia contempla cuenta/identidad, modalidad/actividad, verificación de identidad o representación si aplica, creación CORE del tenant y membresía, perfil/configuración operativa BUSINESS, establecimientos/equipo opcionales, y activación solo de capacidades realmente implementadas y autorizadas. Los pasos con dependencias entre backends se reconcilian por contratos/idempotencia; no se declara una transacción distribuida. No solicita información de local, persona jurídica o personal cuando no corresponde al caso elegido.

CORE autentica al usuario, crea el tenant y las membresías autorizadas, y concede al actor creador la titularidad/administración inicial de ese tenant y actividad dentro de su scope; no le confiere permisos globales ni acredita por sí sola titularidad legal de una persona jurídica. CORE mantiene el contexto estructural. La alta confirmada conserva exactamente el flujo de [ADR-0002](adr/ADR-0002-outbox-alta-tenant-core-platform.md): Outbox→JetStream→Inbox, inicio de `onboarding_pending` en PLATFORM y ausencia de asignación automática de planes, precios, cobros, trials o entitlements. BUSINESS conserva la configuración y el progreso de la actividad operativa que le pertenecen; BFF coordina la presentación, no una transacción distribuida ni reglas de dominio. Cada paso entre owners debe recuperarse por contrato, idempotencia y reconciliación definida antes de implementar ese recorrido.

La operación evoluciona con el mismo tenant, identidad, clientes, reservas, historial y trazabilidad: trabajar individualmente; incorporar colaboradores; abrir el primer establecimiento; añadir sucursales o modalidades. Cada ampliación crea únicamente las relaciones y recursos reales que el usuario autoriza. No migra datos artificialmente entre tenants/organizaciones, no reinicia una suscripción por inferencia ni produce pérdida, duplicación o privilegios implícitos. Si un cambio requiere una transformación de datos, su fase define migración versionada, transaccional y reversible cuando sea viable, junto con verificación y recuperación.

### Servicios, modalidades y ubicaciones

BUSINESS posee el catálogo operativo, las condiciones del servicio, sus profesionales y las reglas específicas de cada vertical. Un servicio declara explícitamente las modalidades que admite:

- en un establecimiento identificado;
- en el domicilio del cliente;
- en otra ubicación acordada;
- remoto/virtual, solo cuando la actividad lo permita.

Un mismo servicio puede admitir varias modalidades sin duplicarse solo por cambiar el lugar. Los atributos propios del servicio —descripción, duración, requisitos y regla de precio— se distinguen de los requisitos de modalidad —lugar elegible, recursos, cobertura, desplazamiento y preparación—. Una combinación no permitida se rechaza en backend, aunque la interfaz la ofrezca.

El contexto de ubicación diferencia: establecimiento, dirección privada del cliente para una atención concreta, zona habilitada, ubicación temporal acordada, ubicación declarada de un profesional y ubicación remota. La dirección podrá contener país, región/municipio, ciudad/localidad, dirección y complemento, coordenadas cuando sean necesarias, instrucciones de acceso, contacto para esa atención y restricciones relevantes; se almacena solo lo necesario, con permisos y retención definidos. BUSINESS mantiene los establecimientos y las políticas/áreas de cobertura; PUBLIC solo expone ubicación y disponibilidad expresamente autorizadas, nunca domicilios privados. CORE, como owner del estado de reserva, conserva la ubicación necesaria como snapshot histórico, minimizado y protegido dentro de la reserva; no se convierte en dirección pública ni en una copia general de CRM. BUSINESS puede mantener direcciones guardadas de clientes solo como tratamiento separado bajo su owner y controles aplicables. La ubicación precisa del profesional no se conserva ni transmite continuamente por defecto.

Las zonas pueden expresarse con divisiones territoriales o límites geográficos adecuados (incluidas áreas, exclusiones o polígonos cuando sean necesarios), distancia máxima, vigencia y restricciones de servicio. No se seleccionan proveedor, método de geocodificación, unidades, umbral ni política comercial por defecto en Fase 0. La validación de cobertura ocurre en backend contra la política vigente de BUSINESS; una coordenada o IP aislada no demuestra elegibilidad, residencia ni autorización. La geolocalización es opcional cuando no sea necesaria y, cuando corresponda, queda sujeta al consentimiento y políticas aplicables; no se implementa seguimiento permanente injustificado. Las restricciones territoriales de producto siguen la [política de acceso geográfico](rules/ENSAMBRIA_Geographic_Access_Policies.md): una incertidumbre de ubicación no bloquea la navegación ni solicitudes de privacidad.

Una reserva a domicilio verifica, según la modalidad y datos mínimos necesarios, dirección y zona, profesional habilitado, duración, disponibilidad, tiempo de desplazamiento y margen entre citas, preparación, capacidad diaria y compatibilidad con la reserva anterior y siguiente. BUSINESS evalúa elegibilidad operativa y expone una opción/versiones por contrato; CORE vuelve a validar el alcance y conserva/actualiza atómicamente el estado central de la reserva para impedir doble asignación. Un cambio concurrente invalida la opción obsoleta en vez de confirmar un itinerario imposible. Las reglas exactas de cálculo, proveedores geográficos, buffers y umbrales son decisión del owner de producto/operación, no defaults de esta especificación.

BUSINESS separa precio del servicio, desplazamiento y cargos adicionales autorizados, descuentos e impuestos aplicables; el total y su composición deben ser explícitos y reproducibles. BUSINESS conserva la regla comercial del servicio/desplazamiento; CORE solo procesa y registra los efectos financieros solicitados conforme a una política aprobada. No se fijan valores, distancias gratuitas, comisiones, impuestos, descuentos, cargos ni mecanismos de cobro. Una obligación legal/fiscal, producto de pagos o mercado que requiera validación previa permanece deshabilitado solo para ese alcance.

La asignación manual puede usar permisos, especialidad, modalidad, disponibilidad, zona y capacidad. La asignación automática solo se habilita con política aprobada, criterios explicables y auditables, y pruebas de equidad/operación que resulten aplicables. Ser titular no asigna automáticamente todas las reservas al titular.

CORE conserva el ciclo de vida común de la reserva, sus transiciones autorizadas y su historia. BUSINESS conserva por separado el estado operativo de la prestación cuando exista (por ejemplo, profesional asignado, preparación, desplazamiento iniciado, llegada, servicio iniciado/finalizado o incidencia); las listas concretas se delimitan con el caso de uso y no se duplican en estados de reserva. Cada transición valida actor, permiso, estado previo, versión e idempotencia. Confirmar, reprogramar o cancelar no debe dejar cargos, reservas o asignaciones parciales.

### Profesionales, clientes, finanzas y privacidad

CORE posee identidad humana, autenticación, membresías y contexto de autorización; BUSINESS posee perfiles operativos, especialidades, disponibilidad, servicios y asignaciones de workforce. Una membresía del tenant y una relación operativa/laboral/contractual son conceptos distintos. Incorporar un colaborador no presume que sea empleado laboral, representante legal o titular; sus permisos se delimitan por producto, tenant, organización, sucursal, función y recurso. La misma persona puede prestar servicios en varios ámbitos autorizados sin duplicar identidad.

BUSINESS posee el CRM tenant-scoped: sus clientes, historial, preferencias y comunicaciones autorizadas. Una identidad de cliente no fusiona fichas ni permite compartir historial entre negocios; evitar duplicados o unir perfiles requiere una regla explícita, permiso y evidencia. El acceso a direcciones, ubicación de servicio, datos financieros, historial y datos de personal se minimiza por finalidad y rol. Aplican [PDLM](rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md) y [cumplimiento de privacidad](rules/ENSAMBRIA_Privacy_Legal_Compliance.md); geolocalización es opcional cuando no sea necesaria, no implica seguimiento permanente y debe tener propósito, acceso, retención y auditoría definidos. Datos sensibles y usos sujetos a validación profesional permanecen deshabilitados si falta la aprobación previa requerida.

Los pagos SaaS al proveedor ENSAMBRIA siguen siendo propiedad comercial de PLATFORM y su efecto financiero corresponde a CORE. Los pagos de clientes finales por servicios/productos corresponden al negocio y se procesan por CORE mediante contrato, sujetos al producto de pago, mercado, titularidad y condiciones verificadas. BUSINESS consulta por contrato los ingresos/operaciones que puede mostrar al actor y scope autorizados, sin mantener un ledger paralelo; CORE conserva el estado financiero autoritativo. No se presume que un profesional natural pueda usar cualquier pasarela ni se mezclan cuentas, saldos, facturación, reembolsos o ledger.

El modelo es reutilizable por otros sectores sin declararlos soportados: cada vertical de BUSINESS decide qué modalidades, requisitos, restricciones y datos utiliza. Mobile/Home Services describe la prestación de servicios por profesionales móviles; no es logística de reparto de productos.

### Ownership, contratos y criterios por fase

| Dominio/capacidad | Owner y frontera | Contratos/dependencias | Fase prevista |
|---|---|---|---|
| Identidad, tenant, titularidad, membresía, organización/sucursal y contexto | CORE conserva identidades globales, tenant estable, relaciones estructurales y autorización; organización/sucursal no acreditan forma jurídica ni local físico. | CORE valida producto/actor y propaga scope; BUSINESS consume identificadores y scopes autorizados. Alta CORE→PLATFORM conserva ADR-0002. | Diseño Fase 0; identidad/tenancy/aislamiento y alta operativa Fase 1. |
| Reserva y ubicación de atención | CORE posee reserva, transiciones comunes, concurrencia, idempotencia e historial mínimo de ubicación. | Consulta síncrona de elegibilidad con BUSINESS; cambios durables/realtime solo desde el owner persistente, según necesidad documentada. No hay lecturas de tablas ajenas. | Contratos y núcleo Fase 1; modalidades se introducen con el servicio vertical Fase 2. |
| Catálogo y perfil de negocio | BUSINESS posee actividad, servicios, modalidades permitidas, condiciones comerciales, establecimientos y contenido operativo. | Referencias CORE para tenant/organización/sucursal; CORE para reserva/pago; PUBLIC solo por publicación explícita. | Modalidades iniciales y verticales Fase 2. |
| Servicios móviles, cobertura y prestación | BUSINESS posee cobertura, factibilidad de desplazamiento, reglas operativas y estado de prestación; no posee el estado común de reserva. | Consulta/versiones con CORE para reservar; ubicación/dirección minimizada; eventos solo para consumidores y cambios persistidos realmente requeridos. | Cimientos funcionales con servicios Fase 2; workforce/cobertura, asignación y operación completa Fase 4. |
| CRM, miembros y clientes | BUSINESS posee fichas, historial y preferencias; CORE mantiene solo identidad/membresía/permiso. | Contratos autenticados por BFF; no compartir ni fusionar historial por identidad global. | CRM Fase 3. |
| Planes y relación SaaS | PLATFORM mantiene oferta y relación comercial; CORE no asigna políticas comerciales. | Alta durable e idempotente conforme ADR-0002; cualquier plan/precio/entitlement necesita acción comercial explícita. | Inicio pendiente de onboarding en Fase 1; resto según roadmap PLATFORM. |
| Discovery y perfil profesional público | PUBLIC posee proyecciones mínimas y autorizadas; BUSINESS conserva el contenido fuente. | Publicación/revocación versionada por contrato; nunca publicar domicilio del cliente ni datos privados. | Superficie pública según Fase 5 y Marketplace Fase 9. |

Fase 0 documenta el diseño y los casos, sin implementar formularios, esquemas, APIs, eventos, contratos ejecutables ni infraestructura. La aceptación futura debe incluir, como mínimo: titular natural que crea tenant sin persona jurídica, dirección comercial ni establecimiento; tenant que inicia sin sucursal y luego agrega establecimiento sin cambiar IDs; profesional que agrega colaboradores y permisos acotados; organización que agrega sucursales sin duplicar cuenta, tenant o historial; combinación de modalidades permitidas/prohibidas; cobertura no elegible, zona excluida y dato geográfico insuficiente; dos reservas simultáneas incompatibles o con desplazamiento imposible; reintentos/duplicados y fallo en cada frontera CORE–BUSINESS–PLATFORM; transición de prestación no autorizada; cliente y dirección ajenos; ubicación privada ausente de Discovery, logs y eventos; revocación, cancelación, cambio de cobertura y recuperación por cursor/snapshot. Los tests reales se ejecutan en las fases propietarias y con las dependencias correspondientes; esta lista no afirma que estén escritos o ejecutados.

---

# 3. MÓDULOS VERTICALES POR TIPO DE NEGOCIO

Cada vertical debe aprovechar ENSAMBRIA CORE y proporcionar sus propias configuraciones, entidades, reglas de negocio, pantallas y funcionalidades.

Los verticales no deben duplicar identidad, tenancy, usuarios, pagos, sucursales ni el motor común de reservas.

## 3.1 Módulo BARBERÍAS

Funcionalidades:
- Gestión de barberos.
- Servicios de barbería.
- Cortes.
- Arreglo de barba.
- Tratamientos.
- Servicios combinados.
- Duraciones.
- Precios.
- Comisiones.
- Sillas de barbería.
- Asignación de barberos.
- Horarios.
- Reservas.
- Preferencias de clientes.
- Historial de cortes.
- Fotografías autorizadas.
- Membresías de cortes.
- Paquetes.
- Promociones.
- Control de capacidad.
- Estadísticas por barbero.
- Rentabilidad por servicio.

## 3.2 Módulo PELUQUERÍAS

Funcionalidades:
- Estilistas.
- Servicios capilares.
- Cortes.
- Tintes.
- Colorimetría.
- Decoloraciones.
- Mechas.
- Tratamientos capilares.
- Alisados.
- Peinados.
- Lavado y secado.
- Fórmulas personalizadas.
- Historial de fórmulas.
- Marcas y productos utilizados.
- Tiempo de exposición.
- Valoración o diagnóstico capilar cosmético/no clínico y seguimiento, con observaciones pertinentes al servicio, fecha, profesional responsable y vínculo con tratamientos/fórmulas autorizados.
- Seguimiento de tratamientos.
- Lavacabezas.
- Estaciones de trabajo.
- Recursos compartidos.
- Paquetes.
- Costos de materiales.
- Rentabilidad por tratamiento.

El registro de fórmulas debe permitir consultar información histórica y preservar la trazabilidad de modificaciones.

La evaluación capilar es un registro operativo para personal autorizado, no una historia clínica, diagnóstico médico ni sustituto de valoración profesional regulada. Debe limitarse a información necesaria para prestar y dar seguimiento al servicio, aplicar permisos por tenant/sucursal y conservar trazabilidad de acceso y cambios. Antes de registrar información que pueda ser dato sensible o de salud, el alcance debe contar con finalidad, base aplicable, clasificación, minimización, controles de acceso, retención y derechos documentados y validados por los responsables de Privacidad/Legal para la jurisdicción correspondiente. Si esa validación o los controles necesarios están pendientes, la captura de esos datos permanece deshabilitada; no se presume que el consentimiento sea la única base posible ni que una evaluación cosmética esté automáticamente exenta.

## 3.3 Módulo SALONES DE BELLEZA

Funcionalidades:
- Manicura.
- Pedicura.
- Uñas acrílicas.
- Uñas en gel.
- Nail art.
- Pestañas.
- Cejas.
- Maquillaje.
- Depilación.
- Tratamientos faciales.
- Paquetes de belleza.
- Galería de diseños.
- Historial de técnicas.
- Preferencias del cliente.
- Profesionales especializados.
- Cabinas y estaciones.
- Inventario de materiales.
- Costos por servicio.
- Comisiones.

## 3.4 Módulo SPAS Y ESTÉTICA

Funcionalidades:
- Masajes.
- Tratamientos corporales.
- Tratamientos faciales.
- Sesiones.
- Paquetes de varias sesiones.
- Terapias de bienestar.
- Terapeutas.
- Cabinas.
- Camillas.
- Equipos.
- Recursos compartidos.
- Duraciones.
- Tiempos de preparación.
- Historial de tratamientos.
- Consentimientos.
- Documentación autorizada.
- Seguimiento de sesiones.
- Planes de tratamiento y seguimiento de su progreso entre sesiones, con historial de cambios y acceso limitado al personal autorizado.
- Fotografías de evolución únicamente cuando exista autorización específica para esa finalidad; conservar vínculo con el plan/sesión, permisos, trazabilidad, retención y eliminación conforme a la finalidad y jurisdicción aplicables. No publicarlas ni reutilizarlas en marketing sin autorización independiente.
- Control de equipos.
- Mantenimiento.
- Inventario especializado.

Para centros de estética que realicen procedimientos sujetos a regulación sanitaria, contemplar permisos profesionales, controles de acceso, documentación y requisitos legales aplicables.

Los datos de salud, contraindicaciones y demás información sensible deben tener tratamiento, permisos, protección y retención específicos. No convertir el sistema en una historia clínica sin una definición regulatoria y funcional expresa.

## 3.5 Diseño de los módulos verticales

ENSAMBRIA debe atender desde su núcleo común, sin crear una aplicación independiente por sector:

| Sector | Ejemplos de capacidades verticales |
|---|---|
| Barberías | Cortes, barba, membresías, barberos, sillas y turnos |
| Peluquerías y tratamientos capilares | Cortes, tintes, tratamientos, estilistas, lavacabezas, fórmulas y tiempos de servicio |
| Salones de belleza y centros de uñas | Uñas, maquillaje, pestañas, cejas, depilación, paquetes, estaciones y materiales |
| Spas, estética y bienestar | Masajes, tratamientos corporales/faciales, cabinas, terapeutas, equipos y paquetes |

La arquitectura debe permitir incorporar o activar centros de estética, estudios de maquillaje, centros de uñas, centros de bienestar y negocios especializados en tratamientos capilares. Pueden comenzar como configuraciones/capacidades de un vertical existente; se separan como módulos de dominio cuando sus reglas y ownership lo justifiquen. No se fuerza a todos los sectores a usar formularios, recursos o flujos idénticos. Los datos de salud o sensibles siguen sujetos a las salvaguardas y límites de la sección de privacidad; un vertical de estética no se convierte implícitamente en historia clínica.

Cada módulo vertical debe poder:
- Instalarse o activarse por tenant.
- Configurarse independientemente.
- Utilizar los servicios del CORE.
- Exponer sus propios casos de uso.
- Tener sus propias reglas.
- Tener sus propias pruebas.
- Integrarse con otros módulos mediante contratos explícitos.
- Evolucionar sin romper otros verticales.

Un establecimiento puede tener más de un vertical cuando su modelo comercial lo requiera.

Por ejemplo, un negocio puede ofrecer barbería y tratamientos capilares.

### Separación de capacidades compartidas y reglas por industria

El diseño objetivo mantiene en `backend/business/modules/` las capacidades operativas reutilizables cuya semántica sea común a más de una industria, y agrupa las reglas, datos y casos de uso específicos en `backend/business/verticals/<industria>/`. `backend/business/verticals/beauty-wellness/README.md` es la carta documental del vertical principal; su presencia no significa que exista runtime ni que sus subtipos sean módulos separados. Las futuras carpetas verticales se crean cuando haya un alcance aceptado y contenido propio, no como placeholders. La agrupación por industria no crea otro backend, despliegue o crate automáticamente; cada bounded context y paquete se delimita por ownership y necesidad verificable.

Los módulos compartidos deben modelar conceptos realmente generales y no introducir ramas de negocio específicas de una industria para poder atenderla. Si una capacidad compartida necesita una operación o dato que solo pertenece a un vertical, el vertical conserva su regla y su información especializada e integra la capacidad común mediante un contrato publicado por el owner. El vertical no modifica tablas, migraciones ni implementación interna del módulo compartido. Si una necesidad se demuestra común a varias industrias, su owner puede ampliar el contrato y comportamiento generalizado, preservando compatibilidad y probando los verticales existentes afectados. No se presupone un esquema universal, campos dinámicos, un motor genérico de reglas ni una abstracción de extensiones antes de que exista un caso de uso y diseño aceptados.

Dentro de una misma industria, distintos tipos de negocio pueden compartir el vertical cuando el lenguaje y las reglas lo permitan; no se exige una carpeta ni un bounded context por subtipo. Una nueva industria requiere descubrimiento y aceptación propios, y no se considera compatible o implementada por el solo hecho de tener una ubicación prevista. El módulo `establishment-catalog-verticals` es dueño del registro común de servicios operativos: identidad estable, establecimiento asociado, nombre, estado de ciclo de vida y versión. Cada vertical dueño conserva las definiciones, atributos, parámetros y reglas específicos de su industria, relacionados con el registro común mediante su identificador y contratos publicados. El registro compartido no impone un modelo universal de servicios ni incorpora campos o reglas exclusivos de una industria; esta decisión establece ownership, mientras que los contratos y casos de uso ejecutables se especifican en la Fase 2 antes de implementar.

No imponer exclusividad artificial entre categorías.

---

# 4. MÓDULOS ADICIONALES DE ENSAMBRIA

Estos módulos deben ser independientes y activables según las capacidades contratadas.

El CRM debe ser un módulo independiente, NO una parte inseparable del CORE.

## 4.1 CRM — Gestión de relaciones con clientes

Construir un CRM especializado en belleza y bienestar.

Funcionalidades:
- Perfiles de clientes.
- Datos de contacto.
- Consentimientos.
- Historial de visitas.
- Historial de compras.
- Historial de reservas.
- Preferencias.
- Servicios favoritos.
- Profesionales favoritos.
- Segmentación.
- Etiquetas.
- Notas autorizadas.
- Historial de comunicaciones.
- Clientes nuevos.
- Clientes recurrentes.
- Clientes inactivos.
- Valor de vida del cliente.
- Frecuencia de visitas.
- Ticket promedio.
- Segmentos comerciales.
- Seguimiento de oportunidades.
- Recuperación de clientes.
- Indicadores de fidelización.

El CRM debe integrarse con reservas, membresías, marketing, pagos y verticales, sin crear dependencias circulares.
Los clientes pueden crear y usar su propia cuenta mediante las páginas de registro e inicio de sesión del establecimiento. CORE posee su autenticación global; BUSINESS mantiene la ficha CRM y su asociación autorizada al tenant. No se crea una identidad CRM duplicada ni se revela una ficha por el solo hecho de autenticarse.
Cuando el cliente autorice, conservar fotografías de resultados vinculadas a visitas/servicios. Deben aplicarse permisos de acceso, consentimiento verificable, retención y eliminación conforme a la finalidad y jurisdicción; las fotos no se exponen en galerías públicas sin autorización específica.

Los recordatorios de mantenimiento y las campañas de reactivación deben usar el ciclo del servicio, las preferencias y los consentimientos del cliente. El umbral de inactividad se configura por tenant/campaña; un número de días ilustrativo no es una política global fija.

## 4.2 Membresías y fidelización

Funcionalidades:
- Membresías mensuales.
- Membresías anuales.
- Planes de beneficios.
- Créditos.
- Consumos.
- Servicios incluidos.
- Descuentos.
- Renovaciones.
- Cancelaciones.
- Paquetes prepagados.
- Puntos.
- Niveles VIP.
- Recompensas.
- Bonos.
- Gift cards.
- Referidos.
- Historial inmutable de consumos.

Ejemplos ilustrativos para barberías:
- Básico: $20.000 / 1 corte.
- Premium: $50.000 / 3 cortes.
- VIP: $70.000 / 4 cortes.

Estos valores son ejemplos, no políticas comerciales definitivas.

El sistema debe permitir que cada establecimiento configure sus propios precios y beneficios.

Diseñar correctamente:
- Saldo disponible.
- Consumo concurrente.
- Idempotencia.
- Renovaciones.
- Vencimientos.
- Reembolsos.
- Cancelaciones.
- Reversión autorizada.
- Prevención de consumos duplicados.

## 4.3 Inventario y compras

Debe incluir:
- Productos para venta.
- Productos de consumo interno.
- Insumos.
- Materiales.
- Lotes.
- Vencimientos.
- Existencias.
- Kardex.
- Movimientos.
- Ajustes auditados.
- Compras.
- Proveedores.
- Órdenes de compra.
- Transferencias.
- Inventario por sucursal.
- Costos.
- Alertas.
- Stock mínimo.
- Consumo por servicio.
- Recetas y fórmulas de productos.
- Control de desperdicio.
- Rentabilidad de productos.
- Costeo real por servicio/tratamiento usando consumos registrados y costos vigentes, con trazabilidad de los cambios de fórmulas.
- Cálculo del costo directo real por servicio/tratamiento que integre los materiales efectivamente consumidos y la comisión del empleado que corresponda según la regla vigente de BUSINESS. El cálculo debe conservar la sucursal, fecha, cantidades, costos unitarios, versión de fórmula y regla/base de comisión utilizados para poder explicar y reproducir resultados históricos.
- Diferenciación entre costos estimados y reales, y entre costo directo, comisión del empleado, cargos de pasarela, cargos de ENSAMBRIA, ingresos, margen y utilidad. No sumar dos veces un mismo concepto ni presentar la comisión de pago de CORE o la comisión comercial de PLATFORM como comisión laboral de BUSINESS.
- Separación de inventario y movimientos por sucursal, con transferencias autorizadas y auditadas.
- Alertas configurables de existencias mínimas y vencimiento, sin ocultar lotes vencidos ni inventario inmovilizado.

Las operaciones de inventario deben mantener integridad transaccional y trazabilidad. BUSINESS coordina el costo de materiales con sus reglas y registros de comisiones laborales mediante contratos internos del dominio; no accede a tablas de otros módulos ni modifica el ledger de CORE. Los ajustes de consumos, costos, comisiones o reglas deben ser autorizados y auditables, y los reportes deben distinguir importes calculados de importes efectivamente registrados.

## 4.4 Marketing y automatización comercial

Debe incluir:
- Campañas.
- Segmentación.
- Promociones.
- Cupones.
- Cupones personalizados por cliente o segmento, sujetos a elegibilidad, límites de uso, vigencia y reglas comerciales versionadas de BUSINESS; no revelar segmentación privada en superficies públicas ni aplicar un beneficio sin validarlo en backend.
- Automatizaciones.
- Recuperación de clientes.
- Recordatorios.
- Cumpleaños.
- Campañas de reactivación.
- Campañas por comportamiento.
- Campañas por sucursal.
- Campañas por servicio.
- Medición de resultados.
- Atribución.
- Venta cruzada configurable de productos y servicios complementarios en contextos pertinentes, como consulta de servicio, reserva, compra y campañas autorizadas. Las recomendaciones deben respetar compatibilidad del servicio, disponibilidad, inventario, sucursal, reglas y precios vigentes, y las capacidades contratadas; no deben añadir productos al carrito, confirmar reservas, aplicar descuentos ni cobrar sin una acción explícita del usuario.
- Medición de impresiones, selecciones y conversiones de venta cruzada con atribución suficiente para evaluar resultados, minimizando los datos registrados y sin exponer información personal en métricas o telemetría.
- Control de consentimientos.
- Preferencias de comunicación.
- Exclusión de usuarios.

Integraciones potenciales:
- WhatsApp Business Cloud API.
- Correo electrónico.
- SMS.
- Notificaciones push.
- Canales adicionales autorizados.

Respetar las políticas de cada proveedor, las ventanas de mensajería, plantillas y consentimientos aplicables.

BUSINESS es propietario de las audiencias, reglas comerciales y campañas de venta cruzada de los establecimientos. Cualquier comunicación saliente debe respetar la finalidad, base o consentimiento aplicable, preferencias, exclusiones, límites de frecuencia y políticas del canal en la jurisdicción correspondiente. La recomendación en una interfaz no autoriza por sí sola el envío de marketing.

## 4.5 Inteligencia artificial

Construir un módulo independiente de IA.

Capacidades futuras:
- Asistente empresarial.
- Consultas en lenguaje natural sobre ventas, reservas, clientes y rentabilidad.
- Análisis de ventas.
- Análisis de reservas.
- Predicción de demanda por día, horario, sucursal y tipo de servicio.
- Predicción de cancelaciones/inasistencias, con límites de uso y explicación de las señales relevantes.
- Recomendaciones comerciales para periodos de baja ocupación.
- Recomendación de horarios y capacidad.
- Detección de clientes inactivos.
- Análisis de rentabilidad por servicio, empleado y sucursal.
- Recomendaciones de inventario.
- Generación de campañas.
- Generación de publicaciones.
- Asistente de reservas y atención al cliente por WhatsApp, sujeto a consentimiento, políticas del proveedor y capacidades verificadas.
- Asistente de atención al cliente.
- Resúmenes operativos.
- Detección de anomalías.

La generación de contenido debe respetar la marca, el sector, los permisos y las políticas de los canales; todo contenido se presenta para revisión antes de su publicación. Las recomendaciones de promociones requieren aprobación del usuario y no envían comunicaciones por sí solas.

Ejemplo:
"¿Cuáles fueron mis cinco servicios más rentables este mes y qué debería promocionar la próxima semana?"

La IA debe responder utilizando datos reales, autorizados y correspondientes al tenant.

No permitir que un modelo acceda directamente a toda la base de datos sin controles.

Implementar:
- Autorización por tenant y usuario.
- Herramientas limitadas.
- Consultas controladas.
- Registro de acciones.
- Protección contra filtraciones.
- Límites de consumo.
- Control de costos.
- Evaluación de respuestas.
- Confirmación humana para acciones sensibles.

La IA no debe inventar resultados, realizar cobros, modificar registros financieros ni ejecutar operaciones críticas sin autorización.

## 4.6 Analítica y BI

Debe incluir:
- Dashboard ejecutivo.
- Ventas.
- Ingresos.
- Gastos.
- Margen.
- Rentabilidad.
- Ticket promedio.
- Ocupación.
- Capacidad.
- Reservas.
- Cancelaciones.
- Clientes nuevos.
- Retención.
- Frecuencia de visitas.
- Rendimiento por empleado.
- Rendimiento por sucursal.
- Servicios más rentables.
- Productos más vendidos.
- Comparación de periodos.
- Predicciones.
- Exportaciones.
- Informes programados.

Construir indicadores con definiciones consistentes. Diferenciar ingresos, recaudos, ventas, margen y utilidad.

## 4.7 Franquicias y cadenas

Debe permitir:
- Empresas matrices.
- Franquiciados.
- Sucursales.
- Jerarquías.
- Supervisores.
- Administración regional.
- Catálogos centralizados.
- Precios globales o locales.
- Inventario.
- Transferencias.
- Indicadores consolidados.
- Comparación entre establecimientos.
- Comisiones y regalías configurables.
- Auditoría.
- Permisos jerárquicos.
- Control de caja separado por establecimiento/sucursal, con atribución de operaciones, permisos de acceso y trazabilidad para revisión consolidada por los roles autorizados de la cadena.
- Liquidaciones.
- Reportes consolidados.

Una cadena debe poder administrar todas sus sucursales sin comprometer el aislamiento de otros tenants.

## 4.8 Marketplace — backend PUBLIC

PUBLIC es propietario del canal Marketplace y del descubrimiento público. No es propietario del catálogo operativo, los servicios, empleados, inventario ni operaciones privadas de los establecimientos.

Funcionalidades:
- Directorio de establecimientos.
- Búsqueda por ubicación.
- Descubrimiento de establecimientos cercanos usando una ubicación elegida por la persona o una señal de ubicación autorizada, con controles de privacidad y sin tratar la ubicación aproximada como prueba de residencia o jurisdicción.
- Categorías.
- Filtros.
- Perfiles.
- Fotografías autorizadas.
- Servicios.
- Precios.
- Comparación de establecimientos y ofertas publicadas por categoría, servicio, precio vigente, duración, ubicación/sucursal, disponibilidad y otros atributos públicos autorizados. La comparación debe mostrar las condiciones relevantes y el momento/contexto de la disponibilidad cuando pueda cambiar; no puede presentar información privada, no publicada, revocada o desactualizada como vigente.
- Profesionales.
- Disponibilidad.
- Fotografías autorizadas de establecimientos y trabajos/servicios realizados, sujetas al estado de publicación, autorización específica y controles de privacidad aplicables.
- Reservas.
- Pagos.
- Favoritos.
- Reseñas verificadas.
- Promociones.
- Posicionamiento destacado.
- Historial de reservas.

PUBLIC consume información expresamente publicada por BUSINESS mediante contratos/eventos versionados, y solicita reservas o pagos a los backends propietarios mediante contratos autorizados. No confía en importes o estados calculados por el cliente.

La comparación se construye únicamente con proyecciones públicas autorizadas y conserva la procedencia y vigencia de los datos. CORE sigue siendo propietario de la disponibilidad/reserva y BUSINESS del catálogo, precio e inventario; PUBLIC no recalcula ni altera esas reglas. Los resultados patrocinados o destacados deben identificarse como tales y distinguirse de los criterios orgánicos de búsqueda; una comparación nunca garantiza que el precio o cupo continúe disponible antes de la validación final del backend propietario.

Todo establecimiento/servicio/producto publicado debe tener una ficha pública indexable y optimizada para buscadores. BUSINESS es dueño del contenido operativo y de los campos editoriales SEO de los servicios/productos del tenant; el administrador del tenant puede gestionarlos en la aplicación BUSINESS. PUBLIC es dueño de su proyección/indexación pública y no puede hacer indexable contenido privado o no autorizado. PLATFORM administra SEO del contenido institucional o global ENSAMBRIA que posea, pero no sustituye ni sobrescribe contenido tenant.

PUBLIC registra la atribución del origen de la reserva/compra, sin apropiarse de la regla de reserva de CORE ni de la regla de pedido de BUSINESS. PLATFORM define las políticas comerciales y tarifas de monetización de Marketplace; CORE valida su elegibilidad y registra sus efectos financieros en los pagos/ledger cuando la política haya sido aprobada.

Como alternativas de monetización que requieren decisión y aprobación comercial explícitas, PLATFORM puede evaluar comisiones por adquisición de clientes, publicidad/posicionamiento destacado o planes de posicionamiento. Son opciones por evaluar, no tarifas, cargos ni políticas aprobadas. No se activan ni cobran hasta que PLATFORM documente la modalidad, reglas de atribución, acumulación o sustitución frente a otros cargos, transparencia al usuario y condiciones aplicables, y CORE valide y registre su efecto transaccional. PUBLIC identifica claramente contenido patrocinado y conserva la atribución del canal.

Las reservas iniciadas en Marketplace pueden ofrecer pago anticipado solo cuando el backend propietario confirme disponibilidad, elegibilidad, precio y políticas aplicables, y CORE valide el pago. Los anticipos, cancelaciones y reembolsos deben seguir las políticas aprobadas del servicio/establecimiento y los contratos del proveedor; el Marketplace no inventa términos ni confirma una operación basándose únicamente en información de proyección.

Debe diferenciar:
- Clientes originados directamente por el establecimiento.
- Clientes adquiridos por ENSAMBRIA.
- Clientes procedentes de campañas externas.
- Clientes procedentes de referencias.

No monetizar reservas del Marketplace hasta que PLATFORM haya definido y aprobado las políticas, su acumulación o sustitución frente a otras comisiones, y CORE pueda validar y registrar de manera transaccional su aplicación.

## 4.9 Commerce y comercio electrónico — backend BUSINESS

BUSINESS es propietario de los productos/servicios comerciales, precios, inventario, promociones del establecimiento, carritos, pedidos, checkout, gift cards, paquetes, membresías comercializadas, entregas y devoluciones conforme a sus políticas. Este dominio es distinto del Marketplace: PUBLIC descubre y atribuye; BUSINESS mantiene las operaciones y datos privados del comercio.

Permitir que cada establecimiento venda:
- Productos.
- Bonos.
- Gift cards.
- Membresías.
- Paquetes.
- Servicios.
- Promociones.
- Productos profesionales.

Debe incluir:
- Catálogo.
- Carrito.
- Checkout.
- Presentación de recomendaciones de venta cruzada autorizadas por BUSINESS; validar nuevamente en el momento de la compra precio, elegibilidad, compatibilidad, disponibilidad y existencias. La selección requiere confirmación explícita y queda sujeta al flujo normal de carrito, pedido, pago, autorización y auditoría.
- Inventario.
- Pedidos.
- Pagos.
- Envíos cuando corresponda.
- Recogida en establecimiento.
- Historial.
- Devoluciones autorizadas.
- Promociones.
- Dominios personalizados.

CORE procesa los pagos y reembolsos autorizados solicitados por BUSINESS y conserva los cargos transaccionales y el ledger. PLATFORM define las políticas de comisiones de ENSAMBRIA; BUSINESS no las recalcula ni modifica. Las membresías que representan derechos, saldos, renovaciones y consumos pertenecen al dominio de fidelización de BUSINESS, aunque Commerce gestione su venta.

## 4.10 API e integraciones

Diseñar un ecosistema abierto con:
- API pública.
- API privada.
- Webhooks.
- Credenciales por integración.
- Scopes.
- Rate limiting.
- Versionado.
- Registro de llamadas.
- Idempotencia.
- Revocación.
- Rotación de secretos.
- Documentación.
- Sandbox controlado para desarrolladores externos.

Preparar integraciones futuras con sistemas contables, plataformas comerciales, herramientas empresariales y proveedores externos.

---

# 5. ADMINISTRACIÓN GLOBAL SAAS

ENSAMBRIA debe tener un panel administrativo global independiente de los paneles de los establecimientos.

PLATFORM es propietario de la administración global de ENSAMBRIA y se despliega independientemente de CORE, BUSINESS y PUBLIC. Cada backend conserva composición, identidad de servicio, configuración, secretos, permisos, datos y ciclo de despliegue propios. El BFF público es una aplicación independiente que consume PUBLIC por gRPC; el BFF de administración reside inicialmente como módulo dentro de PLATFORM. BUSINESS puede alojar su BFF de operación; CORE no se expone directamente a navegadores ni aplicaciones. Los BFF no son backends de dominio adicionales. El panel global no es solo un rol o una sección del producto de tenant.

Debe permitir:

- Dashboard general.
- Gestión comercial de tenants (sin alta manual de tenants).
- Gestión administrativa de cuentas existentes mediante los contratos de identidad de CORE (sin alta manual de operadores ni clientes de establecimientos).
- Gestión de planes.
- Suscripciones.
- Facturación SaaS.
- Capacidades contratadas.
- Complementos.
- Límites de consumo.
- Periodos de prueba.
- Suspensiones.
- Reactivaciones.
- Pagos.
- Transacciones.
- Conciliaciones.
- Cupones.
- Promociones comerciales.
- Gestión administrativa de operadores, roles y permisos mediante identidad, autenticación y autorización de CORE; PLATFORM no mantiene identidades ni sesiones duplicadas.
- Auditoría.
- Métricas de plataforma.
- Uso de infraestructura.
- Costos por módulo.
- Consumo de IA.
- Consumo de mensajería.
- Gestión de integraciones.
- Gestión de incidencias.
- Configuración global.

La gestión de tenants en esta consola es comercial y administrativa, no un proceso de alta manual de cuentas. El operador registra su tenant mediante el flujo de autoservicio cuyo owner de identidad/tenancy es CORE; el hecho confirmado inicia automáticamente el registro de relación comercial de PLATFORM conforme a ADR-0002. El administrador de PLATFORM no crea manualmente tenants, identidades de operadores ni cuentas de clientes de los establecimientos. La administración global tampoco sustituye el registro de clientes que realizan estos en sus páginas asociadas al tenant.

El catálogo inicial de planes SaaS incluye **Básico, Profesional, Empresarial y Franquicia**. PLATFORM posee la definición/versionado de planes, precios, contratos, suscripciones y facturación SaaS. CORE valida y aplica los entitlements efectivos y ejecuta los pagos/ledger solicitados por PLATFORM. BUSINESS y PUBLIC no administran suscripciones SaaS. El plan comercial Franquicia no equivale al módulo funcional de cadenas de BUSINESS.

Separar completamente la administración del SaaS de la administración de los establecimientos.

Un administrador global no debe tener acceso indiscriminado a información sensible de clientes finales.

El acceso excepcional debe ser autorizado, limitado, justificado y auditado.

---

# 6. PANEL ADMINISTRATIVO DE CADA ESTABLECIMIENTO

Cada tenant debe tener un panel propio con:
- Dashboard.
- Sucursales.
- Usuarios.
- Empleados.
- Servicios.
- Reservas.
- Recursos.
- Clientes, cuando CRM esté contratado.
- Membresías.
- Pagos.
- Caja.
- Cobros a clientes con Wompi.
- Facturación electrónica del establecimiento con Factus.
- Productos.
- Inventario.
- Compras.
- Marketing.
- Analítica.
- Configuración.
- Integraciones.
- Reportes.

La interfaz debe adaptarse a las capacidades contratadas y al vertical del establecimiento.

No mostrar funcionalidades irrelevantes.

El sistema debe permitir una experiencia sencilla para un negocio pequeño y una experiencia avanzada para una empresa con múltiples sucursales.

---

# 7. CALCULADORA DE CAPACIDAD Y RENTABILIDAD

Construir un módulo de planificación empresarial que permita calcular:

- Cantidad de sillas.
- Cantidad de cabinas.
- Cantidad de profesionales.
- Horas de operación.
- Días de trabajo.
- Duración de cada servicio.
- Capacidad diaria.
- Capacidad mensual.
- Ocupación.
- Reservas posibles.
- Ingresos potenciales.
- Costos variables.
- Costos fijos.
- Comisiones.
- Costos de materiales.
- Margen por servicio.
- Punto de equilibrio.
- Rentabilidad.
- Membresías.
- MRR.
- Escenarios de crecimiento.
- Comparación entre sucursales.

Debe permitir simulaciones con distintos niveles de ocupación y personal.

Los resultados deben derivarse de fórmulas documentadas y configurables.

No presentar estimaciones como resultados financieros reales.

---

# 8. APLICACIÓN PARA CLIENTES

Desarrollar una aplicación móvil para clientes.

Tecnología:
- Flutter y Dart para Android e iOS, con interfaces adaptativas para tablets.
- Swift para módulos nativos de iOS cuando sea necesario.

Funcionalidades:
- Registro.
- Inicio de sesión.
- Perfil.
- Ubicación.
- Descubrir establecimientos.
- Buscar servicios.
- Consultar profesionales.
- Consultar disponibilidad.
- Reservar.
- Pagar.
- Reprogramar.
- Cancelar según políticas.
- Membresías.
- Paquetes.
- Gift cards.
- Historial.
- Favoritos.
- Puntos.
- Promociones.
- Notificaciones.
- Reseñas.
- Historial de compras.
- Atención al cliente.

No duplicar toda la lógica del negocio en la aplicación. Las reglas, validaciones y autorizaciones deben residir en el backend.

---

# 9. PORTAL PARA EMPLEADOS

Preparar una experiencia específica para empleados.

Funcionalidades:
- Agenda.
- Reservas asignadas.
- Disponibilidad.
- Horarios.
- Servicios.
- Clientes autorizados.
- Historial operativo.
- Comisiones.
- Metas.
- Bonificaciones.
- Asistencia.
- Turnos.
- Notificaciones.
- Estadísticas individuales.

Cada empleado debe acceder exclusivamente a la información permitida por su rol y establecimiento.

## 9.1 Gestión avanzada de personal y operación

BUSINESS debe soportar la gestión operativa de empleados sin confundir identidad/autenticación —propiedad de CORE— con su relación laboral/operativa:

- Perfiles operativos, servicios habilitados, especialidades y asignación a sucursal; identidad y credenciales siguen bajo los módulos de identidad/autorización.
- Turnos, horarios, asistencia, ausencias, vacaciones y disponibilidad, con permisos y trazabilidad.
- Comisiones configurables por servicio y por venta de productos, y bonificaciones por objetivos.
- Metas por empleado y sucursal, evaluaciones, indicadores y rankings autorizados.
- Asignación de clientes/profesionales basada en disponibilidad, especialidad y carga, con reglas visibles y control del tenant.
- Liquidaciones/comprobantes de conceptos variables y seguimiento de su estado. No se presume transferencia bancaria, cálculo legal de nómina ni cumplimiento laboral/fiscal sin definición y revisión aplicables.
- Portal privado para que cada empleado consulte agenda, reservas asignadas, comisiones, metas y resultados autorizados.

BUSINESS es propietario de las reglas, cálculos, saldos por pagar y reportes de comisiones de empleados; no se confunden con comisiones de pagos de ENSAMBRIA ni con el ledger transaccional de CORE. Si una liquidación requiere ejecutar un pago, BUSINESS solicita la operación a CORE mediante contrato y no escribe en su ledger. Acceso, correcciones, aprobación y exportación deben quedar auditados.

---

# 10. WEB PÚBLICA PERSONALIZADA POR ESTABLECIMIENTO

Cada establecimiento debe poder disponer de una web pública personalizada.

Debe incluir:
- Plantillas.
- Constructor visual.
- Identidad de marca.
- Logotipo.
- Colores.
- Tipografía.
- Fotografías.
- Servicios.
- Profesionales.
- Precios.
- Promociones.
- Reservas.
- Productos.
- Membresías.
- Información comercial.
- Ubicación.
- Horarios.
- Contacto.
- SEO local.
- SEO técnico y editorial completo de cada página pública: metadatos por ruta, canonical, Open Graph/social, sitemap, robots, datos estructurados y SEO local.
- Dominio propio.
- Subdominio ENSAMBRIA.
- Certificados TLS.

Los dominios personalizados deben tener verificación de propiedad, configuración DNS, validación y renovación de certificados.

Evitar que la personalización de una web permita ejecutar código arbitrario o comprometer otros tenants.

Cada producto/servicio gestionado desde la administración global o desde el panel del tenant debe tener campos SEO propios cuando pueda publicarse: slug estable y único en su ámbito, title, description, imagen pública autorizada con texto alternativo, canonical y datos estructurados aplicables. BUSINESS conserva ownership del catálogo del tenant; PLATFORM solo administra contenido/catálogos globales de ENSAMBRIA si se aprueban explícitamente como capacidad del producto. Si un administrador global modera o ayuda a editar contenido tenant, actúa con permiso delegado y auditoría, sin apropiarse del catálogo. La administración ofrece validación y vista previa sin exponer borradores; publicación y despublicación respetan el backend propietario, permisos y entitlements. Estos campos editoriales no cambian precios, inventario, reglas de servicio ni autoridad del catálogo.

SEO no significa indexar todo: paneles administrativos, portales de empleados, páginas autenticadas, borradores, contenido privado y datos de clientes/reservas deben usar `noindex`/`nofollow` según corresponda, no aparecer en sitemap y no filtrarse en datos estructurados, previews o metadatos. Aplicaciones móviles nativas cumplen los metadatos de tienda y enlaces de app (ASO/deep links), no se tratan como páginas rastreables.

La arquitectura y criterios SEO se fijan en Fase 0 según [ENSAMBRIA_Open_Graph_SEO_Metadata.md](rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md), y la capacidad técnica base de renderizado rastreable y metadatos Open Graph/SEO se implementa desde el primer corte ejecutable de Fase 1. Desde Fase 1 en adelante, cada fase implementa y prueba SEO completo junto con toda página pública o contenido indexable que introduzca o modifique; no se aplaza una superficie ya expuesta a Fase 5 o Fase 9. Esas fases siguen siendo dueñas de sus superficies planeadas si no se publicaron antes. Debe probarse SSR/prerender cuando aplique, metadata única/localizada en HTML inicial, Open Graph/Twitter Cards, canonical/dominio tenant, sitemap/robots, datos estructurados válidos, enlaces/estados de página y exclusión de contenido privado.

---

# 11. FACTURACIÓN Y PAGOS

## 11.1 Pagos con Wompi

CORE es el único propietario del procesamiento de pagos y del registro transaccional financiero. Atiende operaciones iniciadas por PLATFORM (suscripciones SaaS) y BUSINESS (ventas, servicios y reservas de establecimientos), siempre con contextos de cuenta, titularidad, credenciales, callbacks e idempotencia separados. PLATFORM conserva la regla comercial y facturación SaaS; BUSINESS conserva pedidos, caja y facturación comercial tenant.

Verificar la documentación oficial vigente y las capacidades disponibles antes de integrar; no asumir soporte para operaciones que no estén documentadas. Implementar adapters y configuración separados por backend/dominio, con credenciales, cuentas/conexiones, callbacks, auditoría, idempotencia, estados y registros financieros aislados. Una operación de tenant nunca puede modificar, consultar ni liquidar como una operación de ENSAMBRIA y viceversa. No acoplar las reglas de negocio al SDK ni al modelo de Wompi.

La titularidad de las cuentas de liquidación y el modelo de credenciales para los cobros de los tenants se definirá según los productos/condiciones de Wompi vigentes; no se presume que los tenants compartan la cuenta de ENSAMBRIA.

## 11.2 Facturación electrónica con Factus

Usar Factus en dos bounded contexts separados: PLATFORM emite los documentos correspondientes a obligaciones de ENSAMBRIA; BUSINESS emite las facturas electrónicas de cada establecimiento. La integración de un establecimiento no reutiliza la configuración, credenciales, callbacks, documentos ni historial fiscal de ENSAMBRIA.

Debe contemplar:
- Credenciales/conexiones, configuración tributaria, datos del emisor y resoluciones independientes por dueño y emisor.
- Facturas, notas crédito y notas débito cuando correspondan.
- Estados, referencias externas, documentos, eventos, reintentos seguros y trazabilidad.
- Cumplimiento de los requisitos DIAN aplicables al emisor.

Verificar documentación oficial, responsabilidades y capacidades de Factus para cada ámbito; no inventar capacidades ni reutilizar las credenciales, callbacks, datos o estados de otra integración.

La facturación electrónica debe ser configurable por país y jurisdicción para facilitar futuras expansiones.

## 11.3 Separación financiera

Separar estrictamente:
1. Los cobros de ENSAMBRIA a los establecimientos por el uso del SaaS.
2. Los cobros de los establecimientos a sus clientes.
3. Las comisiones que ENSAMBRIA pueda recibir por marketplace o intermediación.
4. Los reembolsos y ajustes correspondientes.

La separación contable y de ownership es:
- **ENSAMBRIA SaaS:** PLATFORM es dueño de planes, suscripciones, obligaciones y facturación SaaS; CORE procesa el pago y registra su efecto financiero.
- **ENSAMBRIA Payments:** PLATFORM define las políticas comerciales de comisión; CORE valida la política aplicable y registra los cargos/comisiones en el ledger. El costo cobrado por la pasarela es un cargo externo, no ingreso de ENSAMBRIA, y se registra separado según el contrato/liquidación verificados.
- **ENSAMBRIA Marketplace:** PLATFORM define la política comercial; PUBLIC es dueño de la atribución de canal; CORE valida y registra el efecto financiero de una política aprobada. No se ha aprobado una tarifa ni se presume si se acumula con otra.
- **Ingresos del establecimiento:** BUSINESS es dueño de catálogo, pedidos, ventas, costos, devoluciones comerciales, comisiones de empleados y reportes del negocio. CORE procesa pagos y conserva los registros transaccionales sin apropiarse de las reglas comerciales del establecimiento.

No mezclar saldos ni titularidades. Aunque Wompi y Factus sean proveedores comunes, cada contexto propietario conserva sus adapters/composition roots, cuentas/conexiones, credenciales, callbacks, permisos, schemas, registros fiscales/financieros, auditoría y ciclos de vida. CORE es dueño de los pagos y ledger; PLATFORM y BUSINESS son dueños de sus obligaciones de facturación respectivas. No se comparten tokens, webhooks entrantes ni referencias de operación entre ENSAMBRIA y los establecimientos.

---

# 12. WHATSAPP BUSINESS Y NOTIFICACIONES

Integrar WhatsApp Business Cloud API de forma multi-tenant.

Cada establecimiento debe poder configurar su propia integración según las capacidades permitidas por Meta.

Debe incluir:
- Configuración por tenant.
- Credenciales protegidas.
- Números.
- Verificación.
- Plantillas.
- Consentimientos.
- Preferencias.
- Notificaciones.
- Confirmaciones.
- Recordatorios.
- Cancelaciones.
- Reprogramaciones.
- Campañas autorizadas.
- Webhooks.
- Colas.
- Reintentos.
- Control de límites.
- Registro de entregas.
- Manejo de errores.

Separar la integración técnica de la lógica de negocio.

Preparar proveedores alternativos de comunicación sin acoplar todo el sistema a WhatsApp.

---

# 13. EXPERIENCIA DE USUARIO Y DISEÑO

ENSAMBRIA debe tener una identidad visual premium, moderna y profesional.

No utilizar una interfaz genérica de administración.

Diseñar una experiencia coherente para:
- Administración global.
- Propietarios.
- Gerentes.
- Empleados.
- Clientes.
- Franquicias.

Características:
- Diseño responsive.
- Accesibilidad.
- Navegación intuitiva.
- Componentes reutilizables.
- Diseño consistente.
- Temas.
- Estados de carga.
- Estados vacíos.
- Errores comprensibles.
- Confirmaciones.
- Formularios robustos.
- Tablas avanzadas.
- Filtros.
- Búsqueda.
- Gráficos.
- Calendarios.
- Paneles adaptables.

Utilizar Next.js + TypeScript para la web.

Utilizar Flutter + Dart en ENSAMBRIA Business y ENSAMBRIA App. Swift se reserva para capacidades nativas específicas de iOS cuando corresponda. React Native no forma parte del stack oficial adoptado.

Mantener contratos tipados entre frontend y backend.

---

# 14. ARQUITECTURA Y TECNOLOGÍAS

## Backend

Rust como lenguaje principal.

Tecnologías a evaluar:
- Tokio.
- Axum.
- SQLx.
- Serde.
- Tonic para gRPC cuando corresponda.
- OpenTelemetry.
- Tracing.

Seleccionar dependencias estables y justificadas.

## Infraestructura técnica compartida

Las capacidades transversales se implementan como bibliotecas Rust neutrales reutilizables por los módulos y procesos que las necesiten: PostgreSQL/SQLx y configuración de pools, transacciones/contexto autenticado para RLS, Tonic/Protobuf, TLS y seguridad de servicio, errores técnicos, observabilidad, Redis con TTL, NATS Core, JetStream/Outbox y cliente de secretos/certificados de Vault. Su granularidad (módulos internos o crates separados) se decide por cohesión, ownership, dependencias, versionado y pruebas; no se crea un crate omnisciente ni un crate por responsabilidad sin necesidad.

Cada proceso crea y configura sus propias conexiones, identidad y ciclo de vida mediante su composition root. Un backend comparte un único pool entre los módulos co-residentes en ese proceso; procesos independientes (incluidos los cuatro backends, el BFF público y workers separados) no comparten pools en memoria. Los módulos de dominio conservan sus repositorios, SQL, schemas, migraciones, contratos, errores de negocio y autorización. La infraestructura común solo proporciona mecanismos técnicos. El contexto para RLS procede de identidad verificada, tiene alcance de transacción y no puede quedar en una conexión reutilizada del pool. Cada bounded context es propietario de sus contratos Protobuf versionados; no se comparte un proto monolítico ni entidades/DTOs de dominio.

## Topología de cuatro backends y ownership

ENSAMBRIA se implementará con cuatro backends independientes, organizados como monolitos modulares. **Cada funcionalidad tiene un único backend propietario de sus reglas de negocio y estado autoritativo**; los demás solo consumen contratos/eventos autorizados y nunca acceden a sus tablas.

| Backend | Responsabilidades propietarias | Límites explícitos |
|---|---|---|
| **CORE (`backend-core`)** | Identidad/autenticación, tenancy y contexto de seguridad —incluidos el contexto de producto validado y la asociación técnica tenant–producto—, organizaciones/sucursales centrales, evaluación runtime de entitlements, reservas, procesamiento de pagos, comisiones transaccionales aprobadas, ledger inmutable, liquidaciones y conciliación | No administra el catálogo/registro comercial de productos SaaS, marcas comerciales, Commerce/pedidos ni el descubrimiento Marketplace |
| **BUSINESS (`backend-business`)** | Operaciones de establecimientos, workforce, catálogo, clientes/CRM, membresías/fidelización, inventario, Commerce/carritos/pedidos, marketing comercial, facturación tenant, franquicias/regalías, comisiones de empleados y analítica operativa | No es dueño de identidad/contexto tenant, pagos/ledger, políticas de monetización ENSAMBRIA ni directorio Marketplace |
| **PLATFORM (`backend-platform`)** | Administración global, catálogo/registro comercial de productos SaaS y sus marcas/configuración comercial versionada, dominios autorizados y capacidades declaradas, gestión comercial de tenants, planes/contratos/suscripciones y facturación SaaS, políticas comerciales de ENSAMBRIA y tarifas de pagos/Marketplace, soporte y auditoría administrativa | No posee la identidad ni el aislamiento técnico tenant–producto, ni opera reglas/datos privados de establecimientos o la atribución Marketplace |
| **PUBLIC (`backend-public`)** | Directorio, Discovery/Marketplace, páginas y proyecciones públicas autorizadas, reseñas, promociones publicadas y atribución de origen | No es dueño del catálogo privado, servicios, empleados, inventario, pedidos ni reglas operativas de BUSINESS |

Tenancy técnica y gestión comercial de tenants no son la misma responsabilidad: CORE es dueño de identidad, contexto y aislamiento; PLATFORM administra la relación comercial; BUSINESS aplica el contexto en sus operaciones; PUBLIC consume solo información cuya publicación autorizó su propietario. CORE es dueño de reservas; BUSINESS provee los servicios/reglas del establecimiento y PUBLIC puede iniciar una reserva sin apropiarse de su estado. BUSINESS es dueño de Commerce y pedidos, PUBLIC del canal de descubrimiento y atribución. PLATFORM define las políticas de comisiones de ENSAMBRIA/Marketplace; CORE valida su vigencia/elegibilidad y registra su efecto transaccional. BUSINESS conserva comisiones laborales y regalías de franquicia.

Los cuatro backends comparten el repositorio/workspace de build, pero no procesos, composición, identidades, secretos, permisos ni despliegues. Cada uno tiene sus propios pools y ownership de schemas/migraciones; una base PostgreSQL física compartida no es un canal de comunicación. Ningún frontend accede directamente a un backend de dominio, gRPC interno o infraestructura.

Frontends y BFF:
- El ecosistema contiene seis proyectos de interfaz: ENSAMBRIA Business (Flutter), ENSAMBRIA Tenant Portal (Next.js), ENSAMBRIA App (Flutter), ENSAMBRIA Marketplace (Next.js), ENSAMBRIA Platform Admin (Next.js) y ENSAMBRIA Corporate (Next.js). Son clientes independientes de la misma plataforma, no backends adicionales. Sus responsabilidades, plataformas y contratos se formalizan en la matriz de aplicaciones de esta sección y en [ADR-0006](adr/ADR-0006-arquitectura-multiplataforma-aplicaciones.md).
- ENSAMBRIA Business y Tenant Portal administran los mismos tenants y casos de uso empresariales autorizados. Usan los mismos contratos de backend y políticas de autorización; la paridad se mide por capacidad y permisos acordados, no por identidad visual. Ninguna regla de negocio ni autorización se delega al cliente.
- Las seis experiencias tienen identidad, navegación, rutas, compilación/versionado y despliegue apropiados a su tecnología y propósito. No se crea un backend por aplicación ni se permite acceso frontend directo a módulos, datos o infraestructura.
- Cada producto implementado tiene un BFF/API REST/OpenAPI especializado, como aplicación y unidad de despliegue independiente, para sus experiencias autorizadas. No se integra el BFF de producto en un backend de dominio ni se crea un backend de dominio por producto. El BFF público también es independiente y llama a PUBLIC por gRPC; el BFF de administración permanece inicialmente integrado en PLATFORM. CORE no se expone directamente a frontends.
- ENSAMBRIA Business y Tenant Portal usan el BFF del producto y las mismas capacidades empresariales. ENSAMBRIA App consume el BFF del producto para operaciones de cliente y puede consumir el BFF público para Discovery según el caso. Marketplace usa el BFF público; Platform Admin usa el BFF integrado de PLATFORM. Corporate publica contenido institucional estático o consume únicamente una superficie pública autorizada; cualquier dato dinámico requiere contrato, owner y controles definidos antes de implementarse.
- Ningún frontend accede directamente a stores ni a gRPC interno. El contexto producto/tenant se resuelve en una frontera confiable y se verifica en CORE y en el owner de cada dato; branding y routing no son autorización. Los clientes comparten contratos de transporte tipados, no modelos internos ni entidades de dominio.

La comunicación entre backends es solo cuando un caso de uso la necesita y se selecciona por semántica:

- **gRPC** para consultas o comandos síncronos que requieren una respuesta inmediata.
- **Outbox transaccional + NATS JetStream** para hechos de negocio persistidos que requieren entrega durable, procesamiento asíncrono, reintento o replay.
- **NATS Core** para fan-out realtime efímero entre réplicas; las señales perdidas se reparan mediante cursor/snapshot autorizado, no mediante Redis.

Los contratos deben estar autenticados, autorizados y versionados. Los consumidores de eventos deben ser idempotentes. No se usa JetStream como RPC, gRPC como reemplazo de eventos durables, ni se genera tráfico entre backends sin necesidad funcional.

### Capacidades tecnológicas transversales: definición desde Fase 0

Las diez capacidades de la matriz deben tener alcance, terminología, ownership, límites, contratos, decisiones pendientes, dependencias, secuencia de implementación y criterios de aceptación definidos en la Fase 0. Esta fase es documental: no crea runtime, esquemas activos, migraciones, APIs, eventos, webhooks ni catálogo ejecutable de extensiones. Diseñar una capacidad no equivale a habilitarla ni a aprobar su política comercial o legal.

Los recorridos de registro e inicio de sesión forman parte del diseño C01 multi-tenant: operadores y clientes se registran por autoservicio en páginas del producto; CORE posee identidad/tenancy y BUSINESS la ficha CRM del cliente. Cada frontend de producto usa su BFF independiente, que invoca por contrato los owners autorizados; Discovery usa BFF público independiente y administración global el BFF integrado en PLATFORM. Ningún frontend accede directamente a CORE y ningún BFF accede a stores. Cada recorrido conserva la autorización del owner y no infiere permisos desde identidad, marca, dominio o configuración. PLATFORM recibe automáticamente el hecho durable de alta de tenant y su producto asociado; sus administradores no registran manualmente tenants ni clientes finales. El contrato de coordinación y recuperación del registro de cliente entre CORE y BUSINESS se define y acepta con sus pruebas negativas antes del corte de implementación; no se permite alta administrativa como alternativa. Los avisos/documentos versionados, aceptación contractual cuando corresponda, evidencia y matriz de rutas también se completan antes de ese corte.

La Fase 1 implementa únicamente los cimientos compartidos requeridos para sus recorridos reales, incluido el aislamiento/autorización base de CORE y Outbox/Inbox desde el primer flujo real que necesite integración durable. Cada fase posterior implementa el comportamiento completo en el backend propietario desde el primer caso de uso que lo necesite; no puede diferirse un control necesario a una fase de madurez. Las capacidades transversales no justifican un backend adicional, una base de datos separada, un motor central con reglas de todos los dominios ni ejecución de código de terceros dentro de los backends.

### Arquitectura de productos y verticales desde Fase 0

La estrategia multiproducto es una decisión arquitectónica adicional a las diez capacidades tecnológicas de C01–C10; no cambia su número ni las sustituye. Fase 0 documenta la matriz producto–industria–vertical–capacidad–frontend/BFF–backend owner–contrato–fase–aceptación y los límites de composición en [docs/arquitectura-dominios.md](arquitectura-dominios.md). No crea catálogo ejecutable, frontends, BFFs, carpetas ni productos runtime.

PLATFORM es la autoridad del catálogo/configuración comercial versionada de producto. CORE recibe/valida el identificador de producto mediante un contrato confiable y mantiene la relación técnica tenant–producto; el dominio y los headers del cliente no la establecen. El flujo de registro de tenant sigue siendo autoservicio y CORE→PLATFORM conserva Outbox/JetStream/Inbox idempotente. El hecho de alta lleva `tenant_id` y la referencia canónica `product_id` para asociar el onboarding pendiente con el producto correspondiente, sin asignar plan, precio, cobro, trial, factura, suscripción ni entitlement automáticamente. Los clientes finales también se registran/inician sesión por las superficies del producto y tenant autorizados; no son creados manualmente por administradores PLATFORM.

El producto inicial debe tener identidad/configuración de producto disponible y verificable antes de aceptar altas autoservicio en CORE; esta dependencia se satisface en el orden del roadmap mediante un contrato y una configuración propiedad de PLATFORM, sin adelantar a Fase 0 una implementación. La administración comercial dinámica de productos, el lanzamiento de productos adicionales y la implementación de sus verticales/frontends se entregan en la fase propietaria aprobada. Ningún producto futuro se declara soportado por aparecer en la matriz.

Las capacidades reutilizables mantienen el ownership actual: CRM, workforce, catálogo, inventario, Commerce, membresías, marketing y reglas operativas de vertical pertenecen a BUSINESS; identidad, contexto tenant/producto, autorización, reservas y pagos al CORE según su alcance vigente; catálogo comercial de productos, planes y suscripciones a PLATFORM; Discovery, canal Marketplace y proyecciones expresamente autorizadas a PUBLIC. Dentro de BUSINESS, el diseño objetivo mantiene capacidades comunes y neutrales en `modules/`, y agrupa comportamiento especializado por industria bajo `verticals/<industria>/`, conforme a [§3.5](#35-diseño-de-los-módulos-verticales) y [ADR-0008](adr/ADR-0008-modulos-compartidos-y-verticales-business.md). Los productos declaran referencias/versiones de contratos compartidos; las reglas específicas quedan en su vertical BUSINESS y no se distribuyen mediante condicionales de marca/industria por todos los backends.

| Capacidad | Decisión arquitectónica y ownership | Implementación funcional |
|---|---|---|
| Multi-tenant avanzado | CORE autentica y valida actor, tenant, pertenencia, permisos y contexto; RLS y autorización protegen cada acceso. BUSINESS posee configuración operativa por negocio y overrides autorizados por sucursal. Cada módulo debe aislar sus datos, cachés, índices/búsquedas, objetos, jobs, eventos y telemetría. Un ID recibido no concede acceso; la identidad global y las membresías tenant-scoped son distintas. | Fase 0 define amenazas, matriz de autorización, datos/sistemas derivados y pruebas negativas. Fase 1 implementa y prueba la base de aislamiento/autorización; cada dominio completa y prueba sus fronteras en la fase que introduce el dato o acción. |
| Multi-sucursal | CORE es dueño de las relaciones estructurales organización/sucursal y propaga el scope autenticado autorizado. BUSINESS es dueño de horarios, catálogo operativo, profesionales, recursos, inventario, caja y reglas por sucursal. La herencia de configuración desde organización, los overrides, la disponibilidad compartida y la consolidación requieren reglas explícitas; no se presumen. | Fase 0 define jerarquía, permisos, defaults, herencia/override, visibilidad y escenarios de aislamiento. Fase 1 establece relaciones/contexto; Fase 2 implementa recursos de vertical; Fase 4 workforce, inventario, caja y operación; Fase 10 consolida cadenas/franquicias. |
| Multi-país y multi-moneda | PLATFORM administra mercados habilitados y sus condiciones comerciales; CORE verifica contexto operativo regional y procesa transacciones con moneda explícita; cada backend conserva importes en moneda original. Usar códigos ISO 3166-1/ISO 4217 y zonas IANA. No convertir importes implícitamente ni usar punto flotante; una conversión exige fuente, tasa, instante, precisión y regla aprobados y persistidos. Impuestos y obligaciones por país requieren decisión de Legal/Finanzas. | Fase 0 inventaría mercados iniciales, owners, moneda de cada flujo, zonas, dependencias de proveedores y decisiones legales/comerciales pendientes. Fase 1 define tipos/contratos compatibles; operación internacional y monedas adicionales se habilitan en Fase 10 tras pruebas y aprobaciones aplicables. |
| Multi-idioma | Contratos transportan códigos estables, no texto de interfaz traducido. Normalizar locales con BCP 47 y documentar selección, fallback, contenido sin traducción, fechas/números/monedas y mensajes de error. Español, inglés y portugués son idiomas iniciales. Cada frontend/backend localiza su propia presentación; el owner de contenido controla las traducciones editoriales. | Fase 0 define locales, ownership, fallback, contenido traducible y pruebas. Fase 1 establece compatibilidad de contexto/contratos; cada ruta pública se entrega localizada y probada desde su primera publicación; activación integral e internacionalización en Fase 10. |
| Motor de reglas | No crear un motor central que posea políticas de múltiples dominios ni permitir expresiones/código arbitrario. CORE posee estado/transiciones y aplicación de políticas autorizadas de reserva/pago; BUSINESS posee reglas operativas/comerciales de establecimientos, membresías, inventario y comisiones laborales; PLATFORM posee planes, entitlements y políticas comerciales ENSAMBRIA. Reglas tipadas, acotadas por tenant/sucursal, deterministas, versionadas, auditables, con precedencia y vigencia explícitas. Toda decisión financiera debe poder explicarse con la versión de regla aplicada. | Fase 0 registra catálogo de reglas, propietario, entradas, precedencia, estados, excepciones y conflictos. Fase 1 implementa solo reglas de CORE necesarias; cada regla se entrega con el dominio propietario (reservas desde Fase 1, verticales Fase 2, fidelización Fase 3, comercio/workforce Fase 4, políticas SaaS según PLATFORM). |
| Sistema de eventos | Cada evento representa un hecho persistido y tiene módulo dueño, consumidores y necesidad funcional. El owner persiste el cambio y Outbox en la misma transacción; publica contrato versionado por JetStream con entrega al menos una vez, idempotencia, orden requerido y replay. Eventos efímeros realtime usan NATS Core solo para fan-out; gRPC resuelve consultas/comandos síncronos. No emitir eventos artificiales ni PII/secretos. | Fase 0 entrega catálogo de hechos/consumidores, contratos y decisiones de orden, privacidad, retención y recuperación, sin runtime. Fase 1 implementa Outbox/Inbox en el primer flujo real que lo requiera y la infraestructura aprobada; cada fase incorpora eventos cuando introduce el hecho. |
| Auditoría completa | Cada módulo dueño registra cambios/decisiones de negocio en un historial auditable append-only separado de logs técnicos. El contrato identifica actor autenticado o workload, acción, resultado, instante, recurso, tenant/sucursal/scope y correlation/trace cuando estén disponibles. El origen se obtiene de señales verificadas y minimizadas; no guardar IP cruda ni otros datos de ubicación sin necesidad, control y aprobación. CORE posee eventos de identidad/seguridad; cada backend sus operaciones; PLATFORM su administración global. | Fase 0 define taxonomía, campos, acceso, integridad, retención, privacidad y casos negativos. Fase 1 establece contrato/capacidad base para seguridad; cada fase implementa auditoría en las mutaciones y accesos sensibles de su dominio y prueba redacción, autorización y evidencia. |
| API pública | Cada backend conserva ownership de su operación y contrato externo. PLATFORM administra el registro/relación comercial de integradores y entitlements; CORE proporciona identidad/credenciales técnicas, autenticación y autorización; el gateway/BFF expone solo superficies aprobadas, nunca módulos, DB o gRPC interno directamente. Definir scopes, límites/cuotas, rate limiting, versionado/compatibilidad, idempotencia, errores, deprecación y sandbox aislado. Integraciones contables o corporativas consumen APIs versionadas, no acceso SQL. | Fase 0 define personas integradoras, recursos/scopes, amenazas, datos, límites y política de versión. Fase 1 define cimientos/contratos de confianza; API externa se habilita en Fase 7 con contrato, controles, documentación y pruebas de compatibilidad/abuso. |
| Webhooks | El dominio emisor define hechos, payload mínimo y autorización de suscripciones. Entrega saliente durable, deduplicable y reintentable, con firma verificable, rotación de secretos, límites, replay controlado, redacción y controles de egress/SSRF; el receptor debe tolerar duplicados y validar versión/firma. Webhooks entrantes de proveedores se verifican según documentación oficial y no confían en datos del cliente. | Fase 0 define catálogo de eventos externos, propietarios, suscripción, amenazas, política de reintento/replay, secretos, privacidad y auditoría, sin endpoints activos. Fase 1 prepara garantías internas; entrega entrante/saliente a integradores se implementa en Fase 7 por evento/proveedor con pruebas de firma, duplicados, fallos y reintentos. |
| Marketplace de extensiones | PLATFORM posee catálogo, revisión/publicación, compatibilidad, oferta/suscripción y entitlement. El dominio propietario conserva APIs, permisos, datos y decisiones expuestos a cada extensión. Integraciones externas usan contratos/scopes aislados; prohibido SQL directo o ejecutar código arbitrario dentro de los backends. Deben definirse ciclo de vida, versiones, revocación, límites, auditoría y revisión de seguridad/proveedor. | Fase 0 define modelo de confianza, amenazas, proceso de revisión, límites y criterios de elegibilidad sin catálogo ejecutable. Fase 1 especifica contratos y controles base; Fase 11 implementa catálogo e instalación solo después de validar aislamiento, compatibilidad y operación. |

Ejemplo de integración: un sistema contable o corporativo de una barbería/cadena consume las APIs y/o eventos/webhooks autorizados con scopes acotados, sin conectarse directamente a PostgreSQL, importar implementaciones internas ni apropiarse de reglas de reservas, ventas, pagos o inventario.

## Ecosistema oficial de aplicaciones

Las seis aplicaciones son proyectos de interfaz con responsabilidades diferentes; su existencia en este diseño no implica que estén implementadas, desplegadas ni que se hayan creado seis repositorios. Pueden compartir el monorepo y bibliotecas de presentación neutrales cuando se justifique, sin compartir sesiones, permisos o autoridad de dominio.

| Proyecto | Tecnología | Responsabilidad y usuarios | Plataformas / superficie | Contrato y BFF autorizado |
|---|---|---|---|---|
| **ENSAMBRIA Business** | Flutter + Dart | Operación empresarial para titulares, administradores, empleados, profesionales y personal autorizado. | Android, iOS, tablets Android/iPad, Windows, Linux y macOS; UX adaptativa para móvil y escritorio. | BFF/API del producto; mismas operaciones empresariales y autorización que Tenant Portal. |
| **ENSAMBRIA Tenant Portal** | React + TypeScript + Next.js | Portal web para administrar los mismos tenants que ENSAMBRIA Business, sin instalación. | Navegadores modernos en Windows, Linux, macOS, ChromeOS y sistemas compatibles. | Mismo BFF/API, contratos de backend, permisos y capacidades empresariales que ENSAMBRIA Business. |
| **ENSAMBRIA App** | Flutter + Dart | Experiencia de clientes finales: perfil, descubrimiento y operaciones aprobadas como reservas. No incluye administración de tenants. | Android, iOS y tablets Android/iPad. | BFF/API del producto para operaciones de cliente; BFF público para Discovery cuando corresponda. |
| **ENSAMBRIA Marketplace** | React + TypeScript + Next.js | Experiencia pública para descubrir negocios, profesionales, servicios, sucursales e información publicada. | Web pública, rastreable y optimizada para navegadores. | BFF público independiente; PUBLIC entrega proyecciones autorizadas. |
| **ENSAMBRIA Platform Admin** | React + TypeScript + Next.js | Herramientas privadas para operadores autorizados de la plataforma, soporte y administración global. | Web empresarial restringida. | BFF de administración integrado inicialmente en PLATFORM; autorización por backend owner. |
| **ENSAMBRIA Corporate** | React + TypeScript + Next.js | Presencia institucional/comercial de ENSAMBRIA, productos y contenido público aprobado. No administra tenants. | Sitio web institucional público. | Contenido estático o contrato público autorizado; no acceso directo a dominios ni stores. |

Business y Tenant Portal son dos clientes independientes de una plataforma empresarial: no duplican reglas de negocio, servicios, autenticación, autorización, aislamiento tenant/sucursal ni APIs por razón de interfaz. Cada acción valida permisos en backend, aunque una interfaz oculte su control.

La paridad funcional se acepta por capacidad aprobada, actor/rol, operación, restricciones y resultados observables. No exige igualdad visual, de navegación ni de componentes. Las diferencias justificadas por dispositivo (por ejemplo, tactilidad móvil o productividad de escritorio) no alteran reglas ni permisos.

No se declara compatibilidad Smart TV. Android TV, Google TV, Apple TV, Samsung Tizen, LG webOS y Amazon Fire TV requieren evaluación y decisión específicas antes de entrar al alcance.

## Frontend web

- React + Next.js para Tenant Portal, Marketplace, Platform Admin y Corporate.
- TypeScript estricto.
- Tailwind CSS.
- Sistema de componentes reutilizable.
- Validación de formularios.
- Manejo seguro de sesiones.
- Integración con APIs tipadas.
- SEO técnico completo para cada superficie web pública y cada página pública indexable, incluidos contenido editorial, metadatos por ruta, canonical, robots, sitemap, Open Graph, datos estructurados y soporte multidioma cuando corresponda.
- `noindex` explícito y ausencia de sitemap/datos estructurados públicos para administración, páginas autenticadas, borradores y datos privados.

## Aplicaciones móviles

- Flutter + Dart para ENSAMBRIA Business y ENSAMBRIA App.
- ENSAMBRIA Business también apunta a Windows, Linux y macOS; su experiencia de escritorio debe diseñarse para teclado, ratón, navegación y herramientas administrativas, no como una ampliación de la interfaz móvil.
- ENSAMBRIA App apunta a Android, iOS y tablets; no incorpora capacidades administrativas de tenants.
- Swift para integraciones nativas específicas de iOS.

## Persistencia

- PostgreSQL como base de datos principal.
- Redis exclusivamente para caché efímera con TTL; no se usa para fan-out, coordinación ni como fuente de verdad.
- NATS Core para fan-out realtime efímero entre réplicas; las señales perdidas se recuperan mediante cursor/snapshot autorizado.
- NATS JetStream con Outbox transaccional para eventos duraderos, reintento y replay.
- Almacenamiento de objetos mediante un puerto neutral del módulo propietario del ciclo de vida y autorización del archivo, con Cloudflare R2 como adapter inicial usando su API compatible con S3. El dominio no depende de SDK ni capacidades exclusivas del proveedor, para permitir cambiarlo sin reescribir las reglas de negocio.
- El backend autoriza cargas/descargas de objetos después de validar permisos y emite autorizaciones temporales; el navegador transfiere los bytes directamente al almacenamiento. No se exponen credenciales permanentes.
- gRPC para consultas y comandos síncronos entre módulos/backends cuando se necesita respuesta inmediata.

## Arquitectura de software

Aplicar:
- Domain-Driven Design.
- Clean Architecture.
- Arquitectura Hexagonal.
- Separación de dominios.
- Inversión de dependencias.
- Contratos explícitos.
- Eventos de dominio.
- Outbox Pattern.
- Idempotencia.
- Observabilidad.

## Organización del repositorio

Monorepo modular.

Cada módulo debe tener límites claros y posibilidad de extraerse como servicio independiente en el futuro.

No crear microservicios simplemente por dividir carpetas.

La separación física debe justificarse por escalabilidad, despliegue, aislamiento, autonomía operativa o necesidades técnicas reales.

---

# 15. MODELO DE TENANCY Y SEGURIDAD

La seguridad multi-tenant debe ser una propiedad estructural del sistema.

Implementar:
- PostgreSQL Row-Level Security.
- Contexto de tenant validado.
- Políticas de autorización.
- Roles y permisos.
- Restricciones de integridad.
- Claves foráneas.
- Índices.
- Restricciones únicas por tenant cuando corresponda.
- Aislamiento en caché.
- Aislamiento en almacenamiento.
- Aislamiento en colas y eventos.
- Aislamiento en búsquedas.
- Auditoría.
- Protección y rotación de secretos mediante HashiCorp Vault, con aislamiento de rutas, roles y políticas por proyecto, entorno y workload.
- Emisión/renovación de certificados internos mediante PKI y TLS validado sobre la red privada.
- Rotación de credenciales.
- Rate limiting.
- Protección contra abuso.
- Gestión de sesiones.
- Registro de accesos.
- Redacción de datos sensibles en logs.

Nunca confiar únicamente en el tenant_id enviado por el frontend.

El contexto debe obtenerse de una identidad autenticada y validarse en cada operación.

Los eventos asíncronos deben transportar contexto verificable y no aceptar identidades de productores arbitrarias.

Las tareas internas deben contar con autenticación y autorización entre servicios.

La autorización debe distinguir actor, tenant, organización, sucursal, producto, acción y recurso según corresponda. Ser titular u operador de un tenant no concede permisos globales ni acceso automático a todas las sucursales. La aplicación de este aislamiento al modelo de persona natural, organización operativa, establecimientos opcionales y ubicaciones de atención se especifica en [§2.2](#22-modelo-empresarial-multimodal-y-servicios-móviles); las direcciones privadas y coordenadas se tratan como datos protegidos, nunca como información pública por defecto.

---

# 16. CONCURRENCIA E INTEGRIDAD OPERATIVA

Diseñar correctamente las operaciones críticas.

Especial atención a:
- Reservas simultáneas.
- Asignación de profesionales.
- Asignación de cabinas.
- Asignación de equipos.
- Consumo de membresías.
- Inventario.
- Pagos.
- Renovaciones.
- Comisiones.
- Facturación.
- Cancelaciones.
- Reembolsos.
- Webhooks.
- Procesamiento de eventos.

Implementar:
- Transacciones.
- Restricciones de base de datos.
- Idempotencia.
- Bloqueos apropiados.
- Control de concurrencia.
- Outbox.
- Inbox idempotente.
- Reintentos seguros.
- Manejo de estados inciertos.
- Reconciliación.

No asumir que una operación es correcta porque el frontend impide realizarla dos veces.

---

# 17. OBSERVABILIDAD Y OPERACIÓN

La arquitectura, instrumentación, controles de datos, monitoreo, alertas, SLI/SLO, pruebas y operación se rigen por [ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md](rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md).

La Fase 0 documenta inventario, ownership, arquitectura, responsables, decisiones, controles de telemetría, SLI/SLO candidatos y criterios de aceptación; no crea runtime, instrumentación ni infraestructura activa. Desde el primer corte ejecutable de Fase 1, cada entrega implementa y prueba la observabilidad pertinente a sus servicios y comportamientos. No se inventan destinos de alertas, credenciales, umbrales, objetivos ni métricas con cardinalidad ilimitada.

---

# 18. PROTECCIÓN DE DATOS Y CUMPLIMIENTO

Diseñar ENSAMBRIA contemplando Colombia como mercado inicial, pero sin limitar la arquitectura a ese país.

Investigar en fuentes oficiales y redactar durante el desarrollo la documentación legal, contractual y de privacidad necesaria para las funciones reales del producto. La preparación e integración técnica de documentos, versiones y mecanismos de transparencia no esperan la revisión de un abogado. La revisión profesional es posterior a la implementación técnica; la falta de esa revisión no detiene documentación ni trabajo independiente y no equivale a aprobación jurídica. Toda pregunta crítica se registra con fuente, tratamiento, responsable y evidencia requerida; solo queda bloqueado el tratamiento o mercado que dependa de una validación previa concreta.

Cada página legal y punto de interacción pertinente debe presentar el aviso visible, accesible y no obstructivo de revisión jurídica pendiente mientras no exista aprobación profesional documentada:

> **Aviso sobre revisión jurídica**
>
> La documentación legal de ENSAMBRIA se encuentra implementada y disponible, pero está pendiente de revisión por un profesional del derecho. Su contenido podrá actualizarse como resultado de dicha revisión.

El texto no se muestra antes de que la documentación esté efectivamente implementada y disponible en esa superficie. No declarar aprobación legal ni certificación de cumplimiento por la existencia de documentos, aceptación de usuarios o pruebas técnicas.

Evaluar:
- Ley 1581 de 2012.
- Reglamentación colombiana aplicable.
- Obligaciones de protección de datos.
- Derechos de los titulares.
- Autorizaciones.
- Tratamiento de datos sensibles.
- Retención.
- Eliminación.
- Portabilidad cuando corresponda.
- Contratos de tratamiento.
- Facturación DIAN.
- Requisitos sanitarios para los verticales aplicables.
- Condiciones de proveedores de pagos.
- Políticas de Meta.

Preparar una arquitectura adaptable a otras jurisdicciones.

No presentar una decisión técnica como aprobación legal.

La Fase 0 prepara la investigación, los borradores completos conforme a los hechos disponibles, la fuente documental canónica y los contratos/criterios de integración; continúa sin runtime de producto. Las rutas, avisos, versiones, consentimientos y evidencias se implementan y prueban en la fase que introduzca cada recorrido. La revisión profesional posterior no es una condición general para esa implementación técnica. Los hechos comerciales o de entidad desconocidos no deben inventarse; se registran y bloquean solo las decisiones o tratamientos que dependan de ellos.

Los sorteos, concursos, premios y promociones sujetas a regulación deben permanecer desactivados hasta contar con revisión jurídica y autorización aplicable.

---

# 19. SORTEOS, REFERIDOS Y FIDELIZACIÓN

Incluir como capacidad futura:
- Sorteos trimestrales.
- Programas de referidos.
- Premios.
- Beneficios.
- Campañas.
- Puntos.
- Recompensas.

Deben contar con:
- Reglas configurables.
- Elegibilidad.
- Registro de participación.
- Auditoría.
- Transparencia.
- Prevención de fraude.
- Condiciones publicadas.
- Revisión legal.
- Activación controlada.

No activar sorteos comerciales automáticamente sin aprobación legal.

---

# 20. REQUISITOS DE CALIDAD DEL DESARROLLO

Estas condiciones son obligatorias.

1. No entregar funcionalidades simuladas.
2. No utilizar mocks o fakes como sustitutos de implementaciones productivas.
3. No dejar TODOs como cumplimiento de requisitos.
4. No implementar endpoints ficticios.
5. No utilizar `any` para eludir el sistema de tipos.
6. No debilitar validaciones.
7. No eliminar pruebas para hacer pasar CI.
8. No ocultar errores.
9. No ignorar advertencias relevantes.
10. No implementar lógica crítica exclusivamente en frontend.
11. No mezclar responsabilidades entre dominios.
12. No inventar capacidades de proveedores.
13. No introducir dependencias sin justificación.
14. No dejar documentación desactualizada.
15. No marcar funcionalidades como terminadas sin evidencia verificable.

Para Rust:
- Preferir archivos de menos de 200 líneas.
- Ningún archivo debe superar 400 líneas salvo excepción técnicamente justificada y documentada.
- Aplicar formato.
- Clippy sin warnings.
- Pruebas completas.
- Tipos explícitos.
- Errores tipados.
- Seguridad de concurrencia.

Para TypeScript:
- Strict mode.
- Sin `any` injustificados.
- Validación de contratos.
- Lint.
- Typecheck.
- Pruebas.
- Accesibilidad.

---

# 21. ESTRATEGIA DE PRUEBAS

Cada caso de uso debe probarse con escenarios positivos y negativos.

Incluir:
- Validaciones.
- Datos inválidos.
- Tipos incorrectos.
- Duplicados.
- Concurrencia.
- Permisos.
- Aislamiento entre tenants.
- Estados inválidos.
- Transiciones.
- Fallos.
- Reintentos.
- Integridad.
- Idempotencia.
- Redacción de datos sensibles.
- Rollback.
- Persistencia.
- Recuperación.

Pruebas obligatorias:
- Unitarias.
- Integración.
- Contratos.
- API.
- Base de datos real.
- RLS.
- Concurrencia.
- Seguridad.
- End-to-end.
- Multi-tenant.
- Multi-sucursal.
- Frontend.
- Aplicaciones móviles.
- Integraciones externas según su disponibilidad.

No sustituir pruebas reales de persistencia por mocks.

Registrar la trazabilidad:

REQUISITO → CASO DE USO → PRUEBA → EJECUCIÓN CI → EVIDENCIA.

Desde Fase 0, cada check obligatorio de GitHub Actions debe poder ejecutarse localmente con el mismo comando, configuración y criterio de aprobación. CI invoca el mismo punto de entrada versionado que el desarrollador; scripts no pueden ocultar fallos ni omitir controles. Los comandos se inventarían a partir de manifests y herramientas reales, y se documentan por tipo de suite, dependencias requeridas, preparación/limpieza y evidencia. Los controles Rust/TypeScript sugeridos son candidatos hasta verificarse contra el workspace y los scripts existentes.

Cada PR ejecuta los controles aplicables de formato/lint, build, unitarias, integración con dependencias reales, contratos/versionado, seguridad y E2E/frontends cuando corresponda. Rendimiento, resiliencia y recuperación se ejecutan según riesgo y etapa, con entornos reproducibles. Un check requerido fallido, pendiente, cancelado u omitido no cuenta como aprobado. Las verificaciones críticas de aislamiento tenant/sucursal, autorización, idempotencia financiera, duplicación de eventos, transiciones de estado, auditoría y redacción deben ser automatizadas y trazables al caso de uso.

---

# 22. ROADMAP DE IMPLEMENTACIÓN

La implementación debe realizarse por fases completas.

No avanzar a una fase posterior dejando incompletos los criterios obligatorios de la fase actual.

## 22.1 Secuencia y subfases

La secuencia, los IDs de subfase, sus dependencias y la trazabilidad con los ADR se mantienen en el [ROADMAP.md](../ROADMAP.md). Esta sección del requerimiento conserva el alcance funcional detallado de cada fase; §23 define su aceptación. El roadmap complementa esos requisitos, no los reemplaza ni los declara implementados.

## FASE 0 — Descubrimiento y arquitectura

Los estándares y reglas obligatorias de [`docs/rules/`](rules/) son requisitos de aceptación de ENSAMBRIA, no lecturas opcionales ni funcionalidades que deban implementarse todas simultáneamente. Esta sección define la secuencia del producto; cada fuente normativa define los controles de su especialidad. En Fase 0 se documentan y trazan los requisitos aplicables; durante la implementación se implementa y prueba cada control en la primera fase/subfase que introduce su comportamiento, sin aplazarlo a una fase posterior. La matriz de Fase 0 debe enlazar requisito de producto → regla normativa aplicable → fase/owner → control → caso y prueba → evidencia. Si no aplica, se justifica antes de implementar el alcance afectado.

- Inspección completa del repositorio.
- Inventario de tecnologías.
- Análisis de requisitos.
- Modelo de dominios.
- Contextos delimitados.
- Dependencias entre módulos.
- Matriz de propiedad única de los cuatro backends y contratos entre dominios, incluidas las fronteras entre CORE, BUSINESS, PLATFORM y PUBLIC.
- Diseño documental multiproducto/multivertical separado de C01–C10: modelo de producto/industria/vertical/marca, catálogo comercial y owner PLATFORM, asociación técnica tenant–producto en CORE, composición de capacidades, matrices de candidatos, frontends/BFF, contratos, dominios, aislamiento, despliegue, dependencias, fase y aceptación. No crear runtime ni implementaciones ficticias en Fase 0.
- Matriz de arquitectura y trazabilidad para las diez capacidades transversales de la sección 14, indicando para cada una: alcance y términos, propietario técnico y de producto, actores/datos, límites de confianza, integraciones y contratos requeridos, dependencias, amenazas/controles, decisiones pendientes y aprobadores, N/A justificados, fase de cada comportamiento, pruebas y evidencia de aceptación. Mantener el detalle de producto en la sección 14; la matriz de Fase 0 puede referenciarla, no duplicarla.
- Definir explícitamente para multitenancy y multi-sucursal la jerarquía tenant/organización/sucursal, pertenencia, autorización, herencia/override y escenarios de aislamiento; para multi-país/moneda/idioma los mercados/locale iniciales, owners, formato de importes, zona horaria, fallback de localización, condiciones de activación y bloqueos legales/comerciales.
- Diseñar por separado el modelo empresarial multimodal de [§2.2](#22-modelo-empresarial-multimodal-y-servicios-móviles): personas naturales/jurídicas, titularidad, tenant estable, organización operativa, sucursal, establecimiento físico opcional, ubicaciones de atención, modalidades de servicio, onboarding progresivo, workforce, CRM, cobertura, desplazamiento y aceptación por fase. Mantener el ownership de los cuatro backends, registrar las decisiones en [ADR-0005](adr/ADR-0005-tenancy-multimodal-y-servicios-moviles.md) y no crear runtime, entidades físicas ni datos ficticios en Fase 0.
- Definir los límites entre reglas tipadas por dominio y cualquier capacidad técnica común; catálogo inicial de hechos y consumidores para eventos; contrato/ciclo de vida de auditoría; modelo de confianza de API pública y webhooks; y modelo de revisión, permisos, compatibilidad y revocación del Marketplace de extensiones. Documentar la integración de sistemas contables/corporativos mediante contratos autorizados, nunca acceso directo a tablas.
- Acordar la matriz requisito→capacidad→dominio/backend owner→decisión/contrato→control→caso de uso/prueba→fase→responsable→evidencia. Cada decisión no aprobada permanece marcada como pendiente, con responsable y alcance bloqueado; la documentación no aprueba por sí sola precios, políticas comerciales, términos de proveedor ni obligaciones legales.
- Modelo de tenancy.
- Política de identificadores: UUID v7 canónico, autoridad de generación por entidad y preservación de IDs en los contratos entre backends; separación de identidad global y membresías por tenant; excepciones de `BIGINT`, IDs de presentación y UUID v4 sustentadas por evidencia/justificación. Verificar las versiones reales de PostgreSQL antes de considerar generación nativa.
- Revisión obligatoria de [ENSAMBRIA_Connection_Pooling_PostgreSQL.md](rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md): aprobar la topología lógica por backend/proceso/workload, roles y credenciales, número máximo de réplicas, presupuestos de conexiones y margen, timeouts, contexto RLS transaccional, métricas/alertas y estrategia de pruebas con PostgreSQL real. Los números de ejemplo no son valores de producción; PgBouncer solo se incorpora si mediciones y pruebas lo justifican.
- Revisión obligatoria de [ENSAMBRIA_Transactional_Outbox_Inbox.md](rules/ENSAMBRIA_Transactional_Outbox_Inbox.md): inventariar hechos persistidos que requieren integración durable por dominio y definir owner, contrato/versionado, atomicidad local, broker/confirmación, estrategia de publicación y recuperación, Inbox/idempotencia, orden, privacidad, retención/replay, observabilidad y pruebas. Justificar cada evento/canal N/A por caso de uso; no crear mensajería sin consumidor/necesidad verificable. Esta planificación es documental y no introduce runtime en Fase 0.
- Revisión y planificación obligatoria de [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md): designación de responsables técnicos y de privacidad/Legal; inventario inicial de datos, propósitos, titulares, propietarios, ubicaciones, destinatarios y sistemas derivados; jurisdicciones y validación legal; análisis de amenazas; matriz requisito→dato/dominio→control→prueba→responsable→evidencia; contratos PDLM y ADRs necesarios antes de implementar. Las propuestas del documento, incluidos PII Vault, particionamiento, claves/KMS, crypto-shredding y celdas regionales, requieren decisión explícita y no se consideran aprobadas por estar documentadas.
- Revisión obligatoria de [ENSAMBRIA_Privacy_Legal_Compliance.md](rules/ENSAMBRIA_Privacy_Legal_Compliance.md) junto con PDLM: registrar por jurisdicción y tratamiento fuentes oficiales, alcance/aplicabilidad, responsables/encargados, bases/consentimientos cuando correspondan, obligaciones de consumidores/menores, cookies/marketing, proveedores/transferencias, retención, incidentes y propiedad intelectual; asignar Privacy/Legal Owner y responsables de producto/técnicos; trazar requisito→control→prueba→evidencia y marcar decisiones pendientes como bloqueos. El documento no es asesoría legal: no declarar una obligación o plazo validado sin aprobación de Legal.
- Revisión obligatoria de [ENSAMBRIA_Geographic_Access_Policies.md](rules/ENSAMBRIA_Geographic_Access_Policies.md): mapear navegación pública, solicitudes de lanzamiento y privacidad, registro y operaciones comerciales por mercado/capacidad. Definir propietarios, motivos, evidencia y pruebas de cada restricción; documentar precisión/limitaciones de señales IP, verificación adicional para bloqueos estrictos por ciudad y canales para corregir errores. Esta definición es documental en Fase 0 y no activa geobloqueos.
- Registro de límites del PDLM: consentimiento/cookies, aviso de privacidad y requerimientos de autoridades necesitan procedimientos y responsables propios, coordinados con el ciclo de vida de datos; no se omiten por estar fuera del alcance del documento especializado.
- Definición del catálogo de datos versionable y de sus reglas de validación CI. En Fase 0 se entrega el inventario/diseño estático; el gate ejecutable se activa junto con los primeros esquemas/migraciones de implementación y no se reporta como runtime o catálogo activo antes de ello.
- Estrategia SEO de todas las superficies: inventario de páginas públicas/privadas por frontend, propietarios de contenido y publicación, dominios tenant, idiomas, canonical/hreflang, robots/sitemap, datos estructurados, privacidad/noindex y criterios de aceptación automatizables.
- Revisión obligatoria de [ENSAMBRIA_Open_Graph_SEO_Metadata.md](rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md): clasificar rutas públicas, privadas y autenticadas por frontend; definir contratos de metadata, ownership editorial/publicación, dominios tenant, imágenes autorizadas, canonical/hreflang, Open Graph/Twitter Cards, Schema.org, sitemap/robots, caché/invalidation, privacidad y pruebas. Fase 0 documenta el diseño y los criterios sin crear runtime; la capacidad base se implementa desde el primer corte ejecutable de Fase 1.
- Revisión obligatoria de [ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md](rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md): inventariar componentes y flujos de telemetría; designar responsable; definir clasificación/redacción, acceso, retención, cardinalidad, stack propuesto, SLI/SLO candidatos, alertas, pruebas y bloqueos. Fase 0 es documental; la instrumentación y la evidencia se incorporan desde el primer corte ejecutable de Fase 1.
- Revisión obligatoria de [ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md](rules/ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md): inventariar los criterios generales de calidad web y lanzamiento por frontend (seguridad, rendimiento, accesibilidad, responsive, compatibilidad, UX y pruebas), sin duplicar ni reemplazar la autoridad de [ENSAMBRIA_Privacy_Legal_Compliance.md](rules/ENSAMBRIA_Privacy_Legal_Compliance.md) sobre privacidad/cumplimiento ni de [ENSAMBRIA_Open_Graph_SEO_Metadata.md](rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md) sobre SEO/Open Graph. Fase 0 define alcance y criterios; cada frontend debe satisfacerlos desde su fase de implementación y antes de lanzamiento.
- Revisión obligatoria de [ENSAMBRIA_README_Documentation_Quality_Security_Standard.md](rules/ENSAMBRIA_README_Documentation_Quality_Security_Standard.md): comprobar que el README raíz presente ENSAMBRIA y su estado real, mantenga instrucciones verificadas y enlaces a documentación especializada, y no duplique contratos ni detalles internos. Los badges, recursos visuales y demás afirmaciones deben estar respaldados y autorizados; esta revisión documental no implica crear elementos sin evidencia.
- Arquitectura del CORE.
- Arquitectura de verticales.
- Arquitectura de módulos adicionales.
- ADR.
- Modelo de datos inicial.
- Contratos.
- Catálogo de infraestructura técnica compartida y mapa de dependencias permitidas por proceso/módulo.
- Decisión de pool PostgreSQL por backend/proceso conforme a [ENSAMBRIA_Connection_Pooling_PostgreSQL.md](rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md), junto con contexto autenticado/RLS transaccional y ownership de contratos Protobuf.
- Decisión de PostgreSQL compartido inicial: topología física, roles de runtime/migración, grants mínimos por backend/dominio, ownership de schemas y evidencia de que no existe acceso irrestricto ni consulta cruzada de tablas.
- Catálogo de los seis proyectos de interfaz y mapeo de cada uno a sus BFF/backends autorizados, aclarando que no crean backends adicionales; paridad empresarial entre Flutter Business y Tenant Portal.
- Contratos/versionado y política de extensiones controladas; criterios para una eventual extracción selectiva de módulos sin convertirlos en microservicios prematuramente.
- Matriz de integración para PostgreSQL/migraciones, gRPC, Redis, NATS Core, JetStream/Outbox, Vault y observabilidad; justificar los N/A por caso de uso.
- Matriz de eventos/integración durable conforme a [ENSAMBRIA_Transactional_Outbox_Inbox.md](rules/ENSAMBRIA_Transactional_Outbox_Inbox.md): para cada hecho aplicable, módulo dueño, tabla/contrato versionado, payload minimizado, transacción de escritura, stream/subject y confirmación, consumidor y efecto local, Inbox/idempotencia, garantía de orden, reintentos/cuarentena/replay, retención y recuperación, métricas/runbook y evidencia de aceptación. Distinguir JetStream durable de NATS Core efímero; registrar N/A con razón verificable.
- Modelo formal de trabajo GitHub desde Fase 0: Ruleset/Branch Protection para `main`, ramas de corta duración y nombres convencionales, Pull Requests obligatorios, CODEOWNERS, plantillas, CI base, scripts/punto de entrada local, pruebas, seguridad de workflows y evidencia de aceptación. Los controles operativos se desarrollan en [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md); este roadmap conserva el alcance y la aceptación propios de ENSAMBRIA. No crear duplicados si ya existe un mecanismo equivalente.
- Paridad local/CI: cada job obligatorio invoca los mismos comandos versionados que los desarrolladores pueden ejecutar localmente; inventariar manifests, lockfiles, scripts y dependencias reales antes de fijarlos. Separar CI de PR, `main`, análisis programados, releases y despliegues.
- Verificación de configuración efectiva de GitHub: checks requeridos, aprobación/resolución de conversaciones, actualización de rama o merge queue, restricciones a force push/eliminación y permisos de Actions. Si faltan permisos o soporte de la plataforma, registrar el bloqueo y no declarar el control configurado.
- Criterio de aceptación de Fase 0: una persona puede clonar el repositorio y ejecutar los comandos locales documentados; los workflows llaman los mismos comandos/criterios; los checks requeridos y los estados de protección efectivos están comprobados, o los bloqueos externos están identificados con responsable y mitigación. No afirmar cumplimiento por existencia de YAML o scripts sin ejecutar.
- Criterio de privacidad/cumplimiento de Fase 0: están asignados los responsables técnicos y el owner/rol de privacidad y revisión profesional; existe un registro versionado de obligaciones por jurisdicción/tratamiento con fuentes oficiales, aplicabilidad, estado de investigación y revisión legal, propietario, control, prueba/evidencia y bloqueo; las preguntas legales críticas bloquean solo los tratamientos/mercados dependientes. La investigación, documentación e integración técnica no esperan aprobación profesional; ésta se registra como estado separado y no se presume. Esto es trabajo documental y de diseño, no runtime ni certificación.
- Criterios de aceptación para la fundación común de la Fase 1, incluyendo pruebas con dependencias reales.
- Seguridad.
- Roadmap.
- Criterios de aceptación.

Entregable: arquitectura y plan ejecutable, revisión técnica/documental de PDLM por sus owners, responsables y roles designados, inventario/matriz iniciales, decisiones requeridas para la primera implementación e investigación de fuentes oficiales de las jurisdicciones iniciales. La revisión profesional se documenta como actividad posterior a la implementación técnica; su ausencia no bloquea la preparación documental ni el trabajo independiente, y nunca se reporta como aprobación obtenida. Toda pregunta pendiente tiene responsable y bloquea explícitamente solo el alcance que dependa de ella. Esta fase es documental respecto al runtime del producto: no crea crates funcionales, servicios de aplicación, esquemas activos ni migraciones PDLM. Puede establecer gobernanza del repositorio, workflows y comandos de calidad como automatización de desarrollo; su estado se verifica en GitHub y localmente, y no sustituye pruebas de aceptación de producto.

La Fase 0 define la topología objetivo completa; no exige implementar simultáneamente los cuatro backends. Cada fase posterior implementa únicamente su alcance en el orden del roadmap, preservando ownership, contratos y criterios de aceptación.

## FASE 1 — Fundación de ENSAMBRIA CORE

- Consumir los contratos, matriz de ownership, fases y decisiones aprobadas/documentadas para las diez capacidades transversales definidos en Fase 0 y sección 14. Resolver, antes de implementar cada comportamiento, cualquier pendiente que lo bloquee; no reabrir ni omitir decisiones por conveniencia.
- Implementar y probar desde el primer corte de Fase 1 los controles de privacidad/cumplimiento aplicables a las capacidades que se construyen, alineados con [ENSAMBRIA_Privacy_Legal_Compliance.md](rules/ENSAMBRIA_Privacy_Legal_Compliance.md) y PDLM: autorización/aislamiento, minimización y redacción, auditoría, retención, preferencias/consentimiento cuando el tratamiento lo requiera, manejo seguro de errores y procedimientos de incidente aplicables. No activar tratamiento o lanzamiento con obligación crítica sin validación de Legal; la implementación técnica no implica certificación legal.
- Cuando una capacidad introduzca restricciones geográficas, implementar y probar en el mismo corte la decisión aprobada por ubicación/clase de acceso. Mantener diferenciadas la navegación informativa, solicitud de lanzamiento, solicitud de privacidad, registro y operación comercial; no basar una denegación estricta por ciudad únicamente en IP ni impedir derechos de privacidad por geolocalización.
- Implementar desde el primer corte ejecutable la capacidad base de metadatos públicos Open Graph/SEO: contratos y generación SSR/prerender que permitan entregar metadata inicial rastreable, política de URL/canonical y privacidad/indexación, y validación automatizada. Si Fase 1 introduce una ruta/contenido público, completar ahí su SEO, metadata social, JSON-LD/sitemap/robots aplicables y pruebas; no introducir contenido público incompleto para completarlo después. Las fases posteriores deben implementar SEO completo en el mismo cambio que introduce cada nueva superficie pública. Aplicar [ENSAMBRIA_Open_Graph_SEO_Metadata.md](rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md); no indexar paneles, rutas autenticadas, borradores ni datos privados.
- Implementar Outbox/Inbox desde el primer corte ejecutable de Fase 1: la fundación CORE debe incluir al menos un flujo real de negocio que requiera integración durable, con mutación + Outbox atómicos, publicación recuperable por JetStream e Inbox/idempotencia en cada consumidor persistente aplicable. La guía [ENSAMBRIA_Transactional_Outbox_Inbox.md](rules/ENSAMBRIA_Transactional_Outbox_Inbox.md) es normativa. No se permite dejarlo solo diseñado, reemplazarlo por publicación directa, ni aplazarlo a otra fase; las fases posteriores lo aplican desde el primer cambio que requiera entregar un hecho durable.
- Implementar y probar el aislamiento multi-tenant efectivo, el contexto autenticado tenant/sucursal, autorización, RLS y auditoría de seguridad que corresponden a CORE; impedir acceso cruzado aun con identificadores válidos de otros tenants/sucursales.
- Implementar las identidades globales y las membresías tenant-scoped bajo ownership de CORE, usando UUID v7 canónicos de forma consistente. Probar que una misma identidad pueda tener relaciones autorizadas independientes con varios tenants, que IDs preservados entre contratos no confieran acceso y que IDs desconocidos o pertenecientes a otro tenant no permitan operaciones no autorizadas. Cualquier `BIGINT` adicional o identificador de presentación debe tener justificación previa y no sustituir la autorización.
- Definir contratos regionales compatibles con país, moneda, zona horaria y locale, usando identificadores normalizados y montos enteros asociados a moneda. No habilitar conversión automática, tributación internacional ni políticas comerciales/legal no aprobadas.
- Definir la propagación y precedencia del contexto regional/configuración de tenant y sucursal, sin trasladar a CORE las reglas operativas de BUSINESS ni el catálogo comercial de PLATFORM.
- Definir convenciones de versionado y compatibilidad para reglas/eventos/APIs, auditoría atribuible, credenciales/scopes, entrega de webhooks y extensiones controladas. La Fase 1 no implementa un motor genérico de reglas, un API de terceros completo, un Marketplace de extensiones ni capacidades propietarias de fases posteriores.
- Implementar las bibliotecas técnicas compartidas requeridas por los primeros recorridos CORE, comenzando por PostgreSQL/SQLx, pool por proceso, transacciones, contexto/RLS y acceso gRPC/Protobuf.
- Para cada flujo durable de Fase 1, implementar en el mismo módulo dueño la escritura atómica de estado + Outbox, publicación recuperable con confirmación y reintentos acotados, y consumo Inbox idempotente cuando otro módulo/proceso aplique; probar duplicados, fallos en ventanas de commit/ACK/publicación y recuperación con PostgreSQL y JetStream reales. No extender el alcance a eventos sin necesidad de integración documentada.
- Verificar en los recorridos de implementación afectados los pools reales con PostgreSQL: límites concurrentes, timeouts, reutilización de conexiones, aislamiento de credenciales/grants, contexto RLS sin fuga entre tenants tras commit/rollback/cancelación y recuperación ante indisponibilidad/saturación. Evidenciar que los workers no agotan el presupuesto de operaciones críticas; PgBouncer, si se adopta, exige pruebas específicas de compatibilidad.
- Integrar TLS/identidad de servicio, propagación de contexto, errores tipados, observabilidad y acceso a Vault para los procesos que lo requieran.
- Implementar desde el primer corte ejecutable la instrumentación de observabilidad aplicable según [ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md](rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md), con pruebas de propagación, correlación, redacción y degradación no bloqueante para los componentes realmente utilizados. No declarar SLI/SLO o alertas calibradas sin base y verificación.
- Integrar Redis solo para caché con TTL, NATS Core para fan-out efímero y JetStream/Outbox para los estados/eventos durables aplicables; documentar N/A concretos antes de implementar.
- Mantener los módulos de dominio como dueños de sus contratos, políticas, schemas, repositorios, migraciones, Outbox y reglas de negocio.
- Probar interoperabilidad, aislamiento, autorización, fallos, recuperación y redacción con PostgreSQL, Redis y NATS reales en el entorno de desarrollo definido. La integración con Vault y certificados no se considera validada sin acceso al entorno/credenciales autorizados; no se reemplaza por simulaciones.
- Identidad.
- Autenticación.
- Autorización.
- Tenancy.
- Alta autoservicio que permita a una persona natural operar sin persona jurídica, local, sucursal o empleados; tenant estable y membresía/organización bajo scope autorizado. No fijar la modalidad como tipo de tenant ni asignar automáticamente plan, precio, trial, cobro o entitlement.
- Usuarios.
- Roles.
- Organizaciones.
- Sucursales.
- Capacidades.
- Catálogo de planes SaaS y suscripciones en PLATFORM (Básico, Profesional, Empresarial y Franquicia); CORE evalúa/aplica los entitlements efectivos según su contrato. El plan comercial Franquicia no reemplaza el módulo de cadenas de BUSINESS de la Fase 10.
- Motor de disponibilidad.
- Reservas comunes.
- Abstracción de pagos.
- Auditoría.
- Persistencia.
- Seguridad multi-tenant.

Entregable: núcleo funcional, persistente y probado, con la infraestructura común efectivamente usada por los recorridos CORE aplicables, aislamiento tenant/sucursal validado y contratos arquitectónicos transversales documentados con ownership y trazabilidad hacia sus fases de implementación. La Fase 1 no se cierra si falta una prueba obligatoria con servicios reales o una integración bloqueada se declara validada.

## FASE 2 — Primeros verticales

Implementar:
- Catálogo de servicios con modalidades permitidas por servicio: establecimiento, domicilio del cliente, otra ubicación acordada o remoto cuando la actividad lo admita. Un servicio no se duplica solo por cambiar modalidad y se rechazan combinaciones no admitidas en backend.
- Barberías: cortes/barba, barberos, sillas, turnos y recursos propios.
- Peluquerías y tratamientos capilares: estilistas, fórmulas y productos usados, lavacabezas, tiempos de exposición y estaciones.
- Salones de belleza/centros de uñas: manicura, pedicura, uñas, pestañas, cejas, maquillaje, depilación y paquetes.
- Spas, centros de estética y bienestar: tratamientos, terapeutas, cabinas/camillas/equipos, sesiones y paquetes.
- Spas/estética: planes de tratamiento y seguimiento de sesiones; fotografías de evolución solo con autorización específica, controles de privacidad, acceso, retención y eliminación aplicables.
- Configuración extensible para estudios de maquillaje, centros de uñas, tratamientos capilares y otros negocios relacionados, sin exigir un nuevo crate mientras no exista un límite de dominio que lo justifique.

Construir sus configuraciones y reglas particulares.

Preparar en los catálogos de servicios los campos editoriales SEO, slug y estado de publicación de acuerdo con su ownership. No indexar ni renderizar públicamente servicios hasta que la página pública autorizada y su SEO completo estén implementados.

Validar que todos reutilicen correctamente el CORE y no existan duplicaciones artificiales.
La operación móvil y a domicilio pertenece a BUSINESS, no es logística de reparto: la aceptación de cada modalidad prueba su ubicación/scope, cobertura y límites operativos junto con CORE, sin exigir un establecimiento o sucursal ficticia. La política completa de desplazamiento/asignación y el workforce se implementan en Fase 4; no se confirma en Fase 2 una reserva cuya factibilidad conocida sea inválida.
La aceptación prueba horarios, recursos simultáneos, duración/preparación, conflictos concurrentes, aislamiento de sucursal/tenant y permisos. Los flujos que recojan datos de salud o sensibles requieren decisión de alcance, controles y justificación regulatoria antes de implementarse.
En peluquerías, la valoración o diagnóstico capilar cosmético/no clínico debe poder registrarse por personal autorizado, vincularse con el cliente, servicio, fórmula y productos utilizados, y conservar fecha, autoría e historial de cambios. Las pruebas deben verificar autorización por tenant/sucursal, acceso mínimo y auditoría; los campos que impliquen datos sensibles o de salud no se habilitan hasta que su tratamiento y controles estén aprobados y probados para el alcance aplicable.
La aceptación de los módulos de spa/estética verifica el historial y seguimiento de planes/sesiones y el control por tenant/sucursal. No se habilita el registro de fotografías de evolución ni de información sensible mientras no se hayan definido y validado su finalidad, autorización específica, permisos, retención, eliminación y requisitos legales aplicables; cuando se habilite, las pruebas deben verificar acceso indebido, revocación y ciclo de vida.

## FASE 3 — CRM y fidelización

- CRM independiente.
- Clientes y su historial privados por tenant; una identidad global no fusiona fichas ni comparte historial entre negocios.
- Clientes.
- Historial.
- Segmentación.
- Preferencias.
- Membresías.
- Créditos.
- Paquetes.
- Puntos.
- Referidos.
- Gift cards.
- Consumos.
- Renovaciones.
- Historial de visitas/preferencias y fotografías de resultados solo con consentimiento y retención controlada.
- Recordatorios de mantenimiento y reactivación según reglas configurables por tenant y permisos de comunicación.

## FASE 4 — Operaciones comerciales

- Pagos, transacciones, comisiones de pagos y ledger inmutable en CORE.
- Caja, ventas, comisiones de empleados y operaciones financieras propias del establecimiento en BUSINESS.
- Productos.
- Inventario.
- Compras.
- Proveedores.
- Costos.
- Rentabilidad.
- Gestión operativa de personal: turnos, asistencia, ausencias/vacaciones, metas y evaluaciones.
- Completar la capacidad BUSINESS de servicios móviles/a domicilio: zonas habilitadas/excluidas y su vigencia, factibilidad de desplazamiento entre reservas, buffers/preparación, capacidad, incidencias, reprogramación y asignación manual; cualquier asignación automática exige política aprobada, explicable, auditable y configurable.
- Definir y probar las transiciones autorizadas del estado operativo de la prestación en BUSINESS, separado del ciclo de vida común de la reserva de CORE; profesionales pueden operar en ubicaciones y modalidades autorizadas sin crear otro tenant.
- Separar precio del servicio, desplazamiento, cargos autorizados, descuentos, impuestos aplicables y total. No introducir valores ni reglas fiscales/comerciales por defecto; los efectos financieros siguen pasando por CORE.
- Comisiones por servicios/productos, bonificaciones y liquidaciones auditables integradas con el ledger dueño.
- Inventario de productos de venta e insumos internos, consumos por servicio, lotes/vencimientos, recetas, mermas y costos por sucursal.
- Costeo real unitario por servicio/tratamiento que sume consumos materiales efectivamente registrados y la comisión laboral aplicable calculada por BUSINESS con la regla/base vigente; guardar la base y versiones usadas para reproducir el resultado histórico, sin doble conteo ni mezcla con cargos de pasarela o comisiones comerciales de ENSAMBRIA.
- Reservas grupales/recurrentes, lista de espera, multi-profesional/multi-recurso y reasignación autorizada, conforme a los casos de uso habilitados.
- Preparar campos editoriales SEO, slug y estado de publicación para productos de BUSINESS; no hacer indexable el catálogo privado ni exponer datos hasta que sus páginas públicas y SEO completo estén disponibles.

La aceptación de los costos comprueba, con datos transaccionales reales, costos estimados frente a reales, cantidades y costos de materiales, cambios de fórmula/precio, comisiones laborales versionadas, sucursal, ajustes autorizados y concurrencia. Debe reproducir el cálculo histórico, impedir doble conteo y probar que los reportes distinguen costo directo, comisión laboral, cargos externos, ingresos y margen; las reglas de BUSINESS no escriben ni alteran el ledger de CORE.

## FASE 5 — Web y aplicaciones

- Incorporar el onboarding progresivo de las tres modalidades descritas en §2.2; permitir guardar y reanudar el progreso, recuperación idempotente y configuración sin datos de local/sociedad/equipo innecesarios. Completar la superficie por BFF y owners reales; no declarar un formulario aislado como flujo aceptado.
- ENSAMBRIA Business (Flutter) y ENSAMBRIA Tenant Portal (Next.js) exponen las capacidades empresariales implementadas sobre los mismos BFF, contratos, roles y autorización; entregar y probar paridad por caso de uso, sin duplicar reglas.
- ENSAMBRIA Platform Admin (Next.js) ofrece la administración global privada mediante PLATFORM y su BFF integrado; no es una consola tenant.
- Portal de empleados.
- Vistas privadas de agenda, reservas asignadas, comisiones, metas y resultados según rol.
- ENSAMBRIA Marketplace (Next.js) ofrece páginas públicas/Discovery desde proyecciones autorizadas de PUBLIC; BUSINESS conserva el contenido fuente.
- ENSAMBRIA Corporate (Next.js) presenta contenido institucional/comercial público propiedad de PLATFORM; contenido dinámico requiere contrato público autorizado y no accede directamente a backend.
- ENSAMBRIA App (Flutter) ofrece los recorridos móviles de clientes aprobados, sin capacidades administrativas de tenant.
- SEO técnico completo desde el primer despliegue de cada web pública, incluida la web tenant: renderizado rastreable, metadatos por ruta, canonical/dominio, sitemap/robots, Open Graph y datos estructurados autorizados.
- Herramientas de administración BUSINESS para previsualizar y editar SEO de servicios y contenido de tenant publicable; PLATFORM administra metadatos únicamente para propiedades/contenido institucional propio. El contenido privado o en borrador nunca se expone a crawlers.
- Constructor de sitios.
- Dominios personalizados.
- Integraciones nativas Swift.

## FASE 6 — Comunicación y marketing

- WhatsApp.
- Notificaciones.
- Email.
- Campañas.
- Automatizaciones.
- Recordatorios.
- Recuperación de clientes.
- Consentimientos.
- Medición de campañas.
- Confirmación/cancelación de reservas y flujos conversacionales de reservas por WhatsApp cuando la capacidad oficial y el consentimiento lo permitan.
- Venta cruzada por campañas o recomendaciones contextuales, con reglas de elegibilidad por tenant/sucursal y medición de impresiones, selecciones y conversiones. La aceptación verifica preferencias/exclusiones y permisos del canal, que no se envíen comunicaciones sin autorización aplicable y que una recomendación no cree una compra, reserva o cobro sin confirmación explícita.

## FASE 7 — Facturación e integraciones

- Integración de pagos Wompi mediante CORE para pagos SaaS solicitados por PLATFORM y ventas/reservas solicitadas por BUSINESS, con contextos de cuenta, credenciales, callbacks y ledger aislados.
- Integración de facturación electrónica Factus en PLATFORM para documentos SaaS de ENSAMBRIA y en BUSINESS para cada establecimiento, con emisores/configuraciones separados.
- DIAN.
- API pública.
- Webhooks.
- Integraciones externas.
- Conciliación.

## FASE 8 — Inteligencia empresarial

- Dashboard.
- BI.
- Rentabilidad.
- Capacidad.
- Predicción de demanda.
- Analítica por sucursal.
- Analítica por profesional.
- Rentabilidad por servicio, empleado y sucursal; métricas de ocupación y demanda.
- Reportes de costo real por servicio/tratamiento que distingan materiales consumidos, comisión laboral, otros cargos, ingresos y margen, y permitan diferenciar resultados estimados de reales con la base y versión de cálculo correspondientes.
- Modelos de predicción de demanda e inasistencias, con evaluación de calidad y límites por tenant.
- Informes.

## FASE 9 — Marketplace y comercio electrónico

- PUBLIC: directorio, geolocalización, páginas y proyecciones públicas autorizadas, reseñas verificadas, búsqueda y atribución del canal.
- Publicar perfiles y modalidades móviles solo con autorización explícita y campos minimizados; nunca exponer domicilios de clientes, ubicaciones privadas de profesionales o direcciones no autorizadas. Revalidar cobertura/disponibilidad con los owners antes de reservar.
- BUSINESS: tiendas, catálogo comercial, carritos, pedidos, inventario y reglas de venta.
- BUSINESS: mostrar recomendaciones de venta cruzada elegibles en el flujo comercial y validar precio, compatibilidad, disponibilidad y existencias al confirmar; una recomendación no incorpora artículos ni genera pedidos/cobros sin acción explícita.
- CORE: reserva, pago anticipado cuando esté habilitado, idempotencia, comisiones transaccionales y ledger para operaciones iniciadas en Marketplace; aplicar políticas aprobadas de cancelación/reembolso y revalidar precio y disponibilidad con el backend propietario.
- PLATFORM: políticas y tarifas comerciales de monetización. Puede evaluar comisiones por adquisición, publicidad/posicionamiento destacado o planes de posicionamiento, pero no se activa ninguna modalidad ni tarifa sin aprobación comercial explícita, reglas de atribución y acumulación documentadas, transparencia al usuario y aplicación transaccional validada por CORE.
- Permitir comparar establecimientos y ofertas según campos públicos autorizados (servicio, precio, duración, ubicación/sucursal y disponibilidad), indicar condiciones y vigencia de datos variables e identificar claramente resultados pagados/destacados. La aceptación verifica que fichas privadas, revocadas o no publicadas nunca aparezcan, y que precio/disponibilidad se revaliden con el backend propietario antes de reservar o comprar.
- Completar SEO y descubribilidad de fichas públicas de establecimientos, servicios y productos publicados; generar sitemap/metadata/datos estructurados desde proyecciones autorizadas, con canonical correcto por dominio y locale. Las fichas privadas, no publicadas o revocadas no se indexan y se retiran de los sitemaps.

La aceptación del Marketplace cubre resultados por ubicación elegida/autorizada, comparación de datos vigentes, ausencia de contenido privado/no publicado/revocado, identificación de publicidad y atribución directa frente a canal Marketplace. Para pagos anticipados debe probarse confirmación y rechazo del proveedor, duplicados/idempotencia, disponibilidad o precio que cambian antes de confirmar y cancelación/reembolso conforme a políticas aprobadas. Debe comprobarse que una modalidad de monetización no aprobada no genere cargos ni efectos en el ledger.

## FASE 10 — Franquicias y expansión

- Gestión de cadenas.
- Franquicias.
- Jerarquías.
- Consolidación.
- Multi-moneda.
- Multi-idioma.
- Internacionalización.
- Reglas regionales.
- Nuevos proveedores.
- Administración centralizada y políticas para catálogos/precios globales o por sucursal, permisos regionales, comparativos, transferencias e informes consolidados.
- Gestión de franquiciados, regalías y liquidaciones solo con reglas comerciales y legales explícitas; el plan SaaS “Franquicia” es distinto de estas funcionalidades.
- Caja por establecimiento/sucursal, con permisos y trazabilidad de operaciones, y reportes consolidados solo para roles autorizados.

La aceptación de cadenas prueba permisos suficientes/insuficientes por sucursal, aislamiento entre sucursales y tenants, consistencia de catálogos/precios globales y locales, transferencias autorizadas, visibilidad de caja por rol y exactitud de reportes consolidados frente a los registros de cada establecimiento.

## FASE 11 — Inteligencia artificial y ecosistema abierto

- Asistente empresarial con consultas autorizadas sobre datos reales del tenant y definiciones de métricas consistentes.
- IA comercial: promociones sugeridas para baja ocupación, análisis de rentabilidad y recomendaciones de inventario.
- Predicción de demanda y riesgo de cancelación/inasistencia, con medición de precisión, sesgos y utilidad antes de decisiones automatizadas.
- Automatización inteligente.
- Asistente conversacional de reservas/atención por WhatsApp, con permisos, consentimiento, límites y confirmación de acciones sensibles.
- Generación de publicaciones/campañas para revisión humana antes de publicarse.
- Marketplace de extensiones.
- API empresarial.
- Ecosistema de integraciones.

Las fases pueden ajustarse después del descubrimiento técnico, pero ningún requisito debe desaparecer sin una decisión explícita.

---

# 23. CRITERIOS DE CIERRE DE CADA FASE

Los criterios de esta sección se aplican tanto a cada subfase del [ROADMAP.md](../ROADMAP.md) como a la fase completa. La aceptación de una subfase acredita únicamente su alcance; no sustituye la aceptación de las demás subfases ni autoriza a saltar una dependencia. Si aparece un bloqueo, se delimita al comportamiento dependiente y pueden continuar las subfases independientes cuya precedencia esté satisfecha.

Una fase o subfase solamente se considera terminada cuando cumple:

CÓDIGO + PRUEBAS + SEGURIDAD + ARQUITECTURA + DOCUMENTACIÓN + REPRODUCIBILIDAD + CI + DEPLOYMENT CUANDO CORRESPONDA + VERIFICACIÓN.

Todo entregable entra mediante PR y los checks obligatorios de CI deben coincidir con las verificaciones locales documentadas. La fase no se cierra por checks omitidos/pendientes ni por haber fusionado sus PR: requiere evidencia de aceptación para todos los requisitos y suites aplicables.

No marcar una fase como completa por tener interfaces visuales o endpoints que no persisten información.

No declarar terminado un módulo que solamente tenga modelos de dominio sin integración operativa.

Cada fase y subfase debe contar con:
- Objetivos.
- Alcance.
- Entregables.
- Requisitos.
- Criterios de aceptación.
- Pruebas.
- Evidencias.
- Riesgos.
- Decisiones pendientes.
- Estado real.
- Documentación actualizada.

---

# 24. INSTRUCCIONES PARA EL AGENTE DE DESARROLLO

Antes de modificar archivos:

1. Inspecciona todo el repositorio.
2. Identifica la arquitectura existente.
3. Identifica las tecnologías.
4. Identifica las funcionalidades ya implementadas.
5. Identifica pruebas y workflows.
6. Identifica documentación.
7. Identifica restricciones.
8. Identifica dependencias.
9. Identifica riesgos.
10. Identifica decisiones pendientes.

No inventes que una funcionalidad existe.

No reescribas módulos que ya cumplen sus requisitos.

No cambies tecnologías existentes sin justificarlo.

No modifiques scripts, workflows o infraestructura fuera del alcance autorizado.

No hagas cambios masivos sin un plan verificable.

No avances directamente a desarrollar funcionalidades antes de definir los límites del dominio y sus contratos.

## Decisiones que deben quedar definidas desde el principio

- Arquitectura del CORE.
- Separación de módulos verticales.
- CRM como módulo independiente.
- Sistema de capacidades y entitlements.
- Modelo multi-tenant.
- Modelo de sucursales.
- Motor común de reservas.
- Sistema de pagos.
- Separación financiera.
- Contratos entre módulos.
- Modelo de eventos.
- Seguridad.
- Persistencia.
- Migraciones.
- Auditoría.
- Observabilidad.
- Estrategia de pruebas.
- Versionado de APIs.
- Gestión de integraciones.
- Estrategia de despliegue.
- Internacionalización.
- Modelo de extensibilidad.

No se deben posponer decisiones estructurales que puedan producir acoplamiento o migraciones destructivas en el futuro.

Las funcionalidades pueden implementarse por etapas, pero su encaje arquitectónico debe quedar definido desde el principio.

## Documentación obligatoria

Mantener actualizados:
- README principal.
- Arquitectura.
- ADR.
- Modelo de dominios.
- Contratos.
- Modelo de datos.
- Seguridad.
- Tenancy.
- Módulos.
- Roadmap.
- Requisitos.
- Criterios de aceptación.
- Manual de desarrollo.
- Manual de despliegue.
- Operación.
- Recuperación.
- Integraciones.
- Evidencias de pruebas.

---

# 25. PRIMERA TAREA DEL AGENTE

No comiences implementando todo el producto.

Primero realiza una fase de descubrimiento y diseño arquitectónico.

Entrega:

1. Diagnóstico del repositorio.
2. Inventario de funcionalidades existentes.
3. Arquitectura propuesta.
4. Diagrama de ENSAMBRIA CORE.
5. Diagrama de módulos verticales.
6. Diagrama de módulos adicionales.
7. Mapa de dependencias.
8. Contextos delimitados de DDD.
9. Modelo inicial de tenancy.
10. Modelo de capacidades y planes.
11. Modelo de datos de alto nivel.
12. Contratos entre módulos.
13. Arquitectura de seguridad.
14. Estrategia de persistencia.
15. Estrategia de pruebas.
16. Estrategia de despliegue.
17. Roadmap por fases.
18. Riesgos técnicos.
19. Decisiones arquitectónicas pendientes.
20. Criterios de aceptación.

Identifica las decisiones que necesitan aprobación del propietario del producto y sepáralas de las decisiones técnicas que puede resolver el equipo.

No inventes políticas comerciales, comisiones, precios, condiciones financieras, requisitos legales ni capacidades de proveedores.

Presenta alternativas y consecuencias cuando una decisión todavía no esté aprobada.

Una vez aprobado el diseño, implementar la fase correspondiente de forma completa y verificable.

---

# OBJETIVO FINAL

Construir ENSAMBRIA como una plataforma SaaS empresarial de referencia para barberías, peluquerías, salones de belleza, centros de estética y spas.

Una plataforma:
- Modular.
- Escalable.
- Segura.
- Multi-tenant.
- Multi-sucursal.
- Extensible.
- Internacionalizable.
- Con arquitectura profesional.
- Con operaciones reales.
- Con integridad financiera.
- Con aislamiento de datos.
- Con automatización.
- Con inteligencia empresarial.
- Con ecosistema abierto.

**ENSAMBRIA no debe ser un conjunto de aplicaciones independientes. Debe ser un ecosistema unificado cuyo núcleo común permita activar módulos especializados según las necesidades de cada negocio.**

Cada decisión técnica debe acercar el producto a esa visión sin sacrificar la calidad, la seguridad ni la integridad de las operaciones.
