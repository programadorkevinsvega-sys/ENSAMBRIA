# k6 Performance, Load, Stress and Capacity Testing Rules

**Clasificación:** Estándar de ingeniería obligatorio y no negociable.<br>
**Aplicación:** Desde la Fase 0 y durante todo el ciclo de vida del proyecto.<br>
**Herramienta estándar:** Grafana k6.<br>
**Ámbito:** Frontends, BFF, APIs REST, servicios gRPC mediante protocolos o herramientas compatibles, backends, bases de datos a través de sus interfaces de aplicación, colas, workers, integraciones y plataformas completas.

---

# 1. Propósito

Establecer las reglas obligatorias para diseñar, implementar, ejecutar, analizar y mantener pruebas de rendimiento y capacidad utilizando Grafana k6.

Las pruebas de rendimiento deben ser parte de la ingeniería del producto desde sus primeras fases.

No es aceptable esperar a que el sistema tenga miles de usuarios para descubrir que:

- Las APIs no soportan concurrencia.
- La base de datos se satura.
- Los pools de conexiones se agotan.
- Las consultas se degradan.
- Los workers acumulan retrasos.
- Las colas crecen indefinidamente.
- Los servicios presentan timeouts.
- La latencia aumenta significativamente.
- Los sistemas pierden disponibilidad.
- Los recursos de infraestructura no soportan la demanda.
- Una operación genera duplicados bajo concurrencia.
- Una funcionalidad funciona individualmente, pero falla bajo carga.

El objetivo es demostrar mediante mediciones reproducibles que el sistema puede soportar las cargas previstas sin incumplir sus objetivos de disponibilidad, integridad, rendimiento y seguridad.

# 2. Principios fundamentales no negociables

Todo proyecto debe cumplir los siguientes principios:

1. k6 es la herramienta estándar de pruebas de rendimiento.
2. Las pruebas deben comenzar desde la Fase 0.
3. Toda funcionalidad crítica debe tener escenarios de carga apropiados.
4. Las pruebas deben representar comportamientos reales de usuarios y sistemas.
5. Los escenarios deben tener objetivos de carga explícitos.
6. Las pruebas deben incluir criterios de aprobación y rechazo mediante thresholds.
7. Los resultados deben ser reproducibles y comparables.
8. La carga debe distinguir entre usuarios virtuales, iteraciones y solicitudes HTTP.
9. El generador de carga no debe convertirse en el cuello de botella sin que se detecte.
10. Las pruebas deben ejecutarse en entornos autorizados.
11. No se debe poner en riesgo a usuarios reales mediante pruebas no autorizadas.
12. La integridad de los datos debe verificarse durante y después de las pruebas.
13. Las pruebas deben correlacionarse con métricas de infraestructura y aplicación.
14. Los fallos deben investigarse y corregirse, no ocultarse.
15. No se debe declarar capacidad soportada sin evidencia suficiente.
16. Las pruebas funcionales no sustituyen las pruebas de rendimiento.
17. Un resultado exitoso de k6 no demuestra por sí solo que todo el sistema sea escalable.
18. Las pruebas deben formar parte del proceso de integración y liberación.

# 3. k6 como estándar de pruebas de rendimiento

Todos los proyectos deben utilizar Grafana k6 para las pruebas de rendimiento de protocolos y APIs.

La integración de k6 debe mantenerse en el repositorio, versionada y revisable.

Reglas:

- Fijar una versión conocida de k6 en los entornos de ejecución.
- No depender de versiones flotantes sin control.
- Mantener scripts dentro del repositorio.
- Mantener dependencias controladas.
- Separar escenarios de negocio y perfiles de carga.
- Reutilizar módulos de autenticación, datos y validación.
- Mantener configuraciones por entorno.
- Evitar scripts monolíticos.
- No almacenar credenciales dentro de los scripts.
- No depender de configuraciones locales no documentadas.
- Ejecutar las pruebas de manera reproducible.

Las funcionalidades de k6 deben utilizarse según la versión instalada y las capacidades verificadas de esa versión.

No asumir compatibilidad con extensiones, módulos o servicios externos sin validarla.

# 4. Pruebas desde la Fase 0

Las pruebas de rendimiento deben planificarse desde la primera fase del proyecto.

## 4.1. Fase 0: preparación

Antes de implementar funcionalidades críticas deben definirse:

- Herramienta de carga.
- Estructura de pruebas.
- Entornos autorizados.
- Datos sintéticos desechables, no personales y generados para la ejecución mediante los flujos reales de la aplicación.
- Estrategia de autenticación para pruebas.
- Modelo de carga.
- Métricas.
- Formato de resultados.
- Criterios de aceptación.
- Integración con CI.
- Procedimiento de ejecución.
- Límites de seguridad.
- Proceso de limpieza de datos.

No es necesario generar carga elevada sobre una aplicación que todavía no está construida.

Sí es obligatorio tener preparado el estándar, la infraestructura de pruebas y los criterios que se utilizarán cuando las funcionalidades estén disponibles.

## 4.2. Durante la implementación

Toda funcionalidad relevante debe incorporar escenarios de rendimiento cuando corresponda.

Ejemplos:

- Inicio de sesión.
- Renovación de sesión.
- Consulta de datos.
- Búsquedas.
- Listados paginados.
- Operaciones de escritura.
- Creación de reservas.
- Gestión de disponibilidad.
- Creación de órdenes.
- Procesamiento de pagos en sandbox.
- Gestión de clientes.
- Consultas multi-tenant.
- Procesamiento de eventos.
- Publicación y consumo de mensajes.
- Procesamiento de trabajos asíncronos.
- Operaciones administrativas.

La cobertura debe responder al riesgo, al volumen esperado y a la importancia del caso de uso.

