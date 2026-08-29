# Evo-Script Engine v0 — Functional Data Dictionary

Status: REVALIDATED — FUNCTIONAL CLOSED

Este documento define el vocabulario funcional canónico de datos y conceptos necesarios para expresar las User Stories y los Functional Use Cases de `evo-script-engine` v0.

Su objetivo no es servir únicamente como glosario. Cada definición debe contener suficiente precisión semántica para que, en la etapa técnica, el Technical Lead pueda determinar representaciones, ownership, lifetimes y Rust Signatures sin reinventar el significado funcional de los datos.

## Governing Rule

Todo dato o concepto necesario para expresar una User Story o un Functional Use Case debe estar definido previamente en este Functional Data Dictionary.

El Functional Data Dictionary define:

- qué representa cada concepto;
- su naturaleza funcional;
- cardinalidades relevantes;
- relaciones con otros conceptos;
- invariantes funcionales;
- qué parte pertenece al Engine y qué parte pertenece a otro componente del ecosistema.

Deliberadamente no define:

- `struct`, `enum`, alias o representación Rust concreta;
- ownership o borrowing;
- lifetimes;
- slices, `Vec`, `String`, `&str` u otros contenedores concretos;
- function pointers;
- Agents, Requesters, Collaborators, Resolvers, Contracts o Tools;
- Tokens, AST, Instructions, Opcodes, Stack Frames u otros datos internos del motor.

Esas decisiones pertenecen al Technical Design y al Technical Data Model posterior.

---

## 1. Boundary Data

### Source Text

- **Functional Nature**: Textual Value / Boundary Input.
- **Definition**: Texto completo de una unidad de programa Evo-Script suministrado al Evo-Script Engine.
- **Represents**: `Evo-Script Program`.
- **Used By**:
  - `Compile`: exactamente 1 por invocación.
  - `Execute Source`: exactamente 1 por invocación.
- **Invariants**:
  - `Source Text != File Path`.
  - `Source Text != AST`.
  - `Source Text != Token Stream`.
  - `Source Text != Compiled Program`.
  - `Source Text != Individual Function`.
  - El Engine recibe texto; no lee el archivo físico `.efn` ni resuelve su ruta.
  - La obtención del Source Text desde filesystem u otro medio pertenece al Consumer o a otro componente.
- **Ownership Note**: La forma técnica de transportar el texto queda diferida al Technical Data Model.
- **Sources**: US-001, US-003, `evo-script`.

### Compiled Program

- **Functional Nature**: Executable Artifact / Boundary Output and Input.
- **Definition**: Artefacto producido por una compilación exitosa que representa un Evo-Script Program validado y preparado para ejecución por `evo-script-engine`.
- **Produced By**: `Compile` en el success path.
- **Consumed By**: `Execute Compiled`, exactamente 1 por invocación.
- **Executable Representation**: Bytecode.
- **Conceptually Preserves Enough Information To**:
  - ejecutar el programa mediante el mecanismo de ejecución del Engine;
  - determinar el entry point conforme a las reglas vigentes de Evo-Script;
  - conocer los Parameters necesarios para validar Invocation Values;
  - conservar `External Symbols` requeridos por el programa;
  - producir diagnostics relacionados con Source Text cuando corresponda.
- **Invariants**:
  - `Compiled Program != Source Text`.
  - Su representación ejecutable no está abierta entre AST, IR o tree-walk: en v0 es bytecode.
  - Puede reutilizarse en múltiples invocaciones independientes de `Execute Compiled`.
  - El estado local de una ejecución no pertenece al Compiled Program.
  - No contiene direcciones físicas de function pointers suministrados por una aplicación o Provider.
  - La persistencia, caché o serialización del artefacto no son responsabilidad del Engine.
- **Sources**: US-001, US-002, Purpose, Public Capabilities.

### Invocation Values

- **Functional Nature**: Ordered Value Sequence / Boundary Input.
- **Definition**: Secuencia ordenada de `0..N Value` suministrada por el Consumer para satisfacer los Parameters del entry point determinado conforme a Evo-Script.
- **Used By**:
  - `Execute Compiled`.
  - `Execute Source`.
- **Not Used By**: `Compile`.
- **Cardinality**: `0..N Value`.
- **Mapping Rule**:

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

