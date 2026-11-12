# ENSAMBRIA — Política global de privacidad, protección de datos, cumplimiento legal y confianza digital

**Archivo:** `ENSAMBRIA_Privacy_Legal_Compliance.md`<br>
**Categoría:** Arquitectura, seguridad, privacidad y cumplimiento normativo<br>
**Aplicación:** Todos los backends, frontends, workers, APIs, integraciones, proveedores y entornos de ENSAMBRIA.<br>
**Vigencia:** Obligatoria desde la Fase 0.<br>
**Ámbito:** Internacional y multi-jurisdiccional.

Esta política se aplica junto con [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](ENSAMBRIA_Privacy_Data_Lifecycle_Management.md), que detalla el ciclo de vida de datos y los derechos de titulares. Esta guía amplía el marco a cumplimiento por jurisdicción, consumidores, menores, consentimiento/cookies, marketing, proveedores, incidentes, contenido y habilitación de mercados; no reemplaza los requisitos especializados de PDLM. Ambos documentos son requisitos técnicos/operativos y no constituyen asesoría jurídica, aprobación de Legal ni certificación de cumplimiento. La investigación, redacción y preparación técnica continúan sin esperar revisión profesional. Una norma, rol, base, plazo u obligación no se presenta como validado jurídicamente hasta que exista evidencia de revisión competente para la jurisdicción, actividad y tratamiento concretos. La revisión profesional posterior no es una condición general para redactar, desarrollar o integrar documentos ni para avanzar trabajo independiente; una obligación previa específica solo bloquea la operación o tratamiento que dependa de ella.

## Estados de documentación y aviso de revisión profesional

Cada documento mantiene estados distintos, versionados y auditables:

- **Preparación documental:** investigación de fuentes oficiales aplicables y redacción vinculada a las funciones reales conocidas. Los hechos de producto, comerciales o jurídicos todavía desconocidos se registran como pendientes; no se completan mediante conjeturas.
- **Integración técnica:** contenido completo para el alcance aprobado, disponible en las superficies correspondientes y conectado con versionado, consulta, aceptación/consentimiento y evidencia cuando aplique. La revisión de un abogado no es una precondición general para desarrollar o integrar el mecanismo. No se marca este estado hasta que las rutas, formularios, permisos y pruebas correspondientes existan y pasen.
- **Revisión profesional:** revisión posterior por abogado competente y responsable de privacidad designado, registrada por jurisdicción, documento, versión, fecha, alcance y resultado. No se infiere aprobación ni certificación del mero hecho de integrar o aceptar una versión.

Mientras la revisión profesional de la versión publicada esté pendiente, cada página legal y punto de interacción pertinente debe mostrar, de manera visible y accesible, el aviso siguiente, sin impedir la consulta:

> **Aviso sobre revisión jurídica**
>
> La documentación legal de ENSAMBRIA se encuentra implementada y disponible, pero está pendiente de revisión por un profesional del derecho. Su contenido podrá actualizarse como resultado de dicha revisión.

El aviso solo puede mostrarse como afirmación de implementación cuando el contenido esté realmente integrado y disponible en esa superficie. En Fase 0 se registra como requisito del diseño, no como una declaración de que ya existe una implementación. No se muestra la etiqueta interna de estado como sustituto del texto legal ni se presenta una versión como aprobada sin evidencia.

Las restricciones de acceso según país, región o ciudad se especifican en [ENSAMBRIA_Geographic_Access_Policies.md](ENSAMBRIA_Geographic_Access_Policies.md). Deben limitar la capacidad afectada sin bloquear automáticamente navegación informativa ni solicitudes de privacidad. La geolocalización por IP es solo una señal aproximada y no basta como prueba de una restricción estricta por ciudad. La determinación de aplicabilidad legal continúa bajo esta política y requiere revisión de Legal.

La calidad general y los criterios de lanzamiento de los sitios y frontends se rigen por [ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md](ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md), que complementa esta política sin reemplazarla. La presentación de Open Graph y los requisitos detallados de SEO se rigen por [ENSAMBRIA_Open_Graph_SEO_Metadata.md](ENSAMBRIA_Open_Graph_SEO_Metadata.md). Esta política es la fuente normativa para privacidad, protección de datos y cumplimiento legal; las otras guías no crean interpretaciones legales independientes.

---

# 1. Objetivo

ENSAMBRIA debe diseñarse desde su origen para operar como una plataforma SaaS internacional, multi-tenant y multivertical, cumpliendo las obligaciones de privacidad, protección de datos personales, seguridad, comunicaciones comerciales, derechos de consumidores, protección de menores y propiedad intelectual que resulten aplicables en cada jurisdicción.

Esta política establece requisitos técnicos y organizativos que deben incorporarse al diseño, desarrollo, pruebas, despliegue y operación de la plataforma.

No se debe esperar a que ENSAMBRIA tenga usuarios internacionales para incorporar estos controles.

La arquitectura debe permitir:

- Operar en Colombia y expandirse a otros países.
- Adaptar los tratamientos de datos según la legislación aplicable.
- Gestionar diferentes edades mínimas y requisitos de consentimiento.
- Administrar preferencias de privacidad y cookies.
- Gestionar solicitudes de titulares de datos.
- Controlar proveedores y transferencias internacionales de información.
- Evitar la exposición innecesaria de datos personales.
- Administrar suscripciones, renovaciones y cancelaciones transparentemente.
- Proteger los datos de clientes, empleados, profesionales y establecimientos.
- Gestionar contenido generado por usuarios y reclamaciones de propiedad intelectual.
- Mantener evidencias verificables de cumplimiento.

**Principio fundamental:** el cumplimiento legal no puede depender exclusivamente de textos legales, casillas de aceptación o configuraciones del frontend. Debe existir aplicación técnica, controles de backend, trazabilidad, pruebas y procedimientos operativos.

# 2. Principios obligatorios

Todos los componentes de ENSAMBRIA deben respetar los siguientes principios:

1. Privacidad desde el diseño y por defecto.
2. Seguridad desde el diseño.
3. Minimización de datos.
4. Finalidad específica y documentada.
5. Transparencia en los tratamientos.
6. Consentimiento válido cuando sea la base jurídica correspondiente.
7. Separación entre consentimiento, aceptación contractual y preferencias comerciales.
8. Control de acceso por identidad, tenant, organización, sucursal y permisos.
9. Retención limitada y justificada.
10. Eliminación, anonimización o conservación restringida según corresponda.
11. Trazabilidad de las operaciones sensibles.
12. Responsabilidad demostrable.
13. Protección reforzada de menores y categorías especiales de información.
14. Control de proveedores y transferencias internacionales.
15. Configuración por jurisdicción sin duplicar los dominios centrales.
16. No recopilar información sensible si no existe una necesidad legítima y una base jurídica adecuada.

No se debe interpretar una casilla de aceptación como autorización universal para cualquier tratamiento de datos.

# 3. Alcance arquitectónico

Esta política es transversal a toda la arquitectura ENSAMBRIA.

## 3.1 CORE BACKEND

Responsabilidades:

