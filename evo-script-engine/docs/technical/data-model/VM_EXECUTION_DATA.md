# Evo-Script Engine — VM Execution Data

Status: VM EXECUTION DATA — IN ANALYSIS — EXACT INVENTORY NEXT

Este documento es la autoridad acumulada del estado runtime mutable utilizado para ejecutar un `CompiledProgram` en `evo-script-engine` v0.

```text
Compiled Program
    ↓ referenced by
VmExecution
    ↓
Outcome
```

Los documentos especializados contienen las reglas completas. Este root consolida su estado normativo y evita duplicar representación semántica dentro de la VM.

## VM-001 — VmExecution Root Responsibility

Status: CLOSED

`VmExecution` representa el estado mutable y aislado de exactamente una invocation de un `CompiledProgram`.

## VM-002 — CompiledProgram Relationship

Status: CLOSED

`VmExecution` referencia exactamente un `CompiledProgram`; no duplica Functions, Constant Pool, External Symbols, Value Shapes ni SourceMap.

## VM-003 — ApplicationBindings Relationship

Status: CLOSED

La ejecución referencia exactamente un `ApplicationBindings` explícito e inmutable durante toda la invocation.

No existe Current Provider, ambient provider lookup, Active Scope ni Host Session State.

## VM-004 — One SharedValueStorage

Status: CLOSED

```rust
struct SharedValueStorage {
    cells: Vec<Option<RuntimeValue>>,
}
```

Parameters, Locals y Operands son regiones lógicas del mismo storage físico.

## VM-005 — Call Frames

Status: CLOSED

```rust
struct InstructionPointer(usize);

struct CallFrame {
    function: FunctionId,
    instruction_pointer: InstructionPointer,
    frame_base: usize,
}
```

`VmExecution` owns ordered LIFO `Vec<CallFrame>`.

## VM-006 — Execution-Lifetime Backing Ownership

Status: CLOSED

```text
borrow mientras alcance
ownership cuando deba sobrevivir
```

Backing externo que debe sobrevivir a su materializador se transfiere/materializa bajo ownership de `VmExecution`.

## VM-007 — Invocation Lifetime

Status: CLOSED

```text
validate invocation boundary
    ↓
create/initialize VmExecution
    ↓
execute bytecode
    ↓
entry Return or Failure
    ↓
materialize Outcome
    ↓
VmExecution ends
```

## VM-008 — Explicitly Excluded from Root

```text
AST
SemanticProgram
TypeId
BindingId
Compiled data copies
entry_point duplicate
root InstructionPointer
current FunctionId duplicate
Active Scope
Host Session State
Current Provider
Outcome
Failure / Diagnostic
line / column
resolved binding cache
```

## VM-009 — Root Conceptual Shape

Status: CLOSED

```text
VmExecution
├── borrows exactly 1 CompiledProgram
├── borrows exactly 1 ApplicationBindings
├── owns exactly 1 SharedValueStorage
├── owns exactly 1 ExecutionBackingStore
└── owns ordered CallFrames
```

## VM-010 — RuntimeValue and evo_values::Value<'a> are distinct

Status: CLOSED

```text
RuntimeValue            = private internal VM descriptor
Value<'a>               = borrowed interchange representation
OwnedValue              = owned interchange representation
```

## VM-011 — RuntimeValue is immutable descriptor

Status: CLOSED

`RuntimeValue` no es automáticamente owner de backing variable/composite.

## VM-012 — Fixed Scalars Inline

Status: CLOSED

```text
Boolean
Int8 / Int16 / Int32 / Int64 / Int128
Uint8 / Uint16 / Uint32 / Uint64 / Uint128
Float32 / Float64
```

```text
int / int32     → Int32
float / float64 → Float64
```

## VM-013 — Variable / Composite Backing Indirection

Status: CLOSED

```text
String
Dynamic Integer
Struct
Enum
```

usan backing indirection.

## VM-014 — No Persistent Self-Borrow

Status: CLOSED

Shared storage no conserva direct Rust references hacia backing owned por la misma `VmExecution`.

## VM-015 — Backing Identity Strategy

Status: CLOSED

