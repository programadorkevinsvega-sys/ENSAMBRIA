# Control de habilitación de mercado y tratamiento

> **BORRADOR TEMPORAL — ENSAMBRIA — PENDIENTE DE REVISIÓN Y APROBACIÓN POR ABOGADO COMPETENTE EN COLOMBIA.**

> **Estado:** checklist documental; no es autorización de lanzamiento. Ningún país, ciudad, tratamiento, proveedor o canal se considera habilitado por estar incluido en esta lista. La aprobación legal, de producto y operativa debe quedar registrada con evidencia.

Esta lista aplica junto a [LEGAL_JURISDICTIONS.md](LEGAL_JURISDICTIONS.md), [política legal](../rules/ENSAMBRIA_Privacy_Legal_Compliance.md), [acceso geográfico](../rules/ENSAMBRIA_Geographic_Access_Policies.md) y [PDLM](../rules/ENSAMBRIA_Privacy_Data_Lifecycle_Management.md).

## Antes de habilitar un país, región o tratamiento

- [ ] Registrar fuente oficial vigente, jurisdicción, fecha de consulta y texto normativo; guardar enlace y aplicar a la actividad, no asumir por nombre del país.
- [ ] Confirmar entidad operadora, responsable/encargado/corresponsable por tratamiento, owner técnico, contacto de privacidad y asesoría Legal.
- [ ] Inventariar titulares, categorías, finalidad, base jurídica, campos, flujos, sistemas derivados, destinatarios, proveedores y transferencias.
- [ ] Validar aviso/contrato, consentimiento y preferencias, menores, cookies/tracking, marketing, derechos, retención, transferencias, incidentes y contenido cuando apliquen.
- [ ] Determinar requisitos fiscales, de consumidores, facturación, pagos y comunicaciones con Producto/Finanzas/Legal.
- [ ] Aprobar proveedor, sandbox/contrato/DPA, ubicación, subprocesadores, seguridad, retención, supresión e incidentes.
- [ ] Definir política geográfica por capacidad (navegación, solicitudes, registro, compra/pago, operación); IP solo como señal, no prueba definitiva.
- [ ] Implementar backend authorization, tenant isolation, minimización, logging/redacción, retention y lifecycle en el mismo corte que introduce el tratamiento.
- [ ] Verificar accesibilidad, contenido localizado, SEO/metadata pública y `noindex` de superficies privadas; no indexar borradores o datos personales.
- [ ] Ejecutar pruebas positivas, negativas, límites, duplicados/concurrencia, autorización, fallos, recuperación y privacidad con dependencias reales aplicables.
- [ ] Ejecutar k6 solo en entorno autorizado con datos transitorios no personales, flujos/adapters reales, límites de parada y thresholds aprobados.
- [ ] Documentar riesgos residuales, controles, casos/tests, comandos/resultados, commit, responsables y evidencia.
- [ ] Obtener aceptación explícita de Producto/Arquitectura/seguridad y aprobación legal para las obligaciones aplicables.
- [ ] Mantener deshabilitada la capacidad dependiente mientras una obligación crítica o control de seguridad siga sin validar.

## Registro de resultado

Para cada decisión de go/no-go, registrar mercado/función, versión de matriz, owner, asesor legal que revisó, fecha, fuentes oficiales, pruebas ejecutadas, condiciones, excepciones aprobadas y fecha de revisión. No publicar como “compliant”, “certificado” o “disponible” sin evidencia y alcance verificables.

**Estado Colombia:** mercado inicial de diseño según el roadmap; la revisión oficial y la aprobación legal están en preparación y pendientes. No implica autorización para procesar datos reales ni habilitar una operación comercial.
