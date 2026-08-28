# Evo-Script Engine — Public Capabilities

Status: FUNCTIONAL CLOSED

## Purpose

Este documento define las capacidades públicas que `evo-script-engine` ofrece a sus Consumers en v0.

Las Public Capabilities representan operaciones completas que tiene sentido solicitar al Engine desde fuera de su implementación. Las responsabilidades internas necesarias para cumplirlas, como análisis léxico, parsing, análisis semántico, generación de bytecode, ejecución de VM, binding de símbolos externos o manejo del Active Scope, no constituyen capacidades públicas independientes.

`evo-script-engine` v0 expone exactamente tres Public Capabilities:

1. `Compile`
2. `Execute Compiled`
3. `Execute Source`

---

## 1. Compile

`Compile` recibe el Source Text completo de un programa Evo-Script y produce un Compiled Program cuando el Source Text cumple la especificación vigente de `evo-script`.

```text
Source Text
    │
    ▼
  Compile
    │
    ▼
Compiled Program
```

Para producir el Compiled Program, el Engine realiza internamente todo el procesamiento requerido por el lenguaje, incluyendo análisis léxico, parsing, validación semántica y generación de bytecode.

### Invariants

- `Compile` recibe Source Text, no una ruta de archivo.
- `Compile` no realiza filesystem I/O.
- `Compile` valida el programa conforme a `evo-script`.
- `Compile` produce bytecode como representación ejecutable del Compiled Program.
- `Compile` no ejecuta la Public Function del programa.
- `Compile` no recibe Invocation Values.
- `Compile` no requiere Providers concretos.
- El Compiled Program puede conservar external symbols todavía no enlazados.
- El Compiled Program nunca conserva direcciones físicas de function pointers suministrados por una aplicación o Provider.

---

## 2. Execute Compiled

`Execute Compiled` recibe un Compiled Program y los Invocation Values requeridos por su Public Function, ejecuta el bytecode y produce un Result.

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

Esta capacidad permite compilar un programa una vez y ejecutar el mismo Compiled Program múltiples veces con diferentes Invocation Values.

### Invariants

- `Execute Compiled` recibe un Compiled Program previamente producido por el Engine.
- `Execute Compiled` no recibe Source Text.
- `Execute Compiled` no recompila el programa.
- Los Invocation Values se enlazan con los parámetros de la Public Function conforme a la semántica definida por `evo-script`.
- Cada invocación mantiene únicamente el estado local necesario para esa ejecución.
- Cuando la semántica lo requiera, la ejecución mantiene su propio Active Scope local.
- Si el programa requiere capacidades externas, estas deben ser suministradas mediante bindings explícitos.
- El Engine no descubre Providers ni capacidades mediante registries ocultos, reflection o Service Locator.
- El outcome público de la ejecución es Result.

---

## 3. Execute Source

`Execute Source` recibe Source Text e Invocation Values y ejecuta el programa en una única operación pública, sin exigir al Consumer gestionar explícitamente un Compiled Program.

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

Conceptualmente, `Execute Source` representa la composición de compilación y ejecución:

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

Esta composición conceptual no obliga a una estructura técnica específica distinta de las decisiones cerradas posteriormente en Technical Design.

### Invariants

- `Execute Source` recibe Source Text completo, no una ruta de archivo.
- `Execute Source` recibe los Invocation Values requeridos por la Public Function.
- El Consumer no necesita obtener ni administrar un Compiled Program.
- `Execute Source` no expone públicamente representaciones intermedias de compilación.
- Si la compilación del Source Text falla, la Public Function no se ejecuta.
- Si la ejecución requiere capacidades externas, estas deben llegar mediante bindings explícitos.
- El outcome público de la operación es Result.

---

## Public Boundary

La frontera pública completa de `evo-script-engine` v0 es:

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

                    evo-script-engine
```

No existen otras Public Capabilities en v0.

---

## Internal Responsibilities Are Not Public Capabilities

Las siguientes responsabilidades pueden existir dentro del Engine, pero no constituyen Public Capabilities independientes:

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
- Active Scope handling;
- argument binding.

Estas responsabilidades existen únicamente para implementar `Compile`, `Execute Compiled` y `Execute Source`.

---

## External Capabilities

`Compile` permanece independiente de una composición concreta de Providers. Un Compiled Program puede conservar external symbols sin resolver.

```text
Source Text
    ↓
Compile
    ↓
Compiled Program
    └── external symbols
```

La resolución e invocación de capacidades externas ocurre durante ejecución cuando sea necesaria y utiliza exclusivamente bindings explícitamente suministrados por la aplicación.

```text
Execute Compiled / Execute Source
                │
                ▼
         external symbol
                │
                ▼
     explicit Application Binding
                │
                ▼
             Provider
```

El Engine nunca descubre ni selecciona Providers concretos por mecanismos ocultos.

---

## Closure

Las Public Capabilities de `evo-script-engine` v0 quedan `FUNCTIONAL CLOSED` con exactamente estas tres operaciones:

- `Compile`
- `Execute Compiled`
- `Execute Source`

Los niveles posteriores de diseño pueden definir sus User Stories, Data Dictionary, Use Cases, firmas y participantes técnicos, pero no pueden agregar una nueva Public Capability sin reabrir explícitamente este cierre funcional.
