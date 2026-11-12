# Bibliotecas técnicas compartidas

## Identificación y estado

- **Tipo:** ubicación prevista para crates Rust neutrales y reutilizables.
- **Ubicación:** `crates/`.
- **Estado:** estructura organizativa; no se declaran crates funcionales implementados.

## Propósito y límites

Contener bibliotecas técnicas compartidas solo cuando tengan uso real, dependencia explícita y responsabilidad neutral. La composición Cargo pertenece al workspace único de la raíz.

Estas bibliotecas pueden proporcionar mecanismos técnicos, pero no poseen reglas de negocio, entidades, tablas, migraciones ni casos de uso de dominio. Los módulos de dominio no se convierten automáticamente en crates.

## Áreas previstas

- `infrastructure/`: adaptadores técnicos para PostgreSQL, Redis, mensajería, gRPC, almacenamiento y seguridad.
- `observability/`: primitivas neutrales de observabilidad.
- `shared/`: solo primitivas realmente universales y sin ownership de dominio.

La existencia de carpetas no significa que haya crates, manifests o APIs publicados.

## Build y pruebas

No hay manifiestos de crate ni comandos específicos en esta carpeta. No se puede declarar compilación o validación de estas bibliotecas.

## Fuentes

- [Reglas de arquitectura](../ARCHITECTURE_RULES.md)
- [ADR-0003: workspace Cargo](../docs/adr/ADR-0003-cargo-workspace.md)
- [Modelo de dominios](../docs/arquitectura-dominios.md)
