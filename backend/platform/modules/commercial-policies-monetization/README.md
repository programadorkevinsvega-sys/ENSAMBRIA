# Políticas comerciales y monetización

## Identificación y estado

- **Tipo:** bounded context de PLATFORM.
- **Ubicación:** `backend/platform/modules/commercial-policies-monetization/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fases previstas:** estructura inicial SaaS en Fase 1; consola en Fase 5; facturación SaaS en Fase 7; tarifas de Marketplace en Fase 9.

## Propósito y responsabilidad

Definir políticas comerciales versionadas de ENSAMBRIA, incluidas ofertas, planes y precios SaaS, separadas de la contratación y la ejecución financiera. PLATFORM es propietario de estas políticas; `saas-subscriptions` las referencia y CORE valida y registra sus efectos financieros autorizados.

## Operaciones previstas

Versionar, revisar, publicar o retirar políticas y consultar su vigencia. El contrato entrega identificador y versión; no ejecuta por sí mismo cargos, crea suscripciones automáticamente ni registra movimientos financieros.

## Límites e integraciones

- Depende de aprobación de Producto/Finanzas y de contratos con CORE.
- No administra el ciclo de vida de suscripciones, obligaciones de billing o facturas SaaS, que pertenecen a módulos PLATFORM separados.
- No escribe ledger, no cobra directamente ni administra comisiones de empleados o regalías de franquicia.
- La tarifa de Marketplace solo puede aplicarse tras ratificación comercial/financiera.

## Seguridad e integridad

Los cambios requieren permisos privilegiados, auditoría y aprobación configurada. No hay retroactividad implícita; la aplicación debe respetar versión y vigencia. Cualquier efecto financiero se ejecuta en CORE con idempotencia.

La aceptación futura cubre versión inválida, política no aprobada o expirada, scope incorrecto, replay y conciliación del cálculo antes de un cargo.

## Implementación y pruebas

Solo existe la estructura de capas; no hay políticas, tarifas ni efectos financieros implementados. No se declara ninguna regla comercial aprobada por este README.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
- [ADR-0007: separación de contextos SaaS](../../../../docs/adr/ADR-0007-separacion-contextos-facturacion-saas.md)
