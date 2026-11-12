# Reservas y disponibilidad común

## Identificación y estado

- **Tipo:** bounded context de CORE.
- **Ubicación:** `backend/core/modules/reservations-availability/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** núcleo en Fase 1; los verticales amplían sus reglas en las fases correspondientes.

## Propósito y responsabilidad

Mantener el estado central y ciclo de vida de una reserva, evitando confirmaciones duplicadas y desacuerdos entre canales. CORE es propietario de la reserva; BUSINESS posee los recursos, horarios y catálogo operativo.

## Operaciones previstas

Crear, confirmar, cancelar o reprogramar reservas; consultar disponibilidad y estado según autorización. Las solicitudes identifican opciones, versiones y scope; las respuestas deben distinguir conflictos y otros errores tipados.

## Límites e integraciones

- Consulta disponibilidad/oferta de BUSINESS mediante contrato síncrono cuando se necesite respuesta inmediata.
- No lee agendas ni tablas de BUSINESS directamente.
- Los hechos confirmados usan Outbox solo cuando exista un consumidor durable real.

## Seguridad e integridad

Valida cliente o miembro, tenant, sucursal y permiso; no confía en IDs ni disponibilidad obsoleta. La confirmación debe ser exclusiva mediante transacción y control de concurrencia; los reintentos requieren idempotencia. Una reserva no implica cobro.

La aceptación futura cubre slots ocupados y límites, concurrencia simultánea, duplicados, permisos, cancelación, timeout de BUSINESS y ausencia de efectos financieros parciales.

## Implementación y pruebas

El directorio contiene solo estructura de capas. No hay contratos ni pruebas ejecutables; no se declara integración validada.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
