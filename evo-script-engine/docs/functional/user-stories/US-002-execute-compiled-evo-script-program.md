# US-002 — Execute Compiled Evo-Script Program

Status: FUNCTIONAL CLOSED

## Story

```text
As a Consumer,
I want to provide a Compiled Program and the Invocation Values
required by its public function to the Evo-Script Engine,
so that the compiled program is executed and I receive its Result.
```

*(Como un Consumer, quiero proporcionar un Compiled Program y los Invocation
Values requeridos por su función pública al Evo-Script Engine, para que el
programa compilado sea ejecutado y yo reciba su Result).*

---

## Context

The Evo-Script Engine is the component responsible for compiling and executing
Evo-Script programs.

Under **US-002 (Execute)**, the Consumer provides a previously compiled
**Compiled Program** and the ordered **Invocation Values** (0..N) required by its
single public function.

Execute does **not** accept source text and does **not** perform compilation. It
evaluates the compiled program directly and returns the execution `Result`.

```text
Consumer
   │
   ├── Compiled Program
   └── Invocation Values 0..N
           │
           ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  Binds Invocation Values to parameters │
│  Executes single public function       │
└──────────────────┬─────────────────────┘
                   │
                   │ execution outcome
                   ▼
                 Result
```

---

## Invocation Values

Invocation Values are the ordered Values supplied by the Consumer to satisfy the
parameters declared by the program's single public function.

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
- Compiled Program containing `sum`
- Invocation Values: `[10, 20]`

Positional binding:
- `InvocationValue[0]` (`10`) $\rightarrow$ bound to parameter `left`
- `InvocationValue[1]` (`20`) $\rightarrow$ bound to parameter `right`

---

## Acceptance Criteria

1. The Consumer can provide a valid Compiled Program to the Evo-Script Engine.
2. The Consumer may provide zero or more Invocation Values.
3. Execute does not receive source text and does not perform source compilation.
4. The Engine determines the parameters declared by the public function of the
   Compiled Program.
5. Invocation Values are matched to parameters strictly by position.
6. Invocation Value count must equal public-function parameter count ($N$).
7. Each Invocation Value must be semantically compatible with its corresponding
    parameter type.
8. No implicit conversion is performed by the Engine to repair incompatible
    Invocation Values.
9. Zero-parameter public functions require zero Invocation Values.
10. Arity mismatch results in a failed Result.
11. Type incompatibility results in a failed Result.
12. The Engine executes the public function according to Evo-Script v0 semantics.
13. Successful execution preserves the Value produced by the public function
    in the successful Result.
14. Failed execution produces a failed Result rather than being silently treated
    as success.
15. The Consumer does not need to know the Engine's internal VM, interpreter, or
    evaluator architecture.
16. Once Result is returned to the Consumer, that Execute invocation is complete.
17. No explicit Engine session object or execution-context object is required at
    the functional boundary.

---

## Execution Outcome Concept (Result)

The functional outcome of executing a compiled program is represented
conceptually as `Result`:
- **Successful Outcome**: Preserves the program's produced `Value`.
- **Failed Outcome**: Represents execution failure (e.g. arity mismatch, type
  incompatibility, runtime evaluation error).

> [!NOTE]
> `Result` is a functional outcome concept aligned with the shared Evo model
> (`Result != Value`, `Result != Failure`).
> Concrete Rust representations, type parameters, generics (`Result<T, E>`),
> error structures, and enum variants are deliberately **not** decided in this
> User Story.

---

## Non-Responsibilities and Out of Scope

For the scope of US-002 and Evo-Script Engine v0:
- Compiling or parsing Evo-Script source text (handled by US-001).
- Loading Compiled Programs from physical storage or filesystems.
- Terminal interaction, formatting, or UI presentation.
- Starting, stopping, or managing Evo Runtime application lifecycles.
- Query parsing or execution semantics belonging to EvoQ.
- Internal execution engine architecture decisions (e.g. stack VM, register VM,
  or tree-walk interpreter).
- External side-effects, console output, `print`, stdout, filesystem I/O,
  Requesters, Providers, or intermediate callbacks during execution.
