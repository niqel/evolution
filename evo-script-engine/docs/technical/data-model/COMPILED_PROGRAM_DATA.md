# Evo-Script Engine — Compiled Program / Bytecode Data

Status: COMPILED PROGRAM / BYTECODE DATA — FINAL INVENTORY PENDING

Este documento es la autoridad acumulada del producto persistente producido por Bytecode Compiler y consumido directamente por la Stack VM de `evo-script-engine` v0.

La autoridad deriva de `TECHNICAL_DESIGN.md`, `SEMANTIC_PROGRAM_DATA.md` y los documentos especializados de este bloque.

```text
Semantic Program
    ↓ Bytecode Compiler
Compiled Program
    ↓ Stack VM
Execution Result
```

## CD-001 — Compiled Program representa mecanismo ejecutable

Status: CLOSED

> `Semantic Program` representa significado resuelto; `Compiled Program` representa mecanismo ejecutable persistente que la VM consume sin volver al AST ni al Semantic Program.

Consecuencias:

1. puede sobrevivir al Source Text y al Compilation Working State;
2. la VM no realiza name resolution, type inference ni semantic validation;
3. semantic information que ya fue lowered a mechanism físico no se conserva por costumbre;
4. no contiene Active Scope, Host Session State, Current Provider ni provider lookup ambiental.

## CD-002 — FunctionId se preserva

Status: CLOSED

```text
SemanticProgram.functions[n]
    ↓
CompiledProgram.functions[n]
```

`FunctionId` se preserva. No existe `CompiledFunctionId`.

## CD-003 — ConstantId

Status: CLOSED

```rust
struct ConstantId(usize);
```

```text
ConstantId(n)
    → CompiledProgram.constants[n]
```

Namespace local al `CompiledProgram`.

## CD-004 — ExternalSymbolId

Status: CLOSED

```rust
struct ExternalSymbolId(usize);
```

```text
ExternalSymbolId(n)
    → CompiledProgram.external_symbols[n]
```

No identifica Provider ni runtime binding.

## CD-005 — Signature Dependency Erasure

Status: CLOSED

Signature Dependencies no son Values de primer orden y se eliminan de la calling convention física.

```text
SignatureBindingId
    → semantic dependency meaning
    → no ParameterSlot
    → no Operand Value
    → ExternalSymbolId when invoked
```

No existen `SignatureSlot`, Function Value ni closure artificial.

## CD-006 — Signature Dependency Forwarding

Status: CLOSED

`SemanticArgument::SignatureDependency` no genera argumento Value físico. El forwarding se resuelve durante compilation.

## CD-007 — Direct Signature / Signature Dependency convergence

Status: CLOSED

```text
DirectSignature(SignatureId)
SignatureDependency(SignatureBindingId)
        ↓ Bytecode Compiler
ExternalSymbolId
```

Ambos usan `CallExternal(ExternalSymbolId)`.

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

No se introducen wrappers `FunctionTable`, `ConstantPool` o `ExternalSymbolTable` sin responsabilidad propia.

## CD-009 — CompiledFunction

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

`max_operand_depth` expresa la profundidad temporal máxima del Operand Window.

## CD-010 — Semantic data lowering boundary

Status: CLOSED

Por defecto no sobreviven:

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
SemanticLiteral     → Constant
SemanticExpression  → Instructions
```

## CD-011 — Compiled storage data

Status: CLOSED — REVALIDATED AFTER FINAL AUDIT

Cerrado en `COMPILED_STORAGE_DATA.md`.

### Slots

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

`ParameterSlot != LocalSlot` aunque compartan backing frame region.

### ExternalSymbol — corrected final representation

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
}
```

`parameter_count` cuenta exclusivamente `SemanticSignatureParameter::Value`.

Signature Dependency Parameters cuentan cero y permanecen erased de la calling convention física.

Este dato permite ejecutar `CallExternal(ExternalSymbolId)` sin repetir aridad en cada call site.

### Constant — canonical physical representation

La auditoría final eliminó la duplicación física `Int(i32)` / `Int32(i32)` y `Float(f64)` / `Float64(f64)`.

Representación final:

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
semantic int   → Constant::Int32
semantic int32 → Constant::Int32