- Identidad y autenticación.
- Gestión de usuarios y pertenencias.
- Contexto de tenancy.
- Autorización y aislamiento.
- Registro de consentimientos cuando corresponda.
- Preferencias de privacidad.
- Gestión central de solicitudes de titulares.
- Coordinación de exportación, rectificación y supresión.
- Gestión de sesiones y credenciales.
- Auditoría de operaciones sensibles.
- Políticas técnicas de acceso y protección de datos.
- Coordinación de restricciones por edad y jurisdicción.
- Registro de evidencias de aceptación contractual cuando corresponda.

CORE debe ser la autoridad técnica para identidad, contexto, autorización y aislamiento.

No debe asumir que es propietario de todos los datos personales de los demás dominios.

## 3.2 BUSINESS BACKEND

Responsabilidades:

- Datos de establecimientos.
- Clientes y CRM.
- Profesionales y empleados.
- Reservas y servicios.
- Membresías y fidelización.
- Inventario y compras.
- Commerce y pedidos.
- Marketing y comunicaciones comerciales del establecimiento.
- Franquicias y sucursales.
- Datos operativos y comerciales de cada negocio.

Debe implementar los controles de privacidad aplicables a sus tratamientos.

Cada establecimiento debe disponer de las capacidades y configuraciones necesarias para cumplir sus propias obligaciones como responsable o encargado, según su relación jurídica con los titulares.

ENSAMBRIA no debe asumir automáticamente que todos los establecimientos tienen la misma responsabilidad jurídica sobre los datos.

## 3.3 PLATFORM BACKEND

Responsabilidades:

- Gestión comercial de ENSAMBRIA SaaS.
- Planes y suscripciones.
- Facturación SaaS.
- Renovaciones y cancelaciones.
- Transparencia contractual.
- Gestión de políticas comerciales.
- Configuración de jurisdicciones.
- Gestión administrativa de solicitudes.
- Gestión de proveedores contratados por ENSAMBRIA.
- Evidencias de aceptación de contratos.
- Políticas globales de cumplimiento.
- Supervisión de obligaciones de la plataforma.

Debe separar los datos de facturación SaaS de la información comercial de los establecimientos.

## 3.4 PUBLIC BACKEND

Responsabilidades:

- Directorio público.
- Marketplace.
- Páginas públicas.
- Perfiles de establecimientos.
- Búsquedas e indexación.
- Reseñas y contenido público.
- Imágenes públicas.
- Promociones.
- Atribución de origen de reservas y compras.

Debe publicar exclusivamente información autorizada.

Debe contemplar solicitudes de retirada, desindexación, corrección y eliminación de información pública.

No debe exponer información privada procedente de BUSINESS o CORE.

## 3.5 Frontends

La política aplica a:

- `ensambria-business`
- `tenant-portal`
- `ensambria-app`
- `marketplace`
- `platform-admin`
- `corporate`

Cada frontend debe presentar las opciones y avisos correspondientes, pero nunca ser la única barrera de seguridad o cumplimiento.

Las restricciones importantes deben verificarse también en el backend.

# 4. Marco internacional de cumplimiento

ENSAMBRIA debe mantener una matriz de jurisdicciones y obligaciones aplicables.

La siguiente relación identifica marcos normativos que deben evaluarse. No significa que todos se apliquen automáticamente a todos los usuarios o establecimientos.

## 4.1 Colombia

Considerar, entre otras disposiciones:

- Ley 1581 de 2012: protección de datos personales.
- Decreto 1074 de 2015 y disposiciones reglamentarias aplicables.
- Ley 1266 de 2008 cuando corresponda al tratamiento de información financiera, crediticia, comercial y de servicios.
- Régimen de protección al consumidor, incluyendo la Ley 1480 de 2011.
- Normativa aplicable a comercio electrónico, comunicaciones comerciales y protección de menores.
- Reglas de transferencias y transmisiones internacionales de datos.

Se deben analizar las responsabilidades de ENSAMBRIA y de cada establecimiento, así como las obligaciones ante la Superintendencia de Industria y Comercio cuando correspondan.

## 4.2 Unión Europea y Espacio Económico Europeo

Evaluar:

- GDPR: Reglamento (UE) 2016/679.
- ePrivacy y normativa nacional que implemente las reglas aplicables a cookies y tecnologías similares.
- Digital Services Act cuando las funciones de ENSAMBRIA entren en su ámbito.
- Normativa de consumidores y contratos a distancia.
- Requisitos de accesibilidad y normativa digital aplicable según el servicio.
- Reglas sobre transferencias internacionales de datos.

Requisitos técnicos:

- Identificación de bases jurídicas.
- Registro de actividades de tratamiento.
- Gestión de derechos de titulares.
- Gestión de encargados y subencargados.
- Evaluaciones de impacto cuando sean necesarias.
- Gestión de transferencias internacionales.
- Notificación de incidentes dentro de los plazos legales aplicables.
- Privacidad desde el diseño y por defecto.

## 4.3 Reino Unido

Evaluar:

- UK GDPR.
- Data Protection Act 2018.
- PECR.
- Requisitos de cookies, comunicaciones electrónicas y marketing.
- Reglas de protección al consumidor.

No asumir que cumplir el GDPR europeo cubre automáticamente todas las obligaciones británicas.

## 4.4 Estados Unidos

Evaluar la legislación federal y estatal aplicable, incluyendo:

- COPPA.
- CCPA y CPRA en California.
- CAN-SPAM.
- TCPA cuando existan comunicaciones telefónicas o SMS dentro de su ámbito.
- Leyes estatales de privacidad.
- Leyes estatales de protección al consumidor.
- Legislación estatal sobre grabación de comunicaciones.
- Normativa aplicable a renovaciones automáticas y cancelaciones.
- Leyes de protección de menores y privacidad infantil.

La aplicabilidad depende de la actividad, el tipo de información, la ubicación de los consumidores, el modelo de negocio y los umbrales legales.

No utilizar un único aviso de privacidad estadounidense como sustituto de todos los requisitos estatales.

## 4.5 Brasil

Evaluar:

- LGPD.
- Reglas de la ANPD.
- Bases jurídicas y derechos de titulares.
- Requisitos para menores.
- Transferencias internacionales.
- Gestión de incidentes.
- Contratos con operadores y encargados.

## 4.6 Canadá

Evaluar:

- PIPEDA.
- Legislación provincial aplicable.
- Quebec Law 25 y normativa relacionada cuando corresponda.
- CASL para comunicaciones comerciales electrónicas.

## 4.7 México

Evaluar:

- Legislación federal vigente de protección de datos personales en posesión de particulares.
- Avisos de privacidad.
- Derechos ARCO.
- Transferencias y encargados.
- Normativa de consumidores y comercio electrónico.

## 4.8 Otras jurisdicciones

La arquitectura debe permitir incorporar nuevas jurisdicciones sin reescribir los dominios.

Entre otras, deberán evaluarse:

- Argentina.
- Chile.
- Perú.
- Uruguay.
- Australia.
- Nueva Zelanda.
- Japón.
- Corea del Sur.
- India.
- Sudáfrica.
- Otros países donde ENSAMBRIA comercialice o preste servicios.

No se debe habilitar un país comercialmente sin una evaluación de las obligaciones legales, fiscales, de privacidad, protección al consumidor y operación que resulten pertinentes.

# 5. Motor de jurisdicciones y políticas

ENSAMBRIA debe disponer de una capacidad central para administrar configuraciones normativas y requisitos aplicables.

