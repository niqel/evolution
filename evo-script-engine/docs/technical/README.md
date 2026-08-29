# Evo-Script Engine — Technical Documentation

Status: TECHNICAL DESIGN — IN PROGRESS

Este directorio contiene la documentación técnica de `evo-script-engine`.

La fase funcional ya se encuentra cerrada. A partir de este punto, todo diseño técnico debe derivarse de los Functional Use Cases y del Functional Data Dictionary sin redefinir retrospectivamente su semántica.

La metodología técnica canónica de Evolution se encuentra en [`TECHNICAL_DESIGN_METHODOLOGY.md`](../../../TECHNICAL_DESIGN_METHODOLOGY.md).

Las decisiones estructurales vigentes del componente se registran en [`TECHNICAL_DESIGN.md`](TECHNICAL_DESIGN.md).

## Canonical Technical Sequence

```text
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

## Technical Views

Evolution utiliza tres vistas técnicas complementarias:

| Vista | Propósito |
| --- | --- |
| Technical Data Diagram | Representar structs, enums, artifacts, borrowed views y sus relaciones. |
| Module Signature Diagram | Representar módulos Rust como identidades arquitectónicas, sus firmas y relaciones. |
| D2 Sequence Diagram | Representar el orden de colaboración entre firmas durante una operación. |

No se utiliza UML Class Diagram como modelo primario. En Evolution el comportamiento pertenece a módulos y funciones; los datos pertenecen a tipos concretos.

## Module Identity Rule

Cuando un archivo Rust representa una responsabilidad arquitectónica:

```text
archivo.rs
    = módulo
    = identidad arquitectónica
```

No se crea una `struct` artificial únicamente para dar identidad a una responsabilidad sin estado.

Ejemplo:

```text
definitions/use_cases/compile.rs
    └── Compile

agents/compiler.rs
    ├── compile(...)
    └── COMPILE: compile::Compile = compile
```

## Directory Organization

```text
technical/
├── README.md
├── TECHNICAL_DESIGN.md
├── data-model/
│   └── Technical Data Model + Technical Data Diagrams
├── signatures/
│   └── Rust Signatures
├── module-signatures/
│   └── Module Signature Diagrams
└── sequences/
    └── D2 Sequence Diagrams
```

## Traceability Rules

1. Todo dato usado por una Rust Signature debe existir previamente en el Technical Data Model.
2. Toda identidad del Module Signature Diagram debe corresponder a un módulo Rust real previsto.
3. Toda interacción del D2 Sequence Diagram debe corresponder a una firma explícita o llamada técnica definida.
4. Los diagramas se derivan del diseño cerrado; no son dibujos independientes del código.
5. Si firma, módulo y diagrama divergen, el diseño no puede considerarse cerrado.

## Current Progress

```text
Functional Design         ✅ CLOSED
Technical Design          ← IN PROGRESS
Technical Data Model      BLOCKED BY OPEN TECHNICAL DECISIONS
Technical Data Diagram    PENDING
Rust Signatures           PENDING
Participants              PENDING
Module Signature Diagram  PENDING
D2 Sequence Diagrams      PENDING
Implementation Tasks      PENDING
```

Las decisiones ya cerradas incluyen Stack VM, Semantic Program como identidad técnica y única IR semántica de v0, resolución de funciones internas durante compilación, forma arquitectónica de Compiled Program y Constant Pool owned.

Las siguientes decisiones abiertas son el modelo Operand Stack / Call Frame, Parameters / Locals y ownership de Values provenientes de External Capabilities.
