# UC-001 — Start Evo Application

Status: FUNCTIONAL

## Objetivo

Evo Runtime inicia una Evo Application ejecutando la acción Run que dicha
aplicación proporciona, manteniéndose activo mientras Run se ejecuta y
retornando su Result final hacia el Host.

## Trigger

El Host solicita a Evo Runtime iniciar una Evo Application proporcionando su
acción Run.

## Precondiciones

- Existe una Evo Application que proporciona una acción Run compatible.
- Evo Runtime está listo para recibir la solicitud del Host.

## Flujo Principal

1. El Host invoca la acción Start de Evo Runtime, pasando la acción Run de la
   Evo Application.
2. Evo Runtime recibe la acción Run.
3. Evo Runtime invoca la acción Run de la aplicación.
4. La aplicación ejecuta su trabajo interno directamente con sus librerías,
   engines y providers, mientras Start permanece a la espera.
5. La acción Run finaliza y produce un Result (éxito o Failure).
6. Evo Runtime recibe el Result de Run.
7. Evo Runtime retorna el Result al Host, concluyendo la llamada a Start.

## Outcome Exitoso

La aplicación finaliza su ejecución y entrega un Result exitoso al Host.

## Outcomes de Fallo

La aplicación finaliza con un fallo y entrega un Result que expresa un Failure
al Host.

## Invariantes

- `Start != Run`: Start es la responsabilidad de Evo Runtime; Run es la acción
  proporcionada por la Evo Application.
- Start recibe la función Run, no el resultado de haber ejecutado Run
  previamente (`Start(run)`).
- La terminación de `run()` determina naturalmente la finalización de `start()`;
  no se requiere un Use Case separado de `Finalize` ni métodos como `stop()`.
- Múltiples llamadas a Start son mutuamente independientes: el Failure de una
  no afecta a las demás y terminar una no finaliza a las otras.
- Evo Runtime no participa en las operaciones internas, resolución de
  dependencias, selección de engines ni transporte de valores dentro de la
  aplicación.
- `Result != Failure` (el modelo de resultados pertenece a `evo-values`).

## User Stories Relacionadas

- US-001 (Iniciar una Aplicación Evo)

## Términos del Data Dictionary Relacionados

- Evo Runtime
- Host
- Evo Application
- Start
- Run
- Result
- Failure

## Fuera de Alcance

- Concurrencia a bajo nivel (hilos del SO, tareas asíncronas, schedulers).
- Resolución de operaciones o dependencias internas.
- Carga o selección de engines y providers.
- Modelo de Context o entidad de Execution.
- Transporte de Values entre componentes internos.
- Lógica de negocio o parsing de la aplicación.
- Definición de structs o enums en Rust.
