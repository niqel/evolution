# Evo-Script Engine — Module Signature Diagrams

Status: TECHNICAL DESIGN — NOT STARTED

Este directorio contendrá los **Module Signature Diagrams** de `evo-script-engine` en D2.

## Module Identity

En Evolution, cuando un archivo Rust representa una responsabilidad arquitectónica:

```text
archivo.rs
    = módulo
    = identidad arquitectónica
```

La unidad primaria del diagrama es el módulo Rust, no una clase ni una instancia.

Cada nodo puede mostrar:

- path del módulo;
- categoría arquitectónica (`Use Case`, `Agent`, `Requester`, `Collaborator`, `Resolver`, `Contract`, `Tool`);
- firma principal;
- función concreta;
- binding tipado cuando corresponda.

Ejemplo conceptual:

```text
<<Use Case Module>>
definitions/use_cases/compile
    Compile
        │ implemented by
        ▼
<<Agent Module>>
agents/compiler
    compile(...)
    COMPILE: Compile = compile
```

Relaciones posibles incluyen:

```text
defines
implemented by
coordinates
collaborates with
resolves through
requires Contract
responds through Requester
uses Tool
```

Regla:

> Toda identidad mostrada en un Module Signature Diagram debe corresponder a un módulo Rust real previsto en la arquitectura.

No deben modelarse clases de servicio, managers o interfaces OO inexistentes en el código.
