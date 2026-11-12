# Franquicias, cadenas y regalías

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/franchises-chains-royalties/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 10.

## Propósito y alcance

Gestionar la relación operativa entre cadenas o franquicias y sus sucursales, además de calcular obligaciones de regalías según reglas aprobadas. BUSINESS conserva la operación comercial y este contexto.

El contexto no define términos contractuales o porcentajes por defecto. PLATFORM no es dueño de las regalías y CORE solo participa si se requiere un movimiento financiero autorizado.

## Operaciones previstas

Crear o cambiar relaciones de cadena, consultar consolidaciones y calcular o cerrar períodos de regalías conforme a política aprobada. Las solicitudes deben especificar los scopes y las respuestas limitarse a la información autorizada.

## Límites e integraciones

- CORE proporciona IDs y contexto de seguridad; las transferencias económicas requieren su contrato financiero.
- Los datos y cálculos de otros módulos se consumen por contratos autorizados.
- No hay acceso SQL entre tenants, módulos o backends; no existe herencia implícita de permisos por pertenecer a una cadena.

## Seguridad e integridad

Se requiere autorización explícita para cada tenant y sucursal delegados. Los cálculos deben ser idempotentes por período, evitar doble conteo y conservar versiones para correcciones.

La aceptación futura debe cubrir cadena ajena, revocación de scope, sucursal duplicada, período repetido, regla sin aprobación y discrepancias de consolidación.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni suite de pruebas; no se documentan comandos disponibles ni cálculos implementados.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
