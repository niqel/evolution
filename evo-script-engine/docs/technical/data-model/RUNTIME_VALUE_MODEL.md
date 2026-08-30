# Evo-Script Engine — Runtime Value Model

Status: RUNTIME VALUE MODEL — IN ANALYSIS

Este documento registra las decisiones cerradas del Runtime Value Model de `evo-script-engine` v0.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`, especialmente TD-005, TD-006, TD-007 y TD-008;
- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_PROGRAM_INVENTORY.md`;
- `VM_EXECUTION_DATA.md`;
- el modelo actual de `evo-values::Value<'a>` como evidencia de una borrowed/interchange view, no como autoridad automática sobre el storage interno de la VM.

## RV-001 — RuntimeValue != evo_values::Value<'a>

Status: CLOSED

Regla canónica:

> `RuntimeValue` y `evo_values::Value<'a>` tienen responsabilidades técnicas distintas y no se identifican como el mismo tipo por conveniencia.

`RuntimeValue` pertenece al estado interno estable de la Stack VM.

`evo_values::Value<'a>` representa una view/interchange value cuyo lifetime puede depender de un materializer externo o de backing data ya owned.

Actualmente `evo-values` conserva una forma histórica como:

```rust
pub enum Value<'value> {
    Text(&'value str),
    Unsigned(u64),
    Signed(i64),
    Boolean(bool),
}
```

Esa forma no cubre todavía el lenguaje completo de Evo-Script v0 y no debe forzar la representación interna de VM Execution Data.

La futura evolución de `evo-values` se decidirá desde su propia responsabilidad arquitectónica una vez cerrado el Runtime Value Model completo.

## RV-002 — RuntimeValue is an internal immutable VM descriptor

Status: CLOSED

Regla canónica:

> `RuntimeValue` es un descriptor interno e inmutable que expresa un Value ya materializado para ejecución; no es por sí mismo el owner obligatorio de todo backing data variable o composite.

Consecuencias:

```text
RuntimeValue
    = executable value descriptor

RuntimeValue
    != semantic type identity
    != Provider-owned borrowed view
    != generic heap owner
```

El descriptor debe poder participar en Parameters, Locals y Operand Window sin reintroducir name resolution, TypeId o semantic metadata.

La arquitectura debe permitir que copiar/mover un descriptor de un Value inmutable no implique copiar por costumbre todo su backing data.

No se cierra todavía `Copy`, `Clone` ni layout Rust exacto; esos traits dependen de la estrategia final de handles/backing identities.

## RV-003 — Fixed scalar data is inline

Status: CLOSED

Los fixed scalar Values se almacenan directamente dentro de `RuntimeValue`.

Familias cerradas:

```text
Boolean

Int8
Int16
Int32
Int64
Int128

Uint8
Uint16
Uint32
Uint64
Uint128

Float32
Float64
```

Canonical physical mapping se conserva:

```text
semantic int   → runtime Int32
semantic int32 → runtime Int32

semantic float   → runtime Float64
semantic float64 → runtime Float64
```

No reaparecen `RuntimeValue::Int` ni `RuntimeValue::Float` como identidades físicas separadas.

No se introduce una indirection/arena obligatoria para fixed scalars mientras el valor pueda vivir directamente en el descriptor.

## RV-004 — Variable/composite data uses backing indirection

Status: CLOSED

Los datos cuyo tamaño o estructura no es apropiado para vivir inline en el descriptor utilizan backing indirection.

La categoría incluye al menos:

```text
String
Dynamic Integer
Struct
Enum
```

Conceptualmente:

```text
RuntimeValue
├── fixed scalar inline
└── variable/composite
    └── backing reference / handle
```

`CompiledProgram` puede continuar siendo owner de constant backing data, mientras `VmExecution` es owner lógico del backing que deba sobrevivir durante la ejecución.

## RV-005 — Shared Value Storage must not self-borrow VmExecution backing

Status: CLOSED

Regla canónica:

> `Shared Value Storage` no contiene referencias Rust directas hacia backing data owned por el mismo `VmExecution` cuando eso convertiría la estructura de ejecución en self-referential.

No se cierra una forma como:

```rust
struct VmExecution<'a> {
    owned_strings: Vec<String>,
    values: Vec<RuntimeValue<'a>>, // references into owned_strings
}
```

como modelo base.

La relación correcta debe permitir:

```text
VmExecution
├── owns execution backing
└── owns Shared Value Storage
      └── RuntimeValue descriptor
            └── stable indirection to backing
```

