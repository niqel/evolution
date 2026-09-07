# UC-001 — Start Evo Application

Status: FUNCTIONAL

## Objetivo

Evo Runtime inicia una Evo Application ejecutando Run y permanece activo
hasta que Run concluye.

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
5. La acción Run retorna naturalmente.
6. Start retorna naturalmente al Host, concluyendo la llamada.

## Finalización

La conclusión natural de Run determina la conclusión natural de Start.

## Invariantes

- `Start != Run`: Start es la responsabilidad de Evo Runtime; Run es la acción
  proporcionada por la Evo Application.
- Start recibe la función Run, no el resultado de haber ejecutado Run
  previamente (`Start(run)`).
- La duración de `Start` está delimitada por la duración de `Run`.
- La terminación natural de `run()` determina naturalmente la finalización de
  `start()`; no se requiere un Use Case separado de `Finalize` ni métodos como
  `stop()`.
- Múltiples llamadas a Start son mutuamente independientes: la terminación de una
  no altera ni finaliza a las demás.
- Evo Runtime no participa en las operaciones internas, resolución de
  dependencias, selección de engines ni transporte de valores dentro de la
  aplicación.
- `Runtime lifecycle control != application outcome semantics`: Evo Runtime
  Model A no posee ni transporta outcomes.

## User Stories Relacionadas

- US-001 (Iniciar una Aplicación Evo)

## Términos del Data Dictionary Relacionados

- Evo Runtime
- Host
- Evo Application
- Start
- Run

## Fuera de Alcance

- Concurrencia a bajo nivel (hilos del SO, tareas asíncronas, schedulers).
- Resolución de operaciones o dependencias internas.
- Carga o selección de engines y providers.
- Modelo de Context o entidad de Execution.
- Transporte de Values entre componentes internos.
- Semántica o transporte de outcomes de la aplicación.
- Lógica de negocio o parsing de la aplicación.
- Definición de structs o enums en Rust.
