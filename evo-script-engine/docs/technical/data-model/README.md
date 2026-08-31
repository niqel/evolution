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
├── ApplicationBindings relation ✅ CLOSED
├── ApplicationBindings exact    ✅ CLOSED
├── SignatureSymbol lookup       ✅ CLOSED
├── lazy missing-binding failure ✅ CLOSED
├── SharedValueStorage owner     ✅ CLOSED
├── Call Frames owner            ✅ CLOSED
├── execution backing owner      ✅ CLOSED
├── RuntimeValue / Value boundary ✅ CLOSED
├── RuntimeValue descriptor role ✅ CLOSED
├── fixed scalars inline         ✅ CLOSED
├── variable/composite backing   ✅ CLOSED
├── no persistent self-borrow    ✅ CLOSED
├── typed backing identities     ✅ CLOSED
├── no universal backing ID      ✅ CLOSED
├── String backing origin        ✅ CLOSED
├── Dynamic Integer backing      ✅ CLOSED
├── Struct / Enum backing IDs    ✅ CLOSED
├── backing ID stability         ✅ CLOSED
├── RuntimeValue exact enum      ✅ CLOSED — 17 variants
├── DynamicValue exact enum      ✅ CLOSED — 3 variants
├── descriptor Clone + Copy      ✅ CLOSED
├── runtime equality boundary    ✅ CLOSED
├── context-relative RuntimeValue ✅ CLOSED
├── ExecutionBackingStore        ✅ CLOSED
├── four typed append-only stores ✅ CLOSED
├── String backing = Box<str>    ✅ CLOSED
├── Dynamic Integer backing      ✅ CLOSED
├── Struct / Enum backing        ✅ CLOSED
├── immutable composite DAG      ✅ CLOSED
├── SharedValueStorage           ✅ CLOSED
├── Vec<Option<RuntimeValue>>    ✅ CLOSED
├── parameter/local/operand cells ✅ CLOSED
├── operand tail mechanics       ✅ CLOSED
├── call argument cell reuse     ✅ CLOSED
├── Return storage transformation ✅ CLOSED
├── CallFrame exact representation ✅ CLOSED — 3 fields
├── InstructionPointer identity  ✅ CLOSED
├── current responsible IP       ✅ CLOSED
├── operand_base derived         ✅ CLOSED
├── caller suspended on Call     ✅ CLOSED
├── InstructionPointer stepping  ✅ CLOSED
├── active IP validity           ✅ CLOSED
├── branch / sequential stepping ✅ CLOSED
├── call / return stepping       ✅ CLOSED
├── ExternalCapability ABI semantics ✅ CLOSED
├── borrowed external arguments ✅ CLOSED
├── owned external success result ✅ CLOSED
├── external N→1 commit-on-success ✅ CLOSED
├── evo-values Value<'a>         ✅ CLOSED — 17 variants
├── evo-values OwnedValue        ✅ CLOSED — 17 variants
├── canonical Dynamic Integer interchange ✅ CLOSED
├── no_std + alloc interchange   ✅ CLOSED
└── VmExecution exact Rust root  ← NEXT

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

