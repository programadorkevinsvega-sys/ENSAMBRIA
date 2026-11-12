# ENSAMBRIA — Estándar global de observabilidad, monitoreo, alertas, SLI, SLO y gestión de errores

**Archivo:** `ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md`<br>
**Categoría:** Arquitectura, infraestructura, confiabilidad y operación<br>
**Aplicación:** Todos los backends, frontends, workers, bases de datos, servicios, integraciones e infraestructura de ENSAMBRIA.<br>
**Vigencia:** Obligatoria desde la Fase 0.<br>
**Prioridad:** Requisito transversal de arquitectura y producción.

La Fase 0 de ENSAMBRIA es documental: define arquitectura, ownership, responsables, decisiones, riesgos, criterios de aceptación y pruebas de observabilidad; no implementa ni despliega runtime, instrumentación o infraestructura de telemetría. La implementación verificable comienza en el primer corte ejecutable de Fase 1 y acompaña cada comportamiento y componente que se incorpore, sin posponer controles aplicables a fases transversales posteriores.

Esta guía es la fuente normativa de observabilidad, monitoreo, alertas, SLI/SLO, gestión de errores y operación. Se aplica junto con [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](ENSAMBRIA_Privacy_Data_Lifecycle_Management.md) y [ENSAMBRIA_Privacy_Legal_Compliance.md](ENSAMBRIA_Privacy_Legal_Compliance.md), que conservan la autoridad en ciclo de vida de datos y obligaciones de privacidad/cumplimiento, respectivamente. La instrumentación y la documentación no constituyen aprobación legal ni certificación.

---

# 1. Objetivo

ENSAMBRIA debe disponer de una plataforma integral de observabilidad que permita conocer el estado, comportamiento, rendimiento, disponibilidad y confiabilidad de todos sus componentes.

La observabilidad no debe limitarse a pagos, reservas, autenticación o un módulo específico.

Debe cubrir la plataforma completa:

- CORE BACKEND.
- BUSINESS BACKEND.
- PLATFORM BACKEND.
- PUBLIC BACKEND.
- Los seis proyectos de interfaz.
- Workers y procesos asíncronos.
- PostgreSQL.
- Redis.
- NATS JetStream.
- Object Storage.
- Vault.
- Sistemas de autenticación.
- APIs internas y externas.
- CDN y servicios de infraestructura.
- Proveedores de pagos.
- Proveedores de email y SMS.
- Servicios de IA.
- Integraciones de terceros.
- Máquinas virtuales y sistemas operativos.
- Redes y comunicaciones.
- Procesos de despliegue y CI/CD.

El objetivo es detectar, diagnosticar y resolver problemas antes de que se conviertan en incidentes graves o afecten a los usuarios.

La plataforma debe permitir responder:

- ¿Qué está fallando?
- ¿Desde cuándo?
- ¿A cuántos usuarios afecta?
- ¿Qué tenant está afectado?
- ¿Qué backend o frontend está involucrado?
- ¿Qué servicio está causando la degradación?
- ¿Qué petición originó el problema?
- ¿Qué consultas de base de datos están tardando?
- ¿Qué worker está acumulando retrasos?
- ¿Qué proveedor externo está fallando?
- ¿Qué cambió antes del incidente?
- ¿Cuál es el impacto real sobre los usuarios?
- ¿Estamos incumpliendo nuestros objetivos de disponibilidad?
- ¿Qué capacidad necesitamos para soportar el crecimiento?

**Principio fundamental:** ningún componente crítico de ENSAMBRIA debe operar en producción sin instrumentación, métricas, registros y mecanismos de diagnóstico apropiados para su naturaleza.

# 2. Alcance arquitectónico

La observabilidad debe ser transversal y respetar los límites de los cuatro backends.

## 2.1 CORE BACKEND

Instrumentar:

- Identidad.
- Autenticación.
- Autorización.
- Tenancy.
- Organizaciones.
- Sucursales.
- Entitlements.
- Reservas.
- Disponibilidad.
- Pagos.
- Idempotencia.
- Ledger.
- Comisiones.
- Liquidaciones.
- Conciliación.
- Sesiones.
- APIs internas.
- Comunicación gRPC.
- Operaciones de seguridad.

## 2.2 BUSINESS BACKEND

Instrumentar:

- Dashboard empresarial.
- Empleados.
- Profesionales.
- Horarios.
- Servicios.
- Clientes.
- CRM.
- Membresías.
- Inventario.
- Compras.
- Commerce.
- Pedidos.
- Marketing.
- Franquicias.
- Comisiones de empleados.
- Analítica operativa.
- BFF empresarial.
- Operaciones de cada vertical.

## 2.3 PLATFORM BACKEND

Instrumentar:

- Administración global.
- Gestión de tenants.
- Planes.
- Suscripciones SaaS.
- Facturación SaaS.
- Políticas comerciales.
- Entitlements comerciales.
- Soporte.
- Auditoría administrativa.
- BFF de administración.

## 2.4 PUBLIC BACKEND

Instrumentar:

- Directorio.
- Discovery.
- Marketplace.
- Establecimientos públicos.
- Perfiles.
- Búsquedas.
- Geolocalización.
- Reseñas.
- Indexación.
- Proyecciones públicas.
- Atribución.
- BFF de experiencias públicas.

## 2.5 Proyectos de interfaz

Instrumentar:

- `ensambria-business`
- `tenant-portal`
- `ensambria-app`
- `marketplace`
- `platform-admin`
- `corporate`

La observabilidad frontend debe permitir identificar errores de navegador, errores de renderizado, fallos de red, problemas de rendimiento y fallos de interacción.

## 2.6 Infraestructura

Instrumentar:

- CPU.
- RAM.
- Disco.
- IOPS.
- Espacio disponible.
- Red.
- Latencia.
- Procesos.
- Servicios.
- Conexiones.
- Certificados TLS.
- Estado de backups.
- Capacidad de bases de datos.
- Uso de pools.
- Colas.
- Brokers.
- Workers.
- CDN.
- Servicios externos.

# 3. Los pilares de observabilidad

