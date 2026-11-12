# GitHub Repository Governance and Engineering Rules

**Clasificación:** Estándar técnico obligatorio<br>
**Aplicación:** Desde la Fase 0 hasta el mantenimiento y operación del proyecto<br>
**Ámbito:** Todos los repositorios, ramas, Pull Requests, commits, workflows, releases y colaboradores humanos o agentes de IA.

---

# 1. Propósito

Establecer un modelo obligatorio de gobierno, ingeniería, colaboración, calidad y seguridad para administrar proyectos de software mediante Git y GitHub.

GitHub no debe utilizarse únicamente como almacenamiento de código. Debe funcionar como una plataforma de control de cambios, trazabilidad, validación automatizada, colaboración, seguridad y entrega de software.

Estas reglas deben aplicarse desde la creación del repositorio, antes de comenzar la primera implementación funcional.

Ninguna fase puede ignorar estas reglas por considerarse una fase inicial, experimental o de desarrollo.

# 2. Principios fundamentales

Todo repositorio debe cumplir los siguientes principios:

1. Trazabilidad completa de los cambios.
2. Protección de la rama principal.
3. Trabajo mediante ramas independientes.
4. Integración mediante Pull Requests.
5. Validaciones automatizadas obligatorias.
6. Pruebas desde la Fase 0.
7. Revisión de código antes de integrar cambios.
8. Seguridad desde el inicio.
9. Documentación sincronizada con la implementación.
10. Reproducibilidad de compilaciones y pruebas.
11. Integridad del historial de cambios.
12. Responsabilidad explícita sobre cada modificación.
13. Prohibición de ocultar o ignorar errores de CI.
14. Prohibición de aprobar implementaciones sin evidencia verificable.

Estas reglas son aplicables tanto a desarrolladores humanos como a agentes de inteligencia artificial.

# 3. Gobierno del repositorio desde la Fase 0

Antes de iniciar la implementación funcional deben establecerse:

- Repositorio y organización propietarios.
- Rama principal.
- Estrategia de ramas.
- Política de Pull Requests.
- Protección de ramas.
- Permisos de colaboradores.
- CODEOWNERS cuando exista una estructura de responsables.
- GitHub Actions.
- Validaciones locales y remotas.
- Plantillas de Pull Requests.
- Política de pruebas.
- Política de seguridad.
- Política de releases.
- Política de versionado.
- Criterios de Definition of Done.

No se debe comenzar a desarrollar funcionalidades importantes mientras los controles fundamentales del repositorio permanezcan sin definir.

# 4. Protección de la rama principal

La rama `main` debe estar protegida durante todo el ciclo de vida del proyecto.

Reglas obligatorias:

1. Prohibido realizar trabajo de implementación directamente sobre `main`.
2. Prohibido hacer push directo a `main`.
3. Prohibido utilizar force push sobre `main`.
4. Prohibido eliminar `main`.
5. Todo cambio debe integrarse mediante Pull Request.
6. Los Pull Requests deben superar las verificaciones obligatorias.
7. Los cambios sensibles deben contar con revisión de los responsables correspondientes.
8. Las reglas de protección deben aplicarse también a administradores, salvo excepciones explícitas, controladas y auditadas.
9. No se deben permitir merges que omitan verificaciones obligatorias.
10. Los permisos administrativos no justifican ignorar las políticas de ingeniería.

Se recomienda utilizar GitHub Rulesets o Branch Protection Rules para implementar estos controles.

La configuración efectiva debe verificarse en GitHub; documentar una regla no significa que esté técnicamente protegida.

# 5. Estrategia de ramas

Cada unidad de trabajo debe desarrollarse en una rama independiente creada desde la base actualizada correspondiente.

Nomenclatura recomendada:

| Prefijo | Uso |
|---|---|
| `feat/` | Nueva funcionalidad |
| `fix/` | Corrección de defectos |
| `refactor/` | Refactorización |
| `test/` | Incorporación o mejora de pruebas |
| `docs/` | Documentación |
| `security/` | Cambios de seguridad |
| `infra/` | Infraestructura |
| `perf/` | Optimización de rendimiento |
| `build/` | Sistema de compilación |
| `ci/` | Integración continua |
| `chore/` | Mantenimiento técnico |

Ejemplos:

- `feat/tenant-management`
- `feat/payment-intents`
- `fix/tenant-isolation`
- `security/api-key-rotation`
- `refactor/identity-domain`
- `test/payment-concurrency`
- `docs/architecture-governance`

