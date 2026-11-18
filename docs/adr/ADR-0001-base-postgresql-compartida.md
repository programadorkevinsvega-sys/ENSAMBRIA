# ADR-0001: Base PostgreSQL compartida y aislamiento de datos

- **Estado:** diseño aceptado expresamente por el propietario el 2026-11-11; implementación futura
- **Fecha:** 2026-11-10
- **Ámbito:** CORE, BUSINESS, PLATFORM, PUBLIC y sus workers
- **Requisito:** Fase 0; §§14–16 y 21 de [docs/requerimiento.md](../requerimiento.md)
- **Fuentes normativas:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [reglas de pooling PostgreSQL](../rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md)

## Contexto

El producto requiere una única base PostgreSQL compartida por los cuatro backends. La guía de pooling y el contrato arquitectónico precisan que se comparten la base y la infraestructura de conexiones, no la autoridad sobre los datos ni los permisos. A la fecha de esta propuesta no hay base desplegada, schemas, migraciones, roles de aplicación ni pruebas de aislamiento ejecutadas.

Esta decisión debe definir además cómo se aíslan las filas multi-tenant, qué backend/módulo posee cada schema, qué identidades pueden usarlos y dónde Row-Level Security (RLS) es obligatorio. Una base común sin límites verificables aumenta el impacto potencial de un error de consulta, una credencial sobredimensionada o un contexto tenant residual en una conexión reutilizada.

## Decisión arquitectónica

### Topología y ownership

1. Los cuatro backends usarán una única base de datos PostgreSQL compartida; no bases separadas por backend o módulo ni bases lógicas distintas solo por compartir un clúster.
2. Cada backend/módulo es dueño de sus schemas, tablas, secuencias, vistas, funciones y migraciones conforme al mapa de ownership. El backend propietario mantiene sus contratos de acceso. Ningún backend consulta o modifica tablas de otro backend ni realiza joins SQL entre dominios.
3. Las proyecciones de PUBLIC son datos propios de PUBLIC, minimizados y publicados mediante contrato por el owner del dato fuente. Una ficha pública no autoriza a PUBLIC a leer la tabla privada de BUSINESS.
4. Compartir base, host, pool de infraestructura o herramienta de migraciones no transfiere ownership ni otorga permisos.
5. Cada proceso/backend mantiene un pool PostgreSQL acotado y una identidad de runtime propias. Los módulos co-residentes reciben el pool de su proceso; cada proceso independiente, incluidos workers desplegados independientemente, mantiene el suyo. No se crean pools por módulo, petición o tenant.

### Aislamiento multi-tenant y autorización

1. CORE es autoridad de identidad, tenant, membresías y contexto de seguridad. Cada backend dueño valida actor, tenant, sucursal y permisos en cada operación. Un UUID, `tenant_id`, header o parámetro del cliente nunca concede acceso.
2. Toda fila persistida que pertenezca a un tenant debe incluir el identificador canónico del tenant y mantenerlo en índices/constraints de unicidad y referencias. Las referencias entre filas tenant-scoped deben validar el mismo tenant, por ejemplo mediante claves foráneas compuestas que incluyan `tenant_id`; un ID válido de otro tenant no puede enlazarse accidentalmente.
3. Las consultas y mutaciones se ejecutan con contexto tenant derivado de una identidad verificada. La infraestructura lo establece transaccionalmente con alcance local (`SET LOCAL` o mecanismo equivalente revisado) antes de acceder a filas protegidas. Nunca se conserva un tenant en el estado de sesión de una conexión pooled; commit, rollback, cancelación y errores no pueden filtrar contexto a la siguiente operación.
4. La autorización del caso de uso y RLS son capas complementarias. RLS no reemplaza autenticación, membresía, autorización por acción/sucursal, validación de referencias ni seguridad de las consultas.
5. Trabajos administrativos o workers que deban abarcar varios tenants reciben identidad de workload, finalidad, scope, privilegios y concurrencia explícitos, mínimos y auditables. No reutilizan una credencial de superusuario ni se presentan como una solicitud de usuario con acceso global.

### Cuándo RLS es obligatorio

