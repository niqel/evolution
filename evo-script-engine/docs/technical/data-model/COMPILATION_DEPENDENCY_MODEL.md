# Evo-Script Engine — Compilation Dependency Model

Status: CLOSED — CORRECTIVE BOUNDARY MODEL

Este documento corrige una dependencia técnica necesaria de `Compile` sin reabrir su Functional Use Case.

La necesidad demostrada es:

```text
Source Text
    ↓
AST import qualifier::symbol
    ↓
Semantic Analyzer
    ↓ requires contract data for imported shared Types / Signatures
```

`SignatureSymbol` identifica una Signature externa, pero por sí solo no contiene sus Parameters, result Type ni el cierre de shared Types requerido para realizar semantic validation.

Al mismo tiempo, `UC-001 — Compile` permanece funcionalmente cerrado:

```text
Functional input
    = exactly one Source Text

Compile does not perform
    filesystem I/O
    path resolution
    Provider discovery
    runtime binding
```

La corrección introduce `CompilationCatalog` como **dependencia técnica explícita, borrowed e inmutable**, construida fuera de `evo-script-engine`.

## Canonical relation

```text
Physical / Module Resolution
outside evo-script-engine
        │
        ▼
CompilationCatalog
validated semantic contract catalog
        │ explicit immutable borrow
        ▼
Compile
├── functional input: Source Text
└── technical dependency: CompilationCatalog
        │
        ▼
Semantic Analyzer
        │
        ├── local source declarations
        ├── imported shared Type contracts
        └── imported Signature contracts
        │
        ▼
SemanticProgram
```

Runtime composition remains separate:

```text
CompilationCatalog
    = compile-time semantic contracts

ApplicationBindings
    = runtime executable capabilities

CompilationCatalog != ApplicationBindings
```

## Exact technical identities

`Compilation Dependency Data` v0 introduces exactly **8 own technical identities**:

```text
01 TypeSymbol
02 CatalogTypeRef
03 CatalogType
04 CatalogField
05 CatalogVariant
06 CatalogSignatureParameter
07 CatalogSignature
08 CompilationCatalog
```

`SignatureSymbol` is reused from the existing canonical contractual identity and is not counted again.

Containers and primitives are not independent identities.

## Canonical Rust data shapes

```rust
struct TypeSymbol {
    module: String,
    name: String,
}

enum CatalogTypeRef {
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

    Shared(TypeSymbol),
}

enum CatalogType {
    Struct {
        fields: Vec<CatalogField>,
    },
    Enum {
        variants: Vec<CatalogVariant>,
    },
}

struct CatalogField {
    name: String,
    type_ref: CatalogTypeRef,
}

enum CatalogVariant {
    Simple {
        name: String,
    },
    Associated {
        name: String,
        type_ref: CatalogTypeRef,
    },
    Structured {
        name: String,
        fields: Vec<CatalogField>,
    },
}

enum CatalogSignatureParameter {
    Value(CatalogTypeRef),
    SignatureDependency(SignatureSymbol),
}

struct CatalogSignature {
    parameters: Vec<CatalogSignatureParameter>,
    result_type: CatalogTypeRef,
}

struct CompilationCatalog {
    types: HashMap<TypeSymbol, CatalogType>,
    signatures: HashMap<SignatureSymbol, CatalogSignature>,
}
```

`HashMap` expresses direct symbolic lookup at this technical boundary. It is not an ambient registry: the concrete `CompilationCatalog` instance is supplied explicitly to a compilation operation and borrowed immutably.

## CDM-001 — Source Text remains the only functional input

Status: CLOSED

The correction does not change `UC-001`.

```text
Compile functional input
    = Source Text exactly 1
```

`CompilationCatalog` is an explicit technical dependency required by the implementation of semantic resolution, not a second functional business/domain input supplied by Evo-Script syntax.

The later Rust Signature may expose this dependency explicitly; that technical signature does not redefine the Functional Use Case.

## CDM-002 — CompilationCatalog is built outside evo-script-engine

Status: CLOSED

`evo-script-engine` does not construct the catalog by reading `.elib`, `.emod`, `.esig`, `.estc`, `.enum` or filesystem paths.

Conceptually:

```text
module/library resolver
    ↓
validated CompilationCatalog
    ↓
evo-script-engine Compile
```

Therefore the Engine does not own normal failures such as:

```text
LibraryArtifactPathError
LibraryArtifactNotFoundError
DuplicateLibraryArtifactError
ModuleBoundaryError
DuplicateModuleError
DuplicateModuleSymbolError
```

Those failures belong to the component responsible for physical/module catalog construction.

## CDM-003 — Explicit immutable borrowed dependency

Status: CLOSED

A compile operation borrows one validated `CompilationCatalog` immutably for the duration needed by semantic analysis.

No ambient lookup exists:

```text
No Current Library
No Current Module
No global catalog registry
No filesystem Provider discovery
```

Different compile operations may borrow different catalog instances.

An empty catalog is valid for Source Text that requires no imported external semantic contracts.

## CDM-004 — TypeSymbol is canonical shared-Type identity

Status: CLOSED

```rust
struct TypeSymbol {
    module: String,
    name: String,
}
```

`TypeSymbol` is the canonical qualified identity of a shared Struct/Enum contract visible across compilation boundaries.

It is distinct from:

```text
AST QualifiedName
    = source occurrence / syntax

TypeSymbol
    = canonical external shared-Type identity

TypeId
    = compact identity local to one SemanticProgram
```

Aliases in Source Text never change `TypeSymbol`.

## CDM-005 — CatalogTypeRef is self-contained contract type vocabulary

Status: CLOSED

`CatalogTypeRef` expresses exactly the 17 native Evo-Script type families plus a shared type reference:

```text
17 native variants
+ Shared(TypeSymbol)
= 18 CatalogTypeRef variants
```

The catalog does not borrow `TypeId` from any `SemanticProgram` and does not require a parallel catalog-local numeric ID namespace.

This symbolic representation is intentional because `CompilationCatalog` is a reusable boundary artifact, not one compilation's internal IR.

## CDM-006 — Catalog preserves structural data required for semantic resolution

Status: CLOSED

For imported shared Types, the catalog preserves exactly what Semantic Analyzer requires to resolve meaning:

```text
Struct
    ordered fields
    field names
    field type references

Enum
    ordered variants
    variant names
    Simple / Associated / Structured payload shape
    payload type references / field definitions
```

The catalog preserves no runtime layout:

```text
No FieldIndex
No VariantDiscriminant
No backing IDs
No CompiledValueShapeId
No bytecode
```

Those are derived later.

## CDM-007 — CatalogSignature contains semantic contract, not implementation

Status: CLOSED

A catalog Signature contains:

```text
ordered Parameters
├── Value(CatalogTypeRef)
└── SignatureDependency(SignatureSymbol)

result_type: CatalogTypeRef
```

It contains no:

```text
ExternalCapability fn pointer
Provider identity
Application binding
runtime address
ExternalSymbolId
```

Therefore:

```text
CompilationCatalog answers
    "what contract is this?"

ApplicationBindings answers
    "what executable capability satisfies it now?"
```

## CDM-008 — CompilationCatalog is a validated contract artifact

Status: CLOSED

The catalog supplied to the Engine is already internally valid.

Catalog-construction invariants include:

```text
all referenced TypeSymbol values resolve
all referenced SignatureSymbol dependencies resolve
no invalid public-symbol ambiguity
shared type graphs satisfy the external module rules
field / variant definitions are internally valid
```

A malformed catalog with dangling internal references is an integration/invariant violation, not a normal Source Text `CompileFailure`.

By contrast, when a **Source Text import** requests a symbol that is absent from an otherwise valid supplied catalog, Semantic Analyzer has enough information to reject that source. The exact `SemanticFailure` variant is defined in the next block.

## CDM-009 — Semantic Analyzer lowers catalog contracts into compilation-local identities

Status: CLOSED

Catalog identities do not survive as the internal resolution mechanism of `SemanticProgram`.

```text
TypeSymbol
    ↓ Semantic Analyzer
TypeId

SignatureSymbol + CatalogSignature
    ↓ Semantic Analyzer
SignatureId + SemanticSignature
```

Only transitively required external contracts need to be materialized into the Compilation Working State.

After semantic success, Bytecode Compiler continues using only `SemanticProgram`; it does not query `CompilationCatalog`.

This preserves the existing rule:

> Bytecode Compiler does not perform name/type resolution.

## CDM-010 — CompilationCatalog never persists into CompiledProgram or VmExecution

Status: CLOSED

The catalog is compile-time dependency data only.

```text
CompilationCatalog
    ↓ borrowed during semantic resolution
SemanticProgram
    ↓
CompiledProgram
```

`CompiledProgram` preserves only the persistent data already justified by its model, including `SignatureSymbol`, compiled external symbols and boundary value shapes.

It does not retain:

```text
&CompilationCatalog
CatalogType
CatalogSignature
TypeSymbol lookup tables
module/library resolution state
```

`VmExecution` has no relation to `CompilationCatalog`.

## Failure ownership boundary

The correction separates two classes of failures:

```text
CATALOG CONSTRUCTION / MODULE SYSTEM
    LibraryArtifactPathError
    LibraryArtifactNotFoundError
    DuplicateLibraryArtifactError
    ModuleBoundaryError
    DuplicateModuleError
    DuplicateModuleSymbolError
    other physical/module catalog construction failures

                    !=

EVO-SCRIPT-ENGINE SEMANTIC ANALYSIS
    imported symbol requested by Source Text is absent
    local alias/name collision
    call cannot be resolved
    arity/type mismatch
    shared Type usage mismatch
    Signature satisfaction mismatch
    semantic graph validation
```

The exact Engine-side variants belong to `SemanticFailure`.

## Corrected processing map

```text
                          CompilationCatalog
                          validated externally
                                 │
                                 │ immutable borrow
                                 ▼
Source Text
    ↓
Lexical Data
    ↓
AST Data
    ↓
Semantic Analyzer ◄────────────────────────────┘
    ↓
SemanticProgram
    ↓
Bytecode Compiler
    ↓
CompiledProgram
```

## Explicitly not introduced

```text
filesystem access inside Compile
global CompilationCatalog
Current Library
Current Module
Provider discovery during Compile
ApplicationBindings during Compile
catalog-local TypeId reused as Semantic TypeId
catalog-local runtime layouts
module-resolution failures as Source Text failures
CompilationCatalog reference inside CompiledProgram
CompilationCatalog reference inside VmExecution
```

## Closure

```text
CompilationCatalog technical dependency                    ✅ CLOSED
Source Text remains sole functional Compile input           ✅ CLOSED
external catalog construction boundary                      ✅ CLOSED
immutable explicit borrow                                   ✅ CLOSED
TypeSymbol canonical shared-Type identity                   ✅ CLOSED
CatalogTypeRef                                               ✅ CLOSED — 18 variants
CatalogType / Field / Variant contract data                 ✅ CLOSED
CatalogSignature / Parameter contract data                  ✅ CLOSED
validated catalog invariant                                 ✅ CLOSED
catalog → SemanticProgram lowering                          ✅ CLOSED
no catalog persistence into Compiled/VM                     ✅ CLOSED
exact compilation dependency inventory                      ✅ CLOSED — 8 identities

NEXT
    SemanticFailure exact family
```