Reglas:

1. Las ramas deben tener un objetivo concreto.
2. No mezclar funcionalidades independientes en una misma rama.
3. Mantener las ramas pequeñas y revisables.
4. Evitar ramas de larga duración.
5. Actualizar la base de la rama antes de integrar cambios cuando sea necesario.
6. No reutilizar ramas para objetivos diferentes.
7. Eliminar ramas remotas después de fusionar, cuando corresponda.
8. No desarrollar sobre ramas de otros trabajos sin coordinación.
9. No iniciar una rama desde una base desactualizada sin analizar las consecuencias.
10. No realizar merges fuera del orden de dependencias establecido.

La duración de una rama debe minimizarse, preferiblemente entre uno y tres días de trabajo, sin convertir este plazo en una restricción que obligue a dividir incorrectamente una unidad técnica.

# 6. Política de commits

Los commits deben ser pequeños, coherentes, descriptivos y verificables.

Se recomienda Conventional Commits:

- `feat:`
- `fix:`
- `refactor:`
- `test:`
- `docs:`
- `security:`
- `perf:`
- `build:`
- `ci:`
- `chore:`

Ejemplos:

- `feat: implement tenant membership management`
- `fix: prevent cross-tenant data access`
- `test: add concurrent payment idempotency tests`
- `security: enforce api key scope validation`
- `docs: document database ownership boundaries`

Reglas:

1. Un commit debe representar un cambio lógico.
2. No utilizar mensajes como `update`, `changes`, `fixes` o `final`.
3. No incluir archivos generados innecesarios.
4. No incluir secretos, credenciales ni información sensible.
5. No modificar el historial compartido mediante force push sin autorización y justificación.
6. Los commits deben ser compatibles con las convenciones del repositorio.
7. No introducir cambios no relacionados en un commit.

# 7. Política de Pull Requests

Todo cambio que vaya a integrarse en `main` debe pasar por un Pull Request.

Cada PR debe incluir:

- Título descriptivo.
- Objetivo.
- Contexto del problema.
- Alcance.
- Cambios realizados.
- Arquitectura afectada.
- Riesgos identificados.
- Pruebas incorporadas.
- Pruebas ejecutadas.
- Resultados de CI.
- Documentación actualizada.
- Migraciones, si existen.
- Consideraciones de seguridad.
- Criterios de aceptación.

Reglas:

1. No crear PR vacíos.
2. No fusionar PR sin la revisión aplicable de acuerdo con la sección 8; cuando el repositorio tenga una sola persona mantenedora, se admite auto-revisión documentada para cambios ordinarios sin presentarla como revisión independiente.
3. No fusionar PR con checks fallidos.
4. No fusionar PR con checks obligatorios pendientes.
5. No fusionar PR con conversaciones de revisión pendientes.
6. No ocultar fallos mediante desactivación de tests o workflows.
7. No reducir requisitos de calidad para facilitar una fusión.
8. No introducir cambios fuera del alcance sin justificación.
9. No fusionar implementaciones que contradigan las reglas arquitectónicas.
10. Resolver conflictos antes de integrar.
11. Mantener los PR enfocados y revisables.
12. Actualizar documentación y requisitos cuando el cambio los afecte.

Se recomienda utilizar Squash Merge como estrategia predeterminada, salvo que el proyecto tenga una razón documentada para utilizar otro método.

# 8. Revisión de código

Toda revisión debe evaluar tanto la corrección funcional como la calidad técnica.

La revisión debe comprobar:

- Arquitectura.
- Ownership de datos.
- Límites entre dominios.
- Seguridad.
- Autorización.
- Validación de entradas.
- Manejo de errores.
- Concurrencia.
- Integridad transaccional.
- Rendimiento.
- Observabilidad.
- Pruebas.
- Documentación.
- Compatibilidad.
- Riesgos de regresión.

Los cambios críticos deben requerir revisión de responsables especializados.

Los agentes de IA no deben considerarse automáticamente revisores independientes cuando han implementado el mismo cambio.

## 8.1 Repositorio con una sola persona mantenedora

Cuando solo exista una persona mantenedora autorizada, no se exige designar una segunda persona como CODEOWNER ni bloquear todos los PR por falta de aprobación independiente. La protección de `main` debe conservar PR obligatorio, checks requeridos, historial protegido y resolución de conversaciones; la aprobación humana/CODEOWNER obligatoria puede configurarse en cero hasta que se incorporen más colaboradores.

