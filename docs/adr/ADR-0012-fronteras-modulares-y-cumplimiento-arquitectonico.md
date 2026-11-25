# ADR-0012: Fronteras modulares ejecutables y cumplimiento arquitectónico

- **Estado:** decisión expresa aprobada por el propietario el 2026-11-24; documentación normativa previa a implementación.
- **Fecha:** 2026-11-24
- **Ámbito:** módulos de CORE, BUSINESS, PLATFORM y PUBLIC; contratos, PostgreSQL, mensajería, BFF, CI/CD y staging.
- **Fuentes:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [requerimiento](../requerimiento.md), [ADR-0001](ADR-0001-base-postgresql-compartida.md), [ADR-0002](ADR-0002-outbox-alta-tenant-core-platform.md), [ADR-0003](ADR-0003-cargo-workspace.md), [ADR-0007](ADR-0007-separacion-contextos-facturacion-saas.md).

## Contexto

ENSAMBRIA requiere monolitos modulares con ownership estricto y posibilidad de separar módulos/procesos sin reescribir sus casos de uso. La revisión de reglas identificó que las convenciones y scripts estáticos por sí solos no son una defensa suficiente contra imports de implementación, acceso SQL cruzado, contratos incompatibles, APIs HTTP de dominio, eventos durables incompletos o despliegues declarados que no existen en runtime.

Esta decisión concreta cómo imponer esos límites con el compilador, PostgreSQL, herramientas de contratos, inventarios versionados y gates de CI/staging. No modifica los cuatro backends ni crea backends adicionales.

## Decisiones

### 1. Un crate por módulo y crate de contratos

1. Cada bounded context/módulo funcional implementado tiene un crate propietario separado en el único workspace Cargo raíz.
2. Cada módulo expone un crate `<module>-contracts` con traits públicos, DTO propios y tipos de error estables. Los consumidores dependen solo de contracts; no del crate propietario de implementación.
3. Dominio, aplicación interna, adapters, repositorios, modelos SQLx y tipos de persistencia no son API pública ni se exponen a consumidores.
4. El wiring en memoria implementa/injecta el trait en composition root. La interfaz de consumo no depende del transporte.
5. Los contratos Protobuf versionados se definen desde el inicio. Al separar procesos, Tonic implementa el mismo contrato mediante adapters servidor/cliente sin cambiar el trait ni los casos de uso.
6. Los crates neutrales compartidos solo contienen infraestructura sin dominio y se registran explícitamente.
7. Esta decisión precisa y actualiza ADR-0003: workspace único no significa crate único por backend ni autoriza imports internos; la granularidad de crate por módulo y el crate contracts son obligatorios cuando el módulo tiene implementación funcional.

### 2. PostgreSQL como barrera primaria de ownership

1. Cada módulo propietario de datos dispone de rol runtime mínimo y grants solo sobre su schema/objetos autorizados.
2. Roles DDL/migración, runtime, setup de tests y workers se separan. Workers y pruebas funcionales usan los mismos roles restringidos de runtime del owner.
3. Se revocan privilegios heredados y se prueban grants negativos contra tablas, secuencias, vistas y funciones ajenas.
4. RLS sigue siendo obligatoria donde aplica por aislamiento tenant; no reemplaza el ownership por rol/schema.
5. Análisis estático de SQL/migraciones es una segunda capa para consistencia del inventario, no la barrera de seguridad principal.

### 3. Contratos Protobuf

Se adopta Buf como control normativo: `buf lint` y `buf breaking` contra la referencia base aprobada se ejecutan en CI. Los cambios incompatibles requieren nueva versión y estrategia de coexistencia. Se prueban equivalencia y compatibilidad entre traits/DTO y mensajes protobuf.

### 4. Dependencias

El compilador y el grafo Cargo son la defensa primaria. CI usa `cargo metadata` y herramientas mantenidas como cargo-deny/cargo-modules para validar dependencias directas/transitivas y políticas; no crea un analizador Rust duplicado. El validador de arquitectura verifica el inventario y las reglas adicionales de imports.

### 5. Validador único

Se adopta `cargo xtask architecture` como punto único de entrada. El validador Rust carga los inventarios YAML una sola vez, comparte validación y genera errores accionables. Puede invocar herramientas especializadas. No habrá múltiples scripts Bash con listas de ownership o reglas duplicadas.

