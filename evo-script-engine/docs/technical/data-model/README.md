# Evo-Script Engine — Technical Data Model

Status: CLOSED

Este directorio contiene la autoridad técnica del Data Model de `evo-script-engine` v0.

> Toda estructura, enum, alias, artifact o dato interno necesario para expresar una Rust Signature o implementar un Participant debe estar definido previamente en el Technical Data Model.

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

`CompilationCatalog` no es un segundo functional input de `Compile`; es una dependencia técnica explícita, borrowed e inmutable construida fuera del Engine.

## Global Status

```text
Lexical Data                         ✅ CLOSED
├── TokenKind                       ✅ 50 variants
├── SourceSpan                      ✅
├── Token                           ✅
└── TokenSequence                   ✅

AST Data                             ✅ CLOSED
└── exact AST inventory             ✅ 31 identities

Compilation Dependency Data          ✅ CLOSED
├── CompilationCatalog              ✅
├── CatalogTypeRef                  ✅ 18 variants
└── exact dependency inventory      ✅ 8 identities

Semantic Program Data                ✅ CLOSED
└── exact semantic inventory        ✅ 33 identities

Compiled Program / Bytecode Data     ✅ CLOSED / REVALIDATED
├── NumericKind                     ✅ 12 variants
├── Instruction                     ✅ 48 variants
├── CompiledValueShape              ✅ 17 variants
├── CompiledEnumValueShape          ✅ 3 variants
└── exact compiled inventory        ✅ 21 identities

VM Execution Data                    ✅ CLOSED
├── VmExecution                     ✅ 5 fields
├── RuntimeValue                    ✅ 17 variants
├── DynamicValue                    ✅ 3 variants
├── CallFrame                       ✅ 3 fields
├── ExternalCapability ABI          ✅ exact Rust signature complete
└── exact VM inventory              ✅ 19 identities

Outcome / Diagnostic Data            ✅ CLOSED
├── CompileOutcome / ExecutionOutcome ✅
├── CompileFailure                  ✅
│   ├── LexicalFailure              ✅ 6 variants
│   ├── SyntaxFailure               ✅ 10 variants
│   └── SemanticFailure             ✅ 12 own identities
├── Diagnostic provenance           ✅ SourceSpan / 0 new identities
├── ExternalCapabilityFailure       ✅ 1 identity
├── ExecutionFailure                ✅ 5 own identities
└── exact Outcome inventory         ✅ 24 identities

TECHNICAL DATA MODEL                  ✅ CLOSED
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
Outcome / Diagnostic Data     24 own identities
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

Estas dos composiciones son intencionalmente distintas y nunca son estado ambient/global del Engine.

## Host Boundary

```text
Host Interactive State
    !=
.efn Compile / Execution Data
```

No se introducen `Active Scope`, Host Session State, Current Provider, Use Node, Use Instruction ni `SET_SCOPE` dentro del Data Model de `.efn`.

## Closed diagnostic boundary

```text
CompileFailure
├── kind
└── source_span: SourceSpan

ExecutionFailure
├── kind
└── source_span: Option<SourceSpan>
```

No existen en v0:

```text
DiagnosticAnchor
SourceLocation
SourceId
```

Runtime materialization:

```text
CallFrame.function
+
InstructionPointer ordinal
    ↓
SourceMap
    ↓
SourceSpan
```

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

Provider/vendor errors are normalized before crossing the Engine ABI.

## Closed execution-failure boundary

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

Normal runtime evaluation failures are exactly:

```text
Overflow
DivisionByZero
Conversion
DynamicNumericType
```

Normal external execution failures are exactly:

```text
MissingBinding
CapabilityFailure
ResultContractMismatch
```

VM/compiler invariant violations remain internal bugs, not normal language outcomes.

## Exact Outcome / Diagnostic Inventory

Authority: [`OUTCOME_DIAGNOSTIC_INVENTORY.md`](./OUTCOME_DIAGNOSTIC_INVENTORY.md).

```text
Public outcome aliases             2
Compile failure root               2
Lexical / Syntax failure families  2
Semantic failure family           12
External capability failure        1
Execution failure family           5
                                  ──
TOTAL                              24
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
- [`OUTCOME_DIAGNOSTIC_INVENTORY.md`](./OUTCOME_DIAGNOSTIC_INVENTORY.md)
- [`COMPILE_FAILURE.md`](./COMPILE_FAILURE.md)
- [`LEXICAL_FAILURE.md`](./LEXICAL_FAILURE.md)
- [`SYNTAX_FAILURE.md`](./SYNTAX_FAILURE.md)
- [`SEMANTIC_FAILURE.md`](./SEMANTIC_FAILURE.md)
- [`DIAGNOSTIC_PROVENANCE.md`](./DIAGNOSTIC_PROVENANCE.md)
- [`EXTERNAL_CAPABILITY_FAILURE.md`](./EXTERNAL_CAPABILITY_FAILURE.md)
- [`EXECUTION_FAILURE.md`](./EXECUTION_FAILURE.md)

## Technical Data Diagram Rule

El siguiente artifact representa datos y relaciones ya cerrados; no behavioral classes.

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
Technical Data Diagram ← NEXT
```

El Technical Data Diagram debe visualizar el Data Model ya cerrado. Cualquier nueva identity detectada durante el diagrama debe tratarse como una inconsistencia explícita y reabrir únicamente el bloque afectado; no debe inventarse silenciosamente durante diagramación.