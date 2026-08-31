# Evo-Script Engine — Execution Participant Design

Status: EXECUTION PARTICIPANT DESIGN — CLOSED

Este documento cierra las Rust Signatures y Participants internos requeridos por los Use Cases `ExecuteCompiled` y `ExecuteSource` para la fase de ejecución.

La autoridad deriva de:

- `ROOT_SIGNATURE_DESIGN.md`;
- `COMPILE_PARTICIPANT_DESIGN.md`;
- `TECHNICAL_DESIGN.md`;
- `docs/technical/data-model/`;
- `TECHNICAL_DESIGN_METHODOLOGY.md`;
- `ENGINEERING_PRINCIPLES.md`.

Los nombres de artifacts, Participants y conceptos técnicos canónicos se mantienen en English; las explicaciones, decisiones, reglas e invariantes se redactan en español.

## Execution participant tree — root

Árbol cerrado:

```text
ExecuteCompiled Agent
├── initialize_execution       Collaborator
├── execute_instruction        Collaborator
└── resolve_external_call      Resolver
```

`ExecuteSource Agent`, después de completar `lex_source → parse_tokens → analyze_program → lower_program`, reutiliza directamente estas mismas firmas de ejecución bajo `RSD-010`; no llama a `ExecuteCompiled Agent`.

No se introduce un mega-Collaborator `execute_program` / `execute_vm` cuya responsabilidad sea coordinar otros Collaborators y Resolvers. El loop de ejecución y la decisión de qué Participant interviene pertenecen al Agent.

## RSD-021 — El Agent posee la orquestación dinámica de ejecución

Status: CLOSED

La ejecución bytecode requiere coordinación dinámica porque la instruction actualmente alcanzada determina qué responsabilidad participa.

```text
current instruction
    │
    ├── CallExternal
    │      ↓
    │   resolve_external_call
    │      Resolver
    │
    └── cualquier otra instruction v0
           ↓
        execute_instruction
           Collaborator
```

El Agent puede observar la instruction actual únicamente para decidir qué Participant participa. No implementa semántica de opcodes, no manipula directamente Operand/Frame state y no cruza la frontera `ExternalCapability`.

La repetición del loop pertenece a la orquestación del Agent. No se crea `ExecutionLoop`, `VmRunner`, `ExecutionManager` ni otro Participant cuya única responsabilidad sea esconder esta coordinación.

## RSD-022 — Firma exacta de `initialize_execution`

Status: CLOSED

```rust
pub type Initialize =
    for<'compiled, 'value, 'bindings> fn(
        &'compiled CompiledProgram,
        &'value [Value<'value>],
        &'bindings ApplicationBindings,
    ) -> Result<
        VmExecution<'compiled, 'bindings>,
        ExecutionFailure,
    >;
```

Responsabilidad:

```text
CompiledProgram
+ Invocation Values
+ ApplicationBindings
        ↓
initialize_execution
        ├── validate invocation boundary
        ├── materialize Parameter RuntimeValues
        ├── reserve Locals
        ├── create entry CallFrame
        └── materialize VmExecution
```

Invariantes:

- toda `InvocationFailure` ocurre antes de existir una `VmExecution` válida;
- `InvocationFailure` produce `source_span: None`;
- `ApplicationBindings` no se preflight-inspecciona; missing bindings permanecen lazy hasta `CallExternal`;
- Invocation Values no quedan borrowed dentro de `VmExecution`;
- entry frame usa `entry_point`, `InstructionPointer(0)` y `frame_base = 0`;
- no se introduce `InvocationContext`, `ExecutionContext` o `Session`.

## RSD-023 — Firma exacta de `execute_instruction`

Status: CLOSED

```rust
pub type ExecuteInstruction =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<
        Option<OwnedValue>,
        ExecutionFailure,
    >;
```

```text
Ok(None)
    = instruction completada; ejecución continúa

Ok(Some(OwnedValue))
    = entry Return completado; ejecución termina
```

`CallExternal` queda fuera de esta responsabilidad. Las 47 instructions restantes pertenecen a una sola responsabilidad de ejecución interna y no se promueven automáticamente a Collaborators independientes.

## RSD-024 — `resolve_external_call` es Resolver

Status: CLOSED

```rust
pub type ResolveExternalCall =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<(), ExecutionFailure>;
```

Precondición:

```text
current instruction = CallExternal(ExternalSymbolId)
```

El Resolver realiza lookup explícito por `SignatureSymbol`, adapta argumentos a `Value<'a>`, invoca `ExternalCapability`, valida el `OwnedValue` de retorno, contextualiza failures externas, materializa el result runtime y aplica el commit `N → 1` únicamente tras success.

## RSD-025 — No Contract ni Requester duplicados para `CallExternal`

Status: CLOSED

