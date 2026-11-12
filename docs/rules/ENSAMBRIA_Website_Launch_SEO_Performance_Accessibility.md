# ENSAMBRIA — Estándar obligatorio de calidad web, seguridad, SEO, rendimiento y accesibilidad

**Archivo:** `ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md`<br>
**Categoría:** Estándar de ingeniería y calidad de producto<br>
**Aplicación:** Todos los frontends, sitios públicos, páginas, formularios y experiencias web de ENSAMBRIA.<br>
**Vigencia:** Obligatorio desde la Fase 0.<br>
**Prioridad:** Requisito transversal de arquitectura, desarrollo, pruebas y lanzamiento.

---

# 1. Objetivo

ENSAMBRIA debe ofrecer experiencias web profesionales, seguras, rápidas, accesibles, optimizadas para buscadores y correctamente adaptadas a dispositivos móviles.

Este estándar establece las reglas obligatorias que deben respetar los agentes de desarrollo y los equipos de ingeniería al crear, modificar, desplegar o mantener cualquier frontend de ENSAMBRIA.

No se considera terminada una página por el simple hecho de que visualmente funcione.

Cada experiencia debe cumplir requisitos verificables de:

- Seguridad.
- Privacidad.
- SEO técnico.
- Open Graph.
- Identidad visual.
- Rendimiento.
- Accesibilidad.
- Diseño responsive.
- Experiencia de usuario.
- Formularios.
- Protección antispam.
- Analítica.
- Calidad de enlaces y navegación.
- Pruebas automatizadas.
- Compatibilidad entre navegadores.

El cumplimiento debe formar parte del desarrollo y no convertirse en una corrección improvisada antes de publicar.

# 2. Alcance

Este estándar aplica a los siguientes frontends:

| Frontend | Aplicación |
|---|---|
| `ensambria-business` | Cliente Flutter privado para operación empresarial en móvil, tablet y escritorio. |
| `tenant-portal` | Portal Next.js privado para administrar los mismos tenants/capacidades que Business. |
| `ensambria-app` | Cliente Flutter de clientes finales; no administra tenants. |
| `marketplace` | Experiencia Next.js pública de descubrimiento, directorio y Marketplace. |
| `platform-admin` | Experiencia Next.js privada de administración global de ENSAMBRIA. |
| `corporate` | Sitio Next.js institucional/comercial público; no administra tenants. |

También aplica a:

- Landing pages.
- Páginas de registro.
- Páginas de autenticación.
- Formularios públicos.
- Páginas de reserva.
- Páginas de productos y servicios.
- Páginas de establecimientos.
- Páginas de profesionales.
- Páginas de sucursales.
- Páginas promocionales.
- Páginas de contratación.
- Páginas de suscripciones.
- Páginas de ayuda y soporte.
- Páginas de error.
- Contenido generado dinámicamente.

Los requisitos se aplican según la naturaleza de cada frontend. Por ejemplo, el SEO público es obligatorio donde exista contenido indexable, pero no debe habilitarse indiscriminadamente en paneles privados.

## 2.1 Relación con las reglas especializadas

Este estándar convive con las dos guías especializadas y las complementa; no las reemplaza. Cada documento conserva una única fuente de verdad para su ámbito:

| Documento | Responsabilidad normativa |
|---|---|
| Este estándar | Calidad general web y preparación para lanzamiento: seguridad del frontend, rendimiento, accesibilidad, diseño responsive, compatibilidad, UX, formularios, pruebas y checklist de publicación. |
| [ENSAMBRIA_Privacy_Legal_Compliance.md](ENSAMBRIA_Privacy_Legal_Compliance.md) | Obligaciones de privacidad, protección de datos y cumplimiento legal por actividad y jurisdicción, incluidos los requisitos aplicables a cookies, consentimiento, analítica y terceros. |
| [ENSAMBRIA_Open_Graph_SEO_Metadata.md](ENSAMBRIA_Open_Graph_SEO_Metadata.md) | Especificación detallada de SEO técnico y editorial, metadatos HTML, Open Graph, canonical, indexabilidad, sitemap, robots, datos estructurados y vistas previas. |
| [ENSAMBRIA_Geographic_Access_Policies.md](ENSAMBRIA_Geographic_Access_Policies.md) | Disponibilidad por mercado/ubicación y experiencia diferenciada entre navegación pública, solicitudes de lanzamiento/privacidad, registro y operaciones comerciales. |

Las referencias a privacidad, cookies, SEO o disponibilidad geográfica en este estándar expresan controles de calidad y aceptación de la experiencia web; no crean una política legal ni duplican las especificaciones especializadas. La implementación debe cumplir las guías correspondientes. Si hay discrepancia, cada guía especializada gobierna su ámbito y este estándar sigue gobernando la calidad web general. Registra y resuelve cualquier incompatibilidad real antes de implementar o lanzar el alcance afectado.

# 3. Principios obligatorios

