# UC-001 — Establecer un filesystem scope mediante un comando

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y ejecuta el comando:

```text
scope-fs "<path>"
```

Evo Shell recibe texto, lo tokeniza incrementalmente, interpreta los tokens, resuelve el comando, ejecuta el comando consumiendo el use case de frontera `SetFilesystemScope` de Evo Shell Engine, recibe un `FilesystemScope` válido y resuelto o un error, reemplaza el scope activo únicamente si la operación tuvo éxito y actualiza la presentación del prompt.

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
- Existe una instancia operativa de `Shell`.
- `Shell` posee un filesystem scope activo válido.

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
9. `execution::resolve` resuelve que `Command::ScopeFs(path)` debe operar sobre `Shell`.
10. `execution::resolve` consume el use case de frontera `SetFilesystemScope`.
11. Evo Shell llama `SetFilesystemScope` de Evo Shell Engine.
12. Evo Shell Engine devuelve un `FilesystemScope` válido y resuelto.
13. `Shell` reemplaza el scope anterior por el nuevo `FilesystemScope`.
14. Evo Shell actualiza el prompt a partir del nuevo scope activo.

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
5. El `FilesystemScope` owned by `Shell` permanece intacto.

## Error del engine

Un error operativo del engine ocurre cuando Evo Shell llamó a `SetFilesystemScope`, pero Evo Shell Engine no pudo resolver el scope.

Flujo:

1. Evo Shell interpreta correctamente `scope-fs "<path>"`.
2. `execution::resolve` consume `SetFilesystemScope`.
3. Evo Shell Engine devuelve un error.
4. Evo Shell presenta un error conceptual al usuario.
5. El `FilesystemScope` owned by `Shell` permanece intacto.

## Resultado exitoso

Ejemplo conceptual:

```text
scope-fs …/documents >
```

Un cambio exitoso de scope no debe imprimir una línea adicional como:

```text
Scope activo: /home/...
```

El prompt representa el estado activo.

## Comportamiento del estado

UC-001 reemplaza el filesystem scope activo previamente existente en `Shell`.

Representación conceptual:

```text
Shell
└── owns FilesystemScope
```

Regla observable:

```text
&mut Shell
        ↓
scope-fs "<path>"
        ↓
SetFilesystemScope
        ↓
nuevo FilesystemScope
        ↓
Shell reemplaza scope anterior

si SetFilesystemScope devuelve éxito:
    el nuevo FilesystemScope reemplaza al anterior

si SetFilesystemScope devuelve error:
    el FilesystemScope anterior permanece intacto
```

Una Evo Shell operativa nunca queda sin filesystem scope activo por un fallo de `scope-fs`.

El engine no mantiene este estado interactivo.

La propiedad del `FilesystemScope` pertenece a `Shell`.

El `FilesystemScope` recibido desde Evo Shell Engine ya debe representar una ubicación filesystem resuelta.

Evo Shell no corrige, normaliza ni canonicaliza ese path para hacerlo presentable.

Este caso de uso no fija todavía la firma Rust definitiva de `execute`.

Consecuencia conceptual:

```text
execute(
    shell,
    command
)
```

`scope-fs` necesita poder modificar `Shell`.

## Inmutabilidad y ownership

1. La entrada textual puede prestarse durante tokenizer/parser/executor.
2. Los tokens deben prestarse del texto cuando sea posible.
3. `Command` puede ser efímero y prestar sus argumentos mientras la entrada siga viva.
4. Solo se toma ownership cuando el dato necesita sobrevivir independientemente.
5. La mutabilidad del tokenizer debe quedar localizada en el cursor incremental.
6. El `FilesystemScope` válido recibido del engine sí debe sobrevivir entre comandos y por tanto `Shell` conserva su ownership.

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

Evo Shell tampoco resuelve componentes de navegación como `..`.

La invariante de ubicación resuelta pertenece al engine.

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

## Presentación

El prompt debe usar una representación compacta del scope activo:

```text
scope-fs …/<último-segmento> >
```

Ejemplo:

```text
FilesystemScope:
/home/user/repos/evolution/evo-shell/src

Prompt:
scope-fs …/src >
```

`…/` es únicamente presentación, no forma parte del path ni de la sintaxis de usuario.

El prompt no modifica ni corrige el `FilesystemScope`.

Una futura capacidad equivalente a `pwd` podría mostrar la ubicación completa, pero UC-001 no la define.

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
