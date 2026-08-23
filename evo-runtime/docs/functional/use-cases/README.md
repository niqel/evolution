# Evo Runtime Model A — Functional Use Cases

Status: FUNCTIONAL

This directory contains the canonical functional Use Cases derived for Evo
Runtime Model A from User Stories US-001 through US-014 and the consolidated
[DATA_DICTIONARY.md](../DATA_DICTIONARY.md).

## Nature of Use Cases

Use Cases represent discrete functional actions performed by or coordinated
through Evo Runtime during an Execution.

They follow the **action-first** principle (starting with an active verb) and
describe system behavior from a functional perspective.

Use Cases do not have a mandatory 1:1 relationship with User Stories:
- A User Story expresses a functional requirement or capability need.
- A Use Case specifies an observable functional action taken by the system.
- Cross-cutting definitions (such as US-011 for static Core composition and
  US-013 for shared Value semantics) do not constitute standalone runtime
  actions; instead, they participate across multiple operational Use Cases.

Use Cases in this phase:
- do not represent Rust function signatures or APIs;
- do not define Rust structs, enums, or traits;
- do not represent Sequence Diagrams or internal module calls;
- do not constitute Technical Mapping.

## Catalog of Functional Use Cases

| ID | Action | Description |
| --- | --- | --- |
| [UC-001](UC-001-start-evo-application.md) | Start Evo Application | Inicia una Execution a partir del Entry Point declarado a solicitud de un Host. |
| [UC-002](UC-002-resolve-required-operation.md) | Resolve Required Operation | Resuelve una Required Operation hacia exactamente una Implementation válida. |
| [UC-003](UC-003-make-implementation-available.md) | Make Implementation Available | Asegura que una Implementation resuelta esté disponible bajo demanda antes de Invocation. |
| [UC-004](UC-004-invoke-operation.md) | Invoke Operation | Coordina la participación efectiva de una Operation mediante su Implementation disponible. |
| [UC-005](UC-005-transport-value.md) | Transport Value | Transporta un Value a través de fronteras funcionales preservando su significado. |
| [UC-006](UC-006-determine-engine-for-implementation.md) | Determine Engine for Implementation | Determina el Engine compatible requerido por una Implementation cuando ésta lo necesita. |
| [UC-007](UC-007-propagate-failure.md) | Propagate Failure | Preserva y propaga un Failure evitando que el trabajo dependiente continúe como exitoso. |
| [UC-008](UC-008-maintain-execution-context.md) | Maintain Execution Context | Mantiene la continuidad funcional que asocia el trabajo transitivo con una misma Execution. |
| [UC-009](UC-009-finalize-execution.md) | Finalize Execution | Reconoce la conclusión del trabajo y produce el Result final hacia el Host. |
| [UC-010](UC-010-provide-capability.md) | Provide Capability | Permite a un Provider poner una Capability a disposición de Evo Runtime. |
| [UC-011](UC-011-execute-evo-script-implementation.md) | Execute Evo-Script Implementation | Ejecuta una Implementation Evo-Script mediante el engine EvoS. |
| [UC-012](UC-012-execute-query-work.md) | Execute Query Work | Ejecuta trabajo funcional de consulta sobre Values mediante el engine EvoQ. |

## Traceability Matrix (User Stories → Use Cases)

| User Story | Title | Primary Use Case Coverage | Notes |
| --- | --- | --- | --- |
| US-001 | Start an Application | UC-001 | Direct primary action |
| US-002 | Resolve a Required Operation | UC-002 | Direct primary action |
| US-003 | Load a Required Implementation | UC-003 | Named *Make Implementation Available* to avoid prescribing physical loading |
| US-004 | Invoke an Operation | UC-004 | Direct primary action |
| US-005 | Transport Values between Operations | UC-005 | Direct primary action |
| US-006 | Select an Engine for an Implementation | UC-006 | Named *Determine Engine* to avoid arbitrary selection semantics |
| US-007 | Propagate an Execution Failure | UC-007 | Direct primary action |
| US-008 | Maintain an Execution Context | UC-008 | Direct primary action |
| US-009 | Finalize an Execution | UC-009 | Direct primary action |
| US-010 | Provide a Capability | UC-010 | Direct primary action at Provider boundary |
| US-011 | Provide the Evo Base Core | Cross-cutting Core definition | No standalone Use Case; supports UC-006, UC-010, UC-011, UC-012 |
| US-012 | Execute an Evo-Script Implementation | UC-011 | Direct primary action |
| US-013 | Share Values across Core Components | Cross-cutting shared Value semantics | No standalone Use Case; supports UC-005, UC-011, UC-012 |
| US-014 | Execute Query Work with EvoQ | UC-012 | Direct primary action |

### Cross-cutting Stories Justification

- **US-011 (Provide the Evo Base Core)**: US-011 defines the static structural
  composition of the Core (`Evo Runtime`, `EvoV`, `EvoQ`, `EvoS`) and establishes
  that these base components do not require dynamic discovery. This is an
  architectural property of the execution environment, not a runtime operational
  action performed independently by the Runtime.
- **US-013 (Share Values across Core Components)**: US-013 establishes that
  Values share a common semantic base provided by EvoV across Evo Runtime, EvoQ,
  and EvoS. This semantic consistency governs the data flow in UC-005 (Transport
  Value), UC-011 (Execute Evo-Script Implementation), and UC-012 (Execute Query
  Work), rather than constituting an independent action.
