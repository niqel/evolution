# WP-ESE-RECON-001: Evo-Script Engine ↔ evo-values v0.1 Reconciliation

```text
STATUS: ACTIVE

PROGRESS:
0 / 15 APPROVED

BASE COMMIT:
02365bc9a4c79f55fa009f8afca28533b23ec8db
```

Este Work Package define la secuencia obligatoria para reconciliar documentalmente y en código `evo-script-engine` con la arquitectura cerrada de `evo-values v0.1`.

---

## 1. Reglas Operativas

- Una Task a la vez.
- Un commit por Task que cambie archivos.
- Push es checkpoint, no aprobación.
- Cada commit debe ser revisado directamente en GitHub.
- APPROVED antes de abrir la siguiente Task.
- NO-OP no genera empty commit.
- No merge.
- No rebase.
- No squash.
- No amend.
- No force push.
- Detenerse ante contradicción de arquitectura o PRODUCT DEFECT FOUND.

---

## 2. Inventario Canónico de Tareas

### TASK-ESE-RECON-001 — Registrar autoridad documental y Work Package
- **Estado**: PENDING REVIEW
- **Base obligatoria**: `02365bc9a4c79f55fa009f8afca28533b23ec8db`
- **Alcance**: Exclusivamente documental. Registrar `EVO_VALUES_V0_1_RECONCILIATION.md`, `WP-ESE-RECON-001.md`, y actualizar `README.md`, `TECHNICAL_DESIGN.md` e `implementation-tasks/README.md`.

### TASK-ESE-RECON-002 — Reconciliar Compiled/Data Model documental
- **Estado**: NOT STARTED
- **Alcance**: Reconciliar formalmente las definiciones documentales del Compiled Program Data Model y la eliminación de planes de igualdad técnica.

### TASK-ESE-RECON-003 — Reconciliar Runtime Data documental
- **Estado**: NOT STARTED
- **Alcance**: Reconciliar formalmente la especificación documental de `DynamicIntegerBacking` y `RuntimeValue`.

### TASK-ESE-RECON-004 — Reconciliar Technical Data Diagrams
- **Estado**: NOT STARTED
- **Alcance**: Actualizar los diagramas D2 del modelo de datos técnico para reflejar las 18 identidades de compiled program y el total de 137 identidades.

### TASK-ESE-RECON-005 — Agregar secuencias cross-component
- **Estado**: NOT STARTED
- **Alcance**: Materializar los diagramas de secuencia D2 para las 5 nuevas vistas de reconciliación cross-component con `evo-values`.

### TASK-ESE-RECON-006 — Restablecer compatibilidad con la API pública de evo-values
- **Estado**: NOT STARTED
- **Alcance**: Actualizar imports y dependencias de tipos de `evo-values` en el código de `evo-script-engine`.

### TASK-ESE-RECON-007 — Delegar fixed arithmetic y Boolean NOT
- **Estado**: NOT STARTED
- **Alcance**: Conectar las instrucciones de aritmética fija y negación booleana con las operaciones de `evo-values`.

### TASK-ESE-RECON-008 — Delegar scalar comparisons
- **Estado**: NOT STARTED
- **Alcance**: Conectar las instrucciones de comparación escalar con las operaciones canónicas de `evo-values`.

### TASK-ESE-RECON-009 — Eliminar equality plans y delegar structural equality
- **Estado**: NOT STARTED
- **Alcance**: Eliminar en código `EqualityRule`, `CompositeEqualityPlan` y `EnumEqualityPayloadPlan`, delegando la igualdad estructural en `evo-values`.

### TASK-ESE-RECON-010 — Delegar fixed conversions y Numeric ToString
- **Estado**: NOT STARTED
- **Alcance**: Conectar las instrucciones de conversión de tipos numéricos y to-string hacia las funciones de `evo-values`.

### TASK-ESE-RECON-011 — Reconciliar Dynamic Value end-to-end
- **Estado**: NOT STARTED
- **Alcance**: Conectar evaluación de enteros y flotantes dinámicos usando las 6 operaciones de Dynamic Numeric Arithmetic de `evo-values v0.1`.

### TASK-ESE-RECON-012 — Eliminar dependencia directa num-bigint
- **Estado**: NOT STARTED
- **Alcance**: Remover `num-bigint` de `evo-script-engine/Cargo.toml` asegurando que toda manipulación use `evo-values`.

### TASK-ESE-RECON-013 — Regression suite integrada
- **Estado**: NOT STARTED
- **Alcance**: Ejecutar e integrar suite completa de pruebas de regresión en `evo-script-engine` y el workspace.

### TASK-ESE-RECON-014 — Cerrar documentación de implementación
- **Estado**: NOT STARTED
- **Alcance**: Actualizar el estado de cierre de implementación en la documentación técnica del crate.

### TASK-ESE-RECON-015 — Final Quality Gate
- **Estado**: NOT STARTED
- **Alcance**: Ejecución del gate de calidad final (`cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy`).
