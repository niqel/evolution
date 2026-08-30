# Evo-Script Engine — Semantic Program Structure

Status: CLOSED

Este documento cierra las estructuras owner principales de `Semantic Program Data` para `evo-script-engine` v0.

La autoridad deriva de `SEMANTIC_PROGRAM_DATA.md`, `TECHNICAL_DESIGN.md` y la especificación Evo-Script v0.1.

## 1. Owner-index identity rule

Los IDs semánticos cerrados identifican elementos por su posición dentro de la colección de su owner. El elemento no almacena nuevamente su propio ID.

```text
TypeId(n)              → SemanticProgram.types[n]
FunctionId(n)          → SemanticProgram.functions[n]
SignatureId(n)         → SemanticProgram.signatures[n]

FieldId(n)             → structural_owner.fields[n]
VariantId(n)           → semantic_enum.variants[n]

BindingId(n)           → SemanticFunction.bindings[n]
SignatureBindingId(n)  → SemanticFunction.signature_bindings[n]
```

Regla:

> El owner es la única fuente de verdad para la relación índice-identidad; no se duplica `id` dentro del elemento referenciado.

Esto no convierte los IDs en layout de Runtime.

```text
FieldId             != FieldOffset
VariantId           != runtime discriminant
BindingId           != ParameterSlot / LocalSlot
SignatureBindingId  != ExternalSymbolId / runtime binding
```

## 2. NativeType

En Semantic Program sí existe una identidad explícita para tipos nativos porque type resolution ya concluyó.

```rust
enum NativeType {
    Int,
    Float,
    Bool,
    String,
    Dynamic,

    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,

    Float32,
    Float64,
}
```

`NativeType` no pertenece al AST.

```text
AST Identifier("int")
    ↓ Semantic Analyzer
TypeId
    ↓
SemanticType::Native(NativeType::Int)
```

## 3. SemanticType

```rust
enum SemanticType {
    Native(NativeType),
    Struct {
        fields: Vec<SemanticField>,
    },
    Enum {
        variants: Vec<SemanticVariant>,
    },
}
```

No se introduce `SemanticTypeKind`: la propia variant expresa la naturaleza resuelta.

`TypeId` puede identificar Native, local Struct, local Enum, imported shared Type o tipo transitivamente requerido. El origen textual no crea categorías adicionales.

## 4. SemanticField

```rust
struct SemanticField {
    type_id: TypeId,
}
```

El nombre textual se utiliza durante Semantic Analysis para resolver `FieldId`, pero no es mecanismo de resolución para Bytecode Compiler.

La misma estructura se reutiliza para Struct fields y Structured Enum Variant fields.

## 5. SemanticVariant

```rust
enum SemanticVariant {
    Simple,
    Associated {
        type_id: TypeId,
    },
    Structured {
        fields: Vec<SemanticField>,
    },
}
```

La identidad proviene de `semantic_enum.variants[VariantId]`.

`VariantId != runtime discriminant`.

## 6. SemanticBinding

```rust
struct SemanticBinding {
    type_id: TypeId,
}
```

`SemanticFunction.bindings` contiene Value bindings originados por:

```text
Value Parameter
Let Binding
Associated when extraction
Structured when extraction
```

No contiene Parameter/Local Slot.

## 7. SemanticSignatureBinding

```rust
struct SemanticSignatureBinding {
    signature: SignatureId,
}
```

Cada elemento representa una Signature Dependency concreta dentro de una función. Dos dependencies distintas pueden referenciar el mismo `SignatureId`.

No es Value binding ni runtime external binding.

## 8. SemanticParameter

La lista de parámetros de una Function Implementation conserva una sola secuencia posicional común para Values y Signature Dependencies.

```rust
enum SemanticParameter {
    Value(BindingId),
    SignatureDependency(SignatureBindingId),
}
```

No se separan los parámetros en listas independientes.

## 9. SemanticSignatureParameter

Una Signature define contrato, no lexical bindings ejecutables.

```rust
enum SemanticSignatureParameter {
    Value(TypeId),
    SignatureDependency(SignatureId),
}
```

No utiliza `BindingId` ni `SignatureBindingId`.

## 10. SignatureSymbol

La revisión del Semantic Program confirmó que una Signature externa necesita conservar su identidad contractual canónica para que Bytecode Compiler pueda construir el External Symbol persistente sin volver al AST o imports.

```rust
struct SignatureSymbol {
    module: String,
    name: String,
}
```