- **Invariants**:
  - El binding es estrictamente posicional.
  - La aridad debe coincidir exactamente con los Parameters requeridos.
  - Cada Value debe ser semánticamente compatible con el tipo declarado del Parameter correspondiente.
  - El Engine no realiza coerciones implícitas para reparar incompatibilidades.
  - `Invocation Values != Command-Line Strings`.
  - La representación concreta como slice, colección u otro contenedor queda diferida al Technical Data Model.
- **Sources**: US-002, US-003, `evo-script`, `evo-values`.

---

## 2. Outcome Data

### Compile Outcome

- **Functional Nature**: Compilation Outcome.
- **Definition**: Resultado funcional completo de una invocación de `Compile`.
- **Conceptual Shape**:

```text
Compile Outcome
├── Success
│   └── Compiled Program
└── Failure
    └── Failure
```

- **Invariants**:
  - La notación abreviada `Source Text -> Compiled Program` describe únicamente el success path de `Compile`.
  - Una compilación fallida no produce un Compiled Program válido.
  - Este concepto no determina todavía si la Rust Signature utilizará un enum propio, un Result compartido, un Requester u otra representación técnica.
- **Sources**: US-001.

### Result

- **Functional Nature**: Execution Outcome.
- **Definition**: Outcome funcional completo producido por una operación de ejecución del Evo-Script Engine.
- **Produced By**:
  - `Execute Compiled`.
  - `Execute Source`.
- **Conceptual Shape**:

```text
Result
├── Success
│   └── Value
└── Failure
    └── Failure
```

- **Invariants**:
  - `Result != Value`.
  - `Result != Failure`.
  - Un success preserva el Value producido por la ejecución.
  - Un failure expresa un Failure.
  - El concepto se alinea con el modelo compartido de outcomes de `evo-values`, sin fijar todavía su representación Rust concreta.
- **Sources**: US-002, US-003, `evo-values`.

### Failure

- **Functional Nature**: Failure Data.
- **Definition**: Información semántica que explica por qué una operación del Engine no pudo concluir exitosamente.
- **May Originate In**:
  - compilación;
  - frontera de invocación;
  - evaluación en runtime;
  - resolución de una capacidad externa requerida.
- **Conceptually Contains**:
  - una descripción del fallo;
  - `0..1 Source Location` cuando el fallo puede asociarse a una ubicación del Source Text.
- **Invariants**:
  - Las categorías técnicas y variantes concretas de Failure no se congelan en este nivel.
  - No se decide todavía un enum de errores.
  - No se exige en v0 stack trace, severity, numeric code, byte offset o span completo.
  - Un desajuste de aridad o incompatibilidad de Invocation Values puede producir Failure sin Source Location.
  - Un fallo originado directamente en una construcción del programa puede preservar Source Location cuando exista.
- **Sources**: US-001, US-002, US-003.

### Source Location

- **Functional Nature**: Structured Diagnostic Data.
- **Definition**: Ubicación dentro del Source Text asociada a un diagnostic funcional.
- **Minimum v0 Data**:
  - `line`: número de línea 1-based.
- **Cardinality In Failure**: `0..1`.
- **Invariants**:
  - La primera línea del Source Text es `line 1`.
  - No se crean ubicaciones artificiales como `line 0` cuando el fallo no pertenece a una línea fuente.
  - Column, span y byte offset pueden evaluarse posteriormente si el diseño técnico o la especificación los requieren.
- **Sources**: US-001, US-002, US-003.

---

## 3. Language Domain Data

Los siguientes conceptos pertenecen normativamente a `evo-script`. Este documento los referencia porque participan en compilación o ejecución, pero no redefine sus reglas de lenguaje.

### Evo-Script Program

- **Functional Nature**: Language Domain Concept.
- **Definition**: Unidad completa de programa definida por la especificación vigente de Evo-Script.
- **Relations**:
  - `Source Text` representa textualmente un Evo-Script Program.
  - `Compiled Program` representa ejecutablemente un Evo-Script Program compilado exitosamente.
- **Invariant**: La cantidad de Public Functions, reglas de módulos, funciones privadas, structs, enums y demás estructura del programa pertenecen exclusivamente a `evo-script`.
- **Sources**: `evo-script`.

### Public Function