semantic float   → Constant::Float64
semantic float64 → Constant::Float64
```

Esto expresa mecanismo físico común, no alias semántico.

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

Integer magnitude es minimal unsigned big-endian; zero = empty magnitude + `negative = false`.

Constant interning es optimización opcional.

## CD-012 — Core Load / Store Instructions

Status: CLOSED

Cerrado en `COMPILED_CORE_CALL_INSTRUCTIONS.md`.

```rust
Instruction::LoadConstant(ConstantId)
Instruction::LoadParameter(ParameterSlot)
Instruction::LoadLocal(LocalSlot)
Instruction::StoreLocal(LocalSlot)
```

Stack contracts:

```text
LoadConstant    0 → 1
LoadParameter   0 → 1
LoadLocal       0 → 1
StoreLocal      1 → 0
```

`StoreLocal` representa inicialización física de `let` o `when` extraction binding; no representa mutabilidad semántica.

Invariante:

> Todo execution path que alcance `LoadLocal(slot)` ya inicializó ese slot.

No existe `StoreParameter`.

## CD-013 — Internal / External Calls

Status: CLOSED

Cerrado en `COMPILED_CORE_CALL_INSTRUCTIONS.md`.

```rust
Instruction::Call(FunctionId)
Instruction::CallExternal(ExternalSymbolId)
```

Internal call arity:

```text
CompiledProgram.functions[target].parameter_count
```

External call arity:

```text
CompiledProgram.external_symbols[target].parameter_count
```

Para `N` Value Parameters físicos:

```text
N argument Values → 1 result Value
```

La creación de Call Frame y transferencia física de argumentos/result pertenecen a `VM Execution Data`.

## CD-014 — NumericKind

Status: CLOSED

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

```text
int     → Int32
int32   → Int32
float   → Float64
float64 → Float64

dynamic ∉ NumericKind
```

## CD-015 — Fixed numeric arithmetic and comparisons

Status: CLOSED

```rust
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

Arithmetic fixed es checked:

```text
no wrapping
no saturation
overflow → OverflowError
divide/remainder by zero → DivisionByZeroError
```

`Remainder` solo acepta integer `NumericKind`.

## CD-016 — Dynamic numeric lifting and arithmetic

Status: CLOSED

```rust
LiftDynamic(NumericKind)
DynamicNegate
DynamicAdd
DynamicSubtract
DynamicMultiply
DynamicDivide
DynamicRemainder
```

Dynamic runtime families:

```text
Dynamic Numeric Value
├── Integer
├── Float32
└── Float64
```

Cross-family arithmetic produce `DynamicNumericTypeError`.

No existen Dynamic comparison instructions.

## CD-017 — Instruction representation / InstructionIndex

Status: CLOSED

`Instruction` es un typed enum; no se introduce `Opcode + generic operands`.

```rust
struct InstructionIndex(usize);
```

```text
InstructionIndex(n)
    → CompiledFunction.instructions[n]
```

`InstructionIndex` es local a una función, no byte offset, address ni `InstructionPointer`.

## CD-018 — Control Flow and short-circuit

Status: CLOSED

Cerrado en `COMPILED_CONTROL_FLOW.md`.

```rust
Jump(InstructionIndex)
JumpIfFalse(InstructionIndex)
Discard
Return
```

Branches usan absolute `InstructionIndex`.

`&&` y `||` se reducen a branching real; no existen eager `AndBoolean` / `OrBoolean`.

`Discard` consume un Value no utilizado.

`Return` transfiere exactamente un Value normal al caller / outer result.

## CD-019 — Conversion Instructions

Status: CLOSED

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

Reglas:

```text
fixed numeric → fixed numeric
    exact or ConversionError

dynamic → fixed numeric
    exact or ConversionError

fixed numeric → string
    NumericToString

dynamic → string
    DynamicToString
```

No hay implicit conversions ni string → numeric parsing.

## CD-020 — Scalar Boolean / String Equality

Status: CLOSED

```rust
NotBoolean
EqualBoolean
NotEqualBoolean
EqualString
NotEqualString
```

Bool y string no poseen ordering operators.

String equality compara contenido textual UTF-8.

## CD-021 — Composite Layout

Status: CLOSED

Cerrado en `COMPILED_COMPOSITE_LAYOUT.md`.

