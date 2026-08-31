# Evo-Script Engine — Backing Data Representation

Status: CLOSED

Este documento cierra la representación v0 del backing data owned por una `VmExecution` para los `RuntimeValue` variables y composites.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`, especialmente TD-006, TD-007 y TD-008;
- `COMPILED_COMPOSITE_LAYOUT.md`;
- `COMPILED_COMPOSITE_INSTRUCTIONS.md`;
- `RUNTIME_VALUE_MODEL.md`;
- `BACKING_IDENTITY_STRATEGY.md`;
- `RUNTIME_VALUE_REPRESENTATION.md`.

Este bloque define qué dato representa cada backing y cómo se relaciona con sus typed IDs. No redefine `RuntimeValue`, no introduce runtime reflection y no decide todavía la representación del Shared Value Storage.

## BD-001 — VmExecution owns exactly one ExecutionBackingStore

Status: CLOSED

`VmExecution` posee exactamente un owner técnico para backing data producido durante la ejecución:

```text
VmExecution
├── Shared Value Storage
│   └── RuntimeValue descriptors
└── ExecutionBackingStore
    └── execution-owned backing
```

`ExecutionBackingStore` no reemplaza el Constant Pool del `CompiledProgram`.

Los backings con origen `Compiled(ConstantId)` continúan siendo owned por `CompiledProgram`; no se copian automáticamente al store de ejecución.

## BD-002 — Four typed append-only backing stores

Status: CLOSED

`ExecutionBackingStore` contiene exactamente cuatro stores lógicos tipados:

```text
ExecutionBackingStore
├── Strings
├── Dynamic Integers
├── Structs
└── Enums
```

Forma Rust v0 recomendada y cerrada como representación base:

```rust
struct ExecutionBackingStore {
    strings: Vec<Box<str>>,
    dynamic_integers: Vec<DynamicIntegerBacking>,
    structs: Vec<StructBacking>,
    enums: Vec<EnumBacking>,
}
```

Los cuatro stores son append-only durante una `VmExecution`:

```text
append backing
    → new typed ID
    → backing remains addressable by that ID
    → no remove / no ID reuse in v0
```

Una reallocación interna de un `Vec` no invalida identities porque `RuntimeValue` conserva índices tipados, no referencias Rust directas hacia elementos del store.

## BD-003 — Typed BackingId resolves positionally

Status: CLOSED

Cada typed backing identity resuelve por posición exclusivamente contra su store correspondiente:

```text
StringBackingId(n)
    → strings[n]

DynamicIntegerBackingId(n)
    → dynamic_integers[n]

StructBackingId(n)
    → structs[n]

EnumBackingId(n)
    → enums[n]
```

No existe búsqueda por nombre, runtime type tag, hash lookup ni tabla universal de objetos.

La validez de un typed ID implica que el índice está dentro del store correspondiente de la misma `VmExecution` que lo creó.

## BD-004 — Execution String backing is immutable Box<str>

Status: CLOSED

Los Strings producidos durante ejecución se almacenan como UTF-8 owned e inmutable:

```rust
Box<str>
```

Regla:

> Una vez insertado un execution String backing, su contenido textual no cambia.

`Box<str>` expresa ownership, contenido UTF-8, longitud fija e inmutabilidad del Value sin conservar capacidad mutable innecesaria.

Resolución conceptual:

```text
StringBackingRef::Execution(StringBackingId(n))
    ↓
ExecutionBackingStore.strings[n]
    ↓ borrow temporal
&str
```

Los String constants mantienen su backing en:

```text
StringBackingRef::Compiled(ConstantId)
    → CompiledProgram.constants
    → Constant::String
```

sin copia obligatoria hacia `ExecutionBackingStore`.

## BD-005 — DynamicIntegerBacking owns arbitrary-precision signed integer data

Status: CLOSED

`DynamicIntegerBacking` representa y posee exactamente un entero signed de precisión arbitraria para runtime arithmetic.

```rust
struct DynamicIntegerBacking {
    // owned arbitrary-precision signed integer representation
}
```

Regla contractual:

```text
DynamicIntegerBacking
    = owned arbitrary-precision signed integer
    = exact mathematical integer value
    = no fixed-width overflow caused by representation size
```

La implementación concreta del entero arbitrario queda encapsulada detrás de `DynamicIntegerBacking`.

No se hace parte de la arquitectura una crate específica de BigInt.

Tampoco se obliga a reutilizar como arithmetic representation el encoding persistente de:

```rust
DynamicConstant::Integer {
    negative,
    magnitude,
}
```

El Constant encoding y el runtime arithmetic backing tienen responsabilidades diferentes.

## BD-006 — StructBacking contains canonical ordered RuntimeValue fields

Status: CLOSED

La representación exacta de un Struct backing es:

```rust
struct StructBacking {
    fields: Box<[RuntimeValue]>,
}
```

Invariantes:

```text
fields.len()
    = physical field cardinality

FieldIndex(n)
    → fields[n]
```

Los fields están almacenados en el canonical field order ya establecido durante compilation/lowering.

No se almacenan dentro del backing:

```text
field names
FieldId
TypeId
StructTypeId
field type metadata
reflection metadata
separate field count
```

`RuntimeValue` descriptors nested dentro del Struct pueden compartir otros backings mediante typed IDs sin duplicar su contenido.

## BD-007 — EnumBacking = VariantDiscriminant + RuntimeEnumPayload

Status: CLOSED

La representación exacta es:

```rust
struct EnumBacking {
    variant: VariantDiscriminant,
    payload: RuntimeEnumPayload,
}
```

```rust
enum RuntimeEnumPayload {
    Simple,

