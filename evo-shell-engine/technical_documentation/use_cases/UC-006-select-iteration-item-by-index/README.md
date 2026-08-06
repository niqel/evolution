# UC-006 — Seleccionar un elemento estructurado por índice

## Objetivo

Este caso de uso permite seleccionar exactamente un elemento de una colección estructurada a partir de su índice estructurado.

Si el elemento no existe, la operación falla con un error semántico explícito.

La operación es pura respecto al filesystem y al scope.

## Actor

- Consumidor del engine

## Entrada

La capacidad pública conceptual es:

```text
Index(StructuredItems<'a>, usize)
```

## Precondiciones

- existe una colección estructurada;
- el índice solicitado ya fue construido por una capa superior;
- no se recibe texto sin analizar.

## Flujo principal

1. El consumidor invoca `Index`.
2. `indexer::index` coordina el caso de uso.
3. `index::resolve` busca el elemento cuyo índice estructurado coincide con el solicitado.
4. Si existe exactamente uno, se devuelve una colección estructurada de un solo elemento.
5. Si no existe, se devuelve un error semántico explícito.

## Semántica

- `index` selecciona un elemento puntual.
- `index` no proyecta propiedades.
- `index` no modifica el filesystem.
- `index` no modifica el scope.
- la colección vacía de entrada es válida;
- la ausencia del elemento solicitado produce error;
- `null` no forma parte de la semántica visible.
- `index` no reindexa.
- `index` no convierte el resultado en `value`, `values` o `args`.

## Errores

La operación puede fallar si:

- el elemento solicitado no existe;
- la colección contiene múltiples elementos que violan la unicidad esperada del índice;
- el índice solicitado no puede satisfacerse de forma inequívoca.

`index` no falla por recibir una colección vacía en sí misma; falla porque no encuentra el elemento solicitado.

## Relación con la documentación funcional

[US-005 — Seleccionar un elemento estructurado por índice](../../../functional_documentation/user_stories/US-005-select-iteration-item-by-index.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
- [use-case.d2](use-case.d2)
