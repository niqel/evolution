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

## RSD-014 — Firma exacta de `parse_tokens`

Status: CLOSED

`parse_tokens` es un Collaborator interno de compilación responsable de transformar una `TokenSequence<'source>` léxicamente válida en un `Program<'source>` estructuralmente válido.

Firma cerrada:

```rust
pub type Parse =
    for<'source> fn(
        &TokenSequence<'source>,
        &'source str,
    ) -> Result<
        Program<'source>,
        CompileFailure,
    >;
```

El primer argumento es el working input que Parser interpreta. El segundo argumento es el `Source Text` original correspondiente a esa misma `TokenSequence` y existe únicamente como dependencia explícita de provenance fuente cuando la posición responsable no puede recuperarse de un Token existente.

Caso determinante:

```text
Source Text = "   // comment final"
TokenSequence = []

MissingPublicFunction
    → SourceSpan [source_len, source_len)
```

Como `TokenSequence` no materializa `EOF`, whitespace ni comments, la longitud total del Source Text no puede derivarse correctamente desde los Tokens. `Diagnostic Provenance` exige que ausencias detectadas en EOF se materialicen exactamente en `[source_len, source_len)`.

Invariantes:

- `TokenSequence` y `Source Text` corresponden al mismo source coordinate space;
- Parser no vuelve a tokenizar, escanear ni reinterpretar lexicalmente el `Source Text`;
- el acceso al `Source Text` no convierte parsing en una segunda fase lexical;
- Parser utiliza los Tokens como autoridad de reconocimiento lexical;
- `Source Text` aporta únicamente la extensión/coordenadas fuente necesarias para provenance que no exista en un Token materializado;
- en éxito produce `Program<'source>`;
- todo failure normal pertenece exclusivamente a `CompileFailureKind::Syntax(...)`;
- Parser materializa directamente `CompileFailure` con el `SourceSpan` responsable;
- no se introduce un error intermedio `ParserError`, `LocatedSyntaxFailure` o equivalente sin semántica propia.

## RSD-015 — Ownership y Participants de `parse_tokens`

Status: CLOSED

Parser observa `TokenSequence<'source>` mediante borrow y materializa un AST owned como Compilation Working State.

```text
Source Text
    ▲
    │ borrowed lexemes
    │
TokenSequence<'source>
    │ observed by Parser
    ▼
Program<'source>
    ├── owns AST containers / tree structure
    └── borrows textual lexemes from Source Text
```

El AST no borrowea el almacenamiento del `Vec<Token<'source>>`.

Por tanto:

```text
parse_tokens success
    ↓
Program<'source>
    ↓
TokenSequence puede destruirse
    ↓
Program continúa válido mientras Source Text siga vivo
```

Las operaciones internas de grammar navigation, lookahead, precedence, grouping, construcción de expressions, validación de cardinalidades estructurales y manejo del cursor pertenecen a la implementación privada del Parser.

No se promueven automáticamente a Tools o Collaborators independientes.

Inventario arquitectónico cerrado para `parse_tokens`:

```text
Collaborator   1  parse_tokens
Contract       0
Resolver       0
Requester      0
Tool           0
```

`parse_tokens` tampoco recibe `CompilationCatalog` ni `ApplicationBindings`; identity resolution, type resolution y signature resolution pertenecen a `analyze_program`.

## Compile participant progress

```text
Compile Agent
├── lex_source          ✅ CLOSED
├── parse_tokens        ✅ CLOSED
├── analyze_program     ← NEXT
└── lower_program       PENDING
```

## Closure parcial

```text
RSD-011 Tool classification rule        ✅ CLOSED
RSD-012 lex_source exact signature      ✅ CLOSED
RSD-013 lex_source Tool inventory       ✅ CLOSED
RSD-014 parse_tokens exact signature    ✅ CLOSED
RSD-015 parse_tokens ownership/inventory✅ CLOSED

Compile participant design              ← IN PROGRESS
Execution participant design            PENDING
Module Signature Diagrams               AFTER PARTICIPANTS
D2 Sequence Diagrams                    AFTER SIGNATURES/PARTICIPANTS
Implementation Tasks                    AFTER DIAGRAMS
```
