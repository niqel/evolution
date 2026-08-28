# Evo-Script Engine — Purpose

Status: FUNCTIONAL CLOSED

## Purpose

`evo-script-engine` es el motor operativo de Evo-Script.

Su propósito es implementar la especificación definida por `evo-script`: recibir programas Evo-Script, validarlos, compilarlos a bytecode y ejecutar ese bytecode de forma determinista para producir resultados semánticos de Evolution.

Cuando un programa requiere capacidades externas, el Engine las utiliza únicamente mediante capacidades y bindings explícitamente suministrados. El Engine no conoce, descubre ni selecciona Providers concretos por mecanismos ocultos.

En términos arquitectónicos:

```text
evo-script
    │
    │ especifica el lenguaje
    ▼
evo-script-engine
    │
    ├── análisis léxico
    ├── parsing
    ├── análisis semántico
    ├── compilación a bytecode
    └── ejecución del bytecode
            │
            ▼
          Result
```

## Responsibilities

Dentro de su propósito, `evo-script-engine` es responsable de:

- implementar las reglas léxicas definidas por `evo-script`;
- implementar las reglas sintácticas definidas por `evo-script`;
- implementar las reglas semánticas definidas por `evo-script`;
- compilar un programa Evo-Script válido a bytecode;
- ejecutar bytecode de Evo-Script;
- enlazar los Invocation Values con los parámetros de la Public Function correspondiente;
- mantener únicamente el estado local necesario para evaluar una ejecución de Evo-Script;
- mantener el Active Scope local cuando la semántica de la ejecución lo requiera;
- solicitar capacidades externas mediante bindings explícitos suministrados por la composición de la aplicación;
- producir los outcomes públicos definidos posteriormente por las Public Capabilities del Engine.

El Engine puede utilizar representaciones internas como tokens, AST u otras estructuras intermedias cuando sean necesarias para implementar la compilación. Estas representaciones no sustituyen al bytecode como representación ejecutable del Compiled Program.

## Non-Responsibilities

`evo-script-engine` no es responsable de:

- definir retrospectivamente la semántica de Evo-Script;
- leer archivos `.efn` desde filesystem;
- resolver rutas físicas;
- escribir o persistir archivos;
- imprimir en terminal o stdout;
- construir interfaces gráficas;
- serializar respuestas HTTP o JSON como responsabilidad propia;
- administrar el ciclo de vida de una Evo Application;
- descubrir Providers;
- mantener registries globales de Providers o capacidades;
- utilizar Service Locator, reflection o mecanismos equivalentes de descubrimiento oculto;
- poseer Providers concretos;
- implementar filesystem, database, network u otras infraestructuras externas;
- convertir una capacidad externa en parte implícita del lenguaje.

## Architectural Position

La relación normativa entre `evo-script` y `evo-script-engine` es:

```text
evo-script
    define QUÉ significa el lenguaje

                ↓

evo-script-engine
    hace QUE ese lenguaje funcione
```

La relación con capacidades externas es:

```text
Evo-Script Engine
        │
        │ capacidad explícitamente suministrada
        ▼
Application Binding / Requester
        │
        ▼
Standard Capability o Provider Extension
        │
        ▼
Provider
```

El Engine puede ejecutar operaciones que dependen de capacidades externas, pero no implementa esas capacidades ni conoce al Provider concreto que las resuelve.

## Invariants

- `evo-script` es la autoridad normativa del lenguaje; `evo-script-engine` es su implementación operativa.
- El Compiled Program de Evo-Script utiliza bytecode como representación ejecutable.
- El Engine no descubre dependencias ni Providers.
- Toda capacidad externa requerida por una ejecución debe llegar mediante bindings explícitos.
- El bytecode puede conservar símbolos externos, pero nunca direcciones físicas de function pointers de una aplicación o Provider.
- Una ejecución `.efn` mantiene su propio estado de evaluación y su propio Active Scope local cuando corresponda.
- El Engine no convierte presentación, infraestructura o Providers en semántica del lenguaje.

## Closure

Este Purpose se considera `FUNCTIONAL CLOSED`.

Los siguientes niveles de diseño no pueden ampliar la responsabilidad de `evo-script-engine` fuera de este propósito sin reabrir explícitamente esta decisión arquitectónica.
