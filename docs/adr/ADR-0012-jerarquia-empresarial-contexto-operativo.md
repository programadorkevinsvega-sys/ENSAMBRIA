# ADR-0012: Jerarquía empresarial y contexto obligatorio de operaciones

**Estado:** Aceptado por el propietario el 2026-11-18. Decisión de diseño documental; no declara runtime, esquema, migración, API, evento ni prueba implementados. No constituye aprobación legal o fiscal.

## Contexto

El tenant es el ámbito técnico estable de aislamiento, pero no equivale a una empresa legal ni a una sucursal. La actividad individual y los negocios sin entidad legal deben poder operar sin crear organizaciones, marcas, establecimientos u otros registros ficticios. El modelo existente distingue sucursal operativa y establecimiento físico, y ADR-0005 ya acepta que son opcionales y diferentes.

El modelo también cubre la cuenta propia del proveedor SaaS, propietarios externos, cadenas, negocios independientes y profesionales independientes sin convertirlos en tipos distintos de tenant. Se precisan las cardinalidades empresariales expresamente decididas, la selección de emisor fiscal, los recursos de operación y el contexto de autorización. Se conserva la propiedad única de los cuatro backends, el aislamiento entre tenants y la separación fiscal establecida en ADR-0007 y ADR-0009.

## Decisión

### Jerarquía conceptual y opcionalidad

Se adopta la siguiente secuencia de referencia:

`tenant → grupo empresarial (opcional) → empresa legal (opcional) → marca del negocio (opcional) → sucursal (opcional) → establecimiento (opcional) → área (opcional) → caja, terminal y bodega (cuando apliquen)`

La secuencia no impone una cadena obligatoria de pertenencia. Solo el tenant es requerido para toda operación empresarial; `product_id` validado y actor responsable son obligatorios en su contexto. Los demás niveles son opcionales y se representan únicamente mediante relaciones explícitas existentes y autorizadas. Un tenant puede contener varios grupos, empresas legales, marcas y sucursales; un grupo puede agrupar varias empresas legales; una empresa legal puede operar varias marcas y sucursales; y una marca puede operar varias sucursales. Una sucursal no requiere marca. Una persona profesional puede operar sin grupo, empresa legal, marca ni sucursal.

Los términos se mantienen diferenciados:

- **Tenant:** ámbito técnico estable de aislamiento.
- **Organización:** estructura operativa opcional, no sinónimo de empresa legal ni nivel obligatorio de la jerarquía.
- **Empresa legal:** entidad fiscal y contable, opcional.
- **Marca del negocio:** nombre comercial/formato, distinto de la marca/configuración de producto SaaS de PLATFORM.
- **Sucursal:** ámbito organizativo/operativo.
- **Establecimiento:** lugar físico; la relación estructural de CORE no sustituye los datos y operación cuyo owner es BUSINESS.
- **Área:** división interna genérica y opcional; sus recursos y reglas sectoriales pertenecen al vertical BUSINESS correspondiente.
- **Bodega:** ubicación de almacenamiento opcional, con productos/insumos y ubicaciones internas según el caso; BUSINESS posee existencias, configuración y responsables.
- **Caja y terminal:** recursos operativos opcionales de BUSINESS; CORE mantiene la identidad del terminal cuando actúa como workload. Un terminal puede asociarse a una caja/sucursal y admitir operación offline si se habilita mediante configuración.

Las cardinalidades descritas son relaciones posibles, no herencia automática de permisos, configuración o filiación fiscal. Toda relación adicional debe ser explícita y tenant-scoped. Un ID o parentesco no concede acceso.

### Configuración operativa y herencia explícita

La configuración operativa pertenece a BUSINESS y no se hereda de forma implícita por pertenecer a un grupo, empresa legal, marca o sucursal. Cada clave declara si admite herencia, sus scopes y excepciones permitidos, versión y vigencia. No se heredan permisos, scopes de autorización ni afiliaciones fiscales desde la configuración. La resolución específica del emisor fiscal permanece como se define en este ADR.

Para una clave heredable, el valor efectivo debe exponer su origen/scope, versión y vigencia. Los cambios centrales no sobrescriben excepciones locales sin autorización explícita; la aplicación a varias entidades debe poder previsualizarse, limitarse a las que aún heredan, programarse y revertirse con una nueva versión auditada. No se adopta un motor universal ni una precedencia común: BUSINESS define y prueba la precedencia por clave antes de habilitar el caso.

