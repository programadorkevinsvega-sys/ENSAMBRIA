---
name: rust-quality-engineering
description: "Use when implementing, testing, reviewing, or validating Rust code, including error handling, dependencies, security, performance, CI, and release readiness."
---

# Rust Quality Engineering

## Objetivo

Organizar una validación de ingeniería basada en el repositorio y sus requisitos reales, con resultados reproducibles y sin duplicar el estándar de calidad.

## Alcance

Inspección de cambios Rust, diseño de pruebas, errores, seguridad, concurrencia, rendimiento, dependencias, CI y evidencia de aceptación.

## Cuándo activarla

- Obligatoria al añadir o cambiar código, pruebas, dependencias o workflows Rust.
- Consultiva para auditorías de calidad, incidentes, preparación de release o análisis de fallos.

## Referencias normativas obligatorias

- [ADR-0012 — enforceable modular boundaries](../../../docs/adr/ADR-0012-fronteras-modulares-y-cumplimiento-arquitectonico.md)
- [Modular boundaries and architecture compliance](../../../docs/rules/ENSAMBRIA_Modular_Boundaries_and_Architecture_Compliance.md)

- [Prompt de calidad empresarial para Rust](../../../docs/rules/ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md)
- [k6 performance, load, stress and capacity testing](../../../docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md) para diseño, ejecución segura y evidencia de pruebas de rendimiento.
- [Estándar de gobierno del repositorio GitHub](../../../docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md), para protección de ramas, PR, CI, seguridad de workflows y evidencia de integración.
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Reglas técnicas Rust de monolito modular](../../../docs/rules/ENSAMBRIA_reglas-estrictas-de-arquitectura-monolito-modular.md), solo para detalle de implementación Rust cuando el cambio afecte arquitectura; el contrato central rige topología y ownership.
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), para requisitos y aceptación del producto afectados.

## Aplicación obligatoria en ENSAMBRIA

Antes de ejecutar el procedimiento:

1. Confirma fase, alcance, owner y aceptación; lee [AGENTS.md](../../../AGENTS.md), el contrato central [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), criterios del producto/fase, las secciones completas de reglas aplicables y contratos/ADR relacionados. Consulta únicamente las secciones pertinentes de la guía estricta para detalle Rust cuando corresponda; verifica el estado de cada decisión y no evalúes un cambio contra una propuesta como si estuviera ratificada.
2. Deriva la matriz de escenarios de los requisitos y fronteras reales: positivos, negativos, límites, autorización/tenant, duplicados/concurrencia, fallos y ausencia de efectos parciales cuando apliquen.
3. Asocia cada requisito con prueba real, comando local/CI, resultado y evidencia. Distingue prueba escrita de ejecutada, mocks permitidos de dependencias reales requeridas y fallos/bloqueos de no ejecutado.
4. No declares aceptación por cobertura, compilación, mocks o artefactos documentales aislados. Reporta **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO** por separado y solicita resolución de cualquier conflicto normativo.

## Procedimiento operativo

1. Inspecciona el diff, el estado real del código, los manifests, herramientas, tests y workflows; separa implementado de planificado.
2. Traduce requisitos a actores, precondiciones, invariantes, resultados, particiones de prueba y criterios de aceptación.
3. Selecciona pruebas unitarias, integración, contrato y E2E según las fronteras afectadas; usa dependencias reales donde la aceptación lo exige.
4. Ejecuta los comandos documentados y aplicables al repo, registrando comando, alcance y resultado. Compara los gates locales con CI cuando exista.
5. Revisa el resultado y comunica estados como aprobado, fallido, bloqueado, no ejecutado o no aplicable, con evidencia.

## Restricciones

- No inventes scripts, comandos, cobertura, resultados, integraciones o checks configurados.
- No uses mocks como sustituto de aceptación con dependencias reales; limita su uso a pruebas aisladas permitidas por la normativa.
- No silencies errores o advertencias, ni relajes tests/gates para producir un resultado verde.
- No generalices fases, stack ni ownership específicos de ENSAMBRIA a otros proyectos.

## Validaciones obligatorias

- Ejecuta formato, compilación, lint y pruebas relevantes según manifests y workflows existentes; declara los no ejecutados.
- Comprueba escenarios positivos, negativos, límites, autorización, duplicados/concurrencia, fallos y ausencia de efectos parciales cuando apliquen.
- Para fallos, valida errores tipados, recuperación y observabilidad/redacción de acuerdo con los estándares especializados.

## Criterios de finalización

Los cambios cumplen criterios observables, los checks pertinentes se ejecutaron sin supresiones introducidas y el informe distingue evidencia, fallos y bloqueos.

## Conflictos y excepciones

Ante criterios incompatibles entre el requisito, arquitectura, prompts, código o CI, detén solo las verificaciones afectadas, identifica las fuentes y solicita resolución. N/A requiere una justificación concreta y previa cuando así lo exige la fuente.

## Mantenimiento

Revisada el 2026-11-24. Cuando cambie el prompt o el contrato citado, revisa la Skill y ejecuta el validador del repositorio. No inventes versiones.
