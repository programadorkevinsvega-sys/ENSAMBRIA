# Diseño inicial de transferencias y acceso transfronterizo

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** evaluación de arquitectura de Fase 0, no registro de transferencias activas ni aprobación para alojar/procesar datos fuera de Colombia. No se eligieron proveedor, región, entidad responsable, ubicaciones de soporte ni subencargados. Las rutas jurídicas concretas deben validarse por abogado competente antes de habilitar una transferencia.

Esta matriz se relaciona con [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md), [inventario de tratamientos](DATA_PROCESSING_REGISTER.md), [proveedores](THIRD_PARTY_PROCESSORS.md) y [política legal](../rules/ENSAMBRIA_Privacy_Legal_Compliance.md).

## Rutas técnicas previsibles (no habilitadas)

| Ruta | Datos potenciales | Estado actual | Criterio antes de habilitar |
|---|---|---|---|
| Navegador/app → frontend/BFF/backend | Cuenta, solicitud, operación y metadatos de red | No existen aplicaciones ejecutables; hospedaje/terminación TLS desconocidos | Definir ubicación, finalidad, logs, acceso de soporte y jurisdicción; no usar IP como prueba definitiva de residencia. |
| Backend → PostgreSQL compartido | Datos del schema y referencias del owner | No hay base/servidor desplegado | Determinar región, acceso, backup y DR; mapear datos por owner y revisar transferencia por acceso remoto. |
| Backend → Redis/NATS/observabilidad | Claves TTL, hechos minimizados, métricas/logs/trazas | Infraestructura no desplegada | Clasificación/payload minimizado, región, retención y ACL por entorno/workload; no PII/secretos en fan-out/logs. |
| Backend → almacenamiento R2 u otro objeto | Archivos, exportaciones e imágenes | Cloudflare R2 es adapter inicial previsto en la arquitectura, no configurado | Confirmar contrato, regiones, subprocesadores, acceso, lifecycle, borrado y salvaguardas antes de enviar archivos reales. |
| Frontend → Sentry/analítica | Errores, release, datos técnicos potencialmente identificables | Sentry frontend requerido por norma; proyecto/región/contrato/configuración no verificados; analítica/replay sin aprobación | Minimizar/redactar, validar ubicación/retención, transferencias, términos, DPA y acceso; replay permanece deshabilitado. |
| Servicio → email/SMS/pagos/otros | Contacto, operación o referencia financiera | Wompi y Factus son proveedores iniciales previstos; no hay integraciones ni ubicaciones/regiones verificadas. Otros proveedores aún no seleccionados. | Evaluación de subencargado y sandbox, datos transmitidos, ubicación, eliminación y base contractual/legal. |

## Reglas de diseño

- Registrar acceso remoto de soporte como posible flujo transfronterizo a evaluar.
- No inferir residencia o nacionalidad por IP, locale, tarjeta o dirección aislada.
- Separar región de origen, región de procesamiento, región de backup, soporte y subprocesadores.
- Enviar el mínimo necesario por contrato; preferir referencias opacas y prohibir PII directa en eventos de fan-out.
- No escoger mecanismo jurídico de transferencia ni prometer residencia/localización hasta validación.
- Mantener regiones, contratos, aprobaciones, versiones y fechas en el registro legal; revisar cambios de subprocesador o ubicación antes de desplegar.

## Bloqueo de mercado

Colombia es mercado inicial de diseño en el roadmap, pero no una autorización legal o comercial para procesar datos en cualquier región. Antes de activar el tratamiento se deben documentar fuente oficial vigente, aplicabilidad, rol, destinatario, mecanismo, control, prueba, responsable y aprobación. Ninguna transferencia internacional se considera aprobada por la mera disponibilidad de una región o contrato de proveedor.
