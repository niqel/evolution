# Evo-Script Engine — Exact Semantic Program Inventory

Status: CLOSED

Este documento consolida la segunda revisión de `Semantic Program Data` de `evo-script-engine` v0 después de corregir:

```text
language conversions (`to_tipo`)
arbitrary-precision integer semantic literals
canonical external Signature identity
```

La revisión valida cobertura AST → Semantic Program, cardinalidades, suficiencia para Bytecode Compiler y ausencia de identities propias de Compiled Program / VM.

## 1. Review Result

Resultado:

```text
Semantic Program responsibility        ✅
AST semantic coverage                   ✅
resolved identity coverage              ✅
Type information completeness           ✅
Call target completeness                ✅
Signature Dependency forwarding         ✅
External Signature symbolic identity    ✅
Conversion representation               ✅
arbitrary integer semantic preservation ✅
Struct / Enum construction              ✅
when branching / extraction             ✅
Pipeline semantic reduction             ✅
Source Mapping input                     ✅
cardinality consistency                 ✅
no unresolved local syntax identity     ✅
no Compiled/VM identity leakage         ✅

Semantic Program Data                   ✅ CLOSED
```

No se encontró una cuarta identidad o responsabilidad necesaria para Bytecode Compiler.

## 2. Exact Identity Count

Semantic Program Data v0 contiene exactamente **33 identidades técnicas propias**.

### Semantic identities — 7

```text
01 TypeId
02 FunctionId
03 BindingId
04 FieldId
05 VariantId
06 SignatureId
07 SignatureBindingId
```

### Program / Type / Signature / Function structures — 12

```text
08 NativeType
09 SemanticType
10 SemanticField
11 SemanticVariant
12 SemanticBinding
13 SemanticSignatureBinding
14 SemanticParameter
15 SemanticSignatureParameter
16 SignatureSymbol
17 SemanticSignature
18 SemanticFunction
19 SemanticProgram
```

### Function body / Expressions — 14

```text
20 SemanticFunctionBody
21 SemanticStatement
22 SemanticExpression
23 SemanticExpressionKind
24 SemanticLiteral
25 SemanticCallTarget
26 SemanticArgument
27 SemanticCall
28 SemanticFieldValue
29 SemanticEnumPayload
30 SemanticWhen
31 SemanticWhenBranch
32 SemanticVariantExtraction
33 SemanticFieldBinding
```

No se cuentan nuevamente las identidades reutilizadas de fases anteriores:

```text
SourceSpan
UnaryOperator
BinaryOperator
```

porque no son identidades nuevas de Semantic Program Data.

`Conversion`, `Binding`, `FieldAccess`, `StructConstruction` y `EnumConstruction` son variants estructurales de `SemanticExpressionKind`, no structs/enums independientes.

## 3. AST → Semantic Coverage

Toda forma AST cerrada posee un destino semántico o es consumida durante resolution.

```text
Program
    → SemanticProgram

ImportDeclaration
    → consumed by resolution
    → canonical external Signature identity survives as SignatureSymbol
    → imported/shared Types survive through TypeId + SemanticType

Visibility
    → entry_function selection

TypedBinding
    → BindingId + SemanticBinding

StructDefinition / FieldDefinition
    → SemanticType::Struct + SemanticField

EnumDefinition / EnumVariant
    → SemanticType::Enum + SemanticVariant

FunctionDefinition
    → SemanticFunction

Parameter
    → SemanticParameter

FunctionBody
    → SemanticFunctionBody

LetBinding
    → SemanticStatement::Bind

OperationStatement
    → SemanticStatement::Operation

Literal
    → SemanticLiteral

Identifier Value reference
    → BindingId

Unary / Binary
    → typed SemanticExpression + reused operator

FieldAccess
    → FieldId

FunctionCall
    → SemanticCall
    → or Conversion when target is language `to_tipo`

StructConstruction
    → TypeId + SemanticFieldValue

EnumConstruction
    → TypeId + VariantId + SemanticEnumPayload

Pipeline
    → Semantic Expression Composition

When
    → SemanticWhen + resolved VariantId/extractions
```

No queda forma AST que obligue a Bytecode Compiler a consultar AST otra vez.

## 4. External Signature Review

Una Signature externa conserva dos niveles distintos:

```text
SignatureId
    = compact semantic identity inside SemanticProgram

SignatureSymbol { module, name }
    = canonical contractual identity
      required to construct Compiled External Symbol
```

Los aliases locales y los nombres locales de Signature Dependency Parameters no sobreviven para resolution.

Flujo:

```text
source `values::search`
    ↓ Semantic Analyzer
SignatureId
    ↓
SemanticSignature.symbol = SignatureSymbol("values", "search")
    ↓ Bytecode Compiler
Compiled External Symbol
```

`ExternalSymbolId` continúa fuera de Semantic Program.

