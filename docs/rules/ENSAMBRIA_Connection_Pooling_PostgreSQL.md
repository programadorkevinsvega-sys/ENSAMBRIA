# ENSAMBRIA — Estrategia de Connection Pooling y Gestión de Conexiones PostgreSQL

**Estado:** Topología compartida definida para ENSAMBRIA; requiere dimensionamiento y revisión formal antes de implementar<br>
**Stack principal:** Rust, Tokio, SQLx y PostgreSQL<br>
**Ámbito:** CORE, BUSINESS, PLATFORM, PUBLIC y procesos worker<br>
**Objetivo:** Controlar el consumo de conexiones, mantener aislamiento entre backends, proteger PostgreSQL bajo concurrencia y permitir escalar horizontalmente sin crear una conexión nueva por petición.

---

## 1. Objetivo y principios

ENSAMBRIA debe utilizar connection pooling desde el inicio. Una petición HTTP no debe abrir una conexión TCP/PostgreSQL nueva y cerrarla al terminar. Cada proceso mantiene un pool acotado de conexiones reutilizables.

Principios obligatorios:

1. Ningún backend abre una conexión nueva por petición.
2. Cada backend administra su propio pool lógico y sus propias credenciales.
3. Los workers tienen pools y límites de concurrencia independientes.
4. Los límites se calculan considerando todas las réplicas y procesos, no sólo una instancia.
5. Las conexiones se adquieren con timeout; nunca se espera indefinidamente.
6. Las transacciones deben ser cortas y liberar la conexión tan pronto termine el trabajo de base de datos.
7. Las consultas y operaciones costosas deben tener límites de tiempo y concurrencia.
8. El contexto de tenant debe establecerse de forma segura por transacción cuando se utilice RLS.
9. El agotamiento del pool debe producir degradación controlada, métricas y errores manejables; no una caída en cascada.
10. Los valores de configuración se validan al iniciar el proceso y se documentan.

El pooling reutiliza conexiones; no convierte automáticamente varias consultas en una sola operación ni sustituye índices, caché, diseño de consultas o control de concurrencia.

## 2. Topología lógica de pools

Se definen pools independientes por backend:

| Proceso | Pool lógico | Responsabilidad |
|---|---|---|
| CORE Backend | `core_db_pool` | Identidad, tenancy, organizaciones, sucursales, entitlements, reservas comunes y operaciones financieras centrales |
| BUSINESS Backend | `business_db_pool` | Operaciones de establecimientos, profesionales, servicios, clientes, CRM, membresías, inventario y comercio |
| PLATFORM Backend | `platform_db_pool` | Administración SaaS, planes, suscripciones, facturación SaaS y políticas comerciales |
| PUBLIC Backend | `public_db_pool` | Directorio, perfiles públicos, discovery, marketplace, reseñas y proyecciones públicas |
| Workers | Pools específicos por workload | Exportaciones, privacidad/eliminación, conciliación, outbox, tareas programadas y procesamiento asíncrono |

No se debe crear un pool por tenant. Los tenants comparten la infraestructura de conexión del backend propietario, mientras que el aislamiento se aplica mediante autorización, consultas, roles, políticas y RLS cuando corresponda.

### 2.1 Aislamiento de credenciales y permisos

Cada backend debe utilizar una identidad PostgreSQL propia, con permisos mínimos sobre los objetos que posee.

- CORE no debe escribir directamente las tablas propiedad de BUSINESS, PLATFORM o PUBLIC.
- BUSINESS no debe escribir directamente las tablas propiedad de CORE, PLATFORM o PUBLIC.
- PLATFORM no debe escribir directamente las tablas propiedad de CORE, BUSINESS o PUBLIC.
- PUBLIC debe acceder sólo a los datos y proyecciones que tiene autorizados.
- Los workers deben usar identidades y permisos específicos para el trabajo que ejecutan; no deben recibir automáticamente permisos de superusuario ni credenciales de todos los backends.

Los cuatro backends y sus módulos utilizan **una única base de datos PostgreSQL compartida**; no se crean bases independientes por backend o módulo. La infraestructura que aloja esa base (servidor, servicio administrado o clúster) se dimensiona según capacidad y requisitos operativos, pero no cambia la decisión de una sola base lógica compartida. La separación de seguridad es obligatoria: schemas, roles, grants, migraciones y ownership por módulo/backend, sin acceso cruzado a tablas. Esta topología no significa que los procesos compartan pools o credenciales.

## 3. SQLx en Rust

SQLx proporciona un pool asíncrono para PostgreSQL. Cada proceso crea su pool durante el arranque y lo comparte mediante el estado de la aplicación.

