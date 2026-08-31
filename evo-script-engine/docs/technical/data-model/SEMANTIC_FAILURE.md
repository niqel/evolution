# Evo-Script Engine — SemanticFailure

Status: CLOSED

Este documento cierra la familia técnica exacta `SemanticFailure` para `evo-script-engine` v0.

`SemanticFailure` pertenece exclusivamente al Semantic Analyzer y expresa invalidez de significado resuelto sobre un AST sintácticamente válido, usando un `CompilationCatalog` válido y ya construido fuera del Engine.

No representa failures de Lexer, Parser, filesystem, `.elib`, `.emod`, construcción del catálogo, ejecución VM ni `ExternalCapability`.

## Canonical root shape

```rust
enum SemanticFailure {
    Resolution(ResolutionFailure),
    Declaration(DeclarationFailure),
    TypeChecking(TypeCheckingFailure),
    Call(CallFailure),
    Composite(CompositeFailure),
    When(WhenFailure),
    SignatureMismatch {
        signature: SignatureSymbol,
        mismatch: SignatureMismatchKind,
    },
}
```

Inventario exacto del root:

```text
SemanticFailure variants = 7
```

## Exact supporting identities

La familia introduce exactamente doce identities técnicas propias:

```text
01 SemanticFailure
02 ResolutionFailure
03 DeclarationFailure
04 TypeCheckingFailure
05 CallFailure
06 CompositeFailure
07 WhenFailure
08 SignatureMismatchKind
09 SemanticTypeDescriptor
10 SemanticNameRole
11 SemanticArgumentKind
12 EnumPayloadShape
```

Reutiliza identities ya existentes y no las vuelve a contar:

```text
NativeType
TypeSymbol
SignatureSymbol
UnaryOperator
BinaryOperator
```

No sobreviven como payloads de outcome:

```text
TypeId
BindingId
FieldId
VariantId
SignatureId
SemanticProgram references
AST references
CompilationCatalog references
```

## ResolutionFailure

```rust
enum ResolutionFailure {
    ImportedSymbolNotFound {
        module: Box<str>,
        name: Box<str>,
    },

    UnknownType {
        name: Box<str>,
    },

    UnknownValueSymbol {
        name: Box<str>,
    },

    UnknownSignature(
        SignatureSymbol,
    ),
}
```

```text
ResolutionFailure variants = 4
```

Responsabilidad:

```text
syntactically valid name
    ↓
Semantic Analyzer
    ↓
cannot resolve to required semantic identity
```

`ImportedSymbolNotFound` aplica cuando el Source Text solicita explícitamente un símbolo que no existe en el `CompilationCatalog` válido entregado al Engine.

No representa:

```text
ModuleNotFoundError
LibraryArtifactNotFoundError
filesystem resolution failure
catalog construction failure
```

Esas condiciones pertenecen al componente externo que construye/valida `CompilationCatalog`.

## DeclarationFailure

```rust
enum DeclarationFailure {
    TypeNameCollision {
        name: Box<str>,
    },

    DuplicateFunction {
        name: Box<str>,
    },

    DuplicateField {
        name: Box<str>,
    },

    DuplicateVariant {
        name: Box<str>,
    },

    BindingNameCollision {
        name: Box<str>,
    },

    InvalidNamingConvention {
        role: SemanticNameRole,
    },

    RecursiveTypeCycle,
}
```

```text
DeclarationFailure variants = 7
```

`BindingNameCollision` cubre la regla general de no-shadowing para bindings visibles, incluidos bindings introducidos por parámetros y extracción de `when`.

`RecursiveTypeCycle` cubre ciclos directos, indirectos y mixtos struct/enum. No introduce paths/cycle traces como payload canónico v0.

### SemanticNameRole

```rust
enum SemanticNameRole {
    Type,
    Variant,
    Field,
    Function,
    Binding,
    SignatureAlias,
    SignatureDependency,
}
```

```text
SemanticNameRole variants = 7
```

