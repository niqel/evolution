# Evo-Script Engine — Technical Data Diagram

Status: SCHEME CLOSED / D2 VIEWS PENDING

Este directorio contiene las vistas D2 canónicas del `Technical Data Diagram` de `evo-script-engine` v0.

El `Technical Data Model` ya está cerrado. Esta etapa no diseña nuevas identities: representa visualmente ownership, containment, references, borrows, variant payload relations y cardinalidades ya definidas.

## Scope

El Technical Data Diagram representa datos y artifacts, no comportamiento.

Categorías permitidas:

```text
<<struct>>
<<enum>>
<<artifact>>
<<borrowed view>>
<<alias>>
```

Relaciones permitidas:

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

No se representan:

```text
methods
inheritance
OO service classes
behavioral calls
Use Cases
Agents
Resolvers
Collaborators
Contracts
Requesters
Tools
```

Esos elementos pertenecen a etapas posteriores de Rust Signatures / Participants / Sequence Diagrams.

## Canonical view suite

El Technical Data Diagram se divide exactamente en nueve vistas D2:

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

No se introduce un único mega-diagrama con todas las identities del Engine.

### 00 — Overview

Resume las familias del Data Model y sus conteos cerrados:

```text
Lexical Data                     4 identities
AST Data                        31 identities
Compilation Dependency Data      8 identities
Semantic Program Data           33 identities
Compiled Program Data           21 identities
VM Execution Data               19 identities
Outcome / Diagnostic Data       24 identities
```

La vista no enumera todas las identities ni representa behavior.

### 01 — Lexical Data

Representa las cuatro identities propias:

```text
TokenKind
SourceSpan
Token
TokenSequence
```

`TokenKind` permanece un único enum node con 50 variants.

### 02 — AST Data

Representa las 31 identities exactas del AST organizadas por owners y clusters:

```text
Foundational
Top-level
Local Type Definitions
Functions / Body
Expressions
When
```

Las variants de `ExpressionKind` que no poseen identity independiente permanecen dentro del node enum; no se promueven a structs ficticios.

### 03 — Compilation Dependency Data

Representa las 8 identities del catálogo técnico:

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

La vista muestra compile-time contract data únicamente. No incluye `ApplicationBindings`, Provider runtime ni `ExternalCapability` como owners del catálogo.

### 04 — Semantic Program Data

Representa las 33 identities propias del Semantic Program, distinguiendo:

```text
semantic IDs
program/type/signature/function owners
function body / expression structures
```

Las relaciones ID → owner collection son `references`; los IDs no poseen los objetos a los que indexan.

### 05 — Compiled Program Data

Representa las 21 identities propias del producto compilado.

`Instruction` permanece un único enum node con 48 variants y edges a las identities que aparecen en sus payloads.

`CompiledValueShape` permanece un único enum node con 17 variants y `CompiledEnumValueShape` con 3 variants.

### 06 — VM Execution Data

Representa las 19 identities propias de VM.

El centro visual es:

```text
VmExecution
├── borrows CompiledProgram
├── borrows ApplicationBindings
├── owns SharedValueStorage
├── owns ExecutionBackingStore
└── owns ordered CallFrame collection
```

La vista debe hacer visible que `RuntimeValue` usa typed IDs/references para execution backing y no posee directamente composite backing data.

No se introducen `OperandWindow`, `FrameRegion`, `CurrentFrame`, `CallStack` u otras identities no existentes.

### 07 — Outcome / Diagnostic Data

Representa las 24 identities propias de Outcome.

Las dos aliases públicas aparecen como identities contractuales:

```rust
type CompileOutcome = Result<CompiledProgram, CompileFailure>;
type ExecutionOutcome = Result<OwnedValue, ExecutionFailure>;
```

`Result`, `Option`, `Box`, `Vec` y otros containers no se convierten en nodes de identity.

`SourceSpan` se muestra como identity reutilizada desde Lexical Data, no como identity propia de Outcome.

### 08 — Cross-Phase Boundaries

Representa exclusivamente relaciones importantes entre owners de distintas fases/crates.

Ejemplos obligatorios:

```text
SourceSpan
    reused by AST / Semantic / SourceMap / Outcome

UnaryOperator / BinaryOperator
    AST → reused by Semantic

FunctionId / SignatureSymbol
    Semantic → reused by Compiled / VM / Outcome where applicable

CompiledProgram
    borrowed by VmExecution

ApplicationBindings
    owns ExternalCapability table

ExternalCapability
    references Value<'a>
    references OwnedValue
    references ExternalCapabilityFailure

CompileOutcome
    references CompiledProgram

ExecutionOutcome
    references OwnedValue
```

`Value<'a>` y `OwnedValue` deben marcarse como identities externas compartidas cuyo owner es `evo-values`; no se cuentan como identities propias del Engine.

