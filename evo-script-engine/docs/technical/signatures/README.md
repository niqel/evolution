# Evo-Script Engine — Rust Signatures

Status: RUST SIGNATURES / PARTICIPANT DESIGN — IN PROGRESS

Este directorio contiene las **Rust Signatures** arquitectónicas de `evo-script-engine`.

Las firmas son function pointers y constituyen contratos arquitectónicos verificables por el compilador.

Aquí se cierran, para cada operación:

- tipos concretos de entrada y salida;
- orden de argumentos;
- ownership y borrowing;
- lifetimes;
- Requesters;
- Contracts;
- outcomes y errores semánticos;
- dependencias explícitas necesarias.

Regla:

> Ninguna Rust Signature puede utilizar un dato que no haya sido definido previamente en el Technical Data Model.

Una vez cerradas las firmas se derivan los Participants y posteriormente los Module Signature Diagrams y D2 Sequence Diagrams.

## Current Progress

```text
Root Use Case signatures              ✅ CLOSED — RSD-001..RSD-010
Compile Participant Design            ✅ CLOSED — RSD-011..RSD-020
├── lex_source                        ✅ CLOSED
├── parse_tokens                      ✅ CLOSED
├── analyze_program                   ✅ CLOSED
└── lower_program                     ✅ CLOSED
Execution Participant Design          ← IN PROGRESS — RSD-021..RSD-026
├── initialize_execution              root signature ✅ / internals ← NEXT
├── execute_instruction               root signature ✅ / internals PENDING
└── resolve_external_call             root signature ✅ / internals PENDING
Module Signature Diagrams             PENDING
D2 Sequence Diagrams                  PENDING
Implementation Tasks                  PENDING
```

Documentos canónicos actuales:

```text
ROOT_SIGNATURE_DESIGN.md
COMPILE_PARTICIPANT_DESIGN.md
EXECUTION_PARTICIPANT_DESIGN.md
```
