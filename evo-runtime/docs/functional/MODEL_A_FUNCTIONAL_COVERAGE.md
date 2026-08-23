# Model A — Functional Coverage

Status: FUNCTIONAL CLOSED

## Purpose

This document records the completed functional coverage review of the Evo
Runtime Model A static Core.

The review covers User Stories US-001 through US-014.

No additional functional responsibility has been identified within the static
Core that requires another User Story before proceeding to the Data Dictionary.

This status closes only the functional User Story coverage of Model A.

It does not close the technical architecture, implementation, future extension
models, Provider model, Scope model or Evo composition model.

## Model A Core

The Model A static Core is composed functionally by:

- Evo Runtime, as the platform responsible for coordinating execution;
- EvoV, as the common Values base;
- EvoQ, as the base query engine;
- EvoS, as the base engine for Evo-Script.

Conceptually:

```text
                         Evo Runtime
                              │
                  ┌───────────┴───────────┐
                  ▼                       ▼
                EvoQ                    EvoS
                  │                       │
                  └───────────┬───────────┘
                              ▼
                             EvoV
                     common Values base
```

EvoQ and EvoS are engines of the Core.

EvoV is not an engine.

Evo-Script is the language.

EvoS is the Core engine capable of working with Evo-Script.

## Functional Coverage

| User Story | Functional Responsibility |
| --- | --- |
| US-001 | Start an Evo Application |
| US-002 | Resolve a Required Operation |
| US-003 | Make a required Implementation available on demand |
| US-004 | Invoke an Operation |
| US-005 | Transport Values between Operations |
| US-006 | Determine an Engine for an Implementation |
| US-007 | Propagate an execution Failure |
| US-008 | Maintain an Execution Context |
| US-009 | Finalize an Execution |
| US-010 | Use a Capability provided by a Provider |
| US-011 | Provide the Evo base Core |
| US-012 | Execute an Evo-Script Implementation through EvoS |
| US-013 | Share Values across Core components through EvoV |
| US-014 | Execute query work through EvoQ |

Together, these stories cover the functional execution path of Model A:

```text
Host
  ↓
Evo Runtime
  ↓
Required Operation
  ↓
Implementation
  ↓
Execution
  ├──────────────► EvoS
  │                 ↓
  │             Evo-Script
  │
  └──────────────► EvoQ
                    ↓
                Query Work

EvoV provides the common Values base used across the Core.
```

The reviewed functional coverage includes:

- application start;
- Required Operation resolution;
- on-demand Implementation availability;
- operation invocation;
- Value transport;
- Engine determination;
- Failure propagation;
- Execution Context continuity;
- execution finalization;
- Provider and Capability participation at the functional boundary;
- availability of the static Evo Core;
- EvoS participation for Evo-Script execution;
- EvoQ participation for query work;
- shared Value semantics through EvoV.

No further User Story is required for the functional coverage of the Model A
static Core at this stage.

## Boundaries of This Closure

`FUNCTIONAL CLOSED` does not define or close:

- the Data Dictionary;
- Use Cases;
- Sequence Diagrams;
- Rust architecture;
- Rust implementation;
- function pointers;
- Contracts;
- Requesters;
- Agents;
- concrete Providers;
- Rust structs;
- Rust enums;
- ownership;
- borrowing;
- lifetimes;
- ABI;
- Cargo dependencies;
- crate composition;
- static linking;
- dynamic linking;
- physical loading;
- physical lifecycle management;
- Engine initialization;
- Provider lifecycle;
- Scope;
- Provider discovery;
- dynamic Providers;
- plugins;
- Model B;
- Model C;
- `.main`;
- `.root`;
- `.elib`;
- `.esig`;
- `.emod`;
- concrete Evo application composition;
- internal EvoQ design;
- EvoQ query syntax;
- EvoQ query operators;
- internal EvoS design;
- Evo-Script language semantics.

Those concerns remain for later functional, data, use-case, technical or
component-specific work as appropriate.

## Next Phase

The functional User Story phase for Model A is complete.

The next planned phase is:

```text
User Stories
    ✅ FUNCTIONAL CLOSED

        ↓

Data Dictionary
    NEXT

        ↓

Use Cases

        ↓

Sequence Diagrams

        ↓

Technical Mapping

        ↓

Rust Implementation
```

No US-015 is currently required for Model A.
