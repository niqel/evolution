# US-003 — Execute Evo-Script Source

Status: REVALIDATED — FUNCTIONAL CLOSED

## Historia

```text
Como Consumer,
quiero proporcionar el Source Text completo de un programa Evo-Script
y los Invocation Values requeridos para su ejecución
al Evo-Script Engine,
para ejecutar el programa conforme a Evo-Script
y obtener su Result.
```

## Contexto

`Execute Source` compila y ejecuta Source Text como una sola operación pública. El Consumer no necesita invocar previamente `Compile` ni administrar externamente el Compiled Program temporal.

```text
Consumer
   │
   ├── Source Text
   ├── Invocation Values 0..N
   └── explicit capability bindings
           │
           ▼
     Evo-Script Engine
           │
           ├── Compile semantics
           │       ↓
           │  Compiled Program
           │       ↓
           └── Execute Compiled semantics
                   │
                   ▼
                 Result
```

La frontera `.efn` / Host se rige por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.

## Input Boundary

- Source Text != File Path.
- Source Text != AST / Token Sequence.
- Source Text != Individual Function.
- Source Text != Compiled Program.
- Invocation Values != Command-Line Strings.
- Invocation Values no transportan Active Scope ni estado interactivo del Consumer.
- el Engine no realiza filesystem I/O para obtener Source Text.

## Semantic Equivalence

Bajo el mismo Source Text, Invocation Values y capacidades externas explícitas:

```text
Execute Source(source, values)

        ≡

Compile(source)
    ↓ success
Execute Compiled(compiled, values)
```

La equivalencia es semántica y no obliga a una composición concreta de funciones Rust.

## Invocation Values

1. cardinalidad `0..N Value`;
2. aridad exacta respecto de Parameters;
3. binding estrictamente posicional;
4. compatibilidad semántica exacta;
5. sin conversiones implícitas;
6. mismatch de aridad o tipo produce Result fallido.

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

## Compilation and Execution

La compilación:

1. valida lexical syntax;
2. valida grammar;
3. valida semantics;
4. genera bytecode;
5. puede conservar External Symbols;
6. no requiere concrete Providers o execution bindings;
7. no ejecuta si Compile falla.

La ejecución posterior:

1. utiliza el Compiled Program temporal;
2. enlaza Invocation Values;
3. ejecuta bytecode;
4. mantiene estado local independiente;
5. satisface External Symbols solo mediante bindings explícitos;
6. produce Result.

## Local Execution State

Conceptualmente:

```text
Local Execution State
├── Pipeline Data
├── function / frame evaluation state
└── temporary Values
```

Explícitamente no contiene:

```text
Active Scope
Host Prompt
CLI/UI/API Session State
Current Provider
```

Reglas:

- `.efn` no hereda Scope de una terminal, UI, API u otra ejecución;
- `.efn` no establece ni cambia Active Scope;
- `use` no forma parte de `.efn`;
- Pipeline representa composición de datos;
- el estado termina al concluir la invocación;
- el Engine no mantiene Session implícita persistente.

## External Symbols and Capabilities

```text
External Symbol
      │
      ▼
explicit application binding
      │
      ├── available   ──► continue execution
      └── unavailable ──► Result failure
```

- Compile puede preservar External Symbols sin concrete Provider;
- Engine no descubre Providers;
- no existe Provider activo ambiental;
- una misma función puede utilizar diferentes capabilities explícitas;
- una capability requerida no disponible al alcanzarse produce Result fallido.

## Consumer Neutrality

La semántica del `.efn` no depende de la superficie que lo invoca.

```text
                Result
                  │
        ┌─────────┼─────────┐
        ▼         ▼         ▼
     evo-cli    evo-ui    evo-api
```

Presentación, impresión, renderizado o serialización pertenecen al Consumer y no al Evo-Script Engine.

## Criterios de Aceptación

1. Consumer proporciona exactamente un Source Text y `0..N Invocation Values`.
2. No necesita invocar Compile previamente.
3. Engine compila conforme a Evo-Script vigente.
4. El programa produce bytecode antes de ejecución.
5. Compilation Failure evita ejecución y produce Result fallido.
6. Compiled Program temporal no se expone ni persiste implícitamente.
7. Binding de Invocation Values es posicional, exacto y sin coerciones implícitas.
8. Cada ejecución tiene estado local independiente.
9. No existe Active Scope dentro de `.efn`.
10. `use` no forma parte de `.efn`.
11. Pipeline Data representa composición de datos.
12. External Symbols se satisfacen mediante bindings explícitos.
13. Engine no descubre Providers ni mantiene Current Provider.
14. Missing capability produce Result fallido cuando es requerida.
15. Success preserva Value mediante Result exitoso.
16. Engine no realiza filesystem I/O ni presentación.
17. Al concluir no queda Session implícita.
18. CLI, UI, API u otro Consumer no alteran la semántica del `.efn`.
19. Execute Source es semánticamente equivalente a Compile + Execute Compiled bajo las mismas entradas/capabilities.

## No Responsabilidades

- leer `.efn` desde filesystem;
- persistir/cachear Compiled Program;
- command-line parsing;
- terminal/UI/HTTP presentation;
- Interactive Host Scope / prompt;
- lifecycle de Evo Application;
- Provider discovery;
- VM internals, Participants o Rust Signatures en esta fase funcional.

## Closure

US-003 fue revalidada contra Purpose, Public Capabilities y `EFN_HOST_BOUNDARY_v0.1.md`.

**US-003 queda `REVALIDATED — FUNCTIONAL CLOSED`.**
