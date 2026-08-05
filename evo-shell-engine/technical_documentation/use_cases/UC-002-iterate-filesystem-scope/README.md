# UC-002 — Iterar los elementos de un filesystem scope

## Objetivo

Este caso de uso obtiene los elementos directamente contenidos en un `FilesystemScope` válido.

La operación no es recursiva.

El resultado conceptual es estructurado y no debe confundirse con la representación visual de terminal.

US-004 de Evo Shell requiere que la iteración exponga datos suficientes para presentar una salida estructurada. Esa evolución pertenece a este mismo caso de uso porque no cambia la intención funcional de iterar un filesystem scope; amplía el dato producido por cada avance.

US-005 de Evo Shell requiere que la iteración conserve el contexto completo del filesystem scope que la originó. Esa evolución también pertenece a este caso de uso porque el path es estado del scope iterado, no un nuevo comportamiento operativo.

US-006 de Evo Shell requiere que cada entrada exponga la fecha de creación cuando el filesystem la proporciona. Esa evolución pertenece a `FilesystemEntry` porque `created` es metadata propia del elemento del filesystem, no de la presentación ni de la iteración.

## Actor

- Usuario de Evo Shell

## Entrada

El caso de uso recibe conceptualmente:

- la operación `iter`
- un `FilesystemScope` previamente resuelto

UC-002 no vuelve a resolver si el scope es válido como directorio. Esa garantía proviene de UC-001.

Frontera pública del engine para iniciar la iteración:

```text
Iter(&FilesystemScope)
```

Frontera pública del engine para avanzar la iteración:

```text
Advance(&mut FilesystemIteration)
```

El consumidor externo no proporciona `ReadDirectory`, `NextDirectoryEntry` ni providers internos.

## Precondiciones

- Existe un `FilesystemScope` válido.
- El scope fue establecido previamente mediante UC-001 o una capacidad equivalente.

## Flujo principal

1. El usuario ejecuta `iter`.
2. Evo Shell Engine utiliza el `FilesystemScope` activo.
3. Evo Shell Engine solicita iterar los elementos directamente contenidos en ese scope.
4. El sistema intenta obtener los elementos disponibles en la ruta.
5. `Iter` devuelve una `FilesystemIteration` lazy que conserva el path completo del scope de origen.
6. Cada llamada a `Advance` obtiene como máximo el siguiente elemento disponible.
7. Por cada elemento producido, el engine resuelve un `FilesystemEntry` con datos estructurados de filesystem.
8. El engine envuelve esa entrada en un `FilesystemIterationItem` con su índice ordinal dentro de la iteración actual.
9. Los directorios se devuelven como elementos, pero no se recorren.
10. La presentación visual del resultado se realiza separadamente.

## FilesystemEntry

`FilesystemEntry` sigue representando una entrada real del filesystem.

Debe contener conceptualmente:

```text
FilesystemEntry
├── name
├── path
├── kind
├── created
├── modified
└── size
```

Responsabilidades de cada dato:

- `name`: nombre del elemento dentro de su directorio.
- `path`: ruta del elemento.
- `kind`: tipo de entrada filesystem.
- `created`: fecha de creación reportada por el filesystem cuando está disponible.
- `modified`: última modificación reportada por el filesystem cuando está disponible.
- `size`: tamaño en bytes cuando aplica.

`FilesystemEntryKind` conserva los valores conceptuales:

- `File`
- `Directory`
- `Symlink`
- `Other`

`index` no pertenece a `FilesystemEntry`.

Razón:

el índice no es propiedad del archivo ni del directorio. Es propiedad de la posición del elemento dentro de una iteración concreta.

## Created

`created` representa la fecha de creación reportada por el filesystem cuando está disponible.

El engine debe exponer un dato temporal estructurado.

El engine no debe devolver una fecha formateada como:

```text
05/08/2026 02:15
```

Ese formato pertenece a Evo Shell.

Conceptualmente, el dato puede apoyarse en el tipo estándar apropiado para tiempo de filesystem, como `SystemTime`.

La forma conceptual mínima correcta es optional:

```text
Option<SystemTime>
```

Razón:

la capacidad estándar de metadata puede indicar que el filesystem o la plataforma no soporta creation time para una entrada concreta.

Si la fecha de creación no está disponible, `FilesystemEntry` conserva:

```text
created: None
```

No se debe usar `modified` como fallback.

`created` y `modified` representan propiedades distintas.

## Modified

`modified` representa la última modificación reportada por el filesystem cuando está disponible.

El engine debe exponer un dato temporal estructurado.

El engine no debe devolver una fecha formateada como:

```text
05/08/2026 00:42
```

Ese formato pertenece a Evo Shell.

