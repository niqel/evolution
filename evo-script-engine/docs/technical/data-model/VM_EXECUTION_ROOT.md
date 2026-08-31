# Evo-Script Engine — VmExecution Exact Root

Status: CLOSED

Este documento cierra la representación Rust exacta del root mutable de una `VmExecution` en `evo-script-engine` v0.

La autoridad deriva de:

- `VM_EXECUTION_DATA.md`;
- `SHARED_VALUE_STORAGE.md`;
- `BACKING_DATA_REPRESENTATION.md`;
- `CALL_FRAME.md`;
- `APPLICATION_BINDINGS.md`;
- `EXTERNAL_CAPABILITY_ABI.md`;
- `evo-values/INTERCHANGE_MODEL.md`.

Este bloque cierra únicamente la composición persistente del root de ejecución. La validación exacta de Value shapes en fronteras de invocation/external capability se analiza separadamente en `COMPILED_BOUNDARY_VALUE_SHAPE.md`.

## VE-001 — Exact five-field root

Status: CLOSED

La representación exacta v0 es:

```rust
struct VmExecution<'compiled, 'bindings> {
    compiled_program: &'compiled CompiledProgram,
    application_bindings: &'bindings ApplicationBindings,
    value_storage: SharedValueStorage,
    backing_store: ExecutionBackingStore,
    call_frames: Vec<CallFrame>,
}
```

`VmExecution` contiene exactamente cinco fields persistentes.

## VE-002 — Independent external borrow lifetimes

Status: CLOSED

`CompiledProgram` y `ApplicationBindings` poseen lifetimes de borrow independientes:

```text
'compiled
    → CompiledProgram must outlive VmExecution

'bindings
    → ApplicationBindings must outlive VmExecution
```

No existe una regla semántica que obligue a ambos owners externos a compartir el mismo lifetime.

Por tanto v0 no los acopla artificialmente mediante un único lifetime.

## VE-003 — Exactly three mutable runtime roots

Status: CLOSED

`VmExecution` posee exactamente tres roots mutables de estado runtime:

```text
SharedValueStorage
ExecutionBackingStore
Vec<CallFrame>
```

Los otros dos fields son borrows inmutables hacia artifacts/composición externa.

## VE-004 — No persistent self-borrows

Status: CLOSED

`VmExecution` no conserva referencias Rust persistentes hacia datos owned por sí mismo.

Las relaciones internas utilizan:

```text
typed backing IDs
FunctionId
InstructionPointer
frame_base
positional boundaries
```

No existen fields persistentes como:

```text
&self.backing_store.strings[n]
&self.call_frames[n]
&self.value_storage.cells[n]
```

Esto preserva la regla de no self-referential execution storage.

## VE-005 — Active frame is the LIFO tail

Status: CLOSED

El frame activo es exactamente:

```text
call_frames.last()
```

No se almacenan duplicados:

```text
current_frame
current_frame_index
current_function
root InstructionPointer
```

El `FunctionId` y `InstructionPointer` activos pertenecen al último `CallFrame`.

## VE-006 — entry_point is derived

Status: CLOSED

`entry_point` permanece owned por `CompiledProgram` y no se duplica dentro de `VmExecution`.

```text
VmExecution.compiled_program.entry_point
    → entry FunctionId
```

## VE-007 — Invocation Values are initialization input only

Status: CLOSED

Los Invocation Values no permanecen como colección separada dentro de `VmExecution`.

Después de validación/materialización válida:

```text
Invocation Values
    ↓ materialize
RuntimeValue descriptors
    ↓
entry Parameter cells in SharedValueStorage
```

Por tanto no existe:

```text
VmExecution.invocation_values
```

como estado persistente v0.

La validación exacta de compatibilidad de Value shapes antes de comenzar una ejecución válida depende de metadata compilada suficiente y se analiza en `COMPILED_BOUNDARY_VALUE_SHAPE.md`.

## VE-008 — Entry CallFrame initialization

Status: CLOSED

Una vez materializados los entry Parameters y reservados sus locals, el primer frame se crea como:

```rust
CallFrame {
    function: compiled_program.entry_point,
    instruction_pointer: InstructionPointer(0),
    frame_base: 0,
}
```

Los `local_count` locals del entry se agregan como `None` después de sus Parameter cells conforme a `SHARED_VALUE_STORAGE.md`.

`operand_base` continúa derivándose y no se almacena.

## VE-009 — No persistent execution outcome/state flag

Status: CLOSED

`VmExecution` no almacena:

```text
Running / Completed / Failed enum
completed bool
Outcome
Failure
Diagnostic
result RuntimeValue
```

Mientras existe ejecución bytecode activa, `call_frames` contiene el stack de invocations.

Successful entry `Return` o failure concluyen la frontera de ejecución y producen el outcome correspondiente fuera del root mutable.

Un `RuntimeValue` final con handles execution-relative debe materializarse mientras sus owners runtime siguen vivos; su representación pública pertenece a Outcome / Diagnostic Data.

## VE-010 — No derived/cache fields in v0

Status: CLOSED

No se introducen por prevención:

```text
operand_base field
current frame index
current FunctionId
root InstructionPointer
return address
entry_point duplicate
resolved external binding cache
CompiledFunction reference
parameter_count copy
local_count copy
max_operand_depth copy
Invocation Values collection
```

Toda información derivable se obtiene desde las autoridades ya existentes.

## Exact Closed Shape

```rust
struct VmExecution<'compiled, 'bindings> {
    compiled_program: &'compiled CompiledProgram,
    application_bindings: &'bindings ApplicationBindings,
    value_storage: SharedValueStorage,
    backing_store: ExecutionBackingStore,
    call_frames: Vec<CallFrame>,
}
```

Relación:

```text
VmExecution
│
├── borrows
│   ├── CompiledProgram
│   └── ApplicationBindings
│
└── owns
    ├── SharedValueStorage
    ├── ExecutionBackingStore
    └── Vec<CallFrame>
```

## Boundary inconsistency discovered during closure

El cierre del root detectó que la estructura de `VmExecution` es suficiente, pero la metadata compilada vigente todavía no permite cumplir dos verificaciones de frontera ya requeridas:

```text
1. Execute Compiled invocation input
   → arity can be validated
   → exact Value shape currently cannot be validated from CompiledFunction

2. CallExternal success result
   → one OwnedValue is returned
   → exact expected Value shape currently cannot be validated from ExternalSymbol
```

El problema pertenece al persistent executable contract, no al mutable root de `VmExecution`.

Por tanto este documento NO reintroduce `TypeId`, `SemanticType` o reflection en la VM y NO agrega metadata al root.

La corrección mínima se analiza en:

```text
COMPILED_BOUNDARY_VALUE_SHAPE.md
```

## Closure

```text
VE-001 exact five-field VmExecution root                 ✅ CLOSED
VE-002 independent compiled/bindings lifetimes           ✅ CLOSED
VE-003 exactly three mutable runtime roots               ✅ CLOSED
VE-004 no persistent self-borrows                        ✅ CLOSED
VE-005 active frame = call_frames.last()                 ✅ CLOSED
VE-006 entry_point derived from CompiledProgram          ✅ CLOSED
VE-007 Invocation Values initialization-only             ✅ CLOSED
VE-008 entry frame = entry_point / ip0 / frame_base0     ✅ CLOSED
VE-009 no persistent execution state/outcome/result      ✅ CLOSED
VE-010 no derived/cache fields                           ✅ CLOSED

VmExecution exact Rust root                              ✅ CLOSED
Compiled Boundary Value Shape                            ← NEXT
VM Execution exact inventory                             PENDING
```