# Evo-Script Engine — Technical Data Model

Status: TECHNICAL DATA MODEL — IN PROGRESS / OUTCOME INVENTORY ONLY REMAINS

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

`CompilationCatalog` no es un segundo functional input de `Compile`. `Source Text` continúa siendo el único input funcional; el catálogo es una dependency técnica explícita y validada construida fuera del Engine.

## `.efn` / Host Boundary

```text
Host Interactive State
    !=
`.efn` Compile / Execution Data
```

No se introducen Active Scope, Host Session State, Current Provider, Use Node, Use Instruction ni `SET_SCOPE` dentro del Technical Data Model de `.efn`.

## Current Progress

```text
Lexical Data                         ✅ CLOSED
├── Token Kind                       ✅ CLOSED — 50 variants
├── SourceSpan                       ✅ CLOSED
├── Token                            ✅ CLOSED
└── TokenSequence                    ✅ CLOSED

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
├── VmExecution                      ✅ CLOSED — 5 fields
├── ApplicationBindings              ✅ CLOSED
├── SharedValueStorage               ✅ CLOSED
├── ExecutionBackingStore            ✅ CLOSED
├── RuntimeValue                     ✅ CLOSED — 17 variants
├── DynamicValue                     ✅ CLOSED — 3 variants
├── typed backing identities         ✅ CLOSED
├── CallFrame                        ✅ CLOSED — 3 fields
├── InstructionPointer               ✅ CLOSED
├── stepping semantics               ✅ CLOSED
├── ExternalCapability ABI           ✅ CLOSED — exact Rust signature complete
├── Value<'a> / OwnedValue boundary  ✅ CLOSED
├── Compiled Boundary Value Shape    ✅ CLOSED
└── exact VM inventory               ✅ CLOSED — 19 identities

Outcome / Diagnostic Data            ◉ IN PROGRESS — INVENTORY ONLY
├── Outcome root                     ✅ CLOSED
│   ├── CompileOutcome               ✅ CLOSED
│   └── ExecutionOutcome             ✅ CLOSED
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
│   ├── CompileFailure span          ✅ mandatory SourceSpan
│   ├── ExecutionFailure span        ✅ Option<SourceSpan>
│   ├── DiagnosticAnchor             ❌ NOT NEEDED v0
│   └── SourceLocation / SourceId    ❌ NOT NEEDED v0
├── ExternalCapabilityFailure        ✅ CLOSED — 1 identity / 1 field
│   └── code: Box<str>               ✅ CLOSED
├── ExecutionFailure                 ✅ CLOSED — 5 own identities
│   ├── ExecutionFailureKind         ✅ CLOSED — 4 variants
│   ├── InvocationFailure            ✅ CLOSED — 2 variants
│   ├── EvaluationFailure            ✅ CLOSED — 4 variants
│   └── ExternalExecutionFailure     ✅ CLOSED — 3 variants
└── exact Outcome inventory          ← NEXT / FINAL DATA-MODEL BLOCK
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
        ↓ exact boundary validation
VmExecution                   19 own VM identities
        ↓
Outcome / Diagnostic Data     shapes CLOSED / inventory pending
```

## Compile-time vs runtime external composition

```text
CompilationCatalog
    = describes semantic contracts at compile time
    = shared Types + Signatures
    = no Provider / fn pointer

ApplicationBindings
    = supplies executable capabilities at runtime
    = SignatureSymbol → ExternalCapability
    = no compile-time type resolution
```

They are intentionally distinct and never ambient/global Engine state.

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

Physical `.elib` / `.emod` / filesystem / catalog-construction failures remain outside Engine `SemanticFailure`.

## Closed diagnostic provenance boundary

```text
CompileFailure
├── kind
└── source_span: SourceSpan

ExecutionFailure
├── kind
└── source_span: Option<SourceSpan>
```

No existe `DiagnosticAnchor`, `SourceLocation` ni `SourceId` v0.

Runtime:

```text
CallFrame.function + InstructionPointer ordinal
        ↓
SourceMap
        ↓
SourceSpan
```

Invocation failures before a valid VM use `source_span = None`.

## Closed external capability boundary

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}

type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

The adapter normalizes Provider/vendor failures before crossing the ABI. `MissingBinding` and `ResultContractMismatch` remain Engine-owned execution failures.

## Closed execution-failure boundary

Authority: [`EXECUTION_FAILURE.md`](./EXECUTION_FAILURE.md).

