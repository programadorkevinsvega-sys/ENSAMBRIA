# Aviso de privacidad de ENSAMBRIA (borrador interno)

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado del contenido:** preparación documental incompleta; no usar esta versión como el aviso final de una operación real. Faltan hechos de la entidad operadora, contactos, finalidades efectivas, proveedores, jurisdicciones, plazos y canales. No hay aplicaciones de ENSAMBRIA ejecutables ni tratamiento runtime verificado. Esta limitación deriva de hechos y alcance de producto aún no confirmados, no de la falta de revisión de un abogado.

La investigación, redacción e integración técnica no esperan una revisión profesional. Una vez completo con hechos verificables, este documento puede integrarse y mostrarse con el aviso visible de revisión jurídica pendiente definido en la política normativa; la consulta del documento no implica que esté aprobado. La revisión profesional se registra por versión, jurisdicción y tratamiento como estado separado. No recolectar datos reales bajo una finalidad, rol o base no determinados, y mantener deshabilitado solo el tratamiento que dependa de una validación jurídica previa concreta.

## 1. Identidad y alcance del responsable

**ENSAMBRIA es el nombre del producto/proyecto.** La persona jurídica que operará el servicio, su domicilio, datos registrales y canales oficiales de privacidad todavía no están confirmados. No publicar este documento hasta insertar información oficial y definir qué entidad determina finalidades/medios para cada tratamiento. Un establecimiento que usa el producto puede tener responsabilidades propias sobre la información que gestiona; el rol de ENSAMBRIA puede variar según la finalidad y debe validarse, no inferirse de esta descripción.

## 2. Información que el diseño contempla

Según las funciones que finalmente se aprueben, la plataforma podría tratar:

- identidad de cuenta, contacto, autenticación, membresía, organización y permisos;
- reservas, estados operativos y referencias financieras mínimas;
- información de operación del establecimiento, workforce, catálogo, CRM, pedidos, inventario y facturación;
- relación comercial SaaS, suscripciones, soporte y administración;
- perfiles comerciales, contenido, reseñas y datos necesarios para publicar una ficha autorizada;
- solicitudes de privacidad, preferencias, consentimientos y evidencia;
- información técnica de seguridad, registros y diagnóstico minimizados;
- medios o texto libre que, por su contenido, podrían incluir información de terceros o categorías sensibles.

No se presume que ENSAMBRIA recopile datos de salud, biométricos, menores, documentos oficiales, ubicación precisa o tarjetas. Una función que los requiera permanece bloqueada hasta probar necesidad, definir controles y validar obligaciones.

## 3. Finalidades y reglas de uso

Las finalidades candidatas se limitan a: crear y proteger cuentas; autorizar operaciones; ejecutar reservas y operaciones del dominio; proporcionar la funcionalidad del establecimiento o del canal público; administrar contratos comerciales; proteger integridad y seguridad; responder solicitudes; y mantener disponibilidad/diagnóstico. Marketing, perfilado, analítica no esencial, replay, publicidad, entrenamiento de modelos o publicación de imagen no se incluyen como autorización general.

El inventario detallado, owners y estado de validación se mantienen en [DATA_PROCESSING_REGISTER.md](DATA_PROCESSING_REGISTER.md). Para cada tratamiento se documentan su finalidad, datos, actores, rol y fundamento según las fuentes aplicables; solo se mantiene deshabilitado el tratamiento que dependa de una validación jurídica previa concreta. Esa revisión no bloquea la investigación, documentación ni los flujos independientes.

## 4. Origen, destinatarios y proveedores

Las fuentes previstas son la interacción directa con la aplicación, la relación con un establecimiento y proveedores autorizados por un caso de uso. Los frontends deben llegar a su BFF autorizado; cada backend consulta y modifica únicamente los datos que posee.

No hay un catálogo de proveedores runtime aprobado. La infraestructura y adaptadores previstos, categorías de destinatarios, transferencias y bloqueos aparecen en [THIRD_PARTY_PROCESSORS.md](THIRD_PARTY_PROCESSORS.md) y [INTERNATIONAL_DATA_TRANSFERS.md](INTERNATIONAL_DATA_TRANSFERS.md). No se enviará información real a un proveedor hasta evaluar contrato, datos, propósito, acceso, ubicación, subprocesadores, retención y respuesta a incidentes.

