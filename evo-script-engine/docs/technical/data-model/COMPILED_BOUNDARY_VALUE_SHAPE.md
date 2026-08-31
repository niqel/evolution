# Evo-Script Engine — Compiled Boundary Value Shape

Status: CLOSED

Este documento cierra la metadata compilada mínima utilizada para validar Values que cruzan dos fronteras ejecutables:

```text
Consumer → Execute Compiled entry Parameters
ExternalCapability → CallExternal result
```

La solución no reintroduce `TypeId`, `SemanticProgram`, runtime reflection ni metadata de tipos dentro de `RuntimeValue`.

## CB-001 — Dedicated compiled boundary shape model

Status: CLOSED

La compatibilidad de Values en fronteras ejecutables se representa mediante un modelo compilado dedicado:

```text
Semantic Type
    ↓ lowering
CompiledValueShape
    ↓ boundary validation
Value<'a> / OwnedValue
```

`CompiledValueShape` no es `SemanticType`, `TypeId` ni `RuntimeValue`.

## CB-002 — Owner-indexed CompiledValueShapeId

Status: CLOSED

```rust
struct CompiledValueShapeId(usize);
```

Owner rule:

```text
CompiledValueShapeId(n)
    → CompiledProgram.value_shapes[n]
```

La identity no se duplica dentro del elemento referenciado.

## CB-003 — Exact CompiledValueShape family

Status: CLOSED

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

Estas variants representan expected boundary shape; no contienen Value data.

## CB-004 — Dynamic is one boundary shape

Status: CLOSED

`dynamic` conserva una sola shape compilada:

```rust
CompiledValueShape::Dynamic
```

Acepta únicamente `Value::Dynamic(...)` / `OwnedValue::Dynamic(...)`, independientemente de que su runtime family sea Integer, Float32 o Float64.

No acepta un fixed `Int*`, `Uint*`, `Float*` como coerción implícita.

No se introducen:

```text
DynamicIntegerShape
DynamicFloat32Shape
DynamicFloat64Shape
```

## CB-005 — Struct shape

Status: CLOSED

```rust
CompiledValueShape::Struct {
    fields: Vec<CompiledValueShapeId>,
}
```

Los fields preservan canonical field order.

Boundary compatibility exige:

```text
Value is Struct
field cardinality exact
field order exact
cada field matches su CompiledValueShapeId recursivamente
```

No se almacenan field names, `FieldId` ni `TypeId`.

## CB-006 — Enum shape

Status: CLOSED

```rust
enum CompiledEnumValueShape {
    Simple,
    Associated(CompiledValueShapeId),
    Structured {
        fields: Vec<CompiledValueShapeId>,
    },
}
```

```rust
CompiledValueShape::Enum {
    variants: Vec<CompiledEnumValueShape>,
}
```

El orden de `variants` corresponde al canonical `VariantDiscriminant` ordinal ya cerrado.

Validation exige:

```text
valid variant ordinal
payload kind exact
payload cardinality exact
nested Values match recursively
```

No se almacenan variant names, `VariantId` ni `TypeId`.

## CB-007 — CompiledProgram owns only boundary-reachable shapes

Status: CLOSED

`CompiledProgram` agrega:

```rust
value_shapes: Vec<CompiledValueShape>
```

El Bytecode Compiler necesita persistir únicamente shapes transitivamente alcanzables desde:

```text
entry Value Parameters
external result types actually represented by ExternalSymbol
```

No existe obligación de copiar toda `SemanticProgram.types`.

Durante lowering puede existir temporalmente:

```text
TypeId → CompiledValueShapeId
```

para preservar sharing del DAG; ese mapping no persiste.

`CompiledProgram` también agrega:

```rust
entry_parameter_shapes: Vec<CompiledValueShapeId>
```

ordenado por los Value Parameters físicos del entry point.

## CB-008 — Entry shape cardinality invariant

Status: CLOSED

```text
CompiledProgram.entry_parameter_shapes.len()
    ==
CompiledProgram.functions[entry_point].parameter_count
```

Signature Dependency Parameters no forman parte de esta lista porque no son Value Parameters físicos.

Antes de iniciar una `VmExecution` válida:

```text
Invocation Values arity
    ↔ entry_parameter_shapes cardinality

InvocationValue[n]
    ↔ value_shapes[entry_parameter_shapes[n]]
```

Debe cumplirse exact match y ninguna coerción implícita.

## CB-009 — ExternalSymbol stores only result shape

Status: CLOSED

