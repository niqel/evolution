# Evo-Script Engine — Exact VM Execution Inventory

Status: CLOSED

Este documento cierra el inventario exacto de identities técnicas propias de `VM Execution Data` para `evo-script-engine` v0.

La auditoría consolida las autoridades especializadas de RuntimeValue, backing, shared storage, frames, stepping, application bindings, external ABI y `VmExecution` root.

La fase siguiente, `Outcome / Diagnostic Data`, definirá el tipo técnico de failure referenciado por `ExternalCapability`. Esa referencia cruzada no agrega una identity propia a VM ni reabre las reglas cerradas aquí.

## VMI-001 — Exactly 19 own VM identities

Status: CLOSED

`VM Execution Data` contiene exactamente **19 identities técnicas propias**.

```text
Root / external composition       3
Runtime Value descriptors         2
Backing identities / references   6
Execution backing data            5
Execution state / control         3
                                 ──
TOTAL                            19
```

## VMI-002 — Exact category inventory

Status: CLOSED

### Root / external composition — 3

```text
01 VmExecution
02 ApplicationBindings
03 ExternalCapability
```

### Runtime Value descriptors — 2

```text
04 RuntimeValue
05 DynamicValue
```

### Backing identities / references — 6

```text
06 StringBackingId
07 DynamicIntegerBackingId
08 StructBackingId
09 EnumBackingId
10 StringBackingRef
11 DynamicIntegerBackingRef
```

### Execution backing data — 5

```text
12 ExecutionBackingStore
13 DynamicIntegerBacking
14 StructBacking
15 EnumBacking
16 RuntimeEnumPayload
```

### Execution state / control — 3

```text
17 SharedValueStorage
18 InstructionPointer
19 CallFrame
```

## VMI-003 — ExternalCapability is a VM identity

Status: CLOSED

`ExternalCapability` se cuenta como identity propia de VM porque representa el ABI uniforme ejecutable almacenado por `ApplicationBindings`:

```rust
struct ApplicationBindings {
    capabilities: HashMap<SignatureSymbol, ExternalCapability>,
}
```

Su forma conceptual cerrada es:

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<
        OwnedValue,
        /* technical failure defined by Outcome / Diagnostic Data */,
    >;
```

El tipo de failure pertenece a la fase siguiente. Completarlo no crea una nueva identity VM.

## VMI-004 — Reused identities are not counted again

Status: CLOSED

VM reutiliza identities definidas por otras fases/crates y no las vuelve a contar:

```text
Semantic / Compiled Program
---------------------------
CompiledProgram
FunctionId
ConstantId
ExternalSymbolId
FieldIndex
VariantDiscriminant
SignatureSymbol
CompiledValueShapeId
CompiledValueShape
CompiledEnumValueShape

Shared evo-values
-----------------
Value<'a>
OwnedValue
```

Su uso por VM no cambia su owner semántico/técnico.

## VMI-005 — Containers, fields, primitives and lifetimes are not identities

Status: CLOSED

No se cuentan como identities independientes:

```text
Vec<T>
Box<T>
Option<T>
HashMap<K,V>
usize
f32
f64
'compiled
'bindings
'value
```

Tampoco fields o boundaries derivadas como:

```text
frame_base
operand_base
parameter_count
local_count
max_operand_depth
entry_point
current frame tail position
```

## VMI-006 — No wrapper identities without responsibility

Status: CLOSED

No se introducen en v0:

```text
OperandWindow
OperandStack
OperandSlot
FrameBase
FrameRegion
CallFrameId
InvocationId
CallStack
CurrentFrame
CurrentFunction
ExecutionState
RunningState
CompletedState
ResultSlot
RuntimeBackingId
RuntimeObjectId
RuntimeTypeId
ApplicationBinding
ResolvedExternalBindings
ProviderHandle
CurrentProvider
ExecutionContext
Session
```

Las responsabilidades correspondientes ya están expresadas por owners, typed IDs, collections y boundaries existentes.

## VMI-007 — Exact internal variant/field counts remain stable

Status: CLOSED

La auditoría confirma:

```text
RuntimeValue variants                 17
DynamicValue variants                  3
StringBackingRef variants              2
DynamicIntegerBackingRef variants      2
RuntimeEnumPayload variants            3

VmExecution persistent fields          5
CallFrame persistent fields             3
ExecutionBackingStore typed stores      4
```

Estos conteos pertenecen a VM y no se mezclan con los conteos del producto compilado.

Como referencia separada:

```text
Compiled Program identities            21
Instruction variants                   48
CompiledValueShape variants            17
CompiledEnumValueShape variants         3
```

## VMI-008 — Inventory closes before Outcome failure representation

Status: CLOSED

El inventario exacto de VM puede cerrarse antes de definir la representación técnica de `Failure`.

Relación de fases:

```text
VM Execution Data
    owns ExternalCapability identity

Outcome / Diagnostic Data
    owns technical Failure representation

ExternalCapability
    references that Failure type
```

Cuando Outcome cierre el tipo concreto de failure:

```text
ExternalCapability placeholder
    → exact Outcome-owned failure type
```

Esto:

```text
does not add a VM identity
does not change VmExecution
does not reopen ApplicationBindings
does not reopen ExternalCapability argument/result semantics
does not reopen RuntimeValue or storage mechanics
```

## Exact VM identity list

```text
01 VmExecution
02 ApplicationBindings
03 ExternalCapability
04 RuntimeValue
05 DynamicValue
06 StringBackingId
07 DynamicIntegerBackingId
08 StructBackingId
09 EnumBackingId
10 StringBackingRef
11 DynamicIntegerBackingRef
12 ExecutionBackingStore
13 DynamicIntegerBacking
14 StructBacking
15 EnumBacking
16 RuntimeEnumPayload
17 SharedValueStorage
18 InstructionPointer
19 CallFrame
```

## Root shape cross-check

```rust
struct VmExecution<'compiled, 'bindings> {
    compiled_program: &'compiled CompiledProgram,
    application_bindings: &'bindings ApplicationBindings,
    value_storage: SharedValueStorage,
    backing_store: ExecutionBackingStore,
    call_frames: Vec<CallFrame>,
}
```

No identity adicional se requiere para describir active frame, operand window, entry point, function metadata, result state o provider state.

## Closure

```text
VMI-001 exact 19 own VM identities                         ✅ CLOSED
VMI-002 exact category counts                              ✅ CLOSED
VMI-003 ExternalCapability counted as VM identity          ✅ CLOSED
VMI-004 reused cross-phase identities not recounted        ✅ CLOSED
VMI-005 containers/fields/primitives/lifetimes not counted ✅ CLOSED
VMI-006 no responsibility-free wrapper identities          ✅ CLOSED
VMI-007 exact internal variant/field counts                 ✅ CLOSED
VMI-008 Outcome failure completes ABI without new VM id     ✅ CLOSED

VM Execution exact inventory                               ✅ CLOSED — 19 identities
VM structural/data model                                   ✅ CLOSED
ExternalCapability failure type                            PENDING — Outcome / Diagnostic Data

NEXT
    Outcome / Diagnostic Data — after architecture map/review
```