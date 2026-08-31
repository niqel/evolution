# Evo-Script Engine — Outcome / Diagnostic Data

Status: IN ANALYSIS — ALL OUTCOME SHAPES CLOSED / EXACT INVENTORY PENDING

Este documento es la autoridad acumulada de `Outcome / Diagnostic Data` para `evo-script-engine` v0.

La fase representa outcomes públicos, failures técnicos y provenance diagnóstica sin mezclar presentación humana, estado mutable de VM o datos de Host.

## Closed outcome roots

### OD-001 — CompileOutcome uses Rust Result

Status: CLOSED

```rust
type CompileOutcome = Result<CompiledProgram, CompileFailure>;
```

No se introduce un enum `CompileOutcome { Success, Failure }` duplicando `Result`.

### OD-002 — ExecutionOutcome uses Rust Result and OwnedValue

Status: CLOSED

```rust
type ExecutionOutcome = Result<OwnedValue, ExecutionFailure>;
```

`OwnedValue` puede sobrevivir a `VmExecution`; `RuntimeValue` nunca escapa como outcome público.

### OD-003 — Execute Source reuses ExecutionOutcome

Status: CLOSED

`Execute Source` y `Execute Compiled` producen la misma identity técnica `ExecutionOutcome`. No existe `ExecuteSourceOutcome`.

### OD-004 — RuntimeValue materializes before VmExecution ends

Status: CLOSED

```text
RuntimeValue
    ↓ materialize while VmExecution alive
OwnedValue
    ↓
ExecutionOutcome::Ok
```

### OD-005 — Compile success returns CompiledProgram directly

Status: CLOSED

No existe `CompileSuccess` wrapper.

### OD-006 — CompileFailure and ExecutionFailure are distinct

Status: CLOSED

```text
Compile    → CompileFailure
Execution  → ExecutionFailure
```

No existe un universal Failure enum compartido indiscriminadamente por todas las fases.

### OD-007 — Evaluation failures are semantic families, not opcode failures

Status: CLOSED

Los failures normales de evaluación se agrupan por significado semántico. No se introducen failures específicos por opcode.

### OD-008 — ExternalCapability owns a dedicated failure type

Status: CLOSED

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

`ExternalCapabilityFailure` no es `ExecutionFailure`.

### OD-009 — Missing binding / result mismatch are Engine failures

Status: CLOSED

```text
MissingBinding
ResultContractMismatch
```

pertenecen a `ExternalExecutionFailure`, no a `ExternalCapabilityFailure`.

### OD-010 — Failure meaning and provenance are separate

Status: CLOSED

La failure expresa qué ocurrió; `SourceSpan` expresa dónde ocurrió cuando existe provenance fuente válida.

No existen `DiagnosticAnchor`, `SourceLocation` ni `SourceId` en v0.

---

## CompileFailure

Status: CLOSED — ROOT + ALL SUBFAMILIES + PROVENANCE

Authority: [`COMPILE_FAILURE.md`](./COMPILE_FAILURE.md).

```rust
struct CompileFailure {
    kind: CompileFailureKind,
    source_span: SourceSpan,
}

enum CompileFailureKind {
    Lexical(LexicalFailure),
    Syntax(SyntaxFailure),
    Semantic(SemanticFailure),
}
```

Exact closed subfamilies:

```text
LexicalFailure        ✅ CLOSED — 6 variants
SyntaxFailure         ✅ CLOSED — 10 variants
SemanticFailure       ✅ CLOSED — 12 own identities / 7 root variants
```

Semantic failure supporting families:

```text
ResolutionFailure           4 variants
DeclarationFailure          7 variants
TypeCheckingFailure         8 variants
CallFailure                 7 variants
CompositeFailure           10 variants
WhenFailure                11 variants
SignatureMismatchKind       6 variants
SemanticTypeDescriptor      3 variants
SemanticNameRole            7 variants
SemanticArgumentKind        2 variants
EnumPayloadShape            3 variants
```

Compilation-time external contracts come from a valid borrowed `CompilationCatalog`; filesystem/module/catalog-construction failures remain outside Engine `CompileFailure`.

---

## Diagnostic provenance

Status: CLOSED — 0 NEW IDENTITIES

Authority: [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md).

```text
DiagnosticAnchor      ❌ NOT NEEDED v0
SourceLocation        ❌ NOT NEEDED v0
SourceId              ❌ NOT NEEDED v0
```

Canonical roots:

```rust
struct CompileFailure {
    kind: CompileFailureKind,
    source_span: SourceSpan,
}

struct ExecutionFailure {
    kind: ExecutionFailureKind,
    source_span: Option<SourceSpan>,
}
```

Interpretation:

```text
CompileFailure                     → mandatory SourceSpan
Invocation ExecutionFailure        → None
Bytecode ExecutionFailure          → Some(SourceSpan)
```

Runtime provenance materialization:

```text
CallFrame.function
+
InstructionPointer ordinal
    ↓
CompiledProgram.source_map
    ↓
SourceSpan
```

