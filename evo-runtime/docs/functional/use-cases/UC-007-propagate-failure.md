# UC-007 — Propagate Failure

Status: FUNCTIONAL

## Goal

Evo Runtime preserva y propaga un Failure producido durante una Execution para
evitar que el trabajo dependiente continúe como si existiera un Value exitoso.

## Trigger

Cualquier etapa funcional durante una Execution produce un Failure (por
ejemplo, resolución fallida, indisponibilidad de implementación, fallo de
motor, fallo en invocación, error en EvoS o error en EvoQ).

## Preconditions

- Existe una Execution activa.
- Se ha producido un Failure en alguna actividad de dicha Execution.

## Main Flow

1. Se origina un Failure en alguna etapa funcional coordinada por Evo Runtime.
2. Evo Runtime reconoce el Failure y mantiene la estricta distinción entre
   Failure y Value.
3. Evo Runtime propaga el Failure a través del flujo de ejecución correspondiente.
4. El trabajo dependiente que requería un Value inexistente no procede como si
   el resultado hubiera sido exitoso.
5. El Failure puede contribuir al Result fallido de una Operation o de la
   Execution, según corresponda.

## Successful Outcome

El Failure es propagado correctamente preservando su significado y su
distinción respecto de un Value exitoso.

## Failure Outcomes

No aplica un modelo de fallo secundario para este Use Case. La propagación
informa fielmente el Failure original.

## Invariants

- `Failure != Value`
- `Result != Failure`
- `Result != Value`
- Un Failure no debe transformarse silenciosamente en un Value exitoso.
- Propagar un Failure en una Operation no implica necesariamente finalizar
  toda la Evo Application.
- El Failure permanece asociado con la Execution correspondiente.

## Related User Stories

- US-007 (Propagate an Execution Failure)
- US-001 (Start an Application)
- US-002 (Resolve a Required Operation)
- US-003 (Load a Required Implementation)
- US-004 (Invoke an Operation)
- US-006 (Select an Engine for an Implementation)
- US-008 (Maintain an Execution Context)
- US-009 (Finalize an Execution)
- US-012 (Execute an Evo-Script Implementation)
- US-014 (Execute Query Work with EvoQ)

## Related Data Dictionary Terms

- Failure
- Result
- Value
- Execution
- Execution Context
- Evo Runtime

## Out of Scope

- Excepciones en Rust (`std::panic`, `catch_unwind`).
- Formato de stack traces o backtraces de depuración.
- Mecanismos de logging o telemetría.
- Políticas globales de reintentos (retries) o circuit breakers.
- Políticas de terminación de procesos del sistema operativo.
