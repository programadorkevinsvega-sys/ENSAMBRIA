# ENSAMBRIA — README & Documentation Quality and Security Standard

**Tipo:** Estándar obligatorio de documentación<br>
**Ámbito:** Todo el ecosistema ENSAMBRIA<br>
**Aplicación:** Monorepo, backends, frontends, módulos, workers, infraestructura y herramientas<br>
**Responsables:** Todos los desarrolladores y agentes de IA que modifiquen el repositorio<br>
**Estado:** Estándar arquitectónico y de ingeniería

---

# 1. Objetivo

Establecer reglas obligatorias para crear, mantener, actualizar, revisar y publicar archivos `README.md` y documentación técnica de ENSAMBRIA.

La documentación debe ser:

- Profesional.
- Precisa.
- Coherente con la arquitectura.
- Verificable.
- Reproducible.
- Segura.
- Actualizada.
- Fácil de comprender.
- Consistente entre todos los componentes.
- Adecuada para colaboradores, desarrolladores y usuarios externos cuando corresponda.

El objetivo es evitar documentación inventada, desactualizada, contradictoria, incompleta o que exponga información sensible.

**Regla fundamental: la documentación debe describir la realidad del software, nunca una implementación imaginada o futura como si ya estuviera terminada.**

# 2. Alcance obligatorio

Este estándar se aplica a:

- README raíz del monorepo.
- CORE BACKEND.
- BUSINESS BACKEND.
- PLATFORM BACKEND.
- PUBLIC BACKEND.
- BFF de cada backend.
- Frontends web.
- Aplicación móvil.
- Módulos de dominio.
- Módulos de infraestructura.
- Workers y procesos asíncronos.
- Bibliotecas y crates compartidos.
- Contratos API y Protobuf.
- Scripts y herramientas.
- Infraestructura y despliegues.
- Paquetes y aplicaciones independientes.

Cada componente debe tener documentación propia cuando aporte información relevante para comprenderlo, instalarlo, ejecutarlo, probarlo o integrarlo.

No crear README vacíos, redundantes o artificialmente extensos.

# 3. Principios obligatorios

## 3.1. Exactitud documental

Toda afirmación debe tener respaldo verificable en alguno de estos elementos:

- Código implementado.
- Configuración existente.
- Contratos vigentes.
- Pruebas ejecutables.
- Scripts existentes.
- Infraestructura realmente configurada.
- ADR aprobados.
- Decisiones arquitectónicas oficiales.

Está prohibido:

- Inventar funcionalidades.
- Inventar endpoints.
- Inventar comandos.
- Inventar dependencias.
- Inventar versiones.
- Inventar configuraciones.
- Declarar integraciones inexistentes.
- Presentar ejemplos como funcionalidades reales.
- Declarar pruebas exitosas sin ejecución verificable.
- Afirmar que un servicio está desplegado cuando solamente existe código.
- Afirmar que una característica está completa cuando tiene implementaciones parciales.

Si una información no puede verificarse, debe omitirse o identificarse explícitamente como pendiente de confirmación.

## 3.2. Coherencia arquitectónica

Todos los README deben respetar la arquitectura oficial de ENSAMBRIA.

Arquitectura de cuatro backends:

1. CORE BACKEND.
2. BUSINESS BACKEND.
3. PLATFORM BACKEND.
4. PUBLIC BACKEND.

Arquitectura de seis proyectos de interfaz:

1. ENSAMBRIA Business (`ensambria-business`, Flutter + Dart).
2. ENSAMBRIA Tenant Portal (`tenant-portal`, React + TypeScript + Next.js).
3. ENSAMBRIA App (`ensambria-app`, Flutter + Dart).
4. ENSAMBRIA Marketplace (`marketplace`, React + TypeScript + Next.js).
5. ENSAMBRIA Platform Admin (`platform-admin`, React + TypeScript + Next.js).
6. ENSAMBRIA Corporate (`corporate`, React + TypeScript + Next.js).

Los README no pueden modificar informalmente las responsabilidades, ownership, límites de dominio o contratos entre estos componentes.

Las modificaciones arquitectónicas deben seguir el proceso de decisión correspondiente y actualizar la documentación oficial.