```rust
struct StringBackingId(usize);
struct DynamicIntegerBackingId(usize);
struct StructBackingId(usize);
struct EnumBackingId(usize);

enum StringBackingRef {
    Compiled(ConstantId),
    Execution(StringBackingId),
}

enum DynamicIntegerBackingRef {
    Compiled(ConstantId),
    Execution(DynamicIntegerBackingId),
}
```

No existe universal `RuntimeBackingId`.

## VM-016 — RuntimeValue Exact Representation

Status: CLOSED

`RuntimeValue` tiene exactamente 17 variants y `DynamicValue` exactamente 3.

```rust
#[derive(Clone, Copy)]
enum DynamicValue {
    Integer(DynamicIntegerBackingRef),
    Float32(f32),
    Float64(f64),
}
```

```rust
#[derive(Clone, Copy)]
enum RuntimeValue {
    Boolean(bool),
    Int8(i8), Int16(i16), Int32(i32), Int64(i64), Int128(i128),
    Uint8(u8), Uint16(u16), Uint32(u32), Uint64(u64), Uint128(u128),
    Float32(f32), Float64(f64),
    String(StringBackingRef),
    Dynamic(DynamicValue),
    Struct(StructBackingId),
    Enum(EnumBackingId),
}
```

Copiar descriptor no copia backing.

## VM-017 — Backing Data Representation

Status: CLOSED

```rust
struct ExecutionBackingStore {
    strings: Vec<Box<str>>,
    dynamic_integers: Vec<DynamicIntegerBacking>,
    structs: Vec<StructBacking>,
    enums: Vec<EnumBacking>,
}

struct StructBacking {
    fields: Box<[RuntimeValue]>,
}

struct EnumBacking {
    variant: VariantDiscriminant,
    payload: RuntimeEnumPayload,
}

enum RuntimeEnumPayload {
    Simple,
    Associated(RuntimeValue),
    Structured { fields: Box<[RuntimeValue]> },
}
```

Backings son typed, append-only, inmutables después de insertion; composites forman DAG finito.

## VM-018 — Shared Value Storage Exact Representation

Status: CLOSED

```text
Some(RuntimeValue) = occupied materialized cell
None               = reserved LocalSlot not yet materialized
```

```text
frame_base
    ↓
[parameters][locals][operands...]
                     ↑
                 operand_base
```

Internal `Call` reutiliza top-N argument cells como Parameter cells del callee. `Return` trunca a `callee.frame_base` y empuja un result descriptor para el caller.

## VM-019 — CallFrame Exact Representation

Status: CLOSED

```rust
struct CallFrame {
    function: FunctionId,
    instruction_pointer: InstructionPointer,
    frame_base: usize,
}
```

`operand_base` es derivado:

```text
frame_base + parameter_count + local_count
```

Internal caller permanece suspendido sobre `Call`; `CallExternal` no crea frame.

## VM-020 — InstructionPointer Stepping Semantics

Status: CLOSED

```text
ordinary success           → ip += 1
ordinary failure           → unchanged
Jump(target)               → ip = target
JumpIfFalse(false)         → ip = target
JumpIfFalse(true)          → ip += 1
internal Call success      → caller unchanged; callee ip = 0
internal Return success    → remove callee; caller ip += 1
entry Return success       → execution complete
CallExternal success       → ip += 1
CallExternal failure       → unchanged
```

Active IP siempre identifica una instruction válida y actualmente responsable.

## VM-021 — ApplicationBindings Exact Model

Status: CLOSED

```rust
struct ApplicationBindings {
    capabilities: HashMap<SignatureSymbol, ExternalCapability>,
}
```

Resolution en `CallExternal`:

```text
ExternalSymbolId
    → ExternalSymbol.symbol
    → SignatureSymbol
    → ApplicationBindings lookup
```

Missing binding falla lazy únicamente cuando se alcanza.

## VM-022 — ExternalCapability ABI Semantics

Status: CLOSED

```text
VM → capability
    borrowed &[Value<'a>]

capability → VM on success
    OwnedValue
```

Top-N argument cells permanecen hasta conocer success/failure.

Success materializa un valid `RuntimeValue` antes del commit `N → 1`.

Failure no hace commit y conserva IP sobre `CallExternal`; no garantiza rollback ni resumability.

Plain `fn` representa composición estática v0 y no captura state de instancia.

