# Aplicación API de PLATFORM

## Identificación y estado

- **Tipo:** punto de entrada del proceso desplegable de PLATFORM.
- **Ubicación:** `backend/platform/apps/api/`.
- **Estado:** estructura de directorios; no hay aplicación ejecutable implementada.

## Propósito

Componer e iniciar el backend PLATFORM y sus módulos. Aloja contratos gRPC para interacciones síncronas autorizadas con otros backends y consumidores de servicio.

No debe confundirse con un BFF externo de producto. El BFF de administración está integrado inicialmente en PLATFORM como una superficie diferenciada; esta carpeta representa la aplicación/proceso del backend, no las rutas HTTP del BFF ni la lógica de negocio.

## Responsabilidades previstas

- Componer los módulos PLATFORM que se implementen en el proceso.
- Exponer contratos autenticados y autorizados cuando una integración síncrona lo requiera.
- Alojar la composición del BFF de administración integrado, según se implemente, respetando sus límites de transporte.
- Mantener configuración, credenciales, artefactos y despliegue propios de PLATFORM.

Los hechos comerciales que necesiten entrega durable usan Outbox + JetStream; gRPC no se usa como sustituto de eventos durables.

## Límites de ownership

PLATFORM posee el control global y la administración comercial de tenants, catálogo de productos, planes/suscripciones SaaS y políticas comerciales. CORE conserva tenancy técnica y efectos financieros; BUSINESS conserva operaciones de establecimientos; PUBLIC conserva las proyecciones públicas autorizadas. PLATFORM no accede a sus tablas.

## Ejecución y pruebas

No hay manifiesto, binario, configuración ejecutable ni pruebas en este directorio. No se documentan comandos de ejecución ni se afirma que existan servidores gRPC o BFF operativos.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
