# ENSAMBRIA — Architecture, Engineering & Project Governance Standard

**Tipo:** Estándar obligatorio de gobierno técnico y documental<br>
**Aplicación:** Desde la Fase 0<br>
**Alcance:** Todo el ecosistema ENSAMBRIA<br>
**Cumplimiento:** Obligatorio<br>
**Responsables:** Arquitectura, ingeniería, seguridad y responsables de cada dominio<br>
**Estado:** Estándar normativo

---

## 1. Objetivo

Establecer un sistema integral y obligatorio de gobierno arquitectónico, técnico, documental, de seguridad, calidad y operación para ENSAMBRIA.

El objetivo es garantizar que el proyecto evolucione de manera controlada, verificable, segura, escalable y coherente, evitando decisiones improvisadas, duplicación de responsabilidades, desviaciones arquitectónicas y documentación desconectada de la implementación real.

Este estándar debe definirse, aprobarse y comenzar a aplicarse desde la Fase 0. No se permite posponer su definición hasta fases posteriores.

## 2. Principios obligatorios

Todo el desarrollo de ENSAMBRIA debe respetar estos principios:

1. La arquitectura debe estar definida antes de implementar funcionalidades que dependan de ella.
2. Las decisiones arquitectónicas relevantes deben documentarse.
3. Cada dominio y componente debe tener responsabilidades y ownership explícitos.
4. Los contratos entre componentes deben estar definidos y versionados.
5. Toda regla de negocio importante debe tener una especificación verificable.
6. Toda funcionalidad debe tener criterios de aceptación y pruebas.
7. La seguridad debe incorporarse desde el diseño.
8. La documentación debe mantenerse sincronizada con el código.
9. Toda implementación debe contar con evidencia verificable.
10. Ninguna fase se considera completada por tener únicamente código funcional.
11. No se deben crear documentos vacíos para aparentar cumplimiento.
12. No se deben inventar decisiones, políticas comerciales, requisitos o capacidades que no hayan sido aprobados.

## 3. Sistema de documentación y gobierno

ENSAMBRIA debe mantener las siguientes categorías documentales.

### 3.1 Arquitectura

Documentación obligatoria:

- Architecture Overview.
- System Context.
- C4 Model.
- Container Diagrams.
- Component Diagrams cuando corresponda.
- Deployment Architecture.
- Data Architecture.
- Integration Architecture.
- Security Architecture.
- Architecture Decision Records (ADR).
- Architecture Principles.
- Architecture Constraints.
- Matriz de capacidades transversales con ownership, límites, contratos, dependencias, secuencia de fases y aceptación.

Los diagramas deben representar la arquitectura real o el diseño aprobado, identificando claramente ambos estados.

### 3.2 Diseño de dominio y negocio

Documentación obligatoria:

- Domain-Driven Design (DDD) Strategic Design.
- Bounded Contexts.
- Context Maps.
- Domain Glossary.
- Ownership Matrix.
- Business Rules Catalog.
- Capability Matrix.
- Domain Models.
- Business Process Definitions.
- Business Requirements.
- Functional Requirements.
- Non-Functional Requirements.

Las responsabilidades de CORE, BUSINESS, PLATFORM y PUBLIC deben permanecer coherentes con el modelo de ownership aprobado.

No se permite que un backend asuma silenciosamente la propiedad de datos, reglas o procesos pertenecientes a otro dominio.

### 3.3 Contratos e integraciones

Documentación obligatoria:

- API Design Standards.
- OpenAPI Specifications para contratos HTTP.
- Protobuf Definitions para contratos gRPC.
- Event Catalog.
- Event Schemas.
- Integration Contracts.
- API Versioning Policy.
- Event Versioning Policy.
- Backward Compatibility Policy.
- Deprecation Policy.

Todo contrato utilizado entre componentes debe tener una definición verificable.

No se permiten contratos implícitos, estructuras duplicadas sin justificación o cambios incompatibles sin una estrategia aprobada.

### 3.4 Seguridad y privacidad

Documentación obligatoria:

- Security Architecture.
- Security Policies.
- Threat Models.
- Access Control Matrix.
- Identity and Authentication Standards.
- Authorization Standards.
- Tenant Isolation Standard.
- Data Classification Policy.
- Secrets Management Standard.
- Encryption Standards.
- Audit Logging Standard.
- Privacy Policy.
- Data Retention Policy.
- Data Export and Deletion Procedures.
- Incident Response Plan.
- Vulnerability Management Policy.

La seguridad debe evaluarse desde el diseño y durante la implementación, no solamente antes de producción.

Los requisitos de seguridad deben convertirse en controles y pruebas verificables.

