# Evo-Script Engine — Semantic Program Data

Status: SEMANTIC PROGRAM DATA — IN ANALYSIS

Este documento define `Semantic Program Data` de `evo-script-engine` v0.

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

## SD-001 — Semantic Program representa significado resuelto

Status: CLOSED

`Semantic Program` es la única Semantic IR de v0 y solo existe después de semantic analysis exitoso.

Regla canónica:

> El Bytecode Compiler no vuelve a resolver nombres. Toda identidad necesaria para generar bytecode llega ya resuelta desde Semantic Program.

Consecuencias:

1. `Semantic Program` no es AST decorado.
2. Imports cumplen su responsabilidad durante semantic resolution y no sobreviven como `SemanticImport` por defecto.
3. Names pueden conservarse como diagnostic/debug metadata, pero nunca como mecanismo de identity resolution dentro del Bytecode Compiler.
4. Semantic Program contiene todo el significado necesario para compilar sin volver a consultar AST ni reconstruir scope/name resolution.
5. Semantic identities no son VM storage identities.
6. `ParameterSlot`, `LocalSlot`, operand layout y otras identidades físicas pertenecen a Bytecode / VM Data, no a Semantic Program.
7. External capabilities continúan separadas de Internal Functions conforme a TD-003 y TD-011.

## SD-002 — Base Semantic Identities

Status: CLOSED

Las identidades semánticas base de v0 son:

```text
TypeId
FunctionId
BindingId
```

Representaciones Rust cerradas:

```rust
struct TypeId(usize);
struct FunctionId(usize);
struct BindingId(usize);
```

Los tres son newtypes opacos. El valor `usize` es representación interna temporal del Compilation Working State; no constituye ABI, formato persistente ni identificador estable entre compilaciones.

No se utilizan aliases como:

```rust
type TypeId = usize;
type FunctionId = usize;
type BindingId = usize;
```

porque permitirían mezclar accidentalmente espacios de identidad distintos.

### TypeId

`TypeId` identifica de forma única un tipo resuelto dentro de un `SemanticProgram`.

El espacio de `TypeId` es universal para el programa semántico e incluye los tipos necesarios para compilar, sin obligar al consumidor a distinguir por nombre entre:

```text
Native Type
Local Struct
Local Enum
Imported Type
Transitively required Type
```

La naturaleza concreta del tipo pertenece a la definición semántica referenciada por `TypeId`.

Invariantes:

1. `TypeId` es único dentro de un `SemanticProgram`.
2. No es estable entre compilaciones distintas.
3. No expresa layout físico ni tamaño de runtime.
4. No es `TypeSlot`, `TypeIndex` de bytecode ni discriminante de VM.
5. Semantic Expressions y Semantic Bindings pueden referenciar su tipo mediante `TypeId`.

### FunctionId

`FunctionId` identifica de forma única una Internal Function resuelta dentro de un `SemanticProgram`.

Flujo canónico:

```text
AST function name
    ↓ Semantic Analyzer
FunctionId
    ↓ Bytecode Compiler
compiled CALL target
```

Invariantes:

1. `FunctionId` es único dentro de un `SemanticProgram`.
2. Solo identifica Internal Functions del programa semántico.
3. No identifica External Signatures ni Providers.
4. No es physical function address.
5. No es stable ABI identifier.
6. El Bytecode Compiler puede transformar/conservar `FunctionId` como identidad de llamada interna sin volver a buscar por nombre.

### BindingId

`BindingId` identifica un Value binding resuelto dentro de una única `SemanticFunction`.

Puede representar bindings originados por:

```text
Value Parameter
Let Binding
Associated when extraction
Structured when extraction
```

Invariantes:

1. `BindingId` es único dentro de su `SemanticFunction`.
2. No necesita unicidad global dentro de todo `SemanticProgram`.
3. Toda referencia semántica a un Value local usa `BindingId`, no el nombre textual como mecanismo de resolución.
4. `BindingId` no distingue físicamente Parameter Slot de Local Slot.
5. Bytecode Compiler transforma bindings hacia el layout físico apropiado cuando construye la función compilada.
6. Un binding extraído por `when` conserva su scope semántico mediante el análisis correspondiente; no requiere `WhenScope` AST/Semantic node adicional por identidad.

## SD-003 — Identity Scope

Status: CLOSED

```text
SemanticProgram
├── TypeId namespace      global to program
└── FunctionId namespace  global to program

SemanticFunction
└── BindingId namespace   local to function
```

La diferencia de scope es intencional.

`TypeId` y `FunctionId` deben poder referenciarse desde cualquier función semántica del programa. `BindingId` solo tiene significado dentro de la función que posee el binding y sus Semantic Expressions.

Esto evita una Global Binding Table sin responsabilidad real y mantiene el Bytecode Compiler trabajando naturalmente una función a la vez.

## SD-004 — Semantic Identity != Physical Layout

Status: CLOSED

Separación obligatoria:

```text
TypeId
    != runtime type layout

FunctionId
    != physical function address

BindingId
    != ParameterSlot / LocalSlot
```