Ejemplo ilustrativo:

```rust
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(15)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
}
```

Los números anteriores son ejemplos, no valores aprobados para producción. Deben ser configurables por entorno y proceso, y ajustarse con pruebas de carga y capacidad real de PostgreSQL.

### 3.1 Configuración mínima requerida

Cada proceso debe permitir configurar, como mínimo:

- URL o mecanismo seguro de conexión.
- Máximo de conexiones.
- Mínimo de conexiones, si se necesita.
- Timeout de adquisición.
- Tiempo máximo de inactividad.
- Vida máxima de conexión.
- Timeout de consulta.
- Timeout de transacción.
- Límites de concurrencia para operaciones costosas.
- Parámetros TLS y verificación del certificado, según el entorno.

Los secretos no deben quedar en el repositorio, archivos versionados, imágenes frontend ni logs. Deben obtenerse mediante el mecanismo de secretos autorizado para el entorno, por ejemplo Vault o el gestor de secretos de la infraestructura.

### 3.2 Compartir el pool dentro del proceso

El pool se crea una vez durante el arranque y se inyecta en el estado de la aplicación. Los handlers y casos de uso reciben acceso al pool mediante las dependencias de infraestructura definidas por la arquitectura.

No crear un pool dentro de cada handler, petición, tarea o caso de uso.

No mantener una conexión adquirida durante llamadas HTTP/gRPC externas, esperas de usuario, publicación lenta de mensajes o procesamiento que no requiera la base de datos.

## 4. Presupuesto global de conexiones

El límite de conexiones no se calcula mirando una sola instancia. Se calcula para todo el despliegue.

Fórmula:

```text
C_total =
    Σ (réplicas_del_proceso × max_connections_del_pool)
    + conexiones_de_workers
    + migraciones
    + administración
    + monitoreo
    + margen_operativo
```

Ejemplo puramente ilustrativo:

| Proceso | Réplicas | Máximo por réplica | Máximo agregado |
|---|---:|---:|---:|
| CORE | 1 | 15 | 15 |
| BUSINESS | 1 | 15 | 15 |
| PLATFORM | 1 | 8 | 8 |
| PUBLIC | 1 | 10 | 10 |
| Workers | 1 | 5 | 5 |
| **Subtotal** | | | **53** |

Este ejemplo no es una recomendación definitiva. Si BUSINESS escala a cuatro réplicas con un pool máximo de 15, su presupuesto pasa a 60 conexiones. El presupuesto debe recalcularse antes de aumentar réplicas o máximos.

PostgreSQL también necesita margen para administración, migraciones, observabilidad, tareas operativas y conexiones de emergencia. No configurar la suma de pools al límite absoluto de `max_connections`.

### 4.1 Procedimiento para definir límites

1. Medir la capacidad de CPU, RAM, almacenamiento y configuración de PostgreSQL.
2. Identificar procesos y número máximo esperado de réplicas.
3. Estimar concurrencia y duración de consultas por workload.
4. Reservar conexiones para operaciones administrativas y mantenimiento.
5. Configurar límites iniciales conservadores.
6. Ejecutar pruebas de carga, concurrencia y saturación.
7. Observar throughput, latencia, espera de pool, CPU, I/O y locks.
8. Ajustar límites con evidencia.
9. Documentar el presupuesto agregado y sus supuestos.
10. Repetir el cálculo al cambiar réplicas, workers o topología.

Más conexiones no significan automáticamente más rendimiento. Un número excesivo puede aumentar cambios de contexto, memoria y contención en PostgreSQL.

## 5. PgBouncer: uso opcional y condicionado

PgBouncer puede incorporarse cuando las mediciones indiquen que el número de conexiones directas o la cantidad de instancias lo justifican.

No es obligatorio introducirlo desde el primer despliegue. SQLx ya ofrece pooling dentro de cada proceso.

Antes de utilizar PgBouncer se debe:

- Definir el modo de pooling.
- Verificar compatibilidad con SQLx y el comportamiento de la aplicación.
- Probar consultas preparadas y prepared statements.
- Revisar el uso de estado de sesión, tablas temporales, `LISTEN/NOTIFY` y otras funciones que dependan de una sesión persistente.
- Verificar cómo se establece y limpia el contexto de tenant.
- Probar migraciones y tareas administrativas por una ruta compatible.
- Medir latencia y comportamiento bajo concurrencia.

El modo transaction pooling puede reasignar la conexión PostgreSQL física después de cada transacción. Por eso, la aplicación no debe depender de estado de sesión que sobreviva entre transacciones. No asumir compatibilidad sin pruebas.