La persona autora debe revisar el diff completo, verificar alcance, secretos, documentación, pruebas y riesgos, y dejar constancia en el PR. Esta auto-revisión aporta trazabilidad, pero no equivale a aprobación independiente. CODEOWNERS puede identificar a la persona titular actual sin convertirla en requisito de aprobación.

Los cambios cuyo riesgo o fuente normativa exija una revisión independiente —incluidos cambios críticos de seguridad, permisos, autenticación, privacidad, infraestructura o workflows— no se autoaprueban. Si no hay revisor independiente disponible, se detiene la integración del alcance afectado hasta obtener revisión autorizada. La excepción de operación unipersonal se reevalúa al incorporar otra persona con permisos de revisión.

# 9. GitHub Actions e integración continua

La integración continua debe existir desde la Fase 0.

Cada repositorio debe definir workflows apropiados para las tecnologías que realmente utilice.

Las verificaciones deben incluir, cuando correspondan:

- Formato.
- Lint.
- Compilación.
- Verificación de tipos.
- Pruebas unitarias.
- Pruebas de integración.
- Pruebas de contratos.
- Pruebas de seguridad.
- Análisis de dependencias.
- Validación de documentación.
- Construcción de artefactos.

Reglas:

1. Las verificaciones locales y las de CI deben ser coherentes.
2. Los workflows deben ser reproducibles.
3. Las dependencias deben instalarse de manera determinista.
4. Deben fijarse versiones de herramientas cuando corresponda.
5. Los resultados deben ser verificables.
6. Los errores deben corregirse en su origen.
7. No ocultar fallos mediante `continue-on-error` injustificado.
8. No desactivar verificaciones para permitir un merge.
9. No reducir artificialmente la cobertura o exigencia de los tests.
10. No ejecutar despliegues productivos desde workflows de validación ordinarios.

## 9.1. CI selectivo

Los workflows deben ejecutar las verificaciones correspondientes a los componentes afectados, considerando también sus dependencias.

Reglas:

- Cambios exclusivos de documentación no deben ejecutar innecesariamente compilaciones y pruebas completas de todos los backends.
- Cambios en una biblioteca compartida deben activar las pruebas de los consumidores afectados.
- Cambios en contratos deben validar productores y consumidores relacionados.
- Cambios de infraestructura deben ejecutar validaciones de infraestructura.
- Cambios globales deben ejecutar las verificaciones globales correspondientes.
- No omitir verificaciones obligatorias por utilizar filtros de rutas incorrectos.

La optimización de CI nunca debe reducir la cobertura efectiva de validación.

# 10. Pruebas desde la Fase 0

Las pruebas no son una actividad que deba posponerse hasta una fase avanzada.

Cada funcionalidad debe incorporar las pruebas correspondientes desde su implementación inicial.

Tipos de pruebas:

1. Unitarias.
2. Integración.
3. Contratos.
4. Seguridad.
5. Concurrencia.
6. Aislamiento multi-tenant.
7. End-to-end.
8. Rendimiento.
9. Recuperación ante fallos.
10. Compatibilidad.

La selección debe responder al riesgo y a la naturaleza de cada funcionalidad.

## 10.1. Requisitos de calidad

Las pruebas deben comprobar:

- Casos positivos.
- Casos negativos.
- Entradas inválidas.
- Límites.
- Duplicados.
- Concurrencia.
- Permisos.
- Aislamiento.
- Estados.
- Fallos.
- Reintentos.
- Integridad.
- Idempotencia.
- Redacción de información sensible, cuando corresponda.

No se considera suficiente comprobar únicamente el camino exitoso.

## 10.2. Calidad de las pruebas

Está prohibido:

- Deshabilitar pruebas para conseguir un CI verde.
- Sustituir pruebas de integración por mocks sin justificación.
- Ignorar errores de pruebas.
- Reducir aserciones para evitar fallos.
- Omitir escenarios negativos importantes.
- Declarar cobertura sin evidencia.
- Utilizar datos productivos sensibles en pruebas.

Los mocks pueden utilizarse para aislar unidades cuando sea técnicamente correcto, pero nunca deben sustituir las pruebas necesarias de integración con dependencias reales.

# 11. Seguridad del repositorio

La seguridad debe implementarse desde la creación del repositorio.

Controles recomendados y obligatorios según el riesgo:

- Secret scanning.
- Push protection cuando esté disponible.
- Dependabot o gestión equivalente de dependencias.
- Code scanning.
- Revisión de dependencias vulnerables.
- Revisión de permisos de GitHub Actions.
- CODEOWNERS.
- Revisión de cambios en workflows.
- Protección de ramas.
- Restricción de tokens.
- Protección de artefactos y releases.
- Revisión de licencias.
- SBOM y procedencia de artefactos cuando corresponda.

## 11.1. Secretos

Está prohibido almacenar en Git:

- Contraseñas.
- API Keys.
- Tokens.
- Claves privadas.
- Certificados privados.
- Credenciales de bases de datos.
- Secretos de proveedores.
- Archivos `.env` con valores reales.
- Credenciales de infraestructura.

Los secretos deben gestionarse mediante mecanismos seguros como GitHub Secrets, Vault u otros gestores autorizados.

Si se expone un secreto:

1. Revocarlo o rotarlo inmediatamente.
2. Investigar el alcance.
3. Evaluar accesos no autorizados.
4. Corregir la exposición.
5. Revisar el historial cuando corresponda.
6. Documentar el incidente.

Eliminar el secreto del último commit no significa que haya dejado de estar comprometido.

# 12. GitHub Actions: permisos y cadena de suministro

Los workflows deben operar con privilegios mínimos.

Reglas:

1. Declarar permisos explícitos mediante `permissions`.
2. Evitar permisos globales excesivos.
3. Fijar acciones de terceros a SHA completos cuando la política de seguridad lo requiera.
4. No ejecutar código no confiable con secretos privilegiados.
5. No exponer secretos a workflows de Pull Requests de forks.
6. Revisar cambios en `.github/workflows/`.
7. Evitar ejecutar comandos construidos directamente a partir de entradas no confiables.
8. Utilizar entornos protegidos para despliegues.
9. Separar CI de CD.
10. Auditar las modificaciones de workflows.

Los workflows de CI no deben tener permisos de escritura si solamente necesitan leer el repositorio y ejecutar pruebas.

# 13. Gestión de dependencias

Las dependencias forman parte de la superficie de ataque y del mantenimiento del proyecto.

Reglas:

- Utilizar versiones controladas.
- Mantener lockfiles.
- Revisar vulnerabilidades.
- Evaluar licencias.
- Evitar dependencias abandonadas cuando existan alternativas razonables.
- Analizar dependencias transitivas.
- Evitar duplicaciones innecesarias.
- No actualizar dependencias críticas sin validaciones.
- Registrar cambios relevantes.
- Mantener instalaciones reproducibles.

En Rust debe utilizarse `Cargo.lock` en aplicaciones y ejecutarse validaciones con `--locked` cuando corresponda.

En proyectos Node.js debe utilizarse el gestor y lockfile definidos por el repositorio, con instalaciones deterministas.

# 14. CODEOWNERS y responsabilidad técnica

Los repositorios deben utilizar `CODEOWNERS` cuando exista una estructura de responsables técnicos.

El archivo debe asignar responsables a áreas sensibles, como:

- Arquitectura.
- Seguridad.
- Infraestructura.
- Bases de datos.
- Workflows.
- Autenticación.
- Pagos.
- Contratos.
- Datos sensibles.

Las reglas de CODEOWNERS deben acompañarse de configuraciones de revisión obligatoria en GitHub cuando se requiera que dichas aprobaciones sean exigibles.

No inventar usuarios ni equipos. Utilizar exclusivamente responsables reales y autorizados.

# 15. Versionado y releases

Los releases deben ser reproducibles y trazables.

Se recomienda Semantic Versioning cuando el tipo de proyecto lo permita:

- MAJOR: cambios incompatibles.
- MINOR: funcionalidades compatibles.
- PATCH: correcciones compatibles.

Cada release debe identificar:

- Versión.
- Commit asociado.
- Cambios incluidos.
- Correcciones.
- Cambios incompatibles.
- Migraciones necesarias.
- Dependencias relevantes.
- Artefactos generados.
- Evidencias de validación.

Reglas:

1. No publicar releases sin validaciones.
2. No declarar una versión estable sin cumplir los criterios establecidos.
3. No generar artefactos desde código no identificado.
4. Mantener trazabilidad entre versión y commit.
5. No publicar secretos en artefactos.
6. Documentar procedimientos de rollback cuando correspondan.
7. No confundir una etiqueta Git con una verificación de calidad.

# 16. Gestión de cambios de base de datos

Las migraciones deben tratarse como cambios críticos.

Reglas:

- Deben versionarse.
- Deben revisarse.
- Deben validarse.
- Deben probarse en entornos apropiados.
- Deben preservar integridad.
- Deben considerar compatibilidad con versiones coexistentes de la aplicación.
- Deben documentar riesgos.
- Deben considerar rollback o estrategia de recuperación.

No ejecutar migraciones productivas automáticamente desde un workflow genérico de CI.

Las migraciones productivas requieren un procedimiento de despliegue autorizado y verificable.

# 17. Documentación y trazabilidad

La documentación debe mantenerse sincronizada con la implementación.

Cuando un cambio afecte arquitectura, contratos, seguridad, operación o requisitos, actualizar los documentos correspondientes.

Los PR deben permitir rastrear:

Requisito → Issue o tarea → Rama → Commit → Pull Request → CI → Merge → Release.

No crear documentación que declare capacidades inexistentes.

Los estados documentales deben distinguir entre implementado, parcialmente implementado, en progreso, planificado, bloqueado y obsoleto.

# 18. Trabajo con agentes de inteligencia artificial

Los agentes que operen sobre el repositorio deben respetar las mismas reglas que los desarrolladores humanos.

Reglas:

1. No trabajar directamente en `main`.
2. No realizar merges sin autorización.
3. No modificar workflows fuera del alcance autorizado.
4. No eliminar pruebas para conseguir resultados satisfactorios.
5. No modificar políticas de seguridad para facilitar una implementación.
6. No inventar resultados de CI.
7. No afirmar que una prueba pasó si no se ejecutó.
8. No introducir cambios ajenos al objetivo.
9. Informar todos los archivos modificados.
10. Respetar CODEOWNERS y las aprobaciones requeridas.
11. No publicar releases ni desplegar producción sin autorización.
12. Mantener trazabilidad de sus modificaciones.

El agente coordinador debe comprobar que los especialistas respetan estas reglas antes de considerar una tarea terminada.

# 19. Definition of Done para Pull Requests

Un Pull Request solo puede considerarse listo para integrar cuando:

- El objetivo está cumplido.
- El alcance está respetado.
- La arquitectura es correcta.
- No se violan límites de ownership.
- Las pruebas pertinentes existen.
- Las pruebas se ejecutaron.
- Las verificaciones de CI están aprobadas.
- No existen vulnerabilidades críticas o altas sin tratamiento aprobado.
- No se introdujeron secretos.
- La documentación está actualizada.
- Los cambios fueron revisados.
- Las conversaciones obligatorias están resueltas.
- Las migraciones fueron validadas cuando corresponda.
- Los criterios de aceptación están satisfechos.

Un CI verde por sí solo no demuestra que la implementación sea correcta.

# 20. Política de excepciones

Las excepciones deben ser explícitas, justificadas, aprobadas y trazables.

Toda excepción debe indicar:

- Regla afectada.
- Motivo.
- Riesgo.
- Alcance.
- Responsable que la aprueba.
- Duración.
- Medida compensatoria.
- Condición para eliminar la excepción.

No se permiten excepciones silenciosas ni permanentes sin revisión.

La configuración temporal de una sola persona mantenedora descrita en §8.1 es una excepción limitada de revisión humana; no elimina PR, checks, protección de rama ni revisión de riesgos, y debe actualizarse cuando cambie el número de colaboradores.

# 21. Auditoría y mejora continua

El gobierno del repositorio debe revisarse periódicamente.

Revisar:

- Protección de ramas.
- Permisos.
- Secretos.
- Dependencias.
- Workflows.
- CODEOWNERS.
- Reglas de revisión.
- Historial de cambios.
- Releases.
- Incidentes.
- Excepciones.
- Cumplimiento de CI.

Los cambios a este estándar deben revisarse mediante Pull Request y conservar trazabilidad.

# 22. Criterio final de cumplimiento

Estas reglas son obligatorias desde la Fase 0 y continúan aplicándose durante todo el ciclo de vida del proyecto.

Ninguna funcionalidad, fase, urgencia, agente de IA o desarrollador puede justificar la eliminación de los controles fundamentales de calidad, seguridad, trazabilidad y gobierno.

El repositorio debe permitir demostrar qué se cambió, quién o qué agente lo cambió, por qué se cambió, qué validaciones se ejecutaron, quién lo revisó y bajo qué condiciones se integró.

**La calidad del software no se demuestra únicamente porque compila o porque GitHub muestra un CI verde. Se demuestra mediante arquitectura correcta, pruebas suficientes, seguridad, revisión, trazabilidad y evidencia reproducible.**
