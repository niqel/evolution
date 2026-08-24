# Evo Runtime — Runtime Type Classification

Status: TECHNICAL DESIGN NOTE — NOT CLOSED

This document classifies which concepts in the Evo Runtime coordination domain
actually require a concrete, standalone technical data type in `evo-runtime`
before proceeding to the Type Relationship Diagram.

This analysis builds directly upon:
- [DATA_DICTIONARY.md](../../functional/DATA_DICTIONARY.md)
- [COMPONENT_OWNERSHIP_MAP.md](COMPONENT_OWNERSHIP_MAP.md)

---

## 1. Runtime Domain Classification

Following the component ownership decision to locate common values and outcome
types (`Value`, `Result`, `Failure`) in `evo-values`, exactly nine concepts
remain in the Evo Runtime coordination domain.

The table below classifies these nine concepts:

| Canonical Term | Technical Nature | Standalone Data Type? | Technical Name | Representation Family | Notes |
| --- | --- | --- | --- | --- | --- |
| **Entry Point** | Behavioral / Callable Concept | **NO** | N/A | N/A | Application execution entry point; exact callable representation DEFERRED. |
| **Execution** | Process / Lifecycle Concept | **NO** | N/A | N/A | Coordinated lifecycle from Entry Point to conclusion; data continuity belongs to `Context`. |
| **Execution Context** | Data Type | **YES** | `Context` | `struct` | Tracks operational continuity across activities; `Execution Context != Scope`; fields DEFERRED. |
| **Invocation** | Functional Action | **NO** | N/A | N/A | Action of executing an operation via an available implementation; behavioral execution flow. |
| **Operation** | Behavioral / Callable Concept | **NO** | N/A | N/A | Functional unit of work; exact callable representation DEFERRED. |
| **Required Operation** | Callable Requirement Concept | **NO** | N/A | N/A | Requirement resolved to an implementation; exact representation DEFERRED. |
| **Implementation** | Concrete Callable / Behavioral Concept | **NO** | N/A | N/A | Concrete execution realization; exact representation DEFERRED. |
| **Provider** | Architectural / Behavioral Role | **NO** | N/A | N/A | Component supplying capabilities; exact provider representation DEFERRED. |
| **Capability** | Functional Boundary / Callable Concept | **NO** | N/A | N/A | Discrete capability provided by a Provider; exact representation DEFERRED. |

> [!NOTE]
> Among all nine concepts in the Evo Runtime coordination domain, **only
> `Execution Context`** requires a standalone technical data structure in
> `evo-runtime`, with technical name `Context` and representation family
> `struct`. The remaining concepts represent behavioral, callable, process, or
> boundary concepts whose technical representations remain deferred to
> subsequent mapping phases.

---

## 2. Shared EvoV Types

Common data and outcome representations are owned by `evo-values` to enable
shared semantic interoperability across the Core:

| Term | Component Owner | Technical Nature | Representation Family | Notes |
| --- | --- | --- | --- | --- |
| **Value** | `evo-values` | Data Type | Open / Component-defined | Shared valid data transported across the Core; `Value != Failure`. |
| **Result** | `evo-values` | Outcome Sum Type | `enum` | Sum type representing operation or execution outcomes; exact variant names DEFERRED. |
| **Failure** | `evo-values` | Data Type | `struct` | Outcome data type indicating uncompleted or failed work; fields DEFERRED; `Failure != Value`. |

### Conceptual Structure of Result

Conceptually, `Result` expresses two distinct outcome branches:

```text
Result
  ├── success branch ──► Value
  └── failure branch ──► Failure
```

- `Result != Value`
- `Result != Failure`
- `Value != Failure`
- Specific Rust variant names (such as `Ok`, `Err`, `Success`, `Failure`) are
  **DEFERRED** and not frozen here.

---

## 3. Related EvoV Concept — Option

Option expresses the presence or absence of a Value in successful work:

```text
Option (conceptual)
  ├── Some(Value)
  └── None
```

> [!IMPORTANT]
> **None != Failure**
>
> A successful operation that produces no value represents **successful
> absence**, which is fundamentally distinct from an **execution failure**:
>
> - **Successful Result with Value**: `Result::success -> Option::Some(Value)`
> - **Successful Result with no Value**: `Result::success -> Option::None`
> - **Failed Result**: `Result::failure -> Failure`

*Option is **not** one of the 23 canonical terms of Evo Runtime Model A.*
Its internal design and technical type definition belong to the future separate
specification of `evo-values`.

---

## 4. Component Dependency Rationale

Placing `Value`, `Result`, and `Failure` in `evo-values` establishes a clean,
non-circular dependency architecture for Model A:

```text
                     evo-values
              (Value, Result, Failure)
                         │
         ┌───────────────┼───────────────┐
         │               │               │
         ▼               ▼               ▼
    evo-runtime         EvoQ            EvoS
    (Context)      (Query Work)  (Evo-Script Impl)
```

This ensures:
1. `evo-query-engine` and `evo-script-engine` can consume and produce `Value`,
   `Result`, and `Failure` without depending on `evo-runtime`.
2. `evo-runtime` coordinates executions and context continuity without owning
   the core data representations.
3. EvoV remains the common semantic data foundation and is not an engine.

---

## 5. What This Classification Does Not Decide

This document deliberately does **not** decide:

1. Concrete fields of `Context`.
2. Concrete fields of `Failure`.
3. Variant names or syntax for `Result`.
4. Technical type representation for `Option`.
5. Memory ownership models (`owned` vs `borrowed`).
6. Reference lifetimes (`'a`) or smart pointers (`Arc`, `Rc`, `Box`).
7. Function signatures (`pub fn`).
8. Concrete function pointer types or traits.
9. Definitions of Contracts, Requesters, Agents, or concrete Providers.
10. Concrete mapping between `Required Operation` and `Capability`.
11. Sequence diagrams or technical interaction protocols.
12. Cargo dependency manifests (`Cargo.toml`).
13. Internal memory layout or value variants in `evo-values`.
14. Internal query planner, optimizer, or operators in `evo-query-engine`.
15. Internal AST, parser, bytecode, or VM in `evo-script-engine`.
