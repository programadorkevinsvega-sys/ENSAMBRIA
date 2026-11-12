# Comisiones de empleados

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/employee-commissions/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 4, conforme al roadmap operacional.

## Propósito y alcance

Calcular y mantener evidencia de las comisiones derivadas de la operación del establecimiento, bajo reglas aprobadas por ese establecimiento. BUSINESS conserva este dominio; no se transfieren las comisiones laborales a la plataforma SaaS.

El contexto no define porcentajes ni reglas comerciales en esta etapa. CORE no es owner del cálculo y PLATFORM no administra estas comisiones.

## Operaciones previstas

Configurar reglas versionadas, calcular resultados, revisar y cerrar períodos, y consultar evidencia. Los cálculos referencian ventas o reservas de BUSINESS y la versión de regla aplicada. Un desembolso, si se autoriza, requiere un contrato financiero con CORE.

## Límites e integraciones

- Consume hechos confirmados de Commerce/ventas y Workforce mediante contratos.
- CORE puede intervenir únicamente para un movimiento de fondos autorizado.
- No accede a tablas ajenas ni asigna políticas de compensación por defecto.

## Seguridad e integridad

El acceso está limitado por tenant, sucursal y rol; la información de compensación requiere acceso restringido y auditoría. Los cálculos deben ser reproducibles por versión, y el cierre idempotente. Las correcciones retroactivas deben ser explícitas, no silenciosas.

La aceptación futura debe cubrir límites y redondeos según política aprobada, duplicados, cierres simultáneos, reversión de ventas, accesos no autorizados y conciliación. No se fija aquí porcentaje ni regla de cálculo.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni pruebas ejecutables; no se documentan comandos de ejecución.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