### 3.5 Calidad y pruebas

Documentación obligatoria:

- Testing Strategy.
- Test Plan.
- Quality Gates.
- Definition of Done.
- Acceptance Criteria.
- Requirements Traceability Matrix.
- Unit Testing Standards.
- Integration Testing Standards.
- Contract Testing Standards.
- End-to-End Testing Standards.
- Security Testing Standards.
- Performance Testing Standards.
- Resilience and Recovery Testing Standards.

Cada requisito verificable debe relacionarse con su implementación y sus pruebas correspondientes.

Las pruebas no deben limitarse al camino exitoso. Deben contemplar errores, permisos, aislamiento, concurrencia, límites, duplicados, estados inválidos y recuperación cuando corresponda.

### 3.6 Ingeniería y desarrollo

Documentación obligatoria:

- Engineering Standards.
- Coding Standards.
- Repository Structure Standard.
- Naming Conventions.
- Error Handling Standards.
- Logging Standards.
- Dependency Management Policy.
- Git Branching Strategy.
- Pull Request Policy.
- Code Review Policy.
- CI/CD Standards.
- Release Management Policy.
- Change Management Policy.
- Dependency Update Policy.

Estos estándares deben respetar las tecnologías y decisiones aprobadas para ENSAMBRIA.

No se deben introducir tecnologías, herramientas o patrones adicionales sin justificar su necesidad y compatibilidad arquitectónica.

El gobierno operativo del repositorio —ramas, commits, revisiones, GitHub Actions, seguridad de workflows y releases— se rige por [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](ENSAMBRIA_github_repository_governance_and_engineering_rules.md). Este documento conserva los requisitos de gobierno técnico y documental y no sustituye esa fuente especializada.

### 3.7 Infraestructura y operaciones

Documentación obligatoria:

- Infrastructure Architecture.
- Environment Strategy.
- Configuration Management.
- Deployment Standards.
- Deployment Runbooks.
- Rollback Procedures.
- Backup and Recovery Plan.
- Disaster Recovery Plan.
- Observability Standard.
- Monitoring and Alerting Strategy.
- SLI and SLO Definitions.
- Capacity Planning.
- Resource Management.
- Operational Incident Procedures.

Las instrucciones operativas deben ser reproducibles y no deben contener secretos ni información sensible expuesta.

## 4. Clasificación de documentos

Todo documento debe clasificarse según su finalidad:

| Categoría | Propósito |
|---|---|
| Estándar | Define cómo se debe trabajar o implementar. |
| Especificación | Define qué debe hacer un componente o funcionalidad. |
| Decisión | Registra por qué se eligió una solución. |
| Plan | Define qué se va a implementar y cuándo. |
| Procedimiento | Explica cómo ejecutar una actividad. |
| Evidencia | Demuestra que un requisito o control se cumple. |

No se deben utilizar ADR como sustitutos de especificaciones, estándares, planes o procedimientos.

## 5. Architecture Decision Records (ADR)

Toda decisión arquitectónica relevante debe documentarse mediante ADR.

Cada ADR debe incluir, como mínimo:

- Identificador único.
- Título.
- Fecha.
- Estado: proposed, accepted, rejected, deprecated o superseded.
- Contexto.
- Problema que se busca resolver.
- Decisión adoptada.
- Alternativas consideradas.
- Consecuencias positivas y negativas.
- Riesgos.
- Impacto en seguridad.
- Impacto en rendimiento y escalabilidad.
- Impacto en operación.
- Componentes afectados.
- Referencias relacionadas.

Los ADR aceptados deben respetarse en las implementaciones posteriores.

Si una decisión cambia, debe registrarse una nueva decisión que sustituya o complemente la anterior. No se debe modificar silenciosamente el historial de decisiones.

## 6. Ownership y límites arquitectónicos

Debe existir una matriz oficial de ownership que identifique:

- Backend propietario.
- Dominio y subdominio.
- Entidades y datos bajo su responsabilidad.
- Reglas de negocio.
- Casos de uso.
- Contratos expuestos.
- Eventos publicados.
- Eventos consumidos.
- Dependencias autorizadas.
- Responsabilidades operativas.

La matriz debe reflejar el modelo aprobado de ENSAMBRIA:

- CORE: identidad, tenancy, contexto, reservas comunes, pagos y responsabilidades financieras centrales.
- BUSINESS: operaciones comerciales de los establecimientos y sus dominios especializados.
- PLATFORM: administración global del SaaS, planes, suscripciones y políticas comerciales de ENSAMBRIA.
- PUBLIC: descubrimiento, directorio, marketplace y experiencias públicas.

