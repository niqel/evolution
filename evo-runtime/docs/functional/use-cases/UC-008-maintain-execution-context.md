# UC-008 — Maintain Execution Context

Status: FUNCTIONAL

## Goal

Evo Runtime mantiene la continuidad funcional (Execution Context) que permite
reconocer que múltiples actividades y operaciones transitivas pertenecen a una
misma Execution.

## Trigger

Una Execution está activa y genera trabajo directo o transitivo a través de
Evo Runtime.

## Preconditions

- Existe una Execution iniciada y activa.

## Main Flow

1. Evo Runtime asocia y mantiene el Execution Context correspondiente a la
   Execution activa.
2. Las Required Operations originadas transitivamente durante la ejecución
   permanecen asociadas con dicho contexto.
3. Las Implementations resueltas, Values transportados y Failures producidos
   permanecen vinculados a esa misma Execution.
4. El trabajo ejecutado mediante EvoS o EvoQ se desarrolla dentro de la
   continuidad del mismo Execution Context.
5. Otra Execution mantiene un Execution Context distinto.

## Successful Outcome

La continuidad funcional de la Execution se preserva de manera coherente a lo
largo de todas sus actividades directas y transitivas.

## Failure Outcomes

La pérdida de la continuidad necesaria impide reconocer correctamente el
trabajo como perteneciente a la misma Execution.

## Invariants

- `Execution Context != Scope`
- Una Execution no depende accidentalmente del Execution Context de otra
  Execution distinta.
- Las unidades participantes no crean ni administran directamente la estructura
  interna del contexto de Evo Runtime.
- No se definen identificadores físicos ni estructuras de datos rígidas en esta
  fase funcional.

## Related User Stories

- US-008 (Maintain an Execution Context)
- US-009 (Finalize an Execution)
- US-012 (Execute an Evo-Script Implementation)
- US-014 (Execute Query Work with EvoQ)

## Related Data Dictionary Terms

- Execution
- Execution Context
- Required Operation
- Implementation
- Value
- Failure
- EvoQ
- EvoS
- Evo Runtime

## Out of Scope

- Estructuras técnicas concretas (`ExecutionContext`, `ExecutionId`).
- Almacenamiento local a hilos (Thread-Local Storage o Task-Local Storage).
- Modelos de `Scope` o ámbitos de variables.
- Gestión de memoria o propiedad en Rust.
- Ciclo de vida técnico a bajo nivel.
