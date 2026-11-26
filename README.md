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
- [ADR-0013 — AWS para configuración, secretos y mTLS](docs/adr/ADR-0013-aws-configuration-secrets-mtls.md)
- [Decisiones arquitectónicas](docs/adr/)

El requerimiento, el contrato de arquitectura, los documentos de dominio, los ADR y las reglas aplicables definen el diseño y las obligaciones futuras. Su existencia no demuestra implementación ni aceptación de una fase.

Las decisiones arquitectónicas aprobadas conservan su estado hasta ser sustituidas mediante el proceso de decisión del proyecto. Una aprobación documental no equivale a implementación ni a aceptación de fase.

## Arquitectura objetivo

El diseño establece cuatro backends de dominio con ownership separado: CORE, BUSINESS, PLATFORM y PUBLIC. El repositorio ya contiene una implementación parcial de CORE/PLATFORM y pruebas de integración; eso no acredita que los cuatro backends, todos los frontends o las integraciones estén implementados o desplegados. Los límites objetivo y su estado real se mantienen en el [roadmap](ROADMAP.md), el [registro de Fase 0](docs/registro-fase-0-descubrimiento-arquitectura.md) y el [registro de Fase 1](docs/registro-fase-1-fundacion-core.md).

La arquitectura y responsabilidades completas están en [ARCHITECTURE_RULES.md](ARCHITECTURE_RULES.md) y [docs/arquitectura-dominios.md](docs/arquitectura-dominios.md). La secuencia de implementación está en [ROADMAP.md](ROADMAP.md).

## Estado de las fases

**Las fases se aceptan por separado y ninguna se considera cerrada por la mera existencia de código o CI configurado.** Fase 0 permanece abierta; Fase 1 contiene trabajo implementado y validado parcialmente, con aceptación de fase y verificación de staging pendientes. Consulta el [estado del roadmap](ROADMAP.md#estado-vigente-de-fases) y exige CI del commit exacto más aceptación explícita.

No hay instrucciones verificadas para instalar, ejecutar, compilar o desplegar el producto. No se deben inferir a partir de la arquitectura objetivo ni de registros históricos.

## Contribución y gobierno

Todo cambio debe seguir [AGENTS.md](AGENTS.md) y las [reglas de gobierno del repositorio](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md). La documentación de una política no demuestra que la correspondiente configuración de GitHub esté activa; el estado del repositorio remoto debe verificarse antes de depender de ella.

## Licencia

Consulta [LICENSE](LICENSE) para las condiciones de uso.
# ENSAMBRIA