Conceptualmente, el dato puede apoyarse en el tipo estándar apropiado para tiempo de filesystem, como `SystemTime`, con optionalidad si la plataforma o el filesystem no proporcionan el dato.

Diferencia de responsabilidades:

```text
Evo Shell Engine
    dato temporal estructurado

Evo Shell
    DD/MM/YYYY HH:MM
```

Esta documentación no introduce un tipo artificial nuevo para fechas.

## Size

`size` representa el tamaño del archivo en bytes cuando ese dato aplica.

El engine debe conservar el valor estructurado en bytes.

El engine no debe devolver texto formateado como:

```text
52.7 kB
```

Ese formato pertenece a Evo Shell.

La forma conceptual mínima correcta es:

```text
Option<u64>
```

Razón:

`0` no debe significar "no aplica", porque un archivo válido puede medir `0` bytes.

Semántica inicial:

- `File`: `Some(bytes)`
- `Directory`: `None`
- `Symlink`: semántica mínima coherente con la metadata disponible y sin recursividad
- `Other`: semántica mínima coherente con la metadata disponible y sin inventar significado visual

Para directorios no se calcula tamaño recursivo.

UC-002 no suma tamaños globales.

## Metadata

La obtención de `created`, `modified` y `size` pertenece a Evo Shell Engine porque son datos del filesystem.

No debe hacerse en:

- Evo Shell
- `iteration_presenter`
- `main.rs`

La arquitectura actual ya materializa cada `FilesystemEntry` dentro de `filesystem_entry::resolve` a partir del siguiente elemento de infraestructura producido por `NextDirectoryEntry`.

La ampliación mínima debe mantener esa frontera:

```text
filesystem_entry::resolve
    ↓
DirEntry
    ↓
file type + metadata necesaria
    ↓
FilesystemEntry
```

Si la metadata puede obtenerse correctamente desde el elemento de infraestructura ya disponible durante la resolución de la entrada, no hace falta introducir un provider adicional.

Solo debe documentarse una capacidad externa adicional si la implementación no puede obtener la metadata respetando la arquitectura actual.

No se debe duplicar trabajo de filesystem sin necesidad.

Si falla la obtención de metadata necesaria para materializar el item, el error debe encajar con la semántica existente de `IterError`: un fallo operativo al materializar la entrada no se oculta silenciosamente.

Esta documentación no introduce una política de omitir entradas rotas.

La ausencia específica de creation time no invalida la entrada.

Política:

```text
metadata()
    → OK

metadata.created()
    → OK(SystemTime)
    → FilesystemEntry.created = Some(SystemTime)

metadata.created()
    → Err(no soportado / no disponible)
    → FilesystemEntry.created = None
```

Solo falla la iteración si falla la obtención general de metadata necesaria para materializar la entrada.

## FilesystemIterationItem

`FilesystemIterationItem` representa un elemento producido por una iteración de filesystem junto con su posición ordinal.

Modelo conceptual:

```text
FilesystemIterationItem
├── index
└── entry: FilesystemEntry
```

Conceptualmente:

```text
index: usize
entry: FilesystemEntry
```

No se debe agregar `index` a `FilesystemEntry`.

No se debe usar inode ni file ID como índice.

No se crea todavía una abstracción genérica `IterationItem<T>`.

Motivo:

hoy solo existe una necesidad concreta para filesystem. Si futuros scopes demuestran la misma abstracción, podrá emerger más adelante.

## Index

Reglas:

- empieza en `0`;
- se incrementa una vez por cada elemento producido por la iteración fuente;
- representa el orden en que `iter` produce los elementos;
- no es identidad persistente;
- no depende de presentación;
- no se genera en Evo Shell;
- no se genera en `iteration_presenter`;
- pertenece al engine.

Ejemplo conceptual:

```text
Advance
→ FilesystemIterationItem {
     index: 0,
     entry: ...
   }

Advance
→ FilesystemIterationItem {
     index: 1,
     entry: ...
   }
```

Exponer el índice como dato estructurado permite futuras operaciones sin depender de la presentación visual.

Esta documentación no define todavía una sintaxis como `index 0`.

## FilesystemIteration

`FilesystemIteration` conserva el estado mínimo necesario para producir resultados de forma lazy.

Con US-004 también debe conservar el estado mínimo necesario para producir índices incrementales.

Con US-005 también debe conservar el contexto completo del scope que originó esa iteración.

Modelo conceptual:

```text
FilesystemIteration
├── read_dir state
├── next_index
└── path
```

`next_index` representa el ordinal que se asignará al próximo elemento producido.

`path` representa la ubicación completa del `FilesystemScope` que originó la iteración.

El path no participa en `Advance`.

El path no pertenece a `FilesystemEntry`.

El path no pertenece a `FilesystemIterationItem`.

No se materializan entradas.

No se guarda `Vec<FilesystemEntry>`.

