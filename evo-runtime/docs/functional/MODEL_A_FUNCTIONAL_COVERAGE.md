# Model A — Functional Coverage

Status: FUNCTIONAL CLOSED

## Propósito

Este documento registra la cobertura funcional completada de Evo Runtime Model A.

En Model A, Evo Runtime tiene una responsabilidad única y mínima: iniciar una
Evo Application invocando su acción Run proporcionada y concluir Start cuando
Run concluye.

## Alcance Funcional y Trazabilidad

La cobertura funcional de Model A está completamente cubierta por exactamente
una User Story y un Use Case:

| User Story | Responsabilidad Funcional | Primary Use Case |
| --- | --- | --- |
| [US-001](user-stories/US-001-start-application.md) | Iniciar una Evo Application | [UC-001](use-cases/UC-001-start-evo-application.md) |

## Ruta de Ejecución Funcional

La ruta de ejecución canónica de Model A es:

```text
Host
  │
  │ llama Start(Run)
  ▼
Evo Runtime
  │
  │ invoca Run()
  ▼
Evo Application (Run activo)
  │
  │ Run termina
  ▼
Evo Runtime
  │
  │ Start termina
  ▼
Host (recupera control)
```

## Independencia de Múltiples Invocaciones de Start

Evo Runtime soporta múltiples invocaciones independientes de Start:

```text
Host / Caller
  ├── Start(Run_A) ──► Application A (activa) ──► Run_A termina ──► Start termina
  ├── Start(Run_B) ──► Application B (activa) ──► Run_B termina ──► Start termina
  └── Start(Run_C) ──► Application C (activa) ──► Run_C termina ──► Start termina
```

- Cada invocación de Start opera de manera independiente.
- La terminación de una invocación de Run no termina, altera ni produce la
  terminación de otra invocación de Start.
- No existe ningún Context compartido ni entidad de seguimiento de Execution en
  Evo Runtime.

## No Responsabilidades de Evo Runtime Model A

Todas las operaciones internas, ejecuciones de engines, transporte de datos y
manejo de outcomes ocurren fuera de Evo Runtime:

- Evo Runtime **no** resuelve operaciones ni dependencias.
- Evo Runtime **no** selecciona ni carga engines (como EvoS o EvoQ).
- Evo Runtime **no** administra providers, capabilities ni contracts.
- Evo Runtime **no** parsea comandos ni ejecuta archivos Evo-Script.
- Evo Runtime **no** mantiene un Context interno ni una entidad Execution.
- Evo Runtime **no** transporta Values a través de las fronteras internas de la
  aplicación.
- Evo Runtime **no** posee ni transporta outcomes (`Result`/`Failure`).
- Evo Runtime **no** depende arquitectónicamente de `evo-values`.

Una vez que Start invoca a Run, la aplicación interactúa directamente con sus
propias bibliotecas, engines y providers.
