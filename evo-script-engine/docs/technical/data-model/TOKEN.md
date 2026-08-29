# Evo-Script Engine — Token

Status: TOKEN — CLOSED

Este documento cierra la representación técnica de `Token` dentro del Technical Data Model de `evo-script-engine`.

Los nombres de artefactos y conceptos técnicos canónicos se mantienen en inglés; las reglas, decisiones e invariantes se documentan en español.

## 1. Responsibility

`Token` representa exactamente una ocurrencia léxica reconocida por el Lexer dentro de un `Source Text`.

No resuelve significado semántico. La interpretación de funciones, tipos, bindings, parámetros, firmas, variantes u otras identidades corresponde a etapas posteriores.

## 2. Rust Representation

Representación técnica cerrada:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Token<'source> {
    kind: TokenKind,
    lexeme: &'source str,
    span: SourceSpan,
}
```

La visibilidad concreta de `Token` y de sus campos no se cierra todavía. Esa decisión pertenece al diseño de módulos y firmas.

## 3. Lifetime

`Token` posee el lifetime explícito `'source` porque `lexeme` es una borrowed view del `Source Text` original.

```text
Source Text lifetime
──────────────────────────────

Token<'source>
      ──────────────────
```

Invariante:

> Un `Token<'source>` nunca puede sobrevivir al `Source Text` del que deriva su `lexeme`.

`Token` no posee backing textual ni prolonga artificialmente el lifetime mediante `String`, `Arc<str>`, intern pools u otros mecanismos no justificados.

## 4. Fields

### `kind: TokenKind`

Clasifica la forma léxica reconocida conforme al inventario cerrado de `Token Kind`.

No contiene payload textual y no resuelve significado semántico.

### `lexeme: &'source str`

Es la vista textual exacta reconocida dentro del `Source Text`.

No copia ni posee texto.

### `span: SourceSpan`

Identifica exactamente la región física `[start, end)` del `Source Text` donde aparece el lexema.

## 5. Canonical Invariant

Para todo `Token` válido producido por el Lexer debe cumplirse conceptualmente:

```text
Token.lexeme
==
&SourceText[Token.span.start .. Token.span.end]
```

Por tanto:

```text
Lexeme
    = contenido reconocido

Source Span
    = ubicación reconocida
```

Ambos datos son complementarios y no sustituyen uno al otro.

Para un `Token` normal se cumple además:

```text
span.start < span.end
```

Un `Token` nunca representa una región vacía del source.

## 6. Ownership

`Token` no posee recursos externos ni backing data.

```text
Source Text
    │ owns UTF-8 text
    │
    └── Token<'source>
          └── lexeme borrows
```

Copiar un `Token` no copia el `Source Text` ni el contenido del lexema; únicamente copia el descriptor técnico y la referencia prestada.

## 7. Copy and Equality Semantics

`Token` puede implementar:

```rust
Debug
Clone
Copy
PartialEq
Eq
```

porque sus componentes técnicos son compatibles con esas propiedades y ninguna implica ownership artificial.

`Copy` expresa que duplicar el descriptor del Token es seguro y barato desde el punto de vista de ownership. No obliga al Parser a copiar Tokens; el Parser puede seguir operando mediante referencias o índices dentro de `Token Sequence`.

La igualdad estructural incluye:

```text
kind
lexeme
span
```

Dos ocurrencias con el mismo `kind` y `lexeme` pero distinto `Source Span` son Tokens distintos.

## 8. Traits Not Justified

No se introducen todavía:

```text
Default
Hash
Ord
PartialOrd
```

Razones:

- no existe un `Token` válido por defecto;
- no se ha demostrado necesidad de hashing;
- no existe una semántica arquitectónica de orden total entre Tokens distinta de su posición dentro de `Token Sequence`.

Estas propiedades solo podrán añadirse si una necesidad posterior las justifica.

## 9. Construction Boundary

El Technical Data Model cierra el invariante de construcción, pero no prescribe todavía el mecanismo concreto para producir `Token`.

Rust puede garantizar el lifetime de `lexeme`, pero los campos por sí solos no garantizan que `lexeme` corresponda exactamente al mismo rango indicado por `span`.

Por tanto, toda construcción válida debe preservar:

```text
lexeme == SourceText[span]
```

La decisión de si esto se protege mediante una función privada, Tool, constructor de módulo u otra forma pertenece al diseño posterior de firmas y módulos.

No se introduce artificialmente un método o constructor en esta fase.

## 10. Architectural Lifetime

`Token` pertenece principalmente a la frontera:

```text
Lexer
    ↓
Token Sequence<'source>
    ↓
Parser
```

No se asume que `Semantic Program` conserve Tokens completos.

Etapas posteriores pueden preservar `Source Span` u otros datos necesarios sin transportar `Token` indefinidamente por el pipeline.

## 11. Closed Invariants

1. `Token<'source>` representa una única ocurrencia léxica reconocida.
2. `Token` contiene exactamente `TokenKind`, `&'source str` y `SourceSpan`.
3. `lexeme` borrows del `Source Text` correspondiente.
4. El lifetime de `Token` no puede exceder `'source`.
5. `kind` clasifica forma léxica sin resolver significado semántico.
6. `span` identifica exactamente la región física del lexema.
7. `lexeme == &SourceText[span.start..span.end]` para todo Token válido.
8. Para Tokens normales, `span.start < span.end`.
9. `Token` no posee backing data ni recursos externos.
10. `Token` puede ser `Debug + Clone + Copy + PartialEq + Eq`.
11. `Default`, `Hash`, `Ord` y `PartialOrd` no están justificados en v0.
12. La visibilidad Rust y el mecanismo concreto de construcción se posponen al diseño de módulos y firmas.

## 12. Next Technical Data Decision

`Token` queda cerrado.

El siguiente dato del bloque lexical es `Token Sequence<'source>`.

Debe definirse:

- quién posee el almacenamiento de Tokens;
- qué lifetime conserva;
- qué acceso necesita el Parser;
- si requiere identidad propia como struct/artifact o si una representación de secuencia estándar expresa completamente el dato;
- qué colección Rust, si alguna, está realmente justificada.
