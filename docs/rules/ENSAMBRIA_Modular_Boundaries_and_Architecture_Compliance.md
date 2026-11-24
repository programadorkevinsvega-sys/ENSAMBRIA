# ENSAMBRIA — Fronteras modulares y cumplimiento arquitectónico ejecutable

**Estado:** regla normativa aprobada por el propietario el 2026-11-24; aplicación obligatoria desde el primer cambio de implementación.
**Ámbito:** cuatro backends modulares, módulos/bounded contexts, BFF, PostgreSQL, contratos, eventos, CI/CD, staging y agentes humanos o automatizados.
**Autoridad:** complementa y concreta `ARCHITECTURE_RULES.md`, `docs/requerimiento.md`, ADR-0001, ADR-0002, ADR-0003 y ADR-0007. No sustituye sus decisiones de producto ni permite excepciones implícitas.

## 1. Principio de cumplimiento

Estas reglas son invariantes arquitectónicas, no recomendaciones. Cada cambio debe respetarlas. Un agente no puede elegir una arquitectura alternativa, reducir una frontera, posponer un control obligatorio ni declarar una excepción por conveniencia. Si detecta una contradicción documental real, detiene únicamente el alcance afectado, informa al propietario y solicita una decisión explícita antes de modificar implementación.

La documentación normativa se consulta antes de planificar y antes de editar. Los cambios que afecten una decisión se documentan coordinadamente en todas las fuentes afectadas y se integran mediante PR. Una instrucción de tarea, scaffold, estado verde de compilación o decisión técnica unilateral no anula estas reglas.

## 2. Unidad modular y crates Rust

1. Cada bounded context/módulo implementado debe tener un crate Rust propietario independiente dentro del único Cargo workspace raíz.
2. El crate propietario contiene su dominio, casos de uso, puertos y adapters internos. Sus módulos internos de dominio, aplicación, persistencia e infraestructura no son API de consumo de otros módulos.
3. Cada módulo publica un crate separado `<module>-contracts` con sus traits/puertos públicos, DTO propios y tipos de error estables. Los consumidores dependen únicamente de ese crate de contratos, nunca del crate de implementación del módulo propietario.
4. Los contratos no contienen entidades, agregados, structs de persistencia, modelos SQLx, repositorios ni tipos internos del owner. No se comparten structs de dominio entre módulos.
5. La composición en memoria ocurre mediante una implementación del trait público e inyección en el composition root. El consumidor solo conoce el contrato; no conoce ni importa la implementación concreta.
6. Los crates neutrales de infraestructura compartida se permiten únicamente cuando no contienen reglas ni modelos de dominio y su alcance está explícitamente inventariado.
7. Un workspace común no concede permiso para imports cruzados. Las dependencias deben formar un grafo dirigido y acíclico conforme al mapa de contratos. Las dependencias entre backends también pasan por contratos públicos autorizados; no se permite importar implementación de otro backend.
8. Un módulo sin implementación todavía no recibe un crate vacío/placeholder para aparentar cumplimiento. El inventario registra su estado y el requisito de crate se activa cuando se implementa código funcional.
9. Tonic es un adapter de transporte, no el contrato lógico. Al separar procesos, se implementa servidor/cliente Tonic sobre el mismo contrato; los casos de uso y consumidores no cambian su interfaz.
10. Un crate de contrato puede generar bindings desde Protobuf, pero la correspondencia entre trait, DTO y mensajes debe ser comprobada por pruebas de contrato. No se reutilizan tipos de dominio como mensajes protobuf.

**Defensa primaria:** el grafo de Cargo impide compilar imports de implementación no declarados. **Defensa secundaria:** `cargo metadata`, cargo-deny/cargo-modules y el validador de arquitectura comprueban miembros, dependencias directas/transitivas, imports y excepciones. No se reimplementa un analizador completo de Rust si el compilador y herramientas existentes ya imponen el límite.

## 3. Contratos Protobuf

1. Cada módulo implementado mantiene contratos Protobuf versionados desde el inicio, aunque su transporte inicial sea una llamada trait en memoria.
2. Los archivos `.proto` se organizan por owner y versión, por ejemplo `contracts/proto/<backend>/<module>/v1/*.proto`.
3. Se ejecutan obligatoriamente `buf lint` y `buf breaking` contra la referencia base aprobada. La referencia no puede apuntar al propio HEAD ni omitir la comparación.
4. Un cambio incompatible requiere nueva versión de paquete/servicio y estrategia explícita de coexistencia/migración; no se reescribe una versión publicada.
5. CI valida generación reproducible, lint, compatibilidad y pruebas de correspondencia DTO/trait/Protobuf.
6. Los contratos síncronos no sustituyen eventos. Los hechos asíncronos mantienen esquemas/versiones de evento independientes.

