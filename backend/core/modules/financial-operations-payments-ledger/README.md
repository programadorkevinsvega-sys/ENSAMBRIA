# Operaciones financieras, pagos y ledger

## Identificación y estado

- **Tipo:** bounded context de CORE.
- **Ubicación:** `backend/core/modules/financial-operations-payments-ledger/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fases previstas:** cimientos de identidad/contexto y onboarding en Fase 1; pagos y ledger operativo en Fase 4; conciliación e integraciones en Fase 7; pagos iniciados en Marketplace en Fase 9.

## Propósito y responsabilidad

Registrar efectos transaccionales financieros autorizados y su evidencia. CORE es propietario de operaciones de pago, cargos y reembolsos aprobados, ledger y referencias. BUSINESS mantiene pedidos y ventas; PLATFORM conserva políticas y facturación SaaS.

## Operaciones previstas

Iniciar, confirmar o revertir una operación y consultar el ledger autorizado. Las solicitudes deben incluir importe exacto, moneda, actor y clave de idempotencia; las respuestas reflejan estados tipados y no exponen datos de tarjeta.

## Límites e integraciones

- gRPC atiende solicitudes que requieren respuesta inmediata; Outbox distribuye hechos persistidos cuando haya consumidores durables.
- Adapters de proveedores permanecen desacoplados, separados por proveedor, workload y caso de uso.
- BUSINESS y PLATFORM no escriben el ledger ni acceden a sus tablas.
- Un proveedor solo se integra tras verificar documentación oficial, contrato, sandbox y autorizaciones requeridas.

## Seguridad e integridad

Se requiere autorización del owner y scope, credenciales mínimas y minimización de datos. Cada operación debe ser idempotente; importes/monedas explícitos, transiciones válidas y ledger append-only. Los cambios financieros dependen de las aprobaciones aplicables; no se consideran habilitados por este diseño.

La aceptación futura cubre replay, timeout/fallo/firma inválida de proveedor, moneda incompatible, cancelación concurrente, falta de autorización y conciliación sin parcialidad.

## Implementación y pruebas

Solo existe la estructura de capas. No hay proveedor, credenciales, contratos ejecutables ni pruebas reales; no se declaran operaciones financieras implementadas o validadas.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
