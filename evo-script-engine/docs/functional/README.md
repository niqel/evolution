# Evo-Script Engine — Functional Documentation

Status: FUNCTIONAL CLOSED

Este directorio contiene la documentación de diseño funcional para el componente
`evo-script-engine`.

La documentación se desarrolla y se cierra por niveles. Cada nivel cerrado se
convierte en autoridad para el nivel siguiente.

Los nombres estructurales de artefactos y etapas se expresan en inglés. El
contenido explicativo y normativo de los documentos se redacta en español.

## Canonical Design Sequence

El diseño de `evo-script-engine` sigue esta secuencia canónica:

1. **Purpose** — definición definitiva de la responsabilidad del componente.
2. **Public Capabilities** — capacidades públicas que ofrece el componente.
3. **User Stories** — objetivos funcionales desde la perspectiva de los Consumers.
4. **Functional Data Dictionary** — vocabulario funcional canónico y datos necesarios para derivar posteriormente las firmas.
5. **Functional Use Cases** — acciones funcionales discretas derivadas de las User Stories.

```text
──────────── FUNCTIONAL CLOSED ────────────
```

6. **Technical Design** — arquitectura técnica interna necesaria para cumplir el modelo funcional.
7. **Technical Data Model** — representación concreta de structs, enums, artifacts, borrowed views, ownership, lifetimes y datos internos necesarios.
8. **Technical Data Diagram** — vista D2 de tipos y relaciones del Technical Data Model.
9. **Rust Signatures** — function pointers, tipos, ownership y lifetimes concretos de las operaciones arquitectónicas.
10. **Participants** — Agents, Requesters, Collaborators, Resolvers, Contracts y Tools requeridos por las firmas.
11. **Module Signature Diagram** — vista D2 de módulos Rust como identidades arquitectónicas, sus firmas y relaciones.
12. **D2 Sequence Diagrams** — flujos técnicos derivados directamente de las firmas cerradas.
13. **Implementation Tasks** — lista de trabajo ejecutable para AGY/Codex.

La metodología técnica completa se define en [`TECHNICAL_DESIGN_METHODOLOGY.md`](../../../TECHNICAL_DESIGN_METHODOLOGY.md).

## Current Progress

| Step | Artifact | Status |
| --- | --- | --- |
| 1 | [Purpose](PURPOSE.md) | FUNCTIONAL CLOSED |
| 2 | [Public Capabilities](CAPABILITIES.md) | FUNCTIONAL CLOSED |
| 3 | [User Stories](user-stories/README.md) | REVALIDATED — FUNCTIONAL CLOSED |
| 4 | [Functional Data Dictionary](DATA_DICTIONARY.md) | REVALIDATED — FUNCTIONAL CLOSED |
| 5 | [Functional Use Cases](use-cases/README.md) | FUNCTIONAL CLOSED |
| 6 | Technical Design | NEXT |
| 7–13 | Remaining Technical Artifacts | PENDING |

Todos los artefactos funcionales de `evo-script-engine` v0 están cerrados y constituyen autoridad para el diseño técnico.

## Functional Data Dictionary Rule

Todo dato o concepto necesario para expresar una User Story o Functional Use Case debe estar definido previamente en el Functional Data Dictionary.

El diccionario funcional debe permitir que posteriormente el Technical Lead determine representaciones técnicas y Rust Signatures sin reinventar el significado de los datos. No define todavía structs, enums, ownership, lifetimes ni participantes técnicos.

## Technical Data Model Rule

Toda estructura, enum, artifact o dato interno necesario para expresar una Rust Signature o implementar un Participant debe estar definido previamente en el Technical Data Model.

Los diagramas técnicos no sustituyen esta definición: son vistas derivadas del diseño cerrado.

## Closed Functional Model

```text
Purpose
   ✅
Public Capabilities
   ✅
User Stories
   ✅
Functional Data Dictionary
   ✅
Functional Use Cases
   ✅

──────────── FUNCTIONAL CLOSED ────────────

Technical Design
   ← NEXT
```

Los tres Functional Use Cases canónicos son:

```text
Compile
Execute Compiled
Execute Source
```

con la relación:

```text
Execute Source
    ≡
Compile + Execute Compiled
```

bajo las mismas entradas y capacidades externas disponibles.

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
        ├── data-model/
        ├── signatures/
        ├── module-signatures/
        └── sequences/
```

El siguiente nivel de trabajo es **Technical Design**. A partir de este punto, el diseño técnico debe derivarse del modelo funcional cerrado y no redefinir retrospectivamente su semántica.