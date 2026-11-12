# Contratos de ENSAMBRIA

## Identificación y estado

- **Tipo:** ubicación prevista para contratos versionados con owner explícito.
- **Ubicación:** `contracts/`.
- **Estado:** estructura de directorios; no se declaran contratos ejecutables aquí.

## Propósito y límites

Mantener definiciones explícitas y versionadas para las fronteras entre backends, BFFs y clientes. Cada contrato pertenece a un owner y debe representar una integración real aprobada; no se inventan esquemas, endpoints ni eventos por anticipado.

Los contratos describen interfaces, no trasladan ownership de datos o reglas. No sustituyen autorización del backend propietario.

## Organización prevista

- `protobuf/`: contratos gRPC, separados por backend propietario.
- `openapi/`: contratos HTTP/OpenAPI de superficies BFF autorizadas, incluyendo Beauty & Wellness cuando se definan.
- `schemas/`: esquemas de eventos o datos versionados cuando exista un uso aprobado.

## Versionado y validación

Cambios futuros deben conservar owner, compatibilidad y estrategia de evolución verificables. gRPC se usa para interacción síncrona; eventos durables requieren Outbox/JetStream cuando aplique. Un nombre o carpeta no implica un contrato implementado.

No hay aquí compiladores, generadores, validadores ni comandos configurados que puedan documentarse como ejecutables.

## Fuentes

- [Modelo de dominios](../docs/arquitectura-dominios.md)
- [Reglas de arquitectura](../ARCHITECTURE_RULES.md)
- [Requerimiento y roadmap](../docs/requerimiento.md)
