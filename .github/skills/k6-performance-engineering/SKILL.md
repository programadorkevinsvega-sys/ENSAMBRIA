---
name: k6-performance-engineering
description: "Use when planning, implementing, reviewing, or validating Grafana k6 performance, load, stress, spike, soak, capacity, browser, or regression tests."
---

# k6 Performance Engineering

## Objetivo

Diseñar y validar pruebas de rendimiento ENSAMBRIA con Grafana k6 de forma reproducible, segura y basada en los objetivos y evidencia reales del sistema.

## Alcance

Planificación, scripts, modelos de carga, thresholds, datos de prueba, ejecución autorizada, análisis de resultados, capacidad, regresiones e integración CI de pruebas k6.

## Cuándo activarla

- Al planificar o implementar pruebas de carga, estrés, picos, resistencia, capacidad o regresión con k6.
- Al evaluar rendimiento de una API, frontend, backend, worker, integración o flujo de negocio.
- Al analizar resultados, thresholds, saturación del generador o regresiones.
- Junto con Skills de backend, PostgreSQL, mensajería, observabilidad, web o calidad Rust según las fronteras afectadas.

## Referencias normativas obligatorias

- [ADR-0012 — modular boundaries and architecture compliance](../../../docs/adr/ADR-0012-fronteras-modulares-y-cumplimiento-arquitectonico.md) (consult when the task changes module ownership, data access, or release acceptance)

- [Reglas ENSAMBRIA de pruebas k6](../../../docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md), fuente normativa de alcance, seguridad, modelos de carga, aceptación y evidencia.
- [Ingeniería de calidad empresarial para Rust](../../../docs/rules/ENSAMBRIA_prompt_maestro_enterprise_quality_engineering.md), cuando aplique a código, pruebas o CI Rust.
- [Observabilidad, monitoreo, alertas y SLO](../../../docs/rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md), para correlacionar carga con señales de aplicación e infraestructura.
- [Contrato de arquitectura ENSAMBRIA](../../../ARCHITECTURE_RULES.md), para ownership, límites y despliegues.
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), para alcance y criterios afectados.

## Aplicación obligatoria en ENSAMBRIA

Antes de diseñar o ejecutar:

1. Confirma fase, caso de uso/owner, alcance autorizado y criterios; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), la regla k6 completa, criterios del producto, requisitos de observabilidad y ADR/contratos pertinentes.
2. Convierte los objetivos aprobados en un modelo de carga, datos, thresholds, límites y criterios de aborto verificables. No infieras autorización, valores de capacidad o parámetros desde ejemplos ni desde un resumen.
3. Verifica que los datos sintéticos sean transitorios, no personales y generados mediante flujos/adapters reales en un entorno autorizado; protege aislamiento, evita efectos externos reales y define limpieza antes de la ejecución.
4. Relaciona cada escenario con su componente dueño, prueba, comando, resultado y evidencia. Si falta aprobación o ambiente, no generes carga; reporta el bloqueo. Separa **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO**.

## Procedimiento operativo

1. Lee íntegramente la regla k6 y las fuentes aplicables; identifica el caso de uso, riesgos, entorno y responsables.
2. Inspecciona scripts, manifests, pipelines, endpoints, autenticación, datos, observabilidad y herramientas ya disponibles. Reutiliza el diseño existente y confirma la versión real de k6 antes de usar sus capacidades.
3. Documenta el objetivo, el modelo de carga, executor, duración, datos, thresholds aprobados, criterios de interrupción, autorización del destino y procedimiento de limpieza. No inventes niveles de carga ni objetivos de rendimiento.
4. Implementa escenarios modulares que verifiquen el flujo de negocio, efectos posteriores e integridad bajo concurrencia. Mantén secretos fuera de scripts, logs y reportes.
5. Antes de ejecutar, verifica explícitamente que el destino está autorizado. No inicies cargas altas, pruebas destructivas o pruebas sobre producción/proveedores sin autorización específica.
6. Ejecuta solo comandos aplicables y aprobados. Registra versión, entorno, commit, perfil, carga alcanzada, thresholds, checks, `dropped_iterations`, métricas correlacionadas y recuperación.
7. Interpreta limitaciones y resultados fallidos; no declares capacidad validada si la carga no fue alcanzada o faltan datos suficientes. Actualiza documentación y CI cuando el alcance lo requiera.

## Restricciones

- No ejecutes carga contra terceros, dominios no confirmados o producción sin autorización explícita.
- No realices pruebas de estrés, spike, breakpoint ni destructivas sin aprobación y límites de interrupción definidos.
- No uses información personal productiva, credenciales productivas ni pagos/notificaciones reales para cargas ordinarias.
- No inventes escenarios, umbrales, objetivos de capacidad, resultados, comandos o compatibilidad de extensiones.
- No ocultes thresholds fallidos, checks, iteraciones descartadas o errores ni cambies perfiles para fabricar resultados positivos.
- Una respuesta HTTP exitosa o un smoke test no demuestra capacidad, escalabilidad ni éxito completo de un flujo asíncrono.

## Validaciones obligatorias

- Verifica versión de k6, sintaxis, autorización y reproducibilidad del escenario.
- Comprueba checks funcionales, thresholds, perfiles de carga y evaluación de `dropped_iterations`.
- Analiza latencias pertinentes (p50/p90/p95/p99 según el caso), errores, throughput, recursos del generador y métricas de los componentes afectados.
- Verifica datos e integridad durante y después de la carga, procedimiento de limpieza y recuperación del sistema.
- Contrasta los comandos locales con CI aplicable y reporta exactamente lo ejecutado, fallido, bloqueado y no ejecutado.

## Criterios de finalización

El escenario representa el objetivo aprobado, se ejecutó únicamente en un destino autorizado, alcanzó la carga solicitada o declara la limitación, aplica criterios de aceptación explícitos, conserva evidencia reproducible y verifica integridad y recuperación.

## Conflictos y excepciones

Si faltan autorización, perfil de carga aprobado, thresholds, entorno seguro o señales para interpretar resultados, no ejecutes la prueba afectada ni declares capacidad validada. Expón el bloqueo y solicita la decisión requerida. Toda excepción debe seguir la aprobación y vigencia establecidas por la fuente normativa.

## Mantenimiento

Revisada el 2026-11-24. Al cambiar las normas k6, la versión disponible o los pipelines, actualiza esta Skill y los perfiles asociados; ejecuta `python3 scripts/validate_engineering_skills.py`.
