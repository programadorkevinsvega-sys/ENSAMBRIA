# ADR-0010: Gestión de dependencias con Dependabot y Rulesets como complemento de Branch Protection

**Estado:** decisión de gobierno aceptada; configuración remota verificada el 2026-11-26 con las diferencias enumeradas abajo.
**Fecha:** 2026-11-16
**Autores:** Equipo de ingeniería ENSAMBRIA
**Revisores:** `@programadorkevinsvega-sys`
**Reglas aplicables:** §11, §12, §13 de `ENSAMBRIA_github_repository_governance_and_engineering_rules.md`

> La política conserva su aceptación. Los hechos de configuración descritos abajo son una fotografía verificada el 2026-11-26 y deben revalidarse antes de usarlos como evidencia futura.

---

## Contexto

La auditoría de cumplimiento de Fase 1 identificó dos controles pendientes:

1. **Dependabot** — §11 exige secret scanning, push protection y gestión de dependencias vulnerables. §13 exige lockfiles y revisión de vulnerabilidades. El ecosistema activo es Rust (Cargo workspace con `Cargo.lock`) y GitHub Actions.
2. **GitHub Rulesets** — §4 recomienda Rulesets o Branch Protection Rules para proteger `main`. La Branch Protection clásica se configuró en la auditoría de Fase 1, pero los Rulesets ofrecen ventajas adicionales que merecen evaluación e implementación complementaria.

---

## Decisión

### Dependabot (`dependabot.yml`)

Se habilita Dependabot para dos ecosistemas:

| Ecosistema | Directorio | Frecuencia | Límite PRs | Agrupación |
|---|---|---|---|---|
| `cargo` | `/` (workspace root) | Semanal (lunes) | 5 | Patches agrupados en un PR; minor/major individuales |
| `github-actions` | `/` | Semanal (lunes, +30 min) | 3 | Individual (cada acción tiene su SHA) |

**Justificación:**
- `Cargo.lock` está en la raíz del workspace (ADR-0003); Dependabot lo lee desde `/`.
- Las acciones de GitHub Actions están fijadas a SHA completo (§12). Dependabot propone SHAs actualizados manteniendo el comentario de versión semántica.
- Los patches se agrupan para minimizar el ruido de PR; minor y major se revisan individualmente por riesgo de rotura de API.
- Las actualizaciones de seguridad (CVE) eluden el horario semanal y se abren inmediatamente.

**Limitaciones conocidas:**
- Dependabot no ejecuta `cargo audit` ni `cargo deny`; el workflow actual del repositorio ejecuta ambos controles cuando existen el workspace y su lockfile.
- [`deny.toml`](../../deny.toml) mantiene una lista explícita de licencias SPDX observadas en el árbol bloqueado y rechaza nuevas licencias no incluidas. La lista es una política automatizada de dependencias, no una aprobación o dictamen jurídico. Los crates propios `publish = false` se excluyen del chequeo de licencias externas; su licencia de producto permanece gobernada por el archivo [`LICENSE`](../../LICENSE).
- No se añade `flutter` ni `dart` porque los proyectos de aplicaciones mobile no existen aún (Fase 5+).

### Estado remoto verificado — 2026-11-26

La API del repositorio muestra activo el Ruleset `main-protection` (ID `27596836`) en `refs/heads/main`, sin bypass actors. Requiere Pull Request, actualización estricta de la rama, los checks `repository-quality` y `security-dependencies`, y resolución de conversaciones; bloquea force-push y borrado. El mínimo de aprobaciones está configurado en cero conforme a la excepción de mantenedor único de §8.1. La consulta de Branch Protection clásica devuelve HTTP 404: no existe una segunda capa clásica comprobada. El Ruleset anterior `protect-main` (ID `27409010`) y la afirmación de coexistencia son históricos, no el estado vigente.

| Control remoto adicional | Estado verificado |
|---|---|
| SHA pinning obligatorio en ajustes de Actions | No requerido (`sha_pinning_required: false`); las referencias `uses:` en workflows actuales están fijadas a SHA completo. La configuración remota no previene nuevas referencias flotantes. |
| Allowlist de Actions | Se permiten todas (`allowed_actions: all`). |
| Actions security fixes | Endpoint devuelve HTTP 200. |
| Dependabot configuration | `.github/dependabot.yml` declara `cargo` y `github-actions`; el estado del archivo no prueba que cada actualización haya sido procesada. |
| Vulnerability alerts endpoint | La consulta de API devolvió HTTP 404; estado no confirmado por esa interfaz. |

La configuración se inspeccionó mediante GitHub API de solo lectura. Este registro no autoriza modificar Rulesets, protección, Actions ni secretos.

---

## Consecuencias

### Positivas
- Vulnerabilidades en dependencias Rust y en versiones de acciones de CI se detectan automáticamente.
- Los Rulesets añaden resolución obligatoria de hilos de revisión y auditabilidad extendida.
- La protección de `main` pasa a ser visible en el tab "Rules" de GitHub y trazable en el audit log.

### Negativas / Riesgos
- Dependabot genera PRs automáticos que requieren revisión. Con `open-pull-requests-limit: 5` (Cargo) y 3 (Actions) se controla el volumen.
- La coexistencia de Branch Protection y Ruleset implica que ambas capas deben mantenerse sincronizadas si se modifica una; documentar cambios en ADR o issue.
- La política SPDX deberá revisarse cuando cambie el árbol de dependencias; no implica aprobación legal de ENSAMBRIA ni autoriza licencias nuevas automáticamente.

### Acciones futuras requeridas
- [ ] Revisar y decidir merge de PRs de Dependabot según se abran.
- [ ] Al incorporar más colaboradores, actualizar `required_approving_review_count` a ≥1 y reevaluar §8.1.
- [ ] Al migrar a organización GitHub, evaluar Rulesets de organización y deprecar Branch Protection clásica.

---

## Referencias

- [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](../rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md) §4, §11, §12, §13
- [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md)
- [ADR-0003: Cargo workspace único](ADR-0003-cargo-workspace.md)
- Ruleset creado: https://github.com/programadorkevinsvega-sys/ENSAMBRIA/rules/27409010