Los límites deben aplicarse también a las bases de datos, permisos, contratos, eventos y pruebas.

## 7. Trazabilidad obligatoria

Debe mantenerse una relación verificable entre:

**Requisito → Especificación → Decisión (cuando aplique) → Implementación → Prueba → Ejecución CI → Evidencia.**

La trazabilidad debe permitir conocer:

- Qué requisito originó una funcionalidad.
- Qué componente es responsable.
- Qué contrato utiliza.
- Qué decisiones condicionan su diseño.
- Qué pruebas verifican su comportamiento.
- Qué ejecución de CI demuestra su cumplimiento.

No todos los requisitos necesitan un ADR. Los ADR se reservan para decisiones arquitectónicas relevantes.

## 8. Reglas de documentación

Toda documentación debe:

1. Reflejar el estado real del proyecto.
2. Identificar claramente lo implementado, parcial, planificado, bloqueado o depreciado.
3. Evitar afirmaciones sin evidencia.
4. Mantener enlaces válidos.
5. Referenciar contratos y componentes existentes.
6. Evitar duplicaciones contradictorias.
7. Tener responsable de mantenimiento cuando corresponda.
8. Proteger secretos, credenciales, datos personales y detalles sensibles.
9. Actualizarse cuando cambien los componentes afectados.
10. Ser revisada dentro del Pull Request correspondiente.

La documentación no debe utilizarse para presentar como implementadas funcionalidades que solamente están planificadas.

## 9. Reglas obligatorias para agentes de IA

Cualquier agente de IA que trabaje en ENSAMBRIA debe:

1. Leer los estándares aplicables antes de modificar código.
2. Consultar los ADR y decisiones vigentes.
3. Revisar ownership y contratos antes de modificar componentes.
4. Inspeccionar la implementación existente antes de proponer cambios.
5. No inventar arquitectura, endpoints, eventos, políticas ni configuraciones.
6. No introducir dependencias o tecnologías sin justificación.
7. No modificar decisiones aceptadas sin autorización y documentación.
8. Actualizar los documentos afectados por sus cambios.
9. Implementar pruebas correspondientes.
10. Presentar evidencia de las validaciones ejecutadas.
11. Identificar las decisiones pendientes sin convertirlas en decisiones implícitamente aprobadas.
12. No declarar una fase terminada si sus criterios de aceptación no están satisfechos.

Si existe una contradicción entre código, documentación y ADR, el agente debe identificarla, detener los cambios que dependan de esa contradicción y solicitar resolución o seguir el procedimiento de gobierno aprobado.

## 10. GitHub y controles de cumplimiento

Este estándar debe integrarse en el flujo de trabajo de GitHub. Para las reglas generales de gobierno del repositorio, ramas, Pull Requests, revisiones y seguridad de Actions, aplica [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](ENSAMBRIA_github_repository_governance_and_engineering_rules.md). Esta sección define los requisitos específicos de calidad arquitectónica, documental y de trazabilidad que deben verificarse en ese flujo.

### 10.1 Pull Requests

Todo PR debe:

- Identificar el alcance del cambio.
- Relacionar requisitos o tareas.
- Identificar componentes afectados.
- Actualizar documentación cuando corresponda.
- Incluir pruebas.
- Declarar riesgos relevantes.
- Indicar decisiones arquitectónicas nuevas o modificadas.
- Presentar resultados de validación.

### 10.2 Quality Gates

La CI debe comprobar progresivamente:

- Formato y estructura documental.
- Enlaces y referencias internas.
- Presencia de documentos obligatorios.
- Validez de contratos.
- Compatibilidad de esquemas.
- Validación de diagramas cuando existan herramientas adecuadas.
- Formato y lint de código.
- Compilación.
- Pruebas unitarias e integración.
- Pruebas de contratos.
- Pruebas de seguridad.
- Reglas de arquitectura y dependencias.
- Secret scanning y controles de dependencias.

Los controles deben ser automáticos cuando sea técnicamente viable.

Las comprobaciones deben ejecutarse selectivamente según los archivos y componentes afectados. Un cambio exclusivamente documental no debe ejecutar innecesariamente toda la compilación y batería de pruebas de Rust.

### 10.3 Protección de ramas

- `main` debe estar protegida.
- Los cambios deben ingresar mediante Pull Requests.
- No se permite fusionar PR con controles obligatorios fallidos.
- Los controles críticos de seguridad y arquitectura no pueden omitirse para acelerar una entrega.
- Los cambios en estándares y políticas deben tener revisión apropiada.

## 11. Aplicación desde la Fase 0

