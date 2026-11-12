# Inventario, compras y costos

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/inventory-purchasing-costs/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 4.

## Propósito y alcance

Controlar existencias y sus movimientos de compra, recepción, consumo, venta, transferencia, merma y costo para cada tenant y sucursal. El contexto evita saldos negativos no autorizados y ajustes sin evidencia.

Es dueño de SKU/productos de inventario, lotes, proveedores, recepciones, movimientos y costos. No es fuente de verdad pública ni owner de pedidos o reservas.

## Operaciones previstas

Recibir, ajustar, transferir y consumir existencias; consultar disponibilidad y costos con paginación o cursor. Las operaciones identifican scope, motivo y versión, y devuelven un movimiento confirmado o un saldo derivado.

## Límites e integraciones

- Se integra con catálogo, Commerce y operación mediante contratos.
- Puede consumir hechos autorizados de Commerce o servicios cuando el caso lo requiere.
- No utiliza JOINs SQL ni lecturas directas a reservas, CORE o PLATFORM.
- PUBLIC no consume inventario operativo como fuente de publicación.

## Seguridad e integridad

Se requiere autorización por tenant, sucursal y acción, RLS en datos tenant-scoped y auditoría de ajustes. Los movimientos deben conservar trazabilidad; operaciones concurrentes que afectan el saldo deben serializarse o controlar conflictos, y recepciones/consumos deben ser idempotentes.

La aceptación futura debe cubrir stock cero y negativo, doble consumo concurrente, lotes, devoluciones, sucursal ajena, replay y rollback.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni suite de pruebas; no se documentan comandos ni integraciones como existentes.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
