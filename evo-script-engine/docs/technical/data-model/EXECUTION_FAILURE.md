# Evo-Script Engine — ExecutionFailure

Status: CLOSED

Este documento cierra la familia técnica exacta `ExecutionFailure` para `evo-script-engine` v0.

`ExecutionFailure` es la failure owned transportada por:

```rust
type ExecutionOutcome = Result<OwnedValue, ExecutionFailure>;
```

La familia representa únicamente failures normales que pueden ocurrir al ejecutar `Execute Source` / `Execute Compiled`: compile contextualizado para `Execute Source`, invocation-boundary validation, runtime evaluation y external execution.

No representa invariant violations internas de VM/compiler.

## Canonical shape

```rust
struct ExecutionFailure {
    kind: ExecutionFailureKind,
    source_span: Option<SourceSpan>,
}

enum ExecutionFailureKind {
    Compilation(CompileFailureKind),
    Invocation(InvocationFailure),
    Evaluation(EvaluationFailure),
    External(ExternalExecutionFailure),
}

enum InvocationFailure {
    ArityMismatch {
        expected: usize,
        actual: usize,
    },

    ArgumentShapeMismatch {
        position: usize,
    },
}

enum EvaluationFailure {
    Overflow,
    DivisionByZero,
    Conversion,
    DynamicNumericType,
}

enum ExternalExecutionFailure {
    MissingBinding {
        signature: SignatureSymbol,
    },

    CapabilityFailure {
        signature: SignatureSymbol,
        failure: ExternalCapabilityFailure,
    },

    ResultContractMismatch {
        signature: SignatureSymbol,
    },
}
```

## Exact inventory consequence

Este bloque introduce exactamente cinco identities técnicas propias:

```text
01 ExecutionFailure
02 ExecutionFailureKind
03 InvocationFailure
04 EvaluationFailure
05 ExternalExecutionFailure
```

Reutiliza, sin volver a contarlas:

```text
SourceSpan
CompileFailureKind
SignatureSymbol
ExternalCapabilityFailure
```

Variant / field counts:

```text
ExecutionFailure fields             2
ExecutionFailureKind variants       4
InvocationFailure variants          2
EvaluationFailure variants          4
ExternalExecutionFailure variants   3
```

## EXF-001 — Exact ExecutionFailure root

Status: CLOSED

La raíz exacta es:

```rust
struct ExecutionFailure {
    kind: ExecutionFailureKind,
    source_span: Option<SourceSpan>,
}
```

Separación:

```text
kind
    = qué failure terminó la ejecución

source_span
    = provenance fuente cuando existe una construcción ejecutable responsable
```

La semántica exacta de provenance está cerrada por `DIAGNOSTIC_PROVENANCE.md`.

## EXF-002 — Exactly four execution failure families

Status: CLOSED

`ExecutionFailureKind` posee exactamente cuatro variants:

```text
Compilation
Invocation
Evaluation
External
```

Responsabilidades:

```text
Compilation
    = Compile interno de Execute Source terminó en CompileFailure

Invocation
    = Invocation Values no satisfacen la frontera de entrada del CompiledProgram

Evaluation
    = una operación Evo-Script válida puede fallar legítimamente durante bytecode execution

External
    = failure relacionada con CallExternal / ApplicationBindings / external result contract
```

No se introduce un universal `SystemError` o una familia por opcode.

## EXF-003 — Compilation stores CompileFailureKind, not CompileFailure

Status: CLOSED

`Execute Source` reutiliza `ExecutionOutcome`.

Cuando su fase Compile falla:

```text
CompileFailure {
    kind,
    source_span,
}
        ↓ contextualize
ExecutionFailure {
    kind: ExecutionFailureKind::Compilation(kind),
    source_span: Some(source_span),
}
```

La variant exacta es:

```rust
Compilation(CompileFailureKind)
```

No:

```rust
Compilation(CompileFailure)
```

porque duplicaría provenance dentro de la raíz `ExecutionFailure`.

`Execute Compiled` nunca produce `ExecutionFailureKind::Compilation`.

## EXF-004 — Exact InvocationFailure family

Status: CLOSED

```rust
enum InvocationFailure {
    ArityMismatch {
        expected: usize,
        actual: usize,
    },

    ArgumentShapeMismatch {
        position: usize,
    },
}
```

