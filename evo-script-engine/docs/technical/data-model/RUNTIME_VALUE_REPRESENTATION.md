# Evo-Script Engine — RuntimeValue Exact Representation

Status: CLOSED

Este documento cierra la representación exacta v0 de `RuntimeValue` y `DynamicValue` dentro de `evo-script-engine`.

La autoridad deriva de:

- `RUNTIME_VALUE_MODEL.md`;
- `BACKING_IDENTITY_STRATEGY.md`;
- `COMPILED_PROGRAM_INVENTORY.md`;
- `DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`.

## RV-012 — RuntimeValue exact variant inventory

Status: CLOSED

`RuntimeValue` contiene exactamente **17 variants**:

```rust
#[derive(Clone, Copy)]
enum RuntimeValue {
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

    String(StringBackingRef),
    Dynamic(DynamicValue),
    Struct(StructBackingId),
    Enum(EnumBackingId),
}
```

Count:

```text
Boolean                  1
signed fixed integers    5
unsigned fixed integers  5
fixed floats             2
String                   1
Dynamic                  1
Struct                   1
Enum                     1
                        ──
TOTAL                    17
```

Canonical physical mapping permanece:

```text
semantic int   → RuntimeValue::Int32
semantic int32 → RuntimeValue::Int32

semantic float   → RuntimeValue::Float64
semantic float64 → RuntimeValue::Float64
```

No existen `RuntimeValue::Int` ni `RuntimeValue::Float` separados.

## RV-013 — No NumericValue intermediate identity

Status: CLOSED

Los fixed numeric kinds son variants directas de `RuntimeValue`.

No se introduce:

```text
NumericValue
FixedNumericValue
RuntimeNumeric
```

como identity intermedia.

El `Instruction` ya contiene el `NumericKind` necesario para seleccionar el mecanismo ejecutable; `RuntimeValue` representa únicamente el dato físico correspondiente.

## RV-014 — Dynamic is represented as RuntimeValue::Dynamic(DynamicValue)

Status: CLOSED

`dynamic` conserva una única entrada dentro del enum general:

```rust
RuntimeValue::Dynamic(DynamicValue)
```

No se dispersa como:

```text
RuntimeValue::DynamicInteger
RuntimeValue::DynamicFloat32
RuntimeValue::DynamicFloat64
```

La familia runtime específica pertenece a `DynamicValue`.

## RV-015 — DynamicValue exact variant inventory

Status: CLOSED

`DynamicValue` posee exactamente **3 variants**:

```rust
#[derive(Clone, Copy)]
enum DynamicValue {
    Integer(DynamicIntegerBackingRef),
    Float32(f32),
    Float64(f64),
}
```

Estas variants corresponden exactamente a las familias normativas runtime de `dynamic`:

```text
Dynamic Numeric Value
├── Integer
├── Float32
└── Float64
```

No existe `DynamicKind`, `RuntimeTypeId` ni otro type tag separado; el discriminante de `DynamicValue` expresa la familia runtime.

## RV-016 — Dynamic Integer uses backing; Dynamic floats stay inline

Status: CLOSED

```text
Dynamic Integer
    → DynamicIntegerBackingRef

Dynamic Float32
    → inline f32

Dynamic Float64
    → inline f64
```

Dynamic Integer necesita backing porque conserva precisión arbitraria.

Dynamic Float32 y Dynamic Float64 poseen tamaño fijo y permanecen inline dentro del descriptor.

## RV-017 — RuntimeValue family is copyable descriptor data

Status: CLOSED

Las identities cerradas para descriptor/handle son `Clone + Copy` en v0:

```rust
#[derive(Clone, Copy)]
struct StringBackingId(usize);

#[derive(Clone, Copy)]
struct DynamicIntegerBackingId(usize);

#[derive(Clone, Copy)]
struct StructBackingId(usize);

#[derive(Clone, Copy)]
struct EnumBackingId(usize);
```

