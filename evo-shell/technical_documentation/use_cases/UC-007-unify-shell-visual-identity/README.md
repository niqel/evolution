# UC-007 — Unificar la identidad visual de la tabla, los archivos y el prompt

## Objetivo

Este caso de uso documenta cómo Evo Shell debe unificar la identidad visual de superficies existentes, según:

[US-007 — Unificar la identidad visual de la tabla, los archivos y el prompt](../../../functional_documentation/user_stories/US-007-unify-shell-visual-identity.md)

US-007 no cambia la sintaxis de comandos.

No cambia la semántica de:

- `iter`;
- `enter`;
- `scope-fs`.

No cambia datos producidos por Evo Shell Engine.

La evolución pertenece exclusivamente a presentación de Evo Shell.

## Frontera técnica

Evo Shell Engine no conoce colores ni estilos.

No se agregan estilos a:

- `FilesystemEntry`;
- `FilesystemEntryKind`;
- `FilesystemIteration`;
- `FilesystemIterationItem`;
- entidades de dominio de `Shell`.

Los estilos se aplican únicamente al renderizar:

- prompt;
- tabla de `iter`;
- footer `Path:`;
- texto que escribe el usuario mientras se captura input.

No se crea:

- `Theme`;
- `Style` entity;
- enum de colores;
- trait;
- crate nuevo;
- dependencia nueva.

## Presentation Style

El módulo existente:

```text
evo-shell/src/presentation_style.rs
```

debe ser la fuente mínima compartida de constantes ANSI simples.

Constantes conceptuales:

```text
PRIMARY_STYLE
    green

LOCATION_STYLE
    cyan

FILE_STYLE
    pale yellow

CREATED_STYLE
    moderate dark teal

MODIFIED_STYLE
    slightly darker teal

RESET
```

Si el código actual ya usa nombres claros como:

```text
PROMPT_SCOPE_STYLE
PROMPT_LOCATION_STYLE
```

pueden conservarse como alias o nombres principales.

Lo importante es evitar duplicar valores mágicos para el mismo estilo.

Paleta técnica aprobada:

```text
green               \x1b[32m
cyan                \x1b[36m
pale yellow         \x1b[38;2;255;255;229m
created dark teal   \x1b[38;2;33;142;128m
modified dark teal  \x1b[38;2;24;130;115m
reset               \x1b[0m
```

El amarillo palido corresponde exactamente a:

```text
HEX #FFFFE5
RGB 255, 255, 229
```

El teal temporal de `Created` corresponde exactamente a:

```text
HEX #218E80
RGB 33, 142, 128
```

El teal temporal de `Modified` corresponde exactamente a:

```text
HEX #188273
RGB 24, 130, 115
```

No se agrega crate de terminal.

## Prompt

El prompt conserva su formato:

```text
scope-fs …/src >
```

Render conceptual:

```text
PRIMARY_STYLE scope-fs RESET
LOCATION_STYLE …/src RESET
PRIMARY_STYLE > RESET
space
FILE_STYLE
```

El símbolo `>` debe usar el mismo estilo que `scope-fs`.

Después del espacio final del prompt, Evo Shell activa `FILE_STYLE` para que el texto introducido por el usuario se vea como amarillo palido exacto.

Después de leer la línea, Evo Shell debe emitir `RESET` inmediatamente antes de parsear, ejecutar o renderizar cualquier resultado.

Ese reset debe ocurrir para:

- comando válido;
- comando inválido;
- línea vacía;
- EOF, si `read_line` lo reporta.

No se cambia cómo se lee `stdin`.

No se agrega line editor.

## Iteration Presenter

El agent existente:

```text
iteration_presenter::present
```

mantiene su responsabilidad de presentar `FilesystemIteration`.

US-007 cambia solo estilos:

- header completo en `PRIMARY_STYLE` y bold si ya se usa bold;
- índice de fila en `PRIMARY_STYLE`;
- `Type=file` en `FILE_STYLE`;
- `Size` de file en `FILE_STYLE`;
- nombre de archivo en `FILE_STYLE`;
- `Type=dir` y nombre de directorio conservan `LOCATION_STYLE`;
- symlink y other conservan sus estilos existentes;
- `Created` usa `CREATED_STYLE`;
- `Modified` usa `MODIFIED_STYLE`;
- summary colorea directories y files sin cambiar conteos;
- footer `Path:` conserva `PRIMARY_STYLE` para label y `LOCATION_STYLE` para full path.

## Header

El header visible permanece:

```text
#   Created              Modified             Type    Size      Name
```

Todo el contenido visible del header usa `PRIMARY_STYLE`.

Puede conservarse bold.

El reset debe ocurrir al final del header para no afectar filas.

El color no debe cambiar la alineación visible.

## Índice

Cada índice de fila usa `PRIMARY_STYLE`:

```text
0
1
2
```

El padding de la columna de índice debe calcularse sobre el texto visible del índice, no sobre secuencias ANSI.

Solución mínima esperada:

```text
format visible index with width
then wrap with PRIMARY_STYLE + RESET
```

No se introduce parser ANSI, medición genérica de ancho ni renderer de tabla.

## Files

