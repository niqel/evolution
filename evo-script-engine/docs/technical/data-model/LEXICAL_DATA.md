# Evo-Script Engine — Lexical Data

Status: STRUCTURAL RULES — CLOSED

Este documento define las reglas estructurales cerradas del primer bloque del Technical Data Model de `evo-script-engine`: los datos producidos por el Lexer y consumidos por el Parser.

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

No determina todavía si un identificador representa una función, parámetro, binding local, tipo, External Symbol u otra identidad semántica.

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

La representación Rust concreta se define posteriormente dentro del Technical Data Model.

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

La representación fundamental no duplica obligatoriamente `line` y `column` dentro de cada Token.

La información de línea y columna puede derivarse posteriormente para diagnóstico a partir del `Source Text` y del `Source Span`.

Invariantes:

- `Source Span` representa un rango, no solamente un punto;
- sus límites deben permitir identificar inequívocamente la región textual reconocida;
- cuando el span materializa una vista textual UTF-8, sus límites deben corresponder a fronteras válidas del texto;
- el tipo numérico concreto de `start` y `end` se define posteriormente;
- `Source Span` y `Source Location` no se consideran automáticamente el mismo concepto técnico: el primero representa un rango; el segundo puede representar una ubicación concreta de diagnóstico cuando sea necesario.

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

Esto mantiene la separación:

```text
Lexer
    reconoce forma textual

Parser
    construye estructura

Semantic Analyzer
    resuelve significado
```

### LD-006 — Token Sequence contiene 0..N Tokens sin prescribir colección Rust

Status: CLOSED

El output lexical es conceptualmente una `Token Sequence` con cardinalidad `0..N`.

```text
Token Sequence
    └── Token 0..N
```

Esta identidad no prescribe `Vec<Token>` ni otra colección concreta.

La representación física de la secuencia se decidirá únicamente cuando el Technical Data Model demuestre qué operaciones, ownership, cardinalidad y acceso necesita el Parser.

## 3. Current Lexical Data Identities

```text
Source Text
    │ owns textual content
    │
    ├── Lexeme       <<borrowed view>>
    └── Source Span  <<range value>>
             │
             ▼
           Token
             ├── Token Kind
             ├── Lexeme
             └── Source Span
             │
             ▼
       Token Sequence
             └── Token 0..N
```

Identidades estructurales demostradas hasta este punto:

- `Token Kind` — clasificación lexical; variantes pendientes de inventario contra la especificación.
- `Lexeme` — borrowed textual view.
- `Source Span` — rango técnico del source.
- `Token` — unidad lexical reconocida.
- `Token Sequence` — output lexical 0..N.

## 4. Open Lexical Decisions

Las reglas estructurales anteriores están cerradas. Antes de cerrar completamente el bloque lexical deben resolverse contra la especificación Evo-Script v0.1:

```text
Token Kind exact variants          ← NEXT
EOF token                          PENDING
whitespace/comments emission       PENDING
```

No se crearán variantes de `Token Kind` por analogía con Rust u otros lenguajes. Cada variante debe corresponder a una forma lexical requerida por Evo-Script.
