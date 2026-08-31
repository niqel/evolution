# Evo-Script Engine — Implementation Tasks

Status: IMPLEMENTATION TASKS — CLOSED / READY FOR PROGRAMMING

Este documento es el backlog técnico canónico para implementar `evo-script-engine` v0 mediante AGY/Codex.

No es una nueva fase de diseño. Cada tarea implementa decisiones ya cerradas.

## Reglas para el programador

1. No reabrir arquitectura durante implementación.
2. No introducir nuevos Use Cases, Agents, Collaborators, Resolvers, Requesters, Contracts, Tools o identities técnicas sin detener la tarea y reportar la contradicción.
3. No introducir `trait`, `dyn`, service locator, manager, service class, global registry, `ExecutionContext`, `Session`, `Current Provider` o wrappers genéricos no cerrados.
4. No permitir Agent → Agent ni Collaborator → Collaborator.
5. Toda firma debe coincidir exactamente con `docs/technical/signatures/`.
6. Todo módulo conductual debe coincidir con `docs/technical/module-signatures/MODULE_SIGNATURE_DESIGN.md`.
7. Toda interacción debe poder rastrearse hacia `docs/technical/sequences/`.
8. Todo dato debe corresponder al Technical Data Model cerrado; no inventar DTOs o identities para facilitar código.
9. Mantener borrowing/ownership conforme a los documentos; no resolver lifetimes mediante clones/copies artificiales de Source Text o working state.
10. Si dos documentos cerrados parecen contradictorios, detenerse y reportar el conflicto; no escoger silenciosamente uno.

---

# WP-00 — Prerequisito `evo-values`

`evo-script-engine` no puede implementar su frontera runtime mientras `evo-values` conserve el modelo histórico `Text / Unsigned / Signed`.

## EVO-V-001 — Implementar `Value<'a>` v0 de 17 familias

Dependencias: ninguna.

Implementar exactamente `EV-003..EV-007` de `evo-values/INTERCHANGE_MODEL.md`.

Debe incluir:

```text
Value<'a>
DynamicValue<'a>
DynamicIntegerValue<'a>
EnumPayload<'a>
```

Aceptar únicamente las 17 familias cerradas.

Eliminar como autoridad pública del modelo v0:

```text
Text
Unsigned
Signed
```

Criterios:

- compile;
- lifetimes borrowed correctos;
- String usa `&str`;
- Dynamic Integer conserva sign + canonical magnitude;
- Struct/Enum preservan orden;
- no VM handles.

## EVO-V-002 — Implementar `OwnedValue` v0

Dependencias: EVO-V-001.

Implementar exactamente:

```text
OwnedValue
OwnedDynamicValue
OwnedDynamicInteger
OwnedEnumPayload
```

con las 17 familias correspondientes.

Criterios:

- cero Rust references dentro de `OwnedValue`;
- String owned `Box<str>`;
- composites owned recursivos;
- Dynamic Integer canonical owned representation;
- no `Value<'static>` como sustituto.

## EVO-V-003 — Ajustar exports y `no_std + alloc`

Dependencias: EVO-V-001, EVO-V-002.

Alinear `evo-values/src/lib.rs` y módulos públicos con `INTERCHANGE_MODEL.md`.

Criterios:

- `#![no_std]` + `alloc` cuando aplique según el modelo cerrado;
- API exporta las identities cerradas;
- no dependencia concreta de VM/Engine;
- no reflection metadata.

## EVO-V-004 — Pruebas del Interchange Model

Dependencias: EVO-V-003.

Cubrir:

```text
17 borrowed families
17 owned families
Dynamic Integer canonical zero
Dynamic Integer positive/negative magnitude
nested Struct
Simple/Associated/Structured Enum
borrowed String lifetime
OwnedValue autonomy
```

Gate:

```text
cargo test -p evo-values
```

---

# WP-01 — Scaffold de `evo-script-engine`

## ESE-001 — Crear crate y agregarlo al workspace

Dependencias: EVO-V-003.

Crear:

```text
evo-script-engine/Cargo.toml
evo-script-engine/src/lib.rs
```

Agregar `evo-script-engine` a `workspace.members`.

Dependencia obligatoria:

```text
evo-values
```

No agregar crates adicionales salvo necesidad concreta de implementación demostrada por el Data Model.

Criterios:

```text
cargo check -p evo-script-engine
```

funciona con un crate mínimo.

