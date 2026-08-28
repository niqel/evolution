# Evo-Script Engine — Functional Documentation

Status: FUNCTIONAL DESIGN — IN PROGRESS

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
4. **Functional Data Dictionary** — vocabulario funcional canónico.
5. **Functional Use Cases** — acciones funcionales discretas derivadas de las User Stories.

```text
──────────── FUNCTIONAL CLOSED ────────────
```

6. **Technical Design** — arquitectura técnica interna necesaria para cumplir el modelo funcional.
7. **Rust Signatures** — function pointers, tipos, ownership y lifetimes concretos.
8. **Participants** — Agents, Requesters, Collaborators, Resolvers, Contracts y Tools requeridos por las firmas.
9. **D2 Sequence Diagrams** — flujos técnicos derivados directamente de las firmas cerradas.
10. **Implementation Tasks** — lista de trabajo ejecutable para AGY/Codex.

## Current Progress

| Step | Artifact | Status |
| --- | --- | --- |
| 1 | [Purpose](PURPOSE.md) | FUNCTIONAL CLOSED |
| 2 | Public Capabilities | NEXT |
| 3 | [User Stories](user-stories/README.md) | EXISTING — TO BE REVALIDATED |
| 4 | [Functional Data Dictionary](DATA_DICTIONARY.md) | EXISTING — TO BE REVALIDATED |
| 5 | [Functional Use Cases](use-cases/README.md) | EXISTING — TO BE REVALIDATED |
| 6–10 | Technical artifacts | PENDING |

Los artefactos funcionales existentes de etapas posteriores se conservan como
evidencia y trabajo previo, pero deben ser revalidados bajo la arquitectura
canónica actual antes de considerarse autoridad para el nuevo cierre funcional.

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
```

`CAPABILITIES.md` corresponde al siguiente paso y se crea cuando Public Capabilities sea discutido y cerrado.
