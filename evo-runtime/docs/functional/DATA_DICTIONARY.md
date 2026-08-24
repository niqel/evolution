# Evo Runtime Model A — Data Dictionary

Status: FUNCTIONAL

This document consolidates the canonical architectural and functional vocabulary
derived from User Stories US-001 through US-014 for Evo Runtime Model A.

Model A Functional Coverage is closed (see
[MODEL_A_FUNCTIONAL_COVERAGE.md](MODEL_A_FUNCTIONAL_COVERAGE.md)).

This document defines canonical functional concepts, roles, requirements, and
outcomes. It does not define Rust structs, enums, traits, function pointers,
crates, or files. Different conceptual categories do not imply equivalent
technical representations. Technical Mapping remains a separate, subsequent
phase.

---

## 1. Core Components

### Evo Runtime

- **Category**: Core Component
- **Definition**: Plataforma del Core responsable de coordinar una Execution de
  una Evo Application.
- **Relationships**:
  - Inicia y coordina Executions solicitadas por un Host.
  - Resuelve Required Operations.
  - Hace disponibles las Implementations necesarias.
  - Coordina Invocation.
  - Determina Engines cuando corresponde.
  - Transporta Values a través de sus fronteras funcionales.
  - Propaga Failures.
  - Mantiene Execution Context.
  - Reconoce Execution Finalization.
  - Conoce EvoQ y EvoS como Engines base del Core.
  - Trabaja con Values y Outcomes sobre la base común de EvoV.
- **Invariants / Distinctions**:
  - `Evo Runtime != Engine`
  - `Evo Runtime != EvoQ`
  - `Evo Runtime != EvoS`
  - Evo Runtime coordina la ejecución general.
- **Sources**: US-001, US-002, US-003, US-004, US-005, US-006, US-007, US-008,
  US-009, US-010, US-011, US-012, US-013, US-014
- **Technical Mapping**: `evo-runtime`

### EvoV

- **Category**: Core Component
- **Definition**: Base semántica común de Values y Outcomes (Result y Failure)
  del Core de Evo.
- **Relationships**:
  - Proporciona la base semántica común utilizada por Evo Runtime.
  - Proporciona la base semántica común utilizada por EvoQ.
  - Proporciona la base semántica común utilizada por EvoS.
  - Proporciona la base común para Value, Result y Failure.
- **Invariants / Distinctions**:
  - `EvoV != Engine`
  - `EvoV != EvoQ`
  - `EvoV != EvoS`
  - Compartir la base de EvoV NO implica:
    - same memory address
    - physical copy
    - serialization
    - shared ownership representation
- **Sources**: US-011, US-013, US-014
- **Technical Mapping**: `evo-values`

### EvoQ

- **Category**: Core Component / Engine
- **Definition**: Engine base de consultas del Core de Evo.
- **Relationships**:
  - Es conocido por Evo Runtime.
  - Realiza Query Work.
  - Puede recibir Values.
  - Trabaja con Values y Outcomes sobre la base común de EvoV.
  - Puede producir un Value.
  - Puede producir un Failure.
  - El resultado vuelve a participar en la Execution a través de Evo Runtime.
- **Invariants / Distinctions**:
  - `EvoQ = Engine`
  - `EvoQ != Evo Runtime`
  - `EvoQ != Provider`
  - EvoQ no coordina la Execution global.
  - EvoQ no requiere discovery dinámico en Model A.
- **Sources**: US-011, US-013, US-014
- **Technical Mapping**: `evo-query-engine`

### EvoS

- **Category**: Core Component / Engine
- **Definition**: Engine base del Core capaz de ejecutar Implementations
  escritas en Evo-Script.
- **Relationships**:
  - Es conocido por Evo Runtime.
  - Ejecuta Evo-Script Implementations.
  - Puede recibir Values.
  - Trabaja con Values y Outcomes sobre la base común de EvoV.
  - Puede producir Value.
  - Puede producir Failure.
  - Las Required Operations transitivas continúan pasando por Evo Runtime.
- **Invariants / Distinctions**:
  - `EvoS = Engine`
  - `EvoS != Evo Runtime`
  - `EvoS != Evo-Script`
  - `EvoS != Provider`
  - EvoS no requiere discovery dinámico en Model A.
