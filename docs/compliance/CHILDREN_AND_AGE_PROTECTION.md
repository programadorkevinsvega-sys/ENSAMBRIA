# Diseño inicial de protección de menores y edad

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** evaluación preliminar de Fase 0; no hay edad mínima global, verificación de edad, flujo parental o tratamiento activo aprobado. La edad de elegibilidad, consentimiento/representación, necesidad de datos y salvaguardas se determinan por servicio y jurisdicción con abogado competente; no inferir un número universal de la UI.

Esta evaluación se relaciona con [registro de tratamientos](DATA_PROCESSING_REGISTER.md), [consentimiento](CONSENT_MANAGEMENT.md) y [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md).

## Decisiones de diseño

- Distinguir a los menores como usuarios con cuenta, clientes finales de un establecimiento, sujetos de contenido/fotografías o personas mencionadas en datos de terceros.
- No pedir fecha de nacimiento completa si no existe una necesidad demostrada; escoger el dato menos intrusivo después de validar la obligación.
- No asumir que una casilla de edad demuestra edad o consentimiento parental.
- Antes de permitir uso por menores, decidir mercado, producto/función, rol de cada responsable, base, representación, verificación proporcional, avisos, retención y datos que se prohíben.
- No habilitar perfilado, marketing, publicidad, replay, analítica no esencial ni publicación de imagen de un menor hasta evaluación y autorización específica aplicables.
- Evitar que notas, fotos o campos libres conviertan un flujo ordinario en un registro no previsto de datos de menores.
- Si no puede verificarse una obligación crítica o una autoridad parental, deshabilitar la función afectada en vez de aceptar datos sin control.

## Supuestos y bloqueos actuales

No hay evidencia de usuarios, cuentas, tratamientos o datos de menores. Los establecimientos podrían atender públicos de distintas edades; eso no autoriza a ENSAMBRIA a recopilar edad o información sensible. El alcance real de clientes menores, consentimiento, imágenes y servicios debe confirmarse con Producto y Legal antes de crear formularios o perfiles.

## Pruebas futuras

Cubrir edad ausente/inválida/límite según política aprobada; flujo de representante y poder insuficiente; rechazo del tratamiento no elegible; revocación; acceso y eliminación; ausencia de marketing/tracking no autorizado; exposición de edad/identidad en HTML, logs, Sentry, eventos, exportaciones y búsqueda pública. Usar cuentas transitorias sintéticas en entorno aislado y flujos reales, no datos de menores.

La normativa oficial, aplicabilidad, límites y aprobaciones se registran en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md). Hasta contar con esa validación, no se declara que la plataforma esté habilitada para menores.
