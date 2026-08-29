# UC-002 — Execute Compiled

Status: REVALIDATED — FUNCTIONAL CLOSED

---

## 1. Purpose

Definir funcionalmente cómo un `Consumer` solicita a `evo-script-engine` ejecutar un `Compiled Program` existente, proporcionando los `Invocation Values` requeridos para esa ejecución y obteniendo un `Result`.

La operación canónica es:

```text
Compiled Program + Invocation Values
              │
              ▼
       Execute Compiled
              │
              ▼
            Result
```

`Execute Compiled` es el nombre semántico canónico del Use Case y debe conservarse al derivar el diseño técnico y la Rust Signature correspondiente.

---

## 2. Traceability

- **Deriva de**: [`US-002 — Execute Compiled Evo-Script Program`](../user-stories/US-002-execute-compiled-evo-script-program.md)
- **Utiliza conceptos de**: [`Functional Data Dictionary`](../DATA_DICTIONARY.md)
- **Aplica normativamente**: [`Evo-Script Language Specification v0`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md)
- **Corresponde a Public Capability**: `Execute Compiled`

---

## 3. Functional Operation

La frontera funcional pública se expresa como:

```text
Execute Compiled(
    Compiled Program,
    Invocation Values
) -> Result
```

Esta notación expresa la necesidad funcional del Consumer. No fija todavía una Rust Signature ni decide ownership, borrowing, lifetimes, Requesters, Contracts, Resolvers, Providers, Collaborators o representación técnica de las dependencias externas.

Cuando el `Compiled Program` requiere `External Capabilities`, dichas capacidades constituyen dependencias funcionales de la ejecución y deben estar disponibles mediante bindings explícitamente suministrados por la composición de la aplicación. Su forma técnica se decide posteriormente.

---

## 4. Consumer

El `Consumer` es el rol funcional externo que solicita `Execute Compiled`.

Responsabilidades funcionales:

- suministrar exactamente un `Compiled Program`;
- suministrar `0..N Invocation Values`;
- haber compuesto explícitamente las capacidades externas necesarias cuando el programa las requiera;
- recibir exactamente un `Result` al concluir la invocación.

El Consumer no necesita crear ni administrar una sesión persistente del Engine.

---

## 5. Functional Inputs

### 5.1 Compiled Program

Cardinalidad: exactamente `1`.

El `Compiled Program`:

- representa un Evo-Script Program previamente compilado y validado;
- contiene bytecode como representación ejecutable;
- puede contener `External Symbols` todavía no ligados a una implementación concreta;
- puede reutilizarse en múltiples invocaciones independientes;
- no contiene el estado local mutable de una ejecución concreta.

Distinciones de frontera:

```text
Compiled Program != Source Text
Compiled Program != File Path
Compiled Program != AST público
Compiled Program != Token Stream público
```

`Execute Compiled` no recibe Source Text y no recompila el programa.

### 5.2 Invocation Values

Cardinalidad: `0..N Values` ordenados.

Los `Invocation Values` satisfacen los `Parameters` del entry point determinado conforme a las reglas vigentes de Evo-Script.

Invariantes:

- el binding es estrictamente posicional;
- la aridad debe coincidir exactamente;
- cada Value debe ser semánticamente compatible con su Parameter correspondiente;
- no existen coerciones o conversiones implícitas para reparar incompatibilidades;
- `Invocation Values != Command-Line Strings`.

---

## 6. Functional Output

Toda invocación concluida produce exactamente un `Result`.

```text
Result
├── Success
│      └── Value
│
└── Failure
       └── Failure
```

Invariantes:

- `Result != Value`;
- `Result != Failure`;
- una ejecución exitosa preserva el Value producido por el programa;
- una ejecución fallida expresa un Failure;
- el `Result` concluye únicamente la invocación actual y no invalida el `Compiled Program`.

---

## 7. Entry Point and Invocation Binding

