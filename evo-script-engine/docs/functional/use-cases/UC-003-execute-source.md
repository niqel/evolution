# UC-003 — Execute Source

Status: REVALIDATED — FUNCTIONAL CLOSED

## 1. Purpose

Definir funcionalmente cómo un `Consumer` solicita al Evo-Script Engine ejecutar directamente el `Source Text` completo de un programa Evo-Script junto con sus `Invocation Values`, sin requerir una invocación pública previa de `Compile` ni administrar externamente un `Compiled Program`.

```text
Source Text + Invocation Values
                │
                ▼
          Execute Source
                │
                ▼
              Result
```

`Execute Source` debe ser semánticamente equivalente a `Compile` seguido de `Execute Compiled` cuando la compilación es exitosa.

---

## 2. Traceability

- **Deriva de**: `US-003 — Execute Evo-Script Source`.
- **Utiliza**: `Functional Data Dictionary`.
- **Aplica**: `UC-001 — Compile` y `UC-002 — Execute Compiled`.
- **Aplica normativamente**: Evo-Script v0.1 más `EFN_HOST_BOUNDARY_v0.1.md`.

---

## 3. Functional Operation

```text
Execute Source(
    Source Text,
    Invocation Values
) -> Result
```

Esta frontera no define todavía Rust Signature, ownership, borrowing, lifetimes ni Participants técnicos.

---

## 4. Consumer

El Consumer:

- proporciona exactamente 1 Source Text;
- proporciona `0..N Invocation Values`;
- compone explícitamente las External Capabilities requeridas;
- recibe exactamente 1 Result;
- no necesita invocar públicamente Compile;
- no recibe ni administra el Compiled Program interno.

El Consumer puede ser CLI, UI, API u otro componente. Su prompt, Scope, presentación o estado de sesión no se heredan por el `.efn`.

---

## 5. Functional Inputs

### Source Text

- exactamente 1;
- representa un Evo-Script Program completo;
- `Source Text != File Path`;
- `Source Text != AST`;
- `Source Text != Token Sequence`;
- `Source Text != Compiled Program`;
- el Engine no realiza filesystem I/O para obtenerlo.

### Invocation Values

- `0..N Value` ordenados;
- binding estrictamente posicional;
- aridad exacta;
- compatibilidad semántica exacta;
- sin coerciones implícitas;
- `Invocation Values != Command-Line Strings`;
- no incluyen Active Scope ni estado interactivo del Consumer.

---

## 6. Functional Output

Toda invocación concluida produce exactamente 1 `Result`.

```text
Result
├── Success → Value
└── Failure → Failure
```

No se exponen públicamente Compile Outcome, Compiled Program, bytecode, AST, Tokens ni otras representaciones intermedias.

La presentación o uso específico del Result pertenece al Consumer.

---

## 7. Semantic Equivalence

Para el mismo Source Text, los mismos Invocation Values y las mismas External Capabilities explícitas disponibles:

```text
Execute Source(source, values)

        ≡

Compile(source)
    │
    ├── Failure ───────────► Result.failure
    │
    └── Success(compiled)
             │
             ▼
      Execute Compiled(
          compiled,
          values
      )
```

La equivalencia no obliga a que la implementación Rust invoque literalmente una Public Capability desde otra.

---

## 8. Compilation Phase

La fase de compilación:

1. recibe Source Text;
2. valida reglas lexicales;
3. valida reglas sintácticas;
4. valida reglas semánticas;
5. produce bytecode dentro de un Compiled Program;
6. puede preservar External Symbols sin resolver;
7. no requiere Providers concretos ni bindings de ejecución;
8. no ejecuta el programa.

La gramática vigente de `.efn` no contiene `use` ni estado Active Scope.

Si Compile falla, no comienza ejecución.

---

## 9. Internal Compiled Program

Cuando la compilación es exitosa se produce conceptualmente un Compiled Program temporal:

```text
Source Text
    ↓
Compile semantics
    ↓
Compiled Program
    ├── bytecode
    └── External Symbols 0..N
```

- no se devuelve al Consumer;
- no se persiste o cachea implícitamente;
- no contiene Active Scope ni Host Session State.

---

## 10. Execution Phase

Cuando Compile es exitoso, Execute Source aplica las mismas reglas de Execute Compiled:

1. utiliza el Compiled Program;
2. determina el entry point conforme a Evo-Script;
3. valida aridad;
4. realiza binding posicional;
5. valida compatibilidad de Values;
6. no realiza coerciones implícitas;
7. ejecuta bytecode;
8. mantiene estado local independiente;
9. resuelve External Symbols únicamente mediante capabilities explícitas;
10. produce Result.

