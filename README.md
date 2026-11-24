# ENSAMBRIA

ENSAMBRIA es una plataforma SaaS multiproducto y multindustria en desarrollo. El producto inicial previsto se orienta a belleza, estética y bienestar.

> **Estado del repositorio:** contiene crates Rust, migraciones PostgreSQL, procesos CORE/PLATFORM y workflows de integración. La presencia de código no implica aceptación de fase ni disponibilidad de producción; consulta el estado vigente y la evidencia por SHA en [ROADMAP.md](ROADMAP.md) y en los registros de fase.

## Fuentes principales

- [Requerimientos y alcance del producto](docs/requerimiento.md)
- [Roadmap](ROADMAP.md)
- [Contrato arquitectónico](ARCHITECTURE_RULES.md)
- [Modelo de dominios y estructura objetivo](docs/arquitectura-dominios.md)
- [Instrucciones para agentes](AGENTS.md)
- [Reglas normativas](docs/rules/)
- [Fronteras modulares y cumplimiento arquitectónico obligatorio](docs/rules/ENSAMBRIA_Modular_Boundaries_and_Architecture_Compliance.md)
- [ADR-0012 — Fronteras modulares ejecutables](docs/adr/ADR-0012-fronteras-modulares-y-cumplimiento-arquitectonico.md)
- [Decisiones arquitectónicas](docs/adr/)

El requerimiento, el contrato de arquitectura, los documentos de dominio, los ADR y las reglas aplicables definen el diseño y las obligaciones futuras. Su existencia no demuestra implementación ni aceptación de una fase.

Las decisiones arquitectónicas que fueron aprobadas conservan su estado hasta ser sustituidas mediante el proceso de decisión del proyecto. El reinicio de código no las revoca; tampoco convierte esa aprobación en implementación o aceptación de fase.

## Arquitectura objetivo

El diseño establece cuatro backends de dominio con ownership separado: CORE, BUSINESS, PLATFORM y PUBLIC. También define superficies de presentación y BFF autorizados. Estos son límites objetivo; no implican que los servicios, aplicaciones o integraciones existan en este reinicio.

La arquitectura y responsabilidades completas están en [ARCHITECTURE_RULES.md](ARCHITECTURE_RULES.md) y [docs/arquitectura-dominios.md](docs/arquitectura-dominios.md). La secuencia de implementación está en [ROADMAP.md](ROADMAP.md).

## Estado de las fases

**Las fases se aceptan por separado y ninguna se considera cerrada por la mera existencia de código o CI configurado.** La Fase 1 tiene trabajo implementado en el repositorio; la subfase 1.3 tiene implementación y resiliencia CORE→PLATFORM validadas por CI #400; su aceptación formal sigue pendiente. Consulta el [estado del roadmap](ROADMAP.md#estado-vigente-de-fases) y exige CI del commit exacto más aceptación explícita.

No hay instrucciones verificadas para instalar, ejecutar, compilar o desplegar el producto. No se deben inferir a partir de la arquitectura objetivo ni de registros históricos.

## Contribución y gobierno

Todo cambio debe seguir [AGENTS.md](AGENTS.md) y las [reglas de gobierno del repositorio](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md). La documentación de una política no demuestra que la correspondiente configuración de GitHub esté activa; el estado del repositorio remoto debe verificarse antes de depender de ella.

## Licencia

Consulta [LICENSE](LICENSE) para las condiciones de uso.
# ENSAMBRIA
