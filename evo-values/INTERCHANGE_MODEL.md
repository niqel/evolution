# Evo Values — Interchange Model

Status: CLOSED

Este documento cierra el modelo v0 de intercambio de Values compartido por los componentes Evolution.

`evo-values` posee representaciones neutrales para cruzar fronteras entre componentes. No sustituye las representaciones internas especializadas de cada engine.

La autoridad deriva de:

- `evo-script-engine/docs/technical/data-model/RUNTIME_VALUE_MODEL.md`;
- `evo-script-engine/docs/technical/data-model/RUNTIME_VALUE_REPRESENTATION.md`;
- `evo-script-engine/docs/technical/data-model/EXTERNAL_CAPABILITY_ABI.md`;
- `evo-script-engine/docs/technical/data-model/BACKING_DATA_REPRESENTATION.md`;
- `evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`;
- `ENGINEERING_PRINCIPLES.md`.

## EV-001 — evo-values owns neutral interchange representations

Status: CLOSED

`evo-values` define representaciones neutrales de Value para intercambio entre crates y fronteras de ejecución.

```text
evo_values::Value<'a>
    = borrowed/interchange representation

evo_values::OwnedValue
    = owned/interchange representation

evo_script_engine::RuntimeValue
    = private VM execution descriptor
```

`RuntimeValue` no se convierte en el Value compartido del ecosistema y sus backing handles nunca son parte del contrato de `evo-values`.

## EV-002 — no_std + alloc remains sufficient

Status: CLOSED

El modelo permanece compatible con:

```rust
#![no_std]
extern crate alloc;
```

No requiere `std` ni una implementación concreta de BigInt.

`evo-values` no hace arithmetic de precisión arbitraria; únicamente transporta una representación canónica neutral cuando un Dynamic Integer cruza una frontera.

## EV-003 — Value<'a> has exactly 17 semantic value families

Status: CLOSED

`Value<'a>` refleja exactamente las 17 familias semánticas cerradas para Evo-Script v0, pero mediante representaciones de intercambio y no mediante runtime backing handles.

```rust
pub enum Value<'value> {
    Boolean(bool),

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

    String(&'value str),

    Dynamic(DynamicValue<'value>),

    Struct(Box<[Value<'value>]>),

    Enum {
        variant: usize,
        payload: EnumPayload<'value>,
    },
}
```

La coincidencia de familias con `RuntimeValue` es semántica, no física.

## EV-004 — Historical Text / Signed / Unsigned are replaced

Status: CLOSED

El modelo histórico:

```rust
Text(&str)
Signed(i64)
Unsigned(u64)
```

no conserva suficiente información para el lenguaje cerrado.

La forma v0 utiliza:

```text
String
Int8 .. Int128
Uint8 .. Uint128
Float32 / Float64
```

Por tanto `Text`, `Signed` y `Unsigned` no forman parte del interchange model v0 cerrado.

## EV-005 — Borrowed Dynamic Value

Status: CLOSED

`DynamicValue<'a>` contiene exactamente tres families:

```rust
pub enum DynamicValue<'value> {
    Integer(DynamicIntegerValue<'value>),
    Float32(f32),
    Float64(f64),
}
```

Dynamic Integer utiliza una representación canónica neutral:

```rust
pub struct DynamicIntegerValue<'value> {
    pub negative: bool,
    pub magnitude: Cow<'value, [u8]>,
}
```

Formato canónico:

```text
negative
+ unsigned big-endian magnitude bytes
```

Invariantes:

```text
magnitude is minimal
zero magnitude = empty bytes
zero.negative = false
```

`Cow` permite:

- borrowing directo cuando el owner ya posee magnitude bytes canónicos;
- ownership temporal cuando una representación runtime diferente debe canonicalizarse para observación externa.

Esto no fija la representación arithmetic interna de ningún engine.

## EV-006 — Borrowed Struct owns only its temporary descriptor tree

Status: CLOSED

La forma de intercambio es:

```rust
Value::Struct(Box<[Value<'value>]>)
```

El `Box<[Value<'a>]>` posee únicamente el árbol temporal de descriptors de intercambio.

Backing pesado puede continuar borrowed donde corresponda, por ejemplo:

```text
Value::Struct
└── Box<[Value]>
    ├── Int32(10)
    ├── String(&str)
    └── Boolean(true)
```

No se introduce una arena o estructura self-referential únicamente para construir una vista de Struct.

## EV-007 — Borrowed Enum representation

Status: CLOSED

Enum utiliza ordinal de variant y payload explícito:

```rust
pub enum EnumPayload<'value> {
    Simple,
    Associated(Box<Value<'value>>),
    Structured {
        fields: Box<[Value<'value>]>,
    },
}
```

```rust
Value::Enum {
    variant: usize,
    payload: EnumPayload<'value>,
}
```

`Box<Value<'a>>` en `Associated` es indirection física necesaria para romper recursión infinita de tamaño; no agrega semántica de ownership de backing.