## 3.3. Fuente única de verdad

Cada decisión debe tener una ubicación documental oficial.

Ejemplos:

- Arquitectura: ADR y documentación arquitectónica.
- Contratos: especificaciones API y Protobuf.
- Seguridad: estándares de seguridad.
- Despliegues: documentación de infraestructura y runbooks.
- Configuración: documentación de configuración del componente.
- Desarrollo: README y guías de contribución.

El README debe resumir y enlazar a la documentación correspondiente, no duplicar documentos técnicos completos.

Si existen contradicciones, el agente debe identificarlas y solicitar o aplicar la resolución conforme a las decisiones oficiales. No debe elegir arbitrariamente una versión.

# 4. Seguridad y protección de información

## 4.1. Información prohibida en README públicos

Nunca publicar:

- Contraseñas.
- API keys.
- Tokens de acceso.
- Refresh tokens.
- Secretos JWT.
- Claves privadas.
- Credenciales SSH.
- Credenciales TLS.
- Credenciales de Vault.
- Secretos de proveedores externos.
- DSN o cadenas reales de conexión.
- Credenciales PostgreSQL.
- Credenciales Redis.
- Credenciales NATS.
- Credenciales Cloudflare.
- Credenciales AWS.
- Variables reales de producción.
- Cookies de sesión.
- URLs firmadas temporales.
- Backups.
- Dumps de bases de datos.
- Datos personales reales.
- Datos financieros de clientes.
- Información privada de tenants.
- Información de autenticación administrativa.
- Información que permita acceder a sistemas internos.

Tampoco publicar:

- IP privadas.
- IP públicas de servidores de infraestructura privada.
- Hostnames internos.
- Nombres reales de máquinas de producción.
- Topologías de red privadas.
- Rutas internas de almacenamiento sensible.
- Reglas operativas que faciliten comprometer sistemas.
- Configuraciones de firewall que revelen detalles innecesarios.
- Información de incidentes de seguridad no autorizada para publicación.

La información técnica necesaria debe documentarse de forma abstracta y segura.

## 4.2. Protección de infraestructura

Se permite documentar tecnologías y arquitecturas generales.

Ejemplos permitidos:

- PostgreSQL.
- Redis.
- NATS JetStream.
- Cloudflare.
- HashiCorp Vault.
- Tailscale.
- Nginx.
- Object Storage.

No se deben publicar detalles reales de infraestructura que faciliten el acceso o reconocimiento de sistemas privados.

Los ejemplos deben utilizar valores ficticios, dominios reservados para documentación o placeholders.

## 4.3. Variables de entorno

Está permitido documentar variables necesarias para ejecutar un componente.

Ejemplo:

```dotenv
APP_ENV=development
DATABASE_URL=postgres://USER:PASSWORD@localhost:5432/ENSAMBRIA
REDIS_URL=redis://localhost:6379
JWT_SECRET=REPLACE_WITH_A_LOCAL_SECRET
VAULT_ADDR=https://vault.example.invalid
```

Estas configuraciones son ejemplos ilustrativos. El agente debe sustituirlas por los nombres y formatos reales que utilice el proyecto.

Reglas:

- Nunca copiar `.env` reales.
- Nunca copiar secretos de producción.
- Nunca incluir credenciales reales.
- Documentar si una variable es obligatoria u opcional.
- Documentar su finalidad.
- Documentar su formato esperado.
- Documentar valores predeterminados seguros cuando existan.
- No inventar variables que el código no consuma.
- No exponer secretos mediante ejemplos de frontend.
- Diferenciar configuración pública de configuración privada.

Cuando corresponda, proporcionar un `.env.example` sin secretos y mantenerlo sincronizado con la implementación.

## 4.4. Información histórica expuesta

Si un README o archivo documental contiene un secreto:

1. Eliminar la exposición del contenido actual.
2. Identificar el alcance de la exposición.
3. Revocar o rotar la credencial comprometida.
4. Revisar el historial Git y los artefactos publicados.
5. Aplicar el procedimiento de respuesta a incidentes.
6. Verificar que la corrección no dependa únicamente de eliminar el texto visible.

Eliminar una credencial del último commit no significa que haya dejado de estar comprometida.

