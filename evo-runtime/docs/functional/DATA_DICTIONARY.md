# Evo Runtime Model A — Data Dictionary

Status: FUNCTIONAL CLOSED

Este documento consolida el vocabulario arquitectónico y funcional canónico
para Evo Runtime Model A, derivado del alcance funcional cerrado de Model A.

Evo Runtime Model A define una frontera mínima con exactamente **siete términos
canónicos**.

---

## 1. Términos Canónicos

### Evo Runtime

- **Categoría**: Core Component / Platform
- **Definición**: Plataforma mínima responsable de iniciar la ejecución de una
  Evo Application mediante la acción funcional Start.
- **Relaciones**:
  - Proporciona la acción `Start`.
  - Recibe la acción `Run` proporcionada por una Evo Application.
  - Retorna el `Result` final hacia el Host.
- **Invariantes / Distinciones**:
  - Evo Runtime no administra la lógica interna, engines ni dependencias de la
    aplicación tras haberla iniciado.
- **Fuentes**: US-001, UC-001
- **Mapeo Técnico**: `evo-runtime`

### Host

- **Categoría**: External Role
- **Definición**: Caller externo (usuario, proceso, shell o sistema operativo)
  que solicita a Evo Runtime iniciar una Evo Application y recibe el Result final.
- **Relaciones**:
  - `Host -> requests -> Start(Run)`
  - `Host <- receives <- Result`
- **Invariantes / Distinciones**:
  - `Host != Evo Application` (aunque un producto integrado pueda actuar en
    ambos roles).
  - El Host no administra el ciclo de vida interno de la aplicación.
- **Fuentes**: US-001, UC-001
- **Mapeo Técnico**: External caller

### Evo Application

- **Categoría**: External Application Domain
- **Definición**: Aplicación cliente cuya acción ejecutable Run es proporcionada
  a Evo Runtime para iniciar su ejecución.
- **Relaciones**:
  - Proporciona la acción `Run` compatible con Evo Runtime.
  - Ejecuta su trabajo interno interactuando directamente con sus librerías,
    engines y providers.
- **Invariantes / Distinciones**:
  - Las operaciones y estados internos de la Evo Application no pertenecen a
    Evo Runtime.
- **Fuentes**: US-001, UC-001
- **Mapeo Técnico**: Client application crate

### Start

- **Categoría**: Provided Functional Action / Use Case
- **Definición**: Única acción funcional proporcionada por Evo Runtime que
  recibe la acción Run de una Evo Application, la invoca y retorna su Result.
- **Relaciones**:
  - Recibe una función `Run`.
  - Permanece activa mientras `Run` esté activo.
  - Produce un `Result` final.
- **Invariantes / Distinciones**:
  - `Start != Run`
  - `Start` es responsabilidad exclusiva de Evo Runtime.
  - Múltiples llamadas a Start son independientes entre sí.
- **Fuentes**: US-001, UC-001
- **Mapeo Técnico**: `definitions/use_cases/start.rs` (`pub type Start = fn(...) -> Result;`)

### Run

- **Categoría**: Consumed Functional Action / Requester
- **Definición**: Acción ejecutable proporcionada por una Evo Application que
  Evo Runtime invoca para iniciar y mantener la aplicación en ejecución.
- **Relaciones**:
  - Entregada a `Start`.
  - Invocada por Evo Runtime.
  - Concluye entregando un `Result`.
- **Invariantes / Distinciones**:
  - `Run != Start`
  - `Run` es responsabilidad de la Evo Application.
  - La conclusión de `Run` determina la finalización de `Start`.
- **Fuentes**: US-001, UC-001
- **Mapeo Técnico**: `definitions/requesters/run_request.rs` (`pub type Request = fn() -> Result;`)

### Result

- **Categoría**: Outcome Concept
- **Definición**: Outcome final producido al concluir la ejecución de Run y
  retornado por Start hacia el Host.
- **Relaciones**:
  - Producido por la acción `Run`.
  - Retornado por la acción `Start`.
  - Entregado al `Host`.
- **Invariantes / Distinciones**:
  - `Result != Failure` (Failure es la expresión de un resultado fallido dentro
    del modelo de Result).
  - La semántica y tipos de Result pertenecen a `evo-values`.
- **Fuentes**: US-001, UC-001
- **Mapeo Técnico**: `evo-values`

### Failure

- **Categoría**: Outcome Concept
- **Definición**: Outcome que indica que la ejecución de la aplicación no pudo
  completarse exitosamente.
- **Relaciones**:
  - Puede ser expresado como el resultado fallido dentro de `Result`.
- **Invariantes / Distinciones**:
  - `Failure != Result`
  - La semántica y tipos de Failure pertenecen a `evo-values`.
- **Fuentes**: US-001, UC-001
- **Mapeo Técnico**: `evo-values`

---

## 2. Distinciones Canónicas

- `Start != Run`: Start es la responsabilidad provista por Evo Runtime; Run es
  la acción provista por la Evo Application.
- `Host != Evo Application`: El invocador externo y la aplicación destino son
  roles conceptualmente distintos.
- `Result != Failure`: Result es el tipo de resultado de la ejecución; Failure
  es el estado de fallo.
- `Evo Runtime != Engine`: Evo Runtime no ejecuta lenguajes ni motores
  especializados; inicia la aplicación.

---

## 3. Límites de Alcance

Los siguientes conceptos pertenecen a otras capas o componentes del ecosistema
Evo y **no** forman parte del Data Dictionary de Evo Runtime Model A:

- *Sin Context / Sin entidad Execution*: El Runtime no mantiene almacenamiento de
  contexto ni entidades de ejecución en Model A.
- *Sin Providers / Sin Capabilities / Sin Contracts*: La gestión de providers y
  capacidades ocurre fuera del Runtime.
- *Sin selección de Engines / Sin transporte de Value*: La selección de engines y el flujo
  de datos ocurren directamente entre la aplicación y sus componentes.
