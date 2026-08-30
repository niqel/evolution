# Evo-Script Engine — Semantic Program Data

Status: SEMANTIC PROGRAM DATA — CLOSED

Este documento define las reglas base e identidades de `Semantic Program Data` de `evo-script-engine` v0.

La autoridad técnica deriva de `TECHNICAL_DESIGN.md`, especialmente TD-002, TD-003, TD-010, TD-011 y TD-012.

```text
AST
    ↓
Semantic Analyzer
    ↓
Semantic Program
    ↓
Bytecode Compiler
    ↓
Compiled Program
```

Documentos especializados:

- `SEMANTIC_PROGRAM_STRUCTURE.md` — owner structures y root.
- `SEMANTIC_EXPRESSIONS.md` — function body y significado ejecutable.
- `SEMANTIC_PROGRAM_INVENTORY.md` — segunda revisión, inventario exacto y cierre.

## SD-001 — Semantic Program representa significado resuelto

Status: CLOSED

`Semantic Program` es la única Semantic IR de v0 y solo existe después de semantic analysis exitoso.

Regla canónica:

> El Bytecode Compiler no vuelve a resolver nombres. Toda identidad necesaria para generar bytecode llega ya resuelta desde Semantic Program.

Consecuencias:

1. `Semantic Program` no es AST decorado.
2. Imports cumplen su responsabilidad durante resolution y no sobreviven como `SemanticImport`.
3. Local names/aliases no son mecanismo de identity resolution en Bytecode Compiler.
4. La identidad contractual externa `module::signature` sí sobrevive mediante `SignatureSymbol` porque debe materializarse como External Symbol persistente.
5. Semantic Program contiene todo el significado necesario para compilar sin volver al AST.
6. Semantic identities no son VM storage identities.
7. External capabilities permanecen separadas de Internal Functions.

## SD-002 — Base Semantic Identities

Status: CLOSED

```rust
struct TypeId(usize);
struct FunctionId(usize);
struct BindingId(usize);
```

Los tres son newtypes opacos del Compilation Working State. No constituyen ABI ni identidad estable entre compilaciones.

```text
TypeId
    = resolved type identity inside SemanticProgram

FunctionId
    = resolved Internal Function identity inside SemanticProgram

BindingId
    = resolved Value binding identity inside one SemanticFunction
```

`BindingId` puede representar Value Parameter, Let Binding y bindings extraídos por `when`.

## SD-003 — Secondary Semantic Identities

Status: CLOSED

```rust
struct FieldId(usize);
struct VariantId(usize);
struct SignatureId(usize);
struct SignatureBindingId(usize);
```

```text
FieldId
    = field inside structural owner

VariantId
    = variant inside Semantic Enum

SignatureId
    = resolved Signature definition inside SemanticProgram

SignatureBindingId
    = concrete Signature Dependency inside one SemanticFunction
```

## SD-004 — Identity Scope

Status: CLOSED

```text
SemanticProgram
├── TypeId namespace               global to program
├── FunctionId namespace           global to program
└── SignatureId namespace          global to program

SemanticFunction
├── BindingId namespace            local to function
└── SignatureBindingId namespace   local to function

Semantic Struct / Structured Variant
└── FieldId namespace              local to structural owner

Semantic Enum
└── VariantId namespace            local to enum owner
```

## SD-005 — Semantic Identity != Physical Layout

Status: CLOSED

```text
TypeId              != runtime type layout
FunctionId          != physical function address
BindingId           != ParameterSlot / LocalSlot
FieldId             != FieldOffset
VariantId           != runtime discriminant
SignatureId         != runtime external binding
SignatureBindingId  != ExternalSymbolId / Provider binding
```

Semantic Analyzer resuelve significado. Bytecode Compiler y VM deciden representación ejecutable.

## SD-006 — Owner-index identity

Status: CLOSED

```text
TypeId(n)              → SemanticProgram.types[n]
FunctionId(n)          → SemanticProgram.functions[n]
SignatureId(n)         → SemanticProgram.signatures[n]
FieldId(n)             → structural_owner.fields[n]
VariantId(n)           → semantic_enum.variants[n]
BindingId(n)           → SemanticFunction.bindings[n]
SignatureBindingId(n)  → SemanticFunction.signature_bindings[n]
```

No se duplica `id` dentro del elemento referenciado.

## SD-007 — Semantic owner structures

Status: CLOSED

Cerradas en `SEMANTIC_PROGRAM_STRUCTURE.md`:

```text
NativeType
SemanticType
SemanticField
SemanticVariant
SemanticBinding
SemanticSignatureBinding
SemanticParameter
SemanticSignatureParameter
SignatureSymbol
SemanticSignature
SemanticFunction
SemanticProgram
```

Forma raíz:

```rust
struct SemanticProgram {
    types: Vec<SemanticType>,
    signatures: Vec<SemanticSignature>,
    functions: Vec<SemanticFunction>,
    entry_function: FunctionId,
}
```

## SD-008 — SignatureSymbol

Status: CLOSED

La segunda revisión confirmó que una Signature externa debe conservar su identidad contractual canónica:

```rust
struct SignatureSymbol {
    module: String,
    name: String,
}
```

`SignatureId` continúa siendo la identity usada dentro de Semantic Program. `SignatureSymbol` solo conserva el significado contractual `module::signature` necesario para que Bytecode Compiler produzca External Symbol data.

