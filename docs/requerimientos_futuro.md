# Horizonte futuro de producto ENSAMBRIA

> **Estado:** ideas y funcionalidades candidatas para incorporar en el futuro. Este documento no sustituye el requerimiento vigente ni autoriza por sí solo la implementación o habilitación de una capacidad. El alcance vigente se define en [docs/requerimiento.md](requerimiento.md), la secuencia en [ROADMAP.md](../ROADMAP.md) y los límites/owners en [docs/arquitectura-dominios.md](arquitectura-dominios.md). Si una idea difiere de esas fuentes, prevalecen los requisitos y decisiones vigentes.

Las capacidades de este horizonte se conservan para evaluación y planificación futura. Antes de implementarlas se definirán owner, fase, contratos, criterios de aceptación y pruebas; se obtendrán las aprobaciones comerciales, de privacidad, seguridad y legales aplicables. Una propuesta aquí descrita no significa que la capacidad esté implementada, aprobada o disponible.

## 1. Ampliar los sectores atendidos

ENSAMBRIA Beauty & Wellness podrá incorporar y configurar capacidades para sectores relacionados:

### Barberías

- Cortes y barba.
- Membresías.
- Barberos y otros profesionales.
- Sillas y turnos.

### Peluquerías

- Cortes, tintes y tratamientos cosméticos.
- Estilistas.
- Lavacabezas.
- Tiempos y recursos del servicio.

### Salones de belleza

- Uñas, maquillaje, pestañas y cejas.
- Depilación.
- Paquetes de belleza.

### Spas

- Masajes y tratamientos corporales o faciales.
- Cabinas y terapeutas.
- Paquetes de servicios.

También se contempla evaluar centros de estética, estudios de maquillaje, centros de uñas y negocios de bienestar. Estos sectores son candidatos a configuración o verticales de BUSINESS; no crean automáticamente productos, backends, aplicaciones o módulos independientes. La definición vigente está en [docs/requerimiento.md, §§3.1–3.5](requerimiento.md) y [docs/arquitectura-dominios.md](arquitectura-dominios.md).

## 2. Funcionalidades avanzadas candidatas

### A. Inteligencia artificial para el negocio

- Asistente inteligente para responder preguntas sobre ventas, reservas, clientes y rentabilidad con datos autorizados.
- Predicción de demanda para estimar días y horarios con mayor cantidad de reservas.
- Predicción o evaluación de cancelaciones e inasistencias.
- Recomendaciones comerciales para sugerir promociones en horarios de baja ocupación.
- Análisis de rentabilidad de servicios, empleados y sucursales.
- Asistente de reservas por WhatsApp, sujeto a integración oficial, permisos y condiciones del canal.
- Generación de borradores de contenido, promociones y campañas para redes sociales.

Ejemplo de consulta futura:

> ¿Cuáles fueron mis cinco servicios más rentables este mes y qué debería promocionar la próxima semana?

ENSAMBRIA podría responder utilizando datos reales únicamente cuando el actor tenga autorización y las definiciones de cálculo sean verificables. Una respuesta o recomendación no concede permisos ni ejecuta por sí sola una reserva, cobro, campaña o publicación.

### B. Gestión avanzada de empleados

- Control de comisiones por servicio.
- Comisiones por venta de productos.
- Bonificaciones por objetivos.
- Liquidación de pagos a empleados, conforme a reglas y aprobaciones aplicables.
- Control de turnos y asistencia.
- Gestión de vacaciones y ausencias.
- Evaluaciones de desempeño.
- Metas individuales y por sucursal.
- Ranking de empleados con criterios definidos y acceso autorizado.
- Distribución de clientes según disponibilidad, especialidad y carga de trabajo, si las reglas aprobadas lo permiten.
- Portal privado para que cada empleado consulte sus reservas, comisiones, agenda y resultados permitidos.

### C. CRM y fidelización avanzada

- Historial completo de visitas.
- Preferencias de cada cliente.
- Fotografías de resultados anteriores, con autorización específica y controles aplicables.
- Registro de fórmulas de tintes y tratamientos cosméticos.
- Recordatorios de mantenimiento.
- Puntos de fidelización.
- Niveles VIP.
- Bonos o tarjetas de regalo.
- Referidos con recompensas.
- Cupones personalizados.
- Recuperación de clientes inactivos mediante campañas autorizadas.

El umbral de inactividad se configura por tenant/campaña. No se fija aquí un número universal de días. La recuperación de clientes debe respetar preferencias, permisos, límites de frecuencia y bajas efectivas.

### D. Gestión de inventario profesional

El inventario futuro no se limitaría a productos para venta al público. También podría incluir:

- Productos de uso interno.
- Consumo de materiales por servicio.
- Fórmulas y recetas operativas de tratamientos cosméticos.
- Lotes y fechas de vencimiento.
- Alertas de existencias mínimas.
- Compras a proveedores.
- Costos reales por tratamiento.
- Control de desperdicios.
- Inventario independiente por sucursal.
- Transferencias entre sucursales.

Estas capacidades permitirían calcular el costo real de un servicio, incluidos materiales y comisiones, conservando el detalle y las reglas versionadas necesarias para reproducir los resultados.

### E. Reservas y operaciones inteligentes

- Reservas grupales.
- Reservas recurrentes.
- Listas de espera automáticas.
- Confirmación y cancelación por WhatsApp, si el canal y el flujo se autorizan.
- Anticipos y depósitos, sujetos a condiciones contractuales y financieras aprobadas.
- Políticas de cancelación configurables.
- Reservas con varios profesionales.
- Servicios que requieren varias etapas.
- Gestión de cabinas, sillas, camillas y equipos.
- Bloqueos de disponibilidad.
- Propuestas de reasignación ante ausencias, sujetas a reglas y confirmación autorizada cuando corresponda.
- Control de sobrecupos y tiempos de preparación.

