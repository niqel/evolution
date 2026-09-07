# Evo-Script Engine — Compiled Numeric Instructions

Status: CLOSED (reconciled with evo-values v0.1)
Authority: [`../EVO_VALUES_V0_1_RECONCILIATION.md`](../EVO_VALUES_V0_1_RECONCILIATION.md)

Este documento cierra las identidades y reglas de bytecode para arithmetic numérico fijo, numeric comparisons y dynamic arithmetic de `evo-script-engine` v0 reconciliado con `evo-values v0.1`.

La autoridad deriva de:

- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_STORAGE_DATA.md`;
- `SEMANTIC_PROGRAM_DATA.md`;
- `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`;
- `evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`;
- `../EVO_VALUES_V0_1_RECONCILIATION.md`.

## 1. NumericKind

`NumericKind` representa mecanismo numérico fijo ejecutable, no identidad semántica completa de Evo-Script.

```rust
enum NumericKind {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,

    Float32,
    Float64,
}
```

Lowering canónico:

```text
NativeType::Int     → NumericKind::Int32
NativeType::Int32   → NumericKind::Int32

NativeType::Float   → NumericKind::Float64
NativeType::Float64 → NumericKind::Float64
```

Esto no declara equivalencia semántica entre `int` e `int32`, ni entre `float` y `float64`; únicamente expresa que comparten el mismo mecanismo físico después de semantic validation.

Regla:

```text
dynamic ∉ NumericKind
```

`dynamic` posee un modo de evaluación propio y se cierra en la sección correspondiente.

## 2. Fixed Numeric Unary Instruction

```rust
Instruction::Negate(NumericKind)
```

Stack effect:

```text
1 → 1
```

Invariantes:

1. solo puede generarse para signed integer o floating kinds;
2. `Negate(Uint*)` constituye Compiled Program inválido y Bytecode Compiler no lo produce;
3. fixed signed negation es checked;
4. negar el mínimo representable de un signed fixed kind produce `OverflowError`;
5. no existe wrapping, saturation ni unchecked variant.

## 3. Fixed Arithmetic Instructions

```rust
Instruction::Add(NumericKind)
Instruction::Subtract(NumericKind)
Instruction::Multiply(NumericKind)
Instruction::Divide(NumericKind)
Instruction::Remainder(NumericKind)
```

Stack effect común:

```text
before
... left right

instruction

 after
... result
```

La VM consume primero `right`, después `left`, aplica la operación y produce un único result.

El orden de evaluación de las subexpresiones permanece determinado por el orden de Instructions emitidas por Bytecode Compiler: left se evalúa antes de right.

## 4. Checked Fixed Arithmetic

### División de Responsabilidades:

```text
operator availability / lowering
    → evo-script-engine

universal arithmetic semantics
    → evo-values
```

Para fixed integer kinds:

```text
checked arithmetic
fixed overflow
    → Overflow
integer divide/remainder by zero
    → DivisionByZero

no wrapping
no saturation
no unchecked arithmetic
```

El engine traduce `NumericFailure` hacia `EvaluationFailure`.

No se introducen opcodes alternativos `CheckedAdd`, `WrappingAdd`, `SaturatingAdd` o equivalentes porque Evo-Script v0 posee una sola semántica observable.

## 5. Divide

`Divide(NumericKind)` conserva las reglas según kind:

### Fixed Integer:
- signed integer: quotient truncated toward zero.
- unsigned integer: unsigned quotient.
- divisor integer zero (`0`): produce `EvaluationFailure::DivisionByZero`.
- signed fixed integer: `MIN_VALUE / -1` produce `EvaluationFailure::Overflow`.

### Fixed Float (Semántica IEEE 754 / Rust):
El cálculo en Float32 y Float64 delega en `evo-values` y sigue la semántica estándar IEEE 754 de Rust:

```text
Float32 / Float64 arithmetic
→ evo-values
→ Rust / IEEE semantics

Infinity
-Infinity
NaN
+0.0
-0.0
are legitimate floating results/values
```

Por tanto:

```text
Float / ±0.0
→ IEEE result (±Infinity o NaN)
→ NO automatic DivisionByZero

