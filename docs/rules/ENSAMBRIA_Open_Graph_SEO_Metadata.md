# ENSAMBRIA — Regla de implementación: Open Graph, SEO técnico y metadatos públicos

**Estado:** Regla arquitectónica y de implementación obligatoria
**Ámbito:** Marketplace, Corporate y contenido público autorizado de ENSAMBRIA
**Backends relacionados:** PUBLIC, BUSINESS y CORE
**Tecnologías:** Next.js, TypeScript estricto, Rust y PostgreSQL
**Objetivo:** Garantizar que todas las páginas públicas de ENSAMBRIA tengan metadatos correctos, vistas previas profesionales al compartir enlaces, indexación apropiada y datos estructurados para buscadores.

**Aplicación por fases:** Fase 0 define inventario de rutas/superficies, ownership, contratos, privacidad y criterios de aceptación; no crea runtime de producto. La capacidad base de metadatos rastreables/Open Graph forma parte de la primera fase de implementación (Fase 1). Desde Fase 1 en adelante, toda fase que introduzca o modifique páginas o contenido públicos implementa y prueba en esa misma fase su SEO completo. Si las superficies previstas para Fase 5 o Fase 9 se exponen antes, sus requisitos no se aplazan. No se crean páginas públicas en estado incompleto para completarlas después.

Esta guía es la fuente normativa detallada de SEO técnico/editorial, metadatos, Open Graph e indexabilidad. Convive y se complementa con [ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md](ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md), que define la calidad web general y los criterios de lanzamiento, y con [ENSAMBRIA_Privacy_Legal_Compliance.md](ENSAMBRIA_Privacy_Legal_Compliance.md), fuente normativa de privacidad y cumplimiento legal. Ninguna de esas guías reemplaza esta especificación de SEO; esta guía tampoco reemplaza los requisitos de calidad web o privacidad de sus documentos propietarios.

---

# 1. Objetivo y regla fundamental

ENSAMBRIA debe implementar desde Fase 1 la capacidad base y, en cada fase posterior que introduzca páginas públicas, una estrategia integral de:

- Open Graph.
- Twitter Cards / metadatos para X.
- SEO técnico.
- Metadatos dinámicos por página.
- URLs canónicas.
- Datos estructurados mediante Schema.org y JSON-LD.
- Sitemap XML.
- Robots.txt.
- Optimización de imágenes públicas.
- Indexabilidad y control de contenido duplicado.
- Caché e invalidación de metadatos.

No se debe tratar Open Graph como una característica decorativa o un añadido posterior.

Cada página pública debe proporcionar metadatos coherentes con su contenido, negocio, idioma, dominio y estado de publicación.

**Regla fundamental:** los metadatos deben estar disponibles en el HTML inicial generado por el servidor o durante la generación estática. No depender exclusivamente de JavaScript ejecutado en el navegador para que los rastreadores puedan encontrarlos.

---

# 2. Alcance dentro de la arquitectura ENSAMBRIA

La implementación debe respetar las responsabilidades de los cuatro backends y los seis proyectos de interfaz.

## 2.1 PUBLIC BACKEND

Es responsable de proporcionar los datos públicos autorizados utilizados por:

- Directorio de establecimientos.
- Marketplace.
- Discovery.
- Perfiles públicos de negocios.
- Búsqueda y descubrimiento.
- Reseñas publicadas.
- Promociones públicas autorizadas.
- Proyecciones públicas.

PUBLIC puede proporcionar los datos necesarios para generar metadatos y datos estructurados, pero no debe apropiarse de los datos operativos privados de BUSINESS.

## 2.2 BUSINESS BACKEND

Es propietario de los datos operativos de los establecimientos y de la información de negocio que corresponda publicar:

- Nombre comercial.
- Descripciones públicas.
- Servicios.
- Productos.
- Profesionales.
- Sucursales.
- Horarios.
- Imágenes autorizadas.
- Promociones.
- Información comercial.

La publicación de estos datos debe respetar los permisos, configuración y estado de visibilidad del negocio.

PUBLIC consume proyecciones autorizadas de BUSINESS. No debe leer directamente sus tablas privadas.

## 2.3 CORE BACKEND

Mantiene las responsabilidades centrales de identidad, tenancy, organizaciones y autorización.

Los metadatos públicos no deben permitir inferir información privada de usuarios, membresías, reservas, pagos o datos internos de un tenant.