## ESE-002 — Crear árbol modular conductual

Dependencias: ESE-001.

Crear exactamente la estructura prevista por `MSD-009`:

```text
src/definitions/use_cases/
src/agents/
src/collaborators/
src/resolvers/
src/tools/
```

con los 21 módulos conductuales previstos, inicialmente compilables aunque su implementación llegue en tareas posteriores.

No crear carpetas espejo:

```text
definitions/collaborators
definitions/resolvers
definitions/tools
```

---

# WP-02 — Technical Data Model en Rust

La organización física interna puede agrupar identities por familia técnica, pero no debe alterar las 140 identities cerradas ni promover fields/variants a nuevas identities.

## ESE-010 — Lexical Data + SourceSpan

Dependencias: ESE-001.

Implementar:

```text
TokenKind — 50 variants
SourceSpan
Token<'source>
TokenSequence<'source> = Vec<Token<'source>>
```

y sus invariantes.

No EOF, Invalid, Whitespace o Comment Tokens.

## ESE-011 — Compile / Lexical / Syntax failure data

Dependencias: ESE-010.

Implementar:

```text
CompileOutcome
CompileFailure
CompileFailureKind
LexicalFailure — 6 variants
SyntaxFailure — 10 variants
```

Preservar provenance exacta y zero-width spans.

## ESE-012 — AST foundational + top-level data

Dependencias: ESE-010.

Implementar:

```text
Identifier
QualifiedName
Visibility
TypedBinding
Program
ImportDeclaration
Declaration
```

No semantic resolution.

## ESE-013 — AST local types + function/body data

Dependencias: ESE-012.

Implementar identities cerradas de:

```text
StructDefinition
FieldDefinition
EnumDefinition
EnumVariant
FunctionDefinition
Parameter
FunctionBody
BodyStatement
LetBinding
OperationStatement
```

## ESE-014 — AST Expression + When data

Dependencias: ESE-013.

Implementar el inventario restante hasta completar exactamente 31 identities AST.

Reglas físicas:

```text
typed nested tree
Box<Expression> para recursión directa
Vec para ordered collections
no NodeId
no AST Arena
```

## ESE-015 — Compilation Dependency Data

Dependencias: ESE-001.

Implementar exactamente las 8 identities:

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

`CompilationCatalog` es explícito, validated, immutable y reusable; no filesystem I/O.

## ESE-016 — Semantic Program core data

Dependencias: ESE-014, ESE-015.

Implementar IDs, root/owner structures y semantic functions conforme a `SEMANTIC_PROGRAM_DATA.md` y documentos especializados.

Completar exactamente 33 semantic identities.

No Provider identity, runtime binding, Pipeline identity ni SemanticImport.

## ESE-017 — SemanticFailure family

Dependencias: ESE-016.

Implementar exactamente las 12 identities propias de la familia `SemanticFailure` y todos sus variants cerrados.

No dejar escapar IDs locales de `SemanticProgram` dentro del outcome.

## ESE-018 — Compiled Program core + constants + identities

Dependencias: ESE-016, EVO-V-003.

Implementar root, functions, persistent IDs, constants y ExternalSymbol conforme a `COMPILED_PROGRAM_DATA.md`.

## ESE-019 — Instructions, equality, boundary shapes y SourceMap

Dependencias: ESE-018.

Implementar:

```text
Instruction — exactamente 48 variants
NumericKind — 12 variants
CompiledValueShape — 17 variants
CompiledEnumValueShape — 3 variants
EqualityRule
CompositeEqualityPlan
EnumEqualityPayloadPlan
SourceMap
```

Completar exactamente 21 own Compiled Program identities.

## ESE-020 — RuntimeValue + backing data

Dependencias: ESE-019.

Implementar:

```text
RuntimeValue — 17 variants
DynamicValue — 3 variants
Backing IDs / Refs
ExecutionBackingStore
DynamicIntegerBacking
StructBacking
EnumBacking
RuntimeEnumPayload
```

Preservar typed append-only backing stores y DAG inmutable.

La crate concreta usada internamente para arbitrary integer, si se necesita, no puede escapar por API ni convertirse en identity arquitectónica.

## ESE-021 — SharedValueStorage + CallFrame + VmExecution

Dependencias: ESE-020.

Implementar:

```text
SharedValueStorage
InstructionPointer
CallFrame
VmExecution<'compiled, 'bindings>
```

con exactamente los cinco fields cerrados del root.

