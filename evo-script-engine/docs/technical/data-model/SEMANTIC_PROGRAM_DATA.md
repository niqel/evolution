# Evo-Script Engine — Semantic Program Data

Status: SEMANTIC PROGRAM DATA — IN ANALYSIS

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

Las estructuras owner cerradas se detallan en `SEMANTIC_PROGRAM_STRUCTURE.md`.

## SD-001 — Semantic Program representa significado resuelto

Status: CLOSED

`Semantic Program` es la única Semantic IR de v0 y solo existe después de semantic analysis exitoso.

Regla canónica:

> El Bytecode Compiler no vuelve a resolver nombres. Toda identidad necesaria para generar bytecode llega ya resuelta desde Semantic Program.

Consecuencias:

1. `Semantic Program` no es AST decorado.
2. Imports cumplen su responsabilidad durante semantic resolution y no sobreviven como `SemanticImport` por defecto.
3. Names pueden conservarse como diagnostic/debug metadata cuando se demuestre necesario, pero nunca como mecanismo de identity resolution dentro de Bytecode Compiler.
4. Semantic Program contiene todo el significado necesario para compilar sin volver a consultar AST ni reconstruir scope/name resolution.
5. Semantic identities no son VM storage identities.
6. `ParameterSlot`, `LocalSlot`, operand layout y otras identidades físicas pertenecen a Bytecode / VM Data.
7. External capabilities continúan separadas de Internal Functions conforme a TD-003 y TD-011.

## SD-002 — Base Semantic Identities

Status: CLOSED

```rust
struct TypeId(usize);
struct FunctionId(usize);
struct BindingId(usize);
```

Los tres son newtypes opacos. `usize` es representación interna del Compilation Working State; no constituye ABI, formato persistente ni identificador estable entre compilaciones.

### TypeId

`TypeId` identifica de forma única un tipo resuelto dentro de un `SemanticProgram`.

Su namespace incluye los tipos necesarios para compilar:

```text
Native Type
Local Struct
Local Enum
Imported Type
Transitively required Type
```

No expresa layout físico, tamaño de runtime ni discriminante.

### FunctionId

`FunctionId` identifica una Internal Function resuelta dentro de un `SemanticProgram`.

```text
AST function name
    ↓ Semantic Analyzer
FunctionId
    ↓ Bytecode Compiler
compiled CALL target
```

No identifica Signature, Provider, physical address ni stable ABI identity.

### BindingId

`BindingId` identifica un Value binding dentro de una única `SemanticFunction`.

Orígenes válidos:

```text
Value Parameter
Let Binding
Associated when extraction
Structured when extraction
```

No identifica Signature Dependency ni Parameter/Local Slot.

## SD-003 — Secondary Semantic Identities

Status: CLOSED

```rust
struct FieldId(usize);
struct VariantId(usize);
struct SignatureId(usize);
struct SignatureBindingId(usize);
```

### FieldId

Identifica un field dentro de su owner estructural:

```text
Semantic Struct Type
Structured Enum Variant
```

`FieldId != FieldOffset` y no representa layout físico.

### VariantId

Identifica una variante dentro de un Semantic Enum Type.

`VariantId != runtime discriminant`.

### SignatureId

Identifica una Signature semántica resuelta dentro de `SemanticProgram`.

Representa el contrato/capability definido, no una dependencia local concreta, Provider ni runtime binding.

### SignatureBindingId

Identifica una Signature Dependency concreta dentro de una única `SemanticFunction`.

Dos dependencies distintas pueden referenciar el mismo `SignatureId`.

```text
SignatureId
├── SignatureBindingId A
└── SignatureBindingId B
```

No es `BindingId`, `ExternalSymbolId` ni Provider binding.

## SD-004 — Identity Scope

Status: CLOSED

```text
SemanticProgram
├── TypeId namespace              global to program
├── FunctionId namespace          global to program
└── SignatureId namespace         global to program

SemanticFunction
├── BindingId namespace           local to function
└── SignatureBindingId namespace  local to function

Semantic Struct / Structured Variant
└── FieldId namespace             local to structural owner

Semantic Enum
└── VariantId namespace           local to enum owner
```

