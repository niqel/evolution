# UC-009 — Convertir una proyección estructurada en una colección de valores

## Objetivo

Este caso de uso convierte una `StructuredProjection` de una sola propiedad en una colección tipada de `ProjectedValue`.

La operación admite cero o más filas y preserva el orden de las filas.

La operación es pura respecto al filesystem y al scope.

## Actor

- Consumidor del engine

## Entrada

La capacidad pública conceptual es:

```text
ToValues(StructuredProjection)
```

## Precondiciones

- existe una `StructuredProjection`;
- las filas y propiedades ya fueron construidas por una capa superior;
- no se recibe texto sin analizar.

## Flujo principal

1. El consumidor invoca `ToValues`.
2. `values_converter::convert` coordina el caso de uso.
3. `to_values::resolve` valida la cardinalidad.
4. `to_values::resolve` extrae cada `ProjectedValue` conservando el orden.
5. El resultado se devuelve como colección tipada de valores.

## Semántica

- `to-values` exige exactamente 1 propiedad.
- `to-values` acepta 0 filas.
- `to-values` acepta N filas.
- `to-values` devuelve una colección tipada de `ProjectedValue`.
- `to-values` devuelve colección vacía cuando no hay filas.
- `to-values` no devuelve `null`.
- `to-values` no convierte a `String`.
- `to-values` no expande argumentos.
- `to-values` no modifica filesystem.
- `to-values` no modifica scope.

## Errores

La operación puede fallar si:

- hay 0 propiedades;
- hay más de 1 propiedad;
- la forma interna de alguna fila es inconsistente.

`to-values` no falla por una colección vacía.

## Relación con la documentación funcional

[US-008 — Convertir una proyección estructurada en una colección de valores](../../../functional_documentation/user_stories/US-008-convert-structured-projection-to-values.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
- [use-case.d2](use-case.d2)
