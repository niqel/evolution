# Evo-Script Engine — Outcome / Diagnostic Data

Status: IN ANALYSIS — ROOT + COMPILE FAILURE + DIAGNOSTIC PROVENANCE + EXTERNAL CAPABILITY FAILURE CLOSED

Este documento es la autoridad acumulada de `Outcome / Diagnostic Data` para `evo-script-engine` v0.

La fase representa outcomes públicos, failures técnicos y provenance diagnóstica sin mezclar presentación humana, estado mutable de VM o datos de Host.

## OD-001 — CompileOutcome uses Rust Result

Status: CLOSED

```rust
type CompileOutcome = Result<CompiledProgram, CompileFailure>;
```

No se introduce un enum `CompileOutcome { Success, Failure }` duplicando `Result`.

## OD-002 — ExecutionOutcome uses Rust Result and OwnedValue

Status: CLOSED

```rust
type ExecutionOutcome = Result<OwnedValue, ExecutionFailure>;
```

`OwnedValue` es el Value autónomo que puede sobrevivir a `VmExecution`.

## OD-003 — Execute Source reuses ExecutionOutcome

Status: CLOSED

`Execute Source` y `Execute Compiled` producen la misma identity técnica `ExecutionOutcome`.

No se introduce `ExecuteSourceOutcome`.

## OD-004 — RuntimeValue never escapes as public outcome

Status: CLOSED

Un successful entry `Return` materializa/transfiere el `RuntimeValue` final a `OwnedValue` mientras `VmExecution` y su backing siguen vivos.

```text
RuntimeValue
    ↓ materialize while VmExecution alive
OwnedValue
    ↓
ExecutionOutcome::Ok
    ↓
VmExecution ends
```

## OD-005 — Compile success returns CompiledProgram directly

Status: CLOSED

```text
CompileOutcome::Ok(CompiledProgram)
```

No se introduce `CompileSuccess` wrapper.

## OD-006 — CompileFailure and ExecutionFailure are distinct families

Status: CLOSED

```text
Compile        → CompileFailure
Execution      → ExecutionFailure
```

No se introduce un único enum técnico universal capaz de mezclar indiscriminadamente compilation, invocation y execution failures.

## OD-007 — Evaluation failures are grouped by semantic family

Status: CLOSED

Los errores de evaluación se modelan por familias semánticas reales y no por opcode individual.

Ejemplos ya demostrados por bytecode semantics:

```text
OverflowError
DivisionByZeroError
ConversionError
DynamicNumericTypeError
```

No se introducen `AddFailure`, `SubtractFailure`, `DivideInstructionFailure` u otras identities por opcode sin responsabilidad propia.

## OD-008 — ExternalCapability owns a dedicated failure type

Status: CLOSED

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

`ExternalCapabilityFailure` pertenece a `Outcome / Diagnostic Data` y su representación exacta está cerrada en [`EXTERNAL_CAPABILITY_FAILURE.md`](./EXTERNAL_CAPABILITY_FAILURE.md):

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}
```

Una capability externa no retorna `ExecutionFailure` y no puede fabricar failures internas del Engine.

## OD-009 — Engine external failures remain Engine-owned

Status: CLOSED

Las siguientes condiciones no son `ExternalCapabilityFailure`:

```text
Missing external binding
External success result contract mismatch
```

La primera ocurre antes de invocar una capability.
La segunda es detectada por el Engine al validar un `OwnedValue` exitoso contra `ExternalSymbol.result_shape`.

Ambas pertenecen a la futura familia exacta de `ExecutionFailure`.

## OD-010 — Failure meaning and diagnostic provenance are separate

Status: CLOSED

La identity del error expresa **qué falló**.
La provenance diagnóstica expresa **dónde se originó**, cuando existe una ubicación fuente válida.

La representación exacta de provenance quedó cerrada posteriormente en [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md) reutilizando `SourceSpan` directamente.

No se introduce `DiagnosticAnchor` ni `SourceLocation` en v0.

## Closed Root Shape

```rust
type CompileOutcome =
    Result<CompiledProgram, CompileFailure>;

type ExecutionOutcome =
    Result<OwnedValue, ExecutionFailure>;

type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

## CompileFailure

Status: CLOSED — ROOT + ALL THREE SUBFAMILIES + PROVENANCE

La autoridad especializada está en [`COMPILE_FAILURE.md`](./COMPILE_FAILURE.md).

Forma cerrada:

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

Reglas root `CPF-001..CPF-010` están CLOSED.

Subfamilies exactas:

```text
LexicalFailure
    ✅ CLOSED — 6 variants
    authority: LEXICAL_FAILURE.md

SyntaxFailure
    ✅ CLOSED — 10 variants
    authority: SYNTAX_FAILURE.md

SemanticFailure
    ✅ CLOSED — 12 own technical identities
    root variants = 7
    authority: SEMANTIC_FAILURE.md
```

La familia semántica cerrada se divide en:

```text
ResolutionFailure           4 variants
DeclarationFailure          7 variants
TypeCheckingFailure         8 variants
CallFailure                 7 variants
CompositeFailure           10 variants
WhenFailure                11 variants
SignatureMismatchKind       6 variants

supporting descriptors:
SemanticTypeDescriptor      3 variants
SemanticNameRole            7 variants
SemanticArgumentKind        2 variants
EnumPayloadShape            3 variants
```

La fuente de contratos externos durante compile está cerrada en [`COMPILATION_DEPENDENCY_MODEL.md`](./COMPILATION_DEPENDENCY_MODEL.md): Semantic Analyzer borrows un `CompilationCatalog` validado y no realiza filesystem/module resolution.

