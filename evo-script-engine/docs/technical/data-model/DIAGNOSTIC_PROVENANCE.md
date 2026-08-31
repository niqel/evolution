# Evo-Script Engine — Diagnostic Provenance

Status: CLOSED

Este documento cierra la provenance diagnóstica y la materialización de Source Location para `evo-script-engine` v0.

La decisión central es deliberadamente mínima:

> `SourceSpan` ya expresa completamente la provenance fuente requerida por v0. No se introduce `DiagnosticAnchor`, `SourceLocation`, `SourceId` ni otra identity diagnóstica adicional.

La identity reutilizada es la ya cerrada en Lexical Data:

```rust
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}
```

con convención half-open:

```text
[start, end)
```

## Canonical outcome shapes

Compile failure:

```rust
struct CompileFailure {
    kind: CompileFailureKind,
    source_span: SourceSpan,
}
```

Execution failure root, cuya familia exacta permanece pendiente:

```rust
struct ExecutionFailure {
    kind: ExecutionFailureKind,
    source_span: Option<SourceSpan>,
}
```

La diferencia es intencional:

```text
CompileFailure
    → siempre nace al analizar un Source Text
    → siempre posee SourceSpan

ExecutionFailure
    → puede fallar antes de iniciar una VmExecution válida
    → invocation-boundary failure puede no tener SourceSpan
    → bytecode execution failure sí posee SourceSpan
```

## DP-001 — No DiagnosticAnchor in v0

Status: CLOSED

No se introduce:

```rust
struct DiagnosticAnchor {
    span: SourceSpan,
}
```

ni un enum equivalente.

`SourceSpan` ya expresa la única provenance técnica requerida por el modelo v0 de un único source coordinate space.

Un wrapper que solo contuviera `SourceSpan` sería ceremonial y no agregaría responsabilidad.

## DP-002 — CompileFailure owns one mandatory SourceSpan

Status: CLOSED

La forma cerrada de `CompileFailure` es:

```rust
struct CompileFailure {
    kind: CompileFailureKind,
    source_span: SourceSpan,
}
```

No se usa `Option<SourceSpan>` en Compile.

Toda failure normal producida por Lexer, Parser o Semantic Analyzer es localizable dentro del Source Text analizado.

Examples conceptuales:

```text
LexicalFailure
    → forma léxica responsable

SyntaxFailure
    → construcción / posición estructural responsable

SemanticFailure
    → referencia, declaración, call, operator o construcción semántica responsable
```

Failures físicas de filesystem, `.elib`, `.emod` o construcción de `CompilationCatalog` permanecen fuera de este `CompileFailure` del Engine.

## DP-003 — Zero-width SourceSpan is valid diagnostic provenance

Status: CLOSED

`SourceSpan` permite:

```text
start == end
```

cuando la provenance representa una frontera o ausencia detectable en una posición concreta.

Por ejemplo:

```text
MissingPublicFunction
    → [source_len, source_len)

missing required final construct detected at EOF
    → [source_len, source_len)
```

Esto no contradice la regla de Token normal `start < end`; un diagnostic span no es necesariamente un Token span.

## DP-004 — ExecutionFailure provenance is Option<SourceSpan>

Status: CLOSED

La raíz futura de execution failure posee:

```rust
source_span: Option<SourceSpan>
```

Interpretación:

```text
None
    = failure de invocation boundary sin una construcción fuente responsable

Some(span)
    = failure contextualizada a una construcción ejecutable del Source Text
```

Ejemplos `None`:

```text
Invocation Value arity mismatch
Invocation Value boundary shape mismatch
```

Ejemplos `Some(span)`:

```text
Overflow
DivisionByZero
Conversion failure
Dynamic numeric mismatch
Missing external binding
ExternalCapability failure
External result contract mismatch
```

## DP-005 — Compile phases materialize provenance before working-state borrows end

Status: CLOSED

Lexer, Parser y Semantic Analyzer seleccionan/materializan el `SourceSpan` responsable antes de que termine su working state temporal.

No escapan dentro del outcome:

```text
Token<'source>
TokenSequence<'source>
AST<'source>
SemanticProgram references
CompilationCatalog references
Source Text borrow
```

El outcome conserva únicamente el meaning owned del failure y el `SourceSpan` técnico correspondiente.

## DP-006 — Runtime provenance resolves through the active instruction

Status: CLOSED

Mientras una instruction falla, `InstructionPointer` permanece identificando la instruction responsable.

La resolución conceptual es:

```text
active CallFrame.function
+
active CallFrame.instruction_pointer ordinal
        ↓
CompiledProgram.source_map
        ↓
SourceSpan
```

Equivalente posicionalmente a:

```text
(FunctionId(f), InstructionPointer(i))
        ↓ observe ordinal i
SourceMap.functions[f][i]
        ↓
SourceSpan
```

