# UC-003 — Entrar en una ubicación del scope activo mediante el comando enter

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta y ejecuta el comando:

```text
enter <location>
```

Evo Shell recibe texto, lo tokeniza incrementalmente, interpreta los tokens, resuelve `Command::Enter(location)`, ejecuta el comando sobre la `Shell` actual, consume la frontera pública `Enter` de Evo Shell Engine, recibe un nuevo `FilesystemScope` resuelto o un error, reemplaza el scope activo únicamente si la operación tuvo éxito y actualiza la presentación del prompt.

Evo Shell no duplica la lógica de resolución de paths ni la validación de filesystem de Evo Shell Engine.

## Actor

- Usuario de Evo Shell

## Entrada

Entrada textual:

```text
enter agents
```

También debe aceptarse una ubicación entre comillas cuando contiene espacios:

```text
enter "Mis Documentos"
```

Gramática establecida para este caso:

- `enter` es una única instrucción.
- el espacio separa la instrucción de su argumento.
- la ubicación puede recibirse como `Token::Word(location)` o `Token::String(location)`.
- cualquier token adicional representa un error sintáctico.

## Precondiciones

- Evo Shell recibe una línea de entrada textual.
- Existe una instancia operativa de `Shell`.
- `Shell` posee un filesystem scope activo válido.
- Evo Shell puede consumir la frontera pública `Enter` de Evo Shell Engine.

No se requieren precondiciones técnicas adicionales.

## Flujo principal

1. El usuario introduce `enter <location>`.
2. Evo Shell recibe el texto.
3. `tokenizer::tokenize` coordina la tokenización incremental.
4. `token::resolve` resuelve `Token::Word("enter")`.
5. `token::resolve` resuelve la ubicación como `Token::Word(location)` o `Token::String(location)`.
6. `parser::parse` coordina la interpretación de los tokens.
7. `command::resolve` resuelve los tokens como `Command::Enter(location)`.
8. `executor::execute` coordina la ejecución del comando resuelto.
9. `execution::resolve` resuelve que `Command::Enter(location)` debe operar sobre `Shell`.
10. Evo Shell presta el `FilesystemScope` owned by `Shell`.
11. Evo Shell consume la frontera pública `Enter` de Evo Shell Engine.
12. Evo Shell Engine devuelve un nuevo `FilesystemScope` válido y resuelto.
13. `Shell` reemplaza el scope anterior por el nuevo `FilesystemScope`.
14. Evo Shell devuelve `ExecutionResult::ScopeChanged`.
15. Evo Shell actualiza el prompt a partir del nuevo scope activo.

## Tokenización y parsing

UC-003 reutiliza la infraestructura existente:

```text
TokenStream
    ↓
tokenizer::tokenize
    ↓
parser::parse
    ↓
command::resolve
```

No se crea un tokenizer nuevo.

No se crea un parser nuevo.

`command::resolve` debe aceptar conceptualmente:

```text
Token::Word("enter")
Token::Word("agents")
    ↓
Command::Enter("agents")
```

y:

```text
Token::Word("enter")
Token::String("Mis Documentos")
    ↓
Command::Enter("Mis Documentos")
```

Debe rechazar:

```text
enter
```

```text
enter agents extra
```

```text
enter "Mis Documentos" extra
```

La resolución se mantiene incremental y no materializa `Vec<Token>`.

## Command

`Command` sigue siendo la entidad existente que representa una instrucción ya interpretada.

UC-003 agrega conceptualmente una nueva variante:

```text
Command
├── ScopeFs(&str)
├── Iter
└── Enter(&str)
```

La ubicación de `Command::Enter(location)` debe poder prestar texto de la entrada original cuando sea viable.

No se crea una entidad separada `RelativeLocation` en este caso de uso.

## Executor

UC-003 reutiliza el agent existente:

```text
executor::execute(&mut Shell, Command)
```

El agent coordina la ejecución y delega en:

```text
execution::resolve
```

No se crea un executor específico para `enter`.

## Execution resolver

`execution::resolve` debe incorporar conceptualmente una rama exhaustiva para:

```text
Command::Enter(location)
```

Flujo:

```text
Command::Enter(location)
        ↓
execution::resolve
        ↓
borrow Shell.filesystem_scope()
        ↓
Enter(&FilesystemScope, location)
        ↓
Evo Shell Engine
        ↓
new FilesystemScope / error
```

En éxito:

```text
new FilesystemScope
        ↓
Shell reemplaza el scope anterior
        ↓
ExecutionResult::ScopeChanged
```

En error:

- `Shell` no reemplaza el scope;
- el error se propaga o presenta como error de ejecución;
- el `FilesystemScope` anterior permanece intacto.

## Frontera con Evo Shell Engine

Evo Shell consume únicamente la frontera pública documentada del engine:

```text
Enter(&FilesystemScope, location)
```

Evo Shell también conoce el agent público correspondiente:

```text
enterer::enter
```

Conceptualmente:

```text
Enter(
    &current FilesystemScope,
    relative location
)
    ↓
new FilesystemScope / error
```

Evo Shell no conoce ni duplica:

- `filesystem_path::resolve`;
- el join interno de paths;
- `scope_setter::set` como mecanismo interno de `Enter`;
- `filesystem_scope::resolve`;
- providers internos del engine;
- contracts internos del engine;
- `std::fs` interno del engine.

La lógica de combinar:

```text
scope actual + location relativa
```

pertenece a Evo Shell Engine.

La resolución final del `FilesystemScope` también pertenece a Evo Shell Engine.

Evo Shell no corrige paths, no resuelve `..`, no canonicaliza filesystem y no intenta ocultar rutas sin resolver en presentación.

## Shell y estado

`Shell` sigue siendo propietaria del filesystem scope activo.

