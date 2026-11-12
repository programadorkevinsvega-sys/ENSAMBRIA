# Aplicación API de BUSINESS

## Identificación y estado

- **Tipo:** punto de entrada del proceso desplegable del backend BUSINESS.
- **Ubicación:** `backend/business/apps/api/`.
- **Estado:** estructura de directorios; no hay aplicación ejecutable implementada.

## Propósito

Componer e iniciar el backend BUSINESS y sus módulos de dominio. Es la frontera de transporte del proceso: publica los contratos gRPC de BUSINESS que requieren otros backends o consumidores internos autorizados.

No es un BFF ni una API HTTP para que los frontends la consuman directamente. Los frontends usan el BFF/API de producto; el BFF adapta sus solicitudes a las superficies autorizadas. La aplicación tampoco debe contener reglas de negocio propias ni acceder a datos de otros owners.

## Responsabilidades previstas

- Construir la composición del proceso BUSINESS y los módulos que correspondan.
- Configurar los transportes y dependencias de proceso que se implementen.
- Exponer contratos del backend cuando un caso de uso requiera comunicación síncrona.
- Mantener configuración, credenciales, artefactos y despliegue propios de BUSINESS.

La comunicación durable de hechos persistidos usa Outbox + JetStream cuando corresponda; gRPC no sustituye esa entrega durable.

## Límites de ownership

Los módulos BUSINESS mantienen sus reglas, repositorios, schemas y migraciones. La API no consulta tablas de CORE, PLATFORM o PUBLIC ni reemplaza los contratos de sus owners. Las consultas o comandos a otro backend se realizan por contratos autorizados.

## Ejecución y pruebas

No hay manifiesto, binario, configuración ejecutable ni pruebas en este directorio. No se documentan comandos de ejecución ni se afirma que exista un servidor gRPC operativo.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
