# UC-008 — Limpiar la terminal

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y ejecuta:

```text
clear
```

según:

[US-008 — Limpiar la terminal](../../../functional_documentation/user_stories/US-008-clear-terminal.md)

UC-008 pertenece completamente a Evo Shell.

Evo Shell Engine no participa porque limpiar la terminal es presentación/IO de shell, no una operación de filesystem scope.

## Frontera técnica

La terminal es infraestructura externa.

Por tanto, se conserva la dirección arquitectónica:

```text
agent
  ↓
resolver
  ↓
provider
  ↓
terminal
```

Responsabilidades:

- `terminal_clearer` agent coordina.
- `terminal_clearer` resolver resuelve la operación y delega.
- `terminal_clearer` provider realiza la interacción con la terminal.

El agent no escribe secuencias ANSI.

El resolver no escribe directamente en stdout.

El provider encapsula las secuencias ANSI/VT concretas.

## Sintaxis

El parser debe resolver `clear` como:

```text
Command::Clear
```

No se aceptan argumentos posicionales ni opciones (e.g. `clear --all`, `clear all`, `clear foo`).

## Command

`Command` utiliza la variante sin parámetros:

```text
Command::Clear
```

## Use case definition

```text
definitions/use_cases/terminal_clearer.rs

TerminalClearer = fn() -> Result<(), TerminalClearError>
```

Ese use case es consumido por el executor/resolver de ejecución para ejecutar `Command::Clear`.

## Resolver definition

```text
definitions/resolvers/terminal_clearer.rs

Resolve = fn(Provide) -> Result<(), TerminalClearError>
```

## Provider definition

```text
definitions/providers/terminal_clearer.rs

Provide = fn() -> Result<(), TerminalClearError>
```

## Secuencia ANSI/VT

El provider escribe la secuencia completa de limpieza (viewport + scrollback + reposicionamiento de cursor):

```text
ESC[2J
ESC[3J
ESC[H
```

`\x1b[2J\x1b[3J\x1b[H`

Conceptualmente:

- limpia viewport (`2J`);
- solicita limpiar scrollback (`3J`);
- reposiciona el cursor al inicio (`H`).

## Executor

```text
Command::Clear
        ↓
terminal_clearer::clear()
        ↓
ExecutionResult::TerminalCleared
```

El `Shell` no se modifica.

## Scope

`clear` no modifica:

- `Shell`;
- `FilesystemScope`;
- la ubicación activa;
- el prompt compacto.

El siguiente prompt se renderiza desde el mismo scope activo.

## Tests

La implementación cubre:

- parser acepta `clear` como `Command::Clear`;
- parser rechaza `clear --all`;
- parser rechaza `clear foo`;
- provider emite la secuencia ANSI completa (`ESC[2J ESC[3J ESC[H`);
- provider ya no emite la secuencia parcial antigua (`ESC[2J ESC[H`);
- executor ejecuta clear sin modificar scope.