- **Sources**: US-011, US-012, US-013
- **Technical Mapping**: `evo-script-engine`

### Core

- **Category**: Architectural Concept
- **Definition**: Conjunto funcional base del Model A compuesto exactamente
  por:
  - Evo Runtime
  - EvoV
  - EvoQ
  - EvoS
- **Relationships**:
  - Agrupa los componentes base indispensables para la ejecución del Model A.
- **Invariants / Distinctions**:
  - Los cuatro componentes forman el Core estático.
  - Providers adicionales no forman parte automáticamente del Core.
  - La ausencia de Providers adicionales no elimina el Core.
  - Model A no requiere discovery dinámico para estos cuatro componentes.
- **Sources**: US-011, US-013, US-014, MODEL_A_FUNCTIONAL_COVERAGE.md
- **Technical Mapping**: Not defined yet.

---

## 2. Execution Concepts

### Host

- **Category**: External Role
- **Definition**: Unidad externa que solicita a Evo Runtime iniciar una Evo
  Application y recibe el Result final de la Execution.
- **Relationships**:
  - `Host -> requests -> Application Start`
  - `Host <- receives <- Result`
- **Invariants / Distinctions**:
  - El Host no necesita conocer la Implementation interna del Entry Point.
  - El Host no administra el ciclo de finalización interno ni destruye
    directamente el contexto interno del Runtime.
- **Sources**: US-001, US-009
- **Technical Mapping**: Not defined yet.

### Evo Application

- **Category**: Functional Concept
- **Definition**: Aplicación cuya ejecución es iniciada y coordinada mediante
  Evo Runtime.
- **Relationships**:
  - Declara un Entry Point.
  - Puede requerir Operations.
  - Participa en una Execution.
  - No necesita conocer las Implementations concretas.
  - No necesita seleccionar directamente Engines.
- **Invariants / Distinctions**:
  - Participa en la ejecución a través de Evo Runtime sin administrar la
    infraestructura técnica interna.
- **Sources**: US-001, US-002, US-003, US-004, US-005, US-006, US-007, US-008,
  US-009, US-010, US-011, US-012, US-013, US-014
- **Technical Mapping**: Not defined yet.

### Entry Point

- **Category**: Functional Concept
- **Definition**: Punto inicial declarado de una Evo Application desde el cual
  Evo Runtime comienza una Execution.
- **Relationships**:
  - Declarado por la Evo Application.
  - Utilizado por Evo Runtime para iniciar la Execution a petición del Host.
- **Invariants / Distinctions**:
  - Una Application dispone de un único Entry Point inicial declarado.
  - El Host no necesita conocer su Implementation interna.
  - Un Entry Point inválido impide iniciar correctamente la Execution.
- **Sources**: US-001
- **Technical Mapping**: Not defined yet.

### Execution

- **Category**: Functional Concept
- **Definition**: Trabajo iniciado y coordinado por Evo Runtime desde el Entry
  Point de una Evo Application hasta su finalización funcional.
- **Relationships**:
  - Es iniciada a solicitud de un Host.
  - Posee un Execution Context mientras está activa.
  - Puede contener múltiples Operations transitivas.
  - Puede transportar Values.
  - Puede producir Failures.
  - Concluye con un Result.
- **Invariants / Distinctions**:
  - Una Execution finalizada deja de estar activa.
  - La finalización de una Execution no finaliza accidentalmente otra distinta.
- **Sources**: US-001, US-004, US-005, US-007, US-008, US-009, US-012, US-014
- **Technical Mapping**: Not defined yet.

### Execution Context

- **Category**: Functional Concept
- **Definition**: Contexto común mantenido por Evo Runtime que permite
  reconocer que distintas actividades transitivas pertenecen a la misma
  Execution.
- **Relationships**:
  - Pertenece a una Execution.
  - Puede mantenerse durante resolution.
  - Permite la disponibilidad de Implementations.
  - Permite Engine determination.
  - Acompaña la Invocation.
  - Permite Value participation.
  - Permite Failure propagation.
  - Acompaña el trabajo mediante EvoS.
  - Acompaña el trabajo mediante EvoQ.
