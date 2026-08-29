# Evolution — Technical Design Methodology

Status: CANONICAL

Este documento define la metodología técnica canónica de Evolution para transformar un modelo funcional cerrado en una arquitectura Rust modular, funcional y verificable.

Evolution no adopta UML orientado a clases como modelo primario. La arquitectura se expresa mediante datos, módulos, firmas y secuencias porque esas son las identidades reales del código.

## 1. Principio de Identidad Modular

En Evolution, cuando un archivo Rust representa una responsabilidad arquitectónica, el módulo es una identidad de diseño.

```text
archivo.rs
    = módulo
    = identidad arquitectónica
```

Un módulo no necesita convertirse en `struct` para tener identidad. Cuando una responsabilidad no posee estado propio ni identidad de instancia, se representa preferentemente mediante módulo + función.

Ejemplo:

```text
definitions/use_cases/compile.rs
    └── Compile

agents/compiler.rs
    ├── compile(...)
    └── COMPILE: compile::Compile = compile
```

El módulo `compiler` es la identidad del Agent. No se crea una clase o `struct Compiler` únicamente para imitar orientación a objetos.

## 2. Canonical Design Sequence

```text
                    FUNCTIONAL

Purpose
   ↓
Public Capabilities
   ↓
User Stories
   ↓
Functional Data Dictionary
   ↓
Functional Use Cases
   ↓
──────────── FUNCTIONAL CLOSED ────────────

                     TECHNICAL

Technical Design
   ↓
Technical Data Model
   ↓
Technical Data Diagram
   ↓
Rust Signatures
   ↓
Participants
   ↓
Module Signature Diagram
   ↓
D2 Sequence Diagrams
   ↓
Implementation Tasks
```

Cada etapa cerrada es autoridad para la siguiente. El diseño técnico no redefine retrospectivamente la semántica funcional.

## 3. Technical Data Model

El Technical Data Model convierte conceptos definidos funcionalmente en representaciones técnicas concretas.

Debe decidir, cuando corresponda:

- `struct`;
- `enum`;
- artifact owned;
- borrowed view;
- semantic alias;
- secuencias concretas;
- ownership;
- borrowing;
- lifetimes;
- cardinalidades;
- relaciones entre tipos.

Regla:

> Toda estructura, enum, artifact o dato interno necesario para expresar una Rust Signature o implementar un Participant debe estar definido previamente en el Technical Data Model.

Conceptos técnicos internos como `Token`, `AST`, `Instruction`, `Opcode`, `Stack Frame` o `VM State` pertenecen aquí cuando el Technical Design demuestra que son necesarios.

## 4. Technical Data Diagram

El **Technical Data Diagram** es el equivalente de Evolution a la parte de datos de un UML Class Diagram.

Su unidad primaria son tipos y artifacts, no clases con comportamiento.

Puede representar:

```text
<<struct>>
<<enum>>
<<artifact>>
<<borrowed view>>
<<alias>>
```

Y relaciones como:

```text
contains
references
borrows
owns
variant-of
0..1
0..N
```

Ejemplo conceptual:

```text
┌────────────────────────────┐
│ <<artifact>>               │
│ CompiledProgram            │
├────────────────────────────┤
│ bytecode                   │
│ external_symbols 0..N      │
└─────────────┬──────────────┘
              │ contains
              ▼
┌────────────────────────────┐
│ <<struct>>                 │
│ ExternalSymbol             │
├────────────────────────────┤
│ scope                      │
│ operation                  │
└────────────────────────────┘
```

El Technical Data Diagram no modela métodos, herencia, clases de servicio ni interfaces OO ficticias.

## 5. Rust Signatures

Las Rust Signatures materializan las operaciones arquitectónicas mediante function pointers.

```rust
pub type Operation = fn(Input, Request, Contract) -> Outcome;
```

Aquí se cierran:

- tipos concretos;
- orden de argumentos;
- ownership;
- borrowing;
- lifetimes;
- Requesters;
- Contracts;
- outcomes;
- errores semánticos;
- cualquier dependencia explícita requerida por la operación.

Una firma no puede depender de un dato que no exista previamente en el Technical Data Model.

## 6. Participants

Una vez cerradas las firmas se identifican los Participants necesarios:

```text
Use Cases
Agents
Requesters
Collaborators
Resolvers
Contracts
Tools
```

Los Participants no se inventan por plantilla. Se derivan de las responsabilidades y dependencias demostradas por los Use Cases y sus firmas.

## 7. Module Signature Diagram

El **Module Signature Diagram** es el equivalente arquitectónico de Evolution a un Class Diagram de comportamiento.

Su unidad primaria es el módulo Rust, no una clase ni una instancia.

Cada nodo debe corresponder a una identidad modular real prevista en el código y puede mostrar:

- path del módulo;
- categoría arquitectónica;
- firma principal;
- función concreta;
- binding tipado cuando corresponda.

Ejemplo conceptual:

```text
┌────────────────────────────────┐
│ <<Use Case Module>>            │
│ definitions/use_cases/compile  │
├────────────────────────────────┤
│ Compile                        │
│ fn(...) -> ...                 │
└───────────────┬────────────────┘
                │ implemented by
                ▼
┌────────────────────────────────┐
│ <<Agent Module>>               │
│ agents/compiler                │
├────────────────────────────────┤
│ compile(...)                   │
│ COMPILE: Compile = compile     │
└────────────────────────────────┘
```

Relaciones permitidas incluyen, según corresponda:

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

> Toda identidad mostrada en un Module Signature Diagram debe corresponder a un módulo real previsto en la arquitectura.

No deben dibujarse managers, services, interfaces o clases que no existan realmente en el diseño Rust.

## 8. D2 Sequence Diagrams

Los D2 Sequence Diagrams representan el comportamiento dinámico de una operación ya diseñada.

Responden:

> ¿Qué firma invoca a qué firma, en qué orden y con qué datos o capacidades?

Las lifelines representan Consumers, módulos participantes o fronteras técnicas reales. Las flechas deben identificar la acción/firma invocada.

Ejemplo conceptual:

```text
Consumer
   │ Execute Compiled
   ▼
Agent
   │ Collaborate
   ▼
Collaborator
   │ Resolve
   ▼
Resolver
   │ Provide
   ▼
Contract / Provider boundary
```

Regla:

> Toda interacción mostrada en un Sequence Diagram debe corresponder a una firma explícita o a una llamada técnica explícitamente definida por el diseño.

El diagrama no puede inventar pasos ocultos que las firmas no expresan.

## 9. Las Tres Vistas Técnicas

Cada vista responde una pregunta distinta:

| Vista | Pregunta |
| --- | --- |
| Technical Data Diagram | ¿Qué datos existen y cómo se relacionan? |
| Module Signature Diagram | ¿Qué módulos arquitectónicos existen y qué firma/rol representa cada uno? |
| D2 Sequence Diagram | ¿Cómo colaboran esas firmas durante una operación? |

Las tres deben ser coherentes entre sí.

## 10. Reglas de Trazabilidad

```text
Functional Use Case
        ↓
Technical Data Model
        ↓
Technical Data Diagram
        ↓
Rust Signatures
        ↓
Participants
        ↓
Module Signature Diagram
        ↓
D2 Sequence Diagrams
        ↓
Rust files
```

Invariantes:

1. Todo dato presente en una Rust Signature debe existir en el Technical Data Model.
2. Toda relación de datos arquitectónicamente relevante debe poder representarse en el Technical Data Diagram.
3. Todo Participant arquitectónico debe tener una identidad modular prevista.
4. Todo módulo mostrado en el Module Signature Diagram debe corresponder a un módulo Rust real previsto.
5. Todo Agent debe implementar exactamente su Use Case mediante la firma cerrada y su binding tipado.
6. Toda interacción del Sequence Diagram debe poder rastrearse a una firma explícita.
7. Los diagramas se derivan del diseño; no se diseña código para satisfacer dibujos inventados previamente.
8. Si código, firma y diagrama divergen, la divergencia debe resolverse explícitamente antes de cerrar el diseño.

## 11. D2 como Herramienta Canónica

Evolution utiliza D2 para los diagramas técnicos versionables.

Los archivos fuente `.d2` forman parte de la documentación arquitectónica y deben mantenerse junto a la especificación que representan.

Los diagramas son artefactos de diseño, no imágenes decorativas: deben poder reconstruirse a partir del Technical Data Model, Rust Signatures y Participants cerrados.
