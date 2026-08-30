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

Regla canónica:

> `CompiledProgram` describe mecanismo ejecutable persistente; `VmExecution` describe el estado mutable de una sola ejecución de ese mecanismo.

Un mismo `CompiledProgram` puede participar en múltiples `VmExecution` independientes.

```text
CompiledProgram
├── VmExecution A
├── VmExecution B
└── VmExecution C
```

La ejecución no duplica Functions, Constant Pool, External Symbols ni SourceMap.

## VM-002 — CompiledProgram Relationship

Status: CLOSED

`VmExecution` referencia exactamente un `CompiledProgram`; no lo posee como copia mutable.

Conceptualmente:

```text
VmExecution
    │ references
    ▼
CompiledProgram
```

La forma Rust exacta del borrow/lifetime se cierra después junto con Runtime Value Model y las demás relaciones borrowed.

## VM-003 — Application Bindings Relationship

Status: CLOSED — RELATIONSHIP ONLY

La ejecución referencia exactamente un conjunto explícito de `ApplicationBindings` utilizado para resolver `ExternalSymbolId` durante `CallExternal`.

```text
VmExecution
    │ references
    ▼
ApplicationBindings
```

`VmExecution` no posee `Current Provider`, provider lookup ambiental ni Host Session State.

La estructura exacta de `ApplicationBindings` permanece pendiente porque sus firmas dependen del Runtime Value Model y del external borrowing boundary.

## VM-004 — One Shared Value Storage

Status: CLOSED — ROOT OWNERSHIP

TD-006 y TD-007 establecen que Parameters, Locals y Operands comparten un único storage lógico de Values por ejecución.

Por tanto `VmExecution` posee exactamente un `Shared Value Storage` conceptual.

No existen tres owners independientes:

```text
Parameter Storage
Local Storage
Operand Stack
```

La organización lógica es:

```text
Shared Value Storage

frame_base
    ↓
[parameters][locals][temporaries...]
                     ↑
                 operand_base
```

Cada `CallFrame` delimita una región lógica propia dentro de este storage.

La representación física exacta del storage queda pendiente del Runtime Value Model.

## VM-005 — Call Frames

Status: CLOSED — ROOT OWNERSHIP

`VmExecution` posee una colección ordenada LIFO de `CallFrame`.

Conceptualmente:

```text
VmExecution
└── Call Frames
    ├── Frame 0
    ├── Frame 1
    └── Frame N ← active frame
```

No se introduce un wrapper `CallStack` mientras `Vec<CallFrame>` o una colección equivalente no requiera responsabilidad propia adicional.

`InstructionPointer`, current `FunctionId` y frame boundaries pertenecen al `CallFrame` activo, no se duplican en `VmExecution`.

La representación exacta de `CallFrame` permanece pendiente.

## VM-006 — Execution-Lifetime Backing Ownership

Status: CLOSED — LOGICAL OWNERSHIP

Se preserva TD-008:

```text
borrow mientras alcance
ownership cuando deba sobrevivir
```

Cuando datos producidos por una External Capability deben sobrevivir a la invocación/materializador original, `VmExecution` es su owner lógico natural durante el execution lifetime.

```text
Provider / materializer
    │ ownership transfer when required
    ▼
VmExecution
    └── execution-lifetime backing data
```

`CallFrame`, Parameter Slots, Local Slots y Shared Value Storage no se convierten automáticamente en owners del backing data.

La representación física se decide en Runtime Value Model evitando introducir una self-referential structure por accidente.

## VM-007 — Invocation Lifetime

Status: CLOSED

Ciclo conceptual:

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

`entry_point` pertenece a `CompiledProgram`.

Current Function / Instruction Pointer pertenecen al active `CallFrame`.

Outcome / Diagnostic Data pertenece a la fase posterior.

## VM-009 — Root Conceptual Shape

Status: CLOSED

La forma conceptual cerrada es:

```text
VmExecution
│
├── references exactly 1 CompiledProgram
├── references exactly 1 ApplicationBindings
├── owns exactly 1 Shared Value Storage
├── owns ordered CallFrame collection
└── is logical owner of execution-lifetime backing data
```

