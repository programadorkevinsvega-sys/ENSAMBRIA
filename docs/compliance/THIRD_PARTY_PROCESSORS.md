# Registro inicial de proveedores y subencargados

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** inventario de categorías y proveedores previstos; no hay integraciones de proveedores externas verificadas en runtime. La mención de un producto o adapter no acredita cuenta, contrato, región, DPA, configuración ni aprobación. No cargar datos personales ni secretos hasta evaluar finalidad, contrato, seguridad, retención, ubicación y supresión con Legal y el propietario técnico.

## Inventario provisional

| Servicio/categoría | Owner técnico previsto | Datos potenciales | Estado/evaluación requerida |
|---|---|---|---|
| PostgreSQL administrado/host | Cada backend con schema/rol propio | Datos autoritativos del dominio | Proveedor, región, versión, backup, soporte y contrato no seleccionados; topología de base lógica compartida definida. |
| Redis | Cada owner que use caché TTL | Proyecciones/IDs de caché, potencialmente identificables por enlace | No desplegado; política permite solo caché con TTL y requiere claves minimizadas/aislamiento. |
| NATS JetStream/Core | Owners productores/consumidores | Eventos minimizados, IDs, scope y correlación | No desplegado; configuración, ubicación, retención y ACL pendientes; no enviar PII o secretos. |
| Vault / PKI | Seguridad/infraestructura por workload | Secretos y certificados | Norma prevé AppRole, ACL, KV v2 y PKI por entorno/workload; no se configura ni accede a recursos compartidos en Fase 0. |
| Cloudflare R2 / API compatible S3 | Módulo dueño de archivos | Imágenes, adjuntos, exportaciones | Adapter inicial indicado por arquitectura; no hay cuenta, bucket, ubicación, contrato, DPA o borrado verificados. |
| Sentry | Cada frontend por aplicación/entorno | Excepciones, metadatos técnicos y posible dato personal accidental | Sentry es obligatorio para frontends; no se han creado proyectos ni validado región/contrato, accesos, retención, redacción o exportación. Replay deshabilitado por defecto. |
| Pasarela de pagos | CORE para pagos centrales | Referencia tokenizada, importe, estado y metadatos necesarios | Wompi es el proveedor inicial previsto, no exclusivo; no hay integración/configuración verificada. Revisar PCI/alcance, sandbox, webhooks, subprocesadores y retención con documentación oficial. |
| Facturación SaaS/establecimiento | PLATFORM/BUSINESS según dominio | Identidad comercial, factura y estado | Factus es el proveedor inicial previsto, no exclusivo; PLATFORM puede comercializar capacidad Factus. No hay integración/configuración verificada; evaluar regulación fiscal, contrato/condiciones de reventa, región, API, callbacks y eliminación. |
| Email/SMS/push | Owner por propósito (CORE, BUSINESS o PLATFORM) | Contacto, plantilla, preferencia y estado de entrega | Proveedores no elegidos; evaluar consentimiento/opt-out, ubicación, supresión y contratos. |
| CDN, hosting, mapas, fuentes, analítica | Owner frontend / infraestructura | IP, contenido público o datos de navegación | No definidos; evitar recursos externos no evaluados y tracking/replay sin aprobación. |

## Condiciones mínimas de alta

Antes de enviar datos a un proveedor, registrar: nombre jurídico y servicio, owner, finalidad, sujetos/categorías, campos exactos, base/rol, regiones y transferencias, subprocesadores, cifrado, controles de acceso, retención, borrado, DPA/contrato, incidentes, sandbox, prueba, fecha/revisor y decisión de uso. La clasificación técnica como “processor” requiere validación legal; el título de esta matriz no asigna automáticamente ese rol.

Los webhooks deben quedar bajo el backend dueño del caso de uso y verificarse con documentación oficial y sandbox. Mantener credenciales, callbacks, auditoría e historial separados por provider/caso. Ningún adapter puede introducir reglas de negocio ni conceder acceso a tablas de otros módulos.

## Cierre y revisión

Un proveedor se habilita solo tras aceptación documentada y pruebas de integración reales. Al cambiar servicio, ubicación, finalidad o subprocesadores, reevaluar. El registro de rutas y aprobación por jurisdicción está en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md); las transferencias se detallan en [INTERNATIONAL_DATA_TRANSFERS.md](INTERNATIONAL_DATA_TRANSFERS.md).
