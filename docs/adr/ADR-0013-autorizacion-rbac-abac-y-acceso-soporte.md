# ADR-0013: Autorización RBAC/ABAC y acceso temporal de soporte

- **Estado:** diseño aceptado expresamente por el propietario el 2026-11-18; implementación futura
- **Fecha:** 2026-11-18
- **Ámbito:** autenticación, asignación de roles, autorización por acción/recurso/scope y acceso administrativo de soporte
- **Requisitos:** [requerimiento §§14–16 y Fases 1, 4, 5, 7, 9 y 10](../requerimiento.md)
- **Fuentes:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [ADR-0001](ADR-0001-base-postgresql-compartida.md), [ADR-0005](ADR-0005-tenancy-multimodal-y-servicios-moviles.md), [ADR-0012](ADR-0012-jerarquia-empresarial-contexto-operativo.md), [reglas PDLM](../rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md), [regla legal](../rules/ENSAMBRIA_Privacy_Legal_Compliance.md), [reglas geográficas](../rules/ENSAMBRIA_Geographic_Access_Policies.md)

## Contexto

ENSAMBRIA debe autorizar acciones empresariales según el actor autenticado, el tenant y el producto validados, el recurso, los permisos asignados y los atributos de contexto aplicables. Un rol global no debe otorgar acceso implícito al contenido privado de tenants. RLS refuerza el aislamiento tenant-scoped, pero no decide permisos de negocio ni sustituye controles por empresa legal, grupo, marca, sucursal, área, caja o terminal.

El diseño conserva los owners y las entidades opcionales de ADR-0005/0012. No crea una entidad ficticia para conceder scope, no vuelve obligatoria la empresa legal, no confunde sucursal con establecimiento y no adopta `location_id` como identificador universal ambiguo.

## Decisión

### Modelo y ownership

1. La autorización combina **RBAC** —permisos por acción y módulo asignados mediante roles— con **ABAC** —condiciones basadas en atributos verificados de actor, tenant, producto, recurso, relación y sesión—. La política efectiva se aplica en el backend propietario del dato/acción, no en la UI, BFF, un motor central ni por RLS solamente.
2. CORE es dueño de identidad, autenticación, membresías tenant-scoped y del contexto de seguridad verificado. PLATFORM posee la relación comercial SaaS y su administración global. Cada owner conserva sus políticas y permisos de dominio; BUSINESS administra permisos de operaciones empresariales y PUBLIC autoriza sus operaciones/proyecciones públicas. Un backend no lee tablas ni políticas internas de otro.
3. Toda operación empresarial autenticada incluye `tenant_id`, `product_id` previamente validado y actor responsable. `organization_id`, `group_id`, `legal_entity_id`, `brand_id`, `branch_id`, `establishment_id`, `area_id`, `cash_register_id`, `terminal_id`, `warehouse_id` y `workstation_id` son contextuales y opcionales cuando el caso los requiere. Los identificadores canónicos de sucursal y establecimiento permanecen separados.
4. Ningún ID, título de cargo, relación jerárquica o pertenencia concede permiso por sí mismo. Cada owner verifica existencia, relación autorizada, acción, scope y estado. RLS protege las filas tenant-scoped además de esas comprobaciones.

### Roles, permisos y decisión efectiva

- Los roles base pueden corresponder a responsabilidades operativas y configurarse por tenant/módulo. Referencias iniciales tenant: propietario/administrador del tenant, gerente general, administrador de sucursal, supervisor, contador, cajero e inventarista. El superadministrador SaaS es un rol global de PLATFORM, limitado a recursos globales y separado de los roles/membresías tenant. El cargo es una asignación configurable, no una política implícita.
- Se permiten permisos por acción/módulo, permisos heredados mediante una relación de roles explícita y versionada, concesiones individuales y denegaciones individuales. La herencia de roles o scopes nunca cruza tenants y no se infiere de la jerarquía organizativa.
- La decisión es **denegar por defecto**. Una denegación explícita prevalece sobre una concesión; un permiso solo se concede si existe una asignación compatible y todos los atributos/condiciones aplicables se cumplen. Atributo ausente, inválido, obsoleto o de versión desconocida falla cerrado.
- Los scopes pueden ser plataforma para recursos exclusivamente globales de PLATFORM, tenant, grupo empresarial, empresa legal, marca del negocio, sucursal, área, caja, terminal o puesto de trabajo. Acceso a varios scopes requiere asignación explícita por cada scope autorizado; ser miembro de un grupo no concede por sí solo sus datos ni los de sus empresas/sucursales.

El catálogo inicial de permisos concretos incluye:

| Módulo | Permisos por acción |
|---|---|
| Ventas | `sales.view`, `sales.create`, `sales.cancel`, `sales.discount` |
| Caja | `cash.sessions.open`, `cash.sessions.close`, `cash.movements.create`, `cash.expenses.approve`, `cash.counts.approve` |
| Inventario | `inventory.view`, `inventory.adjust` |
| Compras | `purchases.create`, `purchases.approve` |
| Personal | `employees.view` |
| Finanzas | `finance.view` |
| Reportes | `reports.export` |
| Configuración | `settings.update` |

Cada acción sensible define sus condiciones y límites por configuración autorizada del tenant/sucursal. El descuento tiene un tope de 10%; el tenant/sucursal puede fijar un límite menor. También se requieren controles para anulaciones, retiros de caja con umbral monetario, ajustes de inventario con umbral de cantidad, cambios de precio por rol autorizado, reembolsos con aprobación de supervisor y compras con presupuesto. No se inventan los importes, cantidades, aprobadores ni defaults no especificados; los valores efectivos se registran con versión/vigencia antes de habilitar la operación.

### Aprobaciones duales

