# Diseño inicial de solicitudes de titulares

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** diseño documental Fase 0; no existe un canal o workflow de solicitudes implementado. No constituye aviso legal ni fija plazos. Los tipos de derecho, identidad necesaria, calendarios, excepciones y comunicaciones deben ser confirmados por asesoría competente para cada tratamiento/jurisdicción antes de habilitar el proceso.

Esta especificación concreta la interacción entre la [política legal](LEGAL_JURISDICTIONS.md), [PDLM](../rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md), el [registro de tratamientos](DATA_PROCESSING_REGISTER.md) y [retención](DATA_RETENTION.md).

## Ownership y límites

- CORE puede autenticar al solicitante, comprobar autoridad/representación, registrar la solicitud y coordinar estado y plazo aprobado.
- Cada backend consulta, exporta, corrige, restringe, elimina o atestigua únicamente los datos de los que es propietario. CORE no lee ni modifica tablas internas ajenas.
- Un tenant puede ser responsable de datos de sus clientes finales; no se ejecuta una acción sobre esa información sin identificar el rol de ENSAMBRIA y la instrucción o base aplicable.
- La solicitud de privacidad no concede permisos generales a quien la presenta ni cambia la autorización ordinaria de un usuario.
- Las excepciones de retención/hold requieren decisión y alcance validados; no deben generar denegaciones silenciosas ni conservación ilimitada.

## Flujo propuesto

1. Recibir solicitud por un canal público/privado que Legal y Producto aprueben; ese canal y datos de contacto aún no están definidos.
2. Crear referencia opaca e idempotente, fecha de recepción, actor/canal, ámbito y jurisdicción provisional; minimizar contenido libre.
3. Verificar identidad y representación de manera proporcional sin revelar si una persona o dato existe antes de validar autorización.
4. Resolver finalidad, owner, relación responsable/encargado, alcance, conflictos, dependencias y políticas vigentes.
5. Si es válida, CORE coordina instrucciones autenticadas/versionadas; cada dominio ejecuta solo sobre su almacenamiento y registra estado local.
6. Para cambios persistentes entre dominios, usar Outbox/Inbox durable, idempotencia y ACK posterior al commit. No tratar NATS como RPC ni emitir hechos ficticios.
7. Gestionar validación, reintentos, fallo parcial, excepción/legal hold y escalamiento con errores seguros y estados explícitos.
8. Verificar el resultado y derivados; preparar respuesta aprobada y registrar evidencia minimizada.
9. Cerrar solo cuando cada owner aplicable haya respondido o la excepción/bloqueo esté justificado y comunicado según política validada.

## Casos de prueba de aceptación

| Escenario | Resultado esperado |
|---|---|
| Solicitante no verificado o tercero sin representación | No revelar existencia de cuenta/datos ni ejecutar acción. |
| Solicitante autorizado con datos en varios dominios | CORE coordina; cada owner responde por su dato y scope, sin consulta SQL cruzada. |
| Solicitud duplicada/concurrente | Idempotencia evita procesos o efectos duplicados; estado y auditoría permanecen coherentes. |
| Fallo tras persistir solicitud y antes de emitir instrucción | Outbox permite recuperar; no hay instrucción perdida ni efecto parcial. |
| Owner/proveedor no disponible | Estado diagnóstico y reintento/escalamiento explícito; no se declara completado. |
| Dato fuera de tenant/scope o actor sin permiso | Rechazo sin efecto y con diagnóstico seguro correlacionado. |
| Hay obligación de retención/hold pendiente de validación | No ejecutar borrado irreversible; escalar y comunicar solo conforme al procedimiento aprobado. |
| Error al responder o expirar el canal | No perder el resultado; controlar reenvío y no exponer datos a canal no verificado. |

Las pruebas futuras deben usar PostgreSQL, JetStream y proveedores oficiales/sandbox reales donde correspondan. No usar datos de producción, stubs para aceptación ni APIs de borrado sin dictamen. Plazos, canales, templates y contactos se mantienen bloqueados hasta completar [la matriz legal](LEGAL_JURISDICTIONS.md).
