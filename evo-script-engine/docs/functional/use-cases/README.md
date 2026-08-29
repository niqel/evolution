# Evo-Script Engine — Functional Use Cases

Status: FUNCTIONAL DESIGN — IN PROGRESS

Este directorio contiene los Functional Use Cases de `evo-script-engine` v0.

Los Use Cases se derivan formalmente de las User Stories y del Functional Data Dictionary ya cerrados. A partir de esta etapa, cada Use Case fija un nombre semántico canónico que debe conservarse al derivar el diseño técnico y las Rust Signatures.

## Naming Rule

```text
Public Capability
      ↓
Functional Use Case
      ↓
Technical Use Case
      ↓
Rust Signature / file
```

El nombre arquitectónico de la acción no debe cambiar entre estos niveles salvo que una decisión sea reabierta explícitamente.

Ejemplo:

```text
Compile
   ↓
UC-001 — Compile
   ↓
definitions/use_cases/compile.rs
```

La existencia futura de un Agent, Collaborator u otro Participant no se decide en esta etapa.

## Use Case Catalog

| ID | Canonical Name | Estado |
| --- | --- | --- |
| [UC-001](UC-001-compile.md) | Compile | REVALIDATED — FUNCTIONAL CLOSED |
| [UC-002](UC-002-execute-compiled-evo-script-program.md) | Execute Compiled | EXISTING — TO BE REVALIDATED |
| UC-003 | Execute Source | PENDING / TO BE DEFINED OR REVALIDATED |

## Current Progress

```text
UC-001 — Compile             ✅ REVALIDATED / CLOSED
UC-002 — Execute Compiled    ← NEXT
UC-003 — Execute Source      PENDING
```

Los nombres canónicos de v0 deben corresponder a las Public Capabilities ya cerradas:

```text
Compile
Execute Compiled
Execute Source
```

Los títulos descriptivos anteriores como `Compile Evo-Script Source` o `Execute Compiled Evo-Script Program` pueden conservarse únicamente como explicación histórica, pero no como nombres arquitectónicos de los Use Cases.
