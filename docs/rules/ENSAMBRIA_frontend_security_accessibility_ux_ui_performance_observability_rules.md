# Frontend Security, Accessibility, UX/UI, Performance and Observability Rules

**Clasificación:** Estándar técnico obligatorio y no negociable.<br>
**Aplicación:** Desde la Fase 0 durante todo el ciclo de vida del proyecto.<br>
**Ámbito:** Todos los productos frontend web, aplicaciones SPA, SSR, PWA y aplicaciones móviles.<br>
**Observabilidad:** Sentry obligatorio.<br>
**Accesibilidad web:** WCAG 2.2 nivel AA como mínimo.

---

# 1. Propósito

Establecer las reglas obligatorias para diseñar, desarrollar, proteger, probar, observar y mantener todos los frontends del proyecto.

El frontend es una superficie crítica de seguridad, privacidad y operación. No debe considerarse únicamente una capa visual que consume APIs.

Cada frontend debe garantizar:

- Seguridad del lado del cliente.
- Protección de sesiones e información sensible.
- Integración segura con los backends.
- Aislamiento correcto de la información.
- Accesibilidad.
- Experiencia de usuario coherente.
- Diseño visual consistente.
- Rendimiento medible.
- Observabilidad de errores y comportamiento.
- Integración con Sentry.
- Pruebas automatizadas y manuales.
- Compatibilidad con dispositivos y tecnologías de asistencia.

Estas reglas son obligatorias desde la primera pantalla y el primer componente.

No se permite posponer seguridad, accesibilidad, observabilidad o calidad de UX/UI hasta una fase de optimización.

# 2. Principios fundamentales no negociables

Todo frontend debe cumplir estos principios:

1. El frontend nunca es una frontera de seguridad suficiente.
2. Ninguna autorización puede depender exclusivamente del código del cliente.
3. Ningún secreto del servidor puede exponerse al navegador o a la aplicación móvil.
4. Toda información sensible debe protegerse durante su transmisión, almacenamiento y visualización.
5. Todo componente interactivo debe ser accesible.
6. Toda interacción debe proporcionar retroalimentación comprensible.
7. Toda interfaz debe contemplar estados exitosos, fallidos y excepcionales.
8. Todo frontend debe mantener consistencia mediante un sistema de diseño.
9. Todo frontend debe tener observabilidad real mediante Sentry.
10. Los errores relevantes no deben quedar ocultos.
11. Toda implementación debe incluir pruebas.
12. Ningún cambio puede degradar deliberadamente la seguridad, accesibilidad, rendimiento o experiencia de usuario.
13. El frontend nunca debe confiar en datos no verificados provenientes del cliente.
14. Las interfaces deben funcionar correctamente en los dispositivos y tamaños de pantalla soportados.
15. La información privada nunca debe filtrarse mediante errores, logs, analítica, cachés o herramientas de observabilidad.

# 3. Arquitectura y organización del frontend

Todo frontend debe mantener una arquitectura organizada, modular y coherente con las reglas generales del proyecto.

Reglas obligatorias:

- Separar presentación, estado, lógica de aplicación y acceso a datos.
- No concentrar toda la lógica en componentes visuales.
- Evitar componentes monolíticos.
- Separar componentes reutilizables de componentes específicos de una funcionalidad.
- Centralizar el acceso a APIs mediante clientes o adaptadores definidos.
- Centralizar el manejo de errores.
- Centralizar autenticación y gestión de sesiones.
- Mantener separación entre funcionalidades y dominios.
- Evitar dependencias circulares.
- Evitar duplicaciones innecesarias.
- Mantener contratos coherentes con los backends.
- No utilizar el estado visual como sustituto de las reglas de negocio.
- No acoplar componentes a detalles internos de servicios backend.

La arquitectura debe permitir pruebas independientes de componentes, lógica y flujos de usuario.

## 3.1. Separación por producto

Cuando existan varios frontends, cada uno debe tener responsabilidades y límites definidos.

No compartir automáticamente estados, sesiones, permisos o cachés entre productos.

La reutilización debe realizarse mediante componentes, bibliotecas o contratos compartidos claramente definidos.

# 4. Seguridad del frontend

## 4.1. Principio de confianza cero

El frontend debe considerarse un entorno controlado por el usuario y potencialmente comprometible.

Un atacante puede:

- Modificar JavaScript.
- Manipular solicitudes HTTP.
- Alterar parámetros.
- Ejecutar herramientas del navegador.
- Cambiar el almacenamiento local.
- Modificar el estado de la aplicación.
- Reproducir solicitudes.
- Invocar directamente los endpoints.
- Alterar identificadores y datos enviados al backend.

Por tanto:

**Ninguna validación, permiso o restricción implementada exclusivamente en frontend puede considerarse un control de seguridad definitivo.**

El backend debe validar y autorizar todas las operaciones.

## 4.2. Autenticación

Los mecanismos de autenticación deben utilizar protocolos y flujos seguros.

Reglas obligatorias:

1. Utilizar OAuth 2.0 y OpenID Connect cuando corresponda.
2. Para clientes públicos, utilizar Authorization Code Flow con PKCE cuando aplique.
3. No utilizar el flujo implícito como mecanismo de autenticación.
4. No implementar mecanismos criptográficos propios.
5. No incluir contraseñas, tokens o credenciales en URLs.
6. No registrar credenciales en consola ni en herramientas de observabilidad.
7. No considerar una pantalla de login como protección de un endpoint.
8. No almacenar credenciales de usuarios de manera insegura.
9. Gestionar expiración y renovación de sesiones de forma explícita.
10. Manejar correctamente cierre de sesión y revocación mediante los contratos del backend.

### Aplicaciones web

Se debe preferir un patrón BFF con sesiones gestionadas mediante cookies seguras cuando la arquitectura lo permita.

Las cookies de sesión deben utilizar:

- `HttpOnly`.
- `Secure`.
- `SameSite` con una política apropiada al flujo.
- Alcance mínimo de dominio y ruta.
- Expiración controlada.
- Protección CSRF cuando corresponda.

No se deben almacenar tokens de sesión, refresh tokens o credenciales persistentes en `localStorage`, `sessionStorage` o IndexedDB.

Cuando un diseño sin BFF requiera tokens en una SPA, debe existir una decisión arquitectónica documentada que defina el almacenamiento temporal, renovación, mitigaciones de XSS, rotación y revocación.

### Aplicaciones móviles

Las credenciales persistentes deben guardarse exclusivamente mediante mecanismos seguros de la plataforma, como Keychain en iOS o Keystore en Android, mediante bibliotecas mantenidas y apropiadas.