No se debe crear una colección de condiciones dispersas por todo el código como:

`if country == "US"`.

Debe existir una política centralizada, versionada y verificable.

## 5.1 Información de jurisdicción

El sistema debe poder determinar, según el tratamiento y la información disponible:

- País del usuario.
- Región o estado cuando sea necesario.
- País del establecimiento.
- País del responsable del tratamiento.
- Ubicación del servicio.
- Lugar de contratación.
- Mercado al que se dirige la oferta.
- Ubicación de almacenamiento y procesamiento.
- Jurisdicción contractual aplicable cuando corresponda.

No utilizar una sola dirección IP como prueba definitiva de residencia, nacionalidad o jurisdicción legal.

La geolocalización por IP puede servir como señal técnica, pero no debe sustituir los mecanismos legales de determinación de aplicabilidad.

## 5.2 Configuración versionada

La configuración debe permitir definir:

- Edad mínima aplicable.
- Requisitos de consentimiento parental.
- Bases jurídicas disponibles.
- Categorías de datos restringidas.
- Requisitos de cookies.
- Tecnologías que necesitan consentimiento.
- Reglas de marketing.
- Retención.
- Requisitos de exportación.
- Reglas de transferencia internacional.
- Requisitos de renovación automática.
- Requisitos de cancelación.
- Avisos y textos legales.
- Restricciones de publicación.
- Requisitos de accesibilidad cuando correspondan.

Cada política debe disponer de:

- Identificador.
- Jurisdicción.
- Versión.
- Fecha de entrada en vigor.
- Fecha de expiración, si corresponde.
- Estado.
- Fuente normativa.
- Responsable de aprobación.
- Evidencia de revisión.
- Historial de cambios.

Las políticas no pueden modificarse retroactivamente para cambiar la evidencia histórica de un consentimiento o contrato.

## 5.3 Precedencia de políticas

Cuando existan varias obligaciones aplicables:

- Se deben identificar todas las jurisdicciones pertinentes.
- Se debe resolver cada tratamiento por separado.
- Se deben aplicar las obligaciones que correspondan.
- No se debe escoger arbitrariamente la regla menos restrictiva.
- No se debe asumir que una única configuración es suficiente para todos los tratamientos.

Las decisiones jurídicas complejas deben quedar documentadas y aprobadas por personal competente.

# 6. Registro, edad y protección de menores

## 6.1 Edad

Los formularios de registro deben contemplar la edad cuando sea necesaria para determinar elegibilidad y obligaciones de protección de menores.

No se debe solicitar fecha de nacimiento completa indiscriminadamente si una confirmación de mayoría de edad o un rango de edad es suficiente.

La implementación debe permitir:

- Confirmación de mayoría de edad.
- Verificación de edad cuando sea necesaria.
- Restricción de servicios según edad.
- Consentimiento parental verificable cuando la legislación lo requiera.
- Gestión de cuentas de menores cuando su uso esté permitido.
- Restricciones de marketing y perfilado.
- Restricciones de seguimiento y analítica.
- Protección reforzada de información personal.

## 6.2 COPPA

Para servicios sujetos a COPPA, especialmente aquellos dirigidos a menores de 13 años o que tengan conocimiento efectivo de que recopilan información personal de menores de esa edad:

- Evaluar si el servicio puede operar con menores.
- Implementar los mecanismos legales de aviso y consentimiento parental verificable cuando sean exigibles.
- Restringir la recopilación no necesaria.
- Respetar los derechos aplicables.
- Controlar proveedores que procesen información infantil.
- Aplicar reglas especiales de retención y eliminación.
- No habilitar publicidad comportamental o seguimiento prohibido.

No asumir que añadir una casilla de edad resuelve las obligaciones de COPPA.

## 6.3 Configuración por mercado

La edad mínima y los mecanismos de verificación no deben estar codificados como una constante global.

El sistema debe permitir aplicar los requisitos del mercado y servicio correspondientes.

Si ENSAMBRIA no puede cumplir las obligaciones necesarias para una categoría de usuarios, debe bloquear el registro o la funcionalidad correspondiente.

# 7. Consentimientos y preferencias

Debe existir un sistema centralizado para registrar decisiones de privacidad.

## 7.1 Categorías diferenciadas

Separar:

1. Aceptación de términos y condiciones.
2. Reconocimiento de política de privacidad.
3. Consentimiento para tratamientos que lo requieran.
4. Cookies y tecnologías de seguimiento.
5. Analítica no esencial.
6. Marketing por email.
7. Marketing por SMS.
8. Marketing por notificaciones push.
9. Personalización y perfilado.
10. Compartición de datos con terceros cuando corresponda.
11. Consentimiento parental cuando resulte aplicable.
12. Autorizaciones específicas para tratamientos sensibles.

No agrupar todas las decisiones en una única casilla obligatoria.

## 7.2 Evidencia de consentimiento

Registrar, según sea necesario:

- Usuario o sujeto relacionado.
- Tenant.
- Jurisdicción.
- Finalidad.
- Categoría de tratamiento.
- Versión del aviso.
- Versión del texto de consentimiento.
- Estado.
- Fecha y hora.
- Canal de obtención.
- Evidencia de la acción.
- Versión de la política.
- Retirada del consentimiento.
- Contexto técnico estrictamente necesario.

No conservar más información de la necesaria para demostrar la decisión.

La IP no debe registrarse automáticamente como evidencia universal de consentimiento. Su utilización debe tener una finalidad, base jurídica y plazo de retención justificados.

## 7.3 Retirada

La retirada debe ser:

- Accesible.
- Comprensible.
- Efectiva.
- Registrada.
- Respetada por todos los procesos posteriores.

Retirar un consentimiento no implica automáticamente borrar todos los datos, cuando existan otras bases jurídicas válidas para conservarlos.

Los procesos deben actualizarse para impedir nuevos tratamientos que dependan exclusivamente del consentimiento retirado.

# 8. Cookies, trackers y recursos externos

## 8.1 Política general

No se deben cargar tecnologías no esenciales antes de obtener el consentimiento cuando la jurisdicción lo exija.

Esto incluye, según su funcionamiento:

- Google Analytics.
- Meta Pixel.
- Herramientas de publicidad.
- Session replay.
- Heatmaps.
- SDK de seguimiento.
- Scripts de terceros.
- Tecnologías de identificación.
- Cookies publicitarias.
- Integraciones que transmitan datos personales.

No asumir que una herramienta es legal por estar ampliamente utilizada en el mercado.

## 8.2 Consent Management Platform

Implementar una capacidad de gestión de preferencias que permita:

- Aceptar categorías.
- Rechazar categorías no esenciales.
- Configurar preferencias.
- Consultar decisiones anteriores.
- Cambiar preferencias.
- Retirar consentimientos.
- Registrar versiones de avisos.
- Aplicar configuraciones por jurisdicción.

No utilizar interfaces engañosas ni dificultar el rechazo.

## 8.3 Fuentes externas

Evitar cargar automáticamente recursos desde servidores externos cuando puedan transmitir IP, identificadores u otros datos personales.

Revisar especialmente:

- Google Fonts.
- Bibliotecas JavaScript externas.
- Mapas.
- Videos embebidos.
- Captchas.
- Widgets.
- Scripts publicitarios.
- Recursos de analítica.

