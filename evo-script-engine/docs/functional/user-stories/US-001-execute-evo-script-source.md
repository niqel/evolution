# US-001 — Execute Evo-Script Source

Status: FUNCTIONAL DRAFT — NOT CLOSED

## Story

```text
As a Consumer,
I want to provide the complete source text of an Evo-Script program to the Evo-Script Engine,
so that the program is processed according to Evo-Script Language Specification v0, its single public function is executed, and I receive its execution outcome.
```

*(Como un Consumer, quiero proporcionar el texto fuente completo de un programa
Evo-Script al Evo-Script Engine, para que el programa sea procesado según la
Evo-Script Language Specification v0, su única función pública sea ejecutada, y
yo reciba el resultado de la ejecución).*

---

## Context

The Evo-Script Engine is the execution component responsible for processing and
evaluating complete Evo-Script source programs.

A program in Evo-Script v0 lives entirely within a single `.efn` source file and
declares exactly one public entry function (`public fn`), alongside optional
private functions, structs, and enums local to the file.

The Consumer provides the **complete source text** of the program directly to
the Engine. The Consumer is not responsible for parsing, lexing, or analyzing the
source before passing it to the Engine, nor does it require visibility into the
Engine's internal processing pipeline.

```text
Consumer
   │
   │ complete Evo-Script source text
   ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  Processes program (v0 specification)  │
│  Executes single public fn             │
└──────────────────┬─────────────────────┘
                   │
                   │ execution outcome
                   ▼
                 Result
```

### Source Text Clarification
- **Source Text != File Path**: The Engine does not perform file I/O or path
  resolution; reading the physical `.efn` file is the responsibility of the
  Consumer or an external loader.
- **Source Text != AST**: The Consumer passes raw text, not a pre-parsed
  abstract syntax tree or token stream.
- **Source Text != Individual Function**: The input is the complete,
  self-contained source text of the program unit.

---

## Acceptance Criteria

1. The Consumer can provide the complete source text of one Evo-Script v0
   program to the Evo-Script Engine.
2. The Engine treats that source text as one complete Evo-Script program.
3. The Engine processes the program according to:
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md).
4. The Engine executes the program's single public function defined by Evo-Script
   v0.
5. The Consumer does not need to parse or pre-process the source text before
   providing it to the Engine.
6. The Consumer does not need to know the Engine's internal processing
   structure.
7. A successful execution preserves the Value produced by the public function
   in the successful execution outcome.
8. A failed execution is returned as a failed execution outcome rather than
   being silently treated as success.
9. Once the execution outcome is returned to the Consumer, that Engine
   invocation is complete.
10. One invocation does not require an explicit Engine session object or
    execution-context object at the functional boundary.

---

## Execution Outcome Concept (Result)

The functional outcome of executing a program is represented conceptually as
`Result`:
- **Successful Outcome**: Preserves the program's produced `Value`.
- **Failed Outcome**: Represents execution failure (e.g. syntax error, semantic
  error, or evaluation error).

> [!NOTE]
> `Result` is a functional outcome concept aligned with the shared Evo model.
> Concrete Rust representations, type parameters, generics, error structures,
> and enum variants are deliberately **not** decided in this User Story.

---

## Open Questions

- **Invocation Values**: How invocation Values required by the public function
  are supplied to the Engine boundary is not yet closed.

---

## Non-Responsibilities and Out of Scope

For the scope of this User Story, the Evo-Script Engine is **not** responsible
for:
- Reading `.efn` files from the filesystem or resolving file paths.
- Terminal interaction, formatting, or UI presentation.
- Starting, stopping, or managing Evo Runtime application lifecycles.
- Query parsing or execution semantics belonging to EvoQ.
- Dynamic engine loading, packaging, ABI negotiation, or plugin hosting.
- Deciding whether internal Engine implementation uses separate lexer, parser,
  AST, semantic analyzer, or evaluator modules/structs.
