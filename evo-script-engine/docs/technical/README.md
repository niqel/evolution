# Evo-Script Engine — Technical Documentation

Status: RUST SIGNATURES / PARTICIPANT DESIGN — IN PROGRESS

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

## Technical Views

| View | Propósito |
| --- | --- |
| Technical Data Diagram | Representar structs, enums, artifacts, borrowed views y relaciones. |
| Module Signature Diagram | Representar módulos Rust como identidades arquitectónicas, firmas y relaciones. |
| D2 Sequence Diagram | Representar colaboración dinámica entre firmas. |

No se utiliza UML Class Diagram como modelo primario. En Evolution el comportamiento pertenece a módulos/functions y los datos a tipos concretos.

## Module Identity Rule

```text
archivo.rs
    = módulo
    = identidad arquitectónica
```

No se crea una `struct` artificial únicamente para imitar una service class.

## `.efn` / Host Technical Boundary

TD-011 cierra:

```text
Interactive Host State
    !=
Reusable `.efn` Execution State
```

Por tanto el diseño técnico de `.efn` no introduce:

```text
Active Scope
Host Session State
Current Provider
Use Node
Use Instruction
SET_SCOPE Opcode
```

`Scope` permanece en Evo-Shell/Host cuando exista una sesión interactiva persistente. External Symbols de `.efn` se satisfacen mediante explicit Application Bindings.

## Directory Organization

```text
technical/
├── README.md
├── TECHNICAL_DESIGN.md
├── data-model/
│   └── Technical Data Model
├── data-diagram/
│   └── Technical Data Diagrams D2
├── signatures/
│   └── Rust Signatures + Participant Design
├── module-signatures/
│   └── Module Signature Diagrams
└── sequences/
    └── D2 Sequence Diagrams
```

## Traceability Rules

1. Todo dato usado por una Rust Signature debe existir previamente en Technical Data Model.
2. Toda identidad de Module Signature Diagram corresponde a un módulo Rust real previsto.
3. Toda interacción de D2 Sequence Diagram corresponde a una firma/call explícita.
4. Los diagramas derivan del diseño cerrado; no inventan diseño.
5. Si firma, módulo y diagrama divergen, el diseño no está cerrado.

## Current Progress

```text
Functional Design                         ✅ CLOSED / REVALIDATED
Technical Design                          ✅ CLOSED / REVALIDATED
Technical Data Model                      ✅ CLOSED
Technical Data Diagram                    ✅ CLOSED — 9 D2 views
Root Rust Signatures                      ✅ CLOSED — RSD-001..RSD-010
Compile Participant Design                ✅ CLOSED — RSD-011..RSD-020
Execution Participant Design              ← NEXT
Module Signature Diagram                  PENDING
D2 Sequence Diagrams                      PENDING
Implementation Tasks                      PENDING
```

Decisiones técnicas estructurales cerradas incluyen:

- Stack VM;
- Semantic Program como identidad propia y única Semantic IR;
- internal Function resolution durante Compile;
- Compiled Program shape y owned Constant Pool;
- Shared Operand Stack + Shared Frame Region;
- external Value ownership policy;
- Evo-Script-driven VM;
- Compilation Working State policy;
- `.efn` / Host State separation;
- Earliest Responsible Failure;
- Compile Agent con cuatro Collaborators internos cerrados.

El trabajo actual continúa en Execution Participant Design. El Technical Data Model y Compile Participant Design permanecen cerrados salvo contradicción técnica demostrable que obligue a reabrirlos explícitamente.
