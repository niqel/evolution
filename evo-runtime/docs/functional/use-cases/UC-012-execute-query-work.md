# UC-012 — Execute Query Work

Status: FUNCTIONAL

## Goal

Evo Runtime utiliza EvoQ para realizar Query Work requerido durante una
Execution sin que la unidad solicitante administre directamente el Engine de
consultas.

## Trigger

Durante una Execution activa se requiere realizar Query Work.

## Preconditions

- Existe una Execution activa.
- EvoQ forma parte del Core estático disponible para Evo Runtime.

## Main Flow

1. Una unidad participante requiere realizar Query Work.
2. La necesidad es atendida y coordinada mediante Evo Runtime.
3. Evo Runtime utiliza EvoQ.
4. Si el Query Work requiere Values de entrada, EvoQ trabaja con ellos sobre la
   base común de EvoV.
5. EvoQ realiza el Query Work correspondiente.
6. EvoQ produce un Result (Value en caso exitoso o Failure si ocurre un error).
7. El outcome producido regresa a Evo Runtime para continuar participando en la
   Execution.
8. Todo el trabajo ejecutado por EvoQ se mantiene dentro del mismo Execution
   Context activo.
9. Cualquier nueva Required Operation requerida durante o después del trabajo de
   consulta se atiende a través de Evo Runtime.

## Successful Outcome

EvoQ ejecuta el trabajo de consulta y produce un Result exitoso (potencialmente
conteniendo un Value).

## Failure Outcomes

- Se produce un Failure durante Query Work.

## Invariants

- `EvoQ = Engine`
- `EvoQ != Evo Runtime`
- `EvoV != Engine`
- EvoQ es un engine base del Core y no requiere discovery dinámico como Provider
  adicional en Model A.
- EvoQ no asume la coordinación global de la Execution.
- La unidad solicitante no selecciona, crea ni administra directamente el engine
  EvoQ.

## Related User Stories

- US-014 (Execute Query Work with EvoQ)
- US-011 (Provide the Evo Base Core)
- US-013 (Share Values across Core Components through EvoV)
- US-008 (Maintain an Execution Context)
- US-007 (Propagate an Execution Failure)

## Related Data Dictionary Terms

- Query Work
- EvoQ
- EvoV
- Evo Runtime
- Value
- Result
- Failure
- Execution
- Execution Context
- Required Operation

## Out of Scope

- Sintaxis concreta o lenguaje de consultas para EvoQ.
- Operadores específicos (`filter`, `map`, `select`, `where`, `join`, `order`, `aggregate`).
- Parser, AST o planificador/optimizador de consultas.
- Traducción a SQL o integración con bases de datos relacionales.
- Providers externos de almacenamiento o filesystem.
- API técnica en Rust de `evo-query-engine`.