La Fase 0 debe establecer la base documental y de gobierno antes de iniciar la implementación funcional.

Como mínimo, debe definir y aprobar:

- Principios arquitectónicos.
- Architecture Overview.
- System Context.
- Modelo C4 inicial.
- Arquitectura de los cuatro backends y seis proyectos de interfaz, con sus responsabilidades y contratos.
- DDD Strategic Design.
- Bounded Contexts iniciales.
- Context Map inicial.
- Ownership Matrix.
- Domain Glossary.
- Architecture Decision Records iniciales.
- Estrategia de contratos.
- Estándares de seguridad.
- Tenant Isolation Standard.
- Estrategia de pruebas.
- Quality Gates.
- Definition of Done.
- Estándares de ingeniería.
- GitHub workflow.
- Estrategia de entornos.
- Arquitectura inicial de infraestructura.
- Estrategia de observabilidad.
- Estructura documental oficial.
- Política de trazabilidad.
- Matriz de las diez capacidades transversales del producto: aislamiento multi-tenant, multi-sucursal, multi-país/moneda, multi-idioma, reglas de dominio, eventos, auditoría, API pública, webhooks y Marketplace de extensiones. Para cada capacidad, identificar owner, actores/datos, límites de confianza, contratos e integraciones, controles, dependencias, fases, pruebas, evidencia, N/A y decisiones pendientes/aprobadores.

Esta matriz se vincula con la definición y secuencia de producto de la sección 14 de [docs/requerimiento.md](../../docs/requerimiento.md) y el contrato de ownership de [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md); no los duplica. La Fase 0 es documental y no implementa ni activa runtime, APIs externas, mercados, monedas, webhooks, eventos de negocio ni catálogo de extensiones. Las decisiones comerciales o legales pendientes deben conservar owner y alcance bloqueado; su presencia en la matriz no constituye aprobación.

En las fases de implementación, cada comportamiento se entrega en su dominio propietario, en el primer corte que lo requiera, junto con contratos, controles y pruebas de aceptación aplicables. No se agrupan estas capacidades en un motor multi-dominio, backend adicional o acceso cruzado a datos.

Los documentos que dependan de decisiones aún no aprobadas deben registrar explícitamente su estado pendiente, responsable y dependencia. No se debe inventar una decisión para completar un checklist.

## 12. Control de cambios

Los estándares y políticas no pueden modificarse informalmente.

Todo cambio debe:

1. Identificar el estándar afectado.
2. Explicar la razón.
3. Evaluar el impacto arquitectónico y operativo.
4. Identificar componentes y documentos afectados.
5. Obtener las aprobaciones correspondientes.
6. Actualizar la versión documental.
7. Incorporar los controles o pruebas necesarios.

Los cambios que alteren decisiones arquitectónicas deben generar o actualizar los ADR correspondientes.

## 13. Definition of Done documental y arquitectónico

Una fase no puede declararse completada si:

- No se actualizaron las especificaciones afectadas.
- No se actualizaron los contratos afectados.
- No se actualizó el ownership cuando hubo cambios de responsabilidad.
- No se documentaron las decisiones arquitectónicas relevantes.
- No existe trazabilidad de los requisitos aplicables.
- No se ejecutaron las pruebas requeridas.
- No existe evidencia de CI.
- Existen contradicciones conocidas sin resolver.
- Se documentaron capacidades que no existen.

La documentación debe ser parte del entregable de cada fase, no una tarea opcional posterior.

## 14. Gestión de excepciones

Cualquier excepción debe:

- Tener justificación técnica.
- Identificar el requisito que no se cumple.
- Evaluar los riesgos.
- Definir controles compensatorios.
- Tener responsable.
- Tener aprobación.
- Tener fecha de expiración o condición de revisión.
- Mantenerse registrada y auditable.

No se permiten excepciones indefinidas ni aprobaciones implícitas.

Los requisitos críticos de seguridad, privacidad y aislamiento no pueden degradarse por conveniencia de implementación.

## 15. Regla final obligatoria

**ENSAMBRIA no se desarrolla únicamente mediante código. Se desarrolla mediante arquitectura definida, dominios delimitados, contratos explícitos, decisiones registradas, estándares obligatorios, seguridad integrada, pruebas verificables, documentación sincronizada y evidencia de cumplimiento.**

Este estándar entra en vigor desde la Fase 0 y aplica a todos los componentes actuales y futuros de ENSAMBRIA.

Ninguna fase, módulo, backend, frontend, integración o funcionalidad queda exenta de su cumplimiento.

La existencia de un documento no demuestra cumplimiento: el cumplimiento debe verificarse mediante implementación, controles, pruebas y evidencia.
