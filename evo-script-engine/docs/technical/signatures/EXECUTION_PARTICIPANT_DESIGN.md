# Evo-Script Engine — Execution Participant Design

Status: EXECUTION PARTICIPANT DESIGN — IN PROGRESS

Este documento cierra progresivamente las Rust Signatures y Participants internos requeridos por los Use Cases `ExecuteCompiled` y `ExecuteSource` para la fase de ejecución.

La autoridad deriva de:

- `ROOT_SIGNATURE_DESIGN.md`;
- `COMPILE_PARTICIPANT_DESIGN.md`;
- `TECHNICAL_DESIGN.md`;
- `docs/technical/data-model/`;
- `TECHNICAL_DESIGN_METHODOLOGY.md`;
- `ENGINEERING_PRINCIPLES.md`.

Los nombres de artifacts, Participants y conceptos técnicos canónicos se mantienen en English; las explicaciones, decisiones, reglas e invariantes se redactan en español.

## Execution participant tree — root

Dirección raíz cerrada:

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

Regla:

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

El Agent puede observar la instruction actual únicamente para decidir qué Participant participa. No implementa la semántica de los opcodes, no manipula directamente Operand/Frame state y no cruza la frontera `ExternalCapability`.

La repetición del loop pertenece a la orquestación del Agent:

```text
initialize execution
      ↓
inspect current instruction
      ↓
select Collaborator / Resolver
      ↓
continue until entry Return or first Failure
```

No se crea `ExecutionLoop`, `VmRunner`, `ExecutionManager` ni otro Participant cuya única responsabilidad sea esconder esta coordinación.

## RSD-022 — Firma exacta de `initialize_execution`

Status: CLOSED

`initialize_execution` es un Collaborator interno responsable de materializar una `VmExecution` válida a partir de un `CompiledProgram`, los `Invocation Values` y los `ApplicationBindings` explícitos.

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

Responsabilidad completa:

```text
CompiledProgram
+ Invocation Values
+ ApplicationBindings
        ↓
initialize_execution
        │
        ├── validate invocation boundary
        │      arity exact
        │      CompiledValueShape exact
        │      no implicit coercion
        │
        └── success
               ↓
          VmExecution
```

Invariantes:

- toda `InvocationFailure` ocurre antes de existir una `VmExecution` válida;
- `InvocationFailure` produce `ExecutionFailure { source_span: None, ... }`;
- `ApplicationBindings` no se inspecciona para exigir bindings anticipadamente; únicamente queda borrowed por `VmExecution`;
- capabilities faltantes permanecen lazy y solo fallan si se alcanza `CallExternal`;
- los `Invocation Values` no quedan borrowed dentro de `VmExecution`;
- los Values necesarios se materializan hacia `RuntimeValue` / `ExecutionBackingStore` owned por la ejecución;
- el entry `CallFrame` se crea con `entry_point`, `InstructionPointer(0)` y `frame_base = 0`;
- se reservan los Local Slots conforme al `CompiledFunction` de entry;
- no se introduce `InvocationContext`, `ExecutionContext`, `Session` o wrapper equivalente.

`initialize_execution` no cruza frontera externa y por tanto no es Resolver.

## RSD-023 — Firma exacta de `execute_instruction`

Status: CLOSED

`execute_instruction` es un Collaborator interno responsable de ejecutar exactamente la instruction actualmente activa cuando dicha instruction no es `CallExternal`.

```rust
pub type ExecuteInstruction =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<
        Option<OwnedValue>,
        ExecutionFailure,
    >;
```

Interpretación del success:

```text
Ok(None)
    = instruction completada;
      la ejecución continúa

Ok(Some(OwnedValue))
    = entry Return completado;
      la ejecución termina exitosamente
```

No se introduce `ExecutionStep`, `VmSignal`, `InstructionOutcome` ni otro enum de control sin necesidad; `Option<OwnedValue>` expresa completamente las dos posibilidades normales de success requeridas por el loop.

Invariantes:

- la instruction actual nunca es `CallExternal`; esa frontera pertenece al Resolver;
- movement, internal `Call`, `Return`, control flow, numeric operations, conversions, equality y composite mechanics pertenecen a esta responsabilidad de ejecución interna;
- el Collaborator preserva la semántica cerrada de `InstructionPointer`: commit de IP solo después de success;
- un internal `Call` crea el frame correspondiente sin avanzar previamente el caller IP;
- internal `Return` elimina el callee y reanuda al caller;
- entry `Return` materializa `OwnedValue` antes de que `VmExecution` termine;
- los failures normales que nacen aquí pertenecen exclusivamente a `ExecutionFailureKind::Evaluation(...)` y conservan `Some(SourceSpan)` de la instruction responsable;
- invariant violations de VM/compiler no se convierten en `ExecutionFailure` normal.

Las 47 instructions no externas de v0 no se convierten automáticamente en 47 Collaborators. La separación interna por familias, funciones o helpers se analiza posteriormente bajo `RSD-011`.

