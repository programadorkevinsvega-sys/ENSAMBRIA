# Operación del establecimiento, catálogo y verticales

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/establishment-catalog-verticals/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 2; las funciones comunes de reservas corresponden a Fase 1.

## Propósito y alcance

Configurar la operación del establecimiento, sus sucursales, profesionales, recursos, servicios y atributos verticales. El modelo debe admitir las diferencias operativas de los negocios de belleza y bienestar sin imponer un catálogo uniforme.

El contexto es dueño del perfil y la configuración operativa, el catálogo de servicios y los recursos. No es dueño de la identidad ni de la pertenencia de usuarios, de la reserva confirmada, de los pagos ni de la proyección pública del catálogo.

## Operaciones previstas

Crear, actualizar o retirar servicios y recursos; consultar el catálogo operativo y sus capacidades. Las entradas incluyen el scope y la versión aplicable; las respuestas exponen únicamente configuración autorizada y errores de invariantes.

## Límites e integraciones

- Valida tenant, sucursal y rol mediante el contexto de seguridad autorizado por CORE.
- Consulta reservas mediante contrato cuando un caso de uso lo requiera.
- La publicación en PUBLIC requiere una proyección independiente, autorizada y propiedad de PUBLIC; PUBLIC no lee este catálogo como fuente.
- No accede a tablas de otros módulos ni permite que CORE o PUBLIC asuman su ownership.

## Seguridad e integridad

Los datos tenant-scoped requieren aislamiento RLS y autorización explícita por tenant y sucursal. Los cambios deben manejar versiones concurrentes y reintentos sin duplicar servicios. La retirada o modificación de un servicio debe respetar las reservas existentes conforme a reglas que implementará este owner.

La aceptación futura debe cubrir límites de precios y duración, recursos de otra sucursal, duplicados, revocación de acceso, ediciones concurrentes y retirada de servicios con reservas existentes.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni suite de pruebas, por lo que este README no declara APIs, comandos ni validaciones ejecutadas.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
