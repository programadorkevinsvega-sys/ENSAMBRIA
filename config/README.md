# Configuración de ENSAMBRIA

## Identificación y estado

- **Tipo:** ubicación prevista para configuración versionada no secreta.
- **Ubicación:** `config/`.
- **Estado:** estructura organizativa; no hay configuración ejecutable declarada en este directorio.

## Propósito y límites

Organizar configuración propia del producto, entornos y observabilidad cuando los componentes funcionales la requieran. La configuración no es fuente de autorización ni puede habilitar una capacidad no implementada.

No incluir secretos, credenciales, datos reales de tenants ni valores de producción sensibles. Los secretos deben obtenerse por los mecanismos aprobados para el workload.

## Organización prevista

- `products/beauty-wellness/`: configuración declarativa/versionada del producto cuando se implemente.
- `environments/`: configuración por entorno cuando existan contratos y mecanismos reales.
- `observability/`: configuración no sensible de observabilidad cuando aplique.

Cada producto y proceso conserva límites, permisos y composición propios; no se usa una configuración común para fusionar despliegues.

## Uso y validación

No hay esquema, variables de entorno ni comandos de carga configurados aquí. No se deben inferir valores predeterminados ni instrucciones de ejecución.

## Fuentes

- [Modelo de dominios](../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../docs/requerimiento.md)
- [Reglas de arquitectura](../ARCHITECTURE_RULES.md)