### 6. Eventos durables

Cada evento durable debe enlazar producer, Outbox transaccional, relay recuperable, JetStream durable, consumer durable, Inbox/idempotencia, ACK post-commit y prueba de recuperación. Se aplica la guía Outbox/Inbox y sus N/A justificados por caso de uso.

### 7. Despliegue y runtime

Los inventarios distinguen binarios declarados, empaquetados, instalados y activos/conectados. La aceptación de runtime requiere evidencia del staging real y SHA exacto. El E2E de staging es gate de release, separado de integración PR.

### 8. Trazabilidad

El mapa de requisitos/casos/pruebas se versiona sin SHA/resultados manuales. CI inyecta SHA, run/job, comandos, resultados y artefactos. La evidencia solo acepta el SHA exacto ejecutado.

## Consecuencias

- El modelo de módulos se refleja en límites reales del grafo Cargo y no solo en carpetas o visibilidad idiomática.
- La migración a crates por módulo es progresiva por módulo funcional, sin crates vacíos ni placeholders.
- Se requiere inventario de owners y revisión de grants PostgreSQL por módulo.
- Buf, cargo-deny/cargo-modules y xtask se integran después de validar versiones y compatibilidad con el toolchain real.
- El primer CI estricto puede fallar por incumplimientos existentes; no se rebajan controles para conseguir verde.
- Las pruebas de runtime/staging requieren credenciales y entorno autorizado; sin ellos el gate de release queda pendiente, nunca aprobado.
- Esta ADR no declara implementados los crates, roles, herramientas, workflows, workers o servicios runtime. Define decisiones y obligaciones para su implementación y aceptación.

## Referencias

- [Regla de fronteras modulares y cumplimiento](../rules/ENSAMBRIA_Modular_Boundaries_and_Architecture_Compliance.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Requerimiento](../requerimiento.md)
- [ADR-0001 PostgreSQL](ADR-0001-base-postgresql-compartida.md)
- [ADR-0002 Outbox CORE→PLATFORM](ADR-0002-outbox-alta-tenant-core-platform.md)
- [ADR-0003 workspace Cargo](ADR-0003-cargo-workspace.md)

## Implementación PostgreSQL por fases

El inventario canónico es [architecture/data-ownership.yaml](../../architecture/data-ownership.yaml) y el procedimiento de transición está en [module-ownership-cutover.md](../postgres/module-ownership-cutover.md). La primera fase solo provisiona roles/schemas aislados y prueba que no tienen permisos implícitos; no mueve tablas ni revoca todavía los grants legacy. La aceptación del ownership requiere completar las fases de contratos, movimiento de objetos, roles runtime, revocación legacy y pruebas negativas reales descritas en el plan.


## Enmienda — evidencia de ownership PostgreSQL en Fase 0 (2026-11-25)

La aceptación de Fase 0 requiere dos defensas complementarias y obligatorias:

1. Reconciliación estática de cada migración contra architecture/data-ownership.yaml, incluyendo secuencias, triggers y políticas creadas dinámicamente por bucles PL/pgSQL y EXECUTE format(). El validador debe fallar tanto por objetos SQL no inventariados como por entradas inventariadas que no existen.
2. Verificación del estado efectivo después de aplicar las migraciones a PostgreSQL efímero. La consulta de catálogos debe comprobar propietarios de schemas/objetos, atributos y membresías de roles, privilegios heredados/efectivos, PUBLIC, secuencias, funciones, ausencia de FKs entre schemas y cobertura exacta de RLS/policies.

Las pruebas negativas deben usar los roles runtime reales y demostrar denegación de lectura y escritura cruzada, spoofing de tenant/actor y fallo cerrado por contexto ausente para las familias RLS aplicables. El catálogo verifica todas las relaciones/policies; las pruebas de comportamiento cubren las familias tenant y actor. CI ejecuta estos controles después de las migraciones y conserva evidencia asociada al SHA probado.

Esta enmienda no autoriza mover tablas legacy ni ejecutar el cutover. La preparación de roles/schemas y la aceptación del enforcement en PostgreSQL preceden a cualquier migración de ownership de datos existentes.
