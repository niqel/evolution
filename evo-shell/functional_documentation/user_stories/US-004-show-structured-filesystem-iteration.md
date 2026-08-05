# US-004 — Mostrar información estructurada de los elementos mediante `iter`

## Historia de usuario

Como usuario de Evo Shell,
quiero que `iter` muestre información estructurada de cada elemento del filesystem scope activo,
para entender mejor qué contiene la ubicación actual sin perder una lectura simple de la iteración.

## Descripción

Evo Shell permite observar los elementos directamente contenidos en el filesystem scope activo mediante el comando:

```text
iter
```

Esta historia enriquece la presentación observable de `iter` para que cada elemento se muestre como una fila con información útil y legible.

La salida debe incluir conceptualmente estas columnas:

- `#`
- `Modified`
- `Type`
- `Size`
- `Name`

Ejemplo conceptual:

```text
scope-fs …/evo-shell > iter

#   Modified             Type   Size      Name
0   05/08/2026 00:42     file   1.2 kB    Cargo.toml
1   05/08/2026 00:40     dir              src
2   05/08/2026 00:44     dir              target
3   04/08/2026 23:03     dir              technical_documentation
4   04/08/2026 22:40     file   3.8 kB    README.md
5   04/08/2026 21:55     dir              functional_documentation
6   05/08/2026 00:30     file   151 B     Cargo.lock

4 directories
3 files

scope-fs …/evo-shell >
```

Los valores mostrados son conceptuales.

Esta historia no asume que esos archivos ni esas fechas existan siempre.

Esta historia no fija todavía el renderer definitivo, la alineación exacta, colores, códigos de terminal ni detalles internos de presentación.

## Columna `#`

`#` representa el índice ordinal del elemento dentro de la iteración actual.

La numeración inicia en `0`.

Ejemplo:

```text
0
1
2
3
```

El índice representa únicamente la posición del elemento dentro de la iteración producida por `iter`.

El índice no representa:

- inode de Linux;
- file ID de Windows;
- identificador persistente del filesystem;
- posición permanente del archivo;
- identidad del archivo.

Como motivación futura, el índice debe poder entenderse como un dato real de la iteración y no como una numeración visual inventada por la tabla.

Esta historia no define todavía comandos u operadores para seleccionar elementos por índice.

## Columna `Modified`

`Modified` representa la fecha y hora de última modificación del elemento cuando el filesystem la proporciona.

Formato funcional deseado:

```text
DD/MM/YYYY HH:MM
```

Ejemplo:

```text
05/08/2026 00:42
```

La presentación final puede adaptarse cuando la plataforma no pueda proporcionar exactamente el mismo dato.

Esta historia no define todavía reglas de zona horaria ni detalles de obtención del dato.

## Columna `Type`

`Type` muestra conceptualmente el tipo del elemento.

Valores iniciales:

- `file`
- `dir`
- `symlink`
- `other`

Ejemplos:

```text
file
dir
```

La salida no debe mostrar nombres técnicos internos como representación visible del tipo.

## Columna `Size`

`Size` representa el tamaño de archivos cuando ese dato está disponible.

Ejemplos conceptuales:

```text
151 B
1.2 kB
52.7 kB
2.4 MB
```

Esta historia no fija todavía el algoritmo exacto de conversión de bytes.

Para directorios, `iter` no debe calcular recursivamente el tamaño de todo su contenido.

La celda de tamaño de un directorio puede quedar vacía.

Ejemplo conceptual:

```text
Type   Size
dir
file   52.7 kB
```

Lo mismo puede aplicarse a otros tipos donde el tamaño no tenga significado útil para esta presentación.

## Columna `Name`

`Name` muestra el nombre visible del elemento.

Ejemplos:

```text
src
Cargo.toml
```

Evo Shell puede conservar comportamientos visuales que ayuden a distinguir directorios o symlinks cuando eso mejore la lectura.

Esta historia no define colores, sufijos ni símbolos exactos como parte obligatoria del dato.

## Resumen final

Al finalizar la iteración, `iter` debe mostrar un resumen simple de los elementos principales encontrados.

Ejemplo:

```text
2 directories
1 file
```

Otro ejemplo:

```text
4 directories
3 files
```

