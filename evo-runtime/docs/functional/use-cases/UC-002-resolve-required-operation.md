# UC-002 — Resolve Required Operation

Status: FUNCTIONAL

## Goal

Evo Runtime resuelve una Required Operation hacia exactamente una
Implementation válida sin que la unidad solicitante necesite conocerla o
seleccionarla directamente.

## Trigger

Durante una Execution activa, una unidad participante requiere una Operation.

## Preconditions

- Existe una Execution activa.
- Existe una Required Operation solicitada dentro de dicha Execution.

## Main Flow

1. Una unidad participante requiere una Operation.
2. La solicitud de resolución llega a Evo Runtime.
3. Evo Runtime determina qué Implementation disponible o potencial puede
   satisfacer la Required Operation.
4. Si existe exactamente una resolución válida, Evo Runtime selecciona esa
   Implementation funcionalmente.
5. La Implementation resuelta queda determinada y lista para proceder hacia
   disponibilidad e Invocation.

## Successful Outcome

Exactamente una Implementation válida queda resuelta para la Required Operation.

## Failure Outcomes

- No existe ninguna Implementation válida para satisfacer la Required
  Operation.
- Existen múltiples Implementations alternativas sin una regla suficiente para
  resolver la ambigüedad.

## Invariants

- `Required Operation != Implementation`
- `Required Operation != Capability`
- La unidad solicitante no selecciona arbitrariamente la Implementation
  concreta.
- Evo Runtime no elige arbitrariamente entre alternativas ambiguas.
- La resolución exitosa produce una única Implementation válida.

## Related User Stories

- US-002 (Resolve a Required Operation)
- US-003 (Load a Required Implementation)
- US-004 (Invoke an Operation)
- US-007 (Propagate an Execution Failure)
- US-012 (Execute an Evo-Script Implementation)

## Related Data Dictionary Terms

- Required Operation
- Operation
- Implementation
- Failure
- Execution
- Evo Runtime

## Out of Scope

- Registries internos, tablas de símbolos o mecanismos de lookup físicos.
- Formatos de manifiestos o archivos de configuración.
- Algoritmos concretos de prioridad o reglas de resolución avanzadas.
- Punteros a función o tablas virtuales en Rust.
- Mapeo técnico entre Required Operation y Capabilities.
