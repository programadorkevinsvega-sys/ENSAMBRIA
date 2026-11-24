# PROMPT MAESTRO — MODELO MULTIMODAL DE TENANCY, PROFESIONALES INDEPENDIENTES, ESTABLECIMIENTOS Y SERVICIOS A DOMICILIO EN ENSAMBRIA

## 1. OBJETIVO

Debes incorporar y definir formalmente en ENSAMBRIA un modelo empresarial multimodal que permita operar tanto a empresas con establecimientos físicos como a personas naturales que prestan servicios profesionales de manera independiente, principalmente a domicilio.

Esta capacidad debe diseñarse desde la Fase 0 y respetarse en todas las fases posteriores del proyecto.

ENSAMBRIA no debe asumir que todo tenant representa una empresa constituida como persona jurídica ni que todo negocio necesita disponer de un establecimiento físico.

La plataforma debe admitir diferentes formas de operación empresarial bajo un mismo modelo de tenancy, identidad, seguridad y capacidades compartidas.

El objetivo es permitir que:

1. Una persona natural pueda registrarse y crear su propio tenant sin constituir una persona jurídica ni registrar una sucursal física.
2. Un negocio con establecimiento físico pueda operar con una o varias sucursales.
3. Un profesional independiente pueda prestar servicios desplazándose hasta el domicilio de sus clientes.
4. Un negocio pueda prestar servicios tanto en sus establecimientos como a domicilio.
5. Un profesional independiente pueda incorporar trabajadores o colaboradores.
6. Un negocio pueda crecer progresivamente hasta convertirse en una empresa con múltiples establecimientos y equipos de trabajo.
7. Todos puedan utilizar las capacidades empresariales de ENSAMBRIA según su modalidad, actividad, permisos y plan contratado.
8. La evolución del negocio no obligue a crear otro tenant, otra cuenta ni a migrar artificialmente la información existente.

**La modalidad de operación no debe determinar una arquitectura de tenancy diferente.**

Debe existir un modelo empresarial común, extensible y capaz de representar todas estas situaciones.

## 2. FUENTES NORMATIVAS Y PRECEDENCIA

Antes de realizar cualquier modificación:

1. Inspecciona el estado completo del repositorio.
2. Lee íntegramente `docs/requerimiento.md`.
3. Lee `ARCHITECTURE_RULES.md`.
4. Lee `docs/arquitectura-dominios.md`.
5. Lee los ADR existentes.
6. Lee el registro de descubrimiento y arquitectura de Fase 0.
7. Lee las matrices de ownership, dominios, capacidades, fases y aceptación.
8. Inspecciona los contratos y definiciones existentes de identidad, tenancy, organizaciones, sucursales, reservas, CRM y workforce, si existen.
9. Identifica contradicciones y decisiones ya ratificadas.

`docs/requerimiento.md` continúa siendo la fuente principal de verdad.

La nueva definición debe integrarse en las fuentes normativas existentes, sin crear documentación duplicada que compita con ellas.

No cambies decisiones arquitectónicas ratificadas sin justificación y registro correspondiente.

No supongas que un modelo ya está implementado por el simple hecho de que aparezca mencionado en la documentación.

Distingue rigurosamente entre:

- Requisito definido.
- Decisión arquitectónica ratificada.
- Contrato especificado.
- Implementación realizada.
- Implementación probada.
- Funcionalidad habilitada comercialmente.

## 3. MODELO FUNDAMENTAL: PERSONA, TENANT, ORGANIZACIÓN Y ESTABLECIMIENTO

Debes revisar y formalizar las siguientes distinciones.

### 3.1. Persona

Representa a una persona real.

Debe distinguirse entre:

- Persona natural.
- Persona jurídica.

La persona natural puede ser propietaria y operadora de un tenant.

La persona jurídica puede ser titular de una organización empresarial que opera dentro de ENSAMBRIA.

No confundir la identidad de una persona con la identidad de su negocio.

Definir la relación entre:

- Identidad de usuario.
- Persona.
- Titular del tenant.
- Representante legal cuando corresponda.
- Propietarios.
- Administradores.
- Empleados.
- Colaboradores.
- Profesionales que prestan servicios.

Una persona puede tener diferentes roles dependiendo del tenant y de la organización donde participa.

No permitir que ser propietario de un tenant conceda automáticamente permisos globales fuera de su ámbito.

### 3.2. Tenant

Representa el ámbito de aislamiento y administración de una cuenta empresarial dentro de ENSAMBRIA.

Un tenant puede pertenecer a una persona natural o a una persona jurídica, conforme a las reglas de identificación y validación correspondientes.

El tenant no debe representar obligatoriamente:

- Una persona jurídica.
- Una sucursal.
- Un establecimiento físico.
- Una empresa con empleados.
- Una organización con múltiples ubicaciones.

Un profesional independiente debe poder crear su tenant aunque trabaje solo y no disponga de local.

El tenant debe conservar una identidad estable durante toda su evolución.

### 3.3. Organización

Representa la estructura empresarial que opera dentro del tenant.

Debe poder representar:

- Una actividad profesional individual.
- Un negocio administrado por una persona natural.
- Una empresa constituida jurídicamente.
- Un negocio con empleados.
- Una organización con varios establecimientos.
- Una organización que presta servicios a domicilio.
- Una organización con operación mixta.

Define su relación con tenant, persona titular, miembros y establecimientos.

No dupliques conceptos que ya tengan ownership definido en CORE o BUSINESS.

### 3.4. Establecimiento

Representa una ubicación física de operación empresarial.

Un establecimiento debe ser opcional.

Un tenant sin local no debe tener que registrar una sucursal ficticia para poder utilizar reservas, CRM, catálogo, agenda, pagos o cualquier otra capacidad autorizada.

Los establecimientos deben poder agregarse posteriormente sin cambiar la identidad del tenant.

Distinguir establecimiento, sucursal, ubicación del cliente, zona de cobertura y ubicación temporal.

No asumir que toda ubicación es una sucursal.

## 4. MODALIDADES DE REGISTRO EN ENSAMBRIA

Debes definir e integrar en el requerimiento el flujo inicial de registro.

El usuario debe poder seleccionar cómo presta sus servicios.

### Pantalla de selección

Título:

**¿Cómo prestas tus servicios?**

Opción 1:

**Tengo un negocio o establecimiento**

Descripción:

Tengo un salón, spa, clínica, gimnasio u otro establecimiento físico.

Opción 2:

**Soy profesional independiente**

Descripción:

Soy persona natural y presto servicios a domicilio o me desplazo hasta mis clientes.

Opción 3:

**Tengo un negocio con varias sucursales**

Descripción:

Administro varios establecimientos y equipos de trabajo.

Estas opciones son modalidades de incorporación y experiencia de registro, no necesariamente tipos rígidos e inmutables de tenant.

### 4.1. Registro de negocio con establecimiento

Permitir:

- Registro del titular.
- Identificación de persona natural o jurídica.
- Datos del negocio.
- Actividad e industria.
- Información comercial.
- Configuración de establecimientos.
- Servicios.
- Horarios.
- Personal, cuando corresponda.
- Ubicaciones.
- Métodos de atención.
- Configuración inicial.

La cantidad de establecimientos dependerá de la configuración comercial y de las capacidades contratadas, no de una limitación estructural del modelo.

### 4.2. Registro de profesional independiente

Debe permitir que una persona natural:

- Cree una cuenta.
- Configure su identidad.
- Cree su tenant.
- Se convierta en titular y administradora de su actividad profesional.
- Configure su perfil profesional.
- Seleccione su industria y especialidad.
- Registre sus servicios.
- Defina sus precios.
- Configure horarios y disponibilidad.
- Configure sus zonas de cobertura.
- Establezca las condiciones de desplazamiento.
- Configure sus medios de pago autorizados.
- Gestione clientes.
- Reciba y administre reservas.
- Consulte sus ingresos.
- Configure su perfil público cuando corresponda.
- Contrate un plan SaaS.

No exigir:

- Persona jurídica.
- NIT de persona jurídica.
- Matrícula de establecimiento como requisito universal.
- Dirección de local comercial.
- Sucursal física.
- Empleados.
- Inventario físico.
- Caja de establecimiento.

Los datos fiscales y de identificación que sean legalmente necesarios deben definirse según país, mercado, tipo de contribuyente y operación. No inventar requisitos legales universales.

El profesional puede declarar los datos de identificación y fiscales aplicables sin que la arquitectura imponga que todos son iguales.

### 4.3. Registro de negocio con varias sucursales

Debe permitir:

- Registrar la organización.
- Configurar el titular y representantes.
- Registrar varios establecimientos.
- Definir administradores por ubicación.
- Incorporar empleados.
- Configurar servicios por sucursal.
- Administrar horarios.
- Definir permisos y responsabilidades.
- Configurar atención presencial y a domicilio.
- Consultar información global y por establecimiento.
- Mantener separación de datos y permisos según el alcance de cada usuario.

No crear automáticamente una organización diferente por cada sucursal.

Las sucursales deben pertenecer a la misma estructura empresarial, con ownership y permisos definidos.

### 4.4. Flexibilidad de modalidades

No convertir la opción elegida durante el registro en una condición irreversible.

