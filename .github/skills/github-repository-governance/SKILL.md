---
name: github-repository-governance
description: "Use when changing or applying GitHub repository governance, branch protection, branches, commits, Pull Requests, reviews, CODEOWNERS, Actions, CI policy, dependency governance, releases, or repository security."
---

# GitHub Repository Governance

## Objetivo

Aplicar de forma verificable las políticas de colaboración, calidad, seguridad y trazabilidad del repositorio, sin declarar controles configurados que no hayan sido comprobados.

## Alcance

Protección de ramas, estrategia de ramas y commits, Pull Requests, revisiones, GitHub Actions, CI local/remota, seguridad del repositorio, dependencias, CODEOWNERS, releases, cambios de base de datos, excepciones y trabajo de agentes de IA.

## Cuándo activarla

- Obligatoria al crear, revisar o cambiar políticas, configuración, permisos o controles de un repositorio GitHub.
- Obligatoria al preparar o evaluar un cambio sujeto a reglas de rama, PR, revisión, CI, CODEOWNERS o release.
- Complementaria al modificar workflows, herramientas de CI, dependencias o documentación de gobierno.

## Referencias normativas obligatorias

- [ADR-0012 — enforceable modular boundaries](../../../docs/adr/ADR-0012-fronteras-modulares-y-cumplimiento-arquitectonico.md)
- [Modular boundaries and architecture compliance](../../../docs/rules/ENSAMBRIA_Modular_Boundaries_and_Architecture_Compliance.md)

- [GitHub Repository Governance and Engineering Rules](../../../docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md)
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), solo para alcance y fase específicos del producto.

## Aplicación obligatoria en ENSAMBRIA

Antes de cambiar o declarar efectivo un control:

1. Confirma fase, autoridad, repositorio/branch y alcance; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), la regla GitHub completa, aceptación de fase y contratos/ADR que afecten el control, verificando el estado de cada decisión.
2. Inspecciona la configuración efectiva (workflows, permisos, branch rules, CODEOWNERS, plantillas, scripts y checks); distingue requisito, archivo presente, configuración remota activa y evidencia del PR/commit exacto.
3. Traduce cada regla aplicable a mecanismo efectivo, mínimo privilegio, validación local equivalente, check de CI y evidencia. No agregues gates placebo ni afirmes Rulesets, revisión, CI o autoaprobación sin comprobación.
4. Reporta archivo/configuración, autoridad y estado, comandos/resultados, riesgos y bloqueos; separa **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO**.

## Procedimiento operativo

1. Inspecciona la rama, el estado del worktree, los archivos de política y configuración existentes, los workflows, scripts, CODEOWNERS y plantillas; no presupongas que una configuración existe o está activa.
2. Identifica el cambio, propietario, requisitos, actores de revisión, permisos disponibles, riesgos, checks aplicables y evidencia de aceptación. Mantén separados los requisitos generales del estándar y las decisiones específicas de ENSAMBRIA.
3. Para cambios de código o documentación, usa una rama descriptiva por unidad coherente y prepara un PR con alcance, criterios de aceptación, riesgos y validaciones reales. No mezcles objetivos ni afirmes aprobaciones humanas.
4. Si el repositorio tiene una sola persona mantenedora, aplica la auto-revisión documentada permitida por §8.1 del estándar; no la presentes como aprobación independiente. Detén los cambios que requieran revisión independiente cuando no haya revisor autorizado.
5. Antes de cambiar workflows o controles, comprueba los comandos, manifests y herramientas reales. Conserva mínimo privilegio, acciones externas fijadas según la política, límites de ejecución y separación entre CI y despliegue; no agregues gates placebo ni ocultes fallos.
6. Ejecuta las validaciones locales que correspondan con los mismos comandos, configuración y criterios que CI. Verifica los resultados remotos del PR o commit exacto solo cuando estén disponibles y se hayan consultado.
7. Revisa el diff, secretos, trazabilidad y documentación; comunica permisos faltantes, controles no configurados y validaciones bloqueadas como tales.

## Restricciones

- No trabajes directamente en `main`, no hagas push directo, force-push, merge, release o despliegue sin autorización y condiciones explícitas aplicables.
- No crees ni afirmes Rulesets, Branch Protection, CODEOWNERS, workflows, checks, aprobaciones o resultados de CI sin evidencia verificable.
- No deshabilites pruebas, checks o seguridad para lograr un resultado verde; no ejecutes comandos productivos ni expongas secretos.
- No cambies workflows fuera del alcance autorizado ni uses permisos GitHub superiores a los necesarios.
- No simules revisión independiente ni aprobación humana; un agente que implementó un cambio no se presenta automáticamente como revisor independiente.

## Validaciones obligatorias

- Comprueba estado de rama/worktree y revisa `git diff` y `git diff --check` antes de cerrar.
- Verifica que referencias, plantillas, scripts y comandos citados existan; distingue ausencia de archivo de protección efectiva configurada en GitHub.
- Ejecuta las validaciones aplicables y registra comando, resultado y alcance; identifica explícitamente checks no ejecutados, fallidos o bloqueados.
- Para integración, confirma los checks requeridos, revisiones y conversaciones del PR, y el estado del commit exacto de destino si esos datos están disponibles.

## Criterios de finalización

El cambio tiene alcance y evidencia claros, las políticas y documentación permanecen coherentes, se han ejecutado las validaciones aplicables y no se reporta como activo ningún control no verificado. Los bloqueos de permisos, configuración o aprobación quedan explícitos.

## Conflictos y excepciones

Si una regla general contradice una decisión de arquitectura o una política específica de ENSAMBRIA, identifica las fuentes y el impacto, detén solo el alcance afectado y solicita resolución al responsable. Las excepciones deben seguir el procedimiento de justificación, aprobación, mitigación, vigencia y revisión de la fuente normativa; no las concedas implícitamente.

## Mantenimiento

Revisada el 2026-11-24. Cuando cambie el estándar de gobierno GitHub o las obligaciones aplicables de ENSAMBRIA, revisa esta Skill y su registro en el mismo cambio. Conserva únicamente fechas y decisiones verificables.