Queda prohibido guardar refresh tokens o credenciales en almacenamiento genérico sin protección.

Para autenticación móvil se debe utilizar un flujo compatible con Authorization Code y PKCE mediante el navegador del sistema cuando corresponda.

## 4.3. Autorización y permisos

Las restricciones visuales de acceso no sustituyen la autorización del backend.

Reglas:

- No confiar en roles enviados por el cliente.
- No confiar en permisos guardados únicamente en el estado frontend.
- No permitir que modificar una variable o ruta habilite operaciones.
- No permitir que un identificador de tenant proporcionado por el navegador determine el acceso.
- Verificar que el backend valide ownership y permisos.
- Ocultar funcionalidades no autorizadas únicamente como mejora de UX, nunca como control de seguridad.
- Manejar correctamente respuestas `401` y `403`.
- No reutilizar información de una sesión anterior después de cambiar de usuario o tenant.

La interfaz debe reflejar el estado de autorización vigente, pero el backend conserva la autoridad definitiva.

## 4.4. Protección contra XSS

Todo dato externo debe considerarse potencialmente no confiable.

Reglas obligatorias:

1. No utilizar `eval()`.
2. No utilizar `new Function()` para ejecutar código dinámico.
3. Evitar `innerHTML` con información no confiable.
4. Evitar mecanismos equivalentes que permitan inyectar HTML sin sanitización.
5. No utilizar `dangerouslySetInnerHTML` sin una justificación y sanitización estricta.
6. Sanitizar HTML enriquecido mediante una biblioteca mantenida y una política de elementos y atributos permitidos.
7. Validar URLs antes de utilizarlas como enlaces o destinos de navegación.
8. Rechazar esquemas peligrosos como `javascript:`.
9. Evitar generar scripts a partir de información externa.
10. No renderizar Markdown no confiable con configuraciones que permitan HTML o ejecución de contenido peligroso.
11. No asumir que un framework frontend protege todos los contextos de inyección.
12. No interpolar datos externos dentro de código JavaScript, CSS o atributos sensibles.

Utilizar mecanismos de renderizado seguros del framework siempre que sea posible.

## 4.5. Content Security Policy y cabeceras de seguridad

Los frontends web deben desplegarse con políticas de seguridad adecuadas.

Como mínimo, se debe evaluar e implementar:

- Content-Security-Policy.
- `frame-ancestors` para prevenir embedding no autorizado.
- `X-Content-Type-Options: nosniff`.
- `Referrer-Policy`.
- `Permissions-Policy`.
- HSTS en producción.
- Cookies seguras.
- Restricciones de orígenes y recursos externos.

Reglas:

1. CSP debe tener una política restrictiva.
2. No utilizar `unsafe-eval` como solución ordinaria.
3. No utilizar `unsafe-inline` indiscriminadamente.
4. Preferir nonces o hashes cuando sean necesarios para scripts.
5. Restringir los orígenes de scripts, imágenes, conexiones, fuentes y otros recursos.
6. No permitir que dominios externos no aprobados ejecuten JavaScript en la aplicación.
7. Revisar todas las excepciones de CSP.
8. Probar las políticas antes de aplicarlas en modo de bloqueo.
9. Mantener una política de reportes para detectar violaciones cuando corresponda.

Las cabeceras deben establecerse y verificarse en el servidor, CDN, gateway o plataforma de despliegue correspondiente. El frontend debe definir sus requisitos y comprobar su efectividad, pero no debe asumir que incluir una configuración en el código garantiza que el servidor la aplique.

## 4.6. CSRF y solicitudes HTTP

Cuando se utilicen cookies como mecanismo de autenticación, se deben implementar las defensas CSRF apropiadas.

Reglas:

- No utilizar GET para operaciones que modifiquen estado.
- Aplicar tokens anti-CSRF cuando corresponda.
- Validar origen cuando el diseño lo requiera.
- Configurar cookies SameSite de manera adecuada.
- No confiar exclusivamente en CORS como defensa CSRF.
- No permitir solicitudes de origen arbitrario.
- Utilizar HTTPS.
- No desactivar validaciones TLS.
- Evitar redirecciones externas no validadas.
- No filtrar credenciales en parámetros de URL.

## 4.7. Gestión de datos sensibles

Está prohibido incluir información sensible en:

- URLs.
- Parámetros de navegación.
- Logs.
- Mensajes de error.
- Eventos analíticos.
- Breadcrumbs.
- Capturas de pantalla automáticas.
- Reportes de errores.
- Session Replay sin controles adecuados.
- Almacenamiento local sin necesidad justificada.

Los datos sensibles incluyen, entre otros:

- Contraseñas.
- Tokens.
- Información financiera.
- Datos de pago.
- Información personal.
- Datos privados de clientes.
- Documentos de identidad.
- Datos médicos o estéticos confidenciales.
- Direcciones y teléfonos.
- Información contractual.
- Secretos de negocio.

Recoger únicamente la información necesaria para la finalidad funcional.

No mantener información sensible en memoria o almacenamiento del frontend más tiempo del estrictamente necesario.

## 4.8. Aislamiento multi-tenant

En plataformas multi-tenant, la interfaz debe evitar cualquier filtración de información entre tenants.

Reglas obligatorias:

1. No considerar el tenant seleccionado como prueba de autorización.
2. No confiar en `tenant_id` enviado desde el frontend.
3. No mezclar cachés de usuarios o tenants distintos.
4. Incluir el contexto de tenant apropiado en las claves de caché cuando corresponda.
5. Invalidar o limpiar información privada al cerrar sesión.
6. Limpiar o cambiar el contexto de datos al cambiar de tenant.
7. Evitar que estados persistentes de una sesión se reutilicen en otra.
8. No mostrar datos de un tenant anterior durante la carga del siguiente.
9. No permitir que la navegación directa revele información privada.
10. Verificar con pruebas la transición entre tenants.

La autorización y el aislamiento definitivos deben verificarse en el backend.

## 4.9. Protección de pagos

Cuando el frontend participe en operaciones financieras:

- No almacenar números completos de tarjetas.
- No almacenar CVV.
- No registrar datos de tarjetas.
- No enviar credenciales de proveedores al cliente.
- Utilizar componentes tokenizados u hosted fields oficiales del proveedor cuando corresponda.
- No confiar en el importe, moneda, descuento o comisión calculados en el frontend.
- Mostrar el estado confirmado por el backend.
- Prevenir envíos accidentales repetidos desde la interfaz.
- Mantener estados de pago claros.
- No declarar pagos exitosos por una respuesta visual o una redirección no verificada.

