# US-004 — Seleccionar propiedades estructuradas de una iteración

## Historia de usuario

Como consumidor de Evo Shell Engine,
quiero proyectar propiedades estructuradas de una iteración ya materializada,
para conservar la forma tabular del resultado sin convertirlo todavía en texto ni en escalares.

## Descripción

`select` trabaja sobre una colección estructurada ya materializada.

La operación proyecta únicamente las propiedades solicitadas y conserva las filas originales.

La operación no filtra elementos.

La operación no modifica el filesystem.

La operación no modifica el scope.

La colección vacía es un resultado válido.

La ausencia de valor para una propiedad opcional no significa que la propiedad sea inválida.

La documentación funcional del lenguaje definirá la sintaxis textual de Evo Shell y Evo Script. Este caso de uso define únicamente la semántica estructurada del engine.

## Criterios de aceptación

1. `select` recibe una colección estructurada de elementos.
2. `select` proyecta propiedades.
3. `select` no elimina filas.
4. `select` no agrega filas.
5. `select` conserva el orden de filas.
6. `select` soporta una propiedad.
7. `select` soporta múltiples propiedades.
8. `select` conserva el orden solicitado de propiedades.
9. `select` rechaza propiedades inexistentes o no soportadas.
10. Una colección vacía produce una proyección vacía válida.
11. Una colección vacía no produce `null`.
12. Los valores proyectados conservan su tipo.
13. Una propiedad opcional sin valor conserva ausencia explícita.
14. `select` no convierte automáticamente a `value`.
15. `select` no convierte automáticamente a `values`.
16. `select` no convierte automáticamente a `args`.
17. `select` no modifica el filesystem.
18. `select` no modifica el scope.
19. `select` no depende de presentación.
20. `select` por índice numérico de columna queda fuera de esta historia.

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
select name, type
```

Resultado conceptual:

```text
[
  { name: "README.md", type: file },
  { name: "src", type: directory }
]
```

Colección vacía conceptual:

```text
[]
```

Si la entrada está vacía, el resultado sigue siendo una proyección vacía válida.

## Fuera de alcance

- parsing textual;
- sintaxis de Evo Shell;
- sintaxis de Evo Script;
- presentación;
- filter;
- index/take/to-value/to-values/to-args;
- null;
- provider externo;
- recursión;
- ordenamiento;
- agregaciones;
- selección por posición de columna.

## Relación con la documentación funcional del lenguaje

La sintaxis aprobada de `select` se define en:

[LR-002 — Pipeline Syntax, Grouping and Argument Expansion](../../../evo-shell/functional_documentation/language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
