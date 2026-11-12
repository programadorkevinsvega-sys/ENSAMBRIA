# ADR-0009: Proveedores iniciales de pagos y facturación electrónica; reventa de capacidad Factus

- **Estado:** decisión expresa aceptada por el propietario el 2026-11-11; implementación futura
- **Fecha:** 2026-11-11
- **Ámbito:** estrategia de proveedores para pagos y facturación electrónica y comercialización de capacidad Factus
- **Requisitos:** [requerimiento §§11.1–11.3 y Fase 7](../requerimiento.md)
- **Fuentes arquitectónicas:** [contrato de arquitectura](../../ARCHITECTURE_RULES.md), [modelo de dominios](../arquitectura-dominios.md#23-platform--backend-platform), [ADR-0007](ADR-0007-separacion-contextos-facturacion-saas.md)

## Contexto

El requerimiento nombra Wompi para pagos y Factus para facturación electrónica, y establece la separación de ownership, cuentas, emisores, credenciales, callbacks y registros. La estrategia de proveedores debe reconocer estos proveedores iniciales sin acoplar los dominios a un único proveedor ni presentar integraciones no implementadas como validadas.

El propietario también aprobó que ENSAMBRIA comercialice capacidad de facturación electrónica Factus a sus tenants. Esa capacidad comercial no transfiere a PLATFORM la propiedad de las facturas de los establecimientos ni permite compartir la integración fiscal entre emisores.

## Decisión

1. **Pagos:** Wompi es el proveedor inicial previsto para los adapters de CORE. No es exclusivo. El diseño mantiene adapters desacoplados para permitir incorporar otros proveedores sin cambiar las reglas de los dominios ni el ownership financiero.
2. **Facturación electrónica:** Factus es el proveedor inicial previsto para las integraciones de PLATFORM y BUSINESS. No es exclusivo; se pueden incorporar otros proveedores mediante adapters conforme a los requisitos de cada emisor y jurisdicción.
3. **Separación de pagos:** CORE es el único owner del procesamiento de pagos y del ledger. Las operaciones iniciadas por PLATFORM y BUSINESS conservan cuentas/conexiones, credenciales, callbacks, permisos, idempotencia, auditoría y registros separados. PLATFORM y BUSINESS no escriben el ledger ni comparten referencias de operación.
4. **Separación fiscal:** PLATFORM conserva la integración y los documentos electrónicos correspondientes a obligaciones de ENSAMBRIA. BUSINESS conserva la integración y las facturas electrónicas de cada establecimiento. La configuración, credenciales, callbacks, documentos e historial de un emisor no se reutilizan para otro.
5. **Reventa de capacidad Factus:** PLATFORM puede comercializar a sus tenants la capacidad de facturación electrónica de Factus como oferta/capacidad de producto. PLATFORM es dueño de la oferta y sus condiciones comerciales aprobadas; BUSINESS conserva la operación fiscal y los documentos de cada establecimiento, y PLATFORM conserva los de ENSAMBRIA. CORE procesa los pagos autorizados por los contratos correspondientes.
6. La reventa no convierte a ENSAMBRIA en emisor de las facturas de un establecimiento, no centraliza su configuración fiscal y no permite compartir credenciales, callbacks, datos o estados entre emisores. Tampoco crea un backend, ledger o dominio fiscal compartido.
7. El alcance se implementa en Fase 7 conforme al requerimiento. Antes de habilitar cada integración o comercializar capacidad, se verifican documentación oficial, capacidades, contratos y condiciones de proveedor, requisitos fiscales por emisor/jurisdicción y aprobaciones comerciales/legales aplicables.
8. No se presumen mecanismo de partner/reventa de Factus, capacidad agrupada, unidades/cuotas, precios, márgenes, límites, liquidación ni un modelo de credenciales común. La configuración concreta debe corresponder a las capacidades y contratos verificados de Factus y respetar los owners y emisores definidos arriba.

## Límites aún no definidos

- No hay integración de Wompi o Factus implementada ni validada en runtime.
- Los términos comerciales de la capacidad Factus, sus cuotas o unidades, precios, costos y condiciones de reventa requieren aprobación de Producto/Finanzas.
- La disponibilidad contractual y técnica de las funciones de partner/reventa y el modelo de conexión/credenciales se confirma con Factus antes de habilitarlos; no se atribuyen capacidades no documentadas.
- Las obligaciones fiscales, roles, documentos y requisitos por jurisdicción requieren validación aplicable; este ADR no declara conformidad ni aprobación jurídica.
- La incorporación de otro proveedor requiere selección, revisión de seguridad, documentación oficial, contrato, sandbox y pruebas de aceptación reales para el caso de uso y emisor afectados.

Estas pendientes no revierten la decisión de usar inicialmente Wompi y Factus ni la aprobación de comercializar capacidad Factus. Solo impiden habilitar el comportamiento específico que dependa de una capacidad, condición o aprobación aún no verificada.

## Alternativas consideradas

1. Tratar Wompi o Factus como proveedores exclusivos: descartada porque el propietario requiere flexibilidad para incorporar otros proveedores y los dominios no deben acoplarse al SDK o modelo externo.
2. Compartir un único contexto, cuenta o estado de Factus entre PLATFORM y BUSINESS: descartada porque contradice la separación de owners y emisores del requerimiento.
3. Permitir que PLATFORM emita las facturas de los establecimientos al revender capacidad: descartada porque PLATFORM solo es dueño de la comercialización SaaS; BUSINESS conserva la facturación de cada establecimiento.
4. Asumir cuotas, precios o un programa de partner no confirmado: descartada porque serían capacidades y términos no verificados.

## Consecuencias y criterios de aceptación

- Las referencias documentales identifican Wompi y Factus como proveedores iniciales previstos, no como integraciones implementadas o proveedores exclusivos.
- CORE puede añadir otro adapter de pagos sin cambiar las reglas comerciales de PLATFORM ni las reglas operativas de BUSINESS.
- PLATFORM y BUSINESS mantienen adapters y configuración fiscal separados por owner/emisor; pruebas de aislamiento rechazan documentos, credenciales, callbacks y referencias cruzadas.
- La capacidad comercial Factus se gobierna en PLATFORM; la autorización/entitlement no sustituye la validación del emisor ni da acceso a documentos de otro tenant.
- Las pruebas de pago validan autorización, idempotencia, callbacks y ledger con cuentas/credenciales de sandbox apropiadas por caso de uso. Las pruebas fiscales validan emisor, emisión, callbacks, reintentos, estados e aislamiento con el entorno oficial de prueba disponible.
- La aceptación requiere dependencias reales de integración; mocks, documentos de diseño o la existencia de adapters no prueban una integración.

## Impacto

- **Componentes:** CORE pagos/ledger y adapters; PLATFORM políticas comerciales, billing SaaS y transmisión electrónica; BUSINESS facturación del establecimiento.
- **Seguridad y datos:** credenciales mínimas por workload e integración; datos fiscales y financieros segregados; sin secretos ni contenido sensible en logs.
- **Comercial y legal:** la decisión habilita diseñar la oferta de capacidad Factus, pero no aprueba precios, condiciones contractuales, términos del proveedor ni cumplimiento fiscal.
- **Estado:** decisión documental aceptada; implementación, verificación de proveedores y aprobación de condiciones pendientes.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios](../arquitectura-dominios.md)
- [ADR-0007: separación de contextos SaaS](ADR-0007-separacion-contextos-facturacion-saas.md)
- [CORE: pagos y ledger](../../backend/core/modules/financial-operations-payments-ledger/README.md)
- [PLATFORM: facturación electrónica SaaS](../../backend/platform/modules/electronic-invoicing/README.md)
- [BUSINESS: facturación del establecimiento](../../backend/business/modules/establishment-billing/README.md)
