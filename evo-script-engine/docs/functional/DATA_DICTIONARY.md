# Evo-Script Engine v0 — Functional Data Dictionary

Status: REVALIDATED — FUNCTIONAL CLOSED

Este documento define el vocabulario funcional canónico necesario para expresar las User Stories y Functional Use Cases públicos de `evo-script-engine` v0.

La frontera `.efn` / Host se rige por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.

## Governing Rule

Todo dato o concepto necesario para expresar una User Story o Functional Use Case debe estar definido previamente en este Functional Data Dictionary.

Este nivel define significado, naturaleza, cardinalidad, relaciones e invariantes funcionales. No define structs/enums Rust, ownership, borrowing, lifetimes, containers, function pointers ni Participants técnicos.

---

## 1. Boundary Data

### Source Text

- **Functional Nature**: Textual Value / Boundary Input.
- **Definition**: Texto completo de una unidad de programa Evo-Script suministrado al Engine.
- **Represents**: `Evo-Script Program`.
- **Used By**: `Compile` y `Execute Source`, exactamente 1 por invocación.
- **Invariants**:
  - `Source Text != File Path`.
  - `Source Text != AST`.
  - `Source Text != Token Sequence`.
  - `Source Text != Compiled Program`.
  - el Engine recibe texto y no lee físicamente un `.efn` desde filesystem.

### Compiled Program

- **Functional Nature**: Executable Artifact / Boundary Output and Input.
- **Definition**: Artifact producido por compilación exitosa que representa un Evo-Script Program validado y preparado para ejecución.
- **Executable Representation**: Bytecode.
- **Produced By**: `Compile` success.
- **Consumed By**: `Execute Compiled`.
- **Conceptually Preserves Enough Information To**:
  - ejecutar el programa;
  - determinar el entry point conforme a Evo-Script;
  - conocer Parameters requeridos por invocación;
  - conservar External Symbols;
  - producir diagnostics vinculables al Source Text cuando corresponda.
- **Invariants**:
  - puede reutilizarse en múltiples ejecuciones independientes;
  - no contiene estado local de una ejecución;
  - no contiene `Active Scope` ni estado interactivo de Host;
  - no contiene direcciones físicas de function pointers de una aplicación o Provider;
  - persistencia, caché y serialización física no son responsabilidad del Engine.

### Invocation Values

- **Functional Nature**: Ordered Value Sequence / Boundary Input.
- **Definition**: Secuencia ordenada de `0..N Value` suministrada por el Consumer para satisfacer los Parameters del entry point.
- **Used By**: `Execute Compiled` y `Execute Source`.
- **Not Used By**: `Compile`.
- **Mapping Rule**:

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

- **Invariants**:
  - binding estrictamente posicional;
  - aridad exacta;
  - compatibilidad semántica exacta conforme a Evo-Script;
  - sin coerciones implícitas;
  - `Invocation Values != Command-Line Strings`;
  - no transportan estado interactivo de Host.

---

## 2. Outcome Data

### Compile Outcome

- **Functional Nature**: Compilation Outcome.
- **Conceptual Shape**:

```text
Compile Outcome
├── Success
│   └── Compiled Program
└── Failure
    └── Failure
```

- una compilación fallida no produce un `Compiled Program` válido;
- este nivel no prescribe todavía su representación Rust.

### Result

- **Functional Nature**: Execution Outcome.
- **Produced By**: `Execute Compiled` y `Execute Source`.
- **Conceptual Shape**:

```text
Result
├── Success
│   └── Value
└── Failure
    └── Failure
```

- `Result != Value`;
- `Result != Failure`;
- el Result es semántico y neutral respecto de CLI, UI, API u otro Consumer;
- presentación, impresión, renderizado y serialización pertenecen al Consumer.

### Failure

- **Functional Nature**: Failure Data.
- **May Originate In**: compilación, frontera de invocación, evaluación runtime o invocación de External Capability.
- **Conceptually Contains**: descripción del fallo y `0..1 Source Location` cuando corresponde.
- **Invariants**:
  - las variantes técnicas concretas se definen posteriormente;
  - no se inventa Source Location para fallos que no pertenecen al Source Text.

### Source Location

- **Functional Nature**: Structured Diagnostic Data.
- **Definition**: Ubicación funcional dentro del Source Text asociada a un diagnostic.
- **Minimum v0 Data**: `line`, 1-based.
- **Cardinality In Failure**: `0..1`.
- Column, span y byte offsets pueden existir técnicamente cuando el Technical Data Model los requiera.

---

## 3. Language Domain Data

### Evo-Script Program