Un negocio puede comenzar como profesional independiente y luego abrir un local.

Un negocio con local puede incorporar servicios a domicilio.

Una empresa puede comenzar con un establecimiento y después incorporar más sucursales.

Un profesional puede incorporar colaboradores.

La arquitectura debe permitir estos cambios sin crear otra cuenta ni duplicar el tenant.

## 5. EVOLUCIÓN NATURAL DEL TENANT

Esta es una regla estructural obligatoria.

El tenant debe evolucionar progresivamente.

Ejemplo de ciclo de vida:

ETAPA 1 — PROFESIONAL INDEPENDIENTE

- Una persona natural.
- Un tenant.
- Una actividad profesional.
- Sin establecimiento.
- Servicios a domicilio.
- Agenda individual.
- Clientes.
- Reservas.
- Pagos.
- Perfil profesional.

ETAPA 2 — PROFESIONAL CON COLABORADORES

- Conserva el mismo tenant.
- Incorpora empleados o colaboradores.
- Administra perfiles profesionales.
- Asigna servicios.
- Administra agendas individuales.
- Define permisos.
- Gestiona disponibilidad.
- Distribuye reservas.

ETAPA 3 — NEGOCIO CON ESTABLECIMIENTO

- Conserva el mismo tenant.
- Registra su primer establecimiento.
- Configura atención presencial.
- Mantiene atención a domicilio si corresponde.
- Puede asignar profesionales a servicios presenciales o móviles.
- Mantiene clientes, reservas, historial y demás información.

ETAPA 4 — EMPRESA CON VARIAS SUCURSALES

- Conserva el mismo tenant.
- Administra varias sucursales.
- Define responsables por sucursal.
- Organiza equipos.
- Configura servicios por ubicación.
- Gestiona agendas y asignaciones.
- Mantiene operación móvil y presencial.
- Dispone de administración centralizada.

La evolución no debe provocar:

- Creación obligatoria de otro tenant.
- Duplicación de usuarios.
- Pérdida de reservas.
- Pérdida de clientes.
- Pérdida de historial.
- Pérdida de información financiera.
- Migraciones artificiales entre organizaciones.
- Ruptura de relaciones de ownership.
- Pérdida de auditoría.
- Reinicio de suscripciones sin una política explícita.

Si una modificación requiere cambios estructurales de datos, definir migraciones reales, transaccionales, reversibles cuando sea viable y verificadas.

## 6. MODALIDADES DE PRESTACIÓN DEL SERVICIO

El modelo debe permitir que un servicio se preste en una o varias modalidades.

Como mínimo:

1. En establecimiento.
2. A domicilio del cliente.
3. En ubicación acordada.
4. Remoto o virtual, cuando la actividad lo admita.

No todos los servicios admiten todas las modalidades.

Cada servicio debe declarar las modalidades permitidas.

Debe ser posible que un mismo negocio tenga:

- Servicios exclusivamente presenciales.
- Servicios exclusivamente a domicilio.
- Servicios en ambas modalidades.
- Servicios con condiciones diferentes según ubicación.

No duplicar artificialmente un servicio cuando únicamente cambia la modalidad, salvo que las reglas comerciales o de operación lo justifiquen.

Definir qué atributos pertenecen al servicio y cuáles a su modalidad de prestación.

## 7. CAPACIDAD EMPRESARIAL DE SERVICIOS A DOMICILIO

Incorporar al modelo de BUSINESS una capacidad transversal para servicios móviles y a domicilio.

Nombre conceptual:

**Mobile Services / Home Services — Servicios a domicilio y profesionales móviles.**

No confundir este dominio con repartidores, mensajería o logística de entrega de productos.

El objetivo es gestionar profesionales que se desplazan para prestar servicios personales o empresariales.

### 7.1. Zonas de cobertura

Definir:

- País.
- Ciudad.
- Municipio.
- Localidades o barrios cuando corresponda.
- Radios de cobertura.
- Polígonos geográficos cuando sea necesario.
- Áreas excluidas.
- Áreas habilitadas.
- Distancias máximas.
- Restricciones de servicio.
- Vigencia de la cobertura.

No depender exclusivamente de radios circulares si se necesitan límites geográficos reales.

No aceptar coordenadas o direcciones como prueba suficiente de que una zona está autorizada.

Definir validación de cobertura en backend.

### 7.2. Ubicación del cliente

Para una reserva a domicilio se debe poder registrar:

- Dirección.
- País y región.
- Ciudad o municipio.
- Información complementaria.
- Coordenadas cuando estén disponibles.
- Indicaciones de acceso.
- Referencias necesarias.
- Contacto para la atención.
- Restricciones relevantes para la prestación.