## 4. Ownership y seguridad PostgreSQL

1. El inventario `schema/table → backend → bounded context → owner de migración` es obligatorio y versionado.
2. PostgreSQL impone el ownership mediante roles y grants, no mediante confianza en convenciones ni análisis estático.
3. Cada módulo que posee datos tiene identidad/rol runtime propio con privilegios mínimos sobre su schema. No recibe grants de lectura o escritura sobre schemas de otros módulos.
4. Los roles de migración/DDL son distintos de los roles runtime; nunca se usan credenciales owner, superuser o BYPASSRLS en workers ni pruebas funcionales.
5. Los workers usan el rol del módulo propietario de los datos que procesan. Los tests de integración usan los mismos roles restringidos que runtime; la preparación de fixtures se hace mediante un rol de setup separado y nunca se reutiliza para ejecutar el escenario.
6. Se revocan permisos heredados de PUBLIC y grants amplios; se revisan memberships, ownership de objetos, secuencias, vistas, funciones, procedimientos, tipos, schemas y privilegios por defecto. Las funciones SECURITY DEFINER y vistas con privilegios requieren revisión explícita.
7. Cada prueba negativa intenta leer y modificar objetos de otro bounded context y demuestra denegación real de PostgreSQL. Deben probarse tablas, secuencias, vistas y funciones pertinentes.
8. RLS complementa, no sustituye, el aislamiento por schema/rol y las reglas tenant-scoped de ADR-0001.
9. El análisis estático SQL es defensa secundaria para detectar ownership de migraciones, referencias cruzadas, SQL dinámico evidente y desviaciones del inventario; no se presenta como barrera de seguridad primaria.
10. Ninguna transacción, constraint, FK o join cruza bounded contexts. Las referencias entre módulos son IDs y contratos; cada owner valida sus propias invariantes.

## 5. Comunicación y superficies

- Dentro de un mismo backend/proceso: trait + DTO del módulo, invocado en memoria por composition root.
- Al separar módulos en procesos/backends: gRPC/Tonic sobre el contrato Protobuf versionado.
- Frontend → BFF autorizado: HTTP REST/OpenAPI.
- BFF → backend owner: contrato gRPC autorizado.
- Los backends de dominio no exponen endpoints HTTP de negocio. Health, readiness y métricas técnicas solo se permiten según la lista explícita de superficies técnicas autorizadas; no son APIs de dominio.
- Las llamadas síncronas no se convierten en eventos solo para evitar diseñar contratos; los hechos asíncronos no se implementan como llamadas síncronas encadenadas.

## 6. Eventos y Outbox/Inbox

Todo hecho que requiera entrega durable debe tener owner, esquema versionado, productor, transacción de mutación+Outbox, relay recuperable, stream/subject JetStream, consumer durable, Inbox/idempotencia y recuperación verificable. La publicación ocurre fuera de la transacción PostgreSQL y solo se considera confirmada con PubAck durable. El consumer confirma ACK después del commit de Inbox y efecto local. Se asume entrega al menos una vez, con deduplicación; no se afirma exactly-once extremo a extremo.

Los consumidores y workers deben existir como unidades ejecutables cuando su responsabilidad lo requiera, con configuración, despliegue, health/observabilidad y política de recuperación. NATS Core se reserva para fan-out efímero; JetStream para hechos durables. No se inventan eventos para operaciones que no requieren integración durable; cada N/A se justifica por caso de uso.

## 7. Topología de despliegue y runtime

Cada backend, BFF independiente, relay y worker desplegable tiene artefacto, configuración, identidad, unidad de ejecución y ciclo de despliegue propios. El inventario relaciona binario/crate, paquete, configuración, systemd (o mecanismo aprobado), health/readiness, dependencias, puertos y SHA.

Se distinguen cuatro estados: declarado, empaquetado, instalado y activo/conectado. CI compara los estados y no declara desplegado un servicio solo porque exista código o un binario local. La verificación runtime se ejecuta en staging autorizado y recoge servicio activo, binario/versión, SHA, health, puertos y dependencias sin exponer secretos.

## 8. Pruebas y aceptación

- PR: pruebas deterministas, cargo metadata, límites de dependencias, Buf lint/breaking, tests con PostgreSQL real y roles restringidos, pruebas de contratos, Outbox/Inbox y topología de artefactos.
- Release/staging: inventario runtime y E2E real contra servicios desplegados, incluyendo recorridos completos y recuperación. No se sustituye por mocks, pruebas unitarias o compilación.
- El gate de staging se vincula al SHA exacto desplegado y probado. Si el SHA cambia, la evidencia anterior no acepta el nuevo SHA.
- Una prueba omitida, pendiente, cancelada o bloqueada no equivale a éxito.
- Todo control aplicable debe ser required check; si no puede ejecutarse por falta de entorno, el release permanece no aceptado, no se marca como aprobado.

