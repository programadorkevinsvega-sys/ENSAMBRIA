# Diseño inicial de cookies y tecnologías similares

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** inventario y reglas preliminares de Fase 0. No hay frontends desplegados ni un escaneo de cookies runtime; este documento no afirma que el sitio actual use cookies ni constituye un aviso público aprobado. Los requisitos de consentimiento y textos dependen de jurisdicción y revisión legal.

## Principios

1. Antes de liberar cada frontend, inventariar cookies, local storage, SDKs, píxeles, etiquetas, identificadores, recursos externos y solicitudes de red por ruta y entorno.
2. Describir finalidad, proveedor, dato, duración/expiración, dominio, acceso y mecanismo de rechazo/eliminación; no asumir que el nombre de una tecnología determina su clasificación legal.
3. No cargar tecnología no esencial antes de la preferencia requerida en el mercado aplicable. Permitir rechazar con claridad y cambiar la decisión después.
4. No usar almacenamiento local como autoridad de autorización; backend valida identidad, permisos y tenant.
5. Minimizar identificadores, limitar duración y proveedores; no incluir credenciales, tokens ni datos sensibles en cookies no estrictamente necesarias.
6. Separar configuración de cookies/trackers por superficie, mercado y entorno. El locale o IP no determina por sí solo la jurisdicción.
7. Probar que una preferencia se respeta en HTML inicial, scripts, redirecciones, SDK y solicitudes externas.

## Inventario actual

| Elemento | Estado verificado |
|---|---|
| ENSAMBRIA Business, Tenant Portal, ENSAMBRIA App, Marketplace, Platform Admin y Corporate | No hay aplicaciones/rutas ejecutables para escanear. |
| Cookies necesarias, autenticación, preferencias | Sin implementación ni duración aprobada. |
| Analítica, publicidad, replay, heatmaps | No se verificó integración runtime; no habilitar por defecto. |
| Recursos externos | Proveedor por superficie no inventariado. Evaluar antes de solicitar/cargar fuentes, mapas, videos o widgets externos. |
| Aviso/banner de preferencias | No implementado; texto y comportamiento requieren revisión legal. |

## Aceptación antes de publicar una ruta

El owner del frontend conserva inventario automatizado/manual por ruta, proveedor, finalidad y jurisdicción; una opción de rechazo efectiva; ausencia de trackers no esenciales hasta la condición aplicable; retiro de preferencia; política de privacidad vinculada; pruebas de privacidad/red y accesibilidad. La clasificación de una cookie, el plazo, la obligación de banner y el texto final se ratifican en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md) y [consentimiento](CONSENT_MANAGEMENT.md).
