# UC-010 — Terminar Evo Shell mediante el comando `exit`

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y ejecuta:

```text
exit
```

según:

[US-010 — Terminar Evo Shell mediante el comando `exit`](../../../functional_documentation/user_stories/US-010-exit-shell.md)

La sintaxis sin argumentos ni opciones se apoya en:

[LR-001 — Command Arguments and Options](../../../functional_documentation/language_rules/LR-001-command-arguments-and-options.md)

UC-010 pertenece completamente a Evo Shell.

Evo Shell Engine no participa porque terminar la sesión es control de ejecución de shell, no una operación de filesystem scope.

## Frontera técnica

El comando `exit` representa una intención de control de sesión.

Por tanto, la arquitectura aprobada es mínima:

```text
agent
  ↓
use case
  ↓
execution result
  ↓
run_loop
  ↓
return Ok(())
```

Responsabilidades:

- `exiter` agent coordina la intención de terminar la sesión.
- `Exit` use case expresa la capacidad pública.
- `ExecutionResult::Exit` comunica al loop principal que debe terminar.

No existe resolver propio.

No existe provider propio.

Razón:

`exit` no resuelve una operación externa ni interactúa con infraestructura externa.

## Sintaxis

El parser debe resolver:

```text
exit
```

como:

```text
Command::Exit
```

No se acepta:

```text
exit now
exit --force
exit 0
```

La política de error reutiliza los errores de parsing existentes.

No se implementan opciones cortas.

## Domain model

UC-010 introduce una variante mínima en `Command`:

```text
Command
├── ScopeFs(&str)
├── Iter
├── Enter(&str)
├── Clear(TerminalClearMode)
└── Exit
```

UC-010 también introduce una variante en `ExecutionResult`:

```text
ExecutionResult
├── ScopeChanged
├── FilesystemIteration(...)
├── TerminalCleared
└── Exit
```

No se crea un AST genérico nuevo.

No se crea un modelo artificial de `bool exiting`.

## Use case definition

Se agrega una definición de use case siguiendo el patrón mínimo existente:

```text
definitions/use_cases/exiter.rs

Exit =
    fn()
```

El use case no retorna error porque la salida cooperativa no realiza IO externa ni resolución operativa adicional.

## Agent

El agent:

```text
agents/exiter.rs
```

expone:

```text
exiter::exit()
```

Responsabilidad:

1. representar la decisión de terminar la sesión;
2. permitir que el executor produzca `ExecutionResult::Exit`.

No contiene resolvers.

No contiene providers.

No interactúa con terminal, filesystem, red ni DB.

## Exit y loop

La salida cooperativa se comunica al propietario del loop:

```text
Command::Exit
    ↓
executor::execute
    ↓
ExecutionResult::Exit
    ↓
run_loop
    ↓
return Ok(())
```

El loop principal de `main.rs` es quien decide terminar la sesión al recibir `ExecutionResult::Exit`.

No se usa `std::process::exit(0)` en el flujo normal.

La terminación real del proceso ocurre por salida normal de `run`, `main` y el ownership de Rust.

## EOF

EOF sigue siendo un camino independiente de salida.

UC-010 no transforma EOF en `Command::Exit`.

## Tests

La implementación debe cubrir:

- `exit` se parsea como `Command::Exit`;
- tokens adicionales rechazan el comando;
- `Command::Exit` produce `ExecutionResult::Exit`;
- `exiter::exit` coincide con su function pointer de use case;
- `ExecutionResult::Exit` hace que el loop termine;
- no se usa `std::process::exit` para la salida normal;
- EOF sigue funcionando de forma independiente;
- `exit` no cambia el filesystem scope.

## Fuera de alcance

- `exit --force`;
- `exit --code`;
- `exit 1`;
- `quit`;
- `logout`;
- `close`;
- `shutdown`;
- `restart`;
- confirmaciones;
- mensaje de despedida;
- clear al salir;
- persistencia de sesión;
- historial de shell;
- manejo de señales;
- política de `Ctrl+C`;
- códigos de salida personalizados;
- cambios en Evo Shell Engine.

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
