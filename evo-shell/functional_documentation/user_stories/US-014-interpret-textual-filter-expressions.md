# US-014 — Interpretar expresiones textuales de `filter`

## Historia de usuario

Como usuario de Evo Shell,
quiero escribir expresiones textuales de `filter` dentro de un pipeline,
para que la shell las convierta en una expresión tipada `FilterExpression` sin ejecutar todavía el filtrado.

## Descripción

Evo Shell ya interpreta pipelines textuales básicos. Esta historia extiende esa capacidad para reconocer la etapa `filter` y convertir su expresión textual en la estructura tipada que ya existe en Evo Shell Engine.

La shell no ejecuta el filtro en esta historia.
La shell solo interpreta la entrada textual y construye la operación estructurada `PipelineOperation::Filter(FilterExpression)`.

La semántica de evaluación pertenece al engine.

## Flujo observable

Ejemplo canónico:

```text
iter |> filter name equals "file.txt"
```

Resultado conceptual:

```text
Command::Pipeline(Pipeline [..., Filter(FilterExpression), ...])
```

Las etapas se conservan en el orden escrito.

## Alcance de la interpretación

La primera versión de esta capacidad reconoce:

- `filter`;
- propiedades estructuradas aprobadas;
- operadores comparativos aprobados;
- operadores lógicos `and` y `or`;
- agrupación con paréntesis;
- continuidad de pipeline con `|>`.

Las expresiones reconocidas se convierten directamente al modelo tipado del engine.

## Semántica observable

1. `filter` puede aparecer dentro de un pipeline textual.
2. La expresión de `filter` se interpreta en forma tipada.
3. La entrada se convierte en `PipelineOperation::Filter(FilterExpression)`.
4. Las etapas anteriores y posteriores al `filter` conservan su orden.
5. La shell no guarda la expresión como texto suelto.
6. `equals` es la igualdad textual aprobada.
7. `not-equals` es la desigualdad textual aprobada.
8. `>` y `<` son operadores aprobados.
9. `at-least` y `at-most` son operadores aprobados.
10. `between` y `not-between` son operadores aprobados.
11. `and` y `or` son operadores lógicos aprobados.
12. Los paréntesis agrupan la expresión.
13. Mezclar `and` y `or` sin paréntesis es ambiguo y debe fallar.
14. Un operador desconocido produce error.
15. Una propiedad desconocida produce error.
16. Una expresión vacía produce error.
17. `filter` no ejecuta evaluación.
18. `filter` no modifica scope.
19. `filter` no modifica filesystem.

## Reglas de interpretación

- `filter` consume una expresión estructurada, no texto libre sin contrato.
- La coma separa los dos límites de `between` y `not-between`.
- La interpretación conserva la forma de la expresión y no la reduce a string.
- La validación semántica de evaluación sigue perteneciendo al engine.

## Valores y propiedades

Esta historia interpreta como propiedades aprobadas:

- `index`;
- `created`;
- `modified`;
- `type`;
- `size`;
- `name`.

Esta primera versión interpreta literales textuales cuando el contrato textual ya está definido con claridad.

La semántica textual de `created` y `modified` queda diferida si no existe un literal aprobado para representarlas de forma estable.

Las unidades textuales de tamaño usan base decimal:

- `kB` = `1_000` bytes;
- `MB` = `1_000_000` bytes;
- `GB` = `1_000_000_000` bytes.

## Errores observables

Esta historia reconoce como errores de interpretación, entre otros:

- expresión vacía;
- propiedad desconocida;
- operador desconocido;
- operador faltante;
- valor faltante;
- valor inválido;
- límite superior faltante en `between` / `not-between`;
- paréntesis abierto sin cerrar;
- paréntesis de cierre inesperado;
- mezcla ambigua de `and` y `or`.

## Compatibilidad

Los comandos simples existentes siguen funcionando sin `filter`.

La interpretación de pipelines básicos sigue funcionando como en la historia anterior.

## Fuera de alcance

- ejecución del filtro;
- semántica interna del engine;
- multilinea;
- subpipelines;
- pipelines como argumentos;
- presentación;
- reordenamiento automático de expresiones;
- aliases sintácticos no aprobados;
- lexer clásico;
- AST textual paralelo.
