# US-002 — Iterar el filesystem scope activo mediante el comando iter

## Historia de usuario

Como usuario de Evo Shell,
quiero ejecutar el comando `iter` sobre mi filesystem scope activo,
para ver los elementos directamente contenidos dentro de ese ámbito.

## Descripción

Evo Shell proporciona una interfaz de comandos sobre las capacidades de Evo Shell Engine.

El comando:

```text
iter
```

permite solicitar la iteración de los elementos directamente contenidos en el filesystem scope activo.

`iter` representa una única instrucción sin argumentos.

Sintaxis:

```text
iter
```

Una Evo Shell operativa siempre posee un `FilesystemScope` válido.

Ejemplo conceptual:

```text
scope-fs "/home/user/documents"
iter
```

Evo Shell es responsable de:

- reconocer `iter`;
- consumir la capacidad `Iter` de Evo Shell Engine;
- consumir la capacidad `Advance` de Evo Shell Engine;
- consumir la iteración incrementalmente;
- presentar cada elemento al usuario;
- conservar intacto el filesystem scope activo.

Evo Shell Engine es responsable de:

- iniciar una `FilesystemIteration`;
- producir elementos del filesystem mediante su dominio;
- mantener el comportamiento lazy de la iteración;
- reportar errores operativos.

## Criterios de aceptación

1. El usuario puede introducir `iter`.
2. Evo Shell reconoce `iter` como una única instrucción sin argumentos.
3. Una Evo Shell operativa siempre posee un `FilesystemScope` válido.
4. `iter` recibe acceso al filesystem scope activo durante operación normal.
5. Evo Shell consume `Iter(&FilesystemScope)`.
6. Evo Shell consume `Advance(&mut FilesystemIteration)`.
7. El filesystem scope se pasa mediante préstamo y no se modifica.
8. Si el engine inicia correctamente la iteración:
   - Evo Shell consume los elementos de forma incremental/lazy.
9. Evo Shell presenta cada `FilesystemEntry` conforme es consumido.
10. Evo Shell no necesita acumular todos los elementos antes de presentarlos.
11. El comando no es recursivo.
12. Si ocurre un error operativo durante la iteración:
    - Evo Shell informa el error;
    - el filesystem scope activo permanece intacto.
13. `iter` no modifica ni reemplaza el scope activo.

## Ejemplos

### Ejemplo exitoso

Entrada:

```text
scope-fs "/home/user/documents"
iter
```

Resultado conceptual:

```text
report.txt
images/
notes.md
```

El formato visual es conceptual.

Esta historia no fija todavía el renderer definitivo.

## Dependencia funcional

Evo Shell es responsable de:

- recibir la instrucción;
- interpretar que `iter` no recibe argumentos;
- prestar el filesystem scope activo al engine;
- consumir incrementalmente los elementos producidos;
- presentar el resultado al usuario.

Evo Shell Engine es responsable de:

- iniciar la iteración del filesystem scope;
- producir `FilesystemEntry` como resultado estructurado del dominio;
- informar errores operativos de iteración.

Evo Shell consume el use case de frontera `Iter` de Evo Shell Engine.

Evo Shell consume también `Advance` de Evo Shell Engine para obtener como máximo un `FilesystemEntry` por llamada.

Evo Shell no duplica la implementación de iteración del filesystem.

## Estado

Esta historia consume el estado funcional introducido por US-001.

Una Evo Shell operativa siempre posee un `FilesystemScope` válido.

No existe un error normal de “scope ausente” dentro de una shell operativa.

Relación conceptual:

```text
inicio de Evo Shell
    ↓
directorio actual
    ↓
FilesystemScope inicial

scope-fs
    ↓
reemplaza FilesystemScope activo en éxito

iter
    ↓
borrow &FilesystemScope
    ↓
Iter
    ↓
Advance
```

`iter` no toma ownership del filesystem scope activo.

`iter` no modifica ni reemplaza el filesystem scope activo.

## Fuera de alcance

Esta historia no define todavía:

- iteración recursiva;
- filtros;
- pipes;
- sorting;
- metadata adicional;
- paginación;
- argumentos de `iter`;
- múltiples scopes;
- autocomplete;
- history;
- aliases;
- Evo Script;
- UI gráfica.
