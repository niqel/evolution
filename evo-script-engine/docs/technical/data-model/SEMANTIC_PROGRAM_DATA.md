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

## 5. Representation Policy

El uso de `usize` se cierra para v0 porque estas identidades pertenecen exclusivamente al Compilation Working State y naturalmente pueden indexar colecciones temporales del Semantic Program.

Si una versión futura exige serialización del Semantic Program, ABI estable, caché incremental persistente o intercambio cross-process, la representación deberá reabrirse explícitamente.

No se introduce en v0:

```text
UUID semantic identities
String-based semantic identity
Global BindingId namespace
Slot identity inside Semantic Analyzer
Stable cross-compilation IDs
```

## 6. Next Identities in Analysis

Las siguientes identidades todavía requieren análisis antes de cerrarse:

```text
FieldId
VariantId
SignatureId
External Symbol identity
SemanticType
SemanticFunction
SemanticBinding
SemanticExpression
```

La siguiente revisión debe determinar si `FieldId`, `VariantId` y `SignatureId` representan identidades semánticas reales independientes o si alguna puede derivarse con seguridad de su owner + posición sin duplicar conceptos.

## 7. Current Closure

```text
Semantic Program responsibility   ✅ CLOSED
No name re-resolution             ✅ CLOSED
TypeId                             ✅ CLOSED
FunctionId                         ✅ CLOSED
BindingId                          ✅ CLOSED
Identity scope                     ✅ CLOSED
Semantic identity != VM layout     ✅ CLOSED

FieldId / VariantId / SignatureId ← IN ANALYSIS
Semantic Program inventory         ← IN ANALYSIS
```
