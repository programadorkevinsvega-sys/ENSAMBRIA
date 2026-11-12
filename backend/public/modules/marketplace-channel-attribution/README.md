# Marketplace y atribución de canal

## Identificación y estado

- **Tipo:** bounded context de PUBLIC.
- **Ubicación:** `backend/public/modules/marketplace-channel-attribution/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** Fase 9; tarifas o cobros requieren decisión comercial/financiera ratificada.

## Propósito y responsabilidad

Registrar el origen y participación del canal público en una interacción o conversión. PUBLIC es dueño de las referencias minimizadas de campaña/listing/click y de la atribución; no es dueño del pedido, su precio ni el ledger.

## Operaciones previstas

Registrar una referencia de canal, resolver una conversión según política aprobada y consultar agregados autorizados. Los identificadores y datos de seguimiento no se amplían más allá de la finalidad permitida.

## Límites e integraciones

- BUSINESS y CORE pueden aportar referencias o confirmaciones mediante contratos autorizados.
- PLATFORM es propietario de políticas/tarifas; CORE registra cualquier movimiento financiero autorizado.
- No modifica pedidos, precios ni ledger.
- No aplica tarifas hasta que exista ratificación comercial/financiera.

## Seguridad e integridad

Se limitan audiencia y propósito; los datos de seguimiento requieren autorización aplicable. La conversión debe deduplicarse y conservar la versión de política usada; un reintento no cuenta dos veces.

La aceptación futura cubre repetición, falta de consentimiento cuando aplique, llegada tardía, venta cancelada, tenant cruzado y conciliación con el owner fuente.

## Implementación y pruebas

El directorio contiene solo estructura de capas. No hay pipeline de atribución, contrato ejecutable ni política tarifaria aprobada; no se declara monetización implementada.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
