# US-007 — Convertir una proyección estructurada en un valor escalar

## Historia de usuario

Como consumidor de Evo Shell Engine,
quiero convertir una proyección estructurada en un único valor escalar,
para obtener el resultado final de una selección de una sola fila y una sola propiedad.

## Descripción

`to-value` trabaja sobre una `StructuredProjection` ya materializada.

La operación exige exactamente una fila y una propiedad.

La operación devuelve un `ProjectedValue` tipado y conserva la ausencia explícita si el valor proyectado ya la representa.

La operación no vuelve a consultar filesystem, scope o providers.

La operación no convierte el resultado a texto.

La colección vacía de entrada sigue siendo un resultado válido a nivel de `select`, pero no cumple el contrato de `to-value`.

La documentación funcional del lenguaje define la sintaxis textual de Evo Shell y Evo Script. Este caso de uso define únicamente la semántica estructurada del engine.

## Criterios de aceptación

1. `to-value` recibe una `StructuredProjection`.
2. `to-value` exige exactamente 1 fila.
3. `to-value` exige exactamente 1 propiedad.
4. `to-value` devuelve un valor escalar tipado.
5. `to-value` no devuelve `null`.
6. `to-value` no toma automáticamente la primera fila.
7. `to-value` no toma automáticamente la primera propiedad.
8. `to-value` falla si no hay filas.
9. `to-value` falla si hay más de una fila.
10. `to-value` falla si no hay propiedades.
11. `to-value` falla si hay más de una propiedad.
12. `to-value` conserva el valor opcional explícito cuando existe.
13. `to-value` no convierte todo a `String`.
14. `to-value` no modifica filesystem ni scope.
15. `to-value` no depende de presentación.

## Ejemplo

Entrada conceptual:

```text
StructuredProjection {
  properties: [name],
  rows: [
    [Name("README.md")]
  ]
}
```

Resultado conceptual:

```text
Name("README.md")
```

## Fuera de alcance

- parsing textual;
- sintaxis de Evo Shell;
- sintaxis de Evo Script;
- presentación;
- filter/select/index/take;
- to-values;
- to-args;
- null;
- provider externo;
- acumulación de múltiples valores;
- expansión de argumentos.

## Relación con la documentación funcional del lenguaje

La sintaxis aprobada de `to-value` se define en:

[LR-002 — Pipeline Syntax, Grouping and Argument Expansion](../../../evo-shell/functional_documentation/language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