Cuando sea viable, utilizar recursos autoalojados o mecanismos que minimicen la exposición.

La autoalojación no elimina todas las obligaciones de privacidad, pero puede reducir transferencias innecesarias.

## 8.4 Google Fonts

Preferir almacenar las fuentes necesarias en infraestructura controlada por ENSAMBRIA, siempre que las licencias lo permitan.

No cargar automáticamente fuentes desde servidores de terceros sin evaluar las implicaciones jurídicas y técnicas correspondientes.

# 9. Analítica y grabación de sesiones

La analítica debe diseñarse con privacidad por defecto.

## 9.1 Prohibiciones

No capturar de forma indiscriminada:

- Contraseñas.
- Tokens.
- Datos de tarjetas.
- Datos bancarios.
- Documentos de identidad.
- Información médica.
- Información sensible.
- Mensajes privados.
- Campos de formularios que contengan datos personales.
- Contenido de reservas que no sea necesario para la finalidad analítica.

No se permite registrar pulsaciones de teclado de forma indiscriminada.

## 9.2 Session replay

Las herramientas de grabación de sesiones deben estar desactivadas por defecto hasta completar una evaluación de privacidad y legalidad.

Antes de habilitarlas se requiere:

- Evaluar la jurisdicción.
- Determinar la base jurídica.
- Obtener consentimiento cuando corresponda.
- Configurar exclusión de campos sensibles.
- Enmascarar contenido personal.
- Definir retención.
- Controlar acceso.
- Registrar proveedores y subencargados.
- Evaluar transferencias internacionales.
- Implementar mecanismos de exclusión.

No considerar que el consentimiento general de uso de la plataforma autoriza automáticamente la grabación de sesiones.

## 9.3 Eventos analíticos

Preferir eventos estructurados y minimizados en lugar de registrar contenido libre de los usuarios.

Ejemplo:

`booking_created`

Evitar enviar innecesariamente:

- Nombre del cliente.
- Email.
- Teléfono.
- Dirección.
- Información sensible.
- Contenido completo de formularios.

# 10. Comunicaciones y marketing

ENSAMBRIA debe diferenciar comunicaciones operativas de comunicaciones comerciales.

## 10.1 Comunicaciones transaccionales

Ejemplos:

- Confirmación de registro.
- Confirmación de reserva.
- Cambios de reserva.
- Recuperación de cuenta.
- Notificaciones de seguridad.
- Recibos.
- Información necesaria para ejecutar un contrato.

Deben limitarse a la finalidad operativa correspondiente.

No incorporar publicidad encubierta en comunicaciones transaccionales.

## 10.2 Marketing

Para email, SMS y notificaciones comerciales:

- Identificar el remitente.
- Mantener evidencia de autorización cuando sea necesaria.
- Incluir mecanismos de baja aplicables.
- Respetar exclusiones.
- Gestionar listas de supresión.
- Evitar reactivar contactos que se dieron de baja.
- Controlar la frecuencia.
- Identificar el propósito comercial.
- Cumplir los requisitos de identificación postal o empresarial que correspondan.

## 10.3 Legislación internacional

Evaluar:

- CAN-SPAM para Estados Unidos.
- CASL para Canadá.
- GDPR y ePrivacy para mercados europeos.
- Normativa colombiana aplicable.
- Legislación de comunicaciones comerciales de cada mercado.
- Requisitos locales para SMS y llamadas.

No utilizar una única plantilla de email como cumplimiento universal.

## 10.4 Lista de supresión

La baja debe propagarse a los sistemas de envío y a los proveedores correspondientes.

La información mínima necesaria para respetar una baja puede tener que conservarse de forma restringida incluso después de otras solicitudes de eliminación.

No volver a incorporar contactos dados de baja por una sincronización, importación o integración.

# 11. Suscripciones, renovaciones y cancelaciones

Aplica a:

- Suscripciones SaaS de ENSAMBRIA.
- Membresías comerciales de establecimientos.
- Paquetes y servicios recurrentes.
- Renovaciones automáticas.
- Compras recurrentes.

## 11.1 Transparencia antes de contratar

Antes de confirmar una contratación, mostrar de forma clara:

- Precio.
- Moneda.
- Periodicidad.
- Duración.
- Impuestos cuando corresponda.
- Fecha del primer cobro.
- Fecha o regla de renovación.
- Importe de renovaciones.
- Condiciones de cancelación.
- Restricciones relevantes.
- Política de reembolsos.
- Identidad del proveedor comercial.

No ocultar las condiciones relevantes en enlaces difíciles de encontrar.

## 11.2 Evidencia contractual

Registrar:

- Producto o plan.
- Precio y moneda.
- Periodicidad.
- Versión de términos.
- Fecha de contratación.
- Identidad del contratante.
- Consentimiento o confirmación cuando corresponda.
- Condiciones aceptadas.
- Estado de renovación.
- Historial de modificaciones.
- Historial de cancelación.

La evidencia histórica debe ser inmutable o conservarse mediante un historial auditable.

## 11.3 Renovaciones automáticas

Implementar controles para:

- Avisos previos cuando sean exigibles.
- Renovación transparente.
- Cambios de precio.
- Cancelación.
- Confirmación de cancelación.
- Revocación de autorizaciones de cobro cuando corresponda.
- Gestión de reembolsos.
- Reclamaciones.

## 11.4 Cancelación

La cancelación no debe depender de contactar obligatoriamente a un agente cuando la normativa exige una vía más sencilla.

El mecanismo debe ser accesible y proporcional al mecanismo de contratación.

No utilizar patrones oscuros, obstáculos artificiales ni opciones engañosas.

## 11.5 Separación de dominios

- PLATFORM es propietario de suscripciones y facturación SaaS ENSAMBRIA.
- BUSINESS es propietario de membresías, paquetes y suscripciones comerciales de los establecimientos.
- CORE ejecuta las operaciones centrales de pagos, idempotencia, ledger y conciliación según los contratos autorizados.

# 12. Derechos de los titulares y ciclo de vida de datos

Implementar una capacidad transversal de Privacy & Data Lifecycle Management.

Debe contemplar:

- Acceso.
- Rectificación.
- Actualización.
- Portabilidad o exportación cuando corresponda.
- Supresión.
- Oposición.
- Limitación.
- Retirada de consentimiento.
- Revisión de decisiones automatizadas cuando aplique.
- Otros derechos reconocidos por cada jurisdicción.

## 12.1 Ownership

CORE coordina la solicitud y controla identidad y autorización.

Cada backend ejecuta las operaciones sobre los datos de los que es propietario:

- CORE: identidad, autenticación, pertenencias y datos centrales.
- BUSINESS: clientes, reservas, CRM, membresías, inventario, pedidos y operaciones.
- PLATFORM: suscripciones SaaS, facturación y administración.
- PUBLIC: perfiles, publicaciones, reseñas y proyecciones públicas.

Un backend no debe modificar directamente las tablas privadas de otro backend para ejecutar una solicitud.

## 12.2 Procesamiento asíncrono

Las exportaciones y eliminaciones masivas deben ejecutarse mediante workers.

Requisitos:

- Lotes limitados.
- Keyset pagination.
- Checkpoints.
- Idempotencia.
- Reanudación.
- Transacciones cortas.
- Control de concurrencia.
- Backoff y jitter.
- Auditoría.
- Reintentos.
- Dead-letter queue cuando corresponda.

