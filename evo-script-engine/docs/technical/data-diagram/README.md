# Evo-Script Engine — Technical Data Diagram

Status: CLOSED

Este directorio contiene las nueve vistas D2 canónicas del `Technical Data Diagram` de `evo-script-engine` v0.

El `Technical Data Model` está CLOSED. Esta etapa representa visualmente identities, ownership, containment, references, borrows, variant payload relations y cardinalidades ya cerradas; no introduce comportamiento ni nuevas identities.

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
00-overview.d2                    ✅ BUILT
01-lexical-data.d2                ✅ BUILT — 4 own identities
02-ast-data.d2                    ✅ BUILT — 31 own identities
03-compilation-dependency-data.d2 ✅ BUILT — 8 own identities
04-semantic-program-data.d2       ✅ BUILT — 33 own identities
05-compiled-program-data.d2       ✅ BUILT — 21 own identities
06-vm-execution-data.d2           ✅ BUILT — 19 own identities
07-outcome-diagnostic-data.d2     ✅ BUILT — 24 own identities
08-cross-phase-boundaries.d2      ✅ BUILT
```

No existe un mega-diagrama autoritativo con las 140 identities propias.

## Closed view responsibilities

### 00 — Overview

Resume las familias cerradas y sus conteos:

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

Representa las 31 identities exactas organizadas por:

```text
Foundational                  4
Top-level                     3
Local Type Definitions       4
Functions / Body             6
Expressions                 10
When                         4
                            ──
TOTAL                       31
```

`Literal`, Unary/Binary forms, `FieldAccess` y `StructConstruction` permanecen variants de `ExpressionKind`; `return` y `this` no sobreviven como identities AST.

### 03 — Compilation Dependency Data

Representa exactamente 8 identities:

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

`SignatureSymbol` aparece como identity contractual reutilizada. El catálogo no contiene Provider/runtime composition y no persiste hacia `CompiledProgram` o `VmExecution`.

### 04 — Semantic Program Data

Representa exactamente:

```text
Semantic IDs                       7
Program/Type/Signature/Function   12
Function Body / Expressions       14
                                 ──
TOTAL                             33
```

Los IDs usan `references` hacia datos dentro de owner collections; nunca `own` sus targets. `Pipeline` no sobrevive como identity semántica y queda reducido a Semantic Expression Composition.

### 05 — Compiled Program Data

Representa exactamente 21 identities.

`Instruction` permanece un único enum node con 48 variants; `CompiledValueShape` un único enum con 17 variants y `CompiledEnumValueShape` con 3 variants.

`FunctionId`, `SignatureSymbol` y `SourceSpan` aparecen como identities reutilizadas. Equality plans viven directamente como payload de Instructions; no existe `EqualityPlanId`/table.

### 06 — VM Execution Data

Representa exactamente 19 identities.

Centro visual:

```text
VmExecution
├── borrows CompiledProgram
├── borrows ApplicationBindings
├── owns SharedValueStorage
├── owns ExecutionBackingStore
└── owns CallFrame 1..N ordered
```

`RuntimeValue` es descriptor relativo a execution context. Shared storage, backing y frames mantienen owners separados; no existen `OperandWindow`, `FrameRegion`, `CurrentFrame`, `CallStack`, `RuntimeTypeId` ni Provider identity.

### 07 — Outcome / Diagnostic Data

Representa exactamente 24 identities:

```text
Public outcome aliases             2
Compile failure root               2
Lexical / Syntax failure families  2
Semantic failure family           12
External capability failure        1
Execution failure family           5
                                  ──
TOTAL                              24
```

`SourceSpan` es reutilizado y provenance añade 0 identities. No existen `DiagnosticAnchor`, `SourceLocation` ni `SourceId`.

### 08 — Cross-Phase Boundaries

Representa exclusivamente relaciones persistentes/reutilizadas entre owner phases/crates:

```text
SourceSpan reuse
UnaryOperator / BinaryOperator reuse
FunctionId / SignatureSymbol reuse
CompiledProgram borrowed by VmExecution
ApplicationBindings owns ExternalCapability table
ExternalCapability references Value<'a>, OwnedValue and ExternalCapabilityFailure
CompileOutcome references CompiledProgram
ExecutionOutcome references OwnedValue
```

`Value<'a>` y `OwnedValue` están marcados como data compartida cuyo owner es `evo-values`.

## TDD-001 — Nine views, not one mega-diagram

Status: CLOSED

El Technical Data Diagram es una suite de exactamente nueve vistas D2.

## TDD-002 — Canonical location

Status: CLOSED

```text
evo-script-engine/docs/technical/data-diagram/
```

## TDD-003 — Exact view set

Status: CLOSED

Las vistas `00..08` anteriores son el set canónico.

## TDD-004 — Every identity has one owner-phase authority

Status: CLOSED

Una identity reutilizada puede reaparecer como reference, pero nunca cambia owner ni se vuelve a contar.

## TDD-005 — Restricted relation vocabulary

Status: CLOSED

Solo `owns`, `contains`, `references`, `borrows`, `variant payload` + cardinalidad explícita.

## TDD-006 — Enum variants are not promoted to identities

Status: CLOSED

Large enums permanecen nodes únicos con variant count y edges hacia payload identities ya existentes.

## TDD-007 — Data only; no Participants or services

Status: CLOSED

Ninguna vista introduce services/classes/Participants o behavioral call arrows.

## TDD-008 — Ordered cardinality explicit

Status: CLOSED

Las colecciones cuyo orden es semánticamente relevante se marcan `ordered`.

## TDD-009 — D2 source is canonical

Status: CLOSED

Los `.d2` son la autoridad versionada. SVG/PNG/PDF son outputs derivados y no autoridad arquitectónica.

La auditoría arquitectónica/textual de la suite está cerrada. El entorno utilizado para esta edición no dispone de ejecutable D2, por lo que no se declara una validación de render/parser local que no se haya realizado.

## TDD-010 — No silent identity invention

Status: CLOSED

La construcción de las nueve vistas no reveló ninguna identity faltante. Ningún bloque del Technical Data Model tuvo que reabrirse.

## Final audit

```text
Canonical view files present                   9 / 9 ✅
Owner-phase identity counts preserved          ✅
Cross-phase owners preserved                   ✅
New technical identities introduced            0
Technical Data Model blocks reopened            0
Participants/services introduced               0
D2 source authority                             ✅
Local D2 render/parser validation               NOT PERFORMED — executable unavailable
```

## Closure

```text
TDD-001..TDD-010                  ✅ CLOSED
Technical Data Model              ✅ CLOSED
Technical Data Diagram scheme     ✅ CLOSED
Technical Data Diagram suite      ✅ CLOSED — 9 views

NEXT ARCHITECTURAL STAGE
    Rust Signatures / Participant Design
```

La siguiente etapa vuelve al rol de Líder Técnico: usar User Stories + Technical Data Model para definir function-pointer signatures de Use Cases y, cuando corresponda, Contracts, Requesters, Collaborators, Resolvers y Tools. Los Sequence Diagrams vienen después de tener signatures y data shapes suficientes.
