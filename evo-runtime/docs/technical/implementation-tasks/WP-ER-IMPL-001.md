# WP-ER-IMPL-001 — Evo Runtime Model A Implementation

```text
STATUS:
PLANNED

PROGRESS:
0 / 6 APPROVED

APPROVED THROUGH:
NONE

NEXT:
TASK-ER-IMPL-002

BASE COMMIT:
f16df11595f0167c1b0fdb72c4aeb56056ae487b
```

Este Work Package define la secuencia obligatoria para implementar de forma mínima, canónica e incremental el crate `evo-runtime` bajo **Evo Runtime Model A**, materializando las decisiones cerradas `ARQ-ER-001`, `ARQ-ER-002`, `LT-ER-DM-001`, `LT-ER-SIG-001`, `LT-ER-SIG-002`, `LT-ER-PART-001` y `LT-ER-PART-002`.

---

## 1. Reglas Operativas

- Una Task a la vez.
- Orden estrictamente lineal.
- Una Task debe quedar APPROVED antes de iniciar la siguiente.
- Un commit por Task que cambie archivos.
- Correcciones posteriores usan un commit nuevo.
- NO amend.
- NO-OP no genera empty commit.
- Push es checkpoint, no aprobación.
- Cada candidato debe revisarse directamente en GitHub.
- No merge.
- No rebase.
- No squash.
- No force push.
- Detenerse ante PRODUCT DEFECT FOUND.
- No reabrir arquitectura cerrada desde una Task de implementación.

---

## 2. Autoridad Técnica Cerrada

- **ARQ-ER-001** (CLOSED): Control exclusivo de la duración de `Start(Run)` con terminación natural.
- **ARQ-ER-002** (CLOSED): Sin `Result`, sin `Failure`, sin transporte de outcomes y sin dependencia a `evo-values`.
- **LT-ER-DM-001** (CLOSED): Technical Data Model = 0 identidades (0 Structs, 0 Enums, 0 Artifacts, 0 Borrowed Views, 0 Errors, 0 Outcomes).
- **LT-ER-SIG-001** (CLOSED): Requester `Run`: `pub type Request = fn();`.
- **LT-ER-SIG-002** (CLOSED): Use Case `Start`: `pub type Start = fn(run_request::Request);`.
- **LT-ER-PART-001** (CLOSED): Agent Starter. Módulo canónico: `agents/starter.rs`.
- **LT-ER-PART-002** (CLOSED): Contracts 0, Resolvers 0, Collaborators 0, Tools 0, Providers 0.

### Inventario de Participantes Resultante

```text
Use Cases       1
Agents          1
Requesters      1
Contracts       0
Resolvers       0
Collaborators   0
Tools           0
Providers       0

Total           3 participants
```

---

## 3. Inventario Canónico de Tareas

### TASK-ER-IMPL-001 — Registrar Work Package
- **Estado**: PENDING REVIEW
- **Base obligatoria**: `f16df11595f0167c1b0fdb72c4aeb56056ae487b`
- **Alcance**:
  - Crear la autoridad documental del Work Package (`WP-ER-IMPL-001.md` y `README.md`).
  - Registrar el estado inicial de implementación en `TECHNICAL_DESIGN.md`.
  - Exclusivamente documental. Cero Rust, cero Cargo.

### TASK-ER-IMPL-002 — Materializar crate y estructura modular
- **Estado**: NOT STARTED
- **Objetivo**: Crear físicamente el crate `evo-runtime`.
- **Scope previsto**:
  - `evo-runtime/Cargo.toml`
  - `evo-runtime/src/lib.rs`
  - `evo-runtime/src/agents/mod.rs`
  - `evo-runtime/src/definitions/mod.rs`
  - `evo-runtime/src/definitions/requesters/mod.rs`
  - `evo-runtime/src/definitions/use_cases/mod.rs`
  - Agregar `evo-runtime` al workspace raíz (`Cargo.toml`).
- **Cargo canónico**:
  ```toml
  [package]
  name = "evo-runtime"
  version = "0.1.0"
  edition = "2024"

  [lib]
  path = "src/lib.rs"

  [dependencies]
  ```
  Sin dependencias productivas.
  No implementar aún: `Run`, `Start`, `Starter`.
