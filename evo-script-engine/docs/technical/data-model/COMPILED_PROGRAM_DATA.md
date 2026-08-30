# Evo-Script Engine — Compiled Program / Bytecode Data

Status: COMPILED PROGRAM / BYTECODE DATA — IN ANALYSIS

Este documento define el producto persistente producido por Bytecode Compiler y consumido directamente por la Stack VM de `evo-script-engine` v0.

La autoridad deriva de `TECHNICAL_DESIGN.md`, especialmente TD-003, TD-004, TD-005, TD-007, TD-009, TD-010 y TD-011, de `SEMANTIC_PROGRAM_DATA.md` y de los documentos especializados de este bloque.

```text
Semantic Program
    ↓ Bytecode Compiler
Compiled Program
    ↓ Stack VM
Execution Result
```

## CD-001 — Compiled Program representa mecanismo ejecutable

Status: CLOSED

Regla canónica:

> `Semantic Program` representa significado resuelto; `Compiled Program` representa el mecanismo ejecutable persistente que la VM consume sin volver al AST ni al Semantic Program.

Consecuencias:

1. `Compiled Program` puede sobrevivir al Source Text y al Compilation Working State.
2. La VM no realiza name resolution, type inference ni semantic validation.
3. Semantic identities pueden conservarse solamente cuando siguen siendo una identity técnica útil en el producto compilado.
4. Semantic information que ya fue lowered a layout, constants, symbols o Instructions no se conserva por costumbre.
5. No se introducen Active Scope, Host Session State, Current Provider ni provider lookup ambiental.

## CD-002 — FunctionId se preserva

Status: CLOSED

`FunctionId` se reutiliza como identidad de Internal Function desde Semantic Program hacia Compiled Program.

```text
SemanticProgram.functions[n]
    ↓ Bytecode Compiler preserves function identity ordering
CompiledProgram.functions[n]
```

No se introduce `CompiledFunctionId`.

`FunctionId` no es stable ABI identity ni physical function address.

## CD-003 — ConstantId

Status: CLOSED

```rust
struct ConstantId(usize);
```

```text
ConstantId(n)
    → CompiledProgram.constants[n]
```

Namespace local al `CompiledProgram`; no es address de memoria ni identity estable entre compilaciones.

## CD-004 — ExternalSymbolId

Status: CLOSED

```rust
struct ExternalSymbolId(usize);
```

```text
ExternalSymbolId(n)
    → CompiledProgram.external_symbols[n]
```

No identifica Provider ni runtime binding. Runtime lo resuelve mediante explicit Application Bindings.

## CD-005 — Signature Dependency Erasure

Status: CLOSED

Signature Dependencies no son Values de primer orden y se eliminan como parámetros físicos durante Bytecode lowering.

```text
SignatureBindingId
    → semantic dependency meaning
    → no ParameterSlot
    → ExternalSymbolId when invoked
```

No existen `SignatureSlot`, Function Value ni closure artificial para forwarding.

## CD-006 — Signature Dependency Forwarding se resuelve en compilation

Status: CLOSED

`SemanticArgument::SignatureDependency` no genera Value argument físico. Una internal CALL transporta únicamente Value arguments.

## CD-007 — Direct Signature y Signature Dependency convergen

Status: CLOSED

```text
DirectSignature(SignatureId)
SignatureDependency(SignatureBindingId)
        ↓ Bytecode Compiler
ExternalSymbolId
```

El origen semántico diferente no requiere mecanismo external-call diferente en runtime.

## CD-008 — CompiledProgram root

Status: CLOSED — shell

```rust
struct CompiledProgram {
    functions: Vec<CompiledFunction>,
    entry_point: FunctionId,
    constants: Vec<Constant>,
    external_symbols: Vec<ExternalSymbol>,
    source_map: SourceMap,
}
```

```text
functions         1..N
entry_point       exactly 1 valid FunctionId
constants         0..N
external_symbols  0..N
source_map        exactly 1
```

No se introducen wrappers `FunctionTable`, `ConstantPool` o `ExternalSymbolTable` mientras no agreguen responsabilidad propia.

## CD-009 — CompiledFunction shell

Status: CLOSED

```rust
struct CompiledFunction {
    parameter_count: usize,
    local_count: usize,
    max_operand_depth: usize,
    instructions: Vec<Instruction>,
}
```