El frontend nunca debe reemplazar la idempotencia y la validación financiera del backend.

## 4.10. Dependencias y código de terceros

Todo paquete, SDK, script y recurso de terceros representa una posible superficie de ataque.

Reglas:

- Revisar dependencias antes de introducirlas.
- Mantener versiones controladas y lockfiles.
- Evaluar vulnerabilidades.
- Revisar mantenimiento y reputación.
- Evitar dependencias abandonadas.
- Minimizar scripts de terceros.
- No introducir trackers desconocidos.
- No cargar código externo innecesario.
- Aplicar Subresource Integrity cuando sea compatible y pertinente.
- No conceder acceso innecesario a terceros.
- Revisar las actualizaciones críticas.
- Evitar bibliotecas que ejecuten código remoto dinámico sin controles.

Las herramientas de analítica, publicidad, chat, mapas y atención al cliente deben pasar por una revisión de seguridad y privacidad.

# 5. Privacidad y protección de información

La privacidad debe formar parte del diseño desde la Fase 0.

Reglas obligatorias:

1. Aplicar minimización de datos.
2. Recoger únicamente información necesaria.
3. Definir finalidad para cada dato recopilado.
4. No almacenar datos privados indefinidamente.
5. No incluir información sensible en analítica sin autorización y controles.
6. No realizar fingerprinting invasivo.
7. No activar trackers no esenciales sin cumplir los requisitos de consentimiento aplicables.
8. Permitir gestionar preferencias de privacidad cuando corresponda.
9. Respetar los procesos de exportación, eliminación y retención definidos por el sistema.
10. No mantener datos privados en caché después de cerrar sesión.
11. No utilizar información productiva real en ejemplos, demos o pruebas sin autorización y protección apropiadas.

La política de privacidad y los requisitos regulatorios deben determinarse según las jurisdicciones y el tipo de información tratado.

# 6. Accesibilidad

## 6.1. Estándar obligatorio

Todo frontend web debe cumplir WCAG 2.2 nivel AA como mínimo.

No es aceptable desarrollar primero una interfaz inaccesible y dejar la accesibilidad como una mejora posterior.

Las interfaces deben permitir la participación de personas que utilizan:

- Teclado.
- Lectores de pantalla.
- Ampliación de texto.
- Navegación por voz.
- Tecnologías de asistencia.
- Dispositivos móviles.
- Métodos alternativos de interacción.

Las aplicaciones móviles deben respetar las APIs de accesibilidad de iOS y Android y proporcionar una experiencia equivalente conforme a sus capacidades.

## 6.2. HTML semántico y estructura

Para aplicaciones web:

- Utilizar elementos HTML semánticos.
- Mantener una jerarquía correcta de encabezados.
- Utilizar landmarks.
- Utilizar botones para acciones.
- Utilizar enlaces para navegación.
- Asociar correctamente etiquetas y controles.
- Evitar controles interactivos construidos únicamente con elementos genéricos.
- Proporcionar nombres accesibles.
- Evitar roles ARIA innecesarios o incorrectos.

ARIA debe complementar la semántica nativa y no sustituirla cuando exista un elemento HTML adecuado.

## 6.3. Navegación por teclado

Toda funcionalidad debe poder utilizarse sin depender exclusivamente del mouse.

Reglas:

- Navegación coherente con Tab y Shift+Tab.
- Activación de controles mediante teclado.
- Indicadores visibles de foco.
- Orden lógico de tabulación.
- Gestión correcta del foco en diálogos.
- Restauración del foco al cerrar diálogos.
- No crear trampas de teclado.
- No depender exclusivamente de gestos complejos.
- Los elementos interactivos deben tener un orden de foco coherente.

## 6.4. Contraste y percepción visual

Los colores y estados deben ser perceptibles.

Como mínimo:

- Texto normal: contraste de 4.5:1.
- Texto grande: contraste de 3:1.
- Componentes de interfaz e información gráfica esencial: contraste de 3:1 cuando aplique el criterio correspondiente.

Reglas:

1. No comunicar información exclusivamente mediante color.
2. Los errores deben tener texto descriptivo además de color.
3. Los estados exitosos deben incluir indicadores comprensibles.
4. Los estados deshabilitados deben ser distinguibles.
5. Los iconos funcionales deben tener nombres accesibles.
6. Las imágenes informativas deben tener alternativas textuales.
7. Las imágenes decorativas deben ser ignoradas por tecnologías de asistencia cuando corresponda.

## 6.5. Tipografía y adaptación visual

Reglas:

- Permitir ampliación de texto hasta el 200% sin pérdida de contenido o funcionalidad.
- Soportar reflow en los anchos exigidos por WCAG 2.2.
- Evitar tamaños de texto ilegibles.
- Evitar alturas o contenedores rígidos que oculten contenido.
- No truncar información funcional importante.
- Mantener jerarquía tipográfica.
- No utilizar texto incrustado en imágenes como sustituto de contenido accesible.

## 6.6. Foco y objetivos interactivos

Los elementos interactivos deben ser visibles, identificables y utilizables.

Reglas:

- El foco no debe quedar oculto por encabezados, barras o paneles fijos.
- Los controles deben contar con indicadores de foco.
- Respetar los criterios WCAG 2.2 de foco visible, no oculto y tamaño mínimo del objetivo.
- Adoptar 44 × 44 píxeles CSS como objetivo de diseño recomendado para controles táctiles, salvo limitaciones justificadas.
- Mantener separación suficiente para evitar activaciones accidentales.

## 6.7. Formularios accesibles

Todos los formularios deben:

- Contar con etiquetas visibles.
- Identificar los campos obligatorios.
- Comunicar restricciones.
- Asociar errores con sus campos.
- Proporcionar instrucciones comprensibles.
- Mantener los valores cuando un envío falla, salvo que exista una razón de seguridad.
- Evitar depender únicamente del color.
- Anunciar errores y resultados relevantes a lectores de pantalla.
- Permitir navegación por teclado.
- Evitar perder información durante errores recuperables.

Los errores deben explicar cómo corregir el problema, sin revelar información interna sensible.

## 6.8. Componentes complejos

Los siguientes componentes requieren especial atención:

- Modales.
- Menús.
- Combobox.
- Selectores.
- Tabs.
- Acordeones.
- Tablas.
- Calendarios.
- Carruseles.
- Autocompletados.
- Notificaciones.
- Toasts.
- Formularios dinámicos.

Deben contar con roles, estados, teclado y gestión del foco apropiados.

## 6.9. Movimiento y animaciones

Reglas:

- Respetar `prefers-reduced-motion` en web.
- Respetar las preferencias de movimiento del sistema en aplicaciones móviles.
- Evitar animaciones innecesarias.
- Evitar parpadeos que representen riesgos.
- No utilizar animaciones como única forma de transmitir información.
- No bloquear interacciones durante animaciones decorativas.
- Permitir desactivar movimientos no esenciales cuando corresponda.

## 6.10. Multimedia

El contenido multimedia debe ofrecer alternativas accesibles.

Cuando corresponda:

- Subtítulos.
- Transcripciones.
- Descripciones alternativas.
- Controles accesibles.
- Controles de reproducción utilizables mediante teclado.
- Identificación de idioma.
- No iniciar audio automáticamente sin controles apropiados.

## 6.11. Aplicaciones móviles

Las aplicaciones móviles deben:

- Definir etiquetas accesibles para controles.
- Proporcionar roles y estados.
- Soportar VoiceOver y TalkBack.
- Soportar ampliación de texto.
- Respetar las preferencias del sistema.
- No transmitir información únicamente mediante color.
- Mantener áreas táctiles utilizables.
- Permitir navegación accesible.
- Evitar que los gestos sean el único mecanismo disponible para funcionalidades esenciales.

# 7. UX — Experiencia de usuario

La experiencia de usuario debe diseñarse de forma intencional y verificable.

## 7.1. Principios de experiencia

Toda interfaz debe:

1. Tener una finalidad comprensible.
2. Reducir fricción innecesaria.
3. Mantener consistencia entre pantallas.
4. Evitar pasos innecesarios.
5. Proporcionar información contextual.
6. Comunicar claramente las acciones disponibles.
7. Evitar sorpresas en el comportamiento.
8. Evitar interacciones ambiguas.
9. Facilitar recuperación ante errores.
10. Mantener una navegación predecible.

No se deben implementar mecanismos engañosos, patrones oscuros o restricciones diseñadas para confundir al usuario.

## 7.2. Estados obligatorios de interfaz

Toda pantalla o funcionalidad que dependa de datos debe contemplar los estados pertinentes:

- Inicial.
- Cargando.
- Éxito.
- Vacío.
- Sin resultados.
- Error.
- Sin conexión.
- Acceso no autorizado.
- Sesión expirada.
- Operación pendiente.
- Operación en proceso.
- Operación completada.
- Operación fallida.
- Estado parcial.

No se deben presentar pantallas en blanco sin explicación.

No se debe permitir que errores de red dejen la interfaz en un estado indefinido.

## 7.3. Retroalimentación de acciones

Toda operación debe proporcionar una respuesta comprensible.

Ejemplos:

- Confirmación al guardar.
- Indicador de carga.
- Error de validación.
- Confirmación de eliminación.
- Estado pendiente.
- Estado de pago.
- Resultado de una búsqueda.
- Confirmación de actualización.

Las operaciones importantes deben protegerse frente a múltiples envíos accidentales, sin depender exclusivamente del bloqueo visual del botón.

## 7.4. Manejo de errores

Los mensajes deben:

- Explicar el problema en lenguaje comprensible.
- Indicar cómo continuar cuando sea posible.
- Evitar información técnica innecesaria.
- No revelar stack traces.
- No mostrar consultas SQL.
- No mostrar secretos.
- No exponer información privada.
- Diferenciar errores corregibles de errores definitivos.

Los detalles técnicos deben enviarse a los sistemas de observabilidad con las protecciones de privacidad correspondientes.

## 7.5. Operaciones destructivas

Las acciones irreversibles o de alto impacto deben contar con protección apropiada.

Ejemplos:

- Eliminar cuentas.
- Eliminar información.
- Cancelar reservas.
- Cancelar suscripciones.
- Anular operaciones.
- Revocar accesos.
- Modificar configuraciones críticas.

La confirmación debe explicar claramente el impacto de la operación.

Cuando técnicamente sea posible, se debe proporcionar un mecanismo de recuperación o deshacer.

## 7.6. Formularios y procesos largos

Los formularios deben diseñarse para reducir errores.

Reglas:

- Agrupar campos relacionados.
- Evitar solicitar información innecesaria.
- Validar sin interrumpir constantemente.
- Preservar información válida.
- Comunicar progreso en procesos largos.
- Evitar pérdida accidental de datos.
- Informar claramente el resultado del envío.
- No depender exclusivamente de validaciones posteriores al envío.
- No utilizar placeholders como único sustituto de etiquetas.

## 7.7. Internacionalización

La arquitectura debe contemplar internacionalización cuando el proyecto tenga alcance multiidioma o internacional.

Reglas:

- Evitar textos funcionales incrustados directamente en componentes.
- Separar traducciones.
- Formatear fechas y horas según localización.
- Gestionar correctamente zonas horarias.
- Formatear monedas según su contexto.
- Manejar formatos numéricos locales.
- Soportar textos de distinta longitud.
- Evitar asumir que el idioma determina la moneda o el país.
- Preparar la interfaz para expansión de textos.

No convertir fechas, horas o valores monetarios mediante reglas arbitrarias del navegador.

# 8. UI — Diseño visual y sistema de diseño

## 8.1. Sistema de diseño obligatorio

El proyecto debe mantener un sistema de diseño consistente.

Debe definir, según corresponda:

- Colores.
- Tipografía.
- Escalas de espaciado.
- Radios.
- Sombras.
- Elevación.
- Tamaños.
- Breakpoints.
- Iconografía.
- Estados.
- Componentes.
- Variantes.
- Tokens de diseño.
- Temas.

Los valores compartidos deben centralizarse mediante tokens o mecanismos equivalentes.

Está prohibido construir cada pantalla con decisiones visuales arbitrarias que generen inconsistencias.

## 8.2. Componentes reutilizables

Crear componentes reutilizables para patrones comunes.

Ejemplos:

- Botones.
- Inputs.
- Selectores.
- Modales.
- Tablas.
- Tarjetas.
- Navegación.
- Alertas.
- Notificaciones.
- Formularios.
- Indicadores de progreso.
- Estados vacíos.
- Estados de error.

Los componentes deben tener contratos claros, variantes controladas y comportamiento accesible.

No crear abstracciones innecesarias ni componentes excesivamente genéricos.

## 8.3. Consistencia visual

Todas las pantallas de un mismo producto deben mantener:

- Jerarquía visual.
- Espaciado coherente.
- Tipografía consistente.
- Iconografía uniforme.
- Patrones de navegación.
- Estados interactivos.
- Tratamiento uniforme de errores.
- Componentes compartidos.

Las diferencias entre productos deben responder a decisiones de diseño explícitas, no a implementaciones improvisadas.