## 4.3. Antes de liberar una funcionalidad

Las funcionalidades críticas deben contar con resultados de pruebas acordes a su perfil de carga.

No se deben liberar capacidades que incumplan los objetivos de rendimiento aprobados sin una excepción formal, explícita y temporal.

# 5. Tipos de pruebas obligatorias

El proyecto debe seleccionar los tipos de pruebas necesarios de acuerdo con su arquitectura, funcionalidades y riesgos.

## 5.1. Smoke Performance Test

Objetivo:

Comprobar que los scripts funcionan y que el sistema responde ante una carga mínima.

Debe verificar:

- Conectividad.
- Autenticación.
- Respuestas esperadas.
- Flujo básico.
- Ausencia de errores graves.
- Correcta ejecución del script.
- Recepción de métricas.

La prueba debe ser suficientemente pequeña para utilizarse en validaciones frecuentes.

No debe considerarse evidencia de capacidad.

## 5.2. Load Test — Prueba de carga normal

Objetivo:

Determinar el comportamiento del sistema bajo una carga normal o esperada.

Debe medir:

- Latencia.
- Percentiles.
- Tasa de errores.
- Throughput.
- Concurrencia.
- Utilización de recursos.
- Comportamiento de dependencias.
- Estabilidad durante el período evaluado.

El perfil de carga debe construirse a partir de estimaciones justificadas, telemetría disponible o requisitos de capacidad aprobados.

No inventar números de usuarios o solicitudes por segundo.

## 5.3. Stress Test — Prueba de estrés

Objetivo:

Evaluar el comportamiento del sistema cuando la demanda supera gradualmente su carga esperada.

Debe permitir identificar:

- Punto de degradación.
- Saturación de recursos.
- Aumento de latencia.
- Agotamiento de pools.
- Errores.
- Timeouts.
- Rechazo de solicitudes.
- Acumulación de trabajos.
- Pérdida de capacidad.
- Comportamiento de recuperación.

El incremento de carga debe realizarse de manera controlada.

La prueba debe identificar si el sistema se degrada de manera progresiva o presenta fallos abruptos.

## 5.4. Spike Test — Prueba de picos

Objetivo:

Comprobar cómo responde el sistema ante incrementos repentinos de demanda.

Debe simular situaciones como:

- Campañas comerciales.
- Aperturas de reservas.
- Notificaciones masivas.
- Eventos de alto tráfico.
- Incrementos repentinos de sesiones.
- Picos de búsquedas.
- Acceso simultáneo a funcionalidades populares.

Debe analizarse:

- Capacidad de absorción.
- Escalado.
- Latencia.
- Errores.
- Colas.
- Recuperación después del pico.
- Comportamiento de caché.
- Disponibilidad.

## 5.5. Soak Test — Prueba de resistencia prolongada

Objetivo:

Verificar la estabilidad durante períodos prolongados de carga sostenida.

Debe detectar:

- Fugas de memoria.
- Acumulación de recursos.
- Conexiones no liberadas.
- Pools degradados.
- Crecimiento continuo de colas.
- Acumulación de tareas.
- Procesos bloqueados.
- Degradación progresiva.
- Agotamiento de recursos.
- Problemas de limpieza.

La duración debe ser suficiente para revelar comportamientos que no aparecen en pruebas cortas.

La duración se determinará según el riesgo, los recursos y los ciclos operativos del sistema.

## 5.6. Breakpoint Test — Identificación del punto de capacidad

Objetivo:

Determinar el punto aproximado en el que el sistema deja de cumplir sus objetivos de rendimiento.

Debe identificar:

- Máximo throughput sostenible.
- Concurrencia soportada.
- Latencias bajo carga.
- Recursos saturados.
- Tasa de errores.
- Punto de degradación.
- Capacidad de recuperación.

Debe realizarse en entornos aislados y con autorización específica.

No debe confundirse el máximo volumen de solicitudes que consigue emitir k6 con la capacidad sostenible del sistema.

## 5.7. Scalability Test — Prueba de escalabilidad

Objetivo:

Comprobar cómo cambia el rendimiento al aumentar recursos o instancias.

Debe comparar:

- Capacidad antes y después del escalado.
- Throughput.
- Latencia.
- Utilización de CPU y memoria.
- Conexiones a bases de datos.
- Consumo de workers.
- Longitud de colas.
- Distribución de carga.
- Coste de recursos, cuando se mida.

No asumir escalabilidad lineal.

Debe verificarse que aumentar instancias realmente produce una mejora útil y que no desplaza el cuello de botella hacia PostgreSQL, Redis, NATS, gateways u otros servicios.

## 5.8. Performance Regression Test

Objetivo:

Detectar degradaciones respecto a una línea base conocida.

Comparar:

- Latencia p50.
- Latencia p90.
- Latencia p95.
- Latencia p99.
- Tasa de errores.
- Throughput.
- Iteraciones descartadas.
- Consumo de recursos.
- Tiempos de consultas.
- Capacidad de procesamiento.

Las comparaciones deben realizarse con el mismo escenario, configuración de carga, datos y entorno o con diferencias expresamente documentadas.

# 6. Modelo de carga

El diseño del modelo de carga debe representar correctamente el comportamiento del sistema.

No se debe confundir el número de usuarios virtuales con el número de solicitudes por segundo.

## 6.1. Virtual Users

Un Virtual User representa una ejecución virtual del flujo definido en el script.

Un VU puede realizar:

- Una solicitud.
- Varias solicitudes.
- Una secuencia de operaciones.
- Una navegación.
- Una operación de negocio completa.

Por tanto:

**VUs no equivalen a RPS.**

El número de solicitudes depende de las iteraciones, solicitudes por iteración, duración, pausas y comportamiento del script.

