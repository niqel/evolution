# UC-003 — Entrar en una ubicación de un filesystem scope

## Objetivo

Este caso de uso permite resolver una nueva ubicación de filesystem a partir de un `FilesystemScope` actual y una ubicación relativa solicitada.

La capacidad pública se denomina `Enter`.

Conceptualmente:

```text
Enter(
    current FilesystemScope,
    relative location
)
    ↓
new FilesystemScope / error
```

`Enter` no conserva estado ni reemplaza scopes por sí mismo. Devuelve un nuevo `FilesystemScope` válido o un error.

El consumidor decide si reemplaza el scope anterior.

## Entrada

El caso de uso recibe conceptualmente:

- un `FilesystemScope` actual;
- una ubicación relativa.

Ejemplos:

```text
current scope:
/home/user/projects/evo-shell

location:
src

resultado:
/home/user/projects/evo-shell/src
```

```text
location:
..

resultado conceptual:
scope padre correspondiente
```

```text
location:
../..
```

```text
location:
src/agents
```

Frontera pública conceptual:

```text
Enter(&FilesystemScope, &Path) -> Result<FilesystemScope, EnterError>
```

La forma final del error puede reutilizar `ScopeError` si esa opción conserva mejor la frontera pública al implementarse.

## Precondiciones

- Existe un `FilesystemScope` actual válido.
- La ubicación solicitada representa una ubicación relativa dentro del contexto del filesystem scope.

No se definen todavía otros tipos de scope para `Enter`.

## Flujo principal

1. El consumidor solicita `Enter` con el `FilesystemScope` actual y una ubicación relativa.
2. `enterer::enter` coordina la acción.
3. `enterer::enter` solicita a `filesystem_path::resolve` resolver el path candidato.
4. `filesystem_path::resolve` usa la ruta del `FilesystemScope` actual y la ubicación relativa solicitada.
5. `filesystem_path::resolve` devuelve un candidate `PathBuf`.
6. `enterer::enter` entrega el candidate path a `scope_setter::set`.
7. `scope_setter::set` reutiliza el flujo existente de UC-001 para resolver si el path puede convertirse en un `FilesystemScope`.
8. Si el candidate path puede utilizarse, Evo Shell Engine devuelve un nuevo `FilesystemScope`.

Cadena conceptual:

```text
Enter
    ↓
enterer::enter
    ↓
filesystem_path::resolve
    ↓
candidate PathBuf
    ↓
scope_setter::set
    ↓
FilesystemScope
```

## Responsabilidades

`Enter` es el use case público para entrar en una ubicación relativa desde un filesystem scope actual.

`enterer::enter` coordina la acción.

`filesystem_path::resolve` resuelve el candidate path a partir del scope actual y la ubicación relativa.

`scope_setter::set` valida y materializa el nuevo `FilesystemScope` mediante la capacidad existente de UC-001.

`Enter` no duplica la resolución del filesystem.

## Relación con SetFilesystemScope

`Enter` reutiliza `SetFilesystemScope` mediante `scope_setter::set`.

Después de obtener el candidate path:

```text
filesystem_path::resolve
    ↓
candidate PathBuf
    ↓
scope_setter::set
    ↓
new FilesystemScope / ScopeError
```

No se vuelve a implementar:

- `filesystem_scope::resolve`;
- validación de directorio;
- providers existentes;
- `std::fs`;
- errores de filesystem ya manejados por scope.

## Relación con FilesystemScope

El `FilesystemScope` actual se recibe conceptualmente mediante préstamo.

`Enter` no modifica ese scope.

El resultado exitoso es un nuevo `FilesystemScope` owned.

Conceptualmente:

```text
old scope
    ↓ borrow
Enter
    ↓
new scope / error
```

En éxito, el consumidor puede reemplazar el scope anterior por el nuevo.

En error, el scope anterior permanece fuera de `Enter` y no se modifica.

## Comportamiento de `..` y rutas relativas

`..` se trata como una ubicación relativa más.

No existe una acción especial para `..` dentro del agent.

Ejemplos conceptuales:

```text
filesystem_path::resolve(scope, "..")
filesystem_path::resolve(scope, "../..")
filesystem_path::resolve(scope, "src/agents")
```

La semántica se deriva de la resolución del path.

`..` no representa historial ni un comando `back`.

## Ownership y borrowing conceptual

El scope actual se presta:

```text
&FilesystemScope
```

La ubicación solicitada también debe tratarse mediante préstamo cuando sea viable:

```text
&Path
```

El candidate path es un valor temporal owned:

```text
PathBuf
```

El resultado exitoso es un nuevo `FilesystemScope` owned.

No se almacena estado global.

## Errores

`Enter` debe devolver error cuando:

1. la ubicación relativa no puede resolverse como candidate path;
2. el candidate path no puede convertirse en `FilesystemScope`;
3. la capacidad existente de scope reporta un error.

El error se devuelve al consumidor.

El `FilesystemScope` original permanece sin modificación.

No se crea fallback ni historial de navegación.

## Frontera pública e internas

Un consumidor externo debería necesitar únicamente:

- `Enter`;
- `enterer::enter`;
- `FilesystemScope`;
- el error público correspondiente.

El consumidor externo no debe conocer:

- `filesystem_path::resolve`;
- providers internos;
- contracts internos;
- `std::fs`;
- `filesystem_scope::resolve`.

Relación conceptual:

```text
consumer
    ↓
Enter(&FilesystemScope, &Path)
    ↓
evo-shell-engine
    ↓
enterer::enter
    ↓
filesystem_path::resolve
    ↓
scope_setter::set
    ↓
FilesystemScope / error
```

## Fuera de alcance

- nuevos providers
- nuevos contracts
- historial
- comando `back`
- stack de ubicaciones
- navegación recursiva
- database scopes
- URL scopes
- Web API scopes
- canonicalización adicional fuera de lo ya soportado
- expansión de `~`
- variables
- globbing
- detalles de implementación Rust

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
