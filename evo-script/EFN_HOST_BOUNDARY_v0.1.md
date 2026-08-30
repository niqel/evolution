# Evo-Script `.efn` — Host Boundary v0.1

Status: NORMATIVE — CLOSED

Este documento tiene precedencia sobre reglas anteriores de `EVO_SCRIPT_SPECIFICATION_v0.1.md` que coloquen `Scope`, `Active Scope`, `use` o navegación de sesión interactiva dentro de un archivo `.efn`.

## Reusable Program Unit

Un `.efn` es una unidad reusable de programa Evo-Script. Recibe entradas explícitas, utiliza dependencias semánticas explícitas y produce un `Result` semántico. No es una Interactive Session y no posee prompt ni estado heredado de CLI, UI, API u otro Host.

## Scope Boundary

`Scope` y `Active Scope` pertenecen al Host interactivo cuando este necesita conservar un contexto entre comandos, por ejemplo Evo-Shell o Evo-CLI.

```text
Interactive Host Session
    may own Active Scope

.efn execution
    does not own Active Scope
```

Un Host no inyecta implícitamente su Active Scope en una ejecución `.efn`.

## `use` Removed from `.efn`

`use` deja de ser una construcción válida dentro de `.efn`. No activa Scope, no selecciona Provider, no cambia module y no selecciona Consumer. La palabra deja de ser Structural Keyword de la gramática `.efn`; cuando cumpla la gramática de Identifier puede clasificarse como `Identifier` sin significado especial.

Ejemplos históricos como `use documents |> ...` o `data |> use terminal |> print` quedan inválidos dentro de `.efn`.

## No Ambient `enter`

La semántica histórica de `enter(target)` como modificación de la ubicación de Active Scope tampoco existe dentro de `.efn`. `enter` puede existir como Identifier de una función importada, pero entonces es una capacidad explícita ordinaria y no navegación de estado ambiental.

## Explicit Capabilities

Un `.efn` puede utilizar múltiples capacidades en una misma función sin cambiar Scope o Provider activo. `import` declara símbolos semánticos publicados, no crates Rust ni Providers concretos. El binding a Providers pertenece a la composición externa de la aplicación durante ejecución.

## Consumer Neutrality

Una función `.efn` no conoce si la invoca `evo-cli`, `evo-ui`, `evo-api` u otro Consumer. Produce datos semánticos; presentación, impresión, renderizado o serialización pertenecen al Consumer. Requesters y adapters pueden existir en la composición externa, pero no forman parte de la sintaxis `.efn`.

## Pipeline

Dentro de `.efn`, Pipeline representa composición de datos. No existe un canal paralelo de Active Scope dentro de la ejecución.

`this` permanece como marcador sintáctico contextual del valor transportado por Pipeline y no representa Scope, Consumer ni estado de Host.

## Engine Boundary

`Compile`, `Execute Compiled` y `Execute Source` no reciben, crean, heredan ni mantienen Active Scope para ejecutar `.efn`.

## Closure

```text
Reusable `.efn`                         ✅ CLOSED
No Active Scope inside `.efn`          ✅ CLOSED
No inherited Host Scope                ✅ CLOSED
`use` removed from `.efn`              ✅ CLOSED
No ambient `enter` semantics           ✅ CLOSED
Explicit semantic capabilities         ✅ CLOSED
Consumer-neutral Result                ✅ CLOSED
Pipeline is data composition           ✅ CLOSED
`this` remains pipeline syntax         ✅ CLOSED
```