CORE no debe convertirse en un generador de HTML ni en un servicio de SEO.

## 2.4 Frontends

| Frontend | Responsabilidad |
|---|---|
| `ensambria-business` | Administrar, cuando corresponda, información del negocio que puede publicarse; compartir URLs canónicas, no generar páginas web indexables |
| `tenant-portal` | Administrar contenido publicable; panel autenticado `noindex`, sin previews privadas |
| `ensambria-app` | Compartir enlaces públicos mediante URLs canónicas/deep links; no sustituye HTML indexable |
| `marketplace` | Metadatos de Discovery, directorio, perfiles públicos y páginas de negocio/servicios publicadas |
| `platform-admin` | Administración global privada; `noindex`, sin información de tenant en previews públicas |
| `corporate` | Metadatos de contenido institucional/comercial público propiedad de PLATFORM |

Los frontends administrativos y autenticados no deben indexar información privada.

---

# 3. Open Graph: implementación obligatoria

Open Graph permite controlar la información utilizada por las plataformas que generan vistas previas al compartir un enlace.

Ejemplo conceptual:

```text
Usuario comparte:
https://studiohair.ENSAMBRIA.com/servicios/coloracion

Vista previa:
+--------------------------------------+
|                                      |
|             IMAGEN OG                |
|                                      |
+--------------------------------------+
| Coloración profesional               |
| Studio Hair Medellín                  |
| Descubre nuestros servicios y reserva|
| studiohair.ENSAMBRIA.com                |
+--------------------------------------+
```

Cada página pública debe definir metadatos Open Graph dinámicos según el contenido que representa.

## 3.1 Etiquetas obligatorias

Para las páginas públicas que admitan vista previa:

```html
<meta property="og:type" content="website" />

<meta property="og:title"
      content="Studio Hair Medellín" />

<meta property="og:description"
      content="Conoce nuestros servicios y reserva tu cita." />

<meta property="og:url"
      content="https://studiohair.ENSAMBRIA.com/" />

<meta property="og:image"
      content="https://cdn.ENSAMBRIA.com/public/og/studio-hair.jpg" />

<meta property="og:image:alt"
      content="Interior de Studio Hair Medellín" />

<meta property="og:image:width"
      content="1200" />

<meta property="og:image:height"
      content="630" />

<meta property="og:site_name"
      content="Studio Hair Medellín" />

<meta property="og:locale"
      content="es_CO" />
```

La imagen de ejemplo es ilustrativa. Las imágenes y URLs reales deben proceder de recursos públicos autorizados.

## 3.2 Reglas para las etiquetas

- `og:title`: título específico de la página.
- `og:description`: resumen claro, relevante y fiel al contenido.
- `og:type`: tipo de contenido.
- `og:url`: URL canónica absoluta.
- `og:image`: imagen pública representativa.
- `og:image:alt`: descripción accesible de la imagen.
- `og:image:width` y `og:image:height`: dimensiones reales de la imagen cuando se declaren.
- `og:site_name`: nombre comercial o de la experiencia pública correspondiente.
- `og:locale`: idioma y región del contenido.

No repetir automáticamente el mismo título, descripción e imagen en todas las páginas de un negocio.

## 3.3 Tipo de contenido

Seleccionar el tipo Open Graph que corresponda al contenido real.

Ejemplos:

- Página general del negocio: `website`.
- Artículo o publicación editorial: `article`.
- Otras experiencias: utilizar un tipo compatible con la especificación cuando corresponda.

No declarar que una página es un artículo si realmente representa un servicio, establecimiento o producto.

---

# 4. Twitter Cards y metadatos para X

ENSAMBRIA debe implementar metadatos compatibles con las vistas previas de enlaces de X.

Ejemplo:

```html
<meta name="twitter:card"
      content="summary_large_image" />

<meta name="twitter:title"
      content="Studio Hair Medellín" />

<meta name="twitter:description"
      content="Conoce nuestros servicios y reserva tu cita." />

<meta name="twitter:image"
      content="https://cdn.ENSAMBRIA.com/public/og/studio-hair.jpg" />

<meta name="twitter:image:alt"
      content="Interior de Studio Hair Medellín" />
```

Reglas:

- Utilizar `summary_large_image` cuando exista una imagen horizontal apropiada.
- Los títulos y descripciones deben corresponder al contenido de la página.
- La imagen debe ser pública y accesible para los rastreadores.
- No depender exclusivamente de metadatos de X; mantener también Open Graph.
- No asumir que todas las plataformas muestran las tarjetas de forma idéntica.

