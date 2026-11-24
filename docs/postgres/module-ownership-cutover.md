# PostgreSQL desde cero: propiedad modular y bootstrap reproducible

**Decisión:** ADR-0012 y confirmación del propietario el 2026-11-24.
**Estado:** diseño de destino aprobado; implementación completa y pruebas de aislamiento pendientes.
**Alcance:** crear una base nueva y vacía. No existe cutover de datos históricos.

Este documento sustituye cualquier procedimiento anterior de traslado, reconciliación, comparación de hashes o rollback de registros históricos. La base anterior fue eliminada por el propietario. No se deben inventar mecanismos para recuperar o reconciliar datos que ya no existen.

## 1. Propietarios canónicos

La única fuente de verdad para la asignación de tablas es `architecture/data-ownership.yaml`. Los cinco bounded contexts y sus identidades destino son:

| Bounded context | Backend | Schema | Rol propietario | Rol runtime |
|---|---|---|---|---|
| identity-authentication | CORE | `core_identity_auth` | `core_identity_auth_schema_owner` | `core_identity_auth_runtime` |
| tenancy-membership-organization-branch-product | CORE | `core_tenancy` | `core_tenancy_schema_owner` | `core_tenancy_runtime` |
| establishment-catalog-verticals | BUSINESS | `business_establishment_catalog` | `business_establishment_catalog_schema_owner` | `business_establishment_catalog_runtime` |
| product-catalog-configuration | PLATFORM | `platform_product_catalog` | `platform_product_catalog_schema_owner` | `platform_product_catalog_runtime` |
| commercial-account-onboarding | PLATFORM | `platform_tenant_onboarding` | `platform_tenant_onboarding_schema_owner` | `platform_tenant_onboarding_runtime` |

No se permite doble propiedad, FK entre propietarios, consultas SQL a tablas privadas ajenas, vistas puente, funciones SECURITY DEFINER para ocultar acceso, triggers puente ni transacciones PostgreSQL entre propietarios. Las referencias entre módulos son IDs opacos y se resuelven mediante contratos públicos autorizados.

## 2. Identidades y privilegios

Cada módulo tendrá un rol propietario NOLOGIN y un rol runtime LOGIN, NOINHERIT, NOSUPERUSER, NOCREATEDB, NOCREATEROLE y NOBYPASSRLS. Los roles propietarios no son credenciales de aplicación.

El migrador es una identidad separada. `core_migrator` puede recibir membresía explícita en los roles propietarios únicamente para ejecutar migraciones controladas mediante `SET ROLE`; ningún runtime ni worker puede asumir roles propietarios o migradores. La configuración debe eliminar membresías heredadas o grants previos que contradigan esta política y comprobarlos en CI.

Las credenciales LOGIN se provisionan fuera del repositorio y del historial de CI. Ninguna contraseña se almacena en SQL, YAML, documentación, variables versionadas ni logs.

Los permisos de runtime se conceden explícitamente y solo sobre los objetos del propietario correspondiente. PUBLIC no recibe acceso indebido. El runtime no recibe CREATE en schemas ni privilegios DDL. Los permisos de conexión a la base se evalúan y conceden explícitamente conforme al modelo de ejecución, sin confundir CONNECT con permisos sobre objetos.

## 3. Bootstrap desde base vacía

El procedimiento reproducible deberá ejecutar, en este orden:

1. Crear la base vacía y las identidades administrativas/migradoras mediante el mecanismo operativo autorizado.
2. Ejecutar `backend/core/deployment/provision-module-ownership.sql` con la identidad administrativa de bootstrap.
3. Verificar los diez roles, sus flags, membresías, schemas y ACLs antes de aplicar migraciones.
4. Ejecutar las migraciones de cada módulo con el migrador autorizado y `SET ROLE` al propietario correspondiente.
5. Crear tablas, secuencias, índices, constraints, vistas, funciones, triggers y políticas RLS dentro del schema de su propietario.
6. Aplicar grants runtime mínimos y explícitos para tablas, secuencias y funciones requeridas.
7. Ejecutar pruebas positivas y negativas conectándose como cada identidad runtime real.
8. Repetir el bootstrap completo en una segunda base vacía y comprobar que el resultado es reproducible.

