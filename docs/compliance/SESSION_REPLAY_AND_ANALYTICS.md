# Control de analítica, Sentry y grabación de sesiones

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** diseño documental; no hay apps ni SDKs instalados. Sentry es obligatorio para los frontends según la regla normativa, pero cada proyecto, ubicación, acceso, retención y configuración debe aprobarse antes de integrar. Session Replay y captura de contenido quedan deshabilitados por defecto; no existe aprobación de privacidad/legal ni configuración activa.

Esta guía se vincula con [registro de tratamientos](DATA_PROCESSING_REGISTER.md), [consentimiento](CONSENT_MANAGEMENT.md), [cookies](COOKIE_POLICY.md), [proveedores](THIRD_PARTY_PROCESSORS.md) y [observabilidad](../rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md).

## Límites de captura

- Capturar errores, releases y métricas de rendimiento con atributos técnicos mínimos; separar projects/entornos por superficie según plan operativo.
- No enviar contraseñas, tokens, cookies, URL firmadas, documentos, teléfonos, emails, texto libre, contenido de cliente, datos financieros completos ni datos de salud.
- Redactar parámetros de URL y cuerpos por defecto; agregar un campo solo con finalidad y clasificación documentadas.
- No habilitar replay, grabación de formularios, autocapture o trazado de pulsaciones por defecto. Cualquier excepción requiere evaluación de necesidad, base/consentimiento aplicable, masking probado, retención, acceso y proveedor.
- Separar DSN público del token administrativo de Sentry. El token de upload/source maps se guarda como secreto restringido de CI y no en el bundle.
- Aplicar muestreo y límites de cardinalidad para reducir identificación, costos y riesgo; no sacrificar registros operativos locales si exportación falla.
- Configurar control de acceso, MFA, auditoría, retención, región y procedimiento de incidente del proveedor antes de enviar datos.

## Matriz de superficie

| Superficie | Cobertura futura | Estado/bloqueo |
|---|---|---|
| ENSAMBRIA Business (Flutter) | Errores/rendimiento de UI y correlación con BFF; atributos de plataforma estrictamente necesarios, con configuraciones separadas por destino móvil/escritorio cuando corresponda. | Sin app; identidad de proyecto y acceso pendientes. No contactos, ubicación precisa, advertising IDs ni replay por defecto. |
| ENSAMBRIA Tenant Portal (Next.js) | Error cliente/SSR, release y rendimiento seguro; revisar rutas privadas y públicas separadamente. | Obligatorio Sentry; no runtime ni proyecto configurado. |
| ENSAMBRIA App (Flutter) | Errores y releases; atributos de dispositivo estrictamente necesarios. | Sin app; no contactos, ubicación precisa, advertising IDs ni replay por defecto. |
| ENSAMBRIA Marketplace (Next.js) | Errores/rendimiento de HTML/SSR; rutas públicas de baja cardinalidad, sin búsqueda identificable ni reseñas privadas. | Sin app; no capturar contenido privado. |
| ENSAMBRIA Platform Admin (Next.js) | Errores/rendimiento de operaciones administrativas con campos mínimos. | Sin app; minimizar datos de soporte y acceso. |
| ENSAMBRIA Corporate (Next.js) | Errores/rendimiento de páginas institucionales públicas; evitar datos de visitantes identificables. | Sin app; no habilitar tracking, replay ni captura de formularios por defecto. |
| BFF/backend | Logs/trazas y métricas según sus estándares; cualquier exportación Sentry backend requiere decisión específica. | No afirmar Sentry backend obligatorio por la regla frontend; respetar observabilidad normativa. |

## Pruebas de aceptación antes de habilitar

1. Enviar valores canario no sensibles por formulario de prueba y demostrar que no aparecen en evento, URL, breadcrumb, log, source map público ni replay.
2. Confirmar que fallos de exportación no fallan ni bloquean una operación de negocio y que el diagnóstico local sigue disponible.
3. Verificar correlación/release sin IDs sensibles ni alta cardinalidad.
4. Probar acceso por rol, retención/borrado y ambiente de destino; validar proveedores/subprocesadores y residencia.
5. Si se propone replay o analítica no esencial, probar consentimiento/opt-out y que campos privados no se capturan.

El inventario real de tratamiento, proveedor y transferencia debe anotarse en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md) y [THIRD_PARTY_PROCESSORS.md](THIRD_PARTY_PROCESSORS.md). No enviar datos de usuarios hasta completar las aprobaciones.