`Execute Compiled` ejecuta el entry point determinado conforme a la semántica vigente de Evo-Script.

La regla que determina cuántas Public Functions puede declarar un programa y cómo se selecciona el entry point pertenece a `evo-script`; este Use Case no la redefine.

### Exact Arity

Si el entry point declara `N Parameters`, la invocación debe proporcionar exactamente `N Invocation Values`.

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

Una diferencia de aridad produce un `Result` fallido antes de comenzar la ejecución válida del entry point.

### Semantic Compatibility

Cada Invocation Value debe ser compatible con el tipo declarado de su Parameter correspondiente conforme a Evo-Script.

Una incompatibilidad produce un `Result` fallido. El Engine no realiza coerciones implícitas para adaptar el Value.

---

## 8. Bytecode Execution

Una vez validada la frontera de invocación, `Execute Compiled` ejecuta el bytecode contenido por el `Compiled Program` conforme a la semántica de Evo-Script.

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

Que la representación ejecutable sea bytecode está cerrado funcionalmente.

La arquitectura técnica de la VM permanece abierta para la fase de Technical Design. Este Use Case no decide Stack VM, Register VM, frames, instruction pointer, dispatch, layouts de memoria ni otras estructuras internas.

---

## 9. Local Execution State

Cada invocación de `Execute Compiled` posee estado local independiente durante su ejecución.

Ese estado puede incluir conceptualmente todo lo necesario para evaluar el bytecode, y debe respetar al menos las siguientes semánticas funcionales:

```text
Local Execution State
├── Active Scope
├── Pipeline Data
└── evaluation state requerido por la ejecución
```

Esto no introduce ni exige un objeto técnico llamado `ExecutionContext` o `Session`.

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

Invariantes:

- una ejecución no contamina el estado local de otra;
- una ejecución fallida no degrada el `Compiled Program`;
- una invocación no consume ni invalida funcionalmente el artefacto compilado;
- el estado local termina cuando concluye la invocación.

---

## 10. Active Scope

Cada invocación inicia sin heredar implícitamente un `Active Scope` externo.

```text
Execute Compiled starts
        │
        ▼
Active Scope = none
```

El programa puede establecer o cambiar su `Active Scope` conforme a la semántica de Evo-Script, por ejemplo mediante la semántica de `use`.

El `Active Scope`:

- pertenece a la ejecución actual;
- no pertenece al `Compiled Program`;
- no se hereda automáticamente desde CLI, UI, otra ejecución o una ejecución previa;
- puede cambiar durante la invocación;
- termina junto con el estado local de la ejecución.

---

## 11. Pipeline Data

`Pipeline Data` representa el dato o flujo semántico transportado durante la evaluación del programa.

Regla canónica:

```text
Pipeline Data != Active Scope
```

Ambos constituyen canales semánticos independientes.

Cambiar el `Active Scope` no destruye, reemplaza ni redefine automáticamente el `Pipeline Data` existente.

Este Use Case no decide todavía la representación Rust de ninguno de los dos conceptos.

---

## 12. External Symbols and External Capabilities

Un `Compiled Program` puede contener `External Symbols` preservados durante `Compile`.

Cuando la ejecución alcanza un `External Symbol`, debe intentar satisfacerlo mediante una `External Capability` disponible a través de un binding explícito de la aplicación.

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

- el Engine no descubre Providers concretos;
- el Engine no utiliza registries globales, service locators o reflexión para resolver capacidades;
- el bytecode no almacena direcciones físicas de function pointers de Providers;
- la selección concreta de capacidad pertenece a la composición explícita de la aplicación;
- la ausencia de una capacidad requerida produce un `Result` fallido durante ejecución;
- la disponibilidad de una capacidad no se exige durante `Compile`.

La forma técnica mediante la cual la futura Rust Signature recibirá o alcanzará estas capacidades se decide en Technical Design.

---

## 13. Failure Flows

