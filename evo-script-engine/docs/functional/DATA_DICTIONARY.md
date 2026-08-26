# Evo-Script Engine v0 — Functional Data Dictionary

Status: FUNCTIONAL CLOSED

This document consolidates the canonical architectural and functional vocabulary
for `evo-script-engine` v0, derived from closed User Stories US-001 through
US-003 and the Evo-Script Language Specification v0.

---

## 1. Purpose and Scope

The purpose of this Data Dictionary is to formally define the canonical data
concepts, roles, and boundaries utilized by `evo-script-engine` v0.

The Evo-Script Engine provides three distinct public functional operations:

```text
1. Compile
   Source Text ──► Compiled Program (on successful compilation)

2. Execute Compiled
   Compiled Program + Invocation Values (0..N) ──► Result

3. Execute Source
   Source Text + Invocation Values (0..N) ──► Result
```

This document establishes functional definitions and constraints. It deliberately
does **not** define concrete Rust structs, enums, traits, generics, memory
layouts, or binary formats.

---

## 2. Boundary Data

### Source Text

- **Category**: Boundary Input Data
- **Definition**: The complete textual representation of exactly one Evo-Script
  v0 Program supplied to the Evo-Script Engine.
- **Characteristics**:
  - Represents a complete, self-contained Evo-Script Program.
  - When persisted externally as a source file artifact, Evo-Script v0 uses the
    canonical file extension `.efn`.
  - The Engine receives the raw textual content, **not** a physical file or path.
  - `Source Text != File Path` (the Engine does not perform path resolution or
    filesystem I/O).
  - `Source Text != AST / Token Stream` (the Consumer supplies raw source text).
  - `Source Text != Compiled Program` (Source Text is uncompiled source code).
  - `Source Text != Individual Function` (it contains the entire program unit).
  - The Consumer is solely responsible for reading or obtaining the text from
    external storage before invoking the Engine.
- **Cardinality**:
  - Exactly 1 `Source Text` per `Compile` invocation.
  - Exactly 1 `Source Text` per `Execute Source` invocation.
- **Sources**: US-001, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Compiled Program

- **Category**: Boundary Data (Output of Compile / Input of Execute Compiled)
- **Definition**: The Engine-produced executable representation of an
  Evo-Script Program that has been successfully processed according to
  Evo-Script Language Specification v0 and is suitable for later execution by the
  Evo-Script Engine.
- **Characteristics**:
  - `Compiled Program != Source Text` (represents an already processed and
    validated compilation artifact).
  - Produced solely as the successful outcome of `Compile`.
  - Can be supplied subsequently as input to `Execute Compiled`.
  - Reusable across multiple independent `Execute Compiled` invocations.
  - Persistence, caching, and storage belong to the Consumer or external
    components; the Engine does not manage physical storage or program
    registries in v0.
  - Its internal technical representation remains **open** (e.g. bytecode, IR,
    validated AST, or binary stream are technical candidates deferred to
    technical design).
  - No file extension is assigned to Compiled Program in v0.
- **Source Location Guarantee**:
  - A `Compiled Program` must preserve sufficient relationship to the original
    `Source Text` so that, when a program-originated execution failure occurs,
    the Evo-Script Engine can identify the corresponding source line during
    `Execute Compiled`.
- **Cardinality**:
  - Produced on successful `Compile` invocations (0..1 per compilation).
  - Exactly 1 `Compiled Program` per `Execute Compiled` invocation.
- **Sources**: US-001, US-002

### Invocation Values

- **Category**: Boundary Input Data
- **Definition**: The ordered Values supplied by the Consumer for the parameters
  declared by the program's single Public Function.
- **Characteristics**:
  - Cardinality: `0..N` Values.
  - A Public Function with zero parameters requires zero Invocation Values.
  - A Public Function with $N$ parameters requires exactly $N$ Invocation Values.
  - Mapping to parameters is strictly positional:
    ```text
    InvocationValue[0]     ──► Parameter[0]
    InvocationValue[1]     ──► Parameter[1]
    ...
    InvocationValue[N - 1] ──► Parameter[N - 1]
    ```
  - The order of Invocation Values corresponds directly to the declaration order
    of parameters in the Public Function signature.
  - Each Invocation Value must be semantically compatible with its corresponding
    parameter type (including native types, struct types, and enum types defined
    by the program).
  - The Engine performs no implicit conversions or coercions.
  - Used by `Execute Compiled` and `Execute Source`.
  - **Not** used by `Compile`.
  - `Invocation Values != Command-Line Strings` (represents structured data
    values, not raw terminal arguments).