ENSAMBRIA debe implementar los siguientes pilares.

## 3.1 Logs

Los logs responden:

**¿Qué ocurrió?**

Deben registrar eventos, errores, advertencias y operaciones relevantes.

Ejemplos:

- Una reserva fue creada.
- Una autenticación falló.
- Un pago fue rechazado.
- Una consulta tardó demasiado.
- Un worker agotó sus reintentos.
- Un proveedor externo no respondió.
- Un proceso se reinició.
- Un mensaje fue enviado a una DLQ.

Los logs deben ser estructurados y centralizados.

## 3.2 Métricas

Las métricas responden:

**¿Cómo se está comportando el sistema?**

Deben permitir observar tendencias y detectar anomalías.

Ejemplos:

- Peticiones por segundo.
- Tasa de errores.
- Latencia p50, p95 y p99.
- Conexiones activas.
- Pool de PostgreSQL.
- Memoria.
- CPU.
- Mensajes pendientes.
- Retrasos de workers.
- Errores de proveedores.
- Reservas completadas.
- Pagos exitosos.

Las métricas deben utilizar dimensiones controladas y cardinalidad limitada.

## 3.3 Trazas distribuidas

Las trazas responden:

**¿Por dónde pasó una operación y cuánto tardó cada componente?**

Una petición debe poder seguirse a través de varios backends, bases de datos, brokers y proveedores.

Ejemplo:

1. Cliente inicia una reserva.
2. ENSAMBRIA App o un cliente web autorizado envía una petición al BFF correspondiente.
3. PUBLIC resuelve la información pública.
4. BUSINESS consulta disponibilidad.
5. CORE valida la operación.
6. PostgreSQL persiste los datos.
7. Outbox registra el evento.
8. NATS distribuye el evento.
9. Un worker procesa la notificación.
10. El proveedor de email entrega el mensaje.

La traza debe permitir identificar qué componente consumió el tiempo y dónde se produjo el error.

## 3.4 Correlación

Toda operación distribuida debe tener contexto de correlación.

Se deben poder relacionar:

- Logs.
- Métricas cuando corresponda.
- Trazas.
- Errores de Sentry.
- Eventos.
- Peticiones HTTP.
- Llamadas gRPC.
- Mensajes de NATS.
- Procesos de workers.

No se debe utilizar un identificador de usuario o petición como etiqueta de métrica de alta cardinalidad.

## 3.5 Alertas

Las alertas responden:

**¿Cuándo necesitamos intervenir?**

Deben detectar situaciones que requieren atención.

No generar notificaciones por cada evento normal.

Priorizar:

- Impacto sobre usuarios.
- Degradación de servicios.
- Riesgo de pérdida de datos.
- Riesgo de seguridad.
- Incumplimiento de SLO.
- Agotamiento de recursos.
- Acumulación de trabajo.
- Fallos de infraestructura.

## 3.6 SLI, SLO y SLA

Estos conceptos deben diferenciarse.

- SLI: indicador que mide la calidad real del servicio.
- SLO: objetivo interno que ENSAMBRIA establece para ese indicador.
- SLA: compromiso contractual frente a clientes.

No convertir automáticamente un SLO interno en una garantía contractual.

# 4. Arquitectura de observabilidad

ENSAMBRIA debe adoptar una arquitectura de telemetría basada en estándares abiertos.

## 4.1 OpenTelemetry

OpenTelemetry será el estándar principal de instrumentación y propagación de telemetría.

Debe permitir recolectar:

- Logs.
- Métricas.
- Trazas.

Utilizar instrumentación compatible con los SDK y protocolos oficiales.

No acoplar la lógica de negocio a un proveedor comercial específico de observabilidad.

## 4.2 OpenTelemetry Collector

Implementar OpenTelemetry Collector como capa central de recepción, procesamiento y exportación de telemetría.

Responsabilidades:

- Recibir telemetría de los servicios.
- Recibir telemetría de los SDK.
- Procesar atributos.
- Enriquecer datos.
- Filtrar información sensible.
- Aplicar políticas de muestreo.
- Controlar volumen.
- Enviar datos a los destinos autorizados.
- Gestionar fallos de exportación.
- Evitar pérdida innecesaria de telemetría.

La arquitectura debe permitir desplegar Collectors de forma independiente cuando el volumen o los requisitos de disponibilidad lo justifiquen.

## 4.3 Separación de responsabilidades

La plataforma debe diferenciar:

| Componente | Responsabilidad |
|---|---|
| OpenTelemetry | Instrumentación y estándares de telemetría. |
| Collector | Recepción, procesamiento y exportación. |
| Sistema de logs | Almacenamiento y consulta de logs. |
| Sistema de métricas | Almacenamiento de series temporales. |
| Sistema de trazas | Almacenamiento y consulta de trazas. |
| Sentry | Excepciones y rendimiento de aplicaciones. |
| Grafana | Visualización y dashboards. |
| Alertmanager | Gestión y agrupación de alertas. |

Las herramientas concretas deben seleccionarse según infraestructura, coste, retención, volumen y operación real.

# 5. Stack tecnológico de referencia

La arquitectura debe favorecer estándares abiertos y evitar dependencias innecesarias.

Stack de referencia:

| Necesidad | Tecnología de referencia |
|---|---|
| Instrumentación | OpenTelemetry |
| Recolección | OpenTelemetry Collector |
| Métricas | Prometheus |
| Dashboards | Grafana |
| Alertas | Prometheus Alertmanager |
| Logs | Loki o plataforma equivalente |
| Trazas | Tempo o plataforma equivalente |
| Errores de aplicación | Sentry |
| Profiling | Pyroscope o herramienta equivalente |
| Infraestructura | Node Exporter y exporters específicos |
| PostgreSQL | Exporter y métricas nativas |
| Redis | Exporter y métricas nativas |
| NATS | Métricas y endpoints de monitoreo autorizados |
| Frontend | SDK de Sentry y OpenTelemetry cuando corresponda |

Este stack es una arquitectura de referencia, no una autorización para desplegar todas las herramientas sin evaluar recursos y necesidades.

No introducir componentes redundantes.