- [`COMPILED_PROGRAM_DATA.md`](./COMPILED_PROGRAM_DATA.md) — autoridad raíz del producto compilado, CLOSED.
- [`COMPILED_PROGRAM_INVENTORY.md`](./COMPILED_PROGRAM_INVENTORY.md) — inventario exacto: 18 identities propias, 48 Instruction variants y cobertura Semantic → Compiled.
- [`COMPILED_STORAGE_DATA.md`](./COMPILED_STORAGE_DATA.md) — `ParameterSlot`, `LocalSlot`, `ExternalSymbol.parameter_count`, Constant Pool canonicalizado y `DynamicConstant`.
- [`COMPILED_CORE_CALL_INSTRUCTIONS.md`](./COMPILED_CORE_CALL_INSTRUCTIONS.md) — Load/Store, `Call`, `CallExternal` y calling convention física.
- [`COMPILED_NUMERIC_INSTRUCTIONS.md`](./COMPILED_NUMERIC_INSTRUCTIONS.md) — `NumericKind`, fixed arithmetic/comparison, `LiftDynamic` y dynamic arithmetic.
- [`COMPILED_CONTROL_FLOW.md`](./COMPILED_CONTROL_FLOW.md) — `InstructionIndex`, branching, short-circuit, `Discard` y `Return`.
- [`COMPILED_CONVERSIONS.md`](./COMPILED_CONVERSIONS.md) — fixed/dynamic numeric conversions y string conversion.
- [`COMPILED_SCALAR_EQUALITY.md`](./COMPILED_SCALAR_EQUALITY.md) — bool negation/equality y string equality.
- [`COMPILED_COMPOSITE_LAYOUT.md`](./COMPILED_COMPOSITE_LAYOUT.md) — `FieldIndex`, `VariantDiscriminant` y canonical composite layout.
- [`COMPILED_COMPOSITE_INSTRUCTIONS.md`](./COMPILED_COMPOSITE_INSTRUCTIONS.md) — struct/enum construction, access, variant testing, payload extraction y `when` lowering.
- [`COMPILED_STRUCTURAL_EQUALITY.md`](./COMPILED_STRUCTURAL_EQUALITY.md) — `EqualityRule`, `CompositeEqualityPlan` y struct/enum structural equality.
- [`COMPILED_SOURCE_MAP.md`](./COMPILED_SOURCE_MAP.md) — dense `(FunctionId, InstructionIndex) → SourceSpan` mapping.

### VM Execution Data

- [`VM_EXECUTION_DATA.md`](./VM_EXECUTION_DATA.md) — autoridad acumulada de VM Execution Data.
- [`RUNTIME_VALUE_MODEL.md`](./RUNTIME_VALUE_MODEL.md) — frontera `RuntimeValue` / `evo_values::Value<'a>`, descriptor interno, scalar/backing policy y exact runtime value family.
- [`BACKING_IDENTITY_STRATEGY.md`](./BACKING_IDENTITY_STRATEGY.md) — typed backing IDs, referencias `Compiled | Execution`, estabilidad por invocation y separación frente al container físico.
- [`RUNTIME_VALUE_REPRESENTATION.md`](./RUNTIME_VALUE_REPRESENTATION.md) — exact `RuntimeValue` de 17 variants, `DynamicValue` de 3 variants, copy semantics y execution-context-relative handles.
- [`BACKING_DATA_REPRESENTATION.md`](./BACKING_DATA_REPRESENTATION.md) — `ExecutionBackingStore`, String/Dynamic Integer/Struct/Enum backing, inmutabilidad y composite DAG.
- [`SHARED_VALUE_STORAGE.md`](./SHARED_VALUE_STORAGE.md) — `Vec<Option<RuntimeValue>>`, regiones Parameter/Local/Operand, reuse de argument cells y transformación de storage en `Return`.
- [`CALL_FRAME.md`](./CALL_FRAME.md) — `CallFrame` exacto de tres fields, `InstructionPointer`, `frame_base`, `operand_base` derivado y suspensión del caller sobre `Call`.
- [`INSTRUCTION_POINTER_STEPPING.md`](./INSTRUCTION_POINTER_STEPPING.md) — valid IP invariant, commit-after-success, sequential/branch stepping y transitions exactas de `Call`, `Return` y `CallExternal`.
- [`APPLICATION_BINDINGS.md`](./APPLICATION_BINDINGS.md) — `HashMap<SignatureSymbol, ExternalCapability>`, composición explícita reusable y resolución lazy de `CallExternal`.
- [`EXTERNAL_CAPABILITY_ABI.md`](./EXTERNAL_CAPABILITY_ABI.md) — ABI uniforme por function pointer, `&[Value<'a>]` arguments, `OwnedValue` success result y commit `N → 1` después de success.
- [`../../../../evo-values/INTERCHANGE_MODEL.md`](../../../../evo-values/INTERCHANGE_MODEL.md) — modelo compartido `Value<'a>` / `OwnedValue`, 17 familias exactas, Dynamic Integer canónico y `no_std + alloc`.

### Normative language amendments used by compiled/runtime model

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
VmExecution exact Rust root ← NEXT
```

Aquí se cerrará el `struct VmExecution<'...>` exacto: lifetimes de `CompiledProgram` y `ApplicationBindings`, fields owned (`SharedValueStorage`, `ExecutionBackingStore`, `Vec<CallFrame>`) y cualquier invariante de inicialización que realmente pertenezca al root, sin introducir Context, Session ni estado duplicado.