Este complemento registra requisitos del propietario para configuración multi-sucursal; no decide la precedencia entre grupo, empresa legal, marca y sucursal ni declara runtime. Los casos que dependan de esa precedencia permanecen bloqueados hasta su resolución.

### Emisor fiscal y continuidad histórica

Cuando una operación necesita un emisor fiscal, su resolución es:

1. excepción de sucursal, solo si la configuración la permite;
2. empresa legal predeterminada de la marca;
3. empresa legal predeterminada del tenant;
4. titular persona natural del tenant, si no hay empresa legal configurada.

La selección, autorización y auditoría de excepciones por sucursal requiere una definición de BUSINESS y aprobación competente antes de habilitarlas. Cada documento fiscal conserva el emisor usado al emitirse. Agregar una empresa legal después no cambia el tenant, sus IDs ni el emisor de documentos históricos. Sin empresa legal o emisor definido, el tenant conserva su capacidad de operar y reservar; solo se bloquea la emisión del documento que requiera emisor. Los datos mínimos fiscales de una persona natural se recogen únicamente cuando el mercado/alcance lo exige.

### Contexto y aislamiento de operaciones

Toda operación empresarial conoce `tenant_id`, `product_id` validado y actor responsable. Incluye referencias a grupo empresarial, empresa legal, marca, sucursal y, cuando apliquen, establecimiento, área, caja, terminal y bodega. Las referencias opcionales no se inventan ni conceden autorización por sí mismas.

El backend dueño valida las relaciones autorizadas entre entidades antes de operar. No se mezclan empresas legales, marcas, sucursales o recursos sin una relación explícita. RLS protege el aislamiento por tenant y se complementa con autorización del dominio, restricciones de integridad y pruebas negativas para los scopes internos. Un ID válido de otro tenant no concede acceso.

### Ownership

| Backend | Responsabilidad |
|---|---|
| CORE | Identidad, tenant, producto validado, contexto de seguridad y relaciones estructurales necesarias para autorizar organización/sucursal. Es owner de la identidad/autenticación del dispositivo o terminal cuando actúa como workload. No posee la empresa legal, sus datos fiscales ni la operación de los recursos de BUSINESS. |
| BUSINESS | Empresa legal y datos fiscales operativos; marca del negocio; relación de emisor predeterminado y reglas de herencia/excepción fiscal; grupo empresarial como entidad de negocio y consolidación; operación de sucursales/establecimientos; áreas, bodegas, cajas y terminales operativos. Valida sus referencias estructurales de CORE por contrato. |
| PLATFORM | Relación comercial SaaS, planes, suscripciones y políticas comerciales de ENSAMBRIA. No posee ni duplica grupo, empresa legal, marca del negocio, sucursal o recursos operativos del tenant. Su marca/configuración de producto continúa siendo distinta. |
| PUBLIC | Consume y sirve únicamente proyecciones explícitamente autorizadas. No posee las entidades privadas fuente ni sus reglas. |

Cada dato tiene un único owner. Ningún backend consulta tablas ajenas; las integraciones se realizan por contrato versionado y con UUID v7 canónico conforme a la política común de IDs.

### Fases

La Fase 0 contiene solo diseño, ownership, ADR, trazabilidad y criterios de aceptación. No habilita runtime.

| Entregable de producto | Fase prevista | Owner y condición |
|---|---|---|
| Tenant, producto validado, estructura y contexto base | Fase 1, especialmente 1.2 | CORE; relaciones estructurales y autorización, no entidades fiscales/operativas ficticias. |
| Áreas genéricas de sucursal/establecimiento | Fase 2.1, solo al requerirlas un caso operativo | BUSINESS; tipos específicos permanecen en el vertical correspondiente. |
| Marca del negocio y perfil inicial que la requiera | Fase 5.1; proyección pública autorizada en 5.3 | BUSINESS posee la fuente; PUBLIC solo recibe proyecciones permitidas. La asociación fiscal se completa con la capacidad de emisor de Fase 7. |
| Cajas y terminales operativos, incluida la operación offline configurable | Fase 4.1 | BUSINESS para operación; CORE para identidad de terminal/workload cuando aplique. Las acciones offline, autorización, idempotencia, recuperación y conciliación se definen antes de implementar; esto no habilita cobros/liquidaciones ni emisión fiscal offline. |
| Bodegas y proveedores | Fase 4.2 | BUSINESS. |
| Configuración operativa centralizada y excepciones autorizadas | En los dominios que introducen la clave; configuración transversal multi-sucursal en Fase 10.2 | BUSINESS; sin herencia de permisos y con precedencia por clave definida antes de habilitarla. |
| Empresa legal, datos fiscales y emisor de persona natural/legal; resolución del emisor para facturación electrónica y eventual numeración reservada por terminal | Fase 7.2 | BUSINESS para emisor tenant; PLATFORM conserva por separado sus documentos SaaS. La numeración por terminal depende de reglas aplicables y capacidades oficiales verificadas. |
| Grupo empresarial y consolidación | Fase 10.1–10.2 | BUSINESS; pertenencia a grupo no amplía permisos sin concesión explícita. |

