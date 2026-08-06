# UC-008 — Convertir una proyección estructurada en un valor escalar

## Objetivo

Este caso de uso convierte una `StructuredProjection` de una fila y una propiedad en un único `ProjectedValue`.

La operación preserva el valor opcional explícito si ya está representado en el valor proyectado.

La operación es pura respecto al filesystem y al scope.

## Actor

- Consumidor del engine

## Entrada

La capacidad pública conceptual es:

```text
ToValue(StructuredProjection)
```

## Precondiciones

- existe una `StructuredProjection`;
- la estructura ya fue construida por una capa superior;
- no se recibe texto sin analizar.

## Flujo principal

1. El consumidor invoca `ToValue`.
2. `value_converter::convert` coordina el caso de uso.
3. `to_value::resolve` valida la cardinalidad.
4. `to_value::resolve` extrae el único `ProjectedValue`.
5. El valor se devuelve como resultado escalar tipado.

## Semántica

- `to-value` exige exactamente 1 fila y 1 propiedad.
- `to-value` devuelve un `ProjectedValue`.
- `to-value` no convierte a `String`.
- `to-value` no toma automáticamente la primera fila ni la primera propiedad.
- `to-value` no modifica filesystem.
- `to-value` no modifica scope.
- `null` no forma parte de la semántica visible.
- el valor opcional explícito se conserva como parte del `ProjectedValue`.

## Errores

La operación puede fallar si:

- no hay filas;
- hay más de una fila;
- no hay propiedades;
- hay más de una propiedad;
- la forma interna de la proyección es inconsistente.

## Relación con la documentación funcional

[US-007 — Convertir una proyección estructurada en un valor escalar](../../../functional_documentation/user_stories/US-007-convert-structured-projection-to-value.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
- [use-case.d2](use-case.d2)