- **Invariants / Distinctions**:
  - `Execution Context != Scope`
  - Una Execution no debe depender accidentalmente del Execution Context de
    otra Execution.
- **Sources**: US-008, US-009, US-012, US-014
- **Technical Mapping**: Not defined yet.

### Invocation

- **Category**: Functional Action
- **Definition**: Participación efectiva de una Operation mediante una
  Implementation disponible durante una Execution.
- **Relationships**:
  - Ocurre después de resolver una Required Operation.
  - Requiere una Implementation disponible.
  - Puede recibir información (Values).
  - Puede producir Result.
  - Puede originar nuevas Required Operations transitivas.
- **Invariants / Distinctions**:
  - Requiere que la Implementation esté disponible y con su Engine determinado
    si aplica.
- **Sources**: US-004, US-006, US-007, US-008
- **Technical Mapping**: Not defined yet.

---

## 3. Operation and Resolution Concepts

### Operation

- **Category**: Functional Concept
- **Definition**: Unidad funcional de trabajo que puede ser requerida e
  invocada durante una Execution.
- **Relationships**:
  - Puede requerir otra Operation.
  - Puede recibir Values.
  - Puede producir Result.
  - Puede estar satisfecha por una Implementation.
- **Invariants / Distinctions**:
  - `Operation != Implementation`
- **Sources**: US-002, US-004, US-005, US-007
- **Technical Mapping**: Not defined yet.

### Required Operation

- **Category**: Functional Requirement
- **Definition**: Operation que una unidad participante necesita durante una
  Execution sin conocer su Implementation concreta.
- **Relationships**:
  ```text
  Required Operation
          ↓ resolved by
      Evo Runtime
          ↓
     Implementation
  ```
- **Invariants / Distinctions**:
  - `Required Operation != Implementation`
  - `Required Operation != Capability`
  - La resolución correcta produce una única Implementation válida.
  - Si no existe una Implementation válida: Failure.
  - Si existe ambigüedad sin regla suficiente: Failure.
- **Sources**: US-002, US-003, US-004, US-007, US-012
- **Technical Mapping**: Not defined yet.

### Implementation

- **Category**: Functional Concept
- **Definition**: Realización concreta capaz de satisfacer una Required
  Operation y participar en una Execution.
- **Relationships**:
  - Puede ser resuelta desde una Required Operation.
  - Debe estar disponible antes de Invocation.
  - Puede requerir un Engine.
  - Puede requerir transitivamente nuevas Operations.
  - Puede ser una Evo-Script Implementation.
- **Invariants / Distinctions**:
  - `Implementation != Required Operation`
  - `Implementation != Engine`
  - Estar disponible funcionalmente NO define:
    - physical file loading
    - dynamic library loading
    - process loading
    - memory representation
- **Sources**: US-002, US-003, US-004, US-006, US-012
- **Technical Mapping**: Not defined yet.

### Engine

- **Category**: Architectural Role
- **Definition**: Componente capaz de ejecutar una Implementation o tipo de
  trabajo que requiere un mecanismo de ejecución específico.
- **Relationships**:
  - Evo Runtime determina el Engine correspondiente cuando uno es necesario.
  - EvoQ es un Engine.
  - EvoS es un Engine.
  - Una Implementation no necesariamente requiere un Engine.
- **Invariants / Distinctions**:
  - `Engine != Implementation`
  - `Engine != Evo Runtime`
  - `EvoV != Engine`
- **Sources**: US-006, US-011, US-012, US-014
- **Technical Mapping**: Not defined yet.

---

## 4. Values and Outcomes

### Value

- **Category**: Data Concept
- **Definition**: Información válida que puede participar como entrada o como
  resultado exitoso durante una Execution.
- **Relationships**:
  - Puede ser recibido por una Operation.
  - Puede ser producido por una Operation.
  - Puede atravesar la frontera de Evo Runtime.
  - Puede participar en EvoQ.
  - Puede participar en EvoS.
  - Utiliza la base común proporcionada por EvoV.
- **Invariants / Distinctions**:
  - `Value != Failure`
  - `Value != Result`
  - El transporte funcional de Value NO implica:
    - physical copy
    - move
    - serialization
    - same memory instance
    - Rust ownership semantics
