# UC-011 — Execute Evo-Script Implementation

Status: FUNCTIONAL

## Goal

Evo Runtime utiliza EvoS para ejecutar una Implementation escrita en Evo-Script
durante una Execution sin que la Evo Application administre directamente el
Engine.

## Trigger

Una Required Operation ha sido resuelta hacia una Evo-Script Implementation
que debe participar en la Execution.

## Preconditions

- Existe una Execution activa.
- Existe una Evo-Script Implementation resuelta y disponible.
- EvoS forma parte del Core estático disponible para Evo Runtime.

## Main Flow

1. Evo Runtime reconoce que la Implementation disponible es una Evo-Script
   Implementation.
2. Evo Runtime utiliza EvoS.
3. Los Values de entrada provistos participan sobre la base común de EvoV.
4. EvoS ejecuta la Evo-Script Implementation.
5. El trabajo ejecutado produce un Result (Value en caso exitoso o Failure si
   ocurre un error).
6. El outcome producido regresa a Evo Runtime para continuar en el flujo de la
   Execution.
7. Si durante la ejecución la implementación requiere nuevas Operations
   transitivas, éstas se canalizan a través de Evo Runtime.
8. Todo el trabajo ejecutado por EvoS permanece dentro del mismo Execution
   Context activo.

## Successful Outcome

EvoS ejecuta correctamente la implementación Evo-Script y produce un Result
exitoso (potencialmente conteniendo un Value).

## Failure Outcomes

- Se produce un Failure durante la ejecución de la Evo-Script Implementation.

## Invariants

- `Evo-Script != EvoS`
- `EvoS = Engine`
- `EvoS != Evo Runtime`
- `EvoV != Engine`
- Evo Runtime coordina la ejecución; EvoS ejecuta Evo-Script Implementations.
- EvoS no asume la coordinación global de dependencias ni el ciclo de vida de la
  Execution.
- La Evo Application no selecciona, crea ni administra directamente el engine
  EvoS.

## Related User Stories

- US-012 (Execute an Evo-Script Implementation through EvoS)
- US-011 (Provide the Evo Base Core)
- US-013 (Share Values across Core Components through EvoV)
- US-008 (Maintain an Execution Context)
- US-007 (Propagate an Execution Failure)

## Related Data Dictionary Terms

- Evo-Script
- Evo-Script Implementation
- EvoS
- EvoV
- Evo Runtime
- Implementation
- Value
- Failure
- Result
- Execution
- Execution Context
- Required Operation

## Out of Scope

- Sintaxis, gramática y léxico de Evo-Script.
- Parser, lexer o AST de Evo-Script.
- Implementación interna de la máquina virtual o intérprete de Evo-Script.
- Bytecode o compilador JIT/AOT.
- API técnica en Rust de `evo-script-engine`.
