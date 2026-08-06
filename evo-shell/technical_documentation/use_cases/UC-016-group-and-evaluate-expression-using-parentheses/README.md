# UC-016 — Agrupar y evaluar una expresión mediante paréntesis

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y evalúa expresiones agrupadas entre paréntesis `(...)`.

Ejemplo:

```text
(
    iter
    |> take 1
    |> select name
    |> to-value
)
```

## Modelo conceptual

Se extiende `Command` para representar expresiones agrupadas:

```text
Command
├── ScopeFs(&str)
├── Iter
├── Enter(&str)
├── Clear(TerminalClearMode)
├── Exit
├── Pipeline(Pipeline)
└── Grouped(Box<Command>)
```

## Flujo conceptual

1. **Recolección de entrada (`main.rs`):** `requires_continuation` contabiliza el balance de delimitadores estructurales `Token::LeftParen` y `Token::RightParen` (omitidos dentro de `Token::String`). Mientras existan paréntesis abiertos sin cerrar, la shell solicita las líneas secundarias con `... > `.
2. **Parsing (`command.rs`):** Al identificar `Token::LeftParen` como token inicial de comando, delega la resolución a la expresión contenida y verifica la existencia del `Token::RightParen` de cierre. Produce `Command::Grouped(Box::new(inner_command))`.
3. **Ejecución (`executor.rs`):** `executor::execute` procesa `Command::Grouped(inner)` ejecutando recursivamente el comando contenido `inner`, retornando su resultado estructurado `ExecutionResult`.

## Responsabilidades

- **Input Collection (`main.rs`):** Determina el balance estructural de paréntesis sin interpretar semántica.
- **Command Resolver (`command.rs`):** Construye la representación de dominio `Command::Grouped`.
- **Executor Agent (`executor.rs`):** Evalúa el comando agrupado interior y retorna el resultado de su ejecución.

## Separación con Filter Expression

`filter` posee su propio resolver (`filter_expression.rs`) que maneja paréntesis de agrupación lógica. UC-016 no interfiere con el parsing interno de `filter`.

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