Y, por composición:

```text
StringBackingRef          Clone + Copy
DynamicIntegerBackingRef  Clone + Copy
DynamicValue              Clone + Copy
RuntimeValue              Clone + Copy
```

Regla canónica:

> Copiar un `RuntimeValue` duplica únicamente el descriptor; nunca duplica el backing referenciado.

Esto permite que `LoadParameter` y `LoadLocal` materialicen un Value en Operand Window sin mover o clonar el backing completo.

## RV-018 — Rust PartialEq/Eq is not language equality mechanism

Status: CLOSED

La igualdad semántica de Evo-Script no se define mediante identity equality de `RuntimeValue` ni de sus backing handles.

Dos descriptors distintos pueden representar Values semánticamente iguales:

```text
String backing A != String backing B
pero contenido textual A == contenido textual B
```

Lo mismo aplica a Struct y Enum backing.

Las operaciones del lenguaje permanecen gobernadas por:

```text
EqualNumeric / NotEqualNumeric
EqualBoolean / NotEqualBoolean
EqualString / NotEqualString
EqualComposite / NotEqualComposite
EqualityRule
CompositeEqualityPlan
```

Por tanto `PartialEq` / `Eq`, si una implementación futura los agrega por razones técnicas internas, no constituyen la semántica de `==` / `!=` del lenguaje.

## RV-019 — RuntimeValue is execution-context-relative

Status: CLOSED

`RuntimeValue` no es un Value portable e independiente del contexto.

Ejemplos:

```text
StringBackingRef::Compiled(ConstantId)
    → se resuelve contra el CompiledProgram de la VmExecution

StringBackingRef::Execution(StringBackingId)
    → se resuelve contra backing owned por esa VmExecution

StructBackingId / EnumBackingId
    → se resuelven contra backing de esa VmExecution
```

Regla canónica:

> Un `RuntimeValue` que contenga handles runtime no puede escapar de `VmExecution` como resultado autónomo sin materialización o transferencia de ownership apropiada.

La transformación hacia un Outcome Value capaz de sobrevivir a `VmExecution` pertenece a `Outcome / Diagnostic Data`.

## Exact Closed Family

```rust
#[derive(Clone, Copy)]
enum DynamicValue {
    Integer(DynamicIntegerBackingRef),
    Float32(f32),
    Float64(f64),
}

#[derive(Clone, Copy)]
enum RuntimeValue {
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

    String(StringBackingRef),
    Dynamic(DynamicValue),
    Struct(StructBackingId),
    Enum(EnumBackingId),
}
```

## Explicitly Excluded

```text
RuntimeValue::Int
RuntimeValue::Float
NumericValue
FixedNumericValue
DynamicKind
RuntimeTypeId
TypeId inside RuntimeValue
StructTypeId / EnumTypeId
semantic equality through handle identity
RuntimeValue as portable outcome data
```

## Closure

```text
RV-012 RuntimeValue exact 17 variants                  ✅ CLOSED
RV-013 no NumericValue intermediate                    ✅ CLOSED
RV-014 RuntimeValue::Dynamic(DynamicValue)             ✅ CLOSED
RV-015 DynamicValue exact 3 variants                   ✅ CLOSED
RV-016 Dynamic Integer backing / floats inline         ✅ CLOSED
RV-017 descriptor family Clone + Copy                  ✅ CLOSED
RV-018 Rust equality != Evo language equality          ✅ CLOSED
RV-019 RuntimeValue execution-context-relative         ✅ CLOSED

RuntimeValue exact representation                      ✅ CLOSED
Dynamic Value exact representation                     ✅ CLOSED

Backing Data Representation                            ← NEXT
Shared Value Storage exact representation              PENDING
CallFrame                                               PENDING
InstructionPointer                                      PENDING
ApplicationBindings exact model                         PENDING
call / return mechanics                                 PENDING
```
