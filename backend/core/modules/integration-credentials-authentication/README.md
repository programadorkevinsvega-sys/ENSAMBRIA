# Credenciales y autenticación de integraciones

## Identificación y estado

- **Tipo:** módulo de CORE para credenciales técnicas de integraciones externas.
- **Ubicación:** `backend/core/modules/integration-credentials-authentication/`.
- **Estado:** estructura de diseño; no hay implementación funcional.
- **Fase prevista:** los cimientos de confianza se definen en Fase 1; la API externa y sus credenciales se habilitan en Fase 7.

## Propósito y responsabilidad

Autenticar integraciones técnicas que consumen la API pública versionada de ENSAMBRIA. Este módulo es el owner de las credenciales técnicas usadas para autenticar esos consumidores, incluidas API keys cuando el contrato de integración las admita.

Separa tres responsabilidades:

- **CORE — este módulo:** autenticación de la credencial técnica de integración y el contexto de identidad asociado.
- **PLATFORM — `integrations-api-extensions`:** registro y relación comercial del integrador, catálogo, revisión y estado comercial.
- **Backend propietario del recurso:** autorización de cada operación y recurso; una credencial válida no concede acceso por sí sola.

Este contexto no gestiona identidad de personas, sesiones humanas ni el catálogo comercial de integradores.

## Capacidades previstas

- Emitir, validar, revocar y rotar credenciales de integración según contratos aprobados.
- Resolver la identidad técnica de una integración y entregar contexto verificable para la autorización.
- Rechazar credenciales inválidas, vencidas o revocadas.

El formato, ciclo de vida detallado, representación persistida de secretos y protocolo de emisión deben definirse antes de implementar. Este README no especifica algoritmos, formatos de token ni valores de configuración.

## Límites e integraciones

- El gateway de API externa valida la credencial mediante un contrato de CORE; no consulta almacenamiento de credenciales directamente.
- PLATFORM administra la relación comercial, pero no emite ni autentica la credencial técnica.
- Cada backend dueño aplica scopes y autoriza sus propios recursos; conocer tenant, integración o recurso por ID no basta para permitir acceso.
- No se exponen credenciales a otros servicios ni se incluyen secretos en eventos, trazas o logs.
- La API externa no expone gRPC interno, módulos ni acceso directo a bases de datos.

## Seguridad y aceptación futura

La implementación deberá definir y probar mínimo privilegio, scopes, límites de consumo, revocación, rotación, idempotencia, auditoría y resistencia a abuso. Las pruebas deben incluir credencial inválida, expirada o revocada, scope insuficiente, acceso cruzado entre tenants, concurrencia de revocación y ausencia de filtración de secretos.

Los detalles de protección criptográfica, almacenamiento y gestión de secretos quedan sujetos a diseño de seguridad y validación conforme a las reglas del proyecto; no se consideran aprobados por la existencia de esta carpeta.

## Estructura

Las carpetas `domain/`, `application/`, `adapters/` e `infrastructure/` establecen límites organizativos DDD + Clean Architecture + Hexagonal. No contienen todavía código, contratos, migraciones ni adapters funcionales.

## Ejecución y pruebas

No hay manifiesto, API implementada ni suite ejecutable en este directorio. No se documentan comandos de instalación, ejecución o pruebas.

## Fuentes

- [Requerimiento: API pública e integraciones](../../../../docs/requerimiento.md)
- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
