---
name: enterprise-architecture-governance
description: "Use when defining or changing architecture, ownership, ADRs, C4 views, contracts, governance, traceability, documentation standards, quality gates, exceptions, or phase acceptance."
---

# Enterprise Architecture Governance

## Objetivo

Guiar decisiones de arquitectura y gobierno con trazabilidad, responsabilidades claras y evidencia, sin convertir esta Skill en otra fuente normativa.

## Alcance

Decisiones arquitectónicas, límites y ownership, contratos, ADR, capacidades transversales (multi-tenant, multi-sucursal, mercados/monedas/idiomas, reglas, eventos, auditoría, API, webhooks y extensiones), documentación técnica, cambios de estándares, excepciones, quality gates y aceptación de fases.

## Cuándo activarla

- Obligatoria al definir o cambiar límites de sistemas/módulos, ownership, contratos, ADR/C4, fases, excepciones o gobierno documental.
- Obligatoria desde Fase 0 al definir, revisar o implementar cualquiera de las diez capacidades transversales enumeradas en la sección 14 de `docs/requerimiento.md`.
- Consultiva al implementar un cambio que pueda alterar esas decisiones, aunque la tarea principal pertenezca a otra especialidad.

## Referencias normativas obligatorias

Leer completo el contrato central y los criterios del producto aplicables; en estándares extensos, leer íntegramente las secciones pertinentes antes de decidir:

- [Estándar de gobierno arquitectónico y documental](../../../docs/rules/ENSAMBRIA_architecture_doc_engineering_governance_standard.md)
- [Estándar de gobierno del repositorio GitHub](../../../docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md), cuando el alcance incluya políticas, PR, revisión o CI del repositorio.
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), solo para alcance y fase del producto.

## Aplicación obligatoria en ENSAMBRIA

Antes de ejecutar el procedimiento:

1. Confirma la fase, alcance autorizado, owner, requisitos y criterios de aceptación; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), la sección de producto/fase aplicable, las reglas pertinentes y los contratos/ADR relacionados. Verifica si cada ADR está propuesto o ratificado.
2. Aplica la precedencia de AGENTS.md. Una Skill, resumen o instrucción aislada no modifica una fuente normativa; eleva contradicciones y decisiones sin aprobación antes de escoger una solución.
3. Convierte cada regla aplicable en requisito trazable, artefacto/owner, implementación autorizada, prueba/check, aceptación y evidencia. En Fase 0 limita los resultados a diseño/gobierno documental y no marques runtime como implementado.
4. Reporta **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO** por separado, con archivos, pruebas/checks y resultados reales. Ninguna referencia documental demuestra implementación ni aprobación.

## Procedimiento operativo

1. Inspecciona la estructura, implementación, documentación, contratos y decisiones existentes; separa hechos verificados de propuestas.
2. Identifica el requisito afectado, su dueño, interesados, límites, riesgos y evidencia requerida.
3. Para una capacidad transversal, consulta su matriz de definición/ownership/fases de la sección 14 y comprueba la salida documental de Fase 0: actor/dato, owner, contratos/integraciones, controles, dependencias, secuencia, aceptación, N/A y decisiones pendientes/aprobadores.
4. Identifica los artefactos que requiere el alcance —por ejemplo contexto, vista C4, contrato o ADR— y vincula la trazabilidad requisito → decisión (si aplica) → cambio → prueba → CI/evidencia.
5. Actualiza la fuente propietaria del contenido. Enlaza las fuentes especializadas en vez de copiar sus reglas.
6. Revisa compatibilidad, dependencias, referencias, aceptación y responsabilidades antes de cerrar o solicitar revisión.

## Restricciones

- No inventes aprobaciones, ADR, owners, contratos, fase actual ni capacidades implementadas.
- Mantén los requisitos de producto de ENSAMBRIA como específicos del proyecto; no los presentes como estándares universales.
- No crees documentación redundante ni conviertas una recomendación en obligación.
- Una excepción debe seguir el proceso de justificación, aprobación, alcance y revisión definido por las fuentes oficiales.

## Validaciones obligatorias

- Confirma que el requisito, el dueño y la fuente de verdad estén identificados.
- Comprueba trazabilidad, referencias locales, evidencia de aceptación y cambios documentales sincronizados.
- Para un cambio transversal, comprueba los cuatro owners de backend, límites de datos, fases de entrega y que Fase 0 siga documental sin habilitar runtime.
- Informa qué quedó implementado, pendiente, bloqueado o no aplicable; no declares una fase aceptada sin evidencia ejecutada.

## Criterios de finalización

La decisión o cambio queda en su documento propietario, conserva trazabilidad verificable, tiene responsables y aceptación claros, y no crea reglas paralelas ni afirma una aprobación inexistente.

## Conflictos y excepciones

Si hay contradicción real entre código, roadmap, contrato, ADR o estándares, detén únicamente el trabajo que dependa de ella. Registra las fuentes y el impacto, y solicita una resolución explícita antes de elegir una interpretación.

## Mantenimiento

Revisada el 2026-11-10. Si cambia una fuente enlazada, revisa esta Skill en el mismo PR. Registra la fecha de revisión real; conserva versiones de fuente solo cuando existan.
