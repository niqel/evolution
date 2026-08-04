# UC-002 — Iterar el filesystem scope activo mediante el comando iter

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y ejecuta el comando:

```text
iter
```

Evo Shell recibe texto, lo tokeniza incrementalmente, interpreta los tokens, resuelve `Command::Iter`, presta el filesystem scope activo garantizado durante operación, consume el use case de frontera `Iter` de Evo Shell Engine, obtiene una `FilesystemIteration`, consume elementos de forma incremental mediante `Advance` y presenta cada resultado al usuario.

Evo Shell no duplica la lógica de iteración del filesystem de Evo Shell Engine.

## Actor

- Usuario de Evo Shell

## Entrada

Entrada textual:

```text
iter
```

Gramática establecida para este caso:

- `iter` es una única instrucción.
- `iter` no recibe argumentos.
- cualquier token adicional representa un error sintáctico.

## Precondiciones

- Evo Shell recibe una línea de entrada textual.
- Evo Shell puede consumir el use case de frontera `Iter` de Evo Shell Engine.
- Evo Shell está en estado operativo y posee un filesystem scope activo válido.

## Flujo principal

1. El usuario introduce `iter`.
2. Evo Shell recibe el texto.
3. `tokenizer::tokenize` coordina la tokenización incremental.
4. `token::resolve` resuelve `Token::Word("iter")`.
5. `parser::parse` coordina la interpretación de los tokens.
6. `command::resolve` resuelve los tokens como `Command::Iter`.
7. `executor::execute` coordina la ejecución del comando resuelto.
8. `execution::resolve` utiliza el filesystem scope activo garantizado de Evo Shell.
9. Evo Shell presta el scope activo como `&FilesystemScope`.
10. Evo Shell consume el use case de frontera `Iter(&FilesystemScope)` de Evo Shell Engine.
11. Evo Shell Engine devuelve una `FilesystemIteration`.
12. Evo Shell conserva temporalmente ownership de la `FilesystemIteration`.
13. Evo Shell llama `Advance(&mut FilesystemIteration)`.
14. Por cada elemento disponible, Evo Shell recibe como máximo un `FilesystemEntry`.
15. Evo Shell presenta cada elemento al usuario y puede descartarlo cuando ya no se necesita.
16. Evo Shell repite el avance hasta recibir `None` o hasta que ocurra un error operativo.
17. El filesystem scope activo permanece intacto.

## Error sintáctico

Un error sintáctico pertenece a Evo Shell.

Ejemplos conceptuales:

```text
iter extra
```

```text
iter "/home/user/documents"
```

Flujo:

1. Evo Shell recibe la entrada.
2. La entrada no cumple la sintaxis requerida por `iter`.
3. Evo Shell no llama a `Iter` del engine.
4. Evo Shell presenta un error conceptual al usuario.
5. Si ya existía un filesystem scope válido, permanece activo.

## Error del engine

Un error operativo del engine ocurre cuando Evo Shell Engine no puede iniciar o consumir la iteración.

Flujo:

1. Evo Shell interpreta correctamente `iter`.
2. Evo Shell presta el filesystem scope activo garantizado.
3. `execution::resolve` consume `Iter(&FilesystemScope)`.
4. Evo Shell Engine devuelve un error operativo al iniciar la iteración o al avanzar mediante `Advance`.
5. Evo Shell presenta un error conceptual al usuario.
6. El filesystem scope activo permanece intacto.

## Comportamiento lazy

La ejecución de `iter` debe conservar el comportamiento lazy proporcionado por Evo Shell Engine.

Flujo conceptual:

```text
FilesystemIteration
    ↓
Advance
    ↓
FilesystemEntry
    ↓
presentar
    ↓
drop entry
    ↓
Advance
    ↓
FilesystemEntry
    ↓
presentar
    ↓
...
    ↓
None
```

Evo Shell no necesita acumular todos los elementos antes de presentarlos.

No se documenta `Vec<FilesystemEntry>` ni `collect()` como estrategia de ejecución.

## Ownership y borrowing

US-002 consume el filesystem scope activo garantizado durante la operación normal de Evo Shell.

Relación conceptual:

```text
UC-001:
scope-fs
    ↓
reemplaza FilesystemScope activo en éxito

UC-002:
iter
    ↓
borrow &FilesystemScope
    ↓
Iter
    ↓
FilesystemIteration
    ↓
Advance(&mut FilesystemIteration)
```

El ownership del filesystem scope permanece en Evo Shell.

`iter` solo presta el scope al engine.

`iter` no modifica ni reemplaza el filesystem scope activo.

`FilesystemIteration` se posee temporalmente durante la ejecución del comando.

Cada `FilesystemEntry` se produce individualmente y puede presentarse sin conservar entradas anteriores.

No existe un flujo normal de `iter` sin filesystem scope activo dentro de una Evo Shell operativa.

## Presentación

Cada `FilesystemEntry` contiene conceptualmente:

- name
- path
- kind

Evo Shell decide cómo presentar esos datos al usuario.

Para esta historia, la salida visual sigue siendo conceptual:

```text
report.txt
images/
notes.md
```

Este caso de uso no diseña tablas, colores, columnas, iconos ni un renderer definitivo.

La presentación no cambia el dominio de Evo Shell Engine.

## Relación con US-002

[US-002 — Iterar el filesystem scope activo mediante el comando iter](../../../functional_documentation/user_stories/US-002-iterate-filesystem-scope-command.md)

## Relación con UC-001 de Evo Shell

[UC-001 — Establecer un filesystem scope mediante un comando](../UC-001-set-filesystem-scope-command/README.md)

UC-001 produce el filesystem scope activo de Evo Shell.

UC-002 consume ese scope mediante préstamo.

Si `iter` falla, el filesystem scope activo permanece intacto.

## Relación con UC-002 de Evo Shell Engine

Evo Shell consume dos capacidades públicas de Evo Shell Engine:

1. `Iter(&FilesystemScope) -> Result<FilesystemIteration, IterError>`
2. `Advance(&mut FilesystemIteration) -> Result<Option<FilesystemEntry>, IterError>`

Flujo de frontera:

```text
&FilesystemScope
    ↓
Iter
    ↓
FilesystemIteration
    ↓
Advance(&mut FilesystemIteration)
    ↓
FilesystemEntry / fin / error
```

Consumo desde Evo Shell:

```text
obtener FilesystemIteration
    ↓
conservar ownership temporal de la iteración
    ↓
llamar Advance
    ↓
recibir como máximo un FilesystemEntry
    ↓
presentar ese entry
    ↓
descartar el entry cuando ya no se necesita
    ↓
repetir hasta None
```

Evo Shell conoce y consume únicamente la API pública necesaria:

- `Iter`;
- `Advance`;
- `IterError`;
- `FilesystemScope`;
- `FilesystemIteration`;
- `FilesystemEntry`;
- `FilesystemEntryKind`;
- `iteration_advancer::advance`.

Evo Shell no conoce ni duplica:

- `ReadDirectory`;
- `NextDirectoryEntry`;
- providers internos del engine;
- resolvers internos del engine;
- `ReadDir`;
- `DirEntry`;
- `std::fs` interno del engine.

## Fuera de alcance

- `scope-fs`
- iteración recursiva
- filtros
- pipes
- sorting
- metadata adicional
- paginación
- argumentos de `iter`
- múltiples scopes
- autocomplete
- history
- aliases
- Evo Script
- UI gráfica
- AST general
- renderer definitivo
- implementación Rust
- cambios en Evo Shell Engine

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