No ejecutar operaciones masivas de eliminación de manera síncrona desde una solicitud HTTP.

## 12.3 Conservación legal

No borrar automáticamente información que deba conservarse por obligaciones fiscales, contables, contractuales, regulatorias o defensa de reclamaciones.

Debe existir separación entre:

- Datos eliminables.
- Datos anonimizables.
- Datos sujetos a conservación.
- Datos sujetos a bloqueo o restricción.
- Datos sujetos a retención legal.

Toda excepción debe tener justificación, responsable y periodo de revisión.

## 12.4 Backups

La política debe contemplar qué sucede con las solicitudes de supresión cuando se restauran copias de seguridad.

Implementar mecanismos de reconciliación para volver a aplicar las supresiones y restricciones vigentes después de una restauración.

# 13. Seguridad y protección de información

La privacidad depende también de los controles de seguridad.

Requisitos obligatorios:

- TLS para comunicaciones externas e internas según el modelo de amenaza.
- Cifrado de almacenamiento y backups.
- Gestión de secretos mediante Vault o mecanismo seguro equivalente.
- Acceso de mínimo privilegio.
- Rotación de credenciales.
- MFA para operaciones administrativas sensibles.
- Protección contra IDOR.
- Aislamiento multi-tenant.
- Rate limiting.
- Protección contra abuso de APIs.
- Registro de auditoría.
- Control de sesiones.
- Revocación de tokens.
- Gestión de vulnerabilidades.
- Escaneo de dependencias.
- Gestión de incidentes.
- Backups y restauraciones verificadas.

No registrar secretos, tokens, contraseñas ni datos sensibles en logs.

## 13.1 Multi-tenancy

Toda consulta y operación debe verificar el contexto autorizado.

Las comprobaciones de tenant deben realizarse en backend.

No confiar en:

- `tenant_id` enviado por el frontend.
- Identificadores de URL.
- Headers no autenticados.
- Datos proporcionados por un proveedor externo.
- Campos de eventos como única prueba de identidad.

RLS, permisos de base de datos y controles de aplicación deben trabajar conjuntamente.

# 14. Proveedores, subencargados y transferencias internacionales

ENSAMBRIA debe mantener un inventario de terceros que procesen información.

Ejemplos:

- Proveedores de email.
- SMS.
- Pasarelas de pago.
- Analítica.
- Almacenamiento.
- CDN.
- Mapas.
- Autenticación.
- Monitoreo.
- Soporte.
- Servicios de IA.
- Infraestructura cloud.

## 14.1 Evaluación de proveedores

Antes de integrar un proveedor, documentar:

- Nombre.
- Servicio.
- Finalidad.
- Categorías de datos.
- Ubicación de procesamiento.
- Subencargados.
- Transferencias internacionales.
- Retención.
- Cifrado.
- Controles de acceso.
- Certificaciones o evidencias disponibles.
- Contrato de tratamiento.
- Mecanismos de eliminación.
- Procedimiento de incidentes.

No seleccionar proveedores únicamente por precio o facilidad de integración.

## 14.2 Contratos

Evaluar la necesidad de:

- Data Processing Agreement.
- Cláusulas contractuales internacionales.
- Mecanismos de transferencia reconocidos.
- Acuerdos de confidencialidad.
- Condiciones de subcontratación.
- Obligaciones de notificación de incidentes.

No asumir que alojar datos en un país determinado elimina las obligaciones de transferencia internacional.

## 14.3 Servicios de IA

Antes de enviar datos personales a servicios de IA:

- Identificar la finalidad.
- Evaluar la base jurídica.
- Minimizar la información.
- Evitar enviar información sensible innecesaria.
- Evaluar la ubicación de procesamiento.
- Revisar retención y entrenamiento.
- Revisar subencargados.
- Evaluar transferencias internacionales.
- Implementar controles de acceso y auditoría.

Los datos de clientes no deben enviarse automáticamente a modelos externos para entrenamiento.

# 15. Propiedad intelectual y contenido generado por usuarios

ENSAMBRIA permitirá que usuarios y establecimientos publiquen:

- Fotos de perfil.
- Fotografías de establecimientos.
- Imágenes de servicios.
- Fotografías de resultados.
- Logos.
- Videos.
- Reseñas.
- Descripciones.
- Material promocional.
- Otros contenidos.

## 15.1 Declaración de derechos

Los términos deben establecer que el usuario:

- Conserva los derechos que legalmente le correspondan.
- Declara tener autorización para subir y utilizar el contenido.
- Concede a ENSAMBRIA una licencia limitada para almacenar, procesar, mostrar y distribuir el contenido dentro de las funcionalidades autorizadas.
- No debe subir contenido que infrinja derechos de terceros.
- Puede estar sujeto a procedimientos de reclamación y retirada.

La licencia debe ser proporcional a la finalidad del servicio y no debe interpretarse como transferencia universal de propiedad.

## 15.2 Reclamaciones

Implementar un procedimiento para:

- Recibir reclamaciones.
- Identificar el contenido.
- Registrar la fecha.
- Evaluar la reclamación.
- Restringir o retirar contenido cuando corresponda.
- Notificar a las partes según la normativa aplicable.
- Gestionar contranotificaciones cuando proceda.
- Registrar decisiones.
- Gestionar reincidencia.

## 15.3 Estados del contenido

Utilizar estados controlados, por ejemplo:

- `pending_review`
- `published`
- `restricted`
- `under_claim`
- `removed`
- `restored`

Las transiciones deben ser auditables.

## 15.4 DMCA y Estados Unidos

Si ENSAMBRIA ofrece servicios sujetos a la DMCA y pretende acogerse a sus protecciones aplicables, debe evaluar los requisitos de elegibilidad y cumplimiento.

Entre otros, analizar:

- Designación y registro del agente DMCA ante la autoridad correspondiente.
- Publicación de datos de contacto.
- Procedimiento de notificación.
- Procedimiento de contranotificación.
- Política de infractores reincidentes.
- Respuesta adecuada a notificaciones válidas.

El registro de un agente DMCA no concede inmunidad automática ni elimina la responsabilidad por cualquier contenido.

## 15.5 Otros países

No asumir que DMCA es un procedimiento universal.

Debe existir un procedimiento adaptable a las leyes de propiedad intelectual y responsabilidad de intermediarios de cada jurisdicción.

# 16. Fotografías, imágenes y datos sensibles

Las imágenes pueden contener datos personales, información biométrica, datos de menores u otra información sensible.

Requisitos:

- Validar formatos y tamaños.
- Inspeccionar archivos.
- Eliminar metadatos EXIF cuando corresponda.
- Evitar publicar coordenadas GPS.
- Utilizar almacenamiento privado para material no público.
- Aplicar URLs firmadas y temporales cuando corresponda.
- Controlar permisos de visualización.
- Respetar solicitudes de retirada.
- Evitar reconocimiento facial sin evaluación jurídica y técnica específica.
- Evitar inferir atributos sensibles mediante IA.
- Definir retención y eliminación.

Una fotografía de un cliente no debe convertirse automáticamente en material publicitario del establecimiento.

Se deben gestionar autorizaciones específicas cuando la utilización de la imagen requiera consentimiento o autorización.

