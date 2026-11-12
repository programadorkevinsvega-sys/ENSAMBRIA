# Membresías, fidelización y beneficios del establecimiento

## Identificación

- **Tipo:** bounded context del backend BUSINESS.
- **Ubicación:** `backend/business/modules/memberships-loyalty-benefits/`.
- **Estado:** diseño documentado; no hay implementación funcional en este directorio.
- **Fase prevista:** Fase 3, conforme al roadmap de CRM y fidelización.

## Propósito y alcance

Administrar membresías y beneficios comerciales locales ofrecidos por un establecimiento. Su ownership comprende el plan local, la relación con el cliente, los saldos o beneficios y su consumo.

Estas membresías son distintas de las membresías de acceso y pertenencia de CORE y de las suscripciones SaaS comerciales de PLATFORM. El módulo no determina precios ni planes SaaS.

## Operaciones previstas

Emitir, renovar —si la regla está aprobada—, cancelar y redimir membresías o beneficios; consultar saldo y estado. Cada operación debe identificar cliente, tenant, regla y versión, y protegerse frente a reintentos.

## Límites e integraciones

- Puede relacionarse con CRM y Commerce mediante contratos.
- Requiere CORE para cualquier efecto financiero autorizado.
- No usa PLATFORM como owner de la fidelización local ni mezcla estos hechos con suscripciones SaaS.
- No comparte datos con otros tenants ni depende de acceso SQL cruzado.

## Seguridad e integridad

La autorización se limita al cliente, tenant, sucursal y acción correspondientes. La comunicación promocional requiere una preferencia independiente. El consumo o ajuste de beneficios debe ser atómico y evitar doble redención ante concurrencia o repetición.

La aceptación futura debe cubrir saldo cero y límites, consumos simultáneos, revocación, vencimiento, reembolso y reintentos sin doble beneficio. Las reglas de expiración requieren aprobación de producto y las validaciones aplicables antes de habilitarse.

## Implementación y pruebas

Este directorio contiene solo la estructura de capas. No hay manifiesto, contrato ejecutable ni suite de pruebas; no se declara comportamiento implementado ni comandos disponibles.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
