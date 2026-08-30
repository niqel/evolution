# Evo-Script Engine — Technical Data Model

Status: TECHNICAL DATA MODEL — IN PROGRESS

Este directorio contiene el Technical Data Model de `evo-script-engine` y sus Technical Data Diagrams en D2.

## Responsibility

El Technical Data Model transforma Functional Data Dictionary + Technical Design cerrado en representaciones técnicas concretas.

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

## `.efn` / Host Boundary

```text
Host Interactive State
    !=
`.efn` Compile / Execution Data
```

No se introducen `Active Scope`, Host Session State, Current Provider, Use Node, Use Instruction ni `SET_SCOPE` dentro del Technical Data Model de `.efn`.

## Current Progress

```text
Lexical Data                     ✅ CLOSED
├── Token Kind                   ✅ CLOSED — 50 variants
├── Source Span                  ✅ CLOSED
├── Token                        ✅ CLOSED
└── Token Sequence               ✅ CLOSED

AST Data                         ✅ CLOSED
├── syntactic responsibility     ✅ CLOSED
├── Parser/Semantic boundary     ✅ CLOSED
├── Program / types / functions  ✅ CLOSED
├── expression representation    ✅ CLOSED
├── `when`                       ✅ CLOSED
└── exact AST inventory          ✅ CLOSED — 31 identities

Semantic Program Data            ✅ CLOSED
├── semantic responsibility      ✅ CLOSED
├── semantic identity family     ✅ CLOSED
├── identity scopes              ✅ CLOSED
├── owner-index rule             ✅ CLOSED
├── SemanticProgram root         ✅ CLOSED
├── SemanticType / variants      ✅ CLOSED
├── SemanticSignature            ✅ CLOSED
├── SignatureSymbol              ✅ CLOSED
├── SemanticFunction             ✅ CLOSED
├── Semantic body / expressions  ✅ CLOSED
├── resolved calls / arguments   ✅ CLOSED
├── language conversions         ✅ CLOSED
├── arbitrary integer literals   ✅ CLOSED
├── constructions / `when`       ✅ CLOSED
├── Pipeline semantic lowering   ✅ CLOSED
└── exact semantic inventory     ✅ CLOSED — 33 identities

Compiled Program / Bytecode Data ← IN ANALYSIS
├── compiled responsibility      ✅ CLOSED
├── FunctionId preservation      ✅ CLOSED
├── ConstantId                   ✅ CLOSED
├── ExternalSymbolId             ✅ CLOSED
├── Signature Dependency erasure ✅ CLOSED
├── external call convergence    ✅ CLOSED
├── CompiledProgram root shell   ✅ CLOSED
├── CompiledFunction shell       ✅ CLOSED
├── ParameterSlot                ✅ CLOSED
├── LocalSlot                    ✅ CLOSED
├── Constant / DynamicConstant   ✅ CLOSED
├── ExternalSymbol               ✅ CLOSED
├── NumericKind                  ✅ CLOSED
├── fixed arithmetic/comparison  ✅ CLOSED
├── dynamic numeric lifting      ✅ CLOSED
├── dynamic arithmetic           ✅ CLOSED
├── Instruction typed enum       ✅ CLOSED
├── InstructionIndex             ✅ CLOSED
├── control flow / short-circuit ✅ CLOSED
├── conversions                  ✅ CLOSED
├── bool equality / negation     ✅ CLOSED
├── string equality              ✅ CLOSED
├── FieldIndex                   ✅ CLOSED
├── VariantDiscriminant          ✅ CLOSED
├── Composite Layout             ✅ CLOSED
└── Struct / Enum Instructions   ← NEXT

VM Execution Data                PENDING
Outcome / Diagnostic Data        PENDING
```

## Current Documents

Lexical Data:

- [`LEXICAL_DATA.md`](./LEXICAL_DATA.md)
- [`TOKEN.md`](./TOKEN.md)
- [`TOKEN_SEQUENCE.md`](./TOKEN_SEQUENCE.md)

AST Data:

- [`AST_DATA.md`](./AST_DATA.md)
- [`AST_TYPE_DEFINITIONS.md`](./AST_TYPE_DEFINITIONS.md)
- [`AST_FUNCTION_DEFINITIONS.md`](./AST_FUNCTION_DEFINITIONS.md)
- [`AST_EXPRESSION_REPRESENTATION.md`](./AST_EXPRESSION_REPRESENTATION.md)
- [`AST_EXPRESSIONS.md`](./AST_EXPRESSIONS.md)
- [`AST_WHEN.md`](./AST_WHEN.md)
- [`AST_INVENTORY.md`](./AST_INVENTORY.md)

Semantic Program Data:

- [`SEMANTIC_PROGRAM_DATA.md`](./SEMANTIC_PROGRAM_DATA.md) — reglas base y cierre.
- [`SEMANTIC_PROGRAM_STRUCTURE.md`](./SEMANTIC_PROGRAM_STRUCTURE.md) — owners, `SignatureSymbol`, types, signatures y functions.
- [`SEMANTIC_EXPRESSIONS.md`](./SEMANTIC_EXPRESSIONS.md) — body, expressions, calls, conversions, constructions, `when`, Pipeline lowering y SourceSpan.
- [`SEMANTIC_PROGRAM_INVENTORY.md`](./SEMANTIC_PROGRAM_INVENTORY.md) — segunda revisión, cobertura AST → Semantic Program, inventario exacto de 33 identidades y suficiencia para Bytecode Compiler.

Compiled Program / Bytecode Data:

- [`COMPILED_PROGRAM_DATA.md`](./COMPILED_PROGRAM_DATA.md) — autoridad base y estado acumulado del producto compilado.
- [`COMPILED_STORAGE_DATA.md`](./COMPILED_STORAGE_DATA.md) — `ParameterSlot`, `LocalSlot`, `Constant`, `DynamicConstant`, `ExternalSymbol`, Constant Pool policy y mapping temporal de bindings.
- [`COMPILED_NUMERIC_INSTRUCTIONS.md`](./COMPILED_NUMERIC_INSTRUCTIONS.md) — `NumericKind`, fixed arithmetic/comparison, `LiftDynamic`, dynamic arithmetic, errores de evaluación y fronteras de lowering.
- [`COMPILED_CONTROL_FLOW.md`](./COMPILED_CONTROL_FLOW.md) — typed `Instruction`, `InstructionIndex`, absolute branches, `Jump`, `JumpIfFalse`, short-circuit `&&` / `||`, `Discard` y `Return`.
- [`COMPILED_CONVERSIONS.md`](./COMPILED_CONVERSIONS.md) — fixed/dynamic numeric conversions, exact representability, `ConversionError`, `NumericToString` y `DynamicToString`.
- [`COMPILED_SCALAR_EQUALITY.md`](./COMPILED_SCALAR_EQUALITY.md) — `NotBoolean`, bool equality y string equality; structural equality queda pendiente de Composite Layout.
- [`COMPILED_COMPOSITE_LAYOUT.md`](./COMPILED_COMPOSITE_LAYOUT.md) — `FieldIndex`, `VariantDiscriminant`, canonical composite ordering, struct/enum conceptual runtime layout y ausencia de runtime type-layout tables en v0.

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
