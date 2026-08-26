# Evo-Script Engine — User Stories

Status: FUNCTIONAL CLOSED

This directory contains the canonical functional User Stories for
`evo-script-engine` v0.

In version v0, the public functional responsibilities of the Evo-Script Engine
are completely defined by **exactly three User Stories**:

1. **US-001 (Compile)**: Compiling complete Evo-Script source text into a
   Compiled Program.
2. **US-002 (Execute Compiled)**: Executing a Compiled Program with Invocation
   Values to produce a Result.
3. **US-003 (Execute Source)**: Executing complete Evo-Script source text
   directly with Invocation Values to produce a Result.

There are no additional functional User Stories in v0.

---

## Conceptual Execution Relationship

The functional relationships across compilation and execution operations are
structured as follows:

```text
                     Complete Source Text
                     /                  \
                    /                    \
                   ▼                      ▼
             Compile (US-001)       Execute Source (US-003)
                   │                (+ Invocation Values)
                   ▼                      │
            Compiled Program              ▼
                   │                    Result
                   │ + Invocation Values
                   ▼
         Execute Compiled (US-002)
                   │
                   ▼
                 Result
```

### Key Functional Distinctions
- **Compile and Execute Source are distinct operations**: `Compile` produces a
  `Compiled Program` without executing; `Execute Source` executes source text
  directly and returns a `Result`.
- **Compile does not automatically execute**: Compilation outputs a compiled
  unit for later execution.
- **Execute Source does not require a prior Compile call**: The Consumer passes
  source text directly without managing intermediate compilation artifacts.
- **Execute Compiled operates on a Compiled Program**: It evaluates an
  already-compiled program unit and does not accept raw source text.
- **Execute Source operates on Source Text**: It evaluates source text directly
  and does not accept a Compiled Program.
- **Retention and Re-execution**: A `Compiled Program` produced by `Compile` can
  be retained externally by the Consumer and executed multiple times through
  `Execute Compiled`.

---

## Catalog

| ID | Title | Status |
| --- | --- | --- |
| [US-001](US-001-compile-evo-script-source.md) | Compile Evo-Script Source | FUNCTIONAL CLOSED |
| [US-002](US-002-execute-compiled-evo-script-program.md) | Execute Compiled Evo-Script Program | FUNCTIONAL CLOSED |
| [US-003](US-003-execute-evo-script-source.md) | Execute Evo-Script Source | FUNCTIONAL CLOSED |
