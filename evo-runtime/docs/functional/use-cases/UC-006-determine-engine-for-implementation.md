# UC-006 — Determine Engine for Implementation

Status: FUNCTIONAL

## Goal

Evo Runtime determina el Engine compatible requerido por una Implementation
cuando dicha Implementation necesita un mecanismo de ejecución específico.

## Trigger

Una Implementation disponible requiere un Engine para ejecutar su trabajo.

## Preconditions

- Existe una Execution activa.
- Existe una Implementation resuelta y disponible que requiere un Engine.

## Main Flow

1. Evo Runtime reconoce que la Implementation disponible requiere un Engine para
   su ejecución.
2. Evo Runtime determina qué Engine compatible está disponible (por ejemplo,
   EvoS para una Implementation Evo-Script).
3. Si existe una determinación válida, el Engine queda asociado funcionalmente
   con la ejecución de la Implementation.
4. La Implementation puede continuar hacia Invocation con el Engine
   determinado.

## Successful Outcome

Un Engine compatible queda determinado para la Implementation.

## Failure Outcomes

- No existe un Engine compatible para la Implementation requerida.
- Existen alternativas ambiguas sin una regla funcional suficiente para
  determinar el Engine.

## Invariants

- `Implementation != Engine`
- `EvoV != Engine`
- No toda Implementation requiere obligatoriamente un Engine.
- Evo Runtime determina el Engine compatible sin realizar elecciones
  arbitrarias.
- Esta acción no define un registro dinámico ni prioridades arbitrarias en
  Model A.

## Related User Stories

- US-006 (Select an Engine for an Implementation)
- US-011 (Provide the Evo Base Core)
- US-012 (Execute an Evo-Script Implementation)
- US-014 (Execute Query Work with EvoQ)

## Related Data Dictionary Terms

- Engine
- Implementation
- Evo Runtime
- EvoQ
- EvoS
- Failure

## Out of Scope

- Registro dinámico de motores (engine registry).
- Descubrimiento de motores vía plugins.
- Algoritmos de ranking o prioridad entre múltiples motores.
- Inicialización física o ciclo de vida del motor.
- Estructuras o traits en Rust para Engine.