Aplicar minimización de datos y protección de información de ubicación.

No exponer direcciones privadas en perfiles públicos.

### 7.3. Desplazamientos

Definir reglas para:

- Tiempo estimado de desplazamiento.
- Margen entre reservas.
- Tiempo de preparación.
- Distancia.
- Disponibilidad del profesional.
- Compatibilidad geográfica.
- Capacidad diaria.
- Reservas consecutivas.
- Retrasos.
- Reprogramaciones.
- Cancelaciones.
- Incidencias de desplazamiento.

La disponibilidad no debe depender únicamente de que exista un espacio libre en el calendario.

Debe considerar si el profesional puede llegar al siguiente destino dentro de las condiciones configuradas.

### 7.4. Tarifas

Separar claramente:

- Precio del servicio.
- Cargo por desplazamiento.
- Cargo adicional autorizado.
- Descuentos.
- Impuestos aplicables.
- Total.

Definir quién es propietario de las reglas y cómo se calculan.

No inventar precios, comisiones, impuestos, distancias gratuitas ni políticas comerciales.

Los valores comerciales no definidos deben permanecer como decisiones pendientes de aprobación.

### 7.5. Asignación de profesionales

Permitir:

- Asignación manual.
- Asignación automática cuando exista una política aprobada.
- Asignación por especialidad.
- Disponibilidad.
- Zona de cobertura.
- Ubicación.
- Capacidad.
- Permisos.
- Compatibilidad con el servicio.

La asignación automática debe ser explicable, auditable y configurable.

No asumir que la persona titular debe ejecutar todas las reservas.

### 7.6. Estados de atención

Diseñar un ciclo de vida explícito, con transiciones autorizadas y auditables.

Evaluar estados como:

- Solicitud recibida.
- Pendiente de confirmación.
- Confirmada.
- Profesional asignado.
- Preparación.
- Desplazamiento iniciado.
- Profesional en ubicación.
- Servicio iniciado.
- Servicio finalizado.
- Cancelada.
- Reprogramada.
- Incidencia.

No introducir estados redundantes ni permitir transiciones arbitrarias.

Definir quién puede realizar cada transición y bajo qué condiciones.

### 7.7. Geolocalización y privacidad

La geolocalización debe ser opcional cuando no sea necesaria y estar sujeta a consentimiento y políticas aplicables.

Distinguir:

- Ubicación del establecimiento.
- Ubicación del cliente.
- Zona de cobertura.
- Ubicación del profesional.
- Ubicación durante un desplazamiento.

No implementar seguimiento permanente injustificado.

Definir acceso, finalidad, retención, visibilidad y auditoría.

## 8. PERSONAL, EMPLEADOS Y COLABORADORES

Un profesional independiente debe poder comenzar solo y posteriormente incorporar personal.

La arquitectura debe contemplar:

- Miembros del tenant.
- Empleados.
- Colaboradores.
- Profesionales prestadores.
- Administradores.
- Responsables de establecimientos.
- Roles operativos.
- Permisos.
- Agenda individual.
- Especialidades.
- Disponibilidad.
- Asignación de servicios.
- Relación con sucursales.
- Relación con atención móvil.
- Historial de actividad.

No asumir que todo colaborador es empleado laboralmente.

Distinguir las relaciones laborales, contractuales y operativas conforme a las necesidades del dominio y los mercados.

No inventar políticas laborales, comisiones ni obligaciones legales.

Definir las relaciones entre personas, membresías de tenant, workforce y profesionales prestadores evitando duplicar identidad.

Un profesional puede estar asociado a un tenant y prestar servicios en una o varias ubicaciones autorizadas.

Los permisos deben limitarse por producto, tenant, organización, sucursal y función según corresponda.

## 9. AGENDA Y RESERVAS MULTIMODALES

Las reservas deben poder funcionar sin sucursal física.

Definir cómo se relacionan:

- Tenant.
- Servicio.
- Modalidad.
- Profesional.
- Cliente.
- Agenda.
- Disponibilidad.
- Ubicación.
- Zona de cobertura.
- Tiempo de desplazamiento.
- Reserva.
- Pago.
- Estado.
- Historial.

Una reserva presencial debe poder asociarse a un establecimiento.

Una reserva a domicilio debe asociarse a una ubicación de atención del cliente.

Una reserva remota debe cumplir sus condiciones específicas.

No crear una sucursal ficticia para representar el domicilio del cliente.

La reserva debe conservar su ubicación histórica aunque el negocio modifique posteriormente sus direcciones o cobertura.

Definir concurrencia, idempotencia, prevención de doble reserva, bloqueos, reintentos, cancelaciones y recuperación de errores.