# 5. Estructura del README raíz

El README raíz debe ser el punto de entrada oficial de ENSAMBRIA.

Debe contener, cuando corresponda:

1. Nombre y descripción del proyecto.
2. Propósito de ENSAMBRIA.
3. Capacidades generales.
4. Arquitectura de alto nivel.
5. Componentes principales.
6. Tecnologías verificadas.
7. Requisitos previos.
8. Instalación y preparación del entorno.
9. Instrucciones para ejecutar componentes.
10. Comandos de calidad y pruebas.
11. Estructura del monorepo.
12. Enlaces a documentación adicional.
13. Seguridad y reporte responsable de vulnerabilidades.
14. Contribución y flujo de trabajo.
15. Licencia, únicamente si está definida.

El README raíz no debe convertirse en una copia de todos los README de los componentes.

Debe proporcionar una visión general y enlaces claros a la documentación especializada.

## 5.1. Encabezado o Hero Section

La parte superior del README debe identificar ENSAMBRIA de inmediato y facilitar la navegación inicial. Incluir, cuando exista y esté aprobado:

1. Logo oficial de ENSAMBRIA, usando únicamente un archivo oficial y autorizado del repositorio.
2. Nombre del proyecto: **ENSAMBRIA**.
3. Descripción institucional breve y consistente con el estado real del producto.
4. Eslogan o propuesta de valor aprobados; no inventar promesas comerciales.
5. Badges informativos, solo con fuentes reales y verificables.
6. Diagrama general de arquitectura o imagen representativa autorizada, si ayuda a comprender el producto.
7. Estado real del proyecto, distinguiendo arquitectura/documentación, scaffold, implementación parcial y capacidades verificadas.
8. Enlaces de navegación a requisitos, arquitectura, reglas, roadmap, contribución y demás documentación principal existente.

El diagrama debe reflejar la topología aprobada —CORE, BUSINESS, PLATFORM, PUBLIC y sus superficies frontend/BFF— sin sugerir que componentes planificados ya están implementados. Preferir un diagrama mantenido junto al proyecto o una representación Markdown/Mermaid que pueda revisarse con el README. No copiar ni enlazar como recurso visual del proyecto imágenes de referencia, capturas de terceros o material sin licencia/autorización verificada.

### Badges

Los badges son opcionales y solo se publican si el dato tiene una fuente accesible y vigente:

- Lenguajes, frameworks y base de datos: respaldados por manifests o documentación vigente; no se usan para insinuar que todas las capacidades ya están operativas.
- Arquitectura: texto descriptivo, no una certificación ni una garantía de escalabilidad.
- Estado de desarrollo: consistente con el estado verificable del repositorio.
- CI/CD: enlazado a un workflow configurado y a su estado real; no agregar badges de workflows inexistentes o nunca ejecutados.
- Cobertura de pruebas: solo cuando exista medición automatizada reproducible, con umbral/metodología conocidos y enlace a evidencia; nunca usarla como sustituto de aceptación funcional.
- Licencia: solo si existe un archivo/licencia aprobado para el repositorio.

No usar badges decorativos que aparenten versiones, cobertura, seguridad, licencia o checks aprobados. Si la fuente deja de estar disponible o el badge se vuelve inexacto, retirarlo o corregirlo.

## 5.2. Cuerpo del README raíz

El cuerpo debe presentar la información en este orden, en forma concisa y con enlaces a sus fuentes normativas o guías de componente:

1. ¿Qué es ENSAMBRIA?
2. Visión y objetivos.
3. Problema que resuelve.
4. Capacidades principales.
5. Verticales de negocio.
6. Arquitectura general.
7. Backends y sus responsabilidades.
8. Frontends y sus responsabilidades.
9. Tecnologías.
10. Estructura del monorepo.
11. Requisitos previos.
12. Instalación y configuración.
13. Ejecución local.
14. Pruebas y calidad.
15. Seguridad.
16. Infraestructura y despliegue.
17. Observabilidad.
18. Documentación técnica.
19. Roadmap y estado de desarrollo.

