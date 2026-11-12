# Paquetes compartidos

## Identificación y estado

- **Tipo:** espacio de paquetes compartidos del monorepo.
- **Ubicación:** `packages/`.
- **Estado:** estructura organizativa; no hay paquetes funcionales implementados.

## Propósito y límites

Contener paquetes reutilizables cuando exista una necesidad demostrable de consumo por más de una aplicación. La reutilización no transfiere ownership de dominio ni autoriza compartir entidades, reglas de negocio, permisos o persistencia.

Los paquetes deben respetar sus tecnologías y límites de composición. No se agregan manifests, dependencias ni placeholders por anticipado.

## Estructura prevista

`frontend/` agrupa capacidades técnicas compartidas por los frontends, descritas en su [README](frontend/README.md). Cada paquete dentro de esa área se incorpora únicamente con una implementación y consumidor reales.

## Instalación, uso y pruebas

No hay paquetes instalables ni comandos de workspace configurados en este directorio. La estructura no constituye una publicación ni una biblioteca ejecutable.

## Fuentes

- [Reglas de arquitectura](../ARCHITECTURE_RULES.md)
- [Modelo de dominios](../docs/arquitectura-dominios.md)