`InstructionPointer` y `InstructionIndex` continúan siendo identities semánticamente distintas; compartir el ordinal no las convierte en el mismo tipo.

## DP-007 — Runtime execution coordinates do not escape after span materialization

Status: CLOSED

Una vez resuelta la provenance runtime, el public outcome no conserva:

```text
FunctionId
InstructionPointer
InstructionIndex
CallFrame
CallFrameId
VmExecution
CompiledProgram borrow
```

La ubicación fuente autónoma requerida por v0 es el `SourceSpan` materializado.

Esto permite terminar/destruir `VmExecution` sin invalidar el outcome.

## DP-008 — ExternalCapabilityFailure owns no script source location

Status: CLOSED

`ExternalCapabilityFailure` expresa únicamente qué falló dentro de la capability externa.

No recibe ni conserva:

```text
SourceSpan
FunctionId
InstructionPointer
CompiledProgram
SourceMap
```

Cuando una capability falla durante `CallExternal`, el Engine contextualiza esa failure usando el `SourceSpan` de la instruction `CallExternal` responsable antes de producir `ExecutionFailure`.

```text
ExternalCapabilityFailure
    + current CallExternal provenance
        ↓
ExecutionFailure {
    ...,
    source_span: Some(span),
}
```

## DP-009 — SourceSpan is provenance, not presentation

Status: CLOSED

`SourceSpan` conserva byte offsets técnicos relativos al source coordinate space original.

No se almacenan dentro del failure:

```text
line
column
end_line
end_column
snippet
highlight
Source Text
source path
source display name
preformatted message
```

Cuando el Consumer conserva o puede volver a resolver el Source Text:

```text
SourceSpan
    ↓
line / column / snippet / highlight
```

La presentación es responsabilidad posterior del Consumer/UI/CLI/LSP.

## DP-010 — No SourceLocation / SourceId identity in v0

Status: CLOSED

Un `CompiledProgram` v0 pertenece a un único source coordinate space.

Por tanto no se introduce:

```rust
struct SourceLocation {
    source: SourceId,
    span: SourceSpan,
}
```

ni:

```text
SourceId
SourcePath
SourceName
RuntimeSourceLocation
DiagnosticSourceLocation
```

Si una versión futura permite que un solo `CompiledProgram` contenga Instructions originadas por múltiples Source Text, esta decisión deberá reabrirse y podrá evolucionar naturalmente a una ubicación compuesta `source + span`.

## Materialization map

Compile:

```text
Source Text
    ↓
Lexer / Parser / Semantic Analyzer
    ↓ responsible SourceSpan
CompileFailure {
    kind,
    source_span,
}
```

Runtime:

```text
VmExecution instruction failure
    ↓
CallFrame.function + InstructionPointer
    ↓
SourceMap
    ↓
SourceSpan
    ↓
ExecutionFailure {
    kind,
    source_span: Some(span),
}
```

Invocation boundary:

```text
CompiledProgram + Invocation Values
    ↓ boundary validation fails before valid VmExecution
ExecutionFailure {
    kind,
    source_span: None,
}
```

## Exact identity consequence

Este bloque introduce:

```text
0 new technical identities
```

Reutiliza:

```text
SourceSpan
SourceMap
FunctionId
InstructionPointer
```

según la fase correspondiente.

## Explicitly not introduced

```text
DiagnosticAnchor
SourceLocation
SourceId
SourcePath
SourceName
RuntimeSourceSpan
InstructionLocation
RuntimeInstructionIndex
line / column fields
Source Text ownership in failures
VM execution state in failures
script location inside ExternalCapabilityFailure
```

## Closure

```text
DP-001 no DiagnosticAnchor v0                                  ✅ CLOSED
DP-002 CompileFailure mandatory SourceSpan                     ✅ CLOSED
DP-003 zero-width SourceSpan valid for diagnostic boundaries   ✅ CLOSED
DP-004 ExecutionFailure Optional SourceSpan                    ✅ CLOSED
DP-005 compile provenance materialized before borrows end      ✅ CLOSED
DP-006 runtime provenance resolves active instruction          ✅ CLOSED
DP-007 runtime coordinates do not escape                       ✅ CLOSED
DP-008 ExternalCapabilityFailure owns no script location       ✅ CLOSED
DP-009 SourceSpan = provenance, not presentation               ✅ CLOSED
DP-010 no SourceLocation / SourceId v0                         ✅ CLOSED

Diagnostic provenance exact model                             ✅ CLOSED
Source Location materialization                               ✅ CLOSED
new technical identities                                      0

ExternalCapabilityFailure exact representation                ← NEXT
ExecutionFailure exact family                                 PENDING
Outcome exact inventory                                       PENDING
```
