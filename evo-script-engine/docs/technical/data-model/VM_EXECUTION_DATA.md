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

Este cierre no prescribe:

```text
Vec
Arena
Box
Rc
Arc
internal references
handles
indices
```

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

## Current Closure

```text
VmExecution responsibility                 ✅ CLOSED
one invocation per VmExecution             ✅ CLOSED
CompiledProgram relationship               ✅ CLOSED
ApplicationBindings relationship           ✅ CLOSED — exact model pending
one Shared Value Storage root               ✅ CLOSED
Call Frames root ownership                  ✅ CLOSED
execution-lifetime backing logical owner    ✅ CLOSED
root InstructionPointer                     ❌ EXCLUDED
Host / Active Scope / Current Provider      ❌ EXCLUDED
Outcome / Diagnostic data                   ❌ SEPARATE PHASE

Runtime Value Model                         ← NEXT
Shared Value Storage exact representation   PENDING
CallFrame                                   PENDING
InstructionPointer                          PENDING
ApplicationBindings exact model             PENDING
call / return mechanics                     PENDING
```