Flujo conceptual:

```text
Shell
└── owns FilesystemScope A
        ↓ borrow
      Enter(location)
        ↓
FilesystemScope B
        ↓ success
Shell reemplaza A por B
```

`FilesystemScope B` debe representar la ubicación filesystem resuelta producida por Evo Shell Engine.

En error:

```text
Shell
└── sigue poseyendo FilesystemScope A
```

No se introduce `Option<FilesystemScope>`, `Session`, `ShellState` ni `ActiveScope`.

## ExecutionResult

UC-003 reutiliza:

```text
ExecutionResult::ScopeChanged
```

No se crea una variante nueva exclusivamente para `enter`.

Desde la perspectiva de ejecución, tanto `scope-fs` como `enter` pueden terminar exitosamente con un nuevo scope activo.

La intención funcional de ambos comandos sigue siendo diferente.

La presentación interactiva no necesita producir una línea adicional para `ExecutionResult::ScopeChanged`.

El prompt compacto representa el nuevo estado activo.

Ejemplo conceptual:

```text
scope-fs …/evo-shell >
enter src
scope-fs …/src >
```

## Errores

### Error sintáctico

Un error sintáctico pertenece a Evo Shell.

Ejemplos conceptuales:

```text
enter
```

```text
enter agents extra
```

Flujo:

1. Evo Shell recibe la entrada.
2. La entrada no cumple la sintaxis requerida por `enter`.
3. Evo Shell no llama a `Enter` del engine.
4. Evo Shell presenta un error conceptual al usuario.
5. El `FilesystemScope` owned by `Shell` permanece intacto.

### Error del engine

Un error operativo del engine ocurre cuando Evo Shell interpreta correctamente `enter <location>`, pero Evo Shell Engine no puede resolver la nueva ubicación.

Ejemplos conceptuales:

- ubicación inexistente;
- ubicación no válida como filesystem scope;
- cualquier error público de `Enter`.

Flujo:

1. Evo Shell interpreta correctamente `enter <location>`.
2. `execution::resolve` presta el scope actual.
3. `execution::resolve` consume `Enter`.
4. Evo Shell Engine devuelve un error.
5. Evo Shell presenta o propaga el error correspondiente.
6. El `FilesystemScope` anterior permanece intacto.

## `enter ..`

Desde Evo Shell, `enter ..` no tiene lógica especial.

Debe llegar conceptualmente como:

```text
Command::Enter("..")
```

Después:

```text
execution::resolve
    ↓
Enter(&current_scope, "..")
```

La semántica de resolver `..` pertenece al engine.

El resultado que vuelve del engine debe ser un `FilesystemScope` resuelto.

Ejemplo:

```text
scope actual:
/home/user/repos/evolution/evo-shell/src

enter ..

candidate en engine:
/home/user/repos/evolution/evo-shell/src/..

FilesystemScope resultante:
/home/user/repos/evolution/evo-shell
```

Lo mismo aplica para:

```text
enter ../..
enter src/agents
```

Evo Shell no debe introducir lógica equivalente a:

```text
match location {
    ".." => ...
}
```

## Borrowing y ownership

Relación conceptual:

```text
input String
    ↓ borrow
TokenStream
    ↓
Token
    ↓
Command::Enter(&str)

Shell
    owns FilesystemScope

Enter:
    borrows &FilesystemScope
    borrows location when viable
    returns owned FilesystemScope
```

Evo Shell no clona `FilesystemScope` para ejecutar `Enter`.

El nuevo `FilesystemScope` solo reemplaza al anterior cuando `Enter` devuelve éxito.

## Presentación

El prompt debe mostrar una representación compacta del scope activo:

```text
scope-fs …/<último-segmento> >
```

Ejemplos:

```text
FilesystemScope:
/home/user/repos/evolution/evo-shell/src

Prompt:
scope-fs …/src >
```

```text
FilesystemScope:
/home/user/repos/evolution/evo-shell/src/agents

Prompt:
scope-fs …/agents >
```

`…/` indica que existen componentes anteriores no mostrados.

`…/` no forma parte del `FilesystemScope`, no es sintaxis de usuario y no modifica el path.

El prompt consume el estado correcto del dominio; no corrige paths ni resuelve componentes como `..`.

Un `enter` exitoso no debe imprimir adicionalmente:

```text
Scope activo: /home/...
```

`ExecutionResult::ScopeChanged` sigue existiendo conceptualmente, pero la capa de presentación no necesita producir una línea adicional para ese resultado.

Una futura capacidad equivalente a `pwd` podría mostrar la ubicación completa, pero UC-003 no define ese comando.

## Relación con US-003

[US-003 — Entrar en una ubicación del scope activo mediante el comando enter](../../../functional_documentation/user_stories/US-003-enter-active-scope-location.md)

Este caso de uso implementa técnicamente el comportamiento funcional de:

```text
enter <location>
```

## Relación con UC-003 de Evo Shell Engine

[UC-003 — Entrar en una ubicación de un filesystem scope](../../../../evo-shell-engine/technical_documentation/use_cases/UC-003-enter-filesystem-scope/README.md)

Frontera conceptual:

```text
Evo Shell:
interpreta enter

Evo Shell Engine:
resuelve cómo entrar desde un FilesystemScope a una ubicación relativa
```

Evo Shell consume `Enter`.

Evo Shell Engine resuelve la nueva ubicación y devuelve un `FilesystemScope` o error.

## Fuera de alcance

- database scopes
- URL scopes
- Web API scopes
- expansión de `~`
- variables
- globbing
- history
- comando `back`
- aliases
- pipes
- filtros
- autocomplete
- renderer definitivo
- implementación Rust
- nuevo shell loop
- nueva infraestructura de input/output

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