```rust
struct ExecutionFailure {
    kind: ExecutionFailureKind,
    source_span: Option<SourceSpan>,
}

enum ExecutionFailureKind {
    Compilation(CompileFailureKind),
    Invocation(InvocationFailure),
    Evaluation(EvaluationFailure),
    External(ExternalExecutionFailure),
}
```

Exact subfamilies:

```text
InvocationFailure
├── ArityMismatch { expected, actual }
└── ArgumentShapeMismatch { position }

EvaluationFailure
├── Overflow
├── DivisionByZero
├── Conversion
└── DynamicNumericType

ExternalExecutionFailure
├── MissingBinding { signature }
├── CapabilityFailure { signature, failure }
└── ResultContractMismatch { signature }
```

Provenance:

```text
Compilation → Some(CompileFailure.source_span)
Invocation  → None
Evaluation  → Some(current instruction span)
External    → Some(CallExternal span)
```

VM/compiler invariant violations do not become normal execution outcomes.

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
- [`COMPILATION_DEPENDENCY_MODEL.md`](./COMPILATION_DEPENDENCY_MODEL.md)

### Semantic Program Data
- [`SEMANTIC_PROGRAM_DATA.md`](./SEMANTIC_PROGRAM_DATA.md)
- [`SEMANTIC_PROGRAM_STRUCTURE.md`](./SEMANTIC_PROGRAM_STRUCTURE.md)
- [`SEMANTIC_EXPRESSIONS.md`](./SEMANTIC_EXPRESSIONS.md)
- [`SEMANTIC_PROGRAM_INVENTORY.md`](./SEMANTIC_PROGRAM_INVENTORY.md)

### Compiled Program / Bytecode Data
- [`COMPILED_PROGRAM_DATA.md`](./COMPILED_PROGRAM_DATA.md)
- [`COMPILED_PROGRAM_INVENTORY.md`](./COMPILED_PROGRAM_INVENTORY.md)
- [`COMPILED_STORAGE_DATA.md`](./COMPILED_STORAGE_DATA.md)
- [`COMPILED_CORE_CALL_INSTRUCTIONS.md`](./COMPILED_CORE_CALL_INSTRUCTIONS.md)
- [`COMPILED_NUMERIC_INSTRUCTIONS.md`](./COMPILED_NUMERIC_INSTRUCTIONS.md)
- [`COMPILED_CONTROL_FLOW.md`](./COMPILED_CONTROL_FLOW.md)
- [`COMPILED_CONVERSIONS.md`](./COMPILED_CONVERSIONS.md)
- [`COMPILED_SCALAR_EQUALITY.md`](./COMPILED_SCALAR_EQUALITY.md)
- [`COMPILED_COMPOSITE_LAYOUT.md`](./COMPILED_COMPOSITE_LAYOUT.md)
- [`COMPILED_COMPOSITE_INSTRUCTIONS.md`](./COMPILED_COMPOSITE_INSTRUCTIONS.md)
- [`COMPILED_STRUCTURAL_EQUALITY.md`](./COMPILED_STRUCTURAL_EQUALITY.md)
- [`COMPILED_SOURCE_MAP.md`](./COMPILED_SOURCE_MAP.md)
- [`COMPILED_BOUNDARY_VALUE_SHAPE.md`](./COMPILED_BOUNDARY_VALUE_SHAPE.md)

### VM Execution Data
- [`VM_EXECUTION_DATA.md`](./VM_EXECUTION_DATA.md)
- [`VM_EXECUTION_INVENTORY.md`](./VM_EXECUTION_INVENTORY.md)
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
- [`EXECUTION_FAILURE.md`](./EXECUTION_FAILURE.md)

## Technical Data Diagram Rule

El Technical Data Diagram representa datos y artifacts, no behavioral classes.

Categories:

```text
<<struct>>
<<enum>>
<<artifact>>
<<borrowed view>>
<<alias>>
```

Relations:

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
exact Outcome / Diagnostic inventory ← NEXT
```

Este es el último bloque del Technical Data Model. No debe introducir semántica nueva: solamente auditar aliases, own identities y reused identities de Outcome / Diagnostic Data.

Si la auditoría no revela inconsistencias:

```text
Outcome / Diagnostic Data ✅ CLOSED
Technical Data Model       ✅ CLOSED
```

Después comienza la siguiente etapa metodológica: `Technical Data Diagram`.