# Evo-Script Engine — Exact Compiled Program Inventory

Status: CLOSED

Este documento consolida la revisión final de `Compiled Program / Bytecode Data` de `evo-script-engine` v0 después de cerrar storage, calls, numeric execution, control flow, conversions, composite mechanics, structural equality y Source Mapping.

La revisión valida:

```text
Semantic Program → Compiled Program coverage
exact compiled identity inventory
exact Instruction variant inventory
cardinality / owner consistency
absence of unresolved semantic work in VM
absence of VM Execution Data leakage
absence of Outcome / Diagnostic Data leakage
```

Autoridad utilizada:

- `SEMANTIC_PROGRAM_INVENTORY.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `SEMANTIC_PROGRAM_STRUCTURE.md`;
- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_STORAGE_DATA.md`;
- `COMPILED_CORE_CALL_INSTRUCTIONS.md`;
- `COMPILED_NUMERIC_INSTRUCTIONS.md`;
- `COMPILED_CONTROL_FLOW.md`;
- `COMPILED_CONVERSIONS.md`;
- `COMPILED_SCALAR_EQUALITY.md`;
- `COMPILED_COMPOSITE_LAYOUT.md`;
- `COMPILED_COMPOSITE_INSTRUCTIONS.md`;
- `COMPILED_STRUCTURAL_EQUALITY.md`;
- `COMPILED_SOURCE_MAP.md`.

## 1. Final Review Result

```text
Compiled Program responsibility             ✅
Compiled root / function structure          ✅
physical storage identities                 ✅
constant representation                     ✅
external symbolic calling convention        ✅
core Load / Store                            ✅
internal / external Calls                    ✅
fixed numeric execution                      ✅
dynamic numeric execution                    ✅
control flow / short-circuit                 ✅
explicit conversions                         ✅
scalar equality                              ✅
composite layout / construction              ✅
when lowering / extraction                   ✅
structural equality                          ✅
Source Mapping                               ✅
SemanticExpressionKind coverage              ✅
SemanticStatement coverage                   ✅
Semantic Function coverage                   ✅
no AST dependency                            ✅
no runtime name/type resolution              ✅
no VM Execution Data leakage                 ✅
no Outcome / Diagnostic Data leakage         ✅

Compiled Program / Bytecode Data             ✅ CLOSED
```

La auditoría previa detectó y corrigió antes de este cierre:

```text
missing Core Load / Store formal contracts
missing Call(FunctionId)
missing CallExternal(ExternalSymbolId)
missing ExternalSymbol.parameter_count
redundant Constant::Int / Constant::Float physical variants
```

No se encontró un segundo hueco después de esas correcciones.

## 2. Exact Own Identity Count

`Compiled Program / Bytecode Data` v0 contiene exactamente **18 identities técnicas propias**.

### Program identities — 2

```text
01 ConstantId
02 ExternalSymbolId
```

### Program / Function structures — 2

```text
03 CompiledProgram
04 CompiledFunction
```

### Physical storage / persistent executable data — 5

```text
05 ParameterSlot
06 LocalSlot
07 ExternalSymbol
08 Constant
09 DynamicConstant
```

### Instruction mechanism — 3

```text
10 NumericKind
11 Instruction
12 InstructionIndex
```

### Composite physical identities — 2

```text
13 FieldIndex
14 VariantDiscriminant
```

### Structural equality plan — 3

```text
15 EqualityRule
16 CompositeEqualityPlan
17 EnumEqualityPayloadPlan
```

### Source mapping — 1

```text
18 SourceMap
```

## 3. Reused Identities Are Not Counted Again

No se cuentan nuevamente identities definidas por fases previas y reutilizadas por el producto compilado:

```text
FunctionId       ← Semantic Program Data
SignatureSymbol  ← Semantic Program Data
SourceSpan       ← Lexical Data
```

Su reutilización no crea una identity nueva dentro de `Compiled Program / Bytecode Data`.

Tampoco se cuentan como identities independientes:

```text
parameter_count
local_count
max_operand_depth
field_order
source / target fields of ConvertNumeric
Vec containers
usize backing values
```

porque son fields o representaciones internas de identities ya contadas.

