# ADR-0013: AWS para configuración, secretos e identidad mTLS de workloads

- **Estado:** proveedor AWS indicado expresamente por el propietario el 2026-11-26; selección de servicios propuesta para ratificación técnica antes de aprovisionar recursos.
- **Fecha:** 2026-11-26
- **Ámbito:** configuración de runtime, secretos, certificados de servicio y credenciales de workloads que consumen AWS; no cambia el proveedor de cómputo ni autoriza crear recursos.
- **Owners:** Plataforma/SRE y owners de CORE, BUSINESS, PLATFORM, PUBLIC y cada proceso/BFF.
- **Fuentes:** [requerimiento §22](../requerimiento.md#22-roadmap-de-implementación), [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md), [gestión de secretos de AWS](https://docs.aws.amazon.com/secretsmanager/latest/userguide/intro.html), [Parameter Store](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html), [AWS Private CA](https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html), [ACM: certificados privados exportables](https://docs.aws.amazon.com/acm/latest/userguide/export-private.html) e [IAM Roles Anywhere](https://docs.aws.amazon.com/rolesanywhere/latest/userguide/introduction.html).

## Contexto

El roadmap anterior reservaba la integración centralizada de secretos/PKI para 11.4. El propietario decidió usar AWS para mTLS, variables de configuración y secretos. Esa decisión se aplica desde la primera subfase que dependa de dichos recursos; 11.4 conserva la automatización transversal completa y no posterga seguridad/integración del primer uso. La topología de ENSAMBRIA conserva VMs/VPS independientes por proceso; no se decidió que el cómputo deba migrarse a AWS. Por ello, el diseño debe permitir tanto workloads en AWS como VMs fuera de AWS sin distribuir claves IAM permanentes.

La guía de AWS distingue parámetros de configuración estáticos de secretos: Parameter Store admite parámetros `String` y `SecureString`, y recomienda Secrets Manager para credenciales y otros secretos; Secrets Manager también admite claves privadas y certificados. AWS Private CA puede emitir certificados privados. IAM Roles Anywhere permite que servidores y aplicaciones fuera de AWS obtengan credenciales temporales mediante certificados X.509. Estas capacidades no prueban que ENSAMBRIA tenga cuenta, región, contratos, CA o recursos configurados.

## Decisión

### Proveedor y servicios objetivo

1. **Configuración/variables no sensibles:** usar AWS Systems Manager Parameter Store. Solo valores clasificados como no secretos pueden ser `String`. `SecureString` queda reservado para configuración sensible que no sea una credencial rotatoria; la excepción debe tener owner, clasificación, política KMS y justificación documentada. Los procesos pueden convertir referencias aprobadas en configuración de proceso al arrancar, pero no exportan valores secretos a `.env`, archivos de despliegue o variables persistentes del host.
2. **Secretos y credenciales:** usar AWS Secrets Manager, con permisos IAM específicos por ambiente, backend, proceso y secreto. Cada workload obtiene únicamente los secretos necesarios. Secretos no se almacenan en archivos versionados, imágenes, argumentos de proceso, bundles web/mobile, logs, trazas ni artefactos CI.
3. **mTLS interno:** usar AWS Private CA para emitir certificados privados de servicio. Cada workload genera y conserva su clave privada localmente y presenta una CSR; AWS firma y devuelve certificado/cadena, sin custodiar la clave privada del servicio en Secrets Manager. La política de emisión limita CA, template, sujeto/SAN y vigencia; los detalles de restricciones disponibles deben verificarse contra el diseño de IAM/Private CA antes de desplegar.
4. **Credenciales AWS de workloads:** las VMs EC2 usan roles asociados al compute. Los workloads en otros VPS/VM usan IAM Roles Anywhere para recibir credenciales temporales de roles IAM y no usan access keys permanentes. La identidad que obtiene credenciales AWS y la identidad de transporte mTLS tienen permisos y atributos de certificado acotados; no se reutiliza un certificado con permisos más amplios por conveniencia.
5. **Superficie de confianza:** mTLS acredita identidad de servicio mediante cadena de confianza e identidad SAN; cada backend sigue autorizando método, operación, audiencia y scope. El certificado no sustituye RBAC, autorización de dominio, rate limits ni autorización tenant.
6. **Aislamiento:** cuentas, parámetros, secretos, claves KMS, perfiles/roles, CA, certificados y auditoría se separan al menos por ambiente y owner. El patrón de nombres sugerido para parámetros y secretos es `/ensambria/{environment}/{backend}/{process}/{purpose}`; no fija cuenta, región, nombre real, política IAM ni valor de runtime.
7. **Propagación al frontend:** ninguna credencial AWS, secreto de backend, certificado cliente mTLS o clave privada se envía al navegador o a una aplicación móvil. La configuración pública que se compile en un frontend debe ser allowlist no sensible y no concede permisos.
8. **Fase:** Fase 0 registra el diseño y sus pendientes sin crear recursos, código runtime, esquemas ni credenciales. Cada fase/subfase que introduzca consumo de configuración/secretos o identidad/certificados mTLS debe aplicar y probar esta decisión con AWS real en ese corte. La subfase 11.4 completa automatización centralizada de provisión, renovación/rotación, auditoría, recuperación y operación para todos los workloads; no aplaza controles exigidos en Fase 1 ni permite aceptar integraciones previas con mocks.

### Ciclo de certificados y operación

- La PKI de backend usa identidad distinta por proceso/ambiente y verificación estricta de CA, SAN, vigencia y propósito. Los clientes mantienen el trust bundle autorizado y aceptan solapamiento controlado durante la rotación de certificados/CA.
- La renovación y despliegue en VMs externas requieren automatización que exporte la cadena pública, instale el certificado de forma atómica, compruebe el servicio y retire el certificado anterior tras el periodo de solapamiento. Un certificado ACM renovado para uso fuera de un servicio integrado de AWS requiere exportación e instalación en el destino; por eso no se considera renovación automática completada solo por emitir el certificado. El diseño base de este ADR favorece clave local + CSR a AWS Private CA.
- La emisión, rotación, revocación, caducidad y recuperación se auditan. El servicio rechaza conexiones con certificados no confiables, vencidos, revocados o con SAN/audiencia incorrectos. La caída de AWS no convierte una conexión sin certificado válido en permitida. Los procesos no registran material secreto; los logs locales y la observabilidad se mantienen no bloqueantes según la norma aplicable.
- La bootstrap identity para Roles Anywhere y la primera emisión del certificado mTLS se diseñan sin circularidad: un operador/proceso de provisión autorizado establece la identidad inicial; el workload no puede obtener su primer permiso leyendo el secreto cuya lectura depende de ese mismo permiso.

## Aceptación de implementación

La aceptación se ejecuta en la fase propietaria con cuenta y entorno autorizados, identidades y recursos AWS reales. Debe incluir:

1. Lectura permitida y denegada de parámetros y secretos para cada ambiente/backend/proceso, incluyendo `AccessDenied`, secreto inexistente y cuenta/región incorrectas.
2. Pruebas de que ningún rol obtiene un secreto de otro backend/ambiente y de que el acceso está registrado en CloudTrail sin revelar el valor.
3. Autenticación de workload externo con IAM Roles Anywhere: credencial temporal válida, certificado desconocido/expirado/revocado, perfil/rol no autorizado y expiración/renovación de la sesión.
4. Emisión de certificados desde CSR, verificación SAN/issuer/CA, mTLS exitoso entre servicios autorizados y rechazo de certificado cliente inválido, vencido, revocado o de otro ambiente.
5. Rotación de secret y certificado con solapamiento, proceso de instalación atómica, health check, recuperación ante reinicio y eliminación segura del material reemplazado.
6. Caída/timeout de los endpoints AWS, error KMS, respuesta incompleta y reintentos: no filtrar secreto, no aceptar identidad degradada ni bloquear operaciones no relacionadas cuando la política permita una caché vigente.
7. Inspección de repositorio, imágenes, logs, trazas, salida de CI y frontend para verificar ausencia de secretos, claves privadas y credenciales de larga duración.
8. Restauración/recuperación de configuración, certificados y trust anchors; prueba de sucesión de CA, actualización de trust stores y reversión en ambiente aislado.

Cada prueba se traza a requisito, caso, comando local/CI o job AWS controlado, SHA, resultado y evidencia. La aceptación de una unidad no cubre automáticamente los otros backends.

## Decisiones operativas pendientes

La selección de AWS como proveedor está decidida por el propietario. Antes de aprovisionar o usar datos reales quedan por registrar y aprobar. La fase/subfase propietaria de la primera integración real debe añadirse al roadmap con sus dependencias y criterios, sin cambiar en Fase 0 el estado de aceptación histórico de 1.1:

- cuenta(s), ambiente(s), región(es), topología de red y endpoints permitidos;
- costos y operación de AWS Private CA, Secrets Manager, KMS y Roles Anywhere;
- jerarquía de CA, trust anchors, perfiles y atributos de certificado por tipo de workload;
- autoridad de bootstrap, revocación, rotación/solapamiento, tiempos de respuesta y recuperación;
- claves KMS y separación de duties, acceso de operadores, monitoreo, CloudTrail y retención;
- evaluación de proveedor, ubicación, transferencias, subprocesadores, contrato/DPA y Legal por tratamiento;
- inventario de secretos actual, clasificación y secuencia de migración desde el mecanismo temporal;
- estrategia de disponibilidad de parámetros/secretos ya cargados durante indisponibilidad AWS.

Estas decisiones no hacen que la Fase 0 sea aceptada ni detienen trabajo documental independiente. Los datos reales o mercados que dependan de una evaluación/validación previa permanecen bloqueados hasta cumplirla. No se ha realizado revisión jurídica de AWS ni se presume aprobación.

## Alternativas consideradas

- **HashiCorp Vault como proveedor nativo:** ya no es el destino seleccionado por la instrucción del propietario; no se desplegará ni accederá a un Vault compartido para ENSAMBRIA.
- **Guardar todo en Parameter Store `SecureString`:** no seleccionado para contraseñas, API keys o tokens; AWS recomienda Secrets Manager para esos valores y sus necesidades de ciclo de vida.
- **Guardar claves privadas mTLS en Secrets Manager:** no es el diseño base; el workload genera y conserva su clave privada y AWS Private CA firma su CSR.
- **Access keys IAM de larga duración en VMs externas:** descartadas; los workloads externos usan credenciales temporales obtenidas mediante IAM Roles Anywhere.
- **Exigir que todo runtime se ejecute en AWS:** descartado como conclusión de esta decisión; solo se decide usar AWS para configuración, secretos y mTLS.

## Consecuencias y referencias oficiales

- Se necesita un proceso de enrolamiento, IAM, rotación e instalación en hosts que no usan servicios integrados de AWS.
- AWS Private CA y las claves/secretos gestionados generan costo operativo; el presupuesto y las regiones no se dan por aprobados en este ADR.
- El hecho de que AWS ofrezca estos servicios no determina residencia de datos, roles legales, transferencia internacional ni cumplimiento de ENSAMBRIA.
- [Parameter Store: comparación de usos y tipos](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html)
- [Secrets Manager: autenticación y control de acceso](https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html)
- [AWS Private CA: certificados de clave privada y solicitud/exportación](https://docs.aws.amazon.com/privateca/latest/userguide/PcaGetCert.html)
- [IAM Roles Anywhere: workloads fuera de AWS](https://docs.aws.amazon.com/rolesanywhere/latest/userguide/introduction.html)
- [ACM: ciclo de renovación de certificados privados](https://docs.aws.amazon.com/acm/latest/userguide/renew-private-cert.html)
- [Registro de proveedores/subencargados](../compliance/THIRD_PARTY_PROCESSORS.md)
- [Registro de Fase 0](../registro-fase-0-descubrimiento-arquitectura.md)
