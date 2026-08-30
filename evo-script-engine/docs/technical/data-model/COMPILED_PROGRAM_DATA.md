# Evo-Script Engine — Compiled Program / Bytecode Data

Status: COMPILED PROGRAM / BYTECODE DATA — CLOSED

Este documento es la autoridad raíz del producto persistente producido por Bytecode Compiler y consumido directamente por la Stack VM de `evo-script-engine` v0.

El detalle de cada familia se encuentra en los documentos especializados y el inventario final se cierra en `COMPILED_PROGRAM_INVENTORY.md`.

```text
Semantic Program
    ↓ Bytecode Compiler
Compiled Program
    ↓ Stack VM
Execution Result
```

## 1. Responsibility

Regla canónica:

> `Semantic Program` representa significado resuelto; `Compiled Program` representa mecanismo ejecutable persistente que la VM consume sin volver al AST ni al Semantic Program.

Consecuencias:

```text
no name resolution in VM
no type inference in VM
no semantic validation in VM
no Active Scope
no Host Session State
no Current Provider
no ambient provider lookup
```

`CompiledProgram` puede sobrevivir al Source Text y al Compilation Working State sin borrowearlos.

## 2. Closed Root Representation

```rust
struct CompiledProgram {
    functions: Vec<CompiledFunction>,
    entry_point: FunctionId,
    constants: Vec<Constant>,
    external_symbols: Vec<ExternalSymbol>,
    source_map: SourceMap,
}
```

```rust
struct CompiledFunction {
    parameter_count: usize,
    local_count: usize,
    max_operand_depth: usize,
    instructions: Vec<Instruction>,
}
```

Cardinalities:

```text
functions         1..N
entry_point       exactly 1 valid FunctionId
constants         0..N
external_symbols  0..N
source_map        exactly 1
```

`FunctionId` se preserva:

```text
SemanticProgram.functions[n]
    ↕
CompiledProgram.functions[n]
```

No existe `CompiledFunctionId`.

## 3. Persistent Compiled Identities

```rust
struct ConstantId(usize);
struct ExternalSymbolId(usize);
struct ParameterSlot(usize);
struct LocalSlot(usize);
struct InstructionIndex(usize);
struct FieldIndex(usize);
struct VariantDiscriminant(usize);
```

Owner rules:

```text
ConstantId(n)
    → CompiledProgram.constants[n]

ExternalSymbolId(n)
    → CompiledProgram.external_symbols[n]

InstructionIndex(n)
    → CompiledFunction.instructions[n]
```

`ParameterSlot`, `LocalSlot` e `InstructionIndex` son locales a una `CompiledFunction`.

## 4. Semantic Data Lowering Boundary

Semantic data no persiste cuando ya fue traducido a mecanismo físico.

```text
TypeId              → executable mechanism
BindingId           → ParameterSlot / LocalSlot
FieldId             → FieldIndex
VariantId           → VariantDiscriminant
SignatureId         → ExternalSymbolId when required
SignatureBindingId  → erased / ExternalSymbolId
SemanticLiteral     → Constant
SemanticExpression  → Instructions
SourceSpan           → SourceMap entry
```

No sobreviven por costumbre:

```text
TypeId
BindingId
FieldId
VariantId
SignatureId
SignatureBindingId
SemanticExpression
SemanticStatement
parameter type metadata
local type metadata
```

## 5. Signature Dependency Erasure

Signature Dependencies no son Values de primer orden.

```text
Signature Dependency Parameter
    → no ParameterSlot
    → no operand Value

Signature Dependency Argument
    → no physical Value argument
```

Direct Signature y Signature Dependency calls convergen:

```text
DirectSignature(SignatureId)
SignatureDependency(SignatureBindingId)
        ↓ Bytecode Compiler
ExternalSymbolId
        ↓
CallExternal(ExternalSymbolId)
```

No existen:

```text
SignatureSlot
Function Value
closure generated for dependency forwarding
```

## 6. Storage Data

Cerrado en `COMPILED_STORAGE_DATA.md`.

### ExternalSymbol

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
}
```

`parameter_count` cuenta únicamente `SemanticSignatureParameter::Value`.

Signature Dependency Parameters cuentan cero.

No contiene Provider, runtime binding, parameter TypeIds ni result TypeId.

### Constant

```rust
enum Constant {
    Boolean(bool),
    String(String),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),

    Float32(f32),
    Float64(f64),

    Dynamic(DynamicConstant),
}
```

Canonical physical lowering:

```text
semantic int   → Int32
semantic int32 → Int32

