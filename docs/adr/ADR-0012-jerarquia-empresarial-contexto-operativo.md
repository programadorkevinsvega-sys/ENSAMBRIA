# ADR-0012: Jerarquía empresarial y contexto obligatorio de operaciones

**Estado:** Aceptado por el propietario el 2026-11-18. Decisión de diseño documental; no declara runtime, esquema, migración, API, evento ni prueba implementados. No constituye aprobación legal o fiscal.

## Contexto

El tenant es el ámbito técnico estable de aislamiento, pero no equivale a una empresa legal ni a una sucursal. La actividad individual y los negocios sin entidad legal deben poder operar sin crear organizaciones, marcas, establecimientos u otros registros ficticios. El modelo existente distingue sucursal operativa y establecimiento físico, y ADR-0005 ya acepta que son opcionales y diferentes.

El requerimiento necesita precisar además la relación opcional entre entidades empresariales, la selección de emisor fiscal y el contexto con que se autorizan operaciones. Debe conservarse la propiedad única de los cuatro backends, el aislamiento entre tenants y la separación fiscal establecida en ADR-0007 y ADR-0009.

## Decisión

### Jerarquía conceptual y opcionalidad

Se adopta la siguiente secuencia de referencia:

`tenant → grupo empresarial (opcional) → empresa legal (opcional) → marca del negocio (opcional) → sucursal (opcional) → establecimiento (opcional) → área (opcional) → caja, terminal y bodega (cuando apliquen)`

La secuencia no impone una cadena obligatoria de pertenencia. Solo el tenant es requerido para toda operación empresarial; `product_id` validado y actor responsable son obligatorios en su contexto. Los demás niveles son opcionales y se representan únicamente mediante relaciones explícitas existentes y autorizadas. Una sucursal no requiere marca. Una persona profesional puede operar sin grupo, empresa legal, marca ni sucursal.

Los términos se mantienen diferenciados:

- **Tenant:** ámbito técnico estable de aislamiento.
- **Organización:** estructura operativa opcional, no sinónimo de empresa legal ni nivel obligatorio de la jerarquía.
- **Empresa legal:** entidad fiscal y contable, opcional.
- **Marca del negocio:** nombre comercial/formato, distinto de la marca/configuración de producto SaaS de PLATFORM.
- **Sucursal:** ámbito organizativo/operativo.
- **Establecimiento:** lugar físico; la relación estructural de CORE no sustituye los datos y operación cuyo owner es BUSINESS.

El tenant puede contener más de una estructura empresarial dentro de sus límites autorizados. No se infieren cardinalidades, permisos, herencia de configuración ni filiación fiscal a partir de un ID o parentesco no documentado.

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
| Cajas y terminales operativos | Fase 4.1 | BUSINESS para operación; CORE para identidad de terminal/workload cuando aplique. |
| Bodegas y proveedores | Fase 4.2 | BUSINESS. |
| Empresa legal, datos fiscales y emisor de persona natural/legal; resolución del emisor para facturación electrónica | Fase 7.2 | BUSINESS para emisor tenant; PLATFORM conserva por separado sus documentos SaaS. Verificar requisitos y proveedor oficial en el alcance aplicable. |
| Grupo empresarial y consolidación | Fase 10.1–10.2 | BUSINESS; pertenencia a grupo no amplía permisos sin concesión explícita. |

Las capacidades se completan en la fase indicada y solo para los casos que las requieren. Los cambios de fase no crean dependencias que impidan la operación básica de un tenant sin empresa legal.

## Alternativas consideradas

1. **Exigir empresa legal y marca para cada tenant:** rechazada porque excluiría la actividad profesional individual y requeriría registros ficticios.
2. **Usar organización, empresa legal y tenant como sinónimos:** rechazada porque mezcla aislamiento técnico, estructura operativa y personalidad fiscal.
3. **Hacer obligatoria la pertenencia de sucursal a una marca:** rechazada; sucursales sin marca son válidas y la relación, cuando exista, debe ser explícita.
4. **Asignar toda la jerarquía a CORE o PLATFORM:** rechazada porque CORE posee tenancy/seguridad y PLATFORM la relación SaaS, mientras que BUSINESS posee las reglas y datos operativos tenant.
5. **Actualizar retroactivamente documentos al cambiar el emisor:** rechazada porque destruye la reproducibilidad del documento originalmente emitido.
6. **Bloquear toda operación por falta de emisor fiscal:** rechazada; la ausencia de emisor condiciona únicamente el documento fiscal que lo requiera.

## Pendientes y alcance condicionado

| Decisión pendiente | Responsable | Alcance que permanece bloqueado |
|---|---|---|
| Quién puede autorizar una excepción fiscal por sucursal, cuándo se permite y cómo se audita. | Owner BUSINESS con aprobación de Finanzas y revisión competente de Legal cuando aplique. | Habilitar excepciones fiscales por sucursal; no bloquea la operación sin excepciones. |
| Datos mínimos de persona natural requeridos para emitir documentos fiscales por mercado. | Legal/Finanzas, con evidencia de fuentes oficiales aplicables. | Emisión dependiente de esos datos en el mercado correspondiente; no bloquea operación ni reservas. |
| Si un tenant permite más de un titular persona natural, cómo seleccionar el emisor titular cuando no existe empresa legal. | CORE y BUSINESS antes del flujo de selección que lo requiera. | Solo la emisión afectada si el producto admite multiplicidad de titulares; no se presume la multiplicidad. |
| Cardinalidades y relaciones entre entidades más allá de las expresamente descritas. | BUSINESS para relaciones de negocio; CORE para las estructurales/de autorización. | Solo el vínculo o comportamiento que dependa de la relación no definida. |

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

Estos son criterios documentados para sus fases propietarias; no se afirma que estén implementados, escritos como pruebas o ejecutados.

## Relación con ADR existentes

- **ADR-0002:** conserva el hecho durable de alta de tenant CORE→PLATFORM y su referencia a `product_id`. No crea entidades BUSINESS, asigna emisor fiscal ni convierte el alta técnica en una relación comercial o fiscal.
- **ADR-0005:** este ADR lo complementa; conserva tenant estable, opcionalidad y distinción entre sucursal, establecimiento y modalidades. No lo sustituye ni reabre sus decisiones aceptadas.
- **ADR-0009:** conserva la separación de proveedores y contextos fiscales SaaS/tenant, con Factus como proveedor inicial previsto no exclusivo. Este ADR añade el diseño de emisor para BUSINESS, sin asumir capacidades del proveedor, requisitos legales aprobados ni comercialización automática.

## Referencias

- [Requerimiento §2.2](../requerimiento.md#22-modelo-empresarial-multimodal-y-servicios-móviles)
- [Requerimiento §15](../requerimiento.md#15-modelo-de-tenancy-y-seguridad)
- [Modelo de dominios §2.6](../arquitectura-dominios.md#26-modelo-multimodal-de-tenancy-y-servicios-móviles)
- [ROADMAP.md](../../ROADMAP.md)
- [ADR-0002](ADR-0002-outbox-alta-tenant-core-platform.md)
- [ADR-0005](ADR-0005-tenancy-multimodal-y-servicios-moviles.md)
- [ADR-0009](ADR-0009-proveedores-iniciales-y-reventa-factus.md)
