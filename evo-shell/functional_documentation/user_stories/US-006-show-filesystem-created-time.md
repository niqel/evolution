# US-006 — Mostrar fecha de creación de elementos en `iter`

## Historia de usuario

Como usuario de Evo Shell,
quiero que `iter` muestre la fecha de creación de cada elemento cuando esté disponible,
para distinguir cuándo fue creado un archivo o directorio sin perder la información de modificación existente.

## Descripción

Evo Shell permite observar los elementos directamente contenidos en el filesystem scope activo mediante el comando:

```text
iter
```

US-004 define que `iter` presenta una tabla estructurada con:

- `#`
- `Modified`
- `Type`
- `Size`
- `Name`

US-006 agrega una nueva columna:

- `Created`

La columna `Created` debe aparecer antes de `Modified`.

La tabla queda conceptualmente:

```text
#   Created              Modified             Type    Size      Name
```

Esta historia no cambia la semántica de `iter`, el orden de los elementos, el índice, el resumen ni el contexto `Path:` definido por US-005.

## Resultado conceptual

Ejemplo conceptual:

```text
scope-fs …/src > iter

#   Created              Modified             Type    Size      Name
0   05/08/2026 02:15     05/08/2026 03:21     dir               agents/
1   05/08/2026 02:16     05/08/2026 03:22     file    1.2 kB    lib.rs
2                        05/08/2026 03:23     dir               providers/

2 directories
1 file
Path: /home/user/repos/evolution/evo-shell/src

scope-fs …/src >
```

Los valores son conceptuales.

El ejemplo muestra que `Created` puede estar disponible para algunos elementos y no estar disponible para otros.

## Columna `Created`

`Created` representa la fecha y hora de creación del elemento reportada por el filesystem cuando ese dato está disponible.

Formato funcional deseado:

```text
DD/MM/YYYY HH:MM
```

Ejemplos:

```text
05/08/2026 02:15
31/12/2025 23:59
01/01/2026 07:05
```

No todos los sistemas operativos ni filesystems garantizan que este dato exista.

Conceptualmente:

- Windows normalmente puede reportar fecha de creación;
- macOS normalmente puede reportar birth time;
- Linux depende del kernel y del filesystem;
- Android depende del kernel y del filesystem;
- otros filesystems pueden no proporcionar este dato.

Esta historia debe mantenerse portable.

No define lógica específica por sistema operativo.

Si `Created` no está disponible, la celda debe quedar vacía.

No debe mostrarse:

- `unknown`;
- `N/A`;
- `-`;
- `0`.

`Created` no debe inventarse usando `Modified`.

`Created` y `Modified` representan propiedades distintas.

## Columna `Modified`

`Modified` sigue representando la fecha y hora de última modificación del elemento cuando el filesystem la proporciona.

US-006 no cambia su significado ni su formato visual aprobado.

`Modified` sigue mostrándose como:

```text
DD/MM/YYYY HH:MM
```

cuando está disponible.

## Columnas existentes

US-006 no cambia la semántica de:

- `#`;
- `Type`;
- `Size`;
- `Name`.

El índice sigue comenzando en `0` y sigue representando la posición dentro de la iteración actual.

`Type` sigue distinguiendo al menos:

- `file`;
- `dir`;
- `symlink`;
- `other`.

`Size` sigue representando tamaño de archivos cuando está disponible.

Los directorios no requieren cálculo recursivo de tamaño.

`Name` sigue mostrando el nombre visible del elemento.

## Summary y Path

US-006 no cambia el resumen final.

El resumen conserva:

```text
N directories
N files
```

No se agrega total acumulado de bytes.

No se agregan conteos nuevos de symlink u other.

US-006 no cambia el footer `Path:` definido por US-005.

Conceptualmente:

```text
2 directories
1 file
Path: /home/user/repos/evolution/evo-shell/src
```

`Path:` sigue apareciendo como footer final de `iter`.

## Colores

US-006 no introduce colores nuevos.

Los colores existentes de la presentación se conservan:

- header diferenciado;
- prompt compacto con diferenciación visual;
- footer `Path:` con identidad visual coherente con el prompt;
- `dir` y el nombre de directorio con la misma diferenciación visual.

`Created` y `Modified` pueden permanecer con presentación neutra o default.

## Comportamiento progresivo

`iter` conserva comportamiento progresivo desde la perspectiva del usuario.

El usuario puede recibir filas conforme la iteración produce elementos.

US-006 no exige que toda la colección esté disponible antes de comenzar a mostrar resultados.

## Criterios de aceptación

1. `iter` muestra `Created` antes de `Modified`.
2. `Created` representa la fecha de creación cuando está disponible.
3. `Created` usa formato:
   - `DD/MM/YYYY HH:MM`.
4. Si `Created` no está disponible, la celda queda vacía.
5. `Modified` sigue funcionando igual.
6. `Created` no se sustituye por `Modified`.
7. No cambia el índice.
8. No cambia `Type`.
9. No cambia `Size`.
10. No cambia `Name`.
11. No cambia el summary.
12. No cambia el footer `Path:`.
13. No cambia el comportamiento progresivo de `iter`.

## Ejemplos

### A. Elementos con y sin Created

Entrada:

```text
scope-fs …/src
iter
```

Resultado conceptual:

```text
scope-fs …/src > iter

#   Created              Modified             Type    Size      Name
0   05/08/2026 02:15     05/08/2026 03:21     dir               agents/
1   05/08/2026 02:16     05/08/2026 03:22     file    1.2 kB    lib.rs
2                        05/08/2026 03:23     dir               providers/

2 directories
1 file
Path: /home/user/repos/evolution/evo-shell/src

scope-fs …/src >
```

La fila `providers/` muestra una celda `Created` vacía porque el dato no está disponible en ese ejemplo.

### B. Iteración vacía

Entrada:

```text
scope-fs …/empty
iter
```

Resultado conceptual:

```text
scope-fs …/empty > iter

#   Created              Modified             Type    Size      Name

0 directories
0 files
Path: /home/user/empty

scope-fs …/empty >
```

Incluso sin filas, el header incluye `Created` y el footer `Path:` sigue apareciendo.

## Relación con historias anteriores

US-004 define la tabla estructurada de `iter`.

US-005 define el contexto visual, el footer `Path:` y la legibilidad del prompt.

US-006 amplía la tabla con un dato adicional del filesystem cuando está disponible.

No modifica:

- semántica de `iter`;
- índice;
- orden;
- lazy iteration;
- type;
- size;
- name;
- summary;
- footer `Path:`;
- prompt;
- colores existentes.

## Fuera de alcance

Esta historia no define todavía:

- accessed;
- permissions;
- owner;
- group;
- inode;
- file id;
- ordenamiento por created;
- filter por created;
- pipelines;
- created operator;
- DB scopes;
- metadata recursiva;
- implementación Rust.