## 6. RLS y contexto multi-tenant

Cuando se utilice PostgreSQL Row-Level Security (RLS), el contexto del tenant debe establecerse de forma segura y consistente con la transacción que ejecuta la consulta.

Patrón conceptual:

```sql
BEGIN;

SET LOCAL app.tenant_id = 'UUID-DEL-TENANT';

-- Consultas autorizadas dentro de esta transacción.

COMMIT;
```

`SET LOCAL` limita el contexto a la transacción. Es preferible a dejar una variable de sesión persistente en una conexión reutilizada por el pool, porque esa conexión puede atender después otra petición o tenant.

Reglas:

1. El tenant se obtiene de un contexto autenticado y autorizado; nunca se confía en un `tenant_id` arbitrario enviado por el cliente.
2. La configuración del contexto y las consultas protegidas ocurren dentro de la misma transacción.
3. No ejecutar consultas protegidas antes de establecer el contexto.
4. No dejar contexto de tenant en estado de sesión después de devolver la conexión al pool.
5. Probar reutilización de conexiones entre tenants distintos.
6. Probar que una transacción fallida, cancelada o revertida no deja contexto filtrado.
7. Las políticas RLS deben complementarse con autorización de aplicación y permisos SQL mínimos.
8. Los roles de aplicación no deben poder omitir RLS salvo que exista una justificación técnica y controles específicos.

El uso exacto de `SET LOCAL`, funciones de contexto y políticas RLS debe quedar centralizado en una capa de infraestructura probada, no repetido de forma inconsistente en cada handler.

## 7. Transacciones y consultas

### 7.1 Transacciones cortas

Una transacción debe contener únicamente las operaciones que requieren atomicidad en PostgreSQL.

No mantener transacciones abiertas mientras se:

- Llama a una pasarela de pagos.
- Invoca otro backend por HTTP o gRPC.
- Espera una respuesta de usuario.
- Sube o descarga un archivo.
- Ejecuta un proceso largo de exportación.
- Espera una publicación externa lenta.

Para coordinar efectos externos se deben usar patrones adecuados, como Outbox/Inbox, idempotencia y procesos asíncronos, según el dominio.

### 7.2 Timeouts

Definir y probar límites para:

- Adquisición de conexión.
- Ejecución de consultas.
- Duración de transacciones.
- Operaciones de lock.
- Operaciones externas.
- Trabajos de workers.

Los timeouts deben ser coherentes entre aplicación, SQLx y PostgreSQL. No establecer límites arbitrariamente bajos que provoquen fallos legítimos ni dejar operaciones sin límite.

### 7.3 Consultas

El pooling no reemplaza el diseño correcto de consultas. Se deben aplicar:

- Índices adecuados a los patrones de acceso.
- Paginación por cursor/keyset en conjuntos grandes.
- Evitar N+1.
- Proyecciones y columnas necesarias, no lecturas indiscriminadas.
- Límites de filas y tamaños de respuesta.
- Planes de ejecución revisados para consultas críticas.
- Consultas parametrizadas.
- Límites de concurrencia para consultas costosas.

## 8. Pools independientes para workers

Los workers no deben consumir todas las conexiones disponibles y degradar reservas, pagos o autenticación.

Separar pools o presupuestos para workloads como:

- Exportación de datos.
- Eliminación/anonimización y ciclo de vida de datos.
- Reconciliación.
- Procesamiento Outbox/Inbox.
- Notificaciones.
- Tareas programadas.
- Indexación y proyecciones públicas.

Controles requeridos:

1. Pool pequeño y acotado por worker/workload.
2. Concurrencia limitada.
3. Lotes pequeños y configurables.
4. Transacciones cortas.
5. Checkpoints para reanudar trabajos.
6. Idempotencia y reintentos con backoff y jitter.
7. Pausa o reducción de velocidad ante saturación.
8. Cola o prioridad separada para tareas de baja prioridad.
9. Métricas por tipo de trabajo.
10. Pruebas que demuestren que los workers no agotan el presupuesto de conexiones de operaciones críticas.

Los tamaños de lote y la concurrencia deben dimensionarse mediante pruebas; no deben convertirse en constantes universales sin evidencia.

## 9. Manejo del agotamiento del pool

Cuando no haya conexiones disponibles, la aplicación debe esperar sólo hasta el timeout de adquisición configurado. Si no consigue conexión:

- Registrar el evento con información técnica útil, sin secretos ni datos personales.
- Incrementar métricas de espera y timeout.
- Devolver un error controlado y apropiado al protocolo/API.
- Evitar reintentos inmediatos ilimitados.
- Aplicar backpressure o límites de concurrencia donde corresponda.
- No abrir conexiones adicionales por fuera del pool como solución improvisada.

No convertir un error de pool en una cascada de reintentos que empeore la saturación.

Distinguir entre:
- Pool agotado.
- PostgreSQL no disponible.
- Timeout de consulta.
- Lock timeout.
- Error de autenticación/configuración.
- Error de red/TLS.

Cada categoría debe tener tratamiento, observabilidad y política de reintento apropiados.

## 10. Observabilidad

Exponer métricas por proceso y pool, como mínimo:

- Conexiones abiertas.
- Conexiones inactivas.
- Conexiones en uso.
- Máximo configurado.
- Tiempo de adquisición.
- Número de adquisiciones en espera.
- Timeouts de adquisición.
- Errores de conexión.
- Duración de consultas y transacciones.
- Errores y cancelaciones.
- Saturación de workers y concurrencia activa.

Las métricas deben permitir identificar qué backend o workload consume las conexiones. No incluir URLs de conexión, contraseñas, tokens, datos personales ni SQL con parámetros sensibles.

Definir umbrales de alerta a partir de la capacidad medida y los SLO del sistema; no inventar umbrales universales sin conocer la infraestructura.

## 11. Seguridad

- TLS para conexiones PostgreSQL cuando la topología lo requiera, validando certificados y autoridades de confianza.
- Credenciales distintas por backend y entorno.
- Secretos obtenidos desde Vault o el gestor autorizado.
- Rotación de credenciales con procedimiento probado.
- Permisos SQL mínimos.
- Sin credenciales de producción en código, frontend, logs o CI.
- No usar superusuario en procesos de aplicación.
- No exponer PostgreSQL públicamente si puede mantenerse en red privada.
- Restringir acceso por firewall/red privada y autenticación.
- Registrar accesos y cambios administrativos conforme a la política de auditoría.
- Evitar que errores de conexión filtren secretos o detalles internos al cliente.

## 12. Pruebas obligatorias

Las pruebas deben ejecutarse localmente y en CI con comandos y criterios equivalentes. Deben utilizar PostgreSQL real de prueba para validar comportamiento de SQL, transacciones, locks y RLS; no sustituir estas verificaciones por mocks.

### 12.1 Pruebas unitarias

- Validación de configuración del pool.
- Rechazo de valores inválidos.
- Construcción de configuración por backend/workload.
- Políticas de timeout y clasificación de errores.
- Límites de concurrencia de workers.

### 12.2 Pruebas de integración

- Creación y cierre correcto del pool.
- Adquisición y liberación de conexiones.
- Reutilización de conexiones.
- Timeout cuando el pool está ocupado.
- Recuperación ante caída y restablecimiento de PostgreSQL.
- Rollback ante errores.
- Timeouts de consulta y transacción.
- Permisos de cada rol.
- RLS con tenants diferentes.
- Limpieza del contexto al reutilizar una conexión.
- Comportamiento con cancelación de tareas.

### 12.3 Pruebas de concurrencia y aislamiento

- Muchas peticiones concurrentes sin exceder el máximo configurado.
- Competencia por conexiones y espera acotada.
- Aislamiento entre tenants usando conexiones reutilizadas.
- Ausencia de filtración de contexto después de commit, rollback o cancelación.
- Workers concurrentes sin agotar conexiones de operaciones críticas.
- Contención y locks en operaciones de alto uso.
- Comportamiento ante saturación y recuperación.

### 12.4 Pruebas de carga

Medir al menos:

- Throughput.
- Latencia p50, p95 y p99.
- Uso de CPU y memoria de PostgreSQL.
- Conexiones activas e inactivas.
- Tiempo de espera del pool.
- Timeouts y errores.
- Locks y consultas lentas.
- Impacto de workers sobre endpoints críticos.

Los escenarios y umbrales de aceptación deben definirse a partir de los objetivos de rendimiento y la infraestructura real. No declarar escalabilidad sólo porque el pool fue configurado.

## 13. Despliegue y escalado

Antes de aumentar réplicas:

1. Calcular el presupuesto global de conexiones.
2. Confirmar capacidad de PostgreSQL.
3. Ajustar máximos por proceso si hace falta.
4. Revisar conexiones reservadas para administración y mantenimiento.
5. Ejecutar pruebas de carga representativas.
6. Verificar métricas y alertas.
7. Confirmar que los workers tienen límites independientes.

