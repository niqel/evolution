# UC-009 — Finalize Execution

Status: FUNCTIONAL

## Goal

Evo Runtime reconoce funcionalmente cuándo una Execution ha concluido todo su
trabajo y produce su Result final hacia el Host.

## Trigger

El trabajo funcional requerido directa o transitivamente por una Execution ha
concluido.

## Preconditions

- Existe una Execution activa.

## Main Flow

1. Evo Runtime reconoce que todas las actividades funcionales requeridas para la
   Execution han terminado.
2. El trabajo transitivo pendiente recibe la oportunidad de concluir de forma
   ordenada.
3. Evo Runtime determina el Result final de la Execution.
4. El Result final refleja el estado global: éxito (potencialmente conteniendo un
   Value) o fracaso (expresando un Failure).
5. El Host recibe el Result final de la Execution.
6. La Execution concluye y deja de estar activa.

## Successful Outcome

La Execution finaliza formalmente y el Host recibe el Result final.

## Failure Outcomes

- La conclusión incorrecta de actividades transitivas o un fallo no recuperado
  genera un Result final de tipo Failure.

## Invariants

- El retorno de la función de entrada (`entry return`) no equivale
  automáticamente a la finalización total de la Execution si existen tareas
  transitivas pendientes.
- Finalizar una Execution no afecta ni finaliza accidentalmente otra Execution
  distinta.
- `Result != Value`
- `Result != Failure`
- La finalización funcional no define una máquina de estados técnicos rígida.

## Related User Stories

- US-009 (Finalize an Execution)
- US-001 (Start an Application)
- US-007 (Propagate an Execution Failure)
- US-008 (Maintain an Execution Context)

## Related Data Dictionary Terms

- Execution
- Result
- Value
- Failure
- Host
- Execution Context
- Evo Runtime

## Out of Scope

- Contadores de referencias o tareas activas en memoria.
- Máquina de estados a bajo nivel.
- Planificadores de tareas o hilos.
- Protocolos de cancelación asíncrona.
- Destrucción de recursos físicos o implementación de `Drop` en Rust.
