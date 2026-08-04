# US-002 — Iterar los elementos del scope activo

## Historia de usuario

Como usuario de Evo Shell,
quiero iterar los elementos contenidos directamente en el `scope` activo,
para conocer los recursos disponibles dentro de ese ámbito.

## Descripción

La operación `iter` trabaja sobre el `scope` activo de Evo Shell Engine.

Para un scope de sistema de archivos, `iter` debe obtener únicamente los elementos contenidos directamente en la ubicación seleccionada.

Ejemplo conceptual:

```text
scope fs "/home/user/documents"
iter
```

Si /home/user/documents contiene:

```text
report.txt
images/
notes.md
```

iter debe devolver esos elementos.

No debe recorrer el contenido de images/ ni de otros subdirectorios.

El resultado conceptual de iter no debe limitarse a texto plano.

Cada elemento debe representar datos estructurados del recurso encontrado.

Para un filesystem scope, cada elemento debe contener como mínimo:

```text
nombre
ruta
tipo de recurso
```

El formato visual mostrado en terminal es responsabilidad de Evo Shell y puede presentarse de manera legible para el usuario.

La representación visual no debe confundirse con el resultado estructurado de la operación.

## Criterios de aceptación

1. Debe existir un scope activo antes de ejecutar iter.
2. iter utiliza el scope activo como ámbito de trabajo.
3. En un filesystem scope, iter obtiene únicamente los elementos directamente contenidos en la ruta activa.
4. Los directorios encontrados se devuelven como elementos, pero su contenido no se recorre.
5. Cada elemento devuelto contiene información estructurada suficiente para identificar:

```text
nombre
ruta
tipo de recurso
```

6. El resultado estructurado de iter es independiente de cómo Evo Shell lo muestre visualmente.
7. Si el scope no puede iterarse, Evo Shell debe informar que la operación no pudo resolverse.
8. La ejecución de iter no debe modificar el scope activo.

## Ejemplo

Scope activo:

```text
fs "/home/user/documents"
```

Contenido:

```text
report.txt
images/
notes.md
```

Entrada:

```text
iter
```

Resultado conceptual estructurado:

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
  },
  {
    name: "notes.md",
    path: "/home/user/documents/notes.md",
    kind: file
  }
]
```

Representación visual posible en Evo Shell:

```text
report.txt
images/
notes.md
```

## Fuera de alcance

Esta historia no define todavía:

- iteración recursiva;
- filtros;
- ordenamiento;
- pipes;
- variables;
- transformación de resultados;
- metadata adicional como tamaño, fechas, permisos, propietario o extensión;
- scopes de base de datos, URL o Web API;
- implementación mediante Vec, Iterator, streaming, arena u otra estructura técnica;
- detalles técnicos de implementación.
