# Evo-Script Engine — User Stories

Status: FUNCTIONAL CLOSED

This directory contains the canonical functional User Stories for
`evo-script-engine` v0.

In version v0, the public functional responsibilities of the Evo-Script Engine
are completely defined by **exactly two User Stories**:

1. **US-001 (Compile)**: Compiling complete Evo-Script source text into a
   Compiled Program.
2. **US-002 (Execute)**: Executing a Compiled Program with Invocation Values to
   produce a Result.

There are no additional functional User Stories in v0.

---

## Conceptual Execution Relationship

The functional separation between compilation and execution is structured as
follows:

```text
Source Text
    │
    ▼
  Compile (US-001)
    │
    ▼
Compiled Program
    │
    │ + Invocation Values
    ▼
  Execute (US-002)
    │
    ▼
   Result
```

> [!NOTE]
> `Compile` and `Execute` are distinct functional operations:
> - `Compile` does not automatically trigger execution.
> - `Execute` does not perform source compilation.
> - A `Compiled Program` produced by `Compile` can be retained and executed
>   subsequently by `Execute`.

---

## Catalog

| ID | Title | Status |
| --- | --- | --- |
| [US-001](US-001-compile-evo-script-source.md) | Compile Evo-Script Source | FUNCTIONAL CLOSED |
| [US-002](US-002-execute-compiled-evo-script-program.md) | Execute Compiled Evo-Script Program | FUNCTIONAL CLOSED |