Los failures físicos/de construcción de catálogo permanecen fuera de `CompileFailure` del Engine.

## Diagnostic provenance

Status: CLOSED

La autoridad especializada está en [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md).

Decisiones cerradas `DP-001..DP-010`:

```text
DiagnosticAnchor                    ❌ NOT NEEDED v0
SourceLocation identity             ❌ NOT NEEDED v0
SourceId                            ❌ NOT NEEDED v0
new diagnostic identities           0

CompileFailure.source_span          SourceSpan — mandatory
ExecutionFailure.source_span        Option<SourceSpan>
```

Compile provenance:

```text
Lexer / Parser / Semantic Analyzer
    ↓ responsible SourceSpan
CompileFailure {
    kind,
    source_span,
}
```

Runtime provenance:

```text
CallFrame.function
+
InstructionPointer ordinal
    ↓
SourceMap
    ↓
SourceSpan
    ↓
ExecutionFailure.source_span = Some(span)
```

Invocation-boundary failure before a valid `VmExecution` begins:

```text
ExecutionFailure.source_span = None
```

`SourceSpan` es provenance técnica, no presentación. Line/column/snippet/path se derivan posteriormente por el Consumer cuando dispone del Source Text.

## ExternalCapabilityFailure

Status: CLOSED

La autoridad especializada está en [`EXTERNAL_CAPABILITY_FAILURE.md`](./EXTERNAL_CAPABILITY_FAILURE.md).

Forma cerrada:

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}
```

Reglas `ECF-001..ECF-008` están CLOSED.

Consecuencias:

```text
ExternalCapabilityFailure               1 own identity
fields                                  1
code                                    stable symbolic lowercase snake_case
universal external error catalog        ❌ NOT INTRODUCED
Provider/vendor error across ABI        ❌ NOT INTRODUCED
SourceSpan inside capability failure    ❌ NOT INTRODUCED
human message / arbitrary details       ❌ NOT INTRODUCED
```

El adapter de aplicación normaliza el failure físico del Provider antes de cruzar el ABI.

El significado contextual completo se obtiene por:

```text
SignatureSymbol known by Engine
+
ExternalCapabilityFailure.code
```

Si la capability falla durante `CallExternal`, el Engine añade además el `SourceSpan` de la instruction responsable al materializar `ExecutionFailure`.

`MissingBinding` y `ResultContractMismatch` permanecen failures propios del Engine.

## Compile phase failure map

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
AST
    +
CompilationCatalog
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

Bytecode Compiler no agrega una cuarta familia normal de compile failure: una imposibilidad al lowering de un `SemanticProgram` válido es una invariant violation interna.

## Explicitly Not Introduced

```text
CompileSuccess
ExecutionSuccess
ExecuteSourceOutcome
OutcomeValue
ResultValue
universal Failure enum shared blindly by every phase
universal SystemError enum
opcode-specific failure types
RuntimeValue as public outcome
line/column embedded in every error
VmExecution state embedded in public failure
BytecodeFailure as normal language CompileFailure
CompileFailure<'source>
TypeId / SignatureId escaping through SemanticFailure
Vec<Diagnostic> multi-error compile result
physical/module/catalog-construction failures inside SemanticFailure
DiagnosticAnchor
SourceLocation
SourceId
Source Text ownership in failure outcomes
script source location inside ExternalCapabilityFailure
universal ExternalCapability failure enum
Provider/vendor error objects across Engine ABI
human message/details inside ExternalCapabilityFailure
```

## Closure

```text
OD-001 CompileOutcome = Result<CompiledProgram, CompileFailure>    ✅ CLOSED
OD-002 ExecutionOutcome = Result<OwnedValue, ExecutionFailure>    ✅ CLOSED
OD-003 Execute Source reuses ExecutionOutcome                     ✅ CLOSED
OD-004 RuntimeValue materializes before VmExecution ends          ✅ CLOSED
OD-005 no CompileSuccess wrapper                                  ✅ CLOSED
OD-006 distinct CompileFailure / ExecutionFailure                 ✅ CLOSED
OD-007 semantic error families, not opcode failures               ✅ CLOSED
OD-008 ExternalCapabilityFailure dedicated boundary type          ✅ CLOSED
OD-009 missing binding / result mismatch are Engine failures      ✅ CLOSED
OD-010 failure meaning separated from diagnostic provenance       ✅ CLOSED

Outcome / Diagnostic root model                                   ✅ CLOSED
CompileFailure root / CPF-001..CPF-010                            ✅ CLOSED
LexicalFailure exact family                                       ✅ CLOSED — 6 variants
SyntaxFailure exact family                                        ✅ CLOSED — 10 variants
CompilationCatalog corrective dependency                          ✅ CLOSED — 8 identities
SemanticFailure exact family                                      ✅ CLOSED — 12 own identities
CompileFailure exact subfamilies                                  ✅ CLOSED
Diagnostic provenance / DP-001..DP-010                           ✅ CLOSED — 0 new identities
Source Location materialization                                   ✅ CLOSED
DiagnosticAnchor                                                  ❌ NOT NEEDED v0
SourceLocation identity                                           ❌ NOT NEEDED v0
ExternalCapabilityFailure / ECF-001..ECF-008                     ✅ CLOSED — 1 identity
ExternalCapability exact Rust ABI                                 ✅ CLOSED

ExecutionFailure exact family                                     ← NEXT
exact Outcome inventory                                           PENDING
```
