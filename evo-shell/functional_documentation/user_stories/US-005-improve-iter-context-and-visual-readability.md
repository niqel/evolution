# US-005 — Mejorar el contexto y la legibilidad visual de `iter`

## Historia de usuario

Como usuario de Evo Shell,
quiero que `iter` muestre claramente el contexto completo de la ubicación que está enumerando y mejore su separación visual,
para poder entender una salida larga sin perder de vista dónde estoy trabajando.

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

US-005 mejora la presentación observable de esa salida para que:

- el usuario vea la ruta completa que está siendo iterada;
- esa ruta completa aparezca antes y después de la lista;
- exista separación visual clara entre comando, contexto, tabla, resumen y siguiente prompt;
- el prompt compacto distinga visualmente el tipo de scope y la ubicación;
- los directorios se identifiquen de manera coherente en `Type` y `Name`.

Esta historia no cambia la semántica de `iter`.

## Resultado conceptual

Ejemplo conceptual:

```text
scope-fs …/target > iter

Path: /home/user/repos/evolution/evo-shell/target

#   Modified             Type    Size      Name
0   04/08/2026 12:10     dir               flycheck0/
1   05/08/2026 00:18     dir               debug/
2   04/08/2026 12:10     file    1.7 kB    .rustc_info.json
3   04/08/2026 02:02     file    177 B     CACHEDIR.TAG
4   05/08/2026 00:18     dir               release/

3 directories
2 files

Path: /home/user/repos/evolution/evo-shell/target

scope-fs …/target >
```

Los valores son conceptuales.

Los colores no pueden representarse completamente en texto Markdown.

Funcionalmente:

- `scope-fs` y `…/target` tienen colores distintos en el prompt;
- `dir` y el nombre de directorio, por ejemplo `release/`, comparten la misma diferenciación visual de directorio.

## Regla de espaciado

`iter` comienza su presentación con una línea vacía y termina su presentación con una línea vacía.

Secuencia conceptual:

```text
command
<blank line>

Path
<blank line>

table
<blank line>

summary
<blank line>

Path
<blank line>

prompt
```

Esta historia habla de líneas vacías.

No define “dos espacios” como regla visual.

## Path superior

Después de ejecutar:

```text
iter
```

debe mostrarse la ruta completa de la ubicación que está siendo iterada.

Formato conceptual:

```text
Path: /home/user/repos/evolution/evo-shell/target
```

Debe aparecer después de una línea vacía desde el comando.

Después debe existir otra línea vacía antes de la tabla.

Ejemplo:

```text
scope-fs …/target > iter

Path: /home/user/repos/evolution/evo-shell/target

#   Modified ...
```

## Path inferior

Después del resumen debe mostrarse nuevamente la ruta completa.

Ejemplo:

```text
3 directories
2 files

Path: /home/user/repos/evolution/evo-shell/target

scope-fs …/target >
```

Motivo funcional:

si la iteración produce muchas filas, el usuario debe poder conocer el contexto del resultado sin regresar visualmente al principio.

Debe existir una línea vacía antes del siguiente prompt.

## Path como contexto

`Path:` representa la ubicación completa que está siendo iterada.

No representa:

- el prompt compacto;
- una ruta abreviada;
- `…/target`;
- cómo llegó el usuario a esa ubicación.

Debe representar conceptualmente el path completo correspondiente al scope de la iteración.

Esta historia no define todavía cómo se obtiene internamente.

## Prompt compacto

Se mantiene el formato funcional ya existente:

```text
scope-fs …/target >
```

El prompt no vuelve a mostrar el path completo.

El prompt compacto y `Path:` cumplen responsabilidades diferentes.

Prompt:

- rápido;
- compacto;
- deja espacio para escribir.

`Path:`:

- muestra contexto completo;
- aparece al consultar la iteración.

## Colores del prompt

El prompt debe distinguir visualmente:

```text
scope-fs
```

y:

```text
…/target
```

mediante colores diferentes.

Conceptualmente:

```text
scope-fs  → color A
…/target  → color B
>          → presentación neutra o consistente
```

Esta historia no fija códigos de terminal, valores RGB ni una paleta definitiva.

Solo exige diferenciación visual coherente.

## Color de directorios en tabla

En una fila de directorio:

```text
Type:
dir

Name:
target/
```

ambos deben compartir la misma diferenciación visual de directorio.

Conceptualmente:

```text
dir      target/
^^^^     ^^^^^^^
mismo color visual
```

Esto permite asociar rápidamente tipo y nombre.

Esta historia no exige que el resto de columnas tengan color.

## Files

Para archivos:

```text
Type:
file

Name:
Cargo.toml
```

pueden conservar presentación neutra o default.

Esta historia no exige color especial para archivos.

## Symlink y Other

`symlink` y `other` pueden seguir diferenciándose visualmente.

