# Evo-Script Engine — CallFrame

Status: CLOSED

Este documento cierra la representación v0 de `CallFrame` para invocations internas activas o suspendidas dentro de una `VmExecution`.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`, especialmente TD-006 y TD-007;
- `COMPILED_CORE_CALL_INSTRUCTIONS.md`;
- `COMPILED_CONTROL_FLOW.md`;
- `COMPILED_PROGRAM_DATA.md`;
- `SHARED_VALUE_STORAGE.md`;
- `VM_EXECUTION_DATA.md`.

Este bloque define exactamente qué estado pertenece a cada invocation interna. No introduce storage per-frame y no duplica metadata ya owned por `CompiledFunction`.

## CF-001 — One CallFrame per active/suspended internal invocation

Status: CLOSED

`CallFrame` representa exactamente una invocation interna activa o suspendida de una `CompiledFunction`.

```text
FunctionId
    = identity of compiled function

CallFrame
    = runtime state of one invocation of that function
```

Recursion puede producir múltiples `CallFrame` con el mismo `FunctionId` y diferentes execution positions / frame bases.

No se introduce `CallFrameId` o `InvocationId` en v0; la colección LIFO de frames ya expresa el orden runtime.

## CF-002 — Exact CallFrame representation

Status: CLOSED

La representación exacta v0 es:

```rust
struct InstructionPointer(usize);

struct CallFrame {
    function: FunctionId,
    instruction_pointer: InstructionPointer,
    frame_base: usize,
}
```

El frame contiene exactamente tres datos persistentes durante su lifetime runtime.

## CF-003 — FunctionId resolves CompiledFunction

Status: CLOSED

```text
CallFrame.function
    → CompiledProgram.functions[FunctionId]
```

Desde `CompiledFunction` se obtienen:

```text
instructions
parameter_count
local_count
max_operand_depth
```

No se duplican dentro de `CallFrame`:

```text
parameter_count
local_count
max_operand_depth
CompiledFunction reference
function name
TypeId metadata
```

`FunctionId` es suficiente como identity técnica y evita introducir un borrow adicional hacia `CompiledFunction` dentro de cada frame.

## CF-004 — InstructionPointer is mutable runtime state distinct from InstructionIndex

Status: CLOSED

```rust
struct InstructionPointer(usize);
```

Separación canónica:

```text
InstructionIndex
    = persistent bytecode position
    = branch target stored in CompiledProgram

InstructionPointer
    = mutable execution position of one CallFrame
```

Aunque ambas identities puedan usar `usize`, no son intercambiables por responsabilidad.

`InstructionPointer` no se almacena en `VmExecution` root; pertenece al `CallFrame` activo o suspendido correspondiente.

## CF-005 — InstructionPointer identifies the current responsible instruction

Status: CLOSED

`instruction_pointer` identifica la instruction actualmente responsable de la posición de ejecución del frame, no anticipadamente la siguiente instruction.

Regla:

> Mientras una instruction no haya terminado exitosamente, el `InstructionPointer` del frame continúa identificando esa instruction.

Esto permite que una failure pueda localizarse usando:

```text
CallFrame.function
+ CallFrame.instruction_pointer
    ↓ runtime-to-persistent position
SourceMap
```

La política exacta de avance se cierra en `INSTRUCTION_POINTER_STEPPING.md` mediante IP-001..IP-010.

## CF-006 — frame_base is the absolute beginning of the frame region

Status: CLOSED

```rust
frame_base: usize
```

`frame_base` identifica la primera cell física de la invocation dentro de `SharedValueStorage`.

```text
frame_base
    ↓
[parameters][locals][operands...]
```

Se utiliza para:

```text
ParameterSlot absolute position
LocalSlot absolute position
frame truncation during Return
callee boundary establishment
```

No se introduce `FrameBase` newtype en v0; `frame_base` es un boundary absoluto privado de la ejecución y no una identity persistente entre artifacts.

## CF-007 — operand_base is derived and not stored

Status: CLOSED

`operand_base` no forma parte de `CallFrame`.

Se deriva exactamente como:

```text
operand_base
    = frame_base
    + compiled_function.parameter_count
    + compiled_function.local_count
