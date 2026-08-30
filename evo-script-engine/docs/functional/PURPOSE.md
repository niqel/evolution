# Evo-Script Engine — Purpose

Status: FUNCTIONAL CLOSED — REVALIDATED AFTER EFN HOST BOUNDARY

## Purpose

`evo-script-engine` es el motor operativo de Evo-Script.

Su propósito es implementar la especificación definida por `evo-script`: recibir programas Evo-Script, validarlos, compilarlos a bytecode y ejecutar ese bytecode de forma determinista para producir resultados semánticos de Evolution.

Cuando un programa requiere capacidades externas, el Engine las utiliza únicamente mediante capacidades y bindings explícitamente suministrados. El Engine no conoce, descubre ni selecciona Providers concretos por mecanismos ocultos.

La frontera `.efn` / Host se rige por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.

```text
evo-script
    │ specifies language
    ▼
evo-script-engine
    ├── lexical analysis
    ├── parsing
    ├── semantic analysis
    ├── bytecode compilation
    └── bytecode execution
            │
            ▼
          Result
```

## Responsibilities

`evo-script-engine` es responsable de:

- implementar las reglas léxicas, sintácticas y semánticas vigentes de Evo-Script;
- compilar un programa válido a bytecode;
- ejecutar bytecode de Evo-Script;
- enlazar `Invocation Values` con los Parameters de la Public Function correspondiente;
- mantener únicamente el estado local necesario para evaluar una invocación;
- mantener `Pipeline Data` y demás estado de evaluación requerido por el programa;
- solicitar capacidades externas únicamente mediante bindings explícitos suministrados por la composición de la aplicación;
- producir los outcomes públicos de sus Public Capabilities.

El Engine puede utilizar Tokens, AST, Semantic Program y otras representaciones temporales de compilación cuando estén justificadas. Estas representaciones no sustituyen al bytecode como representación ejecutable del `Compiled Program`.

## `.efn` / Host Boundary

Una ejecución `.efn` no es una Interactive Session.

```text
Host / Consumer
    │ Invocation Values + explicit capability bindings
    ▼
evo-script-engine
    │ executes reusable .efn
    ▼
Result
    │
    ▼
Host / Consumer decides presentation or use
```

Invariantes:

- el Engine no recibe, hereda, crea ni mantiene `Active Scope` para ejecutar `.efn`;
- `Scope` y `Active Scope` pertenecen al Host/Shell interactivo cuando dicho Host necesita contexto persistente entre comandos;
- el estado interactivo del Host no cruza implícitamente la frontera de ejecución;
- `use` no forma parte de la gramática `.efn` vigente;
- la navegación ambiental de Scope no forma parte de la ejecución `.efn`;
- un `.efn` utiliza capacidades semánticas explícitas y puede utilizar varias durante una misma función sin seleccionar un Provider activo;
- el Consumer puede ser CLI, UI, API u otro Host sin cambiar la semántica del `.efn`.

## Non-Responsibilities

`evo-script-engine` no es responsable de:

- definir retrospectivamente la semántica de Evo-Script;
- leer archivos `.efn` desde filesystem ni resolver sus rutas físicas;
- escribir o persistir archivos como responsabilidad del Engine;
- imprimir en terminal o stdout;
- construir interfaces gráficas;
- serializar respuestas HTTP/JSON como responsabilidad propia;
- mantener prompts, sesiones interactivas o `Active Scope` de un Host;
- administrar el ciclo de vida de una Evo Application;
- descubrir Providers;
- mantener registries globales de Providers o capacidades;
- utilizar Service Locator, reflection o mecanismos equivalentes de descubrimiento oculto;
- poseer Providers concretos;
- implementar filesystem, database, network u otras infraestructuras externas;
- convertir una capacidad externa o una superficie Consumer en dependencia implícita del lenguaje.

## Architectural Position

```text
evo-script
    defines WHAT the language means

        ↓

evo-script-engine
    makes that language operational
```

Relación con capacidades externas:

```text
Evo-Script Engine
        │ explicit capability requirement
        ▼
Application Binding
        │
        ▼
Standard Capability / Provider Extension
        │
        ▼
Provider
```

Relación con Consumers:

```text
             reusable .efn Result
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
     evo-cli      evo-ui      evo-api
        │           │           │
     present      render      respond
```

El Engine ejecuta la operación y entrega su `Result`; la presentación o reacción específica pertenece al Consumer y a su composición arquitectónica.

## Invariants

- `evo-script` es la autoridad normativa del lenguaje; `evo-script-engine` es su implementación operativa.
- El `Compiled Program` utiliza bytecode como representación ejecutable.
- El Engine no descubre dependencias ni Providers.
- Toda capacidad externa requerida debe llegar mediante bindings explícitos.
- El bytecode puede conservar `External Symbols`, pero nunca direcciones físicas de function pointers de una aplicación o Provider.
- Cada ejecución `.efn` mantiene únicamente su propio estado local de evaluación; no mantiene `Active Scope`.
- `Pipeline Data` pertenece a la evaluación del programa y no representa estado de Host.
- El Engine no convierte presentación, infraestructura, Provider concreto o sesión interactiva en semántica implícita del `.efn`.

## Closure

Este Purpose queda `FUNCTIONAL CLOSED` bajo la nueva frontera `.efn` / Host.

Reintroducir `Active Scope`, `use` o estado interactivo de Host dentro de `.efn` requiere reabrir explícitamente esta decisión arquitectónica y `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.