non-finite result
→ NO automatic Overflow
```

La regla histórica que transformaba la división float por cero en `DivisionByZeroError` queda formalmente corregida y superseded por la reconciliación con `evo-values`.

## 6. Remainder

```rust
Instruction::Remainder(NumericKind)
```

Invariante de validez:

```text
NumericKind must be integer
```

`Remainder(Float32)` y `Remainder(Float64)` son estados compilados inválidos y Bytecode Compiler no los produce (`INTERNAL INVARIANT VIOLATION`).

Evo-Script v0 no expone fixed Float remainder. La existencia de Float remainder en `evo-values` **NO** obliga al lenguaje a exponerla.

Semántica para integer:

```text
integer divisor zero
    → DivisionByZero

signed MIN_VALUE % -1
    → Overflow
```

Para signed integers, el quotient asociado se define por truncation toward zero y el remainder conserva el signo del dividend cuando es non-zero.

No se introduce `IntegerKind` separado en v0 mientras `Remainder` sea su único consumidor significativo.

## 7. Fixed Numeric Comparisons

Las comparison instructions numéricas son:

```rust
Instruction::EqualNumeric(NumericKind)
Instruction::NotEqualNumeric(NumericKind)
Instruction::LessNumeric(NumericKind)
Instruction::LessEqualNumeric(NumericKind)
Instruction::GreaterNumeric(NumericKind)
Instruction::GreaterEqualNumeric(NumericKind)
```

Stack effect:

```text
2 → 1 bool
```

### Delegación de Autoridad:

```text
EqualNumeric
NotEqualNumeric
LessNumeric
LessEqualNumeric
GreaterNumeric
GreaterEqualNumeric

→ evo-values Comparison
```

Bytecode Compiler ya comprobó exact type compatibility; el engine mantiene compatibilidad estática y `NumericKind`. La VM delega la evaluación universal en las operaciones de Comparison de `evo-values`. Ordering solo existe para concrete numeric kinds.

## 8. Logical Operators Are Not Numeric Instructions

`&&` y `||` poseen short-circuit semantics y no se representan como arithmetic binary instructions que requieran ambos operands previamente evaluados.

Por tanto no se introducen aquí:

```text
AndNumeric
OrNumeric
AndBoolean as eager binary instruction
OrBoolean as eager binary instruction
```

Su lowering pertenece al bloque de Control Flow.

## 9. Dynamic Lifting

```rust
Instruction::LiftDynamic(NumericKind)
```

`LiftDynamic` transforma una representación numérica fija válida en una representación runtime `dynamic` preservando exactamente el valor.

Stack effect:

```text
1 → 1
```

No es una conversión visible del lenguaje, no corresponde a `to_tipo` y es infalible para un fixed numeric value válido.

Lowering conceptual:

```text
signed/unsigned integer fixed
    → Dynamic Integer exact mathematical value

Float32
    → Dynamic Float32

Float64
    → Dynamic Float64
```

Una vez elevado a Dynamic Integer, width y signedness originales dejan de limitar la representación; se conserva el valor matemático signed exacto.

## 10. Dynamic Context Lifts Before Arithmetic

Cuando una arithmetic subtree se evalúa bajo contexto semántico `dynamic`, fixed operands se elevan antes de ejecutar la operación.

Incorrecto:

```text
LOAD fixed-left
LOAD fixed-right
ADD Int8
LIFT_DYNAMIC Int8
```

Correcto:

```text
LOAD fixed-left
LIFT_DYNAMIC Int8
LOAD fixed-right
LIFT_DYNAMIC Int8
DYNAMIC_ADD
```

Regla canónica:

> Dynamic evaluation starts at the arithmetic origin; fixed overflow no ocurre primero para luego convertirse a dynamic.

## 11. Dynamic Arithmetic Instructions

```rust
Instruction::DynamicNegate
Instruction::DynamicAdd
Instruction::DynamicSubtract
Instruction::DynamicMultiply
Instruction::DynamicDivide
Instruction::DynamicRemainder
```

### Separación de Responsabilidades:

```text
operation availability
runtime family restrictions
failure translation
    → evo-script-engine

universal Dynamic Numeric operation semantics
    → evo-values
