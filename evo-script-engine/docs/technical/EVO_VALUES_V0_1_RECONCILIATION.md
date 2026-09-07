# Evo-Script Engine ↔ evo-values v0.1 — Technical Reconciliation Authority

```text
Historical Technical Design
CLOSED / PRESERVED

evo-values v0.1 Technical Reconciliation
CLOSED

Current Reconciled Implementation
CLOSED
```

Este documento constituye la autoridad técnica compacta que rige la reconciliación arquitectónica entre `evo-script-engine` y `evo-values v0.1`.

---

## 1. División de Responsabilidades Semánticas y Arquitectónicas

```text
evo-values
    = autoridad semántica de las operaciones universales
      sobre valores

evo-script-engine
    = semántica propia del lenguaje:
      disponibilidad de operadores
      type checking
      lowering
      bytecode
      control flow
      RuntimeValue
      backing runtime
      traducción de failures
```

### Principios rectores:

1. **`evo-values` como Dependencia Semántica Interna**: `evo-values` es la autoridad universal de representación y operaciones semánticas sobre valores (aritmética escalar y dinámica, comparaciones [EQUAL / NOT_EQUAL sobre Value] y conversiones numéricas). Es una dependencia semántica interna de `evo-script-engine` y **NO un Provider**.
2. **Responsabilidad de Lenguaje de `evo-script-engine`**: `evo-script-engine` retiene la responsabilidad exclusiva sobre las reglas sintácticas del lenguaje `.efn`, la disponibilidad de operadores, el chequeo estático de tipos, el lowering a bytecode, el conjunto de instrucciones de la VM, el control de flujo, la estructura interna de `RuntimeValue` y su backing storage, así como la traducción contextual de fallas operacionales hacia `ExecutionFailure` / `EvaluationFailure`.
3. **No Duplicación de Semántica Universal**: `evo-script-engine` no replica algoritmos de evaluación de valores cuando `evo-values` ya posee la operación universal. `InstructionExecutor` delega directamente en las operaciones provistas por `evo-values`.

---

## 2. Decisiones Técnicas y Ajustes del Modelo

```text
RuntimeValue != Value

DynamicIntegerBacking
    → payload actual reconciliado:
      OwnedDynamicInteger

execution observation
    → OwnedDynamicInteger::as_borrowed()
    → zero-copy magnitude

direct num-bigint dependency
    → REMOVED

num-bigint
    → eliminado como dependencia directa de evo-script-engine
    → permanece únicamente como detalle privado transitivo
      de evo-values (v0.5.1)

EqualityRule
CompositeEqualityPlan
EnumEqualityPayloadPlan
    → removidos del código y modelo técnico reconciliado

Instruction variants
    → permanecen exactamente 48

Compiled Program identities
    → 21 históricas
    → 18 reconciliadas

Technical identities total
    → 140 históricas
    → 137 reconciliadas

Participants de evo-script-engine
    → permanecen exactamente 21

Architectural signature changes
    → 0

Historical orchestration sequence views
    → 4 preserved

Cross-component reconciliation sequence views
    → 5

Current canonical sequence views
    → 9
```

### Detalle de ajustes:

- **`RuntimeValue != Value`**: `Value<'a>` y `OwnedValue` son los tipos de intercambio del ecosistema. `RuntimeValue` es el descriptor de ejecución de la VM en `evo-script-engine`, optimizado para evaluación y backing inmutable.
- **`DynamicIntegerBacking`**: El payload de almacenamiento para enteros de precisión arbitraria en runtime se reconcilia utilizando `OwnedDynamicInteger` de `evo-values`. La observación en execution backing se realiza mediante `OwnedDynamicInteger::as_borrowed()`, preservando la identidad de puntero de magnitude con zero-copy. La representación compilada `DynamicConstant::Integer` permanece canónicamente como `(negative, magnitude)` sin conversión a BigInt.
- **Eliminación de `num-bigint` directo**: `num-bigint` quedó eliminado como dependencia productiva directa de `evo-script-engine`. Permanece únicamente como detalle privado transitivo de `evo-values` (v0.5.1). Toda la representación y aritmética de enteros dinámicos se consume a través de la API pública de `evo-values`.
- **Eliminación de planes de igualdad técnica**: Al delegar la igualdad en `evo_values::comparison::EQUAL` y `evo_values::comparison::NOT_EQUAL` sobre `Value`, la comparación recursiva de Struct/Enum pertenece internamente al kernel de Comparison de `evo-values`. Por ello, las identidades intermedias `EqualityRule`, `CompositeEqualityPlan` y `EnumEqualityPayloadPlan` quedaron eliminadas del código fuente y del modelo técnico de Compiled Program.
- **Variantes de `Instruction`**: El conjunto de instrucciones de la VM permanece estrictamente en 48 variantes.
- **Identidades Técnicas de Compiled Program**: Se reducen de 21 identidades históricas a 18 identidades reconciliadas (-3 por la remoción de los planes de igualdad).
- **Total de Identidades Técnicas**: Se reduce de 140 identidades históricas a 137 identidades reconciliadas en el modelo técnico general.
- **Participantes de `evo-script-engine`**: Permanecen estrictamente en 21 participantes (3 Agents, 6 Collaborators, 1 Resolver, 0 Requesters, 0 Additional Contracts, 8 Tools; 21 módulos conductuales).
- **Firmas Arquitectónicas**: 0 cambios en las firmas arquitectónicas públicas y cerradas.

