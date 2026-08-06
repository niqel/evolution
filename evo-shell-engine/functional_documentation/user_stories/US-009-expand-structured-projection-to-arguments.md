# US-009 — Expandir una proyección estructurada en argumentos posicionales

## Historia de usuario

Como consumidor de Evo Shell Engine,
quiero expandir una proyección estructurada en argumentos posicionales,
para reutilizar una selección de una sola propiedad como entrada de otro comando.

## Descripción

`to-args` trabaja sobre una `StructuredProjection` ya materializada.

La operación exige exactamente una propiedad.

La operación admite cero o más filas.

La operación devuelve una colección estructurada de argumentos posicionales basada en `ProjectedValue`.

La operación no vuelve a consultar filesystem, scope o providers.

La operación no convierte el resultado en texto fuente.

La operación no inserta comas, espacios ni quoting.

La colección vacía es un resultado válido.

La documentación funcional del lenguaje define la sintaxis textual de Evo Shell y Evo Script. Este caso de uso define únicamente la semántica estructurada del engine.

## Criterios de aceptación

1. `to-args` recibe una `StructuredProjection`.
2. `to-args` exige exactamente 1 propiedad.
3. `to-args` acepta 0 filas.
4. `to-args` acepta N filas.
5. `to-args` devuelve argumentos posicionales estructurados.
6. `to-args` devuelve cero argumentos cuando no hay filas.
7. `to-args` no devuelve `null`.
8. `to-args` falla si hay 0 propiedades.
9. `to-args` falla si hay más de 1 propiedad.
10. `to-args` conserva el orden de las filas.
11. `to-args` conserva el tipo de cada argumento.
12. `to-args` conserva la ausencia opcional explícita.
13. `to-args` no convierte todo a `String`.
14. `to-args` no expande por múltiples propiedades.
15. `to-args` no modifica filesystem ni scope.
16. `to-args` no depende de presentación.

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
- to-values;
- null;
- provider externo;
- quoting;
- separación por comas;
- intercalado de propiedades;
- expansión por tuplas.

## Relación con la documentación funcional del lenguaje

La sintaxis aprobada de `to-args` se define en:

[LR-002 — Pipeline Syntax, Grouping and Argument Expansion](../../../evo-shell/functional_documentation/language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
