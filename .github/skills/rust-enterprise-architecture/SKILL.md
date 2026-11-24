---
name: rust-enterprise-architecture
description: "Use when designing or changing Rust workspaces, backend or module boundaries, DDD/Clean/Hexagonal layers, ownership, dependency direction, persistence boundaries, or service contracts."
---

# Rust Enterprise Architecture

## Objetivo

Aplicar los límites arquitectónicos correctos a cambios Rust y preservar el ownership de cada dominio, sin copiar el contrato de arquitectura a esta Skill.

## Alcance

Workspace/crates, módulos, capas, dependencias, contratos entre módulos y backends, persistencia, composición y evolución arquitectónica; incluye modelado técnico de scopes tenant/sucursal, contexto regional y reglas tipadas por dominio cuando afecten límites Rust.

## Cuándo activarla

- Obligatoria al crear, dividir, mover o cambiar módulos, crates, servicios, contratos, composición o persistencia Rust.
- Complementaria cuando una tarea funcional altere ownership, dependencias o límites de confianza.
- Desde Fase 0, consulta el diseño transversal de la sección 14 de `docs/requerimiento.md` antes de proponer estructuras, contratos o composición relacionados con tenancy, sucursales, regiones, reglas o integraciones.

## Referencias normativas obligatorias

- [ADR-0012 — enforceable modular boundaries](../../../docs/adr/ADR-0012-fronteras-modulares-y-cumplimiento-arquitectonico.md)
- [Modular boundaries and architecture compliance](../../../docs/rules/ENSAMBRIA_Modular_Boundaries_and_Architecture_Compliance.md)

- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md): autoridad central para topología, ownership y decisiones del proyecto
- [Reglas técnicas Rust de monolito modular](../../../docs/rules/ENSAMBRIA_reglas-estrictas-de-arquitectura-monolito-modular.md): detalle complementario de implementación; no redefine la topología ni el ownership fijados en el contrato central
- [Estándar de gobierno del repositorio GitHub](../../../docs/rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md), cuando el cambio afecte reglas de repositorio, workflows o proceso de integración.
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), para verificar ownership y fase del caso de uso.

## Aplicación obligatoria en ENSAMBRIA

Antes de ejecutar el procedimiento:

1. Confirma fase, alcance autorizado, owner, requisito y aceptación; lee [AGENTS.md](../../../AGENTS.md), el contrato central [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), el requerimiento/fase pertinente y las secciones completas de reglas técnicas aplicables y contratos/ADR relacionados. Usa solo las secciones pertinentes de la guía estricta para detalle de implementación Rust, no como autoridad de topología/ownership.
2. Mantén cada regla, dato, schema/migración, API/Protobuf/evento y despliegue bajo el owner establecido; contrasta imports y dependencias con la topología real de cuatro backends. La Skill no autoriza crear crates ni runtime fuera de la fase.
3. Traduce límites y contratos a cambios concretos y pruebas de dependencia/ownership/aislamiento y checks de build aplicables. Si el manifest, estructura, contrato o decisión contradice una regla, detén ese alcance y reporta el conflicto.
4. Reporta fuente/sección, fase y owner, archivos, comprobaciones ejecutadas y evidencia; separa **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO**.

## Procedimiento operativo

1. Inspecciona el workspace Cargo, los crates, los puntos de composición y los límites existentes; no infieras arquitectura a partir de nombres de directorio.
2. Asigna cada regla de negocio, dato, migración y contrato a su dueño; verifica las dependencias permitidas y la dirección de las capas.
3. En tenancy/sucursales, conserva el contexto autenticado y los límites tenant-scoped; no asumas herencia de configuración ni acceso cruzado. Las reglas permanecen en el módulo propietario y no se centralizan en una biblioteca técnica.
4. Para cada integración, determina si necesita respuesta síncrona o entrega durable y consulta el estándar correspondiente; no agregues una llamada o evento sin necesidad funcional.
5. Diseña los cambios dentro de la composición y despliegue aprobados. Registra decisiones, contratos y N/A en la documentación de la fase antes de implementar.
6. Revisa el diff y ejecuta las validaciones de arquitectura/calidad aplicables a los crates afectados.

## Restricciones

- No conviertas módulos en microservicios ni cruces ownership, tablas, repositorios o imports internos.
- No asumas un framework, ORM, bus, layout o feature de Rust que no esté demostrado en manifests, código o decisiones aprobadas.
- Las reglas de ENSAMBRIA sobre backends, tecnologías e IDs no son defaults para otros proyectos.
- No presentes una regla del contrato como verificada solo porque esté documentada.

## Validaciones obligatorias

- Verifica límites de dependencias, ownership, contratos, migraciones, composición y despliegue contra las fuentes vigentes.
- Ejecuta los chequeos Cargo pertinentes mediante el procedimiento de `rust-quality-engineering`.
- Documenta toda no aplicabilidad con justificación técnica previa y evidencia verificable.

## Criterios de finalización

El diseño y el código respetan límites aprobados; los contratos y decisiones están documentados; el cambio mantiene ownership y cuenta con las pruebas aplicables ejecutadas.

## Conflictos y excepciones

La topología ENSAMBRIA de cuatro backends, el workspace único y el ownership se establecen en [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md). La guía estricta aporta detalle técnico Rust sin ser una segunda autoridad de arquitectura de producto. Si fuentes vigentes divergen más allá de esa delimitación, detén únicamente el alcance afectado y solicita una resolución documental explícita antes de implementarlo.

## Mantenimiento

Revisada el 2026-11-24. Actualizada junto con la guía estricta el 2026-11-10 para reflejar los cuatro backends. Revisa esta Skill junto con cambios a las fuentes enlazadas; conserva únicamente metadatos de versión/fecha presentes en la fuente.