- **Sources**: US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Result

- **Category**: Boundary Output Data
- **Definition**: The functional outcome of an execution operation performed by
  the Evo-Script Engine.
- **Characteristics**:
  - Represents the completed outcome of `Execute Source` or `Execute Compiled`.
  - **Not** assigned as the outcome of `Compile` in this functional phase.
  - Conceptual structure:
    ```text
    Result
    ├── success ──► preserves produced Value
    └── failure ──► represents Failure
    ```
  - `Result != Value`
  - `Result != Failure`
  - Aligned with the shared outcome model of `evo-values`.
  - Does not assume concrete Rust generics (`Result<T, E>`) or
    `std::result::Result`.
- **Cardinality**:
  - Exactly 1 `Result` per completed `Execute Source` invocation.
  - Exactly 1 `Result` per completed `Execute Compiled` invocation.
- **Sources**: US-002, US-003

---

## 3. Shared Data Concepts

### Value

- **Category**: Shared Data Concept
- **Definition**: The shared Evo data concept used to transport semantic values
  between the Consumer, function parameters, and execution outcomes.
- **Characteristics**:
  - Belongs conceptually to the shared `evo-values` data model.
  - `Invocation Values` contains `0..N` Values.
  - A successful `Result` preserves the `Value` produced by the Public Function.
  - Concrete value types supported are defined by the Evo-Script Language
    Specification v0 (primitives, structs, enums).
  - This Data Dictionary does not redefine the complete internal semantics or
    memory layout of Value.
  - Does not introduce Rust ownership, lifetimes, or smart pointer semantics.
- **Sources**: US-002, US-003, `evo-values`, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Failure

- **Category**: Shared Outcome Concept
- **Definition**: The minimal functional diagnostic describing why Evo-Script
  processing or execution did not complete successfully.
- **Characteristics**:
  - Single shared concept across the Engine; no separate `Error`,
    `CompileError`, or `ExecutionError` concepts.
  - Minimal v0 data elements:
    ```text
    Failure
    ├── description: exactly 1
    └── source line: 0..1 (1-based line number in Source Text, if applicable)
    ```
  - **Description**: Textual explanation of the failure (always present).
  - **Source Line**: 1-based line index (`line 1` = first line of Source Text)
    associated with the failure when a source location exists.
  - **Presence of Source Line**:
    - *Required* for lexical, syntactic, semantic, and program-originated
      runtime evaluation failures.
    - *Absent* for boundary invocation failures that do not correspond to an
      internal program line (e.g. arity mismatch or boundary type
      incompatibility). No artificial lines (such as line 0) are generated.
  - **Relationship to Compile**: Failed compilation produces a Failure
    diagnostic (the technical wrapper or return mechanism for Compile remains
    open).
  - **Relationship to Execution**: A failed `Result` from `Execute Source` or
    `Execute Compiled` expresses a Failure.
  - **Excluded in v0**: Does not mandate columns, error codes, categories,
    severities, stack traces, spans, or byte offsets.
- **Sources**: US-001, US-002, US-003

---

## 4. Referenced Evo-Script Language Concepts

### Evo-Script Program

- **Category**: Language Domain Concept
- **Definition**: The complete, self-contained program unit defined by
  Evo-Script Language Specification v0.
- **Characteristics**:
  - Contained entirely within a single source file (`.efn`).
  - Represented textually by `Source Text`.
  - Represented executably by a `Compiled Program` following successful
    compilation.
  - Declares exactly 1 Public Function (`public fn`).
  - May declare `0..N` private functions, structs, and enums local to the file.
  - Does not represent a technical Rust struct.
- **Sources**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Public Function

