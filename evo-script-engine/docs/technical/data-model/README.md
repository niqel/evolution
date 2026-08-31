# Evo-Script Engine — Technical Data Model

Status: TECHNICAL DATA MODEL — IN PROGRESS

Este directorio contiene el Technical Data Model de `evo-script-engine` y sus Technical Data Diagrams.

## Responsibility

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
Lexical Data                         ✅ CLOSED
├── Token Kind                       ✅ CLOSED — 50 variants
├── Source Span                      ✅ CLOSED
├── Token                            ✅ CLOSED
└── Token Sequence                   ✅ CLOSED

AST Data                             ✅ CLOSED
└── exact AST inventory              ✅ CLOSED — 31 identities

Semantic Program Data                ✅ CLOSED
└── exact semantic inventory         ✅ CLOSED — 33 identities

Compiled Program / Bytecode Data     ✅ CLOSED — REVALIDATED
├── compiled root                    ✅ CLOSED
├── FunctionId preservation          ✅ CLOSED
├── constants / storage              ✅ CLOSED
├── external symbolic calls          ✅ CLOSED
├── NumericKind                      ✅ CLOSED — 12 variants
├── Instruction                      ✅ CLOSED — 48 variants
├── control flow / conversions       ✅ CLOSED
├── composite layout / equality      ✅ CLOSED
├── SourceMap                        ✅ CLOSED
├── CompiledValueShapeId             ✅ CLOSED
├── CompiledValueShape               ✅ CLOSED — 17 variants
├── CompiledEnumValueShape           ✅ CLOSED — 3 variants
├── entry_parameter_shapes           ✅ CLOSED
├── ExternalSymbol.result_shape      ✅ CLOSED
├── boundary validation              ✅ CLOSED — exact / recursive / no coercion
└── exact compiled inventory         ✅ CLOSED — 21 identities

VM Execution Data                    ✅ STRUCTURAL / INVENTORY CLOSED
├── VmExecution responsibility       ✅ CLOSED
├── CompiledProgram relationship     ✅ CLOSED
├── ApplicationBindings              ✅ CLOSED
├── SharedValueStorage               ✅ CLOSED
├── ExecutionBackingStore            ✅ CLOSED
├── RuntimeValue                     ✅ CLOSED — 17 variants
├── DynamicValue                     ✅ CLOSED — 3 variants
├── typed backing identities         ✅ CLOSED
├── backing representation           ✅ CLOSED
├── CallFrame                        ✅ CLOSED — 3 fields
├── InstructionPointer               ✅ CLOSED
├── stepping semantics               ✅ CLOSED
├── ExternalCapability ABI           ✅ CLOSED — failure type cross-phase pending
├── Value<'a> / OwnedValue boundary  ✅ CLOSED
├── VmExecution exact Rust root      ✅ CLOSED — 5 fields
├── Compiled Boundary Value Shape    ✅ CLOSED
└── exact VM inventory               ✅ CLOSED — 19 identities

Outcome / Diagnostic Data            ← NEXT
├── execution Result                 PENDING
├── compile Outcome                  PENDING
├── technical Failure family         PENDING
├── ExternalCapability failure type  PENDING
├── source-linked diagnostic anchor  PENDING
├── Source Location materialization  PENDING
└── exact outcome inventory          PENDING
```

## Phase Map

```text
SOURCE / COMPILE SIDE
────────────────────────────────────────────────────────────
Source Text
    ↓
Lexical Data                  4 identities / TokenKind 50
    ↓
AST Data                      31 identities
    ↓
Semantic Program Data         33 identities
    ↓
Compiled Program Data         21 identities / Instruction 48

EXECUTION SIDE
────────────────────────────────────────────────────────────
CompiledProgram
    + ApplicationBindings
    + Invocation Values
        ↓ boundary validation through CompiledValueShape
VmExecution                   19 own VM identities
        ↓ bytecode execution
Outcome / Diagnostic Data     ← not yet modeled technically
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

- [`SEMANTIC_PROGRAM_DATA.md`](./SEMANTIC_PROGRAM_DATA.md)
- [`SEMANTIC_PROGRAM_STRUCTURE.md`](./SEMANTIC_PROGRAM_STRUCTURE.md)
- [`SEMANTIC_EXPRESSIONS.md`](./SEMANTIC_EXPRESSIONS.md)
- [`SEMANTIC_PROGRAM_INVENTORY.md`](./SEMANTIC_PROGRAM_INVENTORY.md)

### Compiled Program / Bytecode Data