Las operaciones que la política del owner marque como sensibles pueden requerir dos aprobaciones. La política identifica monto/condición que activa la aprobación, solicitante, aprobador distinto, alcance, vencimiento y regla de integridad. La solicitud y cada decisión quedan auditadas. La operación se rechaza si la aprobación expiró, si cambia el recurso, importe, payload o contexto solicitado, si el aprobador no es distinto del solicitante o si falta el segundo aprobador requerido. La aprobación no sustituye autorización, transacción ni idempotencia.

### Administración SaaS y soporte

1. Los roles de superadministrador/administración global de SaaS operan sobre recursos globales de PLATFORM. No obtienen acceso automático a datos privados ni autoridad sobre las reglas internas de CORE o BUSINESS.
2. El soporte requiere un acceso just-in-time, temporal, mínimo y acotado a un tenant/recurso y propósito/caso concretos; registra solicitante, motivo, scope, inicio y expiración, aprobador y resultado. Cuando corresponda, requiere autorización del tenant antes de habilitarse.
3. El owner de cada dato vuelve a validar el permiso temporal en cada operación. Los datos se enmascaran por defecto y solo se muestran los campos necesarios para el caso. Todas las lecturas/cambios quedan auditados por el owner, incluyendo actor humano, identidad de soporte, tenant/scope, acción, finalidad, resultado, instante y correlación disponible. El acceso se revoca al cerrar el caso o expirar, lo que ocurra primero; no se extiende automáticamente.
4. El acceso excepcional de emergencia (*break-glass*) requiere doble aprobación, vigencia corta y alerta inmediata a seguridad, conforme a PDLM. Soporte no puede iniciar ni aprobar por sí solo solicitudes de baja, eliminación o derechos de titulares; toda acción humana en esos flujos requiere permiso específico, motivo y registro.
5. El acceso remoto desde otra región se evalúa como posible transferencia y no se habilita hasta mapear/justificar la ruta y obtener la validación legal aplicable. La política geográfica/red es un control adicional, no sustituye autorización del owner ni permite usar IP como prueba única de residencia, identidad o permiso.

## Alternativas consideradas

1. RBAC sin atributos contextuales: descartada porque no expresa de forma suficiente los scopes empresariales y condiciones de recursos/operaciones.
2. ABAC central con acceso a datos de todos los owners: descartada porque concentra ownership y crea un punto de acceso cruzado.
3. Autorizar solo en frontend/BFF o únicamente con `tenant_id`/RLS: descartada; cada backend owner debe aplicar permisos de acción y relaciones del dominio.
4. Acceso global permanente para soporte: descartada por incompatibilidad con aislamiento, mínimo privilegio y privacidad.

## Consecuencias, fases y aceptación

- CORE implementa identidad, membresías y contexto en Fase 1.2. BUSINESS, CORE, PLATFORM y PUBLIC completan evaluación por acción/recurso y pruebas en su propio dominio desde el primer caso de uso; RLS y autorización de datos/acciones se validan con PostgreSQL real.
- Los permisos operativos de caja/ventas se entregan en Fase 4.1; inventario y compras en 4.2; workforce en 4.3; soporte administrativo temporal en PLATFORM/Fase 5.2; la autorización de pagos se completa con CORE/Fase 7.1; Commerce en BUSINESS/Fase 9.3; scopes de grupo y consolidación en Fase 10.
- El diseño no habilita runtime ni demuestra que roles, políticas, auditoría o soporte estén implementados.
- La aceptación futura prueba acceso permitido/denegado, permiso/scope faltante, sesión expirada, atributo inválido, tenant/sucursal/empresa/marca ajenos, grants/denies, herencia explícita, revocación concurrente, aprobación dual incompleta/modificada/vencida, y ausencia de efectos parciales.
- Para soporte: no se accede sin caso/motivo/scope/expiración y aprobación aplicable; tenant no autorizado denegado; expiración/cierre revoca; auditoría captura lectura y mutación sin datos sensibles innecesarios.
- Las pruebas de aislamiento usan PostgreSQL real y roles de runtime; la aceptación de flujos integrados usa BFF/HTTP, contratos del owner y dependencias reales requeridas. Tests escritos o mocks no equivalen a ejecución/aceptación.

## Relación con otras decisiones

- **ADR-0001:** fija UUID canónicos, tenant/RLS, credenciales, schemas y autorización complementaria; esta ADR define el modelo RBAC/ABAC sin alterar la base compartida ni su ownership.
- **ADR-0005 y ADR-0012:** mantienen identidad/tenancy en CORE y jerarquía empresarial opcional; esta ADR usa esas referencias como atributos, no cambia su pertenencia ni infiere permisos por jerarquía.
- **ADR-0008:** la autorización de capacidades comunes y verticales permanece en el owner de BUSINESS; no se introducen roles sectoriales en el modelo común. Roles específicos de industrias futuras requieren su propio alcance y fase.
- **ADR-0009:** los permisos de pago no trasladan a BUSINESS o PLATFORM la propiedad del ledger de CORE ni mezclan emisores/cuentas.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Modelo de dominios](../arquitectura-dominios.md)
- [ADR-0001](ADR-0001-base-postgresql-compartida.md)
- [ADR-0005](ADR-0005-tenancy-multimodal-y-servicios-moviles.md)
- [ADR-0008](ADR-0008-modulos-compartidos-y-verticales-business.md)
- [ADR-0009](ADR-0009-proveedores-iniciales-y-reventa-factus.md)
- [ADR-0012](ADR-0012-jerarquia-empresarial-contexto-operativo.md)
- [Reglas PDLM](../rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md)
- [Regla legal](../rules/ENSAMBRIA_Privacy_Legal_Compliance.md)
- [Reglas geográficas](../rules/ENSAMBRIA_Geographic_Access_Policies.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