No `OperandStack`, `CurrentFrame`, root IP, outcome flag o execution state enum.

## ESE-022 — ApplicationBindings + ExternalCapability ABI

Dependencias: ESE-020, EVO-V-003.

Implementar:

```text
ApplicationBindings
ExternalCapability
ExternalCapabilityFailure
```

Firma exacta:

```rust
for<'value> fn(
    &'value [Value<'value>],
) -> Result<OwnedValue, ExternalCapabilityFailure>
```

No `dyn Fn`, Provider object o captured closure ABI.

## ESE-023 — ExecutionOutcome / ExecutionFailure family

Dependencias: ESE-011, ESE-017, ESE-022.

Implementar exactamente:

```text
ExecutionOutcome
ExecutionFailure
ExecutionFailureKind
InvocationFailure
EvaluationFailure
ExternalExecutionFailure
```

Preservar `Option<SourceSpan>` según provenance cerrada.

---

# WP-03 — Public Use Case signatures

## ESE-030 — Implementar las tres definiciones Use Case

Dependencias: ESE-015, ESE-018, ESE-022, ESE-023, EVO-V-003.

Crear exactamente:

```text
definitions/use_cases/compile.rs
definitions/use_cases/execute_compiled.rs
definitions/use_cases/execute_source.rs
```

con las firmas RSD-002 / RSD-003 / RSD-004.

No implementación dentro de `definitions/`.

---

# WP-04 — Compile Participants

## ESE-040 — Implementar `collaborators/lexer.rs`

Dependencias: ESE-010, ESE-011.

Implementar:

```text
Lex
lex_source
LEX_SOURCE
```

Cubrir las 50 formas TokenKind, comments/whitespace y 6 LexicalFailure.

No Tools arquitectónicas.

## ESE-041 — Implementar `collaborators/parser.rs`

Dependencias: ESE-014, ESE-011, ESE-040.

Implementar:

```text
Parse
parse_tokens
PARSE_TOKENS
```

Firma incluye `&TokenSequence<'source>` + `&'source str` para EOF provenance.

No retokenizar Source Text.

## ESE-042 — Implementar `collaborators/semantic_analyzer.rs`

Dependencias: ESE-015, ESE-016, ESE-017, ESE-041.

Implementar:

```text
Analyze
analyze_program
ANALYZE_PROGRAM
```

Resolver completamente semantic identities y producir `SemanticProgram` owned.

## ESE-043 — Implementar `collaborators/bytecode_compiler.rs`

Dependencias: ESE-019, ESE-042.

Implementar:

```text
Lower
lower_program
LOWER_PROGRAM
```

No `Result` normal.

Cubrir lowering de todas las `SemanticExpressionKind`, statements, calls, short-circuit, composites, equality, SourceMap y boundary shapes.

## ESE-044 — Implementar `agents/compiler.rs`

Dependencias: ESE-030, ESE-040, ESE-041, ESE-042, ESE-043.

Implementar exactamente el pipeline:

```text
LEX_SOURCE
→ PARSE_TOKENS
→ ANALYZE_PROGRAM
→ LOWER_PROGRAM
```

Binding obligatorio:

```rust
pub const COMPILE: compile::Compile = compile;
```

Agent = orchestration only.

---

# WP-05 — Execution Tools

## ESE-050 — `tools/matches_value_shape.rs`

Dependencias: ESE-019, EVO-V-003.

Implementar alias, función y binding tipado exactos de RSD-027.

## ESE-051 — `tools/materialize_value.rs`

Dependencias: ESE-020, EVO-V-003.

Implementar RSD-028; borrowed `Value` → `RuntimeValue` + execution backing.

## ESE-052 — `tools/own_runtime_value.rs`

Dependencias: ESE-020, ESE-019, EVO-V-003.

Implementar RSD-030; `RuntimeValue` → autonomous `OwnedValue`.

## ESE-053 — `tools/locate_source_span.rs`

Dependencias: ESE-019, ESE-021.

Implementar RSD-031 mediante `FunctionId + InstructionPointer → SourceMap → SourceSpan`.

## ESE-054 — `tools/observe_runtime_value.rs`

Dependencias: ESE-020, EVO-V-003.

Implementar RSD-033; Runtime descriptor → borrowed interchange `Value<'a>`.

## ESE-055 — `tools/matches_owned_value_shape.rs`

Dependencias: ESE-019, EVO-V-003.

