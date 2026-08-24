# Model A — Functional Coverage

Status: FUNCTIONAL CLOSED

## Purpose

This document records the completed functional coverage of Evo Runtime Model A.

In Model A, Evo Runtime has a single, minimal responsibility: starting an Evo
Application by invoking its provided Run action and returning the final Result.

## Functional Scope and Traceability

The functional coverage of Model A is fully covered by exactly one User Story
and one Use Case:

| User Story | Functional Responsibility | Primary Use Case |
| --- | --- | --- |
| [US-001](user-stories/US-001-start-application.md) | Start an Evo Application | [UC-001](use-cases/UC-001-start-evo-application.md) |

## Functional Execution Path

The canonical execution path of Model A is:

```text
Host
  │
  │ calls Start(Run)
  ▼
Evo Runtime
  │
  │ invokes Run()
  ▼
Evo Application (Run active)
  │
  │ completes with Result
  ▼
Evo Runtime
  │
  │ returns Result
  ▼
Host
```

## Independence of Multiple Start Invocations

Evo Runtime supports multiple independent Start invocations:

```text
Host / Caller
  ├── Start(Run_A) ──► Application A (active) ──► Result A
  ├── Start(Run_B) ──► Application B (active) ──► Result B
  └── Start(Run_C) ──► Application C (active) ──► Result C
```

- Each Start invocation operates independently.
- Failure of Application A does not cause failure of Application B or C.
- There is no shared Context or Execution tracking entity in Evo Runtime.

## Non-Responsibilities of Evo Runtime Model A

All internal operations, engine executions, and provider integrations occur
outside of Evo Runtime:

- Evo Runtime does **not** resolve operations or dependencies.
- Evo Runtime does **not** select or load engines (such as EvoS or EvoQ).
- Evo Runtime does **not** manage providers, capabilities, or contracts.
- Evo Runtime does **not** parse commands or execute Evo-Script files.
- Evo Runtime does **not** maintain an internal Context or Execution entity.
- Evo Runtime does **not** transport Values across internal application boundaries.

Once Start invokes Run, the application interacts directly with its own
libraries, engines, and providers.
