# UC-001 — Start Evo Application

Status: FUNCTIONAL

## Goal

Evo Runtime inicia una Execution de una Evo Application a partir de su Entry
Point declarado cuando un Host solicita iniciar la aplicación.

## Trigger

Un Host solicita a Evo Runtime iniciar una Evo Application.

## Preconditions

- Existe una Evo Application identificada.
- La Evo Application declara un Entry Point.
- Evo Runtime está disponible para recibir la solicitud del Host.

## Main Flow

1. El Host solicita a Evo Runtime iniciar una Evo Application.
2. Evo Runtime identifica el Entry Point declarado por la aplicación.
3. Evo Runtime inicia una nueva Execution asociada con ese Entry Point.
4. Evo Runtime establece la continuidad funcional necesaria para esa Execution
   (Execution Context).
5. El trabajo de la Evo Application comienza a participar mediante Evo Runtime.
6. La Execution continúa activa hasta producir posteriormente su Result final.

## Successful Outcome

Una nueva Execution queda iniciada correctamente y en estado activo.

## Failure Outcomes

- El Entry Point declarado es inválido o no utilizable.
- La Execution no puede iniciarse correctamente debido a un fallo en el entorno
  inicial.

## Invariants

- El Host no necesita conocer la Implementation interna del Entry Point.
- Iniciar una Application no implica cargar anticipadamente todas sus
  dependencias.
- Cada Execution iniciada permanece funcionalmente distinguible e independiente
  de otras Executions.

## Related User Stories

- US-001 (Start an Application)
- US-008 (Maintain an Execution Context)
- US-009 (Finalize an Execution)

## Related Data Dictionary Terms

- Host
- Evo Application
- Entry Point
- Execution
- Execution Context
- Result
- Failure
- Evo Runtime

## Out of Scope

- Representación física o formato de archivo del Entry Point.
- Sintaxis o directivas específicas (`.main`, `.root`).
- Configuración de empaquetado Cargo o crate manifests.
- Procesos del sistema operativo, threads o ejecutables nativos.
- Modelos asíncronos o runtimes de concurrencia.
- Firmas de funciones o APIs técnicas en Rust.