```

Las seis operaciones universales son provistas por `evo-values v0.1`:
- `Negate` (`DYNAMIC_NEGATE`)
- `Add` (`DYNAMIC_ADD`)
- `Subtract` (`DYNAMIC_SUBTRACT`)
- `Multiply` (`DYNAMIC_MULTIPLY`)
- `Divide` (`DYNAMIC_DIVIDE`)
- `Remainder` (`DYNAMIC_REMAINDER`)

Stack effects:

```text
DynamicNegate      1 → 1
DynamicAdd         2 → 1
DynamicSubtract    2 → 1
DynamicMultiply    2 → 1
DynamicDivide      2 → 1
DynamicRemainder   2 → 1
```

## 12. Dynamic Runtime Dispatch Scope

Dynamic arithmetic requiere runtime dispatch únicamente dentro del universo numérico `dynamic`:

```text
Dynamic Numeric Value
├── Integer
├── Float32
└── Float64
```

No constituye general Value type dispatch y no inspecciona string, bool, struct, enum, Function ni Signature.

Compatible same-family arithmetic:

```text
Integer with Integer
Float32 with Float32
Float64 with Float64
```

No existe promoción o coerción implícita entre familias dinámicas.

Cross-family arithmetic es inválida durante evaluación:

```text
Integer with Float32
Integer with Float64
Float32 with Float64
```

y el engine traduce `DynamicNumericFailure::DifferentFamily` hacia:

```text
EvaluationFailure::DynamicNumericType
```

## 13. Dynamic Integer Semantics

Para Dynamic Integer:

```text
Add / Subtract / Multiply
    → arbitrary-precision exact arithmetic
    → no OverflowError caused by representation width

Divide
    → truncation toward zero

Remainder
    → Evo integer remainder semantics

Divide / Remainder by zero
    → DivisionByZero (EvaluationFailure::DivisionByZero)
```

La representación física de arbitrary-precision runtime values se reconcilia con `OwnedDynamicInteger` de `evo-values` (VM Execution Data).

## 14. Dynamic Floating Semantics

Dynamic Float32 conserva `f32` semantics y Dynamic Float64 conserva `f64` semantics (IEEE 754 / Rust):

```text
Dynamic Float32
Dynamic Float64
→ Rust / IEEE semantics

division by ±0.0
→ IEEE result (±Infinity o NaN)
→ NOT DivisionByZero
```

### Restricción de lenguaje en DynamicRemainder:

Aunque `evo-values` posee Dynamic Float remainder como operación universal:

```text
Evo-Script DynamicRemainder
    Integer + Integer
        → delegate to DYNAMIC_REMAINDER

    Float32 / Float64 family
        → EvaluationFailure::DynamicNumericType
        → NO call to DYNAMIC_REMAINDER
```

## 15. Dynamic Context Does Not Cross Concrete Contracts

El contexto dynamic exterior no modifica internamente una Function, Signature o explicit conversion cuyo contrato produce un tipo fijo.

```text
concrete function/capability/conversion
    → evaluates under its own declared type semantics
    → produces fixed value or EvaluationError
    → only then may LiftDynamic occur in outer expression
```

No se altera el contrato para evitar `OverflowError` interno.

## 16. Dynamic Comparisons Remain Absent

No existen instructions:

```text
DynamicEqual
DynamicNotEqual
DynamicLess
DynamicLessEqual
DynamicGreater
DynamicGreaterEqual
```

Evo-Script v0.1 prohíbe comparison directa sobre `dynamic`; se requiere explicit conversion a un concrete type antes de comparar.

## 17. Current Numeric Instruction Inventory

```text
NumericKind

Negate
Add
Subtract
Multiply
Divide
Remainder

EqualNumeric
NotEqualNumeric
LessNumeric
LessEqualNumeric
GreaterNumeric
GreaterEqualNumeric

LiftDynamic
DynamicNegate
DynamicAdd
DynamicSubtract
DynamicMultiply
DynamicDivide
DynamicRemainder
```

## 18. Closure

```text
NumericKind                              ✅ CLOSED
fixed unary numeric lowering             ✅ CLOSED
fixed arithmetic                         ✅ CLOSED
checked overflow semantics               ✅ CLOSED
fixed divide/remainder errors            ✅ CLOSED
fixed numeric comparisons                ✅ CLOSED
dynamic excluded from NumericKind        ✅ CLOSED
LiftDynamic                              ✅ CLOSED
dynamic context pre-arithmetic lifting   ✅ CLOSED
DynamicNegate                            ✅ CLOSED
DynamicAdd/Subtract/Multiply             ✅ CLOSED
DynamicDivide/Remainder                  ✅ CLOSED
same-family dynamic runtime dispatch     ✅ CLOSED
DynamicNumericTypeError boundary         ✅ CLOSED
no dynamic comparisons                   ✅ CLOSED
concrete contract boundary               ✅ CLOSED

Control Flow / short-circuit             ← NEXT
Conversion Instructions                  PENDING
Composite Value Instructions             PENDING
SourceMap                                PENDING
```