Para `FilesystemEntryKind::File`:

```text
Type=file
Size=228 B
Name=Cargo.toml
```

los tres usan `FILE_STYLE`.

`Type`, `Size` y `Name` deben compartir exactamente el mismo estilo.

`Created` y `Modified` no heredan `FILE_STYLE`.

No se colorea toda la fila.

El padding de `Size` debe calcularse sobre el texto visible antes de agregar ANSI.

Si `Size` es vacio, no hace falta emitir `FILE_STYLE` para esa celda.

## Información temporal

La categoría técnica temporal se representa mediante dos estilos teal oscuros relacionados.

Constantes:

```text
CREATED_STYLE = \x1b[38;2;33;142;128m
MODIFIED_STYLE = \x1b[38;2;24;130;115m
```

Reglas:

- `Created` usa `CREATED_STYLE`;
- `Modified` usa `MODIFIED_STYLE`;
- `Created` no usa `BOLD`;
- `Modified` no usa `BOLD`;
- `CREATED_STYLE` y `MODIFIED_STYLE` son distintos;
- la diferencia visual se logra solo mediante el tono;
- `Modified` es ligeramente más oscuro que `Created`;
- ambos mantienen el mismo ancho visible aprobado;
- el padding se calcula sobre el texto visible antes de aplicar ANSI;
- si el valor temporal está ausente, la celda conserva el ancho y no necesita emitir estilo visible.

Los estilos temporales no se aplican a:

- header;
- index;
- `Type`;
- `Size`;
- `Name`;
- summary;
- prompt;
- footer.

## Directories

Se conserva el comportamiento de US-005:

```text
Type=dir
Name=directory/
```

ambos usan `LOCATION_STYLE`.

El padding de `Type` debe seguir calculándose sobre texto visible.

## Symlink y Other

`symlink` y `other` conservan los estilos actuales salvo contradicción real.

US-007 no redefine su paleta.

## Footer

El footer conserva el layout de US-005:

```text
N directories
N files
Path: /full/path
```

Reglas:

- `Path:` usa `PRIMARY_STYLE`;
- full path usa `LOCATION_STYLE`;
- no hay blank line entre summary y `Path:`;
- hay blank line después de `Path:`.

## Summary

El summary conserva sus lineas:

```text
N directories
N files
```

US-007 no agrega contadores ni totales.

Pero cambia la presentación visual:

```text
directories line:
BOLD + LOCATION_STYLE + number + RESET
LOCATION_STYLE + " directory/directories" + RESET

files line:
BOLD + FILE_STYLE + number + RESET
FILE_STYLE + " file/files" + RESET
```

Solo el numero va en bold.

La palabra queda en estilo normal del mismo color.

Singular/plural conserva la semántica actual:

```text
1 directory
2 directories

1 file
2 files
```

## Flujo principal

1. El main/shell loop renderiza prompt con estilos compartidos.
2. Después del espacio final, activa `FILE_STYLE` para entrada del usuario.
3. Lee la línea con el mecanismo actual.
4. Emite `RESET`.
5. Si hay EOF, termina sin dejar estilo activo.
6. Si la línea está vacía, vuelve a prompt sin salida adicional.
7. Si la línea contiene comando inválido, renderiza el error sin heredar `FILE_STYLE`.
8. Si el comando produce una iteración, `iteration_presenter::present` renderiza header, filas, summary y footer con estilos de US-007.

## Errores

US-007 no introduce errores nuevos.

El reset posterior a lectura debe evitar que errores de parseo o ejecución hereden el estilo de input.

No se cambia la semántica de errores existente.

## Tests

La implementación debe cubrir:

- header contiene `PRIMARY_STYLE` y `RESET`;
- header visible no cambia;
- índice de fila usa `PRIMARY_STYLE`;
- índice conserva ancho/alineación;
- `file` usa `FILE_STYLE`;
- size de file usa `FILE_STYLE`;
- nombre de archivo usa `FILE_STYLE`;
- `dir` y nombre de directory conservan `LOCATION_STYLE`;
- `Created` contiene `CREATED_STYLE`;
- `Modified` contiene `MODIFIED_STYLE`;
- `Created` no contiene `BOLD`;
- `Modified` no contiene `BOLD`;
- `CREATED_STYLE` y `MODIFIED_STYLE` son diferentes;
- `Created` y `Modified` no heredan `FILE_STYLE`;
- summary directories usa numero bold + `LOCATION_STYLE` y palabra con `LOCATION_STYLE`;
- summary files usa numero bold + `FILE_STYLE` y palabra con `FILE_STYLE`;
- prompt `>` usa `PRIMARY_STYLE`;
- prompt activa `FILE_STYLE` después del espacio final;
- se emite `RESET` después de leer input;
- ubicación compacta sigue en `LOCATION_STYLE`;
- footer sigue en `PRIMARY_STYLE` + `LOCATION_STYLE`;
- estructura visible no cambia al retirar ANSI.

## Fuera de alcance

- themes configurables
- selector de colores
- colores por extensión
- colores por permisos
- iconos
- autocomplete
- history
- line editing
- cambios de engine
- nuevos campos de metadata
- nuevas columnas
- crates nuevos

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
