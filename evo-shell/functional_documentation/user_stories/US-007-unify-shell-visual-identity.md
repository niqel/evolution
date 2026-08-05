# US-007 — Unificar la identidad visual de la tabla, los archivos y el prompt

## Historia de usuario

Como usuario de Evo Shell,
quiero que el prompt, la tabla de `iter` y el footer de contexto compartan una identidad visual coherente,
para leer comandos, ubicaciones y resultados con menos esfuerzo visual.

## Descripción

Evo Shell ya presenta el filesystem scope activo con un prompt compacto:

```text
scope-fs …/evo-shell >
```

US-004, US-005 y US-006 definen la salida estructurada de `iter`:

```text
#   Created              Modified             Type    Size      Name
```

US-007 no cambia la estructura de esa salida.

Esta historia solo unifica la identidad visual de:

- prompt;
- header de la tabla;
- índices de filas;
- tipos, tamaños y nombres de archivos;
- tipos y nombres de directorios;
- información temporal;
- summary;
- footer `Path:`.

No cambia comandos, datos, columnas, orden, summary, navegación ni comportamiento progresivo.

## Resultado conceptual

Ejemplo conceptual:

```text
scope-fs …/evo-shell > iter

#   Created              Modified             Type    Size      Name
0   04/08/2026 02:00     05/08/2026 00:17     file    228 B     Cargo.toml
1   04/08/2026 01:59     05/08/2026 03:19     dir               src/

1 directory
1 file
Path: /home/user/repos/evolution/evo-shell

scope-fs …/evo-shell >
```

Los colores no pueden representarse completamente en texto Markdown.

Funcionalmente:

- `scope-fs`, `>`, `Path:`, los títulos del header y los índices de filas comparten la misma identidad visual principal;
- la ubicación compacta `…/evo-shell` y el full path del footer comparten la identidad visual de ubicación;
- `dir` y `src/` conservan la identidad visual de directorio;
- `file`, `228 B` y `Cargo.toml` usan una identidad visual amarillo palido exacto;
- `Created` y `Modified` usan identidades visuales temporales teal relacionadas;
- `Created` usa un teal oscuro moderado;
- `Modified` usa un teal ligeramente más oscuro, sin negrita;
- el texto que el usuario escribe en el prompt usa esa misma identidad visual amarillo claro.

## Identidad visual principal

La identidad visual principal se usa para:

- `scope-fs`;
- el símbolo `>`;
- `Path:`;
- todos los títulos de la tabla:
  - `#`;
  - `Created`;
  - `Modified`;
  - `Type`;
  - `Size`;
  - `Name`;
- los ordinales de fila:
  - `0`;
  - `1`;
  - `2`;
  - ...

Esto crea una relación visual entre:

- prompt;
- contexto final;
- estructura de la tabla;
- índice de iteración.

## Ubicación

La ubicación compacta del prompt conserva su identidad visual actual:

```text
…/evo-shell
```

El full path del footer conserva esa misma identidad visual:

```text
/home/user/repos/evolution/evo-shell
```

El prompt sigue siendo compacto.

El footer `Path:` sigue mostrando la ruta completa.

## Archivos

Para archivos, `Type`, `Size` y `Name` usan una identidad visual amarillo palido exacto.

El amarillo elegido es:

```text
#FFFFE5
```

Ejemplo conceptual:

```text
file    228 B     Cargo.toml
^^^^    ^^^^^     ^^^^^^^^^^
mismo amarillo palido
```

Esto aplica a:

- `Type=file`;
- `Size` del archivo;
- nombre del archivo.

`Created` y `Modified` no cambian de identidad visual por tratarse de un archivo.

## Información temporal

La información temporal usa una familia visual teal oscura.

`Created` usa un teal oscuro moderado:

```text
#218E80
```

`Modified` usa un teal ligeramente más oscuro:

```text
#188273
```

Esto aplica a:

- `Created`;
- `Modified`.

`Created` usa solamente su identidad visual temporal clara.

`Modified` usa solamente su identidad visual temporal ligeramente más oscura.

Ninguno de los dos valores temporales usa negrita.

La diferencia visual entre `Created` y `Modified` se logra únicamente mediante el tono.

La intención es que `Modified` se distinga de `Created` sin dominar visualmente la tabla.

Estos tonos temporales no deben confundirse con:

- la identidad visual principal del prompt y header;
- la identidad visual de ubicación, paths y directorios;
- la identidad visual amarillo palido de archivos.

Esta categoría visual no se aplica a:

- header;
- índices;
- `Type`;
- `Size`;
- `Name`;
- summary;
- prompt;
- footer `Path:`.

