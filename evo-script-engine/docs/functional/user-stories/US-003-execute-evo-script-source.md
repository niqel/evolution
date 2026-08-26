# US-003 — Execute Evo-Script Source

Status: FUNCTIONAL CLOSED

## Story

```text
As a Consumer,
I want to provide the complete source text of an Evo-Script program
and the Invocation Values required by its public function
to the Evo-Script Engine,
so that the source program is executed according to Evo-Script Language
Specification v0 and I receive its Result.
```

*(Como un Consumer, quiero proporcionar el texto fuente completo de un programa
Evo-Script y los Invocation Values requeridos por su función pública al
Evo-Script Engine, para que el programa fuente sea ejecutado según la Evo-Script
Language Specification v0 y yo reciba su Result).*

---

## Context

The Evo-Script Engine is the component responsible for compiling and executing
Evo-Script programs.

Under **US-003 (Execute Source)**, the Consumer provides the **complete source
text** of an Evo-Script program and the ordered **Invocation Values** (0..N)
directly to the Engine, executing the program in a single public operation
without requiring a prior separate Compile step or the external management of a
Compiled Program.

```text
Consumer
   │
   ├── Complete Evo-Script Source Text
   └── Invocation Values 0..N
           │
           ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  Execute Source (v0 specification)     │
│  Binds Invocation Values to parameters │
│  Executes single public function       │
└──────────────────┬─────────────────────┘
                   │
                   │ execution outcome
                   ▼
                 Result
```

### Boundary Input Distinctions
- **Source Text != File Path**: The Engine does not perform file I/O or path
  resolution; reading the physical `.efn` file is the responsibility of the
  Consumer or an external loader.
- **Source Text != AST / Token Stream**: The Consumer passes raw text, not a
  pre-parsed abstract syntax tree or token stream.
- **Source Text != Individual Function**: The input is the complete,
  self-contained source text of the program unit.
- **Source Text != Compiled Program**: The Consumer supplies raw source code;
  Execute Source does not require or expect a previously compiled artifact.
- **Invocation Values != Command-Line Strings**: The Engine receives structured
  Values, not raw terminal strings requiring implicit string-parsing.
- **No Prior Compile Required**: The Consumer is not required to call Compile or
  manage intermediate representations before calling Execute Source.

---

## Functional Semantics of Execute Source

1. **Direct Execution**: Execute Source accepts source text and Invocation Values,
   processes the program according to
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md),
   executes its single public function, and returns `Result`.
2. **Invalid Source Handling**: If the source text violates lexical, syntactic,
   or semantic rules of Evo-Script v0, Execute Source produces a failed
   `Result` without executing the public function.
3. **No Compiled Program Returned**: Execute Source does not return a Compiled
   Program as part of its public functional contract; its sole public outcome is
   `Result`.
4. **No Persistence or Caching**: Execute Source does not save, register,
   persist, or cache a Compiled Program as a public functional responsibility.
5. **Internal Strategy Open**: The internal implementation strategy of Execute
   Source (e.g. direct tree-walk interpretation, temporary bytecode compilation,
   or intermediate representation evaluation) remains intentionally open and
   unconstrained by this User Story.

---

## Invocation Values

Invocation Values are the ordered Values supplied by the Consumer to satisfy
the parameters declared by the program's single public function.

### Rules of Invocation Values

1. **Cardinality**: Invocation Values contains zero or more Values (`0..N`).
2. **Zero Parameters**: A public function without parameters requires zero
   Invocation Values.
3. **Exact Arity**: A public function with $N$ parameters requires exactly $N$
   Invocation Values.
4. **Strict Positional Mapping**: Mapping from Invocation Values to parameters
   is strictly positional:
   ```text
   InvocationValue[0]     ──► Parameter[0]
   InvocationValue[1]     ──► Parameter[1]
   ...
   InvocationValue[N - 1] ──► Parameter[N - 1]
   ```
5. **Declaration Order**: The order of Invocation Values corresponds directly to
   the declaration order of parameters in the public function signature.