Las capacidades se completan en la fase indicada y solo para los casos que las requieren. Los cambios de fase no crean dependencias que impidan la operación básica de un tenant sin empresa legal.

## Alternativas consideradas

1. **Exigir empresa legal y marca para cada tenant:** rechazada porque excluiría la actividad profesional individual y requeriría registros ficticios.
2. **Usar organización, empresa legal y tenant como sinónimos:** rechazada porque mezcla aislamiento técnico, estructura operativa y personalidad fiscal.
3. **Hacer obligatoria la pertenencia de sucursal a una marca:** rechazada; sucursales sin marca son válidas y la relación, cuando exista, debe ser explícita.
4. **Asignar toda la jerarquía a CORE o PLATFORM:** rechazada porque CORE posee tenancy/seguridad y PLATFORM la relación SaaS, mientras que BUSINESS posee las reglas y datos operativos tenant.
5. **Actualizar retroactivamente documentos al cambiar el emisor:** rechazada porque destruye la reproducibilidad del documento originalmente emitido.
6. **Bloquear toda operación por falta de emisor fiscal:** rechazada; la ausencia de emisor condiciona únicamente el documento fiscal que lo requiera.
7. **Crear un tipo de tenant privilegiado para la cuenta propia de ENSAMBRIA:** rechazado; se mantiene el mismo aislamiento tenant y los privilegios de proveedor SaaS se autorizan separadamente en PLATFORM.

## Pendientes y alcance condicionado

| Decisión pendiente | Responsable | Alcance que permanece bloqueado |
|---|---|---|
| Quién puede autorizar una excepción fiscal por sucursal, cuándo se permite y cómo se audita. | Owner BUSINESS con aprobación de Finanzas y revisión competente de Legal cuando aplique. | Habilitar excepciones fiscales por sucursal; no bloquea la operación sin excepciones. |
| Datos mínimos de persona natural requeridos para emitir documentos fiscales por mercado. | Legal/Finanzas, con evidencia de fuentes oficiales aplicables. | Emisión dependiente de esos datos en el mercado correspondiente; no bloquea operación ni reservas. |
| Si un tenant permite más de un titular persona natural, cómo seleccionar el emisor titular cuando no existe empresa legal. | CORE y BUSINESS antes del flujo de selección que lo requiera. | Solo la emisión afectada si el producto admite multiplicidad de titulares; no se presume la multiplicidad. |
| Relaciones adicionales no definidas entre organización, sucursal, establecimiento, área y recursos; las cardinalidades tenant→grupo/empresa/marca/sucursal, grupo→empresa, empresa→marca/sucursal y marca→sucursal sí quedan documentadas como posibles relaciones múltiples. | BUSINESS para relaciones de negocio; CORE para las estructurales/de autorización. | Solo el vínculo o comportamiento que dependa de la relación no definida; no se infieren permisos por jerarquía. |
| Precedencia por clave, fuentes elegibles, conflictos y reversión de valores heredados entre grupo, empresa legal, marca y sucursal. | Owner BUSINESS de cada configuración, con Producto/Finanzas u otros responsables funcionales según la clave. | Solo los cambios centralizados y su cálculo de valor efectivo; no bloquea la configuración local ni la operación independiente. |
| Acciones permitidas en modo offline, autoridad local durante la desconexión, recuperación/reconciliación e interacción con pagos. | BUSINESS para la operación de terminal/venta y CORE para efectos financieros; Producto/Finanzas resuelven la política operativa antes del alcance correspondiente. | Solo operación offline del terminal y cualquier efecto de pago que dependa de esas reglas; modo conectado y operaciones independientes no se bloquean. |
| Si la numeración fiscal puede reservarse por terminal, con qué autoridad/proveedor, alcance, unicidad y auditoría. | BUSINESS con Finanzas y revisión competente de Legal cuando aplique; verificar capacidad oficial del proveedor/autoridad. | Solo la reserva/uso de numeración por terminal; no bloquea facturación que pueda operar por un mecanismo validado diferente. |

