# Evo-Script Engine — Compiled Program / Bytecode Data

Status: COMPILED PROGRAM / BYTECODE DATA — CLOSED — REVALIDATED AFTER BOUNDARY SHAPE CORRECTION

Este documento es la autoridad raíz del producto persistente producido por Bytecode Compiler y consumido directamente por la Stack VM de `evo-script-engine` v0.

```text
Semantic Program
    ↓ Bytecode Compiler
Compiled Program
    ↓ Stack VM
Execution Result
```

## 1. Responsibility

> `Semantic Program` representa significado resuelto; `Compiled Program` representa mecanismo ejecutable persistente que la VM consume sin volver al AST ni al Semantic Program.

Consecuencias:

```text
no name resolution in VM
no type inference in VM
no semantic validation in ordinary bytecode execution
no Active Scope
no Host Session State
no Current Provider
no ambient provider lookup
```

La corrección de `Compiled Boundary Value Shape` agrega únicamente contrato ejecutable para validar Values que cruzan fronteras externas. No convierte a `CompiledProgram` en un segundo Semantic Program.

## 2. Closed Root Representation

```rust
struct CompiledProgram {
    functions: Vec<CompiledFunction>,
    entry_point: FunctionId,
    entry_parameter_shapes: Vec<CompiledValueShapeId>,
    constants: Vec<Constant>,
    external_symbols: Vec<ExternalSymbol>,
    value_shapes: Vec<CompiledValueShape>,
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

Cardinalidades:

```text
functions               1..N
entry_point             exactly 1 valid FunctionId
entry_parameter_shapes  exactly entry Value-parameter count
constants               0..N
external_symbols        0..N
value_shapes             0..N
source_map               exactly 1
```

Invariante:

```text
entry_parameter_shapes.len()
    == functions[entry_point].parameter_count
```

`FunctionId` se preserva desde `SemanticProgram.functions`.

## 3. Persistent Compiled Identities

Program-level identities:

```rust
struct ConstantId(usize);
struct ExternalSymbolId(usize);
struct CompiledValueShapeId(usize);
```

Function/storage identities:

```rust
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

CompiledValueShapeId(n)
    → CompiledProgram.value_shapes[n]

InstructionIndex(n)
    → CompiledFunction.instructions[n]
```

## 4. Semantic Data Lowering Boundary

```text
TypeId
    → executable mechanism
    → and, only when boundary-reachable,
      CompiledValueShapeId

BindingId           → ParameterSlot / LocalSlot
FieldId             → FieldIndex
VariantId           → VariantDiscriminant
SignatureId         → ExternalSymbolId when required
SignatureBindingId  → erased / ExternalSymbolId
SemanticLiteral     → Constant
SemanticExpression  → Instructions
SourceSpan           → SourceMap entry
```

No persisten por costumbre:

```text
TypeId
SemanticType
BindingId
FieldId
VariantId
SignatureId
SignatureBindingId
SemanticExpression
SemanticStatement
local type metadata
per-function parameter type metadata
```

`CompiledValueShape` persiste solo para boundary validation y no para ordinary bytecode dispatch.

## 5. Signature Dependency Erasure

Signature Dependencies continúan sin ser Values físicos.

```text
Signature Dependency Parameter
    → no ParameterSlot
    → no Invocation Value

Signature Dependency Argument
    → no operand Value
```

Direct Signature y Signature Dependency calls convergen a:

```text
ExternalSymbolId
    → CallExternal(ExternalSymbolId)
```

## 6. Storage Data

### ExternalSymbol

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
    result_shape: CompiledValueShapeId,
}
```

`parameter_count` cuenta solo Value Parameters físicos.

`result_shape` permite validar el `OwnedValue` retornado por la `ExternalCapability` uniforme antes de materializarlo como `RuntimeValue` y antes del commit `N → 1`.

No persisten external argument shapes.

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

Canonical lowering:

```text
int / int32     → Int32
float / float64 → Float64
```

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

Integer usa minimal unsigned big-endian magnitude y zero canónico no-negativo.

## 7. Compiled Boundary Value Shape

Cerrado en `COMPILED_BOUNDARY_VALUE_SHAPE.md`.

```rust
struct CompiledValueShapeId(usize);
```

```rust
enum CompiledValueShape {
    Boolean,

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

    String,
    Dynamic,

    Struct {
        fields: Vec<CompiledValueShapeId>,
    },

    Enum {
        variants: Vec<CompiledEnumValueShape>,
    },
}
```

Exactamente 17 variants.

```rust
enum CompiledEnumValueShape {
    Simple,
    Associated(CompiledValueShapeId),
    Structured {
        fields: Vec<CompiledValueShapeId>,
    },
}
```

Exactamente 3 variants.

`CompiledProgram.value_shapes` conserva únicamente shapes boundary-reachable desde:

```text
entry Value Parameters
external result types
```

Uso runtime permitido:

```text
Invocation Value validation
ExternalCapability result validation
```

