# Evo-Script Engine — Rust Signatures

Status: RUST SIGNATURES / PARTICIPANT DESIGN — CLOSED

Este directorio contiene las **Rust Signatures** arquitectónicas cerradas de `evo-script-engine` v0.

Las firmas son function pointers y constituyen contratos arquitectónicos verificables por el compilador.

## Closure

```text
Root Use Case signatures              ✅ CLOSED — RSD-001..RSD-010
Compile Participant Design            ✅ CLOSED — RSD-011..RSD-020
Execution Participant Design          ✅ CLOSED — RSD-021..RSD-040

Use Cases                              3
Agents                                 3
Collaborators                          6 unique
Resolvers                              1 unique
Requesters                             0
Additional Contract types              0
Tools                                  8 unique
```

La frontera runtime `ExternalCapability` permanece como function-pointer técnico ya cerrado y no se duplica con otro tipo `Contract`.

## Canonical documents

```text
ROOT_SIGNATURE_DESIGN.md
COMPILE_PARTICIPANT_DESIGN.md
EXECUTION_PARTICIPANT_DESIGN.md
EXECUTION_INITIALIZATION_DESIGN.md
INSTRUCTION_EXECUTION_DESIGN.md
EXTERNAL_CALL_RESOLUTION_DESIGN.md
EXECUTE_SOURCE_PARTICIPANT_DESIGN.md
```

Regla:

> Ningún módulo o diagrama posterior puede introducir una firma, Participant o dependencia conductual que no pueda rastrearse hacia este cierre sin reabrir explícitamente el diseño.

## Next architectural stage

```text
Module Signature Diagrams
    ↓
D2 Sequence Diagrams
    ↓
Implementation Tasks
```
