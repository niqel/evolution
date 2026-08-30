# Evo-Script Engine — VM Execution Data

Status: VM EXECUTION DATA — IN ANALYSIS

Este documento es la autoridad acumulada del estado runtime mutable utilizado para ejecutar un `CompiledProgram` en `evo-script-engine` v0.

```text
Compiled Program
    ↓ referenced by
VM Execution
    ↓
Outcome
```

## VM-001 — VmExecution Root Responsibility

Status: CLOSED

`VmExecution` representa el estado mutable y aislado de exactamente una invocation de un `CompiledProgram`.

> `CompiledProgram` describe mecanismo ejecutable persistente; `VmExecution` describe el estado mutable de una sola ejecución de ese mecanismo.

Un mismo `CompiledProgram` puede participar en múltiples `VmExecution` independientes.

La ejecución no duplica Functions, Constant Pool, External Symbols ni SourceMap.

## VM-002 — CompiledProgram Relationship

Status: CLOSED

`VmExecution` referencia exactamente un `CompiledProgram`; no lo posee como copia mutable.

La forma Rust exacta del borrow/lifetime se cierra junto con las demás relaciones runtime borrowed.

## VM-003 — Application Bindings Relationship

Status: CLOSED — RELATIONSHIP ONLY

La ejecución referencia exactamente un conjunto explícito de `ApplicationBindings` utilizado para resolver `ExternalSymbolId` durante `CallExternal`.

`VmExecution` no posee `Current Provider`, provider lookup ambiental ni Host Session State.

La estructura exacta de `ApplicationBindings` permanece pendiente.

## VM-004 — One Shared Value Storage

Status: CLOSED — ROOT OWNERSHIP

TD-006 y TD-007 establecen que Parameters, Locals y Operands comparten un único storage lógico de Values por ejecución.

```text
Shared Value Storage

frame_base
    ↓
[parameters][locals][temporaries...]
                     ↑
                 operand_base
```

Cada `CallFrame` delimita una región lógica propia dentro de este storage.

## VM-005 — Call Frames

Status: CLOSED — ROOT OWNERSHIP

`VmExecution` posee una colección ordenada LIFO de `CallFrame`.

`InstructionPointer`, current `FunctionId` y frame boundaries pertenecen al `CallFrame` activo, no se duplican en `VmExecution`.

## VM-006 — Execution-Lifetime Backing Ownership

Status: CLOSED — LOGICAL OWNERSHIP

Se preserva TD-008:

```text
borrow mientras alcance
ownership cuando deba sobrevivir
```

Cuando datos producidos por una External Capability deben sobrevivir a la invocación/materializador original, `VmExecution` es su owner lógico natural durante el execution lifetime.

`CallFrame`, Parameter Slots, Local Slots y Shared Value Storage no se convierten automáticamente en owners del backing data.

## VM-007 — Invocation Lifetime

Status: CLOSED

```text
invocation
    ↓
create VmExecution
    ↓
initialize entry CallFrame
    ↓
execute bytecode
    ↓
final Return
    ↓
produce Outcome
    ↓
VmExecution ends
```

`VmExecution` no representa una VM global con estado persistente entre invocations.

## VM-008 — Explicitly Excluded from Root

No pertenecen al root de ejecución:

```text
AST
SemanticProgram
TypeId
BindingId
Functions copy
Constant Pool copy
External Symbols copy
SourceMap copy
entry_point duplicate
root InstructionPointer
current FunctionId duplicate
Active Scope
Host Session State
Current Provider
Pipeline Data field
Outcome
EvaluationError
Diagnostic
line / column
```

## VM-009 — Root Conceptual Shape

Status: CLOSED

```text
VmExecution
│
├── references exactly 1 CompiledProgram
├── references exactly 1 ApplicationBindings
├── owns exactly 1 Shared Value Storage
├── owns ordered CallFrame collection
└── is logical owner of execution-lifetime backing data
```

No se cierra todavía un Rust struct exacto porque los fields concretos dependen de Shared Value Storage, CallFrame y backing representations.

## VM-010 — RuntimeValue and evo_values::Value<'a> are distinct

Status: CLOSED

```text
RuntimeValue
    = internal stable VM descriptor

evo_values::Value<'a>
    = borrowed/interchange view
```

## VM-011 — RuntimeValue is an immutable internal descriptor

Status: CLOSED