RLS es obligatorio en cada tabla con filas tenant-scoped, en todos los backends y módulos que las almacenen, incluidos los datos derivados no públicos, tablas de relación, índices persistidos, proyecciones privadas, estados de trabajo y tablas de integración que contengan información por tenant. Debe proteger lectura y escritura; las políticas de `SELECT`/`DELETE` usan predicados de fila y las de `INSERT`/`UPDATE` validan el tenant tanto con `USING` como con `WITH CHECK`, según corresponda. La política predeterminada es denegar cuando falta contexto o permiso.

Las tablas de catálogos globales que no representan datos ni relaciones de tenant pueden quedar fuera de RLS solo con una justificación concreta, owner y evidencia previa en el catálogo de datos. Una proyección que sea públicamente visible sigue requiriendo control explícito de publicación y de sus filas; la excepción “público” no habilita consultar el origen privado ni exponer borradores, retirados o datos personales.

Cada tabla con filas tenant-scoped debe habilitar y aplicar RLS. El diseño recomienda `FORCE ROW LEVEL SECURITY` como defensa adicional para que también se aplique al owner de la tabla; confirmar su uso por tabla al implementar no reabre ni debilita la obligación de RLS para todo dato tenant-scoped. Las cuentas de runtime no son superuser, no tienen `BYPASSRLS`, no son propietarias de tablas y no pueden asumir roles de migración. El owner de schema/migración es una identidad separada, no usada por la aplicación. Cualquier excepción a RLS requiere necesidad concreta, mitigación, owner y aprobación previa.

RLS se considera defensa en profundidad: PostgreSQL no autentica por sí mismo la membresía de producto a partir de un GUC que la aplicación establece. Por ello, el contexto solo puede establecerlo la capa de infraestructura después de autorización; las consultas son parametrizadas; no se ejecuta SQL arbitrario controlado por usuarios; y los casos de uso vuelven a validar el alcance autorizado. Un compromiso que permita ejecutar SQL arbitrario con la identidad de runtime puede intentar cambiar variables de contexto y debe tratarse como riesgo residual, no como una garantía que RLS elimine por sí sola.

### Ownership de schemas y separación de credenciales

- Un rol propietario `NOLOGIN` por schema/módulo mantiene ownership de objetos y migraciones. Las cuentas de migración son distintas de las de runtime, limitadas al schema propietario, usadas de forma coordinada y nunca configuradas en una aplicación.
- Cada backend y workload usa identidad PostgreSQL distinta por entorno, sin secretos compartidos entre entornos, backends o procesos. Los grants de runtime son mínimos para el schema/dominio que el proceso necesita; no se concede `CREATE`, `ALTER`, `DROP`, ownership, `BYPASSRLS` ni acceso de superusuario a roles de aplicación.
- Workers usan identidades propias con acceso acotado a las tablas/operaciones que requieren. Los relays pueden recibir acceso de lectura/actualización limitado al Outbox y no a las tablas de dominio de otros backends.
- Las credenciales y grants por backend/dominio se inventarían antes de implementar. La decisión no define proveedor, región, versión de PostgreSQL, número de réplicas, valores de pool ni instalación de PgBouncer.

### Pool y fronteras de acceso

El pool compartido por los módulos de un proceso/backend es una obligación arquitectónica, no una alternativa de ratificación. Las credenciales y grants se mantienen separados por backend y workload; el pool de un backend no accede a schemas de otro backend. Dentro del backend, la separación de módulos se aplica mediante APIs públicas internas, repositorios de propiedad exclusiva, dependencias verificables y pruebas que prohíben SQL, joins e imports de implementación cruzados. El pool compartido no es una frontera SQL entre módulos co-residentes y no debe describirse como tal.

## Pruebas negativas de aceptación requeridas

La decisión no se considera implementada hasta que pruebas automatizadas contra PostgreSQL real, con los mismos roles no privilegiados del runtime, demuestren:

