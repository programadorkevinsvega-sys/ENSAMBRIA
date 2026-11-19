# Términos de suscripción SaaS para establecimientos (borrador temporal)

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **BORRADOR INTERNO — NO OFERTAR, FIRMAR NI COBRAR.** Versión 0.1, 2026-11-10. No hay entidad operadora, planes, precios, moneda, periodicidad, pruebas gratuitas, renovación, impuestos, política de cancelación o proveedor de facturación aprobados. Este texto no constituye una oferta ni asesoría jurídica. Requiere revisión de Producto, Finanzas y abogado competente en Colombia.

Este borrador se prepara y versiona desde Fase 0 para integrarse en entornos no productivos en el recorrido comercial correspondiente. No se muestra como oferta ni se acepta en el simple alta de autoservicio del tenant: ADR-0002 establece que la relación comercial inicial queda `onboarding_pending` y no asigna plan, precio, trial, cobro ni entitlement. La oferta y aceptación contractual solo podrán habilitarse con condiciones aprobadas; la integración preparatoria no se aplaza por esa revisión.

## 1. Partes y finalidad

El operador jurídico de ENSAMBRIA y sus datos de identificación permanecen `[PENDIENTES]`. La parte cliente será la persona jurídica o comerciante que contrate la suscripción, identificada con datos que se validarán al activar el flujo. El administrador de un tenant no representa automáticamente a todas las sucursales ni a las personas cuyos datos cargue.

Este borrador contempla acceso SaaS a herramientas de administración de establecimientos. No regula la venta del establecimiento a sus clientes, la prestación de servicios de belleza/salud/bienestar ni pagos de reservas; esos contratos requieren condiciones separadas y deben identificar el vendedor real.

## 2. Oferta, precio, duración y renovación

Antes de contratar deben mostrarse de manera comprensible: capacidades incluidas y límites; plan; precio y moneda; impuestos/cargos; periodicidad; fecha de inicio y fin; condiciones de renovación; cambios de precio; período de prueba solo si fue aprobado; forma de pago; facturación; consecuencias de fallos de cobro; cancelación y reembolso.

Ningún dato se completa por inferencia. `Plan`, `precio`, `trial`, `renovación automática`, preaviso, tratamiento de mora, suspensión y reembolso permanecen pendientes de decisión. No se iniciará un cobro recurrente sin contrato, aceptación y configuración del proveedor revisados.

## 3. Cuenta, administración y uso

El cliente designará administradores autorizados, controlará membresías y sucursales y comunicará cambios. ENSAMBRIA aplicará autorización tenant-scoped conforme a la arquitectura; ningún tenant obtiene acceso a otro por compartir infraestructura. Los permisos, límites de usuarios, uso aceptable, soporte y medidas ante abuso deben coincidir con las capacidades reales.

El establecimiento mantiene responsabilidad por la exactitud y licitud de sus instrucciones y contenidos, sin que esta cláusula elimine obligaciones propias que ENSAMBRIA pueda tener como proveedor, responsable o encargado según el flujo.

## 4. Datos del establecimiento y privacidad

El reparto entre responsable y encargado depende de quién determine finalidad y medios esenciales para cada tratamiento. No se presume que el establecimiento sea siempre responsable ni que ENSAMBRIA sea siempre encargado.

Cuando se confirme una relación responsable-encargado, el contrato deberá adjuntar el [Acuerdo de tratamiento de datos](DATA_PROCESSING_AGREEMENT.md), inventario de datos/instrucciones, medidas, subencargados, ubicaciones, asistencia a derechos, incidentes, transferencias, devolución/eliminación y condiciones de auditoría. Sin esos anexos aprobados no cargar datos personales reales.

## 5. Disponibilidad, cambios y soporte

No se establecen SLA, disponibilidad, RPO/RTO, horario de soporte, tiempos de respuesta ni compensaciones. Solo se incluirán después de aprobar capacidad y operación medibles. Cambios de producto, mantenimiento y deprecación deben informarse según materialidad y contrato final, sin reducir derechos irrenunciables.

## 6. Suspensión, terminación y portabilidad

Los supuestos de incumplimiento, fraude, riesgo de seguridad, impago, aviso, período de subsanación, terminación inmediata, exportación, acceso de solo lectura y devolución de datos requieren decisión de Producto/Legal. Deben distinguirse restricciones de seguridad urgentes de terminación comercial; las acciones se registran y se limitan al tenant afectado.

Al finalizar, cada dominio ejecuta la política de retención aprobada. No se prometen plazos ni borrado instantáneo de backups hasta validar obligaciones fiscales, evidencia y ciclo de vida de proveedores.

## 7. Facturación, pagos y responsabilidades comerciales

La parte que factura, emisor tributario, receptor de fondos, proveedor de pago, reversiones y disputas deben definirse por operación. La suscripción SaaS de PLATFORM no se mezcla con pagos de clientes finales de BUSINESS ni con pagos/ledger de CORE. El establecimiento no debe interpretar una referencia de pago como confirmación de una obligación que el flujo no valida.

## 8. Reclamos, ley y límites

Se deben insertar canales de soporte/reclamación, entidad, ley y foro solo tras revisión legal. No se incluyen renuncias generales a derechos de consumidores, responsabilidad legal, privacidad, seguridad o protección de datos. Cualquier límite contractual debe ser proporcional y validado.

## 9. Registro de aprobaciones requerido

| Decisión | Responsable | Evidencia |
|---|---|---|
| Entidad contratante y autoridad del representante | Legal/representante | Datos registrales y flujo de representación |
| Planes, precios, moneda, impuestos, trial y cobros | Producto/Finanzas | Catálogo aprobado y prueba de checkout |
| Renovación, baja, impago, reembolso y facturación | Producto/Finanzas/Legal | Contrato y escenarios aceptados |
| Rol y obligaciones de datos por tratamiento | Privacidad/Legal | Mapa de roles, DPA y subencargados |
| SLA, suspensión, exportación y borrado | Producto/Operaciones/Legal | Capacidad medida, política aprobada y pruebas |
| Ley/foro y obligaciones de consumo | Abogado competente | Revisión normativa y cláusulas finales |

No utilizar este borrador con clientes ni como pantalla de aceptación hasta contar con revisión jurídica, ratificación comercial y evidencia de implementación.
