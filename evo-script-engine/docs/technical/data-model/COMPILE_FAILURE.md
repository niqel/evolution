# Evo-Script Engine — CompileFailure

Status: CLOSED / REVALIDATED AFTER OUTCOME CLOSURE

Este documento cierra la raíz técnica y las tres subfamilias exactas de `CompileFailure` para `evo-script-engine` v0.

```rust
type CompileOutcome = Result<CompiledProgram, CompileFailure>;

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

## CPF-001 — CompileFailure is the owned Compile error

Status: CLOSED

`CompileFailure` es la failure owned transportada exclusivamente por `CompileOutcome::Err`.

## CPF-002 — Meaning and provenance are separate

Status: CLOSED

```text
kind        = qué falló
source_span = dónde ocurrió dentro del Source Text
```

Todo `CompileFailure` normal del Engine posee un `SourceSpan` obligatorio.

## CPF-003 — Exactly three compile failure families

Status: CLOSED

```text
Lexer              → LexicalFailure
Parser             → SyntaxFailure
Semantic Analyzer  → SemanticFailure
```

`CompileFailureKind` posee exactamente 3 variants.

## CPF-004 — No normal BytecodeCompiler / lowering failure

Status: CLOSED

Después de Semantic Analyzer success, un `SemanticProgram` válido debe poder bajarse a `CompiledProgram`.

No existen como failures normales:

```text
BytecodeFailure
LoweringFailure
CodeGenerationFailure
CompilerInternalFailure
```

Una imposibilidad allí es invariant violation interna.

## CPF-005 — LexicalFailure is Lexer-owned

Status: CLOSED

Authority: [`LEXICAL_FAILURE.md`](./LEXICAL_FAILURE.md).

```text
LexicalFailure = 6 variants
```

## CPF-006 — SyntaxFailure is Parser-owned

Status: CLOSED

Authority: [`SYNTAX_FAILURE.md`](./SYNTAX_FAILURE.md).

```text
SyntaxFailure = 10 variants
```

## CPF-007 — SemanticFailure is Semantic Analyzer-owned

Status: CLOSED

Authority: [`SEMANTIC_FAILURE.md`](./SEMANTIC_FAILURE.md).

```text
SemanticFailure
├── root variants            7
└── own technical identities 12
```

Filesystem/module/catalog-construction failures permanecen fuera del Engine; Semantic Analyzer recibe un `CompilationCatalog` válido.

## CPF-008 — CompileFailure is autonomous owned data

Status: CLOSED

No sobreviven borrows hacia:

```text
Source Text
Token / TokenSequence
AST
SemanticProgram working state
CompilationCatalog
```

La provenance sobrevive únicamente como `SourceSpan` de byte offsets.

## CPF-009 — Structured, Consumer-neutral failure data

Status: CLOSED

El failure conserva datos estructurados y provenance técnica; CLI/UI/API/LSP producen mensajes y presentación posteriormente.

No existe un `message: String` como autoridad primaria.

## CPF-010 — One primary deterministic failure in v0

Status: CLOSED

`Compile` produce un único failure primario conforme a `Earliest Responsible Failure`.

No existe en v0:

```text
Vec<Diagnostic>
multi-error recovery
continue-after-error compiler model
```

## Exact compile failure identities contributed to Outcome

El root aporta:

```text
CompileFailure
CompileFailureKind
```

Las subfamilias aportan:

```text
LexicalFailure                      1 identity
SyntaxFailure                       1 identity
SemanticFailure family             12 identities
```

Por tanto la familia completa de compile failures aporta:

```text
2 + 1 + 1 + 12 = 16 identities
```

`CompileOutcome` se cuenta separadamente como alias público de Outcome.

## Diagnostic provenance

Authority: [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md).

```text
CompileFailure.source_span: SourceSpan   mandatory
DiagnosticAnchor                         NOT NEEDED
SourceLocation / SourceId                NOT NEEDED
```

Zero-width spans son válidos para ausencias detectadas en fronteras como EOF.

## Explicitly not introduced

```text
universal compile/runtime Failure enum
SystemError universal enum
BytecodeFailure as normal language failure
CompileFailure<'source>
Token / AST / SemanticProgram / Catalog references in outcome
TypeId / SignatureId escaping through SemanticFailure
preformatted message as canonical model
Vec<Diagnostic>
physical/library/module resolution failures inside SemanticFailure
DiagnosticAnchor
SourceLocation / SourceId
Option<SourceSpan> in CompileFailure
line / column duplication
Source Text ownership in CompileFailure
```

## Closure

```text
CPF-001 CompileFailure owned Compile error                         ✅ CLOSED
CPF-002 meaning separated from SourceSpan provenance               ✅ CLOSED
CPF-003 exactly Lexical / Syntax / Semantic families              ✅ CLOSED
CPF-004 no normal BytecodeCompiler failure                        ✅ CLOSED
CPF-005 LexicalFailure = Lexer responsibility                     ✅ CLOSED
CPF-006 SyntaxFailure = Parser responsibility                     ✅ CLOSED
CPF-007 SemanticFailure = Semantic Analyzer responsibility        ✅ CLOSED
CPF-008 autonomous owned failure data                             ✅ CLOSED
CPF-009 structured Consumer-neutral representation                ✅ CLOSED
CPF-010 one primary deterministic failure v0                      ✅ CLOSED

CompileFailure family contribution                                ✅ 16 identities
Outcome / Diagnostic Data                                         ✅ CLOSED — 24 identities
Technical Data Model                                              ✅ CLOSED

NEXT
    Technical Data Diagram
```