### Delegaciones semánticas implementadas:

```text
Fixed arithmetic
    → evo-values numeric UCs

Boolean NOT
    → evo-values boolean::NOT

Scalar comparison
    → evo-values comparison

Structural equality
    → OBSERVE_RUNTIME_VALUE
    → EQUAL / NOT_EQUAL

Fixed conversion
    → evo-values target-specific conversion UCs

NumericToString
    → evo-values conversion

LiftDynamic
    → evo-values conversion

Dynamic Numeric
    → six DYNAMIC_* UCs

ConvertDynamic
    → TO_*_FROM_DYNAMIC

DynamicToString
    → TO_STRING_FROM_DYNAMIC
```

La operación de módulo dinámico sobre flotantes (`Dynamic Float remainder`) permanece no disponible en Evo-Script, rechazada preventivamente con `EvaluationFailure::DynamicNumericType` antes de invocar `DYNAMIC_REMAINDER`.

---

## 3. Invariantes Arquitectónicas y Exclusiones

La reconciliación técnica cerrada **NO crea**:

```text
ValueSemanticBridge
mega-adapter
nuevo Agent
nuevo Collaborator
nuevo Resolver
nuevo Requester
nuevo Contract
nuevo Tool
```

### Regla RSD-011 aplicada:

```text
Una función auxiliar interna no es una Tool arquitectónica
únicamente por ser pequeña.
```

Toda función auxiliar introducida durante la reconciliación para delegación de operaciones se modela como función interna dentro de los módulos existentes (`instruction_executor`, etc.), sin proliferar en participantes arquitectónicos ficticios.

Todas las reglas y comentarios arquitectónicos RSD permanecen redactados en español.

---

## 4. Vistas de Secuencia Canónicas

El inventario canónico de secuencias se compone de:

```text
Historical orchestration sequence views
    → 4 preserved

Cross-component reconciliation sequence views
    → 5

Current canonical sequence views
    → 9
```

- **4 Secuencias Históricas de Orquestación** (preservadas):
  1. `COMPILE` orchestration
  2. `EXECUTE_COMPILED` orchestration
  3. `EXECUTE_SOURCE` orchestration
  4. `RESOLVE_EXTERNAL_CALL` interaction
- **5 Secuencias Cross-Component de Reconciliación** (MATERIALIZED / CLOSED):
  1. `04-direct-scalar-delegation.d2`
  2. `05-scalar-comparison-delegation.d2`
  3. `06-conversion-delegation.d2`
  4. `07-dynamic-numeric-delegation.d2`
  5. `08-structural-equality-delegation.d2`

---

## 5. Estado de la Implementación

```text
Current Reconciled Implementation
CLOSED

Implementation reconciliation
TASK-ESE-RECON-006..013
COMPLETED / APPROVED
```

La reconciliación en código Rust, dependencias Cargo, diagramas D2 y modelo de datos ha sido completada y verificada mediante la ejecución aprobada de las tareas `TASK-ESE-RECON-001` a `TASK-ESE-RECON-013`.

### Snapshot de Gates de Regresión (TASK-ESE-RECON-013):
```text
evo-values
297 tests PASS

evo-script-engine
485 tests PASS
```

> [!NOTE]
> `cargo test --workspace` presenta una falla informativa derivada de un defecto externo preexistente en `evo-query` (falta de derivación de `Debug`/`PartialEq` para `Value` en queries). Dicha falla es ajena a `evo-script-engine` y no constituye un gate de la reconciliación.

El Work Package de implementación y su cierre formal se encuentran registrados en:

[`WP-ESE-RECON-001.md`](implementation-tasks/WP-ESE-RECON-001.md)
