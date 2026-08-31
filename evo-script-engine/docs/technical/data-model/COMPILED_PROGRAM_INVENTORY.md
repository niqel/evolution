# Evo-Script Engine — Exact Compiled Program Inventory

Status: CLOSED — REVALIDATED AFTER COMPILED BOUNDARY SHAPE CORRECTION

Este documento consolida el inventario exacto de `Compiled Program / Bytecode Data` de `evo-script-engine` v0 después de cerrar también `COMPILED_BOUNDARY_VALUE_SHAPE.md`.

La corrección agrega boundary executable contract metadata para validar:

```text
Consumer Invocation Values
ExternalCapability success result
```

sin cambiar el Instruction Set ni reintroducir Semantic Program en runtime.

## 1. Final Review Result

```text
Compiled Program responsibility                 ✅
Compiled root / function structure              ✅
physical storage identities                     ✅
constant representation                         ✅
external symbolic calling convention            ✅
Compiled Boundary Value Shape                   ✅
entry boundary validation metadata              ✅
external result validation metadata             ✅
core Load / Store                                ✅
internal / external Calls                       ✅
fixed numeric execution                         ✅
dynamic numeric execution                       ✅
control flow / short-circuit                     ✅
explicit conversions                            ✅
scalar equality                                 ✅
composite layout / construction                  ✅
when lowering / extraction                       ✅
structural equality                             ✅
Source Mapping                                  ✅
SemanticExpressionKind coverage                  ✅
SemanticStatement coverage                       ✅
Semantic Function coverage                       ✅
no AST dependency                               ✅
no general runtime name/type resolution          ✅
no VM Execution Data leakage                     ✅
no Outcome / Diagnostic Data representation      ✅

Compiled Program / Bytecode Data                 ✅ CLOSED
```

## 2. Exact Own Identity Count

`Compiled Program / Bytecode Data` v0 contiene exactamente **21 identities técnicas propias**.

### Program identities — 3

```text
01 ConstantId
02 ExternalSymbolId
03 CompiledValueShapeId
```

### Program / Function structures — 2

```text
04 CompiledProgram
05 CompiledFunction
```

### Physical storage / persistent executable data — 5

```text
06 ParameterSlot
07 LocalSlot
08 ExternalSymbol
09 Constant
10 DynamicConstant
```

### Instruction mechanism — 3

```text
11 NumericKind
12 Instruction
13 InstructionIndex
```

### Composite physical identities — 2

```text
14 FieldIndex
15 VariantDiscriminant
```

### Structural equality plan — 3

```text
16 EqualityRule
17 CompositeEqualityPlan
18 EnumEqualityPayloadPlan
```

### Source mapping — 1

```text
19 SourceMap
```

### Boundary executable contract — 2

```text
20 CompiledValueShape
21 CompiledEnumValueShape
```

Equivalent chronological list preserving the original first 18 identities:

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

The chronological list is the canonical numbering used when discussing the 18→21 correction.

## 3. Reused Identities Not Counted Again

```text
FunctionId       ← Semantic Program Data
SignatureSymbol  ← Semantic Program Data
SourceSpan       ← Lexical Data
```

No se cuentan fields/containers como identities independientes:

```text
parameter_count
local_count
max_operand_depth
entry_parameter_shapes
value_shapes Vec
field_order
Vec containers
usize backing values
```

## 4. Corrected Root Representation

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

Invariante nueva:

```text
entry_parameter_shapes.len()
    == functions[entry_point].parameter_count
```

`CompiledFunction` permanece sin metadata de parameter shapes porque internal calls ya están semanticamente validadas.

## 5. Corrected ExternalSymbol

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
    result_shape: CompiledValueShapeId,
}
```

`parameter_count` continúa contando solo Value Parameters físicos.

`result_shape` permite comprobar el `OwnedValue` retornado por `ExternalCapability` antes de materializarlo y antes del stack commit `N → 1`.

No persiste external parameter-shape list.

## 6. Boundary Value Shape Inventory

```rust
struct CompiledValueShapeId(usize);
```

```text
CompiledValueShapeId(n)
    → CompiledProgram.value_shapes[n]
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

Exactamente **17 variants**.

```rust
enum CompiledEnumValueShape {
    Simple,
    Associated(CompiledValueShapeId),
    Structured {
        fields: Vec<CompiledValueShapeId>,
    },
}
```

Exactamente **3 variants**.

Boundary-shape metadata se usa únicamente para:

```text
entry Invocation Value validation
external result validation
```

No es runtime reflection general.

## 7. NumericKind Exact Inventory

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