## 6.2. Iterations

Una iteración representa una ejecución del código de usuario definido para el escenario.

Debe documentarse:

- Qué representa una iteración.
- Cuántas solicitudes puede realizar.
- Cuáles son las operaciones incluidas.
- Cuánto tiempo espera entre operaciones.
- Qué datos utiliza.
- Qué resultado funcional debe producir.

No utilizar iteraciones como sinónimo de usuarios reales.

## 6.3. Requests Per Second

RPS representa solicitudes HTTP por segundo.

Debe medirse a partir de las solicitudes realmente generadas.

Una iteración puede generar varias solicitudes, por lo que la tasa de iteraciones no debe presentarse como RPS sin calcular y verificar la relación.

## 6.4. Modelo cerrado y modelo abierto

La elección del modelo de carga debe responder al comportamiento que se desea medir.

**Modelo cerrado:**

El número de usuarios virtuales se mantiene controlado. Los usuarios ejecutan iteraciones y vuelven a comenzar según el tiempo que tarda cada iteración.

Se utiliza cuando interesa representar un conjunto de usuarios concurrentes cuyo ritmo de solicitudes depende de la respuesta del sistema.

Executors apropiados según el caso:

- `constant-vus`.
- `ramping-vus`.

**Modelo abierto:**

Las iteraciones se programan según una tasa de llegada, independientemente de la velocidad de respuesta del sistema.

Se utiliza cuando interesa comprobar cómo responde el sistema ante una demanda de llegada determinada.

Executors apropiados:

- `constant-arrival-rate`.
- `ramping-arrival-rate`.

No elegir un modelo solamente porque sea más fácil de configurar.

Documentar por qué representa la carga real o el objetivo experimental.

## 6.5. Selección de executors

Utilizar los executors según su propósito:

| Executor | Uso |
|---|---|
| `shared-iterations` | Cantidad total fija de iteraciones compartidas |
| `per-vu-iterations` | Cantidad determinada de iteraciones por VU |
| `constant-vus` | Concurrencia estable |
| `ramping-vus` | Incremento o reducción gradual de VUs |
| `constant-arrival-rate` | Tasa fija de inicio de iteraciones |
| `ramping-arrival-rate` | Tasa de llegada variable |

No se permite utilizar un executor sin comprender sus características y límites.

## 6.6. Think time y pausas

Los usuarios reales no ejecutan operaciones sin pausas durante toda su sesión.

Los escenarios deben representar tiempos de interacción razonables cuando corresponda.

Reglas:

- Modelar pausas entre acciones de usuario.
- Evitar pausas arbitrarias sin justificación.
- No insertar `sleep()` al final de una iteración de arrival-rate para intentar controlar su ritmo.
- No utilizar esperas artificiales que oculten la capacidad real.
- Separar escenarios de navegación interactiva de pruebas específicas de throughput.

El comportamiento de las pausas debe documentarse.

# 7. Diseño de escenarios de negocio

Las pruebas deben representar procesos de negocio completos, no únicamente solicitudes aisladas.

Ejemplos:

## Plataforma SaaS multi-tenant

- Autenticación.
- Selección de organización.
- Cambio de sucursal.
- Consulta de dashboard.
- Búsqueda de clientes.
- Consulta de servicios.
- Consulta de disponibilidad.
- Creación de reservas.
- Modificación de reservas.
- Cancelación de reservas.
- Consultas administrativas.

## Ecommerce

- Consulta de catálogo.
- Búsqueda de productos.
- Consulta de detalle.
- Gestión de carrito.
- Inicio de checkout.
- Creación de orden.
- Consulta de estado.
- Simulación de pago en sandbox.

## Sistemas financieros

- Creación de intenciones de pago.
- Consulta de estados.
- Idempotencia.
- Eventos normalizados.
- Procesamiento asíncrono.
- Consulta de transacciones.
- Conciliación.

No ejecutar operaciones financieras reales durante pruebas de carga ordinarias.

## Sistemas distribuidos

- Publicación de eventos.
- Procesamiento de workers.
- Consumo de mensajes.
- Persistencia.
- Confirmaciones.
- Reintentos.
- Recuperación.
- Gestión de duplicados.

Las pruebas deben comprobar también los efectos posteriores de las operaciones, no limitarse a verificar que el endpoint respondió correctamente.

# 8. Organización de los scripts

Los scripts deben mantenerse dentro del repositorio y seguir una estructura modular.

Una estructura orientativa es:

```text
tests/
└── performance/
    ├── config/
    │   ├── environments.js
    │   └── thresholds.js
    ├── data/
    │   └── README.md
    ├── lib/
    │   ├── authentication.js
    │   ├── requests.js
    │   ├── assertions.js
    │   ├── metrics.js
    │   └── test-data.js
    ├── scenarios/
    │   ├── authentication.js
    │   ├── tenant-operations.js
    │   ├── reservations.js
    │   └── payments.js
    ├── workloads/
    │   ├── smoke.js
    │   ├── load.js
    │   ├── stress.js
    │   ├── spike.js
    │   └── soak.js
    ├── reports/
    │   └── .gitkeep
    └── README.md
```

Esta estructura es orientativa y debe adaptarse a la arquitectura real del repositorio.

Reglas:

- Separar escenarios de negocio y perfiles de carga.
- Compartir funciones reutilizables.
- Centralizar configuración.
- Centralizar thresholds cuando sea apropiado.
- Mantener datos de prueba independientes.
- Evitar repetir lógica de autenticación.
- Evitar scripts monolíticos.
- Evitar dependencias innecesarias.
- Versionar la documentación de cada escenario.
- No almacenar reportes temporales o datos sensibles en Git.

# 9. Datos de prueba

Los datos son determinantes para la validez de una prueba de rendimiento.