- **Sources**: US-004, US-005, US-007, US-008, US-011, US-012, US-013, US-014
- **Technical Mapping**: `evo-values`

### Failure

- **Category**: Outcome Concept
- **Definition**: Outcome funcional que indica que determinado trabajo no pudo
  completarse correctamente.
- **Relationships**:
  - Puede originarse durante resolution.
  - Puede originarse al hacer disponible una Implementation.
  - Puede originarse durante Engine determination.
  - Puede originarse durante Invocation.
  - Puede originarse mediante EvoS.
  - Puede originarse mediante EvoQ.
  - Puede propagarse mediante Evo Runtime.
  - Puede formar el resultado fallido de una Execution.
- **Invariants / Distinctions**:
  - `Failure != Value`
  - `Failure != successful Value`
  - Un Failure no debe convertirse silenciosamente en un Value correcto.
- **Sources**: US-001, US-002, US-003, US-004, US-005, US-006, US-007, US-008,
  US-009, US-012, US-013, US-014
- **Technical Mapping**: `evo-values`

### Result

- **Category**: Outcome Concept
- **Definition**: Outcome funcional producido al completar una Operation o una
  Execution.
- **Conceptual Relationship**:
  ```text
  Operation / Execution
          ↓
        Result
        /    \
       /      \
    Value    Failure
  ```
- **Relationships**:
  - Una Operation puede producir un Result.
  - Una Execution puede producir un Result final.
  - El Host puede recibir el Result final.
  - Un Result correcto puede contener funcionalmente un Value.
  - Un Result fallido expresa un Failure.
- **Invariants / Distinctions**:
  - `Result != Value`
  - `Result != Failure`
  - La semántica funcional de Result no equivale a asumir la sintaxis o tipo
    concreto de `std::result::Result` en el nivel funcional.
- **Sources**: US-001, US-004, US-007, US-009, US-012, US-013, US-014
- **Technical Mapping**: `evo-values`

---

## 5. Provider Boundary

### Provider

- **Category**: Architectural Role
- **Definition**: Componente que proporciona una o más Capabilities utilizables
  por Evo Runtime.
- **Relationships**:
  ```text
  Provider
      ↓ provides
  Capability
  ```
  - La unidad solicitante no necesita:
    - conocer directamente el Provider;
    - localizarlo directamente;
    - crearlo directamente;
    - administrarlo directamente.
- **Invariants / Distinctions**:
  - `Provider != Capability`
  - EvoQ y EvoS del Model A no deben reinterpretarse como Providers
    adicionales.
- **Sources**: US-010, US-011
- **Technical Mapping**: Not defined yet.

### Capability

- **Category**: Functional Concept
- **Definition**: Capacidad funcional individual proporcionada por un Provider y
  que puede participar durante una Execution.
- **Relationships**:
  ```text
  Provider
      ↓ provides
  Capability
  ```
  - La relación exacta:
    ```text
    Required Operation
            ↓
           ???
            ↓
        Capability
    ```
    permanece sin definir.
- **Invariants / Distinctions**:
  - `Capability != Provider`
  - `Capability != catalog`
  - `Capability != module`
  - `Capability != namespace`
  - `Capability != group of operations`
  - `Required Operation != Capability`
- **Sources**: US-010
- **Technical Mapping**: Not defined yet.

---

## 6. Engine-specific Concepts

### Evo-Script

- **Category**: Language
- **Definition**: Lenguaje cuyas Implementations pueden ser ejecutadas mediante
  EvoS.
- **Invariants / Distinctions**:
  - `Evo-Script != EvoS`
  - `Evo-Script != evo-script-engine`
- **Sources**: US-011, US-012, US-013
- **Technical Mapping**: Not defined yet.

### Evo-Script Implementation

- **Category**: Implementation Kind
- **Definition**: Implementation escrita en Evo-Script que puede participar en
  una Execution mediante EvoS.
- **Relationships**:
  ```text
  Required Operation
          ↓
  Evo-Script Implementation
          ↓
     Evo Runtime
          ↓
        EvoS
  ```
  - Puede recibir Values.
  - Puede producir Value.
  - Puede producir Failure.
  - Puede requerir transitivamente nuevas Operations mediante Evo Runtime.
