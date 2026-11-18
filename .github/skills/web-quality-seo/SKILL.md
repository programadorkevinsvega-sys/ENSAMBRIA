---
name: web-quality-seo
description: "Use when creating or changing a website, frontend route, public content, metadata, Open Graph previews, accessibility, responsive behavior, performance, indexing, sitemap, or web launch readiness."
---

# Web Quality and SEO

## Objetivo

Coordinar la calidad de la experiencia web y, cuando corresponda, el SEO público usando cada estándar en su ámbito propietario.

## Alcance

Frontends web, navegación, formularios, accesibilidad, rendimiento, rutas públicas/privadas, metadatos, indexación y publicación.

## Cuándo activarla

- Obligatoria para una nueva experiencia web o cambios en rutas, formularios, contenido o lanzamiento.
- En todo cambio frontend, aplica también los controles de seguridad, accesibilidad, UX/UI, rendimiento, observabilidad, pruebas y móvil de la guía cross-cutting enlazada en las referencias normativas.
- Consulta el estándar k6 cuando el cambio afecte rendimiento, carga o capacidad de una superficie frontend; usa escenarios de navegador o protocolo según el objetivo medido.
- Activa las validaciones SEO/Open Graph cuando la página o contenido sea público o se comparta públicamente.
- Activa también `security-and-privacy` si la experiencia trata datos personales, cookies, consentimiento o acceso geográfico.

## Referencias normativas obligatorias

- [Website Launch, SEO, Performance and Accessibility](../../../docs/rules/ENSAMBRIA_Website_Launch_SEO_Performance_Accessibility.md) para calidad web general.
- [Frontend Security, Accessibility, UX/UI, Performance and Observability](../../../docs/rules/ENSAMBRIA_frontend_security_accessibility_ux_ui_performance_observability_rules.md) para seguridad, privacidad de cliente, accesibilidad, UX/UI, rendimiento, observabilidad, pruebas y controles móviles.
- [k6 Performance, Load, Stress and Capacity Testing](../../../docs/rules/ENSAMBRIA_k6_performance_load_stress_and_capacity_testing_rules.md) para pruebas de carga de frontend, pruebas de navegador y métricas web aplicables.
- [Open Graph SEO Metadata](../../../docs/rules/ENSAMBRIA_Open_Graph_SEO_Metadata.md) para metadatos y descubribilidad pública.
- [Contrato integrado de arquitectura, políticas y responsabilidades de ENSAMBRIA](../../../ARCHITECTURE_RULES.md), para ownership y límites de publicación.

## Aplicación obligatoria en ENSAMBRIA

Antes de cambiar una experiencia o ruta:

1. Confirma fase, owner de frontend/BFF/datos, contrato y clasificación pública/privada; lee [AGENTS.md](../../../AGENTS.md), [ARCHITECTURE_RULES.md](../../../ARCHITECTURE_RULES.md), cada fuente web aplicable completa, criterios de producto y contratos/ADR relacionados.
2. Traduce las reglas en cambios de UI/HTML/BFF y aceptación verificable: acceso y tenant, privacidad, navegación/accesibilidad, SEO/metadata inicial, observabilidad (incluido Sentry cuando lo exige la guía) y rendimiento/k6 cuando aplique.
3. Prueba rutas públicas y privadas y la frontera HTTP/BFF/backend real; `noindex` no sustituye autorización. No expongas datos que otro owner no autorizó para publicar.
4. Informa archivos, rutas, checks/comandos, resultados, evidencia y límites. Separa **REQUERIDO, DISEÑADO, IMPLEMENTADO, VALIDADO, APROBADO y BLOQUEADO** y solicita resolución ante conflictos o decisiones pendientes.

## Procedimiento operativo

1. Inspecciona frontend, rutas, contenido, componentes y pruebas existentes; identifica ownership y naturaleza pública, autenticada, privada o borrador.
2. Define en la fase las validaciones de UX, seguridad, accesibilidad y rendimiento pertinentes; para contenido público, determina metadatos, indexación y previews.
3. Implementa en el frontend y el BFF/contrato autorizados, respetando ownership; no expongas páginas privadas por conveniencia SEO.
4. Prueba el HTML inicial y las respuestas reales para páginas públicas; prueba autorización y ausencia de exposición/indexación para rutas privadas.
5. Ejecuta validaciones automatizadas y manuales ya definidas por las fuentes, reportando su alcance y límites.

## Restricciones

- El estándar web general no sustituye privacidad/legal ni la guía especializada de SEO/Open Graph.
- `noindex`/`robots.txt` no son controles de acceso; exige autenticación/autorización apropiada.
- No publiques datos privados, de borrador o tenant ajeno en HTML, sitemap, datos estructurados, previews o cachés compartidas.
- No afirmes accesibilidad, rendimiento o SEO aceptados por tener metadata o pasar un único chequeo.

## Validaciones obligatorias

- Revisa enlaces, estados de formulario/error, navegación, responsive, accesibilidad y rendimiento según la guía web.
- Para páginas públicas, valida HTML inicial, metadata/canonical/preview, datos estructurados, sitemap y robots cuando apliquen.
- Para rutas privadas, comprueba controles de acceso y exclusión de indexación/datos públicos; ejecuta pruebas de aislamiento donde corresponda.

## Criterios de finalización

Los estándares general y especializado se cumplen en sus ámbitos, privacidad/ownership se preservan y cada validación reclamada tiene evidencia ejecutada.

## Conflictos y excepciones

Si una ruta mezcla contenido público y privado, divide la superficie o resuelve explícitamente el contrato antes de publicar. Ante conflicto entre calidad web, privacidad y SEO, detén el aspecto afectado hasta resolverlo con sus owners.

## Mantenimiento

Revisada el 2026-11-10. Revisa las guías general y especializada en conjunto cuando cambien rutas o criterios; conserva la autoridad separada de cada una y no inventes versiones.