```rust
struct FieldIndex(usize);
struct VariantDiscriminant(usize);
```

```text
FieldId(n)   → FieldIndex(n)
VariantId(n) → VariantDiscriminant(n)
```

Layout conceptual:

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

No existen `StructLayoutId`, `EnumLayoutId`, `CompositeTypeId`, RuntimeTypeId ni runtime reflection metadata.

## CD-022 — Struct / Enum Instructions

Status: CLOSED — CORRECTED FINAL DESIGN

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

Stack contracts:

```text
ConstructStruct(N)            N → 1
GetField                      1 → 1
ConstructEnumSimple           0 → 1
ConstructEnumAssociated       1 → 1
ConstructEnumStructured(N)    N → 1
TestVariant                   1 → 2
ExtractEnumAssociated         1 → 1
ExtractEnumStructured(N)      1 → N
```

Payload extraction consume el Enum después de confirmar la variant; no obliga a owner/payload aliasing.

## CD-023 — Structural Equality

Status: CLOSED

La regla normativa `EqualityComparable` está cerrada en `evo-script/COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md`.

```text
fixed numeric  → comparable
bool           → comparable
string         → comparable
dynamic        → NOT comparable

Struct
    → comparable iff all fields comparable

Enum
    → comparable iff all variant payloads comparable
```

Un composite que contiene `dynamic` directa o transitivamente produce `ComparisonTypeError` durante Semantic Analysis al intentar `==` / `!=`.

Compiled plans:

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

No existe `EqualityRule::Dynamic`, `EqualValue` genérico, RuntimeTypeId ni Equality Plan Table.

## CD-024 — SourceMap

Status: CLOSED

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

Invariantes:

```text
source_map.functions.len()
    == compiled_program.functions.len()

source_map.functions[f].len()
    == compiled_program.functions[f].instructions.len()
```

Cada persistent Instruction posee exactamente un source anchor.

Un `CompiledProgram` usa un único source coordinate space en v0.

No se introducen `SourceId`, SourcePath, SourceName ni line/column duplicados.

La nested storage shape de SourceMap queda encapsulada para permitir futura evolución a `SourceLocation { source, span }` sin cambiar Instruction, CompiledFunction ni VM execution semantics.

`persistent product != portable serialized bytecode format`.

## CD-025 — Final audit corrections

Status: CLOSED

La auditoría Semantic Program → Compiled Program detectó y corrigió antes del exact inventory:

```text
Core Load / Store formal contracts       ✅ CLOSED
Call(FunctionId)                         ✅ CLOSED
CallExternal(ExternalSymbolId)           ✅ CLOSED
ExternalSymbol.parameter_count           ✅ CLOSED
Constant physical canonicalization       ✅ CLOSED
```

Correcciones explícitas:

```text
ExternalSymbol { symbol }
    → ExternalSymbol { symbol, parameter_count }

Constant::Int(i32)     ❌ REMOVED
Constant::Float(f64)   ❌ REMOVED
Constant::Int32(i32)   ✅ canonical int/int32 representation
Constant::Float64(f64) ✅ canonical float/float64 representation
```

Después de estas correcciones, toda forma de `SemanticExpressionKind` y `SemanticStatement` posee un lowering compilado identificado.

Esto todavía debe validarse mediante el **Exact Compiled Inventory** antes de declarar cerrado todo `Compiled Program / Bytecode Data`.

## CD-026 — Current closure

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
Constant / DynamicConstant               ✅ CLOSED — canonicalized
ExternalSymbol                           ✅ CLOSED — corrected
External physical arity                  ✅ CLOSED
Core Load / Store                        ✅ CLOSED
Internal Call                            ✅ CLOSED
External Call                            ✅ CLOSED
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
Struct / Enum Instructions               ✅ CLOSED — corrected
when composite lowering                  ✅ CLOSED
owner/payload aliasing not required      ✅ CLOSED
EqualityComparable                       ✅ CLOSED
Struct / Enum Structural Equality        ✅ CLOSED
no hidden dynamic equality               ✅ CLOSED
SourceMap                                ✅ CLOSED
SourceMap encapsulation boundary         ✅ CLOSED
future multi-source migration seam       ✅ CLOSED
Final audit corrections                  ✅ CLOSED

Compiled Program exact inventory         ← NEXT
```
