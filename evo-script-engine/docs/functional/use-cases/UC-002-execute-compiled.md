# UC-002 — Execute Compiled

Status: REVALIDATED — FUNCTIONAL CLOSED

## 1. Purpose

Definir funcionalmente cómo un `Consumer` solicita a `evo-script-engine` ejecutar un `Compiled Program` existente, proporcionando los `Invocation Values` requeridos y obteniendo un `Result`.

```text
Compiled Program + Invocation Values
              │
              ▼
       Execute Compiled
              │
              ▼
            Result
```

`Execute Compiled` es el nombre semántico canónico del Use Case.

---

## 2. Traceability

- **Deriva de**: `US-002 — Execute Compiled Evo-Script Program`.
- **Utiliza**: `Functional Data Dictionary`.
- **Aplica normativamente**: Evo-Script v0.1 más `EFN_HOST_BOUNDARY_v0.1.md`.
- **Corresponde a Public Capability**: `Execute Compiled`.

---

## 3. Functional Operation

```text
Execute Compiled(
    Compiled Program,
    Invocation Values
) -> Result
```

La notación expresa la necesidad funcional del Consumer. No fija Rust Signature, ownership, borrowing, lifetimes, Requesters, Contracts, Resolvers, Providers, Collaborators ni representación técnica de capabilities externas.

Cuando el `Compiled Program` requiere `External Capabilities`, estas deben estar disponibles mediante bindings explícitamente suministrados por la composición de la aplicación.

---

## 4. Consumer

El Consumer debe:

- suministrar exactamente un `Compiled Program`;
- suministrar `0..N Invocation Values`;
- haber compuesto explícitamente las External Capabilities necesarias cuando el programa las requiera;
- recibir exactamente un `Result`.

El Consumer no necesita crear una sesión persistente del Engine y su estado interactivo no se hereda por la ejecución.

El Consumer puede ser CLI, UI, API u otra superficie. La semántica del `.efn` no cambia por ello.

---

## 5. Functional Inputs

### 5.1 Compiled Program

Cardinalidad: exactamente `1`.

- representa un Evo-Script Program previamente compilado y validado;
- contiene bytecode;
- puede contener External Symbols aún no ligados;
- puede reutilizarse en múltiples invocaciones independientes;
- no contiene estado local mutable de una ejecución;
- no contiene `Active Scope` ni estado de Host.

```text
Compiled Program != Source Text
Compiled Program != File Path
Compiled Program != public AST
Compiled Program != public Token Sequence
```

`Execute Compiled` no recibe Source Text y no recompila.

### 5.2 Invocation Values

Cardinalidad: `0..N Value` ordenados.

- satisfacen los Parameters del entry point conforme a Evo-Script;
- binding estrictamente posicional;
- aridad exacta;
- compatibilidad semántica exacta;
- sin coerciones implícitas;
- `Invocation Values != Command-Line Strings`;
- no transportan prompt, Scope ni estado interactivo del Consumer.

---

## 6. Functional Output

Toda invocación concluida produce exactamente un `Result`.

```text
Result
├── Success
│      └── Value
└── Failure
       └── Failure
```

- `Result != Value`;
- `Result != Failure`;
- un success preserva el Value producido por el programa;
- un failure expresa Failure;
- el Result concluye únicamente la invocación actual;
- presentación, renderizado o serialización del Result pertenecen al Consumer.

---

## 7. Entry Point and Invocation Binding

`Execute Compiled` ejecuta el entry point determinado por Evo-Script.

Si el entry point declara `N Parameters`, deben suministrarse exactamente `N Invocation Values`:

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

Un desajuste de aridad o tipo produce `Result.failure` antes de comenzar una ejecución válida del entry point.

---

## 8. Bytecode Execution

```text
Compiled Program
      │
      ▼
   Bytecode
      │
      ▼
Local Execution State
      │
      ▼
    Result
```

La representación ejecutable es bytecode. La arquitectura concreta de la VM pertenece al Technical Design.

---

## 9. Local Execution State

Cada invocación posee estado local independiente únicamente para evaluar el bytecode.

Conceptualmente puede incluir:

```text
Local Execution State
├── Pipeline Data
├── function / frame evaluation state
└── temporary values required by execution
```

No incluye:

```text
Active Scope
Host Prompt
CLI/UI/API Session State
Current Provider
```

Esto no introduce ni exige un objeto técnico `ExecutionContext` o `Session`.

### Independence

```text
Compiled Program P
       │
 ┌─────┼─────┐
 ▼     ▼     ▼
Exec A Exec B Exec C
 │      │      │
State A State B State C
 │      │      │
Result A Result B Result C
```

