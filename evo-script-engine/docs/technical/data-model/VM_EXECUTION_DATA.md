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

> `CompiledProgram` describe mecanismo ejecutable persistente; `VmExecution` describe el estado mutable de una sola ejecución de ese mecanismo.

Un mismo `CompiledProgram` puede participar en múltiples `VmExecution` independientes.

## VM-002 — CompiledProgram Relationship

Status: CLOSED

`VmExecution` referencia exactamente un `CompiledProgram`; no duplica Functions, Constant Pool, External Symbols ni SourceMap.

## VM-003 — Application Bindings Relationship

Status: CLOSED

La ejecución referencia exactamente un conjunto explícito e inmutable de `ApplicationBindings` durante toda la invocation.

La representación exacta queda cerrada en `APPLICATION_BINDINGS.md` y se resume en VM-021.

No existe `Current Provider`, provider lookup ambiental ni Host Session State dentro de la ejecución.

## VM-004 — One Shared Value Storage

Status: CLOSED

```rust
struct SharedValueStorage {
    cells: Vec<Option<RuntimeValue>>,
}
```

Parameters, Locals y Operands son regiones lógicas del mismo storage físico.

## VM-005 — Call Frames

Status: CLOSED

`VmExecution` posee una colección ordenada LIFO de `CallFrame`.

```rust
struct InstructionPointer(usize);

struct CallFrame {
    function: FunctionId,
    instruction_pointer: InstructionPointer,
    frame_base: usize,
}
```

`CallFrame` no posee containers de Parameters, Locals u Operands.

## VM-006 — Execution-Lifetime Backing Ownership

Status: CLOSED

Se preserva:

```text
borrow mientras alcance
ownership cuando deba sobrevivir
```

Cuando datos externos deben sobrevivir a su materializador inmediato, `VmExecution` es owner lógico del backing durante el execution lifetime.

## VM-007 — Invocation Lifetime

Status: CLOSED

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

## VM-008 — Explicitly Excluded from Root

No pertenecen al root:

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

## VM-009 — Root Conceptual Shape

Status: CLOSED

```text
VmExecution
│
├── references exactly 1 CompiledProgram
├── references exactly 1 ApplicationBindings
├── owns exactly 1 SharedValueStorage
├── owns ordered CallFrame collection
├── owns exactly 1 ExecutionBackingStore
└── is logical owner of execution-lifetime backing data
```

La representación Rust exacta se cierra finalmente en VM-024 / `VM_EXECUTION_ROOT.md`.

## VM-010 — RuntimeValue and evo_values::Value<'a> are distinct

Status: CLOSED

```text
RuntimeValue
    = internal stable VM descriptor

evo_values::Value<'a>
    = borrowed/interchange view
```

## VM-011 — RuntimeValue is an immutable internal descriptor

Status: CLOSED

`RuntimeValue` representa un Value ejecutable materializado sin ser automáticamente owner de backing variable/composite.

## VM-012 — Fixed Scalars Inline

Status: CLOSED

```text
Boolean
Int8 / Int16 / Int32 / Int64 / Int128
Uint8 / Uint16 / Uint32 / Uint64 / Uint128
Float32 / Float64
```

Canonical physical mapping:

```text
int   / int32   → Int32
float / float64 → Float64
```

## VM-013 — Variable / Composite Data Uses Backing Indirection

Status: CLOSED

```text
String
Dynamic Integer
Struct
Enum
```

utilizan backing indirection.

## VM-014 — No Persistent Self-Borrow in Shared Value Storage

Status: CLOSED

`SharedValueStorage` no conserva direct Rust references hacia backing data owned por el mismo `VmExecution` cuando eso produciría una estructura self-referential.

## VM-015 — Backing Identity Strategy

Status: CLOSED

Cerrado en `BACKING_IDENTITY_STRATEGY.md`:

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

No existe `RuntimeBackingId` universal. IDs son estables y no se reutilizan durante una `VmExecution`.

