# Evo-Script Engine — Lexical Data

Status: LEXICAL DATA — CLOSED / REVALIDATED

Este documento define los datos producidos por Lexer y consumidos por Parser para archivos `.efn` de Evo-Script.

La frontera `.efn` / Host se rige por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`. En consecuencia, `use` ya no es Structural Keyword de `.efn` y `Active Scope` no forma parte del lenguaje compilado por este Engine.

## 1. Canonical Lexical Flow

```text
Source Text
    ↓
Lexer
    ↓
Token Sequence<'source>
    └── Token 0..N
```

## 2. Closed Structural Rules

### LD-001 — Token representa reconocimiento léxico, no significado semántico

Status: CLOSED

`Token` representa una unidad textual reconocida por Lexer. No determina si un Identifier es Function, Parameter, local binding, type, External Symbol u otra identidad semántica.

```text
Lexer              = lexical recognition
Parser             = syntactic structure
Semantic Analyzer  = resolved meaning
```

### LD-002 — Token contiene Token Kind, Lexeme y Source Span

Status: CLOSED

```text
Token
├── Token Kind
├── Lexeme
└── Source Span
```

### LD-003 — Lexeme es borrowed view del Source Text

Status: CLOSED

`Source Text` permanece como owner del contenido textual durante lexical/parsing working state.

```text
Source Text
    │ owns UTF-8 text
    └── Lexeme borrows textual region
```

Invariantes:

- Lexeme no copia ni adquiere ownership artificial;
- lifetime de Lexeme no excede Source Text;
- Parser puede obtener el texto reconocido desde Token;
- no se introduce un wrapper `Lexeme` si `&str` expresa completamente el dato.

### LD-004 — Source Span representa un rango técnico

Status: CLOSED

`Source Span` identifica una región del Source Text mediante `start` y `end`. Line/column son derivables para diagnostics y no se duplican dentro de cada Token.

### LD-005 — Lexer no materializa `evo-values::Value`

Status: CLOSED

```text
"123"
   ↓ Lexer
IntegerLiteral Token
```

No:

```text
"123"
   ↓ Lexer
Value::Unsigned(123)
```

### LD-006 — Token Sequence es working state temporal materializado

Status: CLOSED

El output lexical posee cardinalidad `0..N Token<'source>` y en v0 se materializa como:

```rust
type TokenSequence<'source> = Vec<Token<'source>>;
```

La gramática no exige almacenar todos los Tokens: el Parser requiere como máximo un Token futuro de lookahead en las decisiones revisadas. La materialización se acepta bajo TD-010 porque simplifica separación, testing, inspección y determinismo del compiler.

Los detalles e invariantes completos están en `TOKEN_SEQUENCE.md`.

### LD-007 — Token Kind es un único enum de clasificación lexical

Status: CLOSED / REVALIDATED

`Token Kind` es una única identidad enum. No se introducen sub-enums artificiales mientras una sola clasificación exprese completamente las formas léxicas requeridas.

Regla:

> Las formas de texto variable se clasifican por familia; las formas fijas reservadas de `.efn` reciben variante exacta. El texto concreto permanece en Lexeme.

El inventario vigente contiene exactamente **50 variantes**.

#### Variable textual

```text
Identifier
```

Los nombres de tipos nativos son lexicalmente Identifier. También `use` es ahora lexicalmente `Identifier` cuando aparece en una posición compatible con la gramática de identifiers: dejó de ser keyword `.efn` y no posee semántica especial.

`scope` tampoco es keyword `.efn`; el command interactivo de Scope pertenece a Evo-Shell/Host.

#### Literals

```text
IntegerLiteral
FloatingLiteral
StringLiteral
BooleanLiteral
```

Reglas:

- IntegerLiteral representa forma decimal entera;
- FloatingLiteral representa decimal o scientific notation;
- StringLiteral conserva delimitadores en Lexeme;
- BooleanLiteral clasifica `true` y `false`, cuyo Lexeme conserva la forma exacta;
- `-` se reconoce como Minus, separado del literal;
- Lexer no materializa Values semánticos.

#### Structural Keywords

```text
Artifact
Let
Struct
Enum
Fn
Public
Private
Return
When
Esig
Import
As
Module
Publish
Bind
To
Entry
This
```

Estas **18 variantes** corresponden al catálogo vigente de Structural Keywords aplicable al lenguaje/artifacts que este modelo lexical reconoce, con `Use` eliminado de `.efn` por la frontera normativa posterior.

`This` permanece porque es marcador sintáctico contextual de Pipeline Data dentro de `.efn`.

Nombres funcionales o de otros dominios como `filter`, `select`, `take`, `enter`, `to_string`, `search` o `use` permanecen Identifier cuando cumplen su gramática; su significado, si existe, se resuelve posteriormente.

#### Operators

```text
FieldAccess
Not
Minus
Multiply
Divide
Remainder
Plus
Less
LessEqual
Greater
GreaterEqual
Equal
NotEqual
And
Or
Pipeline
```

Correspondencia:

```text
FieldAccess      .
Not              !
Minus            -
Multiply         *
Divide           /
Remainder        %
Plus             +
Less             <
LessEqual        <=
Greater          >
GreaterEqual     >=
Equal            ==
NotEqual         !=
And              &&
Or               ||
Pipeline         |>
```

Parser determina si Minus participa como unary o binary operator.

#### Structural Symbols

```text
Association
Colon
Qualification
ReturnType
Correspondence
LeftParenthesis
RightParenthesis
LeftBrace
RightBrace
Comma
Semicolon
```

Correspondencia:

```text
Association       =
Colon             :
Qualification     ::
ReturnType        ->
Correspondence    =>
LeftParenthesis   (
RightParenthesis  )
LeftBrace         {
RightBrace        }
Comma             ,
Semicolon         ;
```

Reglas:

- Association no se denomina assignment porque Evo-Script no posee asignación general;
- Qualification representa `::` en nombres calificados y enum variants;
- Correspondence representa `=>` dentro de `when`, no un expression operator;
- ReturnType representa `->`.

### LD-008 — Whitespace y comments no forman Tokens evaluables

Status: CLOSED

Space, tab, LF y CR actúan como separación lexical. Comments `//` se reconocen para ignorar contenido hasta el final de línea.

No existen Token Kinds normales:

```text
Whitespace
NewLine
Tab
Comment
```

### LD-009 — No existen EOF ni Invalid como Token Kind normal

Status: CLOSED

```text
recognized form
    → Token

unrecognized / malformed form
    → Lexical Failure
```

El final de Token Sequence representa EOF; no se materializa un Token artificial.

### LD-010 — Source Span utiliza byte offsets half-open

Status: CLOSED

Representación Rust:

```rust
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}
```

Convención:

```text
[start, end)
```

Invariantes:

- `start <= end`;
- offsets en bytes desde el inicio del mismo Source Text;
- límites dentro del Source Text;
- límites UTF-8 válidos cuando materializan texto;
- Token normal cumple `start < end`;
- Source Span no contiene referencia al Source Text;
- almacena start/end, no start/length;
- line/column son derivables;
- puede reutilizarse en AST, Semantic Program, Source Mapping y diagnostics;
- `usize` se usa por alineación natural con Rust slicing interno del compiler.

### LD-011 — Lexeme es borrowed `&str` y no sustituye Source Span

Status: CLOSED

```text
Lexeme      = qué texto fue reconocido
Source Span = dónde fue reconocido
```

Representación:

```rust
&'source str
```

Para cada Token:

```text
Token.lexeme
==
&SourceText[Token.span.start .. Token.span.end]
```

Invariantes:

- Lexeme borrows Source Text;
- no copia ni decodifica contenido;
- conserva forma textual exacta;
- StringLiteral conserva quotes/escapes escritos;
- FloatingLiteral conserva representación original;
- Lexeme y Source Span no son duplicación semántica: content != location.

## 3. Closed Token Kind Inventory

```text
Token Kind                          <<enum: 50 variants>>
│
├── Identifier
│
├── IntegerLiteral
├── FloatingLiteral
├── StringLiteral
├── BooleanLiteral
│
├── Artifact
├── Let
├── Struct
├── Enum
├── Fn
├── Public
├── Private
├── Return
├── When
├── Esig
├── Import
├── As
├── Module
├── Publish
├── Bind
├── To
├── Entry
├── This
│
├── FieldAccess
├── Not
├── Minus
├── Multiply
├── Divide
├── Remainder
├── Plus
├── Less
├── LessEqual
├── Greater
├── GreaterEqual
├── Equal
├── NotEqual
├── And
├── Or
├── Pipeline
│
├── Association
├── Colon
├── Qualification
├── ReturnType
├── Correspondence
├── LeftParenthesis
├── RightParenthesis
├── LeftBrace
├── RightBrace
├── Comma
└── Semicolon
```

Total: **50 variantes**.

`Use` ya no pertenece al enum.

## 4. Current Lexical Data Identities

```text
Source Text
    │ owns UTF-8 text
    │
    ├──────────────┐
    │              │
    │ borrows      │ locates
    ▼              ▼
Lexeme          Source Span
&'source str    <<struct>>
                ├── start: usize
                └── end: usize
                    [start, end)
    │              │
    └──────┬───────┘
           ▼
         Token
         ├── Token Kind <<enum: 50 variants>>
         ├── Lexeme
         └── Source Span
           │
           ▼
     Token Sequence<'source>
           = Vec<Token<'source>>
```

## 5. Host Boundary Consequence

El Lexer de `.efn` no modela la command language de Evo-Shell.

```text
Evo-Shell interactive command vocabulary
    !=
Evo-Script `.efn` Token Kind
```

Por tanto, conceptos interactivos como `scope` no se agregan a Token Kind por existir en el Host, y `use` no se conserva como keyword histórica una vez removido de `.efn`.

## 6. Closure

```text
Token Kind              ✅ CLOSED — 50 variants
Source Span             ✅ CLOSED
Lexeme representation   ✅ CLOSED
Token                    ✅ CLOSED
Token Sequence           ✅ CLOSED
Lexical Data             ✅ CLOSED

AST Data                 ← IN ANALYSIS
```
