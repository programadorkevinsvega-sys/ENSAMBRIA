# Política inicial de retención y eliminación de datos

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** diseño interno preliminar de Fase 0, no política pública ni calendario de retención aprobado. No hay runtime de datos de ENSAMBRIA. Ningún plazo numérico de conservación se establece aquí; los plazos, excepciones fiscales/contractuales y legal holds requieren revisión de Legal y, cuando aplique, Finanzas por jurisdicción y tratamiento.

Esta política se relaciona con [PDLM](../rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md), el [registro de tratamientos](DATA_PROCESSING_REGISTER.md) y [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md). No reemplaza una obligación legal validada ni autoriza a eliminar o conservar datos reales.

## Regla de configuración

Cada categoría debe tener una política versionada con: identificador, owner, tratamiento/finalidad, categoría de datos, jurisdicción, base o instrucción contractual validada, inicio del plazo, duración o criterio, acción al vencer, excepciones, responsables, método de verificación, dependencias/derivados, fecha de aprobación y evidencia. El sistema no debe inferir la duración desde una tabla genérica ni aplicar borrado por edad del registro sin evaluar su finalidad y retenciones.

## Inventario inicial de categorías

| Categoría prevista | Owner | Disparador a definir | Acción propuesta a validar | Sistemas/derivados que deben cubrirse |
|---|---|---|---|---|
| Cuenta, identidad, credenciales y sesiones | CORE | Cierre de cuenta, expiración, revocación o solicitud | Revocar sesión/credencial; suprimir o desvincular según obligación aprobada | Replicas, tokens, audit trail minimizado, correo/SMS de identidad |
| Reservas y estado de operación | CORE; información operativa de BUSINESS | Fin del servicio, cancelación, expiración de relación o solicitud | Eliminar, anonimizar o conservar restringido conforme a política por dato | Proyecciones, notificaciones, búsqueda, caché y respaldos |
| Operación/CRM de establecimiento, clientes, workforce, inventario, pedido/Commerce | BUSINESS | Relación comercial, cierre de cuenta/tenant, fin de finalidad o solicitud | Owner determina tratamiento; no borrar tablas ajenas ni afectar integridad de transacción | PUBLIC si fue publicado, reportes, objetos, eventos, proveedor |
| Pagos, ledger, liquidaciones, conciliación y referencias de facturación | CORE; documentos comerciales pertenecen al owner correspondiente | Fecha/estado de transacción, cierre, reclamo o vencimiento legal | Restricción y minimización si hay obligación de conservar; no fijar plazo sin validación | Proveedor de pago, conciliación, facturas, logs y backups |
| Planes, suscripciones, facturas SaaS y administración | PLATFORM | Fin de contrato, cancelación, factura o reclamación | Retención limitada y acceso restringido por finalidad validada | CORE para efecto financiero; proveedor contable/fiscal si se aprueba |
| Perfiles publicados, reseñas, fotos y atribución | PUBLIC; fuente editorial de BUSINESS | Retirada, revocación, expiración de publicación, cierre/solicitud | Retirar de superficie pública e indexación; resolver retención de evidencia por separado | CDN, sitemap, buscadores, backups, medios y analítica |
| Marketing, consentimiento y preferencias | Owner según finalidad: BUSINESS, CORE o PLATFORM | Retirada, baja, expiración de finalidad o fin contractual | Detener tratamiento dependiente; conservar solo la supresión mínima si existe base validada para no recontactar | Proveedores de mensajería, listas de supresión y colas |
| Solicitudes y evidencia de privacidad | CORE coordina; cada owner conserva su atestación | Cierre, rechazo, retención parcial o vencimiento de plazo aprobado | Minimizar y conservar por período validado; no incluir datos eliminados en la evidencia | Auditoría, documentos de respuesta y registros de ejecución |
| Logs, trazas, Sentry y auditoría de seguridad/negocio | Cada productor; destino externo pendiente | Caducidad operativa o evento de investigación/hold autorizado | Retención diferenciada y acotada; redacción; no usar telemetría como archivo comercial | Exportadores, backups, búsqueda de logs y proveedores |
| Archivos, exportaciones y adjuntos | Backend dueño del objeto | Expiración del link/paquete, cierre del caso o solicitud | Acceso privado, expiración automática y borrado verificable | Versiones de objeto, CDN, links firmados y backups |

Las filas son categorías propuestas, no calendarios aprobados. Si una categoría incluye datos sensibles, de menores, de pago, financieros o de terceros, el owner debe documentar su alcance y bloquear el uso hasta validar necesidad, base, controles y plazo.

## Controles de ciclo de vida

1. Retirar acceso y publicación al alcanzar el evento de vencimiento, sin confundir retirada pública con eliminación de toda copia.
2. Ejecutar supresión/anónimo/restricción solo mediante el backend dueño, con autorización, idempotencia, transacciones cortas y evidencia minimizada.
3. Cubrir Outbox/Inbox, streams, cachés Redis con TTL, índices, objetos, exportaciones, proveedores y backups según el inventario real.
4. No incluir PII directa ni secretos en eventos, logs, trazas, métricas o evidencia de eliminación.
5. Si un legal hold válido existe, limitarlo por sujeto/scope, finalidad, responsable, vigencia y revisión. Su aprobación corresponde a Legal, no al agente ni a un sistema automático.
6. La restauración debe ocurrir primero en cuarentena; aplicar supresiones/holds vigentes y verificar antes de admitir tráfico.
7. Eliminar o anonimizar datos derivados según la política aprobada; un backup sujeto a ciclo de expiración debe describirse con transparencia, no prometer borrado instantáneo.

## Bloqueos de aceptación

Antes de configurar períodos deben conocerse las jurisdicciones y roles legales, finalidades, obligaciones de conservación, contrato con cada establecimiento, subencargados, ubicaciones, backups y necesidad de evidencia. Hasta entonces no se inventan números ni se procesa una solicitud real como si el plazo hubiese sido validado. La prueba futura debe demostrar expiración, concurrencia, fallos/reintentos, retención autorizada, aislamiento de tenant, eliminación de derivados y restauración en cuarentena.
