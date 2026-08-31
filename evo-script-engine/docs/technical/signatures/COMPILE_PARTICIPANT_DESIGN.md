# Evo-Script Engine — Compile Participant Design

Status: COMPILE PARTICIPANT DESIGN — CLOSED

Este documento cierra las Rust Signatures y Participants internos requeridos por los Use Cases `Compile` y `ExecuteSource` para la fase de compilación.

La autoridad deriva de:

- `ROOT_SIGNATURE_DESIGN.md`;
- `TECHNICAL_DESIGN.md`;
- `docs/technical/data-model/`;
- `TECHNICAL_DESIGN_METHODOLOGY.md`;
- `ENGINEERING_PRINCIPLES.md`.

Los nombres de artifacts, Participants y conceptos técnicos canónicos se mantienen en English; las explicaciones, decisiones, reglas e invariantes se redactan en español.

## Compile participant tree

Árbol cerrado:

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

```rust
pub type Lex =
    for<'source> fn(
        &'source str,
    ) -> Result<
        TokenSequence<'source>,
        CompileFailure,
    >;
```

Invariantes:

- recibe únicamente `Source Text`;
- no recibe `CompilationCatalog` ni `ApplicationBindings`;
- no cruza fronteras técnicas externas;
- en éxito materializa `TokenSequence<'source>`;
- el resultado conserva borrow hacia `Source Text` mediante los lexemes de `Token<'source>`;
- todo failure normal pertenece exclusivamente a `CompileFailureKind::Lexical(...)`;
- `lex_source` materializa directamente `CompileFailure` con `SourceSpan`;
- no existe error intermedio sin semántica propia.

## RSD-013 — `lex_source` no requiere Tools arquitectónicas en v0

Status: CLOSED

El reconocimiento de identifiers, literals, reserved forms, operators, structural symbols, whitespace y comments pertenece a la responsabilidad interna del Lexer.

También pertenece a implementación lexical preservar:

```text
Token.lexeme
==
&SourceText[Token.span.start .. Token.span.end]
```

No se introducen en v0 `TokenFactory`, `TokenBuilder` ni scanner Tools por categoría lexical.

Inventario:

```text
Collaborator   1  lex_source
Contract       0
Resolver       0
Requester      0
Tool           0
```

Esto no prescribe una función monolítica; puede existir cualquier cantidad justificada de funciones, métodos o working state privados.

## RSD-014 — Firma exacta de `parse_tokens`

Status: CLOSED

`parse_tokens` transforma una `TokenSequence<'source>` léxicamente válida en un `Program<'source>` estructuralmente válido.

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

El primer argumento es el working input del Parser. El segundo es el `Source Text` original correspondiente a esa misma `TokenSequence` y existe únicamente como dependencia explícita de provenance cuando la posición responsable no puede recuperarse de un Token existente.

Caso determinante:

```text
Source Text = "   // comment final"
TokenSequence = []

MissingPublicFunction
    → SourceSpan [source_len, source_len)
```

Como `TokenSequence` no materializa `EOF`, whitespace ni comments, la longitud total del Source Text no puede derivarse correctamente desde los Tokens.

Invariantes:

- `TokenSequence` y `Source Text` corresponden al mismo source coordinate space;
- Parser no vuelve a tokenizar, escanear ni reinterpretar lexicalmente el Source Text;
- los Tokens son la autoridad de reconocimiento lexical;
- Source Text aporta únicamente extensión/coordenadas requeridas para provenance no recuperable desde Tokens;
- en éxito produce `Program<'source>`;
- todo failure normal pertenece exclusivamente a `CompileFailureKind::Syntax(...)`;
- Parser materializa directamente `CompileFailure` con `SourceSpan`;
- no existe error intermedio sin semántica propia.

## RSD-015 — Ownership y Participants de `parse_tokens`

Status: CLOSED

Parser observa `TokenSequence<'source>` mediante borrow y materializa un AST owned como Compilation Working State.

```text
Source Text
    ▲
    │ borrowed lexemes
    │
TokenSequence<'source>
    │ observed
    ▼
Program<'source>
    ├── owns AST containers / tree structure
    └── borrows textual lexemes from Source Text
```

El AST no borrowea el almacenamiento de `Vec<Token<'source>>`. Tras success, `TokenSequence` puede destruirse y `Program<'source>` continúa válido mientras `Source Text` siga vivo.

Grammar navigation, lookahead, precedence, grouping, construcción de expressions, validación estructural y cursor son mecanismos privados del Parser.

Inventario:

```text
Collaborator   1  parse_tokens
Contract       0
Resolver       0
Requester      0
Tool           0
```

## RSD-016 — Firma exacta de `analyze_program`

Status: CLOSED

`analyze_program` transforma un `Program<'source>` sintácticamente válido en un `SemanticProgram` completamente resuelto.

```rust
pub type Analyze =
    for<'source> fn(
        &Program<'source>,
        &CompilationCatalog,
    ) -> Result<
        SemanticProgram,
        CompileFailure,
    >;
```

Invariantes:

- `Program<'source>` se observa mediante borrow;
- `CompilationCatalog` es dependencia técnica explícita, borrowed e inmutable;
- el catálogo ya fue construido y validado fuera de `evo-script-engine`;
- no realiza filesystem I/O, module discovery, Provider discovery ni runtime binding;
- todo failure normal pertenece exclusivamente a `CompileFailureKind::Semantic(...)`;
- catalog-construction/integration failures permanecen fuera de `CompileFailure`;
- en éxito todas las identidades necesarias para lowering están resueltas;
- Bytecode Compiler no vuelve a resolver nombres, tipos, imports o Signatures;
- no existe error intermedio sin semántica propia.

