# Evo-Script Engine — Technical Documentation

Status: D2 SEQUENCE DIAGRAMS — IN PROGRESS

Este directorio contiene la documentación técnica de `evo-script-engine`.

La fase funcional está cerrada y revalidada bajo `evo-script/EFN_HOST_BOUNDARY_v0.1.md`. Todo diseño técnico deriva de ese modelo sin redefinir retrospectivamente su semántica.

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
D2 Sequence Diagrams                      ← IN PROGRESS
Implementation Tasks                      PENDING
```

## Technical Views

| View | Propósito |
| --- | --- |
| Technical Data Diagram | Representar structs, enums, artifacts, borrowed views y relaciones. |
| Module Signature Diagram | Representar módulos Rust como identidades arquitectónicas, firmas y relaciones. |
| D2 Sequence Diagram | Representar colaboración dinámica entre firmas. |

## Traceability Rules

1. Todo dato usado por una Rust Signature existe previamente en Technical Data Model.
2. Toda identidad de Module Signature Diagram corresponde a un módulo Rust real previsto.
3. Toda interacción de D2 Sequence Diagram debe corresponder a una firma/call explícita cerrada.
4. Los diagramas derivan del diseño; no inventan Participants, helpers o dependencias.
5. Si firma, módulo y diagrama divergen, el diseño no está cerrado.

## Closed technical foundation

La etapa actual no reabre:

- Stack VM y VM Execution Data;
- Technical Data Model de 140 identities;
- Technical Data Diagram suite de 9 vistas;
- Use Case signatures públicas;
- 6 Collaborators únicos;
- 1 Resolver único;
- 8 Tools únicas;
- 21 módulos conductuales previstos;
- `ExternalCapability` como frontera runtime function-pointer sin Contract duplicado;
- ausencia de Requesters en `evo-script-engine` v0.

## Current work

El trabajo actual consiste exclusivamente en derivar D2 Sequence Diagrams desde las firmas y módulos ya cerrados.

Después de cerrar las secuencias, el siguiente paso será producir `Implementation Tasks` para AGY/Codex.