1. Seguridad desde el diseño.
2. Privacidad desde el diseño.
3. Accesibilidad desde el diseño.
4. SEO técnico desde el diseño.
5. Rendimiento como requisito de calidad.
6. Diseño responsive.
7. Componentes reutilizables y consistentes.
8. Validación en frontend y backend.
9. No exponer información sensible.
10. No duplicar reglas de negocio en los frontends.
11. No confiar en controles exclusivamente visuales.
12. Pruebas automatizadas para los requisitos críticos.
13. Compatibilidad con internacionalización.
14. Separación entre páginas públicas y privadas.
15. No declarar una implementación terminada sin evidencia.

# 4. Política de privacidad y términos y condiciones

Todos los sitios públicos deben presentar la información legal que se haya validado para su actividad y jurisdicción. Los requisitos y decisiones normativas se rigen por [ENSAMBRIA_Privacy_Legal_Compliance.md](ENSAMBRIA_Privacy_Legal_Compliance.md); esta sección describe su presentación web y no constituye una fuente legal independiente ni sustituye la revisión de Legal.

## 4.1 Política de privacidad

Debe explicar de forma clara:

- Identidad del responsable del tratamiento.
- Datos recopilados.
- Finalidades.
- Bases jurídicas aplicables.
- Categorías de información.
- Proveedores y terceros.
- Transferencias internacionales.
- Periodos o criterios de conservación.
- Derechos de los titulares.
- Procedimientos para ejercer derechos.
- Canales de contacto.
- Uso de cookies y tecnologías similares.
- Tratamiento de datos sensibles cuando corresponda.
- Tratamiento de información de menores cuando esté permitido.

Los avisos deben adaptarse a las jurisdicciones donde ENSAMBRIA opere.

No utilizar un texto legal genérico como sustituto de una revisión jurídica por mercado.

## 4.2 Términos y condiciones

Deben cubrir, según el servicio:

- Identificación del proveedor.
- Descripción del servicio.
- Condiciones de uso.
- Registro y cuentas.
- Obligaciones del usuario.
- Obligaciones del establecimiento.
- Reservas.
- Cancelaciones.
- Pagos.
- Reembolsos.
- Suscripciones.
- Renovaciones automáticas.
- Limitaciones y responsabilidades legalmente admisibles.
- Propiedad intelectual.
- Contenido generado por usuarios.
- Suspensión y terminación.
- Atención de reclamaciones.
- Legislación y jurisdicción aplicables.

Los términos deben diferenciar las condiciones de uso de ENSAMBRIA y las condiciones comerciales propias de cada establecimiento.

## 4.3 Versionado legal

Cada documento debe disponer de:

- Identificador.
- Versión.
- Idioma.
- Jurisdicción.
- Fecha de publicación.
- Fecha de entrada en vigor.
- Historial de modificaciones.

Las versiones anteriores deben conservarse cuando sea necesario demostrar qué condiciones se presentaron al usuario.

# 5. Seguridad del frontend

## 5.1 Secretos y claves API

Está prohibido incluir secretos privados en código frontend.

Nunca incluir:

- Claves privadas de proveedores.
- Contraseñas.
- Tokens de acceso privados.
- Credenciales de bases de datos.
- Credenciales de Vault.
- Claves de firma.
- Secretos de servicios internos.
- Credenciales administrativas.
- Claves privadas de pagos.
- Tokens de infraestructura.

Las variables públicas de configuración no deben confundirse con secretos.

Ejemplos de variables que pueden ser públicas si su proveedor las define así:

- URL pública de API.
- Identificador público de aplicación.
- Clave publicable de un proveedor.
- Identificador de proyecto.

Toda credencial debe clasificarse según las condiciones del proveedor y su capacidad real de autorización.

Las claves publicables no deben confundirse con claves secretas.

## 5.2 Gestión de secretos

Los secretos deben permanecer en backend o en el gestor seguro correspondiente.

ENSAMBRIA utilizará Vault cuando corresponda a la arquitectura y entorno.

Los backends deben obtener exclusivamente las credenciales necesarias para su función.

Nunca enviar secretos al navegador mediante:

- HTML inicial.
- JSON de hidratación.
- Variables serializadas.
- Source maps públicos.
- Archivos estáticos.
- Respuestas de API.
- Logs del navegador.

## 5.3 HTTPS

Todas las experiencias de producción deben utilizar HTTPS.

Requisitos:

- Redirección HTTP a HTTPS.
- Certificados válidos.
- Renovación automatizada.
- TLS moderno.
- Eliminación de mixed content.
- Cookies seguras.
- Configuración adecuada de proxies y CDN.
- HSTS cuando la infraestructura esté preparada.
- Evaluación de subdominios antes de utilizar `includeSubDomains`.
- No permitir degradación insegura.

No basta con que el sitio tenga un certificado instalado: todos los recursos y conexiones deben utilizar canales seguros.

## 5.4 Cabeceras de seguridad

Evaluar e implementar según el frontend:

- Content-Security-Policy.
- Strict-Transport-Security.
- X-Content-Type-Options.
- Referrer-Policy.
- Permissions-Policy.
- Protección contra clickjacking mediante `frame-ancestors` u otra configuración apropiada.
- Cookies `Secure`, `HttpOnly` y `SameSite` cuando corresponda.

La CSP debe adaptarse a los recursos realmente utilizados. No utilizar `unsafe-eval` o `unsafe-inline` indiscriminadamente para evitar resolver problemas de implementación.

## 5.5 Control de acceso

Las rutas privadas deben validar autenticación y autorización en el servidor.

No considerar como seguridad:

- Ocultar botones.
- Ocultar enlaces.
- Validar solamente en React.
- Proteger solamente mediante middleware visual.
- Comprobar roles exclusivamente en el cliente.

Los endpoints deben validar identidad, permisos, tenant y contexto.

## 5.6 Protección de formularios

Implementar:

- Validación de entrada.
- Protección CSRF cuando corresponda.
- Rate limiting.
- Límites de tamaño.
- Validación de tipos.
- Protección contra XSS.
- Protección contra inyección.
- Control de redirecciones.
- Gestión segura de errores.
- Protección contra abuso automatizado.

# 6. Consentimiento de cookies y tecnologías de seguimiento

ENSAMBRIA debe implementar una gestión de consentimiento conforme a las jurisdicciones aplicables.

## 6.1 Categorías

Diferenciar:

- Cookies estrictamente necesarias.
- Preferencias.
- Analítica.
- Marketing.
- Publicidad.
- Seguimiento de terceros.

No clasificar automáticamente una cookie como necesaria por conveniencia técnica.

## 6.2 Banner

Cuando sea legalmente exigible, el banner debe permitir:

- Aceptar.
- Rechazar.
- Configurar preferencias.

El rechazo debe ser tan accesible como la aceptación.

No utilizar patrones engañosos.

## 6.3 Comportamiento

Antes de obtener consentimiento cuando sea necesario:

- No cargar scripts no esenciales.
- No activar píxeles publicitarios.
- No iniciar session replay.
- No almacenar identificadores de seguimiento no autorizados.
- No transmitir información personal innecesaria a terceros.

Las preferencias deben poder modificarse posteriormente.

La retirada debe propagarse a las integraciones correspondientes.

## 6.4 Integración con privacidad

El sistema debe respetar la política central de jurisdicciones y consentimientos definida en:

`ENSAMBRIA_Privacy_Legal_Compliance.md`

No implementar un sistema independiente de consentimiento en cada frontend.

# 7. SEO técnico

El SEO debe implementarse desde el servidor y formar parte de la arquitectura de las páginas públicas. Los detalles normativos de metadatos, indexabilidad y rastreo se rigen por [ENSAMBRIA_Open_Graph_SEO_Metadata.md](ENSAMBRIA_Open_Graph_SEO_Metadata.md); las secciones 7 a 10 de este estándar resumen criterios de calidad web y no sustituyen esa especificación.

## 7.1 Metatítulos

Cada página pública indexable debe tener un título:

- Único.
- Descriptivo.
- Relevante.
- Coherente con el contenido.
- Adaptado al idioma.
- Sin repetición artificial de palabras clave.

No utilizar un único título global para todas las páginas.

## 7.2 Metadescripciones

Cada página pública indexable debe disponer de una descripción:

- Original.
- Descriptiva.
- Relacionada con el contenido.
- Clara para usuarios.
- Adaptada al idioma y mercado.

Las descripciones no deben contener información inventada.

## 7.3 Canonical

Las páginas públicas deben definir una URL canónica absoluta cuando corresponda.

Evitar:

- Duplicación por parámetros.
- Duplicación entre dominios.
- URLs inconsistentes.
- Canónicas hacia páginas privadas.
- Canónicas hacia dominios no autorizados.

Los dominios personalizados de establecimientos deben validarse contra la configuración registrada.

No confiar ciegamente en el encabezado Host para construir URLs canónicas.

## 7.4 Robots

Configurar correctamente:

- Meta robots.
- `X-Robots-Tag` cuando corresponda.
- Reglas de indexación.
- Páginas duplicadas.
- Páginas privadas.
- Páginas temporales.
- Resultados de búsqueda internos.

Las páginas privadas deben protegerse mediante autenticación y autorización.

`noindex` no es un mecanismo de seguridad.

# 8. Sitemap XML

Los sitios públicos deben disponer de sitemap cuando corresponda.

## 8.1 Contenido

Incluir exclusivamente URLs públicas, canónicas e indexables.

Ejemplos:

- Inicio.
- Establecimientos.
- Sucursales.
- Servicios.
- Profesionales.
- Productos.
- Categorías públicas.
- Contenido editorial público.

Excluir:

- Paneles administrativos.
- Perfiles privados.
- Datos personales.
- URLs temporales.
- Resultados internos de búsqueda que no deban indexarse.
- Páginas de autenticación cuando no corresponda.
- Páginas duplicadas.
- Contenido eliminado.
- URLs no autorizadas.

## 8.2 Reglas