- una ejecución no contamina otra;
- una ejecución fallida no degrada el Compiled Program;
- el estado local termina al concluir la invocación.

---

## 10. Pipeline Data

`Pipeline Data` representa el dato o flujo semántico transportado por la composición del programa.

```text
Pipeline Data
    ↓
operation
    ↓
Pipeline Data
```

Invariantes:

- dentro de `.efn`, Pipeline es composición de datos;
- no existe un canal paralelo `Active Scope`;
- Pipeline Data no representa estado de Host;
- `this` puede participar sintácticamente como marcador del transported value conforme a Evo-Script.

---

## 11. External Symbols and External Capabilities

Un `Compiled Program` puede conservar External Symbols durante Compile.

```text
Bytecode
   │
   ▼
External Symbol
   │
   ▼
Explicit Application Binding
   │
   ▼
External Capability
   │
   ├── available  ──► continue execution
   └── unavailable ─► Result Failure
```

Invariantes:

- el Engine no descubre Providers;
- no utiliza global registries, Service Locator o reflection;
- bytecode no almacena function pointers físicos de Providers;
- no existe un Provider activo seleccionado mediante `use` o Scope;
- distintas capabilities pueden satisfacerse explícitamente durante una misma ejecución;
- la disponibilidad de una capability no se exige durante Compile;
- la ausencia de una capability requerida cuando se alcanza produce Failure.

---

## 12. `.efn` / Host Boundary

La ejecución no recibe ni hereda `Active Scope` de un Host.

```text
Host / Consumer state
        ╳ no implicit inheritance
Execute Compiled
```

`Scope`, prompt y navegación contextual pertenecen a la Interactive Host Session cuando exista.

`use` no es una construcción válida dentro de `.efn`.

Una aplicación puede invocar la misma `.efn` desde CLI, UI o API y programar externamente cómo reaccionar al Result. Requesters y adapters se definirán en fases técnicas si las signatures los requieren; no forman parte de la semántica `.efn`.

---

## 13. Failure Flows

Todo fallo concluido se expresa mediante `Result Failure`.

Puede originarse por:

- mismatch de Invocation Values;
- fallo de evaluación definido por Evo-Script;
- External Symbol requerido sin capability explícita disponible;
- fallo propagable de una External Capability.

Cuando el fallo corresponde a una ubicación del programa puede conservar `Source Location`; no se inventa una ubicación para fallos de frontera externos al Source Text.

---

## 14. Functional Invariants

1. El nombre canónico es `Execute Compiled`.
2. Recibe exactamente un `Compiled Program`.
3. Recibe `0..N Invocation Values`.
4. Produce exactamente un `Result`.
5. No recibe Source Text ni recompila.
6. Ejecuta el bytecode del Compiled Program.
7. Ejecuta el entry point determinado por Evo-Script.
8. Binding posicional y aridad exacta.
9. Sin coerciones implícitas.
10. El Compiled Program puede contener External Symbols.
11. External Symbols se satisfacen durante ejecución mediante capabilities explícitas.
12. El Engine no descubre Providers.
13. No existe Provider activo ambiental.
14. Cada ejecución posee estado local independiente.
15. El estado local no contiene Active Scope.
16. Pipeline Data representa composición de datos, no contexto de Host.
17. `use` no forma parte de `.efn`.
18. El Compiled Program es reutilizable.
19. El Consumer no necesita sesión persistente del Engine.
20. CLI, UI o API no alteran la semántica de la función ejecutada.
21. El Engine no realiza presentación por el Consumer.
22. Este nivel no define todavía Participants ni Rust Signatures.

---

## 15. Out of Scope

- lectura o resolución física de `.efn`;
- compilación de Source Text;
- persistencia/caché/serialización física de Compiled Programs;
- ciclo de vida de Evo Applications;
- presentación terminal/UI/HTTP;
- Interactive Host Scope y prompt;
- descubrimiento automático de Providers;
- arquitectura concreta de la VM;
- Rust representation y Participants.

---

## 16. Signature Preparation

```text
Use Case
    Execute Compiled

Consumes
    Compiled Program
    Invocation Values

Produces
    Result

May require during execution
    External Capabilities
        as required by External Symbols

Execution-local semantics
    Pipeline Data
    evaluation state

Explicitly absent
    Active Scope
    Host Session State
```

## Closure

`UC-002 — Execute Compiled` queda revalidado y `FUNCTIONAL CLOSED` bajo la frontera `.efn` / Host vigente.
