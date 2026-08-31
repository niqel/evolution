# Evo-Script Engine — Module Signature Diagrams

Status: MODULE SIGNATURE DIAGRAMS — CLOSED

Este directorio contiene la suite D2 canónica de **Module Signature Diagrams** de `evo-script-engine` v0.

La unidad primaria es el módulo Rust previsto, no una clase ni una instancia.

## Authority

```text
Rust Signatures / Participant Design
    ↓
MODULE_SIGNATURE_DESIGN.md
    ↓
D2 Module Signature Diagrams
```

Los diagramas no crean Participants ni firmas. Representan exclusivamente las identidades y relaciones ya cerradas.

## Canonical suite

```text
00-overview.d2           ✅ BUILT
01-compile.d2            ✅ BUILT
02-execute-compiled.d2   ✅ BUILT
03-execute-source.d2     ✅ BUILT
```

### 00 — Overview

Representa los **21 módulos conductuales previstos**:

```text
Use Case definitions   3
Agents                 3
Collaborators          6
Resolvers              1
Tools                  8
                      ──
TOTAL                  21
```

### 01 — Compile

Representa:

```text
Compile Use Case
    ↓ implemented by
compiler Agent
    ├── coordinates lexer
    ├── coordinates parser
    ├── coordinates semantic_analyzer
    └── coordinates bytecode_compiler
```

No existen Tools, Resolver, Requester ni Contract en Compile.

### 02 — ExecuteCompiled

Representa:

```text
ExecuteCompiled Use Case
    ↓ implemented by
compiled_program_executor Agent
    ├── coordinates execution_initializer
    ├── coordinates instruction_executor
    └── resolves through external_call_resolver
```

También muestra las siete Tools de ejecución y su relación exacta con Collaborators/Resolver.

### 03 — ExecuteSource

Representa composición directa sin Agent→Agent:

```text
source_executor Agent
├── Compile Collaborators ×4
├── contextualize_compile_failure Tool
└── Execution Participants
    ├── execution_initializer
    ├── instruction_executor
    └── external_call_resolver
```

Las relaciones internas de Tools de ejecución permanecen autoritativas en `02-execute-compiled.d2` y no se duplican innecesariamente en esta vista.

## Module Signature invariants

1. Todo módulo mostrado corresponde a una identidad modular prevista por `MODULE_SIGNATURE_DESIGN.md`.
2. Todo Use Case apunta a exactamente un Agent implementation module.
3. Todo Agent coordina únicamente Participants cerrados por Rust Signatures / Participant Design.
4. Ningún Collaborator coordina otro Collaborator.
5. Solo `external_call_resolver` cruza la frontera `ExternalCapability`.
6. Tools únicamente aparecen en relaciones `uses` ya demostradas.
7. No se muestran helpers privados, parser productions, opcode-family handlers o wrappers no arquitectónicos.
8. `ExecuteSource` reutiliza módulos de Compile y Execution; no duplica implementaciones.
9. Los `.d2` son autoridad versionada; SVG/PNG/PDF serían outputs derivados.
10. La suite no reabre el Technical Data Model ni Rust Signatures.

## Validation

Auditoría textual/arquitectónica:

```text
Canonical views present                    4 / 4 ✅
Use Case modules represented               3 / 3 ✅
Agent modules represented                  3 / 3 ✅
Collaborator modules represented           6 / 6 ✅
Resolver modules represented               1 / 1 ✅
Tool modules represented                   8 / 8 ✅
Unknown Participants introduced            0
Agent → Agent relations introduced         0
Collaborator → Collaborator relations       0
Duplicate Contract wrapper introduced       0
```

El entorno utilizado para esta edición no dispone de ejecutable `d2`; por tanto no se declara una validación local de render/parser que no se realizó.

## Closure

```text
MODULE_SIGNATURE_DESIGN.md             ✅ CLOSED — MSD-001..MSD-010
Module Signature Diagram suite          ✅ CLOSED — 4 views
Conductual module identities            ✅ CLOSED — 21

NEXT ARCHITECTURAL STAGE
    D2 Sequence Diagrams
```
