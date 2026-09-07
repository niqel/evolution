# Evo-Script Engine — Runtime Value Model

Status:
- HISTORICAL RUNTIME VALUE DESIGN: CLOSED / PRESERVED
- CURRENT RECONCILED RUNTIME VALUE MODEL: CLOSED
- Authority: [`../EVO_VALUES_V0_1_RECONCILIATION.md`](../EVO_VALUES_V0_1_RECONCILIATION.md)

Este documento registra las decisiones cerradas del Runtime Value Model de `evo-script-engine` v0 reconciliado con `evo-values v0.1`.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`, especialmente TD-005, TD-006, TD-007 y TD-008;
- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_PROGRAM_INVENTORY.md`;
- `VM_EXECUTION_DATA.md`;
- `../EVO_VALUES_V0_1_RECONCILIATION.md`;
- el modelo de `evo-values::Value<'a>` como borrowed/interchange view, no como storage interno de la VM.

## RV-001 — RuntimeValue != evo_values::Value<'a>

Status: CLOSED

Regla canónica:

> `RuntimeValue` y `evo_values::Value<'a>` tienen responsabilidades técnicas distintas y no se identifican como el mismo tipo por conveniencia.

`RuntimeValue` pertenece al estado interno estable de la Stack VM.

`evo_values::Value<'a>` representa una view/interchange value cuyo lifetime puede depender de un materializer externo o de backing data ya owned.

> [!NOTE]
> **HISTORICAL / SUPERSEDED**: La observación histórica de que *"la forma de evo-values no cubre todavía el lenguaje completo de Evo-Script v0"* ha quedado superada por la reconciliación de `evo-values v0.1`, que provee el modelo canónico universal de valores, operaciones dinámicas y comparaciones. La separación de responsabilidades entre `RuntimeValue` (descriptor interno de ejecución) y `Value<'a>` (interchange view) permanece estrictamente vigente.

## RV-002 — RuntimeValue is an internal immutable VM descriptor

Status: CLOSED

> `RuntimeValue` es un descriptor interno e inmutable que expresa un Value ya materializado para ejecución; no es por sí mismo el owner obligatorio de todo backing data variable o composite.

```text
RuntimeValue
    = executable value descriptor

RuntimeValue
    != semantic type identity
    != Provider-owned borrowed view
    != generic heap owner
```

El descriptor participa en Parameters, Locals y Operand Window sin reintroducir name resolution, TypeId o semantic metadata.

## RV-003 — Fixed scalar data is inline

Status: CLOSED

Los fixed scalar Values se almacenan directamente dentro de `RuntimeValue`:

```text
Boolean
Int8 / Int16 / Int32 / Int64 / Int128
Uint8 / Uint16 / Uint32 / Uint64 / Uint128
Float32 / Float64
```

Canonical physical mapping:

```text
semantic int   → runtime Int32
semantic int32 → runtime Int32
semantic float   → runtime Float64
semantic float64 → runtime Float64
```

No reaparecen `RuntimeValue::Int` ni `RuntimeValue::Float`.

## RV-004 — Variable/composite data uses backing indirection

Status: CLOSED

Los datos cuyo tamaño o estructura no es apropiado para vivir inline utilizan backing indirection.

La categoría incluye:

```text
String
Dynamic Integer
Struct
Enum
```

`CompiledProgram` puede continuar siendo owner de constant backing data, mientras `VmExecution` es owner lógico del backing que deba sobrevivir durante la ejecución.

## RV-005 — Shared Value Storage must not self-borrow VmExecution backing

Status: CLOSED

> `Shared Value Storage` no contiene referencias Rust directas hacia backing data owned por el mismo `VmExecution` cuando eso convertiría la estructura de ejecución en self-referential.

La relación correcta es:

```text
VmExecution
├── owns execution backing
└── owns Shared Value Storage
      └── RuntimeValue descriptor
            └── stable indirection to backing
```

Borrowed views temporales siguen permitidas al observar/materializar Values.

## RV-006 — Typed Backing Identities

Status: CLOSED

