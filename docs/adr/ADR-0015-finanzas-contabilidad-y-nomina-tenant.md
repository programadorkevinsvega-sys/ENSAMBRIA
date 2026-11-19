# ADR-0015: Finanzas, contabilidad y nómina del tenant

- **Estado:** propuesta; no aceptada
- **Fecha de propuesta:** 2026-11-19
- **Ámbito:** libros y operaciones financieras de establecimientos, cuentas por cobrar/pagar, contabilidad y nómina/compensación empresarial
- **Requisitos:** [requerimiento §§4.3, 4.11, 9 y 22](../requerimiento.md)
- **Fuentes:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [modelo de dominios](../arquitectura-dominios.md), [ADR-0007](ADR-0007-separacion-contextos-facturacion-saas.md), [ADR-0012](ADR-0012-jerarquia-empresarial-contexto-operativo.md), [ADR-0013](ADR-0013-autorizacion-rbac-abac-y-acceso-soporte.md), [ADR-0014](ADR-0014-coordinacion-ventas-pagos-inventario.md)

## Contexto

El requerimiento solicita contabilidad tenant, tesorería, cuentas por cobrar/pagar, compras, libros, cierres y estados financieros. También propone que Finanzas/Payroll gestione el cálculo y pago de nómina, bonos y distribución de comisiones.

La arquitectura vigente separa responsabilidades: BUSINESS posee las operaciones empresariales, compras, facturación tenant y comisiones de empleados; CORE posee pagos, cargos transaccionales aprobados, ledger, liquidaciones y conciliación de pagos; PLATFORM posee suscripciones y facturación SaaS. No existe una decisión aceptada que asigne el libro contable tenant, la tesorería, la conciliación de caja/bancos ni las cuentas por cobrar/pagar. El cambio propuesto de subdominio para cálculo/distribución de comisiones requiere conciliarse con el ownership aceptado de BUSINESS.

Este ADR propone una asignación funcional **dentro de BUSINESS**, no un quinto backend, una nueva unidad de despliegue ni acceso directo a datos de otros owners. No propone cambiar la propiedad financiera de CORE ni la facturación SaaS de PLATFORM. La propuesta no constituye aprobación legal, comercial o de fase, ni autoriza implementar o procesar datos de nómina.

## Propuesta de decisión

1. Asignar a una capacidad **Finance/Payroll de BUSINESS** la contabilidad y las obligaciones financieras propias de las entidades empresariales tenant: plan de cuentas, centros de costos, asientos y ajustes, cierres/reaperturas, tesorería, cuentas por cobrar/pagar, conciliación contable de caja/bancos y reportes financieros. BUSINESS conserva las facturas del establecimiento.
2. Asignar a esa capacidad propuesta la liquidación contable de nómina, bonos y comisiones de empleados, incluidas sus aprobaciones y obligaciones. Workforce conservaría los datos de relación laboral/operativa, condiciones contractuales, turnos y asistencia que sean necesarios, bajo su owner y por contratos explícitos.
3. Mantener el pago/transferencia y su ledger transaccional bajo CORE. Finance/Payroll solicita a CORE el efecto financiero autorizado mediante contrato; no escribe ni replica el ledger. El resultado que deba actualizar una obligación o liquidación BUSINESS se propaga durablemente por el flujo Outbox/JetStream/Inbox acordado, con idempotencia y ACK posterior al commit. No hay transacción SQL distribuida ni llamadas a proveedores dentro de una transacción.
4. Mantener en PLATFORM la facturación SaaS, suscripciones y políticas comerciales de ENSAMBRIA. La propuesta no traslada cuentas, proveedores, documentos fiscales ni obligaciones SaaS a BUSINESS.
5. Tratar un asiento del libro contable tenant como representación contable de un hecho empresarial referenciado por contrato, no como una segunda operación de pago. Cada asiento conserva origen, moneda, vigencia y versión; el registro transaccional de pago continúa siendo autoritativo en CORE. No se acepta contabilización duplicada del mismo efecto.
6. Si esta propuesta se acepta, revisar/sustituir explícitamente el alcance vigente de `employee-commissions`; hasta entonces, ese ownership aceptado de BUSINESS permanece vigente y no se presume una separación de contexto ya aprobada.

## Alternativas consideradas

1. **Asignar toda la contabilidad tenant a CORE:** se descarta como propuesta porque acoplaría reglas contables y obligaciones específicas de los establecimientos al procesamiento de pagos y ledger transaccional central de CORE.
2. **Asignar finanzas o nómina a PLATFORM:** se descarta porque PLATFORM es dueño de la relación comercial SaaS, no de libros, personal u obligaciones privadas de un establecimiento.
3. **Crear un backend o despliegue Finance/Payroll separado:** se descarta porque la topología aprobada mantiene cuatro backends y el requerimiento no demuestra necesidad de un quinto.
4. **Mantener sin cambios todas las capacidades financieras tenant en el estado actual:** conserva los owners aceptados, pero no resuelve el nuevo alcance de libros, cuentas, tesorería y nómina.
5. **Ubicar la capacidad propuesta dentro de BUSINESS y conservar los efectos de pagos en CORE:** opción recomendada para ratificación; mantiene la operación tenant en su backend y los límites financieros centrales existentes.