---

# 5. Metadatos dinámicos por tipo de página

Los metadatos deben generarse de acuerdo con la página y el recurso público.

| Tipo de página | Contenido mínimo |
|---|---|
| Página principal del negocio | Nombre, descripción e imagen de marca |
| Página de sucursal | Nombre de la sucursal, ubicación pública e imagen |
| Página de servicio | Nombre del servicio, descripción e imagen |
| Página de profesional | Nombre público, especialidad e imagen autorizada |
| Página de producto | Nombre, descripción e imagen del producto |
| Página de promoción | Nombre, descripción, vigencia e imagen cuando corresponda |
| Página de reserva | Información pública del negocio y contexto general, sin datos privados |
| Perfil del marketplace | Nombre comercial, descripción, imagen y ubicación pública |
| Página de categoría | Nombre, descripción y contexto de la categoría |
| Página editorial | Título, descripción, autor y fechas cuando correspondan |

## 5.1 Reglas de generación

Cada página debe:

1. Resolver el recurso público correspondiente.
2. Validar que exista y esté publicado.
3. Obtener exclusivamente los datos autorizados.
4. Generar título y descripción específicos.
5. Resolver su URL canónica.
6. Seleccionar una imagen pública válida.
7. Generar los metadatos Open Graph.
8. Generar los metadatos para X.
9. Incorporar datos estructurados cuando corresponda.
10. Aplicar la política de indexación de esa página.

Si el recurso no está publicado, fue eliminado o no es accesible públicamente, no debe generar una página indexable con información obsoleta o privada.

---

# 6. Imágenes Open Graph

Las imágenes utilizadas para compartir enlaces son parte de la experiencia pública de cada establecimiento.

## 6.1 Requisitos

- Utilizar preferiblemente una proporción de 1.91:1.
- Como dimensión de referencia, utilizar 1200 × 630 píxeles.
- Utilizar formatos compatibles con los consumidores de la vista previa.
- Evitar imágenes demasiado pesadas.
- Mantener una calidad visual apropiada.
- Evitar marcas de agua no autorizadas.
- Asegurar que el contenido principal no quede oculto por recortes.
- Incorporar texto alternativo.
- Verificar que la imagen se pueda descargar sin autenticación.
- Utilizar URLs HTTPS absolutas y estables.

Las dimensiones recomendadas son una referencia de implementación, no una garantía de que todas las plataformas representen la imagen de idéntica manera.

## 6.2 Variantes de imágenes

Se deben contemplar imágenes diferenciadas para:

- Marca del establecimiento.
- Portada del establecimiento.
- Sucursales.
- Servicios.
- Productos.
- Profesionales.
- Promociones.
- Marketplace.

Cuando un negocio no tenga una imagen específica, se puede utilizar una imagen predeterminada de ENSAMBRIA que sea apropiada para la categoría.

No utilizar una imagen genérica de una barbería para representar una página específica de spa, peluquería o centro de estética si existe una alternativa más representativa.

## 6.3 Almacenamiento y distribución

Las imágenes pueden almacenarse en el servicio de Object Storage utilizado por ENSAMBRIA, con distribución mediante CDN.

Reglas:

- La imagen pública debe tener una URL accesible por los rastreadores.
- No utilizar URLs privadas que requieren cookies o sesión.
- No exponer buckets completos ni credenciales de almacenamiento.
- No utilizar URLs temporales que expiren antes de que se genere la vista previa.
- Mantener una separación entre imágenes públicas y privadas.
- Validar los permisos antes de publicar una imagen.
- Invalidar o actualizar cachés cuando cambie la imagen.
- No asumir que modificar una imagen actualiza inmediatamente las vistas previas ya almacenadas por terceros.

La URL pública debe apuntar a un recurso que ENSAMBRIA tenga autorización para exponer.

---

# 7. SEO técnico

Open Graph y SEO tienen propósitos diferentes.

Open Graph controla principalmente las vistas previas de enlaces compartidos. SEO técnico permite que los buscadores descubran, interpreten y gestionen las páginas.

ENSAMBRIA debe implementar ambas capacidades de forma coordinada.

## 7.1 Etiquetas SEO básicas

Cada página pública debe generar:

```html
<title>Studio Hair Medellín | Servicios y reservas</title>

<meta name="description"
      content="Conoce los servicios, profesionales y disponibilidad de Studio Hair Medellín." />

<meta name="robots"
      content="index, follow" />

<link rel="canonical"
      href="https://studiohair.ENSAMBRIA.com/" />
```

Las directivas exactas de indexación deben depender del estado y la naturaleza de la página.

No establecer `index, follow` indiscriminadamente para todas las rutas.

## 7.2 Título y descripción

Reglas:

- Cada página indexable debe tener un título único y descriptivo.
- Cada página indexable debe tener una descripción relevante.
- Evitar títulos repetidos y descripciones genéricas.
- No incluir palabras clave artificiales o repetitivas.
- No generar contenido engañoso.
- No insertar datos inexistentes.
- Evitar títulos excesivamente largos.
- Mantener coherencia entre el contenido visible y sus metadatos.

## 7.3 URLs canónicas

Cada página pública debe declarar una URL canónica absoluta.

Reglas:

- La canonical debe apuntar al dominio oficial de la página.
- Evitar canonical hacia dominios de otros tenants.
- Resolver correctamente parámetros de consulta.
- Evitar generar múltiples URLs indexables para el mismo contenido.
- Mantener coherencia entre URLs con y sin barra final conforme a la política de rutas.
- Utilizar URLs HTTPS.
- Las páginas eliminadas o trasladadas deben seguir una política explícita de redirección o respuesta HTTP.

No construir una canonical confiando ciegamente en el encabezado `Host` recibido.

El dominio debe resolverse mediante la configuración y el registro autorizado de dominios de ENSAMBRIA.

---

# 8. Dominios personalizados y arquitectura multi-tenant

ENSAMBRIA permitirá que cada establecimiento tenga una URL o dominio personalizado.

Ejemplos:

```text
studiohair.ENSAMBRIA.com

barberia.ENSAMBRIA.com

www.studiohair.com

reservas.studiohair.com
```

La generación de metadatos debe respetar el dominio oficial asignado al negocio.

## 8.1 Resolución de tenant

Antes de generar metadatos:

1. Resolver el host recibido.
2. Consultar el registro autorizado de dominios.
3. Determinar el tenant correspondiente.
4. Obtener la página pública asociada.
5. Verificar que la publicación esté activa.
6. Generar las URLs canónicas utilizando el dominio verificado.
7. Asegurar que no se filtren metadatos de otro tenant.

No permitir que un usuario manipule el encabezado `Host` para obtener información o URLs pertenecientes a otro establecimiento.

## 8.2 Aislamiento entre tenants

Probar obligatoriamente:

- Un tenant no puede mostrar metadatos de otro tenant.
- Un dominio no verificado no puede suplantar un dominio oficial.
- Una sucursal no puede obtener los datos privados de otra.
- Una página eliminada no puede seguir exponiendo información privada mediante cachés.
- Una imagen de un tenant no puede utilizarse para obtener recursos privados de otro.
- La generación de URLs canónicas siempre respeta el dominio autorizado.

Las URLs UUID no sustituyen autorización ni aislamiento multi-tenant.

---

# 9. Schema.org y JSON-LD

ENSAMBRIA debe implementar datos estructurados mediante Schema.org utilizando JSON-LD cuando sean apropiados para el contenido.

Estos datos permiten describir entidades y contenido de manera estructurada para los buscadores.

## 9.1 Tipos de contenido

Evaluar el tipo apropiado para cada página:

| Página | Tipos estructurados que se deben evaluar |
|---|---|
| Negocio local | `LocalBusiness` y subtipos válidos |
| Peluquería | `HairSalon` |
| Salón de belleza | `BeautySalon`, cuando corresponda |
| Producto | `Product` |
| Servicio | `Service` |
| Artículo | `Article` |
| Navegación | `BreadcrumbList` |
| Organización ENSAMBRIA | `Organization` |
| Sitio web | `WebSite` |

Seleccionar el tipo que represente correctamente la entidad. No utilizar un tipo sólo porque parezca favorecer el posicionamiento.

