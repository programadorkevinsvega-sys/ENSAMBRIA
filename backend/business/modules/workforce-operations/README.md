# Workforce y operación del personal

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/workforce-operations/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** operación laboral, turnos, asistencia y comisiones en Fase 4; portal y vistas privadas de personal en Fase 5.

## Propósito y alcance

Gestionar perfiles operativos, asignaciones, turnos y recursos humanos del establecimiento, manteniendo la disponibilidad coordinada con reservas sin exponer información laboral a clientes.

El contexto es dueño de la información operativa del trabajador, sus asignaciones y turnos. No crea identidades, membresías globales ni autenticación, y no es dueño del ledger ni de las reservas comunes.

## Operaciones previstas

Invitar o asignar personal, retirar asignaciones, actualizar turnos y consultar disponibilidad y permisos operativos. Las solicitudes referencian al miembro de CORE y a una sucursal; las respuestas contienen únicamente información autorizada y referencias no credenciales.

## Límites e integraciones

- CORE valida identidad, membresía y contexto de seguridad.
- La coordinación con reservas se realiza por contrato.
- Los cambios relevantes para una reserva se integran únicamente cuando exista contrato y necesidad aprobados.
- No publica datos laborales en PUBLIC, no crea sesiones y no accede al ledger.

## Seguridad e integridad

El acceso requiere tenant, sucursal, rol y autorización por acción. La información laboral es privada y la administración debe ser auditable. Cambios de asignación y turno deben detectar solapamientos y carreras; BUSINESS no sustituye la autoridad de identidad de CORE.

La aceptación futura debe incluir rol insuficiente, baja concurrente, doble asignación o turno, acceso a otra sucursal o tenant, sesión expirada y conflictos con reservas.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni suite de pruebas, así que no se publican comandos ni se afirma validación.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