## 4. Closed Root Representation

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

CompiledFunction.instructions
                  finite ordered sequence
```

`FunctionId` conserva owner-index identity entre `SemanticProgram.functions` y `CompiledProgram.functions`.

## 5. Closed Storage Representations

### ParameterSlot / LocalSlot

```rust
struct ParameterSlot(usize);
struct LocalSlot(usize);
```

```text
parameter absolute
    = frame_base + ParameterSlot

local absolute
    = frame_base + parameter_count + LocalSlot

operand_base
    = frame_base + parameter_count + local_count
```

`frame_base` y el storage runtime pertenecen a `VM Execution Data`; no son producto compilado.

### ExternalSymbol

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
}
```

`parameter_count` cuenta únicamente `SemanticSignatureParameter::Value`.

Signature Dependency Parameters no son Values físicos y cuentan cero.

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

Dynamic Integer utiliza minimal unsigned big-endian magnitude; zero es `negative = false` + empty magnitude.

## 6. NumericKind Exact Inventory

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

```text
int     → Int32
int32   → Int32
float   → Float64
float64 → Float64

dynamic ∉ NumericKind
```

## 7. Exact Instruction Enum

`Instruction` v0 contiene exactamente **48 variants**.

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

Count validation:

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

No existe `Opcode` separado.

## 8. InstructionIndex

```rust
struct InstructionIndex(usize);
```

```text
InstructionIndex(n)
    → CompiledFunction.instructions[n]
```

Es local a una `CompiledFunction`, no byte offset, address física ni `InstructionPointer`.

Branches v0 usan absolute `InstructionIndex`.

## 9. Composite Physical Identities

```rust
struct FieldIndex(usize);
struct VariantDiscriminant(usize);
```

Canonical lowering:

```text
FieldId(n)   → FieldIndex(n)
VariantId(n) → VariantDiscriminant(n)
```

No existen runtime type-layout tables en v0.

## 10. Structural Equality Plan Inventory

```rust
enum EqualityRule {
    Numeric(NumericKind),
    Boolean,
    String,
    Composite(CompositeEqualityPlan),
}
```

Exactamente **4 variants**.

```rust
enum CompositeEqualityPlan {
    Struct {
        fields: Vec<EqualityRule>,
    },
    Enum {
        variants: Vec<EnumEqualityPayloadPlan>,
    },
}
```

Exactamente **2 variants**.

```rust
enum EnumEqualityPayloadPlan {
    Simple,
    Associated(EqualityRule),
    Structured {
        fields: Vec<EqualityRule>,
    },
}
```

Exactamente **3 variants**.

No existe `EqualityRule::Dynamic`.

## 11. SourceMap

```rust
struct SourceMap {
    functions: Vec<Vec<SourceSpan>>,
}
```

Dense positional mapping:

```text
SourceMap.functions[f][i]
        ↕
CompiledProgram.functions[f].instructions[i]
```

Invariantes:

```text
source_map.functions.len()
    == compiled_program.functions.len()

source_map.functions[f].len()
    == compiled_program.functions[f].instructions.len()
```

Todo persistent Instruction posee exactamente un source anchor.

## 12. SemanticExpressionKind → Compiled Coverage

Las **10 variants** de `SemanticExpressionKind` tienen lowering completo.

### 12.1 Literal

```text
SemanticLiteral + resolved type
    → Constant / ConstantId
    → LoadConstant(ConstantId)
```

El negative sign continúa siendo Unary Negate salvo signed-min literal folding permitido por el compiler.

### 12.2 Binding

```text
Value Parameter BindingId
    → ParameterSlot
    → LoadParameter

Local BindingId
    → LocalSlot
    → LoadLocal
```

### 12.3 Unary

```text
Not over bool
    → NotBoolean

Negate over fixed signed/floating numeric
    → Negate(NumericKind)

Negate under dynamic numeric semantics
    → DynamicNegate
```

Invalid unsigned negation no llega a Compiled Program.

### 12.4 Binary

