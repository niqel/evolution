# Evo-Script Engine — D2 Sequence Diagrams

Status: D2 SEQUENCE DIAGRAMS — CLOSED

Este directorio contiene las secuencias dinámicas canónicas de `evo-script-engine` v0 derivadas exclusivamente de Rust Signatures, Participants y Module Signature Diagrams ya cerrados.

## Canonical suite

```text
00-compile.d2            ✅ BUILT
01-execute-compiled.d2   ✅ BUILT
02-call-external.d2      ✅ BUILT
03-execute-source.d2     ✅ BUILT
```

## 00 — Compile

Representa la secuencia exacta:

```text
Consumer
  ↓ COMPILE
compiler Agent
  ↓ LEX_SOURCE
lexer
  ↓ PARSE_TOKENS
parser
  ↓ ANALYZE_PROGRAM
semantic_analyzer
  ↓ LOWER_PROGRAM
bytecode_compiler
  ↓
CompiledProgram
```

La primera `Err(CompileFailure)` de Lexer, Parser o Semantic Analyzer termina el flujo. Lowering no posee failure normal.

## 01 — ExecuteCompiled

Representa:

```text
Consumer
  ↓ EXECUTE_COMPILED
compiled_program_executor
  ↓ INITIALIZE_EXECUTION
execution_initializer
  ↓
VmExecution
  ↓ execution loop
instruction_executor
  ↓ entry Return
OwnedValue
```

Incluye el uso explícito de:

```text
matches_value_shape
materialize_value
own_runtime_value
```

`CallExternal` se separa deliberadamente en la vista `02-call-external.d2` porque cruza una frontera técnica externa.

## 02 — CallExternal

Representa la única frontera técnica externa de ejecución:

```text
Agent
  ↓ RESOLVE_EXTERNAL_CALL
external_call_resolver
  ↓ observe RuntimeValues
observe_runtime_value
  ↓
Value<'call> arguments
  ↓
ExternalCapability
  ↓
OwnedValue / ExternalCapabilityFailure
```

En success:

```text
matches_owned_value_shape
    ↓
materialize_owned_value
    ↓
RuntimeValue
    ↓
N → 1 commit
    ↓
ip += 1
```

En failure se reutiliza `locate_source_span`; no hay stack commit y el IP permanece en `CallExternal`.

## 03 — ExecuteSource

Representa composición directa:

```text
Consumer
  ↓ EXECUTE_SOURCE
source_executor
  ├── Compile participants
  ├── contextualize_compile_failure cuando corresponda
  └── Execution participants
```

No existe:

```text
source_executor → Compile Agent
source_executor → ExecuteCompiled Agent
```

`ExecuteSource` reutiliza las mismas firmas internas y conserva un `CompiledProgram` local durante la `VmExecution`.

## Sequence invariants

1. Toda lifeline conductual corresponde a un módulo/Participant cerrado, salvo `Consumer` y la frontera técnica explícita `ExternalCapability`.
2. Toda llamada entre lifelines corresponde a una firma cerrada o al ABI `ExternalCapability` ya cerrado.
3. Ningún Agent llama otro Agent.
4. Ningún Collaborator llama otro Collaborator.
5. Solo `external_call_resolver` invoca `ExternalCapability`.
6. Requesters no aparecen porque el inventario v0 es exactamente 0.
7. No se introduce un Contract duplicado para `ExternalCapability`.
8. Los helpers privados no aparecen como lifelines.
9. El resultado público exitoso de ejecución es siempre `OwnedValue`.
10. Los diagramas no reabren el Technical Data Model ni las Rust Signatures.

## Audit

```text
Canonical sequence files present          4 / 4 ✅
Public Use Cases covered                  3 / 3 ✅
Compile participants traced               ✅
Execution participants traced             ✅
External boundary traced                  ✅
Agent → Agent calls                       0 ✅
Collaborator → Collaborator calls         0 ✅
Unknown architectural calls introduced    0 ✅
```

El entorno utilizado para esta edición no dispone de ejecutable `d2`; por tanto no se declara una validación local de render/parser que no se realizó. Los `.d2` son la autoridad versionada y SVG/PNG/PDF serían outputs derivados.

## Closure

```text
D2 Sequence Diagram suite       ✅ CLOSED — 4 views

NEXT ARCHITECTURAL STAGE
    Implementation Tasks
```