## Consecuencias y límites propuestos

- El libro contable tenant debe mantener sus propias reglas, cierres y auditoría sin sustituir el ledger de CORE ni leer sus tablas. La relación entre importes, asientos y efectos de pago se resuelve por referencias y contratos versionados.
- Los datos de nómina y compensación requieren acceso tenant/empresa legal/sucursal mínimo, autorización RBAC/ABAC, auditoría, minimización, retención y controles de privacidad aplicables. No se infieren obligaciones legales, períodos, conceptos remunerativos ni plazos; las validaciones profesionales se registran por jurisdicción y tratamiento.
- Los pagos parciales/programados a proveedores o personas no quedan habilitados por esta propuesta. Requieren obligación autorizada, contratos de CORE, capacidades oficiales verificadas y pruebas de fallos/reintentos.
- La propiedad de facturas de proveedor y obligaciones de cuentas por pagar debe confirmarse junto con la frontera contable tenant antes de implementar ese comportamiento en compras.
- Las subfases actuales mantienen sus alcances: Fase 4.1 cubre pagos/caja y efectos CORE; 4.2 compras/inventario; 7 documentos e integraciones fiscales; 8 analítica/reporting; 10 consolidación. No se asigna ni se crea una fase de libro contable o nómina mediante este ADR propuesto; Producto debe actualizar el roadmap si ratifica el alcance.
- No se definen esquemas, tablas, endpoints, eventos, contratos Protobuf ni porcentajes/importes. Su diseño y aceptación corresponden a la fase aprobada y al owner de cada dato.

## Criterios de aceptación futuros

Si la propuesta se acepta, la fase propietaria deberá especificar y probar con dependencias reales:

1. Un único owner para cada cuenta, obligación, asiento, registro de nómina y hecho de pago; ninguna consulta o escritura SQL cruzada.
2. Moneda explícita e importes `NUMERIC` con precisión/escala definida; cada asiento y corrección es reproducible y conserva historial.
3. Duplicados, concurrencia, períodos cerrados, permisos insuficientes, empresa/sucursal ajena, sesión expirada y fallo de persistencia no generan doble pago, doble contabilización ni efectos parciales.
4. Workforce entrega solo datos mínimos autorizados; cálculo y aprobación de nómina/comisiones usan una versión reproducible; un reintento no vuelve a liquidar ni pagar.
5. CORE registra el efecto de pago una sola vez; la propagación durable al owner consumidor tolera duplicados, reinicio, replay y entrega fuera de orden según contrato, con recuperación verificable.
6. Reportes y exportaciones aplican scopes por tenant/empresa/marca/sucursal, indican moneda y frescura y concilian con sus fuentes autorizadas.
7. Datos personales/financieros, documentos y telemetría se minimizan, autorizan, auditan y retienen conforme a las fuentes aplicables; no se afirma aprobación legal sin evidencia.

Estos criterios son diseño futuro. No hay implementación, esquema, contrato, proveedor ni prueba de aceptación demostrados por este ADR.

## Pendientes para ratificar o rechazar

| Decisión | Responsable | Condición |
|---|---|---|
| Aceptar/rechazar BUSINESS como owner del libro contable tenant, tesorería y cuentas por cobrar/pagar; definir sus límites con CORE. | Propietario/Producto y owners de BUSINESS/CORE. | No asignar módulo, esquema ni fase de implementación hasta ratificar la propuesta. |
| Aceptar/rechazar la capacidad Finance/Payroll dentro de BUSINESS y su responsabilidad por liquidación de nómina, bonos y comisiones; resolver el alcance vigente de `employee-commissions`. | Propietario/Producto y owners de BUSINESS/Workforce; revisión profesional para los tratamientos/jurisdicciones que la requieran. | No mover ownership ni habilitar cálculos/desembolsos antes de una decisión aceptada y las validaciones aplicables. |
| Definir secuencia, fase, casos de uso, entidades legales, períodos, autorizaciones, contratos de integración, retención y aceptación. | Producto y owners de cada dominio; Legal/Privacy Owner según tratamiento. | Incorporar las decisiones y pruebas en el roadmap y documentos de fase antes del corte dependiente. |

La presencia de esta propuesta en el repositorio o en un PR no equivale a ratificación. ADR-0012/0013/0014 y el ownership aceptado siguen vigentes hasta que una decisión aprobada los complemente o sustituya explícitamente.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios](../arquitectura-dominios.md)
- [ROADMAP.md](../../ROADMAP.md)
- [ADR-0007](ADR-0007-separacion-contextos-facturacion-saas.md)
- [ADR-0012](ADR-0012-jerarquia-empresarial-contexto-operativo.md)
- [ADR-0013](ADR-0013-autorizacion-rbac-abac-y-acceso-soporte.md)
- [ADR-0014](ADR-0014-coordinacion-ventas-pagos-inventario.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
