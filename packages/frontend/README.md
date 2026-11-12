# Paquetes compartidos de frontend

## Identificación y estado

- **Tipo:** paquetes técnicos compartidos para aplicaciones frontend.
- **Ubicación:** `packages/frontend/`.
- **Estado:** estructura de directorios; no hay paquetes implementados.

## Propósito y límites

Agrupar componentes visuales y utilidades frontend comunes únicamente cuando exista un uso real en más de una aplicación. Las aplicaciones conservan su navegación, composición, estado y despliegues independientes.

Los paquetes compartidos no contienen reglas de negocio, autorización de backend ni acceso directo a PostgreSQL, Redis, NATS o proveedores.

## Áreas previstas

- `design-system/`: tokens y convenciones visuales compartidas.
- `ui/`: componentes reutilizables.
- `accessibility/`: utilidades o validaciones accesibles.
- `api-clients/`: clientes tipados de contratos autorizados.
- `localization/`: recursos y mecanismos comunes de localización.
- `tooling/`: herramientas de frontend compartidas.

La existencia de estas carpetas no afirma que haya componentes, clientes API, herramientas o recursos de idioma implementados.

## Tecnologías y validación

Los frontends web usan React, TypeScript y Next.js; las aplicaciones Business y App usan Flutter y Dart. No se define aquí una tecnología de empaquetado compartido ni comandos porque no hay manifests o paquetes ejecutables.

## Fuentes

- [Modelo de dominios](../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../docs/requerimiento.md)
- [Reglas de arquitectura](../../ARCHITECTURE_RULES.md)