El resumen inicial se limita a:

- cantidad de directories;
- cantidad de files.

Si existen symlinks u otros tipos, pueden seguir apareciendo como filas sin alterar el resumen inicial.

`iter` no debe mostrar una suma total de tamaños.

Ejemplo de algo que esta historia no solicita:

```text
5.1 kB
```

como total acumulado de todos los archivos.

## Comportamiento progresivo

Desde la perspectiva del usuario, `iter` debe continuar produciendo resultados de manera progresiva.

El usuario puede recibir filas conforme la iteración produce elementos.

Esta historia no exige que toda la colección esté disponible antes de comenzar a mostrar resultados.

## Orden

Esta historia no introduce un orden nuevo para los elementos.

El índice representa el orden concreto en que `iter` produce los elementos.

La historia no promete:

- orden alfabético;
- directorios primero;
- orden por fecha;
- orden por tamaño.

Si el orden depende del filesystem o de comportamiento ya existente, eso permanece fuera del alcance de esta historia.

## Colores

Evo Shell puede usar color para mejorar la legibilidad de la salida.

Ejemplos conceptuales:

- encabezados diferenciados;
- directorios diferenciados visualmente;
- files o symlinks distinguibles.

Los colores son presentación, no datos del filesystem.

Esta historia no fija códigos de terminal, RGB, paleta exacta, tema oscuro o claro, ni dependencia de terminal.

## Criterios de aceptación

1. `iter` muestra una fila por cada elemento producido.
2. Cada fila incluye conceptualmente:
   - índice;
   - modified;
   - type;
   - size;
   - name.
3. El índice comienza en `0`.
4. El índice representa la posición dentro de la iteración actual.
5. El índice no representa identidad persistente del filesystem.
6. `Modified` muestra fecha y hora de modificación cuando esté disponible.
7. `Type` distingue al menos:
   - `file`;
   - `dir`;
   - `symlink`;
   - `other`.
8. `Size` muestra tamaño para archivos cuando esté disponible.
9. Los directorios no requieren cálculo recursivo de tamaño.
10. `Name` muestra el nombre del elemento.
11. Al finalizar se muestra un resumen de files y directories.
12. No se muestra suma total de tamaños.
13. `iter` conserva comportamiento progresivo desde la perspectiva del usuario.
14. Esta historia no introduce ordenamiento nuevo.
15. La salida puede usar color como ayuda visual sin convertirlo en propiedad del dato.

## Ejemplos

### Ejemplo con salida estructurada

Entrada:

```text
scope-fs …/evo-shell
iter
```

Resultado conceptual:

```text
scope-fs …/evo-shell > iter

#   Modified             Type   Size      Name
0   05/08/2026 00:42     file   1.2 kB    Cargo.toml
1   05/08/2026 00:40     dir              src
2   05/08/2026 00:44     dir              target
3   04/08/2026 23:03     dir              technical_documentation
4   04/08/2026 22:40     file   3.8 kB    README.md
5   04/08/2026 21:55     dir              functional_documentation
6   05/08/2026 00:30     file   151 B     Cargo.lock

4 directories
3 files

scope-fs …/evo-shell >
```

El formato visual es conceptual.

Esta historia no fija todavía alineación exacta, colores, ni comportamiento específico de terminal.

## Relación con historias anteriores

Esta historia extiende la presentación observable de `iter` definida en US-002.

US-002 define que `iter` enumera los elementos directamente contenidos en el filesystem scope activo.

US-004 define que esa enumeración se presenta con información estructurada adicional.

`iter` sigue sin ser recursivo.

`iter` no modifica ni reemplaza el filesystem scope activo.

## Fuera de alcance

Esta historia no define todavía:

- pipelines;
- `index 0` como comando u operador;
- filter;
- select;
- first;
- to-value;
- to-text;
- Evo Script;
- ordenamiento;
- paginación;
- búsqueda;
- tamaño recursivo de directorios;
- total acumulado de bytes;
- permisos;
- owner;
- group;
- inode;
- Windows file ID;
- hash;
- checksum;
- creation time;
- access time;
- renderer técnico;
- ANSI;
- tema de colores configurable;
- DB scopes;
- URL scopes;
- Web API scopes;
- implementación Rust.
