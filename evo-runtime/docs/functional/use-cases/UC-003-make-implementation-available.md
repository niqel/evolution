# UC-003 — Make Implementation Available

Status: FUNCTIONAL

## Goal

Evo Runtime asegura que una Implementation resuelta esté funcionalmente
disponible bajo demanda antes de participar en Invocation.

## Trigger

Una Required Operation ya ha sido resuelta hacia una Implementation específica
que debe participar en la Execution.

## Preconditions

- Existe una Execution activa.
- Existe una Implementation resuelta para una Required Operation.

## Main Flow

1. Evo Runtime continúa con la Implementation resuelta.
2. Evo Runtime determina si la Implementation ya está funcionalmente disponible
   para participar.
3. Si ya está disponible, continúa sin trabajo adicional de disponibilidad.
4. Si aún no está disponible pero puede hacerse disponible, Evo Runtime la hace
   funcionalmente disponible bajo el modelo vigente.
5. La Implementation queda funcionalmente disponible para continuar hacia
   Invocation.

## Successful Outcome

La Implementation queda disponible bajo demanda para participar en la
Execution.

## Failure Outcomes

- La Implementation requerida no puede hacerse funcionalmente disponible.

## Invariants

- Disponibilidad funcional no prescribe un mecanismo de carga física.
- `available != physical file load`
- `available != dynamic library load`
- `available != process creation`
- `available != memory mapping`
- La disponibilidad de una Implementation ocurre bajo demanda, no como carga
  masiva anticipada obligatoria.

## Related User Stories

- US-003 (Load a Required Implementation)
- US-004 (Invoke an Operation)
- US-007 (Propagate an Execution Failure)

## Related Data Dictionary Terms

- Implementation
- Required Operation
- Invocation
- Execution
- Failure
- Evo Runtime

## Out of Scope

- Acceso físico al filesystem o lectura de disco.
- Carga dinámica de bibliotecas compartidas (`.so`, `.dll`, `.dylib`, `cdylib`).
- Formato binario o interfaces ABI.
- Creación de procesos o hilos del sistema operativo.
- Ciclo de vida físico de plugins o hot reload.
