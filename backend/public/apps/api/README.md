# Aplicación API de PUBLIC

## Identificación y estado

- **Tipo:** punto de entrada del proceso desplegable del backend PUBLIC.
- **Ubicación:** `backend/public/apps/api/`.
- **Estado:** estructura de directorios; no hay aplicación ejecutable implementada.

## Propósito

Componer e iniciar PUBLIC y sus módulos propietarios. Expone por gRPC los contratos autorizados que consume el BFF público independiente para Discovery y otras consultas públicas permitidas.

No es el BFF público ni la API HTTP para navegadores. El BFF público adapta REST/OpenAPI y consume PUBLIC mediante los contratos gRPC correspondientes.

## Responsabilidades previstas

- Componer los módulos PUBLIC que se implementen en el proceso.
- Atender consultas sobre datos proyectados y autorizados mediante contratos explícitos. Las páginas públicas tenant consumen la proyección base aceptada en Fase 5.3; Discovery/Marketplace se incorpora en Fase 9.1 sobre esa proyección, sin ser precondición de las páginas de Fase 5.
- Mantener configuración, credenciales, artefactos y despliegue propios de PUBLIC.

Los cambios durables de proyecciones se reciben desde sus owners mediante los mecanismos aprobados; esta API no consulta directamente los stores privados de BUSINESS ni de otros backends.

## Límites de ownership

PUBLIC es dueño de Discovery/directorio, estado de publicación y proyecciones autorizadas, reseñas y atribución del canal Marketplace. No posee CRM, catálogo operativo, inventario, pedidos ni reservas privadas. La indexabilidad no concede autorización para publicar.

## Ejecución y pruebas

No hay manifiesto, binario, configuración ejecutable ni pruebas en este directorio. No se afirma que exista un servidor gRPC operativo o una integración validada con el BFF público.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