- XML válido.
- URLs absolutas.
- HTTPS.
- Canónicas.
- Actualización coherente.
- Separación por sitemap cuando sea necesario.
- Sitemap index cuando el volumen lo requiera.
- Límites y formatos compatibles con los motores de búsqueda.

No declarar `lastmod` con fechas artificiales.

## 8.3 Multi-tenant

Cada dominio de establecimiento debe generar únicamente URLs autorizadas para ese dominio.

No mezclar URLs de distintos tenants en un sitemap que corresponda a un dominio individual.

Los sitemaps deben respetar la configuración de publicación del establecimiento.

# 9. Robots.txt

Cada sitio público debe disponer de una configuración correcta de `robots.txt`.

Debe:

- Permitir el rastreo de contenido público.
- Evitar el rastreo innecesario de recursos privados cuando proceda.
- Referenciar el sitemap correcto.
- Evitar reglas contradictorias.
- Mantener coherencia con las políticas de indexación.

No utilizar `robots.txt` como mecanismo de protección de información.

No publicar rutas internas, nombres de recursos sensibles o detalles innecesarios de infraestructura.

Las restricciones de acceso deben aplicarse en el servidor.

# 10. Open Graph y vistas previas

Todas las páginas públicas que puedan compartirse deben tener metadatos de previsualización.

Incluir según corresponda:

- `og:title`
- `og:description`
- `og:url`
- `og:type`
- `og:image`
- `og:image:alt`
- `og:site_name`
- `og:locale`

También configurar Twitter Cards u otros metadatos compatibles.

## 10.1 Imágenes

La imagen de referencia para Open Graph será:

- 1200 × 630 píxeles.
- Relación aproximada 1.91:1.
- Formato optimizado.
- URL pública HTTPS.
- Accesible para los sistemas que generan vistas previas.

No utilizar URLs firmadas temporales ni recursos privados.

## 10.2 Contenido dinámico

Generar metadatos según el tipo de página:

- Establecimiento.
- Sucursal.
- Servicio.
- Profesional.
- Producto.
- Promoción.
- Página de descubrimiento.

No mostrar datos personales privados en títulos, descripciones o imágenes públicas.

No inventar precios, calificaciones, horarios ni disponibilidad.

## 10.3 Multi-tenant

Cada establecimiento debe mostrar su propia identidad visual y contenido autorizado.

Validar el dominio y la asociación con el tenant antes de construir metadatos.

La caché de metadata debe respetar el aislamiento entre tenants.

# 11. Favicon e identidad visual

Todos los sitios deben disponer de favicon.

Preparar los recursos necesarios para:

- Navegadores de escritorio.
- Navegadores móviles.
- Pantallas de inicio cuando corresponda.
- PWA cuando corresponda.

Los recursos deben mantener coherencia con la identidad visual de ENSAMBRIA o del establecimiento.

No utilizar un favicon genérico cuando el tenant tenga una identidad de marca configurada.

Validar que:

- Los archivos existan.
- Las rutas sean correctas.
- Los formatos sean compatibles.
- Los tamaños sean adecuados.
- No existan errores 404.

# 12. Imágenes y recursos multimedia

## 12.1 Optimización

Las imágenes deben optimizarse para el uso real.

Preferir formatos modernos como:

- AVIF.
- WebP.

Mantener formatos alternativos cuando exista una necesidad de compatibilidad.

## 12.2 Responsive images

Utilizar:

- `srcset`.
- `sizes`.
- Dimensiones explícitas.
- Variantes de resolución.
- Carga adaptada al dispositivo.

Evitar entregar imágenes enormes a dispositivos móviles cuando no sea necesario.

## 12.3 Lazy loading

Aplicar carga diferida a imágenes que estén fuera del área inicial de visualización.

No aplicar lazy loading indiscriminadamente a la imagen principal o LCP.

## 12.4 Dimensiones

Definir ancho y alto cuando corresponda para evitar cambios de diseño durante la carga.

Prevenir CLS.

## 12.5 Metadatos

Eliminar metadatos innecesarios, como coordenadas GPS, cuando corresponda.

No publicar información sensible contenida en archivos multimedia.

## 12.6 Propiedad intelectual

Verificar que ENSAMBRIA o el establecimiento tenga autorización para utilizar las imágenes.

Aplicar la política:

`ENSAMBRIA_Privacy_Legal_Compliance.md`

# 13. Texto alternativo de imágenes

Todas las imágenes informativas deben disponer de un atributo `alt` apropiado.

Reglas:

- Describir la información visual relevante.
- Ser conciso.
- Evitar palabras clave artificiales.
- No repetir innecesariamente el texto adyacente.
- No inventar información.
- Considerar el contexto de la imagen.

Las imágenes decorativas deben utilizar `alt=""`.

Las imágenes funcionales deben describir la acción o propósito del elemento.

Los iconos utilizados como controles deben disponer de un nombre accesible.

El texto alternativo no debe incluir información personal innecesaria.

# 14. Rendimiento y velocidad de carga

El rendimiento es un requisito funcional de calidad.