Ninguna de estas decisiones se aprueba mediante este ADR. La revisión profesional jurídica y la investigación jurisdiccional mantienen su estado separado; este documento no afirma su realización ni bloquea alcance independiente.

## Consecuencias y criterios de aceptación futuros

La aplicación debe propagar contexto validado en cada backend owner y rechazar relaciones no permitidas aunque las referencias existan. Las pruebas de implementación usarán dependencias reales y cubrirán, como mínimo:

1. Un tenant sin empresa legal opera y crea reservas con normalidad.
2. Un tenant sin emisor definido no puede emitir factura electrónica, pero sí operar.
3. Agregar una empresa legal después no altera documentos históricos ni IDs.
4. Una excepción fiscal de sucursal no permitida se rechaza en backend.
5. Una operación que mezcla empresas legales, marcas o sucursales sin relación autorizada se rechaza.
6. Un ID válido de otro tenant no concede acceso.
7. Una sucursal hereda correctamente la empresa legal predeterminada de la marca y una excepción permitida prevalece sobre la herencia.
8. El documento fiscal conserva su emisor original tras cambios posteriores.

También se documentan para sus fases propietarias: cada cardinalidad aprobada admite relaciones múltiples sin herencia implícita de permisos; una cuenta propia de ENSAMBRIA permanece aislada como cualquier otro tenant y sus privilegios SaaS provienen de roles PLATFORM; un terminal offline solo ejecuta las acciones que una política aprobada permita y recupera/reconcilia sin duplicar operaciones; una reserva de numeración fiscal por terminal se rechaza o mantiene deshabilitada si la capacidad oficial y las reglas aplicables no se verifican. Estos son criterios documentados, no pruebas implementadas o ejecutadas.

## Relación con ADR existentes

- **ADR-0002:** conserva el hecho durable de alta de tenant CORE→PLATFORM y su referencia a `product_id`. No crea entidades BUSINESS, asigna emisor fiscal ni convierte el alta técnica en una relación comercial o fiscal.
- **ADR-0005:** este ADR lo complementa; conserva tenant estable, opcionalidad y distinción entre sucursal, establecimiento y modalidades. No lo sustituye ni reabre sus decisiones aceptadas.
- **ADR-0009:** conserva la separación de proveedores y contextos fiscales SaaS/tenant, con Factus como proveedor inicial previsto no exclusivo. Este ADR añade el diseño de emisor para BUSINESS, sin asumir capacidades del proveedor, requisitos legales aprobados ni comercialización automática.
- **ADR-0014:** conserva el ownership de ventas/caja/terminal en BUSINESS y de pagos/ledger en CORE; concreta que el modo offline requiere reglas y recuperación propias, y que la numeración fiscal por terminal no se presume.

La ampliación documental de 2026-11-18 registra las cardinalidades y los requisitos de cuenta propia/terminal comunicados expresamente por el propietario; no cambia el ownership aprobado ni afirma que el modo offline o la numeración fiscal por terminal estén implementados. Los límites de ambos comportamientos quedan pendientes para sus fases propietarias.

## Referencias

- [Requerimiento §2.2](../requerimiento.md#22-modelo-empresarial-multimodal-y-servicios-móviles)
- [Requerimiento §15](../requerimiento.md#15-modelo-de-tenancy-y-seguridad)
- [Modelo de dominios §2.6](../arquitectura-dominios.md#26-modelo-multimodal-de-tenancy-y-servicios-móviles)
- [ROADMAP.md](../../ROADMAP.md)
- [ADR-0002](ADR-0002-outbox-alta-tenant-core-platform.md)
- [ADR-0005](ADR-0005-tenancy-multimodal-y-servicios-moviles.md)
- [ADR-0009](ADR-0009-proveedores-iniciales-y-reventa-factus.md)
- [ADR-0014](ADR-0014-coordinacion-ventas-pagos-inventario.md)
