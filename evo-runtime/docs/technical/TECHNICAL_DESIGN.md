# Evo Runtime Model A — Technical Design

Status: TECHNICAL DESIGN CLOSED

Este documento consolida el diseño técnico formal de `evo-runtime Model A`, materializando las decisiones cerradas `LT-ER-DM-001`, `LT-ER-SIG-001`, `LT-ER-SIG-002`, `LT-ER-PART-001` y `LT-ER-PART-002`.

---

## 1. Referencia al Functional Model CLOSED

El modelo técnico implementa estrictamente el modelo funcional cerrado documentado en:
- [DATA_DICTIONARY.md](../functional/DATA_DICTIONARY.md) (5 términos canónicos)
- [MODEL_A_FUNCTIONAL_COVERAGE.md](../functional/MODEL_A_FUNCTIONAL_COVERAGE.md) (1 User Story, 1 Use Case)
- [US-001](../functional/user-stories/US-001-start-application.md)
- [UC-001](../functional/use-cases/UC-001-start-evo-application.md)
- [EVO_RUNTIME_SPECIFICATION_v0.md](../../EVO_RUNTIME_SPECIFICATION_v0.md) (ARQ-ER-001 CLOSED, ARQ-ER-002 CLOSED)

---

## 2. Technical Data Model

```text
Structs:         0
Enums:           0
Artifacts:       0
Borrowed Views:  0
Errors:          0
Outcomes:        0
─────────────────
Total:           0 identities
```

Model A no posee estructuras de datos, enums, errores ni modelos de outcome. No depende de `evo-values`.

---

## 3. Firmas Técnicas Canónicas (Run y Start)

### Requester Run
```rust
// definitions/requesters/run_request.rs

pub type Request = fn();
```

### Use Case Start
```rust
// definitions/use_cases/start.rs

use crate::definitions::requesters::run_request;

pub type Start = fn(run_request::Request);
```

---

## 4. Agent Starter

El único ejecutor de Model A reside directamente en `agents/starter.rs`:

```rust
// agents/starter.rs

use crate::definitions::{
    requesters::run_request,
    use_cases::start,
};

pub fn start(run: run_request::Request) {
    run();
}

pub const START: start::Start = start;
```

---

## 5. Inventario de Participantes

```text
Use Cases:       1  (start)
Agents:          1  (starter)
Requesters:      1  (run_request)

Contracts:       0
Resolvers:       0
Collaborators:   0
Tools:           0
Providers:       0
──────────────────
Total:           3 participants
```

---

## 6. Componentes No Utilizados (Cero Absoluto)

```text
Contracts:       0
Resolvers:       0
Collaborators:   0
Tools:           0
Providers:       0
Context:         0
Execution:       0
```

---

## 7. Technical Data Diagrams

```text
Technical Data Diagrams: 0
```
Al tener un Technical Data Model de 0 identidades, no existen diagramas de datos.

---

## 8. Boundary / Module Signature Views

```text
Boundary & Module Views: 1
```
- [RUNTIME_BOUNDARY_MAP.d2](interfaces/RUNTIME_BOUNDARY_MAP.d2) / [RUNTIME_BOUNDARY_MAP.svg](interfaces/RUNTIME_BOUNDARY_MAP.svg)
- Documentación: [RUNTIME_BOUNDARY_MAP.md](interfaces/RUNTIME_BOUNDARY_MAP.md)

---

## 9. Sequence Views

```text
Sequence Views: 1
```
- [00-start-application.d2](sequences/00-start-application.d2) / [00-start-application.svg](sequences/00-start-application.svg)
- Registro: [sequences/README.md](sequences/README.md)

---

## 10. Estado de Implementación

```text
Implementation:
CLOSED

Implementation Head:
3ca34c606251c07236d37280ae70d794ae98741a

Work Package:
WP-ER-IMPL-001

Progress:
4 / 6 APPROVED

Current Task:
TASK-ER-IMPL-005 — PENDING REVIEW

Next after approval:
TASK-ER-IMPL-006 — Final Quality Gate

Final Quality Gate:
PENDING
```

El diseño técnico y la implementación productiva en Rust están formalmente cerrados (`Implementation: CLOSED`). Nótese que `Implementation CLOSED != Work Package CLOSED`: el código productivo y sus pruebas unitarias están completos, pero el Work Package permanece abierto hasta que se verifique y apruebe el Final Quality Gate en `TASK-ER-IMPL-006`. La secuencia y estado de tareas se gestiona a través del Work Package [WP-ER-IMPL-001](implementation-tasks/WP-ER-IMPL-001.md).
