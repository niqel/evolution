# UC-015 — Recolectar entrada textual multilínea de pipeline

## Objetivo

Este caso de uso documenta cómo Evo Shell recolecta la entrada textual del usuario a través de múltiples líneas cuando una línea termina con el separador de pipeline `|>`.

Ejemplo:

```text
iter |>
    filter type equals "file" |>
    take 1 |>
    select name |>
    to-value
```

Una vez recolectado el texto completo, se envía como una única cadena textual al interpretador existente (`parser::parse`).

UC-015 pertenece completamente a la capa de entrada textual de Evo Shell.

Evo Shell Engine no participa en la recolección textual ni conoce la existencia de saltos de línea en la entrada.

## Naturaleza del caso de uso

La recolección multilínea ocurre en el bucle interactivo de la shell (`main.rs`) o función de captura de entrada.

No introduce un nuevo `Agent` público ni altera la frontera de `Parse`.

Reutiliza la tokenización existente (`TokenStream` y `tokenizer::tokenize`) para determinar si el último token estructural de la cadena acumulada es `Token::PipelineSeparator`.

## Modelo conceptual

```text
Entrada de usuario (línea por línea)
    ↓
Detección de continuación (TokenStream -> tokenizer)
    ↓ ¿Último token es PipelineSeparator?
    ├── Sí: presenta prompt de continuación ("... > ") y lee siguiente línea
    └── No: entrega la entrada textual acumulada
    ↓
parser::parse(&mut stream, tokenizer::tokenize)
    ↓
Command::Pipeline(Pipeline)
```

## Flujo de recolección

1. La shell escribe el prompt principal de ubicación (`scope-fs …/dir >`).
2. Lee la primera línea de `stdin`.
3. Evalúa si la entrada acumulada requiere continuación (es decir, el último token válido tokenizado es `Token::PipelineSeparator`).
4. Mientras requiera continuación:
   a. Escribe el prompt de continuación (`... > `).
   b. Lee la siguiente línea.
   c. Acumula la línea leída.
5. Al finalizar la recolección, entrega la cadena completa a `parser::parse`.
6. El parser construye `Command` (por ejemplo `Command::Pipeline`).
7. `executor::execute` procesa el comando de la forma estándar.

## Detección de continuación

Para evitar fragilidad con cadenas entre comillas (por ejemplo, `filter name equals "foo |> bar"`), la recolección no realiza una búsqueda simple de subcadena al final del texto.

En su lugar, tokeniza la entrada acumulada utilizando `TokenStream` y `tokenizer::tokenize`. La entrada requiere continuación si y solo si el último token resultante antes del fin de la cadena es `Token::PipelineSeparator`.

## Manejo de EOF y líneas vacías

- **EOF durante continuación:** Si la lectura alcanza EOF (0 bytes leídos) mientras la entrada sigue requiriendo continuación, la shell no ejecuta una instrucción incompleta y retorna `Ok(None)`.
- **Líneas vacías durante continuación:** Si el usuario presiona Enter sin escribir texto adicional tras un `|>`, la entrada acumulada conserva su separador final `|>`, por lo que el estado de continuación se mantiene y la shell vuelve a solicitar la siguiente etapa.

## Responsabilidades

- **Input Collection (`main.rs`):** Acumula líneas de entrada y gestiona prompts de continuación.
- **Tokenizer (`tokenizer.rs` / `token.rs`):** Reutilizado sin modificaciones para analizar los tokens de la entrada acumulada.
- **Parser Agent (`parser.rs`):** Recibe la entrada textual completa unificada y produce el `Command` correspondiente.
- **PipelineExecutor:** Inalterado; no conoce saltos de línea ni multilínea.

## Decisiones arquitectónicas

- No se modifica `evo-shell-engine`.
- No se agrega un `Agent` o `Resolver` artificial para la recolección si la función encapsulada dentro del loop de entrada es suficiente.
- No se modifican las semánticas de ejecución de `PipelineOperation`.

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
