# Evo-Script Engine — Technical Data Diagram

Status: SCHEME CLOSED / GLOBAL VIEWS + LEXICAL VIEW BUILT

Este directorio contiene las vistas D2 canónicas del `Technical Data Diagram` de `evo-script-engine` v0.

El `Technical Data Model` está CLOSED. Esta etapa únicamente representa visualmente identities, ownership, containment, references, borrows, variant payload relations y cardinalidades ya cerradas.

## Allowed diagram vocabulary

Categorías:

```text
<<struct>>
<<enum>>
<<artifact>>
<<borrowed view>>
<<alias>>
```

Relaciones:

```text
owns
contains
references
borrows
variant payload
```

Cardinalidades:

```text
1
0..1
0..N
1..N
0..N ordered
1..N ordered
```

No se representan methods, inheritance, OO service classes, behavioral calls, Use Cases, Agents, Resolvers, Collaborators, Contracts, Requesters ni Tools.

## Canonical view suite

```text
00-overview.d2
01-lexical-data.d2
02-ast-data.d2
03-compilation-dependency-data.d2
04-semantic-program-data.d2
05-compiled-program-data.d2
06-vm-execution-data.d2
07-outcome-diagnostic-data.d2
08-cross-phase-boundaries.d2
```

No existe un mega-diagrama autoritativo con las 140 identities propias.

## Current progress

```text
00-overview.d2                    ✅ BUILT
01-lexical-data.d2                ✅ BUILT — 4 own identities
02-ast-data.d2                    ← NEXT — 31 own identities
03-compilation-dependency-data.d2 PENDING — 8 own identities
04-semantic-program-data.d2       PENDING — 33 own identities
05-compiled-program-data.d2       PENDING — 21 own identities
06-vm-execution-data.d2           PENDING — 19 own identities
07-outcome-diagnostic-data.d2     PENDING — 24 own identities
08-cross-phase-boundaries.d2      ✅ BUILT
```

Global views:

- [`00-overview.d2`](./00-overview.d2)
- [`08-cross-phase-boundaries.d2`](./08-cross-phase-boundaries.d2)

Owner-phase view built:

- [`01-lexical-data.d2`](./01-lexical-data.d2)

## View responsibilities

### 00 — Overview

Resume únicamente las familias cerradas y sus conteos:

```text
Lexical Data                     4
AST Data                        31
Compilation Dependency Data      8
Semantic Program Data           33
Compiled Program Data           21
VM Execution Data               19
Outcome / Diagnostic Data       24
                               ───
TOTAL                           140
```

No contiene flechas conductuales.

### 01 — Lexical Data

Representa exactamente:

```text
TokenKind
SourceSpan
Token
TokenSequence
```

`TokenKind` permanece un único enum node con 50 variants. `lexeme: &'source str` es un field borrowed, no una identity propia.

### 02 — AST Data

Debe representar las 31 identities exactas del AST organizadas por:

```text
Foundational
Top-level
Local Type Definitions
Functions / Body
Expressions
When
```

Variants de `ExpressionKind` sin identity propia no se promueven a nodes ficticios.

### 03 — Compilation Dependency Data

Debe representar exactamente las 8 identities:

```text
TypeSymbol
CatalogTypeRef
CatalogType
CatalogField
CatalogVariant
CatalogSignatureParameter
CatalogSignature
CompilationCatalog
```

No incluye runtime Provider/ApplicationBindings ownership.

### 04 — Semantic Program Data

Debe representar las 33 identities propias distinguiendo semantic IDs, owner collections, type/signature/function structures y expression/body data.

ID → indexed data usa `references`; los IDs nunca `own` sus targets.

### 05 — Compiled Program Data

Debe representar las 21 identities propias. `Instruction` permanece un único enum node con 48 variants; `CompiledValueShape` permanece un único enum node con 17 variants.

### 06 — VM Execution Data

Debe representar las 19 identities propias y hacer visible:

```text
VmExecution
├── borrows CompiledProgram
├── borrows ApplicationBindings
├── owns SharedValueStorage
├── owns ExecutionBackingStore
└── owns ordered CallFrame collection
```

No se introducen OperandWindow, FrameRegion, CurrentFrame, CallStack u otros wrappers inexistentes.

### 07 — Outcome / Diagnostic Data

Debe representar las 24 identities propias. `CompileOutcome` y `ExecutionOutcome` son aliases contractuales; `Result`, `Option`, `Box` y `Vec` no son identities.

`SourceSpan` aparece como reutilizado desde Lexical Data y no se vuelve a contar.

### 08 — Cross-Phase Boundaries

Representa únicamente relaciones persistentes/reutilizadas entre owner phases/crates.

La vista construida incluye:

```text
SourceSpan reuse
UnaryOperator / BinaryOperator reuse
FunctionId / SignatureSymbol reuse
CompiledProgram borrowed by VmExecution
ApplicationBindings → ExternalCapability ownership
ExternalCapability references Value<'a>, OwnedValue and ExternalCapabilityFailure
CompileOutcome → CompiledProgram
ExecutionOutcome → OwnedValue
```

`Value<'a>` y `OwnedValue` están marcados como external shared data cuyo owner es `evo-values`.

## TDD closure rules

### TDD-001 — Nine views, not one mega-diagram
Status: CLOSED

### TDD-002 — Canonical location
Status: CLOSED

```text
evo-script-engine/docs/technical/data-diagram/
```

### TDD-003 — Exact view set
Status: CLOSED

Las nueve vistas `00..08` anteriores son el set canónico.

### TDD-004 — Every identity has one owner-phase authority
Status: CLOSED

Una identity reutilizada puede reaparecer como reference, pero nunca cambia owner ni se vuelve a contar.

### TDD-005 — Restricted relation vocabulary
Status: CLOSED

Solo `owns`, `contains`, `references`, `borrows`, `variant payload` + cardinalidad explícita.

### TDD-006 — Enum variants are not promoted to identities
Status: CLOSED

Large enums permanecen nodes únicos con variant count y edges hacia payload identities existentes.

### TDD-007 — Data only; no Participants or services
Status: CLOSED

### TDD-008 — Ordered cardinality must be explicit
Status: CLOSED

### TDD-009 — D2 source is canonical
Status: CLOSED

SVG/PNG/PDF son outputs derivados, no autoridad arquitectónica.

### TDD-010 — Diagramming cannot silently invent identities
Status: CLOSED

Si una vista demuestra que falta una identity o cardinalidad imposible de representar, se reabre únicamente el owning Data Model block; no se corrige silenciosamente desde D2.

## Scheme closure

```text
TDD-001..TDD-010                  ✅ CLOSED
Technical Data Diagram scheme     ✅ CLOSED
Technical Data Model              ✅ CLOSED
00 Overview                       ✅ BUILT
08 Cross-Phase Boundaries         ✅ BUILT
01 Lexical Data                   ✅ BUILT
```

## Next

```text
Build 02-ast-data.d2
```

AST será la primera vista owner-phase grande y deberá cubrir exactamente sus 31 identities sin promover variants o syntax-only forms a identities nuevas.
