# Evo-Script Engine — Root Signature Design

Status: CLOSED

Este documento cierra las firmas raíz y reglas de Participants de los tres Use Cases públicos de `evo-script-engine` v0.

La autoridad deriva de:

- `docs/functional/use-cases/README.md`;
- `UC-001 — Compile`;
- `UC-002 — Execute Compiled`;
- `UC-003 — Execute Source`;
- `docs/technical/data-model/README.md`;
- `ENGINEERING_PRINCIPLES.md`.

## Canonical Use Case Set

```text
Compile
Execute Compiled
Execute Source
```

No se promueven responsabilidades internas como Lexer, Parser, Semantic Analyzer, lowering o VM a Public Use Cases.

## Canonical Root Signatures

```rust
pub type Compile =
    for<'source, 'catalog> fn(
        &'source str,
        &'catalog CompilationCatalog,
    ) -> CompileOutcome;

pub type ExecuteCompiled =
    for<'compiled, 'value, 'bindings> fn(
        &'compiled CompiledProgram,
        &'value [Value<'value>],
        &'bindings ApplicationBindings,
    ) -> ExecutionOutcome;

pub type ExecuteSource =
    for<'source, 'value, 'catalog, 'bindings> fn(
        &'source str,
        &'value [Value<'value>],
        &'catalog CompilationCatalog,
        &'bindings ApplicationBindings,
    ) -> ExecutionOutcome;
```

`CompilationCatalog` es dependencia técnica explícita de Compile-time; no se reinterpreta como segundo Functional Input.

`ApplicationBindings` es dependencia técnica explícita de Runtime; no es estado ambiental ni Service Locator.

## RSD-001 — Exact Use Case signature set

Status: CLOSED

`evo-script-engine` v0 posee exactamente tres Use Case signatures raíz:

```text
Compile
ExecuteCompiled
ExecuteSource
```

## RSD-002 — Functional inputs first, technical dependencies after

Status: CLOSED

Dentro de cada firma, los argumentos que expresan Functional Inputs aparecen primero. Dependencias técnicas explícitas aparecen después.

La regla mejora legibilidad sin ocultar dependencias.

## RSD-003 — Compile exact root signature

Status: CLOSED

```rust
pub type Compile =
    for<'source, 'catalog> fn(
        &'source str,
        &'catalog CompilationCatalog,
    ) -> CompileOutcome;
```

`Source Text` es el Functional Input. `CompilationCatalog` es una dependencia técnica borrowed, validada y construida fuera del Engine.

## RSD-004 — ExecuteCompiled exact root signature

Status: CLOSED

```rust
pub type ExecuteCompiled =
    for<'compiled, 'value, 'bindings> fn(
        &'compiled CompiledProgram,
        &'value [Value<'value>],
        &'bindings ApplicationBindings,
    ) -> ExecutionOutcome;
```

`CompiledProgram` es borrowed/reusable. Invocation Values son borrowed y ordenados. `ApplicationBindings` permanece borrowed e inmutable durante la invocation.

## RSD-005 — ExecuteSource exact root signature

Status: CLOSED

```rust
pub type ExecuteSource =
    for<'source, 'value, 'catalog, 'bindings> fn(
        &'source str,
        &'value [Value<'value>],
        &'catalog CompilationCatalog,
        &'bindings ApplicationBindings,
    ) -> ExecutionOutcome;
```

`ExecuteSource` conserva la equivalencia funcional `Compile + Execute Compiled` sin obligar a una llamada Agent → Agent.

## RSD-006 — Direct owned outcomes; no root Requester

Status: CLOSED

Los tres Use Cases retornan directamente outcomes owned:

```text
Compile          → CompileOutcome
ExecuteCompiled  → ExecutionOutcome
ExecuteSource    → ExecutionOutcome
```

No se introduce Requester para la respuesta final v0.

Motivo:

- `CompiledProgram` debe sobrevivir al Compilation Working State;
- `OwnedValue` debe sobrevivir a `VmExecution`;
- por tanto el ownership de success/failure es semánticamente real y no un DTO artificial.

## RSD-007 — Do not duplicate ExternalCapability as another Contract type

Status: CLOSED

`ApplicationBindings` + `ExternalCapability` constituyen la frontera runtime explícita ya cerrada del Engine.

No se introduce un segundo function-pointer type meramente envolviendo `ExternalCapability` con otro nombre `Contract`.

Esto no prejuzga si la operación de `CallExternal` requiere un Resolver como Participant interno; esa decisión pertenece al árbol de ejecución.

## RSD-008 — Resolver inventory remains local to technical boundaries

Status: CLOSED

No se asume `0 Resolvers` globalmente desde las firmas raíz.

Cada Resolver debe justificarse por una frontera técnica concreta. En particular, `CallExternal` cruza la frontera runtime de `ExternalCapability` y debe analizarse al diseñar los Participants de ejecución.

## RSD-009 — No aggregate Collaborator hides collaborator orchestration

Status: CLOSED

No se introduce un mega-Collaborator `compile_program` o equivalente cuya responsabilidad sea coordinar otros Collaborators significativos únicamente para reutilizar el pipeline.

Regla vigente:

> Un Collaborator no llama a otro Collaborator; la coordinación pertenece al Agent.

Compile se descompone en las etapas internas significativas que correspondan, y el Compile Agent las coordina directamente.

## RSD-010 — ExecuteSource coordinates internal participant signatures directly

Status: CLOSED

`ExecuteSource` no llama a `Compile Agent` ni a `ExecuteCompiled Agent`.

Su Agent coordina directamente las firmas internas necesarias para:

```text
Source Text
→ lexical analysis
→ parsing
→ semantic analysis
→ lowering
→ execution
→ ExecutionOutcome
```

Esto reutiliza lógica interna sin acoplar Public Use Cases entre sí.

## Compile participant direction established by this closure

Las primeras responsabilidades internas significativas del lado Compile son:

```text
Compile Agent
├── lex_source
├── parse_tokens
├── analyze_program
└── lower_program
```

Sus firmas exactas se cierran en documentos posteriores de Participant Design.

No existen para Compile root:

```text
Requester final
Contract de filesystem
Resolver de filesystem
Provider discovery
Host Scope
Global compiler context
```

## Closure

```text
RSD-001..RSD-010                 ✅ CLOSED
Root Use Case signatures         ✅ CLOSED
Root Requester inventory         ✅ 0
Duplicate External Contract      ❌ NOT INTRODUCED
Root Resolver global assumption  ❌ NOT MADE
Compile participant tree         ← NEXT
Execution participant tree       PENDING
Sequence Diagrams                AFTER SIGNATURES
```