## RSD-024 — `resolve_external_call` es Resolver

Status: CLOSED

`resolve_external_call` cruza la frontera técnica explícita `ExternalCapability` alcanzada mediante `ApplicationBindings` y por tanto es un Resolver interno del Engine.

Firma cerrada:

```rust
pub type ResolveExternalCall =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<
        (),
        ExecutionFailure,
    >;
```

Precondición arquitectónica:

```text
current instruction
    = CallExternal(ExternalSymbolId)
```

Responsabilidad:

```text
CallExternal
    ↓
ExternalSymbolId
    ↓
ExternalSymbol
    ↓ SignatureSymbol
ApplicationBindings lookup
    │
    ├── missing
    │      ↓
    │   ExecutionFailure::External(MissingBinding)
    │
    └── found
           ↓
      ExternalCapability
           ↓
      Result<OwnedValue, ExternalCapabilityFailure>
           │
           ├── Err
           │     ↓
           │  ExecutionFailure::External(CapabilityFailure)
           │
           └── Ok(OwnedValue)
                  ↓
             validate result_shape
                  │
                  ├── mismatch
                  │      ↓
                  │  ResultContractMismatch
                  │
                  └── match
                         ↓
                    materialize RuntimeValue
                         ↓
                    commit N → 1
                         ↓
                    ip += 1
```

Invariantes:

- el Resolver no descubre Providers;
- utiliza únicamente `ApplicationBindings` explícitamente suministrado;
- `ExternalCapability` es la frontera function-pointer ya cerrada;
- no se introduce un segundo tipo `Contract` que duplique `ExternalCapability`;
- no utiliza Requester para el one-result ABI de v0;
- los argumentos cruzan como borrowed `Value<'a>` mientras los `RuntimeValue` originales permanecen en `SharedValueStorage`;
- success cruza como `OwnedValue` porque el resultado debe sobrevivir a la capability;
- el resultado se valida antes de materializar y antes del commit `N → 1`;
- failure conserva argumentos e IP sin commit;
- el Resolver traduce/contextualiza la frontera hacia `ExternalExecutionFailure` + `SourceSpan` de la instruction `CallExternal`.

## RSD-025 — No Contract ni Requester duplicados para `CallExternal`

Status: CLOSED

La frontera runtime ya posee la firma técnica exacta:

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

La ausencia de un tipo etiquetado `Contract` adicional no elimina la responsabilidad del Resolver: la frontera externa existe y el Resolver realiza lookup, adaptación, invocación, validación, contextualización de failure y commit del resultado.

## RSD-026 — Orquestación raíz de `ExecuteCompiled Agent`

Status: CLOSED

Flujo conceptual:

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

La representación anterior expresa exclusivamente la coordinación. `current_instruction_is_call_external` no implica una Tool arquitectónica; puede permanecer como mecanismo privado del Agent mientras no aparezca responsabilidad genérica independiente.

El Agent:

```text
NO valida shapes por sí mismo
NO materializa RuntimeValue / backing
NO ejecuta opcodes
NO invoca ExternalCapability directamente
NO traduce ExternalCapabilityFailure
NO llama otro Agent
```

Decide únicamente qué Participant participa y controla la repetición hasta `OwnedValue` o `ExecutionFailure`.

## Execution participant progress

```text
ExecuteCompiled Agent
├── initialize_execution       ✅ ROOT SIGNATURE CLOSED
├── execute_instruction        ✅ ROOT SIGNATURE CLOSED
└── resolve_external_call      ✅ ROOT SIGNATURE CLOSED

Detailed Tool/private-helper analysis
├── initialize_execution       ← NEXT
├── execute_instruction        PENDING
└── resolve_external_call      PENDING
```

Inventario raíz actualmente cerrado:

```text
Use Case        1  ExecuteCompiled
Agent           1
Collaborators   2  initialize_execution, execute_instruction
Resolvers       1  resolve_external_call
Contracts       0 additional
Requesters      0
Tools           pending detailed analysis
```

## Closure parcial

```text
RSD-021 Agent owns dynamic execution orchestration     ✅ CLOSED
RSD-022 initialize_execution exact signature           ✅ CLOSED
RSD-023 execute_instruction exact signature            ✅ CLOSED
RSD-024 resolve_external_call exact Resolver signature ✅ CLOSED
RSD-025 no duplicate Contract / no Requester            ✅ CLOSED
RSD-026 ExecuteCompiled Agent root orchestration        ✅ CLOSED

Execution Participant Design                           ← IN PROGRESS
Tool/private-helper inventory                          ← NEXT
ExecuteSource execution reuse                          structurally established
Module Signature Diagrams                              AFTER PARTICIPANTS
D2 Sequence Diagrams                                   AFTER SIGNATURES/PARTICIPANTS
Implementation Tasks                                   AFTER DIAGRAMS
```
