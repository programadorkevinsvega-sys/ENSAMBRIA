# Diseño inicial de comunicaciones y marketing

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** diseño preliminar; no hay campañas, canales, proveedores ni datos de contacto cargados. Para Colombia se verificó el texto de la Ley 2300 de 2023 en el Normograma oficial de MinTIC; su aplicación al emisor, canal, relación y mensaje concreto requiere revisión legal. No hay autorización para enviar comunicaciones.

## Ownership por propósito

| Comunicación | Owner | Regla de límite |
|---|---|---|
| Seguridad, identidad y acceso | CORE | Solo mensaje necesario para la operación de cuenta/seguridad; no insertar promoción. |
| Recordatorios y marketing de un establecimiento | BUSINESS | El establecimiento define su propósito dentro de controles de la plataforma; autorización, preferencias, lista de supresión y límites deben resolverse por mercado. |
| Cuenta SaaS, suscripción y relación comercial ENSAMBRIA | PLATFORM | Mantener comunicaciones de servicio separadas de ofertas comerciales y transparentar renovaciones/cancelaciones. |
| Confirmación o cambio de reserva/pedido | Backend propietario del estado; CORE para evento transaccional central | El mensaje no debe convertir silenciosamente una notificación necesaria en publicidad. |

El proveedor y su rol deben registrarse en [THIRD_PARTY_PROCESSORS.md](THIRD_PARTY_PROCESSORS.md). El front no decide autorización; cada envío valida preferencias/scope en el backend propietario.

## Requisitos de diseño

- Separar correo, SMS, push y canales futuros; guardar la prueba de preferencia y la versión de su aviso cuando sea necesaria.
- Mantener mensajes operativos separados de marketing en modelo, plantilla, propósito, audiencia, métrica y logs.
- Respetar baja/supresión en cada proveedor y evitar reintroducir contactos mediante importación, sincronización, replay o restauración.
- Identificar emisor, propósito y mecanismo de baja según la obligación validada; no diseñar patrones oscuros ni obstaculizar cancelación.
- Definir alcance tenant/sucursal, frecuencia, límites antiabuso, quiet hours y acceso a listas; no enviar a otra cartera o tenant.
- Minimizar payload y evitar datos sensibles o contenido innecesario en subject, preview, push y logs.
- Registrar fallos y reintentos sin exponer direcciones, tokens o contenido en métricas/logs; tratar callbacks en el owner correspondiente con verificación oficial.

## Alcance legal preliminar para Colombia

El texto oficial de la [Ley 2300 de 2023](https://normograma.mintic.gov.co/mintic/compilacion/docs/ley_2300_2023.htm) se consultó con conexión TLS válida el 10 de noviembre de 2026. La ley trata en sus artículos 1–4 canales, horario, periodicidad y contacto de cobranza; el artículo 5 extiende sus reglas a relaciones comerciales de productores/proveedores con consumidores en el envío de comunicaciones comerciales/publicitarias. El parágrafo 2 del artículo 5 prohíbe condicionar una transacción a aceptar mensajes comerciales, salvo asuntos estrictamente relacionados con el bien o servicio; requiere aceptación explícita para promociones que alimenten bases de datos y un mecanismo ágil, sencillo y eficiente de cancelación. El parágrafo 3 remite las comunicaciones publicitarias a los horarios del artículo 3.

El artículo 3 fija para las prácticas de cobranza lunes a viernes de 7:00 a. m. a 7:00 p. m. y sábados de 8:00 a. m. a 3:00 p. m., excluidos domingos y festivos; el parágrafo 3 del artículo 5 remite a esos horarios para los mensajes y llamadas publicitarias enumerados. El producto no automatizará supuestos sobre excepciones, frecuencia u horario hasta que Legal confirme la vigencia consolidada, el actor, el tipo de relación, el mensaje, canal y destinatario. Los mensajes estrictamente operativos se clasificarán con cuidado y no se etiquetarán como transaccionales si contienen promoción.

El derecho de retiro de autorización y las obligaciones generales de datos personales se revisan además bajo Ley 1581/2012 y su reglamentación vigente. No se convierte este análisis en una regla universal de opt-in para todo canal ni se infiere que una reserva autorice publicidad. La [matriz Colombia](LEGAL_JURISDICTIONS.md) registra fuentes, límites de acceso y decisiones pendientes.

## Bloqueos y pruebas

No enviar comunicaciones de marketing hasta que un abogado competente confirme la interpretación aplicable, y se aprueben base/consentimiento, texto, jurisdicción, proveedor, DPA/transferencia y baja. Las pruebas futuras cubren consentimiento inexistente/retirado, canal no autorizado, restricciones de horario aplicables, destinatario de otro tenant, duplicado, concurrencia, proveedor caído, webhook falso, baja propagada y ausencia de mensaje promocional en correo transaccional. Usar sandbox oficial y direcciones de prueba controladas, no contactos productivos.

El mapeo de obligaciones oficiales y sus bloqueos vive en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md); las políticas de consentimiento y cookies están en [CONSENT_MANAGEMENT.md](CONSENT_MANAGEMENT.md) y [COOKIE_POLICY.md](COOKIE_POLICY.md).
