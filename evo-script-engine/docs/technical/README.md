# Evo-Script Engine — Technical Documentation

Status: TECHNICAL DESIGN PACKAGE — CLOSED / PROGRAMMING READY

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
Technical Data Model                      ✅ CLOSED
Technical Data Diagram                    ✅ CLOSED — 9 D2 views
Root Rust Signatures                      ✅ CLOSED — RSD-001..RSD-010
Compile Participant Design                ✅ CLOSED — RSD-011..RSD-020
Execution Participant Design              ✅ CLOSED — RSD-021..RSD-040
Rust Signatures / Participant Design      ✅ CLOSED
Module Signature Design                   ✅ CLOSED — MSD-001..MSD-010
Module Signature Diagram                  ✅ CLOSED — 4 D2 views
D2 Sequence Diagrams                      ✅ CLOSED — 4 D2 views
Implementation Tasks                      ✅ CLOSED — 54 tasks
Programming                               ← READY
```

## Technical artifact structure

```text
technical/
├── README.md
├── TECHNICAL_DESIGN.md
├── data-model/
│   └── closed Technical Data Model
├── data-diagram/
│   └── 9 Technical Data Diagram D2 views
├── signatures/
│   └── RSD-001..RSD-040 + participant design
├── module-signatures/
│   ├── MODULE_SIGNATURE_DESIGN.md
│   └── 4 Module Signature D2 views
├── sequences/
│   └── 4 D2 Sequence views
└── implementation-tasks/
    └── README.md — 54 task programming backlog
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

## Programming prerequisite detected

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
Programming Backlog                    ✅ CLOSED

NEXT ROLE
    Programmer

PROGRAMMERS
    AGY / Codex
```
