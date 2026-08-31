# Evo-Script Engine — Compile Participant Design

Status: COMPILE PARTICIPANT DESIGN — IN PROGRESS

Este documento cierra progresivamente las Rust Signatures y Participants internos requeridos por los Use Cases `Compile` y `ExecuteSource` para la fase de compilación.

La autoridad deriva de:

- `ROOT_SIGNATURE_DESIGN.md`;
- `TECHNICAL_DESIGN.md`;
- `docs/technical/data-model/`;
- `TECHNICAL_DESIGN_METHODOLOGY.md`;
- `ENGINEERING_PRINCIPLES.md`.

Los nombres de artifacts, Participants y conceptos técnicos canónicos se mantienen en English; las explicaciones, decisiones, reglas e invariantes se redactan en español.

## Compile participant tree

La dirección raíz cerrada es:

```text
Compile Agent
├── lex_source
├── parse_tokens
├── analyze_program
└── lower_program
```

`Compile Agent` coordina directamente estas responsabilidades internas significativas. No se introduce un mega-Collaborator que coordine otros Collaborators.

## RSD-011 — Una Tool no es una función auxiliar

Status: CLOSED

Una función auxiliar interna no se convierte en una Tool arquitectónica únicamente por ser pequeña.

Una Tool representa una operación interna pequeña, genérica y semánticamente independiente del Participant que la utiliza.

Debe poder reutilizarse sin depender de la responsabilidad interna específica del Participant consumidor.

Que una Tool tenga actualmente uno o varios consumidores no determina por sí mismo su naturaleza arquitectónica.

Regla de clasificación:

```text
operación pertenece a la responsabilidad interna del Participant
    → función/mecanismo privado de implementación

operación pequeña + genérica + semánticamente independiente
    → candidata a Tool
```

No se promueven funciones privadas a Participants únicamente para fragmentar una implementación.

## RSD-012 — Firma exacta de `lex_source`

Status: CLOSED

`lex_source` es un Collaborator interno de compilación.

Firma cerrada:

```rust
pub type Lex =
    for<'source> fn(
        &'source str,
    ) -> Result<
        TokenSequence<'source>,
        CompileFailure,
    >;
```

Responsabilidad:

```text
Source Text
    ↓
lex_source
    ├── success → TokenSequence<'source>
    └── failure → CompileFailure {
                     kind: Lexical(...),
                     source_span,
                 }
```

Invariantes:

- recibe únicamente `Source Text`;
- no recibe `CompilationCatalog`;
- no recibe `ApplicationBindings`;
- no cruza fronteras técnicas externas;
- en éxito materializa `TokenSequence<'source>`;
- el resultado conserva borrow hacia el `Source Text` mediante los lexemes de `Token<'source>`;
- todo failure normal pertenece exclusivamente a `CompileFailureKind::Lexical(...)`;
- `lex_source` materializa directamente `CompileFailure` porque posee la información necesaria para conservar provenance mediante `SourceSpan`;
- no se introduce un error intermedio `LexerError`, `LocatedLexicalFailure` o equivalente sin semántica propia.

## RSD-013 — `lex_source` no requiere Tools arquitectónicas en v0

Status: CLOSED

Las operaciones necesarias para reconocer identifiers, literals, reserved forms, operators, structural symbols, whitespace y comments pertenecen a la responsabilidad interna del Lexer.

Su posible separación en funciones, métodos, scanner state o estructuras privadas es una decisión de implementación y no crea Participants arquitectónicos.

También pertenece a la implementación lexical la construcción de `Token` que preserve el invariante:

```text
Token.lexeme
==
&SourceText[Token.span.start .. Token.span.end]
```

No se introducen en v0:

```text
TokenFactory
TokenBuilder
IdentifierScanner Tool
NumericScanner Tool
StringScanner Tool
KeywordClassifier Tool
```

mientras no aparezca una responsabilidad pequeña, genérica e independiente del Lexer que justifique dicha identidad.

Inventario arquitectónico cerrado para `lex_source`:

```text
Collaborator   1  lex_source
Contract       0
Resolver       0
Requester      0
Tool           0
```

Esto no prescribe una función monolítica. El Collaborator puede poseer tantos mecanismos privados como requiera una implementación clara y correcta.

## Compile participant progress

```text
Compile Agent
├── lex_source          ✅ CLOSED
├── parse_tokens        ← NEXT
├── analyze_program     PENDING
└── lower_program       PENDING
```

## Closure parcial

```text
RSD-011 Tool classification rule        ✅ CLOSED
RSD-012 lex_source exact signature      ✅ CLOSED
RSD-013 lex_source Tool inventory       ✅ CLOSED

Compile participant design              ← IN PROGRESS
Execution participant design            PENDING
Module Signature Diagrams               AFTER PARTICIPANTS
D2 Sequence Diagrams                    AFTER SIGNATURES/PARTICIPANTS
Implementation Tasks                    AFTER DIAGRAMS
```