## VM-016 — RuntimeValue Exact Representation

Status: CLOSED

Cerrado en `RUNTIME_VALUE_REPRESENTATION.md`.

`RuntimeValue` posee exactamente 17 variants y `DynamicValue` exactamente 3.

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

Copiar el descriptor nunca copia backing. `RuntimeValue` es execution-context-relative.

## VM-017 — Backing Data Representation

Status: CLOSED

Cerrado en `BACKING_DATA_REPRESENTATION.md`.

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

Los cuatro stores son tipados, append-only y positionally indexed. Backings son inmutables después de insertion. Composite backing forma un DAG finito e inmutable.

`DynamicIntegerBacking` encapsula un signed arbitrary-precision integer sin fijar una crate concreta como dependencia arquitectónica.

## VM-018 — Shared Value Storage Exact Representation

Status: CLOSED

Cerrado en `SHARED_VALUE_STORAGE.md` mediante SV-001..SV-011.

```rust
struct SharedValueStorage {
    cells: Vec<Option<RuntimeValue>>,
}
```

```text
Some(RuntimeValue) = occupied materialized cell
None               = reserved LocalSlot not yet materialized
```

Layout:

```text
frame_base
    ↓
[parameters][locals][operands...]
                     ↑
                 operand_base
```

```text
Parameters → always Some
Locals     → None → Some exactly once
Operands   → always Some
```

El Operand Window activo es el tail `cells[operand_base .. cells.len()]`.

Internal `Call` reutiliza los N argument cells superiores como Parameter cells del callee:

```text
callee.frame_base = cells.len() - N
```

`Return` copia el result descriptor, trunca a `callee.frame_base`, elimina el frame y empuja el result para el caller.

## VM-019 — CallFrame Exact Representation

Status: CLOSED

Cerrado en `CALL_FRAME.md` mediante CF-001..CF-010.

```rust
struct InstructionPointer(usize);

struct CallFrame {
    function: FunctionId,
    instruction_pointer: InstructionPointer,
    frame_base: usize,
}
```

Reglas centrales:

```text
CallFrame = one active/suspended internal invocation
FunctionId resolves CompiledFunction
InstructionPointer = mutable runtime identity distinct from InstructionIndex
InstructionPointer identifies current responsible instruction
frame_base = absolute beginning of frame region
operand_base = derived, never stored
```

Internal `Call(FunctionId)` mantiene al caller sobre la `Call` y crea callee en `InstructionPointer(0)`.

`CallExternal` no crea `CallFrame`.

## VM-020 — InstructionPointer Stepping Semantics

Status: CLOSED

Cerrado en `INSTRUCTION_POINTER_STEPPING.md` mediante IP-001..IP-010.

Mientras exista un frame activo:

```text
0 <= ip < compiled_function.instructions.len()
```

No existe past-end IP normal. Todo frame nuevo comienza en `InstructionPointer(0)`.

Regla de commit:

> El IP solo aplica su transición después de completar exitosamente la instruction actual. En failure permanece sobre la instruction responsable.

Esto no implica rollback transaccional ni resumability.

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

No existen `next_ip`, `return_ip`, past-end sentinel ni root-level `InstructionPointer`.

## VM-021 — ApplicationBindings Exact Model

Status: CLOSED

Cerrado en `APPLICATION_BINDINGS.md` mediante AB-001..AB-009.

Representación base v0:

```rust
struct ApplicationBindings {
    capabilities: HashMap<SignatureSymbol, ExternalCapability>,
}
```

Separación de identities:

```text
ExternalSymbolId
    = program-local compiled identity

SignatureSymbol
    = cross-boundary contractual identity used for lookup

ExternalCapability
    = uniform executable function pointer supplied by application
```

`ApplicationBindings` es application-oriented y reusable entre distintos `CompiledProgram`; no está indexado por `ExternalSymbolId`.

`VmExecution` lo borrows de forma inmutable durante toda la invocation. Capabilities extra son válidas.