Esta historia no redefine su paleta concreta.

Esta historia no modifica su semántica funcional.

## Header

La tabla conserva el header aprobado:

```text
#   Modified             Type    Size      Name
```

No se agregan columnas nuevas.

No se quitan columnas.

No se modifica la semántica de:

- `#`
- `Modified`
- `Type`
- `Size`
- `Name`

## Summary

El resumen conserva:

```text
N directories
N files
```

No se agrega total acumulado de bytes.

No se agregan conteos nuevos de symlink u other.

No se cambia la lógica funcional aprobada en US-004.

## Lista vacía

Si la ubicación no contiene elementos, `Path:` debe mostrarse igualmente antes y después de la tabla.

Ejemplo conceptual:

```text
scope-fs …/empty > iter

Path: /home/user/empty

#   Modified             Type    Size      Name

0 directories
0 files

Path: /home/user/empty

scope-fs …/empty >
```

El contexto `Path:` se muestra incluso si la iteración está vacía.

## Relación con futuro `pwd`

Mostrar el path completo dentro de la salida de `iter` puede ser reutilizable conceptualmente por una futura capacidad equivalente a `pwd`.

Esta historia no define:

- comando `pwd`;
- sintaxis de `pwd`;
- historia de usuario para `pwd`;
- arquitectura para `pwd`;
- implementación relacionada con `pwd`.

US-005 debe poder completarse sin que exista `pwd`.

## Colores y terminales

La diferenciación visual debe funcionar en terminales compatibles con capacidades estándar modernas de terminal.

Esta historia no fija:

- una terminal concreta;
- códigos de terminal exactos;
- true color;
- tema oscuro;
- tema claro.

La historia no depende de una terminal específica.

## Criterios de aceptación

1. `iter` deja una línea vacía antes de comenzar su presentación.
2. Se muestra `Path:` con la ruta completa antes de la tabla.
3. Existe una línea vacía entre `Path:` y la tabla.
4. La tabla conserva:
   - `#`;
   - `Modified`;
   - `Type`;
   - `Size`;
   - `Name`.
5. El resumen conserva:
   - `N directories`;
   - `N files`.
6. Después del resumen existe una línea vacía.
7. `Path:` vuelve a mostrarse después del resumen.
8. Existe una línea vacía entre el `Path:` final y el siguiente prompt.
9. El `Path:` superior y el `Path:` inferior representan la misma ubicación iterada.
10. `Path:` se muestra incluso cuando no existen elementos.
11. El prompt permanece compacto:

    ```text
    scope-fs …/target >
    ```

12. `scope-fs` y `…/target` se distinguen visualmente mediante colores diferentes.
13. Para directories, `dir` y el nombre del directorio usan la misma diferenciación visual.
14. Esta historia no altera index, metadata, size ni comportamiento lazy.
15. No se agrega total acumulado de bytes.

## Ejemplos

### A. Iteración con elementos

Entrada:

```text
scope-fs …/target
iter
```

Resultado conceptual:

```text
scope-fs …/target > iter

Path: /home/user/repos/evolution/evo-shell/target

#   Modified             Type    Size      Name
0   04/08/2026 12:10     dir               flycheck0/
1   05/08/2026 00:18     dir               debug/
2   04/08/2026 12:10     file    1.7 kB    .rustc_info.json
3   04/08/2026 02:02     file    177 B     CACHEDIR.TAG
4   05/08/2026 00:18     dir               release/

3 directories
2 files

Path: /home/user/repos/evolution/evo-shell/target

scope-fs …/target >
```

Los colores no pueden representarse completamente en texto Markdown.

Funcionalmente:

- `scope-fs` y `…/target` tienen colores distintos;
- `dir` y `directory-name/` comparten color de directory.

### B. Iteración vacía

Entrada:

```text
scope-fs …/empty
iter
```

Resultado conceptual:

```text
scope-fs …/empty > iter

Path: /home/user/empty

#   Modified             Type    Size      Name

0 directories
0 files

Path: /home/user/empty

scope-fs …/empty >
```

## No modificar comportamiento

US-005 no modifica:

- semántica de `iter`;
- índice;
- orden;
- lazy iteration;
- tamaño;
- modified;
- type;
- name;
- summary;
- `enter`;
- `scope-fs` como comando;
- filesystem scope activo.

Es una mejora de contexto y presentación.

## Fuera de alcance

Esta historia no define todavía:

- `pwd`;
- pipelines;
- index operator;
- filter;
- select;
- first;
- to-value;
- to-text;
- sorting;
- pagination;
- terminal-width detection;
- configurable themes;
- configurable colors;
- generic renderer;
- DB scopes;
- URL scopes;
- Web API scopes;
- implementación Rust;
- agents;
- resolvers;
- providers;
- contracts;
- estructuras internas para transportar `Path`.