Los puntos aplicables deben resumirse, no convertirse en especificaciones extensas duplicadas. Los procedimientos detallados pertenecen al README del componente, a las reglas normativas, al roadmap, a los ADR o a las guías de operación. Si un tema no aplica al estado/scope actual o no puede verificarse, no se inventa una sección de apariencia completa: se omite o se indica brevemente que está pendiente, y se enlaza la fuente correspondiente cuando exista.

Los requisitos previos, instalación, configuración, ejecución y pruebas solo se documentan como instrucciones utilizables cuando los manifests, herramientas, scripts y dependencias existen y se han verificado. No presentar comandos objetivo, despliegues futuros ni propuestas de arquitectura como instrucciones disponibles. El estado de implementación y roadmap debe enlazar a la evidencia o documento que lo mantiene y separar explícitamente lo implementado de lo planificado o bloqueado.

## 5.3. Cierre del README raíz

El cierre debe ofrecer información práctica para colaborar y conocer las condiciones de uso del proyecto. Incluir, si está definido y es verificable:

- Cómo contribuir y dónde consultar los estándares de desarrollo.
- Flujo de GitHub: ramas, Pull Requests, revisiones y checks requeridos que estén realmente configurados.
- Política de seguridad y canal/instrucciones autorizadas para reportar vulnerabilidades, enlazando a la política existente.
- Licencia, enlazando a la licencia del repositorio únicamente si está definida.
- Organización responsable y contacto oficial, solo si han sido designados y autorizados para publicación.
- Agradecimientos pertinentes y autorizados.

No inventar una organización, correo, canal de vulnerabilidades, colaboradores, licencia, aprobación, check de CI ni flujo operativo. Cuando la información todavía no esté aprobada o configurada, no mostrar placeholders como si fueran canales funcionales; registrar el pendiente en su documento/proceso dueño y omitir la afirmación pública hasta resolverlo.

## 5.4. Elementos visuales adicionales

El README puede usar elementos visuales para mejorar comprensión, sin sacrificar seguridad, exactitud ni derechos de uso:

- Identidad visual: logo oficial, nombre y descripción institucional.
- Diagrama: relación entre los cuatro backends, frontends y BFF autorizados, respetando ownership y estado real.
- Vista del producto: capturas reales de interfaces ENSAMBRIA, solo cuando existan, estén aprobadas para publicación y no muestren datos personales, secretos ni información de tenants.

Las imágenes de productos ajenos, enlaces de inspiración o ejemplos externos no son assets de ENSAMBRIA y no deben incrustarse ni presentarse como producto propio. Antes de publicar un recurso visual, verificar su procedencia, licencia, autorización, accesibilidad (incluido texto alternativo) y que no revele información restringida. No crear capturas o maquetas para insinuar funcionalidades que todavía no existen.

# 6. Estructura de los README de componentes

Cada README debe adaptar su contenido al propósito del componente.

Cuando corresponda, utilizar esta estructura:

## 6.1. Identificación

- Nombre del componente.
- Tipo de componente.
- Responsabilidad principal.
- Ubicación dentro del monorepo.

## 6.2. Descripción

Explicar:

- Qué es.
- Qué problema resuelve.
- Qué responsabilidades tiene.
- Qué responsabilidades no tiene.

## 6.3. Capacidades

Enumerar únicamente funcionalidades verificadas.

Diferenciar claramente implementaciones terminadas, parciales y planificadas.

## 6.4. Arquitectura

Documentar:

- Límites de responsabilidad.
- Ownership.
- Dependencias autorizadas.
- Contratos.
- Integraciones.
- Flujos relevantes.

No duplicar ADR completos.

## 6.5. Tecnologías

Enumerar tecnologías realmente utilizadas.

Indicar versiones únicamente cuando estén verificadas y sean relevantes.

## 6.6. Requisitos previos

Documentar herramientas, servicios y dependencias necesarios.

## 6.7. Instalación

Proporcionar instrucciones reproducibles y verificadas.

## 6.8. Configuración

Documentar variables de entorno y configuración segura.

## 6.9. Ejecución

Proporcionar comandos reales para iniciar el componente.

## 6.10. Pruebas y calidad

Documentar comandos reales de:

- Formato.
- Lint.
- Compilación.
- Tests unitarios.
- Tests de integración.
- Tests de contratos.
- Tests E2E.
- Validaciones de seguridad.