## RSD-017 — Ownership y Participants de `analyze_program`

Status: CLOSED

`SemanticProgram` es owned Compilation Working State y no conserva borrows hacia AST ni `CompilationCatalog`.

```text
Program<'source>
      │ observed
      ▼
analyze_program ◄── &CompilationCatalog
      │
      ▼
SemanticProgram
      ├── owns semantic structures
      └── preserves SourceSpan where required
```

Tras success, AST puede destruirse y `CompilationCatalog` deja de ser requerido por lowering.

Symbol collection, name/type resolution, type checking, Signature validation, call-graph validation, composite validation, `when` validation y materialización de semantic identities pertenecen a la responsabilidad completa del Semantic Analyzer. Pueden implementarse mediante múltiples funciones, estructuras o pases privados sin convertirse automáticamente en Collaborators independientes.

Inventario:

```text
Collaborator   1  analyze_program
Contract       0
Resolver       0
Requester      0
Tool           0
```

`CompilationCatalog` no es Contract: es dato técnico validado suministrado explícitamente.

## RSD-018 — Firma exacta de `lower_program`

Status: CLOSED

`lower_program` es un Collaborator interno de compilación responsable de transformar un `SemanticProgram` válido en el `CompiledProgram` ejecutable persistente de v0.

Firma cerrada:

```rust
pub type Lower =
    fn(
        &SemanticProgram,
    ) -> CompiledProgram;
```

Invariantes:

- recibe únicamente un `SemanticProgram` válido;
- no recibe AST, `Source Text`, `CompilationCatalog` ni `ApplicationBindings`;
- no realiza name resolution, type resolution o semantic validation normal;
- produce un `CompiledProgram` owned que puede sobrevivir a todo Compilation Working State;
- transforma semantic identities en mechanisms ejecutables, constants, instructions, external symbols, boundary value shapes y SourceMap según el Technical Data Model;
- no conserva borrow hacia `SemanticProgram`.

## RSD-019 — `lower_program` no posee failure normal

Status: CLOSED

Después de `analyze_program` success, un `SemanticProgram` válido debe poder bajarse a `CompiledProgram`.

Por tanto la firma no retorna `Result`.

No existen como failures normales de lenguaje:

```text
BytecodeFailure
LoweringFailure
CodeGenerationFailure
CompilerInternalFailure
```

Si lowering no puede representar un `SemanticProgram` que satisface los invariantes cerrados, existe una violación interna del compiler, no un `CompileFailure` normal atribuible al Source Text.

Las operaciones internas para construir functions, instructions, constants, external symbols, boundary shapes, slots, source maps o equality plans pertenecen a implementación privada del lowering mientras no aparezca una responsabilidad arquitectónica independiente demostrada.

Inventario:

```text
Collaborator   1  lower_program
Contract       0
Resolver       0
Requester      0
Tool           0
```

## RSD-020 — Orquestación exacta del Compile Agent

Status: CLOSED

El Agent de `Compile` implementa exactamente la firma pública `Compile` y coordina los cuatro Collaborators cerrados.

Flujo conceptual:

```rust
fn compile(
    source: &str,
    catalog: &CompilationCatalog,
) -> CompileOutcome {
    let tokens = lex_source(source)?;
    let program = parse_tokens(&tokens, source)?;
    let semantic_program = analyze_program(&program, catalog)?;
    let compiled_program = lower_program(&semantic_program);
    Ok(compiled_program)
}
```

La representación anterior documenta la orquestación; la implementación concreta deberá enlazar las signatures mediante los módulos/bindings tipados definidos en Module Design.

El Agent:

```text
NO tokeniza
NO parsea gramática
NO resuelve significado
NO genera bytecode
NO llama otro Agent
NO cruza Providers
```

Únicamente coordina el pipeline y propaga el primer `CompileFailure` producido por Lexer, Parser o Semantic Analyzer.

## Compile participant closure

```text
Compile Agent
├── lex_source          ✅ CLOSED
├── parse_tokens        ✅ CLOSED
├── analyze_program     ✅ CLOSED
└── lower_program       ✅ CLOSED
```

Inventario interno de Compile v0:

```text
Use Case        1  Compile
Agent           1  compiler / compile
Collaborators   4  lex_source, parse_tokens, analyze_program, lower_program
Contracts       0
Resolvers       0
Requesters      0
Tools           0
```

`ExecuteSource` reutiliza directamente estas cuatro signatures bajo RSD-010; no invoca `Compile Agent`.

## Closure

```text
RSD-011..RSD-020                    ✅ CLOSED
Compile root signature              ✅ CLOSED
Compile Agent orchestration         ✅ CLOSED
Compile Collaborator signatures     ✅ CLOSED — 4
Compile Contract inventory          ✅ 0
Compile Resolver inventory          ✅ 0
Compile Requester inventory         ✅ 0
Compile Tool inventory              ✅ 0

Compile Participant Design          ✅ CLOSED
Execution Participant Design        ← NEXT
Module Signature Diagrams           AFTER PARTICIPANTS
D2 Sequence Diagrams                AFTER SIGNATURES/PARTICIPANTS
Implementation Tasks                AFTER DIAGRAMS
```
