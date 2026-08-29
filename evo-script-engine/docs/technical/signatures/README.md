# Evo-Script Engine — Rust Signatures

Status: TECHNICAL DESIGN — NOT STARTED

Este directorio contendrá las **Rust Signatures** arquitectónicas de `evo-script-engine`.

Las firmas son function pointers y constituyen contratos arquitectónicos verificables por el compilador.

Aquí se cerrarán, para cada operación:

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
