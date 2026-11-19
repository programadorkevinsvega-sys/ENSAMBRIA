# ENSAMBRIA

ENSAMBRIA es una plataforma tecnológica en etapa de diseño documental para construir productos SaaS multiproducto y multindustria. El producto inicial previsto se orienta a belleza, estética y bienestar.

> **Estado del repositorio:** reinicio de la implementación. El código de producto y sus artefactos ejecutables anteriores fueron retirados. Actualmente este repositorio conserva documentación y la estructura restante. No se declara ninguna capacidad de producto implementada, validada ni disponible.

## Fuentes principales

- [Requerimientos y alcance del producto](docs/requerimiento.md)
- [Roadmap](ROADMAP.md)
- [Contrato arquitectónico](ARCHITECTURE_RULES.md)
- [Modelo de dominios y estructura objetivo](docs/arquitectura-dominios.md)
- [Instrucciones para agentes](AGENTS.md)
- [Reglas normativas](docs/rules/)
- [Decisiones arquitectónicas](docs/adr/)

El requerimiento, el contrato de arquitectura, los documentos de dominio, los ADR y las reglas aplicables definen el diseño y las obligaciones futuras. Su existencia no demuestra implementación ni aceptación de una fase.

Las decisiones arquitectónicas que fueron aprobadas conservan su estado hasta ser sustituidas mediante el proceso de decisión del proyecto. El reinicio de código no las revoca; tampoco convierte esa aprobación en implementación o aceptación de fase.

## Arquitectura objetivo

El diseño establece cuatro backends de dominio con ownership separado: CORE, BUSINESS, PLATFORM y PUBLIC. También define superficies de presentación y BFF autorizados. Estos son límites objetivo; no implican que los servicios, aplicaciones o integraciones existan en este reinicio.

La arquitectura y responsabilidades completas están en [ARCHITECTURE_RULES.md](ARCHITECTURE_RULES.md) y [docs/arquitectura-dominios.md](docs/arquitectura-dominios.md). La secuencia de implementación está en [ROADMAP.md](ROADMAP.md).

## Estado de las fases

**Todas las fases 0–11 están abiertas, no aceptadas y sin implementación de producto vigente.** Los registros de Fase 0, 1 y 2 contienen material de cortes anteriores; las afirmaciones sobre código, pruebas, migraciones o integraciones retiradas no son evidencia vigente. Consulta el [estado del roadmap](ROADMAP.md#estado-vigente-de-fases).

No hay instrucciones verificadas para instalar, ejecutar, compilar o desplegar el producto. No se deben inferir a partir de la arquitectura objetivo ni de registros históricos.

## Contribución y gobierno

Todo cambio debe seguir [AGENTS.md](AGENTS.md) y las [reglas de gobierno del repositorio](docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md). La documentación de una política no demuestra que la correspondiente configuración de GitHub esté activa; el estado del repositorio remoto debe verificarse antes de depender de ella.

## Licencia

Consulta [LICENSE](LICENSE) para las condiciones de uso.