No presentar comandos que no existan o no correspondan al componente.

## 6.11. Estructura del código

Explicar los directorios existentes y sus responsabilidades.

No documentar estructuras futuras como si ya existieran.

## 6.12. Documentación relacionada

Enlazar documentos técnicos relevantes.

## 6.13. Seguridad

Indicar los canales y procedimientos autorizados para reportar vulnerabilidades.

No incluir información explotable ni credenciales.

# 7. Estados de implementación

Cuando se documente el estado de una funcionalidad, utilizar estados inequívocos:

- IMPLEMENTED: implementada y verificada.
- PARTIALLY_IMPLEMENTED: existe una implementación incompleta.
- IN_PROGRESS: se encuentra en desarrollo.
- PLANNED: planificada, pero no implementada.
- NOT_AVAILABLE: no está disponible.
- DEPRECATED: obsoleta y en proceso de retirada o retirada.
- BLOCKED: bloqueada por una dependencia o decisión pendiente.

Los estados deben tener evidencia verificable.

Reglas:

- No declarar IMPLEMENTED por tener solamente interfaces, estructuras o contratos.
- No declarar IMPLEMENTED por tener solamente tests simulados.
- No declarar producción verificada por haber ejecutado tests locales.
- No utilizar porcentajes de avance sin una metodología objetiva.
- No utilizar badges de CI exitoso sin una fuente real.
- No confundir compilación con funcionamiento productivo.
- No ocultar limitaciones conocidas.

Cuando un componente tenga funcionalidades de diferentes estados, documentarlas individualmente.

# 8. Coherencia entre componentes ENSAMBRIA

Todos los README deben respetar el siguiente ownership.

## 8.1. CORE BACKEND

Responsabilidades:

- Identidad y autenticación.
- Tenancy y contexto tenant.
- Organizaciones y sucursales centrales.
- Usuarios y pertenencia.
- Entitlements técnicos.
- Reservas y disponibilidad común.
- Pagos y operaciones financieras centrales.
- Idempotencia financiera.
- Comisiones transaccionales ENSAMBRIA.
- Ledger.
- Liquidaciones.
- Conciliación.

No presentar CORE como propietario de todos los módulos de negocio.

## 8.2. BUSINESS BACKEND

Responsabilidades:

- Operaciones de establecimientos.
- Empleados y profesionales.
- Horarios.
- Servicios.
- Clientes.
- CRM.
- Membresías comerciales y fidelización.
- Inventario y compras.
- Commerce.
- Pedidos.
- Marketing.
- Comisiones de empleados.
- Franquicias y cadenas.
- Regalías de franquicias.
- Analítica operativa.
- Verticales especializados.

Los verticales barber, hair, beauty y spa deben utilizar las capacidades comunes sin duplicar las responsabilidades de CORE.

## 8.3. PLATFORM BACKEND

Responsabilidades:

- Administración global ENSAMBRIA.
- Gestión comercial de tenants.
- Planes.
- Suscripciones SaaS.
- Facturación SaaS.
- Políticas comerciales.
- Configuración comercial de capacidades.
- Políticas de monetización del Marketplace.
- Soporte.
- Supervisión.
- Auditoría administrativa.
- Administración global de restricciones de acceso.

No presentar PLATFORM como propietario de las franquicias de los establecimientos.

## 8.4. PUBLIC BACKEND

Responsabilidades:

- Directorio.
- Marketplace.
- Descubrimiento.
- Perfiles públicos.
- Páginas públicas.
- Búsquedas.
- Filtros.
- Reseñas.
- Indexación geográfica.
- Promociones autorizadas.
- Atribución de origen de reservas y compras.
- Proyecciones públicas.

No presentar PUBLIC como propietario de las operaciones privadas de los establecimientos.

## 8.5. Frontends

Los README de los seis proyectos de interfaz deben describir sus responsabilidades reales:

- ENSAMBRIA Business: operación empresarial multiplataforma para usuarios autorizados del tenant.
- ENSAMBRIA Tenant Portal: administración web de los mismos tenants/capacidades que Business.
- ENSAMBRIA App: experiencia móvil de clientes, separada de la administración empresarial.
- ENSAMBRIA Marketplace: directorio y descubrimiento público autorizado.
- ENSAMBRIA Platform Admin: administración global privada de ENSAMBRIA.
- ENSAMBRIA Corporate: contenido institucional público; no administra tenants.

