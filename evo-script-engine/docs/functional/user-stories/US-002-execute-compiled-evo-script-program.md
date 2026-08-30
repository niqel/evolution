# US-002 — Execute Compiled Evo-Script Program

Status: REVALIDATED — FUNCTIONAL CLOSED

## Historia

```text
Como Consumer,
quiero proporcionar un Compiled Program y los Invocation Values
requeridos para su ejecución al Evo-Script Engine,
para ejecutar el programa compilado conforme a Evo-Script
y obtener su Result.
```

## Contexto

`Execute Compiled` ejecuta bytecode ya compilado. El Consumer proporciona un `Compiled Program`, `0..N Invocation Values` y, cuando corresponda, una composición explícita capaz de satisfacer los External Symbols alcanzados durante ejecución.

```text
Consumer
   │
   ├── Compiled Program
   ├── Invocation Values 0..N
   └── explicit capability bindings
           │
           ▼
     Evo-Script Engine
           │
           ├── bind Invocation Values
           ├── execute bytecode
           ├── maintain local evaluation state
           └── resolve External Symbols explicitly
           │
           ▼
         Result
```

La frontera `.efn` / Host se rige por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.

## Invocation Values

1. contienen `0..N Value`;
2. una Public Function sin Parameters requiere cero Values;
3. una Public Function con `N Parameters` requiere exactamente `N Values`;
4. el binding es estrictamente posicional;
5. cada Value debe ser semánticamente compatible con el Parameter correspondiente;
6. no existen coerciones implícitas para reparar incompatibilidades;
7. mismatch de aridad o tipo produce Result fallido;
8. Invocation Values no son command-line strings ni transportan estado de Host.

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

## Execution State

Cada invocación mantiene estado local independiente necesario únicamente para ejecutar el programa.

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

1. una ejecución no comparte implícitamente su estado con otra;
2. una ejecución `.efn` no recibe ni hereda Active Scope del Consumer;
3. `.efn` no establece ni cambia Active Scope;
4. Pipeline Data representa composición de datos;
5. el estado local termina con la invocación;
6. el Consumer no necesita proporcionar Session, Context o Service Locator;
7. el mismo Compiled Program puede ejecutarse desde distintos Consumers sin cambiar su semántica.

## External Symbols and Capabilities

Un Compiled Program puede contener External Symbols preservados durante Compile.

```text
External Symbol
      │
      ▼
explicit application binding
      │
      ├── available   ──► continue
      └── unavailable ──► Result failure
```

Reglas:

1. External Symbols se resuelven durante ejecución;
2. capabilities requeridas llegan mediante composición explícita;
3. el Engine no descubre Providers;
4. no existen global registries o Service Locator;
5. no existe un Provider activo ambiental;
6. distintas capabilities pueden resolverse mediante distintos bindings durante una misma ejecución;
7. el mecanismo técnico exacto se define posteriormente.

## Consumer Neutrality

El `.efn` no conoce si el Consumer es CLI, UI, API u otra superficie.

```text
                Result
                  │
        ┌─────────┼─────────┐
        ▼         ▼         ▼
     evo-cli    evo-ui    evo-api
```

Imprimir, renderizar, responder HTTP o cualquier reacción específica pertenece al Consumer y su composición exterior.

## Criterios de Aceptación

1. El Consumer puede proporcionar un Compiled Program válido.
2. Puede proporcionar `0..N Invocation Values`.
3. Execute Compiled no recibe Source Text ni recompila.
4. Ejecuta bytecode del Compiled Program.
5. Binding de Values es posicional, exacto y sin coerciones implícitas.
6. Cada ejecución mantiene estado local independiente.
7. No existe Active Scope dentro de la ejecución `.efn`.
8. `use` no forma parte de `.efn`.
9. Pipeline Data representa únicamente composición de datos.
10. External Symbols se satisfacen mediante bindings explícitos.
11. El Engine no descubre Providers ni mantiene Current Provider.
12. Una capability requerida no disponible produce Result fallido.
13. Success preserva el Value producido; failure produce Result fallido.
14. Un mismo Compiled Program puede reutilizarse con distintos Invocation Values y Consumers.
15. Al retornar Result concluye esa invocación.
16. Presentación y reacción al Result quedan fuera del Engine.

## No Responsabilidades

- compilación de Source Text;
- carga física de Compiled Programs;
- persistencia del estado después de la invocación;
- terminal, stdout, UI o protocolos de presentación;
- Interactive Host Scope o prompt;
- lifecycle de Evo Application;
- implementación o descubrimiento de Providers;
- arquitectura concreta de VM;
- Participants y Rust Signatures.

## Revalidation

US-002 fue revalidada contra Purpose, Public Capabilities y `EFN_HOST_BOUNDARY_v0.1.md`.

**US-002 queda `REVALIDATED — FUNCTIONAL CLOSED`.**