Implementar RSD-034.

## ESE-056 — `tools/materialize_owned_value.rs`

Dependencias: ESE-020, EVO-V-003.

Implementar RSD-035; consumir `OwnedValue` y transferir ownership a runtime backing cuando corresponda.

## ESE-057 — `tools/contextualize_compile_failure.rs`

Dependencias: ESE-011, ESE-023.

Implementar exactamente EXF-003 / RSD-037.

---

# WP-06 — Execution Participants

## ESE-060 — Implementar `collaborators/execution_initializer.rs`

Dependencias: ESE-021, ESE-023, ESE-050, ESE-051.

Implementar:

```text
Initialize
initialize_execution
INITIALIZE_EXECUTION
```

Responsabilidades:

```text
arity validation
shape validation
Invocation Value materialization
entry Parameter cells
Local reservation
entry CallFrame
VmExecution root
```

No preflight de ExternalCapability bindings.

## ESE-061 — Implementar `collaborators/instruction_executor.rs`

Dependencias: ESE-019, ESE-020, ESE-021, ESE-023, ESE-052, ESE-053.

Implementar exactamente las 47 Instructions no `CallExternal`.

Firma:

```text
Result<Option<OwnedValue>, ExecutionFailure>
```

Preservar todas las reglas de IP commit, Call/Return, stack/frame invariants y EvaluationFailure.

No crear Collaborators por family de opcode.

## ESE-062 — Implementar `resolvers/external_call_resolver.rs`

Dependencias: ESE-022, ESE-023, ESE-053, ESE-054, ESE-055, ESE-056.

Implementar:

```text
ResolveExternalCall
resolve_external_call
RESOLVE_EXTERNAL_CALL
```

Flujo exacto:

```text
CallExternal
→ SignatureSymbol lookup
→ observe arguments
→ ExternalCapability
→ validate OwnedValue
→ materialize runtime result
→ commit N → 1
→ ip += 1
```

En failure:

```text
no commit
IP unchanged
ExternalExecutionFailure + SourceSpan
```

## ESE-063 — Implementar `agents/compiled_program_executor.rs`

Dependencias: ESE-030, ESE-060, ESE-061, ESE-062.

Implementar loop RSD-026.

Binding:

```rust
pub const EXECUTE_COMPILED: execute_compiled::ExecuteCompiled = execute_compiled;
```

Agent decide únicamente:

```text
CallExternal → Resolver
other instruction → Collaborator
```

## ESE-064 — Implementar `agents/source_executor.rs`

Dependencias: ESE-044, ESE-057, ESE-060, ESE-061, ESE-062.

No llamar a `COMPILE` ni `EXECUTE_COMPILED` Agents.

Coordinar directamente:

```text
LEX_SOURCE
PARSE_TOKENS
ANALYZE_PROGRAM
LOWER_PROGRAM
INITIALIZE_EXECUTION
EXECUTE_INSTRUCTION / RESOLVE_EXTERNAL_CALL loop
```

Binding:

```rust
pub const EXECUTE_SOURCE: execute_source::ExecuteSource = execute_source;
```

---

# WP-07 — Tests por fase

## ESE-070 — Lexer tests

Dependencias: ESE-040.

Cubrir todas las TokenKind families, keyword inventory, comments/whitespace, UTF-8 strings, numeric forms y las 6 LexicalFailure variants.

## ESE-071 — Parser tests

Dependencias: ESE-041.

Cubrir las 31 AST identities/formas, precedence/grouping, imports, final return, exactly one public function, `this`, operation statements y las 10 SyntaxFailure variants.

Incluir EOF provenance con source vacío y whitespace/comment trailing.

## ESE-072 — Semantic Analyzer tests

Dependencias: ESE-042.

Cubrir resolución local/catalog, aliases, types, calls, cycles, composites, when, conversions, signature satisfaction y todas las SemanticFailure families.

## ESE-073 — Bytecode Compiler tests

Dependencias: ESE-043.

Cubrir:

```text
48 Instruction variants emitted as required
SourceMap density
internal FunctionId calls
ExternalSymbol emission
boundary shapes
short-circuit jumps
struct/enum layout
structural equality plans
```

## ESE-074 — Execution Tool tests

Dependencias: ESE-050..ESE-057.

Cubrir borrowed/owned conversions, recursive shapes, backing ownership, SourceSpan lookup y compile-failure contextualization.

