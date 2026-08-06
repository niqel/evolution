# UC-004 — Filtrar una iteración estructurada

## Objetivo

Este caso de uso permite aplicar un predicado estructurado a una colección de elementos ya materializados.

El resultado conserva los elementos completos que cumplen la condición.

La operación es pura respecto al filesystem y al scope.

## Actor

- Consumidor del engine

## Entrada

La capacidad pública conceptual es:

```text
Filter(&[FilesystemIterationItem], &FilterExpression)
```

## Precondiciones

- existe una colección estructurada;
- la expresión de filtro ya fue construida por una capa superior;
- no se recibe texto sin analizar.

## Flujo principal

1. El consumidor invoca `Filter`.
2. `filterer::filter` coordina el caso de uso.
3. `filter::resolve` evalúa la expresión para cada elemento.
4. Si el elemento cumple, continúa en el resultado.
5. Si no cumple, se descarta.
6. El resultado se devuelve como una colección estructurada filtrada.

## Semántica

- `filter` conserva elementos completos.
- `filter` no proyecta propiedades.
- `filter` no modifica el filesystem.
- `filter` no modifica el scope.
- 0 elementos es un resultado válido.
- `null` no forma parte de la semántica visible.

## Errores

La operación puede fallar si:

- la propiedad no es soportada;
- el operador no es compatible con la propiedad;
- falta un valor comparable requerido por la expresión.

`NoMatches` no es un error.

## Relación con la documentación funcional

[US-003 — Filtrar una iteración estructurada](../../../functional_documentation/user_stories/US-003-filter-structured-iteration.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
- [use-case.d2](use-case.d2)