## 10. CRM, CLIENTES Y RELACIONES

Los profesionales independientes deben disponer de CRM según sus permisos y plan.

El modelo debe permitir:

- Registrar clientes.
- Mantener historial de servicios.
- Gestionar reservas.
- Registrar preferencias autorizadas.
- Consultar actividad.
- Administrar comunicaciones.
- Gestionar consentimientos.
- Relacionar clientes con el tenant.
- Evitar duplicados según reglas aprobadas.
- Mantener aislamiento entre tenants.

La misma persona cliente puede relacionarse con distintos negocios sin que estos compartan automáticamente sus historiales privados.

No permitir que una empresa consulte la información de clientes de otra.

## 11. PAGOS, FINANZAS Y SUSCRIPCIONES

Los profesionales independientes deben poder acceder a las capacidades financieras habilitadas para su modalidad y mercado.

Distinguir:

1. Suscripción que el profesional paga a ENSAMBRIA.
2. Pagos que el cliente final realiza al profesional o negocio.
3. Comisiones de plataforma, si están aprobadas.
4. Cargos por desplazamiento.
5. Reembolsos.
6. Liquidaciones.
7. Conciliación.
8. Registro financiero.

No confundir el cobro del SaaS con el cobro de servicios prestados por los tenants.

No asumir que una persona natural puede utilizar cualquier mecanismo de cobro sin verificar las condiciones del proveedor de pagos y del mercado.

No inventar reglas fiscales, tributarias o financieras.

Mantener el ownership financiero centralizado conforme al requerimiento vigente.

## 12. MODELO DE UBICACIONES

Definir un modelo que distinga claramente:

- Establecimiento.
- Sucursal.
- Domicilio del cliente.
- Ubicación temporal.
- Zona de cobertura.
- Ubicación del profesional.
- Ubicación remota.

Un establecimiento pertenece a una estructura empresarial.

Un domicilio de cliente pertenece al contexto de atención y a la reserva correspondiente.

Una zona de cobertura representa un área donde se puede prestar un servicio.

Una ubicación temporal no debe convertirse automáticamente en una sucursal.

No reutilizar una misma entidad para representar conceptos con reglas y ownership diferentes.

Definir qué datos se guardan, quién los administra, quién puede consultarlos y cuánto tiempo se conservan.

## 13. COMPATIBILIDAD CON LA ARQUITECTURA MULTIPRODUCTO Y MULTIVERTICAL

Esta funcionalidad debe ser reutilizable en la plataforma multiproducto definida para ENSAMBRIA.

No limitar el modelo de profesionales independientes exclusivamente a Beauty & Wellness.

Debe ser posible adaptar servicios móviles a otras industrias, por ejemplo:

- Belleza y bienestar a domicilio.
- Veterinaria a domicilio.
- Fisioterapia a domicilio.
- Entrenamiento personal.
- Servicios técnicos.
- Reparaciones.
- Limpieza.
- Otros servicios móviles.

Sin embargo, no asumir que todas las industrias comparten las mismas reglas.

Cada vertical debe declarar qué capacidades utiliza y qué restricciones específicas necesita.

No implementar los verticales futuros únicamente para demostrar compatibilidad.

## 14. OWNERSHIP EN LOS CUATRO BACKENDS

Conservar los cuatro backends establecidos.

### CORE

Responsable de:

- Identidad.
- Autenticación.
- Autorización.
- Tenancy.
- Contexto de seguridad.
- Organizaciones y relaciones centrales conforme al ownership ratificado.
- Entitlements.
- Aislamiento.
- Contexto de producto y tenant.

### BUSINESS

Responsable de:

- Operaciones empresariales.
- Servicios.
- Verticales.
- Workforce.
- Profesionales.
- Agenda y reservas conforme a su ownership.
- Servicios móviles.
- Zonas de cobertura.
- Asignación.
- CRM como capacidad independiente.
- Reglas operativas.

### PLATFORM

Responsable de:

- Administración global.
- Productos SaaS.
- Planes.
- Suscripciones.
- Facturación SaaS.
- Políticas comerciales.
- Límites contratados.
- Administración de capacidades y entitlements conforme al ownership establecido.

### PUBLIC

Responsable de:

- Perfiles públicos autorizados.
- Discovery.
- Directorios.
- Proyecciones públicas.
- Disponibilidad pública autorizada.
- Información de negocios y profesionales que pueda publicarse.

No duplicar ownership.

No crear un quinto backend.

No crear un backend exclusivo para profesionales independientes.

No permitir acceso directo a bases de datos ajenas.

No invocar implementaciones internas de otros módulos.