Cerrado en `BACKING_IDENTITY_STRATEGY.md`.

```rust
struct StringBackingId(usize);
struct DynamicIntegerBackingId(usize);
struct StructBackingId(usize);
struct EnumBackingId(usize);
```

No existe un `RuntimeBackingId` universal en v0.

## RV-007 — String Backing Reference

Status: CLOSED

```rust
enum StringBackingRef {
    Compiled(ConstantId),
    Execution(StringBackingId),
}
```

`Compiled(ConstantId)` requiere `Constant::String`.

## RV-008 — Dynamic Integer Backing Reference

Status: CLOSED

```rust
enum DynamicIntegerBackingRef {
    Compiled(ConstantId),
    Execution(DynamicIntegerBackingId),
}
```

`Compiled(ConstantId)` requiere `Constant::Dynamic(DynamicConstant::Integer { ... })`.

## RV-009 — Struct / Enum Backing Identities

Status: CLOSED

Struct y Enum se construyen durante ejecución en v0 y utilizan directamente:

```rust
StructBackingId
EnumBackingId
```

## RV-010 — Backing ID Stability

Status: CLOSED

Todo backing ID válido conserva la misma identity durante la vida completa de la `VmExecution` que lo creó y no se reutiliza dentro de esa ejecución.

```text
allocate ID
    → stable for VmExecution lifetime
    → no reuse in v0
    → released with VmExecution
```

## RV-011 — Identity Does Not Prescribe Container

Status: CLOSED

Backing identities por sí mismas no prescriben el container físico.

La estrategia concreta queda definida separadamente por `BACKING_DATA_REPRESENTATION.md` para v0.

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

## RV-013 — No NumericValue intermediate identity

Status: CLOSED

Los fixed numeric kinds son variants directas de `RuntimeValue`.

No se introduce `NumericValue`, `FixedNumericValue` ni `RuntimeNumeric` como identity intermedia.

`Instruction` ya contiene el `NumericKind` requerido para seleccionar el mecanismo ejecutable.

## RV-014 — Dynamic uses RuntimeValue::Dynamic(DynamicValue)

Status: CLOSED

`dynamic` conserva una única entrada dentro del enum general:

```rust
RuntimeValue::Dynamic(DynamicValue)
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

No existe `DynamicKind`, `RuntimeTypeId` ni type tag separado; el discriminante del propio `DynamicValue` expresa la familia runtime.

## RV-016 — Dynamic Integer uses backing; Dynamic floats stay inline

Status: CLOSED

```text
Dynamic Integer → DynamicIntegerBackingRef
Dynamic Float32 → inline f32
Dynamic Float64 → inline f64
```

Dynamic Integer necesita backing por precisión arbitraria. Dynamic Float32/Float64 permanecen inline.

## RV-017 — RuntimeValue family is copyable descriptor data

Status: CLOSED

La familia de descriptors/handles runtime es `Clone + Copy` en v0.

> Copiar un `RuntimeValue` duplica únicamente el descriptor; nunca duplica el backing referenciado.

Esto permite que `LoadParameter` y `LoadLocal` materialicen el mismo Value lógico en Operand Window sin copiar String, Dynamic Integer, Struct o Enum backing.

## RV-018 — Rust PartialEq/Eq is not language equality mechanism

Status: CLOSED (reconciled with evo-values v0.1)

La igualdad semántica de Evo-Script no se define mediante identity equality de `RuntimeValue` ni de sus backing handles.

Dos backings distintos pueden representar Values semánticamente iguales.

```text
Rust PartialEq/Eq
    !=
Evo-Script semantic equality
```

La autoridad de igualdad semántica está dividida conforme a la reconciliación cerrada:

```text
scalar/composite semantic comparison
    → evo-values Comparison
    → EQUAL / NOT_EQUAL y demás UCs aplicables

Evo-Script
    → conserva availability + static compatibility
```

Para structural equality:

```text
RuntimeValue
    → OBSERVE_RUNTIME_VALUE
    → Value
    → EQUAL / NOT_EQUAL (evo-values Comparison)