Ejemplo:

```text
values::search
    ↓ Semantic Analyzer
SignatureId(3)
    ↓
SemanticProgram.signatures[3].symbol
    → SignatureSymbol { module: "values", name: "search" }
```

Separación obligatoria:

```text
SignatureId
    = identidad compacta dentro de SemanticProgram

SignatureSymbol
    = identidad contractual canónica necesaria para producir
      el símbolo externo persistente

local import alias
local Signature Dependency name
    = no sobreviven como mecanismo de resolución
```

`SignatureSymbol` no es `ExternalSymbolId`, Provider identity ni runtime binding.

## 11. SemanticSignature

```rust
struct SemanticSignature {
    symbol: SignatureSymbol,
    parameters: Vec<SemanticSignatureParameter>,
    result_type: TypeId,
}
```

`symbol` conserva la identidad formal `module::signature`. La resolución interna continúa utilizando `SignatureId`.

La Signature no contiene Provider, runtime binding ni `ExternalSymbolId`.

## 12. SemanticFunction

```rust
struct SemanticFunction {
    parameters: Vec<SemanticParameter>,
    bindings: Vec<SemanticBinding>,
    signature_bindings: Vec<SemanticSignatureBinding>,
    result_type: TypeId,
    satisfaction: Option<SignatureId>,
    body: SemanticFunctionBody,
}
```

Invariantes:

1. `parameters` preserva el orden posicional único de Value Parameters y Signature Dependency Parameters.
2. `bindings` es owner del namespace `BindingId`.
3. `signature_bindings` es owner del namespace `SignatureBindingId`.
4. `result_type` ya está resuelto.
5. `satisfaction` conserva la Signature resuelta que la función declara satisfacer, cuando existe.
6. Entry selection vive en `SemanticProgram.entry_function`, no en visibility textual.
7. No contiene slots, bytecode, runtime bindings ni physical addresses.

## 13. SemanticProgram

```rust
struct SemanticProgram {
    types: Vec<SemanticType>,
    signatures: Vec<SemanticSignature>,
    functions: Vec<SemanticFunction>,
    entry_function: FunctionId,
}
```

```text
SemanticProgram
├── types: Vec<SemanticType>
│      ├── Native
│      ├── Struct
│      │    └── SemanticField 0..N
│      └── Enum
│           └── SemanticVariant 1..N
│
├── signatures: Vec<SemanticSignature>
│      └── SignatureSymbol
│
├── functions: Vec<SemanticFunction>
│      ├── parameters
│      ├── bindings
│      ├── signature_bindings
│      ├── result_type
│      ├── satisfaction 0..1
│      └── body
│
└── entry_function: FunctionId
```

Semantic Program no conserva por defecto:

```text
imports
local aliases
name lookup tables
AST declarations
Providers
runtime capability bindings
ExternalSymbolId
Constant Pool
bytecode
ParameterSlot / LocalSlot
operand layout
runtime discriminants
field offsets
```

## 14. Names and symbolic identity boundary

Los nombres locales no sobreviven como mecanismo de resolution. La excepción intencional es `SignatureSymbol`, porque `module::signature` es identidad contractual externa que debe persistir hacia Compiled Program.

Regla:

> Bytecode Compiler nunca usa strings para resolver Type, Function, Binding, Field, Variant o Signature dentro de Semantic Program; solo materializa `SignatureSymbol` al producir external-symbol data.

## 15. Explicitly Excluded

```text
SemanticTypeKind
id field duplicated inside owner elements
SemanticImport
Global Binding table
Provider identity
ExternalSymbolId
FieldOffset
runtime discriminant
ParameterSlot
LocalSlot
bytecode / Opcode
local import alias persistence
local Signature Dependency name persistence
```

## 16. Closure

```text
Owner-index identity rule          ✅ CLOSED
NativeType                         ✅ CLOSED
SemanticType                       ✅ CLOSED
SemanticField                      ✅ CLOSED
SemanticVariant                    ✅ CLOSED
SemanticBinding                    ✅ CLOSED
SemanticSignatureBinding           ✅ CLOSED
SemanticParameter                  ✅ CLOSED
SemanticSignatureParameter         ✅ CLOSED
SignatureSymbol                    ✅ CLOSED
SemanticSignature                  ✅ CLOSED
SemanticFunction shell             ✅ CLOSED
SemanticProgram root               ✅ CLOSED
```
