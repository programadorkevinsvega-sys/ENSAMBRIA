# BFF público

## Identificación y estado

- **Tipo:** BFF independiente para Discovery y superficies públicas autorizadas.
- **Ubicación:** `bff-public/`.
- **Estado:** estructura de directorios; no hay implementación funcional.

## Propósito y límites

Adaptar REST/OpenAPI y, cuando corresponda al contrato aprobado, WebSocket hacia PUBLIC por gRPC. Es una aplicación con proceso y despliegue propios; no constituye un quinto backend de dominio.

No posee datos ni reglas de dominio, no modifica pedidos o inventario y no accede directamente a PostgreSQL, Redis o NATS. PUBLIC conserva la autoridad sobre las proyecciones públicas autorizadas.

## Estructura prevista

- `src/handlers/` y `src/dto/`: adaptación de solicitudes y respuestas.
- `src/grpc/`: clientes de contratos PUBLIC.
- `src/openapi/`: documentación o artefactos OpenAPI cuando existan.
- `src/websocket/`: superficie realtime si el contrato y la implementación la requieren.
- `tests/` y `deployment/`: pruebas y artefactos del componente cuando se implementen.

## Seguridad y aceptación futura

Debe validar el contrato de transporte, propagar contexto y correlación y dejar autorización y publicación al owner correspondiente. Las rutas privadas y borradores no se exponen como contenido público.

No hay endpoints ni pruebas implementados; no se afirma integración o SEO validado.

## Ejecución y pruebas

No existe manifiesto ni configuración ejecutable. No se documentan comandos de instalación, ejecución o pruebas.

## Fuentes

- [Modelo de dominios](../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../docs/requerimiento.md)
- [Reglas de arquitectura](../ARCHITECTURE_RULES.md)