## 9. Trazabilidad inmutable por ejecución

El mapa requisito→caso→prueba→evidencia se versiona en el repositorio. Contiene identificadores estables, owner, alcance, tipo de verificación y criterio de aceptación; no contiene SHA ni resultados editados manualmente como fuente de verdad. CI inyecta commit SHA, workflow/run ID, job, comando, resultado, artefactos y fecha en el informe de ejecución. La evidencia debe identificar exactamente el SHA probado. El reporte no puede convertir un requisito sin prueba en aprobado mediante una casilla manual.

## 10. Arquitectura Compliance Gate

Un único punto de entrada Rust `cargo xtask architecture` carga y valida los inventarios YAML una sola vez y ejecuta los controles. No se mantienen listas duplicadas en scripts Bash. Puede delegar en herramientas especializadas (cargo metadata, cargo-deny/cargo-modules, buf, PostgreSQL, systemd/SSH) y producir resultados por control, con errores accionables y evidencia.

Controles obligatorios:
1. grafo Cargo/crates e imports;
2. ownership SQL y pruebas negativas de grants;
3. traits, DTO, Protobuf, buf lint y breaking;
4. superficies HTTP/BFF;
5. Outbox/Inbox y recuperación;
6. artefactos y topología de despliegue;
7. inventario de runtime;
8. E2E de staging;
9. trazabilidad requisito/caso/prueba/evidencia/SHA.

Los controles PR y release/staging son jobs separados según sus dependencias, pero todos los gates aplicables son obligatorios. Los controles independientes pueden ejecutarse en paralelo. El job agregado falla si cualquier control aplicable falla, se omite o no tiene evidencia requerida.

## 11. Inventarios canónicos

Los inventarios YAML son la fuente única para:
- módulos, backend, crate propietario y crate contracts;
- dependencias permitidas;
- schema/tabla/objeto SQL y rol propietario;
- contratos protobuf, versiones y compatibilidad;
- superficies HTTP autorizadas;
- eventos durables, producer, relay, stream, consumer e Inbox;
- binarios, paquetes, unidades, configuración y health;
- requisitos, casos, pruebas y tipo de evidencia.

No se duplican estas listas en workflow, scripts o documentación paralela. Los cambios de inventario requieren revisión de ownership y validación de consistencia. No se permiten excepciones wildcard ni excepciones implícitas.

## 12. Regla para agentes y mantenedores

Antes de cualquier cambio, el agente debe leer `AGENTS.md`, `ARCHITECTURE_RULES.md`, este documento, `docs/requerimiento.md`, `docs/arquitectura-dominios.md`, ADR aplicables y reglas especializadas. Debe inspeccionar el estado real del repositorio y mapear requisitos/owners antes de editar.

Está prohibido:
- importar implementación de otro módulo/backend;
- compartir structs de dominio o acceder a tablas ajenas;
- exponer HTTP de dominio fuera de BFF autorizados;
- crear eventos durables sin Outbox/Inbox y recuperación;
- declarar un servicio desplegado sin evidencia de runtime;
- aceptar integración con mocks cuando se requiere staging real;
- modificar SHA/resultados de trazabilidad manualmente;
- reducir, saltar, neutralizar o convertir en warning un control obligatorio;
- crear excepciones o cambiar una decisión arquitectónica sin aprobación explícita del propietario.

Si falta un contrato, owner, grant, evidencia, entorno o decisión, el agente debe detener el alcance dependiente, explicar el bloqueo y solicitar decisión/acceso. No improvisa.

## 13. Referencias normativas

- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Requerimiento](../requerimiento.md)
- [Modelo de dominios](../arquitectura-dominios.md)
- [ADR-0001 PostgreSQL](../adr/ADR-0001-base-postgresql-compartida.md)
- [ADR-0002 Outbox CORE→PLATFORM](../adr/ADR-0002-outbox-alta-tenant-core-platform.md)
- [ADR-0003 workspace Cargo](../adr/ADR-0003-cargo-workspace.md)
- [ADR-0007 ownership SaaS](../adr/ADR-0007-separacion-contextos-facturacion-saas.md)
- [Outbox/Inbox](ENSAMBRIA_Transactional_Outbox_Inbox.md)
- [Pooling PostgreSQL](ENSAMBRIA_Connection_Pooling_PostgreSQL.md)
