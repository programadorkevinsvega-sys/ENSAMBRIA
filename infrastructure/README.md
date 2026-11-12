# Infraestructura compartida

## Identificación y estado

- **Tipo:** organización de servicios de infraestructura compartida.
- **Ubicación:** `infrastructure/`.
- **Estado:** estructura objetivo; no hay configuración de servicios declarada aquí como implementada.

## Propósito y límites

Agrupar documentación y configuración de servicios compartidos de desarrollo u operación cuando se incorporen de forma autorizada. La infraestructura compartida no fusiona los procesos, imágenes, credenciales, despliegues ni ownership de los cuatro backends, BFFs o frontends.

Si existe un Compose raíz, se reserva para servicios compartidos; no se agregan aplicaciones de producto como unidad de despliegue.

## Organización

Los componentes compartidos se encuentran bajo [`shared/`](shared/README.md): PostgreSQL, Redis, NATS Core, NATS JetStream, Vault y observabilidad. Cada servicio conserva configuración, seguridad, operación y validaciones propias.

## Seguridad y operación

No almacenar secretos en el repositorio. La configuración futura debe aplicar mínimo privilegio y seguir las guías especializadas de cada servicio. No se documentan comandos de inicio, endpoints, puertos ni entornos hasta verificarlos en archivos reales.

## Ejecución y pruebas

No hay un manifiesto de infraestructura o procedimiento ejecutable documentado en esta carpeta.

## Fuentes

- [Reglas de arquitectura](../ARCHITECTURE_RULES.md)
- [Reglas de PostgreSQL y pools](../docs/rules/ENSAMBRIA_Connection_Pooling_PostgreSQL.md)
- [Reglas de Outbox e Inbox](../docs/rules/ENSAMBRIA_Transactional_Outbox_Inbox.md)
