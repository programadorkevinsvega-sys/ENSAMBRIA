# Registro de implementación — Fundación CORE y PLATFORM

## Estado de las fases

- **Fase 1.2:** implementación integrada; pruebas y aceptación formal pendientes para una ejecución posterior.
- **Fase 1.3:** implementación integrada y CI automatizado aprobado; runner E2E manual en staging y aceptación formal pendientes para una ejecución posterior.
- **Fase 1.4:** autorizada para implementación en paralelo con esos pendientes. La autorización no los cierra ni declara aceptadas 1.2/1.3.

## Fase 1.4 — alcance y estado

PLATFORM posee ofertas SaaS versionadas y suscripciones contractuales. CORE conserva la evaluación y aplicación runtime de entitlements. El alta técnica del tenant crea únicamente `onboarding_pending`; no asigna plan, trial, precio ni suscripción. Billing, pagos y facturas SaaS pertenecen a Fase 7.

La implementación de esta fase añade persistencia de ofertas/versiones inmutables, ciclo explícito de suscripción, RLS tenant-scoped, auditoría, idempotencia y Outbox transaccional. No se inventan precios, moneda, impuestos, periodicidad, trial, auto-renovación ni gracia comercial. Antes de aceptar la fase deben completarse la administración autenticada de ofertas, el adapter HTTP/BFF o gRPC autorizado, la publicación JetStream, el consumo contractual de CORE y sus pruebas E2E.
