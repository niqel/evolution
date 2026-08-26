# Evo Runtime — Mapa de Frontera de Model A

Status: TECHNICAL MODEL CLOSED — IMPLEMENTATION DEFERRED

Este documento ilustra la frontera arquitectónica mínima de Evo Runtime
Model A.

---

## 1. Arquitectura de Frontera Mínima

En Model A, Evo Runtime define una interfaz de ejecución estrictamente mínima:
- **1 Use Case**: `Start` (proporcionado por `evo-runtime`)
- **1 Requester**: `Run` (consumido desde la `Evo Application`)
- **1 Outcome**: `Result` (definido por `evo-values`)

---

## 2. Mapa Visual de Frontera

![Evo Runtime Boundary Map](RUNTIME_BOUNDARY_MAP.svg)

---

## 3. Flujo de Ejecución

1. **Invocación del Host**: El caller externo invoca `Start`, suministrando el
   function pointer del requester `Run` ejecutable de la aplicación
   (`Start(run)`).
2. **Ejecución del Runtime**: Evo Runtime invoca `run()` y permanece activo en
   el call stack.
3. **Autonomía de la Aplicación**: La aplicación ejecuta su lógica interna
   directamente con sus propias bibliotecas, engines y providers.
4. **Finalización**: Cuando `run()` termina, retorna `Result`.
5. **Entrega de Outcome**: `Start` retorna el `Result` directamente al Host.

---

## 4. Firmas Técnicas

```rust
// definitions/requesters/run_request.rs
pub type Request = fn() -> Result;

// definitions/use_cases/start.rs
pub type Start = fn(run_request::Request) -> Result;
```

---

## Referencias

- [DEFINITION_NAMING_CONVENTIONS.md](../DEFINITION_NAMING_CONVENTIONS.md)
- [EVO_RUNTIME_SPECIFICATION_v0.md](../../EVO_RUNTIME_SPECIFICATION_v0.md)
- [DATA_DICTIONARY.md](../../functional/DATA_DICTIONARY.md)
- [MODEL_A_FUNCTIONAL_COVERAGE.md](../../functional/MODEL_A_FUNCTIONAL_COVERAGE.md)
