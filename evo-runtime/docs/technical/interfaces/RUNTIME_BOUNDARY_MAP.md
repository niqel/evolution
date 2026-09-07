# Evo Runtime — Mapa de Frontera de Model A

Status: TECHNICAL MODEL CLOSED — IMPLEMENTATION PENDING

Este documento ilustra la frontera arquitectónica mínima y la vista modular de Evo Runtime
Model A.

---

## 1. Arquitectura de Frontera Mínima

En Model A, Evo Runtime define una frontera estrictamente mínima:
- **1 Use Case**: `Start` (proporcionado por `evo-runtime`)
- **1 Requester**: `Run` (consumido desde la `Evo Application`)
- **1 Agent**: `Starter` (implementación canónica de `Start`)
- **0 Outcomes**: No transporta outcomes ni depende de `evo-values`

---

## 2. Mapa Visual de Frontera

![Evo Runtime Boundary Map](RUNTIME_BOUNDARY_MAP.svg)

---

## 3. Flujo de Ejecución

1. **Invocación del Host**: El caller externo invoca `START`, suministrando la acción `Run` ejecutable de la aplicación (`START(run)`).
2. **Ejecución del Starter**: El Agent `Starter` invoca `run()` a través del Requester.
3. **Autonomía de la Aplicación**: La aplicación ejecuta su trabajo interno directamente con sus propias bibliotecas, engines y providers.
4. **Retorno de Run**: Cuando la aplicación concluye naturalmente, `run()` retorna `()`.
5. **Retorno de Start**: `Starter` retorna naturalmente `()` al Host, concluyendo la llamada.

```text
Host
 ↓ START(run)
Starter
 ↓ run()
Run Requester
 ↓ implementación
Evo Application
 ↑ return ()
Starter
 ↑ return ()
Host
```

---

## 4. Firmas Técnicas Canónicas

```rust
// definitions/requesters/run_request.rs
pub type Request = fn();

// definitions/use_cases/start.rs
use crate::definitions::requesters::run_request;

pub type Start = fn(run_request::Request);

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

## Referencias

- [DEFINITION_NAMING_CONVENTIONS.md](../DEFINITION_NAMING_CONVENTIONS.md)
- [EVO_RUNTIME_SPECIFICATION_v0.md](../../EVO_RUNTIME_SPECIFICATION_v0.md)
- [DATA_DICTIONARY.md](../../functional/DATA_DICTIONARY.md)
- [MODEL_A_FUNCTIONAL_COVERAGE.md](../../functional/MODEL_A_FUNCTIONAL_COVERAGE.md)
- [TECHNICAL_DESIGN.md](../TECHNICAL_DESIGN.md)
