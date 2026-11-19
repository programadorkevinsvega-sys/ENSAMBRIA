# Diseño inicial de consentimiento y preferencias

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** propuesta técnica de Fase 0, no interfaz, registro de consentimientos ni autorización legal activa. La necesidad de consentimiento, su texto, granularidad y evidencia deben definirse por finalidad y jurisdicción con abogado competente.

Esta guía se vincula con [política legal](LEGAL_JURISDICTIONS.md), [inventario de tratamientos](DATA_PROCESSING_REGISTER.md), [cookies](COOKIE_POLICY.md), [marketing](MARKETING_COMPLIANCE.md) y [PDLM](../rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md).

Los borradores legales y los requisitos identificados se preparan desde ahora como contenido versionado para las páginas de registro de operadores de tenant y de clientes de cada establecimiento, así como para las operaciones comerciales que correspondan. Pueden integrarse en entornos no productivos claramente identificados como borrador. Antes de aceptar personas o datos reales, el texto aplicable debe pasar revisión profesional y los controles del lanzamiento. Las páginas de inicio de sesión mantienen acceso a los avisos vigentes, pero no constituyen una nueva aceptación por cada login.

## Decisiones de diseño

- Separar términos/contrato, aviso de privacidad, consentimiento para finalidades que lo requieran, cookies no esenciales, analítica, marketing por canal, personalización y permisos de notificación.
- En el registro de operadores de tenant, presentar el aviso de privacidad y los términos aplicables al uso de cuenta; los términos SaaS se presentan al contratar una oferta comercial, no como aceptación automática del alta `onboarding_pending`. En el registro de clientes del establecimiento, presentar los avisos y términos aplicables a esa relación. Mantener enlaces accesibles a las versiones vigentes en ambos flujos de inicio de sesión.
- No inferir aceptación desde la creación de cuenta, una política general, el silencio, un consentimiento de otra finalidad o un control premarcado.
- Presentar opciones granulares y comprensibles; no condicionar una función necesaria a aceptar marketing no requerido.
- Registrar solo evidencia mínima: actor/referencia, finalidad, jurisdicción aplicada, versión de texto/política, estado, instante, canal y prueba de acción. No usar IP como evidencia universal sin justificación y aprobación.
- Mantener historia de aceptación y retirada; no reescribir evidencia histórica cuando cambia una política.
- Respetar retirada y preferencias aguas abajo mediante contratos idempotentes. La retirada detiene tratamientos dependientes, pero no equivale automáticamente a borrado de datos conservables bajo otra base validada.
- Revalidar si cambia finalidad, receptor, categoría de datos o alcance de proveedor; bloquear cambio material hasta aviso/aprobación requeridos.
- Diseñar evidencia separada por acto y actor: aceptación contractual cuando corresponda, constancia de aviso presentado/reconocido si aplica y opt-ins por finalidad/canal. Un login no es por sí mismo aceptación ni renovación de consentimiento.

## Categorías que requieren análisis

| Categoría | Owner por finalidad | Estado |
|---|---|---|
| Acceso y registro de operador/tenant | PLATFORM posee los términos de acceso/SaaS según el acto; CORE posee identidad y evidencia técnica autorizada | Preparar texto versionado y evidencia de la acción contractual pertinente; el onboarding `onboarding_pending` no implica aceptar plan, precio, trial o cobro. |
| Cuenta de cliente del establecimiento | BUSINESS define el contexto de la relación; CORE posee identidad global | Preparar el aviso y términos aplicables por tenant/actividad; no inferir aceptación de marketing ni de finalidades opcionales desde el registro. |
| Contrato/términos de ENSAMBRIA | PLATFORM; CORE gestiona identidad según contrato | Versionado y presentación por flujo; la versión aplicable y mecanismo de evidencia requieren revisión antes del uso público. |
| Marketing del establecimiento | BUSINESS | Opt-in/opt-out, base jurídica, canales y reglas de contacto por mercado pendientes de Legal. |
| Marketing institucional SaaS | PLATFORM | No activar hasta definir responsable, proveedor, base y retiro. |
| Cookies/trackers no esenciales | Owner del frontend y proveedor tecnológico | No existe inventario runtime; consentimiento por mercado por validar. |
| Analítica, personalización y replay | Owner de la app; Sentry obligatorio por regla frontend sujeto a privacidad | Replay y captura de contenido deshabilitados por defecto; no habilitar sin evaluación. |
| Imagen, reseña y publicación | BUSINESS produce; PUBLIC publica proyección autorizada | Permiso de publicación/uso y retirada requieren condiciones aprobadas. |

## Casos de prueba

Probar rechazo y aceptación separados; ausencia de trackers antes de consentimiento cuando aplique; retirada; expiración/revocación de sesión; consentimiento desactualizado; duplicados/concurrencia; cambios de versión; datos no autorizados; fallos de propagación; notificaciones y evidencia redactadas. Las pruebas deben observar tanto el backend como los requests de red y no depender solo de ocultar una interfaz.

La matriz de campos, textos, períodos, base y obligación se mantiene en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md). Hasta que se apruebe, no hay consentimiento activo o cumplimiento declarado.