## 5. Derechos, solicitudes y canales

El tipo de derecho, requisitos de identidad/representación, responsable de atenderlo, excepciones y plazos dependen de la jurisdicción y del tratamiento. No se publica un correo, dirección, formulario ni plazo porque no hay entidad ni canal legal aprobados. El diseño de atención coordinada está en [DATA_SUBJECT_REQUESTS.md](DATA_SUBJECT_REQUESTS.md); el canal debe completarse y validarse antes de recibir solicitudes reales.

## 6. Conservación y seguridad

Los plazos numéricos, obligaciones fiscales/contractuales, legal holds y período de backups no están aprobados. Los criterios por categoría están en [DATA_RETENTION.md](DATA_RETENTION.md); no prometer eliminación inmediata de copias de respaldo ni conservar indefinidamente.

El diseño contempla autorización del backend, límites tenant/sucursal, minimización, credenciales segregadas, cifrado, registros protegidos, recuperación y controles por proveedor. La existencia de reglas documentales no demuestra que esos controles estén implementados.

## 7. Cookies, analítica y marketing

No existe inventario de cookies o trackers runtime porque no hay frontends desplegados. La clasificación, banner, consentimiento y textos se definirán por mercado antes de publicación; ver [COOKIE_POLICY.md](COOKIE_POLICY.md) y [CONSENT_MANAGEMENT.md](CONSENT_MANAGEMENT.md). Sentry es un requisito de observabilidad frontend, pero no se ha configurado; los datos deben minimizarse y redactarse. Session Replay y captura de contenido quedan deshabilitados por defecto. Ver [SESSION_REPLAY_AND_ANALYTICS.md](SESSION_REPLAY_AND_ANALYTICS.md).

Las comunicaciones necesarias del servicio deben separarse de promociones. Marketing requiere su propio análisis, preferencias y mecanismo de retiro por canal; ver [MARKETING_COMPLIANCE.md](MARKETING_COMPLIANCE.md).

## 8. Menores, contenido público y transferencias

No se ha definido una edad mínima, autorización parental o servicio dirigido a menores. Tampoco existe flujo activo de publicación de contenido generado por usuarios. Ver [CHILDREN_AND_AGE_PROTECTION.md](CHILDREN_AND_AGE_PROTECTION.md) y [CONTENT_AND_COPYRIGHT.md](CONTENT_AND_COPYRIGHT.md); requieren decisión y revisión legal antes de operar.

El proveedor y región de hospedaje no están seleccionados y ninguna transferencia internacional está autorizada por este borrador. Las restricciones por ciudad/país no se infieren solo de la IP; revisar la política geográfica y la matriz legal.

## 9. Incidentes y cambios

El procedimiento técnico preliminar está en [INCIDENT_RESPONSE.md](INCIDENT_RESPONSE.md). No se define aquí un plazo universal de notificación ni se publica contacto operativo. Cualquier versión pública debe incluir la entidad/contacto, jurisdicción, texto aprobado, versión, vigencia y mecanismo de actualización.

## 10. Datos y revisiones pendientes

1. Identificar la entidad operadora, sus datos registrales y responsables de privacidad.
2. Completar y verificar los hechos de tratamiento por producto, actor, finalidad, proveedor y jurisdicción.
3. Determinar roles/finalidades/bases por cada fila del registro; no afirmar que están aprobados si no existe evidencia.
4. Completar proveedores, ubicaciones, transferencias, retención y subprocesadores antes de activar los tratamientos dependientes.
5. Definir los canales operativos de derechos e incidentes y los textos de consentimiento/cookies/marketing aplicables.
6. Implementar y probar los controles reales de backend/frontend, seguridad, eliminación y redacción en la fase propietaria.
7. Registrar la revisión profesional posterior por versión y completar [MARKET_LAUNCH_CHECKLIST.md](MARKET_LAUNCH_CHECKLIST.md) para cada tratamiento o mercado cuyo lanzamiento dependa de esas decisiones.

La matriz de leyes y fuentes oficiales está en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md); su cotejo oficial está incompleto. Este texto no está completo para su uso como aviso final hasta resolver los hechos indicados. La falta de revisión profesional, por sí sola, no impide integrarlo técnicamente ni mostrarlo con el aviso de revisión pendiente una vez completado. Ninguna integración o aceptación demuestra aprobación jurídica.
