# Evo Runtime Model A — Data Dictionary

Status: FUNCTIONAL CLOSED

This document consolidates the canonical architectural and functional vocabulary
for Evo Runtime Model A, derived from the closed Model A functional scope.

Evo Runtime Model A defines a minimal boundary with exactly **seven canonical
terms**.

---

## 1. Canonical Terms

### Evo Runtime

- **Category**: Core Component / Platform
- **Definition**: Plataforma mínima responsable de iniciar la ejecución de una
  Evo Application mediante la acción funcional Start.
- **Relationships**:
  - Proporciona la acción `Start`.
  - Recibe la acción `Run` proporcionada por una Evo Application.
  - Retorna el `Result` final hacia el Host.
- **Invariants / Distinctions**:
  - Evo Runtime no administra la lógica interna, engines ni dependencias de la
    aplicación tras haberla iniciado.
- **Sources**: US-001, UC-001
- **Technical Mapping**: `evo-runtime`

### Host

- **Category**: External Role
- **Definition**: Caller externo (usuario, proceso, shell o sistema operativo)
  que solicita a Evo Runtime iniciar una Evo Application y recibe el Result final.
- **Relationships**:
  - `Host -> requests -> Start(Run)`
  - `Host <- receives <- Result`
- **Invariants / Distinctions**:
  - `Host != Evo Application` (aunque un producto integrado pueda actuar en
    ambos roles).
  - El Host no administra el ciclo de vida interno de la aplicación.
- **Sources**: US-001, UC-001
- **Technical Mapping**: External caller

### Evo Application

- **Category**: External Application Domain
- **Definition**: Aplicación cliente cuya acción ejecutable Run es proporcionada
  a Evo Runtime para iniciar su ejecución.
- **Relationships**:
  - Proporciona la acción `Run` compatible con Evo Runtime.
  - Ejecuta su trabajo interno interactuando directamente con sus librerías,
    engines y providers.
- **Invariants / Distinctions**:
  - Las operaciones y estados internos de la Evo Application no pertenecen a
    Evo Runtime.
- **Sources**: US-001, UC-001
- **Technical Mapping**: Client application crate

### Start

- **Category**: Provided Functional Action / Use Case
- **Definition**: Única acción funcional proporcionada por Evo Runtime que
  recibe la acción Run de una Evo Application, la invoca y retorna su Result.
- **Relationships**:
  - Recibe una función `Run`.
  - Permanece activa mientras `Run` esté activo.
  - Produce un `Result` final.
- **Invariants / Distinctions**:
  - `Start != Run`
  - `Start` es responsabilidad exclusiva de Evo Runtime.
  - Múltiples llamadas a Start son independientes entre sí.
- **Sources**: US-001, UC-001
- **Technical Mapping**: `definitions/use_cases/start.rs` (`pub type Start = fn(...) -> Result;`)

### Run

- **Category**: Consumed Functional Action / Requester
- **Definition**: Acción ejecutable proporcionada por una Evo Application que
  Evo Runtime invoca para iniciar y mantener la aplicación en ejecución.
- **Relationships**:
  - Entregada a `Start`.
  - Invocada por Evo Runtime.
  - Concluye entregando un `Result`.
- **Invariants / Distinctions**:
  - `Run != Start`
  - `Run` es responsabilidad de la Evo Application.
  - La conclusión de `Run` determina la finalización de `Start`.
- **Sources**: US-001, UC-001
- **Technical Mapping**: `definitions/requesters/run_request.rs` (`pub type Request = fn() -> Result;`)

### Result

- **Category**: Outcome Concept
- **Definition**: Outcome final producido al concluir la ejecución de Run y
  retornado por Start hacia el Host.
- **Relationships**:
  - Producido por la acción `Run`.
  - Retornado por la acción `Start`.
  - Entregado al `Host`.
- **Invariants / Distinctions**:
  - `Result != Failure` (Failure es la expresión de un resultado fallido dentro
    del modelo de Result).
  - La semántica y tipos de Result pertenecen a `evo-values`.
- **Sources**: US-001, UC-001
- **Technical Mapping**: `evo-values`

### Failure

- **Category**: Outcome Concept
- **Definition**: Outcome que indica que la ejecución de la aplicación no pudo
  completarse exitosamente.
- **Relationships**:
  - Puede ser expresado como el resultado fallido dentro de `Result`.
- **Invariants / Distinctions**:
  - `Failure != Result`
  - La semántica y tipos de Failure pertenecen a `evo-values`.
- **Sources**: US-001, UC-001
- **Technical Mapping**: `evo-values`

---

## 2. Canonical Distinctions

- `Start != Run`: Start es la responsabilidad provista por Evo Runtime; Run es
  la acción provista por la Evo Application.
- `Host != Evo Application`: El invocador externo y la aplicación destino son
  roles conceptualmente distintos.
- `Result != Failure`: Result es el tipo de resultado de la ejecución; Failure
  es el estado de fallo.
- `Evo Runtime != Engine`: Evo Runtime no ejecuta lenguajes ni motores
  especializados; inicia la aplicación.

---

## 3. Scope Boundaries

Los siguientes conceptos pertenecen a otras capas o componentes del ecosistema
Evo y **no** forman parte del Data Dictionary de Evo Runtime Model A:

- *No Context / No Execution entity*: El Runtime no mantiene almacenamiento de
  contexto ni entidades de ejecución en Model A.
- *No Providers / No Capabilities / No Contracts*: La gestión de providers y
  capacidades ocurre fuera del Runtime.
- *No Engine selection / No Value transport*: La selección de engines y el flujo
  de datos ocurren directamente entre la aplicación y sus componentes.
