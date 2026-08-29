# Evo-Script Engine — Technical Data Model

Status: TECHNICAL DATA MODEL — IN PROGRESS

Este directorio contiene el **Technical Data Model** de `evo-script-engine` y sus **Technical Data Diagrams** en D2.

## Responsibility

El Technical Data Model transforma los conceptos del Functional Data Dictionary y las decisiones cerradas de Technical Design en representaciones técnicas concretas.

Aquí se definen, cuando corresponda:

- structs;
- enums;
- artifacts owned;
- borrowed views;
- aliases semánticos;
- ownership;
- borrowing;
- lifetimes;
- cardinalidades;
- relaciones entre tipos;
- datos técnicos internos requeridos por la implementación.

Regla:

> Toda estructura, enum, artifact o dato interno necesario para expresar una Rust Signature o implementar un Participant debe estar definido previamente en el Technical Data Model.

## Definition Order

El modelo se construye siguiendo el pipeline técnico cerrado:

```text
Source Text
    ↓
Lexical Data
    ↓
AST Data
    ↓
Semantic Program Data
    ↓
Compiled Program / Bytecode Data
    ↓
VM Execution Data
    ↓
Outcome / Diagnostic Data
```

Este orden no implica que todos los conceptos requieran un tipo independiente. Cada identidad debe justificarse por una necesidad real del diseño.

## Technical Data Diagram

El Technical Data Diagram es una vista D2 derivada del modelo.

Representa tipos y artifacts, no clases de comportamiento.

Puede utilizar categorías como:

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

No se representan métodos, herencia, service classes ni interfaces OO ficticias.

La metodología global se define en [`TECHNICAL_DESIGN_METHODOLOGY.md`](../../../../TECHNICAL_DESIGN_METHODOLOGY.md).