No afirmar que un frontend accede directamente a bases de datos o servicios internos cuando la arquitectura no lo permite.

## 8.6. BFF

Los BFF adaptan y componen contratos.

No deben presentarse como propietarios de reglas de negocio ni como mecanismos para evitar las políticas de autorización.

# 9. Reglas de comandos y ejemplos

Todo comando documentado debe cumplir:

- Existir en el repositorio.
- Ser compatible con el entorno documentado.
- Tener una finalidad clara.
- Haber sido verificado cuando sea posible.
- No depender de archivos inexistentes.
- No requerir secretos publicados.
- No ejecutar acciones destructivas inesperadas.

Los comandos destructivos deben incluir advertencias explícitas y requisitos previos.

Los ejemplos de código deben:

- Ser válidos para el lenguaje correspondiente.
- Respetar las convenciones del proyecto.
- Evitar APIs inexistentes.
- Evitar tipos inseguros.
- No utilizar credenciales reales.
- No contener pseudocódigo presentado como implementación funcional.

# 10. Reglas de enlaces y referencias

Todos los README deben:

- Utilizar rutas relativas cuando corresponda.
- Enlazar a documentos existentes.
- Evitar referencias rotas.
- Evitar enlaces temporales.
- Evitar enlaces privados inaccesibles para los destinatarios.
- Evitar referencias a ramas o archivos que no existan.
- Mantener una navegación documental clara.

Cuando un documento cambie de ubicación, actualizar sus referencias.

No duplicar documentos para solucionar enlaces incorrectos.

# 11. Estándar visual y profesional

Todos los README deben mantener una presentación profesional.

Reglas:

- Títulos descriptivos.
- Jerarquía de encabezados coherente.
- Párrafos concisos.
- Listas cuando faciliten la lectura.
- Tablas únicamente cuando aporten claridad.
- Bloques de código con lenguaje especificado.
- Badges solamente cuando su información sea verificable.
- Diagramas cuando expliquen realmente la arquitectura.
- Enlaces con nombres descriptivos.
- Ortografía y terminología consistentes.
- No utilizar emojis excesivamente.
- No utilizar mensajes publicitarios engañosos.
- No utilizar afirmaciones como "100% seguro", "sin vulnerabilidades" o "totalmente escalable" sin evidencia objetiva.

Los README deben poder entenderse sin conocer previamente todo el repositorio.

# 12. Separación entre documentación pública, interna y restringida

ENSAMBRIA debe clasificar la documentación según su sensibilidad.

## 12.1. Documentación pública

Puede incluir:

- Descripción del producto.
- Capacidades públicas.
- Arquitectura de alto nivel.
- Requisitos generales.
- Instalación autorizada.
- Ejemplos seguros.
- Tecnologías generales.
- Guías de contribución.
- Licencia.

## 12.2. Documentación técnica interna

Puede incluir, con acceso controlado:

- ADR.
- Arquitectura detallada.
- Contratos internos.
- Flujos de autorización.
- Modelos de dominio.
- Estrategias de integración.
- Procedimientos técnicos.

No debe contener secretos ni credenciales reales.

## 12.3. Documentación operativa restringida

Puede incluir procedimientos operativos sensibles, tales como:

- Recuperación de infraestructura.
- Gestión de incidentes.
- Procedimientos administrativos.
- Accesos operativos.
- Configuraciones internas.
- Topologías privadas.

Debe almacenarse en ubicaciones con permisos adecuados.

Los secretos y credenciales nunca deben almacenarse directamente en documentos, independientemente de su clasificación.

El README público debe enlazar únicamente a recursos que sus destinatarios puedan consultar de forma autorizada.

# 13. Sincronización documental

La documentación debe mantenerse sincronizada con la implementación.

Actualizar el README correspondiente cuando se modifiquen:

- Responsabilidades de un componente.
- Funcionalidades disponibles.
- Contratos.
- Endpoints públicos.
- Variables de entorno.
- Dependencias.
- Comandos.
- Requisitos.
- Instalación.
- Ejecución.
- Pruebas.
- Arquitectura.
- Integraciones.
- Procedimientos relevantes.

