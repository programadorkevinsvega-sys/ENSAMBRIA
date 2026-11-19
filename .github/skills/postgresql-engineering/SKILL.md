---
name: postgresql-engineering
description: "Use when a task changes PostgreSQL persistence, SQLx queries, migrations, connection pools, transaction boundaries, RLS, database permissions, or database-dependent workers."
---

# PostgreSQL Engineering

## Objetivo

Promover persistencia PostgreSQL segura, aislada y dimensionada con evidencia, consultando la estrategia oficial en lugar de copiarla.

## Alcance

Pool por proceso, presupuesto de conexiones, credenciales, migraciones, transacciones, RLS, permisos, timeouts, workers y pruebas de persistencia.

## Cuándo activarla

- Obligatoria si el proyecto o componente afectado usa PostgreSQL o SQLx.
- En ENSAMBRIA, consulta esta guía desde Fase 0 y antes de cambiar persistencia, pools, RLS, migraciones o workers.
- En otros proyectos, no impongas PostgreSQL ni sus mecanismos cuando no se utilicen.

## Referencias normativas obligatorias

- [Connection Pooling PostgreSQL ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md)
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), cuando el caso afecte el producto o su fase.

## Aplicación obligatoria en ENSAMBRIA

Antes de tocar persistencia:

1. Confirma fase, proceso/backend y módulo propietario; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), la guía PostgreSQL completa, requisitos de fase y ADR/contratos aprobados pertinentes.
2. Mapea la única base compartida a schemas/migraciones por owner, roles migrator/runtime, grants mínimos, pool por proceso y la condición de RLS. Define cómo una credencial/tenant ajeno será denegado; documentar RLS o aislamiento no demuestra enforcement.
3. Vincula cada control a PostgreSQL real y a pruebas negativas de acceso entre tenants/dominos, fuga de contexto al reutilizar conexión, rollback, timeout/concurrencia y recuperación según aplique. Reporta versiones, comandos, resultados y evidencia.
4. No infieras aprobación de ADR ni configures valores ilustrativos como efectivos. Distingue **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO**; detén cambios cuyo aislamiento dependa de una decisión no ratificada.

## Procedimiento operativo

1. Inspecciona el cliente, versión, manifests, composición de procesos, escalado y configuración existente.
2. Identifica quién posee cada schema/tabla/migración y qué credenciales son necesarias para cada proceso.
3. Calcula el presupuesto agregado considerando réplicas, workers y conexiones operativas; define límites y timeouts con datos reales.
4. Revisa transacciones, duración de checkout, cancelación, concurrencia y contexto tenant/RLS en la reutilización de conexiones.
5. Documenta decisiones y no aplicabilidad antes de implementar; valida contra PostgreSQL real cuando sea requisito de aceptación.

## Restricciones

- No crees pools por solicitud/tenant, ni uses credenciales irrestrictas o acceso cruzado entre módulos.
- No introduzcas PgBouncer sin decisión, medición y pruebas de compatibilidad.
- No conviertas ejemplos de tamaño, timeout o concurrencia de una guía en configuración aprobada.
- No afirmes que PostgreSQL real fue probado si solo se ejecutaron mocks o análisis estático.

## Validaciones obligatorias

- Comprueba permisos mínimos, ownership de migraciones, presupuesto total, adquisición/cancelación y recuperación.
- Prueba con PostgreSQL real los aspectos de persistencia, RLS/limpieza del contexto, concurrencia y saturación que apliquen.
- Ejecuta migraciones y pruebas pertinentes de forma coordinada, sin tocar entornos compartidos o producción sin autorización.

## Criterios de finalización

El cambio mantiene ownership y aislamiento, el presupuesto/configuración es justificable, y cada garantía reclamada tiene evidencia de validación apropiada.

## Conflictos y excepciones

Si el entorno o versión real no está disponible, registra el bloqueo y limita las afirmaciones a lo validado. Cualquier alternativa o N/A debe tener justificación documentada y aprobada conforme a las fuentes del proyecto.

## Mantenimiento

Revisada el 2026-11-10. Actualiza esta Skill junto con cambios a las fuentes citadas. Conserva los metadatos fuente disponibles; no inventes una versión.