# 17. Datos sensibles y sector belleza, estética y bienestar

ENSAMBRIA puede atender negocios que trabajen con tratamientos estéticos, capilares, corporales y de bienestar.

Esto puede involucrar información especialmente protegida, según la legislación aplicable.

Ejemplos:

- Alergias.
- Contraindicaciones.
- Historial de tratamientos.
- Información dermatológica.
- Fotografías de antes y después.
- Información médica.
- Embarazo cuando se recopile.
- Reacciones adversas.
- Datos biométricos si se procesan.
- Información de salud.

## 17.1 Requisitos

No recopilar estos datos por defecto.

Antes de implementar un módulo que los procese:

- Justificar la necesidad.
- Identificar la base jurídica.
- Evaluar si se requiere consentimiento explícito.
- Definir acceso restringido.
- Definir retención.
- Evaluar riesgos.
- Evaluar transferencias.
- Determinar obligaciones profesionales y sectoriales.
- Aplicar controles adicionales.

## 17.2 Separación

Los datos sensibles deben estar separados lógica y funcionalmente de los datos operativos ordinarios.

No deben aparecer en:

- Logs.
- Analítica general.
- Session replay.
- Notificaciones públicas.
- URLs.
- Eventos de dominio innecesarios.
- Herramientas de IA no autorizadas.

La información sensible no debe compartirse entre tenants.

# 18. Logs, observabilidad y auditoría

Los logs no son un depósito de datos personales.

Deben registrar lo necesario para seguridad, diagnóstico y operación.

Prohibido registrar directamente:

- Contraseñas.
- Tokens completos.
- Secretos.
- Datos de tarjetas.
- Documentos de identidad completos.
- Datos sensibles.
- Contenido privado de conversaciones.
- Formularios completos.

Utilizar:

- Identificadores internos.
- Correlation IDs.
- Event IDs.
- Códigos de error.
- Metadatos mínimos.
- Redacción y enmascaramiento.

## 18.1 Auditoría

Las operaciones sensibles deben generar evidencias de:

- Actor.
- Acción.
- Recurso.
- Tenant.
- Resultado.
- Fecha.
- Contexto autorizado.
- Correlation ID.
- Motivo cuando corresponda.

La auditoría debe protegerse contra alteraciones no autorizadas.

Definir retención, acceso y eliminación conforme a los requisitos legales y operativos.

# 19. Conservación y eliminación

Cada categoría de datos debe tener una política de retención.

No establecer un periodo global para todos los datos.

Ejemplos de categorías:

- Credenciales.
- Sesiones.
- Datos de clientes.
- Reservas.
- Historial comercial.
- Facturas.
- Registros financieros.
- Evidencias contractuales.
- Consentimientos.
- Logs.
- Auditoría.
- Imágenes.
- Exportaciones.
- Backups.
- Datos de marketing.

Cada categoría debe documentar:

- Finalidad.
- Responsable.
- Base jurídica.
- Plazo o criterio.
- Evento de inicio del plazo.
- Método de eliminación.
- Excepciones.
- Evidencia de ejecución.

La eliminación debe incluir sistemas secundarios, índices, cachés, archivos y proveedores cuando corresponda.

No prometer una eliminación inmediata de backups cuando la arquitectura utiliza ciclos de retención. Definir un procedimiento verificable de expiración y reconciliación.

# 20. Gestión de incidentes y brechas de seguridad

Implementar un procedimiento global de respuesta a incidentes.

Debe incluir:

1. Detección.
2. Clasificación.
3. Contención.
4. Investigación.
5. Evaluación del impacto.
6. Identificación de datos y titulares afectados.
7. Determinación de jurisdicciones.
8. Evaluación de obligaciones de notificación.
9. Comunicación a autoridades y titulares cuando corresponda.
10. Recuperación.
11. Registro de evidencias.
12. Revisión posterior.

No utilizar un plazo universal de notificación.

Los plazos dependen de la jurisdicción, naturaleza del incidente y obligaciones aplicables.

Debe existir un responsable operativo y un procedimiento para activar asesoría jurídica.

# 21. Accesibilidad y transparencia de interfaces

Las interfaces deben presentar información comprensible y accesible.

Requisitos:

- Lenguaje claro.
- Precios transparentes.
- Condiciones contractuales accesibles.
- Consentimientos específicos.
- Preferencias fáciles de modificar.
- Cancelaciones accesibles.
- Avisos de privacidad comprensibles.
- Traducciones revisadas.
- Compatibilidad con tecnologías de asistencia.
- Ausencia de patrones oscuros.

La traducción automática de textos legales no debe considerarse aprobación jurídica.

Cada versión legal debe revisarse para el mercado correspondiente.

# 22. Internacionalización de datos y contenido legal

ENSAMBRIA debe separar idioma, moneda, país, jurisdicción y zona horaria.

No utilizar el idioma de interfaz para inferir automáticamente la legislación aplicable.

Cada texto legal debe tener:

- Identificador.
- Idioma.
- Jurisdicción.
- Versión.
- Fecha de vigencia.
- Estado.
- Responsable de aprobación.
- Relación con los tratamientos o contratos correspondientes.

Los textos históricos deben permanecer disponibles para demostrar qué condiciones fueron presentadas en un momento determinado.

# 23. Seguridad de pagos y datos financieros

ENSAMBRIA debe minimizar su exposición a datos de tarjetas.

Preferir integraciones tokenizadas y componentes alojados por proveedores de pago cuando sean adecuados.

No almacenar CVV ni datos de autenticación sensibles de tarjetas.

Evaluar PCI DSS según las funciones y responsabilidades reales de ENSAMBRIA y sus proveedores.

Separar:

- Pagos de clientes a establecimientos.
- Pagos de establecimientos a ENSAMBRIA.
- Suscripciones SaaS.
- Comisiones de ENSAMBRIA.
- Comisiones de proveedores de pago.
- Reembolsos.
- Contracargos.
- Liquidaciones.

La información financiera debe tener controles de acceso, trazabilidad y retención propios.

# 24. Arquitectura técnica de implementación

La implementación debe respetar DDD y Clean Architecture.

No introducir lógica legal directamente en controladores HTTP ni duplicar reglas entre frontends.

Se recomienda separar las siguientes capacidades lógicas:

- `privacy`
- `consent`
- `jurisdiction`
- `legal_policies`
- `data_lifecycle`
- `communication_preferences`
- `compliance_audit`
- `content_rights`
- `provider_governance`

Estas capacidades no implican necesariamente crear nuevos microservicios o crates independientes desde la primera fase.

Deben existir límites de dominio claros y contratos explícitos.

## 24.1 Base de datos

Definir ownership por backend.

Cada backend mantiene sus migraciones y permisos.

No permitir que un backend modifique directamente tablas de otro.

Utilizar:

- UUID v7 cuando corresponda al estándar de identificadores.
- Restricciones de integridad.
- Índices apropiados.
- RLS cuando corresponda.
- Historial auditable.
- Cifrado para campos sensibles cuando sea necesario.
- Retención y limpieza por lotes.

## 24.2 Eventos

Las operaciones que deban comunicar cambios a otros backends deben utilizar el patrón Transactional Outbox e Inbox o un mecanismo equivalente formalmente justificado.

Los eventos deben transportar exclusivamente la información necesaria.

No transmitir datos sensibles en eventos generales.