`parameter_count` cuenta exclusivamente Value Parameters físicos.

`local_count` cuenta Value bindings estables non-parameter.

`max_operand_depth` expresa la profundidad máxima temporal requerida por la función compilada.

## CD-010 — Semantic data lowered away

Status: CLOSED

Por defecto no sobreviven dentro de `CompiledFunction`:

```text
TypeId
BindingId
FieldId
VariantId
SignatureId
SignatureBindingId
SemanticExpression
SemanticStatement
SemanticFunction.satisfaction
parameter type metadata
local type metadata
```

Lowering:

```text
TypeId              → executable mechanism
BindingId           → ParameterSlot / LocalSlot
FieldId             → FieldIndex
VariantId           → VariantDiscriminant
SignatureId         → ExternalSymbolId
SignatureBindingId  → erased / ExternalSymbolId
SemanticLiteral     → ConstantId / compiled constant data
SemanticExpression  → Instructions
```

## CD-011 — Compiled storage data

Status: CLOSED

Cerrado en `COMPILED_STORAGE_DATA.md`:

```text
ParameterSlot
LocalSlot
BindingId → slot compiler mapping
ExternalSymbol
Constant
DynamicConstant
Constant Pool ownership
```

La separación lógica permanece:

```text
ParameterSlot != LocalSlot
```

aunque ambos compartan Shared Frame Region durante runtime.

## CD-012 — Numeric execution kind

Status: CLOSED

Cerrado en `COMPILED_NUMERIC_INSTRUCTIONS.md`:

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

`NumericKind` expresa mecanismo numérico fijo, no identidad semántica completa.

```text
int     → Int32
int32   → Int32
float   → Float64
float64 → Float64

dynamic ∉ NumericKind
```

## CD-013 — Fixed numeric arithmetic and comparisons

Status: CLOSED

Instructions cerradas conceptualmente:

```text
Negate(NumericKind)
Add(NumericKind)
Subtract(NumericKind)
Multiply(NumericKind)
Divide(NumericKind)
Remainder(NumericKind)

EqualNumeric(NumericKind)
NotEqualNumeric(NumericKind)
LessNumeric(NumericKind)
LessEqualNumeric(NumericKind)
GreaterNumeric(NumericKind)
GreaterEqualNumeric(NumericKind)
```

Fixed arithmetic implementa semántica checked de Evo-Script:

```text
no wrapping
no saturation
overflow → OverflowError
divide/remainder by zero → DivisionByZeroError
```

`Remainder` solo admite integer `NumericKind`.

## CD-014 — Dynamic numeric lifting and arithmetic

Status: CLOSED

```text
LiftDynamic(NumericKind)
DynamicNegate
DynamicAdd
DynamicSubtract
DynamicMultiply
DynamicDivide
DynamicRemainder
```

Regla canónica:

> Cuando una arithmetic subtree se evalúa bajo contexto `dynamic`, fixed operands se elevan antes de ejecutar arithmetic; no se calcula primero bajo width fijo.

Dynamic runtime dispatch queda restringido al universo:

```text
Dynamic Numeric Value
├── Integer
├── Float32
└── Float64
```

Cross-family dynamic arithmetic no realiza coercion implícita y produce `DynamicNumericTypeError`, conforme al amendment normativo `evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`.

No existen Dynamic comparison instructions.

## CD-015 — Control Flow and short-circuit

Status: CLOSED

Cerrado en `COMPILED_CONTROL_FLOW.md`.

Identidad:

```rust
struct InstructionIndex(usize);
```

Instructions base:

```text
Jump(InstructionIndex)
JumpIfFalse(InstructionIndex)
Discard
Return
```

`Instruction` se representa como typed enum; no se introduce un `Opcode` separado con generic operands.

Branches usan absolute `InstructionIndex` local a `CompiledFunction`.

`JumpIfFalse` consume un `bool` y `&&` / `||` se reducen a branching real de short-circuit.

No existen eager instructions:

```text
AndBoolean
OrBoolean
```

`when` reutilizará la misma branch infrastructure; la inspección específica de enum permanece pendiente de Composite Layout.

## CD-016 — Conversion Instructions

Status: CLOSED

Cerrado en `COMPILED_CONVERSIONS.md`.

Instructions:

```rust
ConvertNumeric {
    source: NumericKind,
    target: NumericKind,
}

ConvertDynamic(NumericKind)
NumericToString(NumericKind)
DynamicToString
```

