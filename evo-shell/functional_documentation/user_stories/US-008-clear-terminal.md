# US-008 — Limpiar la terminal

## Historia de usuario

Como usuario de Evo Shell,
quiero limpiar la terminal mediante el comando `clear`,
para recuperar una vista despejada sin cambiar mi scope activo.

## Descripción

Evo Shell permite limpiar la presentación visible de la terminal mediante:

```text
clear
```

También permite solicitar una limpieza más amplia mediante:

```text
clear --all
```

La sintaxis de opciones y flags sigue la regla transversal:

[LR-001 — Command Arguments and Options](../language_rules/LR-001-command-arguments-and-options.md)

En esta historia:

- `clear` es el comando;
- `--all` es una opción larga;
- `--all` es un flag porque no requiere valor.

US-008 no redefine la regla general de argumentos y opciones.

## `clear`

`clear` limpia únicamente la pantalla visible de la terminal.

Después de ejecutar el comando:

- la pantalla visible queda limpia;
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

La terminal visible ha sido limpiada.

El scope activo continúa siendo el mismo.

## `clear --all`

`clear --all` limpia:

- la pantalla visible;
- el scrollback o historial visual de la terminal cuando la terminal soporte esa operación.

Después de ejecutar el comando:

- la pantalla visible queda limpia;
- el scrollback se limpia cuando sea soportado por la terminal;
- el cursor queda en la posición inicial apropiada;
- Evo Shell continúa ejecutándose;
- el prompt vuelve a mostrarse normalmente;
- el filesystem scope activo no cambia.

Ejemplo conceptual:

```text
scope-fs …/src > clear --all
```

Resultado:

```text
scope-fs …/src >
```

La pantalla visible y, cuando sea soportado, el historial visual de la terminal han sido limpiados.

El scope activo continúa siendo el mismo.

## Sin argumentos posicionales

`clear` no recibe argumentos posicionales en esta historia.

Por tanto, esta no es la sintaxis aprobada:

```text
clear all
```

La forma aprobada para solicitar limpieza de pantalla visible y scrollback es:

```text
clear --all
```

## Opciones reconocidas

US-008 reconoce únicamente:

```text
--all
```

No se reconocen todavía:

- `-a`;
- `--visible`;
- `--screen`;
- `--history`;
- otras opciones.

Las opciones cortas siguen fuera de alcance según LR-001.

## Errores funcionales

Evo Shell debe rechazar entradas que no correspondan a la sintaxis aprobada.

Ejemplos no válidos:

```text
clear all
clear --unknown
clear --all extra
```

El comportamiento observable de error debe seguir la política actual de errores de parsing de Evo Shell.

Esta historia no define un formato nuevo de mensajes de error.

## Scope activo

`clear` y `clear --all` no modifican el filesystem scope activo.

Ejemplo:

```text
scope-fs …/src > clear
scope-fs …/src >
```

El prompt conserva la misma ubicación compacta porque el scope no cambia.

## Criterios de aceptación

1. El usuario puede introducir `clear`.
2. `clear` limpia la pantalla visible de la terminal.
3. Después de `clear`, Evo Shell continúa ejecutándose.
4. Después de `clear`, el prompt vuelve a mostrarse normalmente.
5. Después de `clear`, el filesystem scope activo no cambia.
6. El usuario puede introducir `clear --all`.
7. `clear --all` limpia la pantalla visible.
8. `clear --all` solicita limpiar el scrollback o historial visual cuando la terminal lo soporte.
9. Después de `clear --all`, Evo Shell continúa ejecutándose.
10. Después de `clear --all`, el prompt vuelve a mostrarse normalmente.
11. Después de `clear --all`, el filesystem scope activo no cambia.
12. `--all` se interpreta como opción larga y flag según LR-001.
13. `clear all` no se interpreta como comando válido.
14. `clear --unknown` no se interpreta como comando válido.
15. `clear --all extra` no se interpreta como comando válido.
16. No se agregan opciones cortas.
17. No se agregan opciones distintas de `--all`.

## Fuera de alcance

Esta historia no define:

- `clear -a`;
- aliases;
- opciones cortas;
- opciones configurables;
- detección de capacidades de terminal;
- comandos externos como `clear` o `cls`;
- historial de comandos;
- cambio de scope;
- cambios de colores;
- parser genérico completo de opciones;
- Evo Script.