No instalar múltiples sistemas para resolver la misma necesidad sin justificación.

# 6. Instrumentación de backends Rust

Todos los backends Rust deben tener instrumentación consistente.

Tecnologías de referencia:

- Tokio.
- Axum.
- Tonic.
- SQLx.
- `tracing`.
- `tracing-subscriber`.
- Integración OpenTelemetry.
- Propagación de contexto W3C.

## 6.1 Logs estructurados

Utilizar logs estructurados.

Cada registro debe contener, cuando corresponda:

- Timestamp.
- Nivel.
- Servicio.
- Entorno.
- Versión.
- Módulo.
- Mensaje.
- Trace ID.
- Span ID.
- Request ID.
- Correlation ID.
- Tenant ID cuando sea necesario y seguro.
- Resultado.
- Código de error.

No incluir datos sensibles innecesarios.

## 6.2 Niveles

Utilizar apropiadamente:

- TRACE.
- DEBUG.
- INFO.
- WARN.
- ERROR.

No registrar todos los eventos de negocio como ERROR.

Un pago rechazado por una decisión válida del proveedor no es necesariamente un error técnico.

Un usuario que introduce una contraseña incorrecta tampoco debe generar automáticamente una alerta operativa.

## 6.3 Errores

Los errores deben clasificarse:

- Errores de validación.
- Errores de autorización.
- Errores de negocio.
- Errores de infraestructura.
- Errores de dependencia.
- Timeouts.
- Errores de persistencia.
- Errores de serialización.
- Errores de comunicación.
- Errores inesperados.

Los errores esperados deben registrarse sin generar ruido excesivo.

## 6.4 Instrumentación HTTP y gRPC

Instrumentar:

- Método.
- Ruta normalizada.
- Código de respuesta.
- Duración.
- Tamaño de respuesta cuando sea apropiado.
- Estado de cancelación.
- Timeout.
- Servicio de destino.

No utilizar URLs completas con identificadores variables como etiquetas de métricas.

Preferir rutas normalizadas:

`/api/bookings/{booking_id}`

No:

`/api/bookings/0192f3...`

# 7. Instrumentación de frontend

Los seis proyectos de interfaz deben disponer de observabilidad.

## 7.1 Sentry

Sentry debe utilizarse para:

- Excepciones JavaScript.
- Errores de React.
- Errores de renderizado.
- Errores de navegación.
- Fallos de red cuando corresponda.
- Problemas de rendimiento.
- Transacciones lentas.
- Regresión de errores.
- Seguimiento de releases.
- Source maps controlados.
- Contexto de trazas distribuidas.

Para Flutter deben utilizarse las capacidades específicas de su SDK en Android/iOS y, para Business, en Windows/Linux/macOS; para Next.js se instrumentan los entornos cliente/servidor que correspondan.

## 7.2 Captura de errores

Capturar errores relevantes de:

- Componentes.
- Rutas.
- Formularios.
- APIs.
- Navegación.
- Autenticación.
- Operaciones de reserva.
- Checkout.
- Carga de recursos.
- Integraciones.

No capturar indiscriminadamente cada error esperado de validación.

## 7.3 Source maps

Los source maps deben permitir identificar el código original asociado a un error.

Reglas:

- Generarlos durante el build.
- Subirlos a Sentry mediante un proceso seguro.
- No publicar source maps privados innecesariamente.
- No incluir secretos.
- Asociarlos al release exacto.
- Verificar que la versión desplegada coincide.

## 7.4 Releases

Cada frontend debe informar:

- Nombre del servicio.
- Entorno.
- Versión.
- Release.
- Identificador de build.
- Plataforma.
- Información de despliegue necesaria.

Debe ser posible relacionar un aumento de errores con una versión concreta.

## 7.5 Privacidad

Sentry debe configurarse para no capturar:

- Contraseñas.
- Tokens.
- Cookies privadas.
- Datos de tarjetas.
- Datos bancarios.
- Información médica.
- Datos sensibles.
- Formularios completos.
- Información privada innecesaria.

Aplicar redacción y exclusión de campos.

No activar Session Replay de forma indiscriminada.

Si se utiliza Session Replay:

- Evaluar la jurisdicción.
- Determinar la base jurídica.
- Obtener consentimiento cuando corresponda.
- Enmascarar información sensible.
- Definir retención.
- Restringir acceso.
- Documentar al proveedor.

## 7.6 Integración con backend

Cuando una operación frontend llegue al backend, debe poder relacionarse con la traza correspondiente.

El contexto debe propagarse mediante los estándares aprobados.

Sentry no debe convertirse en un sistema aislado sin relación con las trazas distribuidas.

# 8. Instrumentación de workers y procesos asíncronos

Los workers deben tener observabilidad equivalente a los servicios HTTP.

Instrumentar:

- Ejecuciones.
- Duración.
- Estado.
- Reintentos.
- Errores.
- Timeouts.
- Mensajes procesados.
- Mensajes fallidos.
- Mensajes pendientes.
- DLQ.
- Backlog.
- Throughput.
- Consumo de recursos.
- Heartbeats.
- Leases.
- Checkpoints.

## 8.1 NATS JetStream

Observar:

- Mensajes publicados.
- Mensajes entregados.
- Mensajes confirmados.
- Redeliveries.
- Mensajes pendientes.
- Consumer lag.
- Backpressure.
- Errores.
- Estado de streams.
- Estado de consumers.
- Capacidad de almacenamiento.

## 8.2 Outbox e Inbox

Observar:

- Registros pendientes.
- Antigüedad del evento más antiguo.
- Tiempo hasta publicación.
- Fallos de publicación.
- Reintentos.
- Duplicados.
- Conflictos de idempotencia.
- Consumidores retrasados.
- Errores de procesamiento.
- Registros bloqueados.

Un backlog creciente debe generar una alerta si supera los umbrales operativos definidos.

No generar una alerta por cada reintento individual.

# 9. Instrumentación de bases de datos

## 9.1 PostgreSQL

Observar:

- Conexiones activas.
- Conexiones disponibles.
- Pool exhaustion.
- Tiempo de adquisición de conexiones.
- Latencia de consultas.
- Transacciones.
- Deadlocks.
- Locks.
- Long-running queries.
- Rollbacks.
- Errores.
- Replicación cuando exista.
- WAL.
- Espacio de almacenamiento.
- Autovacuum.
- Índices.
- Cache hit ratio.
- IOPS.
- Saturación.

## 9.2 SQLx

Los pools deben exponer métricas para:

- Conexiones adquiridas.
- Conexiones inactivas.
- Conexiones máximas.
- Tiempo de espera.
- Fallos de adquisición.
- Tiempo de consultas.
- Timeouts.

## 9.3 Consultas lentas

Las consultas lentas deben poder diagnosticarse mediante trazas o registros controlados.

No registrar valores sensibles de parámetros SQL.

Utilizar identificadores o fingerprints de consultas para agrupar problemas similares.

No utilizar cada SQL completo como etiqueta de métrica.

## 9.4 Redis

Observar:

- Memoria.
- Evictions.
- Latencia.
- Conexiones.
- Comandos.
- Errores.
- Hit ratio.
- Uso de CPU.
- Persistencia cuando corresponda.
- Estado de replicación cuando exista.
- Saturación.

## 9.5 Aislamiento

La instrumentación no debe debilitar RLS, permisos ni aislamiento multi-tenant.

Los exporters deben utilizar credenciales de mínimo privilegio.

No exponer interfaces de administración de bases de datos públicamente.

# 10. Instrumentación de infraestructura

Todos los servidores y servicios productivos deben disponer de monitoreo.

## 10.1 Sistema operativo

Observar:

- CPU.
- RAM.
- Swap.
- Disco.
- I/O.
- Load average.
- Red.
- Procesos.
- File descriptors.
- Reinicios.
- Uso de recursos.
- Temperaturas cuando estén disponibles y sean relevantes.

## 10.2 Servicios

Observar:

- Estado del servicio.
- Health checks.
- Readiness.
- Liveness.
- Reinicios.
- Duración de arranque.
- Consumo de recursos.
- Errores.
- Disponibilidad.

## 10.3 TLS y certificados

Monitorear:

- Fecha de expiración.
- Estado de renovación.
- Cadena de certificados.
- Errores de handshake.
- Fallos de validación.

Generar alertas anticipadas antes de que un certificado expire.

## 10.4 Red

Observar:

- Latencia.
- Pérdida de paquetes.
- Conexiones.
- Errores.
- Throughput.
- Disponibilidad de rutas.
- Fallos de resolución DNS.

## 10.5 Backups

Monitorear:

- Último backup exitoso.
- Duración.
- Tamaño.
- Errores.
- Espacio.
- Integridad.
- Retención.
- Última restauración verificada.

Un backup que nunca se ha probado restaurar no debe considerarse una garantía suficiente de recuperación.

# 11. Correlación distribuida

La correlación es obligatoria para diagnosticar operaciones que atraviesan diferentes componentes.

## 11.1 Identificadores

Distinguir:

- `trace_id`
- `span_id`
- `request_id`
- `correlation_id`
- `causation_id`
- `event_id`
- `tenant_id`
- `user_id`

No confundir sus funciones.

## 11.2 Propagación

Utilizar W3C Trace Context cuando corresponda:

- `traceparent`
- `tracestate`

Propagar contexto a través de:

- HTTP.
- gRPC.
- NATS JetStream.
- Workers.
- Outbox.
- Inbox.
- Llamadas a proveedores cuando sea viable.

## 11.3 Procesos asíncronos

Cuando una operación se procese de forma asíncrona, mantener la relación causal con la operación que la originó.

Para procesos de larga duración o procesamiento batch, utilizar vínculos entre spans cuando sea más apropiado que mantener una traza artificialmente larga.

## 11.4 Seguridad de contexto

No confiar en identificadores enviados por clientes como prueba de identidad.

Los identificadores de correlación deben validarse y tratarse como metadatos de diagnóstico.

No utilizarlos como mecanismos de autorización.

# 12. Cardinalidad y control de costes

La cardinalidad es uno de los riesgos principales de una plataforma de observabilidad.

Una etiqueta diferente por usuario, reserva, pedido o pago puede multiplicar las series temporales y elevar los costes.

## 12.1 Prohibición de alta cardinalidad

No utilizar como etiquetas de métricas:

- `user_id`
- `customer_id`
- `tenant_id` cuando el volumen pueda ser elevado.
- `booking_id`
- `payment_id`
- `order_id`
- `request_id`
- `trace_id`
- `email`
- `phone`
- IDs aleatorios.
- URLs con identificadores dinámicos.

## 12.2 Dimensiones permitidas

Preferir dimensiones controladas:

- `service_name`
- `environment`
- `http_method`
- `http_status_class`
- `route`
- `operation`
- `error_type`
- `region`
- `dependency`
- `result`
- `worker_type`

La lista real debe revisarse y limitarse.

## 12.3 Información individual

El detalle individual debe ir en:

- Logs.
- Trazas.
- Sentry.
- Herramientas de diagnóstico autorizadas.

Cuando sea necesario investigar un usuario, buscar por identificadores en fuentes de detalle, no convertir cada usuario en una serie temporal.

## 12.4 Cardinalidad por tenant

El aislamiento y las métricas por tenant pueden ser útiles para operación y soporte.

Sin embargo, no deben generar series ilimitadas.

Utilizar:

- Agregaciones.
- Rankings controlados.
- Consultas bajo demanda.
- Almacenamiento analítico separado cuando esté justificado.
- Métricas de tenants seleccionados.
- Límites de dimensiones.

## 12.5 Presupuestos

Definir presupuestos de:

- Series activas.
- Volumen de logs.
- Volumen de trazas.
- Eventos de Sentry.
- Sampling.
- Retención.
- Almacenamiento.
- Tráfico de telemetría.

Los presupuestos deben medirse y revisarse periódicamente.

# 13. Sentry como componente obligatorio

Sentry debe formar parte de la estrategia de observabilidad de ENSAMBRIA.

No es únicamente una herramienta opcional para registrar errores de frontend.

