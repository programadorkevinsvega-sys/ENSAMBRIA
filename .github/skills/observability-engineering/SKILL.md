---
name: observability-engineering
description: "Use when instrumenting or changing structured logs, metrics, distributed traces, correlation, health checks, SLI/SLO, alerts, dashboards, telemetry privacy, or observability operations."
---

# Observability Engineering

## Objetivo

Orquestar instrumentación y verificación operativa conforme al estándar vigente, sin replicar sus contratos, métricas, umbrales o runbooks en esta Skill.

## Alcance

Logs, métricas, trazas, propagación de contexto, errores diagnosticables, privacidad de telemetría, SLI/SLO, alertas, dashboards y operación.

## Cuándo activarla

- Obligatoria al instrumentar servicios, workers, dependencias o flujos que afecten observabilidad.
- En ENSAMBRIA, el diseño aplica desde Fase 0 y la implementación runtime desde el primer corte ejecutable de Fase 1; revisa el alcance vigente de cada componente.

## Referencias normativas obligatorias

- [Observability, Monitoring, Alerting and SLO ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md)
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Privacy Legal Compliance ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Privacy_Legal_Compliance.md) y [Privacy Data Lifecycle Management ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md), para datos de telemetría.

## Aplicación obligatoria en ENSAMBRIA

Antes de instrumentar o cambiar señales:

1. Confirma fase, owner y fronteras; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), la regla de observabilidad completa, requisitos aplicables de privacidad, contrato/ADR y aceptación del flujo.
2. Traduce cada obligación en eventos/campos/destinos y pruebas de propagación, correlación, redacción, cardinalidad, errores y caída del exporter; mantén identidad de dominio fuera de la biblioteca técnica.
3. Prueba las fronteras reales; no declares instrumentación, SLO, alertas o envío externo operativos por existir configuración o documentación. Registra comandos, resultado y evidencia.
4. Separa **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO** y detén solo el aspecto que dependa de conflicto o decisión sin resolver.

## Procedimiento operativo

1. Inspecciona instrumentación, propagación, exporters, configuración y mecanismos de operación existentes.
2. Identifica fronteras, errores, señales, contexto y necesidades de diagnóstico para el comportamiento afectado.
3. Selecciona primitivas, destinos y umbrales solo con base en decisiones y guías aprobadas; trata ejemplos como ejemplos.
4. Integra controles de privacidad y resiliencia de exportación sin cambiar la semántica de negocio.
5. Prueba la propagación y diagnóstico a través de las fronteras reales, además del comportamiento ante fallos del destino.

## Restricciones

- No agregues SDKs, exporters, destinos, alertas ni SLO no justificados por la guía y el repositorio.
- No incluyas secretos, PII, tokens, cookies, URL firmadas o cardinalidad ilimitada en telemetría.
- No hagas que una falla de exportación remota bloquee el camino de negocio cuando la arquitectura exige operación best-effort.
- No afirmes recepción externa, cumplimiento de SLO o alertas activas sin evidencia.

## Validaciones obligatorias

- Comprueba correlación/contexto, redacción, clasificación de errores y cardinalidad conforme a la fuente normativa.
- Verifica eventos de diagnóstico y cierre del recorrido, y que un destino caído no cause fallas de negocio cuando aplique.
- Contrasta SLI/SLO, alertas y dashboards con datos/evidencia y ejecuta pruebas pertinentes; informa lo no verificado.

## Criterios de finalización

El comportamiento afectado produce señales seguras y accionables, mantiene la operación prevista bajo fallos de telemetría y cuenta con evidencia sobre las fronteras y condiciones probadas.

## Conflictos y excepciones

Si las capacidades existentes no cubren la guía o hay tensión entre diagnóstico y privacidad, detén solo el alcance afectado y solicita decisión documentada. No declares una alerta, destino o SLO como operativo por existir en documentación.

## Mantenimiento

Revisada el 2026-11-10. Mantén la Skill alineada con la guía especializada y los estándares de privacidad; no inventes versiones ni umbrales.