Los consumidores deben verificar identidad del productor, autorización, tenant y contexto del evento.

## 24.3 Workers

Los procesos asíncronos deben gestionar:

- Solicitudes de titulares.
- Exportaciones.
- Supresiones.
- Retención.
- Eliminación de archivos.
- Sincronización de preferencias.
- Propagación de bajas comerciales.
- Revisión de proveedores.
- Reconciliación de datos.
- Notificaciones de incidentes.

Todos deben ser idempotentes, reanudables y observables.

# 25. Seguridad de frontend

Cada frontend debe implementar controles apropiados:

- No exponer secretos.
- No incluir credenciales privadas en bundles.
- No cargar trackers no autorizados.
- No almacenar información sensible innecesariamente.
- No utilizar almacenamiento local como sustituto de autorización.
- Proteger rutas privadas.
- Aplicar CSP.
- Evitar XSS.
- Validar y escapar contenido.
- Gestionar cookies de forma segura.
- Controlar redirecciones.
- No publicar información privada en metadatos SEO u Open Graph.

El servidor debe producir las respuestas autorizadas. Ocultar un botón o una ruta no constituye control de acceso.

# 26. Pruebas obligatorias

Los requisitos de privacidad y cumplimiento deben ser verificables mediante pruebas.

No se considera terminado un control por existir solamente en documentación.

## 26.1 Pruebas unitarias

Cubrir:

- Evaluación de políticas por jurisdicción.
- Versiones legales.
- Consentimientos.
- Retiradas.
- Preferencias.
- Reglas de edad.
- Restricciones de tratamiento.
- Retención.
- Estados de solicitudes.
- Cancelaciones.
- Supresión comercial.
- Autorizaciones de contenido.

## 26.2 Pruebas de integración

Cubrir:

- Registro y elegibilidad por edad.
- Persistencia de consentimientos.
- Revocación y propagación.
- Aislamiento multi-tenant.
- Solicitudes de exportación.
- Supresión y anonimización.
- Gestión de datos conservables.
- Proveedores externos.
- Propagación de bajas.
- Acceso a datos sensibles.
- Auditoría.
- Reconciliación después de restauraciones.

Utilizar servicios reales de prueba cuando sea necesario. Los mocks no deben sustituir las pruebas críticas de persistencia, aislamiento o seguridad.

## 26.3 Pruebas de frontend

Verificar:

- Que los trackers no esenciales no se carguen antes del consentimiento cuando corresponda.
- Que rechazar sea una opción efectiva.
- Que las preferencias puedan modificarse.
- Que no se expongan datos privados en HTML inicial.
- Que no se filtren datos en metadatos.
- Que las condiciones de suscripción se muestren antes de contratar.
- Que la cancelación funcione.
- Que los mecanismos de baja comercial sean efectivos.

## 26.4 Pruebas de seguridad

Cubrir:

- IDOR.
- Escalada de privilegios.
- Acceso cruzado entre tenants.
- Acceso a datos sensibles.
- Manipulación de consentimientos.
- Falsificación de identidad del productor.
- Exposición de tokens.
- SSRF.
- XSS.
- Fugas de datos en logs.
- Exposición de archivos privados.
- Uso indebido de exportaciones.
- Acceso administrativo no autorizado.

## 26.5 Pruebas de resiliencia

Verificar:

- Reintentos.
- Duplicados.
- Concurrencia.
- Caídas de workers.
- Fallos de proveedores.
- Restauración de backups.
- Reanudación de exportaciones.
- Reanudación de eliminaciones.
- Reconciliación de preferencias.

## 26.6 Evidencia

Mantener una relación verificable:

`Requisito -> Control -> Implementación -> Prueba -> Ejecución CI -> Evidencia`

No declarar cumplimiento completo por tener pruebas unitarias aisladas.

# 27. Gestión de cambios normativos

Las leyes y sus interpretaciones pueden cambiar.

ENSAMBRIA debe mantener un proceso de revisión:

- Seguimiento de cambios normativos.
- Revisión periódica por jurisdicción.
- Evaluación de impacto técnico.
- Actualización de políticas.
- Actualización de textos legales.
- Actualización de pruebas.
- Comunicación a responsables.
- Plan de migración.
- Registro de aprobación.

No permitir que un cambio de texto legal modifique silenciosamente el comportamiento de sistemas productivos.

Los cambios que afecten tratamientos, consentimiento, contratos o derechos deben tener control de versiones y aprobación.

# 28. Proceso de habilitación de un nuevo país

Ningún mercado debe considerarse listo únicamente porque el frontend esté traducido.

Antes de habilitar comercialmente un nuevo país, ejecutar una evaluación de entrada al mercado.

Lista obligatoria:

- [ ] Identificar legislación de privacidad.
- [ ] Identificar legislación de protección al consumidor.
- [ ] Identificar reglas de menores.
- [ ] Identificar reglas de marketing.
- [ ] Identificar reglas de cookies y seguimiento.
- [ ] Identificar reglas de contratación y renovaciones.
- [ ] Identificar reglas fiscales y de facturación.
- [ ] Identificar requisitos de pagos y regulación financiera.
- [ ] Identificar requisitos de propiedad intelectual.
- [ ] Identificar transferencias internacionales.
- [ ] Identificar obligaciones de notificación de incidentes.
- [ ] Revisar proveedores disponibles.
- [ ] Revisar contratos y avisos.
- [ ] Configurar políticas por jurisdicción.
- [ ] Implementar traducciones revisadas.
- [ ] Ejecutar pruebas automatizadas.
- [ ] Obtener aprobación legal y de negocio cuando corresponda.
- [ ] Documentar autorización de lanzamiento.

Si existen obligaciones críticas sin resolver, el mercado o la funcionalidad correspondiente debe permanecer deshabilitado.

# 29. Documentación obligatoria

Mantener documentación versionada:

- `docs/compliance/PRIVACY_POLICY.md`
- `docs/compliance/LEGAL_JURISDICTIONS.md`
- `docs/compliance/DATA_PROCESSING_REGISTER.md`
- `docs/compliance/DATA_RETENTION.md`
- `docs/compliance/CONSENT_MANAGEMENT.md`
- `docs/compliance/COOKIE_POLICY.md`
- `docs/compliance/CHILDREN_AND_AGE_PROTECTION.md`
- `docs/compliance/SESSION_REPLAY_AND_ANALYTICS.md`
- `docs/compliance/MARKETING_COMPLIANCE.md`
- `docs/compliance/SUBSCRIPTIONS_AND_AUTO_RENEWAL.md`
- `docs/compliance/INTERNATIONAL_DATA_TRANSFERS.md`
- `docs/compliance/THIRD_PARTY_PROCESSORS.md`
- `docs/compliance/DATA_SUBJECT_REQUESTS.md`
- `docs/compliance/INCIDENT_RESPONSE.md`
- `docs/compliance/CONTENT_AND_COPYRIGHT.md`
- `docs/compliance/MARKET_LAUNCH_CHECKLIST.md`
- `docs/compliance/COMPLIANCE_TEST_MATRIX.md`

Los documentos son una referencia técnica y operativa. No deben presentarse como asesoría jurídica ni como certificación automática de cumplimiento.

# 30. Reglas para el agente de desarrollo

