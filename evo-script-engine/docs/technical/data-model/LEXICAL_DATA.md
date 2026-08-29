# Evo-Script Engine — Lexical Data

Status: SOURCE SPAN + LEXEME — CLOSED

Este documento define las decisiones del primer bloque del Technical Data Model de `evo-script-engine`: los datos producidos por el Lexer y consumidos por el Parser.

El bloque lexical deriva de la especificación de Evo-Script y no introduce significado semántico que corresponda al Parser o al Semantic Analyzer.

## 1. Canonical Lexical Flow

```text
Source Text
    ↓
Lexer
    ↓
Token Sequence
    └── Token 0..N
```

Un `Token` representa una unidad léxica reconocida dentro del `Source Text`.

## 2. Closed Structural Rules

### LD-001 — Token representa reconocimiento léxico, no significado semántico

Status: CLOSED

`Token` representa una unidad textual reconocida por el Lexer.

No determina todavía si un identificador representa una función, parámetro, binding local, tipo nativo, tipo definido, External Symbol u otra identidad semántica.

```text
Lexer
    = reconocimiento léxico

Parser
    = estructura sintáctica

Semantic Analyzer
    = significado resuelto
```

### LD-002 — Token contiene Token Kind, Lexeme y Source Span

Status: CLOSED

Cada `Token` conserva conceptualmente:

```text
Token
├── Token Kind
├── Lexeme
└── Source Span
```

`Token Kind` clasifica la unidad léxica.

`Lexeme` conserva la vista textual concreta reconocida por el Lexer.

`Source Span` identifica el rango ocupado por el token dentro del `Source Text`.

### LD-003 — Lexeme es una borrowed view del Source Text

Status: CLOSED

`Source Text` permanece como owner de su contenido textual durante el pipeline lexical y sintáctico.

`Lexeme` no copia ni adquiere ownership artificial del texto reconocido.

```text
Source Text
    │ owns
    ▼
UTF-8 source content
    │
    └── Lexeme
         borrows textual region
```

Invariantes:

- el lifetime de un `Lexeme` no puede exceder el lifetime del `Source Text` del que deriva;
- el Parser puede consumir la información textual necesaria desde el propio `Token` sin requerir una dependencia oculta adicional hacia `Source Text` únicamente para recuperar el lexema;
- un `Lexeme` borrowed es válido mientras su owner continúe vivo;
- esta decisión no obliga a crear un `struct Lexeme` si una borrowed textual view expresa completamente el concepto.

### LD-004 — Source Span representa un rango técnico; line/column son derivables

Status: CLOSED

`Source Span` identifica el rango que un token ocupa dentro del `Source Text`.

Conceptualmente:

```text
Source Span
├── start
└── end
```

La representación fundamental no duplica `line` y `column` dentro de cada Token.

La información de línea y columna se deriva posteriormente para diagnóstico a partir del `Source Text` y del `Source Span`.

### LD-005 — Lexer no materializa evo-values::Value

Status: CLOSED

El Lexer reconoce la categoría textual de un literal, pero no materializa su significado semántico como `evo-values::Value`.

Ejemplo conceptual:

```text
"123"
   ↓ Lexer
Integer Literal Token
```

No:

```text
"123"
   ↓ Lexer
Value::Unsigned(123)
```

La interpretación sintáctica y semántica pertenece a etapas posteriores.

### LD-006 — Token Sequence contiene 0..N Tokens sin prescribir colección Rust

Status: CLOSED

El output lexical es conceptualmente una `Token Sequence` con cardinalidad `0..N`.

```text
Token Sequence
    └── Token 0..N
```

Esta identidad no prescribe `Vec<Token>` ni otra colección concreta.

La representación física de la secuencia se decidirá únicamente cuando el Technical Data Model demuestre qué operaciones, ownership, cardinalidad y acceso necesita el Parser.

### LD-007 — Token Kind es un único enum de clasificación léxica

Status: CLOSED

`Token Kind` es una única identidad enum que clasifica las formas léxicas reconocidas por Evo-Script v0.1.

No se introducen sub-enums artificiales para keywords, operators, literals o delimiters mientras un único `Token Kind` exprese completamente la clasificación requerida.

Regla:

> Las formas de texto variable se clasifican por familia; las formas fijas y reservadas del lenguaje reciben una variante exacta. El texto concreto permanece en `Lexeme` y no se duplica como payload dentro de `Token Kind`.

El inventario cerrado contiene exactamente **51 variantes**.

#### Variable textual

```text
Identifier
```

Los nombres de tipos nativos (`int`, `int64`, `string`, `dynamic`, etc.) son lexicalmente `Identifier`.

El Lexer no decide si un identificador representa Native Type, User Type, Function, Signature, binding, field, enum variant u otra identidad semántica. Esa resolución pertenece a etapas posteriores.

#### Literals

```text
IntegerLiteral
FloatingLiteral
StringLiteral
BooleanLiteral
```

Reglas:

- `IntegerLiteral` representa la forma lexical entera decimal;
- `FloatingLiteral` representa tanto la forma decimal como la notación científica;
- `StringLiteral` representa el literal textual completo, incluidos sus delimitadores en el `Lexeme`;
- `BooleanLiteral` clasifica tanto `true` como `false`; el `Lexeme` conserva cuál de los dos fue escrito;
- `-` no forma parte de un literal numérico negativo: se reconoce separadamente como `Minus`;
- el Lexer no materializa valores semánticos a partir de estos literales.

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
Use
```

Estas 19 variantes corresponden exactamente al catálogo de Structural Keywords reservado por Evo-Script v0.1.

Nombres funcionales o de otros dominios como `filter`, `select`, `take`, `enter`, `to_string` o `search` no se convierten en keywords por tener significado semántico en otro nivel; lexicalmente permanecen como `Identifier` cuando cumplen su gramática.

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

Correspondencia textual:

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

`Minus` es una sola variante lexical. El Parser determina posteriormente si una ocurrencia de `-` participa como operador unario o binario.

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

Correspondencia textual:

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

- `Association` no se denomina assignment porque Evo-Script v0.1 no posee asignación general; `=` participa en la ligadura inicial de `let`;
- `Colon` conserva un nombre mecánico porque participa en más de una construcción sintáctica;
- `Qualification` representa `::` tanto en nombres modulares calificados como en `TipoEnum::Variante`;
- `Correspondence` representa `=>`, marcador exclusivo de correspondencia en `when`, y no un operador de expresión;
- `ReturnType` representa `->` como delimitador de tipo de resultado.

### LD-008 — Whitespace y comments no forman parte del Token Sequence evaluable

Status: CLOSED

Los caracteres de whitespace definidos por Evo-Script v0.1 (`space`, `tab`, `LF`, `CR`) actúan como separación léxica y no producen `Token`.

Los comentarios `//` son reconocidos por el Lexer para ignorar el contenido hasta el final de la línea física y tampoco producen `Token` evaluable.

Por tanto no existen variantes normales:

```text
Whitespace
NewLine
Tab
Comment
```

Los saltos de línea no poseen significado estructural fuera de su función para terminar comentarios de línea y de las restricciones de String Literal.

### LD-009 — No existen EOF ni Invalid como Token Kind normales

Status: CLOSED

`Token Sequence` expresa su final mediante la terminación de la propia secuencia; Evo-Script v0.1 no demuestra la necesidad de un Token artificial `EndOfFile`.

Por tanto:

```text
EndOfFile  != Token Kind
```

Una forma textual no reconocible tampoco se convierte en un Token artificial `Invalid`.

```text
recognized form
    → Token

unrecognized / malformed form
    → Lexical Failure
```

`Lexical Failure` y sus datos de diagnóstico se definirán en el bloque correspondiente de Outcome / Diagnostic Data.

### LD-010 — Source Span utiliza byte offsets half-open

Status: CLOSED

`Source Span` es un dato técnico con identidad propia que representa una región del `Source Text` mediante dos offsets de bytes desde el inicio del contenido UTF-8.

Representación Rust cerrada:

```rust
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}
```

La convención de rango es **half-open**:

```text
[start, end)
```

`start` está incluido y `end` está excluido.

Esta representación se alinea directamente con el slicing natural de `str` en Rust:

```rust
&source[start..end]
```

Invariantes:

- `start <= end`;
- ambos offsets se expresan en bytes desde el inicio del mismo `Source Text`;
- ambos límites deben estar dentro de la longitud del `Source Text` al que se aplican;
- cuando el span delimita texto UTF-8, `start` y `end` deben corresponder a fronteras UTF-8 válidas;
- un `Token` normal ocupa al menos un byte, por lo que para Tokens se cumple `start < end`;
- `Source Span` no contiene ni posee una referencia al `Source Text`;
- `Source Span` almacena `start` y `end`, no `start` y `length`;
- `line` y `column` no forman parte de su representación fundamental y se derivan cuando un diagnóstico los necesita;
- `Source Span` puede reutilizarse posteriormente en AST, Semantic Program, Source Mapping y diagnostics cuando exista una región del source que deba preservarse;
- se utiliza `usize` porque los offsets son internos al proceso de compilación y se relacionan directamente con longitudes y slices Rust; no se introduce una limitación artificial `u32` sin una necesidad demostrada.

### LD-011 — Lexeme se representa como borrowed `&str` y no sustituye a Source Span

Status: CLOSED

`Lexeme` y `Source Span` describen dos propiedades diferentes de la misma unidad lexical:

```text
Lexeme
    = qué texto fue reconocido

Source Span
    = dónde fue reconocido
```

La representación Rust de `Lexeme` es una vista prestada directa sobre el `Source Text`:

```rust
&'source str
```

No se introduce un `struct Lexeme` ni ownership textual independiente porque `&str` expresa completamente la vista requerida.

Para cada `Token` debe cumplirse conceptualmente:

```text
Token.lexeme
==
&SourceText[Token.span.start .. Token.span.end]
```

Invariantes:

- `Lexeme` borrows del `Source Text` original;
- `Lexeme` no copia ni decodifica el contenido;
- el lexema conserva exactamente la forma textual reconocida, incluidos delimitadores y escapes cuando forman parte del token;
- un `StringLiteral` conserva en su lexema las comillas y secuencias de escape escritas en source; el Lexer no las convierte todavía al valor semántico final;
- un `FloatingLiteral` conserva su representación original, incluida notación científica cuando corresponda;
- mantener simultáneamente `Lexeme` y `Source Span` no se considera duplicación semántica: uno expone contenido y el otro ubicación;
- conservar `Lexeme` evita que el Parser necesite una dependencia adicional hacia `Source Text` únicamente para recuperar el texto de un Token;
- `Source Span` permanece independiente de `Lexeme` y puede representar regiones mayores que un Token en etapas posteriores.

Regla canónica:

```text
Lexeme
    !=
Source Span

content
    !=
location
```

## 3. Closed Token Kind Inventory

```text
Token Kind                          <<enum>>
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
├── Use
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

Total: **51 variantes**.

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
         ├── Token Kind <<enum: 51 variants>>
         ├── Lexeme
         └── Source Span
           │
           ▼
     Token Sequence
           └── Token 0..N
```

Identidades y representaciones cerradas hasta este punto:

- `Token Kind` — enum de clasificación lexical con 51 variantes;
- `Lexeme` — borrowed `&'source str` sobre `Source Text`;
- `Source Span` — `struct` con `start: usize` y `end: usize`, expresados como byte offsets `[start, end)`;
- `Token` — unidad lexical reconocida; representación Rust completa pendiente;
- `Token Sequence` — output lexical 0..N; representación física pendiente.

## 5. Specification Observation

Durante el inventario se detectó una inconsistencia documental en `EVO_SCRIPT_SPECIFICATION_v0.1.md` que no se corrige silenciosamente desde este documento:

- el catálogo léxico declara `artifact` como Structural Keyword exclusiva de archivos `.elib`;
- la sección de Source Encoding enumera varios artefactos de código fuente pero omite `.elib` en esa enumeración.

Esta observación no cambia el inventario de `Token Kind`; debe resolverse explícitamente en la especificación de `evo-script` si se decide corregirla.

## 6. Next Technical Data Decision

```text
Token Kind              ✅ CLOSED
Source Span             ✅ CLOSED
Lexeme representation   ✅ CLOSED
Token                    ← NEXT
Token Sequence           PENDING
```

La siguiente decisión del bloque lexical debe concretar `Token` como estructura Rust completa a partir de las identidades ya cerradas, sin reabrir `Token Kind`, `Source Span` o `Lexeme` salvo que aparezca una contradicción técnica demostrable.
