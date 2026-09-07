# Especificación de Evo Runtime v0

Status: MODEL A CLOSED — RECONCILED

## 1. Propósito

Evo Runtime Model A define la frontera mínima para iniciar una Evo Application.

Su responsabilidad única es:

1. recibir Run;
2. invocar Run;
3. permanecer activo durante la ejecución de Run;
4. concluir cuando Run concluye.

---

## 2. Frontera del Runtime

La frontera arquitectónica de Evo Runtime Model A está definida por:
- Exactamente **un Use Case** proporcionado por el Runtime (`Start`).
- Exactamente **un Requester** consumido desde la aplicación (`Run`).
- Exactamente **0 Outcome Models**.

```text
Host
 │
 │ Start(Run)
 ▼
Evo Runtime
 │
 │ Run()
 ▼
Evo Application
 │
 │ ejecución
 ▼
Run retorna naturalmente
 │
 ▼
Start retorna naturalmente
 │
 ▼
Host
```

Evo Runtime no transporta ningún dato en el retorno.

---

## 3. Use Case Start

- **Categoría**: Use Case (Proporcionado por `evo-runtime`)
- **Definición**: `definitions/use_cases/start.rs`
- **Tipo Técnico**: Technical signature: PENDING TECHNICAL REVALIDATION
- **Semántica**:
  1. Recibe la acción `Run` proporcionada por la Evo Application desde el Host.
  2. Invoca `Run`.
  3. Permanece activo en el call stack durante la duración de `Run`.
  4. Retorna naturalmente cuando `Run` retorna naturalmente.
  5. No requiere operaciones explícitas de `stop()`, `close()` o `finalize()`; la
     terminación de `Run` concluye naturalmente `Start`.

---

## 4. Requester Run

- **Categoría**: Requester (Consumido por `evo-runtime` desde la Evo Application)
- **Definición**: `definitions/requesters/run_request.rs`
- **Tipo Técnico**: Technical signature: PENDING TECHNICAL REVALIDATION
- **Semántica**:
  1. Representa la acción de punto de entrada que la aplicación proporciona al
     Runtime.
  2. Encapsula el ciclo de vida de ejecución de la aplicación desde la perspectiva
     de Runtime.
  3. Su retorno natural determina la conclusión de `Start`.

---

## 5. Reconciliación de Model A

Una versión anterior modelaba `Result` y `Failure` como outcomes transportados por
Evo Runtime y propiedad de `evo-values`.

`ARQ-ER-001` y `ARQ-ER-002` superseden esa decisión:

- Evo Runtime Model A controla únicamente la duración de `Start(Run)`.
- El modelo actual no posee `Result` ni `Failure`.
- El modelo actual no transporta outcomes.
- `evo-runtime Model A` no requiere una dependencia arquitectónica a `evo-values`.

---

## 6. Invocaciones Independientes de Start

Evo Runtime soporta múltiples invocaciones concurrentes o secuenciales de Start:

```text
Start(run_app_1)  ──►  App 1  ──►  concluye naturalmente  ──►  Start concluye
Start(run_app_2)  ──►  App 2  ──►  concluye naturalmente  ──►  Start concluye
```

- Cada invocación de `Start` es aislada e independiente.
- La terminación de una invocación de `Run` no termina, modifica ni altera otra
  invocación de `Start`.
- Evo Runtime no comparte estado a través de las invocaciones.

---

## 7. No Responsabilidades del Runtime

Evo Runtime Model A excluye deliberadamente todos los mecanismos internos de
coordinación y transporte de resultados:
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
- **Sin transporte de outcomes**: El Runtime no recibe ni entrega resultados de
  negocio (`Result`/`Failure`).
- **Sin dependencia a evo-values**: El Runtime no depende arquitectónicamente
  de `evo-values` en Model A.

---

## 8. Engines y Aplicaciones

Una vez que `Start` invoca a `Run`, la Evo Application ejecuta su lógica de
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
| **Start** | Use Case | `definitions/use_cases/start.rs` | Technical signature: PENDING TECHNICAL REVALIDATION |
| **Run** | Requester | `definitions/requesters/run_request.rs` | Technical signature: PENDING TECHNICAL REVALIDATION |

---

## 11. Invariantes Cerrados

1. `Start != Run`.
2. `Start` recibe la acción `Run`, no el resultado de ejecutarla previamente.
3. `Start` invoca `Run` exactamente como acción proporcionada por la aplicación.
4. La duración de `Start` está delimitada por la duración de `Run`.
5. Cuando `Run` retorna, `Start` retorna naturalmente.
6. Evo Runtime proporciona exactamente 1 Use Case: `Start`.
7. Evo Runtime consume exactamente 1 Requester: `Run`.
8. Evo Runtime Model A no posee `Result` ni `Failure`.
9. Evo Runtime Model A no transporta outcomes.
10. Evo Runtime Model A no depende arquitectónicamente de `evo-values`.
11. Evo Runtime no posee `Context`, `Execution`, `Providers` ni engine resolution en Model A.
12. Las invocaciones `Start` son independientes y no comparten estado.
