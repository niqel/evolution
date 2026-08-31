# Evo-Script Engine — Execute Source Participant Design

Status: EXECUTE SOURCE PARTICIPANT DESIGN — CLOSED

Este documento cierra la composición exacta de Participants internos del Use Case `ExecuteSource`.

La firma raíz ya cerrada es:

```rust
pub type ExecuteSource =
    for<'source, 'value, 'catalog, 'bindings> fn(
        &'source str,
        &'value [Value<'value>],
        &'catalog CompilationCatalog,
        &'bindings ApplicationBindings,
    ) -> ExecutionOutcome;
```

Bajo `RSD-010`, `ExecuteSource` no llama a `Compile Agent` ni a `ExecuteCompiled Agent`. Coordina directamente las firmas internas de ambas fases.

## RSD-037 — `contextualize_compile_failure` es Tool de adaptación de Outcome

Status: CLOSED

`ExecuteSource` utiliza `ExecutionOutcome`, pero sus cuatro Collaborators de compilación producen `CompileFailure` cuando fallan.

La transformación cerrada por `EXF-003` es una operación pequeña, determinista e independiente de la fase concreta que produjo el failure:

```text
CompileFailure {
    kind,
    source_span,
}
        ↓
ExecutionFailure {
    kind: Compilation(kind),
    source_span: Some(source_span),
}
```

Firma cerrada:

```rust
pub type ContextualizeCompileFailure =
    fn(
        CompileFailure,
    ) -> ExecutionFailure;
```

Invariantes:

- consume el `CompileFailure` owned;
- mueve `CompileFailure.kind` a `ExecutionFailureKind::Compilation(...)`;
- mueve `CompileFailure.source_span` a `Some(source_span)`;
- no duplica provenance;
- no inspecciona Lexical/Syntax/Semantic variants;
- no formatea diagnostics;
- no conoce Agent, Parser, Semantic Analyzer, VM o Provider;
- no puede fallar.

El Agent utiliza esta Tool al propagar cualquier failure de la fase de compilación; no realiza la transformación inline.

## RSD-038 — Orquestación exacta de `ExecuteSource Agent`

Status: CLOSED

Árbol completo:

```text
ExecuteSource Agent
│
├── COMPILE PHASE
│   ├── lex_source                 Collaborator
│   ├── parse_tokens               Collaborator
│   ├── analyze_program            Collaborator
│   └── lower_program              Collaborator
│
├── contextualize_compile_failure  Tool
│
└── EXECUTION PHASE
    ├── initialize_execution       Collaborator
    ├── execute_instruction        Collaborator
    └── resolve_external_call      Resolver
```

Flujo conceptual:

```rust
fn execute_source(
    source: &str,
    invocation_values: &[Value<'_>],
    catalog: &CompilationCatalog,
    application_bindings: &ApplicationBindings,
) -> ExecutionOutcome {
    let tokens =
        lex_source(source)
            .map_err(contextualize_compile_failure)?;

    let program =
        parse_tokens(&tokens, source)
            .map_err(contextualize_compile_failure)?;

    let semantic_program =
        analyze_program(&program, catalog)
            .map_err(contextualize_compile_failure)?;

    let compiled_program =
        lower_program(&semantic_program);

    let mut execution = initialize_execution(
        &compiled_program,
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

La representación anterior expresa orquestación, no implementación final.

Invariantes:

- `CompiledProgram` local permanece vivo durante toda la `VmExecution` que lo borrowea;
- `ExecutionOutcome` no conserva borrow hacia el `CompiledProgram` local porque success es `OwnedValue` y failure es owned;
- `ExecuteSource` no crea un pipeline alternativo: reutiliza exactamente las mismas signatures de Compile y ExecuteCompiled;
- no existe Agent → Agent;
- no existe mega-Collaborator que coordine ambas fases;
- compilation failure se contextualiza exactamente una vez mediante `contextualize_compile_failure`;
- después de lowering, la ejecución es idéntica semánticamente al camino de `ExecuteCompiled`.

Inventario del Use Case `ExecuteSource`:

```text
Use Case        1
Agent           1
Collaborators   6
                4 compile
                2 execution
Resolvers       1
Contracts       0 adicionales
Requesters      0
Tools           8 disponibles/usadas en el flujo completo
                7 execution Tools
                1 contextualize_compile_failure
```

Los siete Tools de ejecución son:

```text
matches_value_shape
materialize_value
own_runtime_value
locate_source_span
observe_runtime_value
matches_owned_value_shape
materialize_owned_value
```

## Closure

```text
RSD-037 contextualize_compile_failure Tool   ✅ CLOSED
RSD-038 ExecuteSource exact orchestration     ✅ CLOSED

ExecuteSource Participant Design              ✅ CLOSED
Execution Participant Design                  READY FOR FINAL CLOSURE
```
