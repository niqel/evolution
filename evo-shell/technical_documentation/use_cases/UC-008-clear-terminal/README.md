# UC-008 — Limpiar la terminal

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y ejecuta:

```text
clear
clear --all
```

según:

[US-008 — Limpiar la terminal](../../../functional_documentation/user_stories/US-008-clear-terminal.md)

La sintaxis de opción larga y flag se apoya en:

[LR-001 — Command Arguments and Options](../../../functional_documentation/language_rules/LR-001-command-arguments-and-options.md)

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

El parser debe resolver:

```text
clear
```

como:

```text
Command::Clear(TerminalClearMode::Visible)
```

y:

```text
clear --all
```

como:

```text
Command::Clear(TerminalClearMode::All)
```

No se acepta:

```text
clear all
clear --unknown
clear --all extra
```

La política de error reutiliza los errores de parsing existentes.

No se implementan opciones cortas.

## Domain model

UC-008 introduce un value object mínimo:

```text
TerminalClearMode
├── Visible
└── All
```

No se usa `bool all`.

Razón:

`Visible` y `All` representan intenciones explícitas distintas.

`TerminalClearMode` pertenece a Evo Shell porque describe una intención de presentación terminal.

No pertenece a Evo Shell Engine.

## Command

`Command` evoluciona con una variante mínima:

```text
Command
├── ScopeFs(&str)
├── Iter
├── Enter(&str)
└── Clear(TerminalClearMode)
```

No se crea un AST genérico nuevo.

No se crea un parser genérico completo de opciones.

## Use case definition

Se agrega una definición de use case siguiendo el patrón de function pointers existente:

```text
definitions/use_cases/terminal_clearer.rs

TerminalClearer =
    fn(TerminalClearMode) -> Result<(), TerminalClearError>
```

Ese use case es consumido por el executor/resolver de ejecución para ejecutar `Command::Clear(mode)`.

## Resolver definition

Se agrega una definición de resolver:

```text
definitions/resolvers/terminal_clearer.rs

Resolve =
    fn(TerminalClearMode, Provide) -> Result<(), TerminalClearError>
```

El resolver recibe el modo y delega al provider mediante function pointer.

## Provider definition

Se agrega una definición de provider:

```text
definitions/providers/terminal_clearer.rs

Provide =
    fn(TerminalClearMode) -> Result<(), TerminalClearError>
```

El provider realiza la interacción terminal.

## Agent

El agent:

```text
agents/terminal_clearer.rs
```

expone:

```text
terminal_clearer::clear(mode)
```

Responsabilidad:

1. recibir `TerminalClearMode`;
2. seleccionar el resolver existente;
3. invocar el resolver mediante function pointer;
4. devolver éxito o error.

No contiene secuencias ANSI.

No escribe directamente en stdout.

## Resolver

El resolver:

```text
resolvers/terminal_clearer.rs
```

expone:

```text
terminal_clearer::resolve(mode, provide)
```

Responsabilidad:

1. recibir `TerminalClearMode`;
2. invocar el provider mediante function pointer;
3. propagar el resultado.

No escribe ANSI.

No interactúa directamente con stdout.

## Provider

El provider:

```text
providers/terminal_clearer.rs
```

expone:

```text
terminal_clearer::provide(mode)
```

Responsabilidad:

1. traducir `TerminalClearMode` a secuencias ANSI/VT;
2. escribirlas en la terminal;
3. propagar errores de IO.

No ejecuta comandos externos.

No lanza subprocess.

No depende de `clear`, `cls` ni del shell anfitrión.

## Secuencias ANSI/VT

Para:

```text
TerminalClearMode::Visible
```

el provider escribe:

```text
ESC[2J
ESC[H
```

Conceptualmente:

- limpia viewport;
- reposiciona el cursor al inicio.

Para:

```text
TerminalClearMode::All
```

el provider escribe:

```text
ESC[2J
ESC[3J
ESC[H
```

Conceptualmente:

- limpia viewport;
- solicita limpiar scrollback;
- reposiciona el cursor al inicio.

## Executor

El executor existente continúa coordinando ejecución de comandos:

```text
executor::execute(&mut Shell, Command)
```

`execution::resolve` agrega una rama para:

```text
Command::Clear(mode)
```

Flujo:

```text
Command::Clear(mode)
        ↓
terminal_clearer::clear(mode)
        ↓
ExecutionResult::TerminalCleared
```

El `Shell` no se modifica.

## Scope

`clear` y `clear --all` no modifican:

- `Shell`;
- `FilesystemScope`;
- la ubicación activa;
- el prompt compacto.

El siguiente prompt se renderiza desde el mismo scope activo.

## Errores

Los errores de parsing reutilizan `ParseError` existente.

Los errores de IO terminal se representan como:

```text
TerminalClearError
```

`ExecuteError` incorpora ese error únicamente para propagar fallo de clear.

No se introduce una jerarquía nueva innecesaria.

## Tests

La implementación debe cubrir:

- parser resuelve `clear` como `Command::Clear(Visible)`;
- parser resuelve `clear --all` como `Command::Clear(All)`;
- parser rechaza `clear all`;
- parser rechaza `clear --unknown`;
- parser rechaza `clear --all extra`;
- function pointer del use case coincide con el agent;
- agent delega al resolver;
- resolver delega al provider;
- mode se conserva;
- errores de provider se propagan;
- provider visible produce la secuencia ANSI esperada;
- provider all produce la secuencia ANSI esperada;
- executor ejecuta clear sin modificar scope;
- executor ejecuta clear all sin modificar scope.

## Fuera de alcance

- `clear -a`;
- aliases;
- opciones cortas;
- opciones adicionales;
- terminal capability detection;
- comandos externos;
- subprocess;
- themes;
- history command;
- parser genérico completo de opciones;
- cambios de Evo Shell Engine.

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