- **Gates previstos**:
  - `cargo fmt --check`
  - `cargo check -p evo-runtime`

### TASK-ER-IMPL-003 — Materializar firmas arquitectónicas
- **Estado**: NOT STARTED
- **Objetivo**: Materializar las definiciones de Use Case y Requester.
- **Scope previsto**:
  - `evo-runtime/src/definitions/requesters/run_request.rs`:
    ```rust
    pub type Request = fn();
    ```
  - `evo-runtime/src/definitions/use_cases/start.rs`:
    ```rust
    use crate::definitions::requesters::run_request;

    pub type Start = fn(run_request::Request);
    ```
  - Exportar definiciones en módulos correspondientes.
  - No Agent todavía.
- **Gates previstos**:
  - `cargo fmt --check`
  - `cargo check -p evo-runtime`
  - `cargo test -p evo-runtime`

### TASK-ER-IMPL-004 — Implementar Starter y comportamiento Model A
- **Estado**: NOT STARTED
- **Objetivo**: Implementar el Agent canónico `Starter` y la suite de tests unitarios de comportamiento de Model A.
- **Scope previsto**:
  - `evo-runtime/src/agents/starter.rs`:
    ```rust
    use crate::definitions::{
        requesters::run_request,
        use_cases::start,
    };

    pub fn start(run: run_request::Request) {
        run();
    }

    pub const START: start::Start = start;
    ```
  - Reexportar `starter` y `START` en `lib.rs` / `agents/mod.rs`.
- **Tests obligatorios**:
  1. `START` satisface exactamente `start::Start`.
  2. `start` invoca `Run` exactamente una vez.
  3. Cuando `Run` retorna, `start` retorna.
  4. Invocaciones independientes ejecutan sus respectivos `Runs` sin estado compartido del Runtime.
  5. `Start` no transporta outcome.
- **Gates previstos**:
  - `cargo fmt --check`
  - `cargo check -p evo-runtime`
  - `cargo test -p evo-runtime`

### TASK-ER-IMPL-005 — Cerrar documentación de implementación
- **Estado**: NOT STARTED
- **Objetivo**: Cerrar documentalmente la implementación tras verificar la suite unitaria.
- **Scope previsto**:
  - `evo-runtime/docs/technical/TECHNICAL_DESIGN.md`
  - `evo-runtime/docs/technical/implementation-tasks/README.md`
  - `evo-runtime/docs/technical/implementation-tasks/WP-ER-IMPL-001.md`
  - Registrar `Implementation: CLOSED` y `Final Quality Gate: PENDING`.
  - Sin cambios a código Rust, arquitectura, firmas ni participantes.

### TASK-ER-IMPL-006 — Final Quality Gate
- **Estado**: NOT STARTED
- **Objetivo**: Auditoría técnica y verificación final de calidad integral.
- **Resultado esperado**: `NO-OP` si todos los gates pasan y no se detectan defectos.
- **Gates obligatorios**:
  - `cargo fmt --check`
  - `cargo check -p evo-runtime`
  - `cargo test -p evo-runtime`
- **Auditoría técnica**:
  - `Run`: `pub type Request = fn();`
  - `Start`: `pub type Start = fn(run_request::Request);`
  - `Starter`: `pub fn start(run: run_request::Request)` y `pub const START: start::Start = start;`
  - Inventario participantes = 3 (1 Use Case, 1 Agent, 1 Requester; 0 Contracts, 0 Resolvers, 0 Collaborators, 0 Tools, 0 Providers).
  - Data Model = 0 identidades (0 Structs, 0 Enums, 0 Errors, 0 Outcomes).
  - Dependencias prohibidas ausentes (`evo-values`, `tokio`, `serde`, `anyhow`, `thiserror`).
  - Semántica de ejecución: `START(run) -> run() -> return ()`.
  - Ausencia confirmada de: `Result`, `Failure`, `Context`, `Execution`, `stop`, `finalize`, `engine loading`, `provider management`.
  - `cargo test --workspace` se registra como validación informativa debido al defecto conocido y aislado en `evo-query`.
