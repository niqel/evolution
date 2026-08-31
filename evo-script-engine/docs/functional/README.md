# Evo-Script Engine — Functional Documentation

Status: FUNCTIONAL CLOSED — REVALIDATED

Este directorio contiene la documentación funcional de `evo-script-engine`.

La documentación se cierra por niveles; cada nivel cerrado se convierte en autoridad para el siguiente. Los nombres estructurales y conceptos técnicos canónicos se expresan en English; las explicaciones, decisiones e invariantes se redactan en español.

La frontera `.efn` / Host vigente está definida normativamente por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.

## Canonical Design Sequence

```text
Purpose
   ↓
Public Capabilities
   ↓
User Stories
   ↓
Functional Data Dictionary
   ↓
Functional Use Cases
   ↓
FUNCTIONAL CLOSED
   ↓
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

| Step | Artifact | Status |
| --- | --- | --- |
| 1 | [Purpose](PURPOSE.md) | REVALIDATED — FUNCTIONAL CLOSED |
| 2 | [Public Capabilities](CAPABILITIES.md) | REVALIDATED — FUNCTIONAL CLOSED |
| 3 | [User Stories](user-stories/README.md) | REVALIDATED — FUNCTIONAL CLOSED |
| 4 | [Functional Data Dictionary](DATA_DICTIONARY.md) | REVALIDATED — FUNCTIONAL CLOSED |
| 5 | [Functional Use Cases](use-cases/README.md) | REVALIDATED — FUNCTIONAL CLOSED |
| 6 | Technical Design | CLOSED / REVALIDATED |
| 7 | Technical Data Model | CLOSED |
| 8 | Technical Data Diagram | CLOSED — 9 D2 views |
| 9 | Rust Signatures raíz | CLOSED — RSD-001..RSD-010 |
| 10 | Compile Participant Design | CLOSED — RSD-011..RSD-020 |
| 11 | Execution Participant Design | IN PROGRESS / NEXT |
| 12–14 | Module Signature Diagram, D2 Sequence Diagrams, Implementation Tasks | PENDING |

## Closed Public Functional Set

```text
Compile
Execute Compiled
Execute Source
```

Relación central:

```text
Execute Source
    ≡
Compile + Execute Compiled
```

bajo las mismas entradas y External Capabilities explícitamente disponibles.

## `.efn` / Host Boundary

Las tres Public Capabilities operan sobre una unidad `.efn` reusable y Consumer-neutral.

```text
Host / Consumer
    │ explicit inputs + capability composition
    ▼
evo-script-engine
    │
    ▼
Result
```

Invariantes funcionales revalidados:

- `.efn` no posee `Active Scope`;
- `.efn` no hereda prompt, Scope o Session State del Consumer;
- `use` no forma parte de la gramática `.efn` vigente;
- Pipeline representa data composition;
- External Symbols se satisfacen mediante bindings explícitos;
- CLI, UI, API u otro Consumer deciden externamente cómo utilizar/presentar Result.

`Scope` sigue siendo un concepto válido del ecosistema para Hosts interactivos, especialmente Evo-Shell/Evo-CLI; simplemente no forma parte del estado funcional del `.efn`.

## Functional Data Dictionary Rule

> Todo dato o concepto necesario para expresar una User Story o Functional Use Case debe estar definido previamente en el Functional Data Dictionary.

## Technical Data Model Rule

> Toda estructura, enum, artifact o dato interno necesario para expresar una Rust Signature o implementar un Participant debe estar definido previamente en el Technical Data Model.

## Directory Organization

```text
evo-script-engine/
└── docs/
    ├── functional/
    │   ├── PURPOSE.md
    │   ├── CAPABILITIES.md
    │   ├── user-stories/
    │   ├── DATA_DICTIONARY.md
    │   └── use-cases/
    └── technical/
        ├── TECHNICAL_DESIGN.md
        ├── data-model/
        ├── data-diagram/
        ├── signatures/
        ├── module-signatures/
        └── sequences/
```

La fase funcional permanece cerrada. El trabajo actual pertenece al rol de Líder Técnico y continúa en Execution Participant Design; no puede reintroducir retrospectivamente Scope/Host state dentro de `.efn` sin reabrir explícitamente la decisión normativa correspondiente.
