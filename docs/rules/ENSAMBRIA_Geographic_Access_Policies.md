# ENSAMBRIA — Política de acceso geográfico y disponibilidad por mercado

**Categoría:** Política de producto, arquitectura, privacidad y control de acceso
**Aplicación:** Sitios públicos, registro, cuentas, productos, operaciones comerciales y restricciones de acceso por red de ENSAMBRIA
**Vigencia:** Diseño y evaluación desde Fase 0; implementación con cada capacidad que aplique desde la fase correspondiente
**Propietarios:** PLATFORM Policy Owner, Product Owner, Architecture Owner y Privacy/Legal Owner según sus responsabilidades

Esta política define cómo evaluar restricciones geográficas y de red sin bloquear indiscriminadamente el producto ni tratar la geolocalización por IP como prueba concluyente. PLATFORM administra y publica las políticas; cada backend las aplica a las operaciones bajo su propiedad. Complementa [ENSAMBRIA_Privacy_Legal_Compliance.md](ENSAMBRIA_Privacy_Legal_Compliance.md), [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](ENSAMBRIA_Privacy_Data_Lifecycle_Management.md), [ENSAMBRIA_Transactional_Outbox_Inbox.md](ENSAMBRIA_Transactional_Outbox_Inbox.md) y el estándar web [ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md](ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md). No constituye asesoría jurídica ni aprobación legal. La política especializada de privacidad/cumplimiento conserva la autoridad sobre aplicabilidad legal y tratamiento de datos.

## 1. Objetivo y principios

Las restricciones territoriales deben aplicarse únicamente al alcance, actividad y mercado para los que exista una razón legal, contractual, operativa o de producto documentada. ENSAMBRIA debe distinguir navegación, contacto, registro y operaciones comerciales, y ofrecer alternativas accesibles cuando una actividad no esté habilitada.

Principios obligatorios:

1. No bloquear todo el producto por la ubicación estimada de una persona.
2. No inferir residencia, nacionalidad, identidad ni jurisdicción legal únicamente de una IP.
3. No imponer restricciones territoriales más amplias que el requisito documentado.
4. No habilitar una operación comercial cuando una obligación crítica pendiente o una restricción validada la prohíba.
5. Mantener disponibles las páginas informativas y los canales de contacto/solicitud de información de lanzamiento, salvo una prohibición específica validada.
6. Mantener accesible el canal para ejercer derechos de privacidad, independientemente de que el producto comercial esté disponible en el país; las verificaciones de identidad para dichas solicitudes se rigen por la política de privacidad y no por la IP.
7. Minimizar el tratamiento de datos de ubicación, controlar el acceso y la retención conforme a privacidad y cumplimiento.
8. Informar al usuario de forma clara, neutral y accionable cuando una capacidad esté restringida, y ofrecer revisión cuando sea posible.

## 2. Clases de acceso

La decisión se evalúa por capacidad solicitada, no como una denegación global de toda ENSAMBRIA.

| Clase | Comportamiento predeterminado | Regla de restricción |
|---|---|---|
| Navegación pública e información institucional | Permitir lectura de contenido público general. | Restringir solo cuando exista una obligación específica aplicable y validada para ese contenido. No convertir un mercado comercial no habilitado en bloqueo total del sitio. |
| Información de disponibilidad y lanzamiento | Permitir solicitar información, registrarse en una lista de interés o contactar a ENSAMBRIA, sujeto a aviso de privacidad y controles aplicables. | No tratar esa solicitud como alta comercial ni como autorización para otros tratamientos o marketing. Obtener preferencias/consentimiento cuando corresponda. |
| Solicitudes de privacidad | Mantener un canal público y accesible para presentar solicitudes y obtener instrucciones. | No bloquear por geolocalización ni exigir que la IP corresponda a una jurisdicción. La identidad y legitimación se verifican según el procedimiento especializado de privacidad. |
| Registro, creación de tenant o contratación | Evaluar separadamente país, rol, actividad, producto, establecimiento y requisitos de entrada al mercado. | Permitir, limitar o rechazar el alta según una política versionada y revisada; no usar la restricción comercial para bloquear páginas informativas o solicitudes de privacidad. |
| Operaciones comerciales o reguladas | Evaluar la jurisdicción y las condiciones pertinentes a la operación concreta (por ejemplo, contratación, pago, prestación o publicación). | Denegar o mantener deshabilitada únicamente la operación o capacidad restringida hasta cumplir requisitos legales, contractuales y operativos validados. |
| Restricción por ciudad, región o zona de servicio | Aplicar solo al recurso o servicio cuya elegibilidad depende de esa ubicación, como disponibilidad de una sucursal o cobertura local. | No basar una denegación estricta de ciudad únicamente en geolocalización por IP. Requerir verificación adicional proporcional y aprobada. |

