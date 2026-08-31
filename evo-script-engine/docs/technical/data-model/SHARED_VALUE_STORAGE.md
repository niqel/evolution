# Evo-Script Engine — Shared Value Storage

Status: CLOSED

Este documento cierra la representación v0 del storage compartido de `RuntimeValue` utilizado por Parameters, Locals y Operand Window durante una `VmExecution`.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`, especialmente TD-006 y TD-007;
- `COMPILED_STORAGE_DATA.md`;
- `COMPILED_PROGRAM_DATA.md`;
- `RUNTIME_VALUE_REPRESENTATION.md`;
- `BACKING_DATA_REPRESENTATION.md`;
- `VM_EXECUTION_DATA.md`.

Este bloque define el container físico compartido y sus invariantes de ocupación. No cierra todavía la representación exacta de `CallFrame` ni de `InstructionPointer`, aunque establece las fronteras que ambos deberán respetar.

## SV-001 — One SharedValueStorage per VmExecution

Status: CLOSED

`VmExecution` posee exactamente un storage físico compartido para Parameters, Locals y Operands:

```rust
struct SharedValueStorage {
    cells: Vec<Option<RuntimeValue>>,
}
```

No existen owners separados como:

```text
ParameterStorage
LocalStorage
OperandStack
per-frame Value container
```

Los distintos usos son regiones lógicas dentro de la misma secuencia física.

## SV-002 — Physical representation is Vec<Option<RuntimeValue>>

Status: CLOSED

La representación base v0 es:

```rust
struct SharedValueStorage {
    cells: Vec<Option<RuntimeValue>>,
}
```

`Vec` expresa una secuencia contigua que puede crecer, hacer pop y truncarse siguiendo el lifetime de los frames y operands.

Este container es distinto de `ExecutionBackingStore`:

```text
SharedValueStorage
    → temporal execution state
    → push / pop / truncate

ExecutionBackingStore
    → execution-owned backing
    → append-only in v0
```

## SV-003 — Meaning of Some / None

Status: CLOSED

```text
Some(RuntimeValue)
    = occupied materialized Value cell

None
    = reserved stable LocalSlot whose Value has not yet been materialized
```

`None` es exclusivamente estado técnico de VM storage.

No representa:

```text
Evo-Script null
optional language Value
missing argument
operand placeholder
uninitialized parameter
```

Evo-Script v0 continúa sin introducir `null` como Value del lenguaje.

## SV-004 — Stable frame region is [parameters][locals]

Status: CLOSED

Para un frame activo, la región estable es contigua:

```text
frame_base
    ↓
[parameters][locals][operands...]
                     ↑
                 operand_base
```

Con:

```text
parameter absolute position
    = frame_base + ParameterSlot

local absolute position
    = frame_base + parameter_count + LocalSlot

operand_base
    = frame_base + parameter_count + local_count
```

Invariantes de ocupación:

```text
Parameter cells
    → always Some(RuntimeValue)

Local cells at frame creation
    → None

Local cell after StoreLocal
    → Some(RuntimeValue)
    → materialized exactly once
```

`StoreLocal` representa initial materialization, no mutación semántica.

Por tanto un `StoreLocal(slot)` válido requiere que la target cell todavía sea `None`.

Un `LoadLocal(slot)` válido requiere `Some(RuntimeValue)`.

Encontrar un estado contrario indica violación de invariantes del compiler/VM, no un `EvaluationError` normal de Evo-Script.

## SV-005 — Operand Window is the active tail

Status: CLOSED

El Operand Window del frame activo es exactamente:

```text
cells[operand_base .. cells.len()]
```

Toda cell dentro del Operand Window es:

```text
Some(RuntimeValue)
```

`None` nunca es un operand válido.

El top físico del Operand Window coincide con el tail de `cells`.

No se almacena un operand top duplicado dentro de `SharedValueStorage`.

## SV-006 — Operand push/pop use the Vec tail

Status: CLOSED

Operand operations actúan sobre el tail físico:

```text
push value
    → cells.push(Some(value))

pop value
    → cells.pop()
    → must yield Some(RuntimeValue)
```

No se introducen identities persistentes:

```text
OperandSlot
OperandIndex
OperandId
```

Los operands son Values temporales LIFO y no necesitan identity individual en v0.

## SV-007 — Operand depth and max_operand_depth

Status: CLOSED

Para el frame activo:

```text
operand_depth
    = cells.len() - operand_base
```

La invariante ejecutable es:

```text
operand_depth
    <= CompiledFunction.max_operand_depth
```

`max_operand_depth` es bound compilado y puede utilizarse para reservar capacidad cuando convenga.

No significa que el frame agregue `max_operand_depth` cells `None` al crearse.

Los operands existen únicamente cuando han sido materializados.

## SV-008 — Call reuses caller argument cells as callee Parameters

Status: CLOSED

Para una internal `Call(FunctionId)` con `N` Value Parameters, los `N` operands superiores del caller se convierten directamente en la región Parameter del callee.

Conceptualmente, antes del call:

```text
caller stable region
[caller operands ...][arg0][arg1]...[argN-1]
                     ↑
                future callee frame_base
