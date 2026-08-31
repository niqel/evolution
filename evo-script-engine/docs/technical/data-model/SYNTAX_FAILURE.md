# Evo-Script Engine — SyntaxFailure

Status: CLOSED

Este documento cierra la familia técnica exacta `SyntaxFailure` para `evo-script-engine` v0.

`SyntaxFailure` pertenece exclusivamente a la responsabilidad del Parser y expresa invalidez estructural que puede confirmarse a partir de una `TokenSequence` léxicamente válida, antes de identity resolution o type resolution.

## Canonical shape

```rust
enum SyntaxFailure {
    MalformedDeclaration,
    MalformedExpression,
    InvalidImportPlacement,
    MissingFinalReturn,
    InvalidReturnPlacement,
    MissingPublicFunction,
    MultiplePublicFunctions,
    EmptyEnum,
    InvalidOperationStatement,
    InvalidThisUsage,
}
```

Inventario exacto:

```text
SyntaxFailure variants = 10
```

## SF-001 — Exactly ten syntax failure variants

Status: CLOSED

`SyntaxFailure` posee exactamente diez variants:

```text
MalformedDeclaration
MalformedExpression
InvalidImportPlacement
MissingFinalReturn
InvalidReturnPlacement
MissingPublicFunction
MultiplePublicFunctions
EmptyEnum
InvalidOperationStatement
InvalidThisUsage
```

## SF-002 — Parser structural responsibility only

Status: CLOSED

`SyntaxFailure` representa únicamente estructura inválida derivable de una `TokenSequence` léxicamente válida.

No contiene failures de:

```text
identity resolution
type resolution
signature resolution
field/variant semantic lookup
semantic graph validation
```

La frontera permanece:

```text
TokenSequence
    ↓
Parser
    ├── SyntaxFailure
    └── AST
```

## SF-003 — MalformedDeclaration

Status: CLOSED

`MalformedDeclaration` es la failure genérica cuando Parser no puede construir una de las formas legales de declaración de Evo-Script v0.

No se introducen tipos separados por cada production de declaration porque todos expresan la misma responsabilidad estructural.

## SF-004 — MalformedExpression

Status: CLOSED

`MalformedExpression` es la failure genérica cuando Parser no puede construir una `Expression` legal.

Las violaciones particulares de productions internas de Expression no generan automáticamente identities de failure independientes.

## SF-005 — InvalidImportPlacement

Status: CLOSED

`InvalidImportPlacement` es distinta de `MalformedDeclaration` porque un `ImportDeclaration` puede estar localmente bien formado y aun así aparecer después de local declarations.

```text
Program
├── Imports 0..N
└── Declarations 1..N
```

## SF-006 — Exact FunctionBody structural failures

Status: CLOSED

Las failures estructurales específicas de `FunctionBody` son exactamente:

```text
MissingFinalReturn
InvalidReturnPlacement
```

`MissingFinalReturn` significa que no existe el único `return expression;` final requerido.

`InvalidReturnPlacement` cubre una forma de return múltiple, temprana, no final o seguida de statements.

## SF-007 — Exact public-function cardinality failures

Status: CLOSED

La cardinalidad estructural del Program se representa exactamente por:

```text
MissingPublicFunction
MultiplePublicFunctions
```

Parser puede usar working counters temporales, pero esos counters no sobreviven en AST ni en `SyntaxFailure`.

## SF-008 — EmptyEnum is parser-owned

Status: CLOSED

`EmptyEnum` es `SyntaxFailure` porque un `EnumDefinition` v0 requiere `variants 1..N`, mientras Struct fields y Structured Variant fields pueden ser `0..N`.

## SF-009 — InvalidOperationStatement

Status: CLOSED

`InvalidOperationStatement` representa una `Expression` sintácticamente válida utilizada como statement cuando no pertenece a las dos únicas formas permitidas:

```text
FunctionCall
Pipeline
```

No se clasifica como `MalformedExpression`, porque la Expression puede estar bien formada; el problema es su rol estructural dentro de `FunctionBody`.

## SF-010 — InvalidThisUsage

Status: CLOSED

`InvalidThisUsage` es una failure contextual propiedad del Parser.

`this` participa exclusivamente en la sintaxis contextual de Pipeline y debe desaparecer durante parsing exitoso.

No sobreviven al AST:

```text
ThisExpression
ThisNode
ResolvedThis
```

## SF-011 — Stable language meaning, not parser implementation state

Status: CLOSED

`SyntaxFailure` conserva significado estable de estructura del lenguaje, no detalles de implementación del Parser.

No forman parte del modelo canónico v0:

```text
expected TokenKind sets
found Token snapshots
Parser state IDs
grammar-production IDs
TokenSequence borrows
copied Source Text fragments
SyntaxFailure<'source>
```

La provenance fuente permanece separada en la raíz:

```text
CompileFailure.source_span: SourceSpan
```

La autoridad de provenance está en [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md).

## Parser / Semantic Analyzer boundary

Parser conserva como AST, para validación semántica posterior, formas que pueden ser estructuralmente válidas pero semánticamente inválidas:

```text
duplicate functions
duplicate fields
duplicate enum variants
duplicate imports
unknown type names
unknown symbols
unknown enum variants
argument arity/type mismatch
non-exhaustive when
duplicate when correspondence
wrong when payload type
field resolution failures
type cycles
function call cycles
signature resolution/satisfaction failures
naming-convention violations
```

Esas failures pertenecen a `SemanticFailure` cuando el Semantic Analyzer dispone de la información resuelta necesaria.

## Phase boundary

```text
TokenSequence
    ↓
Parser
    ├── Success
    │      ↓
    │     AST
    │
    └── Failure
           ↓
       SyntaxFailure
           ↓
       CompileFailure {
           kind: Syntax(...),
           source_span,
       }
```

## Explicitly not introduced

```text
UnexpectedToken as canonical universal failure
expected-token sets as outcome authority
MalformedStructDeclaration / MalformedFunctionDeclaration families
EmptyStruct
EarlyReturn / MultipleReturns aliases
ExpressionStatement AST
ThisExpression AST
identity/type/signature resolution failures in Parser
SyntaxFailure<'source>
Parser working-state payloads
DiagnosticAnchor
SourceLocation
```

## Closure

```text
SF-001 exactly ten variants                                      ✅ CLOSED
SF-002 Parser structural responsibility only                     ✅ CLOSED
SF-003 MalformedDeclaration                                      ✅ CLOSED
SF-004 MalformedExpression                                       ✅ CLOSED
SF-005 InvalidImportPlacement                                    ✅ CLOSED
SF-006 exact FunctionBody structural failures                    ✅ CLOSED
SF-007 exact public-function cardinality failures                ✅ CLOSED
SF-008 EmptyEnum                                                 ✅ CLOSED
SF-009 InvalidOperationStatement                                 ✅ CLOSED
SF-010 InvalidThisUsage                                          ✅ CLOSED
SF-011 stable language meaning / no parser-state payloads        ✅ CLOSED

SyntaxFailure exact family                                       ✅ CLOSED — 10 variants
SemanticFailure exact family                                     ✅ CLOSED elsewhere
Diagnostic provenance                                            ✅ CLOSED — SourceSpan
```