Reglas obligatorias:

1. Utilizar datos sintéticos o expresamente autorizados.
2. No utilizar información personal productiva sin autorización y controles.
3. Preparar datos de prueba antes de ejecutar cargas importantes.
4. Garantizar que los datos sean suficientes para la carga prevista.
5. Evitar que todos los VUs utilicen exactamente el mismo registro cuando ello distorsione el comportamiento.
6. Evitar colisiones accidentales.
7. Utilizar identificadores únicos para operaciones de escritura.
8. Mantener separación entre datasets y entornos.
9. Documentar preparación y limpieza.
10. Verificar integridad después de cada ejecución.

En ENSAMBRIA, “datos sintéticos” significa registros de prueba no personales, únicos por ejecución y creados/eliminados mediante las APIs y casos de uso reales en un entorno de prueba aislado y autorizado. Esta excepción existe para medir comportamiento y concurrencia; no permite fixtures precargados, bases simuladas, fakes, stubs, payloads de éxito artificiales ni omitir la aplicación. La preparación, el aislamiento, la autorización, la verificación de integridad y la limpieza se documentan antes de ejecutar. Si la limpieza falla, la ejecución debe reportarse como incompleta y se bloquean nuevas cargas hasta resolverlo.

## 9.1. Aislamiento de datos

En entornos multi-tenant:

- Utilizar tenants dedicados a pruebas.
- Separar usuarios de prueba.
- Mantener datos independientes.
- No compartir sesiones por accidente.
- Evitar colisiones de reservas, órdenes o registros.
- Comprobar aislamiento entre tenants.
- No generar escrituras sobre registros de usuarios reales.

## 9.2. Escrituras concurrentes

Cuando varios VUs escriban simultáneamente:

- Definir la estrategia de asignación de datos.
- Utilizar cuentas apropiadas.
- Evitar colisiones no intencionadas.
- Definir claves de idempotencia únicas por operación.
- Comprobar que no se creen duplicados.
- Comprobar que no se pierdan actualizaciones.
- Comprobar integridad de relaciones.
- Verificar resultados después de la prueba.

Si se busca evaluar conflictos sobre el mismo recurso, la colisión debe ser intencional y estar documentada.

## 9.3. Limpieza

Toda prueba que cree datos debe establecer un procedimiento de limpieza.

La limpieza debe ser:

- Segura.
- Idempotente.
- Limitada a los datos de prueba.
- Verificable.
- Independiente de datos reales.

No confiar exclusivamente en `teardown()` para garantizar limpieza ante cualquier fallo del proceso.

Debe existir un procedimiento externo de recuperación o limpieza para ejecuciones interrumpidas.

# 10. Seguridad en la ejecución de pruebas

Las pruebas de carga pueden afectar la disponibilidad de sistemas y servicios.

Por ello, su ejecución debe estar controlada.

## 10.1. Autorización

Está prohibido ejecutar pruebas de carga sobre:

- Sistemas de terceros sin autorización.
- APIs públicas ajenas.
- Proveedores externos.
- Infraestructura no administrada.
- Dominios cuya autorización no esté confirmada.

Todo objetivo debe estar incluido en una lista de destinos autorizados.

## 10.2. Producción

Las pruebas de producción deben estar sujetas a autorización explícita.

Reglas:

- No ejecutar stress tests en producción sin aprobación específica.
- No ejecutar spike tests agresivos sin aprobación específica.
- No realizar breakpoint tests en producción.
- No ejecutar pruebas destructivas.
- No crear transacciones financieras reales.
- No enviar correos, SMS o notificaciones reales de forma masiva.
- No generar órdenes o reservas reales.
- No afectar usuarios ajenos a la prueba.
- No evadir WAF, rate limiting o controles de seguridad para aumentar la carga.
- No omitir mecanismos de protección del sistema.

En producción, las pruebas deben ser controladas, de bajo riesgo y con criterios claros de interrupción.

## 10.3. Protección de credenciales

Las credenciales de prueba deben proporcionarse mediante secretos o variables de entorno del sistema de ejecución.

Queda prohibido:

- Incluir tokens en scripts.
- Incluir contraseñas en repositorios.
- Registrar credenciales.
- Publicar tokens en reportes.
- Compartir archivos de autenticación entre entornos.
- Reutilizar credenciales productivas personales.

## 10.4. Protección de servicios externos

Para proveedores de pago, correo, SMS, mapas y otros servicios externos:

- Utilizar entornos sandbox cuando estén disponibles.
- Utilizar simuladores autorizados cuando sea necesario.
- Controlar límites y costes.
- Evitar solicitudes reales masivas.
- Documentar los servicios simulados.
- Diferenciar capacidad del sistema propio y capacidad del proveedor externo.

Una prueba con un proveedor simulado debe identificarse como tal; no demuestra que el proveedor real soporte la misma carga.

# 11. Thresholds y criterios de aceptación

Los thresholds constituyen los criterios automáticos de aprobación o rechazo de una prueba.

Todo escenario importante debe definir thresholds.

No se considera suficiente ejecutar k6 y observar gráficas sin establecer criterios de aceptación.

## 11.1. Principio de thresholds

Los thresholds deben derivarse de:

- Requisitos funcionales.
- Objetivos de rendimiento.
- SLO aprobados.
- Contratos de servicio.
- Presupuestos de latencia.
- Capacidad de infraestructura.
- Prioridad de la operación.
- Riesgo del negocio.

Está prohibido inventar objetivos universales para todos los endpoints.

No todas las operaciones tienen el mismo presupuesto de latencia.

Por ejemplo, una consulta de lectura, una búsqueda compleja, una operación financiera y un procesamiento asíncrono pueden tener objetivos distintos.

