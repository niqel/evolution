# UC-001 — Establecer un scope de sistema de archivos

## Objetivo

Este caso de uso permite establecer una ubicación del sistema de archivos como scope activo de Evo Shell para que las operaciones posteriores trabajen dentro de ese ámbito.

## Actor

- Usuario de Evo Shell

## Entrada

Representa conceptualmente:

```text
scope fs "<path>"
```

`<path>` representa la ubicación solicitada por el usuario.

## Precondiciones

- El usuario de Evo Shell solicita establecer un scope de sistema de archivos.
- El usuario proporciona una ubicación.

## Flujo principal

1. El usuario solicita un scope de tipo `fs`.
2. El usuario proporciona una ubicación.
3. Evo Shell evalúa si la ubicación puede utilizarse como scope.
4. Si puede utilizarse, Evo Shell establece esa ubicación como scope activo.
5. Evo Shell informa el scope activo.
6. Las operaciones posteriores pueden utilizar ese scope.

## Flujo alternativo — ubicación no utilizable

1. Evo Shell determina que la ubicación no puede utilizarse.
2. Evo Shell informa que el scope no pudo establecerse.
3. Si ya existía un scope válido activo, este debe conservarse.

## Resultado exitoso

Ejemplo conceptual:

```text
Scope activo:
fs "/home/user/documents"
```

## Resultado no exitoso

Evo Shell informa que el scope solicitado no pudo establecerse.

## Fuera de alcance

- iteración de elementos
- operación `iter`
- filtros
- pipes
- otros scopes como `db`, `url` o `webapi`
- detalles de implementación

## Relación con la historia de usuario

[US-001 — Establecer un scope de sistema de archivos](../../../functional_documentation/user_stories/US-001-set-filesystem-scope.md)

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