## 9.2 Ejemplo conceptual JSON-LD

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "HairSalon",
  "name": "Studio Hair Medellín",
  "url": "https://studiohair.ENSAMBRIA.com/",
  "description": "Servicios profesionales de peluquería y cuidado capilar.",
  "image": "https://cdn.ENSAMBRIA.com/public/studio-hair.jpg",
  "telephone": "+57XXXXXXXXXX",
  "address": {
    "@type": "PostalAddress",
    "addressLocality": "Medellín",
    "addressRegion": "Antioquia",
    "addressCountry": "CO"
  }
}
</script>
```

Los datos del ejemplo deben sustituirse por datos reales y autorizados.

No generar números de teléfono, direcciones, calificaciones, precios ni horarios inventados.

## 9.3 Reglas para JSON-LD

- Debe corresponder al contenido visible de la página.
- No declarar reseñas inexistentes.
- No inventar puntuaciones.
- No declarar horarios incorrectos.
- No publicar datos personales no autorizados.
- No duplicar indiscriminadamente entidades.
- Generar valores mediante serialización segura.
- No construir JSON-LD mediante concatenación insegura de strings.
- Validar el tipo y las propiedades utilizadas.
- Verificar periódicamente los datos estructurados con herramientas de validación.

La presencia de datos estructurados no garantiza resultados enriquecidos ni una posición específica en buscadores.

---

# 10. Sitemap XML

ENSAMBRIA debe generar sitemaps para las páginas públicas que puedan ser indexadas.

El sitemap debe incluir únicamente URLs públicas, válidas y canónicas.

Ejemplos:

- Página principal del establecimiento.
- Perfiles públicos.
- Páginas de servicios.
- Páginas de productos publicados.
- Sucursales públicas.
- Páginas de discovery.
- Páginas públicas del marketplace.
- Artículos públicos, cuando existan.

## 10.1 Reglas

- No incluir rutas autenticadas.
- No incluir paneles administrativos.
- No incluir URLs de recuperación de contraseña.
- No incluir URLs privadas de reservas.
- No incluir resultados de búsqueda internos ilimitados.
- No incluir páginas eliminadas.
- No incluir páginas con `noindex`.
- Evitar duplicados.
- Utilizar URLs canónicas.
- Actualizar el sitemap cuando cambie la publicación de recursos.
- Dividir los sitemaps cuando se alcancen los límites aplicables.
- Utilizar índices de sitemap si el volumen lo requiere.
- No generar sitemaps mediante consultas completas sin paginación.

La generación debe ser eficiente y no provocar carga excesiva sobre PostgreSQL.

---

# 11. Robots.txt y control de indexación

Cada experiencia pública debe tener una política de rastreo adecuada.

Ejemplo conceptual:

```text
User-agent: *
Allow: /

Sitemap: https://studiohair.ENSAMBRIA.com/sitemap.xml
```

No utilizar este ejemplo de manera universal sin revisar las rutas de cada frontend.

## 11.1 Páginas que no deben indexarse

Como mínimo, evaluar:

- Paneles administrativos.
- Dashboards autenticados.
- Páginas privadas de clientes.
- Historiales de reservas.
- Detalles privados de transacciones.
- Páginas de pago internas.
- Pantallas de configuración.
- Rutas de autenticación y recuperación.
- Resultados internos de operaciones.
- Recursos que contengan datos sensibles.

La protección no debe depender de robots.txt ni de `noindex`.

Una página privada debe exigir autorización en el servidor. `noindex` únicamente controla indicaciones de indexación; no protege información.

Las respuestas privadas deben utilizar políticas de caché apropiadas y no almacenarse en cachés públicas.

---

# 12. Caché e invalidación de metadatos

Los metadatos públicos deben poder almacenarse en caché para mejorar rendimiento.

Sin embargo, los cambios del negocio deben reflejarse correctamente.

## 12.1 Eventos que deben provocar actualización

Evaluar la invalidación de caché cuando se produzcan cambios en:

- Nombre comercial.
- Descripción.
- Imagen principal.
- Imagen de Open Graph.
- Dominio personalizado.
- Servicios publicados.
- Productos publicados.
- Profesionales públicos.
- Sucursales.
- Horarios.
- Promociones.
- Estado de publicación.
- Estado público del negocio.

## 12.2 Reglas

- Definir políticas de caché por tipo de página.
- Evitar caché compartida incorrectamente entre tenants.
- Incluir la identidad del tenant y el dominio autorizado en la resolución de caché cuando corresponda.
- Invalidar las páginas afectadas tras cambios.
- Utilizar eventos de dominio o mecanismos de invalidación autorizados.
- Mantener un procedimiento de purga manual para incidentes.
- Evitar almacenar información privada en cachés públicas.
- Definir TTL de imágenes y HTML conforme a su estrategia de actualización.
- No prometer actualización inmediata de previews almacenadas por plataformas externas.

La invalidación debe ser consistente con las fronteras de ownership de ENSAMBRIA.

---

# 13. Renderizado del lado del servidor

Las páginas públicas deben entregar los metadatos en el HTML inicial.

Dado que ENSAMBRIA utiliza Next.js, se debe utilizar la estrategia de metadata del framework, Server Components y renderizado del servidor o generación estática cuando corresponda.

## 13.1 Reglas

- Generar metadatos en servidor.
- Obtener datos desde contratos autorizados.
- Evitar llamadas innecesarias al navegador para construir metadatos.
- No depender de `useEffect` para insertar las etiquetas.
- No generar vistas previas basadas exclusivamente en estado del cliente.
- No incluir información privada en el HTML inicial.
- Controlar caché y revalidación de páginas dinámicas.
- Gestionar correctamente errores y recursos no encontrados.

Las implementaciones deben utilizar las APIs compatibles con la versión de Next.js fijada en el repositorio.

## 13.2 Ejemplo conceptual con Next.js

```typescript
import type { Metadata } from "next";

