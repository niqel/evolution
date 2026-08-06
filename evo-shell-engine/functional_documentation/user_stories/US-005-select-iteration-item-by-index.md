# US-005 — Seleccionar un elemento estructurado por índice

## Historia de usuario

Como consumidor de Evo Shell Engine,
quiero seleccionar un elemento estructurado por su índice,
para obtener exactamente ese elemento o una señal clara de que no existe.

## Descripción

`index` trabaja sobre una colección estructurada ya materializada.

La operación busca el elemento cuyo índice estructurado coincide con el índice solicitado y devuelve ese elemento completo.

La operación no proyecta columnas.

La operación no modifica el filesystem.

La operación no modifica el scope.

La colección vacía es una entrada válida, pero no contiene el elemento solicitado y por tanto produce error de ausencia.

La documentación funcional del lenguaje definirá la sintaxis textual de Evo Shell y Evo Script. Este caso de uso define únicamente la semántica estructurada del engine.

## Criterios de aceptación

1. `index` recibe una colección estructurada de elementos.
2. `index` busca exactamente un elemento por índice.
3. `index` devuelve un único elemento o un error.
4. `index` no devuelve `null`.
5. `index` conserva el elemento completo.
6. `index` no proyecta propiedades.
7. `index` no cambia el scope.
8. `index` no modifica el filesystem.
9. `index` usa el índice estructurado existente del elemento.
10. `index` no reindexa.
11. Una colección vacía de entrada es válida.
12. Una colección vacía produce error de elemento no encontrado.
13. `index` no agrega elementos.
14. `index` no elimina elementos salvo por selección puntual del elemento solicitado.
15. `index` no depende de presentación.
16. `index` no convierte el resultado en `value`, `values` o `args`.

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
index 1
```

Resultado conceptual:

```text
[
  { index: 1, type: directory, name: "src" }
]
```

Colección vacía conceptual:

```text
[]
```

Si el índice solicitado no existe, el resultado es un error semántico explícito.

## Fuera de alcance

- parsing textual;
- sintaxis de Evo Shell;
- sintaxis de Evo Script;
- presentación;
- filter;
- select;
- take/to-value/to-values/to-args;
- null;
- provider externo;
- reindexado;
- ordenamiento;
- duplicación silenciosa.

## Relación con la documentación funcional del lenguaje

La sintaxis aprobada de `index` se define en:

[LR-002 — Pipeline Syntax, Grouping and Argument Expansion](../../../evo-shell/functional_documentation/language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