---

## 11. External Symbols and Capabilities

```text
Compiled Program
      │
      ▼
    Bytecode
      │
      ▼
External Symbol
      │
      ▼
explicit application binding
      │
      ▼
External Capability
```

Reglas:

- la ausencia de Provider durante Compile no implica failure;
- el Engine no descubre Providers ni bindings;
- no existe un Provider activo ambiental dentro del `.efn`;
- distintas capabilities pueden utilizarse explícitamente durante una misma función;
- si una capability requerida no está disponible durante ejecución, se produce Result.failure.

---

## 12. Local Execution State and Host Boundary

Cada invocación posee estado local independiente.

```text
Local Execution State
├── Pipeline Data
├── function / frame evaluation state
└── temporary values required by execution
```

Explícitamente no contiene:

```text
Active Scope
Host Prompt
CLI/UI/API Session State
Current Provider
```

Reglas:

1. el programa no hereda Scope de CLI, UI, API u otra sesión;
2. `.efn` no establece ni cambia Active Scope;
3. `use` no es una construcción válida `.efn`;
4. Pipeline Data representa composición de datos;
5. el estado termina al concluir la invocación;
6. el Engine no mantiene una Session implícita persistente.

Un Host interactivo puede mantener su propio Scope fuera del Engine y utilizarlo para decidir qué Command o Use Case invocar.

---

## 13. Failure Flows

### Compilation Failure

Puede originarse por violación lexical, sintáctica o semántica. No comienza ejecución y se produce Result.failure.

### Invocation / Execution Failure

Puede originarse por mismatch de aridad/tipo o por Evaluation Error definido por Evo-Script.

### External Capability Failure

Puede originarse cuando una External Symbol requerida no puede satisfacerse mediante los bindings explícitos o cuando la interacción externa falla de forma propagable.

---

## 14. Postconditions

### Success

- Compile interno exitoso;
- bytecode ejecutado conforme a Evo-Script;
- Result.success preserva el Value producido;
- no queda Session o Active Scope implícito;
- el Compiled Program temporal no se convierte automáticamente en estado persistente.

### Failure

- el fallo se expresa mediante Result.failure;
- no se expone Compiled Program parcial;
- no queda estado local compartido con futuras ejecuciones.

---

## 15. Functional Invariants

1. El nombre canónico es `Execute Source`.
2. Consume exactamente 1 Source Text.
3. Consume `0..N Invocation Values`.
4. Produce exactamente 1 Result.
5. El Consumer no necesita invocar Compile previamente.
6. Aplica las mismas reglas funcionales de Compile.
7. Produce bytecode antes de ejecutar.
8. Compile failure impide ejecución.
9. Compile success aplica las reglas de Execute Compiled.
10. Aridad exacta y binding posicional.
11. Sin coerciones implícitas.
12. External Symbols pueden preservarse en el Compiled Program.
13. External Capabilities deben suministrarse explícitamente.
14. El Engine no descubre Providers.
15. No existe Provider activo ambiental.
16. Cada invocación mantiene estado local independiente.
17. El estado local no contiene Active Scope.
18. `use` no forma parte de `.efn`.
19. Pipeline representa data composition.
20. El Compiled Program interno no se devuelve ni se persiste implícitamente.
21. El Engine no realiza filesystem I/O para obtener Source Text.
22. El Engine no presenta Result en terminal, UI o HTTP.
23. El Consumer no altera la semántica del `.efn`.
24. No queda Session implícita entre invocaciones.
25. Execute Source ≡ Compile + Execute Compiled bajo las mismas entradas y capabilities explícitas.
26. Este nivel no decide Participants ni Rust Signatures.

---

## 16. Out of Scope

- rutas físicas `.efn`;
- command-line string parsing;
- persistencia/caché de artifacts;
- lifecycle de Evo Applications;
- presentación terminal/UI/HTTP;
- Interactive Scope / prompt;
- descubrimiento automático de Providers;
- VM internals y Technical Data Model;
- Participants y Rust Signatures.

---

## 17. Summary

```text
Consumer
   │
   ├── Source Text
   ├── Invocation Values
   └── explicit capability composition
           │
           ▼
      Execute Source
           │
           ├── Compile
           │      ↓
           │  Compiled Program
           │
           └── Execute Compiled
                   ├── Pipeline Data
                   ├── evaluation state
                   ├── External Symbols
                   │       ↓
                   │ explicit bindings
                   ▼
                 Result

No Active Scope crosses or exists inside the `.efn` execution boundary.
```

## Closure

`UC-003 — Execute Source` queda revalidado y `FUNCTIONAL CLOSED` bajo la frontera `.efn` / Host vigente.