## TDD-001 — Nine views, not one mega-diagram

Status: CLOSED

El Technical Data Diagram es una suite de exactamente nueve vistas D2. No se intenta colocar las aproximadamente 140 identities propias de todas las fases en un solo canvas.

## TDD-002 — Canonical location

Status: CLOSED

La ubicación canónica es:

```text
evo-script-engine/docs/technical/data-diagram/
```

con este `README.md` y las fuentes `00..08 .d2`.

## TDD-003 — Exact view set

Status: CLOSED

Las vistas canónicas son exactamente:

```text
Overview
Lexical Data
AST Data
Compilation Dependency Data
Semantic Program Data
Compiled Program Data
VM Execution Data
Outcome / Diagnostic Data
Cross-Phase Boundaries
```

## TDD-004 — Every identity has one owner-phase authority

Status: CLOSED

Toda identity técnica propia cerrada en los inventarios aparece en exactamente una vista owner-phase como identity autoritativa.

Una identity reutilizada por otra fase puede reaparecer como external/reused reference, pero no se vuelve a contar ni se le cambia owner.

## TDD-005 — Restricted relation vocabulary

Status: CLOSED

Las relaciones del Technical Data Diagram se restringen a:

```text
owns
contains
references
borrows
variant payload
```

más cardinalidad explícita.

No se utiliza `calls`, `executes`, `validates`, `resolves` u otras relaciones conductuales como relaciones de Data Diagram.

## TDD-006 — Enum variants are not promoted to identities

Status: CLOSED

Una variant de enum no se convierte en node independiente a menos que exista una identity técnica propia ya cerrada para su payload/modelo.

Enums grandes permanecen nodes únicos con su conteo cerrado y relaciones a identities usadas por sus payloads.

Ejemplos:

```text
TokenKind                 one node / 50 variants
Instruction               one node / 48 variants
CompiledValueShape        one node / 17 variants
RuntimeValue              one node / 17 variants
```

## TDD-007 — Data only; no Participants or services

Status: CLOSED

El diagrama representa únicamente datos/artifacts.

No aparecen servicios ficticios como:

```text
Lexer service
Parser service
SemanticAnalyzer class
BytecodeCompiler class
Vm service
```

ni Participants de las etapas posteriores.

## TDD-008 — Ordered cardinality must be explicit

Status: CLOSED

Cuando el orden de una colección es semánticamente significativo se marca explícitamente como `ordered`.

Ejemplos:

```text
SemanticFunction parameters      0..N ordered
CompiledFunction instructions    1..N ordered
Enum variants                    1..N ordered
CallFrame collection             1..N ordered / LIFO interpretation defined elsewhere
```

El diagrama no debe reducir una relación ordenada a una cardinalidad no ordenada ambigua.

## TDD-009 — D2 source is canonical

Status: CLOSED

Los archivos `.d2` son la autoridad versionada del Technical Data Diagram.

SVG/PNG/PDF renderizados son outputs derivados y no constituyen autoridad arquitectónica.

No es necesario versionar renders para cerrar esta etapa.

## TDD-010 — Diagramming cannot silently invent identities

Status: CLOSED

La diagramación puede revelar una inconsistencia, pero no puede crear silenciosamente una nueva identity o modificar una cardinalidad cerrada.

Si aparece una necesidad genuina no representable con el Data Model actual:

```text
1. identificar owning phase
2. demostrar la inconsistencia
3. reabrir únicamente ese bloque del Technical Data Model
4. corregir autoridad/inventario
5. volver al diagrama
```

## Closed scheme

```text
TDD-001 nine-view suite                          ✅ CLOSED
TDD-002 canonical data-diagram location          ✅ CLOSED
TDD-003 exact view set                           ✅ CLOSED
TDD-004 one authoritative owner-phase view       ✅ CLOSED
TDD-005 restricted relation vocabulary           ✅ CLOSED
TDD-006 enum variants remain variants            ✅ CLOSED
TDD-007 data only / no Participants               ✅ CLOSED
TDD-008 ordered cardinality explicit              ✅ CLOSED
TDD-009 D2 source is canonical                    ✅ CLOSED
TDD-010 no silent identity invention              ✅ CLOSED

Technical Data Diagram scheme                     ✅ CLOSED
Technical Data Model                              ✅ CLOSED
```

## Construction order

La construcción empieza por las dos vistas que validan la arquitectura global:

```text
00-overview.d2
08-cross-phase-boundaries.d2
```

Después se construyen las siete vistas owner-phase:

```text
01-lexical-data.d2
02-ast-data.d2
03-compilation-dependency-data.d2
04-semantic-program-data.d2
05-compiled-program-data.d2
06-vm-execution-data.d2
07-outcome-diagnostic-data.d2
```

## Next

```text
Build 00-overview.d2
Build 08-cross-phase-boundaries.d2
```