`RuntimeValue` representa un Value ejecutable ya materializado sin convertirse automáticamente en owner de todo backing variable/composite.

## VM-012 — Fixed Scalars Inline

Status: CLOSED

Fixed scalar Values viven directamente dentro de `RuntimeValue`:

```text
Boolean
Int8 / Int16 / Int32 / Int64 / Int128
Uint8 / Uint16 / Uint32 / Uint64 / Uint128
Float32 / Float64
```

Canonical physical mapping:

```text
int   / int32   → Int32
float / float64 → Float64
```

## VM-013 — Variable / Composite Data Uses Backing Indirection

Status: CLOSED

La categoría incluye:

```text
String
Dynamic Integer
Struct
Enum
```

## VM-014 — No Persistent Self-Borrow in Shared Value Storage

Status: CLOSED

`Shared Value Storage` no conserva direct Rust references hacia backing data owned por el mismo `VmExecution` cuando eso produciría una estructura self-referential.

Borrowed views temporales siguen permitidas al observar/materializar Values.

## VM-015 — Backing Identity Strategy

Status: CLOSED

Cerrado en `BACKING_IDENTITY_STRATEGY.md`.

```rust
struct StringBackingId(usize);
struct DynamicIntegerBackingId(usize);
struct StructBackingId(usize);
struct EnumBackingId(usize);
```

No existe `RuntimeBackingId` universal.

```rust
enum StringBackingRef {
    Compiled(ConstantId),
    Execution(StringBackingId),
}

enum DynamicIntegerBackingRef {
    Compiled(ConstantId),
    Execution(DynamicIntegerBackingId),
}
```

Struct y Enum usan execution-owned typed IDs. Todo backing ID es estable y no se reutiliza durante la vida de la `VmExecution` que lo creó.

## VM-016 — RuntimeValue Exact Representation

Status: CLOSED

Cerrado en `RUNTIME_VALUE_REPRESENTATION.md`.

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

`DynamicValue` contiene exactamente **3 variants**:

```rust
#[derive(Clone, Copy)]
enum DynamicValue {
    Integer(DynamicIntegerBackingRef),
    Float32(f32),
    Float64(f64),
}
```

Reglas cerradas:

```text
no NumericValue / FixedNumericValue intermediary
Dynamic family represented by DynamicValue discriminant
Dynamic Integer uses backing
Dynamic Float32 / Float64 stay inline
RuntimeValue family is Clone + Copy descriptor data
copying RuntimeValue never copies backing
Rust PartialEq/Eq is not Evo language equality
RuntimeValue is execution-context-relative
```

Un RuntimeValue con handles no puede escapar de `VmExecution` como resultado autónomo sin materialización o ownership transfer apropiado.

## Runtime Value Model Authority

Las reglas detalladas se registran en:

- `RUNTIME_VALUE_MODEL.md`
- `BACKING_IDENTITY_STRATEGY.md`
- `RUNTIME_VALUE_REPRESENTATION.md`

## Current Closure

```text
VmExecution responsibility                  ✅ CLOSED
one invocation per VmExecution              ✅ CLOSED
CompiledProgram relationship                ✅ CLOSED
ApplicationBindings relationship            ✅ CLOSED — exact model pending
one Shared Value Storage root                ✅ CLOSED
Call Frames root ownership                   ✅ CLOSED
execution-lifetime backing logical owner     ✅ CLOSED

RuntimeValue / evo_values::Value boundary    ✅ CLOSED
RuntimeValue immutable descriptor            ✅ CLOSED
fixed scalar inline                          ✅ CLOSED
variable/composite backing indirection       ✅ CLOSED
no persistent self-borrow in storage         ✅ CLOSED
Backing Identity Strategy                    ✅ CLOSED
RuntimeValue exact representation            ✅ CLOSED — 17 variants
DynamicValue exact representation            ✅ CLOSED — 3 variants
descriptor family Clone + Copy               ✅ CLOSED
RuntimeValue execution-context-relative      ✅ CLOSED

root InstructionPointer                      ❌ EXCLUDED
Host / Active Scope / Current Provider       ❌ EXCLUDED
Outcome / Diagnostic data                    ❌ SEPARATE PHASE

Backing Data Representation                  ← NEXT
Shared Value Storage exact representation    PENDING
CallFrame                                    PENDING
InstructionPointer                           PENDING
ApplicationBindings exact model              PENDING
call / return mechanics                      PENDING
```