Reglas:

```text
fixed numeric → fixed numeric
    exact representation or ConversionError

dynamic → fixed numeric
    exact representation or ConversionError

fixed numeric → string
    NumericToString

dynamic → string
    DynamicToString
```

`LiftDynamic` continúa siendo mecanismo técnico fixed → dynamic para arithmetic context; Evo-Script v0.1 no define `to_dynamic`.

No se introducen implicit conversions ni string → numeric parsing.

El Technical Data Model no amplía silenciosamente `to_string` a bool/struct/enum mientras la especificación v0.1 no lo declare explícitamente.

## CD-017 — Scalar Boolean / String Equality

Status: CLOSED

Cerrado en `COMPILED_SCALAR_EQUALITY.md`.

Instructions:

```text
NotBoolean
EqualBoolean
NotEqualBoolean
EqualString
NotEqualString
```

`bool` y `string` no poseen ordering operators en Evo-Script v0.1.

String equality compara contenido textual UTF-8, no address ni ownership identity.

General equality queda parcialmente cerrada:

```text
numeric     ✅ CLOSED
bool        ✅ CLOSED
string      ✅ CLOSED
struct      PENDING Composite Layout
enum        PENDING Composite Layout
dynamic     ❌ prohibited by language
```

## CD-018 — Composite Layout

Status: CLOSED

Cerrado en `COMPILED_COMPOSITE_LAYOUT.md`.

Identities físicas:

```rust
struct FieldIndex(usize);
struct VariantDiscriminant(usize);
```

Lowering canónico:

```text
FieldId(n)   → FieldIndex(n)
VariantId(n) → VariantDiscriminant(n)
```

La igualdad numérica de los índices no convierte las identities en el mismo concepto: `FieldId` / `VariantId` pertenecen a Semantic Program; `FieldIndex` / `VariantDiscriminant` pertenecen al mecanismo físico compilado.

Layout conceptual:

```text
Struct Value
└── ordered fields
    ├── FieldIndex(0) → Value
    └── ...

Enum Value
├── VariantDiscriminant
└── Payload
    ├── Simple
    ├── Associated(Value)
    └── Structured(ordered fields)
```

No se introducen en v0:

```text
StructLayoutId
EnumLayoutId
CompositeTypeId
RuntimeTypeId
runtime type lookup table
reflection metadata
field / variant names at runtime
```

La representación física final usa canonical owner ordering. Sin embargo, Bytecode Compiler debe preservar source evaluation order durante composite construction aunque dicho orden difiera del canonical storage order.

Structural equality puede apoyarse después en este layout sin reintroducir `TypeId`.

## CD-019 — Current closure

```text
Compiled Program responsibility          ✅ CLOSED
FunctionId preservation                  ✅ CLOSED
ConstantId                               ✅ CLOSED
ExternalSymbolId                         ✅ CLOSED
Signature Dependency Erasure             ✅ CLOSED
Signature Dependency Forwarding lowering ✅ CLOSED
Direct/Dependency external convergence   ✅ CLOSED
CompiledProgram root shell               ✅ CLOSED
CompiledFunction shell                   ✅ CLOSED
Semantic data lowering boundary          ✅ CLOSED
ParameterSlot / LocalSlot                ✅ CLOSED
Constant / DynamicConstant               ✅ CLOSED
ExternalSymbol                           ✅ CLOSED
NumericKind                              ✅ CLOSED
Fixed arithmetic                         ✅ CLOSED
Fixed numeric comparisons                ✅ CLOSED
LiftDynamic                              ✅ CLOSED
Dynamic arithmetic                       ✅ CLOSED
DynamicNumericTypeError boundary         ✅ CLOSED
Instruction typed-enum representation    ✅ CLOSED
InstructionIndex                         ✅ CLOSED
Control Flow / short-circuit             ✅ CLOSED
Discard / Return                         ✅ CLOSED
Conversion Instructions                  ✅ CLOSED
Boolean equality / negation              ✅ CLOSED
String equality                          ✅ CLOSED
FieldIndex                               ✅ CLOSED
VariantDiscriminant                      ✅ CLOSED
Composite Layout                         ✅ CLOSED

Struct / Enum Instructions               ← NEXT
Struct / Enum equality                   PENDING
SourceMap                                PENDING
Compiled Program exact inventory         PENDING
```