6. **Semantic Compatibility**: Each Invocation Value must be semantically
   compatible with the parameter type declared by the public function
   (including native types, struct types, and enum types defined in the
   program).
7. **No Implicit Conversions**: The Engine performs no implicit conversions or
   coercions to adapt an incompatible Value to a parameter type.
8. **Arity Mismatch**: An Invocation Value count that does not match the
   parameter count results in a failed Result.
9. **Type Incompatibility**: An Invocation Value incompatible with its
   corresponding parameter type results in a failed Result.
10. **Failure Independence**: Specific Failure categories, error codes, and
    variants are not defined at this functional level.
11. **Technical Representation**: Concrete Rust representations of Invocation
    Values, collections, slices, or type handles remain deferred to technical
    design.

### Conceptual Example

Given an Evo-Script public function:

```text
public fn sum(int left, int right) -> int
{
    return left + right;
}
```

The Consumer supplies:
- Source Text containing `sum`
- Invocation Values: `[10, 20]`

Positional binding:
- `InvocationValue[0]` (`10`) $\rightarrow$ bound to parameter `left`
- `InvocationValue[1]` (`20`) $\rightarrow$ bound to parameter `right`

---

## Acceptance Criteria

1. The Consumer can provide the complete source text of one Evo-Script v0
   program to the Evo-Script Engine.
2. The Consumer may provide zero or more Invocation Values.
3. The Consumer is not required to invoke Compile or obtain a Compiled Program
   prior to invoking Execute Source.
4. The Engine treats the source text as one complete Evo-Script program and
   processes it according to:
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md).
5. If the source text violates lexical, syntactic, or semantic rules of
   Evo-Script v0, Execute Source produces a failed Result without executing the
   public function.
6. The Engine determines the parameters declared by the public function.
7. Invocation Values are matched to parameters strictly by position.
8. Invocation Value count must equal public-function parameter count ($N$).
9. Each Invocation Value must be semantically compatible with its corresponding
   parameter type.
10. No implicit conversion is performed by the Engine to repair incompatible
    Invocation Values.
11. Zero-parameter public functions require zero Invocation Values.
12. Arity mismatch results in a failed Result.
13. Type incompatibility results in a failed Result.
14. The Engine executes the public function according to Evo-Script v0 semantics.
15. Successful execution preserves the Value produced by the public function
    in the successful Result.
16. Failed processing or execution produces a failed Result rather than being
    silently treated as success.
17. Execute Source does not return a Compiled Program as part of its public
    contract.
18. Execute Source does not persist, write to disk, or cache a Compiled Program
    as a public functional responsibility.
19. The Consumer does not need to know the Engine's internal processing, AST,
    VM, or interpreter architecture.
20. Once Result is returned to the Consumer, that Execute Source invocation is
    complete.
21. No explicit Engine session object or execution-context object is required at
    the functional boundary.

---

## Execution Outcome Concept (Result)

The functional outcome of executing source code directly is represented
conceptually as `Result`:
- **Successful Outcome**: Preserves the program's produced `Value`.
- **Failed Outcome**: Represents execution failure (e.g. lexical/syntax error,
  semantic error, arity mismatch, type incompatibility, or runtime evaluation
  error).

> [!NOTE]
> `Result` is a functional outcome concept aligned with the shared Evo model
> (`Result != Value`, `Result != Failure`).
> Concrete Rust representations, type parameters, generics (`Result<T, E>`),
> error structures, and enum variants are deliberately **not** decided in this
> User Story.

---

## Non-Responsibilities and Out of Scope

For the scope of US-003 and Evo-Script Engine v0:
- Reading `.efn` files from the filesystem or resolving file paths.
- Producing or returning a Compiled Program (handled by US-001).
- Persisting, caching, or serializing compiled artifacts.
- Terminal interaction, formatting, or UI presentation.
- Starting, stopping, or managing Evo Runtime application lifecycles.
- Query parsing or execution semantics belonging to EvoQ.
- Internal execution engine architecture decisions (e.g. AST interpreter vs
  temporary bytecode VM).
- External side-effects, console output, `print`, stdout, filesystem I/O,
  Requesters, Providers, or intermediate callbacks during execution.