Utilizar contratos explícitos y mecanismos de comunicación autorizados.

## 15. REGISTRO, ONBOARDING Y EXPERIENCIA DE USUARIO

Diseñar un onboarding coherente con las tres modalidades iniciales:

1. Tengo un negocio o establecimiento.
2. Soy profesional independiente.
3. Tengo un negocio con varias sucursales.

El onboarding debe solicitar únicamente la información necesaria para la modalidad elegida, la industria, el mercado y las obligaciones aplicables.

No pedir datos de sucursales a un profesional independiente que no tiene establecimiento.

No pedir información empresarial de persona jurídica a quien se registra legítimamente como persona natural.

No exigir información de empleados si todavía trabaja solo.

Debe permitir completar la configuración progresivamente.

Definir:

- Pasos.
- Validaciones.
- Persistencia del progreso.
- Reanudación.
- Idempotencia.
- Recuperación de errores.
- Verificación de identidad cuando aplique.
- Validación de datos.
- Prevención de duplicados.
- Confirmación de titularidad.
- Asignación de roles.
- Creación del tenant.
- Creación de la organización cuando corresponda.
- Configuración de modalidad.
- Activación de capacidades.

Evitar transacciones parciales que dejen cuentas o tenants inconsistentes.

Definir compensaciones y recuperación cuando intervengan varios dominios.

## 16. SEGURIDAD Y PRIVACIDAD

El diseño debe considerar:

- Aislamiento por tenant.
- Separación de personas y organizaciones.
- Control de acceso.
- Protección de datos personales.
- Direcciones privadas.
- Geolocalización.
- Historial de clientes.
- Datos financieros.
- Información de empleados.
- Auditoría.
- Consentimientos.
- Retención.
- Redacción de datos sensibles.
- Acceso por sucursal.
- Acceso por función.

RLS debe aplicarse donde corresponda.

El contexto transaccional de seguridad debe ser obligatorio en las operaciones tenant-scoped.

El frontend nunca debe ser la autoridad final para validar permisos, modalidad o pertenencia a un tenant.

No aceptar el tenant ID, product ID, branch ID o ubicación como prueba suficiente de autorización.

Definir controles negativos y pruebas contra acceso cruzado.

## 17. DOCUMENTACIÓN Y MATRICES OBLIGATORIAS

Incorporar el modelo en las fuentes normativas existentes.

Evaluar actualizaciones en:

- `docs/requerimiento.md`.
- `ARCHITECTURE_RULES.md`.
- `docs/arquitectura-dominios.md`.
- Registro de Fase 0.
- ADR correspondientes.
- Matriz de dominios y ownership.
- Matriz de capacidades.
- Matriz de entidades.
- Matriz de productos y verticales.
- Matriz de roles y permisos.
- Matriz de modalidades de servicio.
- Matriz de ubicaciones.
- Matriz de transiciones de reservas.
- Matriz de dependencias.
- Matriz de fases.
- Matriz de aceptación.
- Catálogo de eventos.
- Contratos de API y gRPC.
- Estrategia de auditoría.
- Reglas de agentes y skills aplicables.

La documentación debe responder como mínimo:

1. ¿Qué es un tenant?
2. ¿Puede pertenecer a una persona natural?
3. ¿Puede existir sin persona jurídica?
4. ¿Puede existir sin establecimiento?
5. ¿Cómo se representa una organización?
6. ¿Cómo se representan las sucursales?
7. ¿Cómo se registra un profesional independiente?
8. ¿Cómo se incorporan empleados?
9. ¿Cómo se prestan servicios a domicilio?
10. ¿Cómo se representan las ubicaciones?
11. ¿Cómo se calculan y controlan los desplazamientos?
12. ¿Cómo se administran las reservas?
13. ¿Cómo se produce la evolución del negocio?
14. ¿Cómo se mantienen clientes e historial?
15. ¿Cómo se controlan permisos y seguridad?
16. ¿Cómo se integran pagos y suscripciones?
17. ¿Qué capacidades son transversales?
18. ¿Qué reglas pertenecen a cada vertical?
19. ¿Qué requisitos dependen de la legislación de cada mercado?
20. ¿Cómo se prueba todo lo anterior?

## 18. ROADMAP: DEFINICIÓN DESDE FASE 0

El modelo debe definirse desde Fase 0.

### Fase 0 — Descubrimiento y arquitectura

Debe quedar definido:

- Modelo persona natural/persona jurídica.
- Modelo tenant.
- Modelo organización.
- Modelo establecimiento y sucursal.
- Tenant sin establecimiento.
- Registro de profesional independiente.
- Modalidades de operación.
- Modalidades de prestación del servicio.
- Servicios a domicilio.
- Zonas de cobertura.
- Ubicación del cliente.
- Desplazamientos.
- Workforce.
- Evolución de negocio individual a organización.
- Ownership.
- Seguridad.
- Contratos.
- Eventos.
- Auditoría.
- Dependencias.
- Pruebas y criterios de aceptación.
- Requisitos por mercado.

No crear runtime, migraciones, APIs ejecutables ni módulos productivos en Fase 0 si contradicen las reglas vigentes de esa fase.

### Fase 1 — Foundation

Implementar únicamente los elementos de foundation que correspondan al roadmap vigente, incluyendo el soporte técnico necesario para el modelo aprobado.

No marcar como implementados los servicios móviles por el mero hecho de que tenancy permita existir sin sucursal.

### Fases funcionales posteriores

Distribuir la implementación de:

- Onboarding.
- Organizaciones y establecimientos.
- Profesionales.
- Workforce.
- Servicios móviles.
- Reservas.
- CRM.
- Pagos.
- Discovery.
- Integraciones.

Respetar las fases y dependencias de `docs/requerimiento.md`.

Si alguna funcionalidad ya está asignada a una fase, no moverla arbitrariamente.

No adelantar implementación a Fase 0.

No declarar completada una capacidad únicamente porque se haya documentado.

## 19. PRUEBAS Y CALIDAD OBLIGATORIAS

Diseñar una matriz de pruebas requisito → caso → ejecución → evidencia.

Incluir escenarios positivos y negativos.

### Tenancy e identidad

- Persona natural crea tenant sin sucursal.
- Persona jurídica crea tenant.
- Tenant sin organización física.
- Tenant con una organización.
- Tenant con múltiples sucursales.
- Persona con diferentes roles en tenants diferentes.
- Usuario sin autorización.
- Suplantación de tenant.
- Acceso cruzado.

### Evolución

- Profesional independiente incorpora colaborador.
- Profesional independiente registra establecimiento.
- Negocio con establecimiento activa atención a domicilio.
- Negocio incorpora otra sucursal.
- Cambio de titularidad, cuando esté permitido y validado.
- Cambio de modalidad.
- Conservación de clientes.
- Conservación de reservas.
- Conservación de historial.
- Conservación de información financiera.
- Fallos y recuperación durante la transición.

### Servicios móviles

- Servicio dentro de cobertura.
- Servicio fuera de cobertura.
- Dirección incompleta.
- Ubicación inválida.
- Profesional no disponible.
- Tiempo de desplazamiento incompatible.
- Reservas consecutivas.
- Asignación concurrente.
- Cancelación.
- Reprogramación.
- Retraso.
- Incidencia.
- Fallo de geolocalización.
- Fallo de proveedor externo.
- Reintentos.
- Idempotencia.

### Seguridad

- Aislamiento por tenant.
- RLS.
- Permisos por organización y sucursal.
- Protección de direcciones.
- Protección de geolocalización.
- Acceso a historiales.
- Auditoría.
- Redacción de datos sensibles.

### Calidad

- Pruebas unitarias.
- Pruebas de integración.
- Pruebas de contratos.
- Pruebas de aceptación.
- Pruebas de concurrencia.
- Pruebas de aislamiento.
- Pruebas de regresión.
- Pruebas E2E cuando corresponda.

No utilizar mocks, fakes ni implementaciones simuladas para sustituir las funcionalidades reales.

No debilitar validaciones ni controles de seguridad para conseguir que las pruebas pasen.

## 20. REGLAS DE IMPLEMENTACIÓN

Quedan prohibidos:

- Obligar a todos los tenants a tener sucursales.
- Crear sucursales ficticias.
- Crear tenants separados para cada modalidad de un mismo negocio.
- Duplicar cuentas para crecer.
- Tratar a todo tenant como persona jurídica.
- Confundir usuario con titular del negocio.
- Confundir ubicación de cliente con establecimiento.
- Confundir servicios a domicilio con reparto de productos.
- Mezclar reglas de workforce con identidad.
- Duplicar CRM por modalidad.
- Duplicar reglas de reservas.
- Introducir lógica de modalidad dispersa por el sistema.
- Crear un módulo monolítico de servicios a domicilio con responsabilidades ajenas.
- Crear un backend adicional.
- Crear carpetas vacías.
- Crear APIs placeholder.
- Crear migraciones ficticias.
- Crear datos de prueba como sustituto de implementación.
- Inventar políticas comerciales, laborales o fiscales.
- Habilitar funciones no implementadas mediante configuración.
- Declarar capacidades listas sin pruebas y evidencia.

Toda abstracción debe responder a una necesidad real y tener ownership explícito.

