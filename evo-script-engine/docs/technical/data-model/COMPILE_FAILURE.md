# Evo-Script Engine — CompileFailure

Status: CLOSED — ROOT FAMILY; SUBFAMILIES PENDING

Este documento cierra la raíz técnica de `CompileFailure` para `evo-script-engine` v0.

`CompileFailure` representa el failure owned transportado por:

```rust
type CompileOutcome = Result<CompiledProgram, CompileFailure>;
```

La raíz separa la naturaleza del fallo de su provenance diagnóstica y conserva la frontera `Earliest Responsible Failure` entre Lexer, Parser y Semantic Analyzer.

## Canonical shape

```rust
struct CompileFailure {
    kind: CompileFailureKind,
    diagnostic: Option<DiagnosticAnchor>,
}

enum CompileFailureKind {
    Lexical(LexicalFailure),
    Syntax(SyntaxFailure),
    Semantic(SemanticFailure),
}
```

`DiagnosticAnchor` permanece pendiente dentro de `Outcome / Diagnostic Data`; su placeholder no reabre las reglas cerradas aquí.

## CPF-001 — CompileFailure is the owned Compile error

Status: CLOSED

`CompileFailure` es la failure técnica owned transportada exclusivamente por:

```text
CompileOutcome::Err(CompileFailure)
```

No representa un error de ejecución normal ni una failure de `ExternalCapability`.

## CPF-002 — Meaning and provenance are separate

Status: CLOSED

`CompileFailure` separa dos responsabilidades:

```text
kind
    = qué falló

diagnostic
    = dónde ocurrió, cuando existe provenance fuente válida
```

La forma exacta del diagnostic anchor se define posteriormente.

## CPF-003 — Exactly three compile failure families

Status: CLOSED

`CompileFailureKind` posee exactamente tres variantes:

```rust
enum CompileFailureKind {
    Lexical(LexicalFailure),
    Syntax(SyntaxFailure),
    Semantic(SemanticFailure),
}
```

Correspondencia:

```text
Lexer              → LexicalFailure
Parser             → SyntaxFailure
Semantic Analyzer  → SemanticFailure
```

No se mezcla una cuarta familia genérica de compiler failure.

## CPF-004 — No normal BytecodeCompiler / Lowering failure variant

Status: CLOSED

Después de `Semantic Analyzer success`, `SemanticProgram` es válido para el Bytecode Compiler conforme al Technical Data Model cerrado.

Por tanto no se introduce como failure normal del lenguaje:

```text
BytecodeFailure
LoweringFailure
CodeGenerationFailure
CompilerInternalFailure
```

Una imposibilidad al traducir un `SemanticProgram` válido representa una violación interna de invariantes / bug de implementación, no un `CompileFailure` causado por el Source Text.

## CPF-005 — LexicalFailure belongs only to Lexer responsibility

Status: CLOSED

`LexicalFailure` expresa únicamente invalidez de forma léxica que Lexer puede confirmar con información suficiente.

```text
Source Text
    ↓
Lexer
    ├── TokenSequence
    └── LexicalFailure
```

Parser y Semantic Analyzer no producen `LexicalFailure`.

## CPF-006 — SyntaxFailure belongs only to Parser responsibility

Status: CLOSED

`SyntaxFailure` expresa exclusivamente invalidez sintáctica o estructural que Parser puede confirmar después de recibir Tokens léxicamente válidos.

Incluye las invariantes estructurales `.efn` cuya responsabilidad ya quedó cerrada en AST Data, sin absorber identity resolution o type resolution.

## CPF-007 — SemanticFailure belongs only to Semantic Analyzer responsibility

Status: CLOSED

`SemanticFailure` expresa exclusivamente invalidez de significado resuelto sobre un AST sintácticamente válido.

Su responsabilidad incluye identity resolution, type resolution, duplicate semantic identities, graph validation y Signature resolution conforme a las reglas semánticas cerradas.

## CPF-008 — CompileFailure is autonomous owned data

Status: CLOSED

`CompileFailure` y todo payload que deba sobrevivir a `Compile` son owned.

No pueden conservar borrows persistentes hacia:

```text
Source Text
Token<'source>
TokenSequence<'source>
AST<'source>
SemanticProgram working references
```

Si una failure necesita conservar texto identificado durante compilation, ese dato se materializa como ownership de outcome.

No se introduce:

```rust
CompileFailure<'source>
```

sin una necesidad futura explícita.

## CPF-009 — Structured, Consumer-neutral failure data

Status: CLOSED

La representación canónica conserva datos estructurados, no un único mensaje humano preformateado.

```text
Failure data
    → structured technical meaning

CLI/UI/API/LSP presentation
    → derived later by Consumer/presentation boundary
```

No se usa como autoridad primaria:

```rust
struct CompileFailure {
    message: String,
}
```

Un payload textual puede existir cuando sea dato real de la failure, pero no sustituye la identity estructurada del error.

## CPF-010 — One primary deterministic failure in v0

Status: CLOSED

`Compile` v0 produce un único `CompileFailure` primario y determinista según `Earliest Responsible Failure`.

No se introduce todavía:

```text
Vec<Diagnostic>
multi-error recovery
continue-after-error compiler model
```

Una futura User Story de tooling multi-diagnostic puede agregar recuperación/colección sin cambiar la semántica de esta failure primaria.

## Phase flow

```text
Source Text
    ↓
Lexer
    ├── Err(LexicalFailure)
    │       ↓
    │  CompileFailureKind::Lexical
    │
    ▼
TokenSequence
    ↓
Parser
    ├── Err(SyntaxFailure)
    │       ↓
    │  CompileFailureKind::Syntax
    │
    ▼
AST
    ↓
Semantic Analyzer
    ├── Err(SemanticFailure)
    │       ↓
    │  CompileFailureKind::Semantic
    │
    ▼
SemanticProgram
    ↓
Bytecode Compiler
    ↓
CompiledProgram
```

## Explicitly not introduced

```text
universal compile/runtime Failure enum
BytecodeFailure as normal language failure
LoweringFailure as normal language failure
CompileFailure<'source>
Token / AST references inside public failure
preformatted message as canonical model
Vec<Diagnostic> multi-error compile result
```

## Closure

```text
CPF-001 CompileFailure owned Compile error                         ✅ CLOSED
CPF-002 meaning separated from diagnostic provenance              ✅ CLOSED
CPF-003 exactly Lexical / Syntax / Semantic families              ✅ CLOSED
CPF-004 no normal BytecodeCompiler failure                        ✅ CLOSED
CPF-005 LexicalFailure = Lexer responsibility                     ✅ CLOSED
CPF-006 SyntaxFailure = Parser responsibility                     ✅ CLOSED
CPF-007 SemanticFailure = Semantic Analyzer responsibility        ✅ CLOSED
CPF-008 autonomous owned failure data                             ✅ CLOSED
CPF-009 structured Consumer-neutral representation                ✅ CLOSED
CPF-010 one primary deterministic failure v0                      ✅ CLOSED

CompileFailure root                                                ✅ CLOSED
LexicalFailure exact family                                        ← NEXT
SyntaxFailure exact family                                         PENDING
SemanticFailure exact family                                       PENDING
DiagnosticAnchor exact shape                                       PENDING
```