No se guarda `Vec<FilesystemIterationItem>`.

No se guarda historial.

No se duplica el path por cada item producido.

## Contexto Path de la iteración

La fuente de verdad del path es el `FilesystemScope` válido recibido por `Iter`.

Conceptualmente:

```text
FilesystemScope.path()
       ↓
iterator::iter
       ↓
FilesystemIteration.path
```

No se reconstruye el path desde:

- `FilesystemEntry`;
- el primer elemento producido;
- el directorio actual del proceso;
- el prompt;
- strings de presentación.

`FilesystemIteration` debe exponer conceptualmente:

```text
path() -> &Path
```

No expone:

- `String`;
- `OsString`;
- path abreviado;
- texto con prefijo `Path:`;
- una representación como `…/target`.

El engine conserva el path estructurado completo.

La presentación textual pertenece a Evo Shell.

## Ownership del Path

`FilesystemIteration` debe poder sobrevivir independientemente después de que `Iter` termina.

Si la arquitectura Rust actual no tiene un lifetime compartido apropiado, la solución mínima documentada es que la iteración conserve su propio `PathBuf` derivado del path del scope.

Esta copia ocurre una sola vez al crear la iteración.

No se introducen lifetimes complejos únicamente para evitar clonar un `PathBuf`.

No se copia el path por cada `FilesystemIterationItem`.

## Directorio vacío

El path existe aunque la iteración no produzca elementos.

Ejemplo conceptual:

```text
FilesystemIteration
├── path: /home/user/empty
├── read_dir state
└── next_index: 0

Advance
→ None
```

Esto permite que un consumidor presente el contexto de una iteración vacía sin obtenerlo desde un primer item.

Flujo conceptual de `Advance`:

1. obtiene siguiente entrada de la iteración fuente;
2. resuelve `FilesystemEntry`;
3. produce `FilesystemIterationItem(index, entry)`;
4. incrementa el índice;
5. conserva el estado para el siguiente `Advance`.

## Advance

`Advance` permite a un consumidor externo avanzar una `FilesystemIteration` existente y obtener como máximo un item por llamada.

Antes de US-004, la frontera conceptual era:

```text
&mut FilesystemIteration
→ Result<Option<FilesystemEntry>, IterError>
```

La evolución requerida es:

```text
&mut FilesystemIteration
→ Result<Option<FilesystemIterationItem>, IterError>
```

Significado conceptual:

- `Some(FilesystemIterationItem)`: siguiente elemento disponible con índice ordinal;
- `None`: fin de iteración;
- `Err(IterError)`: error operativo.

`iteration_advancer::advance` sigue siendo el agent existente.

No se crea un agent nuevo para index.

El avance conoce naturalmente la posición actual porque recibe `&mut FilesystemIteration`.

US-005 no modifica la frontera conceptual de `Advance`.

`Advance` no recibe, calcula ni devuelve el path de la iteración.

## Frontera pública e internas

Los use cases son contratos de entrada públicos del engine.

Los contracts y providers son dependencias internas de salida del engine.

La iteración de filesystem expone conceptualmente dos capacidades públicas.

### Inicio de iteración

`Iter` inicia una iteración lazy a partir de un `FilesystemScope` válido.

```text
Iter(&FilesystemScope)
    ↓
FilesystemIteration
```

La iteración resultante conserva:

```text
FilesystemIteration
├── estado de read_dir
├── next_index = 0
└── path = scope.path()
```

Relación interna:

```text
evo-shell
    ↓
Iter(&FilesystemScope)
    ↓
evo-shell-engine
    ↓
iterator::iter
    ↓
filesystem_iteration::resolve
    ↓
ReadDirectory
    ↓
providers::read_directory::provide
    ↓
std::fs::read_dir
    ↓
FilesystemIteration con path del scope
```

### Avance de iteración

```text
evo-shell
    ↓
Advance(&mut FilesystemIteration)
    ↓
evo-shell-engine
    ↓
iteration_advancer::advance
    ↓
filesystem_entry::resolve
    ↓
NextDirectoryEntry
    ↓
providers::next_directory_entry::provide
    ↓
ReadDir::next
    ↓
FilesystemIterationItem
```

`Advance` no representa un nuevo comportamiento funcional independiente del usuario. Es la capacidad técnica pública necesaria para consumir de forma lazy la iteración iniciada por `Iter`.

`ReadDirectory`, `NextDirectoryEntry`, los providers, los resolvers, `ReadDir`, `DirEntry` y `std::fs` permanecen encapsulados dentro de Evo Shell Engine.

Leer `FilesystemScope::path()` o `FilesystemIteration::path()` es lectura de estado de entidad.

No requiere un nuevo use case, agent, resolver, provider ni contract.

## Lazy y memoria

