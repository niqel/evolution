# UC-010 — Expandir una proyección estructurada en argumentos posicionales

## Objetivo

Este caso de uso convierte una `StructuredProjection` de una sola propiedad en argumentos posicionales estructurados.

La operación admite cero o más filas y conserva el orden de las filas.

La operación es pura respecto al filesystem y al scope.

## Actor

- Consumidor del engine

## Entrada

La capacidad pública conceptual es:

```text
ToArgs(StructuredProjection)
```

## Precondiciones

- existe una `StructuredProjection`;
- la estructura ya fue construida por una capa superior;
- no se recibe texto sin analizar.

## Flujo principal

1. El consumidor invoca `ToArgs`.
2. `argument_expander::expand` coordina el caso de uso.
3. `to_args::resolve` valida la cardinalidad.
4. `to_args::resolve` extrae cada `ProjectedValue` conservando el orden.
5. El resultado se devuelve como argumentos posicionales estructurados.

## Semántica

- `to-args` exige exactamente 1 propiedad.
- `to-args` acepta 0 filas.
- `to-args` acepta N filas.
- `to-args` devuelve argumentos posicionales estructurados.
- `to-args` devuelve cero argumentos cuando no hay filas.
- `to-args` no devuelve `null`.
- `to-args` no convierte a `String`.
- `to-args` no inserta comas ni quoting.
- `to-args` no modifica filesystem.
- `to-args` no modifica scope.

## Errores

La operación puede fallar si:

- hay 0 propiedades;
- hay más de 1 propiedad;
- la forma interna de alguna fila es inconsistente.

`to-args` no falla por producir cero argumentos.

## Relación con la documentación funcional

[US-009 — Expandir una proyección estructurada en argumentos posicionales](../../../functional_documentation/user_stories/US-009-expand-structured-projection-to-arguments.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
- [use-case.d2](use-case.d2)