Debe utilizarse para las aplicaciones donde aporte valor.

## 13.1 Frontends

Configurar Sentry para:

- React.
- Next.js.
- Flutter.
- Errores de cliente.
- Errores de servidor cuando corresponda.
- Rendimiento.
- Releases.
- Source maps donde aplique.
- Contexto de trazas.

## 13.2 Backends Rust

Evaluar la integración de Sentry con los servicios Rust para:

- Excepciones no controladas.
- Panics.
- Errores inesperados.
- Contexto de ejecución.
- Releases.
- Trazas cuando la integración sea compatible.

No enviar todos los errores de negocio a Sentry.

Los errores esperados deben permanecer en logs y métricas cuando corresponda.

## 13.3 Integración

Los eventos de Sentry deben poder correlacionarse con:

- Release.
- Servicio.
- Entorno.
- Trace ID.
- Span ID cuando esté disponible.
- Timestamp.
- Identificador de petición.
- Error correspondiente.

## 13.4 Sampling

Configurar límites para evitar:

- Envíos excesivos.
- Eventos duplicados.
- Explosión de costes.
- Repetición masiva del mismo error.

No descartar indiscriminadamente errores críticos.

El sampling debe ser configurable por entorno y tipo de evento.

## 13.5 Alertas

Crear alertas para:

- Nuevos errores críticos.
- Incremento significativo de errores.
- Regresiones.
- Aumento de errores tras un despliegue.
- Fallos de funcionalidades críticas.

No notificar cada ocurrencia individual.

Agrupar eventos y utilizar umbrales.

# 14. Métricas RED y USE

Los servicios HTTP y RPC deben implementar métricas RED:

- Rate: solicitudes por unidad de tiempo.
- Errors: solicitudes fallidas.
- Duration: duración de las solicitudes.

La infraestructura debe implementar métricas USE:

- Utilization: utilización.
- Saturation: saturación.
- Errors: errores.

Estas metodologías deben aplicarse a toda la plataforma, no exclusivamente a pagos.

## 14.1 RED

Por cada servicio:

- Solicitudes.
- Errores.
- Latencia.
- Rutas normalizadas.
- Dependencias.
- Operaciones relevantes.

## 14.2 USE

Por cada recurso:

- CPU.
- RAM.
- Disco.
- Red.
- Pools.
- Colas.
- Conexiones.
- Capacidad.

# 15. Health checks

Cada backend y proceso crítico debe disponer de verificaciones de salud.

Diferenciar:

- Liveness.
- Readiness.
- Health de dependencias.

## 15.1 Liveness

Indica si el proceso sigue funcionando y puede continuar ejecutándose.

No debe depender indiscriminadamente de todas las dependencias externas.

## 15.2 Readiness

Indica si el proceso está preparado para recibir tráfico.

Debe comprobar las dependencias críticas necesarias para cumplir su función.

## 15.3 Dependencias

Verificar cuando corresponda:

- PostgreSQL.
- Redis.
- NATS.
- Vault.
- Servicios internos.
- Proveedores críticos.

No convertir una dependencia opcional en motivo de indisponibilidad global.

## 15.4 Exposición

Los endpoints de salud no deben filtrar:

- Secretos.
- Direcciones internas sensibles.
- Credenciales.
- Consultas.
- Detalles de infraestructura innecesarios.

# 16. Alertas

Las alertas deben basarse en impacto, tendencia, saturación y riesgo.

No crear alertas para cada evento individual.

## 16.1 Severidades

Definir niveles operativos:

### Critical

Incidente con impacto grave o riesgo inmediato.

Ejemplos:

- Servicio central indisponible.
- Pérdida o corrupción de datos.
- Fallo de aislamiento multi-tenant.
- Riesgo de exposición de información.
- Fallo generalizado de autenticación.
- Fallo generalizado de pagos.
- Incidente de seguridad grave.

### Warning

Degradación que necesita atención.

Ejemplos:

- Latencia elevada.
- Error rate creciente.
- Pool de conexiones cerca del límite.
- Backlog creciendo.
- Consumo de disco elevado.
- Proveedor externo degradado.

### Info

Eventos operativos informativos.

No deben generar notificaciones urgentes por defecto.

## 16.2 Reglas

Las alertas deben considerar:

- Umbral.
- Ventana de evaluación.
- Duración.
- Severidad.
- Agrupación.
- Supresión.
- Dependencias.
- Runbook.
- Responsable.
- Condición de recuperación.

## 16.3 Ejemplos

Los siguientes valores son ejemplos iniciales, no SLO ni umbrales definitivos:

- Error rate superior al 5% durante 5 minutos.
- p95 superior al objetivo durante 10 minutos.
- Pool de conexiones por encima del 85% de forma sostenida.
- Backlog con antigüedad superior al límite operativo.
- Disco por encima del 85%.
- Certificado próximo a expirar.
- Fallo de backup.
- Caída de un servicio crítico.

Cada alerta debe calibrarse según la carga real y el impacto.

## 16.4 Notificaciones

No inventar webhooks ni destinos.

Los canales de notificación deben aprobarse y configurarse explícitamente.

Alertmanager puede estar funcionando aunque todavía no exista un destino externo autorizado.

En ese caso:

- Mantener reglas.
- Validar alertas.
- Verificar agrupación.
- Mantener visibilidad local.
- Documentar el canal pendiente.

# 17. SLI, SLO y SLA

ENSAMBRIA debe definir objetivos de confiabilidad para servicios críticos.

## 17.1 SLI

Ejemplos:

- Porcentaje de peticiones exitosas.
- Latencia de APIs.
- Disponibilidad.
- Tiempo de procesamiento de reservas.
- Tiempo de publicación de eventos.
- Tiempo de entrega de notificaciones.
- Éxito de operaciones de autenticación.
- Éxito de checkout.

Los indicadores deben medir resultados reales y excluir únicamente eventos que se justifiquen de manera consistente.

## 17.2 SLO

Cada servicio crítico debe tener objetivos documentados.

Ejemplo ilustrativo:

- SLI: porcentaje de solicitudes exitosas.
- SLO: 99,9% durante un periodo mensual.

No adoptar el mismo objetivo para todos los servicios.

Un servicio interno no crítico puede necesitar un objetivo diferente al de autenticación o reservas.

## 17.3 Error budget

Calcular el margen de error permitido por el SLO.

Cuando el error budget se consuma rápidamente:

- Priorizar confiabilidad.
- Revisar despliegues.
- Reducir cambios de riesgo.
- Corregir causas raíz.
- Revisar capacidad.
- Evaluar dependencias.

## 17.4 Alertas basadas en SLO

Preferir alertas por consumo rápido de error budget y degradación sostenida.

Evitar alertar por cada fluctuación menor.

## 17.5 SLA

Un SLA requiere aprobación contractual y comercial.

No convertir automáticamente los SLO internos en promesas contractuales.

Los créditos, compensaciones y penalizaciones requieren políticas aprobadas.

# 18. Dashboards

Crear dashboards por nivel de operación.

## 18.1 Dashboard global

Debe mostrar:

- Estado general de ENSAMBRIA.
- Disponibilidad.
- Errores.
- Latencia.
- Saturación.
- Incidentes.
- Estado de los cuatro backends.
- Estado de frontends.
- Estado de infraestructura.
- Estado de dependencias críticas.
- SLO globales.

## 18.2 Dashboard por backend

Uno para cada backend:

- CORE.
- BUSINESS.
- PLATFORM.
- PUBLIC.

Mostrar:

- Rate.
- Error rate.
- Latencia.
- Recursos.
- Dependencias.
- Bases de datos.
- Workers.
- Trazas.
- Errores.
- SLO.

## 18.3 Dashboard de frontend

Uno por frontend:

- Errores.
- Releases.
- Latencia.
- Rendimiento.
- Web Vitals.
- Fallos de API.
- Navegación.
- Errores de usuario.

## 18.4 Dashboard de infraestructura

Mostrar:

- CPU.
- RAM.
- Disco.
- Red.
- Procesos.
- Servicios.
- Certificados.
- Backups.
- Recursos disponibles.

## 18.5 Dashboard de bases de datos

Separar PostgreSQL y Redis.

Incluir:

- Conexiones.
- Latencia.
- Saturación.
- Errores.
- Locks.
- Consultas lentas.
- Memoria.
- Espacio.
- Pools.

## 18.6 Dashboard de mensajería

Para NATS JetStream:

- Throughput.
- Consumer lag.
- Mensajes pendientes.
- Redeliveries.
- Errores.
- DLQ.
- Estado de streams.

## 18.7 Dashboard SLO

Mostrar:

- SLI actual.
- Objetivo.
- Error budget restante.
- Consumo del error budget.
- Tendencia.
- Servicios fuera de objetivo.

# 19. Seguridad y privacidad de telemetría

La telemetría puede contener información personal o sensible.

Debe tratarse como información protegida.

## 19.1 Prohibiciones

No registrar:

- Contraseñas.
- Tokens.
- Secretos.
- API keys privadas.
- Cookies completas.
- Datos de tarjetas.
- CVV.
- Documentos de identidad completos.
- Información médica.
- Datos sensibles.
- Contenido privado innecesario.
- Payloads completos de formularios.

## 19.2 Redacción

Implementar filtros de redacción antes de almacenar o exportar telemetría.

La redacción debe aplicarse en:

- Logs.
- Trazas.
- Sentry.
- Errores.
- Eventos.
- Metadatos.

## 19.3 Tenant isolation

No permitir que un cliente o tenant acceda a logs, métricas, trazas o errores internos de otro tenant.

La plataforma de observabilidad debe estar protegida como infraestructura administrativa.

Los accesos de soporte deben ser autorizados y auditados.

## 19.4 Acceso

Aplicar:

- Mínimo privilegio.
- MFA.
- Roles.
- Separación de entornos.
- Auditoría.
- Acceso temporal cuando corresponda.
- Restricciones de exportación.

## 19.5 Retención

Definir retención diferenciada para:

- Logs.
- Métricas.
- Trazas.
- Sentry.
- Auditoría.
- Profiling.

No conservar todo indefinidamente.

Los periodos deben considerar requisitos legales, utilidad operativa, coste y capacidad.

# 20. Sampling y gestión de volumen

No es necesario almacenar absolutamente todas las trazas de manera indefinida.

Definir estrategias de sampling.

## 20.1 Trazas

Considerar:

- Head sampling.
- Tail sampling.
- Sampling por servicio.
- Sampling de errores.
- Sampling de operaciones críticas.

Las operaciones críticas y los errores relevantes deben tener una estrategia que preserve información diagnóstica suficiente.

## 20.2 Logs

Definir niveles y retención por entorno.

Evitar DEBUG permanente en producción sin justificación.

## 20.3 Métricas

Evitar duplicación de métricas y dimensiones innecesarias.

No generar series temporales por identificadores individuales.

## 20.4 Sentry

Configurar sampling de eventos y transacciones.

No descartar errores críticos por utilizar un porcentaje fijo sin evaluar impacto.

# 21. Gestión de releases y despliegues

Toda versión desplegada debe poder identificarse.

Registrar:

- Servicio.
- Versión.
- Commit SHA.
- Build ID.
- Entorno.
- Fecha.
- Resultado del despliegue.

Los despliegues deben poder correlacionarse con:

- Aumento de errores.
- Cambios de latencia.
- Regresiones.
- Excepciones.
- Consumo de recursos.

## 21.1 Verificación posterior

Después de desplegar:

- Verificar health checks.
- Revisar logs.
- Revisar métricas.
- Revisar errores en Sentry.
- Revisar trazas.
- Verificar operaciones críticas.
- Confirmar ausencia de degradación.

## 21.2 Rollback

Si una versión introduce una degradación crítica, debe existir un procedimiento de rollback.

No ejecutar rollback automáticamente sin definir condiciones y seguridad de los datos cuando la migración sea incompatible.

# 22. Alertas de seguridad

La observabilidad debe ayudar a detectar incidentes de seguridad.

Ejemplos:

- Aumento anormal de fallos de autenticación.
- Accesos administrativos sospechosos.
- Intentos de escalada de privilegios.
- Accesos cruzados entre tenants.
- Exceso de rate limiting.
- Errores de autorización.
- Cambios inesperados de permisos.
- Uso anómalo de credenciales.
- Fallos de Vault.
- Certificados inválidos.
- Exposición accidental de secretos.

Los logs de auditoría y seguridad deben tener controles de integridad y acceso propios.

No sustituir un SIEM o un sistema de detección de amenazas por dashboards de métricas.

Evaluar capacidades adicionales según los riesgos y la escala.

# 23. Costes y capacidad de observabilidad

La plataforma de observabilidad también debe monitorearse.

Observar:

- Volumen de logs.
- Volumen de métricas.
- Número de series activas.
- Cardinalidad.
- Volumen de trazas.
- Sampling.
- Eventos de Sentry.
- Almacenamiento.
- CPU y RAM de Collectors.
- Fallos de exportación.
- Retención.
- Coste por proveedor cuando corresponda.

Definir límites y presupuestos.

No permitir que una tormenta de errores haga que el sistema de observabilidad agote los recursos de la aplicación.

La telemetría no debe comprometer la disponibilidad de los servicios principales.

# 24. Operación y procedimientos

Cada alerta crítica debe disponer de un runbook.

Un runbook debe explicar:

- Qué significa la alerta.
- Qué componente afecta.
- Cómo medir el impacto.
- Qué dashboards revisar.
- Cómo localizar la traza.
- Cómo buscar los logs.
- Qué dependencias comprobar.
- Qué acciones son seguras.
- Cómo mitigar.
- Cómo escalar.
- Cómo verificar recuperación.

## 24.1 Incidentes

Documentar:

- Inicio.
- Detección.
- Impacto.
- Servicios afectados.
- Causa conocida o probable.
- Acciones.
- Recuperación.
- Duración.
- Prevención.

## 24.2 Postmortems

Los incidentes importantes deben tener revisión posterior sin cultura de culpabilización.

Identificar:

- Causa raíz.
- Factores contribuyentes.
- Fallos de detección.
- Fallos de mitigación.
- Impacto.
- Acciones correctivas.
- Responsables.
- Plazos.

# 25. Pruebas obligatorias

La observabilidad debe probarse.

## 25.1 Unitarias

Probar:

- Clasificación de errores.
- Atributos de telemetría.
- Redacción.
- Propagación de contexto.
- Normalización de rutas.
- Etiquetas de métricas.
- Políticas de sampling.

## 25.2 Integración

Verificar:

- Logs estructurados.
- Exportación de telemetría.
- Recepción del Collector.
- Propagación HTTP.
- Propagación gRPC.
- Propagación NATS.
- Correlación de workers.
- Integración Sentry.
- Métricas SQLx.
- Health checks.

## 25.3 Fallos controlados

Probar escenarios como:

- PostgreSQL lento.
- Redis indisponible.
- NATS desconectado.
- Timeout de proveedor.
- Worker detenido.
- Error de serialización.
- Pool agotado.
- Servicio reiniciado.
- Fallo de Collector.
- Fallo de exportación.
- Aumento repentino de errores.

Verificar que los sistemas detectan el problema y generan la telemetría correspondiente.

## 25.4 Cardinalidad

Incluir pruebas o controles que detecten etiquetas de alta cardinalidad.

No permitir que IDs aleatorios se incorporen a métricas de producción.

## 25.5 Alertas

Verificar:

- Activación.
- Agrupación.
- Silenciamiento.
- Recuperación.
- Severidad.
- Runbook.
- Ausencia de alertas duplicadas.

# 26. Fases de implementación

## Fase 0 — Diseño y aceptación documental

Fase 0 no crea runtime, instrumentación, servicios de telemetría, dashboards activos ni alertas ejecutables. Antes de cerrar esta fase:

- Inventariar servicios, procesos, frontends, dependencias, propietarios y fronteras de telemetría por fase.
- Definir flujos de logs, métricas, trazas, correlación y diagnóstico de errores; la auditoría funcional conserva su sistema de registro independiente y no se sustituye por telemetría.
- Identificar datos personales/sensibles en telemetría, responsables, clasificación, minimización, redacción, acceso, retención y jurisdicción aplicable junto con las guías de privacidad.
- Documentar la estrategia de cardinalidad, muestreo, volumen, costes, degradación ante fallos de exportación y destinos autorizados.
- Asignar responsable de Observabilidad/SRE y acordar quién aprueba SLI/SLO, alertas, runbooks y criterios de aceptación.
- Definir los SLI/SLO candidatos por servicio crítico con evidencia de requisitos; no fijar objetivos arbitrarios ni convertir SLO en SLA sin aprobación contractual.
- Diseñar pruebas y trazabilidad para propagación/correlación, errores, redacción, cardinalidad, fallos de telemetría y alertas.
- Evaluar y documentar el stack de referencia según las necesidades reales; no instalar ni desplegar componentes como parte de la Fase 0.

El criterio de aceptación es que las decisiones, responsables, inventario, controles, pruebas, N/A y bloqueos estén documentados y aprobados por sus propietarios. La existencia de esta guía no prueba que la infraestructura o controles estén configurados.

## Fase 1 — Primer corte ejecutable y fases posteriores

Desde el primer corte ejecutable, implementar y verificar las capacidades pertinentes en el servicio y flujo reales: logs estructurados seguros, propagación y correlación de contexto, diagnóstico tipado de errores, métricas operativas con cardinalidad controlada, health/readiness aplicables, redacción y degradación no bloqueante si falla un destino de telemetría. Integrar Sentry conforme a la sección 13 y a la configuración aprobada. Probar los recorridos y dependencias reales que existan en ese corte.

Cada fase posterior instrumenta los servicios, dependencias, procesos asíncronos, cambios de estado y superficies que introduce o modifica, y actualiza sus métricas, SLI/SLO, alertas, dashboards, runbooks y evidencia según corresponda. No se difieren controles requeridos por el comportamiento a una fase transversal futura. Los requisitos sin base suficiente para fijar umbrales permanecen como decisiones pendientes y no se reportan como objetivos aprobados.