sin que el storage dependa de referencias internas auto-prestadas del propio root.

Esta regla no prohíbe borrowed views temporales creadas al observar/materializar un RuntimeValue. Prohíbe que el storage persistente de la ejecución dependa estructuralmente de self-borrows.

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

Aunque compartan representación `usize`, estas identities no son intercambiables.

## RV-007 — String Backing Reference

Status: CLOSED

```rust
enum StringBackingRef {
    Compiled(ConstantId),
    Execution(StringBackingId),
}
```

`Compiled(ConstantId)` reutiliza el Constant Pool ya cerrado y requiere que el constant referenciado sea `Constant::String`.

No existe `CompiledStringId`.

## RV-008 — Dynamic Integer Backing Reference

Status: CLOSED

```rust
enum DynamicIntegerBackingRef {
    Compiled(ConstantId),
    Execution(DynamicIntegerBackingId),
}
```

`Compiled(ConstantId)` requiere `Constant::Dynamic(DynamicConstant::Integer { ... })`.

No existe `CompiledDynamicIntegerId`.

## RV-009 — Struct / Enum Backing Identities

Status: CLOSED

Struct y Enum no poseen composite constants persistentes en el modelo compilado v0; se construyen durante ejecución.

Por tanto utilizan directamente:

```rust
StructBackingId
EnumBackingId
```

ambos pertenecientes al backing owned por `VmExecution`.

## RV-010 — Backing ID Stability

Status: CLOSED

Todo backing ID válido conserva la misma identity durante la vida completa de la `VmExecution` que lo creó y no se reutiliza dentro de esa ejecución.

```text
allocate ID
    → stable for VmExecution lifetime
    → no reuse in v0
    → released with VmExecution
```

No se requieren generation counters, GC, reference counting ni per-object liveness tracking en v0.

## RV-011 — Identity Does Not Prescribe Container

Status: CLOSED

Backing identities no prescriben el container físico.

Permanece abierto si la implementación usa:

```text
Vec
Arena
Slab
Box
segmented storage
custom allocator
```

No se requiere `Rc`, `Arc`, raw pointer ni offset como parte contractual de la identity.

## Current Runtime Value Shape — conceptual only

Todavía no se cierra el enum Rust exacto, pero la forma contractual es:

```text
RuntimeValue
│
├── Boolean                       inline
│
├── Fixed Numeric                 inline
│   ├── Int8 .. Int128
│   ├── Uint8 .. Uint128
│   └── Float32 / Float64
│
├── String
│   └── StringBackingRef
│       ├── Compiled(ConstantId)
│       └── Execution(StringBackingId)
│
├── Dynamic
│   ├── Integer
│   │   └── DynamicIntegerBackingRef
│   │       ├── Compiled(ConstantId)
│   │       └── Execution(DynamicIntegerBackingId)
│   ├── Float32                   inline candidate
│   └── Float64                   inline candidate
│
├── Struct
│   └── StructBackingId
└── Enum
    └── EnumBackingId
```

La forma exacta de `RuntimeValue`, Dynamic Float variants y composite backing data todavía debe cerrarse.

## Explicitly Not Closed Yet

```text
RuntimeValue exact Rust enum
Dynamic Value exact representation
String backing physical representation
Dynamic Integer backing physical representation
Struct backing physical representation
Enum backing physical representation
Shared Value Storage concrete container
Copy / Clone traits
arena / Vec / slab / Box strategy
```

## Closure

```text
RuntimeValue != evo_values::Value<'a>                  ✅ CLOSED
RuntimeValue = internal immutable VM descriptor        ✅ CLOSED
fixed scalar data stored inline                        ✅ CLOSED
variable/composite data uses backing indirection       ✅ CLOSED
no persistent self-borrow in Shared Value Storage      ✅ CLOSED

typed backing identities                               ✅ CLOSED
no universal RuntimeBackingId                          ✅ CLOSED
String compiled/execution backing reference            ✅ CLOSED
Dynamic Integer compiled/execution backing reference   ✅ CLOSED
Struct / Enum execution-owned typed IDs                ✅ CLOSED
backing IDs stable / non-reused per execution          ✅ CLOSED
identity independent of physical container             ✅ CLOSED

Backing Identity Strategy                              ✅ CLOSED

RuntimeValue exact representation                      ← NEXT
Dynamic Value exact representation                     PENDING
String / Dynamic Integer backing representation        PENDING
Struct / Enum backing representation                   PENDING
Shared Value Storage exact representation              PENDING
```
