# US-016 — Agrupar y evaluar una expresión mediante paréntesis

## Historia de usuario

Como usuario de Evo Shell,
quiero envolver una expresión entre paréntesis `(...)`,
para que la shell evalúe primero la expresión contenida y produzca su resultado tipado.

## Descripción

Evo Shell ya permite la ejecución de comandos simples y pipelines de una o varias líneas.
Esta historia agrega la capacidad de definir expresiones agrupadas utilizando la sintaxis de paréntesis `(...)`.

El significado funcional de `(...)` es evaluar prioritariamente la expresión interior y devolver su resultado estructurado de forma tipada (`PipelineValue`).

Los paréntesis:
- NO representan invocaciones de función estilo `foo()`.
- NO convierten el resultado en texto sintáctico serializado.
- NO crean subprocesos ni subshells del sistema operativo.
- NO modifican la gramática interna de expresiones dentro de `filter`.

## Flujo observable

Ejemplo de expresión agrupada en nivel superior:

```text
scope-fs …/evo-shell > (
... > iter
... > |> take 1
... > |> select name
... > |> to-value
... > )
```

Resultado visual:

```text
only.txt
```

Equivalente en una sola línea:

```text
(iter |> take 1 |> select name |> to-value)
```

## Multilínea y balance de agrupación

Un paréntesis abierto `(` no cerrado indica que la expresión está incompleta.
Evo Shell mantiene la lectura multilínea mostrando el prompt de continuación (`... > `) hasta que se ingrese el paréntesis de cierre `)` correspondiente (o se alcance un error léxico / EOF).

Los paréntesis dentro de cadenas de texto entre comillas (por ejemplo `"foo (bar)"`) no alteran el balance de agrupación.

## Semántica observable

1. La expresión encerrada entre `(...)` se evalúa prioritariamente y produce el valor tipado derivado de su contenido.
2. Si la expresión agrupada se ejecuta en el nivel superior, su resultado se presenta según las reglas existentes de presentación.
3. Los paréntesis de expresión agrupada son distintos de los paréntesis utilizados internamente por expresiones lógicas de `filter`. Las expresiones `filter` existentes continúan evaluándose normalmente.
4. Si se ingresa un paréntesis de cierre `)` sin su correspondiente apertura, la shell reporta un error sintáctico `ParseError::UnexpectedClosingParenthesis`.
5. Si la entrada finaliza con un grupo abierto `(` sin cerrar y se alcanza EOF, la shell no ejecuta la instrucción parcial.

## Fuera de alcance

- Uso del resultado agrupado como argumento de otro comando (pertenece a US-017).
- Subshells aisladas con ámbito de variables o scope independiente.
- Conversión de colecciones en argumentos variádicos.
