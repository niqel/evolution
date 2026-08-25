# US-001 — Compile Evo-Script Source

Status: FUNCTIONAL CLOSED

## Story

```text
As a Consumer,
I want to provide the complete source text of an Evo-Script program
to the Evo-Script Engine,
so that the source is compiled according to Evo-Script Language
Specification v0 and I receive a Compiled Program.
```

*(Como un Consumer, quiero proporcionar el texto fuente completo de un programa
Evo-Script al Evo-Script Engine, para que el código fuente sea compilado según la
Evo-Script Language Specification v0 y yo reciba un Compiled Program).*

---

## Context

The Evo-Script Engine is the component responsible for compiling and executing
Evo-Script programs.

In Evo-Script v0, a complete program is contained within a single `.efn` source
file and declares exactly one public function (`public fn`), along with optional
private functions, structs, and enums local to that file.

Under **US-001 (Compile)**, the Consumer provides the **complete source text** of
the program directly to the Engine. The Engine processes and validates the
source text according to the language specification and produces a **Compiled
Program** ready for future execution.

```text
Consumer
   │
   │ complete Evo-Script source text
   ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  Processes & validates source text     │
│  Compiles according to v0 spec         │
└──────────────────┬─────────────────────┘
                   │
                   │ successful compilation
                   ▼
            Compiled Program
```

### Boundary Input Distinctions
- **Source Text != File Path**: The Engine does not perform file I/O or path
  resolution; reading the physical `.efn` file is the responsibility of the
  Consumer or an external loader.
- **Source Text != AST / Token Stream**: The Consumer passes raw text, not an
  intermediate or pre-parsed syntax tree.
- **Source Text != Individual Function**: The input is the complete,
  self-contained source text of the program unit.

---

## Compiled Program Concept

A **Compiled Program** is the Engine-produced representation of an Evo-Script
program that has been successfully processed according to the Evo-Script
Language Specification v0 and is suitable for later execution by the Evo-Script
Engine.

### Conceptual Characteristics
- **Compiled Program != Source Text**: It represents an already processed,
  validated compilation output.
- **Format Open**: The internal technical representation of a Compiled Program
  is not frozen at this functional stage (e.g. bytecode, IR, validated AST, or
  binary format remain open technical candidates).
- **No Persistence in Engine**: The Engine produces the Compiled Program in
  memory and returns it to the Consumer. Persisting, caching, or writing the
  Compiled Program to storage is the responsibility of the Consumer or external
  components.

---

## Functional Rules of Compile

1. **Complete Unit**: The Consumer provides the complete source text of one
   Evo-Script v0 program.
2. **Specification Conformance**: The Engine compiles the source according to
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md).
3. **Outcome on Success**: Successful compilation produces a valid Compiled
   Program.
4. **Outcome on Error**: If the source text violates lexical, syntactic, or
   semantic rules of Evo-Script v0, Compile fails and does not produce a
   Compiled Program.
5. **No Execution**: Compile does not execute the public function or evaluate
   expressions.
6. **No Invocation Values**: Compile does not accept or require Invocation
   Values.
7. **No File Persistence**: Compile does not write files to disk or manage
   storage.

---

## Acceptance Criteria

1. The Consumer can provide the complete source text of one Evo-Script v0
   program to the Evo-Script Engine.
2. The Engine treats that source text as one complete Evo-Script program.
3. The Engine processes and compiles the program according to:
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md).
4. Successful compilation produces a Compiled Program representing the compiled
   unit.
5. If the source text contains lexical, syntactic, or semantic errors according
   to Evo-Script v0, the compilation fails and does not produce a valid Compiled
   Program.
6. The Consumer does not need to parse or preprocess the source text before
   providing it to the Engine.
7. The Consumer does not need to know the Engine's internal compiler pipeline,
   AST, or intermediate representations.
8. Compile does not execute the public function or evaluate program
   expressions.
9. Compile does not accept or require Invocation Values.
10. Compile does not persist, write to disk, or store the resulting Compiled
    Program.
11. Once the Compiled Program (or compilation failure) is returned, the Compile
    invocation is complete.

---

## Non-Responsibilities and Out of Scope

For the scope of US-001:
- Reading `.efn` files from the filesystem or resolving file paths.
- Executing the compiled program or evaluating runtime expressions (handled by
  US-002).
- Accepting or binding Invocation Values.
- Persisting, caching, or serializing Compiled Programs to storage.
- Terminal interaction, formatting, or UI presentation.
- Starting, stopping, or managing Evo Runtime application lifecycles.
- Query parsing or execution semantics belonging to EvoQ.
- Internal compiler architecture decisions (e.g. separate lexer/parser/AST
  crates or modules).
