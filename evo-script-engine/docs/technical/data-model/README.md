# Evo-Script Engine — Technical Data Model

Status: TECHNICAL DATA MODEL — IN PROGRESS

Este directorio contiene el Technical Data Model de `evo-script-engine` y sus Technical Data Diagrams.

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

Compiled Program / Bytecode Data ✅ CLOSED
├── compiled responsibility      ✅ CLOSED
├── FunctionId preservation      ✅ CLOSED
├── ConstantId                   ✅ CLOSED
├── ExternalSymbolId             ✅ CLOSED
├── Signature Dependency erasure ✅ CLOSED
├── external call convergence    ✅ CLOSED
├── CompiledProgram              ✅ CLOSED
├── CompiledFunction             ✅ CLOSED
├── ParameterSlot / LocalSlot    ✅ CLOSED
├── Constant / DynamicConstant   ✅ CLOSED
├── ExternalSymbol + arity       ✅ CLOSED
├── core Load / Store            ✅ CLOSED
├── internal / external Calls    ✅ CLOSED
├── NumericKind                  ✅ CLOSED — 12 variants
├── fixed arithmetic/comparison  ✅ CLOSED
├── dynamic numeric lifting      ✅ CLOSED
├── dynamic arithmetic           ✅ CLOSED
├── Instruction typed enum       ✅ CLOSED — 48 variants
├── InstructionIndex             ✅ CLOSED
├── control flow / short-circuit ✅ CLOSED
├── conversions                  ✅ CLOSED
├── bool equality / negation     ✅ CLOSED
├── string equality              ✅ CLOSED
├── FieldIndex                   ✅ CLOSED
├── VariantDiscriminant          ✅ CLOSED
├── Composite Layout             ✅ CLOSED
├── Struct / Enum Instructions   ✅ CLOSED
├── EqualityComparable           ✅ CLOSED
├── Structural Equality          ✅ CLOSED
├── SourceMap                    ✅ CLOSED
└── exact compiled inventory     ✅ CLOSED — 18 identities

VM Execution Data                ← IN ANALYSIS
├── VmExecution root             ✅ CLOSED
├── one invocation lifetime      ✅ CLOSED
├── CompiledProgram relationship ✅ CLOSED
├── ApplicationBindings relation ✅ CLOSED — exact model pending
├── Shared Value Storage owner   ✅ CLOSED — representation pending
├── Call Frames owner            ✅ CLOSED — exact model pending
├── execution backing owner      ✅ CLOSED — representation pending
└── Runtime Value Model          ← NEXT