## 14.1 Core Web Vitals

Utilizar como objetivos de referencia:

| Métrica | Objetivo |
|---|---:|
| LCP | ≤ 2.5 segundos |
| INP | ≤ 200 ms |
| CLS | ≤ 0.1 |

Estos valores deben evaluarse en el percentil 75 cuando existan datos de usuarios reales suficientes.

Lighthouse y pruebas sintéticas son complementarias; no sustituyen las métricas reales.

## 14.2 Optimización

Revisar:

- JavaScript enviado al cliente.
- División de bundles.
- Carga diferida de componentes.
- CSS.
- Fuentes.
- Imágenes.
- Caché.
- Compresión.
- CDN.
- SSR.
- Streaming cuando corresponda.
- Prefetching.
- Consultas duplicadas.
- Recursos externos.
- Hidratación innecesaria.

## 14.3 JavaScript

Evitar enviar lógica innecesaria al navegador.

Preferir renderizado de servidor para contenido público cuando la arquitectura lo permita.

No convertir componentes estáticos en componentes cliente sin una necesidad real.

## 14.4 Fuentes

- Optimizar formatos.
- Limitar variantes.
- Evitar pesos innecesarios.
- Utilizar carga adecuada.
- Evitar solicitudes externas innecesarias.

## 14.5 Caché

Definir políticas de caché según el tipo de contenido.

Diferenciar:

- Contenido público.
- Contenido personalizado.
- Información privada.
- Información sensible.
- Respuestas autenticadas.

No permitir que una respuesta de un tenant se entregue a otro por una clave de caché incorrecta.

## 14.6 Presupuesto de rendimiento

Cada frontend debe definir presupuestos para:

- JavaScript inicial.
- CSS.
- Imágenes.
- Fuentes.
- Número de solicitudes.
- Tiempo de renderizado.
- Core Web Vitals.

Los presupuestos deben justificarse y verificarse en CI cuando sea viable.

# 15. Accesibilidad

ENSAMBRIA utilizará WCAG 2.2 AA como objetivo de referencia.

La accesibilidad debe integrarse en los componentes y sistemas de diseño.

## 15.1 Contraste

Verificar contraste de:

- Texto.
- Botones.
- Formularios.
- Iconos informativos.
- Estados.
- Elementos interactivos.

Como referencia general:

- 4.5:1 para texto normal.
- 3:1 para texto grande.
- 3:1 para componentes gráficos y elementos de interfaz relevantes.

No depender exclusivamente del color para transmitir información.

## 15.2 Navegación

Todas las funcionalidades deben ser accesibles mediante teclado cuando corresponda.

Verificar:

- Orden de tabulación.
- Foco visible.
- Ausencia de trampas de teclado.
- Navegación lógica.
- Accesibilidad de menús.
- Accesibilidad de diálogos.
- Cierre de modales.
- Retorno de foco.

## 15.3 Formularios

Cada campo debe tener:

- Etiqueta.
- Nombre accesible.
- Instrucciones cuando sean necesarias.
- Errores comprensibles.
- Asociación entre error y campo.
- Indicadores de estado.

No utilizar placeholders como sustitutos permanentes de etiquetas.

## 15.4 Semántica

Utilizar HTML semántico.

Evitar controles interactivos construidos con elementos no interactivos sin las propiedades y comportamientos correspondientes.

## 15.5 Lectores de pantalla

Verificar:

- Títulos jerárquicos.
- Etiquetas.
- Nombres accesibles.
- Estados.
- Mensajes dinámicos.
- Alertas.
- Tablas.
- Navegación.
- Elementos multimedia.

## 15.6 Movimiento

Respetar `prefers-reduced-motion`.

Evitar animaciones que generen problemas de accesibilidad.

No utilizar destellos o efectos que puedan provocar riesgos para usuarios sensibles.

# 16. Diseño responsive y compatibilidad móvil

Todos los sitios deben funcionar correctamente en:

- Móviles pequeños.
- Móviles grandes.
- Tablets.
- Portátiles.
- Escritorios.
- Pantallas de alta resolución.

## 16.1 Requisitos

- Diseño adaptable.
- Navegación móvil funcional.
- Botones utilizables.
- Formularios adaptados.
- Tablas adaptables.
- Imágenes responsive.
- Tipografía legible.
- Espaciado adecuado.
- Ausencia de desbordamientos horizontales injustificados.
- Modales adaptados.
- Menús accesibles.

## 16.2 Touch

Los controles deben tener objetivos táctiles adecuados.

No utilizar elementos demasiado pequeños o demasiado próximos.

## 16.3 Pruebas

Verificar en navegadores y tamaños representativos.

No considerar suficiente que el sitio funcione en un único emulador.

# 17. Página 404 y manejo de errores

Todos los sitios públicos deben tener una página 404 personalizada.

Debe:

- Explicar que el recurso no existe.
- Mantener la identidad visual.
- Ofrecer navegación útil.
- Incluir un enlace a inicio.
- Permitir continuar la experiencia.
- No revelar información interna.
- Responder con HTTP 404 real.

