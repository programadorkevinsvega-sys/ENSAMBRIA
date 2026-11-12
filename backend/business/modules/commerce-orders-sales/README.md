# Commerce, pedidos y ventas

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/commerce-orders-sales/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** ventas, caja y comisiones operativas en Fase 4; tiendas, carrito y pedidos de comercio electrónico o Marketplace en Fase 9.

## Propósito y alcance

Poseer las transacciones comerciales del establecimiento: carrito, orden, líneas, precio acordado y estado de venta. BUSINESS mantiene las reglas de venta y distingue la orden comercial de la operación financiera central.

El dinero y sus efectos financieros pertenecen a CORE; el catálogo y el inventario operativo permanecen bajo sus módulos BUSINESS propietarios. PUBLIC es dueño del canal Marketplace y de la atribución, no de los pedidos ni del inventario.

## Operaciones previstas

Cotizar, crear, confirmar, cancelar y consultar pedidos e historial autorizado. El BFF entrega el carrito y la versión de oferta; BUSINESS revalida precio, disponibilidad y entitlement y devuelve estado validado y, si aplica, una referencia financiera de CORE.

## Límites e integraciones

- CORE recibe solicitudes financieras autorizadas y devuelve hechos confirmados por contrato.
- Catálogo e inventario se consultan mediante contratos de BUSINESS.
- PUBLIC puede intervenir en publicación/canal autorizado, pero no adquiere ownership de ventas.
- Un evento conceptual como `OrderCreated` no constituye un contrato; solo se concreta si existe consumidor durable y necesidad aprobados.

## Seguridad e integridad

Cada operación valida actor, cliente, tenant y sucursal. El checkout requiere clave idempotente y versiones de precio/stock; reintentos no deben producir doble cobro ni estado parcial.

La aceptación futura debe cubrir carrito duplicado, cambio de precio, concurrencia de inventario, permisos, pago rechazado o con timeout, replay, cancelación y ausencia de efectos duplicados.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni suite de pruebas; no se declaran endpoints o comandos disponibles.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
