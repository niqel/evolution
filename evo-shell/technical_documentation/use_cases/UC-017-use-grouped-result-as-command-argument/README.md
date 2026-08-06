# UC-017 — Usar el resultado de una expresión agrupada como argumento de un comando

## Objetivo

Este caso de uso describe cómo Evo Shell resuelve expresiones agrupadas entre paréntesis cuando son pasadas como argumentos posicionales de comandos (por ejemplo `enter (...)`).

Ejemplo:

```text
enter (
    iter
    |> filter type equals "directory"
    |> filter name equals "child"
    |> select name
    |> to-value
)
```

## Modelo conceptual

Se introduce `CommandArgument` para representar argumentos de comando escalares o agrupados:

```text
CommandArgument
├── Literal(&str)
└── Grouped(Box<Command>)

Command
├── ScopeFs(&str)
├── Iter
├── Enter(CommandArgument)
├── Clear(TerminalClearMode)
├── Exit
├── Pipeline(Pipeline)
└── Grouped(Box<Command>)
```

## Flujo de resolución y ejecución

1. **Parsing (`command.rs`):** `resolve_enter` verifica si el argumento comienza con `Token::LeftParen`. Si es así, resuelve recursivamente la expresión interior y construye `CommandArgument::Grouped(Box::new(inner))`.
2. **Evaluación de argumento (`execution.rs`):** Al ejecutar `Command::Enter(argument)`:
   - Si es `CommandArgument::Literal(loc)`, se utiliza directamente.
   - Si es `CommandArgument::Grouped(inner)`, se evalúa primeramente `inner` con `resolve_with` sobre el ámbito actual.
   - El `PipelineValue::Value(val)` obtenido se convierte a cadena de ruta.
   - Se invoca `enterer::enter` con la ruta resuelta.
3. **Manejo de errores e incompatibilidad:**
   - Si el pipeline interior retorna `PipelineValue::Values` (múltiples filas) o `PipelineValue::Arguments` (para comandos variádicos sin consumidor), se retorna `ExecuteError::IncompatibleGroupedArgument`.
   - Si el pipeline interior falla, el error de ejecución de pipeline se propaga de inmediato.

## Diagramas

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
