# Evo-Script Engine — LexicalFailure

Status: CLOSED

Este documento cierra la familia técnica exacta `LexicalFailure` para `evo-script-engine` v0.

`LexicalFailure` pertenece exclusivamente a la responsabilidad del Lexer y expresa una forma fuente que no puede materializarse como una secuencia de Tokens léxicamente válida.

## Canonical shape

```rust
enum LexicalFailure {
    UnrecognizedCharacter(char),
    InvalidIdentifier,
    MalformedNumericLiteral,
    UnterminatedStringLiteral,
    InvalidStringEscape(char),
    PhysicalNewlineInStringLiteral,
}
```

Inventario exacto:

```text
LexicalFailure variants = 6
```

## LF-001 — Exactly six lexical failure variants

Status: CLOSED

`LexicalFailure` posee exactamente seis variants:

```text
UnrecognizedCharacter
InvalidIdentifier
MalformedNumericLiteral
UnterminatedStringLiteral
InvalidStringEscape
PhysicalNewlineInStringLiteral
```

No se introduce `InvalidToken`; una forma inválida produce failure antes de materializar un Token inválido.

## LF-002 — Failure meaning only; provenance stays outside

Status: CLOSED

`LexicalFailure` expresa únicamente **qué forma léxica falló**.

La provenance fuente pertenece a:

```text
CompileFailure.source_span
```

No se embeben `SourceSpan`, line/column ni posiciones dentro de cada variant léxica.

## LF-003 — Every lexical failure is source-locatable

Status: CLOSED

Todo `LexicalFailure` producido por `Compile` posee una ubicación fuente determinable.

Por tanto, para esta familia aplica la invariante:

```text
CompileFailureKind::Lexical(...)
    ⇒ CompileFailure.source_span exists
```

La provenance exacta está cerrada en [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md) y reutiliza directamente `SourceSpan`.

## LF-004 — UnrecognizedCharacter

Status: CLOSED

```rust
UnrecognizedCharacter(char)
```

representa un carácter que, en la posición observada, no puede participar en ninguna forma léxica válida de Evo-Script v0.

No se produce un Token de error.

La failure conserva únicamente el carácter ofensivo como payload mínimo estructurado.

## LF-005 — InvalidIdentifier is lexical grammar only

Status: CLOSED

`InvalidIdentifier` representa una candidata a identifier que viola la gramática léxica oficial:

```text
identifier
    := ascii_letter (ascii_letter | digit | "_")*
```

La convención semántica de nombres (`snake_case`, `PascalCase`) no pertenece al Lexer.

```text
invalid identifier character/form
    → LexicalFailure::InvalidIdentifier

valid identifier grammar + wrong naming convention
    → later SemanticFailure
```

## LF-006 — MalformedNumericLiteral is lexical form, not representability

Status: CLOSED

`MalformedNumericLiteral` representa una candidata a literal numérico que no satisface ninguna forma canónica válida de integer, decimal o scientific literal.

Incluye forms lexicalmente mal formadas como exponentes incompletos, separadores `_` prohibidos o sufijos numéricos no definidos.

La representabilidad en el tipo contextual esperado no es lexical:

```text
"300"
    → valid IntegerLiteral Token

300 expected as uint8
    → SemanticFailure
```

No se introducen variants independientes por cada detalle de la gramática numérica.

## LF-007 — Exact string lexical failures

Status: CLOSED

Los failures léxicos propios de un string literal son exactamente:

```text
UnterminatedStringLiteral
InvalidStringEscape(char)
PhysicalNewlineInStringLiteral
```

Interpretación:

```text
opening quote reaches end of Source Text
    → UnterminatedStringLiteral

backslash followed by unsupported escape code
    → InvalidStringEscape(code)

physical newline appears before closing quote
    → PhysicalNewlineInStringLiteral
```

## LF-008 — Unicode string content is valid; Compile receives Source Text

Status: CLOSED

El contenido Unicode UTF-8 directo dentro de strings es válido conforme a la especificación del lenguaje.

No se introducen dentro de esta frontera:

```text
InvalidUtf8
InvalidUnicodeString
SourceEncodingFailure
```

porque el Use Case `Compile` recibe Source Text, no bytes físicos que deban decodificarse.

## LF-009 — Valid Tokens forming invalid grammar belong to SyntaxFailure

Status: CLOSED

Si el Source Text puede descomponerse completamente en Tokens válidos pero su combinación no pertenece a la gramática, el responsable es Parser:

```text
valid Tokens
    + invalid grammar
    → SyntaxFailure
```

El Lexer no infiere intención del programador ni rol sintáctico.

## LF-010 — No copied lexeme/source fragment in v0

Status: CLOSED

`LexicalFailure` no conserva copias completas del lexeme ni slices prestados del Source Text.

No se introduce:

```text
InvalidIdentifier(Box<str>)
MalformedNumericLiteral(Box<str>)
LexicalFailure<'source>
```

La combinación:

```text
structured failure meaning
+
CompileFailure.source_span
```

es suficiente para identificar la región fuente responsable sin acoplar la failure al Source Text.

Solo se conserva payload mínimo cuando aporta significado técnico real, como el `char` ofensivo o el código de escape desconocido.

## Phase boundary

```text
Source Text
    ↓
Lexer
    ├── Success
    │      ↓
    │   TokenSequence
    │
    └── Failure
           ↓
       LexicalFailure
           ↓
       CompileFailure {
           kind: Lexical(...),
           source_span,
       }
```

## Explicitly not introduced

```text
TokenKind::Invalid
InvalidToken
InvalidUtf8
InvalidUnicodeString
SourceEncodingFailure
InvalidBooleanLiteral
UnsupportedMultilineComment
numeric representability as lexical failure
naming-convention failures in Lexer
SourceSpan inside each lexical variant
copied lexeme payloads
LexicalFailure<'source>
DiagnosticAnchor
SourceLocation
```

## Closure

```text
LF-001 exactly six variants                                      ✅ CLOSED
LF-002 meaning separated from provenance                         ✅ CLOSED
LF-003 every lexical failure source-locatable                    ✅ CLOSED
LF-004 UnrecognizedCharacter                                     ✅ CLOSED
LF-005 InvalidIdentifier = identifier grammar only               ✅ CLOSED
LF-006 MalformedNumericLiteral = lexical form only               ✅ CLOSED
LF-007 exact string lexical failure family                       ✅ CLOSED
LF-008 Unicode strings valid / no byte-decoding failure here     ✅ CLOSED
LF-009 valid Tokens + invalid grammar → SyntaxFailure            ✅ CLOSED
LF-010 no copied lexeme/source fragment                          ✅ CLOSED

LexicalFailure exact family                                      ✅ CLOSED — 6 variants
SyntaxFailure exact family                                       ✅ CLOSED elsewhere
SemanticFailure exact family                                     ✅ CLOSED elsewhere
Diagnostic provenance                                            ✅ CLOSED — SourceSpan
```
