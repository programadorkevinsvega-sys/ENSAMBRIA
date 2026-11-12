# Liquidación y conciliación

## Identificación y estado

- **Tipo:** bounded context de CORE.
- **Ubicación:** `backend/core/modules/settlements-reconciliation/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** Fase 7, junto con integraciones y conciliación del roadmap.

## Propósito y responsabilidad

Comparar operaciones internas con datos externos de liquidación para hacer visibles pérdidas y discrepancias. CORE es propietario de lotes, referencias de liquidación, líneas conciliadas y discrepancias; el ledger financiero continúa siendo fuente interna autorizada.

## Operaciones previstas

Iniciar e importar un lote, conciliar registros, marcar discrepancias y consultar resultados. La carga puede provenir de un archivo o API real una vez que exista un contrato aprobado.

## Límites e integraciones

- Se integra con el ledger de CORE y con un proveedor seleccionado mediante adapter.
- La conciliación no depende de esquemas comerciales de BUSINESS o PLATFORM.
- Requiere un workload de mínimo privilegio y no modifica automáticamente el ledger para resolver diferencias.
- El proveedor, su contrato y sandbox deben verificarse antes de habilitar la operación.

## Seguridad e integridad

Las importaciones repetidas deben ser idempotentes; se preservan claves y versiones del proveedor. No se resuelven discrepancias mediante heurísticas silenciosas.

La aceptación futura cubre archivo duplicado, filas inválidas, orden, concurrencia, caída y reanudación, discrepancias y ausencia de cambios automáticos o filtración de credenciales.

## Implementación y pruebas

El directorio contiene solo estructura de capas. No hay proveedor, contrato ni pruebas con sandbox; no se declara conciliación operativa.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