## 11.2. Métricas mínimas

Según el escenario, evaluar:

- `http_req_duration`
- `http_req_failed`
- `http_reqs`
- `iterations`
- `iteration_duration`
- `vus`
- `vus_max`
- `dropped_iterations`
- `data_received`
- `data_sent`
- `checks`
- Métricas personalizadas.

También deben recopilarse métricas de aplicación e infraestructura.

## 11.3. Percentiles

Los promedios no son suficientes para evaluar el rendimiento.

Deben analizarse los percentiles correspondientes.

Como mínimo, para operaciones relevantes:

- p50.
- p90.
- p95.
- p99.

El p95 y p99 son fundamentales para detectar degradaciones que afectan a una parte de los usuarios.

No ocultar colas de latencia mediante promedios.

## 11.4. Checks y Thresholds

Los checks validan el resultado de cada solicitud o condición.

Los thresholds establecen criterios globales de aprobación de métricas.

Ambos deben utilizarse de forma complementaria.

Reglas:

- Validar códigos de respuesta.
- Validar contenido esperado.
- Validar estructuras importantes.
- Validar identificadores.
- Validar estados de negocio.
- Validar ausencia de duplicados cuando corresponda.
- Definir thresholds sobre las métricas relevantes.
- Hacer que los criterios de aceptación fallen de forma visible en CI cuando no se cumplan.

No asumir que un check fallido por sí solo hará que k6 termine con estado de ejecución fallido. Deben existir thresholds apropiados para convertir el criterio funcional en una condición de aprobación del test.

## 11.5. Thresholds por escenario y endpoint

Los criterios deben poder diferenciar:

- Escenarios.
- Endpoints.
- Operaciones.
- Tipos de solicitudes.
- Servicios.
- Flujos de negocio.

Utilizar tags y grupos con cardinalidad controlada.

No utilizar identificadores personales, tokens ni valores de alta cardinalidad como tags.

## 11.6. Iteraciones descartadas

Las iteraciones descartadas deben analizarse obligatoriamente.

La métrica `dropped_iterations` puede indicar:

- VUs insuficientes.
- Configuración incorrecta.
- Agotamiento de capacidad del generador.
- Degradación del sistema.
- Imposibilidad de mantener la tasa de llegada configurada.

No se debe presentar como exitosa una prueba de arrival-rate que no pudo mantener la carga solicitada.

Los thresholds deben establecer el nivel aceptable de iteraciones descartadas según el objetivo de la prueba.

# 12. Línea base de rendimiento

Toda funcionalidad crítica debe disponer de una línea base de rendimiento cuando el entorno permita medirla.

La línea base debe registrar:

- Fecha.
- Versión.
- Commit.
- Entorno.
- Infraestructura.
- Dataset.
- Perfil de carga.
- Número de VUs o tasa de llegada.
- Duración.
- Throughput.
- Percentiles.
- Errores.
- Iteraciones descartadas.
- Recursos utilizados.
- Thresholds.
- Resultado final.

Las comparaciones deben realizarse con escenarios equivalentes.

Si cambia el perfil de carga, dataset o infraestructura, debe registrarse la diferencia.

No comparar directamente resultados obtenidos bajo configuraciones incompatibles.

# 13. Observabilidad durante pruebas

Una prueba de rendimiento no debe limitarse a observar los resultados de k6.

Debe analizarse el comportamiento interno del sistema mientras recibe carga.

Como mínimo, instrumentar cuando corresponda:

## 13.1. Aplicaciones y servicios

- Latencia de endpoints.
- Errores.
- Throughput.
- Concurrencia.
- Timeouts.
- Saturación.
- Operaciones bloqueadas.
- Reintentos.
- Procesamiento asíncrono.

## 13.2. PostgreSQL

- CPU.
- Memoria.
- Conexiones.
- Conexiones activas e inactivas.
- Esperas.
- Locks.
- Deadlocks.
- Consultas lentas.
- Tiempo de ejecución.
- Saturación de pools.
- Transacciones abiertas.
- Bloat y otros indicadores relevantes.

## 13.3. Redis

Cuando esté presente:

- CPU.
- Memoria.
- Latencia.
- Hit ratio.
- Evictions.
- Conexiones.
- Operaciones por segundo.
- Bloqueos.
- Persistencia.
- Saturación.

## 13.4. Mensajería y workers

Cuando existan:

- Mensajes publicados.
- Mensajes procesados.
- Lag.
- Profundidad de colas.
- Reintentos.
- Mensajes fallidos.
- Tiempo de procesamiento.
- Workers activos.
- Errores.
- Duplicados.
- Recuperación.

## 13.5. Infraestructura

- CPU.
- Memoria.
- Disco.
- IOPS.
- Red.
- Conexiones.
- Balanceadores.
- Proxies.
- CDN.
- Contenedores o procesos.
- Reinicios.
- Errores del sistema operativo.

Las métricas deben permitir correlacionar la carga generada con el comportamiento del sistema.

Utilizar los sistemas de observabilidad existentes. No introducir herramientas adicionales sin una justificación técnica.

# 14. Integración con observabilidad distribuida

Cuando la arquitectura disponga de trazas distribuidas:

- Correlacionar ejecuciones k6 con servicios backend.
- Propagar contexto de trazas cuando corresponda.
- Identificar solicitudes relevantes.
- Correlacionar errores con trazas.
- Diferenciar los datos de pruebas de los datos de usuarios reales.
- Mantener cardinalidad controlada.
- No introducir credenciales ni información sensible en atributos.

La instrumentación debe permitir determinar si la degradación proviene del gateway, BFF, backend, base de datos, mensajería, workers o proveedor externo.

# 15. Integración de pruebas web y navegador