## ESE-075 — Execution initializer tests

Dependencias: ESE-060.

Cubrir arity mismatch, shape mismatch, parameter materialization, local reservation, exact entry frame, no binding preflight.

## ESE-076 — Instruction executor tests

Dependencias: ESE-061.

Cubrir todas las 47 non-external Instructions, stack effects, frame transitions, IP semantics, overflow/division/conversion/dynamic errors y entry `OwnedValue` result.

## ESE-077 — External call resolver tests

Dependencias: ESE-062.

Cubrir:

```text
missing binding
successful borrowed arguments
ExternalCapabilityFailure
result shape mismatch
successful OwnedValue materialization
N → 1 commit only on success
IP unchanged on failure
SourceSpan contextualization
```

---

# WP-08 — Public Use Case integration

## ESE-080 — Compile integration tests

Dependencias: ESE-044, ESE-070..ESE-073.

Verificar `Compile` end-to-end:

```text
Source Text + CompilationCatalog
→ CompiledProgram
```

y first responsible failure para lexical/syntax/semantic cases.

## ESE-081 — ExecuteCompiled integration tests

Dependencias: ESE-063, ESE-074..ESE-077.

Verificar:

```text
CompiledProgram + Invocation Values + ApplicationBindings
→ OwnedValue / ExecutionFailure
```

incluyendo internal calls, CallExternal y failure families.

## ESE-082 — ExecuteSource integration/equivalence tests

Dependencias: ESE-064, ESE-080, ESE-081.

Verificar funcionalmente:

```text
ExecuteSource(source, values, catalog, bindings)
    ≡
Compile(source, catalog)
    + ExecuteCompiled(compiled, values, bindings)
```

para success y failures equivalentes.

---

# WP-09 — Architecture and quality gates

## ESE-090 — Typed binding architecture tests

Dependencias: todos los módulos conductuales.

Verificar que los bindings tipados compilan exactamente:

```text
3 Agent bindings
6 Collaborator bindings
1 Resolver binding
8 Tool bindings
```

No testear mocks OO; usar type checking real.

## ESE-091 — Dependency / architecture audit

Dependencias: ESE-090.

Revisar:

```text
Agent → Agent                         0
Collaborator → Collaborator           0
Requester modules                     0
additional Contract modules           0
Provider identity in Engine           0
Active Scope / Session / Context      0
dyn/trait service abstractions        0 unless explicitly reopened
```

## ESE-092 — Full workspace validation

Dependencias: EVO-V-004, ESE-080, ESE-081, ESE-082, ESE-091.

Gate final:

```text
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
```

Cualquier warning/error introducido debe corregirse sin alterar la arquitectura cerrada.

## ESE-093 — Documentation/code trace audit

Dependencias: ESE-092.

Confirmar:

```text
Technical Data Model ↔ Rust types
Rust Signatures ↔ actual aliases
Participants ↔ actual modules
Module Signature Diagrams ↔ module paths/bindings
D2 Sequence Diagrams ↔ actual call graph
Implementation Tasks ↔ completed code/tests
```

Si existe divergencia, corregir código o reabrir formalmente diseño; nunca actualizar documentación para ocultar una divergencia no resuelta.

---

# Recommended execution order

```text
WP-00 evo-values prerequisite
    ↓
WP-01 crate scaffold
    ↓
WP-02 Technical Data Model implementation
    ↓
WP-03 Use Case definitions
    ↓
WP-04 Compile Participants
    ↓
WP-05 Execution Tools
    ↓
WP-06 Execution Participants
    ↓
WP-07 phase tests
    ↓
WP-08 Use Case integration
    ↓
WP-09 architecture / workspace gates
```

Parallelization is allowed only where dependencies above are already satisfied. AGY/Codex must not parallelize files whose types/signatures are still being created by an unmet prerequisite.

## Final backlog inventory

```text
Cross-crate evo-values tasks        4
Engine scaffold tasks               2
Technical Data Model tasks         14
Public signature task               1
Compile Participant tasks           5
Execution Tool tasks                8
Execution Participant tasks         5
Phase test tasks                     8
Use Case integration tasks          3
Architecture/quality tasks          4
                                   ──
TOTAL                               54 tasks
```

## Closure

```text
Implementation Task Backlog        ✅ CLOSED — 54 tasks
Architecture/design prerequisites   ✅ CLOSED
Programming                         READY

PROGRAMMERS
    AGY / Codex
```
