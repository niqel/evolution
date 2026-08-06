# UC-005 — Seleccionar propiedades estructuradas de una iteración

## Objetivo

Este caso de uso permite proyectar propiedades estructuradas de una colección de elementos ya materializados.

El resultado conserva las filas originales y el orden solicitado de las propiedades.

La operación es pura respecto al filesystem y al scope.

## Actor

- Consumidor del engine

## Entrada

La capacidad pública conceptual es:

```text
Select(StructuredItems<'a>, &[SelectProperty])
```

## Precondiciones

- existe una colección estructurada;
- las propiedades a seleccionar ya fueron construidas por una capa superior;
- no se recibe texto sin analizar.

## Flujo principal

1. El consumidor invoca `Select`.
2. `selector::select` coordina el caso de uso.
3. `select::resolve` valida las propiedades solicitadas.
4. `select::resolve` proyecta cada fila conservando su orden.
5. `select::resolve` conserva el orden solicitado de las propiedades.
6. El resultado se devuelve como una proyección estructurada.

## Semántica

- `select` proyecta propiedades estructuradas.
- `select` no filtra filas.
- `select` no modifica el filesystem.
- `select` no modifica el scope.
- 0 filas es un resultado válido.
- `null` no forma parte de la semántica visible.
- los valores proyectados conservan su tipo conceptual.
- la ausencia explícita de un valor opcional se conserva como ausencia, no como `null`.

## Errores

La operación puede fallar si:

- la propiedad no es soportada.

`select` no falla por una colección vacía.

`select` no convierte automáticamente la proyección en `value`, `values` o `args`.

## Relación con la documentación funcional

[US-004 — Seleccionar propiedades estructuradas de una iteración](../../../functional_documentation/user_stories/US-004-select-structured-iteration-properties.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
- [use-case.d2](use-case.d2)
