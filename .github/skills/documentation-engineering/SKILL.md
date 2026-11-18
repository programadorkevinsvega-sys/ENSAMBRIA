---
name: documentation-engineering
description: "Use when creating or updating README files, architecture or product documentation, contracts, operational guides, standards, Markdown links, or documentation status claims."
---

# Documentation Engineering

## Objetivo

Mantener documentación veraz, segura, localizable y sincronizada con el estado del repositorio, usando la fuente normativa correspondiente.

## Alcance

README, documentación de arquitectura/producto, contratos, ADR, guías técnicas y operativas, referencias, estado de implementación y validación documental.

## Cuándo activarla

- Obligatoria al editar README, reglas, contratos, guías operativas o documentación de una funcionalidad.
- Complementaria a la Skill de arquitectura cuando los documentos cambien ownership, decisiones o gobierno.

## Referencias normativas obligatorias

- [README Documentation Quality and Security Standard](../../../docs/rules/ENSAMBRIA_README_Documentation_Quality_Security_Standard.md)
- [Estándar de gobierno arquitectónico y documental](../../../docs/rules/ENSAMBRIA_architecture_doc_engineering_governance_standard.md)
- [Estándar de gobierno del repositorio GitHub](../../../docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md), para revisión en PR, validaciones y trazabilidad del cambio documental.
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), solo para describir alcance y estado del producto.

## Aplicación obligatoria en ENSAMBRIA

Antes de editar documentación:

1. Confirma audiencia, owner, fase y alcance; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), regla documental/estándar propietario, requisitos de fase y contratos/ADR relacionados.
2. Aplica la precedencia de AGENTS.md y verifica la condición de aprobación de cada fuente. No conviertas propuestas, borradores legales, planes o evidencia no ejecutada en decisiones, obligaciones legales aprobadas o runtime implementado.
3. Actualiza la fuente propietaria con trazabilidad requisito→decisión→componente→prueba→CI/evidencia; enlaza fuentes especializadas en vez de duplicarlas y valida rutas/comandos/claims contra el estado real.
4. Reporta cambios, enlaces/comprobaciones exactas, resultado y decisiones pendientes. Distingue **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO**.

## Procedimiento operativo

1. Determina audiencia, propósito y documento propietario; busca si el contenido ya existe antes de crear otro archivo.
2. Contrasta cada afirmación con código, manifests, contratos, decisiones y evidencia disponibles.
3. Ubica el detalle en la fuente dueña y enlaza estándares especializados; evita duplicar requisitos o mezclar roadmap con contratos.
4. Distingue lo implementado, parcial, planificado, bloqueado, no ejecutado o no aplicable.
5. Verifica rutas, enlaces, comandos y afirmaciones técnicas; protege secretos, datos personales y detalles sensibles.

## Restricciones

- No inventes capacidad, comando, resultado de prueba, autorización, versión o estado de lanzamiento.
- No uses `docs/requerimiento.md` como estándar universal ni conviertas propuestas del roadmap en funcionalidad.
- No agregues documentos vacíos, redundantes ni no requeridos; no reproduzcas completas las fuentes normativas.
- No alteres un estándar fuente para hacer coincidir la documentación sin aprobación del cambio correspondiente.

## Validaciones obligatorias

- Valida todos los enlaces locales del alcance y comprueba existencia de comandos/rutas referenciados.
- Revisa consistencia entre código, contratos, estados y documentación afectada; ejecuta las comprobaciones documentales existentes.
- Comprueba redacción de secretos/datos personales y que las diferencias se revisen en PR.

## Criterios de finalización

La información tiene dueño y audiencia claros, refleja el estado comprobado, enlaza fuentes válidas, no duplica normativa y tiene validación documentada.

## Conflictos y excepciones

Si fuentes o implementación divergen, no edites para ocultar el conflicto. Registra la diferencia, limita afirmaciones a lo comprobado y solicita decisión al owner normativo.

## Mantenimiento

Revisada el 2026-11-10. Cuando se actualice una fuente citada, revisa esta Skill en el mismo PR. Usa versiones/fechas existentes y el historial de PR; no inventes metadatos.
