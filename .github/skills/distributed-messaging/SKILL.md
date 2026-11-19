---
name: distributed-messaging
description: "Use when designing or changing durable asynchronous events, transactional Outbox/Inbox, message producers or consumers, delivery guarantees, retries, replay, or event-driven integrations."
---

# Distributed Messaging

## Objetivo

Guiar el análisis y validación de flujos de mensajería según sus garantías y ownership reales, sin imponer un broker ni duplicar el estándar Outbox/Inbox.

## Alcance

Eventos durables, productores, consumidores, Outbox/Inbox, idempotencia, orden, reintentos, recuperación y contratos de eventos, incluyendo entrega durable de webhooks cuando la requiera el producto.

## Cuándo activarla

- Obligatoria cuando un cambio requiere propagar de forma asíncrona y durable un hecho persistido, o modifica productor, consumidor, Outbox/Inbox o replay.
- Desde Fase 0, participa en la definición del catálogo de hechos/consumidores, los canales (RPC vs. evento durable vs. realtime efímero), los contratos y la recuperación para las capacidades de eventos/webhooks de la sección 14 del requerimiento.
- En ENSAMBRIA, consulta además el requisito de Fase 1 y las obligaciones realtime aplicables en [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md).
- En ENSAMBRIA, el primer flujo durable ratificado es el alta operativa CORE→PLATFORM de la subfase 1.3; no se crean eventos artificiales en 1.1 ni se habilita el alta operativa antes de contar con el flujo completo. Un caso durable real anterior debe incluir Outbox/Inbox en el mismo corte.
- En otros proyectos, actívala solo si su arquitectura requiere mensajería durable.

## Referencias normativas obligatorias

- [Transactional Outbox/Inbox ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Transactional_Outbox_Inbox.md)
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), solo para alcance de producto y secuencia ENSAMBRIA.

## Aplicación obligatoria en ENSAMBRIA

Antes de implementar o cambiar un flujo:

1. Confirma fase, hecho de negocio, backend owner, consumidores y aceptación; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), la regla Outbox/Inbox completa, los requisitos de fase y contratos/ADR pertinentes. No trates una propuesta de evento como aprobada.
2. Especifica por cada flujo por qué necesita entrega durable, qué estado ya se confirmó, consumidor real, envelope/versionado, scope/privacidad, idempotencia, orden y recuperación. Si no hay necesidad funcional, justifica N/A según la fuente y no agregues evento.
3. Relaciona mutación del owner, Outbox, publicación, Inbox/efecto/ACK, realtime/cursor y pruebas con integraciones reales; no cruces schemas, transacciones ni ownership.
4. Reporta fallos inducidos, recuperación, pruebas/comandos/resultados y evidencia. Separa **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO**; un mock no valida entrega durable.

## Procedimiento operativo

1. Determina el hecho de negocio, backend dueño, consumidores, contrato y garantía necesaria; distingue RPC síncrono de evento durable, señal realtime efímera y webhook externo.
2. Revisa atomicidad, versionado, deduplicación, autorización, tenant/scope, orden requerido y recuperación ante fallos.
3. Documenta la matriz de integración y escenarios antes o junto con la implementación.
4. En webhooks, distingue suscripciones/eventos salientes de callbacks entrantes de proveedores; verifica firma y egress conforme al contrato, sin confiar en URLs/datos no autorizados.
5. Implementa el flujo según la fuente normativa, con transacciones locales al dueño y sin mantener recursos transaccionales abiertos durante llamadas al broker.
6. Valida con infraestructura real o la dependencia de integración aprobada; registra fallos inducidos, recuperación y resultados.

## Restricciones

- No asumas NATS, Kafka u otro broker fuera de las decisiones verificadas.
- No uses eventos como RPC ni el broker como sustituto de una consulta/comando síncrono.
- No escribas en tablas/Outbox de otro owner, ni consideres mocks evidencia de entrega durable.
- No declares entrega exactamente una vez; el consumidor debe tolerar duplicados conforme al contrato.

## Validaciones obligatorias

- Verifica persistencia atómica, confirmación/publicación, ACK tras commit, idempotencia, reintentos, duplicados, orden/versionado y recuperación según aplicabilidad.
- Ejecuta pruebas de integración con PostgreSQL y broker real/de prueba equivalente cuando lo exija el estándar.
- Verifica redacción de PII/secretos, métricas/diagnóstico de fallos y runbooks requeridos.

## Criterios de finalización

El flujo tiene owner y contrato definidos, soporta su garantía ante fallos y duplicados, y cuenta con evidencia de pruebas reales para los límites de integración afectados.

## Conflictos y excepciones

Si el caso no necesita entrega durable, registra la justificación exigida por el proyecto y no agregues mensajería por rutina. Si el contrato o infraestructura no permite validar la garantía, informa el bloqueo y no declares aceptación.

## Mantenimiento

Revisada el 2026-11-18. Si cambian las garantías o contratos fuente, actualiza la Skill en el mismo PR. No inventes versiones ni contratos de ejemplo como si fueran reales.
