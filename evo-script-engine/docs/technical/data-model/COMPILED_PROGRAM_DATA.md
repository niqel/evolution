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

`when` reutiliza esta misma branch infrastructure junto con `TestVariant` y las extraction instructions cerradas en CD-019.

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

General equality queda cerrada por familias:

```text
numeric     ✅ CLOSED
bool        ✅ CLOSED
string      ✅ CLOSED
struct      ✅ CLOSED in Structural Equality
enum        ✅ CLOSED in Structural Equality
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

## CD-019 — Struct / Enum Instructions

Status: CLOSED — CORRECTED FINAL DESIGN

Cerrado en `COMPILED_COMPOSITE_INSTRUCTIONS.md`.

Instructions:

```rust
ConstructStruct {
    field_order: Vec<FieldIndex>,
}

GetField(FieldIndex)

ConstructEnumSimple(
    VariantDiscriminant,
)

ConstructEnumAssociated(
    VariantDiscriminant,
)

ConstructEnumStructured {
    variant: VariantDiscriminant,
    field_order: Vec<FieldIndex>,
}

TestVariant(
    VariantDiscriminant,
)

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

### Construction ordering

`ConstructStruct` y `ConstructEnumStructured` preservan source evaluation order mediante `field_order` y producen canonical storage ordering.

Para N fields:

```text
field_order.len() == N
field_order = permutation of all valid FieldIndex values
```

No se permite duplicated, missing u out-of-range `FieldIndex` en un Compiled Program válido.

### `when` lowering

`TestVariant` conserva temporalmente el Enum y produce un `bool` para `JumpIfFalse`, de modo que una branch que no coincide pueda dejar el subject disponible para la siguiente prueba.

Una vez confirmada la variant, payload extraction consume el Enum:

```text
ExtractEnumAssociated
    Enum → payload

ExtractEnumStructured { fields }
    Enum → N payload field Values
```

La revisión final rechazó explícitamente:

```text
GetEnumAssociated : 1 → 2
GetEnumField       : 1 → 2
```

porque preservar simultáneamente el Enum owner y Values interiores extraídos podría forzar cloning, interior borrowing o aliasing antes de que `VM Execution Data` justifique una representación concreta.

Regla canónica:

> Después de confirmar la variante correcta, enum payload extraction consume el composite; el Instruction Set no exige por diseño aliasing entre owner y payload Values.

No existen runtime `When`, `Match`, Pattern object, `TypeId`, `StructLayout` o `EnumLayout` para ejecutar estas operaciones.

## CD-020 — Structural Equality

Status: CLOSED

La regla normativa está cerrada en `evo-script/COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md` y el mecanismo compilado en `COMPILED_STRUCTURAL_EQUALITY.md`.

### EqualityComparable

Semantic Analyzer decide estáticamente:

```text
fixed numeric  → comparable
bool           → comparable
string         → comparable
dynamic        → NOT comparable

Struct
    → comparable iff all fields are comparable

Enum
    → comparable iff all variant payloads are comparable
```

La propiedad es transitiva sobre el Type Dependency DAG.

Un composite que contenga `dynamic` directa o transitivamente no admite `==` / `!=` y produce `ComparisonTypeError` durante Semantic Analysis.

No existe igualdad dinámica escondida dentro de Structural Equality.

### Compiled plan

Representaciones cerradas:

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

No existe `EqualityRule::Dynamic`.

Instructions:

```rust
EqualComposite(CompositeEqualityPlan)
NotEqualComposite(CompositeEqualityPlan)
```

Stack effect:

```text
2 composite Values → 1 bool
```

El plan se almacena directamente en la instruction en v0. No se introducen `EqualityPlanId`, Equality Plan Table, runtime `TypeId`, reflection metadata ni `EqualValue` genérico.

Struct equality compara fields en canonical order y puede terminar en el primer field desigual. Enum equality compara primero discriminants; si coinciden aplica el payload plan correspondiente.

Una Structural Equality compilada es total y no produce runtime `ComparisonTypeError`, `DynamicNumericTypeError` ni `ConversionError`.

## CD-021 — SourceMap

Status: CLOSED

Cerrado en `COMPILED_SOURCE_MAP.md`.

Representación:

```rust
struct SourceMap {
    functions: Vec<Vec<SourceSpan>>,
}
```

Relación canónica:

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

Por tanto cada `(FunctionId, InstructionIndex)` resuelve exactamente un `SourceSpan`, y cada Instruction persistente posee exactamente un source anchor.

La estructura es densa; no se introduce `Option<SourceSpan>`, sparse mapping ni `SourceMapEntry` con coordenadas duplicadas.

### Span policy

Cada Instruction recibe el `SourceSpan` de la `SemanticExpression` más específica responsable de producirla. Una Instruction técnica generada por el compiler utiliza el span de la construcción semántica responsable más cercana.

Examples:

```text
Multiply de b * c
    → span(b * c)

JumpIfFalse generado por a && b
    → span(a && b)

TestVariant / extraction machinery
    → span(when) cuando no existe uno más específico

Return final
    → span(result expression)
```

No se vuelve al AST para resolver ubicaciones.

### Source coordinate space v0

Un `CompiledProgram` utiliza un único source coordinate space. Todos sus `SourceSpan` pertenecen al Source Text que produjo ese programa.

No se introducen en v0:

```text
SourceId
SourcePath
SourceName
SourceLocation
```

Una futura extensión multi-source puede evolucionar el dato resuelto a:

```rust
struct SourceLocation {
    source: SourceId,
    span: SourceSpan,
}
```

sin modificar `Instruction`, `CompiledFunction` ni VM execution semantics.

### Encapsulation boundary

La nested storage shape `Vec<Vec<SourceSpan>>` pertenece exclusivamente al subsistema de Source Mapping. Los consumidores no deben depender de esa forma interna; conceptualmente resuelven:

```text
FunctionId + InstructionIndex
        ↓ Source Mapping boundary
SourceSpan
```

Esta frontera contiene el impacto de una futura migración multi-source.

### Lifetime / diagnostics boundary

`SourceMap` no borrowea Source Text y no duplica line/column. `SourceSpan` conserva provenance técnica; line, column, snippet y highlight se derivan posteriormente cuando el Host dispone del Source Text.

No existen mappings separados `ConstantId → SourceSpan` o `ExternalSymbolId → SourceSpan`; la ubicación pertenece a cada Instruction occurrence.

### Persistence scope

`CompiledProgram` persistente respecto del Compilation Working State no implica un portable serialized bytecode format. Los `usize` internos no constituyen ABI estable; una futura serialización portable es una responsabilidad separada.

## CD-022 — Current closure

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
Struct / Enum Instructions               ✅ CLOSED — corrected
when composite lowering                  ✅ CLOSED
owner/payload aliasing not required      ✅ CLOSED
EqualityComparable                       ✅ CLOSED
Struct / Enum Structural Equality        ✅ CLOSED
no hidden dynamic equality               ✅ CLOSED
SourceMap                                ✅ CLOSED
SourceMap encapsulation boundary         ✅ CLOSED
future multi-source migration seam       ✅ CLOSED

Compiled Program exact inventory         ← NEXT
```