    Associated(
        RuntimeValue,
    ),

    Structured {
        fields: Box<[RuntimeValue]>,
    },
}
```

Correspondencia física:

```text
Simple
    → no payload Value

Associated
    → exactly 1 RuntimeValue

Structured
    → ordered RuntimeValue fields
```

Para Structured payload:

```text
FieldIndex(n)
    → fields[n]
```

No se almacenan variant names, FieldId, VariantId, TypeId, EnumTypeId ni runtime reflection metadata.

## BD-008 — Execution backings are immutable after insertion

Status: CLOSED

Una vez asignado un typed backing ID, el backing identificado no cambia semánticamente durante esa `VmExecution`.

Regla:

> Las operaciones que producen un nuevo Value variable/composite crean nuevo backing cuando lo necesitan; no mutan un backing existente para representar un Value diferente.

Ejemplo Dynamic Integer:

```text
read backing A
    + operation
    ↓
new DynamicIntegerBacking B
    ↓
new DynamicIntegerBackingId
```

No:

```text
mutate backing A in-place
while other RuntimeValue descriptors still identify A
```

Esta regla hace seguro compartir un mismo backing ID entre Parameter Slots, Local Slots, Operand Window y nested composites.

## BD-009 — Composite backing graph is a finite immutable DAG

Status: CLOSED

Struct y Enum backing pueden contener `RuntimeValue` descriptors que a su vez referencian otros Struct/Enum backings.

Sharing está permitido:

```text
          Struct A
          ▲      ▲
          │      │
Struct B          Struct C
```

pero runtime composite cycles no forman parte del modelo v0.

Regla canónica:

> El graph de composite backing de una `VmExecution` es finito, inmutable y acíclico.

Esto es consistente con las reglas del lenguaje ya cerradas:

- composite type dependency graph es DAG;
- recursive struct/enum type cycles son inválidos;
- Evo-Script v0 no introduce references/pointers/null para formar ciclos runtime arbitrarios;
- composite Values son inmutables.

Por tanto v0 no necesita para este propósito:

```text
GC
cycle collector
Rc / Arc ownership graph
Weak references
runtime cycle detection as normal execution mechanism
```

## Closed Physical Family

```rust
struct ExecutionBackingStore {
    strings: Vec<Box<str>>,
    dynamic_integers: Vec<DynamicIntegerBacking>,
    structs: Vec<StructBacking>,
    enums: Vec<EnumBacking>,
}

struct DynamicIntegerBacking {
    // owned arbitrary-precision signed integer representation
}

struct StructBacking {
    fields: Box<[RuntimeValue]>,
}

struct EnumBacking {
    variant: VariantDiscriminant,
    payload: RuntimeEnumPayload,
}

enum RuntimeEnumPayload {
    Simple,
    Associated(RuntimeValue),
    Structured {
        fields: Box<[RuntimeValue]>,
    },
}
```

`DynamicIntegerBacking` encapsula el engine-owned arbitrary-precision integer representation; seleccionar una crate concreta no forma parte de este Technical Data Model.

## Execution examples

### GetField

```text
pop RuntimeValue::Struct(id)
    ↓
structs[id].fields[FieldIndex]
    ↓ copy RuntimeValue descriptor
push descriptor
```

### TestVariant

```text
RuntimeValue::Enum(id)
    ↓
enums[id].variant
    ↓
compare VariantDiscriminant
```

### ExtractEnumAssociated

```text
EnumBacking.payload
    → Associated(RuntimeValue)
    → copy descriptor
```

### ExtractEnumStructured

```text
EnumBacking.payload
    → Structured(fields)
    → copy requested RuntimeValue descriptors
```

No owner/payload borrowed aliasing es requerido por estas operations.

## Explicitly Not Introduced

```text
universal RuntimeBacking object table
runtime type reflection for backing
mutable Struct / Enum Values
mutable String Values
backing ID reuse
per-backing deallocation during v0 execution
GC / cycle collector
Rc / Arc requirement
field / variant names in runtime backing
TypeId / StructTypeId / EnumTypeId in backing
specific BigInt crate as architectural dependency
```

## Closure

```text
BD-001 one ExecutionBackingStore per VmExecution         ✅ CLOSED
BD-002 four typed append-only stores                     ✅ CLOSED
BD-003 typed ID positional resolution                    ✅ CLOSED
BD-004 execution String = immutable Box<str>             ✅ CLOSED
BD-005 DynamicIntegerBacking owns arbitrary integer      ✅ CLOSED
BD-006 StructBacking canonical RuntimeValue fields       ✅ CLOSED
BD-007 EnumBacking + RuntimeEnumPayload                  ✅ CLOSED
BD-008 backing immutable after insertion                 ✅ CLOSED
BD-009 composite backing = finite immutable DAG          ✅ CLOSED

Backing Data Representation                              ✅ CLOSED

Shared Value Storage exact representation                ← NEXT
CallFrame                                                 PENDING
InstructionPointer                                        PENDING
ApplicationBindings exact model                           PENDING
call / return mechanics                                   PENDING
```