semantic float   → Float64
semantic float64 → Float64
```

No existen `Constant::Int` ni `Constant::Float` separados.

### DynamicConstant

```rust
enum DynamicConstant {
    Integer {
        negative: bool,
        magnitude: Vec<u8>,
    },
    Float32(f32),
    Float64(f64),
}
```

Integer magnitude es minimal unsigned big-endian; zero usa empty magnitude + `negative = false`.

## 7. Core Data Movement and Calls

Cerrado en `COMPILED_CORE_CALL_INSTRUCTIONS.md`.

```rust
LoadConstant(ConstantId)
LoadParameter(ParameterSlot)
LoadLocal(LocalSlot)
StoreLocal(LocalSlot)

Call(FunctionId)
CallExternal(ExternalSymbolId)
```

Stack contracts:

```text
LoadConstant    0 → 1
LoadParameter   0 → 1
LoadLocal       0 → 1
StoreLocal      1 → 0

Call            N → 1
CallExternal    N → 1
```

Internal call arity proviene de `CompiledFunction.parameter_count`.
External call arity proviene de `ExternalSymbol.parameter_count`.

`StoreLocal` representa initial materialization, no mutability semántica.

## 8. Numeric Execution

Cerrado en `COMPILED_NUMERIC_INSTRUCTIONS.md`.

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

Exactamente 12 variants.

Fixed arithmetic/comparison:

```text
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
```

Dynamic mechanics:

```text
LiftDynamic
DynamicNegate
DynamicAdd
DynamicSubtract
DynamicMultiply
DynamicDivide
DynamicRemainder
```

Dynamic runtime dispatch se restringe a:

```text
Integer
Float32
Float64
```

No existen Dynamic comparison instructions.

## 9. Control Flow

Cerrado en `COMPILED_CONTROL_FLOW.md`.

```rust
Jump(InstructionIndex)
JumpIfFalse(InstructionIndex)
Discard
Return
```

Branches usan absolute `InstructionIndex`.

`&&` y `||` se reducen a branching real de short-circuit.

No existen eager `AndBoolean` / `OrBoolean` ni `JumpIfTrue` en v0.

## 10. Explicit Conversions

Cerrado en `COMPILED_CONVERSIONS.md`.

```rust
ConvertNumeric {
    source: NumericKind,
    target: NumericKind,
}
ConvertDynamic(NumericKind)
NumericToString(NumericKind)
DynamicToString
```

No hay implicit conversion ni string → numeric parsing.

## 11. Scalar Equality

Cerrado en `COMPILED_SCALAR_EQUALITY.md`.

```rust
NotBoolean
EqualBoolean
NotEqualBoolean
EqualString
NotEqualString
```

Bool y string no poseen ordering operators en v0.

## 12. Composite Layout

Cerrado en `COMPILED_COMPOSITE_LAYOUT.md`.

```rust
struct FieldIndex(usize);
struct VariantDiscriminant(usize);
```

```text
FieldId(n)   → FieldIndex(n)
VariantId(n) → VariantDiscriminant(n)
```

Conceptual runtime contract:

```text
Struct Value
└── ordered fields

Enum Value
├── VariantDiscriminant
└── Payload
    ├── Simple
    ├── Associated(Value)
    └── Structured(ordered fields)
```

No existen runtime TypeId/layout tables en v0.

## 13. Composite Instructions

Cerrado en `COMPILED_COMPOSITE_INSTRUCTIONS.md`.

```rust
ConstructStruct {
    field_order: Vec<FieldIndex>,
}
GetField(FieldIndex)

ConstructEnumSimple(VariantDiscriminant)
ConstructEnumAssociated(VariantDiscriminant)
ConstructEnumStructured {
    variant: VariantDiscriminant,
    field_order: Vec<FieldIndex>,
}

TestVariant(VariantDiscriminant)
ExtractEnumAssociated
ExtractEnumStructured {
    fields: Vec<FieldIndex>,
}
```

Construction preserva source evaluation order y produce canonical storage order.

Enum extraction consume el Enum después de confirmar la variant; el Instruction Set no exige owner/payload aliasing.

## 14. Structural Equality

Cerrado en `COMPILED_STRUCTURAL_EQUALITY.md`.

```rust
enum EqualityRule {
    Numeric(NumericKind),
    Boolean,
    String,
    Composite(CompositeEqualityPlan),
}

enum CompositeEqualityPlan {
    Struct {
        fields: Vec<EqualityRule>,
    },
    Enum {
        variants: Vec<EnumEqualityPayloadPlan>,
    },
}

enum EnumEqualityPayloadPlan {
    Simple,
    Associated(EqualityRule),
    Structured {
        fields: Vec<EqualityRule>,
    },
}
```

Instructions:

```rust
EqualComposite(CompositeEqualityPlan)
NotEqualComposite(CompositeEqualityPlan)
```

No existe `EqualityRule::Dynamic` ni generic `EqualValue`.

La regla normativa `EqualityComparable` impide que composites con `dynamic` directa o transitivamente lleguen a structural equality.

## 15. SourceMap

Cerrado en `COMPILED_SOURCE_MAP.md`.

```rust
struct SourceMap {
    functions: Vec<Vec<SourceSpan>>,
}
```

```text
SourceMap.functions[f][i]
        ↕