No se cierra todavía un Rust struct exacto porque los fields concretos dependen de Runtime Value Model, Shared Value Storage y CallFrame.

## VM-010 — RuntimeValue and evo_values::Value<'a> are distinct

Status: CLOSED

```text
RuntimeValue
    = internal stable VM descriptor

evo_values::Value<'a>
    = borrowed/interchange view
```

No se obliga a que ambos conceptos compartan la misma Rust representation.

## VM-011 — RuntimeValue is an immutable internal descriptor

Status: CLOSED

`RuntimeValue` representa un Value ejecutable ya materializado sin convertirse automáticamente en owner de todo backing variable/composite.

La arquitectura debe permitir mover/copiar descriptors inmutables sin clonar por costumbre strings, dynamic integer backing o composite contents.

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
    └── backing indirection
```

## VM-014 — No Persistent Self-Borrow in Shared Value Storage

Status: CLOSED

`Shared Value Storage` no conserva direct Rust references hacia backing data owned por el mismo `VmExecution` cuando eso produciría una estructura self-referential.

La forma conceptual correcta es:

```text
VmExecution
├── owns execution backing
└── owns Shared Value Storage
      └── RuntimeValue descriptor
            └── stable indirection to backing
```

Borrowed views temporales siguen permitidas al observar/materializar Values; la prohibición aplica al storage persistente de la ejecución.

## VM-015 — Backing Identity Strategy

Status: CLOSED

Cerrado en `BACKING_IDENTITY_STRATEGY.md`.

Typed execution identities:

```rust
struct StringBackingId(usize);
struct DynamicIntegerBackingId(usize);
struct StructBackingId(usize);
struct EnumBackingId(usize);
```

No existe `RuntimeBackingId` universal.

String y Dynamic Integer distinguen origen:

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

Struct y Enum utilizan exclusivamente execution-owned typed IDs en v0:

```text
Struct → StructBackingId
Enum   → EnumBackingId
```

Todo backing ID es estable y no se reutiliza durante la vida de la `VmExecution` que lo creó.

La identity no prescribe `Vec`, arena, slab, Box ni allocator concreto.

## Runtime Value Model Authority

Las reglas detalladas se registran en:

- `RUNTIME_VALUE_MODEL.md`
- `BACKING_IDENTITY_STRATEGY.md`

## Current Closure

```text
VmExecution responsibility                  ✅ CLOSED
one invocation per VmExecution              ✅ CLOSED
CompiledProgram relationship                ✅ CLOSED
ApplicationBindings relationship            ✅ CLOSED — exact model pending
one Shared Value Storage root                ✅ CLOSED
Call Frames root ownership                   ✅ CLOSED
execution-lifetime backing logical owner     ✅ CLOSED

RuntimeValue != evo_values::Value<'a>        ✅ CLOSED
RuntimeValue immutable internal descriptor   ✅ CLOSED
fixed scalar inline                          ✅ CLOSED
variable/composite backing indirection       ✅ CLOSED
no persistent self-borrow in storage         ✅ CLOSED

Backing Identity Strategy                    ✅ CLOSED
typed backing IDs                            ✅ CLOSED
no universal RuntimeBackingId                ✅ CLOSED
String compiled/execution backing            ✅ CLOSED
Dynamic Integer compiled/execution backing   ✅ CLOSED
Struct / Enum execution-only backing IDs     ✅ CLOSED
backing ID stability / no reuse              ✅ CLOSED
container-independent identities             ✅ CLOSED

root InstructionPointer                      ❌ EXCLUDED
Host / Active Scope / Current Provider       ❌ EXCLUDED
Outcome / Diagnostic data                    ❌ SEPARATE PHASE

RuntimeValue exact representation            ← NEXT
Dynamic Value exact representation           PENDING
String / Dynamic Integer backing data        PENDING
Struct / Enum backing representation         PENDING
Shared Value Storage exact representation    PENDING
CallFrame                                    PENDING
InstructionPointer                           PENDING
ApplicationBindings exact model              PENDING
call / return mechanics                      PENDING
```