```

Guardar también `operand_base` duplicaría información y permitiría inconsistencia entre el valor derivado y uno almacenado.

Por tanto no existe:

```rust
CallFrame {
    operand_base: usize,
    ...
}
```

como representación v0.

## CF-008 — Internal Call suspends caller on Call and starts callee at 0

Status: CLOSED

Para:

```rust
Instruction::Call(FunctionId)
```

el caller permanece suspendido con su `InstructionPointer` apuntando a la propia `Call` mientras el callee está activo.

```text
caller
    ip → Call(target)

callee
    function = target
    ip = InstructionPointer(0)
```

El `SharedValueStorage` ya cerró que los `N` argument cells superiores del caller se reutilizan directamente como Parameter cells del callee.

Cuando el callee retorna exitosamente:

```text
callee removed
caller resumed
caller InstructionPointer advances exactly one instruction
```

Conservar al caller sobre la `Call` mientras el callee vive también preserva naturalmente el call-site técnico para futuras diagnostics sin field adicional.

## CF-009 — No return address / parent frame / call-site field

Status: CLOSED

No se almacenan:

```text
return_address
parent_frame
caller_index
call_site
SourceSpan
```

La colección LIFO de `CallFrame` + el suspended caller `InstructionPointer` contienen toda la información requerida.

```text
caller frame
    ip → Call

callee frame
    active
```

Successful `Return` elimina el callee y avanza el caller desde la `Call` hacia la instruction siguiente.

No se necesita una return address duplicada.

## CF-010 — CallExternal does not create CallFrame

Status: CLOSED

```rust
Instruction::CallExternal(ExternalSymbolId)
```

no crea un nuevo `CallFrame` porque no entra a bytecode de otra `CompiledFunction`.

```text
CallExternal
    ↓
ApplicationBindings
    ↓
external capability
```

La colección de `CallFrame` permanece sin cambios.

Mientras la external call está en progreso o falla, el active `InstructionPointer` continúa apuntando a esa `CallExternal` instruction.

Solo:

```rust
Call(FunctionId)
```

crea un nuevo `CallFrame` en v0.

## Exact Closed Shape

```rust
struct InstructionPointer(usize);

struct CallFrame {
    function: FunctionId,
    instruction_pointer: InstructionPointer,
    frame_base: usize,
}
```

Derivaciones:

```text
compiled_function
    = CompiledProgram.functions[frame.function]

operand_base
    = frame.frame_base
    + compiled_function.parameter_count
    + compiled_function.local_count

active operand depth
    = SharedValueStorage.cells.len() - operand_base
```

## Explicitly Not Introduced

```text
operand_base field
parameter_count field
local_count field
max_operand_depth field
return_address
parent frame identity
caller index
call-site field
CallFrame-owned Parameter/Local/Operand containers
CompiledFunction reference
SourceSpan in CallFrame
TypeId / semantic metadata
CallFrameId / InvocationId
```

## Closure

```text
CF-001 one frame per internal invocation                 ✅ CLOSED
CF-002 exact 3-field CallFrame                            ✅ CLOSED
CF-003 FunctionId resolves CompiledFunction              ✅ CLOSED
CF-004 InstructionPointer distinct runtime identity      ✅ CLOSED
CF-005 IP identifies current responsible instruction     ✅ CLOSED
CF-006 frame_base absolute frame boundary                ✅ CLOSED
CF-007 operand_base derived / not stored                  ✅ CLOSED
CF-008 caller suspended on Call; callee starts at 0       ✅ CLOSED
CF-009 no return-address / parent / call-site fields      ✅ CLOSED
CF-010 CallExternal creates no CallFrame                  ✅ CLOSED

CallFrame exact representation                            ✅ CLOSED
InstructionPointer identity/current-position semantics    ✅ CLOSED
InstructionPointer stepping semantics                     ✅ CLOSED in INSTRUCTION_POINTER_STEPPING.md

ApplicationBindings exact model                           ← NEXT
remaining external call/value mechanics                   PENDING
```