El estado de mercado habilitado, la disponibilidad de un producto, la jurisdicción del tratamiento y la ubicación física del usuario son dimensiones relacionadas pero no intercambiables. No codificar una única regla global del tipo “país no habilitado = sitio inaccesible”.

## 3. Determinación de ubicación y calidad de evidencia

### 3.1 IP y señales de red

La IP y los datos de geolocalización de Cloudflare u otro proveedor son señales aproximadas. Su precisión puede variar por proveedor, red móvil, VPN/proxy, NAT, asignación del ISP, configuración y granularidad geográfica; la ubicación de ciudad o región no es garantía de presencia física. Una discrepancia o falta de señal no demuestra por sí misma fraude ni elegibilidad.

La IP puede utilizarse, con finalidad documentada, para:

- Seleccionar una experiencia regional o sugerir moneda/idioma.
- Aplicar controles de abuso o seguridad con límites definidos.
- Iniciar una evaluación de elegibilidad de mercado que permita corroboración.
- Mostrar un aviso o solicitar confirmación adicional.

La IP no puede ser la única evidencia para una restricción territorial estricta a nivel de ciudad. Tampoco es prueba suficiente de residencia, nacionalidad, domicilio, lugar de contratación o jurisdicción aplicable.

### 3.2 Confianza en cabeceras de geolocalización

Las cabeceras geográficas solo se aceptan desde la infraestructura perimetral confiable configurada y autenticada para ENSAMBRIA. El borde debe reemplazar o validar las cabeceras entrantes; la aplicación no debe confiar en valores suministrados directamente por el cliente. Si la ruta de red permite acceso que evita el proxy confiable, la cabecera no constituye señal confiable.

### 3.3 Verificación adicional

Cuando una regla territorial estricta requiera confirmar ciudad, región o cobertura, definir antes de habilitarla un mecanismo adicional y proporcional al riesgo. Las opciones dependen del caso de uso y requieren aprobación del Product Owner, Architecture Owner y Privacy/Legal Owner; pueden incluir una dirección del establecimiento/sucursal verificada, dirección de prestación o servicio confirmada, un dato de facturación validado por un proveedor autorizado, evidencia documental estrictamente necesaria o revisión manual.

Estos ejemplos no son requisitos universales ni autorizan recopilar documentos o datos adicionales por defecto. El registro de política debe especificar qué evidencia basta, cómo se verifica, quién la procesa, cuánto se conserva, cómo se corrige una clasificación errónea y qué alternativa existe cuando la persona no puede aportar esa evidencia. Elegir un método que no recolecte datos excesivos y validar su licitud y seguridad antes de implementarlo.

Si solo existe una estimación IP con confianza insuficiente, no aplicar una denegación definitiva de ciudad. Para navegación pública, información de lanzamiento y solicitudes de privacidad, la incertidumbre geográfica no bloquea el acceso. Para una operación comercial potencialmente restringida, solicitar corroboración o mantener esa operación concreta pendiente/no disponible según la política aprobada.

## 4. Registro de políticas y decisión

Las reglas se mantienen centralizadas, versionadas, auditables y evaluables de forma consistente en las superficies y backends propietarios. Antes de implementar una regla, su registro debe incluir al menos:

- Identificador y versión de política; estado y periodo efectivo.
- País, región/ciudad o ámbito territorial; precisión necesaria.
- Dirección IP/CIDR/ASN normalizados y fuente/confiabilidad de señales de red, cuando aplique.
- Acción, selector de ámbito (global, tenant, usuario, aplicación u operación) y reglas explícitas de combinación.
- Política de excepción: regla objetivo, alcance, no-anulabilidad, aprobaciones requeridas y vigencia.
- Capacidad/actividad afectada y clases de acceso que permanecen permitidas.
- Motivo de la restricción y fuente normativa, contractual u operativa.
- Aplicabilidad y aprobación de Legal cuando la razón sea jurídica.
- Propietario de producto y responsable técnico.
- Señales permitidas, nivel de confianza, corroboración requerida y tratamiento de incertidumbre.
- Resultado permitido, limitado, pendiente o denegado y mensaje/alternativa para la persona.
- Datos personales implicados, finalidad, base aplicable, minimización, controles de acceso y retención.
- Casos de prueba, evidencia, revisión periódica y procedimiento de corrección/apelación.

