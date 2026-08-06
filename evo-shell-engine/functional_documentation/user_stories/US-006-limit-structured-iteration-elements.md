# US-006 — Limitar una iteración estructurada a como máximo N elementos

## Historia de usuario

Como consumidor de Evo Shell Engine,
quiero limitar una colección estructurada a como máximo N elementos,
para conservar el orden actual sin exigir que existan N elementos.

## Descripción

`take` trabaja sobre una colección estructurada ya materializada.

La operación conserva como máximo los primeros N elementos disponibles y mantiene el orden recibido.

La operación no modifica el filesystem.

La operación no modifica el scope.

La colección vacía es una entrada válida y sigue siendo una colección vacía válida al salir.

La documentación funcional del lenguaje definirá la sintaxis textual de Evo Shell y Evo Script. Este caso de uso define únicamente la semántica estructurada del engine.

## Criterios de aceptación

1. `take` recibe una colección estructurada de elementos.
2. `take` limita la colección a como máximo N elementos.
3. `take` acepta `0` como límite válido.
4. `take` sobre colección vacía devuelve colección vacía.
5. `take` no devuelve `null`.
6. `take` conserva el orden de los elementos recibidos.
7. `take` no reindexa.
8. `take` no modifica el filesystem.
9. `take` no modifica el scope.
10. `take` no exige que existan N elementos.
11. `take` no agrega elementos.
12. `take` no convierte el resultado en `value`, `values` o `args`.
13. `take` no depende de presentación.

## Ejemplo

Colección conceptual de entrada:

```text
[
  { index: 0, type: file, name: "README.md" },
  { index: 1, type: directory, name: "src" },
  { index: 2, type: file, name: "notes.txt" }
]
```

Entrada:

```text
take 2
```

Resultado conceptual:

```text
[
  { index: 0, type: file, name: "README.md" },
  { index: 1, type: directory, name: "src" }
]
```

Entrada:

```text
take 0
```

Resultado conceptual:

```text
[]
```

## Fuera de alcance

- parsing textual;
- sintaxis de Evo Shell;
- sintaxis de Evo Script;
- presentación;
- filter;
- index;
- select;
- to-value/to-values/to-args;
- null;
- provider externo;
- reindexado;
- ordenamiento;
- validación sintáctica de negativos.

## Relación con la documentación funcional del lenguaje

La sintaxis aprobada de `take` se define en:

[LR-002 — Pipeline Syntax, Grouping and Argument Expansion](../../../evo-shell/functional_documentation/language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