No devolver HTTP 200 para páginas inexistentes.

Las rutas privadas deben evitar revelar si un recurso existe cuando ello facilite enumeración o acceso indebido.

También deben existir páginas de error apropiadas para:

- 401.
- 403.
- 404.
- 429.
- 500.
- 503.

No mostrar trazas, secretos, consultas SQL ni detalles internos.

# 18. Enlaces y navegación

Todos los enlaces deben funcionar correctamente.

Verificar:

- Enlaces internos.
- Enlaces externos.
- Enlaces de navegación.
- Enlaces de pie de página.
- Enlaces legales.
- Enlaces de imágenes.
- Enlaces de botones.
- Enlaces de CTA.
- Enlaces de redirección.
- Enlaces de dominios personalizados.

## 18.1 Reglas

- No dejar enlaces vacíos.
- No utilizar `href="#"` como sustituto de una acción.
- No dejar botones sin comportamiento.
- Evitar redirecciones abiertas.
- Validar destinos externos.
- Gestionar enlaces que ya no existen.
- Corregir referencias a recursos eliminados.

Los enlaces externos que abran una pestaña nueva deben gestionarse de manera segura.

# 19. Formularios y validación

Todos los formularios deben implementar validación en frontend y backend.

## 19.1 Frontend

Validar:

- Campos requeridos.
- Formatos.
- Longitudes.
- Rangos.
- Tipos.
- Estados.
- Dependencias entre campos.

Mostrar mensajes claros.

No perder innecesariamente los datos introducidos cuando ocurra un error recuperable.

## 19.2 Backend

El backend debe repetir las validaciones críticas.

No confiar en los valores recibidos del navegador.

Validar:

- Esquemas.
- Tipos.
- Límites.
- Permisos.
- Identidad.
- Tenant.
- Integridad.
- Restricciones de negocio.

## 19.3 Seguridad

Cubrir:

- XSS.
- SQL injection.
- CSRF cuando corresponda.
- Mass assignment.
- Inyección de comandos.
- Manipulación de precios.
- Manipulación de IDs.
- Cargas maliciosas.
- Abuso de endpoints.

No incluir datos sensibles en mensajes de error.

# 20. Protección contra spam y abuso

Los formularios públicos deben contar con protección proporcional al riesgo.

Opciones:

- Rate limiting.
- Controles de frecuencia.
- Honeypots.
- Verificación de origen cuando corresponda.
- CAPTCHA o alternativas de desafío.
- Detección de patrones automatizados.
- Bloqueos temporales.
- Límites por IP o identificador cuando sean adecuados.

No imponer CAPTCHA a todos los usuarios sin justificación.

Las soluciones antispam de terceros deben evaluarse bajo la política de privacidad.

Los límites deben evitar bloquear injustificadamente a usuarios legítimos.

# 21. Analítica y medición

ENSAMBRIA debe medir la calidad y el uso de sus experiencias sin vulnerar la privacidad.

## 21.1 Objetivos

Medir cuando corresponda:

- Visitas.
- Rendimiento.
- Conversión.
- Reservas.
- Registro.
- Uso de funcionalidades.
- Errores.
- Embudos.
- Abandono.
- Experiencia de navegación.

## 21.2 Privacidad

La analítica debe respetar:

- Jurisdicción.
- Consentimiento.
- Finalidad.
- Minimización.
- Retención.
- Control de proveedores.
- Transferencias internacionales.

No transmitir datos personales innecesarios.

No capturar contraseñas, tarjetas, datos sensibles ni contenido privado.

No activar session replay indiscriminadamente.

## 21.3 Eventos

Los eventos deben estar documentados.

Ejemplos:

- `page_view`
- `registration_started`
- `registration_completed`
- `booking_started`
- `booking_completed`
- `checkout_started`
- `checkout_completed`

No incluir datos personales en nombres o propiedades de eventos cuando no sean estrictamente necesarios y jurídicamente admisibles.

## 21.4 Separación

Diferenciar:

- Analítica de producto ENSAMBRIA.
- Analítica de negocio del establecimiento.
- Analítica pública de discovery.
- Analítica de administración de plataforma.

No mezclar datos de tenants ni exponer métricas privadas.

# 22. CTA y conversión

Cada página debe tener un objetivo principal claro.

Ejemplos:

| Página | CTA principal |
|---|---|
| Sitio de establecimiento | Reservar cita. |
| Página de servicio | Reservar servicio. |
| Página de profesional | Ver disponibilidad o reservar. |
| Discovery | Descubrir establecimientos. |
| Landing de ENSAMBRIA | Crear cuenta o contratar. |
| Panel empresarial | Ejecutar la acción operativa correspondiente. |

## 22.1 Reglas

- CTA visible.
- Texto claro.
- Acción coherente.
- Destino correcto.
- Estado de carga.
- Estado de error.
- Confirmación de éxito.
- Accesibilidad.
- Compatibilidad móvil.

