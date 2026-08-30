# Evo-Script Engine — Technical Data Model

Status: TECHNICAL DATA MODEL — IN PROGRESS

Este directorio contiene el Technical Data Model de `evo-script-engine` y sus Technical Data Diagrams en D2.

## Responsibility

El Technical Data Model transforma Functional Data Dictionary + Technical Design cerrado en representaciones técnicas concretas.

Aquí se definen, cuando corresponda:

- structs;
- enums;
- owned artifacts;
- borrowed views;
- semantic aliases;
- ownership / borrowing / lifetimes;
- cardinalidades y relaciones;
- datos internos necesarios por Rust Signatures y Participants.

Regla:

> Toda estructura, enum, artifact o dato interno necesario para expresar una Rust Signature o implementar un Participant debe estar definido previamente en el Technical Data Model.

## Definition Order

```text
Source Text
    ↓
Lexical Data
    ↓
AST Data
    ↓
Semantic Program Data
    ↓
Compiled Program / Bytecode Data
    ↓
VM Execution Data
    ↓
Outcome / Diagnostic Data
```

El orden no implica que todo concepto necesite tipo independiente. Cada identidad requiere justificación real.

## `.efn` / Host Boundary

El modelo técnico aplica `evo-script/EFN_HOST_BOUNDARY_v0.1.md` y TD-011:

```text
Host Interactive State
    !=
`.efn` Compile / Execution Data
```

No se introducirán `Active Scope`, Host Session State, Current Provider, Use Node, Use Instruction ni `SET_SCOPE` dentro del Technical Data Model de `.efn`.

## Current Progress

```text
Lexical Data                     ✅ CLOSED
├── Token structural rules       ✅ CLOSED
├── Token Kind                   ✅ CLOSED — 50 variants
├── Source Span                  ✅ CLOSED
├── Lexeme representation        ✅ CLOSED
├── Token                        ✅ CLOSED
└── Token Sequence               ✅ CLOSED

AST Data                         ✅ CLOSED
├── AST syntactic responsibility ✅ CLOSED
├── Preserve occurrences         ✅ CLOSED
├── Parser/Semantic boundary      ✅ CLOSED
├── No Host Scope / no `use`     ✅ CLOSED
├── `this` parser-only            ✅ CLOSED
├── Pipeline = data composition  ✅ CLOSED
├── foundational syntax data     ✅ CLOSED
├── Program / imports / decls    ✅ CLOSED
├── local type definitions       ✅ CLOSED
├── functions / body             ✅ CLOSED
├── expression representation    ✅ CLOSED — typed nested tree
├── expression inventory         ✅ CLOSED
├── `when` model                  ✅ CLOSED
└── exact AST inventory           ✅ CLOSED — 31 identities

Semantic Program Data            ← IN ANALYSIS
├── semantic responsibility      ✅ CLOSED
├── no name re-resolution        ✅ CLOSED
├── TypeId                       ✅ CLOSED
├── FunctionId                   ✅ CLOSED
├── BindingId                    ✅ CLOSED
├── FieldId                      ✅ CLOSED
├── VariantId                    ✅ CLOSED
├── SignatureId                  ✅ CLOSED
├── SignatureBindingId           ✅ CLOSED
├── identity scopes              ✅ CLOSED
├── no ExternalSymbolId here     ✅ CLOSED
└── semantic owner structures    ← IN ANALYSIS

Compiled Program / Bytecode Data PENDING
VM Execution Data                PENDING
Outcome / Diagnostic Data        PENDING
```

## Current Documents

Lexical Data:

- [`LEXICAL_DATA.md`](./LEXICAL_DATA.md) — lexical identities, Token Kind, Source Span y Lexeme.
- [`TOKEN.md`](./TOKEN.md) — representación e invariantes de `Token<'source>`.
- [`TOKEN_SEQUENCE.md`](./TOKEN_SEQUENCE.md) — `Vec<Token<'source>>` temporal bajo Compilation Working State.

AST Data:

- [`AST_DATA.md`](./AST_DATA.md) — decisiones base de responsabilidad sintáctica, occurrence preservation, Host exclusion, `this`, Pipeline y top-level Program model.
- [`AST_TYPE_DEFINITIONS.md`](./AST_TYPE_DEFINITIONS.md) — `StructDefinition`, `FieldDefinition`, `EnumDefinition`, `EnumVariant` y cardinalidades cerradas.
- [`AST_FUNCTION_DEFINITIONS.md`](./AST_FUNCTION_DEFINITIONS.md) — `FunctionDefinition`, `Parameter`, `FunctionBody`, `BodyStatement`, `LetBinding`, `OperationStatement` y `return` parser-only.
- [`AST_EXPRESSION_REPRESENTATION.md`](./AST_EXPRESSION_REPRESENTATION.md) — typed nested tree, `Box<Expression>` solo para recursión directa, `Vec` para colecciones y exclusión de ExpressionId/AST Arena en v0.
- [`AST_EXPRESSIONS.md`](./AST_EXPRESSIONS.md) — `Expression`, operators, FunctionCall, FieldInitializer, constructions, Pipeline, `when` variant y forma final de OperationStatement.
- [`AST_WHEN.md`](./AST_WHEN.md) — `WhenExpression`, `WhenCorrespondence`, `WhenPattern`, `PatternField` y frontera Parser/Semantic Analyzer para exhaustividad y bindings.
- [`AST_INVENTORY.md`](./AST_INVENTORY.md) — consolidación final del inventario exacto de 31 identidades AST y cierre de `AST Data` v0.

Semantic Program Data:

- [`SEMANTIC_PROGRAM_DATA.md`](./SEMANTIC_PROGRAM_DATA.md) — responsabilidad de Semantic Program; `TypeId`, `FunctionId`, `BindingId`, `FieldId`, `VariantId`, `SignatureId`, `SignatureBindingId`; scopes de identidad y separación frente a layout/runtime binding.

## Technical Data Diagram

El Technical Data Diagram representa datos y artifacts, no behavioral classes.

Categorías posibles:

```text
<<struct>>
<<enum>>
<<artifact>>
<<borrowed view>>
<<alias>>
```

Relaciones posibles:

```text
contains
references
borrows
owns
variant-of
0..1
0..N
```

No se representan methods, inheritance, service classes ni OO interfaces ficticias.

La metodología global se define en [`TECHNICAL_DESIGN_METHODOLOGY.md`](../../../../TECHNICAL_DESIGN_METHODOLOGY.md).