Exactamente **12 variants**.

## 8. Exact Instruction Enum

`Instruction` continúa conteniendo exactamente **48 variants**.

```rust
enum Instruction {
    // Core data movement — 4
    LoadConstant(ConstantId),
    LoadParameter(ParameterSlot),
    LoadLocal(LocalSlot),
    StoreLocal(LocalSlot),

    // Calls — 2
    Call(FunctionId),
    CallExternal(ExternalSymbolId),

    // Fixed numeric — 12
    Negate(NumericKind),
    Add(NumericKind),
    Subtract(NumericKind),
    Multiply(NumericKind),
    Divide(NumericKind),
    Remainder(NumericKind),
    EqualNumeric(NumericKind),
    NotEqualNumeric(NumericKind),
    LessNumeric(NumericKind),
    LessEqualNumeric(NumericKind),
    GreaterNumeric(NumericKind),
    GreaterEqualNumeric(NumericKind),

    // Dynamic numeric — 7
    LiftDynamic(NumericKind),
    DynamicNegate,
    DynamicAdd,
    DynamicSubtract,
    DynamicMultiply,
    DynamicDivide,
    DynamicRemainder,

    // Control flow — 4
    Jump(InstructionIndex),
    JumpIfFalse(InstructionIndex),
    Discard,
    Return,

    // Explicit conversions — 4
    ConvertNumeric {
        source: NumericKind,
        target: NumericKind,
    },
    ConvertDynamic(NumericKind),
    NumericToString(NumericKind),
    DynamicToString,

    // Scalar bool / string — 5
    NotBoolean,
    EqualBoolean,
    NotEqualBoolean,
    EqualString,
    NotEqualString,

    // Composite mechanics — 8
    ConstructStruct {
        field_order: Vec<FieldIndex>,
    },
    GetField(FieldIndex),
    ConstructEnumSimple(VariantDiscriminant),
    ConstructEnumAssociated(VariantDiscriminant),
    ConstructEnumStructured {
        variant: VariantDiscriminant,
        field_order: Vec<FieldIndex>,
    },
    TestVariant(VariantDiscriminant),
    ExtractEnumAssociated,
    ExtractEnumStructured {
        fields: Vec<FieldIndex>,
    },

    // Structural equality — 2
    EqualComposite(CompositeEqualityPlan),
    NotEqualComposite(CompositeEqualityPlan),
}
```

Count:

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

## 9. Structural Equality Plan Inventory

```text
EqualityRule variants              4
CompositeEqualityPlan variants     2
EnumEqualityPayloadPlan variants   3
```

No existe `EqualityRule::Dynamic`.

## 10. SourceMap

```rust
struct SourceMap {
    functions: Vec<Vec<SourceSpan>>,
}
```

Dense mapping:

```text
SourceMap.functions[f][i]
    ↔ CompiledProgram.functions[f].instructions[i]
```

## 11. Semantic Coverage

El lowering continúa completo:

```text
SemanticExpressionKind  10 / 10
SemanticStatement        2 / 2
SemanticFunction         complete
SemanticProgram          complete
```

La corrección boundary-shape añade:

```text
entry Semantic Value Parameter TypeId
    → CompiledValueShapeId

external SemanticSignature.result_type
    → ExternalSymbol.result_shape
```

solo cuando la shape es necesaria en una frontera ejecutable.

El compiler puede mantener temporalmente:

```text
TypeId → CompiledValueShapeId
```

para preservar sharing y emitir solo shapes boundary-reachable.

## 12. Explicitly Not Introduced by Correction

```text
RuntimeTypeId
TypeId in RuntimeValue
SemanticType persistence
per-function parameter-shape list
external parameter-shape list
general Runtime Type Table
reflection API
hash/fingerprint as sole compatibility proof
new Instruction variants
```

## 13. Final Counts

```text
Compiled own identities             21
Instruction variants                48
NumericKind variants                12
CompiledValueShape variants         17
CompiledEnumValueShape variants      3
EqualityRule variants                4
CompositeEqualityPlan variants       2
EnumEqualityPayloadPlan variants     3
SemanticExpressionKind coverage     10 / 10
SemanticStatement coverage           2 / 2
```

## Closure

```text
Compiled Program exact inventory             ✅ CLOSED — 21 identities
Instruction exact inventory                  ✅ CLOSED — 48 variants
Boundary executable contract inventory       ✅ CLOSED
Semantic → Compiled coverage                  ✅ CLOSED
VM Execution leakage                         ❌ NONE
Outcome/Diagnostic representation leakage    ❌ NONE

NEXT
    VM Execution exact inventory
```