CompiledProgram.functions[f].instructions[i]
```

Cada persistent Instruction posee exactamente un source anchor.

v0 utiliza un único source coordinate space por `CompiledProgram`.

No se introducen SourceId, SourcePath o SourceName en v0.

La nested storage shape queda encapsulada para permitir una futura extensión multi-source.

## 16. Exact Final Inventory

Cerrado en `COMPILED_PROGRAM_INVENTORY.md`.

Resultado:

```text
exact compiled own identities        18
exact Instruction variants           48
exact NumericKind variants           12
exact EqualityRule variants           4
exact CompositeEqualityPlan variants  2
exact EnumEqualityPayloadPlan variants 3

SemanticExpressionKind coverage      10 / 10
SemanticStatement coverage            2 / 2
SemanticFunction lowering             complete
SemanticProgram lowering              complete
```

Identities propias exactas:

```text
01 ConstantId
02 ExternalSymbolId
03 CompiledProgram
04 CompiledFunction
05 ParameterSlot
06 LocalSlot
07 ExternalSymbol
08 Constant
09 DynamicConstant
10 NumericKind
11 Instruction
12 InstructionIndex
13 FieldIndex
14 VariantDiscriminant
15 EqualityRule
16 CompositeEqualityPlan
17 EnumEqualityPayloadPlan
18 SourceMap
```

Reutilizadas y no contadas nuevamente:

```text
FunctionId
SignatureSymbol
SourceSpan
```

## 17. Exact Instruction Variant Count

El `Instruction` enum completo está cerrado en `COMPILED_PROGRAM_INVENTORY.md` con exactamente 48 variants:

```text
Core data movement      4
Calls                   2
Fixed numeric          12
Dynamic numeric         7
Control flow            4
Conversions             4
Scalar bool/string      5
Composite mechanics     8
Structural equality     2
                       ──
TOTAL                   48
```

No se introduce `Opcode` separado.

## 18. VM Boundary

Continúan fuera de este bloque:

```text
InstructionPointer
CallFrame
frame_base runtime value
active execution state
Shared Value Storage
Operand Window runtime bounds
runtime Value representation
runtime Dynamic Integer representation
runtime Struct / Enum backing representation
Application Bindings instance
owned external backing storage
execution lifetime / borrowing state
```

Eso pertenece a `VM Execution Data`.

## 19. Outcome / Diagnostic Boundary

Continúan fuera:

```text
ExecutionOutcome
EvaluationError representation
OverflowError representation
DivisionByZeroError representation
ConversionError representation
DynamicNumericTypeError representation
human diagnostic rendering
line / column presentation
snippet / highlight
```

Eso pertenece a `Outcome / Diagnostic Data`.

## 20. Explicitly Excluded v0

```text
CompiledFunctionId
CompiledBindingId
FrameSlot
OperandSlot
StructLayoutId
EnumLayoutId
CompositeTypeId
RuntimeTypeId
EqualityPlanId
SourceId
SourcePath
SourceName
SourceLocation
Opcode separated from Instruction
Label runtime instruction
JumpIfTrue
StoreParameter
AndBoolean eager
OrBoolean eager
Dynamic comparison instructions
EqualValue generic
Pattern runtime object
When runtime instruction
Function Value
Closure for Signature Dependency
Provider identity
Current Provider
Active Scope
Host Session State
portable bytecode ABI identity
```

## 21. Closure

```text
Compiled Program responsibility             ✅ CLOSED
Compiled root / function structure          ✅ CLOSED
storage identities / constants              ✅ CLOSED
Core Load / Store                            ✅ CLOSED
Internal / External Calls                    ✅ CLOSED
Numeric execution                            ✅ CLOSED
Control Flow                                 ✅ CLOSED
Conversions                                  ✅ CLOSED
Scalar Equality                              ✅ CLOSED
Composite Layout                             ✅ CLOSED
Composite Instructions                       ✅ CLOSED
Structural Equality                          ✅ CLOSED
SourceMap                                    ✅ CLOSED
Exact own identities — 18                    ✅ CLOSED
Exact Instruction variants — 48              ✅ CLOSED
SemanticExpressionKind coverage — 10 / 10    ✅ CLOSED
SemanticStatement coverage — 2 / 2           ✅ CLOSED
SemanticFunction lowering                    ✅ CLOSED
SemanticProgram lowering                     ✅ CLOSED
VM boundary                                  ✅ CLOSED
Outcome / Diagnostic boundary                ✅ CLOSED

Compiled Program / Bytecode Data             ✅ CLOSED

NEXT
    VM Execution Data
```