No añadir múltiples CTA principales que compitan entre sí sin justificación.

No utilizar mensajes engañosos ni urgencia falsa.

# 23. Internacionalización

ENSAMBRIA debe estar preparado para diferentes idiomas y mercados.

Separar:

- Idioma.
- País.
- Moneda.
- Zona horaria.
- Formato de fecha.
- Formato numérico.
- Jurisdicción.
- Dominio.
- Contenido legal.

No inferir legislación exclusivamente a partir del idioma.

## 23.1 Traducciones

Todos los textos visibles deben poder internacionalizarse.

Evitar textos de interfaz dispersos y codificados directamente en componentes.

Los textos legales requieren revisión adecuada por mercado.

## 23.2 SEO internacional

Cuando corresponda, implementar:

- URLs por idioma o mercado.
- `hreflang`.
- Canonical.
- Sitemaps específicos.
- Metadatos traducidos.
- Contenido coherente con cada mercado.

No crear versiones duplicadas artificialmente para manipular posicionamiento.

# 24. Diseño multi-tenant y dominios personalizados

El sistema debe proteger la identidad y los datos de cada establecimiento.

## 24.1 Resolución de dominio

Validar que el dominio solicitado esté registrado y asociado al tenant correspondiente.

No confiar en un dominio enviado por el cliente como fuente de identidad del tenant.

## 24.2 Separación

Garantizar que:

- Los metadatos correspondan al tenant correcto.
- Las imágenes correspondan al tenant correcto.
- El favicon sea el autorizado.
- El sitemap corresponda al dominio.
- La caché esté aislada.
- Las páginas no filtren información de otros establecimientos.
- Los enlaces canónicos utilicen el dominio autorizado.

## 24.3 Contenido público

Solo publicar contenido autorizado por el establecimiento y conforme a las políticas de PUBLIC.

BUSINESS es propietario de los datos operativos y PUBLIC gestiona las proyecciones públicas autorizadas.

No convertir el frontend en propietario de reglas comerciales.

# 25. Arquitectura y tecnologías

La implementación debe respetar las tecnologías y convenciones aprobadas para ENSAMBRIA.

Para Tenant Portal, Marketplace, Platform Admin y Corporate:

- Next.js.
- TypeScript estricto.
- Componentes reutilizables.
- SSR y rendering híbrido cuando corresponda.
- Metadata API.
- Validación tipada.
- Pruebas automatizadas.
- Diseño responsive.

Para ENSAMBRIA Business y ENSAMBRIA App:
- Flutter + Dart.
- Business debe validar experiencias móviles/tablet y de escritorio; no tratar escritorio como una versión ampliada de la UI móvil.
- App se limita a Android/iOS/tablets y a las capacidades aprobadas para clientes.
- Controles de accesibilidad, seguridad, privacidad, rendimiento, sesiones y pruebas correspondientes a cada sistema operativo.

No introducir dependencias sin justificar:

- Seguridad.
- Mantenimiento.
- Tamaño.
- Licencia.
- Impacto en rendimiento.
- Privacidad.
- Compatibilidad.

No debilitar TypeScript con `any`, casts inseguros o desactivación de reglas para evitar resolver errores.

No duplicar lógica de dominio de los backends en los frontends.

# 26. Pruebas automatizadas

Todos los requisitos deben tener pruebas proporcionales al riesgo.

## 26.1 Calidad general

Ejecutar los comandos existentes del repositorio para:

- Formato.
- Lint.
- Typecheck.
- Unit tests.
- Integration tests.
- Build.
- E2E.

No inventar scripts ni modificar los existentes sin autorización.

## 26.2 SEO

Probar:

- Title.
- Description.
- Canonical.
- Open Graph.
- Twitter Cards.
- Sitemap.
- Robots.
- Hreflang.
- Metadata SSR.
- URLs de dominios personalizados.
- Exclusión de páginas privadas.

## 26.3 Seguridad

Probar:

- Ausencia de secretos en bundles.
- CSP.
- HTTPS.
- Cookies.
- Control de acceso.
- XSS.
- CSRF cuando corresponda.
- Formularios.
- Rate limiting.
- Aislamiento de tenants.

## 26.4 Accesibilidad

Utilizar herramientas automatizadas y pruebas manuales.

Verificar:

- Contraste.
- Navegación por teclado.
- Foco.
- Labels.
- Semántica.
- Lectores de pantalla.
- Modales.
- Formularios.
- Estados dinámicos.

Las herramientas automáticas no detectan todos los problemas.

## 26.5 Rendimiento

Ejecutar pruebas sintéticas y, cuando exista tráfico, medición de usuarios reales.

Evaluar:

- LCP.
- INP.
- CLS.
- Tamaño de bundles.
- Peso de imágenes.
- Tiempo de carga.
- Errores de red.
- Hidratación.

## 26.6 E2E

Probar flujos reales:

- Navegación.
- Registro.
- Inicio de sesión.
- Formularios.
- Reservas.
- Checkout.
- Cancelaciones.
- Recuperación de errores.
- Navegación móvil.

