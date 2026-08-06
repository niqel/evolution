# US-008 — Limpiar la terminal

## Historia de usuario

Como usuario de Evo Shell,
quiero limpiar la terminal mediante el comando `clear`,
para recuperar una vista despejada y reiniciar el buffer visual (incluyendo el historial/scrollback) sin cambiar mi scope activo.

## Descripción

Evo Shell permite limpiar completamente la presentación de la terminal (pantalla visible e historial visual de scrollback) mediante el comando:

```text
clear
```

## `clear`

`clear` limpia la pantalla visible y el scrollback / historial visual de la terminal.

Después de ejecutar el comando:

- la pantalla visible queda limpia;
- el scrollback/historial visual se limpia cuando la terminal lo soporte;
- el cursor queda en la posición inicial apropiada;
- Evo Shell continúa ejecutándose;
- el prompt vuelve a mostrarse normalmente;
- el filesystem scope activo no cambia.

Ejemplo conceptual:

```text
scope-fs …/src > clear
```

Resultado:

```text
scope-fs …/src >
```

La terminal ha sido completamente limpiada. El scope activo continúa siendo el mismo.

## Sin argumentos ni opciones

`clear` no recibe argumentos posicionales ni opciones en esta historia.

Por tanto, ninguna de estas formas constituye sintaxis válida:

```text
clear all
clear --all
clear --unknown
clear extra
```

## Errores funcionales

Evo Shell debe rechazar entradas que no correspondan a la sintaxis exacta `clear`.

El comportamiento observable de error sigue la política estándar de errores de parsing de Evo Shell (`ParseError::UnexpectedToken`).

## Scope activo

`clear` no modifica el filesystem scope activo.

Ejemplo:

```text
scope-fs …/src > clear
scope-fs …/src >
```

El prompt conserva la misma ubicación compacta porque el scope no cambia.

## Criterios de aceptación

1. El usuario puede introducir `clear`.
2. `clear` limpia la pantalla visible y el scrollback / historial visual de la terminal.
3. Después de `clear`, Evo Shell continúa ejecutándose.
4. Después de `clear`, el prompt vuelve a mostrarse normalmente.
5. Después de `clear`, el filesystem scope activo no cambia.
6. `clear --all` ya no es sintaxis válida y retorna error de parsing.
7. `clear all` o `clear` con cualquier argumento u opción no se interpreta como comando válido.

## Fuera de alcance

Esta historia no define:

- `clear -a`;
- aliases;
- opciones cortas o largas;
- opciones configurables;
- detección de capacidades avanzadas de terminal;
- comandos externos como `clear` o `cls`;
- historial de comandos;
- cambio de scope;
- cambios de colores;
- Evo Script.