- **Category**: Language Domain Concept
- **Definition**: The single public entry function declared by an Evo-Script
  Program v0 (`public fn`) and executed during execution operations.
- **Characteristics**:
  - Exactly 1 per Evo-Script Program v0.
  - Declares `0..N` Parameters.
  - Receives Invocation Values via strict positional binding.
  - Evaluates expressions and produces a Value according to Evo-Script v0.
  - `Public Function != main` (does not imply OS process entry point).
  - `Public Function != Runtime startup / Run` (independent of Evo Runtime).
- **Sources**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Parameter

- **Category**: Language Domain Concept
- **Definition**: A typed formal parameter declared in the signature of the
  Public Function.
- **Characteristics**:
  - Cardinality: `0..N` per Public Function.
  - Has a declared position and a declared type according to Evo-Script v0.
  - Receives exactly one Invocation Value matching its declaration index during
    a valid execution.
  - In v0, parameters are immutable bindings; there are no named arguments,
    default values, optional parameters, variadic parameters, or reference/mutable
    modifiers.
- **Sources**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

---

## 5. Roles and Components

### Consumer

- **Category**: External Functional Role
- **Definition**: The external caller or system component that invokes the
  public capabilities of the Evo-Script Engine.
- **Responsibilities**:
  - Supplies `Source Text` to `Compile` or `Execute Source`.
  - Supplies `Compiled Program` to `Execute Compiled`.
  - Supplies `Invocation Values` to `Execute Source` or `Execute Compiled`.
  - Receives a `Compiled Program` upon successful `Compile`.
  - Receives a `Result` upon completion of `Execute Source` or `Execute Compiled`.
  - Manages external file reading, program persistence, or caching if desired.
- **Invariants / Distinctions**:
  - `Consumer` is a functional role (e.g. CLI, runner, test suite, or host
    application) and is not a technical data structure.
- **Sources**: US-001, US-002, US-003

### Evo-Script Engine

- **Category**: Core Engine / Component
- **Definition**: The platform component that implements the compilation and
  execution rules of Evo-Script Language Specification v0.
- **Public Capabilities**:
  1. **`Compile`**: `Source Text` $\longrightarrow$ `Compiled Program`
  2. **`Execute Source`**: `Source Text` $+$ `Invocation Values` $\longrightarrow$ `Result`
  3. **`Execute Compiled`**: `Compiled Program` $+$ `Invocation Values` $\longrightarrow$ `Result`
- **Invariants / Distinctions**:
  - `Evo-Script Engine != Evo Runtime` (does not coordinate applications,
    manage providers, or handle Runtime Start/Run).
  - Does not manage terminal I/O, UI, filesystem discovery, or side-effects in
    v0.
- **Sources**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

---

## 6. Canonical Relationships

The canonical relationships across vocabulary terms are summarized below:

```text
Source Text
  └── represents ──────────────────────────► Evo-Script Program

Compiled Program
  └── represents (executable) ─────────────► Successfully compiled Evo-Script Program
  └── preserves source-line relationship ──► Source Text (for failure reporting)

Evo-Script Program
  └── declares ────────────────────────────► Exactly 1 Public Function

Public Function
  └── declares ────────────────────────────► 0..N Parameters

Invocation Values
  └── contains ────────────────────────────► 0..N Values
  └── maps positionally (0..N-1) ──────────► Parameters (0..N-1)

Result
  ├── success branch ──────────────────────► Preserves produced Value
  └── failure branch ──────────────────────► Expresses Failure

Failure
  ├── description ─────────────────────────► Exactly 1 (mandatory textual description)
  └── source line ─────────────────────────► 0..1 (1-based, present when line exists)

Compile
  ├── consumes ────────────────────────────► Source Text
  ├── success outcome ─────────────────────► Compiled Program
  └── failure outcome ─────────────────────► Failure diagnostic available

Execute Source
  ├── consumes ────────────────────────────► Source Text
  ├── consumes ────────────────────────────► Invocation Values (0..N)
  └── produces ────────────────────────────► Result

Execute Compiled
  ├── consumes ────────────────────────────► Compiled Program
  ├── consumes ────────────────────────────► Invocation Values (0..N)
  └── produces ────────────────────────────► Result
```
