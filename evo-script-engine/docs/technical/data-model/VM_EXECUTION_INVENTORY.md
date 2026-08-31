# Evo-Script Engine — Exact VM Execution Inventory

Status: CLOSED / REVALIDATED AFTER OUTCOME CLOSURE

Este documento cierra el inventario exacto de identities técnicas propias de `VM Execution Data` para `evo-script-engine` v0.

## VMI-001 — Exactly 19 own VM identities

Status: CLOSED

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

Status: CLOSED / REVALIDATED

`ExternalCapability` se cuenta como identity propia de VM porque representa el ABI uniforme almacenado por `ApplicationBindings`.

La firma exacta, ya completada por Outcome / Diagnostic Data, es:

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

`ExternalCapabilityFailure` pertenece a Outcome / Diagnostic Data y no se cuenta nuevamente dentro de VM.

Completar esa referencia cruzada no cambió el inventario VM de 19 identities.

## VMI-004 — Reused identities are not counted again

Status: CLOSED

VM reutiliza, sin volver a contar:

```text
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
Value<'a>
OwnedValue
ExternalCapabilityFailure
```

## VMI-005 — Containers, fields, primitives and lifetimes are not identities

Status: CLOSED

No se cuentan:

```text
Vec<T>
Box<T>
Option<T>
HashMap<K,V>
Result<T,E>
usize
f32
f64
lifetimes
fields / numeric boundaries
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

## VMI-007 — Exact internal variant / field counts

Status: CLOSED

```text
RuntimeValue variants                 17
DynamicValue variants                  3
StringBackingRef variants              2
DynamicIntegerBackingRef variants      2
RuntimeEnumPayload variants            3
VmExecution persistent fields          5
CallFrame persistent fields            3
ExecutionBackingStore typed stores     4
```

Reference counts from other phases remain separate:

```text
Compiled Program identities            21
Instruction variants                   48
CompiledValueShape variants            17
CompiledEnumValueShape variants         3
Outcome / Diagnostic identities        24
```

## VMI-008 — Cross-phase failure type does not alter VM inventory

Status: CLOSED / REVALIDATED

La relación final es:

```text
VM Execution Data
    owns ExternalCapability identity

Outcome / Diagnostic Data
    owns ExternalCapabilityFailure identity

ExternalCapability
    references ExternalCapabilityFailure
```

Esto no agrega una identity VM, no cambia `VmExecution`, no cambia `ApplicationBindings` y no reabre RuntimeValue/storage/stepping semantics.

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

## Closure

```text
VMI-001 exact 19 own VM identities                         ✅ CLOSED
VMI-002 exact category counts                              ✅ CLOSED
VMI-003 ExternalCapability counted as VM identity          ✅ CLOSED / REVALIDATED
VMI-004 reused cross-phase identities not recounted        ✅ CLOSED
VMI-005 containers/fields/primitives/lifetimes not counted ✅ CLOSED
VMI-006 no responsibility-free wrapper identities          ✅ CLOSED
VMI-007 exact internal variant/field counts                 ✅ CLOSED
VMI-008 Outcome failure reference does not alter VM         ✅ CLOSED / REVALIDATED

VM Execution exact inventory                               ✅ CLOSED — 19 identities
VM Execution Data                                          ✅ CLOSED
Outcome / Diagnostic Data                                  ✅ CLOSED — 24 identities
Technical Data Model                                       ✅ CLOSED

NEXT
    Technical Data Diagram
```