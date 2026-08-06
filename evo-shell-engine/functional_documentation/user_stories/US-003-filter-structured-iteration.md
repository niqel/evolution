# US-003 — Filtrar una iteración estructurada

## Historia de usuario

Como consumidor de Evo Shell Engine,
quiero filtrar una colección estructurada de elementos del filesystem,
para conservar únicamente los elementos que cumplen un predicado semántico.

## Descripción

`filter` trabaja sobre una colección estructurada ya materializada.

La operación evalúa un predicado por cada elemento y conserva el elemento completo cuando la condición es verdadera.

La operación no proyecta columnas ni convierte el resultado en texto.

La operación no modifica el filesystem.

La operación no modifica el scope.

La colección vacía es un resultado válido.

Si ningún elemento cumple el predicado, el resultado sigue siendo un éxito con una colección vacía.

La documentación funcional del lenguaje definirá la sintaxis textual de Evo Shell y Evo Script. Este caso de uso define únicamente la semántica estructurada del engine.

## Criterios de aceptación

1. `filter` recibe una colección estructurada de elementos.
2. `filter` puede producir 0, 1 o N elementos.
3. 0 elementos es un resultado válido y exitoso.
4. `filter` conserva el elemento completo.
5. `filter` no proyecta propiedades.
6. `filter` no cambia el scope.
7. `filter` no modifica el filesystem.
8. `filter` opera sobre propiedades estructuradas aprobadas del scope-fs.
9. `filter` evalúa `index`, `created`, `modified`, `type`, `size` y `name`.
10. `filter` soporta `equals` y `not-equals`.
11. `filter` soporta `>` y `<` cuando la propiedad lo permite.
12. `filter` soporta `at-least` y `at-most` cuando la propiedad lo permite.
13. `filter` soporta `between` y `not-between` cuando la propiedad lo permite.
14. `filter` soporta `and` y `or`.
15. Mezclar `and` y `or` sin agrupación explícita no debe llegar al engine como una expresión ambigua.
16. `filter` no usa `null`.

## Ejemplo

Colección conceptual de entrada:

```text
[
  { index: 0, type: file, name: "README.md" },
  { index: 1, type: directory, name: "src" }
]
```

Entrada:

```text
filter name equals "README.md"
```

Resultado conceptual:

```text
[
  { index: 0, type: file, name: "README.md" }
]
```

Colección vacía conceptual:

```text
[]
```

Si ningún elemento cumple, el resultado sigue siendo:

```text
[]
```

## Fuera de alcance

- parsing textual;
- sintaxis de Evo Shell;
- sintaxis de Evo Script;
- presentación;
- projection/select;
- index/take/to-value/to-values/to-args;
- null;
- provider externo;
- recursión;
- ordenamiento;
- agregaciones;
- comparación de propiedades no aprobadas.

## Relación con la documentación funcional del lenguaje

La sintaxis aprobada de `filter` se define en:

[LR-002 — Pipeline Syntax, Grouping and Argument Expansion](../../../evo-shell/functional_documentation/language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
