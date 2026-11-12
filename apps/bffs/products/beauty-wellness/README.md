# BFF del producto Beauty & Wellness

## Identificación y estado

- **Tipo:** BFF independiente del producto Beauty & Wellness.
- **Ubicación:** `apps/bffs/products/beauty-wellness/`.
- **Estado:** estructura de directorios; no hay implementación funcional.

## Propósito y límites

Adaptar las solicitudes HTTP del producto a los contratos autorizados de los backends propietarios. Este BFF es una aplicación independiente con configuración, proceso y despliegue propios cuando se implemente.

No posee reglas ni datos de dominio, no sustituye la autorización de los backends y no accede directamente a PostgreSQL, Redis o NATS. No se crea un BFF por tenant o por establecimiento.

## Estructura prevista

- `src/api/`, `src/routes/` y `src/dto/`: superficie de transporte y sus modelos.
- `src/clients/`: clientes de contratos autorizados.
- `src/middleware/`: adaptación de contexto y controles de transporte.
- `src/observability/`: integración de observabilidad del proceso.
- `src/composition/`: composición del BFF.

Estos directorios son organizativos; no implican código, endpoints ni contratos existentes.

## Integraciones y seguridad

Consume únicamente las superficies autorizadas para el producto, propaga identidad y correlación y deja la decisión de acceso al backend propietario. No se documentan protocolos concretos de cada operación hasta que existan contratos aprobados.

## Ejecución y pruebas

No existe aquí un manifiesto, configuración ejecutable ni suite de pruebas. Por ello no se proporcionan comandos de instalación, ejecución o validación.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
