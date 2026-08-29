# Evo-Script Engine — Functional Use Cases

Status: FUNCTIONAL CLOSED

Este directorio contiene los Functional Use Cases canónicos de `evo-script-engine` v0.

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
| [UC-002](UC-002-execute-compiled.md) | Execute Compiled | REVALIDATED — FUNCTIONAL CLOSED |
| [UC-003](UC-003-execute-source.md) | Execute Source | FUNCTIONAL CLOSED |

## Closed Functional Set

```text
UC-001 — Compile             ✅ FUNCTIONAL CLOSED
UC-002 — Execute Compiled    ✅ FUNCTIONAL CLOSED
UC-003 — Execute Source      ✅ FUNCTIONAL CLOSED
```

Los nombres canónicos de v0 corresponden exactamente a las Public Capabilities cerradas:

```text
Compile
Execute Compiled
Execute Source
```

Los títulos descriptivos anteriores como `Compile Evo-Script Source` o `Execute Compiled Evo-Script Program` no son nombres arquitectónicos canónicos.

## Cross-Use-Case Relationship

```text
Compile
Source Text
    │
    ▼
Compiled Program

Execute Compiled
Compiled Program + Invocation Values
    │
    ▼
Result

Execute Source
Source Text + Invocation Values
    │
    ▼
Result
```

La relación funcional central es:

```text
Execute Source
    ≡
Compile + Execute Compiled
```

bajo el mismo `Source Text`, los mismos `Invocation Values` y las mismas `External Capabilities` disponibles.

## Closure

Con `UC-001`, `UC-002` y `UC-003` cerrados, la etapa **Functional Use Cases** de `evo-script-engine` v0 queda `FUNCTIONAL CLOSED`.

Nuevas capacidades públicas o cambios en estos Use Cases requieren reabrir explícitamente el cierre funcional correspondiente.