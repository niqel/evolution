# Evo-Script Engine — VmExecution Exact Root

Status: CLOSED

Este documento cierra la representación Rust exacta del root mutable de una `VmExecution` en `evo-script-engine` v0.

La authority deriva de:

- `VM_EXECUTION_DATA.md`;
- `SHARED_VALUE_STORAGE.md`;
- `BACKING_DATA_REPRESENTATION.md`;
- `CALL_FRAME.md`;
- `APPLICATION_BINDINGS.md`;
- `EXTERNAL_CAPABILITY_ABI.md`;
- `COMPILED_BOUNDARY_VALUE_SHAPE.md`;
- `evo-values/INTERCHANGE_MODEL.md`.

## VE-001 — Exact five-field root

Status: CLOSED

```rust
struct VmExecution<'compiled, 'bindings> {
    compiled_program: &'compiled CompiledProgram,
    application_bindings: &'bindings ApplicationBindings,
    value_storage: SharedValueStorage,
    backing_store: ExecutionBackingStore,
    call_frames: Vec<CallFrame>,
}
```

Exactamente cinco fields persistentes.

## VE-002 — Independent external borrow lifetimes

Status: CLOSED

```text
'compiled → CompiledProgram must outlive VmExecution
'bindings → ApplicationBindings must outlive VmExecution
```

No se acoplan artificialmente en un único lifetime.

## VE-003 — Exactly three mutable runtime roots

Status: CLOSED

```text
SharedValueStorage
ExecutionBackingStore
Vec<CallFrame>
```

Los otros dos fields son borrows inmutables.

## VE-004 — No persistent self-borrows

Status: CLOSED

Las relaciones internas utilizan typed IDs, `FunctionId`, `InstructionPointer`, `frame_base` y positional boundaries.

No se almacenan referencias persistentes hacia datos owned por el mismo root.

## VE-005 — Active frame is the LIFO tail

Status: CLOSED

```text
active frame = call_frames.last()
```

No existen `current_frame`, `current_frame_index`, `current_function` ni root `InstructionPointer`.

## VE-006 — entry_point is derived

Status: CLOSED

```text
VmExecution.compiled_program.entry_point
```

No se duplica dentro del root.

## VE-007 — Invocation Values are initialization input only

Status: CLOSED

Después de validación y materialización:

```text
Invocation Values
    ↓
entry Parameter cells in SharedValueStorage
```

No existe `VmExecution.invocation_values`.

La validación exacta previa se encuentra cerrada en `COMPILED_BOUNDARY_VALUE_SHAPE.md` mediante `CompiledProgram.entry_parameter_shapes`.

## VE-008 — Entry CallFrame initialization

Status: CLOSED

```rust
CallFrame {
    function: compiled_program.entry_point,
    instruction_pointer: InstructionPointer(0),
    frame_base: 0,
}
```

Los `local_count` locals se agregan como `None` después de los Parameters. `operand_base` se deriva.

## VE-009 — No persistent execution outcome/state flag

Status: CLOSED

No se almacenan:

```text
Running / Completed / Failed
completed bool
Outcome
Failure
Diagnostic
result RuntimeValue
```

Successful entry `Return` o Failure concluyen la frontera de ejecución.

## VE-010 — No derived/cache fields in v0

Status: CLOSED

No se introducen:

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

## Compiled boundary validation — CLOSED externally to root

El problema detectado durante el cierre del root fue resuelto sin cambiar `VmExecution`.

`CompiledProgram` ahora conserva:

```text
entry_parameter_shapes: Vec<CompiledValueShapeId>
value_shapes: Vec<CompiledValueShape>
```

`ExternalSymbol` conserva:

```text
result_shape: CompiledValueShapeId
```

Por tanto:

```text
Invocation Values
    → exact validation before valid VmExecution initialization

ExternalCapability Success(OwnedValue)
    → exact validation before runtime materialization / stack commit
```

No se reintroduce `TypeId`, `SemanticType` ni reflection en `VmExecution`.

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
Compiled Boundary Value Shape                            ✅ CLOSED

VmExecution exact Rust root                              ✅ CLOSED
VM Execution exact inventory                             ← NEXT
```