```

La frontera del nuevo frame es:

```text
callee.frame_base
    = cells.len() - N
```

Los argumentos no se copian hacia un `Vec` o container per-frame separado.

La misma cells físicas pasan de responsabilidad lógica:

```text
caller Operand Window
        ↓ Call
callee Parameter Slots
```

Esto preserva TD-006 / TD-007 y evita almacenamiento redundante de argumentos.

## SV-009 — Callee locals are appended after Parameters

Status: CLOSED

Después de fijar el `frame_base` del callee, se agregan exactamente `local_count` cells `None`:

```text
callee.frame_base
    ↓
[parameter 0]
[parameter 1]
...
[parameter N-1]
[None] local 0
[None] local 1
...
[None] local M-1
       ↑
   operand_base
```

Por tanto:

```text
callee.operand_base
    = callee.frame_base
    + parameter_count
    + local_count
```

No se agregan operand placeholders durante frame creation.

## SV-010 — Return storage transformation

Status: CLOSED

Este bloque cierra la transformación del storage requerida por `Return`, sin cerrar todavía toda la representación de `CallFrame`.

Para un callee que retorna exactamente un `RuntimeValue`:

```text
1. obtain/copy result descriptor from callee Operand Window
2. truncate SharedValueStorage to callee.frame_base
3. remove callee CallFrame
4. push Some(result) for the resumed caller
```

Conceptualmente:

```text
before Return

caller retained region
[callee parameters][callee locals][callee temporaries][result]
 ↑ callee.frame_base

        ↓

after Return

caller retained region[result]
```

Copiar el descriptor no copia String, Dynamic Integer, Struct o Enum backing porque `RuntimeValue` es `Copy` y los backings poseen identity estable.

Para el entry frame, la materialización del resultado exterior se completará en `Outcome / Diagnostic Data`; un RuntimeValue con execution-relative handles no escapa autónomamente de `VmExecution`.

## SV-011 — CallFrame describes boundaries; it does not own Value containers

Status: CLOSED

`CallFrame` delimita una región dentro de `SharedValueStorage`; no posee Parameters, Locals u Operands como collections independientes.

Relación obligatoria:

```text
VmExecution
├── owns SharedValueStorage
│   └── owns Value cells
└── owns CallFrames
    └── describe frame boundaries into SharedValueStorage
```

El bloque posterior de `CallFrame` debe conservar esta separación.

No se introduce:

```text
CallFrame.parameters: Vec<RuntimeValue>
CallFrame.locals: Vec<...>
CallFrame.operands: Vec<RuntimeValue>
FrameRegion owned container
```

## Closed Physical Shape

```rust
struct SharedValueStorage {
    cells: Vec<Option<RuntimeValue>>,
}
```

Regiones lógicas:

```text
SharedValueStorage

Frame A
┌────────────────────────────────────┐
│ parameters │ locals │ operands     │
└────────────────────────────────────┘
                         │
                         │ top arguments become
                         ▼
                      Frame B
                      ┌──────────────────────────┐
                      │ parameters │ locals │ op │
                      └──────────────────────────┘
```

Ocupación válida:

```text
parameter     Some(RuntimeValue)
local         None → Some(RuntimeValue) exactly once
operand       Some(RuntimeValue)
```

## Explicitly Not Introduced

```text
separate ParameterStorage
separate LocalStorage
separate OperandStack owner
per-frame Value Vec
OperandSlot / OperandIndex / OperandId
Evo-Script null through Option::None
MaybeUninit as architectural requirement
operand placeholder cells up to max_operand_depth
CallFrame-owned Value containers
```

`MaybeUninit` u otras optimizaciones low-level solo podrían reemplazar internamente esta representación si preservan exactamente las mismas invariantes y existe evidencia de rendimiento que lo justifique; no forman parte del modelo v0 cerrado.

## Closure

```text
SV-001 one SharedValueStorage per VmExecution             ✅ CLOSED
SV-002 Vec<Option<RuntimeValue>> representation           ✅ CLOSED
SV-003 Some / None storage semantics                      ✅ CLOSED
SV-004 contiguous Parameters + Locals stable region       ✅ CLOSED
SV-005 Operand Window = active tail                       ✅ CLOSED
SV-006 operand push/pop on Vec tail; no Operand identity  ✅ CLOSED
SV-007 operand depth / max_operand_depth bound            ✅ CLOSED
SV-008 call reuses argument cells as callee Parameters    ✅ CLOSED
SV-009 callee locals appended as None                     ✅ CLOSED
SV-010 Return truncates to frame_base then pushes result  ✅ CLOSED
SV-011 CallFrame describes boundaries, owns no Values     ✅ CLOSED

Shared Value Storage exact representation                 ✅ CLOSED

CallFrame exact representation                            ← NEXT
InstructionPointer                                        PENDING
ApplicationBindings exact model                           PENDING
remaining call / return frame mechanics                   PENDING
```