## 8.4. Diseño adaptable

Las interfaces deben adaptarse a los dispositivos que el producto soporte.

Se debe validar:

- Móviles.
- Tablets.
- Portátiles.
- Escritorios.
- Pantallas grandes.

No se permite resolver diseños responsivos mediante ocultar indiscriminadamente funcionalidades.

Las funciones esenciales deben seguir disponibles en tamaños soportados.

## 8.5. Temas

Cuando existan temas claro y oscuro:

- Deben mantener contraste suficiente.
- Deben utilizar los mismos tokens semánticos.
- Deben contemplar estados.
- Deben revisar imágenes, iconos y gráficos.
- Deben respetar las preferencias del usuario o sistema cuando corresponda.
- No deben generar información ilegible.

# 9. Rendimiento frontend

El rendimiento es un requisito de calidad y no una optimización opcional.

## 9.1. Objetivos web

Para páginas web, los Core Web Vitals deben alcanzar, como objetivo de calidad, los siguientes umbrales en el percentil 75 de visitas reales:

| Métrica | Objetivo |
|---|---:|
| LCP | ≤ 2.5 segundos |
| INP | ≤ 200 milisegundos |
| CLS | ≤ 0.1 |

Estos valores deben medirse mediante datos reales de usuarios cuando exista tráfico suficiente.

También deben utilizarse mediciones de laboratorio para detectar regresiones antes del despliegue.

## 9.2. Optimización de recursos

Reglas:

- Optimizar imágenes.
- Utilizar formatos modernos cuando estén soportados.
- Servir tamaños adecuados.
- Aplicar lazy loading a recursos fuera de la primera vista cuando corresponda.
- Evitar cargar recursos innecesarios.
- Optimizar fuentes.
- Controlar el tamaño de los bundles.
- Eliminar dependencias no utilizadas.
- Evitar bloquear el renderizado innecesariamente.
- Utilizar caché con políticas correctas.
- Evitar solicitudes redundantes.
- Evitar renderizados innecesarios.
- Aplicar división de código cuando corresponda.

No se debe aplicar lazy loading indiscriminadamente a recursos críticos de la primera vista.

## 9.3. Rendimiento de interacción

La interfaz debe responder de forma predecible.

Reglas:

- Evitar tareas largas en el hilo principal.
- Evitar cálculos pesados durante renderizados.
- Evitar bloqueos innecesarios.
- Utilizar listas virtualizadas cuando exista una razón de rendimiento.
- Evitar solicitudes duplicadas.
- Aplicar debounce o throttle cuando sea funcionalmente apropiado.
- Proporcionar feedback inmediato en operaciones lentas.
- Evitar desplazamientos visuales inesperados.

## 9.4. Gestión del estado y caché

El estado debe diseñarse según su ciclo de vida.

Diferenciar:

- Estado local.
- Estado global.
- Estado remoto.
- Estado persistente.
- Estado temporal de navegación.

Reglas:

- No duplicar innecesariamente la misma información.
- Invalidar datos desactualizados.
- Evitar cachés compartidas entre sesiones.
- Evitar persistir información privada sin justificación.
- Limpiar información al cerrar sesión o cambiar de tenant.
- Manejar correctamente errores y reintentos.
- No reintentar operaciones financieras o no idempotentes sin los controles correspondientes.

# 10. Observabilidad frontend obligatoria

Todo frontend debe contar con observabilidad desde su primera versión funcional.

La observabilidad no es una actividad exclusiva del backend.

Debe permitir detectar:

- Errores de JavaScript.
- Excepciones no controladas.
- Promesas rechazadas sin manejo.
- Fallos de renderizado.
- Errores de navegación.
- Fallos en solicitudes HTTP.
- Fallos de autenticación.
- Degradaciones de rendimiento.
- Problemas de carga.
- Problemas de interacción.
- Regresiones después de releases.
- Fallos específicos de navegador o dispositivo.
- Problemas de aplicaciones móviles.

## 10.1. Sentry obligatorio

Sentry debe integrarse en todos los frontends.

Cada aplicación debe contar con una configuración apropiada para su framework y plataforma.

Como mínimo:

- SDK oficial o compatible y mantenido.
- Captura de errores.
- Contexto de navegación.
- Identificación de versión.
- Identificación de entorno.
- Seguimiento de rendimiento cuando corresponda.
- Captura de errores de solicitudes.
- Integración con releases.
- Source maps o símbolos de depuración.
- Filtros de privacidad.
- Alertas para problemas relevantes.

No se considera implementada la observabilidad simplemente porque el SDK esté instalado.

Debe verificarse que los eventos llegan correctamente y que contienen contexto útil sin información sensible.

## 10.2. Configuración por entorno

Deben diferenciarse los entornos:

- Development.
- Test.
- Staging.
- Production.

Reglas:

1. Cada aplicación debe tener una identidad de proyecto Sentry apropiada.
2. Los entornos deben identificarse correctamente.
3. Las releases deben asociarse a versiones y commits.
4. Los eventos de desarrollo no deben confundirse con producción.
5. La configuración de muestreo debe ser explícita.
6. Las alertas deben diferenciar problemas críticos y errores esperados.
7. No enviar información sensible durante el desarrollo.

## 10.3. Errores y excepciones

Capturar los errores relevantes, incluyendo:

- Errores no controlados.
- Promesas rechazadas.
- Errores de componentes y renderizado.
- Fallos de carga de recursos.
- Fallos de navegación.
- Errores de red.
- Errores funcionales que requieran investigación.

Reglas:

- No capturar indiscriminadamente errores esperados como si fueran fallos críticos.
- No silenciar excepciones sin registrar un resultado apropiado.
- No capturar el mismo error repetidamente sin necesidad.
- Evitar duplicación de eventos.
- Mantener contexto suficiente para diagnóstico.
- No enviar información privada innecesaria.

Los límites de errores de componentes del framework deben integrarse con Sentry cuando corresponda.

## 10.4. Performance Monitoring

Sentry debe proporcionar observabilidad de rendimiento cuando la plataforma y el SDK lo permitan.

Para web se debe instrumentar, según corresponda:

- Navegación.
- Carga de rutas.
- Operaciones relevantes.
- Solicitudes HTTP.
- Tiempos de respuesta.
- Web Vitals.
- Errores asociados a operaciones.
- Trazas distribuidas.

Para aplicaciones móviles:

- Inicio de aplicación.
- Navegación.
- Errores de interfaz.
- Solicitudes de red.
- Tiempos de operaciones.
- Crashes nativos.
- Métricas de rendimiento compatibles con la plataforma.

La instrumentación debe ser proporcional al valor diagnóstico y al costo de recolección.