Todo fallo concluido se representa mediante `Result Failure` que expresa un `Failure`.

### 13.1 Invocation Boundary Failure

Puede originarse antes de la ejecución válida del entry point, por ejemplo:

- desajuste de aridad;
- incompatibilidad semántica entre Invocation Value y Parameter.

No se inventa una `Source Location` cuando el fallo no corresponde a una ubicación del Source Text.

### 13.2 Program Execution Failure

Puede originarse durante la evaluación del bytecode conforme a la semántica de Evo-Script.

Cuando el fallo corresponde a una ubicación del programa, el `Failure` puede incluir una `Source Location` preservada por el `Compiled Program`.

### 13.3 External Capability Failure

Puede originarse cuando un `External Symbol` requerido no puede ser satisfecho mediante las capacidades explícitamente disponibles para la ejecución o cuando la interacción externa concluye con un fallo semánticamente propagable.

El conjunto concreto de categorías, códigos y variantes de Failure permanece abierto hasta el diseño técnico correspondiente.

---

## 14. Functional Invariants

1. El nombre canónico del Use Case es `Execute Compiled`.
2. Recibe exactamente un `Compiled Program`.
3. Recibe `0..N Invocation Values`.
4. Produce exactamente un `Result`.
5. No recibe `Source Text`.
6. No realiza una nueva compilación del programa.
7. Ejecuta el bytecode del `Compiled Program`.
8. Ejecuta el entry point determinado conforme a Evo-Script.
9. El binding de Invocation Values es estrictamente posicional.
10. La aridad debe coincidir exactamente.
11. Los Values deben ser semánticamente compatibles con sus Parameters.
12. No existen conversiones implícitas para reparar incompatibilidades.
13. El `Compiled Program` puede contener `External Symbols`.
14. Los External Symbols se satisfacen durante ejecución mediante capacidades explícitamente disponibles.
15. El Engine no descubre Providers concretos ni bindings mediante mecanismos ocultos.
16. Una capacidad externa requerida que no está disponible produce un `Result` fallido.
17. Cada ejecución posee estado local independiente.
18. Cada ejecución inicia sin `Active Scope` heredado implícitamente.
19. El programa puede modificar su `Active Scope` conforme a Evo-Script.
20. `Pipeline Data != Active Scope`.
21. Una ejecución no contamina el estado local de otra ejecución.
22. El `Compiled Program` es reutilizable.
23. Un fallo no invalida funcionalmente el `Compiled Program`.
24. El Consumer no necesita crear una sesión persistente del Engine.
25. El Engine no realiza filesystem I/O como responsabilidad propia de este Use Case.
26. El Engine no realiza presentación de terminal, UI o stdout como responsabilidad propia de este Use Case.
27. Este nivel no define todavía Requesters, Contracts, Resolvers, Providers, Collaborators, VM internals ni Rust Signatures.

---

## 15. Out of Scope

Queda fuera de UC-002:

- lectura o resolución de archivos `.efn`;
- compilación de Source Text;
- persistencia, caché, carga o serialización física de Compiled Programs;
- ciclo de vida de aplicaciones de `evo-runtime`;
- presentación de terminal, UI o stdout;
- descubrimiento automático de Providers;
- registries globales y service locators;
- representación Rust de `Compiled Program`, Invocation Values, Result, Active Scope o Pipeline Data;
- definición de function pointers, Requesters, Contracts, Resolvers, Providers o demás Participants;
- arquitectura interna concreta de la VM.

---

## 16. Signature Preparation

Este Use Case deja preparado el siguiente conocimiento funcional para la futura Rust Signature:

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
    Active Scope
    Pipeline Data
    evaluation state
```

La siguiente fase técnica deberá determinar cuáles de estos conceptos cruzan directamente la firma, cuáles son materializados internamente y qué Participants son necesarios para cumplir la operación sin introducir dependencias ocultas.