Una obligación crítica que aún no haya sido validada no se presenta como regla legal confirmada. Puede bloquearse preventivamente el alcance comercial dependiente de acuerdo con el proceso de lanzamiento, pero no se extiende por defecto al contenido público general ni a canales de privacidad.

### 4.1 Autoridad, ownership y aplicación

PLATFORM es propietario de la administración, configuración, versionado, publicación, excepciones administrativas y auditoría de las políticas de acceso geográfico y de red. Esta autoridad no convierte a PLATFORM en propietario de la identidad, las sesiones, los datos operativos ni la autorización de los demás dominios:

- CORE autentica y valida identidad, sesión, contexto de tenant y permisos centrales; aplica las políticas a identidad, tenancy y operaciones centrales que posee.
- BUSINESS aplica las políticas a operaciones de establecimientos, workforce, catálogo operativo, Commerce y demás capacidades bajo su propiedad.
- PLATFORM aplica las políticas a la administración global y las operaciones comerciales SaaS que posee.
- PUBLIC aplica las políticas a directorio, Discovery, proyecciones públicas y operaciones del canal Marketplace que posee.
- Cada BFF aplica las salvaguardas de entrada correspondientes, pero no reemplaza la validación del backend propietario ni convierte una decisión de red en autorización de negocio.

Debe existir una autoridad única de configuración y un contrato versionado común para publicar, identificar y resolver las políticas. Las reglas no se duplican en listas locales independientes ni se implementan como motores de negocio incompatibles por backend. Cada backend aplica la versión autorizada en su propia frontera antes de producir efectos protegidos, y mantiene su autorización normal independiente de la evaluación geográfica/de red. La ubicación concreta del evaluador, la estrategia de consulta/distribución y sus garantías de consistencia requieren decisión de arquitectura antes del runtime; esta política no aprueba un nuevo microservicio, acceso cruzado a tablas ni una llamada remota por petición.

Las políticas globales y los eventos de distribución no otorgan permisos a usuarios ni sustituyen identidad, RBAC, entitlements, autorización tenant-scoped o controles antifraude. Un `ALLOW` de red solo significa que esta política no deniega la solicitud; no autoriza la operación por sí mismo.

### 4.2 Direcciones, redes y señales de red

La política admite los siguientes tipos, que deben almacenarse y evaluarse como valores tipados y normalizados, no como cadenas comparadas manualmente:

- Dirección IPv4 individual.
- Dirección IPv6 individual.
- Red o rango IPv4 expresado en CIDR.
- Red o rango IPv6 expresado en CIDR, incluida una red completa representada mediante su prefijo CIDR.
- ASN únicamente cuando se disponga de una fuente de identificación confiable, versionada y con vigencia conocida.
- Allowlist y denylist de direcciones o redes dentro de un alcance explícito.

La implementación debe emplear una biblioteca mantenida y mecanismos estándar para parseo, normalización, validación de prefijos y pertenencia a redes IPv4/IPv6. Debe rechazar entradas malformadas, formatos ambiguos y valores no permitidos antes de guardar o publicar una regla. No implementar aritmética de redes ni comparación de IP mediante parsing manual o coincidencia de texto. La biblioteca y la estrategia de normalización se seleccionan tras inspeccionar el stack real; esta guía no fija una dependencia.

La normalización debe evitar reglas duplicadas semánticamente equivalentes y detectar redes contenidas o superpuestas para presentarlas a revisión. El solapamiento no debe eliminarse ni cambiar silenciosamente el alcance de una regla existente. Una señal ASN, VPN, proxy, Tor, hosting o reputación de red es indicio de riesgo, no prueba de identidad, residencia ni actividad maliciosa; cualquier uso restrictivo debe declarar fuente, calidad, antigüedad y respuesta ante incertidumbre.

### 4.3 Ámbitos y combinación de restricciones

Una política puede dirigirse a uno o más de estos ámbitos, que deben quedar explícitos y tipados:

| Ámbito | Alcance |
|---|---|
| Global ENSAMBRIA | Experiencias y servicios de ENSAMBRIA, salvo excepciones expresamente aprobadas y permitidas por la regla |
| Tenant | Un tenant identificado por contexto autenticado/verificado; nunca implica alcance global |
| Usuario | Una identidad autenticada o un contexto de identificación válido, verificado por CORE |
| Aplicación | `ensambria-business`, `tenant-portal`, `ensambria-app`, `marketplace`, `platform-admin` o `corporate` |
| Operación | Una operación nombrada y versionada, por ejemplo autenticación, registro, creación de tenant/reserva, inicio de pago, checkout, Marketplace, administración, APIs móviles u operación comercial |

Una regla combinada aplica solo cuando coincide la dirección/señal y todos los selectores de ámbito declarados. Los ámbitos no especificados no deben inferirse. Antes de autenticación no se debe confiar en un `user_id` suministrado por el cliente ni revelar si una cuenta tiene reglas personales; las políticas de usuario se aplican después de validar la identidad/contexto correspondiente. La identificación previa a autenticación solo puede utilizar un flujo explícito, limitado y que no permita enumeración de cuentas.

El tenant de una evaluación debe provenir del contexto autenticado y autorizado, no de un parámetro libre. Un error de resolución de tenant no puede ampliar una regla tenant-scoped a todo ENSAMBRIA. La evaluación conserva el alcance efectivo de la solicitud y no permite que un selector específico de tenant, usuario, aplicación u operación se convierta en bloqueo global por omisión, error de parsing o caché.

### 4.4 Acciones, vigencia e historial

Las acciones posibles son `ALLOW`, `DENY`, `REVIEW` y `CHALLENGE`. `REVIEW` requiere el flujo de revisión definido; `CHALLENGE` solo puede solicitar una comprobación adicional si existe un mecanismo implementado, seguro y accesible. Ninguna acción puede presentarse como implementada por estar documentada.

Una regla puede definir bloqueo temporal o permanente y debe tener, según corresponda: fecha/hora de inicio, expiración, motivo administrativo codificado, descripción interna, mensaje público seguro, responsable creador/modificador, fechas de creación/modificación, estado (borrador, programada, publicada, desactivada, expirada o eliminada lógicamente), versión y referencia al historial. Se usa una fuente temporal confiable y UTC para evaluar vigencia; reglas futuras o expiradas no se tratan como activas.

La expiración debe producirse automáticamente y ser verificable. La evaluación comprueba también el intervalo de vigencia para no depender de que un job de limpieza se ejecute puntualmente. El proceso de expiración deja evidencia auditable y, si el cambio afecta a consumidores distribuidos, publica la actualización/versionado correspondiente. La eliminación lógica conserva el historial y no elimina evidencia antes del plazo aprobado. Los cambios se versionan; modificar o revertir una política crea una versión nueva en lugar de alterar el historial previo.

### 4.5 Precedencia y resolución determinista

La decisión se calcula para una solicitud y una versión identificable de las políticas. La ubicación geográfica y la señal de red se evalúan como dimensiones distintas; una operación solo puede continuar si ninguna política aplicable exige denegar, revisar o completar un desafío.

Orden obligatorio de resolución:

1. Rechazar una política malformada, no autenticada, no publicada, fuera de vigencia o con versión/integridad no verificable; no reinterpretarla como una regla más amplia.
2. Evaluar las políticas geográficas y de red que coinciden con la operación, aplicación y contexto autenticado/verificado.
3. Aplicar primero cualquier denegación global crítica marcada explícitamente como no anulable. Una excepción no puede anularla.
4. Considerar solo excepciones explícitas, limitadas a una regla y un ámbito, creadas por un rol autorizado y con aprobación reforzada cuando corresponda. Nunca inferir una excepción a partir de una allowlist general.
5. Entre decisiones coincidentes no anuladas, prevalece la más restrictiva en este orden: `DENY`, `REVIEW`, `CHALLENGE`, `ALLOW`. Una política específica no prevalece automáticamente sobre una política global por ser más específica.
6. La decisión final conserva las reglas, versiones, señales confiables y motivos codificados que determinaron el resultado, sin exponer detalles internos al usuario.

Una excepción solo neutraliza la política concreta que referencia, si dicha política permite excepciones; no borra otras restricciones coincidentes ni evita autorización, MFA, límites, controles antifraude o validaciones del backend. Los `ALLOW` de allowlist no anulan automáticamente `DENY`. Las reglas geográficas y de red se combinan de forma restrictiva: un resultado permitido en una dimensión no cancela una restricción de la otra. Cuando la ubicación sea incierta, se aplican los principios de la sección 3 y no se convierte esa incertidumbre en una prohibición geográfica definitiva.