- **Functional Nature**: Language Domain Concept.
- **Definition**: Función públicamente invocable conforme a las reglas vigentes de Evo-Script.
- **Relations**:
  - puede declarar `0..N Parameters` conforme a la especificación del lenguaje;
  - el Engine utiliza la semántica de Evo-Script para determinar el entry point que corresponde ejecutar.
- **Invariants**:
  - Este Data Dictionary no establece que deba existir exactamente una Public Function.
  - La selección del entry point pertenece a `evo-script`.
  - `Public Function != OS main`.
  - `Public Function != evo-runtime Run`.
- **Sources**: `evo-script`, US-001, US-002, US-003.

### Parameter

- **Functional Nature**: Structured Language Data.
- **Definition**: Parámetro formal tipado perteneciente a la firma de una Public Function conforme a Evo-Script.
- **Functionally Relevant Data**:
  - posición declarada;
  - tipo declarado.
- **Relations**:
  - cada Parameter requerido recibe exactamente un Invocation Value en una ejecución válida;
  - el binding se realiza por posición.
- **Invariant**: Nombre, mutabilidad, optionality, defaults y otras reglas pertenecen a `evo-script`; este diccionario solo preserva los datos funcionalmente necesarios para las historias actuales.
- **Sources**: `evo-script`, US-002, US-003.

---

## 4. Execution Semantic Data

### External Symbol

- **Functional Nature**: Semantic Reference.
- **Definition**: Referencia conservada por un Compiled Program a una capacidad externa que el programa puede requerir durante ejecución y cuya implementación concreta no se liga durante Compile.
- **Relations**:

```text
Compiled Program
    └── External Symbol
            │
            │ execution
            ▼
      explicit binding
            │
            ▼
    External Capability
```

- **Invariants**:
  - Puede permanecer sin resolver después de Compile.
  - Se resuelve únicamente durante ejecución cuando la operación es alcanzada.
  - No contiene una dirección física de function pointer.
  - No contiene una instancia concreta de Provider.
  - No representa una entrada de registry global.
  - Debe preservar suficiente identidad semántica para solicitar la capacidad correcta durante ejecución.
- **Sources**: US-001, US-002, US-003, Purpose.

### External Capability

- **Functional Nature**: Execution Dependency Concept.
- **Definition**: Capacidad externa explícitamente suministrada por la composición de la aplicación que permite satisfacer un External Symbol durante una ejecución.
- **Kinds At This Level**:
  - Standard Evolution Capability.
  - Provider-specific Extension.
- **Invariants**:
  - El Engine no descubre External Capabilities.
  - El Engine no selecciona Providers concretos mediante mecanismos ocultos.
  - La capacidad necesaria debe llegar mediante binding explícito.
  - La ausencia de una capacidad requerida produce Failure durante ejecución.
  - Contracts, Requesters, `Describe`, `Invoke` y function pointers concretos pertenecen al Technical Design.
- **Sources**: US-002, US-003, Purpose, architecture decisions.

### Active Scope

- **Functional Nature**: Execution Semantic State.
- **Definition**: Scope actualmente seleccionado dentro de una ejecución para operaciones que requieren contexto operacional.
- **Lifecycle**:

```text
execution begins
    ↓
Active Scope = none
    ↓
program may select/change Scope
    ↓
execution ends
    ↓
local Active Scope ends
```

- **Invariants**:
  - Pertenece a una ejecución, no al Compiled Program.
  - No se hereda implícitamente desde CLI, UI, otra ejecución o un estado global.
  - Puede cambiar durante la ejecución conforme a la semántica de Evo-Script.
  - No implica que el Engine posea el Provider o recurso externo representado por el Scope.
  - No se define todavía como campo de un `ExecutionContext` ni de ninguna otra estructura Rust.
- **Sources**: US-002, US-003, Purpose.

### Pipeline Data

- **Functional Nature**: Execution Semantic Data.
- **Definition**: Dato o flujo de datos transportado entre operaciones compuestas durante una ejecución de Evo-Script.
- **Invariant Principal**:

```text
Pipeline Data != Active Scope
```

- **Consequences**:
  - cambiar Active Scope no elimina ni reemplaza automáticamente Pipeline Data;
  - Pipeline Data y Active Scope representan canales semánticos independientes;
  - su representación técnica no obliga a crear un objeto de contexto global o giant context.
- **Sources**: US-002, US-003, architecture decisions.

---