No modificar README sin motivo técnico.

No realizar actualizaciones cosméticas masivas que oculten cambios funcionales o dificulten la revisión.

# 14. Validaciones automáticas en GitHub Actions

ENSAMBRIA debe implementar validaciones automáticas para la documentación.

El gobierno general del repositorio, los PR y la seguridad de GitHub Actions se rige por [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](ENSAMBRIA_github_repository_governance_and_engineering_rules.md). Este estándar define los controles particulares de documentación y README que deben ejecutarse dentro de ese flujo.

La selección de herramientas debe respetar las herramientas y convenciones existentes en el repositorio.

## 14.1. Markdown

Validar:

- Sintaxis Markdown.
- Jerarquía de encabezados.
- Enlaces internos.
- Referencias a archivos.
- Formato.
- Errores de estructura.
- Convenciones documentales.

## 14.2. Seguridad documental

Ejecutar detección de:

- Secretos.
- Tokens.
- API keys.
- Credenciales.
- Claves privadas.
- URLs sensibles.
- Datos personales.
- Direcciones IP privadas.
- Hostnames internos.
- Posibles configuraciones de producción expuestas.

Las detecciones deben contemplar falsos positivos.

No incluir secretos reales en excepciones, allowlists o configuraciones de escáneres.

Toda excepción debe estar justificada, acotada y revisada.

## 14.3. Integridad documental

Verificar:

- Rutas existentes.
- Enlaces relativos.
- Referencias a scripts.
- Referencias a archivos.
- Nombres de componentes.
- Ubicaciones documentadas.
- Ejemplos de configuración cuando puedan validarse automáticamente.

## 14.4. Coherencia con el código

Cuando sea técnicamente viable, validar que:

- Las variables documentadas existan.
- Los comandos correspondan a scripts reales.
- Los endpoints documentados estén definidos.
- Los contratos estén actualizados.
- Las versiones coincidan con los manifiestos.
- Los ejemplos de configuración respeten los esquemas reales.

No implementar comprobaciones que dependan de suposiciones o heurísticas inseguras.

## 14.5. Ejecución selectiva de CI

Una modificación exclusivamente documental no debe ejecutar innecesariamente toda la batería de compilación y pruebas de Rust o frontend.

Sin embargo:

- Debe ejecutar las validaciones documentales y de seguridad correspondientes.
- Un cambio de código que afecte documentación y comportamiento debe ejecutar las pruebas aplicables al código.
- Cambios en workflows y herramientas de CI deben someterse a sus validaciones correspondientes.
- La selección de CI debe ser verificable y no permitir omitir controles por clasificar incorrectamente un cambio.

No modificar workflows o scripts existentes sin autorización y alcance explícito.

# 15. Revisión obligatoria en Pull Requests

Aplica también [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](ENSAMBRIA_github_repository_governance_and_engineering_rules.md) para el proceso general de revisión, aceptación y trazabilidad del PR; este apartado añade los criterios específicos de documentación.

Todo PR que modifique funcionalidades, contratos, arquitectura, configuración o procedimientos relevantes debe revisar la documentación afectada.

El PR debe permitir identificar:

- Qué documentación cambió.
- Por qué cambió.
- Qué implementación respalda el cambio.
- Qué información fue verificada.
- Qué se encuentra todavía pendiente.

No aceptar documentación que contradiga el código o decisiones arquitectónicas aprobadas.

No considerar terminado un cambio relevante cuando su documentación obligatoria está desactualizada.

# 16. Reglas para agentes de IA

Todos los agentes que trabajen en ENSAMBRIA deben cumplir estas instrucciones.

Antes de modificar un README:

1. Leer el README completo.
2. Inspeccionar el componente correspondiente.
3. Revisar su código.
4. Revisar manifiestos y configuración.
5. Revisar scripts de ejecución y pruebas.
6. Revisar contratos.
7. Revisar ADR relevantes.
8. Identificar información sensible.
9. Verificar la documentación relacionada.
10. Identificar contradicciones existentes.

Durante la modificación:

