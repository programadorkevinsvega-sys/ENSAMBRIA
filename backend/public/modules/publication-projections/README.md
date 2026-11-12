# Publicación y proyecciones

## Identificación y estado

- **Tipo:** bounded context de PUBLIC.
- **Ubicación:** `backend/public/modules/publication-projections/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fases previstas:** primer contenido web tenant/proyecciones en Fase 5; directorio, Marketplace y fichas públicas en Fase 9.

## Propósito y responsabilidad

Mantener proyecciones públicas con los campos que el owner fuente haya autorizado. PUBLIC es dueño del listing, versión y estado de publicación proyectado; BUSINESS conserva el contenido fuente y la decisión de publicarlo.

## Operaciones previstas

Aplicar, reemplazar o retirar una proyección por referencia y versión; consultar su estado de publicación. La integración durable se concreta mediante contratos versionados cuando exista un flujo real.

## Límites e integraciones

- Consume Outbox/JetStream del owner cuando el caso requiera distribución durable.
- No consulta SQL de origen, no realiza joins ni enriquece proyecciones con datos privados.
- Valida origen, autorización y estado antes de publicar.
- Borradores y contenido revocado quedan excluidos.

## Seguridad e integridad

Se requiere idempotencia por `event_id`/versión y orden por versión del agregado; eventos antiguos no restauran estados retirados. La publicación de datos personales depende de la base y autorización aplicables.

La aceptación futura cubre duplicados, eventos fuera de orden, retiro concurrente, campos prohibidos, indisponibilidad de fuente y recuperación por cursor/snapshot.

## Implementación y pruebas

El directorio contiene solo estructura de capas. No hay consumidor, proyección ni pruebas con dependencias reales implementadas.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
