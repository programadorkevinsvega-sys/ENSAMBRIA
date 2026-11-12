# Identidad y autenticación

## Identificación y estado

- **Tipo:** bounded context de CORE.
- **Ubicación:** `backend/core/modules/identity-authentication/`.
- **Estado:** diseño documentado; no hay implementación funcional.
- **Fase prevista:** Fase 1.

## Propósito y responsabilidad

Crear y validar identidades globales y gestionar autenticación, factores, sesiones y autenticadores. Una identidad global no concede por sí sola acceso a un tenant, producto ni sucursal.

CORE es propietario de los datos de identidad y autenticación. El contexto no es propietario de perfiles CRM, membresías comerciales ni interfaces de usuario.

## Operaciones previstas

Crear, verificar, recuperar y revocar identidades y sesiones; consultar el estado de autenticación; y gestionar rotación de credenciales conforme a contratos que se definan en la implementación.

## Límites y controles

- Usa primitivas técnicas neutrales y proveedores solo cuando exista una selección y un contrato aprobados.
- No depende de CRM, PUBLIC ni de un frontend.
- Mantiene secretos fuera de eventos, respuestas y logs; usa credenciales y grants mínimos.
- La autorización de tenant se resuelve separadamente de la identidad.

La aceptación futura debe cubrir sesión ausente, expirada o revocada, actor inválido, duplicados, concurrencia, fallos de proveedor y ausencia de credenciales en respuestas o logs.

## Implementación y pruebas

El directorio contiene solo estructura de capas. No hay contratos ejecutables, configuración de proveedores ni comandos de prueba documentados; no se declara runtime validado.

## Fuentes

- [Modelo de dominios](../../../../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../../../../docs/requerimiento.md)
- [Reglas de arquitectura](../../../../ARCHITECTURE_RULES.md)