## 10.5. Correlación frontend-backend

Cuando exista instrumentación distribuida:

- Correlacionar solicitudes frontend con trazas backend.
- Propagar contexto de trazas mediante mecanismos compatibles, como W3C Trace Context.
- Configurar destinos permitidos para propagación de headers.
- No enviar headers de tracing indiscriminadamente a dominios externos.
- No incluir datos personales en baggage o atributos de trazas.
- Evitar cardinalidad excesiva.
- Mantener identificadores de correlación útiles.

La trazabilidad no sustituye la autenticación ni la autorización.

## 10.6. Releases y source maps

Toda versión de producción debe asociarse a una release identificable.

Reglas:

1. Identificar la release mediante versión o commit inequívoco.
2. Subir source maps o artefactos de depuración a Sentry desde CI o el proceso seguro de construcción.
3. Asociarlos con la release correcta.
4. Verificar que los errores se resuelvan con nombres de archivos y líneas originales.
5. Restringir el acceso a artefactos que revelen código fuente.
6. No publicar source maps sin protección por defecto.
7. No incluir tokens de Sentry en el bundle frontend.
8. Gestionar los tokens de subida mediante secretos del entorno de construcción.
9. Limitar los permisos de dichos tokens.
10. Verificar la correspondencia entre release y artefactos desplegados.

El DSN de cliente no debe confundirse con un token de autenticación del proyecto o de administración. No debe incluirse ningún token privado de Sentry en el código distribuido al usuario.

## 10.7. Session Replay

Session Replay solo puede activarse bajo una política de privacidad aprobada.

Configuración obligatoria cuando se habilite:

- Enmascarar texto por defecto.
- Enmascarar entradas de usuario.
- Bloquear elementos sensibles.
- Bloquear medios o contenido privado cuando corresponda.
- Excluir pantallas de autenticación y pagos.
- Excluir campos personales y financieros.
- Evitar capturar contraseñas y tokens.
- Establecer muestreo controlado.
- Restringir el acceso a las reproducciones.
- Cumplir las obligaciones de consentimiento y privacidad aplicables.

No se debe habilitar la exposición de texto o campos sensibles mediante excepciones globales.

La activación de Replay requiere una revisión específica de privacidad y seguridad.

## 10.8. Identidad y contexto de eventos

Se puede asociar un identificador pseudónimo de usuario o contexto operativo cuando sea necesario para diagnosticar incidentes.

Reglas:

- No enviar contraseñas.
- No enviar tokens.
- No enviar documentos personales.
- No enviar datos financieros.
- No enviar información médica o sensible.
- No adjuntar registros completos de usuarios.
- No utilizar correos electrónicos o teléfonos como etiquetas indiscriminadas.
- Evitar etiquetas de cardinalidad excesiva.
- Utilizar identificadores opacos y aprobados cuando la correlación sea necesaria.

No identificar a una persona en Sentry mediante datos personales que no sean estrictamente necesarios.

## 10.9. Alertas y gestión de incidentes

Deben existir alertas para:

- Incrementos anormales de errores.
- Errores críticos nuevos.
- Regresiones después de releases.
- Crashes de aplicaciones móviles.
- Degradaciones importantes de rendimiento.
- Fallos recurrentes de navegación o solicitudes.
- Problemas que afecten funcionalidades esenciales.

Las alertas deben tener responsables y procedimientos de atención.

No se debe generar ruido de alertas que provoque que las incidencias importantes sean ignoradas.

# 11. Logging y analítica frontend

Los logs y eventos de analítica deben tener propósitos definidos.

Reglas:

- No utilizar `console.log` como sistema de observabilidad productivo.
- No registrar datos sensibles.
- No registrar tokens.
- No registrar contraseñas.
- No registrar cuerpos completos de solicitudes indiscriminadamente.
- No registrar respuestas completas que contengan información privada.
- No registrar parámetros de URL sensibles.
- Utilizar niveles de logging apropiados.
- Evitar mensajes excesivamente repetitivos.
- Evitar identificadores personales en métricas.

La analítica de producto debe estar separada conceptualmente de los errores técnicos.

No deben utilizarse eventos de analítica para sustituir la observabilidad de errores.

# 12. Aplicaciones Flutter móviles y de escritorio: controles adicionales

Para ENSAMBRIA App y las versiones móviles de ENSAMBRIA Business, además de los requisitos comunes, deben aplicarse controles propios de iOS y Android. ENSAMBRIA Business también debe validar los controles y flujos de seguridad de Windows, Linux y macOS.

La seguridad debe evaluarse siguiendo OWASP MASVS y sus guías de pruebas MASTG.

Reglas:

- Almacenamiento seguro.
- Protección de comunicaciones.
- Gestión de credenciales.
- Integración segura con el sistema operativo.
- No guardar secretos del backend en la app; proteger credenciales y sesiones mediante las capacidades seguras del sistema operativo para cada plataforma.
- Probar permisos, manejo de enlaces profundos, ciclo de vida de sesión y almacenamiento en cada sistema operativo soportado.
- En escritorio, validar acceso a archivos, manejo de sesiones, actualización y distribución dentro de los mecanismos autorizados de cada plataforma; no declarar firma, notarización ni protección de actualización configuradas sin evidencia.
- Permisos mínimos.
- Protección de datos locales.
- Validación de deep links.
- Validación de universal links y app links.
- Protección de WebViews.
- Restricción de esquemas y navegación externos.
- Gestión segura de archivos.
- Protección de datos en notificaciones.
- Gestión de capturas de pantalla en pantallas sensibles cuando corresponda.
- Manejo seguro de sesiones.
- Protección de logs.
- Evaluación de manipulación y riesgos de ingeniería inversa según el riesgo del producto.

No almacenar información privada en archivos, preferencias o bases locales sin controles apropiados.

Los datos temporales deben eliminarse cuando ya no sean necesarios.

No solicitar permisos del sistema operativo sin una finalidad funcional clara.

## 12.1. WebViews

Las WebViews deben utilizarse con restricciones.

Reglas:

- Deshabilitar capacidades innecesarias.
- No permitir navegación arbitraria.
- Validar dominios permitidos.
- Controlar comunicación JavaScript-native.
- No exponer interfaces nativas a contenido no confiable.
- No almacenar credenciales sin protección.
- Revisar el tratamiento de enlaces externos.
- Evitar cargar contenido HTTP inseguro.

# 13. Pruebas obligatorias del frontend

Todo frontend debe incluir una estrategia de pruebas desde la Fase 0.

## 13.1. Pruebas unitarias

Deben cubrir:

- Lógica de presentación.
- Validaciones.
- Transformaciones.
- Hooks o equivalentes.
- Funciones de estado.
- Permisos visuales.
- Formateo.
- Manejo de errores.

## 13.2. Pruebas de componentes

Deben comprobar:

- Renderizado.
- Interacciones.
- Estados.
- Accesibilidad.
- Validaciones.
- Formularios.
- Navegación.
- Manejo de errores.
- Comportamiento responsivo cuando corresponda.

## 13.3. Pruebas de integración

Deben verificar:

- Integración con clientes API.
- Gestión de sesiones.
- Renovación de autenticación.
- Cambios de tenant.
- Manejo de errores HTTP.
- Cachés.
- Estado remoto.
- Flujos funcionales completos.

## 13.4. Pruebas end-to-end

Deben cubrir los procesos esenciales del usuario.

Ejemplos:

- Registro.
- Inicio de sesión.
- Recuperación de acceso.
- Cambio de tenant.
- Operaciones principales.
- Formularios.
- Pagos.
- Reservas.
- Cancelaciones.
- Cierre de sesión.
- Manejo de errores.

Las pruebas deben ejecutarse con entornos y dependencias apropiados.

## 13.5. Pruebas de seguridad

Incluir escenarios para:

- XSS.
- CSRF cuando aplique.
- Control de navegación.
- Sesiones expiradas.
- Autorización.
- Manipulación de parámetros.
- Acceso directo a rutas.
- Fugas de información.
- Caché entre usuarios.
- Caché entre tenants.
- Manipulación de almacenamiento.
- Deep links inseguros.
- Dependencias vulnerables.

## 13.6. Pruebas de accesibilidad

Combinar:

- Herramientas automatizadas.
- Inspección de árbol de accesibilidad.
- Navegación por teclado.
- Revisión de contraste.
- Lectores de pantalla.
- Pruebas manuales.
- Revisión de formularios.
- Verificación de foco.
- Pruebas de ampliación y reflow.

Las herramientas automáticas no garantizan conformidad completa con WCAG.

No se acepta una declaración de accesibilidad basada únicamente en un análisis automático.

## 13.7. Pruebas entre navegadores y dispositivos

Validar los navegadores, sistemas operativos y dispositivos definidos como compatibles.

Las pruebas deben considerar:

- Diferencias de renderizado.
- Diferencias de teclado.
- Diferencias de accesibilidad.
- Orientación.
- Resolución.
- Interacciones táctiles.
- Rendimiento.
- Gestión de memoria.
- Compatibilidad de APIs.

## 13.8. Pruebas por plataforma Flutter

ENSAMBRIA Business y ENSAMBRIA App requieren pruebas unitarias, de widgets, integración, navegación y accesibilidad acordes a Flutter. Probar la app final en dispositivos/emuladores y sistemas operativos representativos antes de release:

- Business: Android, iOS, tablets Android/iPad, Windows, Linux y macOS.
- App: Android, iOS y tablets Android/iPad.
- Business de escritorio: teclado, ratón, foco, escalado, ventanas, ciclo de sesión y flujos empresariales propios del formato de escritorio.
- Mobile/tablet: tactilidad, orientación, tamaños de pantalla, interrupciones y permisos nativos aplicables.

Las pruebas de plataforma deben comprobar autenticación/sesiones y autorización por la API real de la aplicación; una prueba widget aislada no demuestra aislamiento tenant, seguridad del backend ni funcionamiento del recorrido frontend→BFF→owner.

# 14. Calidad de código frontend

Reglas obligatorias:

- Utilizar TypeScript estricto cuando el stack lo soporte.
- Evitar `any`.
- Evitar conversiones de tipos inseguras.
- No suprimir errores del compilador.
- No deshabilitar reglas de lint indiscriminadamente.
- Mantener componentes pequeños y comprensibles.
- Evitar lógica duplicada.
- Evitar dependencias innecesarias.
- Mantener límites entre capas.
- Centralizar contratos y tipos cuando corresponda.
- No manipular directamente el DOM si el framework ofrece mecanismos apropiados.
- Evitar dependencias circulares.
- Evitar estados imposibles o inconsistentes.
- Gestionar cancelación de solicitudes cuando corresponda.
- Evitar condiciones de carrera en interfaces asíncronas.
- Gestionar correctamente desmontaje de componentes y recursos.

No utilizar excepciones técnicas como mecanismo habitual para evitar corregir problemas.

# 15. CI y validaciones obligatorias

Cada frontend debe ejecutar en CI las verificaciones correspondientes a su stack.

Como mínimo:

- Instalación determinista de dependencias.
- Validación de formato.
- Lint.
- Typecheck.
- Pruebas unitarias.
- Pruebas de componentes.
- Pruebas de integración cuando correspondan.
- Pruebas E2E de flujos críticos.
- Análisis de dependencias.
- Construcción de producción.
- Validación de accesibilidad automatizada.
- Validación de configuración de observabilidad.
- Comprobación de artefactos de Sentry cuando corresponda.

Las validaciones locales y de CI deben mantener coherencia.

No desactivar verificaciones para permitir un merge.

No se permite reducir la calidad para obtener un CI verde.

Los cambios exclusivamente documentales deben utilizar CI selectivo cuando corresponda.

# 16. Gestión de defectos y regresiones

Todo defecto frontend debe clasificarse considerando su impacto.

Categorías mínimas:

- Seguridad.
- Privacidad.
- Accesibilidad.
- Integridad de datos.
- Funcionalidad.
- Experiencia de usuario.
- Rendimiento.
- Compatibilidad.
- Observabilidad.

Los defectos críticos deben impedir el despliegue o la liberación de la funcionalidad afectada.

Las regresiones no deben ocultarse mediante desactivación de componentes, funcionalidades o pruebas.

Los problemas detectados en producción deben generar seguimiento, análisis y corrección verificable.

# 17. Restricciones no negociables

Queda prohibido:

1. Confiar en el frontend para autorizar operaciones.
2. Exponer secretos del backend.
3. Guardar tokens persistentes en almacenamiento web inseguro.
4. Renderizar HTML no confiable sin sanitización.
5. Desactivar mecanismos de seguridad para resolver problemas de integración.
6. Desarrollar funcionalidades inaccesibles.
7. Ignorar WCAG 2.2 AA en aplicaciones web.
8. Crear interfaces sin estados de error y carga.
9. Implementar diseños visuales inconsistentes sin justificación.
10. Ignorar errores de navegación o de solicitudes.
11. Desplegar frontends sin observabilidad.
12. Omitir Sentry.
13. Enviar datos privados a Sentry sin controles.
14. Publicar source maps sin evaluar sus riesgos.
15. Habilitar Session Replay sin controles de privacidad.
16. Ignorar errores de compilación o pruebas.
17. Utilizar mocks como sustitutos de pruebas reales de integración cuando estas sean necesarias.
18. Desactivar pruebas de accesibilidad para facilitar una entrega.
19. Ocultar errores mediante capturas vacías o mensajes genéricos sin registro apropiado.
20. Almacenar información de un usuario o tenant para reutilizarla en otro contexto.
21. Ejecutar código remoto no autorizado.
22. Introducir librerías sin evaluar su riesgo.
23. Reducir deliberadamente el rendimiento para simplificar la implementación.
24. Declarar terminado un frontend sin validaciones suficientes.

# 18. Definition of Done del frontend

Una funcionalidad frontend no se considera terminada hasta cumplir los siguientes criterios aplicables:

## Arquitectura
- [ ] Arquitectura modular y coherente.
- [ ] Separación adecuada de responsabilidades.
- [ ] Contratos compatibles con backend.
- [ ] Gestión del estado correcta.
- [ ] Ausencia de duplicaciones críticas.

## Seguridad
- [ ] No existen secretos expuestos.
- [ ] Autenticación correctamente implementada.
- [ ] Autorización delegada y verificada en backend.
- [ ] Entradas externas tratadas de forma segura.
- [ ] Protección contra XSS.
- [ ] Protección CSRF cuando corresponda.
- [ ] Gestión de sesiones correcta.
- [ ] Aislamiento entre usuarios y tenants.
- [ ] Dependencias revisadas.
- [ ] No existen vulnerabilidades críticas o altas sin tratamiento aprobado.

## Accesibilidad
- [ ] Cumplimiento de WCAG 2.2 AA para web.
- [ ] Navegación por teclado.
- [ ] Foco visible.
- [ ] Contraste correcto.
- [ ] Formularios accesibles.
- [ ] Compatibilidad con tecnologías de asistencia.
- [ ] Pruebas automatizadas y manuales.
- [ ] Validación de experiencia móvil cuando corresponda.

## UX/UI
- [ ] Sistema de diseño respetado.
- [ ] Componentes consistentes.
- [ ] Diseño responsivo.
- [ ] Estados de carga.
- [ ] Estados vacíos.
- [ ] Estados de error.
- [ ] Retroalimentación de acciones.
- [ ] Confirmación de operaciones críticas.
- [ ] Textos comprensibles.
- [ ] Sin patrones oscuros.

## Rendimiento
- [ ] Recursos optimizados.
- [ ] Bundles revisados.
- [ ] Carga adecuada.
- [ ] Interacciones responsivas.
- [ ] Ausencia de regresiones críticas.
- [ ] Core Web Vitals verificados cuando aplique.

## Observabilidad
- [ ] Sentry integrado.
- [ ] Eventos de prueba recibidos correctamente.
- [ ] Releases identificables.
- [ ] Entornos diferenciados.
- [ ] Errores correctamente capturados.
- [ ] Datos sensibles filtrados.
- [ ] Source maps o símbolos asociados.
- [ ] Performance Monitoring implementado según plataforma.
- [ ] Alertas configuradas.
- [ ] Replay deshabilitado o configurado conforme a la política aprobada.

## Pruebas
- [ ] Pruebas unitarias.
- [ ] Pruebas de componentes.
- [ ] Pruebas de integración.
- [ ] E2E de flujos esenciales.
- [ ] Pruebas de seguridad.
- [ ] Pruebas de accesibilidad.
- [ ] CI aprobado.
- [ ] Construcción de producción aprobada.

No se puede declarar una funcionalidad terminada si quedan criterios obligatorios sin cumplir.

Las excepciones requieren justificación, aprobación, riesgo documentado y plazo de resolución.

# 19. Gobierno y evolución del estándar

Estas reglas son obligatorias desde la Fase 0 y aplican a todos los productos frontend.

Cualquier excepción debe documentar:

- Regla afectada.
- Justificación técnica.
- Impacto.
- Riesgo.
- Mitigaciones.
- Responsable de aprobación.
- Condición de resolución.

Los cambios al estándar deben realizarse mediante Pull Request.

No se permite eliminar requisitos para justificar una implementación deficiente.

El estándar debe revisarse periódicamente para mantener compatibilidad con las versiones soportadas de las tecnologías, navegadores, frameworks y estándares de accesibilidad.

# 20. Referencias normativas y técnicas

Las siguientes referencias deben utilizarse como base de implementación y revisión. Siempre se debe verificar la versión vigente y la aplicabilidad al stack.

## Accesibilidad
- WCAG 2.2: https://www.w3.org/TR/WCAG22/
- WAI: https://www.w3.org/WAI/
- ARIA Authoring Practices Guide: https://www.w3.org/WAI/ARIA/apg/

## Seguridad web
- OWASP Web Frontend Security Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/Web_Frontend_Security_Cheat_Sheet.html
- OWASP Content Security Policy Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/Content_Security_Policy_Cheat_Sheet.html
- OWASP Cheat Sheet Series: https://cheatsheetseries.owasp.org/

## Seguridad móvil
- OWASP MASVS: https://mas.owasp.org/MASVS/
- OWASP MASTG: https://mas.owasp.org/MASTG/

## Rendimiento web
- Core Web Vitals: https://web.dev/articles/vitals
- Google Search Central: https://developers.google.com/search/docs/appearance/core-web-vitals

## Observabilidad
- Sentry JavaScript: https://docs.sentry.io/platforms/javascript/
- Sentry Flutter: https://docs.sentry.io/platforms/flutter/
- Sentry Next.js: https://docs.sentry.io/platforms/javascript/guides/nextjs/
- Sentry Releases: https://docs.sentry.io/product/releases/
- Sentry Source Maps: https://docs.sentry.io/platforms/javascript/sourcemaps/
- Sentry Session Replay: https://docs.sentry.io/platforms/javascript/session-replay/

---

# Declaración final

La seguridad, la accesibilidad, la experiencia de usuario, la consistencia visual, el rendimiento y la observabilidad son características fundamentales del producto frontend.

No son mejoras opcionales, tareas cosméticas ni actividades que puedan postergarse indefinidamente.

Todo frontend debe ser seguro, accesible, usable, consistente, medible y observable desde sus primeras implementaciones.

**Un frontend no está terminado simplemente porque se ve bien y funciona. Está terminado cuando cumple sus requisitos funcionales, técnicos, de seguridad, accesibilidad, experiencia, rendimiento y observabilidad con evidencia verificable.**
