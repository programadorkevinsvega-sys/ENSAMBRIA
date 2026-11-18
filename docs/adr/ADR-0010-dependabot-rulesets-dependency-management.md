# ADR-0010: Gestión de dependencias con Dependabot y Rulesets como complemento de Branch Protection

**Estado:** decisión de gobierno aceptada en el corte anterior; configuración actual del repositorio **NO VERIFICADA**
**Fecha:** 2026-11-16
**Autores:** Equipo de ingeniería ENSAMBRIA
**Revisores:** `@programadorkevinsvega-sys`
**Reglas aplicables:** §11, §12, §13 de `ENSAMBRIA_github_repository_governance_and_engineering_rules.md`

> Este ADR registra decisiones y configuración del repositorio anterior. El reinicio no revoca la política documental, pero el `Cargo.lock`, el workflow y la configuración remota descritos aquí no se consideran vigentes sin una verificación nueva. El Ruleset e ID mencionados son evidencia histórica, no prueba de que sigan activos en el repositorio actual.

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
- Dependabot no ejecuta `cargo audit` ni `cargo deny`. Ambas herramientas se añadirán en una tarea futura de security hardening para escanear el árbol completo de dependencias transitivas.
- No se añade `flutter` ni `dart` porque los proyectos de aplicaciones mobile no existen aún (Fase 5+).

### GitHub Rulesets como complemento de Branch Protection clásica

En el corte anterior se registró la creación del Ruleset `protect-main` (id: `27409010`) con enforcement `active`, cubriendo `refs/heads/main`. Su existencia/estado actual no está verificado.

**Reglas del Ruleset:**

| Tipo de regla | Parámetros clave |
|---|---|
| `deletion` | Prohíbe eliminar `main` |
| `non_fast_forward` | Prohíbe force-push (historial no reescribible) |
| `pull_request` | PR obligatorio; `required_approving_review_count: 0` (§8.1); `dismiss_stale_reviews_on_push: true`; `required_review_thread_resolution: true` |
| `required_status_checks` | Contexto `repository-quality`; `strict: false` |

**Coexistencia con Branch Protection clásica:**

Ambos mecanismos se mantienen activos de forma complementaria. Las reglas del Ruleset que coinciden con Branch Protection se aplican como capa adicional. Las reglas exclusivas del Ruleset aportan controles no disponibles en la protección clásica:

| Control | Branch Protection clásica | Ruleset `protect-main` |
|---|---|---|
| Prohibir `force-push` | ✅ `allow_force_pushes: false` | ✅ `non_fast_forward` |
| Prohibir borrado de `main` | ✅ `allow_deletions: false` | ✅ `deletion` |
| PR obligatorio | ✅ | ✅ |
| Check `repository-quality` requerido | ✅ | ✅ |
| `enforce_admins` | ✅ | ✅ (Rulesets aplican a todos sin excepción por diseño) |
| Resolución de threads obligatoria | ❌ | ✅ `required_review_thread_resolution: true` |
| `dismiss_stale_reviews_on_push` | ✅ | ✅ |
| `bypass_actors` explícitos | N/A | Ninguno (`[]`) |
| Visibilidad en GitHub UI (Rules tab) | ❌ | ✅ |
| Auditabilidad en audit log | Limitada | ✅ Events `repository_ruleset.*` |
| Exportable como código (IaC) | ❌ | ✅ (API + Terraform) |

**Estrategia a largo plazo:** cuando el proyecto migre a una organización GitHub, los Rulesets de organización ofrecen cobertura de múltiples repositorios. En ese momento se evaluará deprecar la Branch Protection clásica. Por ahora ambas capas coexisten sin conflicto.

---

## Consecuencias

### Positivas
- Vulnerabilidades en dependencias Rust y en versiones de acciones de CI se detectan automáticamente.
- Los Rulesets añaden resolución obligatoria de hilos de revisión y auditabilidad extendida.
- La protección de `main` pasa a ser visible en el tab "Rules" de GitHub y trazable en el audit log.

### Negativas / Riesgos
- Dependabot genera PRs automáticos que requieren revisión. Con `open-pull-requests-limit: 5` (Cargo) y 3 (Actions) se controla el volumen.
- La coexistencia de Branch Protection y Ruleset implica que ambas capas deben mantenerse sincronizadas si se modifica una; documentar cambios en ADR o issue.
- `cargo audit` y `cargo deny` quedan pendientes; Dependabot no cubre dependencias transitivas con la misma profundidad.

### Acciones futuras requeridas
- [ ] Añadir `cargo audit` / `cargo deny` en el workflow CI (tarea futura de security hardening).
- [ ] Revisar y decidir merge de PRs de Dependabot según se abran.
- [ ] Al incorporar más colaboradores, actualizar `required_approving_review_count` a ≥1 y reevaluar §8.1.
- [ ] Al migrar a organización GitHub, evaluar Rulesets de organización y deprecar Branch Protection clásica.

---

## Referencias

- [ENSAMBRIA_github_repository_governance_and_engineering_rules.md](../rules/ENSAMBRIA_github_repository_governance_and_engineering_rules.md) §4, §11, §12, §13
- [ARCHITECTURE_RULES.md](../../ARCHITECTURE_RULES.md)
- [ADR-0003: Cargo workspace único](ADR-0003-cargo-workspace.md)
- Ruleset creado: https://github.com/programadorkevinsvega-sys/ENSAMBRIA/rules/27409010