Son exactamente las dos failures normales de la frontera de Invocation Values:

```text
wrong number of Values
    → ArityMismatch

correct number, but one Value violates entry_parameter_shapes
    → ArgumentShapeMismatch
```

No se introduce una failure distinta por cada `CompiledValueShape` family.

## EXF-005 — Invocation failures occur before a valid VmExecution

Status: CLOSED

Toda `InvocationFailure` ocurre durante boundary validation previa a una `VmExecution` válida.

Por tanto:

```text
ExecutionFailureKind::Invocation(...)
    ⇒ source_span = None
```

No existe todavía una instruction responsable del failure.

No se intenta atribuir el error a la declaración fuente del parameter: la failure pertenece al Value de invocación proporcionado por el Consumer.

## EXF-006 — ArgumentShapeMismatch stores only top-level position

Status: CLOSED

`ArgumentShapeMismatch` conserva únicamente:

```rust
position: usize
```

La validación sigue siendo exacta y recursiva mediante:

```text
CompiledProgram.entry_parameter_shapes[position]
    ↓
CompiledValueShape DAG
    ↓ exact recursive validation
Invocation Value
```

No escapan al outcome:

```text
CompiledValueShapeId
cloned CompiledValueShape graph
CompiledProgram borrow
offending OwnedValue / Value copy
```

La failure conserva la causa estable y autónoma; el artifact compilado conserva el contrato detallado.

## EXF-007 — Exact EvaluationFailure family

Status: CLOSED

```rust
enum EvaluationFailure {
    Overflow,
    DivisionByZero,
    Conversion,
    DynamicNumericType,
}
```

Estas son exactamente las cuatro failures normales demostradas por las semánticas ejecutables v0.

### Overflow

Incluye fixed checked arithmetic y casos equivalentes como:

```text
negate signed MIN
fixed add/subtract/multiply overflow
signed MIN / -1
signed MIN % -1
```

### DivisionByZero

Incluye division/remainder donde corresponda, incluyendo floating `0.0` / `-0.0` y dynamic numeric families.

### Conversion

Una explicit conversion semánticamente válida falla porque el Value concreto no es exactamente representable en el target.

### DynamicNumericType

Arithmetic sobre operands semánticamente `dynamic` alcanza runtime con payload families incompatibles, o una operation dynamic no pertenece a la payload family observada.

No se crean aliases con sufijo `Error` dentro del enum técnico porque la identity ya vive dentro de `EvaluationFailure`.

## EXF-008 — Invariant violations are not EvaluationFailure

Status: CLOSED

Estados que contradicen un `CompiledProgram` válido o las invariantes cerradas de VM no se materializan como outcomes normales Evo-Script.

Ejemplos excluidos:

```text
StackUnderflow
UninitializedLocal
InvalidParameterSlot
InvalidLocalSlot
InvalidFunctionId
InvalidInstructionPointer
WrongRuntimeValueType
InvalidVariantExtraction
OperandDepthExceeded
invalid branch target
invalid compiled shape index
```

Estos estados representan implementation/invariant violations y deben tratarse como bugs internos, no como `ExecutionFailure` del programa.

## EXF-009 — Exact ExternalExecutionFailure family

Status: CLOSED

```rust
enum ExternalExecutionFailure {
    MissingBinding {
        signature: SignatureSymbol,
    },

    CapabilityFailure {
        signature: SignatureSymbol,
        failure: ExternalCapabilityFailure,
    },

    ResultContractMismatch {
        signature: SignatureSymbol,
    },
}
```

Son exactamente las tres failures externas normales de v0.

### MissingBinding

`ApplicationBindings` no contiene una `ExternalCapability` para la `SignatureSymbol` alcanzada por `CallExternal`.

La capability nunca se invoca.

### CapabilityFailure

La capability existe y devuelve:

```text
Err(ExternalCapabilityFailure)
```

El Engine agrega `SignatureSymbol` para contextualizar el code normalizado.

### ResultContractMismatch

La capability devuelve `Ok(OwnedValue)`, pero el Value no satisface exactamente `ExternalSymbol.result_shape`.

El Engine detecta el incumplimiento antes de materializar/commitir el result en VM.

## EXF-010 — External failures preserve SignatureSymbol context

Status: CLOSED

Toda `ExternalExecutionFailure` conserva la `SignatureSymbol` responsable.

