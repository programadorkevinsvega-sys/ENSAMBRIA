# ADR-0003: Workspace Cargo único en la raíz y unidades de despliegue independientes

- **Estado:** diseño aceptado expresamente por el propietario el 2026-11-11; materialización ejecutable reservada para la fase de implementación
- **Fecha:** 2026-11-10
- **Ámbito:** composición de Rust para CORE, BUSINESS, PLATFORM, PUBLIC y BFF público si aplica
- **Fuentes normativas:** [contrato arquitectónico](../../ARCHITECTURE_RULES.md), [requerimiento ENSAMBRIA](../requerimiento.md)

## Contexto comprobado

La arquitectura objetivo define cuatro backends Rust como monolitos modulares, con ownership de dominio y despliegues independientes. Compartir compilación no debe fusionar identidades, procesos, configuración, imágenes ni responsabilidades operativas.

El estado inspeccionado del manifiesto raíz [`Cargo.toml`](../../Cargo.toml) es:

- paquete `[package]` llamado `ENSAMBRIA`, versión `0.1.0`, edición `2024`;
- no declara `[workspace]`, dependencias ni target mediante `src/main.rs`, `src/lib.rs`, `[lib]`, `[[bin]]` u otra ruta;
- no hay crates/backend funcionales declarados por este manifiesto.

La verificación ejecutada fue `cargo metadata --no-deps --format-version 1`. Falló con código de salida 101 porque el manifiesto declara un paquete sin targets. Por ello, no se puede afirmar que el workspace esté configurado, que tenga miembros definidos ni que pueda construir alguno de los cuatro backends. No se ejecutó compilación ni prueba de producto.

## Decisión arquitectónica

1. El único workspace Cargo se declara en `Cargo.toml` en la raíz del repositorio. La forma virtual o con paquete raíz legítimo se determina al crear el primer miembro funcional; no cambia el requisito de un solo workspace.
2. No crear crates vacíos, targets ficticios, bins placeholder, bibliotecas técnicas sin uso ni manifests para aparentar que los cuatro backends ya existen.
3. Agregar cada paquete al workspace solo cuando implemente una unidad real, con identidad, ownership y pruebas propios. Cada backend y el BFF público independiente, si corresponde, conserva configuración de build, artefacto ejecutable, imagen/binario, puertos, secretos y operación de despliegue separados.
4. Compartir workspace o crates neutrales no autoriza imports de implementación ni acceso cruzado a tablas. La composición debe mantener contratos y dirección de dependencias del monolito modular.
5. El workspace facilita resolver dependencias y compilar en una raíz; no implica que todos los paquetes se construyan en una sola imagen o arranquen en un mismo proceso.

### Dirección de dependencias y composición

- Cada módulo conserva sus capas Domain, Application, Adapters e Infrastructure. Domain no depende de capas externas ni de otro módulo. Application depende de Domain y define puertos/casos de uso. Adapters implementa los puertos y puede depender de contratos estables y bibliotecas técnicas neutrales. Infrastructure/composition root construye el grafo, configura el pool por proceso y conecta adapters sin trasladar reglas de dominio.
- Un módulo consume otro módulo o backend únicamente por su contrato publicado; se prohíben imports de implementación, consultas a tablas ajenas y joins entre schemas de ownership distinto.
- Las bibliotecas compartidas se limitan a primitivas técnicas sin modelos/reglas de negocio. Los crates de contrato pertenecen a los bounded contexts propietarios y no contienen lógica de dominio ajena.
- Los BFF y frontends dependen de contratos HTTP/OpenAPI o gRPC autorizados, nunca de repositorios, tablas, NATS, Redis ni lógica interna del módulo.
- La estructura objetivo completa está definida en [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md) y el diseño de los contextos en [modelo de dominios y estructura objetivo](../arquitectura-dominios.md). Es un mapa documental, no una orden de crear todos esos directorios o paquetes en Fase 0.

## Cómo resolver la diferencia entre objetivo y estado actual

En Fase 0 se registra la composición objetivo y el manifiesto actual, sin adelantar implementación. La comprobación de aceptación se hace cuando exista al menos un paquete ejecutable legítimo:

1. `cargo metadata --no-deps --format-version 1` debe completarse y mostrar la raíz esperada, un único workspace y cada miembro funcional presente con su manifest/target real.
2. Revisar la composición declarada contra el mapa de ownership: ningún dominio o frontend se cuenta como backend adicional; no hay workspace anidado ni miembro ficticio.
3. Para cada backend que se haya implementado, ejecutar su build y pruebas con su selección de paquete, revisar que produzca un binario/artefacto identificable y que la configuración de despliegue construya una unidad separada. El BFF independiente se valida como unidad separada solo cuando exista.
4. Revisar CI, Dockerfiles/manifiestos del componente y contratos para confirmar que no se genera una imagen que arranque varios backends. `cargo metadata` por sí solo no demuestra independencia de despliegue ni aceptación arquitectónica.
5. Hasta que haya targets válidos, el fallo actual de `cargo metadata` es evidencia del estado del manifiesto, no una razón para crear scaffolding durante Fase 0. La composición futura queda pendiente de un cambio de implementación con artefactos reales.

## Alternativas consideradas

1. Crear ya cuatro crates mínimos, vacíos o placeholder: descartada; no demuestra arquitectura y contradice la separación entre Fase 0 documental y runtime.
2. Un workspace por backend: descartada por la obligación de workspace Cargo único en la raíz.
3. Un único paquete/proceso que aloje y despliegue todos los backends: descartada; confunde composición de compilación con límites de despliegue y ownership.
4. Workspace raíz que incorpore paquetes legítimos gradualmente y preserve binarios/despliegues separados: decisión del requerimiento.

## Consecuencias y pendientes

- El requisito arquitectónico puede estar decidido documentalmente aunque el `Cargo.toml` actual todavía no lo implemente. El inventario P0 debe distinguir ese diseño de la composición ejecutable.
- Antes de implementar backend Rust, se necesita definir el primer paquete real, su target, dependencias, pruebas, composición y modo de build sin crear código fuera del roadmap.
- No se afirma que Edition 2024, el toolchain, la compatibilidad de dependencias o CI estén validados; la metadata falla antes de permitir esa comprobación.
- El nombre y versión actuales del paquete raíz son datos comprobados, no decisiones aprobadas sobre naming/versionado de los futuros backends.

## Estado de aceptación y aspectos pendientes

El workspace único en la raíz y los despliegues independientes son decisiones ya fijadas por el requerimiento y fueron aceptados documentalmente por el propietario el 2026-11-11. No se ratifica una alternativa arquitectónica ni se declara que Cargo tenga un workspace ejecutable.

La selección entre workspace virtual y paquete raíz con target legítimo se difiere hasta la primera implementación funcional porque depende de la composición real; es una decisión técnica acotada, no una excepción al workspace único. Los nombres, targets, dependencias y miembros se incorporan únicamente cuando exista código funcional y ownership definido. El manifiesto actual no satisface todavía los criterios ejecutables y `cargo metadata` falla por falta de target, según evidencia registrada.

## Referencias

- [Requerimiento ENSAMBRIA](../requerimiento.md)
- [Contrato de arquitectura](../../ARCHITECTURE_RULES.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