La representación corregida es:

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
    result_shape: CompiledValueShapeId,
}
```

No persisten external argument shapes en v0.

Los argumentos de `CallExternal` provienen de bytecode ya validado semánticamente y de `RuntimeValue` internos producidos por la misma ejecución. Revalidarlos por shape en cada call sería metadata redundante y convertiría la VM en type checker defensivo sin responsabilidad nueva.

El `OwnedValue` retornado por la capability sí cruza desde una frontera uniforme y debe validarse contra `result_shape` antes del commit `N → 1`.

## CB-010 — Exact recursive boundary validation

Status: CLOSED

La validación es exacta y recursiva.

Para scalar families:

```text
Int32   ↔ Int32      ✅
Int32   ↔ Int64      ❌
String  ↔ String     ✅
Dynamic ↔ Dynamic    ✅
Dynamic ↔ Float64    ❌
```

Para Struct y Enum se valida exactamente:

```text
variant family
numeric width
field/variant cardinality
canonical order
payload kind
nested shape
```

No existen implicit numeric conversions, fixed→dynamic lifting, string parsing ni otras coerciones en esta frontera.

## CB-011 — Interchange composite Values carry data, expected shape supplies context

Status: CLOSED

`evo_values::Value<'a>` y `OwnedValue` continúan sin transportar nominal type identity, names o `TypeId`.

Por ejemplo, dos semantic Struct types distintos pueden compartir la misma forma física. La identidad nominal continúa siendo responsabilidad del Semantic Analyzer dentro del lenguaje.

En una frontera externa:

```text
expected CompiledValueShape
    + neutral interchange Value data
    → exact boundary materialization
```

Esto no vuelve estructural el type system interno de Evo-Script y no habilita conversiones `A → B` dentro del programa.

## CB-012 — Boundary metadata is not general runtime reflection

Status: CLOSED

`CompiledValueShape` se utiliza únicamente para:

```text
1. validar Invocation Values antes de iniciar una VmExecution válida;
2. validar ExternalCapability Success(OwnedValue) antes de materializar/commit.
```

No participa en:

```text
Add / numeric execution
Load / Store
GetField
ConstructStruct / ConstructEnum
EqualComposite
Call internal
CallFrame
RuntimeValue dispatch
backing lookup
```

Por tanto no es una `RuntimeTypeTable` ni general reflection metadata.

## Corrected CompiledProgram shape

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

`CompiledFunction` permanece sin cambios:

```rust
struct CompiledFunction {
    parameter_count: usize,
    local_count: usize,
    max_operand_depth: usize,
    instructions: Vec<Instruction>,
}
```

## External result validation flow

```text
CallExternal(ExternalSymbolId)
    ↓
ExternalSymbol.result_shape
    ↓
ExternalCapability Success(OwnedValue)
    ↓
exact recursive shape validation
    ├── mismatch → execution Failure; no N→1 commit; IP unchanged
    └── match
          ↓
       materialize RuntimeValue
          ↓
       commit N→1
          ↓
       ip += 1
```

El tipo exacto del Failure permanece en Outcome / Diagnostic Data.

## Explicitly Not Introduced

```text
TypeId in CompiledProgram boundary contract
SemanticType persistence
RuntimeTypeId
StructLayoutId / EnumLayoutId for ordinary execution
field names
variant names
external parameter shape list
per-function parameter shape lists
shape lookup from RuntimeValue during ordinary bytecode
reflection API
hash/fingerprint as sole compatibility truth
inline recursive duplicated shape trees
```

## Inventory impact

Este cierre agrega exactamente tres identities propias a Compiled Program / Bytecode Data:

```text
19 CompiledValueShapeId
20 CompiledValueShape
21 CompiledEnumValueShape
```

Por tanto:

```text
compiled own identities   18 → 21
Instruction variants      48 → 48 unchanged
NumericKind variants      12 unchanged
```

## Closure

```text
CB-001 dedicated compiled boundary shape model              ✅ CLOSED
CB-002 owner-indexed CompiledValueShapeId                    ✅ CLOSED
CB-003 CompiledValueShape exact 17 variants                  ✅ CLOSED
CB-004 Dynamic one boundary shape                            ✅ CLOSED
CB-005 Struct ordered recursive shape                        ✅ CLOSED
CB-006 Enum ordered variants/payload shape                   ✅ CLOSED
CB-007 boundary-reachable owner table + entry shape list     ✅ CLOSED
CB-008 entry shape cardinality invariant                     ✅ CLOSED
CB-009 ExternalSymbol.result_shape only                      ✅ CLOSED
CB-010 exact recursive compatibility; no coercion            ✅ CLOSED
CB-011 neutral Value data + expected boundary context        ✅ CLOSED
CB-012 boundary-only metadata; not general reflection        ✅ CLOSED

Compiled Boundary Value Shape                               ✅ CLOSED
Compiled Program own identity count                         ✅ CLOSED — 21
Instruction variant count                                   ✅ CLOSED — 48 unchanged
VM Execution exact inventory                                ← NEXT
Outcome / Diagnostic Data                                   PENDING
```