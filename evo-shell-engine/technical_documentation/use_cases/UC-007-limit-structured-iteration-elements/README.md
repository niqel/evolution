# UC-007 — Limitar una iteración estructurada a como máximo N elementos

## Objetivo

Este caso de uso permite limitar una colección estructurada a como máximo N elementos conservando el orden recibido.

La operación acepta colecciones vacías y acepta `0` como límite válido.

La operación es pura respecto al filesystem y al scope.

## Actor

- Consumidor del engine

## Entrada

La capacidad pública conceptual es:

```text
Take(StructuredItems<'a>, usize)
```

## Precondiciones

- existe una colección estructurada;
- el límite ya fue construido por una capa superior;
- no se recibe texto sin analizar.

## Flujo principal

1. El consumidor invoca `Take`.
2. `taker::take` coordina el caso de uso.
3. `take::resolve` conserva como máximo N elementos.
4. El orden original de los elementos se preserva.
5. La colección resultante se devuelve como una nueva colección estructurada prestada.

## Semántica

- `take` limita una secuencia a como máximo N elementos.
- `take` acepta `0` y devuelve colección vacía.
- `take` acepta una colección vacía y devuelve colección vacía.
- `take` no modifica el filesystem.
- `take` no modifica el scope.
- `take` no reindexa.
- `take` no depende de presentación.
- `take` no convierte el resultado en `value`, `values` o `args`.
- `null` no forma parte de la semántica visible.

## Errores

`take` no falla por tener menos elementos que el límite solicitado.

`take` no falla por recibir una colección vacía.

## Relación con la documentación funcional

[US-006 — Limitar una iteración estructurada a como máximo N elementos](../../../functional_documentation/user_stories/US-006-limit-structured-iteration-elements.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
- [use-case.d2](use-case.d2)