Las migraciones no pueden depender de superusuario, crear objetos en schemas ajenos, consultar datos privados de otro módulo ni conceder privilegios de migración al runtime. La idempotencia se exige donde la herramienta y el diseño la permitan; no se debe ocultar una migración no repetible envolviéndola en comprobaciones que ignoren fallos.

## 4. Inventario de objetos y validación

`architecture/data-ownership.yaml` es el inventario canónico. Debe enumerar, por módulo, todos los objetos persistentes y sus atributos de propiedad: tablas, particiones, secuencias, vistas/materialized views, funciones/procedimientos, triggers, tipos, policies RLS y grants relevantes. Índices y constraints deben verificarse asociados a su tabla propietaria.

CI debe comparar el inventario con los catálogos reales de PostgreSQL después de las migraciones. Un objeto sin propietario declarado, un objeto extra no inventariado o un objeto ubicado en schema ajeno falla el gate. No se aceptan dos owners para el mismo objeto.

El inventario actual es una base inicial, no evidencia de cobertura completa. No se declara completo hasta que la comparación automática contra PostgreSQL real pase.

## 5. Pruebas PostgreSQL obligatorias

Las pruebas se ejecutan contra PostgreSQL real y con conexiones independientes para cada rol runtime. Deben demostrar:

- acceso permitido a los objetos propios que el módulo necesita;
- SELECT, INSERT, UPDATE y DELETE denegados sobre tablas ajenas;
- USAGE/CREATE denegados en schemas ajenos;
- acceso denegado a secuencias y funciones ajenas;
- imposibilidad de asumir roles owner o migradores;
- ausencia de privilegios PUBLIC indebidos;
- ausencia de permisos DDL para runtime;
- persistencia de los grants correctos después de migraciones;
- uso de la identidad runtime correcta por API, workers, consumers y tests funcionales.

La identidad administrativa solo puede preparar la base y consultar metadatos de verificación. No puede ejecutar operaciones de aplicación ni sustituir las pruebas con runtime real.

## 6. Dependencias entre módulos

Antes de habilitar grants runtime, el código debe consumir exclusivamente contratos públicos:

- BUSINESS resuelve identidad, membresía, organización y sucursal mediante contratos CORE; no consulta tablas CORE.
- CORE identity-authentication no consulta tablas de tenancy; utiliza el contrato público de CORE tenancy cuando una operación lo requiere.
- PLATFORM tenant-onboarding consulta product-catalog mediante su contrato y mantiene sus datos Inbox/onboarding en su schema.
- CORE tenancy consume el contrato de PLATFORM product-catalog cuando necesita información del catálogo.
- Ningún módulo importa crates de implementación ajenos ni comparte entidades de dominio.

La llamada síncrona in-process implementa el mismo trait que podrá adaptar a Tonic al separar procesos. No se crea un runtime gRPC ficticio en esta fase.

## 7. Estado verificable y bloqueos

A la fecha de esta revisión, el repositorio contiene un provisionamiento inicial de roles/schemas y un test preliminar de ACLs. Eso no demuestra todavía:

- que todas las migraciones creen objetos en los cinco schemas;
- que todos los objetos estén inventariados;
- que las membresías de roles estén cerradas;
- que existan grants runtime mínimos y positivos;
- que las pruebas negativas cubran tablas, secuencias y funciones;
- que workers y tests usen credenciales runtime propias.

Estos puntos permanecen pendientes hasta que el código y el CI los demuestren. No se marcarán como completados por documentación o por la mera existencia de roles/schemas.

## 8. Fuera de alcance

No se ejecuta ningún traslado de datos, reconciliación, comparación de hashes históricos, cutover expand/contract ni rollback de registros anteriores. Tampoco se despliega ni se modifica una base existente como parte de este trabajo. La validación de esta fase se realiza en PostgreSQL efímero creado desde cero.
