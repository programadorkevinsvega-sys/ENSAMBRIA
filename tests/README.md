# Pruebas transversales

## Identificación y estado

- **Tipo:** ubicación para pruebas justificadas que cruzan fronteras de módulos o componentes.
- **Ubicación:** `tests/`.
- **Estado:** hay directorios organizativos; no se declara una suite implementada ni ejecutada.

## Propósito y límites

Organizar pruebas transversales de arquitectura, contratos, integración, aceptación, E2E, seguridad y aislamiento cuando el alcance justifique probar más de un owner o frontera.

Las pruebas unitarias pertenecen al módulo que implementa el comportamiento. Las pruebas de integración, contrato y aceptación deben mantenerse con el backend o contrato propietario según corresponda. Esta carpeta no reemplaza esas pruebas ni la evidencia de CI.

## Organización prevista

- `architecture/`: límites y dependencias permitidas.
- `contracts/`: compatibilidad de contratos compartidos.
- `integration/`: integración entre componentes reales.
- `acceptance/`: escenarios de aceptación de extremo a extremo de una frontera.
- `e2e/`: recorridos completos autorizados.
- `security/`: controles de seguridad transversales.
- `isolation/`: aislamiento tenant y de datos entre owners.

La presencia de una carpeta no demuestra que existan casos de prueba.

## Ejecución

No hay un runner o comandos de pruebas transversales configurados en este directorio. Consultar los manifests, workflows y scripts reales cuando se incorporen pruebas.

## Fuentes

- [Reglas de arquitectura](../ARCHITECTURE_RULES.md)
- [Modelo de dominios](../docs/arquitectura-dominios.md)
- [Requerimiento y roadmap](../docs/requerimiento.md)
