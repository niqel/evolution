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
Semantic Analyzer ◄──────── CompilationCatalog
    ↓                       explicit borrowed technical dependency
Semantic Program Data
    ↓
Compiled Program / Bytecode Data
    ↓
VM Execution Data
    ↓
Outcome / Diagnostic Data
```

`CompilationCatalog` no es un segundo functional input de `Compile`. `Source Text` continúa siendo el único input funcional. El catálogo es una dependencia técnica explícita y validada, construida fuera de `evo-script-engine`, necesaria para resolver contratos de shared Types y Signatures importadas sin introducir filesystem/module resolution dentro del Engine.

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

Compilation Dependency Data          ✅ CLOSED — CORRECTIVE MODEL
├── CompilationCatalog               ✅ CLOSED
├── TypeSymbol                       ✅ CLOSED
├── CatalogTypeRef                   ✅ CLOSED — 18 variants
├── Catalog Type structures          ✅ CLOSED
├── Catalog Signature structures     ✅ CLOSED
├── explicit immutable borrow        ✅ CLOSED
├── external catalog construction    ✅ CLOSED
└── exact dependency inventory       ✅ CLOSED — 8 identities

Semantic Program Data                ✅ CLOSED
├── catalog contracts lower to local TypeId / SignatureId
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
├── ExternalCapability ABI           ✅ CLOSED — exact Rust signature complete
├── Value<'a> / OwnedValue boundary  ✅ CLOSED
├── VmExecution exact Rust root      ✅ CLOSED — 5 fields
├── Compiled Boundary Value Shape    ✅ CLOSED
└── exact VM inventory               ✅ CLOSED — 19 identities

Outcome / Diagnostic Data            ◉ IN PROGRESS
├── Outcome root                     ✅ CLOSED
│   ├── CompileOutcome               ✅ CLOSED
│   ├── ExecutionOutcome             ✅ CLOSED
│   └── external failure ownership   ✅ CLOSED
├── CompileFailure                   ✅ CLOSED — ROOT + SUBFAMILIES
│   ├── LexicalFailure               ✅ CLOSED — 6 variants
│   ├── SyntaxFailure                ✅ CLOSED — 10 variants
│   └── SemanticFailure              ✅ CLOSED — 12 own identities
│       ├── root families            ✅ CLOSED — 7 variants
│       ├── ResolutionFailure        ✅ CLOSED — 4 variants
│       ├── DeclarationFailure       ✅ CLOSED — 7 variants
│       ├── TypeCheckingFailure      ✅ CLOSED — 8 variants
│       ├── CallFailure              ✅ CLOSED — 7 variants
│       ├── CompositeFailure         ✅ CLOSED — 10 variants
│       ├── WhenFailure              ✅ CLOSED — 11 variants
│       └── SignatureMismatchKind    ✅ CLOSED — 6 variants
├── Diagnostic provenance            ✅ CLOSED — SourceSpan / 0 new identities
│   ├── CompileFailure span          ✅ CLOSED — mandatory SourceSpan
│   ├── ExecutionFailure span        ✅ CLOSED — Option<SourceSpan>
│   ├── DiagnosticAnchor             ❌ NOT NEEDED v0
│   ├── SourceLocation identity      ❌ NOT NEEDED v0
│   └── runtime SourceMap materialization ✅ CLOSED
├── ExternalCapabilityFailure        ✅ CLOSED — 1 identity / 1 field
│   ├── code: Box<str>               ✅ CLOSED
│   ├── lowercase snake_case         ✅ CLOSED
│   ├── provider normalization       ✅ CLOSED
│   └── exact ExternalCapability ABI ✅ CLOSED
├── ExecutionFailure exact family    ← NEXT
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
                            CompilationCatalog
                            8 own dependency identities
                                   │
                                   ▼
Semantic Analyzer ◄────────────────┘
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
Outcome / Diagnostic Data     ◉ currently being modeled
```

## Compile-time vs runtime external composition

```text
CompilationCatalog
    = describes external semantic contracts at compile time
    = shared Types + Signatures
    = no Provider / fn pointer

ApplicationBindings
    = supplies executable capabilities at runtime
    = SignatureSymbol → ExternalCapability
    = no compile-time type resolution
```

They are intentionally distinct and are never ambient/global Engine state.

## Closed compile-failure boundary

```text
Source Text
    ↓
Lexer
    ├── LexicalFailure       6 variants
    ▼
TokenSequence
    ↓
Parser
    ├── SyntaxFailure       10 variants
    ▼
AST + CompilationCatalog
    ↓
Semantic Analyzer
    ├── SemanticFailure     12 own identities
    ▼
SemanticProgram
```

`SemanticFailure` no absorbe failures físicos de `.elib`, `.emod`, filesystem o construcción del catálogo. Esas condiciones deben resolverse antes de entregar un `CompilationCatalog` válido al Engine.

## Closed diagnostic provenance boundary

```text
CompileFailure
├── kind
└── source_span: SourceSpan

ExecutionFailure
├── kind
└── source_span: Option<SourceSpan>
```

No existe una identity `DiagnosticAnchor` o `SourceLocation` en v0.

Runtime materialization:

```text
CallFrame.function + InstructionPointer ordinal
        ↓
SourceMap
        ↓
SourceSpan
        ↓
ExecutionFailure.source_span = Some(span)
```

Invocation failures que ocurren antes de una `VmExecution` válida usan `source_span = None`.

`SourceSpan` conserva provenance técnica; line/column/snippet/path son presentación derivada por el Consumer cuando dispone del Source Text.

## Closed external capability failure boundary

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}
```

Exact ABI:

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

El code es una identity simbólica estable lowercase snake_case y está contextualizado por la `SignatureSymbol` que el Engine ya conoce.

El adapter de aplicación normaliza errors de Provider/vendor antes de cruzar el ABI.

No cruzan hacia el Engine:

```text
Provider-specific error type
Box<dyn Error>
std::io::Error
SourceSpan
human message
arbitrary details payload
```

`MissingBinding` y `ResultContractMismatch` siguen siendo failures propios del Engine y se definirán dentro de `ExecutionFailure`.

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

### Compilation Dependency Data

- [`COMPILATION_DEPENDENCY_MODEL.md`](./COMPILATION_DEPENDENCY_MODEL.md) — `CompilationCatalog`, shared-Type/Signature contracts y frontera de ownership/failure con el resolver externo.

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

### Outcome / Diagnostic Data

- [`OUTCOME_DIAGNOSTIC_DATA.md`](./OUTCOME_DIAGNOSTIC_DATA.md)
- [`COMPILE_FAILURE.md`](./COMPILE_FAILURE.md)
- [`LEXICAL_FAILURE.md`](./LEXICAL_FAILURE.md)
- [`SYNTAX_FAILURE.md`](./SYNTAX_FAILURE.md)
- [`SEMANTIC_FAILURE.md`](./SEMANTIC_FAILURE.md)
- [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md)
- [`EXTERNAL_CAPABILITY_FAILURE.md`](./EXTERNAL_CAPABILITY_FAILURE.md)

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

```text
ExecutionFailure exact family ← NEXT
```

Ya no quedan placeholders dentro de `ExternalCapability` ni de provenance diagnóstica.

Después:

```text
ExecutionFailure exact family
    ↓
Outcome exact inventory
    ↓
Outcome / Diagnostic Data CLOSED
```
