# US-001 — Establecer un filesystem scope mediante un comando

## Historia de usuario

Como usuario de Evo Shell,
quiero establecer un directorio como mi filesystem scope mediante el comando `scope-fs`,
para que los comandos posteriores puedan operar dentro de ese ámbito.

## Descripción

Evo Shell proporciona una interfaz de comandos sobre las capacidades de Evo Shell Engine.

El comando:

```text
scope-fs
```

permite indicar la ubicación que se utilizará como filesystem scope activo de la shell.

Sintaxis:

```text
scope-fs "<path>"
```

Ejemplo:

```text
scope-fs "/home/user/documents"
```

`scope-fs` representa una única instrucción.

El espacio separa el nombre de la instrucción de su argumento.

La ruta entre comillas representa el argumento proporcionado al comando.

Evo Shell interpreta la instrucción y solicita a Evo Shell Engine resolver el filesystem scope indicado.

Cuando Evo Shell está operativa, ya posee un filesystem scope inicial válido.

`scope-fs` no crea el primer scope operativo.

Si Evo Shell Engine devuelve un `FilesystemScope` válido, Evo Shell reemplaza el filesystem scope activo anterior por el nuevo scope.

## Criterios de aceptación

1. El usuario puede introducir `scope-fs` seguido de una ruta.
2. Evo Shell reconoce `scope-fs` como una única instrucción.
3. La ruta proporcionada se interpreta como argumento de `scope-fs`.
4. Evo Shell solicita a Evo Shell Engine establecer el filesystem scope indicado.
5. Si Evo Shell Engine devuelve un `FilesystemScope` válido, Evo Shell reemplaza el filesystem scope activo anterior.
6. Si Evo Shell Engine rechaza la operación, Evo Shell informa que el scope no pudo establecerse.
7. Si el nuevo scope no puede establecerse, el scope anterior permanece activo.
8. Una instrucción que no cumple la sintaxis requerida por `scope-fs` no debe ejecutarse como una solicitud válida al engine.
9. Evo Shell nunca queda operativa sin scope por un fallo de `scope-fs`.

## Ejemplos

### Ejemplo exitoso

Entrada:

```text
scope-fs "/home/user/documents"
```

Resultado conceptual:

```text
Scope activo:
fs "/home/user/documents"
```

### Ejemplo inválido

Entrada:

```text
scope-fs
```

Resultado conceptual:

```text
El comando requiere una ruta.
```

Los mensajes mostrados son conceptuales.

Esta historia no define todavía el renderer ni el formato definitivo de los mensajes de error.

## Dependencia funcional

Evo Shell es responsable de:

* recibir la instrucción;
* interpretar el comando y sus argumentos;
* conservar el scope activo;
* presentar el resultado al usuario.

Evo Shell Engine es responsable de:

* resolver la ubicación solicitada;
* producir un `FilesystemScope` válido o un error.

Evo Shell consume el use case de frontera correspondiente de Evo Shell Engine.

Evo Shell no duplica la implementación necesaria para resolver un filesystem scope.

## Estado

Cuando Evo Shell está operativa, ya posee un filesystem scope inicial válido.

El comando `scope-fs` reemplaza ese scope activo existente.

Relación conceptual:

```text
inicio de Evo Shell
    ↓
directorio actual
    ↓
FilesystemScope inicial

scope-fs "<path>"
    ↓
nuevo FilesystemScope válido
    ↓
reemplaza el anterior
```

Si el nuevo scope no puede resolverse, el scope anterior permanece activo.

Evo Shell nunca queda operativa sin scope por un fallo de `scope-fs`.

Esta historia no define todavía cómo se representa o almacena internamente ese estado.

## Fuera de alcance

Esta historia no define todavía:

* el comando `iter`;
* múltiples scopes simultáneos;
* `scope-db`;
* `scope-url`;
* `scope-webapi`;
* variables;
* pipes;
* Evo Script;
* recursividad;
* detalles internos del lexer;
* detalles internos del parser;
* estructura del AST;
* implementación de terminal;
* UI gráfica.
