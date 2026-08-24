# Evo Runtime Specification v0

Status: MODEL A CLOSED

## 1. Purpose

Evo Runtime Model A defines the minimal platform boundary for starting an Evo
Application. In Model A, the Runtime has a single, minimal responsibility:
initiating execution by invoking the `Run` action provided by an Evo Application,
maintaining the invocation active while the application runs, and delivering the
final `Result` to the caller.

---

## 2. Runtime Boundary

The architectural boundary of Evo Runtime Model A is defined by:
- Exactly **one Use Case** provided by the Runtime (`Start`).
- Exactly **one Requester** consumed from the application (`Run`).
- Exactly **one outcome type** (`Result`) defined by `evo-values`.

```text
Caller / Host
     │
     │ calls Start(Run)
     ▼
┌───────────────────────────────┐
│ Evo Runtime                   │
│                               │
│  Use Case: Start              │
│       │                       │
│       │ calls run()           │
│       ▼                       │
│  Requester: Run               │
└───────┬───────────────────────┘
        │
        ▼
Evo Application (active)
        │
        │ returns Result
        ▼
   Result (evo-values)
```

---

## 3. Start Use Case

- **Category**: Use Case (Provided by `evo-runtime`)
- **Definition**: `definitions/use_cases/start.rs`
- **Function Pointer Type**: `pub type Start = fn(run_request::Request) -> Result;`
- **Semantics**:
  1. Receives the `Run` requester function pointer from the caller.
  2. Invokes `run()`.
  3. Remains active on the call stack for the duration of `run()`.
  4. Returns the `Result` produced by `run()` directly to the caller.
  5. Does not require explicit `stop()`, `close()`, or `finalize()` operations;
     termination of `run()` naturally concludes `start()`.

---

## 4. Run Requester

- **Category**: Requester (Consumed by `evo-runtime` from the Evo Application)
- **Definition**: `definitions/requesters/run_request.rs`
- **Function Pointer Type**: `pub type Request = fn() -> Result;`
- **Semantics**:
  1. Represents the entry point action that the application provides to the
     Runtime.
  2. Encapsulates the application's entire execution lifecycle.
  3. Returns `Result` upon completion.

---

## 5. Result

- `Result` is the canonical outcome type representing the conclusion of an
  execution (success or failure).
- Defined and owned by `evo-values`.
- From the perspective of Evo Runtime, `Result` is a concrete outcome type;
  no generics are exposed across the Runtime boundary.
- `Result != Failure`: a failed outcome is expressed through the failure branch
  of `Result`.

---

## 6. Independent Start Invocations

Evo Runtime supports multiple concurrent or sequential Start invocations:

```text
Start(run_app_1)  ──►  App 1  ──►  Result 1
Start(run_app_2)  ──►  App 2  ──►  Result 2
```

- Each invocation of `Start` is isolated and independent.
- Failure of one application does not affect another application.
- Evo Runtime does not share state across invocations.

---

## 7. Runtime Non-Responsibilities

Evo Runtime Model A deliberately excludes all internal coordination
mechanisms:
- **No Context struct**: The Runtime does not maintain execution context or
  session state.
- **No Execution entity**: The execution lifecycle is represented solely by the
  active call stack of `Start(run)`.
- **No Engine resolution**: The Runtime does not discover, load, or select
  engines (e.g. EvoS, EvoQ).
- **No Provider / Contract management**: Providers and capabilities are not
  managed by the Runtime.
- **No Value transport**: Data flow between operations occurs directly inside
  the application.
- **No Operation resolution**: The Runtime does not resolve dependencies or
  symbols.

---

## 8. Engines and Applications

Once `Start` invokes `run()`, the Evo Application executes its domain logic
directly with its own dependencies, libraries, and engines:

```text
Evo Application
  ├── Parsers / Lexers
  ├── Evo-Script Engine (EvoS)
  ├── Query Engine (EvoQ)
  └── External Providers / Libraries
```

Evo Runtime does not act as an intermediary, service locator, or message bus for
these internal interactions.

---

## 9. Future Compiled Engine Extension

Future extension architectures may support installing and loading compiled
engines dynamically without recompiling the product.

This future capability:
- Remains completely outside the scope of Model A.
- Does not introduce engine registries or dynamic loaders into `evo-runtime`.
- Will be defined in a separate technical extension specification.

---

## 10. Technical Mapping

| Concept | Architectural Role | Technical Definition File | Technical Type |
| --- | --- | --- | --- |
| **Start** | Use Case | `definitions/use_cases/start.rs` | `pub type Start = fn(run_request::Request) -> Result;` |
| **Run** | Requester | `definitions/requesters/run_request.rs` | `pub type Request = fn() -> Result;` |
| **Starter** | Agent (future) | `agents/starter/start.rs` | `pub fn start(run: run_request::Request) -> Result` |
| **Result** | Outcome Type | `evo-values` | Outcome type from `evo-values` |

---

## 11. Closed Invariants

1. `Start != Run`
2. `Result != Failure`
3. `Start(run)` receives the function pointer `run`, not the evaluated result.
4. Evo Runtime provides exactly 1 Use Case (`Start`) and consumes exactly 1
   Requester (`Run`).
5. Evo Runtime has no Context, no Execution entity, and no Providers in Model A.
