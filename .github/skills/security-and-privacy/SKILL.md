---
name: security-and-privacy
description: "Use when a change affects authentication, authorization, tenant isolation, personal or sensitive data, consent, retention, privacy rights, providers, incidents, or geographic access."
---

# Security and Privacy

## Objetivo

Activar los controles de seguridad, privacidad, ciclo de vida y acceso geográfico pertinentes sin convertir reglas legales particulares en obligaciones universales.

## Alcance

Autenticación/autorización, aislamiento tenant/sucursal, auditoría de seguridad y negocio, datos personales, tratamiento, consentimiento, retención, derechos, transferencias, proveedores, incidentes y restricciones territoriales.

## Cuándo activarla

- Obligatoria cuando el cambio toca seguridad, datos personales/sensibles, acceso geográfico, privacidad o un proveedor con acceso a datos.
- Desde Fase 0, participa en la matriz de amenazas, controles y aceptación para multi-tenancy, multi-sucursal, auditoría, APIs/webhooks y extensiones de terceros; su diseño no habilita runtime ni sustituye revisión legal.
- La revisión legal se activa según jurisdicción, mercado, propósito y tratamiento; no basta con una decisión técnica.
- La falta de revisión profesional no bloquea investigación, redacción o integración técnica general. Registra la revisión como estado separado y bloquea solo el tratamiento que requiera esa validación previa específica.

## Referencias normativas obligatorias

- [ADR-0012 — modular boundaries and architecture compliance](../../../docs/adr/ADR-0012-fronteras-modulares-y-cumplimiento-arquitectonico.md) (consult when the task changes module ownership, data access, or release acceptance)

- [Geographic Access Policies ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Geographic_Access_Policies.md)
- [Privacy Legal Compliance ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Privacy_Legal_Compliance.md)
- [Privacy Data Lifecycle Management ENSAMBRIA](../../../docs/rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md)
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md)
- [Requisitos y roadmap ENSAMBRIA](../../../docs/requerimiento.md), para identificar dominio y alcance pertinentes.

Al revisar esta Skill, verifica en la fuente PDLM su versión y estado actuales. La versión 2.0 recibió aprobación técnica/documental del propietario el 2026-11-11; esa aceptación no certifica cumplimiento legal ni aprueba obligaciones, tratamientos o textos concretos.

## Aplicación obligatoria en ENSAMBRIA

Antes de tratar datos o habilitar una superficie:

1. Confirma fase, propósito, owner y alcance de tratamiento; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), los requisitos afectados, reglas completas de privacidad/geografía y ADR/contratos relacionados.
2. Traza datos, actores, jurisdicción, acceso y sistemas derivados a controles concretos de autorización/minimización/retención/auditoría, escenarios negativos y evidencia. Comprueba quién debe aprobar cada obligación; no sustituyas revisión jurídica por criterio técnico.
3. Mantén bloqueado o deshabilitado solo el tratamiento/mercado que dependa de decisión crítica pendiente y comunica owner, fundamento, riesgo y prueba requerida. No inventes N/A ni asumas la aprobación de propuestas.
4. Informa **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO** separadamente; reporta controles y pruebas realmente ejecutados, decisiones legales pendientes y límites de la evaluación.

## Procedimiento operativo

1. Clasifica los datos, actores, propósito, flujos, sistemas derivados, proveedores y jurisdicciones realmente afectados.
2. Identifica el backend dueño, contexto tenant/sucursal, controles de autorización/aislamiento, minimización, retención, auditoría y evidencia requerida. Comprueba identidad, rol y scope por cada acción; no infieras permisos de IDs, jerarquía o tenant enviado por cliente.
3. Determina si hay obligaciones que requieran validación de Legal/Privacy Owner; consulta fuentes oficiales y registra bloqueos, no supongas interpretaciones.
4. Si hay georrestricción, distingue navegación, solicitudes informativas/privacidad, registro y operaciones comerciales; define verificación proporcional al riesgo.
5. Implementa y prueba controles en el mismo cambio que introduce el tratamiento; documenta el flujo y las excepciones antes de habilitarlo.

## Restricciones

- No presentes una guía técnica como asesoría o aprobación legal. Leyes, bases, roles y plazos dependen de la jurisdicción y del tratamiento.
- No bloquees indiscriminadamente el producto por país ni trates una IP como prueba de identidad, residencia o jurisdicción.
- No registres datos personales, credenciales, cookies, IP cruda ni contenido sensible sin base y protección aprobadas.
- No marques como aplicada una obligación pendiente de validación ni conviertas una propuesta/ADR pendiente en decisión aprobada.

## Validaciones obligatorias

- Verifica autorización backend, aislamiento tenant, minimización, redacción, ciclo de vida y propagación a sistemas derivados aplicables.
- Para auditoría, verifica atribución de actor/workload, scope, resultado, integridad, acceso y retención; minimiza ubicación y no registra IP cruda por defecto.
- Para integraciones públicas/terceros, evalúa scopes mínimos, exposición de datos, revocación, replay, abuso, límites y separación de tenants/sucursales.
- Para restricciones geográficas, prueba incertidumbre de ubicación, excepciones, precedencia, límites y disponibilidad de rutas informativas/privacidad según la fuente.
- Para derechos y retención, valida estados, duplicados/reintentos, holds, fallos de proveedores, restauración y evidencia de resultado cuando aplique.

## Criterios de finalización

Los controles aplicables están identificados, probados y documentados; cualquier obligación legal crítica sin validar mantiene deshabilitado solo el alcance dependiente y queda reportada.

## Conflictos y excepciones

Detén el tratamiento o mercado dependiente si falta una decisión legal crítica. Resuelve conflictos entre privacidad, geografía, arquitectura o comportamiento con los owners correspondientes; no infieras una excepción legal.

## Mantenimiento

Revisada el 2026-11-11. Revisa las tres fuentes en conjunto cuando cambie tratamiento, alcance jurisdiccional o ciclo de vida. No inventes versiones; conserva las fechas/versiones que realmente indiquen los documentos.
