# Especificación de Evo Runtime v0

Status: MODEL A CLOSED

## 1. Propósito

Evo Runtime Model A define la frontera mínima de plataforma para iniciar una
Evo Application. En Model A, el Runtime tiene una responsabilidad única y mínima:
iniciar la ejecución invocando la acción `Run` proporcionada por una Evo
Application, mantener la invocación activa mientras la aplicación se ejecuta, y
entregar el `Result` final al caller.

---

## 2. Frontera del Runtime

La frontera arquitectónica de Evo Runtime Model A está definida por:
- Exactamente **un Use Case** proporcionado por el Runtime (`Start`).
- Exactamente **un Requester** consumido desde la aplicación (`Run`).
- Exactamente **un tipo de outcome** (`Result`) definido por `evo-values`.

```text
Caller / Host
     │
     │ llama Start(Run)
     ▼
┌───────────────────────────────┐
│ Evo Runtime                   │
│                               │
│  Use Case: Start              │
│       │                       │
│       │ llama run()           │
│       ▼                       │
│  Requester: Run               │
└───────┬───────────────────────┘
        │
        ▼
Evo Application (activa)
        │
        │ retorna Result
        ▼
   Result (evo-values)
```

---

## 3. Use Case Start

- **Categoría**: Use Case (Proporcionado por `evo-runtime`)
- **Definición**: `definitions/use_cases/start.rs`
- **Tipo Function Pointer**: `pub type Start = fn(run_request::Request) -> Result;`
- **Semántica**:
  1. Recibe el function pointer del requester `Run` desde el caller.
  2. Invoca `run()`.
  3. Permanece activo en el call stack durante la duración de `run()`.
  4. Retorna el `Result` producido por `run()` directamente al caller.
  5. No requiere operaciones explícitas de `stop()`, `close()` o `finalize()`; la
     terminación de `run()` concluye naturalmente `start()`.

---

## 4. Requester Run

- **Categoría**: Requester (Consumido por `evo-runtime` desde la Evo Application)
- **Definición**: `definitions/requesters/run_request.rs`
- **Tipo Function Pointer**: `pub type Request = fn() -> Result;`
- **Semántica**:
  1. Representa la acción de punto de entrada que la aplicación proporciona al
     Runtime.
  2. Encapsula el ciclo de vida completo de ejecución de la aplicación.
  3. Retorna `Result` al completarse.

---

## 5. Result

- `Result` es el tipo de outcome canónico que representa la conclusión de una
  ejecución (éxito o fallo).
- Definido y propiedad de `evo-values`.
- Desde la perspectiva de Evo Runtime, `Result` es un tipo de outcome concreto;
  no se exponen genéricos a través de la frontera del Runtime.
- `Result != Failure`: un outcome fallido se expresa a través de la rama de
  fallo de `Result`.

---

## 6. Invocaciones Independientes de Start

Evo Runtime soporta múltiples invocaciones concurrentes o secuenciales de Start:

```text
Start(run_app_1)  ──►  App 1  ──►  Result 1
Start(run_app_2)  ──►  App 2  ──►  Result 2
```

- Cada invocación de `Start` es aislada e independiente.
- El fallo de una aplicación no afecta a otra aplicación.
- Evo Runtime no comparte estado a través de las invocaciones.

---

## 7. No Responsabilidades del Runtime

Evo Runtime Model A excluye deliberadamente todos los mecanismos internos de
coordinación:
- **Sin struct Context**: El Runtime no mantiene contexto de ejecución ni
  estado de sesión.
- **Sin entidad Execution**: El ciclo de vida de ejecución está representado
  únicamente por el call stack activo de `Start(run)`.
- **Sin resolución de Engines**: El Runtime no descubre, no carga ni selecciona
  engines (por ejemplo, EvoS, EvoQ).
- **Sin gestión de Provider / Contract**: Los providers y capabilities no son
  administrados por el Runtime.
- **Sin transporte de Value**: El flujo de datos entre operaciones ocurre
  directamente dentro de la aplicación.
- **Sin resolución de operaciones**: El Runtime no resuelve dependencias ni
  símbolos.

---

## 8. Engines y Aplicaciones

Una vez que `Start` invoca `run()`, la Evo Application ejecuta su lógica de
dominio directamente con sus propias dependencias, bibliotecas y engines:

```text
Evo Application
  ├── Parsers / Lexers
  ├── Evo-Script Engine (EvoS)
  ├── Query Engine (EvoQ)
  └── Providers Externos / Bibliotecas
```

Evo Runtime no actúa como intermediario, service locator ni message bus para
estas interacciones internas.

---

## 9. Futura Extensión de Engines Compilados

Arquitecturas de extensión futuras podrían soportar la instalación y carga
dinámica de engines compilados sin recompilar el producto.

Esta capacidad futura:
- Permanece completamente fuera del alcance de Model A.
- No introduce registros de engines ni cargadores dinámicos en `evo-runtime`.
- Se definirá en una especificación técnica de extensión separada.

---

## 10. Mapeo Técnico

| Concepto | Rol Arquitectónico | Archivo de Definición Técnica | Tipo Técnico |
| --- | --- | --- | --- |
| **Start** | Use Case | `definitions/use_cases/start.rs` | `pub type Start = fn(run_request::Request) -> Result;` |
| **Run** | Requester | `definitions/requesters/run_request.rs` | `pub type Request = fn() -> Result;` |
| **Starter** | Agent (futuro) | `agents/starter/start.rs` | `pub fn start(run: run_request::Request) -> Result` |
| **Result** | Tipo de Outcome | `evo-values` | Tipo de outcome desde `evo-values` |

---

## 11. Invariantes Cerrados

1. `Start != Run`
2. `Result != Failure`
3. `Start(run)` recibe el function pointer `run`, no el resultado evaluado.
4. Evo Runtime proporciona exactamente 1 Use Case (`Start`) y consume
   exactamente 1 Requester (`Run`).
5. Evo Runtime no tiene Context, no tiene entidad Execution y no tiene
   Providers en Model A.
