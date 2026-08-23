# UC-004 — Invoke Operation

Status: FUNCTIONAL

## Goal

Evo Runtime coordina la participación efectiva de una Operation mediante su
Implementation resuelta y disponible durante una Execution.

## Trigger

Una Required Operation dispone de una Implementation resuelta y disponible para
su ejecución.

## Preconditions

- Existe una Execution activa.
- Existe una Required Operation.
- Existe una Implementation resuelta y disponible.
- Si la Implementation requiere un Engine, éste ha sido determinado previamente.

## Main Flow

1. Evo Runtime prepara la participación funcional de la Implementation.
2. Los Values de entrada provistos para la Operation se ponen a disposición de la
   Invocation.
3. La Implementation ejecuta el trabajo correspondiente (directamente o a través
   del Engine determinado).
4. La Invocation concluye produciendo un Result.
5. Si la Invocation es exitosa, puede producir un Value como resultado.
6. Si la Invocation no puede completarse correctamente, produce un Failure.
7. Cualquier nueva Required Operation originada transitivamente durante la
   Invocation vuelve a canalizarse a través de Evo Runtime.

## Successful Outcome

La Operation concluye produciendo un Result exitoso (potencialmente conteniendo
un Value).

## Failure Outcomes

- Se produce un Failure durante el trabajo de la Implementation.

## Invariants

- `Failure != successful Value`
- Una Implementation no resuelve ni ejecuta operaciones transitivas por fuera de
  Evo Runtime.
- La Invocation de una Operation no reemplaza la coordinación general de la
  Execution por parte de Evo Runtime.

## Related User Stories

- US-004 (Invoke an Operation)
- US-005 (Transport Values between Operations)
- US-007 (Propagate an Execution Failure)
- US-008 (Maintain an Execution Context)
- US-012 (Execute an Evo-Script Implementation)

## Related Data Dictionary Terms

- Invocation
- Operation
- Implementation
- Value
- Result
- Failure
- Required Operation
- Execution
- Evo Runtime

## Out of Scope

- Convenciones de llamada a bajo nivel (`extern "C"`, `fastcall`, etc.).
- Firmas Rust `fn` concretas.
- Estructura de pila (stack frames) o memoria.
- Planificación de tareas asíncronas o green threads.
- Scheduling multi-hilo.
