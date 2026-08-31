# Evo-Script Engine — InstructionPointer Stepping

Status: CLOSED

Este documento cierra la semántica v0 de avance de `InstructionPointer` durante ejecución de bytecode.

La autoridad deriva de:

- `CALL_FRAME.md`;
- `COMPILED_CONTROL_FLOW.md`;
- `COMPILED_CORE_CALL_INSTRUCTIONS.md`;
- `COMPILED_PROGRAM_INVENTORY.md`;
- `SHARED_VALUE_STORAGE.md`.

`InstructionPointer` ya fue cerrado como estado mutable de un `CallFrame` que identifica la instruction actualmente responsable. Este bloque define exactamente cuándo y cómo cambia.

## IP-001 — Active IP always identifies a valid instruction

Status: CLOSED

Mientras exista un `CallFrame` activo:

```text
0 <= instruction_pointer < compiled_function.instructions.len()
```

No existe un estado normal `InstructionPointer(instructions.len())` para representar fin de función.

Una función válida termina mediante `Return`; no por fall-through fuera del vector de instructions.

No se introducen `End`, `Halt`, `PastEnd` ni `InstructionPointer::Finished` en v0.

## IP-002 — New frame starts at zero

Status: CLOSED

Todo nuevo `CallFrame` comienza en:

```rust
InstructionPointer(0)
```

Esto aplica tanto al entry frame como a cada frame creado por `Call(FunctionId)`.

## IP-003 — IP commits only after successful instruction completion

Status: CLOSED

Regla canónica:

> Mientras la instruction actual no haya terminado exitosamente, el `InstructionPointer` continúa identificando esa instruction.

```text
execute current instruction
        |
        +-- success -> apply its IP transition
        |
        +-- failure -> IP unchanged
```

Esto preserva la ubicación técnica responsable para `SourceMap` y futuras diagnostics.

## IP-004 — Failure position does not imply rollback or resumability

Status: CLOSED

Que el IP permanezca en la instruction responsable después de una failure no convierte la VM en transaccional ni reanudable.

No se garantiza:

```text
rollback de effects
re-execution safe
resume after failure
external capability rollback
```

La regla preserva posición diagnóstica, no semántica de retry.

## IP-005 — Sequential success advances exactly one instruction

Status: CLOSED

Toda instruction exitosa que no reemplace explícitamente el control flow avanza:

```text
ip := ip + 1
```

Esto cubre las familias ordinarias de movement, numeric, conversions, scalar/composite mechanics, equality, `Discard` y `CallExternal` exitoso.

`CallExternal` no crea `CallFrame`; por tanto éxito significa result materializado en el frame actual seguido de `ip += 1`.

## IP-006 — Jump replaces the current IP with target

Status: CLOSED

Para:

```rust
Jump(InstructionIndex(target))
```

la transición exitosa es:

```text
ip := target
```

No se aplica incremento adicional.

`InstructionIndex` sigue siendo la identity persistente de bytecode; `InstructionPointer` sigue siendo estado mutable runtime aunque ambos compartan ordinal `usize`.

## IP-007 — JumpIfFalse transition

Status: CLOSED

`JumpIfFalse(target)` consume exactamente un bool.

```text
condition == false
    -> ip := target

condition == true
    -> ip := ip + 1
```

No existe `target + 1` implícito.

## IP-008 — Internal Call preserves caller IP and starts callee at zero

Status: CLOSED

Para `Call(FunctionId)` exitoso:

```text
caller.ip remains on Call(target)
callee.ip = InstructionPointer(0)
```

El caller queda suspendido sobre el call-site técnico mientras el callee vive.

No se avanza el caller antes de crear el callee y no se almacena `return_address`.

## IP-009 — Return transition

Status: CLOSED

### Internal callee Return

Un `Return` exitoso de un frame con caller:

```text
1. copy result RuntimeValue descriptor
2. truncate SharedValueStorage to callee.frame_base
3. remove callee CallFrame
4. push result for resumed caller
5. caller.ip := caller.ip + 1
```

El frame que ejecutó `Return` deja de existir y por tanto no posee next IP.

### Entry Return

Un `Return` exitoso del entry frame completa la ejecución.

No existe caller y no se avanza el entry IP fuera del vector.

La materialización del resultado exterior pertenece a `Outcome / Diagnostic Data`.

## IP-010 — No alternate IP state identities

Status: CLOSED

No se introducen en v0:

```text
NextInstructionPointer
next_ip
return_ip
root InstructionPointer
past-end sentinel
finished InstructionPointer variant
```

Cada `CallFrame` posee una única execution position: `instruction_pointer`.

## Closed transition table

| Instruction outcome | IP transition |
|---|---|
| ordinary success | `ip += 1` |
| ordinary failure | unchanged |
| `Jump(target)` | `ip = target` |
| `JumpIfFalse(false)` | `ip = target` |
| `JumpIfFalse(true)` | `ip += 1` |
| internal `Call` success | caller unchanged; callee `ip = 0` |
| internal `Return` success | pop callee; caller `ip += 1` |
| entry `Return` success | execution complete |
| `CallExternal` success | `ip += 1` |
| `CallExternal` failure | unchanged |

## Closure

```text
IP-001 active IP always valid                         ✅ CLOSED
IP-002 new frame starts at 0                          ✅ CLOSED
IP-003 IP transition commits only after success       ✅ CLOSED
IP-004 failure position != rollback/resumability      ✅ CLOSED
IP-005 sequential success advances by one             ✅ CLOSED
IP-006 Jump replaces IP with target                    ✅ CLOSED
IP-007 JumpIfFalse exact transition                    ✅ CLOSED
IP-008 internal Call preserves caller IP               ✅ CLOSED
IP-009 Return exact transition                         ✅ CLOSED
IP-010 no alternate/past-end/root IP state             ✅ CLOSED

InstructionPointer identity                            ✅ CLOSED
InstructionPointer current-instruction semantics       ✅ CLOSED
InstructionPointer stepping semantics                  ✅ CLOSED

ApplicationBindings exact model                        ← NEXT
remaining external call/value mechanics                PENDING
VmExecution exact Rust root                            PENDING
VM Execution exact inventory                           PENDING
```