No se utiliza para arithmetic, field access, calls internas, equality, RuntimeValue dispatch ni reflection.

## 8. Core Data Movement and Calls

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

El Instruction Set permanece sin cambios por la corrección de boundary shapes.

## 9. Numeric Execution

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

Fixed arithmetic/comparison continúa mediante Instructions tipadas por `NumericKind`; Dynamic continúa restringido a Integer / Float32 / Float64 y sin comparison instructions propias.

## 10. Control Flow

```rust
Jump(InstructionIndex)
JumpIfFalse(InstructionIndex)
Discard
Return
```

Branches usan absolute `InstructionIndex`. `&&` y `||` continúan lowered mediante branching real de short-circuit.

## 11. Explicit Conversions

```rust
ConvertNumeric {
    source: NumericKind,
    target: NumericKind,
}
ConvertDynamic(NumericKind)
NumericToString(NumericKind)
DynamicToString
```

No existen implicit conversions en bytecode ni en boundary validation.

## 12. Scalar Equality

```text
NotBoolean
EqualBoolean
NotEqualBoolean
EqualString
NotEqualString
```

## 13. Composite Layout

```rust
struct FieldIndex(usize);
struct VariantDiscriminant(usize);
```

Ordinary runtime composite layout continúa sin `TypeId` ni runtime layout tables.

`CompiledValueShape` no reemplaza este mecanismo; solo valida external boundary data.

## 14. Composite Instructions

```text
ConstructStruct
GetField
ConstructEnumSimple
ConstructEnumAssociated
ConstructEnumStructured
TestVariant
ExtractEnumAssociated
ExtractEnumStructured
```

Exactamente 8 composite-mechanics instruction variants dentro del enum general.

## 15. Structural Equality

```rust
enum EqualityRule {
    Numeric(NumericKind),
    Boolean,
    String,
    Composite(CompositeEqualityPlan),
}

enum CompositeEqualityPlan {
    Struct { fields: Vec<EqualityRule> },
    Enum { variants: Vec<EnumEqualityPayloadPlan> },
}

enum EnumEqualityPayloadPlan {
    Simple,
    Associated(EqualityRule),
    Structured { fields: Vec<EqualityRule> },
}
```

Dynamic sigue excluido de structural equality comparability.

## 16. SourceMap

```rust
struct SourceMap {
    functions: Vec<Vec<SourceSpan>>,
}
```

Dense mapping:

```text
(FunctionId, InstructionIndex)
    → exactly one SourceSpan
```

## 17. Exact Final Inventory

Cerrado en `COMPILED_PROGRAM_INVENTORY.md`.

Resultado corregido:

```text
exact compiled own identities         21
exact Instruction variants            48
exact NumericKind variants            12
exact CompiledValueShape variants     17
exact CompiledEnumValueShape variants  3
exact EqualityRule variants            4
exact CompositeEqualityPlan variants   2
exact EnumEqualityPayloadPlan variants 3
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
19 CompiledValueShapeId
20 CompiledValueShape
21 CompiledEnumValueShape
```

Reused y no recontadas:

```text
FunctionId
SignatureSymbol
SourceSpan
```

## 18. Exact Instruction Variant Count

Continúan exactamente 48:

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

## 19. VM Boundary

Continúan fuera de Compiled Program:

```text
InstructionPointer
CallFrame
frame_base
SharedValueStorage
RuntimeValue
ExecutionBackingStore
ApplicationBindings
VmExecution mutable state
```

## 20. Outcome / Diagnostic Boundary

Continúan fuera:

```text
ExecutionOutcome
EvaluationError exact representation
ExternalCapability failure type
human diagnostic rendering
line / column presentation
```

Boundary shape mismatch detection sí es posible desde Compiled Program; su Failure representation pertenece a Outcome / Diagnostic Data.

## 21. Explicitly Excluded v0

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
external parameter shape list
per-function parameter shape lists
general reflection table
```

## Closure

```text
Compiled Program responsibility               ✅ CLOSED
Compiled root / function structure            ✅ CLOSED — corrected root
storage identities / constants                ✅ CLOSED
Compiled Boundary Value Shape                 ✅ CLOSED
entry Invocation Value validation metadata     ✅ CLOSED
external result validation metadata            ✅ CLOSED
Core Load / Store                              ✅ CLOSED
Internal / External Calls                      ✅ CLOSED
Numeric execution                              ✅ CLOSED
Control Flow                                   ✅ CLOSED
Conversions                                    ✅ CLOSED
Scalar Equality                                ✅ CLOSED
Composite Layout                               ✅ CLOSED
Composite Instructions                         ✅ CLOSED
Structural Equality                            ✅ CLOSED
SourceMap                                      ✅ CLOSED
Exact own identities — 21                      ✅ CLOSED
Exact Instruction variants — 48                ✅ CLOSED

Compiled Program / Bytecode Data               ✅ CLOSED

NEXT
    VM Execution exact inventory
```