Si no coincide ninguna política de red, esa dimensión no deniega la solicitud; continúan aplicándose la política geográfica, autenticación, autorización, entitlements y demás controles. Esto no constituye una autorización positiva ni una excepción.

### 4.6 Determinación segura de la IP efectiva y proxies

La dirección de conexión remota no es equivalente a una identidad de persona o dispositivo. Para evaluar reglas IP, el sistema debe obtener una IP efectiva a partir del peer de red y una cadena de proxies explícitamente confiables para ese entorno:

- Nunca confiar directamente en `X-Forwarded-For`, `Forwarded`, `X-Real-IP`, `CF-Connecting-IP` ni headers equivalentes recibidos de un cliente.
- Aceptar headers de forwarding solo cuando la conexión inmediata provenga de un proxy confiable configurado; validar su procedencia y la cadena conforme al orden y límites aprobados, deteniéndose en el primer salto no confiable.
- Configurar Cloudflare como proxy confiable solo en los entornos y rutas donde se valide que el peer es Cloudflare y que sus rangos/origen se mantienen desde una fuente oficial vigente.
- Configurar Nginx/API Gateway con proxies y redes confiables explícitos; no usar rangos amplios o `real_ip` sin validar la topología real.
- Reemplazar o descartar headers aportados por el cliente en el borde. Restringir el acceso directo al origen cuando la topología lo permita y usar TLS/autenticación entre capas.
- Documentar la fuente de la IP efectiva, proxies confiables, headers aceptados, mecanismo de actualización y controles contra bypass para cada entorno.

Si no puede determinarse una IP efectiva confiable, no se debe sustituir por un header arbitrario ni afirmar que el origen fue validado. La respuesta se define por política y criticidad para la operación afectada; no se transforma en un bloqueo global de navegación pública o solicitudes de privacidad. La aceptación debe cubrir tanto falsificación de headers como rutas directas al origen.

### 4.7 Distribución y aplicación en el perímetro y backends

La arquitectura mantiene la responsabilidad: **PLATFORM administra y publica; Cloudflare puede filtrar; Nginx/Gateway puede reforzar; los backends verifican**. El borde no es la autoridad de autorización ni reemplaza la verificación en el dominio propietario.

**Cloudflare**

- Los bloqueos de alcance realmente global pueden aplicarse mediante WAF cuando la regla, su impacto sobre rutas informativas/privacidad y sus excepciones estén aprobados.
- Workers pueden evaluar reglas por tenant, aplicación u operación solo con una identificación de contexto confiable y con políticas distribuidas/versionadas. No generar una regla WAF por tenant ni aplicar en Cloudflare una regla tenant-scoped como global.
- La distribución debe identificar versión, propagación, caché, estado de actualización y revocación. Cambios aún no propagados no se declaran efectivos en todos los puntos.
- El origen conserva verificaciones para las operaciones protegidas; el acceso directo al origen no puede omitirlas.

**Nginx/API Gateway**

- Puede aplicar defensa adicional sobre la IP efectiva ya validada y la cadena de proxies confiables.
- No debe mantener listas de reglas independientes que diverjan de la política publicada por PLATFORM.
- El comportamiento ante indisponibilidad de la distribución debe ser explícito y seguro para cada clase de acceso/operación; proteger operaciones críticas no justifica bloquear por defecto toda navegación o el canal de privacidad.

**Backends**

- Verifican restricciones antes de ejecutar la operación protegida y vuelven a validar autorización, tenant, usuario, estado y permisos propios.
- CORE protege identidad/contexto y operaciones centrales; BUSINESS, las operaciones de establecimientos; PLATFORM, administración global/comercial; PUBLIC, sus experiencias y operaciones públicas.
- Ningún BFF, Worker, regla WAF o gateway puede omitir o sustituir la autorización del backend propietario.
- Las políticas distribuidas se consumen mediante contratos y mecanismos autorizados; no se permite lectura directa de tablas de PLATFORM por otros backends.