Permite conservar la identidad normativa de convenciones `PascalCase` / `snake_case` sin crear un failure diferente por cada clase de símbolo.

## SemanticTypeDescriptor

El outcome no conserva `TypeId`, porque `TypeId` es una identidad local al `SemanticProgram` / working state de una compilación concreta.

La información de tipo que deba sobrevivir a una compilación fallida se materializa como dato owned:

```rust
enum SemanticTypeDescriptor {
    Native(NativeType),
    Local(Box<str>),
    Shared(TypeSymbol),
}
```

```text
SemanticTypeDescriptor variants = 3
```

Relación:

```text
TypeId
    ↓ while Semantic Analysis is alive
resolve descriptive identity
    ↓
SemanticTypeDescriptor
    ↓
CompileFailure owns it
```

Esto es metadata diagnóstica de compilación, no runtime reflection ni runtime type identity.

## TypeCheckingFailure

```rust
enum TypeCheckingFailure {
    BindingInitialization {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },

    FunctionResult {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },

    NumericLiteralNotRepresentable {
        expected: SemanticTypeDescriptor,
    },

    UnaryOperator {
        operator: UnaryOperator,
        operand: SemanticTypeDescriptor,
    },

    ArithmeticOperator {
        operator: BinaryOperator,
        left: SemanticTypeDescriptor,
        right: SemanticTypeDescriptor,
    },

    LogicalOperator {
        operator: BinaryOperator,
        left: SemanticTypeDescriptor,
        right: SemanticTypeDescriptor,
    },

    Comparison {
        operator: BinaryOperator,
        left: SemanticTypeDescriptor,
        right: SemanticTypeDescriptor,
    },

    InvalidConversion {
        source: SemanticTypeDescriptor,
        target: SemanticTypeDescriptor,
    },
}
```

```text
TypeCheckingFailure variants = 8
```

`NumericLiteralNotRepresentable` es estático y contextual. Un literal numérico puede ser lexicalmente válido y, sin embargo, no ser representable en el tipo explícito requerido.

`InvalidConversion` significa que la operación `to_tipo` no está definida semánticamente entre esas familias de tipos.

No debe confundirse con `ConversionError`, que ocurre en runtime para una conversión semánticamente válida cuyo valor concreto no puede representarse exactamente.

## CallFailure

```rust
enum CallFailure {
    FunctionNotFound {
        name: Box<str>,
    },

    AmbiguousTarget {
        name: Box<str>,
    },

    ArityMismatch {
        expected: usize,
        actual: usize,
    },

    ArgumentKindMismatch {
        position: usize,
        expected: SemanticArgumentKind,
        actual: SemanticArgumentKind,
    },

    ArgumentTypeMismatch {
        position: usize,
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },

    SignatureDependencyMismatch {
        position: usize,
        expected: SignatureSymbol,
        actual: SignatureSymbol,
    },

    FunctionCallCycle,
}
```

```text
CallFailure variants = 7
```

`FunctionNotFound` aplica a una expresión de llamada cuyo nombre no resuelve a ningún target invocable disponible.

`AmbiguousTarget` aplica cuando una llamada no calificada coincide simultáneamente con targets incompatibles, por ejemplo Function Implementation local + Signature importada con el mismo nombre local, sin alias que desambigüe.

No existe prioridad implícita `local wins`, `import wins` ni `first wins`.

### SemanticArgumentKind

```rust
enum SemanticArgumentKind {
    Value,
    SignatureDependency,
}
```

```text
SemanticArgumentKind variants = 2
```

La distinción es necesaria porque una Signature Dependency no es un Value de primer orden.

`ArgumentTypeMismatch` aplica a Value Parameters.

`SignatureDependencyMismatch` aplica cuando ambas posiciones son Signature Dependencies pero las identidades contractuales son distintas.

`FunctionCallCycle` representa recursión directa o cualquier ciclo indirecto en el grafo local de llamadas. No introduce un cycle path canónico en v0.

## CompositeFailure