Cada réplica crea su propio pool. Por tanto, escalar horizontalmente multiplica el máximo potencial de conexiones. El sistema de despliegue debe impedir que un escalado automático incremente conexiones más allá del presupuesto aprobado.

Las migraciones deben ejecutarse mediante un proceso controlado, no una vez por cada réplica de aplicación. La estrategia de migraciones debe evitar carreras entre instancias.

## 14. Criterios de aceptación

La capacidad de connection pooling no se considera terminada hasta que:

- [ ] Cada backend crea su pool durante el arranque.
- [ ] No se crean pools por petición ni por tenant.
- [ ] Cada backend usa identidad y permisos PostgreSQL propios.
- [ ] Los workers tienen límites y presupuesto independientes.
- [ ] Los máximos se calculan considerando todas las réplicas.
- [ ] Existe margen para administración, migraciones y operación.
- [ ] Los timeouts de adquisición, consulta y transacción están configurados.
- [ ] El contexto RLS/tenant es transaccional y no se filtra al reutilizar conexiones.
- [ ] Las transacciones no permanecen abiertas durante llamadas externas o trabajos largos.
- [ ] Existe observabilidad por pool y workload.
- [ ] El agotamiento del pool se maneja de forma controlada.
- [ ] Las pruebas de integración utilizan PostgreSQL real.
- [ ] Existen pruebas de concurrencia, aislamiento, saturación y recuperación.
- [ ] Los comandos locales y CI aplican los mismos criterios.
- [ ] La documentación describe los valores y el presupuesto reales del entorno.
- [ ] El escalado de réplicas no puede exceder silenciosamente la capacidad aprobada.

## 15. Decisiones que deben quedar documentadas

Antes de producción, registrar:

1. Versión exacta de PostgreSQL.
2. Versión y capacidad de la infraestructura PostgreSQL que aloja la única base compartida; schemas y topología lógica por módulo.
3. Roles y permisos por backend.
4. Réplicas máximas por proceso.
5. Máximo y mínimo de conexiones por pool.
6. Presupuesto agregado y margen reservado.
7. Timeouts configurados.
8. Política de RLS y establecimiento del tenant context.
9. Configuración TLS y gestión de credenciales.
10. Si se utiliza PgBouncer, modo y pruebas de compatibilidad.
11. Métricas, dashboards y umbrales de alerta.
12. Resultados de pruebas de carga y saturación.
13. Procedimiento de rotación de credenciales y recuperación.

## 16. Recomendación de implementación por fases

### Fase 0 — Fundamentos

- Definir política de pools por backend y workload.
- Definir ownership y roles PostgreSQL.
- Definir presupuesto global de conexiones.
- Establecer configuración tipada y validada.
- Definir estrategia de secretos y TLS.
- Definir métricas mínimas y criterios de pruebas.
- Documentar cómo se ejecutan las pruebas localmente y en CI.

### Fase de infraestructura de datos

- Implementar la fábrica/configuración de pool compartida como infraestructura común, sin mezclar ownership de dominios.
- Crear un pool independiente en cada backend.
- Crear pools específicos y acotados para workers.
- Integrar timeouts y manejo de errores.
- Integrar contexto de tenant transaccional para RLS.
- Añadir pruebas reales de PostgreSQL, reutilización y aislamiento.

### Antes de producción

- Ejecutar pruebas de carga, concurrencia y saturación.
- Verificar presupuesto agregado con el número máximo de réplicas.
- Verificar observabilidad y alertas.
- Ensayar caída y recuperación de PostgreSQL.
- Evaluar PgBouncer sólo si los resultados justifican introducirlo.
- Documentar valores finales y evidencia de aceptación.

---

## 17. Decisión arquitectónica final

ENSAMBRIA utilizará **una única base de datos PostgreSQL compartida** por los cuatro backends, con schema, migraciones y ownership separados por módulo. Usará SQLx con un pool acotado por proceso/backend, credenciales y permisos independientes por backend, y pools separados y limitados para workers. El presupuesto se calculará globalmente según el número máximo de réplicas y la capacidad real de la infraestructura PostgreSQL.

**PgBouncer será opcional**, incorporado únicamente cuando exista evidencia de que aporta valor y después de validar compatibilidad. El aislamiento multi-tenant no dependerá del pool: se garantizará mediante autorización, ownership, permisos SQL y RLS correctamente aplicado dentro de transacciones.

El objetivo no es maximizar el número de conexiones, sino mantener un número controlado que permita atender la concurrencia prevista sin saturar PostgreSQL ni comprometer el aislamiento entre tenants.
