# Evo-Script Engine — Exact Compiled Program Inventory

Status:
- HISTORICAL DESIGN: CLOSED / PRESERVED (21 own identities)
- CURRENT RECONCILED MODEL: CLOSED (18 own identities / Instruction: exactly 48 variants)
- Authority: [`../EVO_VALUES_V0_1_RECONCILIATION.md`](../EVO_VALUES_V0_1_RECONCILIATION.md)

Este documento consolida el inventario exacto de `Compiled Program / Bytecode Data` de `evo-script-engine` v0 reconciliado con `evo-values v0.1`.

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

Compiled Program / Bytecode Data                 ✅ CLOSED (reconciled)
```

## 2. Exact Own Identity Count

`Compiled Program / Bytecode Data` vigente contiene exactamente **18 identities técnicas propias** (21 identidades históricas preservadas, de las cuales 3 quedan superseded por la reconciliación con `evo-values v0.1`).

### Current Reconciled Categorization — 18 identities

```text
Program IDs / coordinates                 3
Program / Function structures             2
Physical storage / executable data        5
Instruction mechanism                     3
Composite physical identities             2
Source mapping                            1
Boundary executable contract              2
                                          ──
TOTAL                                     18
```

#### Program IDs / coordinates — 3

```text
01 ConstantId
02 ExternalSymbolId
03 CompiledValueShapeId
```

#### Program / Function structures — 2

```text
04 CompiledProgram
05 CompiledFunction
```

#### Physical storage / persistent executable data — 5

```text
06 ParameterSlot
07 LocalSlot
08 ExternalSymbol
09 Constant
10 DynamicConstant
```

#### Instruction mechanism — 3

```text
11 NumericKind
12 Instruction
13 InstructionIndex
```

#### Composite physical identities — 2

```text
14 FieldIndex
15 VariantDiscriminant
```

#### Source mapping — 1

```text
16 SourceMap
```

#### Boundary executable contract — 2

```text
17 CompiledValueShape
18 CompiledEnumValueShape
```

### Historical Identities (SUPERSEDED BY evo-values v0.1 RECONCILIATION)

En el diseño histórico v0 se definieron 3 identidades para los planes de igualdad estructural:

```text
EqualityRule
CompositeEqualityPlan
EnumEqualityPayloadPlan
```

Trazabilidad histórica de conteo:

```text
21 historical identities
→ -3 superseded equality-plan identities
→ 18 current reconciled identities
```

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
    EqualComposite,
    NotEqualComposite,
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

## 9. Structural Equality Plan Inventory (HISTORICAL / SUPERSEDED)

> [!NOTE]
> **SUPERSEDED BY evo-values v0.1 RECONCILIATION**: Los planes de igualdad técnica han sido eliminados del modelo de datos de Compiled Program. La igualdad estructural delega directamente en `evo_values::comparison::EQUAL` y `evo_values::comparison::NOT_EQUAL` sobre `Value`. No existen variantes de plan en el inventario actual.

Inventario histórico preservado:

```text
EqualityRule variants              4 (historical / superseded)
CompositeEqualityPlan variants     2 (historical / superseded)
EnumEqualityPayloadPlan variants   3 (historical / superseded)
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
Compiled own identities             18 (21 historical)
Instruction variants                48
NumericKind variants                12
CompiledValueShape variants         17
CompiledEnumValueShape variants      3
SemanticExpressionKind coverage     10 / 10
SemanticStatement coverage           2 / 2
```

Historical superseded plan counts (not present in current inventory):

```text
EqualityRule variants                4 (historical / superseded)
CompositeEqualityPlan variants       2 (historical / superseded)
EnumEqualityPayloadPlan variants     3 (historical / superseded)
```

## Closure

```text
Compiled Program exact inventory             ✅ CLOSED — 18 identities (reconciled)
Instruction exact inventory                  ✅ CLOSED — 48 variants
Boundary executable contract inventory       ✅ CLOSED
Semantic → Compiled coverage                  ✅ CLOSED
VM Execution leakage                         ❌ NONE
Outcome/Diagnostic representation leakage    ❌ NONE

HISTORICAL NEXT STAGE — HISTORICAL / PRESERVED
    VM Execution exact inventory
    ✅ subsequently completed / preserved

CURRENT RECONCILIATION NEXT
    TASK-ESE-RECON-003
    Reconciliar Runtime Data documental
```