Las pruebas de carga de APIs y las pruebas reales de navegador tienen objetivos diferentes.

Para sitios y aplicaciones web se deben distinguir:

1. Pruebas de protocolo HTTP.
2. Pruebas de navegador.
3. Pruebas híbridas.

## 15.1. Protocol-level testing

Utilizar pruebas HTTP y de protocolos para generar la mayor parte de la carga.

Son apropiadas para evaluar:

- APIs.
- Endpoints.
- Flujos.
- Autenticación.
- Backend.
- Throughput.
- Concurrencia.
- Servicios.

## 15.2. Browser-based testing

Utilizar el módulo de navegador de k6 cuando sea necesario medir:

- Navegación real.
- Carga de páginas.
- Renderizado.
- Interacciones.
- Experiencia de usuario.
- Core Web Vitals.
- Comportamiento visual.

Las pruebas de navegador no deben sustituir las pruebas de protocolo para generar grandes volúmenes de carga.

El consumo de recursos de cada navegador virtual debe tenerse en cuenta.

## 15.3. Hybrid testing

Cuando sea necesario combinar experiencia de navegador y volumen de carga:

- Utilizar solicitudes de protocolo para generar la mayor parte de la carga.
- Utilizar una cantidad controlada de usuarios de navegador.
- Separar las métricas.
- Mantener thresholds adecuados para cada tipo de prueba.
- No mezclar resultados incompatibles.

## 15.4. Core Web Vitals

Cuando se realicen pruebas de navegador, evaluar las métricas que k6 Browser proporcione y su compatibilidad con la versión utilizada.

Considerar:

- LCP.
- INP.
- CLS.

Para datos reales de usuarios, contrastar con la telemetría de producción y los sistemas de observabilidad frontend.

Una prueba sintética no sustituye las mediciones de usuarios reales.

# 16. Pruebas de sistemas distribuidos

Los sistemas distribuidos deben probarse bajo concurrencia y carga.

Debe evaluarse:

- Comunicación entre servicios.
- Latencia de red.
- Reintentos.
- Timeouts.
- Circuit breakers.
- Saturación.
- Colas.
- Procesamiento asíncrono.
- Idempotencia.
- Recuperación.
- Duplicados.
- Integridad transaccional.
- Comportamiento bajo fallos parciales.

En arquitecturas con Outbox e Inbox:

- Medir tiempo desde la creación del evento hasta su publicación.
- Medir tiempo de consumo.
- Medir backlog.
- Medir reintentos.
- Medir duplicados.
- Verificar consistencia.
- Verificar que los eventos no se pierdan.
- Verificar recuperación después de reinicios.

Una respuesta HTTP exitosa no demuestra que el procesamiento asíncrono se haya completado correctamente.

# 17. Pruebas de concurrencia e integridad

Las pruebas de rendimiento deben contemplar escenarios donde múltiples usuarios ejecutan operaciones simultáneamente.

Ejemplos:

- Dos usuarios reservando el mismo horario.
- Múltiples solicitudes sobre el mismo recurso.
- Operaciones repetidas con la misma clave de idempotencia.
- Diferentes claves sobre recursos relacionados.
- Cambios simultáneos.
- Múltiples workers procesando el mismo evento.
- Actualizaciones concurrentes.
- Consultas y escrituras simultáneas.

Se debe verificar:

- Ausencia de duplicados indebidos.
- Integridad de datos.
- Cumplimiento de restricciones.
- Idempotencia.
- Aislamiento entre tenants.
- Control de concurrencia.
- Ausencia de condiciones de carrera.
- Manejo correcto de conflictos.

El sistema debe mantener la corrección funcional aunque reciba carga elevada.

# 18. Capacity Planning

Todo proyecto debe establecer una estrategia de planificación de capacidad.

La capacidad se debe estimar a partir de:

- Usuarios esperados.
- Usuarios concurrentes.
- Sesiones.
- Frecuencia de operaciones.
- Distribución de tráfico.
- Tamaño de solicitudes.
- Procesamiento asíncrono.
- Datos almacenados.
- Consultas.
- Dependencias.
- Infraestructura disponible.
- Objetivos de crecimiento.

No utilizar el número total de cuentas registradas como equivalente al número de usuarios concurrentes.

Diferenciar:

- Usuarios registrados.
- Usuarios activos.
- Usuarios concurrentes.
- Solicitudes por segundo.
- Iteraciones por segundo.
- Operaciones de negocio por segundo.
- Mensajes procesados por segundo.

Las estimaciones deben documentarse y revisarse conforme se obtengan datos reales.

No declarar una capacidad máxima sin indicar el perfil de carga, entorno y condiciones bajo los cuales se obtuvo.

# 19. Capacidad del generador de carga

El generador k6 debe tener capacidad suficiente para producir la carga requerida.

Debe monitorearse:

- CPU.
- RAM.
- Red.
- Conexiones.
- Descriptores de archivo.
- Recursos del sistema operativo.
- Limitaciones del proceso.
- Errores internos de k6.

Si el generador se satura, los resultados pueden dejar de representar el comportamiento del sistema evaluado.

Como referencia operativa, se debe conservar margen de CPU y memoria en el generador; una recomendación de la documentación de k6 es mantener aproximadamente un 20% de CPU libre y evitar que la memoria utilizada se acerque al límite físico.

Cuando se requiera carga distribuida:

- Utilizar segmentos de ejecución compatibles.
- Documentar la distribución.
- Evitar duplicar accidentalmente la carga.
- Coordinar los datasets.
- Consolidar los resultados.
- Identificar la ubicación de cada generador.
- Verificar que la red no sea el cuello de botella.

No modificar límites del sistema operativo de forma permanente sin medir, justificar y documentar el cambio.

# 20. CI y automatización