No sustituir pruebas críticas por capturas visuales o mocks.

# 27. Checklist obligatorio por página

Cada página debe evaluarse según sus características.

## Seguridad

- [ ] HTTPS.
- [ ] Sin secretos en frontend.
- [ ] Control de acceso cuando corresponda.
- [ ] Cabeceras adecuadas.
- [ ] Formularios protegidos.
- [ ] Recursos externos evaluados.
- [ ] Sin errores que expongan información interna.

## Legal y privacidad

- [ ] Política de privacidad accesible.
- [ ] Términos aplicables accesibles.
- [ ] Consentimiento cuando corresponda.
- [ ] Cookies configuradas.
- [ ] Analytics conforme a preferencias.
- [ ] Datos personales minimizados.

## SEO

- [ ] Title único.
- [ ] Description.
- [ ] Canonical.
- [ ] Open Graph.
- [ ] Twitter Cards.
- [ ] Sitemap actualizado cuando corresponda.
- [ ] Robots correcto.
- [ ] Indexación adecuada.

## Identidad

- [ ] Favicon.
- [ ] Logo correcto.
- [ ] Imagen de previsualización.
- [ ] Identidad del tenant correcta.

## Imágenes

- [ ] Formato optimizado.
- [ ] Dimensiones adecuadas.
- [ ] Texto alternativo.
- [ ] Lazy loading donde corresponda.
- [ ] Sin metadatos sensibles innecesarios.

## Rendimiento

- [ ] Core Web Vitals evaluados.
- [ ] JavaScript optimizado.
- [ ] CSS optimizado.
- [ ] Imágenes comprimidas.
- [ ] Fuentes optimizadas.
- [ ] Caché correcta.
- [ ] Sin recursos innecesarios.

## Accesibilidad

- [ ] Contraste.
- [ ] Navegación por teclado.
- [ ] Foco visible.
- [ ] Etiquetas.
- [ ] Semántica.
- [ ] Estados accesibles.
- [ ] Responsive.

## Experiencia

- [ ] CTA claro.
- [ ] Enlaces funcionales.
- [ ] Formularios validados.
- [ ] Protección antispam.
- [ ] Página 404.
- [ ] Estados de error y carga.
- [ ] Experiencia móvil comprobada.

# 28. Reglas para el agente de desarrollo

El agente debe:

1. Inspeccionar el repositorio antes de modificar.
2. Identificar el frontend y su objetivo.
3. Identificar las rutas y páginas afectadas.
4. Revisar componentes y convenciones existentes.
5. Revisar la versión real de Next.js y dependencias.
6. Respetar los contratos de backend.
7. Respetar los límites entre dominios.
8. Evitar duplicar lógica de negocio.
9. No inventar endpoints.
10. No inventar proveedores.
11. No introducir secretos.
12. No habilitar trackers sin autorización.
13. No publicar información privada.
14. No desactivar reglas de lint o typecheck.
15. No introducir mocks como sustitutos de pruebas reales.
16. No dejar enlaces o controles sin comportamiento.
17. No dejar páginas incompletas.
18. No añadir TODOs para simular implementación.
19. No modificar scripts, workflows o infraestructura fuera del alcance autorizado.
20. Ejecutar las pruebas existentes y las necesarias.
21. Corregir los errores introducidos.
22. Actualizar documentación y roadmap.
23. Presentar evidencia verificable de los resultados.

Antes de modificar una funcionalidad debe documentar:

- Requisito.
- Página o componente afectado.
- Riesgo.
- Implementación.
- Prueba.
- Resultado.

Si un requisito no aplica, debe justificarlo y documentarlo. No marcarlo como cumplido sin evaluación.

# 29. Definition of Done

Una página o funcionalidad se considera terminada cuando:

- El diseño está implementado.
- La experiencia móvil funciona.
- La seguridad está verificada.
- La privacidad está respetada.
- Los metadatos están implementados.
- Los enlaces funcionan.
- Los formularios están protegidos.
- El rendimiento ha sido evaluado.
- La accesibilidad ha sido evaluada.
- Los errores tienen estados apropiados.
- Las pruebas automatizadas pasan.
- Las pruebas E2E correspondientes pasan.
- La documentación está actualizada.
- No hay secretos expuestos.
- No hay fugas entre tenants.
- CI está en verde.
- Los requisitos pendientes están identificados.

No declarar una página lista para producción únicamente porque carga correctamente en el navegador.

# 30. Regla final

Cada frontend de ENSAMBRIA debe cumplir este estándar desde su primera implementación.

No debe existir una fase posterior de limpieza general para resolver problemas que podían evitarse desde el diseño.

La calidad web debe ser una propiedad permanente de ENSAMBRIA:

**Seguridad + Privacidad + SEO + Rendimiento + Accesibilidad + Responsive + UX + Pruebas + Verificación = Calidad web de producción.**

Este estándar es obligatorio para todos los nuevos desarrollos, modificaciones y lanzamientos de ENSAMBRIA.
