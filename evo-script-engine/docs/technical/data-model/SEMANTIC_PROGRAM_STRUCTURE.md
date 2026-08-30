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

Esto no convierte los IDs en layout de Runtime. Continúa aplicando:

```text
FieldId             != FieldOffset
VariantId           != runtime discriminant
BindingId           != ParameterSlot / LocalSlot
SignatureBindingId  != ExternalSymbolId / runtime binding
```

## 2. NativeType

En Semantic Program sí existe una identidad explícita para los tipos nativos porque la resolución de tipos ya concluyó.

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

`NativeType` no pertenece al AST. El flujo es:

```text
AST Identifier("int")
    ↓ Semantic Analyzer
TypeId
    ↓
SemanticType::Native(NativeType::Int)
```

## 3. SemanticType

Representación cerrada:

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

No se introduce `SemanticTypeKind`: la propia variant Rust expresa la naturaleza semántica resuelta.

`TypeId` puede identificar Native, local Struct, local Enum, imported shared Type o tipo transitivamente requerido; el origen textual no crea categorías adicionales en esta IR.

## 4. SemanticField

Representación cerrada:

```rust
struct SemanticField {
    type_id: TypeId,
}
```

El nombre textual del field se utiliza durante Semantic Analysis para resolver `FieldId`, pero no es mecanismo de resolución para Bytecode Compiler.

La identidad del field proviene de su posición en el owner:

```text
owner.fields[FieldId]
```

La misma estructura se reutiliza para:

```text
Struct fields
Structured Enum Variant fields
```

porque después de resolución ambas expresan exactamente un field de datos con tipo semántico resuelto.

## 5. SemanticVariant

Representación cerrada:

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

La identidad de la variante proviene de:

```text
semantic_enum.variants[VariantId]
```

`VariantId` no es discriminante de runtime. Bytecode Compiler decide posteriormente la representación ejecutable.

## 6. SemanticBinding

Representación cerrada:

```rust
struct SemanticBinding {
    type_id: TypeId,
}
```

`SemanticFunction.bindings` contiene Value bindings resueltos originados por:

```text
Value Parameter
Let Binding
Associated when extraction
Structured when extraction
```

`BindingId` identifica el elemento dentro de esta colección.

No se almacena aquí Parameter/Local Slot ni otra identidad física.

## 7. SemanticSignatureBinding

Representación cerrada:

```rust
struct SemanticSignatureBinding {
    signature: SignatureId,
}
```

Cada elemento representa una dependencia concreta de Signature dentro de una función. Dos dependencies diferentes pueden referenciar el mismo `SignatureId`.

```text
SignatureId
├── SignatureBindingId A
└── SignatureBindingId B
```

No es un Value binding ni un runtime external binding.

## 8. SemanticParameter

La lista de parámetros de una Function Implementation conserva una única secuencia posicional común para Values y Signature Dependencies.

Representación cerrada:

```rust
enum SemanticParameter {
    Value(BindingId),
    SignatureDependency(SignatureBindingId),
}
```

Esto preserva el orden original necesario para llamadas y forwarding sin convertir una Signature Dependency en Value de primer orden.

No se separan parámetros en dos listas independientes.

## 9. SemanticSignatureParameter

Una Signature define contrato, no bindings locales ejecutables.

Representación cerrada:

```rust
enum SemanticSignatureParameter {
    Value(TypeId),
    SignatureDependency(SignatureId),
}
```

No utiliza `BindingId` ni `SignatureBindingId` porque una definición de Signature no posee Function Body ni lexical bindings locales.

## 10. SemanticSignature

Representación cerrada:

```rust
struct SemanticSignature {
    parameters: Vec<SemanticSignatureParameter>,
    result_type: TypeId,
}
```

Expresa únicamente el contrato semántico resuelto necesario para validar/compilar invocaciones y dependency forwarding.

No contiene Provider, runtime binding ni ExternalSymbolId.

## 11. SemanticFunction

Representación cerrada para el shell estructural de una función semántica:

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

`SemanticFunctionBody` queda como identidad requerida pero su representación interna se cierra en el siguiente bloque junto con Semantic Expressions.

Invariantes:

1. `parameters` preserva el orden posicional único de Value Parameters y Signature Dependency Parameters.
2. `bindings` es el owner del namespace `BindingId` de la función.
3. `signature_bindings` es el owner del namespace `SignatureBindingId` de la función.
4. `result_type` ya está completamente resuelto.
5. `satisfaction` conserva la Signature resuelta que la función declara satisfacer, cuando existe.
6. No contiene visibility como mecanismo de entry selection; el entry resuelto vive en `SemanticProgram.entry_function`.
7. No contiene slots, bytecode, external runtime bindings ni physical addresses.

## 12. SemanticProgram

Representación raíz cerrada:

```rust
struct SemanticProgram {
    types: Vec<SemanticType>,
    signatures: Vec<SemanticSignature>,
    functions: Vec<SemanticFunction>,
    entry_function: FunctionId,
}
```

Relaciones:

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

## 13. Names and diagnostic metadata

Las representaciones cerradas anteriores no requieren names para resolution. Si Source Mapping / diagnostics demuestra posteriormente la necesidad de conservar nombres semánticos como metadata, podrán agregarse como diagnostic metadata sin convertir strings nuevamente en identities.

La regla permanece:

> Bytecode Compiler nunca depende de nombres textuales para resolver Type, Function, Binding, Field, Variant o Signature identity.

## 14. Explicitly Excluded

```text
SemanticTypeKind
id field duplicated inside SemanticType
id field duplicated inside SemanticFunction
id field duplicated inside SemanticSignature
id field duplicated inside SemanticField / SemanticVariant
SemanticImport
Global Binding table
Provider identity
ExternalSymbolId
FieldOffset
runtime discriminant
ParameterSlot
LocalSlot
bytecode / Opcode
```

## 15. Closure

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
SemanticSignature                  ✅ CLOSED
SemanticFunction shell             ✅ CLOSED
SemanticProgram root               ✅ CLOSED

SemanticFunctionBody               ← NEXT
SemanticExpression                 ← NEXT
```