Las pruebas k6 deben integrarse en el ciclo de ingeniería.

No todas las pruebas tienen que ejecutarse en cada Pull Request. La frecuencia debe responder a su duración, coste, riesgo y capacidad de aislamiento.

## 20.1. Pull Requests

Los PR deben ejecutar pruebas de rendimiento ligeras cuando exista un entorno apropiado.

Se recomienda:

- Validar sintaxis de scripts.
- Validar configuración.
- Ejecutar smoke tests.
- Ejecutar escenarios de bajo volumen.
- Comprobar thresholds iniciales.
- Detectar regresiones evidentes.

No ejecutar stress tests o breakpoint tests pesados indiscriminadamente en cada PR.

## 20.2. Integración programada

Las pruebas de carga y regresión deben ejecutarse con una frecuencia planificada.

La programación debe considerar:

- Evolución del sistema.
- Riesgo.
- Frecuencia de cambios.
- Disponibilidad de entornos.
- Costes.
- Capacidad de los generadores.

Los resultados deben almacenarse y compararse históricamente.

## 20.3. Pre-release

Antes de releases relevantes se deben ejecutar las pruebas apropiadas para validar:

- Carga normal.
- Estrés.
- Picos.
- Resistencia prolongada cuando corresponda.
- Flujos críticos.
- Capacidad de recuperación.
- Integridad de datos.

Las pruebas de mayor impacto deben realizarse en entornos controlados y con supervisión apropiada.

## 20.4. Resultado de CI

Un threshold incumplido debe producir un resultado de prueba fallido cuando constituya un criterio obligatorio.

Está prohibido:

- Ignorar el fallo.
- Ocultar el resultado.
- Desactivar thresholds.
- Reducir artificialmente la carga.
- Cambiar la configuración sin justificarlo.
- Marcar como exitoso un test que no alcanzó su carga programada.

Las excepciones deben documentarse y aprobarse.

# 21. Resultados y reportes

Toda ejecución importante debe producir un informe.

El informe debe incluir:

- Identificador de ejecución.
- Fecha.
- Commit.
- Versión de la aplicación.
- Versión de k6.
- Entorno.
- Región del generador.
- Perfil de carga.
- Escenarios.
- Duración.
- VUs máximos.
- Tasa de llegada.
- Throughput.
- Latencias p50, p90, p95 y p99.
- Tasa de errores.
- Checks.
- Iteraciones descartadas.
- Recursos.
- Thresholds.
- Resultado.
- Anomalías.
- Limitaciones.
- Recomendaciones.

Los resultados deben permitir reproducir la ejecución.

No almacenar datos personales, tokens ni credenciales en los reportes.

Los artefactos de CI deben tener políticas de retención apropiadas.

# 22. Repetibilidad y confiabilidad estadística

Una conclusión de rendimiento no debe basarse en una única ejecución aislada cuando la decisión tenga un impacto significativo.

Reglas:

- Repetir pruebas importantes.
- Mantener constante la configuración.
- Documentar variaciones.
- Separar calentamiento de medición cuando corresponda.
- Identificar condiciones de caché.
- Mantener datasets equivalentes.
- Registrar cambios de infraestructura.
- Comparar resultados equivalentes.
- Investigar variaciones anormales.

Para decisiones de capacidad o liberaciones críticas, realizar ejecuciones repetidas y evaluar la variabilidad de los resultados.

No ocultar variaciones entre ejecuciones.

# 23. Recuperación después de pruebas

Toda prueba de carga debe finalizar con una evaluación de recuperación.

Comprobar:

- Que los servicios siguen saludables.
- Que las conexiones se liberaron.
- Que las colas regresan a niveles normales.
- Que no quedaron procesos bloqueados.
- Que no existen datos de prueba inconsistentes.
- Que no quedaron transacciones abiertas.
- Que los recursos regresan a niveles esperados.
- Que no existen errores persistentes.
- Que las dependencias mantienen disponibilidad.

No comenzar una nueva prueba importante si la ejecución anterior dejó el sistema en un estado degradado.

# 24. Gestión de fallos y límites de interrupción

Toda prueba de alto impacto debe contar con criterios de interrupción.

Establecer antes de iniciar:

- Máximo de carga autorizado.
- Duración máxima.
- Umbrales de error.
- Umbrales de latencia.
- Recursos máximos.
- Condiciones de saturación.
- Responsable de supervisión.
- Procedimiento de cancelación.
- Procedimiento de recuperación.

Cuando corresponda, utilizar thresholds con opciones de interrupción como `abortOnFail`.

No continuar una prueba si el sistema presenta condiciones que ponen en riesgo su integridad o disponibilidad.

El objetivo es conocer los límites del sistema de forma controlada, no provocar una caída por falta de supervisión.

# 25. Prohibiciones no negociables

Queda prohibido:

1. Ejecutar pruebas contra objetivos sin autorización.
2. Ejecutar cargas destructivas en producción sin aprobación específica.
3. Utilizar datos personales reales sin autorización.
4. Incluir credenciales en los scripts.
5. Ejecutar pruebas sin definir el objetivo de carga.
6. Confundir VUs con usuarios reales.
7. Confundir iteraciones con solicitudes por segundo.
8. Presentar resultados de una prueba de baja carga como evidencia de capacidad.
9. Omitir thresholds en pruebas que deben aprobarse mediante CI.
10. Ignorar `dropped_iterations`.
11. Ocultar fallos de checks.
12. Ignorar errores funcionales.
13. Ejecutar escrituras masivas sin control de datos.
14. Generar pagos reales durante pruebas ordinarias.
15. Ejecutar stress tests sobre terceros.
16. Saturar servicios externos sin autorización.
17. Modificar la carga para conseguir resultados artificialmente positivos.
18. Declarar exitosas pruebas que no alcanzaron el perfil programado.
19. Afirmar que el sistema soporta una cantidad de usuarios sin evidencia.
20. Ignorar la saturación del generador.
21. Comparar resultados de escenarios incompatibles.
22. Ignorar la recuperación después de una prueba.
23. Eliminar resultados fallidos para mejorar estadísticas.
24. Desactivar validaciones de rendimiento para facilitar releases.
25. Confundir una respuesta HTTP exitosa con el éxito completo de una operación de negocio.