```rust
enum CompositeFailure {
    ExpectedStruct {
        actual: SemanticTypeDescriptor,
    },

    ExpectedEnum {
        actual: SemanticTypeDescriptor,
    },

    FieldAccessType {
        actual: SemanticTypeDescriptor,
    },

    FieldNotFound {
        field: Box<str>,
    },

    MissingField {
        field: Box<str>,
    },

    DuplicateFieldInitializer {
        field: Box<str>,
    },

    FieldTypeMismatch {
        field: Box<str>,
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },

    VariantNotFound {
        variant: Box<str>,
    },

    VariantPayloadShapeMismatch {
        expected: EnumPayloadShape,
        actual: EnumPayloadShape,
    },

    AssociatedPayloadTypeMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
}
```

```text
CompositeFailure variants = 10
```

La familia centraliza validación semántica de:

```text
Struct construction
Enum construction
Field Access
```

Los fields se identifican por nombre y las construcciones deben satisfacer exactamente cardinalidad, unicidad y tipo declarado.

### EnumPayloadShape

```rust
enum EnumPayloadShape {
    Simple,
    Associated,
    Structured,
}
```

```text
EnumPayloadShape variants = 3
```

Esta identity describe únicamente la forma contractual necesaria para explicar una failure semántica.

No reutiliza como authority:

```text
SemanticVariant
CompiledEnumValueShape
RuntimeEnumPayload
```

porque pertenecen a otras fases y responsabilidades.

## WhenFailure

```rust
enum WhenFailure {
    SubjectNotEnum {
        actual: SemanticTypeDescriptor,
    },

    PatternEnumMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },

    VariantNotFound {
        variant: Box<str>,
    },

    DuplicateVariantCorrespondence {
        variant: Box<str>,
    },

    NonExhaustive {
        missing: Vec<Box<str>>,
    },

    PayloadShapeMismatch {
        expected: EnumPayloadShape,
        actual: EnumPayloadShape,
    },

    FieldNotFound {
        field: Box<str>,
    },

    DuplicateField {
        field: Box<str>,
    },

    MissingField {
        field: Box<str>,
    },

    ExtractionTypeMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },

    BranchResultTypeMismatch {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
}
```

```text
WhenFailure variants = 11
```

`NonExhaustive.missing` conserva todas las variantes faltantes en el orden canónico de declaración del Enum; por ello el payload es determinista.

Las colisiones de nombres de bindings extraídos por `when` reutilizan:

```text
DeclarationFailure::BindingNameCollision
```

No se introduce un `WhenShadowingError` separado.

## SignatureMismatchKind

```rust
enum SignatureMismatchKind {
    FunctionName,

    ParameterCount {
        expected: usize,
        actual: usize,
    },

    ParameterKind {
        position: usize,
        expected: SemanticArgumentKind,
        actual: SemanticArgumentKind,
    },

    ValueParameterType {
        position: usize,
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },

    SignatureDependency {
        position: usize,
        expected: SignatureSymbol,
        actual: SignatureSymbol,
    },

    ResultType {
        expected: SemanticTypeDescriptor,
        actual: SemanticTypeDescriptor,
    },
}
```

```text
SignatureMismatchKind variants = 6
```

El root conserva la Signature canónica que se declara satisfacer:

```rust
SemanticFailure::SignatureMismatch {
    signature: SignatureSymbol,
    mismatch: SignatureMismatchKind,
}
```

La validation identifica la primera dimensión contractual exacta que no coincide.

El orden de parámetros no requiere una variant independiente: la primera posición cuya clase/tipo/Signature no coincide expresa de forma determinista la divergencia posicional.

## Mapping from normative System / Validation errors

Los nombres normativos de la especificación se preservan semánticamente mediante este mapping técnico:

```text
FunctionNotFoundError
    → SemanticFailure::Call(CallFailure::FunctionNotFound)

FunctionArityError
    → SemanticFailure::Call(CallFailure::ArityMismatch)

FunctionArgumentTypeError
    → SemanticFailure::Call(CallFailure::ArgumentTypeMismatch)

DuplicateFunctionError
    → SemanticFailure::Declaration(DeclarationFailure::DuplicateFunction)

FunctionCallCycleError
    → SemanticFailure::Call(CallFailure::FunctionCallCycle)

FieldNotFoundError
    → SemanticFailure::Composite(CompositeFailure::FieldNotFound)

FieldAccessTypeError
    → SemanticFailure::Composite(CompositeFailure::FieldAccessType)

ComparisonTypeError
    → SemanticFailure::TypeChecking(TypeCheckingFailure::Comparison)

RecursiveTypeCycleError
    → SemanticFailure::Declaration(DeclarationFailure::RecursiveTypeCycle)

TypeNameCollisionError
    → SemanticFailure::Declaration(DeclarationFailure::TypeNameCollision)

SignatureMismatchError
    → SemanticFailure::SignatureMismatch { ... }
```

No se introduce un enum técnico universal `SystemError` dentro del sistema de Values ni como error capturable de Evo-Script.

## Excluded physical / catalog-construction failures

No pertenecen a `SemanticFailure`:

```text
LibraryArtifactPathError
LibraryArtifactNotFoundError
DuplicateLibraryArtifactError
ModuleBoundaryError
DuplicateModuleError
ModuleNotFoundError
ModuleSymbolNotFoundError
DuplicateModuleSymbolError
filesystem I/O failures
catalog construction failures
```

La frontera canónica es:

```text
Physical/module resolution
    ↓
valid CompilationCatalog
    ↓
evo-script-engine Semantic Analyzer
    ↓
SemanticProgram OR SemanticFailure
```

## SEF-001 — Exactly seven semantic root families

Status: CLOSED

`SemanticFailure` posee exactamente siete root variants:

```text
Resolution
Declaration
TypeChecking
Call
Composite
When
SignatureMismatch
```

## SEF-002 — Exactly twelve own technical identities

Status: CLOSED

El modelo exacto introduce doce identities técnicas propias:

```text
SemanticFailure
ResolutionFailure
DeclarationFailure
TypeCheckingFailure
CallFailure
CompositeFailure
WhenFailure
SignatureMismatchKind
SemanticTypeDescriptor
SemanticNameRole
SemanticArgumentKind
EnumPayloadShape
```

Las identities reutilizadas desde AST/Semantic/Catalog no se cuentan nuevamente.

## SEF-003 — No SemanticProgram-local IDs escape in failure

Status: CLOSED

`TypeId`, `BindingId`, `FieldId`, `VariantId`, `SignatureId` y referencias al working state no sobreviven dentro de `CompileFailure`.

La información de tipo necesaria se materializa como `SemanticTypeDescriptor` owned.

## SEF-004 — Exact ResolutionFailure family

Status: CLOSED

`ResolutionFailure` contiene exactamente:

```text
ImportedSymbolNotFound
UnknownType
UnknownValueSymbol
UnknownSignature
```

## SEF-005 — Exact DeclarationFailure family

Status: CLOSED

`DeclarationFailure` contiene exactamente:

```text
TypeNameCollision
DuplicateFunction
DuplicateField
DuplicateVariant
BindingNameCollision
InvalidNamingConvention
RecursiveTypeCycle
```

## SEF-006 — Exact TypeCheckingFailure family

Status: CLOSED

`TypeCheckingFailure` contiene exactamente:

```text
BindingInitialization
FunctionResult
NumericLiteralNotRepresentable
UnaryOperator
ArithmeticOperator
LogicalOperator
Comparison
InvalidConversion
```

## SEF-007 — Exact CallFailure family

Status: CLOSED

`CallFailure` contiene exactamente:

```text
FunctionNotFound
AmbiguousTarget
ArityMismatch
ArgumentKindMismatch
ArgumentTypeMismatch
SignatureDependencyMismatch
FunctionCallCycle
```

## SEF-008 — CompositeFailure centralizes composite validation

Status: CLOSED