El agente que trabaje en ENSAMBRIA debe cumplir estas instrucciones:

1. Inspeccionar el repositorio antes de modificar archivos.
2. Revisar la arquitectura y los contratos existentes.
3. Identificar los cuatro backends y seis proyectos de interfaz.
4. Respetar ownership de cada dominio.
5. No duplicar capacidades existentes.
6. No crear microservicios innecesarios.
7. No introducir proveedores externos sin evaluación.
8. No inventar obligaciones legales ni umbrales.
9. No asumir que una norma aplica universalmente.
10. No inventar obligaciones, hechos de producto, decisiones comerciales ni datos de la entidad. La falta de revisión profesional no impide investigar, redactar, desarrollar o integrar los documentos y mecanismos. Registrar las cuestiones no resueltas y limitar el bloqueo al tratamiento, mercado o decisión que dependa específicamente de ellas.
11. No habilitar tracking, session replay o publicidad sin evaluación.
12. No recopilar información personal innecesaria.
13. No almacenar información sensible en logs.
14. No confiar en validaciones exclusivamente frontend.
15. No utilizar mocks para sustituir controles críticos.
16. No introducir TODOs o implementaciones ficticias.
17. No reducir la rigurosidad de las pruebas.
18. No modificar scripts, workflows o infraestructura fuera del alcance aprobado.
19. Actualizar documentación y roadmap con el estado real.
20. Ejecutar pruebas locales y CI.
21. Corregir fallos introducidos por los cambios.
22. No declarar una fase terminada sin evidencia.

Antes de implementar cada obligación legal, el agente debe identificar:

- Jurisdicción.
- Norma y fuente oficial.
- Obligación concreta.
- Funcionalidad afectada.
- Responsable del tratamiento.
- Base jurídica aplicable.
- Control técnico.
- Evidencia necesaria.
- Pruebas.
- Aprobación pendiente, si existe.

Si no existe certeza legal suficiente, debe documentar la fuente consultada, la pregunta pendiente, el responsable de resolverla y la evidencia requerida. Puede continuar el trabajo documental y técnico independiente; no debe habilitar únicamente el tratamiento o mercado cuyo requisito crítico dependa de esa respuesta.

# 31. Criterios de aceptación (Definition of Done)

Una capacidad de privacidad o cumplimiento se considera terminada únicamente cuando:

- Existe un requisito documentado.
- Existe ownership definido.
- Existe implementación real.
- Existe autorización backend.
- Existe aislamiento multi-tenant.
- Existe persistencia cuando corresponde.
- Existe trazabilidad.
- Existe tratamiento de errores.
- Existe comportamiento seguro ante fallos.
- Existen pruebas unitarias.
- Existen pruebas de integración.
- Existen pruebas de seguridad.
- Existen pruebas de frontend cuando corresponda.
- La documentación está actualizada.
- Los contratos están versionados.
- CI está en verde.
- No existen secretos expuestos.
- No existen datos sensibles en logs.
- Las decisiones jurídicas pendientes están identificadas.
- La aprobación correspondiente está documentada.

**No confundir implementación técnica con aprobación legal.** Ambas son necesarias cuando la funcionalidad está sujeta a obligaciones jurídicas.

# 32. Estrategia de fases

## Fase 0 — Fundamentos documentales

Obligatorio como fundación documental antes de implementar los recorridos afectados. La investigación y redacción se realizan durante el desarrollo; la ausencia de revisión profesional no bloquea la documentación ni las tareas independientes. Fase 0 continúa siendo documental y no introduce runtime:

- Definir ownership de datos.
- Definir inventario inicial de tratamientos.
- Establecer política de privacidad inicial.
- Establecer términos de uso.
- Identificar jurisdicción inicial y asignar explícitamente responsables de privacidad/cumplimiento y Legal.
- Crear un registro de obligaciones por jurisdicción y tratamiento con fuente oficial vigente, aplicabilidad por actividad/rol, estado de validación Legal, propietario, control técnico, prueba/evidencia y bloqueo/fecha objetivo para preguntas pendientes.
- Diseñar modelo de consentimiento.
- Definir política de cookies y trackers.
- Definir política de logs.
- Definir tratamiento de datos sensibles.
- Definir retención.
- Identificar proveedores, subencargados, ubicaciones y transferencias internacionales.
- Establecer controles básicos de seguridad.
- Crear matriz inicial de pruebas.
- Registrar para cada documento la revisión profesional pendiente y preparar su revisión posterior a la implementación técnica. No afirmar que está aprobada. Antes de habilitar un tratamiento concreto, aplicar únicamente las validaciones previas que correspondan a una obligación crítica identificada para ese tratamiento.
- Relacionar estos entregables con PDLM, el roadmap ENSAMBRIA, su matriz de datos y los ADRs; mantenerlos como artefactos documentales, sin introducir runtime de producto en Fase 0. En la fase que introduce cada ruta o tratamiento, integrar y probar el documento y el aviso visible de revisión pendiente; no esperar a una aprobación profesional general para implementar la superficie.

## Fases de implementación posteriores

Desde la primera fase de implementación, construir y probar en el mismo corte ejecutable los controles aplicables al tratamiento y comportamiento que se introduce, incluidos autorización/aislamiento, minimización, consentimiento o preferencias cuando correspondan, retención, gestión de errores/incidentes y redacción. No posponer un control requerido a una fase de madurez por conveniencia. Las capacidades más amplias se implementan progresivamente en las fases propietarias del roadmap:

- Gestión completa de consentimientos.
- Motor de jurisdicciones.
- Protección de menores.
- Preferencias de comunicación.
- Gestión de cookies.
- Solicitudes de titulares.
- Exportación y eliminación asíncronas.
- Gestión de proveedores.
- Transferencias internacionales.
- Auditoría y evidencias.
- Gestión de contenido y copyright.
- Cumplimiento de suscripciones.
- Automatización de retención.
- Gestión de incidentes.
- Habilitación de nuevos mercados.

No aplazar controles fundamentales de seguridad o privacidad con el argumento de que pertenecen a una fase posterior.

# 33. Regla final

ENSAMBRIA debe construirse para que el cumplimiento sea una propiedad de la arquitectura y de la operación, no una corrección improvisada después de recibir una reclamación.

La plataforma debe poder demostrar:

- Qué datos recopila.
- Para qué los recopila.
- Quién es responsable.
- Quién puede acceder.
- Dónde se procesan.
- Con quién se comparten.
- Qué autorización o base jurídica sustenta el tratamiento.
- Cuánto tiempo se conservan.
- Cómo se rectifican, exportan o eliminan.
- Qué sucede cuando una persona retira su consentimiento.
- Qué sucede cuando un proveedor falla.
- Qué sucede cuando se produce una brecha.
- Qué obligaciones aplican en cada mercado.

La expansión internacional debe realizarse mediante políticas y controles verificables por jurisdicción, no mediante excepciones dispersas en el código.

**Principio de lanzamiento:**

Un país, funcionalidad o tratamiento que no tenga sus requisitos críticos identificados, implementados y aprobados no debe habilitarse comercialmente hasta resolverlos.

**Objetivo final:** construir una plataforma ENSAMBRIA internacional, segura, transparente, auditable y preparada para crecer sin sacrificar la privacidad ni los derechos de sus usuarios, establecimientos, empleados y clientes.