Outcome / Diagnostic Data        PENDING
```

## Current Documents

### Lexical Data

- [`LEXICAL_DATA.md`](./LEXICAL_DATA.md)
- [`TOKEN.md`](./TOKEN.md)
- [`TOKEN_SEQUENCE.md`](./TOKEN_SEQUENCE.md)

### AST Data

- [`AST_DATA.md`](./AST_DATA.md)
- [`AST_TYPE_DEFINITIONS.md`](./AST_TYPE_DEFINITIONS.md)
- [`AST_FUNCTION_DEFINITIONS.md`](./AST_FUNCTION_DEFINITIONS.md)
- [`AST_EXPRESSION_REPRESENTATION.md`](./AST_EXPRESSION_REPRESENTATION.md)
- [`AST_EXPRESSIONS.md`](./AST_EXPRESSIONS.md)
- [`AST_WHEN.md`](./AST_WHEN.md)
- [`AST_INVENTORY.md`](./AST_INVENTORY.md)

### Semantic Program Data

- [`SEMANTIC_PROGRAM_DATA.md`](./SEMANTIC_PROGRAM_DATA.md) — reglas base y cierre.
- [`SEMANTIC_PROGRAM_STRUCTURE.md`](./SEMANTIC_PROGRAM_STRUCTURE.md) — owners, `SignatureSymbol`, types, signatures y functions.
- [`SEMANTIC_EXPRESSIONS.md`](./SEMANTIC_EXPRESSIONS.md) — body, expressions, calls, conversions, constructions, `when`, Pipeline lowering y SourceSpan.
- [`SEMANTIC_PROGRAM_INVENTORY.md`](./SEMANTIC_PROGRAM_INVENTORY.md) — inventario exacto de 33 identities y cobertura AST → Semantic Program.

### Compiled Program / Bytecode Data

- [`COMPILED_PROGRAM_DATA.md`](./COMPILED_PROGRAM_DATA.md) — autoridad raíz del producto compilado, ahora CLOSED.
- [`COMPILED_PROGRAM_INVENTORY.md`](./COMPILED_PROGRAM_INVENTORY.md) — inventario exacto: 18 identities propias, 48 Instruction variants y cobertura Semantic → Compiled.
- [`COMPILED_STORAGE_DATA.md`](./COMPILED_STORAGE_DATA.md) — `ParameterSlot`, `LocalSlot`, `ExternalSymbol.parameter_count`, Constant Pool canonicalizado y `DynamicConstant`.
- [`COMPILED_CORE_CALL_INSTRUCTIONS.md`](./COMPILED_CORE_CALL_INSTRUCTIONS.md) — `LoadConstant`, `LoadParameter`, `LoadLocal`, `StoreLocal`, `Call`, `CallExternal` y calling convention física.
- [`COMPILED_NUMERIC_INSTRUCTIONS.md`](./COMPILED_NUMERIC_INSTRUCTIONS.md) — `NumericKind`, fixed arithmetic/comparison, `LiftDynamic` y dynamic arithmetic.
- [`COMPILED_CONTROL_FLOW.md`](./COMPILED_CONTROL_FLOW.md) — `InstructionIndex`, branching, short-circuit, `Discard` y `Return`.
- [`COMPILED_CONVERSIONS.md`](./COMPILED_CONVERSIONS.md) — fixed/dynamic numeric conversions y string conversion.
- [`COMPILED_SCALAR_EQUALITY.md`](./COMPILED_SCALAR_EQUALITY.md) — bool negation/equality y string equality.
- [`COMPILED_COMPOSITE_LAYOUT.md`](./COMPILED_COMPOSITE_LAYOUT.md) — `FieldIndex`, `VariantDiscriminant` y canonical composite layout.
- [`COMPILED_COMPOSITE_INSTRUCTIONS.md`](./COMPILED_COMPOSITE_INSTRUCTIONS.md) — struct/enum construction, access, variant testing, payload extraction y `when` lowering.
- [`COMPILED_STRUCTURAL_EQUALITY.md`](./COMPILED_STRUCTURAL_EQUALITY.md) — `EqualityRule`, `CompositeEqualityPlan` y struct/enum structural equality.
- [`COMPILED_SOURCE_MAP.md`](./COMPILED_SOURCE_MAP.md) — dense `(FunctionId, InstructionIndex) → SourceSpan` mapping y future multi-source seam.

### VM Execution Data

- [`VM_EXECUTION_DATA.md`](./VM_EXECUTION_DATA.md) — autoridad acumulada de VM Execution Data; `VmExecution` root, invocation lifetime, Shared Value Storage owner, Call Frames owner y execution-lifetime backing ownership.

### Normative language amendments used by compiled model

- [`../../../../evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`](../../../../evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md)
- [`../../../../evo-script/COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md`](../../../../evo-script/COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md)
- [`../../../../evo-script/EFN_HOST_BOUNDARY_v0.1.md`](../../../../evo-script/EFN_HOST_BOUNDARY_v0.1.md)
- [`../../../../evo-script/EFN_TYPE_CARDINALITY_v0.1.md`](../../../../evo-script/EFN_TYPE_CARDINALITY_v0.1.md)

## Technical Data Diagram Rule

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

## Next Block

```text
Runtime Value Model ← NEXT
```

Aquí se definirá qué es un `Value` runtime, cómo representa fixed numeric / bool / string / dynamic / struct / enum y cómo se separan Value views de execution-owned backing data sin introducir ownership o lifetimes accidentales.
