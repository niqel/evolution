# Evo-Script Engine — Outcome / Diagnostic Data

Status: CLOSED

Este documento es la autoridad acumulada de `Outcome / Diagnostic Data` para `evo-script-engine` v0.

La fase representa outcomes públicos, failures técnicos y provenance diagnóstica sin mezclar presentación humana, estado mutable de VM o datos de Host.

## Public outcome aliases

```rust
type CompileOutcome = Result<CompiledProgram, CompileFailure>;

type ExecutionOutcome = Result<OwnedValue, ExecutionFailure>;
```

Decisiones root `OD-001..OD-010`: CLOSED.

Reglas centrales:

```text
Compile success      → CompiledProgram
Execution success    → OwnedValue
Execute Source       → reuses ExecutionOutcome
RuntimeValue         → never escapes public outcome
CompileFailure       != ExecutionFailure
ExternalCapability   → returns ExternalCapabilityFailure
failure meaning      != source provenance
```

No existen `CompileSuccess`, `ExecutionSuccess`, `ExecuteSourceOutcome`, `OutcomeValue`, `ResultValue` ni un universal `Failure/SystemError` enum.

## CompileFailure

Status: CLOSED

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

Subfamilies:

```text
LexicalFailure          ✅ CLOSED — 6 variants
SyntaxFailure           ✅ CLOSED — 10 variants
SemanticFailure         ✅ CLOSED — 12 own identities
```

Authorities:

- [`LEXICAL_FAILURE.md`](./LEXICAL_FAILURE.md)
- [`SYNTAX_FAILURE.md`](./SYNTAX_FAILURE.md)
- [`SEMANTIC_FAILURE.md`](./SEMANTIC_FAILURE.md)

La familia semántica exacta contiene:

```text
SemanticFailure             7 variants
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

Filesystem/module/catalog-construction failures permanecen fuera de Engine `CompileFailure`; Semantic Analyzer recibe un `CompilationCatalog` válido y borrowed.

## Diagnostic provenance

Status: CLOSED — 0 new identities

Authority: [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md).

```text
DiagnosticAnchor      ❌ NOT NEEDED v0
SourceLocation        ❌ NOT NEEDED v0
SourceId              ❌ NOT NEEDED v0
```

Canonical provenance:

```text
CompileFailure
    → source_span: SourceSpan
    → mandatory

ExecutionFailure
    → source_span: Option<SourceSpan>
    → Invocation = None
    → bytecode failure = Some(span)
```

Runtime source materialization:

```text
CallFrame.function
+
InstructionPointer ordinal
    ↓
CompiledProgram.source_map
    ↓
SourceSpan
```

Line/column/snippet/path son presentation derivada por el Consumer y no forman parte del outcome técnico.

## ExternalCapabilityFailure

Status: CLOSED — 1 own identity

Authority: [`EXTERNAL_CAPABILITY_FAILURE.md`](./EXTERNAL_CAPABILITY_FAILURE.md).

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}
```

`code` es symbolic lowercase snake_case, owned y Consumer-neutral.

El Provider/application adapter normaliza errors físicos antes de cruzar el ABI.

Exact ABI:

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

`MissingBinding` y `ResultContractMismatch` siguen siendo failures propios del Engine.

## ExecutionFailure

Status: CLOSED — 5 own identities

Authority: [`EXECUTION_FAILURE.md`](./EXECUTION_FAILURE.md).

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

Provenance matrix:

```text
Compilation(...)  → Some(original CompileFailure.source_span)
Invocation(...)   → None
Evaluation(...)   → Some(responsible instruction SourceSpan)
External(...)     → Some(responsible CallExternal SourceSpan)
```

VM/compiler invariant violations no se representan como normal `ExecutionFailure`.

## Exact Outcome / Diagnostic inventory

Status: CLOSED — 24 own identities

Authority: [`OUTCOME_DIAGNOSTIC_INVENTORY.md`](./OUTCOME_DIAGNOSTIC_INVENTORY.md).

```text
Public outcome aliases             2
Compile failure root               2
Lexical / Syntax failure families  2
Semantic failure family           12
External capability failure        1
Execution failure family           5
                                  ──
TOTAL                              24
```

Exact identities:

```text
01 CompileOutcome
02 ExecutionOutcome
03 CompileFailure
04 CompileFailureKind
05 LexicalFailure
06 SyntaxFailure
07 SemanticFailure
08 ResolutionFailure
09 DeclarationFailure
10 TypeCheckingFailure
11 CallFailure
12 CompositeFailure
13 WhenFailure
14 SignatureMismatchKind
15 SemanticTypeDescriptor
16 SemanticNameRole
17 SemanticArgumentKind
18 EnumPayloadShape
19 ExternalCapabilityFailure
20 ExecutionFailure
21 ExecutionFailureKind
22 InvocationFailure
23 EvaluationFailure
24 ExternalExecutionFailure
```

Reused identities are not recounted:

```text
SourceSpan
CompiledProgram
OwnedValue
SignatureSymbol
NativeType
TypeSymbol
UnaryOperator
BinaryOperator
ExternalCapability
```

Containers/primitives/fields are not identities:

```text
Result
Option
Box
Vec
Box<str>
char
usize
kind
source_span
code
expected
actual
position
signature
failure
```

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
    ├── InvocationFailure
    ▼
VmExecution
    ├── EvaluationFailure
    ├── ExternalExecutionFailure
    └── successful entry Return → OwnedValue
```

## Closure

```text
OD-001..OD-010 Outcome root rules                              ✅ CLOSED
CompileFailure / CPF-001..CPF-010                             ✅ CLOSED
LexicalFailure / LF-001..LF-010                              ✅ CLOSED
SyntaxFailure / SF-001..SF-011                               ✅ CLOSED
SemanticFailure / SEF-001..SEF-012                           ✅ CLOSED
Diagnostic provenance / DP-001..DP-010                       ✅ CLOSED
ExternalCapabilityFailure / ECF-001..ECF-008                 ✅ CLOSED
ExecutionFailure / EXF-001..EXF-012                          ✅ CLOSED
Outcome inventory / OI-001..OI-008                           ✅ CLOSED — 24 identities

Outcome / Diagnostic Data                                    ✅ CLOSED
Technical Data Model                                         ✅ CLOSED
```

## Next

```text
Technical Data Diagram
```

El siguiente artifact debe representar visualmente las identities y relaciones ya cerradas. No debe introducir nuevas estructuras conductuales ni reabrir el Technical Data Model sin una inconsistencia explícitamente demostrada.