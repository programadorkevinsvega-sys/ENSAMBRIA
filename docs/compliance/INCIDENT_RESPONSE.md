# Procedimiento inicial de respuesta a incidentes

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** diseño documental preliminar de Fase 0, no runbook activo. No hay servicios o equipos on-call de ENSAMBRIA desplegados. Los roles humanos de coordinación técnica los asume actualmente el propietario del proyecto; Legal, autoridad de notificación, contactos, tiempos y obligaciones deben validarse por abogado competente y no se inventan en este documento.

Este procedimiento se complementa con [cumplimiento legal](LEGAL_JURISDICTIONS.md), [PDLM](../rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md), [inventario de tratamientos](DATA_PROCESSING_REGISTER.md) y [observabilidad](../rules/ENSAMBRIA_Observability_Monitoring_Alerting_SLO.md).

## Alcance y activación

Tratar como incidente potencial una exposición, pérdida o alteración de datos; acceso cruzado entre tenants; credencial o certificado comprometido; evento/callback falso; pago/ledger inconsistente; borrado o restauración indebida; indisponibilidad con impacto; o proveedor/subencargado afectado. Una alerta no confirma una brecha; registrar hechos observados y distinguirlos de hipótesis.

## Secuencia de respuesta

1. **Detectar y registrar:** abrir un caso con identificador opaco, hora, fuente, componente, categoría y alcance conocido. No incluir PII/secretos en tickets/logs.
2. **Triage y preservar:** limitar acceso al personal autorizado; preservar logs/metadatos con cadena de custodia y evitar acciones que destruyan evidencia.
3. **Contener:** revocar o rotar credenciales comprometidas, aislar workload, limitar tráfico o pausar el proceso afectado. No ejecutar operaciones destructivas ni tocar recursos de otros proyectos sin autorización.
4. **Evaluar alcance:** identificar datos, titulares, tenants, regiones, sistemas derivados, proveedores y disponibilidad/integridad afectados; consultar el inventario y los owners de datos.
5. **Determinar obligaciones:** Security/Privacy técnico reúne evidencia; abogado competente determina aplicabilidad, plazos, autoridades, clientes y contenido de notificación por jurisdicción/contrato. No aplicar un plazo universal.
6. **Comunicar:** informar a personas/tenants/autoridades solo con criterio y texto aprobados, canal verificado y registro de quién autorizó. Hasta esa aprobación no publicar detalles sensibles.
7. **Erradicar y recuperar:** corregir causa raíz; restaurar solo según runbook; si se trata de datos suprimidos, restaurar en cuarentena y reaplicar controles de ciclo de vida antes de admitir tráfico.
8. **Cerrar y aprender:** documentar impacto confirmado, decisiones, cronología, evidencias redactadas, medidas, riesgo residual y pruebas de no recurrencia.

## Responsabilidades actuales y pendientes

| Función | Estado |
|---|---|
| Coordinación técnica, seguridad y preservación inicial | Propietario del proyecto, en modalidad unipersonal; no equivale a revisión independiente. |
| Identificación de datos y owner del backend | El propietario asume los cuatro roles de dominio; las credenciales separadas y el aislamiento siguen siendo obligatorios. |
| Determinación legal y notificación | Abogado competente aún por contratar/designar; no hay autoridad ni contacto legal aprobado en el proyecto. |
| Contactos internos, clientes y proveedores | No hay roster de operación ni proveedores runtime aprobados; deben definirse antes de lanzamiento. |

## Escenarios de ejercicio futuro

- Token/SecretID expuesto; probar revocación y rotación sin registrar el valor.
- Lectura de datos de otro tenant; comprobar contención, auditoría, alcance y ausencia de expansión.
- Error de Sentry/log export; confirmar que operación de negocio no se bloquea y que no se filtran datos.
- JetStream/consumer comprometido o duplicado; pausar/reconstruir durable consumer y validar Inbox idempotente.
- Backup que precede a una supresión; validar cuarentena y reconciliación antes de servir tráfico.
- Proveedor de pagos o mensajería notifica incidente; conservar evidencia y enrutar decisión a Legal.

Antes de ejecutar ejercicios se definirán ambiente, autorización, límites, servicios y efectos. Ninguna prueba destructiva se dirige a producción. La matriz de obligaciones y contactos por jurisdicción se mantiene en [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md); hasta que se apruebe, no se afirma un procedimiento de notificación legalmente completo.