## 5. Conversion Review

Las operaciones oficiales `to_tipo` se representan mediante:

```rust
SemanticExpressionKind::Conversion {
    operand: Box<SemanticExpression>,
}
```

La operación queda definida por:

```text
source TypeId = operand.type_id
target TypeId = enclosing expression.type_id
```

No se requiere:

```text
BuiltinFunctionId
ConversionKind
SemanticCallTarget variant para conversions
```

Pipeline puede reducir stages a cualquier composición semántica válida:

```text
ordinary function/signature stage → SemanticCall
conversion stage                  → Conversion
```

## 6. Arbitrary Integer Review

`SemanticLiteral::Integer(String)` conserva la magnitud decimal canónica sin imponer límite `u128` durante Compilation Working State.

Esto permite representar semánticamente literales cuyo `TypeId` sea `dynamic` y cuyo valor exceda 128 bits.

La representación física de arbitrary-precision integers se difiere correctamente a VM / Value Data.

```text
Semantic integer meaning
    != runtime arbitrary integer storage
```

## 7. Cardinality Review

Invariantes cerradas:

```text
SemanticProgram.functions       1..N
SemanticProgram.entry_function  exactly 1 valid FunctionId
SemanticProgram.signatures      0..N
SemanticProgram.types           contains every referenced TypeId

Semantic Struct fields          0..N
Semantic Enum variants          1..N
Structured Variant fields       0..N

SemanticFunction.parameters     0..N ordered
SemanticFunction.bindings       0..N
SemanticFunction.signature_bindings 0..N
SemanticFunction.satisfaction   0..1
SemanticFunction.body.result    exactly 1

SemanticSignature.parameters    0..N ordered
SignatureSymbol.module          exactly 1 non-empty canonical identifier
SignatureSymbol.name            exactly 1 non-empty canonical identifier

SemanticCall.arguments          exact arity already validated
SemanticWhen.branches           1..N and exhaustive for subject Enum
```

Todos los IDs presentes en Semantic Program deben referenciar un elemento válido dentro de su namespace owner. Semantic Analyzer success no produce dangling semantic identities.

## 8. Bytecode Compiler Sufficiency Review

Bytecode Compiler puede compilar usando exclusivamente Semantic Program:

```text
TypeId + SemanticType
    → type/layout lowering

FunctionId + SemanticFunction
    → internal function lowering

BindingId + SemanticBinding
    → frame-slot assignment

FieldId
    → field layout/index assignment

VariantId
    → runtime discriminant assignment

SignatureId + SignatureSymbol
    → compiled external-symbol creation

SignatureBindingId
    → dependency forwarding / external-call lowering

SemanticExpression.type_id
    → resolved expression result type

SemanticLiteral
    → constant creation

Conversion source/target TypeId
    → conversion bytecode lowering

SemanticCallTarget
    → internal/external call lowering

SemanticWhen
    → branch/discriminant lowering

SourceSpan
    → Source Mapping generation
```

No necesita:

```text
AST
imports
local aliases
name lookup
scope reconstruction
type inference
semantic validation
Provider lookup
```

## 9. No Compiled / VM Identity Leakage

Continúan fuera de Semantic Program:

```text
ExternalSymbolId
ConstantId / Constant Pool index
Instruction
Opcode
InstructionPointer
CompiledFunctionId/address
ParameterSlot
LocalSlot
OperandSlot / operand depth
FieldOffset
runtime enum discriminant
CallFrame
Shared Value Storage
Provider binding
runtime capability binding
```

Estas identidades pertenecen a los bloques posteriores del Technical Data Model.

## 10. Explicitly Excluded Semantic Wrappers

No se justifican:

```text
SemanticIdentifier
SemanticQualifiedName
SemanticImport
SemanticTypeKind
SemanticPipeline
SemanticPipelineStage
SemanticWhenPattern
SemanticTypedBinding
BuiltinFunctionId
ConversionKind
Function Value
Closure generated for Signature Dependency
Global Binding table
Global Field table
Global Variant table
ExternalSymbolId
```

## 11. Closure

```text
Semantic Program identity family       ✅ CLOSED
Semantic owner structures              ✅ CLOSED
SignatureSymbol                        ✅ CLOSED
Semantic Function Body                 ✅ CLOSED
Semantic Expressions                   ✅ CLOSED
Conversions                            ✅ CLOSED
Arbitrary integer literal preservation ✅ CLOSED
Pipeline semantic reduction            ✅ CLOSED
Exact identity inventory — 33          ✅ CLOSED
Cardinality consistency                ✅ CLOSED
Bytecode Compiler sufficiency          ✅ CLOSED
Compiled/VM boundary                   ✅ CLOSED

Semantic Program Data                  ✅ CLOSED

NEXT
    Compiled Program / Bytecode Data
```