## VM-023 — evo-values Borrowed / Owned Interchange Model

Status: CLOSED

`Value<'a>` y `OwnedValue` cubren exactamente 17 semantic Value families.

Dynamic mantiene Integer / Float32 / Float64 y el Dynamic Integer cruza frontera como:

```text
negative + minimal unsigned big-endian magnitude
zero = empty magnitude + negative false
```

`evo-values` permanece `no_std + alloc` y sin dependency concreta de BigInt.

## VM-024 — VmExecution Exact Rust Root

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

Reglas:

```text
exactly 5 persistent fields
2 independent external borrow lifetimes
3 mutable runtime roots
active frame = call_frames.last()
entry_point derived
Invocation Values initialization-only
entry frame = entry_point / ip 0 / frame_base 0
no persistent execution status/outcome/result
no derived/cache fields
```

## VM-025 — Compiled Boundary Value Shape Contract

Status: CLOSED

Cerrado en `COMPILED_BOUNDARY_VALUE_SHAPE.md`.

`CompiledProgram` conserva boundary executable contract metadata:

```rust
struct CompiledValueShapeId(usize);

enum CompiledValueShape {
    Boolean,
    Int8, Int16, Int32, Int64, Int128,
    Uint8, Uint16, Uint32, Uint64, Uint128,
    Float32, Float64,
    String,
    Dynamic,
    Struct { fields: Vec<CompiledValueShapeId> },
    Enum { variants: Vec<CompiledEnumValueShape> },
}

enum CompiledEnumValueShape {
    Simple,
    Associated(CompiledValueShapeId),
    Structured { fields: Vec<CompiledValueShapeId> },
}
```

`CompiledProgram` agrega:

```text
entry_parameter_shapes: Vec<CompiledValueShapeId>
value_shapes: Vec<CompiledValueShape>
```

`ExternalSymbol` agrega:

```text
result_shape: CompiledValueShapeId
```

Uso exacto:

```text
Invocation Values
    → validate before valid VmExecution initialization

ExternalCapability Success(OwnedValue)
    → validate before RuntimeValue materialization / N→1 commit
```

Boundary validation es exacta, recursiva y sin coerción.

Esta metadata NO participa en ordinary bytecode dispatch y NO es runtime reflection general.

No modifica el shape de `VmExecution`.

## Current Authorities

- `RUNTIME_VALUE_MODEL.md`
- `BACKING_IDENTITY_STRATEGY.md`
- `RUNTIME_VALUE_REPRESENTATION.md`
- `BACKING_DATA_REPRESENTATION.md`
- `SHARED_VALUE_STORAGE.md`
- `CALL_FRAME.md`
- `INSTRUCTION_POINTER_STEPPING.md`
- `APPLICATION_BINDINGS.md`
- `EXTERNAL_CAPABILITY_ABI.md`
- `VM_EXECUTION_ROOT.md`
- `COMPILED_BOUNDARY_VALUE_SHAPE.md`
- `../../../../evo-values/INTERCHANGE_MODEL.md`

## Current Closure

```text
VmExecution root responsibility                   ✅ CLOSED
CompiledProgram / ApplicationBindings borrows     ✅ CLOSED
SharedValueStorage                                ✅ CLOSED
ExecutionBackingStore                             ✅ CLOSED
CallFrame                                         ✅ CLOSED
InstructionPointer                                ✅ CLOSED
RuntimeValue                                      ✅ CLOSED — 17 variants
DynamicValue                                      ✅ CLOSED — 3 variants
ApplicationBindings                               ✅ CLOSED
ExternalCapability ABI semantics                  ✅ CLOSED
Value<'a> / OwnedValue interchange                ✅ CLOSED
VmExecution exact Rust root                       ✅ CLOSED — 5 fields
Compiled Boundary Value Shape                     ✅ CLOSED
entry Invocation Value validation contract        ✅ CLOSED
external result validation contract               ✅ CLOSED

root InstructionPointer                           ❌ EXCLUDED
Current Provider / Active Scope / Host Session    ❌ EXCLUDED
Outcome / Diagnostic representation               ❌ SEPARATE PHASE

ExternalCapability failure type                   PENDING — Outcome / Diagnostic Data
VM Execution exact inventory                      ← NEXT
```