```text
local alias / local dependency name
    → consumed during semantic resolution

canonical module::signature
    → preserved as SignatureSymbol
```

`SignatureSymbol` no es `ExternalSymbolId`, Provider identity ni runtime binding.

## SD-009 — ExternalSymbolId remains outside Semantic Program

Status: CLOSED

`ExternalSymbolId` pertenece, si se demuestra necesario, a `Compiled Program / Bytecode Data`.

```text
SignatureId
    ↓
SemanticSignature.symbol
    ↓ Bytecode Compiler
Compiled External Symbol
    ↓
Runtime explicit binding
```

## SD-010 — Semantic Function Body and Expressions

Status: CLOSED

Cerradas en `SEMANTIC_EXPRESSIONS.md`:

```text
SemanticFunctionBody
SemanticStatement
SemanticExpression
SemanticExpressionKind
SemanticLiteral
SemanticCallTarget
SemanticArgument
SemanticCall
SemanticFieldValue
SemanticEnumPayload
SemanticWhen
SemanticWhenBranch
SemanticVariantExtraction
SemanticFieldBinding
```

Toda `SemanticExpression` conserva:

```rust
type_id: TypeId
span: SourceSpan
```

Bytecode Compiler no realiza type inference ni name resolution.

## SD-011 — Language conversions

Status: CLOSED

Las operaciones oficiales `to_tipo` se representan como significado propio del lenguaje:

```rust
SemanticExpressionKind::Conversion {
    operand: Box<SemanticExpression>,
}
```

La conversión se determina por:

```text
source = operand.type_id
target = enclosing SemanticExpression.type_id
```

No se introduce `BuiltinFunctionId` ni `ConversionKind`.

## SD-012 — Arbitrary integer semantic literals

Status: CLOSED

```rust
SemanticLiteral::Integer(String)
```

contiene magnitud decimal canónica ya validada. Esto evita imponer un límite `u128` al Semantic Program y preserva la semántica de `dynamic`, que puede requerir enteros de precisión arbitraria.

Runtime arbitrary-integer storage pertenece a VM / Value Data posterior.

## SD-013 — Pipeline semantic lowering

Status: CLOSED

Pipeline no sobrevive como identity semántica.

```text
AST Pipeline
    ↓ Semantic Analyzer
Semantic Expression Composition
```

```text
ordinary function/signature stage → SemanticCall
conversion stage                  → Conversion
```

No existen:

```text
SemanticPipeline
SemanticPipelineStage
SemanticThis
```

## SD-014 — Source mapping boundary

Status: CLOSED

`SemanticExpression.span` conserva ubicación suficiente para que Bytecode Compiler produzca Source Mapping sin volver al AST.

No se duplican spans por costumbre en todas las estructuras semánticas.

## SD-015 — Second review / Exact inventory

Status: CLOSED

La segunda revisión se registra en `SEMANTIC_PROGRAM_INVENTORY.md`.

Resultado:

```text
exact semantic identity inventory         ✅ 33 identities
AST → Semantic coverage                   ✅
cardinality consistency                   ✅
no unresolved local syntax identity       ✅
Bytecode Compiler sufficiency             ✅
external Signature symbolic identity      ✅
conversion coverage                       ✅
arbitrary integer preservation            ✅
no Compiled/VM identity leakage           ✅
```

Todos los IDs presentes deben referenciar elementos válidos dentro de su owner namespace; Semantic Analyzer success no produce dangling semantic identities.

## SD-016 — Representation Policy

Status: CLOSED

`usize` queda cerrado para IDs semánticos v0 porque Semantic Program pertenece al Compilation Working State.

Si una versión futura requiere serialization persistente, stable ABI o incremental cache cross-process, se reabre explícitamente.

No se introduce en v0:

```text
UUID semantic identities
String-based local semantic resolution
Global BindingId namespace
Global FieldId namespace
Global VariantId namespace
Global SignatureBindingId namespace
Slot identity inside Semantic Analyzer
Stable cross-compilation IDs
ExternalSymbolId in Semantic Program
SemanticImport
SemanticPipeline
BuiltinFunctionId
ConversionKind
Provider identity
runtime binding
```

## Closure

```text
Semantic Program responsibility              ✅ CLOSED
Semantic identity family                     ✅ CLOSED
Identity scopes                              ✅ CLOSED
Owner-index rule                             ✅ CLOSED
Semantic owner structures                    ✅ CLOSED
SignatureSymbol                              ✅ CLOSED
SemanticProgram root                         ✅ CLOSED
SemanticFunctionBody                         ✅ CLOSED
SemanticExpression                           ✅ CLOSED
Semantic calls / arguments                   ✅ CLOSED
Language conversions                         ✅ CLOSED
Arbitrary integer semantic preservation      ✅ CLOSED
Semantic constructions                       ✅ CLOSED
Semantic `when`                              ✅ CLOSED
Pipeline semantic lowering                   ✅ CLOSED
SourceSpan propagation                       ✅ CLOSED
Exact semantic inventory — 33 identities     ✅ CLOSED
Second review                                ✅ CLOSED

Semantic Program Data                        ✅ CLOSED

NEXT
    Compiled Program / Bytecode Data
```