El Semantic Analyzer resuelve significado. El Bytecode Compiler y las fases posteriores deciden representación ejecutable.

Ejemplo:

```text
AST Identifier("worker")
        ↓ Semantic Analyzer
BindingId(2)
        ↓ Bytecode Compiler
LocalSlot(0)          // ejemplo conceptual, no cerrado aquí
```

No se adelanta allocation/layout hacia Semantic Program.

## SD-005 — Secondary Semantic Identities

Status: CLOSED

Se cierran las identidades semánticas secundarias:

```rust
struct FieldId(usize);
struct VariantId(usize);
struct SignatureId(usize);
struct SignatureBindingId(usize);
```

Los cuatro son newtypes opacos del Compilation Working State y no constituyen ABI ni identidades estables entre compilaciones.

### FieldId

`FieldId` identifica un field resuelto dentro de su owner estructural.

Owners válidos en v0:

```text
Semantic Struct Type
Structured Enum Variant
```

Invariantes:

1. `FieldId` es único únicamente dentro de su owner estructural.
2. `FieldId` no requiere unicidad global en `SemanticProgram`.
3. Un field access semánticamente resuelto no depende del nombre textual para compilación.
4. `FieldId` no expresa offset, slot ni layout físico.
5. `FieldId != FieldOffset`.

### VariantId

`VariantId` identifica una variante resuelta dentro de un Semantic Enum Type.

Invariantes:

1. `VariantId` es único únicamente dentro de su enum owner.
2. `VariantId` no requiere unicidad global en `SemanticProgram`.
3. Enum construction y `when` usan identidad de variante resuelta, no `QualifiedName` textual como mecanismo de compilación.
4. `VariantId` no expresa discriminante físico de runtime.
5. `VariantId != runtime discriminant`.

### SignatureId

`SignatureId` identifica una Signature semántica resuelta dentro de `SemanticProgram`.

Invariantes:

1. `SignatureId` es único dentro de `SemanticProgram`.
2. Representa la definición semántica de una capability/signature, no una dependencia local concreta.
3. No identifica Provider.
4. No identifica binding de ejecución.
5. No es External Symbol identity de Compiled Program.

### SignatureBindingId

`SignatureBindingId` identifica una Signature Dependency concreta dentro de una única `SemanticFunction`.

Ejemplo conceptual:

```text
workers::search primary_search,
workers::search fallback_search
```

puede resolver a:

```text
SignatureId(3)
├── SignatureBindingId(0) primary_search
└── SignatureBindingId(1) fallback_search
```

Invariantes:

1. `SignatureBindingId` es único dentro de su `SemanticFunction`.
2. Cada Signature Binding referencia exactamente un `SignatureId` resuelto.
3. `SignatureBindingId` no es `BindingId`: Signature Dependencies no son Value bindings.
4. Una external call semántica puede referenciar `SignatureBindingId` sin volver a resolver el nombre local.
5. Runtime binding y Provider resolution permanecen fuera de Semantic Program.

## SD-006 — Expanded Identity Scope

Status: CLOSED

```text
SemanticProgram
├── TypeId namespace            global to program
├── FunctionId namespace        global to program
└── SignatureId namespace       global to program

SemanticFunction
├── BindingId namespace         local to function
└── SignatureBindingId namespace local to function

Semantic Struct / Structured Variant
└── FieldId namespace           local to structural owner

Semantic Enum
└── VariantId namespace         local to enum owner
```

Los espacios son conceptualmente independientes aun cuando todos utilicen `usize` internamente.

## SD-007 — Semantic identities remain distinct from compiled/runtime identities

Status: CLOSED

Separación obligatoria:

```text
FieldId
    != FieldOffset / physical layout

VariantId
    != runtime discriminant

SignatureId
    != runtime external binding

SignatureBindingId
    != ExternalSymbolId
    != Provider binding
```

`ExternalSymbolId` no se introduce en Semantic Program Data v0. Si Compiled Program requiere una identidad compacta para símbolos externos, dicha identidad pertenece a `Compiled Program / Bytecode Data` y será creada por Bytecode Compiler a partir del significado semántico ya resuelto.

## 8. Representation Policy

El uso de `usize` se cierra para v0 porque estas identidades pertenecen exclusivamente al Compilation Working State y naturalmente pueden indexar colecciones temporales del Semantic Program.

Si una versión futura exige serialización del Semantic Program, ABI estable, caché incremental persistente o intercambio cross-process, la representación deberá reabrirse explícitamente.

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

## 9. Next Identities in Analysis

Las siguientes estructuras requieren ahora análisis:

```text
SemanticType
SemanticField
SemanticVariant
SemanticSignature
SemanticFunction
SemanticBinding
SemanticSignatureBinding
SemanticExpression
SemanticProgram root shape
```

La siguiente revisión debe definir primero las estructuras owner de las identidades ya cerradas antes de diseñar el árbol completo de Semantic Expressions.

## 10. Current Closure

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
ExternalSymbolId excluded here      ✅ CLOSED

Semantic owner structures          ← IN ANALYSIS
Semantic Expression model          PENDING
Semantic Program inventory         ← IN ANALYSIS
```
