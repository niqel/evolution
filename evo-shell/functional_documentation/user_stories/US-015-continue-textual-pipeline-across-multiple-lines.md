# US-015 — Continuar un pipeline textual en múltiples líneas

## Historia de usuario

Como usuario de Evo Shell,
quiero escribir un pipeline en varias líneas dividiéndolo tras un separador `|>`,
para construir instrucciones complejas con legibilidad visual sin ejecutarlas prematuramente.

## Descripción

Actualmente, Evo Shell interpreta y ejecuta instrucciones en una sola línea.
Esta historia extiende la captura textual de entrada de la shell para detectar cuando una línea termina sintácticamente con un separador de pipeline `|>`.

Cuando una línea finaliza en `|>`, Evo Shell reconoce que la instrucción está incompleta, solicita las siguientes líneas necesarias mediante un prompt de continuación visual y combina la entrada en una única secuencia textual antes de enviarla al interpretador existente (`parser::parse`).

La lectura multilínea pertenece exclusivamente a la recolección textual de entrada en Evo Shell. No modifica la semántica de los pipelines, no altera el comportamiento del interpretador ni introduce conocimiento de saltos de línea dentro de `PipelineExecutor` o `evo-shell-engine`.

## Flujo observable

Ejemplo multilínea:

```text
scope-fs …/evo-shell > iter |>
... > filter type equals "file" |>
... > take 1 |>
... > select name |>
... > to-value
```

Entrada combinada internamente:

```text
iter |> filter type equals "file" |> take 1 |> select name |> to-value
```

Resultado de parsing:

```text
Command::Pipeline(Pipeline [
    Iter,
    Filter(...),
    Take(1),
    Select([Name]),
    ToValue
])
```

## Semántica observable

1. Si una línea termina sintácticamente en `|>`, la shell considera que el pipeline continúa.
2. Los espacios en blanco al final de la línea después de `|>` no impiden la detección de continuación.
3. El separador `|>` dentro de una cadena entre comillas (por ejemplo, `"foo |> bar"`) no activa la continuación multilínea si no es el último token estructural de la expresión.
4. Evo Shell muestra un prompt de continuación visual (`... > `) mientras la instrucción siga incompleta.
5. Las líneas leídas consecutivamente se combinan en una sola cadena textual completa.
6. La entrada textual completa se parsea exactamente una vez y se ejecuta exactamente una vez.
7. La indentación de las líneas secundarias es puramente visual y no altera el resultado.
8. Si se ingresa una línea vacía mientras se espera la continuación de un `|>`, la shell continúa esperando una etapa válida.
9. Si se detecta un fin de archivo (EOF) mientras la instrucción multilínea está incompleta, la shell no ejecuta la instrucción parcial y finaliza o regresa de forma limpia.
10. La lectura multilínea no afecta la ejecución de comandos simples de una sola línea (`scope-fs`, `iter`, `enter`, `clear`, `exit`).

## Errores observables

Si la entrada multilínea combinada contiene un error sintáctico o de pipeline (por ejemplo, una etapa faltante al final o un comando desconocido), el interpretador reporta el error de parse estándar una vez recopilada toda la entrada.

## Fuera de alcance

- Continuación multilínea basada en paréntesis abiertos `(`.
- Subpipelines o pipelines dentro de argumentos.
- Modificaciones a la semántica o ejecución de etapas de pipeline.
- Alteraciones en `evo-shell-engine`.