No VM coordinates or CompiledProgram borrows escape after span materialization.

---

## ExternalCapabilityFailure

Status: CLOSED — 1 OWN IDENTITY

Authority: [`EXTERNAL_CAPABILITY_FAILURE.md`](./EXTERNAL_CAPABILITY_FAILURE.md).

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}
```

Rules `ECF-001..ECF-008` are CLOSED.

```text
fields                                1
code                                  stable symbolic lowercase_snake_case
universal external error catalog      ❌
Provider/vendor error across ABI      ❌
SourceSpan inside capability failure  ❌
human message/details payload         ❌
```

Provider/vendor failures are normalized by the application adapter before crossing the Engine ABI.

---

## ExecutionFailure

Status: CLOSED — 5 OWN IDENTITIES

Authority: [`EXECUTION_FAILURE.md`](./EXECUTION_FAILURE.md).

Canonical shape:

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

Rules `EXF-001..EXF-012` are CLOSED.

Exact counts:

```text
ExecutionFailure fields             2
ExecutionFailureKind variants       4
InvocationFailure variants          2
EvaluationFailure variants          4
ExternalExecutionFailure variants   3
own technical identities            5
```

Provenance matrix:

```text
Compilation(...)  → Some(original CompileFailure.source_span)
Invocation(...)   → None
Evaluation(...)   → Some(responsible instruction SourceSpan)
External(...)     → Some(responsible CallExternal SourceSpan)
```

`Compilation` stores `CompileFailureKind`, not `CompileFailure`, avoiding duplicated `SourceSpan`.

`ArgumentShapeMismatch` stores only top-level argument `position`; exact recursive validation remains owned by `CompiledProgram.entry_parameter_shapes` + `CompiledValueShape`.

Normal evaluation failures are exactly:

```text
Overflow
DivisionByZero
Conversion
DynamicNumericType
```

VM/compiler invariant violations are internal bugs and never normal `ExecutionFailure` values.

Normal external execution failures are exactly:

```text
MissingBinding
CapabilityFailure
ResultContractMismatch
```

Each preserves the responsible `SignatureSymbol`; only `CapabilityFailure` additionally owns `ExternalCapabilityFailure`.

Execution v0 produces one primary deterministic failure; no catch/resume/multi-failure model exists.

---

## Phase maps

Compile:

```text
Source Text
    ↓
Lexer
    ├── LexicalFailure
    ▼
TokenSequence
    ↓
Parser
    ├── SyntaxFailure
    ▼
AST + CompilationCatalog
    ↓
Semantic Analyzer
    ├── SemanticFailure
    ▼
SemanticProgram
    ↓
Bytecode Compiler
    ↓
CompiledProgram
```

Execution:

```text
Execute Source
    ├── CompileFailure
    │      ↓ contextualize
    │  ExecutionFailure::Compilation
    └── Compile success
           ↓
       same path as Execute Compiled

Execute Compiled
    ↓
Invocation boundary validation
    ├── InvocationFailure / source_span None
    ▼
VmExecution
    ├── EvaluationFailure / Some(SourceSpan)
    ├── ExternalExecutionFailure / Some(SourceSpan)
    └── successful entry Return → OwnedValue
```

---

## Explicitly not introduced

```text
CompileSuccess
ExecutionSuccess
ExecuteSourceOutcome
OutcomeValue
ResultValue
universal Failure/SystemError enum
opcode-specific failure types
RuntimeValue as public outcome
DiagnosticAnchor
SourceLocation
SourceId
line/column/snippet/path embedded in failures
CompileFailure<'source>
ExecutionFailure<'a>
Vec<Diagnostic>
Vec<ExecutionFailure>
catch/resume execution
BytecodeFailure as normal language failure
Provider/vendor error objects across Engine ABI
human messages as canonical failure representation
CompiledValueShapeId inside InvocationFailure
VM invariant violations as normal execution failures
```

## Closure

```text
OD-001..OD-010 Outcome root rules                              ✅ CLOSED
CompileFailure / CPF-001..CPF-010                             ✅ CLOSED
LexicalFailure                                                 ✅ CLOSED — 6 variants
SyntaxFailure                                                  ✅ CLOSED — 10 variants
SemanticFailure                                                ✅ CLOSED — 12 own identities
Diagnostic provenance / DP-001..DP-010                        ✅ CLOSED — 0 new identities
ExternalCapabilityFailure / ECF-001..ECF-008                  ✅ CLOSED — 1 identity
ExternalCapability exact Rust ABI                              ✅ CLOSED
ExecutionFailure / EXF-001..EXF-012                           ✅ CLOSED — 5 own identities

exact Outcome / Diagnostic inventory                           ← NEXT
```

El siguiente bloque no debe introducir semántica nueva: debe auditar y contar todas las identities propias de `Outcome / Diagnostic Data`, distinguir aliases/reused identities y declarar la fase completa CLOSED si no aparece ninguna inconsistencia.