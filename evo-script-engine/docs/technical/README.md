# Evo-Script Engine — Technical Documentation

```text
Historical Technical Design Package
CLOSED / PRESERVED

evo-values v0.1 Technical Reconciliation
CLOSED

Current Reconciled Implementation
CLOSED
```

Este directorio contiene la documentación técnica canónica de `evo-script-engine` v0.

La fase funcional está cerrada y revalidada bajo `evo-script/EFN_HOST_BOUNDARY_v0.1.md`. El paquete técnico completo deriva de ese modelo sin redefinir retrospectivamente su semántica.

La metodología técnica canónica se encuentra en [`TECHNICAL_DESIGN_METHODOLOGY.md`](../../../TECHNICAL_DESIGN_METHODOLOGY.md) y las decisiones estructurales del componente en [`TECHNICAL_DESIGN.md`](TECHNICAL_DESIGN.md).

## Canonical Technical Sequence

```text
Technical Design
   ↓
Technical Data Model
   ↓
Technical Data Diagram
   ↓
Rust Signatures
   ↓
Participants
   ↓
Module Signature Diagram
   ↓
D2 Sequence Diagrams
   ↓
Implementation Tasks
   ↓
PROGRAMMING
```

## Current Progress

```text
Functional Design                         ✅ CLOSED / REVALIDATED
Technical Design                          ✅ CLOSED / REVALIDATED
Historical Technical Data Model           ✅ CLOSED / PRESERVED
Reconciled Technical Data Model           ✅ CLOSED — 137 technical identities
Historical Technical Data Diagrams        ✅ CLOSED / PRESERVED — 9 canonical D2 views
Historical D2 Sequence Diagrams           ✅ CLOSED / PRESERVED — 4 views
Cross-component Sequence Diagrams         ✅ CLOSED — 5 views
Current Canonical Sequence Suite          ✅ CLOSED — 9 views
Root Rust Signatures                      ✅ CLOSED — RSD-001..RSD-010
Compile Participant Design                ✅ CLOSED — RSD-011..RSD-020
Execution Participant Design              ✅ CLOSED — RSD-021..RSD-040
Rust Signatures / Participant Design      ✅ CLOSED
Module Signature Design                   ✅ CLOSED — MSD-001..MSD-010
Module Signature Diagram                  ✅ CLOSED — 4 D2 views
Historical Implementation Tasks           ✅ CLOSED / PRESERVED — 54 tasks (NOT CURRENT PROGRAMMING AUTHORITY)
Reconciliation Authority                  ✅ CLOSED — EVO_VALUES_V0_1_RECONCILIATION.md
Current Reconciled Implementation         ✅ CLOSED — implementation-tasks/WP-ESE-RECON-001.md
```

## Technical artifact structure

```text
technical/
├── README.md
├── TECHNICAL_DESIGN.md
├── EVO_VALUES_V0_1_RECONCILIATION.md
├── data-model/
│   └── closed Technical Data Model (historical & reconciled)
├── data-diagram/
│   └── 9 canonical Technical Data Diagram D2 views
├── signatures/
│   └── RSD-001..RSD-040 + participant design
├── module-signatures/
│   ├── MODULE_SIGNATURE_DESIGN.md
│   └── 4 Module Signature D2 views
├── sequences/
│   ├── 4 historical orchestration views — PRESERVED
│   └── 5 cross-component reconciliation views
│       total canonical = 9
└── implementation-tasks/
    ├── README.md — 54 task historical programming backlog (CLOSED / PRESERVED)
    └── WP-ESE-RECON-001.md — Reconciliation work package (implementation CLOSED, final quality gate PENDING)
```

## Closed behavioral inventory

```text
Use Cases        3
Agents           3
Collaborators    6 unique
Resolvers        1 unique
Requesters       0
Additional Contracts 0
Tools            8 unique
Conductual modules 21
```

`ExternalCapability` permanece como la frontera runtime function-pointer cerrada; no existe un Contract wrapper duplicado.

## Programming prerequisite detected (HISTORICAL / SUPERSEDED)

> [!NOTE]
> **HISTORICAL / SUPERSEDED**: El texto siguiente describe el diagnóstico histórico previo a la evolución de `evo-values v0.1`. Dicha etapa fue completada. La autoridad vigente de reconciliación es [`EVO_VALUES_V0_1_RECONCILIATION.md`](EVO_VALUES_V0_1_RECONCILIATION.md) y el backlog de programación actual es [`implementation-tasks/WP-ESE-RECON-001.md`](implementation-tasks/WP-ESE-RECON-001.md).

El código actual de `evo-values` todavía implementa el modelo histórico:

```text
Text
Unsigned
Signed
Boolean
```

mientras `INTERCHANGE_MODEL.md` exige el modelo v0 de 17 familias con `Value<'a>` y `OwnedValue`.

Por ello `implementation-tasks/README.md` inicia con cuatro tareas `EVO-V-001..EVO-V-004` que deben cerrarse antes de implementar las fronteras runtime de `evo-script-engine`.

## Programming authority order

AGY/Codex deben resolver ambigüedad consultando, en este orden conceptual:

```text
Functional closed artifacts
    ↓
Technical Design
    ↓
Technical Data Model
    ↓
Rust Signatures / Participants
    ↓
Module Signature Design / Diagrams
    ↓
D2 Sequence Diagrams
    ↓
Implementation Tasks
```

Un artefacto posterior implementa/representa al anterior; no puede redefinirlo silenciosamente.

Si dos artifacts cerrados parecen contradictorios, el programador debe detener esa tarea y reportar la contradicción en lugar de improvisar una nueva arquitectura.

## Final technical closure

```text
Architecture / Functional Analysis     ✅ CLOSED
Technical Lead Design                  ✅ CLOSED
Historical Programming Backlog         ✅ CLOSED / PRESERVED (NOT CURRENT PROGRAMMING AUTHORITY)
Reconciliation Authority              ✅ CLOSED (EVO_VALUES_V0_1_RECONCILIATION.md)
Reconciled Implementation              ✅ CLOSED (WP-ESE-RECON-001.md)
Final Quality Gate                     ← PENDING — TASK-ESE-RECON-015

NEXT ROLE
    Programmer / Quality Gate Reviewer

PROGRAMMERS
    AGY / Codex
```
