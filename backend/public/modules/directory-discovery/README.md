# Directorio y Discovery

## Identificación y estado

- **Tipo:** bounded context de PUBLIC.
- **Ubicación:** `backend/public/modules/directory-discovery/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** directorio y Discovery de Marketplace en Fase 9; SEO y metadata se implementan desde Fase 1 para cada ruta pública introducida.

## Propósito y responsabilidad

Ayudar a visitantes a encontrar establecimientos y ofertas que hayan sido publicadas. PUBLIC es dueño del índice/listado y metadatos de búsqueda derivados de proyecciones autorizadas; no vende ni posee catálogo operativo.

## Operaciones previstas

Buscar, filtrar, listar y consultar fichas publicadas con paginación/cursor estable. Las rutas públicas deben representar solo el estado publicado vigente.

## Límites e integraciones

- Consume proyecciones autorizadas; BUSINESS conserva el contenido fuente.
- La superficie externa se ofrece mediante el BFF público independiente.
- No consulta CRM, inventario, precios privados ni bases de datos tenant directamente.
- Que una ficha sea indexable no concede autorización.

## Seguridad e integridad

La retirada no debe restaurarse por una proyección antigua; la actualización y repetición deben ser idempotentes. Se excluyen borradores y datos privados de HTML y sitemap.

La aceptación futura cubre contenido no publicado/retirado, límites de filtros, aislamiento tenant, huecos de indexación, paginación, estado obsoleto y exclusión de borradores.

## Implementación y pruebas

Solo existe la estructura de capas. No hay índice, contrato ejecutable ni rutas públicas implementadas; no se declara SEO validado.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
