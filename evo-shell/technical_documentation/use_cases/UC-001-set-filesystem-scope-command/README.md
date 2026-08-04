# UC-001 — Establecer un filesystem scope mediante un comando

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y ejecuta el comando:

```text
scope-fs "<path>"
```

Evo Shell recibe texto, lo tokeniza incrementalmente, interpreta los tokens, resuelve el comando, ejecuta el comando consumiendo el use case de frontera `SetFilesystemScope` de Evo Shell Engine, recibe un `FilesystemScope` válido o un error, reemplaza el scope activo únicamente si la operación tuvo éxito y presenta un resultado conceptual al usuario.

Evo Shell no duplica la lógica de filesystem de Evo Shell Engine.

## Actor

- Usuario de Evo Shell

## Entrada

Entrada textual:

```text
scope-fs "/home/user/documents"
```

Gramática establecida para este caso:

- `scope-fs` es una única instrucción.
- el espacio separa la instrucción de sus argumentos.
- `"/home/user/documents"` representa el argumento.

## Precondiciones

- Evo Shell recibe una línea de entrada textual.
- Evo Shell puede consumir el use case de frontera `SetFilesystemScope` de Evo Shell Engine.
- Evo Shell está en estado operativo y ya posee un filesystem scope activo válido.

No se requieren precondiciones técnicas adicionales.

## Flujo principal

1. El usuario introduce `scope-fs "<path>"`.
2. Evo Shell recibe el texto.
3. `tokenizer::tokenize` coordina la tokenización incremental.
4. `token::resolve` resuelve `Token::Word("scope-fs")`.
5. `token::resolve` resuelve `Token::String(path)`.
6. `parser::parse` coordina la interpretación de los tokens.
7. `command::resolve` resuelve los tokens como `Command::ScopeFs(path)`.
8. `executor::execute` coordina la ejecución del comando resuelto.
9. `execution::resolve` resuelve que `Command::ScopeFs(path)` debe consumir `SetFilesystemScope`.
10. Evo Shell llama el use case de frontera `SetFilesystemScope` de Evo Shell Engine.
11. Evo Shell Engine devuelve un `FilesystemScope` válido.
12. Evo Shell conserva ownership del nuevo `FilesystemScope` como filesystem scope activo.
13. El nuevo `FilesystemScope` reemplaza el scope anterior.
14. Evo Shell presenta un resultado conceptual al usuario.

## Error sintáctico

Un error sintáctico pertenece a Evo Shell.

Ejemplo conceptual:

```text
scope-fs
```

Flujo:

1. Evo Shell recibe la entrada.
2. La entrada no cumple la sintaxis requerida por `scope-fs`.
3. Evo Shell no ejecuta una solicitud válida al engine.
4. Evo Shell presenta un error conceptual al usuario.
5. El filesystem scope activo actual permanece activo.

## Error del engine

Un error operativo del engine ocurre cuando Evo Shell llamó a `SetFilesystemScope`, pero Evo Shell Engine no pudo resolver el scope.

Flujo:

1. Evo Shell interpreta correctamente `scope-fs "<path>"`.
2. `execution::resolve` consume `SetFilesystemScope`.
3. Evo Shell Engine devuelve un error.
4. Evo Shell presenta un error conceptual al usuario.
5. El filesystem scope activo actual permanece activo.

## Resultado exitoso

Ejemplo conceptual:

```text
Scope activo:
fs "/home/user/documents"
```

## Comportamiento del estado

UC-001 reemplaza el filesystem scope activo previamente existente.

Representación conceptual:

```text
estado de Evo Shell
    filesystem scope activo: requerido durante operación
```

Regla observable:

```text
scope activo actual
        ↓
scope-fs "<path>"
        ↓
SetFilesystemScope
        ↓
nuevo FilesystemScope
        ↓
reemplaza scope anterior

si SetFilesystemScope devuelve éxito:
    el nuevo FilesystemScope reemplaza al anterior

si SetFilesystemScope devuelve error:
    el FilesystemScope anterior permanece intacto
```

Una Evo Shell operativa nunca queda sin filesystem scope activo por un fallo de `scope-fs`.

El engine no mantiene este estado interactivo.

El estado pertenece a Evo Shell.

Este caso de uso no impone todavía nombres técnicos como Session, ShellState o ActiveScope.

## Inmutabilidad y ownership

1. La entrada textual puede prestarse durante tokenizer/parser/executor.
2. Los tokens deben prestarse del texto cuando sea posible.
3. `Command` puede ser efímero y prestar sus argumentos mientras la entrada siga viva.
4. Solo se toma ownership cuando el dato necesita sobrevivir independientemente.
5. La mutabilidad del tokenizer debe quedar localizada en el cursor incremental.
6. El `FilesystemScope` válido recibido del engine sí debe sobrevivir entre comandos y por tanto Evo Shell conserva su ownership.

No se definen todavía firmas Rust concretas ni lifetimes concretos.

## Relación con US-001

[US-001 — Establecer un filesystem scope mediante un comando](../../../functional_documentation/user_stories/US-001-set-filesystem-scope-command.md)

## Relación con Evo Shell Engine

Evo Shell conoce y consume:

- el use case de frontera `SetFilesystemScope`;
- `FilesystemScope`;
- el error público correspondiente.

Evo Shell no conoce ni duplica:

- resolvers internos del engine;
- providers del engine;
- `std::fs`;
- validación/resolución del filesystem.

Flujo de frontera:

```text
Command::ScopeFs(path)
        ↓
execution::resolve
        ↓
SetFilesystemScope
        ↓
Evo Shell Engine
        ↓
Result<FilesystemScope, ...>
```

## Autocompletado

Este caso de uso no implementa ni diseña un sistema de autocompletado.

Representar comandos mediante un enum conceptual no impide agregar posteriormente metadata estática o un catálogo independiente para discovery/autocomplete.

No se crea ese catálogo en UC-001.

## Fuera de alcance

- `iter`
- pipes
- variables
- expresiones
- AST general
- autocomplete
- command registry dinámico
- aliases
- history
- terminal implementation
- line editing
- UI gráfica
- Evo Script
- otros scopes
- async
- Tokio

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
