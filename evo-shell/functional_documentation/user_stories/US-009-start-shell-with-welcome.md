# US-009 — Iniciar Evo Shell con una presentación de bienvenida

## Historia de usuario

Como usuario de Evo Shell,
quiero que la shell me dé la bienvenida al iniciar,
para comenzar con una pantalla limpia y un contexto visual claro.

## Descripción

Al iniciar Evo Shell:

1. la pantalla visible se limpia;
2. se muestra una bienvenida breve;
3. aparece una línea vacía;
4. se muestra el prompt normal.

La limpieza inicial no borra automáticamente el scrollback de la terminal.

La pantalla visible queda limpia, pero el historial visual anterior permanece accesible cuando la terminal lo permita.

La bienvenida muestra exactamente estas tres líneas:

```text
CatarinaSoft
evo-shell {version}
evo-shell is a life :)
```

La versión visible corresponde a la versión actual del paquete `evo-shell`.

## Comportamiento observable

Ejemplo conceptual:

```text
CatarinaSoft
evo-shell 0.1.0
evo-shell is a life :)

scope-fs …/evo-shell >
```

Los valores concretos pueden variar según la versión actual del paquete.

No se agrega ASCII art.

No se agrega Unicode emoji.

No se agrega información adicional como fecha, hora, usuario u hostname.

## Limpieza inicial

La limpieza inicial solo afecta la pantalla visible.

No modifica:

- el filesystem scope activo;
- la ubicación activa;
- la sintaxis del prompt;
- el comportamiento de `iter`;
- el comportamiento de `enter`;
- el comportamiento de `clear` y `clear --all`.

`clear --all` sigue siendo una acción explícita del usuario.

El startup solo limpia la pantalla visible.

## Scope inicial

Evo Shell inicia con un filesystem scope inicial válido.

La bienvenida no altera ese scope.

El prompt que aparece después de la bienvenida corresponde al scope inicial resuelto de la shell.

## Error de inicio

Si Evo Shell no puede inicializarse correctamente:

- no debe mostrarse la bienvenida;
- no debe comenzar el loop normal;
- la shell debe reportar el error de inicialización según la política existente.

Si la limpieza inicial falla:

- la bienvenida no debe mostrarse;
- no debe comenzar el loop normal.

Si la presentación de bienvenida falla:

- no debe comenzar el loop normal.

## Criterios de aceptación

1. Al iniciar Evo Shell, la pantalla visible se limpia.
2. El startup no limpia automáticamente el scrollback.
3. Se muestra `CatarinaSoft`.
4. Se muestra `evo-shell` y la versión actual.
5. La versión corresponde a la versión del paquete `evo-shell`.
6. Se muestra `evo-shell is a life :)`.
7. Después de la bienvenida existe una línea vacía.
8. Después aparece el prompt normal.
9. El prompt conserva su identidad visual actual.
10. El filesystem scope inicial es válido.
11. Si la inicialización falla, no se muestra startup normal.
12. No se agrega ASCII art.
13. No se agregan emojis Unicode.
14. No se agregan dependencias nuevas.

## Fuera de alcance

Esta historia no define:

- opciones de inicio;
- `--no-banner`;
- desactivar welcome;
- detección de capacidades de terminal;
- animaciones;
- retrasos;
- `sleep`;
- sonidos;
- archivos de configuración;
- banner de entorno;
- hostname;
- username;
- cambios en Evo Shell Engine.
