# Evo-Script Engine — Compiled Numeric Instructions

Status: CLOSED

Este documento cierra las identidades y reglas de bytecode para arithmetic numérico fijo, numeric comparisons y dynamic arithmetic de `evo-script-engine` v0.

La autoridad deriva de:

- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_STORAGE_DATA.md`;
- `SEMANTIC_PROGRAM_DATA.md`;
- `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`;
- `evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`.

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

Todas las operaciones sobre fixed numeric kinds implementan directamente la semántica Evo-Script.

```text
fixed overflow
    → OverflowError

no wrapping
no saturation
no unchecked arithmetic
```

No se introducen opcodes alternativos `CheckedAdd`, `WrappingAdd`, `SaturatingAdd` o equivalentes porque Evo-Script v0 posee una sola semántica observable.

## 5. Divide

`Divide(NumericKind)` conserva las reglas del kind:

```text
signed integer  → quotient truncated toward zero
unsigned integer → unsigned quotient
floating         → floating quotient
```

Para cualquier kind válido:

```text
divisor numerically zero
    → DivisionByZeroError
```

Esto incluye `0.0` y `-0.0` para floating kinds; la VM no produce Infinity o NaN silenciosamente por división entre cero.

Para signed fixed integer:

```text
MIN_VALUE / -1
    → OverflowError
```

## 6. Remainder

```rust
Instruction::Remainder(NumericKind)
```

Invariante de validez:

```text
NumericKind must be integer
```

`Remainder(Float32)` y `Remainder(Float64)` son estados compilados inválidos y Bytecode Compiler no los produce.

Semántica:

```text
integer divisor zero
    → DivisionByZeroError

signed MIN_VALUE % -1
    → OverflowError
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

Bytecode Compiler ya comprobó exact type compatibility; la VM no realiza type inference ni coercion.

`EqualNumeric` y `NotEqualNumeric` son explícitamente numéricas porque Evo-Script también define equality sobre bool, string, struct y enum mediante mecanismos que se cerrarán por separado.

Ordering solo existe para concrete numeric kinds.

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

No contienen `NumericKind` porque el payload family de un Value semánticamente `dynamic` puede conocerse solamente durante runtime.

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

Cross-family arithmetic es inválida durante evaluación:

```text
Integer with Float32
Integer with Float64
Float32 with Float64
```

y produce:

```text
DynamicNumericTypeError
```

según el amendment normativo `DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`.

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
    → DivisionByZeroError
```

La representación física de arbitrary-precision runtime values pertenece a VM Execution Data y no se prescribe aquí.

## 14. Dynamic Floating Semantics

Dynamic Float32 conserva `f32` semantics y Dynamic Float64 conserva `f64` semantics.

No existe silent promotion Float32 → Float64.

División por `0.0` o `-0.0` produce `DivisionByZeroError`.

`DynamicRemainder` solo es válido cuando ambos payloads son Dynamic Integer. Payloads Float32/Float64 producen `DynamicNumericTypeError` porque `%` no pertenece a sus familias válidas en Evo-Script v0.

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
