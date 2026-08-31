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
Root Use Case signatures              ✅ CLOSED
RSD-001..RSD-010                      ✅ CLOSED
Compile Participant Design            ← IN PROGRESS
├── lex_source                        ✅ CLOSED — RSD-011..RSD-013
├── parse_tokens                      ← NEXT
├── analyze_program                   PENDING
└── lower_program                     PENDING
Execution Participant Design          PENDING
Module Signature Diagrams             PENDING
D2 Sequence Diagrams                  PENDING
Implementation Tasks                  PENDING
```

Documentos canónicos actuales:

```text
ROOT_SIGNATURE_DESIGN.md
COMPILE_PARTICIPANT_DESIGN.md
```
