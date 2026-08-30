# Evo-Script Engine — Token Sequence

Status: TOKEN SEQUENCE — CLOSED

Este documento cierra la representación técnica de `Token Sequence` dentro del Compilation Working State de `evo-script-engine` v0.

## 1. Responsibility

`Token Sequence` representa la secuencia completa, ordenada y temporal de `Token<'source>` reconocida por el Lexer a partir de un único `Source Text`.

```text
Source Text
    ↓
Lexer
    ↓
Token Sequence<'source>
    ↓
Parser
```

`Token Sequence` pertenece exclusivamente al proceso de compilación. No forma parte de `Compiled Program` ni de Runtime.

## 2. Parser Lookahead Requirement

El análisis de la gramática de Evo-Script v0.1 demuestra que las decisiones sintácticas revisadas requieren como máximo **un Token futuro de lookahead**.

Esto significa que la gramática no obliga a materializar todos los Tokens para que el Parser pueda funcionar.

La materialización de `Token Sequence` se conserva en v0 por una razón diferente: simplifica la separación y prueba independiente de Lexer y Parser, facilita inspección y diagnóstico del Compilation Working State y mantiene una implementación determinista de bajo riesgo.

Regla:

> `Token Sequence` se materializa por simplicidad del compiler, no porque la gramática exija almacenamiento completo.

## 3. Rust Representation

Representación Rust cerrada:

```rust
type TokenSequence<'source> = Vec<Token<'source>>;
```

No se introduce un wrapper `struct TokenSequence` en v0 porque no existe estado adicional propio que justifique otro nivel de empaquetado. La identidad técnica `Token Sequence` queda expresada por el alias y sus invariantes de uso dentro del pipeline.

La representación puede revisarse posteriormente hacia streaming u otra forma si una medición demuestra beneficio real sin degradar claridad o corrección.

## 4. Ownership and Lifetime

```text
Source Text
    owns UTF-8 text
       ▲
       │ borrowed by lexemes
       │
Token Sequence<'source>
    owns Token descriptors
```

Invariantes:

- `Token Sequence` posee los descriptors `Token` almacenados temporalmente;
- no posee ni copia el `Source Text`;
- cada `Token.lexeme` continúa borrowed del mismo `Source Text`;
- el lifetime de `Token Sequence<'source>` no puede exceder `'source` mientras contiene `Token<'source>`;
- el Parser observa la secuencia lexical como input de parsing y no redefine sus Tokens;
- el estado completo puede liberarse cuando el Parser ha producido el `AST` y ningún diagnóstico pendiente requiere conservarlo.

## 5. Sequence Invariants

Una `Token Sequence<'source>` válida cumple:

1. contiene `0..N Token<'source>`;
2. todos los Tokens derivan del mismo `Source Text`;
3. conserva el orden físico de aparición en el source;
4. para Tokens consecutivos se cumple `current.span.end <= next.span.start`;
5. los spans no se solapan;
6. pueden existir gaps correspondientes a whitespace o comments no materializados;
7. no contiene `Whitespace`, `Comment`, `EndOfFile` ni `Invalid` como Tokens normales;
8. cada Token conserva sus invariantes propias de `TOKEN.md`.

## 6. Compilation Working State

`Vec<Token<'source>>` es aceptado aquí bajo TD-010 porque representa working state temporal de un proceso finito.

```text
Compile
  ↓
Token Sequence      temporary owned working state
  ↓
AST
  ↓
Token Sequence      drop when no longer required
```

Esta decisión no crea una regla general de utilizar `Vec` en Runtime ni en otras etapas. Cada representación debe justificarse por su responsabilidad y lifetime.

## 7. Closure

```text
Token Kind              ✅ CLOSED
Source Span             ✅ CLOSED
Lexeme representation   ✅ CLOSED
Token                    ✅ CLOSED
Token Sequence           ✅ CLOSED

Lexical Data             ✅ CLOSED
AST Data                 ← NEXT
```
