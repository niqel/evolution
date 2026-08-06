# US-008 — Convertir una proyección estructurada en una colección de valores

## Historia de usuario

Como consumidor de Evo Shell Engine,
quiero convertir una proyección estructurada en una colección de valores,
para conservar los resultados de una sola propiedad sobre múltiples filas.

## Descripción

`to-values` trabaja sobre una `StructuredProjection` ya materializada.

La operación exige exactamente una propiedad.

La operación admite cero o más filas.

La operación devuelve una colección tipada de `ProjectedValue`.

La operación no vuelve a consultar filesystem, scope o providers.

La operación no convierte el resultado a texto.

La colección vacía es un resultado válido.

La documentación funcional del lenguaje define la sintaxis textual de Evo Shell y Evo Script. Este caso de uso define únicamente la semántica estructurada del engine.

## Criterios de aceptación

1. `to-values` recibe una `StructuredProjection`.
2. `to-values` exige exactamente 1 propiedad.
3. `to-values` acepta 0 filas.
4. `to-values` acepta N filas.
5. `to-values` devuelve una colección tipada de valores.
6. `to-values` devuelve colección vacía cuando no hay filas.
7. `to-values` no devuelve `null`.
8. `to-values` falla si hay 0 propiedades.
9. `to-values` falla si hay más de 1 propiedad.
10. `to-values` conserva el orden de las filas.
11. `to-values` conserva el tipo de cada valor.
12. `to-values` conserva la ausencia opcional explícita.
13. `to-values` no convierte todo a `String`.
14. `to-values` no expande argumentos.
15. `to-values` no modifica filesystem ni scope.
16. `to-values` no depende de presentación.

## Ejemplo

Entrada conceptual:

```text
StructuredProjection {
  properties: [name],
  rows: [
    [Name("README.md")],
    [Name("src")],
    [Name("notes.txt")]
  ]
}
```

Resultado conceptual:

```text
[Name("README.md"), Name("src"), Name("notes.txt")]
```

## Fuera de alcance

- parsing textual;
- sintaxis de Evo Shell;
- sintaxis de Evo Script;
- presentación;
- filter/select/index/take;
- to-value;
- to-args;
- null;
- provider externo;
- tuplas;
- records;
- flattening;
- expansión de argumentos.

## Relación con la documentación funcional del lenguaje

La sintaxis aprobada de `to-values` se define en:

[LR-002 — Pipeline Syntax, Grouping and Argument Expansion](../../../evo-shell/functional_documentation/language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
