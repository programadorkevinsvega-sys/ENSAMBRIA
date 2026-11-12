# Aplicación API de CORE

## Identificación y estado

- **Tipo:** punto de entrada del proceso desplegable de CORE.
- **Ubicación:** `backend/core/apps/api/`.
- **Estado:** estructura de directorios; no hay aplicación ejecutable implementada.

## Propósito

Componer e iniciar el backend CORE y sus módulos propietarios. Aloja la frontera de transporte para contratos gRPC que otros backends o consumidores autorizados necesiten invocar de forma síncrona.

No es un BFF y no se expone directamente a frontends. La interfaz de usuario consume el BFF/API autorizado, que se integra con los contratos del backend. La API de CORE no debe incorporar reglas de dominio ajenas ni conceder acceso solo por conocer un identificador.

## Responsabilidades previstas

- Componer los módulos de CORE que formen parte del proceso.
- Exponer contratos autenticados y autorizados cuando un caso de uso los requiera.
- Aplicar la configuración y dependencias propias del proceso CORE.
- Mantener el despliegue independiente de los otros backends.

Los hechos que requieran procesamiento durable se distribuyen mediante Outbox + JetStream; una llamada gRPC no reemplaza ese flujo.

## Límites de ownership

CORE es dueño de identidad/autenticación, tenancy y contexto de seguridad, evaluación de entitlements, reservas comunes y operaciones financieras centrales. Los módulos mantienen el ownership de sus datos y persistencia. CORE no accede directamente a tablas de BUSINESS, PLATFORM o PUBLIC.

## Ejecución y pruebas

No hay manifiesto, binario, configuración ejecutable ni pruebas en este directorio. No se afirma que el servidor gRPC ni los módulos CORE estén implementados.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
