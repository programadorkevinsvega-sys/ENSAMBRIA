# Facturación del establecimiento

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/establishment-billing/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 7, con facturación electrónica e integraciones.

## Propósito y alcance

Gestionar documentos fiscales asociados a ventas del establecimiento, incluyendo documento, líneas, estados y referencias a un proveedor autorizado. Separa la facturación del establecimiento de la facturación SaaS, que pertenece a PLATFORM.

Factus es el proveedor inicial previsto, no exclusivo. El diseño no afirma conformidad con DIAN ni habilita emisión; emisor, capacidades oficiales y requisitos aplicables deben validarse antes de activar la funcionalidad.

## Operaciones previstas

Solicitar la creación de un documento, consultar su estado y asociar una corrección autorizada. El adapter de facturación será neutral respecto del proveedor; las respuestas reflejarán el estado validado, no una promesa de conformidad fiscal.

## Límites e integraciones

- Consume ventas confirmadas de Commerce por contrato.
- Puede referenciar operaciones financieras de CORE; CORE conserva el ownership de pagos.
- Factus es el adapter inicial previsto; otros proveedores pueden agregarse sin acoplar las reglas del dominio. Ningún adapter se habilita sin verificación oficial, contractual y de los requisitos aplicables.
- No comparte documentos ni reglas con la facturación SaaS de PLATFORM.

## Seguridad e integridad

La autorización requiere tenant, sucursal y rol financiero. Se minimizan datos personales y fiscales y se auditan los accesos. La idempotencia por operación/documento y los estados externos deben impedir documentos duplicados y sobrescrituras por reintentos.

La aceptación futura debe cubrir duplicados, rechazo o timeout del proveedor, consulta posterior, cancelación, tenant incorrecto, datos incompletos y no emitir antes de validar los requisitos aplicables.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay proveedor, contrato ejecutable, manifiesto ni pruebas ejecutables; no hay comandos de emisión o ejecución disponibles.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [ADR-0009: proveedores iniciales y reventa Factus](../../../../docs/adr/ADR-0009-proveedores-iniciales-y-reventa-factus.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