Los namespaces son conceptualmente independientes aun cuando todos utilicen `usize` internamente.

## SD-005 — Semantic Identity != Physical Layout

Status: CLOSED

Separación obligatoria:

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

Los IDs identifican por posición dentro de la colección de su owner. No se duplica `id` dentro del elemento.

```text
TypeId(n)              → SemanticProgram.types[n]
FunctionId(n)          → SemanticProgram.functions[n]
SignatureId(n)         → SemanticProgram.signatures[n]
FieldId(n)             → structural_owner.fields[n]
VariantId(n)           → semantic_enum.variants[n]
BindingId(n)           → SemanticFunction.bindings[n]
SignatureBindingId(n)  → SemanticFunction.signature_bindings[n]
```

La definición completa de los owners se encuentra en `SEMANTIC_PROGRAM_STRUCTURE.md`.

## SD-007 — Semantic owner structures

Status: CLOSED

Quedan cerradas en `SEMANTIC_PROGRAM_STRUCTURE.md`:

```text
NativeType
SemanticType
SemanticField
SemanticVariant
SemanticBinding
SemanticSignatureBinding
SemanticParameter
SemanticSignatureParameter
SemanticSignature
SemanticFunction shell
SemanticProgram root
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

`SemanticFunction` conserva una única lista posicional de `SemanticParameter`, mientras `bindings` y `signature_bindings` son owners separados de sus respectivos namespaces.

## SD-008 — ExternalSymbolId remains outside Semantic Program

Status: CLOSED

`ExternalSymbolId` no pertenece a Semantic Program Data v0.

Semantic Program conserva el significado resuelto mediante `SignatureId` y `SignatureBindingId`. Si Bytecode Compiler necesita una identidad compacta para runtime external symbols, la crea dentro de `Compiled Program / Bytecode Data`.

```text
Semantic Signature meaning
        ↓ Bytecode Compiler
Compiled External Symbol
        ↓
Runtime explicit binding
```

No se introduce Provider identity ni Current Provider.

## SD-009 — Representation Policy

Status: CLOSED

`usize` queda cerrado para las identities semánticas de v0 porque Semantic Program pertenece al Compilation Working State.

Si una versión futura requiere serialización persistente, ABI estable, incremental cache cross-process u otra vida más larga, esta representación deberá reabrirse explícitamente.

No se introduce en v0:

```text
UUID semantic identities
String-based semantic identity
Global BindingId namespace
Global FieldId namespace
Global VariantId namespace
Global SignatureBindingId namespace
Slot identity inside Semantic Analyzer
Stable cross-compilation IDs
ExternalSymbolId in Semantic Program
```

## SD-010 — Next block

Status: IN ANALYSIS

El shell estructural de Semantic Program ya está cerrado. Falta resolver el significado ejecutable dentro de cada función:

```text
SemanticFunctionBody
SemanticStatement
SemanticExpression
resolved calls
resolved constructions
resolved field access
resolved `when`
Pipeline lowering
literal materialization
SourceSpan propagation for Bytecode Source Mapping
```

Este bloque debe permitir que Bytecode Compiler traduzca directamente Semantic Program sin name resolution, type inference o semantic validation adicional.

## Current Closure

```text
Semantic Program responsibility    ✅ CLOSED
No name re-resolution              ✅ CLOSED
TypeId                              ✅ CLOSED
FunctionId                          ✅ CLOSED
BindingId                           ✅ CLOSED
FieldId                             ✅ CLOSED
VariantId                           ✅ CLOSED
SignatureId                         ✅ CLOSED
SignatureBindingId                  ✅ CLOSED
Identity scopes                     ✅ CLOSED
Semantic identity != VM layout      ✅ CLOSED
Owner-index identity rule           ✅ CLOSED
Semantic owner structures           ✅ CLOSED
SemanticProgram root                ✅ CLOSED
ExternalSymbolId excluded here      ✅ CLOSED

SemanticFunctionBody                ← IN ANALYSIS
SemanticExpression                  ← IN ANALYSIS
Semantic Program exact inventory    ← IN ANALYSIS
```