- **Functional Nature**: Language Domain Concept.
- **Definition**: Unidad completa de programa definida por la especificación vigente de Evo-Script y sus amendments normativos.
- `Source Text` la representa textualmente.
- `Compiled Program` la representa ejecutablemente después de compilación exitosa.
- Para `.efn`, no incluye `Active Scope`, `use` ni estado interactivo de Host.

### Public Function

- **Functional Nature**: Language Domain Concept.
- **Definition**: Función públicamente invocable conforme a Evo-Script.
- puede declarar `0..N Parameters`;
- el Engine utiliza la semántica vigente para determinar el entry point;
- `Public Function != OS main` y `Public Function != evo-runtime Run`.

### Parameter

- **Functional Nature**: Structured Language Data.
- **Definition**: Parámetro formal tipado de una Function.
- **Functionally Relevant Data**: posición y tipo declarado.
- cada Parameter recibe exactamente un Invocation Value en una invocación válida.

---

## 4. Execution Semantic Data

### External Symbol

- **Functional Nature**: Semantic Reference.
- **Definition**: Referencia conservada por un Compiled Program a una capacidad externa que puede requerirse durante ejecución y cuya implementación concreta no se liga durante Compile.

```text
Compiled Program
    └── External Symbol
            │ execution
            ▼
      explicit binding
            │
            ▼
    External Capability
```

- puede permanecer sin resolver después de Compile;
- se resuelve durante ejecución cuando se alcanza;
- no contiene function pointer físico, Provider concreto ni registry entry;
- debe conservar suficiente identidad semántica para solicitar la capacidad correcta.

### External Capability

- **Functional Nature**: Execution Dependency Concept.
- **Definition**: Capacidad explícitamente suministrada por la composición de la aplicación para satisfacer un External Symbol.
- **Kinds At This Level**: Standard Evolution Capability o Provider-specific Extension.
- **Invariants**:
  - el Engine no descubre External Capabilities;
  - no existe un Provider activo ambiental dentro de `.efn`;
  - una misma `.efn` puede requerir distintas capabilities explícitas;
  - la ausencia de una capability alcanzada produce Failure;
  - Contracts, Requesters, bindings concretos y function pointers pertenecen al Technical Design posterior.

### Pipeline Data

- **Functional Nature**: Execution Semantic Data.
- **Definition**: Dato o flujo de datos transportado entre operaciones compuestas durante una ejecución Evo-Script.
- **Invariant Principal**:

```text
.efn Pipeline
    = data composition
```

- `Pipeline Data` no representa `Scope`, prompt ni estado del Host;
- dentro de `.efn` no existe un canal paralelo `Active Scope`;
- `this` puede representar sintácticamente la posición del Pipeline Data transportado, conforme a Evo-Script;
- la representación técnica no obliga a crear un contexto global o paquete artificial.

---

## 5. Referenced Shared Concepts

### Value

- **Functional Nature**: Shared Value Concept.
- **Owner**: `evo-values`.
- `Invocation Values` contiene `0..N Value`;
- un Result exitoso preserva un Value;
- Pipeline Data puede transportar datos representables por la semántica compartida del ecosistema;
- `evo-script-engine` no redefine `Value`.

### Scope — Explicitly Outside `.efn`

- **Semantic Owner**: `evo-shell` / Host interactivo.
- **Engine Use In `.efn`**: none.
- `Scope` y `Active Scope` no forman parte del estado funcional de `Compile`, `Execute Compiled` o `Execute Source`.
- un Host interactivo puede conservar Active Scope para su propia experiencia de comandos;
- ese estado no se hereda ni se inyecta implícitamente en el Engine.

---

## 6. Consumer Neutrality

El `Consumer` solicita una Public Capability y recibe su outcome, pero no se convierte en estado del `.efn`.

```text
                    Result
                      │
          ┌───────────┼───────────┐
          ▼           ▼           ▼
       evo-cli      evo-ui      evo-api
```

- un mismo programa puede utilizarse desde diferentes Consumers;
- el `.efn` no conoce la presentación específica;
- Requesters, adapters o mecanismos equivalentes para reaccionar al dato pertenecen a la composición arquitectónica exterior y no a este Functional Data Dictionary salvo que una futura User Story los incorpore explícitamente.

---

## 7. Canonical Relationships

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
    ├── semantically includes ────────────────────► Compile + Execute Compiled
    ├── may require ──────────────────────────────► External Capability
    └── produces ─────────────────────────────────► Result

Interactive Host Scope
    └── outside `.efn` Engine execution boundary
```

## Closure

El Functional Data Dictionary queda revalidado y `FUNCTIONAL CLOSED` bajo la regla:

```text
.efn execution state
    does not contain Active Scope
```

Reintroducir Scope o estado de Host dentro de `.efn` requiere reabrir este cierre y `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.
