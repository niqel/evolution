# Evo-Script Engine — Public Capabilities

Status: FUNCTIONAL CLOSED — REVALIDATED AFTER EFN HOST BOUNDARY

## Purpose

Este documento define las capacidades públicas que `evo-script-engine` ofrece a sus Consumers en v0.

Las Public Capabilities representan operaciones completas que tiene sentido solicitar al Engine desde fuera de su implementación. Las responsabilidades internas necesarias para cumplirlas, como lexical analysis, parsing, semantic analysis, bytecode generation, VM execution, external symbol resolution o capability binding, no constituyen Public Capabilities independientes.

`evo-script-engine` v0 expone exactamente tres Public Capabilities:

1. `Compile`
2. `Execute Compiled`
3. `Execute Source`

La frontera `.efn` / Host se rige por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`: ninguna de estas capacidades recibe o mantiene `Active Scope` para una ejecución `.efn`.

---

## 1. Compile

`Compile` recibe el `Source Text` completo de un programa Evo-Script y produce un `Compiled Program` cuando el Source Text cumple la especificación vigente de `evo-script`.

```text
Source Text
    │
    ▼
  Compile
    │
    ▼
Compiled Program
```

Para producirlo, el Engine realiza internamente lexical analysis, parsing, semantic validation y bytecode generation.

### Invariants

- `Compile` recibe Source Text, no una ruta de archivo.
- `Compile` no realiza filesystem I/O.
- `Compile` valida el programa conforme a `evo-script` y sus amendments normativos vigentes.
- `use` no es una construcción válida de `.efn`.
- `Compile` produce bytecode como representación ejecutable del `Compiled Program`.
- `Compile` no ejecuta la Public Function.
- `Compile` no recibe `Invocation Values`.
- `Compile` no requiere Providers concretos.
- El `Compiled Program` puede conservar `External Symbols` todavía no enlazados.
- El `Compiled Program` nunca conserva direcciones físicas de function pointers suministrados por una aplicación o Provider.

---

## 2. Execute Compiled

`Execute Compiled` recibe un `Compiled Program` y los `Invocation Values` requeridos por su Public Function, ejecuta el bytecode y produce un `Result`.

```text
Compiled Program
       +
Invocation Values
       │
       ▼
Execute Compiled
       │
       ▼
     Result
```

Esta capacidad permite compilar una vez y ejecutar el mismo `Compiled Program` múltiples veces con diferentes `Invocation Values` y composiciones explícitas de capabilities compatibles.

### Invariants

- recibe un `Compiled Program` previamente producido por el Engine;
- no recibe `Source Text` y no recompila;
- los `Invocation Values` se enlazan con Parameters conforme a Evo-Script;
- cada invocación mantiene únicamente estado local de evaluación;
- no recibe, hereda, crea ni mantiene `Active Scope` de CLI, UI, API u otro Host;
- `Pipeline Data` pertenece al programa, no a una sesión interactiva exterior;
- las capacidades externas requeridas deben ser suministradas mediante bindings explícitos;
- el Engine no descubre Providers mediante registries ocultos, reflection o Service Locator;
- el outcome público es `Result`.

---

## 3. Execute Source

`Execute Source` recibe `Source Text` e `Invocation Values` y ejecuta el programa en una única operación pública, sin exigir al Consumer gestionar explícitamente un `Compiled Program`.

```text
Source Text
     +
Invocation Values
       │
       ▼
Execute Source
       │
       ▼
     Result
```

Conceptualmente:

```text
Source Text
    ↓
Compile
    ↓
Compiled Program
    ↓
Execute Compiled
    ↓
Result
```

### Invariants

- recibe Source Text completo, no una ruta de archivo;
- recibe los `Invocation Values` requeridos por la Public Function;
- el Consumer no necesita obtener ni administrar un `Compiled Program`;
- no expone representaciones intermedias de compilación;
- si la compilación falla, la Public Function no se ejecuta;
- si la ejecución requiere capacidades externas, estas llegan mediante bindings explícitos;
- no recibe ni hereda `Active Scope` del Consumer;
- el outcome público es `Result`.

---

## Consumer Neutrality

Las Public Capabilities no presuponen una superficie Consumer concreta.

```text
                         Consumer
                            │
             ┌──────────────┼──────────────┐
             │              │              │
             ▼              ▼              ▼
          Compile      Execute Compiled  Execute Source
             │              │              │
             ▼              ▼              ▼
      Compiled Program    Result          Result
```

Un Consumer puede ser `evo-cli`, `evo-ui`, `evo-api` u otro componente. El Engine no imprime, renderiza o serializa el `Result` por el Consumer. Esa reacción pertenece a la composición exterior, que posteriormente puede expresarse mediante Requesters, adapters u otros Participants definidos por la arquitectura.

---

## Internal Responsibilities Are Not Public Capabilities

Pueden existir internamente, pero no son Public Capabilities independientes:

- lexical analysis;
- parsing;
- semantic analysis;
- validation;
- AST construction;
- bytecode generation;
- bytecode execution;
- VM execution;
- external symbol resolution;
- external capability binding;
- external capability invocation;
- argument binding.

`Active Scope handling` deja de pertenecer a esta lista para `.efn` porque no forma parte de la ejecución del Engine.

---

## External Capabilities

`Compile` permanece independiente de una composición concreta de Providers. Un `Compiled Program` puede conservar External Symbols sin resolver.

```text
Source Text
    ↓
Compile
    ↓
Compiled Program
    └── External Symbols
```

La resolución e invocación ocurre durante ejecución mediante bindings explícitos de la aplicación:

```text
Execute Compiled / Execute Source
                │
                ▼
         External Symbol
                │
                ▼
     explicit Application Binding
                │
                ▼
             Provider
```

No existe un Provider activo ambiental dentro de `.efn`; distintas capabilities pueden ser utilizadas explícitamente durante la misma función.

---

## Closure

Las Public Capabilities v0 permanecen cerradas exactamente como:

- `Compile`
- `Execute Compiled`
- `Execute Source`

La eliminación de `Active Scope` de `.efn` no agrega ni elimina Public Capabilities. Reintroducir una sesión interactiva o Scope del Host dentro de cualquiera de estas operaciones requiere reabrir explícitamente la frontera funcional.