1. Tenant A no puede leer, insertar, modificar ni eliminar filas de Tenant B; el contexto ausente, inválido o revocado deniega por defecto.
2. Un usuario con membresía en varios tenants solo opera sobre el tenant/sucursal explícitamente autorizado para esa petición.
3. IDs válidos de otra organización/sucursal/tenant no permiten acceso, asociación de claves foráneas, cambio de estado ni efectos parciales.
4. `UPDATE` no puede cambiar una fila protegida al tenant de otro scope; `INSERT` con tenant falsificado falla.
5. RLS también se aplica al owner mediante `FORCE`; las credenciales de runtime no son superuser, no tienen `BYPASSRLS` y no pueden asumir el rol dueño/migración.
6. El rol de CORE no puede leer/escribir schemas de BUSINESS, PLATFORM o PUBLIC, y viceversa. Los permisos del worker/relay no exceden su Outbox/Inbox y tarea autorizada.
7. El acceso a schema de otro dominio falla con `permission denied`; no existen consultas/join cruzados en repositorios o contratos.
8. Reusar una conexión tras commit, rollback, cancelación, timeout o error no conserva contexto; una petición posterior de Tenant B no observa ni modifica datos de Tenant A.
9. La proyección pública expone solo filas publicadas y campos permitidos, nunca datos fuente privados, drafts o contenido retirado.
10. Tests de API/gRPC verifican que sesión ausente/expirada, rol insuficiente, membresía revocada, scope ajeno y carrera concurrente no produzcan efectos parciales. Los datos de prueba se crean y eliminan por flujos reales en infraestructura aislada, no por fixtures prefabricados.

## Alternativas consideradas

1. Una base distinta por backend/módulo: descartada para el diseño solicitado; contradice el requisito de una base compartida.
2. Compartir solo un clúster con bases separadas: descartada; no satisface la definición de base única.
3. Base única, schemas y roles por owner, RLS obligatoria en cada tabla tenant-scoped y autorización en el owner: arquitectura requerida por el producto.
4. Un rol/pool SQL exclusivo por módulo además de los cuatro backends: descartado; contradice el pool compartido por proceso/backend establecido en el requerimiento.
5. Filtrar por `tenant_id` únicamente en la aplicación: insuficiente; no ofrece defensa ante una consulta omitida y contradice la exigencia de RLS.

## Consecuencias y riesgos

- La base física común crea un dominio compartido de capacidad y disponibilidad; conexiones, réplicas máximas, workers y margen deben dimensionarse globalmente conforme a la guía de pooling y con valores medidos.
- Un error en roles, migraciones o políticas puede ampliar impacto. Se requieren tests negativos con privilegios reales, migraciones versionadas y revisión de cada nueva tabla/columna tenant-scoped.
- RLS puede bloquear datos legítimos si falta contexto; debe fallar cerrado y diagnosticar el error sin filtrar PII.
- RLS no protege ante superuser, `BYPASSRLS`, un owner sin `FORCE`, credenciales de migración expuestas o ejecución SQL arbitraria; esos roles se excluyen del runtime y se revisa el riesgo residual.
- El pool compartido de módulos co-residentes no constituye aislamiento SQL entre ellos; sus límites se imponen mediante ownership modular, contratos, dependencias y pruebas.
- Backups, restauración, HA, RPO/RTO y plan de contingencia quedan para diseño operacional; no se afirman ni implementan aquí.

## Ratificación y aspectos pendientes

El requerimiento ya determina la base física compartida, ownership separado, pool por proceso/backend compartido por módulos co-residentes, credenciales segregadas, RLS en tablas tenant-scoped y contexto de seguridad transaccional. Esta ADR no propone opciones alternativas ni solicita reabrir esas decisiones.

El propietario aceptó expresamente este diseño documental el 2026-11-11. La aceptación confirma coherencia con el requerimiento; no habilita otra topología, runtime ni una excepción a RLS. No hay excepciones solicitadas. Antes de la implementación, cada backend deberá completar su inventario de schemas, roles, grants, parámetros del pool y casos reales de RLS. Cualquier excepción futura —por ejemplo, una tabla sin alcance tenant o una política distinta de `FORCE ROW LEVEL SECURITY`— requiere justificación concreta, owner y aprobación previa conforme a las fuentes normativas. El dimensionamiento requiere datos del entorno y no es una decisión arquitectónica alternativa.

No hay PostgreSQL runtime ni pruebas reales ejecutadas.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Guía de pooling PostgreSQL](../rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