- **Invariants / Distinctions**:
  - Participa mediante EvoS y requiere que sus necesidades transitivas pasen
    por Evo Runtime.
- **Sources**: US-012
- **Technical Mapping**: Not defined yet.

### Query Work

- **Category**: Functional Work
- **Definition**: Trabajo funcional de consulta realizado mediante EvoQ durante
  una Execution.
- **Relationships**:
  ```text
  participant
      ↓ needs query work
  Evo Runtime
      ↓
     EvoQ
      ↓
  Query Work
      ↓
  Value / Failure
      ↓
  Evo Runtime
  ```
- **Invariants / Distinctions**:
  - La unidad solicitante no selecciona directamente EvoQ.
  - La unidad solicitante no administra EvoQ.
  - EvoQ no coordina la Execution global.
- **Sources**: US-013, US-014
- **Technical Mapping**: Not defined yet.

---

## 7. Technical Names

Esta sección registra únicamente nombres técnicos ya acordados para los
componentes del Core.

| Technical Name | Category | Architectural Mapping |
| --- | --- | --- |
| `evo-runtime` | Cargo Package / Crate | Evo Runtime |
| `evo-values` | Cargo Package / Crate | EvoV |
| `evo-query-engine` | Cargo Package / Crate | EvoQ |
| `evo-script-engine` | Cargo Package / Crate | EvoS |

### Distinciones de Nombres:

- `Evo Runtime`: architectural / core component platform.
- `evo-runtime`: technical Cargo package / crate name.
- `EvoV`: architectural / core component (common Values and Outcomes base).
- `evo-values`: technical Cargo package / crate name.
- `EvoQ`: architectural / core engine (query engine).
- `evo-query-engine`: technical Cargo package / crate name.
- `EvoS`: architectural / core engine (Evo-Script engine).
- `evo-script-engine`: technical Cargo package / crate name.

`EvoS != Evo-Script`. No se crean todavía `evo-script` ni `evo-query` como
mappings normativos adicionales.

---

## 8. Technical Mapping

El Data Dictionary precede a:
1. Use Cases
2. Sequence Diagrams
3. Technical Mapping
4. Rust Implementation

Por lo tanto, salvo los nombres de crates del Core y los tipos fundamentales de
outcomes y valores asignados a `evo-values`, las representaciones técnicas
concretas permanecen abiertas.

No se decide todavía qué términos serán:
- `definitions/structs/owned`
- `definitions/structs/borrowed`
- `definitions/enums`
- `definitions/use_cases`
- `definitions/contracts`
- `definitions/requesters`
- `agents`
- `providers`

Tampoco se decide qué conceptos serán structs, enums, function pointers,
aliases, módulos o archivos dentro de `evo-runtime`.

---

## 9. Deferred Terms

| Term | Status |
| --- | --- |
| Scope | Deferred / Not Defined |
| Model B | Deferred / Not Closed |
| Model C | Deferred / Not Closed |
| `.main` | Deferred / Not Defined |
| `.root` | Deferred / Not Defined |
| `.elib` | Deferred / Not Defined |
| `.esig` | Deferred / Not Defined |
| `.emod` | Deferred / Not Defined |

La presencia de estos términos en esta tabla **no** los incorpora
funcionalmente al Model A cerrado y **no** les asigna semántica normativa.
Registra únicamente que son conceptos históricos o extensiones futuras cuya
definición permanece pendiente.

En particular:
- `Execution Context != Scope`

---

## 10. Canonical Distinctions

Las siguientes distinciones canónicas normativas deben preservarse en todas las
fases posteriores:

- `Evo Runtime != Engine`
- `EvoV != Engine`
- `EvoQ = Engine`
- `EvoS = Engine`
- `Evo-Script != EvoS`
- `Required Operation != Implementation`
- `Required Operation != Capability`
- `Implementation != Engine`
- `Provider != Capability`
- `Capability != catalog / module / namespace / group`
- `Execution Context != Scope`
- `Value != Failure`
- `Result != Value`
- `Result != Failure`
- `EvoQ != Evo Runtime`
- `EvoS != Evo Runtime`
- `functional Value transport != physical copy`
- `same Value meaning != same memory instance`
- `Implementation availability != defined physical loading mechanism`