export async function generateMetadata(): Promise<Metadata> {
  const business = await getPublicBusiness();

  if (!business) {
    return {
      title: "Establecimiento no disponible",
      robots: {
        index: false,
        follow: false,
      },
    };
  }

  const canonicalUrl = business.canonicalUrl;

  return {
    title: `${business.name} | ENSAMBRIA`,
    description: business.publicDescription,
    alternates: {
      canonical: canonicalUrl,
    },
    openGraph: {
      type: "website",
      title: business.name,
      description: business.publicDescription,
      url: canonicalUrl,
      siteName: business.name,
      images: [
        {
          url: business.ogImageUrl,
          width: 1200,
          height: 630,
          alt: business.ogImageAlt,
        },
      ],
      locale: "es_CO",
    },
    twitter: {
      card: "summary_large_image",
      title: business.name,
      description: business.publicDescription,
      images: [business.ogImageUrl],
    },
  };
}
```

Este ejemplo es orientativo. `getPublicBusiness()` representa un acceso real mediante el contrato autorizado correspondiente.

No debe implementarse como una llamada ficticia ni sustituir el acceso real a los datos.

---

# 14. Internacionalización

ENSAMBRIA debe contemplar negocios y usuarios de distintos países e idiomas.

La estrategia de metadatos debe soportar:

- Idioma de cada página.
- Región y localización.
- Títulos traducidos.
- Descripciones traducidas.
- Locale correcto.
- URLs canónicas por versión lingüística.
- Relación entre versiones mediante `hreflang` cuando corresponda.
- Sitemap coherente con las versiones publicadas.

Ejemplos de locale:

- `es_CO`
- `es_MX`
- `en_US`

No asignar una traducción automática como contenido oficial sin validación.

No utilizar el idioma del navegador para generar URLs canónicas inestables.

---

# 15. Seguridad

Reglas obligatorias:

1. Los metadatos sólo pueden incluir información pública autorizada.
2. No exponer información de reservas, pagos o membresías privadas.
3. No incluir tokens, credenciales o identificadores sensibles.
4. Validar y escapar títulos y descripciones.
5. Serializar JSON-LD de forma segura.
6. Validar dominios personalizados.
7. No confiar en el encabezado `Host` sin validación.
8. No permitir que URLs de imagen arbitrarias se conviertan en solicitudes internas del servidor.
9. Validar el origen de las imágenes y proteger contra SSRF cuando se procesen URLs externas.
10. No publicar imágenes privadas por error.
11. No filtrar datos entre tenants mediante caché o generación de HTML.
12. Utilizar HTTPS.
13. Aplicar controles de acceso en backend; no depender de robots.txt.
14. Auditar cambios sensibles en configuración de dominios y publicación.

Los metadatos públicos no deben utilizarse para transportar información privada ni para autorizar operaciones.

---

# 16. Rendimiento

La generación de metadatos no debe provocar consultas innecesarias ni afectar las operaciones principales del negocio.

Reglas:

- Obtener únicamente los datos necesarios para construir metadatos.
- Evitar N+1.
- Utilizar proyecciones públicas apropiadas.
- Aplicar índices a las consultas de resolución de recursos.
- Utilizar caché de forma segura.
- Evitar consultar varios backends innecesariamente por cada renderizado.
- No bloquear el renderizado esperando sistemas externos que no sean necesarios.
- Mantener timeouts.
- Evitar generación de imágenes costosa en cada solicitud.
- Precalcular o cachear imágenes OG cuando sea apropiado.
- Medir latencia y tasa de errores.

PUBLIC puede ofrecer una proyección preparada para las páginas públicas, evitando composiciones innecesarias contra múltiples servicios en cada visita.

La optimización nunca debe comprometer la actualización de información crítica ni el aislamiento entre tenants.

---

# 17. Pruebas obligatorias

Las pruebas deben cubrir los metadatos generados, el contenido real y las fronteras de seguridad.

## 17.1 Pruebas funcionales

- Cada página pública genera un título correcto.
- Cada página pública genera una descripción correcta.
- Open Graph contiene todas las etiquetas obligatorias.
- Twitter Cards contiene los valores esperados.
- La URL canónica es absoluta y correcta.
- La imagen OG es válida y accesible.
- La imagen corresponde al contenido.
- El locale es correcto.
- Las páginas eliminadas no siguen apareciendo como publicadas.
- Los recursos no disponibles generan respuestas correctas.

## 17.2 Pruebas multi-tenant

- No existe filtración de metadatos entre tenants.
- Los dominios resuelven el tenant correcto.
- Los hosts no autorizados no pueden suplantar establecimientos.
- Las URLs canónicas corresponden al dominio verificado.
- Las cachés no mezclan datos entre establecimientos.
- Las imágenes corresponden al tenant autorizado.

## 17.3 Pruebas SEO

- Títulos únicos donde corresponda.
- Descripciones apropiadas.
- Canonical correcta.
- Sitemap sin rutas privadas.
- Robots.txt coherente.
- Páginas privadas no indexables.
- `noindex` aplicado cuando corresponde.
- JSON-LD válido.
- Datos estructurados consistentes con el contenido visible.
- Versiones multilingües correctamente relacionadas.

## 17.4 Pruebas de seguridad

- No existe XSS mediante títulos o descripciones.
- JSON-LD se serializa correctamente.
- No se exponen secretos ni datos personales.
- No se permite SSRF mediante recursos de imagen.
- No se pueden consultar datos públicos de otro tenant por manipulación del host.
- Las imágenes privadas no son accesibles públicamente.

## 17.5 Pruebas de renderizado

Las pruebas deben comprobar el HTML inicial recibido por un cliente HTTP, no sólo el DOM después de ejecutar JavaScript.

Validar:

- Etiquetas presentes en el HTML.
- Contenido correcto.
- Ausencia de metadatos duplicados o contradictorios.
- Respuestas HTTP apropiadas.
- Comportamiento de páginas no encontradas.
- Redirecciones canónicas.
- Comportamiento de caché y revalidación.

## 17.6 Pruebas operativas

- Cambio de imagen principal.
- Cambio de dominio.
- Cambio de nombre comercial.
- Despublicación de un establecimiento.
- Eliminación de un servicio.
- Actualización de una promoción.
- Invalidación de caché.
- Recuperación ante fallos de PUBLIC o del proveedor de imágenes.

---

# 18. CI y GitHub

La validación debe ejecutarse localmente y en GitHub Actions con los mismos criterios.

La CI debe contemplar:

- Formato.
- Lint.
- Typecheck.
- Pruebas unitarias.
- Pruebas de integración aplicables.
- Build de frontends públicos.
- Validación de metadatos.
- Validación de URLs canónicas.
- Validación de sitemap.
- Validación de JSON-LD.
- Pruebas de aislamiento multi-tenant.
- Pruebas de seguridad.
- Compatibilidad con las versiones fijadas de dependencias.

No crear scripts duplicados si el repositorio ya tiene mecanismos equivalentes.

No modificar workflows y scripts fuera del alcance autorizado.

No debilitar las validaciones para conseguir que CI pase.

Los errores introducidos por el cambio deben corregirse antes de cerrar el PR.

---

# 19. Criterios de aceptación

La implementación no se considera completa hasta cumplir los criterios aplicables:

- [ ] Open Graph implementado en rutas públicas de `marketplace`.
- [ ] Open Graph implementado en rutas públicas de `corporate`.
- [ ] Rutas autenticadas de `tenant-portal` y `platform-admin` excluidas de previews e indexación.
- [ ] Twitter Cards implementadas.
- [ ] Metadatos dinámicos por tipo de página.
- [ ] Títulos y descripciones únicos y relevantes.
- [ ] URLs canónicas absolutas y verificadas.
- [ ] Imágenes OG públicas, optimizadas y accesibles.
- [ ] Metadata disponible en el HTML inicial.
- [ ] JSON-LD implementado donde corresponda.
- [ ] Schema.org validado.
- [ ] Sitemap XML funcional.
- [ ] Robots.txt configurado correctamente.
- [ ] Páginas privadas protegidas contra indexación y acceso no autorizado.
- [ ] Resolución correcta de dominios personalizados.
- [ ] Aislamiento multi-tenant en generación de metadatos.
- [ ] Caché e invalidación implementadas.
- [ ] Internacionalización contemplada.
- [ ] No se exponen datos privados.
- [ ] No existe vulnerabilidad de SSRF al procesar imágenes externas.
- [ ] Pruebas de renderizado del HTML inicial.
- [ ] Pruebas de seguridad y aislamiento.
- [ ] Build y checks de CI satisfactorios.
- [ ] Documentación actualizada.
- [ ] Configuración y resultados verificables.

---

# 20. Instrucciones obligatorias para el agente

Antes de implementar:

1. Inspeccionar la estructura real de `marketplace`, `corporate` y rutas privadas de `tenant-portal`/`platform-admin`.
2. Verificar la versión de Next.js y las APIs de metadata compatibles.
3. Identificar cómo se resuelven tenants y dominios personalizados.
4. Revisar los contratos actuales entre PUBLIC y BUSINESS.
5. Identificar las fuentes autorizadas de imágenes públicas.
6. Revisar los mecanismos actuales de CDN y caché.
7. Identificar rutas públicas, privadas y administrativas.
8. Revisar internacionalización existente.
9. Identificar sitemaps, robots.txt o metadatos ya implementados.
10. Presentar un plan que evite duplicar mecanismos existentes.

Durante la implementación:

- No inventar datos públicos.
- No exponer datos privados.
- No asumir que un frontend puede consultar directamente cualquier backend.
- No duplicar lógica de negocio en el frontend.
- No utilizar metadatos estáticos para recursos que deben ser dinámicos.
- No depender exclusivamente de JavaScript cliente.
- No publicar imágenes privadas.
- No dejar URLs temporales para recursos que deben ser públicos.
- No ignorar los dominios personalizados.
- No introducir un sistema de caché que mezcle tenants.
- No declarar terminado SEO únicamente por añadir etiquetas HTML.
- No dejar mocks como sustituto de integraciones reales.
- No dejar TODOs en funcionalidades declaradas como implementadas.
- No rebajar pruebas, seguridad o lint.

Al finalizar:

1. Ejecutar los checks locales correspondientes.
2. Ejecutar las pruebas de frontends y contratos afectados.
3. Verificar el HTML inicial generado.
4. Verificar imágenes y URLs canónicas.
5. Validar sitemap y JSON-LD.
6. Ejecutar pruebas de aislamiento multi-tenant.
7. Corregir los fallos introducidos.
8. Actualizar documentación y roadmap únicamente con trabajo terminado.
9. Informar archivos modificados, pruebas ejecutadas y resultados.

---

# 21. Decisión arquitectónica final

ENSAMBRIA establece Open Graph, Twitter Cards, SEO técnico, Schema.org, sitemap y control de indexación como capacidades obligatorias de sus experiencias públicas.

La responsabilidad se distribuye de esta manera:

- **BUSINESS:** mantiene la información operativa y comercial que puede publicarse.
- **PUBLIC:** proporciona proyecciones y contratos públicos autorizados.
- **marketplace:** genera metadatos de páginas de negocios, perfiles y servicios publicados desde proyecciones autorizadas.
- **corporate:** genera metadatos del contenido institucional público propiedad de PLATFORM.
- **tenant-portal/platform-admin:** no exponen metadatos ni previews de información privada; `noindex` no sustituye autorización.
- **CORE:** conserva la autoridad sobre identidad, tenancy y controles de autorización.

La implementación debe ser dinámica, multi-tenant, segura, internacionalizable, observable y compatible con renderizado del servidor.

**Regla esencial:** cada enlace público de ENSAMBRIA debe poder compartirse y generar una vista previa profesional, correcta y coherente con la página, sin exponer información privada ni permitir filtraciones entre establecimientos.

La calidad de los metadatos, las URLs, las imágenes y los datos estructurados forma parte de la definición de terminado de los frontends públicos, no de una optimización opcional posterior.