- [`COMPILED_PROGRAM_DATA.md`](./COMPILED_PROGRAM_DATA.md) — root compilado corregido y revalidado.
- [`COMPILED_PROGRAM_INVENTORY.md`](./COMPILED_PROGRAM_INVENTORY.md) — inventario exacto: 21 identities propias, 48 Instruction variants.
- [`COMPILED_STORAGE_DATA.md`](./COMPILED_STORAGE_DATA.md) — slots, constants y `ExternalSymbol { symbol, parameter_count, result_shape }`.
- [`COMPILED_CORE_CALL_INSTRUCTIONS.md`](./COMPILED_CORE_CALL_INSTRUCTIONS.md)
- [`COMPILED_NUMERIC_INSTRUCTIONS.md`](./COMPILED_NUMERIC_INSTRUCTIONS.md)
- [`COMPILED_CONTROL_FLOW.md`](./COMPILED_CONTROL_FLOW.md)
- [`COMPILED_CONVERSIONS.md`](./COMPILED_CONVERSIONS.md)
- [`COMPILED_SCALAR_EQUALITY.md`](./COMPILED_SCALAR_EQUALITY.md)
- [`COMPILED_COMPOSITE_LAYOUT.md`](./COMPILED_COMPOSITE_LAYOUT.md)
- [`COMPILED_COMPOSITE_INSTRUCTIONS.md`](./COMPILED_COMPOSITE_INSTRUCTIONS.md)
- [`COMPILED_STRUCTURAL_EQUALITY.md`](./COMPILED_STRUCTURAL_EQUALITY.md)
- [`COMPILED_SOURCE_MAP.md`](./COMPILED_SOURCE_MAP.md)
- [`COMPILED_BOUNDARY_VALUE_SHAPE.md`](./COMPILED_BOUNDARY_VALUE_SHAPE.md) — boundary contract metadata.

### VM Execution Data

- [`VM_EXECUTION_DATA.md`](./VM_EXECUTION_DATA.md) — autoridad acumulada de VM.
- [`VM_EXECUTION_INVENTORY.md`](./VM_EXECUTION_INVENTORY.md) — inventario exacto de 19 identities propias.
- [`RUNTIME_VALUE_MODEL.md`](./RUNTIME_VALUE_MODEL.md)
- [`BACKING_IDENTITY_STRATEGY.md`](./BACKING_IDENTITY_STRATEGY.md)
- [`RUNTIME_VALUE_REPRESENTATION.md`](./RUNTIME_VALUE_REPRESENTATION.md)
- [`BACKING_DATA_REPRESENTATION.md`](./BACKING_DATA_REPRESENTATION.md)
- [`SHARED_VALUE_STORAGE.md`](./SHARED_VALUE_STORAGE.md)
- [`CALL_FRAME.md`](./CALL_FRAME.md)
- [`INSTRUCTION_POINTER_STEPPING.md`](./INSTRUCTION_POINTER_STEPPING.md)
- [`APPLICATION_BINDINGS.md`](./APPLICATION_BINDINGS.md)
- [`EXTERNAL_CAPABILITY_ABI.md`](./EXTERNAL_CAPABILITY_ABI.md)
- [`VM_EXECUTION_ROOT.md`](./VM_EXECUTION_ROOT.md)
- [`../../../../evo-values/INTERCHANGE_MODEL.md`](../../../../evo-values/INTERCHANGE_MODEL.md)

### Normative language amendments used by compiled/runtime model

- [`../../../../evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`](../../../../evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md)
- [`../../../../evo-script/COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md`](../../../../evo-script/COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md)
- [`../../../../evo-script/EFN_HOST_BOUNDARY_v0.1.md`](../../../../evo-script/EFN_HOST_BOUNDARY_v0.1.md)
- [`../../../../evo-script/EFN_TYPE_CARDINALITY_v0.1.md`](../../../../evo-script/EFN_TYPE_CARDINALITY_v0.1.md)

## Technical Data Diagram Rule

El Technical Data Diagram representa datos y artifacts, no behavioral classes.

Categorías:

```text
<<struct>>
<<enum>>
<<artifact>>
<<borrowed view>>
<<alias>>
```

Relaciones:

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

## Next Block

Antes de avanzar, corresponde revisar el mapa arquitectónico completo.

Después de esa revisión:

```text
Outcome / Diagnostic Data ← NEXT TECHNICAL PHASE
```

La primera responsabilidad de esa fase será definir la representación técnica neutral de outcomes y failures sin mezclar presentación humana con datos diagnósticos, y completar finalmente el failure type de `ExternalCapability`.