## 21. PROCEDIMIENTO DE TRABAJO EN VS CODE CON GITHUB COPILOT

El entorno de ejecución es GitHub Copilot dentro de VS Code.

No asumas Codex CLI ni otros agentes externos.

Antes de modificar:

1. Comprueba rama y estado de Git.
2. Protege todos los cambios existentes.
3. Inspecciona las fuentes normativas.
4. Comprueba los agentes y skills realmente disponibles.
5. Identifica contradicciones.
6. Construye un plan de cambios.
7. Identifica dependencias y bloqueos.

No afirmes que has delegado trabajo si no existe evidencia de una invocación real.

No inventes resultados de herramientas.

No trabajes directamente sobre `main`.

No hagas push directo a `main`.

No uses `git add .` indiscriminadamente.

No ejecutes operaciones destructivas.

No modifiques scripts de CI salvo que sea estrictamente necesario y esté dentro del alcance autorizado.

Si no existe autorización para modificar archivos, presenta primero el diagnóstico y el plan, sin ejecutar cambios.

## 22. CRITERIOS DE ACEPTACIÓN

El trabajo documental y arquitectónico solo podrá considerarse completo cuando:

- [ ] Se defina la diferencia entre persona, tenant, organización y establecimiento.
- [ ] Una persona natural pueda ser titular de un tenant.
- [ ] Un tenant pueda existir sin establecimiento.
- [ ] No se exija persona jurídica universalmente.
- [ ] Se definan las tres modalidades iniciales de registro.
- [ ] Se defina el onboarding de profesionales independientes.
- [ ] Se defina el modelo de negocio con empleados.
- [ ] Se defina el modelo de empresa con múltiples sucursales.
- [ ] Se permita combinar atención presencial y a domicilio.
- [ ] Se defina la capacidad de servicios móviles.
- [ ] Se definan zonas de cobertura y desplazamientos.
- [ ] Se defina la relación con workforce y reservas.
- [ ] Se defina el ownership de todos los elementos.
- [ ] Se definan permisos y aislamiento.
- [ ] Se defina la evolución del tenant sin duplicación.
- [ ] Se establezcan criterios de migración y conservación de datos.
- [ ] Se definan pruebas positivas, negativas y de concurrencia.
- [ ] Se actualicen las fuentes normativas.
- [ ] Se actualice el roadmap sin adelantar fases.
- [ ] Se identifiquen decisiones pendientes.
- [ ] Se distingan los elementos documentados de los implementados.

## 23. INFORME FINAL OBLIGATORIO

Presenta:

1. Estado inicial del modelo de tenancy.
2. Problemas y limitaciones encontrados.
3. Modelo conceptual final.
4. Definición de persona natural y jurídica.
5. Definición de tenant, organización y establecimiento.
6. Flujo completo de las tres modalidades de registro.
7. Ciclo de evolución de un tenant.
8. Modelo de servicios móviles.
9. Modelo de workforce y colaboradores.
10. Modelo de agenda y reservas.
11. Modelo de ubicaciones y cobertura.
12. Ownership en CORE, BUSINESS, PLATFORM y PUBLIC.
13. Reglas de seguridad y aislamiento.
14. Documentos actualizados.
15. ADR creados o modificados.
16. Cambios del roadmap.
17. Matriz de pruebas.
18. Validaciones realizadas.
19. Decisiones pendientes.
20. Criterios de aceptación cumplidos y pendientes.

No declares finalizado un criterio que solamente haya sido mencionado en documentación.

No declares cerrada una fase sin cumplir su definición de terminado.

---

# DIRECTIVA ARQUITECTÓNICA FINAL

ENSAMBRIA debe permitir que una persona natural comience sola, sin local, sin empleados y sin persona jurídica, y que utilice la plataforma para administrar profesionalmente sus servicios.

También debe permitir que ese mismo negocio crezca, incorpore colaboradores, abra su primer establecimiento, añada nuevas sucursales y combine atención presencial con servicios a domicilio.

**Todo debe ocurrir dentro del mismo tenant, conservando su identidad, relaciones, clientes, reservas, historial, información financiera y trazabilidad.**

La existencia de una sucursal física debe ser opcional, nunca un requisito estructural de tenancy.

Las modalidades de operación deben ser configurables y evolucionar sin obligar a duplicar cuentas, organizaciones o datos.

El modelo debe ser reutilizable por todas las industrias compatibles de la plataforma multiproducto, respetando las particularidades de cada vertical.

Esta decisión debe quedar definida desde Fase 0 y convertirse en una restricción arquitectónica obligatoria para todas las implementaciones futuras de ENSAMBRIA.