`CompositeFailure` centraliza Struct construction, Enum construction y Field Access y posee exactamente diez variants cerradas en este documento.

No se duplica una familia de field errors por cada composite syntax.

## SEF-009 — WhenFailure owns exact when semantic invariants

Status: CLOSED

`WhenFailure` posee exactamente once variants y cubre las invariantes semánticas exclusivas de correspondencia exhaustiva de Enums.

La regla general de no-shadowing reutiliza `DeclarationFailure::BindingNameCollision`.

## SEF-010 — SignatureMismatch preserves canonical contract identity

Status: CLOSED

`SemanticFailure::SignatureMismatch` conserva el `SignatureSymbol` formal y usa `SignatureMismatchKind` para registrar la primera dimensión contractual exacta que difiere.

## SEF-011 — Normative errors map deterministically without universal SystemError

Status: CLOSED

Los nombres normativos `FunctionNotFoundError`, `FunctionArityError`, `ComparisonTypeError`, `SignatureMismatchError`, etc. mapean determinísticamente a las families técnicas cerradas.

No se introduce `SystemError` como Value, catchable error o enum universal de outcome.

## SEF-012 — Physical/catalog failures are outside SemanticFailure

Status: CLOSED

`SemanticFailure` comienza únicamente después de que el Engine recibe un `CompilationCatalog` válido.

Failures de filesystem, Active Library, physical module boundaries, publicación modular o construcción del catálogo pertenecen al componente externo responsable de producir ese catálogo.

## Exact counts

```text
SemanticFailure variants             7
ResolutionFailure variants           4
DeclarationFailure variants          7
TypeCheckingFailure variants         8
CallFailure variants                 7
CompositeFailure variants           10
WhenFailure variants                11
SignatureMismatchKind variants       6
SemanticTypeDescriptor variants      3
SemanticNameRole variants            7
SemanticArgumentKind variants        2
EnumPayloadShape variants            3

SemanticFailure own identities      12
```

## Phase boundary

```text
AST
 +
CompilationCatalog
    ↓
Semantic Analyzer
    ├── Success
    │      ↓
    │ SemanticProgram
    │
    └── Failure
           ↓
      SemanticFailure
           ↓
      CompileFailure {
          kind: Semantic(...),
          diagnostic: ...,
      }
```

## Explicitly not introduced

```text
SystemError as universal technical enum
SemanticFailure<'source>
TypeId / SignatureId in public failure
AST / SemanticProgram / Catalog borrows in public failure
runtime reflection metadata
one error identity per semantic rule line
ModuleNotFound / LibraryArtifactNotFound inside Engine SemanticFailure
WhenShadowingError duplicate
StructFieldNotFound + EnumFieldNotFound duplicated families
cycle path payload requirement
preformatted human message as canonical semantic error
```

## Closure

```text
SEF-001 exactly seven root families                                ✅ CLOSED
SEF-002 exactly twelve own technical identities                   ✅ CLOSED
SEF-003 no SemanticProgram-local IDs escape                       ✅ CLOSED
SEF-004 exact ResolutionFailure                                   ✅ CLOSED — 4 variants
SEF-005 exact DeclarationFailure                                  ✅ CLOSED — 7 variants
SEF-006 exact TypeCheckingFailure                                 ✅ CLOSED — 8 variants
SEF-007 exact CallFailure                                         ✅ CLOSED — 7 variants
SEF-008 CompositeFailure exact family                             ✅ CLOSED — 10 variants
SEF-009 WhenFailure exact family                                  ✅ CLOSED — 11 variants
SEF-010 SignatureMismatch exact contract dimension                ✅ CLOSED — 6 mismatch variants
SEF-011 deterministic normative-error mapping                     ✅ CLOSED
SEF-012 physical/catalog failures excluded                        ✅ CLOSED

SemanticFailure exact family                                      ✅ CLOSED — 12 own identities
CompileFailure exact subfamilies                                  ✅ CLOSED
ExecutionFailure exact family                                     ← NEXT
DiagnosticAnchor exact shape                                      PENDING
```