La frontera runtime ya posee la firma exacta:

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure>;
```

Por tanto:

```text
Contract adicional       0
Requester CallExternal   0
Resolver                  1
```

`ExternalCapability` sigue siendo la frontera técnica existente; no se envuelve con otro function-pointer type llamado `Contract` únicamente por taxonomía.

## RSD-026 — Orquestación raíz de `ExecuteCompiled Agent`

Status: CLOSED

```rust
fn execute_compiled(
    compiled_program: &CompiledProgram,
    invocation_values: &[Value<'_>],
    application_bindings: &ApplicationBindings,
) -> ExecutionOutcome {
    let mut execution = initialize_execution(
        compiled_program,
        invocation_values,
        application_bindings,
    )?;

    loop {
        if current_instruction_is_call_external(&execution) {
            resolve_external_call(&mut execution)?;
            continue;
        }

        if let Some(result) = execute_instruction(&mut execution)? {
            return Ok(result);
        }
    }
}
```

El Agent no valida shapes, no materializa backing, no ejecuta opcodes, no invoca `ExternalCapability` y no traduce `ExternalCapabilityFailure`.

`current_instruction_is_call_external` permanece mecanismo privado del Agent; no se promueve a Tool sin responsabilidad independiente demostrada.

## Detailed branch closures

Las ramas internas se cierran en documentos especializados:

```text
EXECUTION_INITIALIZATION_DESIGN.md
    RSD-027..RSD-029

INSTRUCTION_EXECUTION_DESIGN.md
    RSD-030..RSD-032

EXTERNAL_CALL_RESOLUTION_DESIGN.md
    RSD-033..RSD-036

EXECUTE_SOURCE_PARTICIPANT_DESIGN.md
    RSD-037..RSD-038
```

## RSD-039 — Inventario exacto de Participants de ejecución

Status: CLOSED

Participants propios de la fase de ejecución:

```text
Collaborators
├── initialize_execution
└── execute_instruction

Resolver
└── resolve_external_call

Tools
├── matches_value_shape
├── materialize_value
├── own_runtime_value
├── locate_source_span
├── observe_runtime_value
├── matches_owned_value_shape
└── materialize_owned_value
```

`ExecuteSource` agrega para la transición Compile → Execution:

```text
Tool
└── contextualize_compile_failure
```

y reutiliza directamente los cuatro Collaborators de Compile:

```text
lex_source
parse_tokens
analyze_program
lower_program
```

Inventario por Use Case:

```text
ExecuteCompiled
├── Use Case        1
├── Agent           1
├── Collaborators   2
├── Resolvers       1
├── Contracts       0 adicionales
├── Requesters      0
└── Tools           7 únicas disponibles/usadas

ExecuteSource
├── Use Case        1
├── Agent           1
├── Collaborators   6
│                   4 Compile + 2 Execution
├── Resolvers       1
├── Contracts       0 adicionales
├── Requesters      0
└── Tools           8 únicas disponibles/usadas
                    7 Execution + contextualize_compile_failure
```

No se introducen en v0:

```text
ExecutionManager
VmRunner
ExecutionLoop Participant
ExecutionContext
Session
Instruction-per-Collaborator
Provider identity inside Engine
Requester for CallExternal
second Contract wrapper around ExternalCapability
```

## RSD-040 — Rust Signatures y Participant Design quedan cerrados

Status: CLOSED

Inventario único de `evo-script-engine` v0:

```text
Use Cases        3
├── Compile
├── ExecuteCompiled
└── ExecuteSource

Agents           3
├── Compile Agent
├── ExecuteCompiled Agent
└── ExecuteSource Agent

Collaborators    6 unique
├── lex_source
├── parse_tokens
├── analyze_program
├── lower_program
├── initialize_execution
└── execute_instruction

Resolvers        1 unique
└── resolve_external_call

Requesters       0
Contracts        0 additional Engine Contract types

Tools            8 unique
├── matches_value_shape
├── materialize_value
├── own_runtime_value
├── locate_source_span
├── observe_runtime_value
├── matches_owned_value_shape
├── materialize_owned_value
└── contextualize_compile_failure
```

`ExternalCapability` permanece como la única frontera runtime function-pointer ya definida por el modelo técnico y no se duplica como otro Contract type.

Todas las firmas necesarias para derivar módulos y comportamiento dinámico están cerradas. Ningún Module Signature Diagram o D2 Sequence Diagram puede introducir Participants o llamadas que no puedan rastrearse hacia este diseño.

## Closure

```text
RSD-021..RSD-040                    ✅ CLOSED
Execution root signatures            ✅ CLOSED
Execution Collaborators              ✅ CLOSED — 2 unique
Execution Resolver                   ✅ CLOSED — 1 unique
Execution Tools                      ✅ CLOSED — 7 unique
ExecuteSource contextualization Tool ✅ CLOSED — 1 additional
Requesters                           ✅ 0
Additional Contract types            ✅ 0

Compile Participant Design           ✅ CLOSED
Execution Participant Design         ✅ CLOSED
Rust Signatures / Participant Design ✅ CLOSED

NEXT ARCHITECTURAL STAGE
    Module Signature Diagrams
```