## Texto introducido por el usuario

El texto que el usuario escribe después del prompt usa la misma identidad visual amarillo palido de archivos.

Conceptualmente:

```text
scope-fs …/evo-shell > iter
                       ^^^^
                       amarillo palido
```

Después de enviar el comando, la salida de Evo Shell no debe heredar accidentalmente ese estilo.

## Directorios

Los directorios conservan la identidad visual aprobada por US-005.

En una fila de directorio:

```text
dir     src/
^^^     ^^^^
mismo color visual de directorio
```

Esto aplica a:

- `Type=dir`;
- nombre del directorio.

## Summary

El summary conserva las mismas lineas:

```text
N directories
N files
```

Pero evoluciona visualmente.

Para directories:

- el numero usa identidad visual de directorio y negrita;
- la palabra `directory` o `directories` usa identidad visual de directorio normal.

Para files:

- el numero usa identidad visual amarillo palido y negrita;
- la palabra `file` o `files` usa identidad visual amarillo palido normal.

Conceptualmente:

```text
4 directories
3 files
^ ^^^^^^^^^^^
| directorio normal
directorio + negrita

^ ^^^^^
| files normal
files + negrita
```

El numero conserva negrita tanto en singular como en plural.

La estructura del summary no cambia.

## Footer `Path:`

El footer conserva el formato aprobado por US-005:

```text
Path: /home/user/repos/evolution/evo-shell
```

Reglas visuales:

- `Path:` usa la identidad visual principal;
- la ruta completa usa la identidad visual de ubicación.

No cambia su posición.

No se agrega una línea vacía entre summary y `Path:`.

## Estructura conservada

US-007 no modifica la estructura visible de `iter`.

Se conserva:

```text
command
<blank line>

table
<blank line>

summary
Path
<blank line>

prompt
```

La tabla conserva:

```text
#   Created              Modified             Type    Size      Name
```

El summary conserva:

```text
N directories
N files
```

## Alineación y resets

Los colores no deben modificar la alineación visual de columnas.

La tabla debe seguir siendo legible aunque algunas celdas tengan color y otras no.

Los estilos visuales deben restaurarse en los puntos necesarios para evitar que se propaguen accidentalmente a:

- otras columnas;
- summary;
- errores;
- siguiente prompt;
- salida posterior.

## Criterios de aceptación

1. `#`, `Created`, `Modified`, `Type`, `Size` y `Name` usan la misma identidad visual que `scope-fs`.
2. Los ordinales de fila usan esa misma identidad visual.
3. El símbolo `>` usa la misma identidad visual que `scope-fs`.
4. Los archivos usan amarillo palido exacto:
   - `Type=file`;
   - `Size` del archivo;
   - `Name` del archivo.
5. El texto introducido por el usuario usa el mismo amarillo palido.
6. Directories conservan cyan:
   - `Type=dir`;
   - `Name=directory/`.
7. La ubicación compacta conserva cyan.
8. El full path conserva cyan.
9. `Path:` conserva verde.
10. El numero de directories usa cyan y negrita.
11. La palabra `directory` o `directories` usa cyan normal.
12. El numero de files usa amarillo palido y negrita.
13. La palabra `file` o `files` usa amarillo palido normal.
14. `Created` usa teal oscuro moderado.
15. `Modified` usa teal ligeramente más oscuro.
16. `Created` no usa negrita.
17. `Modified` no usa negrita.
18. La diferencia entre `Created` y `Modified` se logra únicamente mediante tono.
19. `Modified` se distingue de `Created` sin dominar visualmente la tabla.
20. La categoría temporal no colorea header, índices, `Type`, `Size`, `Name`, summary, prompt ni footer.
21. Los colores no modifican alineación.
22. Los resets evitan que estilos se propaguen accidentalmente.
23. No se modifica la estructura de la tabla.
24. No se modifica comportamiento de `iter`, `enter` o `scope-fs`.

## Relación con historias anteriores

US-004 define la tabla estructurada de `iter`.

US-005 define el footer `Path:` y la relación visual entre prompt y contexto.

US-006 agrega la columna `Created`.

US-007 solo ajusta la identidad visual de elementos ya existentes.

No modifica:

- semántica de `Created`;
- semántica de `Modified`;
- index;
- `Type`;
- `Size`;
- `Name`;
- summary;
- footer `Path:`;
- navegación;
- lazy iteration.

## Fuera de alcance

Esta historia no define:

- themes configurables;
- selector de colores;
- detección light/dark;
- colores por extensión;
- colores por permisos;
- iconos;
- terminal width;
- Created nuevo;
- nuevas columnas;
- cambios del engine;
- implementación Rust.
