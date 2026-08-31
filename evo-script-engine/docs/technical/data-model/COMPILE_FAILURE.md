# Evo-Script Engine — CompileFailure

Status: CLOSED

Este documento cierra la raíz técnica y las tres subfamilias exactas de `CompileFailure` para `evo-script-engine` v0.

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

```text
kind
    = qué falló

diagnostic
    = dónde ocurrió, cuando existe provenance fuente válida
```

La forma exacta del diagnostic anchor se define posteriormente.

## CPF-003 — Exactly three compile failure families

Status: CLOSED

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

No existe una cuarta familia genérica de compiler failure.

## CPF-004 — No normal BytecodeCompiler / Lowering failure variant

Status: CLOSED

Después de `Semantic Analyzer success`, `SemanticProgram` es válido para el Bytecode Compiler conforme al Technical Data Model cerrado.

No se introducen como language failures normales:

```text
BytecodeFailure
LoweringFailure
CodeGenerationFailure
CompilerInternalFailure
```

Una imposibilidad al traducir un `SemanticProgram` válido representa una violación interna de invariantes / bug de implementación.

## CPF-005 — LexicalFailure belongs only to Lexer responsibility

Status: CLOSED

`LexicalFailure` expresa únicamente invalidez de forma léxica que Lexer puede confirmar.

Autoridad especializada: [`LEXICAL_FAILURE.md`](./LEXICAL_FAILURE.md).

```text
LexicalFailure = 6 variants
```

## CPF-006 — SyntaxFailure belongs only to Parser responsibility

Status: CLOSED

`SyntaxFailure` expresa exclusivamente invalidez sintáctica o estructural que Parser puede confirmar después de recibir Tokens léxicamente válidos.

Autoridad especializada: [`SYNTAX_FAILURE.md`](./SYNTAX_FAILURE.md).

```text
SyntaxFailure = 10 variants
```

## CPF-007 — SemanticFailure belongs only to Semantic Analyzer responsibility

Status: CLOSED

`SemanticFailure` expresa exclusivamente invalidez de significado resuelto sobre un AST sintácticamente válido y un `CompilationCatalog` válido.

Autoridad especializada: [`SEMANTIC_FAILURE.md`](./SEMANTIC_FAILURE.md).

```text
SemanticFailure
├── root variants            7
└── own technical identities 12
```

El modelo cubre resolución, declaraciones, type checking, calls, composites, `when` y Signature satisfaction sin introducir un `SystemError` universal.

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
CompilationCatalog references
```

Si una failure necesita conservar texto o identidad descriptiva, ese dato se materializa como ownership de outcome.

No se introduce:

```rust
CompileFailure<'source>
```

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

## CPF-010 — One primary deterministic failure in v0

Status: CLOSED

`Compile` v0 produce un único `CompileFailure` primario y determinista según `Earliest Responsible Failure`.

No se introduce todavía:

```text
Vec<Diagnostic>
multi-error recovery
continue-after-error compiler model
```

## Exact compile failure families

```text
CompileFailure
│
├── LexicalFailure
│      └── 6 variants
│
├── SyntaxFailure
│      └── 10 variants
│
└── SemanticFailure
       ├── 7 root families
       └── 12 own technical identities
```

La dependency externa de compile-time está cerrada en [`COMPILATION_DEPENDENCY_MODEL.md`](./COMPILATION_DEPENDENCY_MODEL.md):

```text
Source Text
    +
borrowed valid CompilationCatalog
    ↓
Semantic Analyzer
```

Failures de filesystem / `.elib` / `.emod` / construcción del catálogo permanecen fuera de `SemanticFailure` y de este `CompileOutcome` del Engine.

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
    +
CompilationCatalog
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
SystemError universal enum
BytecodeFailure as normal language failure
LoweringFailure as normal language failure
CompileFailure<'source>
Token / AST / SemanticProgram / Catalog references inside public failure
TypeId / SignatureId escaping through SemanticFailure
preformatted message as canonical model
Vec<Diagnostic> multi-error compile result
physical/library/module resolution failures inside Engine SemanticFailure
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
LexicalFailure exact family                                        ✅ CLOSED — 6 variants
SyntaxFailure exact family                                         ✅ CLOSED — 10 variants
SemanticFailure exact family                                       ✅ CLOSED — 12 own identities
CompileFailure exact subfamilies                                   ✅ CLOSED
DiagnosticAnchor exact shape                                       PENDING
ExecutionFailure exact family                                      ← NEXT
```