```text
Multiply / Divide / Add / Subtract / Remainder
    → fixed NumericKind instruction
    → or Dynamic* instruction under dynamic arithmetic context

Less / LessEqual / Greater / GreaterEqual
    → fixed numeric comparison

Equal / NotEqual
    → Numeric
    → Boolean
    → String
    → Composite plan
    according to resolved type

And / Or
    → Jump / JumpIfFalse lowering
```

No existen direct dynamic comparisons.

### 12.5 Conversion

```text
fixed numeric → fixed numeric
    → ConvertNumeric

dynamic → fixed numeric
    → ConvertDynamic

fixed numeric → string
    → NumericToString

dynamic → string
    → DynamicToString
```

Una physically identity conversion puede desaparecer cuando es infalible.

### 12.6 FieldAccess

```text
FieldId
    → FieldIndex
    → GetField(FieldIndex)
```

### 12.7 Call

```text
Internal(FunctionId)
    → Call(FunctionId)

DirectSignature(SignatureId)
SignatureDependency(SignatureBindingId)
    → ExternalSymbolId
    → CallExternal(ExternalSymbolId)
```

Solo `SemanticArgument::Value` produce operand Value.

`SemanticArgument::SignatureDependency` se erase de la calling convention física.

### 12.8 StructConstruction

```text
SemanticFieldValue.field
    → FieldIndex

source-order field evaluation
    → ConstructStruct { field_order }
    → canonical stored field order
```

### 12.9 EnumConstruction

```text
Simple
    → ConstructEnumSimple

Associated
    → evaluate payload
    → ConstructEnumAssociated

Structured
    → source-order fields
    → ConstructEnumStructured { variant, field_order }
```

### 12.10 When

```text
evaluate subject once
    → TestVariant
    → JumpIfFalse

Simple branch
    → Discard subject when matched

Associated extraction
    → ExtractEnumAssociated
    → StoreLocal

Structured extraction
    → ExtractEnumStructured
    → StoreLocal(s)

branch result
    → one Value
    → Jump end when required
```

No existe runtime `When`, Match object o Pattern object.

## 13. SemanticStatement → Compiled Coverage

Las **2 variants** de `SemanticStatement` tienen lowering completo.

### Bind

```text
compile value expression
    → StoreLocal(LocalSlot)
```

`StoreLocal` representa initial materialization, no reassignment.

### Operation

```text
compile expression
    → Discard
```

Después del statement no queda un Value semánticamente abandonado en el Operand Window.

## 14. SemanticFunction → Compiled Coverage

```text
SemanticFunction.parameters
    → physical Value parameter order
    → parameter_count + ParameterSlot

SemanticFunction.bindings
    → LocalSlot assignment where non-parameter
    → local_count

SemanticFunction.body
    → ordered Instruction sequence

SemanticFunction.body.result
    → result Value
    → Return

SemanticFunction.result_type
    → used during lowering
    → not persisted as runtime semantic type metadata

SemanticFunction.signature_bindings
    → external-symbol lowering / erased dependency forwarding

SemanticFunction.satisfaction
    → semantic contract validation responsibility completed
    → not required by CompiledFunction execution mechanism
```

`max_operand_depth` se calcula por Bytecode Compiler sobre el lowering final de la función.

## 15. SemanticProgram → CompiledProgram Coverage

```text
SemanticProgram.functions
    → CompiledProgram.functions
    → same FunctionId owner-index ordering

SemanticProgram.entry_function
    → CompiledProgram.entry_point

SemanticProgram.signatures
    → ExternalSymbol data only when required by external call lowering

SemanticProgram.types
    → consumed for physical lowering
    → no runtime TypeId table

SemanticExpression.span
    → SourceMap
```

Bytecode Compiler no vuelve al AST ni realiza resolution por nombre.

## 16. Physical Stack Contract Coverage

Closed families produce finite stack effects describable from the Instruction and referenced compiled metadata.

Examples:

```text
LoadConstant                     0 → 1
LoadParameter                    0 → 1
LoadLocal                        0 → 1
StoreLocal                       1 → 0

Call(FunctionId)                 N → 1
CallExternal(ExternalSymbolId)   N → 1

fixed binary numeric             2 → 1
fixed unary numeric              1 → 1
comparison                       2 → 1

DynamicNegate                    1 → 1
Dynamic binary arithmetic        2 → 1

Jump                             0 → 0
JumpIfFalse                      1 → 0
Discard                          1 → 0
Return                           1 → caller result

Convert*                         1 → 1
NotBoolean                       1 → 1

ConstructStruct(N)               N → 1
GetField                         1 → 1
ConstructEnumSimple              0 → 1
ConstructEnumAssociated          1 → 1
ConstructEnumStructured(N)       N → 1
TestVariant                      1 → 2
ExtractEnumAssociated            1 → 1
ExtractEnumStructured(N)         1 → N

EqualComposite                   2 → 1
NotEqualComposite                2 → 1
```

For calls, `N` is obtained from compiled target metadata, not semantic type inspection.

## 17. No Semantic Work Remains for VM

La VM no realiza:

```text
name resolution
type inference
type compatibility validation
field-name lookup
variant-name lookup
SignatureId resolution
Signature Dependency forwarding
implicit conversion selection
operator overload selection
struct / enum layout discovery
when exhaustiveness validation
EqualityComparable analysis
SourceSpan generation
```

Todo eso terminó antes de `CompiledProgram`.

## 18. No VM Execution Data Leakage

Continúan fuera de `Compiled Program / Bytecode Data`:

```text
InstructionPointer
CallFrame
frame_base runtime value
active FunctionId / active instruction state
Shared Value Storage
Operand Window runtime bounds
runtime Value representation
runtime Dynamic Integer representation
runtime Struct / Enum backing representation
Application Bindings instance
owned external backing storage
execution lifetime / borrowing state
```

Estas identities pertenecen a `VM Execution Data`.

## 19. No Outcome / Diagnostic Data Leakage

Continúan fuera:

```text
ExecutionOutcome
EvaluationError representation
OverflowError representation
DivisionByZeroError representation
ConversionError representation
DynamicNumericTypeError representation
human diagnostic message
line / column rendering
snippet / highlight data
```

`SourceMap` conserva únicamente provenance técnica persistente.

## 20. Explicitly Excluded Compiled Identities

No se justifican en v0:

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

## 21. Cardinality / Owner Validation

A valid Compiled Program satisfies:

```text
CompiledProgram.functions       1..N
CompiledProgram.entry_point     exactly 1 valid FunctionId
CompiledProgram.constants       0..N
CompiledProgram.external_symbols 0..N
CompiledProgram.source_map      exactly 1

ConstantId(n)
    → valid constants[n]

ExternalSymbolId(n)
    → valid external_symbols[n]

FunctionId(n)
    → valid functions[n]

ParameterSlot
    → valid physical Value parameter position of its function

LocalSlot
    → valid local position of its function

InstructionIndex target
    → valid instruction in same CompiledFunction

FieldIndex
    → valid canonical position for the composite mechanism that contains it

VariantDiscriminant
    → valid canonical enum variant position for the mechanism that contains it

SourceMap dimensions
    → exactly match functions / instructions dimensions
```

Compiler-generated temporaries such as branch labels and BindingId→slot maps do not survive.

## 22. Closure

```text
Exact compiled own identities — 18            ✅ CLOSED
Exact Instruction variants — 48               ✅ CLOSED
Exact NumericKind variants — 12                ✅ CLOSED
Exact EqualityRule variants — 4                ✅ CLOSED
Exact CompositeEqualityPlan variants — 2       ✅ CLOSED
Exact EnumEqualityPayloadPlan variants — 3     ✅ CLOSED
SemanticExpressionKind coverage — 10 / 10      ✅ CLOSED
SemanticStatement coverage — 2 / 2             ✅ CLOSED
SemanticFunction lowering coverage              ✅ CLOSED
SemanticProgram lowering coverage               ✅ CLOSED
physical stack-contract coverage                ✅ CLOSED
cardinality / owner consistency                 ✅ CLOSED
no unresolved semantic work in VM               ✅ CLOSED
no VM Execution Data leakage                    ✅ CLOSED
no Outcome / Diagnostic Data leakage            ✅ CLOSED

Compiled Program / Bytecode Data                ✅ CLOSED

NEXT
    VM Execution Data
```