Structured fields preservan orden canónico.

## EV-008 — OwnedValue mirrors the same 17 semantic families

Status: CLOSED

`OwnedValue` es completamente autónomo y refleja las mismas 17 familias semánticas:

```rust
pub enum OwnedValue {
    Boolean(bool),

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

    String(Box<str>),

    Dynamic(OwnedDynamicValue),

    Struct(Box<[OwnedValue]>),

    Enum {
        variant: usize,
        payload: OwnedEnumPayload,
    },
}
```

No contiene Rust references ni handles de una VM concreta.

## EV-009 — Exact owned backing forms

Status: CLOSED

Owned Dynamic:

```rust
pub enum OwnedDynamicValue {
    Integer(OwnedDynamicInteger),
    Float32(f32),
    Float64(f64),
}
```

```rust
pub struct OwnedDynamicInteger {
    pub negative: bool,
    pub magnitude: Box<[u8]>,
}
```

Owned Enum:

```rust
pub enum OwnedEnumPayload {
    Simple,
    Associated(Box<OwnedValue>),
    Structured {
        fields: Box<[OwnedValue]>,
    },
}
```

Owned String:

```rust
String(Box<str>)
```

Owned Struct:

```rust
Struct(Box<[OwnedValue]>)
```

Los composites owned poseen recursivamente sus Values y Dynamic Integer conserva el mismo sign + canonical magnitude utilizado por la vista borrowed.

## EV-010 — No semantic/reflection metadata inside Value

Status: CLOSED

Ni `Value<'a>` ni `OwnedValue` contienen:

```text
TypeId
type name
Struct name
Enum name
field names
variant names
SignatureSymbol
Provider identity
RuntimeValue
StringBackingId
DynamicIntegerBackingId
StructBackingId
EnumBackingId
ConstantId
reflection data
```

La Signature/contractual context determina cómo interpretar positional parameters y result; Value transporta datos.

## EV-011 — OwnedValue is a distinct type

Status: CLOSED

`OwnedValue` no se modela como:

```rust
Value<'static>
```

porque `'static` describe lifetime de una referencia y no ownership de sus bytes.

Tampoco se introduce en v0 una abstracción genérica:

```text
Value<TStorage>
Value<Borrowed>
Value<Owned>
```

con traits/genéricos de storage.

Las dos responsabilidades se expresan directamente mediante:

```text
Value<'a>
OwnedValue
```

## Canonical Boundary Flow

```text
VM internal observation

RuntimeValue
    ↓ materialize borrowed interchange tree
Value<'a>
    ↓
ExternalCapability
```

```text
External success transfer

ExternalCapability
    ↓
OwnedValue
    ↓ consume/materialize
RuntimeValue
    +
ExecutionBackingStore ownership when required
```

Ejemplos:

```text
OwnedValue::String(Box<str>)
    ↓ move
ExecutionBackingStore.strings
    ↓
StringBackingId
    ↓
RuntimeValue::String(Execution(...))
```

```text
OwnedValue::Struct(Box<[OwnedValue]>)
    ↓ recursively materialize
Box<[RuntimeValue]>
    ↓
StructBacking
    ↓
StructBackingId
```

La posible reutilización futura de `OwnedValue` como representación pública de un successful execution Outcome se analizará en Outcome / Diagnostic Data; no queda cerrada por este documento.

## Traits and implementation details not closed here

Este cierre no prescribe todavía derives concretos como:

```text
Clone
PartialEq
Eq
Hash
Debug
```

ni módulos/paths Rust exactos dentro de `evo-values`.

Esos detalles se fijarán al producir las firmas/tareas de implementación si no cambian la semántica aquí cerrada.

## Closure

```text
EV-001 neutral interchange ownership                         ✅ CLOSED
EV-002 no_std + alloc / no BigInt dependency                ✅ CLOSED
EV-003 Value<'a> exact 17 semantic families                 ✅ CLOSED
EV-004 String + exact-width numerics                        ✅ CLOSED
EV-005 borrowed Dynamic exact 3 families + canonical integer ✅ CLOSED
EV-006 temporary owned Struct descriptor tree               ✅ CLOSED
EV-007 borrowed Enum ordinal + payload                      ✅ CLOSED
EV-008 OwnedValue exact 17 semantic families                ✅ CLOSED
EV-009 exact owned string/dynamic/composite forms           ✅ CLOSED
EV-010 no type/reflection/provider/runtime metadata         ✅ CLOSED
EV-011 OwnedValue distinct; no Value<'static>/generic store ✅ CLOSED

Borrowed / owned interchange model                          ✅ CLOSED
ExternalCapability exact Value argument/result types        ✅ CLOSED
VmExecution exact Rust root                                 ✅ CLOSED elsewhere
ExternalCapability failure type                             PENDING — Outcome / Diagnostic Data
Compiled Boundary Value Shape                               ← NEXT in evo-script-engine
```