## 5. Referenced Shared Concepts

### Value

- **Functional Nature**: Shared Value Concept.
- **Owner**: `evo-values`.
- **Use In Engine**:
  - `Invocation Values` contiene `0..N Value`;
  - un `Result` exitoso preserva un Value producido por la ejecución;
  - Pipeline Data puede transportar datos representables mediante la semántica compartida del ecosistema.
- **Invariant**: `evo-script-engine` no redefine Value ni su representación interna.

### Scope

- **Functional Nature**: Shared Semantic Concept.
- **Semantic Owner**: `evo-shell`.
- **Use In Engine**: El Engine puede mantener conocimiento local de cuál Scope está activo durante una ejecución.
- **Invariants**:
  - el Engine no redefine la semántica de Scope;
  - el Engine no descubre el Provider asociado;
  - `Active Scope` es estado local de ejecución; `Scope` es el concepto semántico referenciado.

---

## 6. Canonical Relationships

```text
Source Text
    └── represents ───────────────────────────────► Evo-Script Program

Compile
    ├── consumes ─────────────────────────────────► Source Text
    └── produces ─────────────────────────────────► Compile Outcome
                                                      ├── Success → Compiled Program
                                                      └── Failure → Failure

Compiled Program
    ├── executable representation ────────────────► Bytecode
    ├── may preserve ─────────────────────────────► External Symbol 0..N
    ├── references entry-point semantics ─────────► Evo-Script
    └── reusable across independent executions

Invocation Values
    ├── contains ─────────────────────────────────► Value 0..N
    └── maps positionally ────────────────────────► Parameter 0..N

Execute Compiled
    ├── consumes ─────────────────────────────────► Compiled Program
    ├── consumes ─────────────────────────────────► Invocation Values
    ├── may require ──────────────────────────────► External Capability
    └── produces ─────────────────────────────────► Result

Execute Source
    ├── consumes ─────────────────────────────────► Source Text
    ├── consumes ─────────────────────────────────► Invocation Values
    ├── semantically performs ────────────────────► Compile + Execute Compiled
    └── produces ─────────────────────────────────► Result

Result
    ├── Success ──────────────────────────────────► Value
    └── Failure ──────────────────────────────────► Failure

Failure
    └── may reference ────────────────────────────► Source Location 0..1

Execution
    ├── local semantic state ─────────────────────► Active Scope
    └── transports ───────────────────────────────► Pipeline Data

External Symbol
    └── resolved at execution through explicit ──► External Capability
```

---

## 7. Signature Preparation Matrix

Esta matriz no define Rust Signatures. Su función es mostrar qué conceptos deberá considerar posteriormente el Technical Lead al derivarlas.

| Capability | Direct Boundary Data | Related Functional Concepts |
| --- | --- | --- |
| `Compile` | `Source Text`, `Compile Outcome` | `Compiled Program`, `Failure`, `Source Location`, `External Symbol` |
| `Execute Compiled` | `Compiled Program`, `Invocation Values`, `Result` | `Value`, `Failure`, `External Symbol`, `External Capability`, `Active Scope`, `Pipeline Data`, `Parameter` |
| `Execute Source` | `Source Text`, `Invocation Values`, `Result` | Todo lo requerido por `Compile` + `Execute Compiled` |

La presencia de un concepto en esta matriz no significa que necesariamente cruce la futura firma pública. Esa decisión pertenece al Technical Design. La matriz garantiza únicamente que el significado funcional de cada dato relevante esté definido antes de diseñar las firmas.

---

## Excluded From Functional Data Dictionary

Los siguientes conceptos son deliberadamente técnicos y no se definen aquí:

```text
Token
Token Kind
AST
AST Node
Instruction
Opcode
Instruction Pointer
Stack Frame
VM State
Lexer
Parser
Semantic Analyzer
Compiler
VM
```

Si son necesarios para implementar las historias cerradas, deberán definirse posteriormente en el Technical Design / Technical Data Model.

## Closure

Este Functional Data Dictionary se considera `REVALIDATED — FUNCTIONAL CLOSED` para `evo-script-engine` v0.

Los Functional Use Cases pueden utilizar estos conceptos, pero no redefinir su significado. Cualquier necesidad de ampliar o cambiar este vocabulario funcional debe reabrir explícitamente este artefacto antes de avanzar al diseño técnico.