# 27. Reglas para el agente de desarrollo

El agente debe:

1. Inspeccionar la infraestructura y el repositorio antes de implementar.
2. Identificar los cuatro backends y seis proyectos de interfaz.
3. Identificar todos los procesos y dependencias existentes.
4. Revisar las herramientas ya instaladas.
5. Evitar desplegar componentes redundantes.
6. Utilizar estándares abiertos.
7. No introducir proveedores comerciales sin aprobación.
8. No inventar destinos de alertas.
9. No inventar endpoints de monitoreo.
10. No exponer interfaces internas.
11. No registrar secretos ni información sensible.
12. No crear métricas de cardinalidad ilimitada.
13. No generar logs innecesarios.
14. No enviar todos los errores de negocio a Sentry.
15. No utilizar Sentry como sustituto de la observabilidad completa.
16. No debilitar la seguridad para facilitar instrumentación.
17. No introducir mocks como sustitutos de pruebas reales.
18. No dejar integraciones simuladas.
19. No dejar TODOs para aparentar implementación.
20. No modificar scripts, workflows o infraestructura fuera del alcance autorizado.
21. Actualizar documentación.
22. Actualizar el roadmap.
23. Ejecutar pruebas.
24. Corregir los errores introducidos.
25. Presentar evidencia verificable.

Antes de implementar debe entregar o documentar:

- Inventario de componentes.
- Arquitectura propuesta.
- Flujos de telemetría.
- Stack existente.
- Stack faltante.
- Presupuesto de recursos.
- Políticas de cardinalidad.
- Retención.
- Seguridad.
- Plan de pruebas.
- Riesgos y decisiones pendientes.

No instalar todas las herramientas de referencia sin justificar la necesidad y el consumo de recursos.

## 27.1 Procedimiento de implementación y verificación

Antes de cada cambio, identifica el requisito y la fase ENSAMBRIA, los servicios/procesos reales afectados, la instrumentación e infraestructura existentes, los criterios de aceptación y las pruebas pertinentes. Contrasta documentación con código y pruebas; no asumas que una capacidad existe por estar descrita. Registra brevemente el alcance, decisiones necesarias, riesgos y bloqueos en la documentación de fase o del componente existente. No crees un plan o archivo adicional si un documento vigente ya cubre esa evidencia.

Integra únicamente las capacidades aplicables a componentes reales y a la fase actual. Reutiliza instrumentación y dependencias existentes; no añadas sistemas, crates, exporters, SDKs, proveedores, destinos, variables ni umbrales redundantes o no aprobados. Verifica compatibilidad de APIs y dependencias con MSRV, `Cargo.lock`, frameworks y CI reales; justifica los cambios necesarios y evita actualizaciones ajenas al alcance.

Implementa las pruebas exigidas por esta guía, el caso de uso y la política del proyecto. Ejecuta los comandos de validación aplicables y reporta sus resultados reales; las pruebas escritas o el código de instrumentación por sí solos no demuestran aceptación. Los dobles de prueba no sustituyen integración con dependencias reales. Para Sentry y otros servicios externos, distingue configuración de recepción efectivamente verificada; no uses credenciales productivas ni envíes eventos externos sin autorización.

Actualiza primero la documentación pertinente a la tarea y mantenla consistente con la implementación. No despliegues a producción, alteres infraestructura compartida ni ejecutes operaciones destructivas sin autorización explícita.

# 28. Documentación obligatoria

Mantener versionadas las decisiones y evidencias en esta guía y en los documentos existentes de arquitectura, fase y operación que correspondan. Versionar junto al componente propietario las configuraciones implementadas de dashboards, alertas y runbooks. Crear documentación separada solo cuando exista una necesidad operativa o contractual concreta que no cubra un documento existente; no duplicar políticas ni crear archivos vacíos o listas de documentos sin contenido aprobado. La documentación describe decisiones e implementaciones verificadas, y distingue claramente lo pendiente.

# 29. Definition of Done

La observabilidad de un componente se considera implementada cuando:

- Está instrumentado.
- Emite logs estructurados.
- Expone métricas relevantes.
- Genera trazas cuando corresponda.
- Propaga contexto.
- Redacta datos sensibles.
- Respeta cardinalidad.
- Tiene health checks.
- Dispone de dashboards apropiados.
- Tiene alertas cuando el riesgo lo justifica.
- Cuenta con retención definida.
- Tiene controles de acceso.
- Tiene pruebas.
- Tiene documentación.
- Se verifica la recepción de telemetría.
- Se ha probado el comportamiento ante fallos.

La plataforma de observabilidad global se considera terminada cuando todos los componentes críticos están cubiertos y existe una forma verificable de diagnosticar incidentes de extremo a extremo.

El informe de cada entrega resume alcance, decisiones, archivos/componentes, capacidades implementadas y pendientes, comandos y resultados reales, dependencias/bloqueos y riesgos que requieran revisión. Clasifica cada verificación como aprobada, fallida, bloqueada, no ejecutada o no aplicable con justificación. No afirmes cumplimiento de SLO, integración externa, privacidad legal ni cobertura completa sin evidencia correspondiente.

# 30. Regla final

ENSAMBRIA no debe esperar a que los usuarios reporten los problemas para descubrir que algo está fallando.

Debe detectar degradaciones, identificar sus causas, relacionar operaciones entre servicios y proporcionar información suficiente para resolver incidentes.

La observabilidad debe cubrir toda la aplicación y toda la infraestructura.

No solamente pagos.

No solamente reservas.

No solamente los backends.

No solamente los frontends.

**Toda ENSAMBRIA.**

Principio operativo:

**Logs + Métricas + Trazas + Correlación + Sentry + Alertas + SLI/SLO + Seguridad + Control de costes = Observabilidad integral de producción.**

La observabilidad debe permitir crecer internacionalmente, incorporar nuevos módulos, añadir backends, extraer servicios y aumentar la cantidad de tenants sin perder capacidad de diagnóstico ni control operativo.
