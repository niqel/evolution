# Evo-Script Engine — VM Execution Data

Status: VM EXECUTION DATA — STRUCTURALLY CLOSED — OUTCOME FAILURE TYPE REFERENCE PENDING

Este documento es la autoridad acumulada del estado runtime mutable utilizado para ejecutar un `CompiledProgram` en `evo-script-engine` v0.

```text
Compiled Program
    ↓ referenced by
VmExecution
    ↓
Outcome / Diagnostic Data
```

Las responsabilidades, representaciones e inventario propio de VM quedan cerrados. La fase `Outcome / Diagnostic Data` debe todavía definir el tipo técnico de failure referenciado por `ExternalCapability`; esa dependencia cruzada no agrega una identity VM ni reabre las reglas cerradas aquí.

## VM-001 — VmExecution Root Responsibility

Status: CLOSED

`VmExecution` representa el estado mutable y aislado de exactamente una invocation de un `CompiledProgram`.

## VM-002 — CompiledProgram Relationship

Status: CLOSED

`VmExecution` referencia exactamente un `CompiledProgram`; no duplica Functions, Constant Pool, External Symbols, Compiled Value Shapes ni SourceMap.

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

## VM-010 — RuntimeValue / interchange boundary

Status: CLOSED

```text
RuntimeValue = private internal VM descriptor
Value<'a>    = borrowed interchange representation
OwnedValue   = owned interchange representation
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

`RuntimeValue` posee exactamente 17 variants y `DynamicValue` exactamente 3.

```rust
#[derive(Clone, Copy)]
enum DynamicValue {
    Integer(DynamicIntegerBackingRef),
    Float32(f32),
    Float64(f64),
}

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

Backings son typed, append-only e inmutables después de insertion; composites forman DAG finito.

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

Internal `Call` reutiliza top-N argument cells como Parameters del callee. `Return` trunca a `callee.frame_base` y empuja un result descriptor para el caller.

## VM-019 — CallFrame Exact Representation

Status: CLOSED

```rust
struct CallFrame {
    function: FunctionId,
    instruction_pointer: InstructionPointer,
    frame_base: usize,
}
```

`operand_base = frame_base + parameter_count + local_count` y no se almacena.

## VM-020 — InstructionPointer Stepping Semantics

Status: CLOSED

```text
ordinary success           → ip += 1
ordinary failure           → unchanged
Jump(target)               → ip = target
JumpIfFalse(false)         → ip = target
JumpIfFalse(true)          → ip += 1
internal Call              → caller unchanged; callee ip = 0
internal Return            → remove callee; caller ip += 1
entry Return               → execution complete
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

Top-N argument cells permanecen hasta conocer success/failure. Success valida/materializa un `RuntimeValue` antes del commit `N → 1`. Failure no hace commit y conserva IP sobre `CallExternal`; no garantiza rollback ni resumability.

Plain `fn` representa composición estática v0 y no captura state de instancia.

## VM-023 — evo-values Borrowed / Owned Interchange Model

Status: CLOSED

`Value<'a>` y `OwnedValue` cubren exactamente 17 semantic Value families.

Dynamic mantiene Integer / Float32 / Float64 y Dynamic Integer cruza frontera como:

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

`CompiledProgram` conserva boundary executable contract metadata mediante:

```text
CompiledValueShapeId
CompiledValueShape — 17 variants
CompiledEnumValueShape — 3 variants
entry_parameter_shapes
ExternalSymbol.result_shape
```

Uso exacto:

```text
Invocation Values
    → validate before valid VmExecution initialization

ExternalCapability Success(OwnedValue)
    → validate before RuntimeValue materialization / N→1 commit
```

Validation es exacta, recursiva y sin coerción. Esta metadata no participa en ordinary bytecode dispatch y no es general runtime reflection.

## VM-026 — Exact VM Execution Inventory

Status: CLOSED

Cerrado en `VM_EXECUTION_INVENTORY.md` mediante VMI-001..VMI-008.

`VM Execution Data` contiene exactamente **19 identities técnicas propias**:

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

Categorías:

```text
Root / external composition       3
Runtime Value descriptors         2
Backing identities / references   6
Execution backing data            5
Execution state / control         3
                                 ──
TOTAL                            19
```

Reused Semantic/Compiled/evo-values identities no se cuentan otra vez. Containers, primitives, fields, lifetimes y boundaries derivadas no son identities independientes.

No se introducen wrappers como `OperandWindow`, `CallStack`, `FrameBase`, `ExecutionState`, `RuntimeBackingId`, `ApplicationBinding`, `ExecutionContext` o `Session`.

Conteos internos exactos:

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

`ExternalCapability` cuenta como identity VM aunque el tipo técnico de failure de su `Result` sea owned por `Outcome / Diagnostic Data`.

## Authorities

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
- `VM_EXECUTION_INVENTORY.md`
- `COMPILED_BOUNDARY_VALUE_SHAPE.md`
- `../../../../evo-values/INTERCHANGE_MODEL.md`

## Closure

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
VM Execution exact inventory                      ✅ CLOSED — 19 identities

root InstructionPointer                           ❌ EXCLUDED
Current Provider / Active Scope / Host Session    ❌ EXCLUDED
Outcome / Diagnostic representation               ❌ SEPARATE PHASE

VM structural/data model                          ✅ CLOSED
ExternalCapability failure type                   PENDING — Outcome / Diagnostic Data

NEXT
    Architecture map / review
    then Outcome / Diagnostic Data
```