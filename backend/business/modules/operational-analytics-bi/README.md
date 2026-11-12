# Analítica operativa y BI

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/operational-analytics-bi/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 8.

## Propósito y alcance

Producir indicadores operativos y de rentabilidad autorizados, con definiciones y frescura explícitas. Las métricas son proyecciones derivadas: este contexto no reemplaza ni se convierte en fuente de verdad de los agregados de dominio.

BUSINESS es dueño de los indicadores operativos. Los módulos fuente conservan sus propios datos, reglas y permisos.

## Operaciones previstas

Actualizar proyecciones usando hechos autorizados y consultar métricas con filtro y cursor. Cada contrato de métrica debe especificar definición, unidad, origen, ventana y frescura.

## Límites e integraciones

- Consume solo eventos o contratos autorizados de los owners fuente necesarios.
- No consulta tablas ajenas ni ejecuta analítica en el camino crítico de operaciones.
- No inventa eventos para capturar métricas; los hechos y consumidores se justifican antes de implementarse.

## Seguridad e integridad

El scope del actor, tenant y sucursal debe aplicarse a las métricas. Los datos se minimizan, especialmente cuando un agregado pueda permitir reidentificación. El reprocesamiento requiere idempotencia y watermarks; los resultados deben poder reconciliarse con las fuentes sin contar dos veces tras replay.

La aceptación futura debe cubrir rangos vacíos y límites, retraso, huecos, duplicados, tenant ajeno y discrepancias entre fuente e indicador.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contratos de métricas ejecutables ni suite de pruebas; no se declara ningún indicador disponible.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