```

No se introduce `PartialEq` semántico en `RuntimeValue`, `Value` ni en los tipos backing. Las identidades históricas de planes (`EqualityRule`, `CompositeEqualityPlan`) quedan superadas por la delegación directa en el kernel de Comparison de `evo-values`.

## RV-019 — RuntimeValue is execution-context-relative

Status: CLOSED

`RuntimeValue` no es un Value portable independiente del contexto.

```text
Compiled(ConstantId)
    → se resuelve contra el CompiledProgram de la VmExecution

Execution(BackingId)
    → se resuelve contra backing owned por esa VmExecution
```

> Un `RuntimeValue` que contenga handles runtime no puede escapar de `VmExecution` como resultado autónomo sin materialización o transferencia de ownership apropiada.

La transformación hacia un Outcome Value capaz de sobrevivir a `VmExecution` pertenece a `Outcome / Diagnostic Data`.

## RV-020 — Backing Data Representation

Status: CLOSED (reconciled with evo-values v0.1)

Cerrado en `BACKING_DATA_REPRESENTATION.md` mediante BD-001..BD-009.

`VmExecution` posee exactamente un:

```rust
use evo_values::OwnedDynamicInteger;

struct ExecutionBackingStore {
    strings: Vec<Box<str>>,
    dynamic_integers: Vec<DynamicIntegerBacking>,
    structs: Vec<StructBacking>,
    enums: Vec<EnumBacking>,
}

struct DynamicIntegerBacking {
    value: OwnedDynamicInteger,
}
```

Los stores son tipados, append-only y resuelven sus typed IDs posicionalmente. `OwnedDynamicInteger` es una identity reutilizada desde `evo-values` y no incrementa el inventario de identities propias de VM Execution Data.

Las representaciones cerradas para composites son:

```rust
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

Execution String backing utiliza `Box<str>` inmutable.

`DynamicIntegerBacking` encapsula un `OwnedDynamicInteger` de `evo-values`. La crate/implementación concreta subyacente (`num-bigint`) permanece como detalle privado de `evo-values` y no forma parte de la arquitectura del engine.

Adaptación para observación:

```text
Execution Dynamic Integer
    → backing.value.as_borrowed()
    → borrowed DynamicIntegerValue
```

Esto permite observar y delegar operaciones dinámicas hacia `evo-values` sin convertir a BigInt ni copiar magnitudes.

Todos los execution backings son inmutables después de insertarse. El graph de Struct/Enum backing es finito, inmutable y acíclico; sharing por typed IDs está permitido.

## Exact Closed Runtime Value Family

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

## Closure

```text
RuntimeValue / evo_values::Value boundary                ✅ CLOSED
RuntimeValue immutable descriptor                        ✅ CLOSED
fixed scalar data inline                                 ✅ CLOSED
variable/composite backing indirection                    ✅ CLOSED
no persistent self-borrow                                ✅ CLOSED
Backing Identity Strategy                                ✅ CLOSED
RuntimeValue exact representation                        ✅ CLOSED — 17 variants
DynamicValue exact representation                        ✅ CLOSED — 3 variants
descriptor family Clone + Copy                           ✅ CLOSED
Rust handle equality != language equality                ✅ CLOSED (reconciled with evo-values Comparison)
RuntimeValue execution-context-relative                  ✅ CLOSED
Backing Data Representation                              ✅ CLOSED (reconciled with OwnedDynamicInteger)
ExecutionBackingStore                                    ✅ CLOSED
String execution backing                                 ✅ CLOSED
Dynamic Integer execution backing                        ✅ CLOSED (reconciled)
Struct / Enum backing                                    ✅ CLOSED
immutable finite composite backing DAG                    ✅ CLOSED

HISTORICAL PROGRESSION — CLOSED / PRESERVED

Shared Value Storage exact representation    ✅ subsequently CLOSED
CallFrame                                    ✅ subsequently CLOSED
InstructionPointer                           ✅ subsequently CLOSED
ApplicationBindings exact model              ✅ subsequently CLOSED
call / return mechanics                      ✅ subsequently CLOSED
```