Por ejemplo, una reserva de spa podría necesitar simultáneamente un terapeuta, una cabina y un equipo específico. La disponibilidad deberá considerar todos los recursos necesarios y evitar asignaciones incompatibles o dobles.

## 3. Módulos especializados según el tipo de negocio

Las siguientes capacidades verticales candidatas podrían activarse según el tipo de establecimiento, sin obligar a todos los negocios a usar las mismas herramientas.

### Peluquerías — Colorimetría y tratamientos

- Fórmulas de color personalizadas.
- Historial de tintes y productos.
- Control de tiempos de exposición.
- Valoración y seguimiento capilar cosmético/no clínico.
- Paquetes de tratamientos.

La valoración capilar debe limitarse a observaciones cosméticas/no clínicas necesarias para el servicio; no es un diagnóstico médico ni una historia clínica. La captura de datos sensibles o de salud queda deshabilitada hasta que finalidad, base aplicable, clasificación, minimización, controles, permisos, retención y derechos estén definidos y validados para la jurisdicción correspondiente, conforme a [docs/requerimiento.md, §3.2](requerimiento.md).

### Salones de belleza — Uñas y estética

- Catálogo de diseños.
- Galería de trabajos con autorización de publicación.
- Historial de uñas y técnicas.
- Control de materiales.
- Paquetes de manicura y pedicura.

### Spas — Tratamientos y bienestar

- Fichas de tratamientos.
- Consentimientos informados cuando correspondan y estén definidos.
- Registro de contraindicaciones pertinentes como funcionalidad futura, sujeto a que se definan y validen previamente su finalidad, tratamiento de datos sensibles, permisos, protección, retención y requisitos legales aplicables.
- Historial de sesiones.
- Paquetes de varias sesiones.
- Gestión de cabinas y equipos.

### Centros de estética — Gestión especializada

- Planes de tratamiento.
- Seguimiento de sesiones.
- Documentación y consentimientos, conforme al servicio y los requisitos aplicables.
- Fotografías de evolución autorizadas específicamente, con control de acceso, trazabilidad, retención y eliminación.
- Control de equipos y mantenimiento.

Los módulos de estética deberán contemplar permisos profesionales, protección de datos sensibles y requisitos sanitarios aplicables. No son simples fichas de clientes; no se habilitan procedimientos regulados ni tratamientos sensibles sin las validaciones requeridas.

## 4. Funcionalidades para el crecimiento de los negocios

Estas capacidades podrían formar parte de ofertas futuras, si se definen y aprueban sus condiciones:

| Funcionalidad candidata | Beneficio esperado |
|---|---|
| Tienda online | Vender productos desde la web |
| Gift cards o tarjetas de regalo | Vender bonos de regalo |
| Paquetes prepagados | Cobrar varias sesiones anticipadamente, con términos aprobados |
| Membresías recurrentes | Ofrecer ingresos más predecibles, con condiciones aprobadas |
| Marketplace | Ayudar a conseguir nuevos clientes |
| Programa de referidos | Adquirir clientes mediante recomendaciones |
| Marketing automatizado | Recuperar y fidelizar clientes con permisos y preferencias respetados |
| Campañas segmentadas | Ofrecer promociones según comportamiento autorizado |
| Venta cruzada | Ofrecer productos y servicios complementarios sin crear una compra sin confirmación |
| Franquicias | Administrar múltiples establecimientos con scopes explícitos |

## 5. Marketplace de belleza

Como evolución del directorio público, el Marketplace podría permitir a las personas:

- Encontrar barberías, peluquerías, spas y salones cercanos.
- Comparar servicios y precios publicados.
- Consultar disponibilidad, revalidada por el backend propietario.
- Ver fotografías y trabajos autorizados para publicación.
- Consultar profesionales con información pública autorizada.
- Reservar directamente mediante los contratos de los owners.
- Pagar anticipadamente cuando el flujo y sus términos estén aprobados.
- Consultar reseñas verificadas conforme a políticas de elegibilidad y moderación.
- Guardar sus establecimientos favoritos.

Posibles modelos de monetización para evaluación futura incluyen comisiones por adquisición de clientes, publicidad destacada o planes de posicionamiento; ninguno queda aprobado por esta lista.

Se mantendrá la atribución para distinguir clientes que llegan directamente al negocio de los que llegan a través del Marketplace, sin mezclar cuentas, saldos ni datos privados. BUSINESS conserva las operaciones comerciales y PUBLIC el canal, las proyecciones públicas autorizadas y la atribución, conforme a [docs/arquitectura-dominios.md](arquitectura-dominios.md).

## 6. Herramientas empresariales para cadenas y franquicias

El producto podría atender negocios con varias sucursales, preservando ownership y permisos:

- Administración centralizada.
- Indicadores comparativos entre sucursales.
- Catálogos y precios centralizados o independientes, según política aprobada.
- Transferencias de inventario.
- Control de caja por establecimiento.
- Supervisores regionales.
- Permisos por sucursal.
- Consolidación financiera y reportes consolidados con ownership definido.
- Franquicias y regalías.
- Auditoría de operaciones.

Una empresa podría administrar sus sucursales desde una cuenta con permisos explícitos, mientras cada gerente consulta únicamente los establecimientos incluidos en su scope autorizado. La ubicación de estas capacidades se define en el requerimiento vigente y el [roadmap](../ROADMAP.md).
