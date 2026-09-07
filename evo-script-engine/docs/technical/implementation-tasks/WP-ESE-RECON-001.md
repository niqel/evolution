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
- **Alcance**: Compatibility baseline SIN cambio semántico:
  - sustituir direct private-field access;
  - usar `from_parts` / accessors / `as_borrowed` según corresponda;
  - corregir tests que dependan de construcción directa;
  - eliminar assumptions de `PartialEq` semántico sobre `Value`/`OwnedValue`;
  - NO agregar `PartialEq` a `Value` ni `OwnedValue`;
  - NO cambiar todavía `DynamicIntegerBacking`;
  - NO eliminar todavía `BigInt`;
  - NO delegar todavía operaciones semánticas.

### TASK-ESE-RECON-007 — Delegar fixed arithmetic y Boolean NOT
- **Estado**: NOT STARTED
- **Alcance**: Conectar las instrucciones de aritmética fija y negación booleana con las operaciones de `evo-values`.

### TASK-ESE-RECON-008 — Delegar scalar comparisons
- **Estado**: NOT STARTED
- **Alcance**: Conectar las instrucciones de comparación escalar con las operaciones canónicas de `evo-values`.

### TASK-ESE-RECON-009 — Eliminar equality plans y delegar structural equality
- **Estado**: NOT STARTED
- **Alcance**: Eliminar en código `EqualityRule`, `CompositeEqualityPlan` y `EnumEqualityPayloadPlan`, delegando la comparación y equivalencia en `evo_values::comparison::EQUAL` y `evo_values::comparison::NOT_EQUAL` sobre `Value`.

### TASK-ESE-RECON-010 — Delegar fixed conversions y Numeric ToString
- **Estado**: NOT STARTED
- **Alcance**: Conectar las instrucciones de conversión de tipos numéricos y to-string hacia las funciones de `evo-values`.

### TASK-ESE-RECON-011 — Reconciliar Dynamic Value end-to-end
- **Estado**: NOT STARTED
- **Alcance**: Reconciliación Dynamic end-to-end:
  - `DynamicIntegerBacking`: `BigInt` → `OwnedDynamicInteger`
  - Reconciliar Tools:
    - `MaterializeValue`
    - `MaterializeOwnedValue`
    - `ObserveRuntimeValue`
    - `OwnRuntimeValue`
  - `LiftDynamic`
  - Dynamic numeric operations (delegación en `evo-values v0.1`):
    - `DynamicNegate`
    - `DynamicAdd`
    - `DynamicSubtract`
    - `DynamicMultiply`
    - `DynamicDivide`
    - `DynamicRemainder`
  - Dynamic conversions y string:
    - `ConvertDynamic`
    - `DynamicToString`
  - `DynamicNumericFailure` mapping a `EvaluationFailure`
  - Preservar la regla de lenguaje:
    - Dynamic Float remainder → `EvaluationFailure::DynamicNumericType` (no llamada a `DYNAMIC_REMAINDER`)
  - Al finalizar no debe quedar uso productivo de `BigInt` dentro de `src/` (la dependencia Cargo se elimina únicamente en TASK-ESE-RECON-012).

### TASK-ESE-RECON-012 — Eliminar dependencia directa num-bigint
- **Estado**: NOT STARTED
- **Alcance**: Remover `num-bigint` de `evo-script-engine/Cargo.toml` asegurando que toda manipulación use `evo-values`.

### TASK-ESE-RECON-013 — Regression suite integrada
- **Estado**: NOT STARTED
- **Alcance**: Ejecutar suite completa de pruebas de regresión integrada.
- **Gates obligatorios**:
  ```text
  cargo test -p evo-values
  cargo test -p evo-script-engine
  ```
- **Validación informativa (no gate)**:
  `cargo test --workspace` puede ejecutarse solamente como validación informativa y NO como gate del Work Package debido al defecto separado de `evo-query`.

### TASK-ESE-RECON-014 — Cerrar documentación de implementación
- **Estado**: NOT STARTED
- **Alcance**: Actualizar el estado de cierre de implementación en la documentación técnica del crate.

### TASK-ESE-RECON-015 — Final Quality Gate
- **Estado**: NOT STARTED
- **Alcance**: Ejecución del Quality Gate final de la reconciliación.
- **Gates obligatorios**:
  ```text
  cargo fmt --check
  cargo check -p evo-script-engine
  cargo test -p evo-values
  cargo test -p evo-script-engine
  ```
- **Verificaciones obligatorias adicionales**:
  ```text
  evo-script-engine/Cargo.toml
      no direct num-bigint

  evo-script-engine/src/
      no BigInt
      no EqualityRule
      no CompositeEqualityPlan
      no EnumEqualityPayloadPlan

  Instruction
      exactly 48 variants

  RuntimeValue
      exactly 17 variants

  DynamicValue
      exactly 3 variants

  EvaluationFailure
      exactly 4 variants
  ```
- **Comprobación opcional / informativa**:
  `cargo clippy` no figura como gate obligatorio de este Work Package (es opcional/informativo).
