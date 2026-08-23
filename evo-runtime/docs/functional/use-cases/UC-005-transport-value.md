# UC-005 — Transport Value

Status: FUNCTIONAL

## Goal

Evo Runtime permite que un Value producido durante una Execution continúe
participando como información para trabajo posterior sin perder su significado
funcional.

## Trigger

Una Operation o componente del Core produce o proporciona un Value que debe
participar en etapas posteriores de la Execution.

## Preconditions

- Existe una Execution activa.
- Existe un Value válido producido o provisto sobre la base común de EvoV.

## Main Flow

1. Un Value es producido o provisto durante el trabajo correcto de una
   Operation, EvoS o EvoQ.
2. El Value cruza una frontera funcional coordinada por Evo Runtime.
3. Evo Runtime preserva el significado funcional del Value.
4. El Value puede continuar hacia otra Operation, hacia trabajo ejecutado
   mediante EvoS o hacia Query Work realizado mediante EvoQ, según corresponda.
5. El Value permanece funcionalmente asociado con la misma Execution activa.

## Successful Outcome

El Value participa en el trabajo subsiguiente conservando íntegramente su
significado funcional.

## Failure Outcomes

No existe un Failure inherente al concepto de transporte funcional. Si la
operación que recibe o procesa el Value falla, dicho fallo produce un Failure
independiente que continúa siendo distinto del Value.

## Invariants

- `Value != Failure`
- `Value != Result`
- `functional Value transport != physical copy`
- `same Value meaning != same memory instance`
- EvoV proporciona la base común de Values para todo el Core.
- El transporte funcional no define transferencia de ownership técnica, copia
  física ni serialización.

## Related User Stories

- US-005 (Transport Values between Operations)
- US-008 (Maintain an Execution Context)
- US-013 (Share Values across Core Components)
- US-012 (Execute an Evo-Script Implementation)
- US-014 (Execute Query Work with EvoQ)

## Related Data Dictionary Terms

- Value
- EvoV
- Execution
- Execution Context
- Result
- Failure
- Evo Runtime
- EvoQ
- EvoS

## Out of Scope

- Duplicación física en memoria (`Clone`, `Copy`).
- Semántica de propiedad o préstamos en Rust (`ownership`, `borrowing`, `lifetimes`).
- Dirección de memoria o punteros.
- Serialización / deserialización binaria o textual.
- Diseño de interfaces binarias (ABI).
