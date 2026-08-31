# Evo-Script Engine — Module Signature Design

Status: MODULE SIGNATURE DESIGN — CLOSED

Este documento cierra la identidad modular prevista para los Participants conductuales de `evo-script-engine` v0 antes de construir los D2 Module Signature Diagrams.

La autoridad deriva de:

- `TECHNICAL_DESIGN_METHODOLOGY.md`;
- `docs/technical/signatures/ROOT_SIGNATURE_DESIGN.md`;
- `docs/technical/signatures/COMPILE_PARTICIPANT_DESIGN.md`;
- `docs/technical/signatures/EXECUTION_PARTICIPANT_DESIGN.md` y sus cierres especializados;
- `ARCHITECTURE.md`;
- `ENGINEERING_PRINCIPLES.md`.

## MSD-001 — Un módulo conductual representa una identidad arquitectónica

Status: CLOSED

Para los Participants cerrados:

```text
archivo.rs
    = módulo
    = identidad arquitectónica conductual
```

No se crean `struct Service`, managers, objects o wrappers únicamente para obtener identidad.

## MSD-002 — Use Case signatures permanecen en `definitions/use_cases`

Status: CLOSED

Las tres firmas públicas se ubican como definiciones puras:

```text
src/definitions/use_cases/
├── compile.rs
├── execute_compiled.rs
└── execute_source.rs
```

Cada archivo contiene únicamente la firma Use Case y datos directamente pertenecientes a esa definición cuando corresponda; no implementa la operación.

## MSD-003 — Firma e implementación interna se colocan en el módulo del Participant

Status: CLOSED

Los Collaborators, Resolver y Tools son Participants internos con firma function-pointer cerrada.

No se crean carpetas espejo como:

```text
definitions/collaborators/
definitions/resolvers/
definitions/tools/
```

que obligarían a representar una sola responsabilidad mediante dos identidades modulares artificiales.

Cada módulo interno contiene conceptualmente:

```rust
pub type Signature = fn(...);

pub fn operation(...) {
    // implementation
}

pub const OPERATION: Signature = operation;
```

El alias expresa la firma arquitectónica; la función la implementa; el binding tipado obliga al compilador a verificar que ambas permanecen compatibles.

Esta regla extiende el mismo principio de verificación utilizado por Agents sin introducir otra abstracción.

## MSD-004 — Agents conservan definición e implementación separadas

Status: CLOSED

El Use Case es una definición pública y el Agent es su implementación exacta.

```text
definitions/use_cases/compile.rs
        ↓ implemented by
agents/compiler.rs
```

Cada Agent conserva binding tipado hacia su Use Case:

```rust
pub const COMPILE: compile::Compile = compile;
```

Equivalentes para `ExecuteCompiled` y `ExecuteSource`.

## MSD-005 — Nombres exactos de Agents

Status: CLOSED

Se aplican nombres semánticos subject-agent:

```text
Compile          → agents/compiler.rs
ExecuteCompiled  → agents/compiled_program_executor.rs
ExecuteSource    → agents/source_executor.rs
```

Funciones concretas:

```text
compiler::compile
compiled_program_executor::execute_compiled
source_executor::execute_source
```

Bindings tipados:

```text
compiler::COMPILE
compiled_program_executor::EXECUTE_COMPILED
source_executor::EXECUTE_SOURCE
```

## MSD-006 — Nombres exactos de Collaborator modules

Status: CLOSED

```text
src/collaborators/
├── lexer.rs
├── parser.rs
├── semantic_analyzer.rs
├── bytecode_compiler.rs
├── execution_initializer.rs
└── instruction_executor.rs
```

Correspondencia:

```text
lexer
    Lex
    lex_source
    LEX_SOURCE

parser
    Parse
    parse_tokens
    PARSE_TOKENS

semantic_analyzer
    Analyze
    analyze_program
    ANALYZE_PROGRAM

bytecode_compiler
    Lower
    lower_program
    LOWER_PROGRAM

execution_initializer
    Initialize
    initialize_execution
    INITIALIZE_EXECUTION

instruction_executor
    ExecuteInstruction
    execute_instruction
    EXECUTE_INSTRUCTION
```

## MSD-007 — Nombre exacto del Resolver module

Status: CLOSED

```text
src/resolvers/external_call_resolver.rs
```

Contiene conceptualmente:

```text
ResolveExternalCall
resolve_external_call
RESOLVE_EXTERNAL_CALL
```

No se crea un módulo Contract adicional alrededor de `ExternalCapability`.

## MSD-008 — Nombres exactos de Tool modules

Status: CLOSED

```text
src/tools/
├── matches_value_shape.rs
├── materialize_value.rs
├── own_runtime_value.rs
├── locate_source_span.rs
├── observe_runtime_value.rs
├── matches_owned_value_shape.rs
├── materialize_owned_value.rs
└── contextualize_compile_failure.rs
```

Cada archivo representa exactamente una Tool arquitectónica cerrada y contiene su alias, función concreta y binding tipado correspondiente.

No se agrupan en `utils.rs`, `value_tools.rs`, `vm_helpers.rs` u otros módulos genéricos que borren identidad semántica.

## MSD-009 — Estructura conductual prevista

Status: CLOSED

```text
evo-script-engine/src/
├── definitions/
│   └── use_cases/
│       ├── compile.rs
│       ├── execute_compiled.rs
│       └── execute_source.rs
│
├── agents/
│   ├── compiler.rs
│   ├── compiled_program_executor.rs
│   └── source_executor.rs
│
├── collaborators/
│   ├── lexer.rs
│   ├── parser.rs
│   ├── semantic_analyzer.rs
│   ├── bytecode_compiler.rs
│   ├── execution_initializer.rs
│   └── instruction_executor.rs
│
├── resolvers/
│   └── external_call_resolver.rs
│
└── tools/
    ├── matches_value_shape.rs
    ├── materialize_value.rs
    ├── own_runtime_value.rs
    ├── locate_source_span.rs
    ├── observe_runtime_value.rs
    ├── matches_owned_value_shape.rs
    ├── materialize_owned_value.rs
    └── contextualize_compile_failure.rs
```

Conteo conductual exacto:

```text
Use Case definition modules   3
Agent modules                 3
Collaborator modules          6
Resolver modules              1
Tool modules                  8
                             ──
TOTAL                        21
```

Los `mod.rs`, `lib.rs` y módulos puramente de datos no se cuentan como Participants conductuales. La organización física de las 140 identities del Technical Data Model no se reabre aquí; esas identities ya poseen autoridad en Technical Data Diagram y se referencian desde signatures según necesidad.

## MSD-010 — Relaciones permitidas en los diagramas

Status: CLOSED

Los D2 Module Signature Diagrams pueden mostrar únicamente relaciones demostradas:

```text
Use Case definition
    → implemented by Agent

Agent
    → coordinates Collaborator
    → resolves through Resolver
    → uses Tool cuando la Tool es invocada directamente por el Agent

Collaborator
    → uses Tool

Resolver
    → uses Tool
    → invokes ExternalCapability boundary
```

No se muestran flechas helper→helper privadas, families de opcode, parser productions o métodos internos no arquitectónicos.

## Closure

```text
MSD-001..MSD-010                     ✅ CLOSED
Conductual module identities          ✅ CLOSED — 21
Use Case module paths                 ✅ CLOSED — 3
Agent module paths                    ✅ CLOSED — 3
Collaborator module paths             ✅ CLOSED — 6
Resolver module paths                 ✅ CLOSED — 1
Tool module paths                     ✅ CLOSED — 8
Typed binding policy                  ✅ CLOSED

NEXT
    Build D2 Module Signature Diagram suite
```