```text
MissingBinding
    → SignatureSymbol

CapabilityFailure
    → SignatureSymbol + ExternalCapabilityFailure

ResultContractMismatch
    → SignatureSymbol
```

No se almacena `ExternalSymbolId` porque es local al `CompiledProgram`.

No se duplica `SourceSpan` dentro de estas variants; la provenance está en `ExecutionFailure.source_span`.

No se copia `ExternalSymbol.result_shape` dentro de `ResultContractMismatch`.

## EXF-011 — Bytecode failures materialize SourceSpan before VmExecution ends

Status: CLOSED

Toda failure alcanzada durante bytecode execution:

```text
Evaluation(...)
External(...)
```

materializa:

```text
source_span = Some(responsible instruction SourceSpan)
```

antes de terminar `VmExecution`.

Resolución:

```text
CallFrame.function
+
InstructionPointer ordinal
        ↓
CompiledProgram.source_map
        ↓
SourceSpan
```

Para `ExternalExecutionFailure`, el span corresponde a la instruction `CallExternal` responsable.

No escapan `CallFrame`, `InstructionPointer`, `FunctionId` ni referencias a `CompiledProgram` dentro del outcome.

## EXF-012 — One primary deterministic execution failure in v0

Status: CLOSED

Execution v0 termina en la primera failure normal responsable.

No se introduce:

```text
catch
resume
retry instruction
Vec<ExecutionFailure>
multi-failure aggregation
continue-after-failure execution
```

La instruction responsable no avanza su `InstructionPointer` antes de failure, conforme a `INSTRUCTION_POINTER_STEPPING.md`.

No se requiere rollback general del estado VM ni de side effects externos porque la invocation termina.

## Provenance matrix

```text
ExecutionFailureKind
│
├── Compilation(...)
│      source_span = Some(CompileFailure.source_span)
│
├── Invocation(...)
│      source_span = None
│
├── Evaluation(...)
│      source_span = Some(current instruction span)
│
└── External(...)
       source_span = Some(CallExternal span)
```

## Execution phase map

```text
Execute Source
    │
    ├── Compile failure
    │      ↓
    │  Compilation(CompileFailureKind)
    │  + Some(SourceSpan)
    │
    └── Compile success
           ↓
       same path as Execute Compiled

Execute Compiled
    ↓
Invocation boundary validation
    ├── InvocationFailure
    │      ↓ source_span = None
    │
    ▼
VmExecution
    ↓
bytecode
    ├── EvaluationFailure
    │      ↓ Some(SourceSpan)
    │
    ├── ExternalExecutionFailure
    │      ↓ Some(SourceSpan)
    │
    └── successful entry Return
           ↓
       OwnedValue
```

## Explicitly not introduced

```text
ExecutionFailure<'a>
ExecutionState error wrapper
CompileFailure duplicated inside Compilation variant
CompiledValueShapeId in InvocationFailure
cloned compiled shape graph in InvocationFailure
offending Value copy in InvocationFailure
opcode-specific EvaluationFailure types
StackUnderflow / invalid slots as normal language failures
ExternalSymbolId in public failure
SourceSpan duplicated inside external variants
expected result shape copied into ResultContractMismatch
catch/resume/multi-error execution model
```

## Closure

```text
EXF-001 exact ExecutionFailure root                              ✅ CLOSED
EXF-002 exactly four root families                              ✅ CLOSED
EXF-003 Compilation stores CompileFailureKind                   ✅ CLOSED
EXF-004 InvocationFailure exactly two variants                  ✅ CLOSED
EXF-005 InvocationFailure source_span = None                    ✅ CLOSED
EXF-006 ArgumentShapeMismatch stores only position              ✅ CLOSED
EXF-007 EvaluationFailure exactly four variants                 ✅ CLOSED
EXF-008 invariant violations excluded                           ✅ CLOSED
EXF-009 ExternalExecutionFailure exactly three variants         ✅ CLOSED
EXF-010 external failures preserve SignatureSymbol              ✅ CLOSED
EXF-011 bytecode failures materialize SourceSpan                ✅ CLOSED
EXF-012 one primary deterministic execution failure             ✅ CLOSED

ExecutionFailure exact family                                   ✅ CLOSED — 5 own identities

Outcome exact inventory                                         ← NEXT
```