La elección concreta entre consulta síncrona y distribución durable y la tolerancia máxima de antigüedad de caché requieren una decisión de arquitectura previa a implementación. Como fallback, las capas consumidoras pueden usar únicamente la última versión autenticada y publicada mientras permanezca dentro de esa vigencia aprobada; deben exponer internamente su versión/antigüedad y alertar ante retraso. Si no existe una versión válida, la operación protegida que dependa de esa política se mantiene pendiente o no disponible sin efectos parciales, sin presentar el fallo técnico como prohibición geográfica. Las páginas informativas y solicitudes de privacidad permanecen accesibles salvo una regla específica aprobada. No se permite continuar una operación dependiente de una política crítica con una configuración inválida ni bloquear globalmente las experiencias no afectadas. Toda publicación que distribuya cambios a otros backends o consumidores se persiste y propaga con el mecanismo aprobado de Outbox/Inbox y consumidores idempotentes, de acuerdo con [ENSAMBRIA_Transactional_Outbox_Inbox.md](ENSAMBRIA_Transactional_Outbox_Inbox.md); cada N/A se justifica por flujo antes de implementarlo. La política no declara configurados Cloudflare, Workers, Nginx ni sus reglas.

### 4.8 Administración en PLATFORM ADMIN y salvaguardas

El alcance funcional de PLATFORM ADMIN debe contemplar, cuando se implemente, consultar, crear, editar, activar/desactivar y programar políticas; configurar expiración; asociar tenant, usuario, aplicación y operación; crear excepciones autorizadas; consultar historial, intentos bloqueados, reglas activas/próximas a expirar; simular una evaluación sin efecto lateral; revisar precedencia y motivo; publicar y verificar propagación; y ejecutar rollback controlado mediante una nueva versión.

Las acciones administrativas se protegen con RBAC, auditoría y autenticación reforzada. Los siguientes permisos son nombres ilustrativos que deben ajustarse a las convenciones aprobadas y a la autoridad de identidad de CORE:

- `network_access.read`
- `network_access.create`
- `network_access.update`
- `network_access.publish`
- `network_access.approve`
- `network_access.rollback`
- `network_access.exceptions.manage`

Separar creación, aprobación y publicación para las reglas globales críticas. Aplicar MFA/reautenticación y aprobación de otro administrador cuando el impacto pueda bloquear accesos administrativos o redes esenciales. No permitir que el autor apruebe su propia excepción reforzada. La simulación no publica ni altera reglas, no confirma si una cuenta existe y no debe dejar la IP evaluada en logs/métricas. El rollback conserva auditoría y genera una versión nueva.

Antes de publicar una regla que pueda afectar al operador actual, toda la administración, el acceso a redes corporativas críticas, monitorización, health checks, webhooks autenticados, procesos de privacidad/exportación o recuperación/soporte, el flujo debe ejecutar validaciones de impacto y exigir confirmación/aprobación reforzada según riesgo. Esas salvaguardas no crean bypasses universales: cualquier excepción tiene dueño, alcance, vigencia, motivo y auditoría. Health checks y webhooks se autentican y autorizan por sus mecanismos propios; no se confía en una allowlist IP como autenticación.

### 4.9 IP compartida, privacidad y telemetría

Una IP pública puede ser compartida por hogares, organizaciones, CGNAT, redes móviles, VPN corporativas y proxies. No identificar automáticamente a una persona por IP, ni bloquear una cuenta/tenant completo porque una IP coincida con una regla. Las señales VPN/proxy/Tor se tratan como riesgo contextual no concluyente. Se permiten políticas más específicas cuando el caso de uso y la autorización lo justifican; se registran falsos positivos de forma minimizada y existe revisión administrativa.

Las direcciones IP y los datos de ubicación se tratan conforme a [ENSAMBRIA_Privacy_Data_Lifecycle_Management.md](ENSAMBRIA_Privacy_Data_Lifecycle_Management.md) y [ENSAMBRIA_Privacy_Legal_Compliance.md](ENSAMBRIA_Privacy_Legal_Compliance.md): finalidad, necesidad, clasificación, acceso, minimización y retención deben definirse antes de introducir su tratamiento. No se conserva IP completa por conveniencia ni se usan datos personales como etiquetas métricas.

La observabilidad registra decisiones y contadores de evaluaciones permitidas/denegadas, `REVIEW`/`CHALLENGE`, expiraciones, errores de distribución, versiones observadas, latencia, falsos positivos reportados, cambios administrativos e intentos bloqueados. Las métricas usan etiquetas acotadas (por ejemplo, tipo de regla, acción, aplicación y resultado); nunca IP, `user_id`, `tenant_id` ni otro valor de cardinalidad ilimitada. Logs y auditoría minimizan o transforman IP según política aprobada; no registran headers completos ni cadenas de forwarding. Los requisitos detallados de instrumentación y redacción se rigen por la guía de observabilidad.