# 26. Definition of Done de pruebas de rendimiento

Una funcionalidad crítica se considera validada en rendimiento cuando se cumplen los criterios aplicables.

## Diseño

- [ ] Escenario de negocio identificado.
- [ ] Modelo de carga definido.
- [ ] Executor apropiado.
- [ ] Perfil de carga documentado.
- [ ] Datos de prueba preparados.
- [ ] Entorno autorizado.
- [ ] Umbrales aprobados.

## Implementación

- [ ] Scripts versionados.
- [ ] Código modular.
- [ ] Autenticación segura.
- [ ] Datos parametrizados.
- [ ] Checks funcionales.
- [ ] Thresholds definidos.
- [ ] Métricas personalizadas cuando corresponda.
- [ ] Limpieza de datos definida.

## Ejecución

- [ ] Generador validado.
- [ ] Carga realmente alcanzada.
- [ ] `dropped_iterations` evaluado.
- [ ] Métricas recopiladas.
- [ ] Infraestructura observada.
- [ ] Resultados guardados.
- [ ] Errores investigados.

## Verificación

- [ ] Latencias dentro de objetivos.
- [ ] Tasa de errores dentro de objetivos.
- [ ] Throughput esperado.
- [ ] Concurrencia adecuada.
- [ ] Integridad de datos verificada.
- [ ] Sin duplicados indebidos.
- [ ] Sin degradación crítica.
- [ ] Recuperación comprobada.

## Gobierno

- [ ] Resultados documentados.
- [ ] Comparación con línea base.
- [ ] Limitaciones identificadas.
- [ ] Riesgos registrados.
- [ ] CI integrado cuando corresponda.
- [ ] Aprobación de responsables cuando sea necesaria.

No se puede declarar capacidad validada cuando la carga solicitada no fue realmente alcanzada o cuando no existen métricas suficientes para interpretar el resultado.

# 27. Responsabilidades de los agentes de inteligencia artificial

Los agentes que participen en el desarrollo deben cumplir estas reglas.

El agente de arquitectura debe:

- Identificar componentes críticos.
- Determinar dependencias.
- Identificar posibles cuellos de botella.
- Definir los flujos relevantes para pruebas.

El agente backend debe:

- Facilitar métricas e instrumentación.
- Implementar APIs verificables.
- Mantener consistencia bajo concurrencia.
- Corregir problemas detectados.

El especialista de base de datos debe:

- Evaluar consultas.
- Evaluar índices.
- Analizar conexiones.
- Revisar locks.
- Investigar saturación.

El especialista de sistemas distribuidos debe:

- Evaluar colas.
- Evaluar workers.
- Analizar throughput de eventos.
- Verificar reintentos e idempotencia.

El Quality Engineer debe:

- Revisar los escenarios.
- Ejecutar las pruebas.
- Validar thresholds.
- Revisar integridad.
- Reportar resultados reales.

El coordinador debe:

- Identificar especialistas necesarios.
- Coordinar pruebas.
- Evitar ejecuciones simultáneas peligrosas.
- Revisar resultados.
- Verificar criterios de aceptación.

Ningún agente puede inventar resultados de carga, asumir capacidad o modificar thresholds para ocultar fallos.

# 28. Referencias técnicas oficiales

Las implementaciones deben utilizar la documentación de Grafana k6 como referencia técnica.

## Documentación general

https://grafana.com/docs/k6/latest/

## Diseño de escenarios

https://grafana.com/docs/k6/latest/using-k6/scenarios/

## Executors

https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/

## Arrival Rate

https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/constant-arrival-rate/

https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/ramping-arrival-rate/

## Checks

https://grafana.com/docs/k6/latest/using-k6/checks/

## Thresholds

https://grafana.com/docs/k6/latest/using-k6/thresholds/

## Métricas

https://grafana.com/docs/k6/latest/using-k6/metrics/reference/

## Ciclo de vida

https://grafana.com/docs/k6/latest/using-k6/test-lifecycle/

## Datos de prueba

https://grafana.com/docs/k6/latest/examples/data-parameterization/

## Datos dinámicos y correlación

https://grafana.com/docs/k6/latest/examples/correlation-and-dynamic-data/

## Automatización

https://grafana.com/docs/k6/latest/testing-guides/automated-performance-testing/

## Pruebas web y navegador

https://grafana.com/docs/k6/latest/testing-guides/load-testing-websites/

## Ejecución de pruebas grandes

https://grafana.com/docs/k6/latest/testing-guides/running-large-tests/

---

# 29. Declaración final

Las pruebas de rendimiento forman parte de la ingeniería obligatoria del producto.

No son pruebas opcionales que deban realizarse únicamente cuando aparecen problemas de capacidad.

Desde la Fase 0 deben existir la estrategia, la estructura y los criterios necesarios para validar el rendimiento progresivamente.

Todo proyecto debe demostrar con evidencia qué carga soporta, bajo qué condiciones, con qué latencia, con qué tasa de errores, utilizando qué recursos y con qué capacidad de recuperación.

**Un sistema no puede declararse escalable, preparado para producción o capaz de soportar una cantidad determinada de usuarios sin pruebas reproducibles, métricas suficientes y evidencia verificable de rendimiento y capacidad.**