La resolución es lazy en `CallExternal`:

```text
ExternalSymbolId
    → ExternalSymbol.symbol
    → SignatureSymbol
    → ApplicationBindings lookup
        ├── found   → invoke ExternalCapability
        └── missing → execution Failure
```

Un binding faltante no falla durante Compile ni al crear la ejecución; falla cuando el símbolo correspondiente es realmente alcanzado.

No existen Provider identity, Current Provider, Active Scope, reflection, Service Locator, global registry o binding mutation dentro del modelo.

## VM-022 — ExternalCapability ABI Semantics

Status: CLOSED

Cerrado en `EXTERNAL_CAPABILITY_ABI.md` mediante EC-001..EC-010.

`ExternalCapability` es una única function-pointer identity uniforme almacenada por `ApplicationBindings`.

Reglas de frontera:

```text
VM → external capability
    borrowed evo_values::Value<'a> views

external capability → VM
    one evo_values::OwnedValue on success
```

`RuntimeValue` y sus backing handles nunca cruzan hacia la aplicación.

Para `CallExternal`:

```text
N = ExternalSymbol.parameter_count
arguments = top N active operands
```

Las argument cells permanecen en `SharedValueStorage` durante la invocación. En success, el owned result se materializa/transfiere primero a `RuntimeValue`; solo entonces se reemplazan `N` argumentos por un resultado y `ip += 1`.

En external failure no se hace commit `N → 1` y el IP permanece sobre `CallExternal`; esto no garantiza rollback ni resumability.

No se usa Requester para el one-result `CallExternal` normal porque el resultado debe sobrevivir a la external invocation y por tanto requiere ownership real.

La representation v0 usa plain `fn`, lo cual expresa comportamiento estáticamente compuesto y no captura state de instancia. Una futura necesidad de per-instance state debe reabrir explícitamente esta frontera.

Los tipos exactos de Value del ABI quedan cerrados por VM-023. Solo el tipo de failure permanece pendiente de Outcome / Diagnostic Data.

## VM-023 — evo-values Borrowed / Owned Interchange Model

Status: CLOSED

Cerrado en `evo-values/INTERCHANGE_MODEL.md` mediante EV-001..EV-011.

Separación canónica:

```text
evo_values::Value<'a>
    = borrowed/interchange representation

evo_values::OwnedValue
    = owned/interchange representation

RuntimeValue
    = private VM execution descriptor
```

`Value<'a>` y `OwnedValue` cubren exactamente las mismas 17 familias semánticas de Value que `RuntimeValue`, sin compartir su representación física.

El modelo histórico `Text / Signed / Unsigned` queda sustituido por `String` y numeric variants exact-width.

Dynamic conserva exactamente:

```text
Integer
Float32
Float64
```

Dynamic Integer cruza fronteras mediante representación canónica neutral:

```text
negative + minimal unsigned big-endian magnitude
zero = empty magnitude
zero.negative = false
```

`Value<'a>` puede prestar backing pesado y poseer únicamente árboles temporales de descriptors necesarios para composites/canonicalization. `OwnedValue` es completamente autónomo, sin Rust references ni VM handles.

`evo-values` permanece compatible con `no_std + alloc` y no depende de una implementación BigInt concreta.