### 4.10 Modelo conceptual, distribución y cambios de política

Sin imponer nombres físicos, el diseño puede requerir conceptos equivalentes a `NetworkAccessPolicy`, dirección/red IP, ámbito, acción, excepción, versión/publicación, evaluación y evento de auditoría. Los identificadores de entidades y eventos siguen la política canónica UUID v7 de ENSAMBRIA. Cada concepto necesita owner, ciclo de vida, controles de acceso, versionado y retención explícitos. PLATFORM administra la política y su historial; los backends consumidores aplican únicamente versiones autenticadas/publicadas y no escriben datos en el almacenamiento de otro backend.

Una publicación distribuida debe identificar versión, ámbito, estado efectivo y orden. Los consumidores validan integridad, compatibilidad y secuencia, deduplican por identificador estable, registran la versión aplicada y recuperan cambios perdidos desde la fuente durable autorizada. Revocación y rollback deben tener semántica definida y pruebas de propagación; una señal efímera no sustituye el estado durable. La evaluación no debe afirmar convergencia antes de comprobar la propagación a los consumidores requeridos.

Los cambios persistidos de política que sean visibles para operadores se sincronizan con el patrón realtime del backend propietario: estado y Outbox durables, fan-out efímero entre réplicas por NATS Core cuando corresponda, WebSocket autenticado por el BFF autorizado y recuperación desde REST/cursor o snapshot. La pérdida de una señal no pierde la política publicada ni sustituye la recuperación desde la fuente durable. La consola actualiza sus consultas de forma centralizada y verifica permisos vigentes al recuperar el estado.

## 5. Experiencia, errores y revisión

Los mensajes deben explicar qué capacidad no está disponible, sin revelar reglas antifraude, detalles de infraestructura ni conclusiones legales no verificadas. No etiquetar a una persona como infractora solo por IP, VPN o discrepancia de geolocalización. Cuando el acceso se restringa por datos imprecisos, indicar cómo confirmar/corregir la ubicación o contactar soporte.

No usar redirecciones globales que hagan inaccesibles privacidad, términos, información institucional, contacto o solicitud de lanzamiento. Los fallos del servicio de geolocalización no deben bloquear rutas informativas ni derechos. Las operaciones comerciales restringidas deben seguir el comportamiento seguro definido para esa operación; no convertir un fallo técnico en una afirmación de residencia o prohibición legal.

Las respuestas de una regla de red también deben ser neutrales y accionables, sin revelar si una IP, cuenta o tenant está en una lista ni permitir inferir la existencia de reglas no visibles. Registrar la causa interna con un código estable y la versión de política, sin incluir reglas secretas, cabeceras completas o datos sensibles en el envelope público.

## 6. Pruebas y aceptación

Cada capacidad que aplique incluye pruebas de política y de integración en su fase de implementación. La matriz cubre como mínimo:

- Navegación e información pública desde países habilitados y no habilitados.
- Solicitud de información de lanzamiento desde ambas condiciones.
- Acceso y envío de solicitudes de privacidad sin depender de país/IP.
- Registro, contratación y operaciones comerciales con los resultados esperados por política.
- Distinción entre país no habilitado y una capacidad específica no habilitada.
- Restricción por ciudad con señal IP corroborada y con verificación adicional aprobada.
- IP de VPN, proxy, red móvil, NAT, ubicación imprecisa, señal ausente, proveedor caído y señales contradictorias.
- Cabeceras de geolocalización falsificadas desde el cliente o acceso que evita el proxy confiable.
- No crear efectos comerciales parciales cuando una operación se deniega o queda pendiente de verificación.
- Mensajes seguros, accesibles y con ruta de revisión/corrección.
- Minimización, acceso, auditoría y retención de datos geográficos.
- Parsing y normalización de IPv4/IPv6, CIDR válido/inválido, redes superpuestas y entradas malformadas; comprobar que no haya comparaciones de cadenas ni efectos antes de validar.
- Coincidencia de políticas globales, tenant, usuario, aplicación y operación, incluidos selectores combinados; demostrar que una política de tenant no afecta a otros tenants.
- Precedencia determinista entre ALLOW, DENY, REVIEW y CHALLENGE, excepciones aprobadas y no aprobadas, reglas temporales/permanentes y denegaciones globales críticas no anulables.
- Expiración automática, reglas programadas, cambios concurrentes/versiones obsoletas, historial, revocación y rollback como nueva versión.
- Autorización RBAC, MFA/aprobación reforzada, separación de funciones y auditoría de creación, edición, publicación, excepción, expiración y eliminación lógica.
- Falsificación de `X-Forwarded-For`, `Forwarded`, `X-Real-IP`, `CF-Connecting-IP` y headers equivalentes; peers no confiables, cadenas de proxies malformadas, bypass al origen y configuración efectiva de proxies confiables.
- Propagación, consistencia y revocación de políticas; caída de distribución, consumidor atrasado, caché obsoleta/ausente, recuperación/replay y ausencia de autorización basada en una versión inválida.
- Simulación administrativa sin efectos laterales, protección ante auto-bloqueo, aislamiento de PLATFORM ADMIN, redes críticas, monitorización, health checks autenticados, webhooks, privacidad/exportación y recuperación.
- IP compartida, CGNAT, redes móviles, VPN/proxy/Tor como señales no concluyentes, falsos positivos y revisión sin bloqueo automático de tenant/cuenta.
- Falla de identificación de IP efectiva y comportamiento acotado a la operación protegida, manteniendo accesibles rutas informativas y solicitudes de privacidad.
- ASN solo con fuente confiable, procedencia/versionado, datos antiguos o ausentes y señal contradictoria.
- Métricas con cardinalidad acotada y comprobación de ausencia de IP, `user_id`, `tenant_id`, credenciales o headers crudos en etiquetas/logs/trazas.

Las pruebas legales verifican el comportamiento técnico respecto de políticas aprobadas; no sustituyen la validación de Legal. No usar IPs o localidades de usuarios reales en fixtures de prueba.

## 7. Aplicación por fases

### Fase 0 — Diseño documental

Inventariar las superficies que pueden usar restricciones geográficas y de red y separar navegación, información de lanzamiento, solicitudes de privacidad, registro y operaciones comerciales. Documentar mercados iniciales, capacidades y owners; registrar fuentes oficiales y estado de validación legal; definir señales, nivel de evidencia, casos de incertidumbre, alternativas para el usuario, datos tratados y pruebas. Definir el ownership de PLATFORM para administración/publicación y el de cada backend para enforcement; documentar ámbitos, acciones, precedencia, RBAC, confianza en proxies, fuentes ASN, arquitectura Cloudflare/Nginx/Gateway, distribución/versionado/expiración, recuperación, fallback, privacidad y observabilidad. Registrar los contratos y decisiones/ADRs pendientes, sin afirmar integraciones configuradas. No implementar geobloqueo, middleware, runtime, tablas activas ni reglas publicadas en esta fase.

### Fase 1 y fases de implementación posteriores

Antes de introducir cualquier capacidad que aplique restricciones geográficas o de red, documentar la matriz de integración de la fase/subfase y sus componentes aplicables, implementar la política aprobada en la frontera propietaria, con mínima recopilación, autorización, errores seguros, auditoría y pruebas del recorrido real. PLATFORM ADMIN debe completar en la misma entrega la superficie administrativa cuando el cambio introduzca gestión de esas reglas; los backends afectados consumen/aplican el contrato de políticas en el mismo corte o el alcance se mantiene bloqueado. Si la distribución durable de publicaciones es aplicable, integrar mutación de política y Outbox en la transacción propietaria, consumidores con Inbox/idempotencia y recuperación según la guía Outbox/Inbox; no generar eventos artificiales cuando no haya distribución necesaria. Completar en la misma fase la experiencia de frontend/BFF/backend afectada y la propagación/recuperación requerida. No restringir otras clases de acceso ni marcar un mercado como habilitado sin evidencia y aprobación requeridas.

## 8. Regla final

ENSAMBRIA controla disponibilidad por actividad y mercado, no por una aproximación de ubicación convertida en bloqueo indiscriminado. La navegación pública, las consultas de lanzamiento y el ejercicio de derechos de privacidad permanecen diferenciados de la contratación y las operaciones comerciales. Una restricción estricta por ciudad requiere evidencia adicional a la IP, una política aprobada y una vía para resolver errores de ubicación.
