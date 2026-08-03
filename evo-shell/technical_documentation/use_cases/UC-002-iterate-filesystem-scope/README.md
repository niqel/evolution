# UC-002 — Iterar los elementos de un filesystem scope

## Objetivo

Este caso de uso obtiene los elementos directamente contenidos en un `FilesystemScope` válido.

La operación no es recursiva.

El resultado conceptual es estructurado y no debe confundirse con la representación visual de terminal.

## Actor

- Usuario de Evo Shell

## Entrada

El caso de uso recibe conceptualmente:

- la operación `iter`
- un `FilesystemScope` previamente resuelto

UC-002 no vuelve a resolver si el scope es válido como directorio. Esa garantía proviene de UC-001.

## Precondiciones

- Existe un `FilesystemScope` válido.
- El scope fue establecido previamente mediante UC-001 o una capacidad equivalente.

## Flujo principal

1. El usuario ejecuta `iter`.
2. Evo Shell utiliza el `FilesystemScope` activo.
3. Evo Shell solicita iterar los elementos directamente contenidos en ese scope.
4. El sistema intenta obtener los elementos disponibles en la ruta.
5. Por cada elemento encontrado, el resultado debe contener como mínimo:
   - nombre
   - ruta
   - tipo de recurso
6. Los directorios se devuelven como elementos, pero no se recorren.
7. Evo Shell obtiene un resultado estructurado.
8. La presentación visual del resultado se realiza separadamente.

## Flujo alternativo — scope no iterable en ese momento

Aunque el `FilesystemScope` fue válido cuando se creó, posteriormente pueden existir condiciones externas que impidan leerlo.

Por ejemplo, conceptualmente:

- ubicación eliminada
- permisos cambiados
- filesystem no disponible
- error de entrada/salida

Estos ejemplos no forman una lista técnica cerrada.

Flujo:

1. Evo Shell intenta obtener los elementos del scope.
2. La operación no puede resolverse.
3. Evo Shell informa que `iter` no pudo completarse.
4. El scope activo no se modifica.

## Resultado exitoso

Ejemplo conceptual estructurado:

```text
[
  {
    name: "report.txt",
    path: "/home/user/documents/report.txt",
    kind: file
  },
  {
    name: "images",
    path: "/home/user/documents/images",
    kind: directory
  }
]
```

## Resultado no exitoso

Evo Shell informa que `iter` no pudo completarse.

## Fuera de alcance

- iteración recursiva
- filtros
- ordenamiento
- pipes
- variables
- transformación de resultados
- metadata adicional como tamaño, fechas, permisos, propietario o extensión
- scopes de base de datos, URL o Web API
- implementación mediante Vec, Iterator, streaming, arena u otra estructura técnica
- detalles técnicos de implementación

## Relación con la historia de usuario

[US-002 — Iterar los elementos del scope activo](../../../functional_documentation/user_stories/US-002-iterate-active-scope.md)

## Relación con UC-001

[UC-001 — Establecer un filesystem scope](../UC-001-set-filesystem-scope/README.md)

UC-001 produce un `FilesystemScope` válido.

UC-002 consume ese scope mediante préstamo y no vuelve a validar la invariancia ya establecida por UC-001.

La relación conceptual es:

UC-001:
&Path
→ resuelve
→ FilesystemScope válido

UC-002:
&FilesystemScope
→ iter

Importante:

- UC-002 no vuelve a comprobar si el path es un directorio.
- `FilesystemScope` ya representa esa garantía establecida por UC-001.
- UC-002 recibe el `FilesystemScope` mediante préstamo.
- UC-002 no modifica el `FilesystemScope`.

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