- No inventar información.
- No crear rutas ficticias.
- No inventar comandos.
- No publicar secretos.
- No cambiar ownership.
- No contradecir ADR.
- No presentar funcionalidades planificadas como terminadas.
- No eliminar información correcta sin justificación.
- No duplicar documentación.
- No crear README innecesarios.

Después de modificar:

1. Validar Markdown.
2. Validar enlaces.
3. Revisar rutas.
4. Verificar comandos documentados.
5. Ejecutar validaciones documentales disponibles.
6. Revisar exposición de información sensible.
7. Comprobar coherencia arquitectónica.
8. Informar los cambios realizados.
9. Informar las limitaciones o datos que no pudieron verificarse.

Los agentes no deben declarar éxito en verificaciones que no ejecutaron.

# 17. Definition of Done

Un README se considera terminado únicamente cuando cumple todos los criterios aplicables:

- [ ] Describe correctamente el componente.
- [ ] Respeta el ownership arquitectónico.
- [ ] No contiene información inventada.
- [ ] No contiene información sensible.
- [ ] No publica secretos ni credenciales.
- [ ] Distingue funcionalidades implementadas y planificadas.
- [ ] Los comandos están verificados.
- [ ] Las rutas documentadas existen.
- [ ] Las variables de entorno corresponden al código.
- [ ] Los ejemplos son seguros.
- [ ] Los enlaces funcionan.
- [ ] No duplica documentación oficial.
- [ ] Respeta el estándar visual.
- [ ] Tiene documentación relacionada cuando corresponde.
- [ ] Supera las validaciones automáticas aplicables.
- [ ] Fue revisado en el PR correspondiente.

La documentación general del proyecto también debe cumplir:

- [ ] README raíz actualizado.
- [ ] README de componentes coherentes.
- [ ] Referencias cruzadas correctas.
- [ ] Sin contradicciones entre documentos.
- [ ] Clasificación documental apropiada.
- [ ] Reglas de seguridad aplicadas.
- [ ] CI documental funcionando.

# 18. Prohibiciones expresas

Está prohibido:

1. Inventar funcionalidades.
2. Inventar comandos.
3. Inventar endpoints.
4. Inventar integraciones.
5. Inventar resultados de pruebas.
6. Publicar secretos.
7. Publicar credenciales.
8. Publicar datos reales de clientes.
9. Publicar información privada de infraestructura.
10. Declarar producción cuando solo existe desarrollo local.
11. Declarar escalabilidad o seguridad sin evidencia.
12. Contradecir ADR aprobados.
13. Cambiar responsabilidades arquitectónicas desde un README.
14. Crear README duplicados sin necesidad.
15. Marcar tareas como completadas sin evidencia.
16. Eliminar advertencias o limitaciones conocidas para mejorar la presentación.
17. Utilizar badges falsos.
18. Publicar procedimientos internos sensibles en documentación pública.

# 19. Relación con otros estándares ENSAMBRIA

Este estándar debe coexistir con:

- Estándar de arquitectura.
- Estándares de seguridad.
- Estándar de Transactional Outbox e Inbox.
- Estándar de Open Graph y SEO cuando corresponda.
- Estándar de acceso geográfico y de red.
- Estándar de observabilidad.
- Estándares de pruebas.
- Estándares de CI/CD.
- ADR.
- Políticas de contribución.

No sustituye estos documentos.

Si existe un conflicto, se debe identificar y resolver mediante el proceso de decisión documental y arquitectónica correspondiente.

# 20. Regla final

La documentación de ENSAMBRIA forma parte del producto de ingeniería y está sujeta a los mismos criterios de calidad, seguridad, trazabilidad y revisión que el código.

**Un README profesional no es el que promete más funcionalidades, sino el que explica con precisión lo que existe, cómo funciona, cómo se utiliza y cuáles son sus límites, sin revelar información que comprometa la seguridad del proyecto.**

La regla definitiva es:

**Código real → documentación verificable → validación automática → revisión en PR → documentación actualizada.**

Este estándar debería ser obligatorio desde la Fase 0. Además, te recomiendo que lo conecte con las reglas generales de contribución y CI, para que no dependa de que cada desarrollador recuerde revisar manualmente los README.