El ABI externo puede por tanto fijar sus Value types como:

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, /* failure pending */>;
```

El tipo exacto de external failure se cierra en Outcome / Diagnostic Data y no reabre el intercambio de Values.

## VM-024 — VmExecution Exact Rust Root

Status: CLOSED

Cerrado en `VM_EXECUTION_ROOT.md` mediante VE-001..VE-010.

Representación exacta:

```rust
struct VmExecution<'compiled, 'bindings> {
    compiled_program: &'compiled CompiledProgram,
    application_bindings: &'bindings ApplicationBindings,
    value_storage: SharedValueStorage,
    backing_store: ExecutionBackingStore,
    call_frames: Vec<CallFrame>,
}
```

Reglas centrales:

```text
exactly 5 persistent fields
2 independent external borrow lifetimes
3 mutable runtime roots
no persistent self-borrows
active frame = call_frames.last()
entry_point derived from CompiledProgram
Invocation Values are initialization-only
entry frame = entry_point / ip 0 / frame_base 0
no persistent Running/Completed/Failed state
no Outcome/result/failure field
no derived/cache fields in v0
```

El cierre detectó una inconsistencia previa que NO pertenece al root: el `CompiledProgram` vigente conserva aridad de entry/external calls, pero no conserva información suficiente para validar Value compatibility en dos fronteras.

```text
Execute Compiled Invocation Values
    → exact arity can be checked
    → exact expected Value shape currently cannot

ExternalCapability Success(OwnedValue)
    → one result exists
    → exact expected result Value shape currently cannot
```

La corrección se analiza separadamente en `COMPILED_BOUNDARY_VALUE_SHAPE.md` y no reabre VE-001..VE-010.

## Runtime Value / VM Data Authorities

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
- `COMPILED_BOUNDARY_VALUE_SHAPE.md` — current corrective analysis
- `../../../../evo-values/INTERCHANGE_MODEL.md`

## Current Closure

```text
VmExecution responsibility                     ✅ CLOSED
one invocation per VmExecution                 ✅ CLOSED
CompiledProgram relationship                   ✅ CLOSED
ApplicationBindings relationship               ✅ CLOSED
ApplicationBindings exact model                ✅ CLOSED
one SharedValueStorage root                    ✅ CLOSED
Call Frames root ownership                      ✅ CLOSED
execution-lifetime backing logical owner        ✅ CLOSED
ExecutionBackingStore owner                     ✅ CLOSED

RuntimeValue / evo_values::Value boundary       ✅ CLOSED
RuntimeValue exact representation — 17 variants ✅ CLOSED
DynamicValue exact representation — 3 variants  ✅ CLOSED
Backing Identity Strategy                       ✅ CLOSED
Backing Data Representation                     ✅ CLOSED
Shared Value Storage exact representation       ✅ CLOSED
CallFrame exact representation                  ✅ CLOSED
InstructionPointer identity                     ✅ CLOSED
InstructionPointer current-instruction meaning  ✅ CLOSED
InstructionPointer stepping semantics           ✅ CLOSED
active IP validity / no past-end                 ✅ CLOSED
sequential / branch stepping                     ✅ CLOSED
Call / Return stepping                           ✅ CLOSED
failure preserves responsible IP                 ✅ CLOSED
SignatureSymbol application lookup              ✅ CLOSED
lazy missing-binding failure                     ✅ CLOSED
immutable explicit application composition       ✅ CLOSED
ExternalCapability ABI semantics                ✅ CLOSED
borrowed external arguments                     ✅ CLOSED
owned external success result                   ✅ CLOSED
CallExternal N→1 commit-after-success            ✅ CLOSED
plain fn static composition boundary             ✅ CLOSED
evo-values Value<'a> exact 17 families           ✅ CLOSED
evo-values OwnedValue exact 17 families          ✅ CLOSED
canonical Dynamic Integer interchange            ✅ CLOSED
no_std + alloc shared Value model                 ✅ CLOSED
VmExecution exact 5-field Rust root              ✅ CLOSED
independent CompiledProgram/Bindings lifetimes   ✅ CLOSED
entry frame initialization                       ✅ CLOSED
no VmExecution derived/cache fields              ✅ CLOSED

root InstructionPointer                         ❌ EXCLUDED
Host / Active Scope / Current Provider          ❌ EXCLUDED
Outcome / Diagnostic data                       ❌ SEPARATE PHASE

Compiled Boundary Value Shape                   ← NEXT
ExternalCapability failure type                 PENDING — Outcome / Diagnostic Data
VM Execution exact inventory                    PENDING
```