La iteración conserva evaluación lazy y avanza elemento por elemento mediante `FilesystemIteration`.

Por cada avance exitoso se produce:

```text
FilesystemIterationItem
├── index
└── FilesystemEntry
```

Cada llamada a `Advance` avanza exactamente un elemento.

El consumidor puede procesar o presentar ese item y descartarlo antes de pedir el siguiente.

No se usa:

- `Vec<FilesystemEntry>`
- `Vec<FilesystemIterationItem>`
- `collect()`
- precarga del directorio completo

El costo adicional del índice debe ser únicamente el contador necesario dentro del estado de iteración.

El costo adicional del contexto `path` debe ser una copia única al crear `FilesystemIteration`, cuando sea necesaria por ownership.

La mutabilidad de `&mut FilesystemIteration` existe porque avanzar la iteración modifica el cursor interno y el ordinal siguiente.

No representa modificación del filesystem, modificación del `FilesystemScope`, estado global ni acumulación.

## Flujo alternativo — scope no iterable en ese momento

Aunque el `FilesystemScope` fue válido cuando se creó, posteriormente pueden existir condiciones externas que impidan leerlo.

Por ejemplo, conceptualmente:

- ubicación eliminada
- permisos cambiados
- filesystem no disponible
- error de entrada/salida

Estos ejemplos no forman una lista técnica cerrada.

Flujo:

1. Evo Shell Engine intenta obtener los elementos del scope.
2. La operación no puede resolverse.
3. Evo Shell Engine informa que `iter` no pudo completarse.
4. El scope activo no se modifica.

## Resultado exitoso

Ejemplo conceptual estructurado de avances sucesivos:

```text
FilesystemIteration {
  path: "/home/user/documents",
  read_dir state,
  next_index: 0
}

Some(FilesystemIterationItem {
  index: 0,
  entry: FilesystemEntry {
    name: "report.txt",
    path: "/home/user/documents/report.txt",
    kind: File,
    created: <dato temporal estructurado opcional>,
    modified: <dato temporal estructurado>,
    size: Some(52700)
  }
})

Some(FilesystemIterationItem {
  index: 1,
  entry: FilesystemEntry {
    name: "images",
    path: "/home/user/documents/images",
    kind: Directory,
    created: <dato temporal estructurado opcional>,
    modified: <dato temporal estructurado>,
    size: None
  }
})

None
```

## Resultado no exitoso

Evo Shell Engine informa que `iter` no pudo completarse.

Si ocurre un error operativo al avanzar o materializar una entrada, se reporta como `IterError`.

## Fuera de alcance

- iteración recursiva
- filtros
- ordenamiento
- pipes
- variables
- transformación de resultados
- comando `pwd`
- use case para obtener path
- agent para obtener path
- resolver o provider para obtener path
- formato de fecha para terminal
- formato de tamaño legible para terminal
- colores
- total acumulado de bytes
- tamaño recursivo de directorios
- permisos
- propietario o grupo
- inode
- Windows file ID
- hash o checksum
- scopes de base de datos, URL o Web API
- `IterationItem<T>` genérico
- implementación mediante Vec, collect, arena u otra materialización completa
- detalles técnicos de implementación Rust

## Relación con la historia de usuario

[US-002 — Iterar los elementos del scope activo](../../../functional_documentation/user_stories/US-002-iterate-active-scope.md)

US-004 de Evo Shell amplía la presentación esperada de `iter`, pero no crea un nuevo use case de iteración en el engine.

US-005 de Evo Shell amplía el contexto visible de `iter`, pero tampoco crea un nuevo use case de iteración en el engine.

US-006 de Evo Shell agrega una columna visible `Created`, pero tampoco crea un nuevo use case de iteración en el engine. La evolución técnica se limita a exponer `created` como metadata opcional de `FilesystemEntry`.

Una capacidad futura equivalente a `pwd` podría reutilizar la misma fuente de verdad representada por `FilesystemScope.path()`, pero esta documentación no diseña ese comando ni su arquitectura.

## Relación con UC-001

[UC-001 — Establecer un filesystem scope](../UC-001-set-filesystem-scope/README.md)

UC-001 produce un `FilesystemScope` válido.

UC-002 consume ese scope mediante préstamo y no vuelve a validar la invariancia ya establecida por UC-001.

La relación conceptual es:

```text
UC-001:
&Path
→ resuelve
→ FilesystemScope válido

UC-002:
&FilesystemScope
→ iter
```

Importante:

- UC-002 no vuelve a comprobar si el path es un directorio.
- `FilesystemScope` ya representa esa garantía establecida por UC-001.
- UC-002 recibe el `FilesystemScope` mediante préstamo.
- UC-002 no modifica el `FilesystemScope`.

## Diseño técnico

- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
