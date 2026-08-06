# US-010 — Terminar Evo Shell mediante el comando `exit`

## Historia de usuario

Como usuario de Evo Shell,
quiero terminar la sesión actual mediante un comando explícito,
para salir de forma limpia y cooperativa cuando ya no necesite seguir trabajando en la shell.

## Descripción

Evo Shell permite terminar la sesión actual mediante:

```text
exit
```

El comando `exit` solicita terminar el proceso `evo-shell` de forma normal.

La terminación es cooperativa:

- no mata la terminal anfitriona;
- no ejecuta comandos externos;
- no usa `std::process::exit(0)` dentro del flujo normal;
- no aborta el proceso;
- no salta destructores.

Después de `exit`:

- no aparece otro prompt;
- no se ejecutan más comandos;
- no se limpia la terminal automáticamente;
- no se imprime un mensaje adicional;
- el proceso termina correctamente con código `0` cuando no existe otro error.

## Comportamiento observable

Ejemplo conceptual:

```text
scope-fs …/src > exit
```

Resultado:

```text
Evo Shell termina normalmente.
```

La terminal anfitriona permanece bajo control de su propio proceso padre.

Si `evo-shell` fue lanzado desde otra shell, esa terminal permanece abierta.

Si la terminal fue configurada para ejecutar solo `evo-shell`, cualquier cierre de la aplicación terminal ocurre como consecuencia externa de que el proceso hijo terminó, no porque Evo Shell controle esa terminal.

## Sin argumentos ni opciones

`exit` no recibe argumentos ni opciones en esta historia.

Por tanto, no son válidas estas formas:

```text
exit now
exit --force
exit 0
```

La regla transversal de argumentos y opciones sigue siendo la definida por:

[LR-001 — Command Arguments and Options](../language_rules/LR-001-command-arguments-and-options.md)

US-010 no redefine esa regla.

## EOF

EOF sigue siendo una forma válida e independiente de terminar la sesión.

`exit` y EOF son caminos distintos de salida, pero ambos terminan la sesión de forma normal.

Esta historia no convierte EOF en `exit`.

## Criterios de aceptación

1. El usuario puede ejecutar `exit`.
2. `exit` termina la sesión actual de Evo Shell.
3. Después de `exit` no aparece otro prompt.
4. No se ejecutan más comandos después de `exit`.
5. `exit` no limpia la terminal automáticamente.
6. `exit` no cambia el filesystem scope antes de salir.
7. `exit` no recibe argumentos.
8. `exit` no recibe opciones.
9. `exit now` es inválido.
10. `exit --force` es inválido.
11. `exit 0` es inválido.
12. Evo Shell termina normalmente con código `0` cuando no existe otro error.
13. Evo Shell no intenta cerrar la terminal anfitriona.
14. EOF continúa siendo una forma válida de terminar la sesión.
15. No se agregan dependencias nuevas.

## Fuera de alcance

Esta historia no define:

- `exit --force`;
- `exit --code`;
- `exit 1`;
- `quit`;
- `logout`;
- `close`;
- `shutdown`;
- `restart`;
- confirmaciones;
- mensaje de despedida;
- clear automático al salir;
- persistencia de sesión;
- historial de shell;
- manejo de señales;
- política de `Ctrl+C`;
- códigos de salida personalizados;
- cambios en Evo Shell Engine.
