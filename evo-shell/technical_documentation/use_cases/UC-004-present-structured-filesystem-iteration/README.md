# UC-004 — Presentar una iteración filesystem estructurada

## Objetivo

Este caso de uso documenta cómo Evo Shell debe presentar la iteración filesystem enriquecida requerida por:

[US-004 — Mostrar información estructurada de los elementos mediante `iter`](../../../functional_documentation/user_stories/US-004-show-structured-filesystem-iteration.md)

También incorpora la ampliación de tabla requerida por:

[US-006 — Mostrar fecha de creación de elementos en `iter`](../../../functional_documentation/user_stories/US-006-show-filesystem-created-time.md)

US-004 no cambia la sintaxis del comando:

```text
iter
```

La evolución técnica está en la presentación de los datos producidos por Evo Shell Engine.

Frontera de responsabilidades:

```text
Evo Shell Engine
    produce datos estructurados de filesystem

Evo Shell
    presenta esos datos en terminal
```

Evo Shell no debe obtener metadata directamente del filesystem para esta salida.

## Actor

- Usuario de Evo Shell

## Entrada

Este caso de uso parte del resultado de ejecución existente:

```text
ExecutionResult::FilesystemIteration(iteration)
```

La `FilesystemIteration` fue producida previamente por el flujo técnico de UC-002 de Evo Shell:

```text
iter
    ↓
Command::Iter
    ↓
executor::execute
    ↓
Iter(&FilesystemScope)
    ↓
FilesystemIteration
```

UC-004 documenta cómo se presenta esa iteración.

## Agent

Responsabilidad nueva documentada:

```text
iteration_presenter::present
```

Ubicación conceptual:

```text
evo-shell/src/agents/iteration_presenter.rs
```

Esta documentación no crea todavía el archivo Rust.

`iteration_presenter` es el sujeto agente responsable de presentar una iteración filesystem en terminal.

Acción:

```text
present
```

## Responsabilidad del presenter

`iteration_presenter::present` debe coordinar:

- encabezado;
- consumo progresivo mediante `Advance`;
- render de cada fila;
- formato de `Created`;
- formato de `Modified`;
- formato de `Size`;
- texto visible de `Type`;
- `Name`;
- colores;
- conteo final de files/directories;
- resumen.

No debe:

- obtener metadata del filesystem;
- inventar index;
- recorrer directorios recursivamente;
- ordenar resultados;
- almacenar toda la iteración;
- modificar `Shell`;
- modificar `FilesystemScope`.

## Flujo principal

Conceptualmente:

```text
FilesystemIteration
    ↓
iteration_presenter::present
    ↓
render header

loop:
    iteration_advancer::advance
        ↓
    FilesystemIterationItem
        ├── index
        └── FilesystemEntry
        ↓
    render row
        ↓
    update counters

Advance -> None
    ↓
render summary
```

El presenter consume un item, lo presenta y puede descartarlo antes de solicitar el siguiente.

## Columnas

Formato conceptual:

```text
#   Created              Modified             Type   Size      Name
```

Mapeo:

```text
#        ← FilesystemIterationItem.index

Created  ← FilesystemEntry.created
           convertido en presentación
           o celda vacía si None

Modified ← FilesystemEntry.modified
           convertido en presentación

Type     ← FilesystemEntry.kind
           presentado como:
           file
           dir
           symlink
           other

Size     ← FilesystemEntry.size
           convertido de bytes a:
           B / kB / MB ...
           o vacío si None

Name     ← FilesystemEntry.name
```

`Name` usa el nombre visible del elemento.

No debe usar el path completo como `Name`.

## Formato de fecha

Evo Shell Engine no devuelve una fecha formateada.

`iteration_presenter::present` transforma los datos temporales estructurados de `FilesystemEntry.created` y `FilesystemEntry.modified` a presentación.

Formato funcional deseado:

```text
DD/MM/YYYY HH:MM
```

Ejemplo:

```text
05/08/2026 00:42
```

Esta documentación no define todavía timezone configurable.

Si el formato local exacto requiere una dependencia externa y el proyecto decide no agregarla todavía, la implementación debe dejar esa decisión explícita en el momento de implementarse.

No se agrega un crate de fechas en esta tarea documental.

La implementación actual de Evo Shell ya utiliza `time` para presentar fechas locales legibles. Esa dependencia debe reutilizarse para `Created`; no se introduce `chrono`, `jiff` ni otro crate de fecha.

La lógica base de formateo de `SystemTime` debe compartirse entre `Created` y `Modified` mediante una función mínima de presentación.

No se crea:

- `DateFormatter`;
- trait de formatter;
- abstracción genérica de fechas.

Si `FilesystemEntry.created` es `None`, la celda `Created` queda vacía.

No se usa `Modified` como fallback para `Created`.

## Formato de size

`iteration_presenter::present` convierte bytes a una presentación legible.

Ejemplos conceptuales:

```text
151 B
1.2 kB
52.7 kB
2.4 MB
```

El engine conserva bytes estructurados.

La conversión textual pertenece a Evo Shell.

Esta documentación no fija todavía el algoritmo exacto de redondeo.

Reglas:

- no sumar tamaños globales;
- no calcular tamaño de directorios;
- no tratar el tamaño de directorio como recursividad;
- mostrar la celda vacía cuando `FilesystemEntry.size` sea `None`.

## Type visible

`FilesystemEntry.kind` debe presentarse con valores visibles simples:

```text
File      → file
Directory → dir
Symlink   → symlink
Other     → other
```

Evo Shell no debe mostrar nombres técnicos internos como texto visible si esos nombres no son la forma aprobada para usuario.

## Summary

`iteration_presenter::present` mantiene únicamente contadores locales de presentación:

```text
files
directories
```

Mientras consume items:

```text
files = 3
directories = 4
```

Al finalizar:

```text
4 directories
3 files
```

Los contadores no se devuelven al engine.

Los contadores no se agregan a `FilesystemIteration`.

No se calcula:

```text
total_size
```

No se muestra una suma total como:

```text
5.1 kB
```

## Symlink y Other

Las filas de `symlink` y `other` se muestran normalmente.

El resumen inicial solo cuenta:

- files;
- directories.

No se agregan todavía:

- `N symlinks`;
- `N others`.

## Colores

Los colores pertenecen exclusivamente a Evo Shell y a la presentación.

No se agregan colores a:

- `FilesystemEntry`;
- `FilesystemIterationItem`;
- `FilesystemEntryKind`.

El presenter puede usar secuencias ANSI/VT simples para distinguir visualmente:

- headers;
- directories;
- files;
- symlinks;
- other.

Los colores son presentación, no datos del filesystem.

Esta documentación no fija una paleta definitiva ni agrega crates externos de terminal.

Una paleta mínima futura debe mantenerse legible y separada del dato estructurado.

`Created` no introduce un color propio.

`Created` y `Modified` pueden permanecer con presentación neutra/default.

## Main y shell loop

El código actual de presentación vive conceptualmente en `main.rs` mediante responsabilidades como:

```text
render_iteration
render_entry
```

Cuando `iteration_presenter::present` asuma la presentación estructurada, `main.rs` debe dejar de contener la lógica detallada de render de iteración.

El shell loop debe limitarse conceptualmente a:

```text
ExecutionResult::FilesystemIteration(iteration)
    ↓
iteration_presenter::present(iteration)
```

No se crea otro crate de presentación.

Razón:

solo Evo Shell consume actualmente esta presentación.

No se extrae todavía a:

- `evo-shell-presentation`;
- `evo-terminal`;
- otro crate genérico.

## Comportamiento lazy

UC-004 debe conservar el comportamiento lazy de `iter`.

Flujo:

```text
Advance
    ↓
FilesystemIterationItem
    ↓
render row
    ↓
drop item
    ↓
Advance
```

Evo Shell no debe acumular todos los items antes de presentar la tabla.

No se documenta:

- `Vec<FilesystemIterationItem>`;
- `collect()`;
- precarga de la iteración completa.

El presenter puede mantener estado temporal mínimo de presentación:

- contador de files;
- contador de directories.

## Errores

UC-004 conserva la semántica de errores operativos existente de `IterError`.

Si `Advance` devuelve error:

1. el presenter detiene el consumo de la iteración;
2. Evo Shell presenta el error conceptual al usuario;
3. el `FilesystemScope` activo permanece intacto.

Si la metadata de un item falla y el engine lo reporta como `IterError`, Evo Shell no debe ocultarlo silenciosamente.

Esta documentación no introduce una política de omitir entradas rotas.

## Relación con Evo Shell Engine

Evo Shell consume la frontera pública del engine:

```text
Advance(&mut FilesystemIteration)
    ↓
Result<Option<FilesystemIterationItem>, IterError>
```

Cada `FilesystemIterationItem` contiene:

```text
FilesystemIterationItem
├── index
└── entry: FilesystemEntry
```

Cada `FilesystemEntry` contiene conceptualmente:

```text
FilesystemEntry
├── name
├── path
├── kind
├── created
├── modified
└── size
```

Evo Shell usa esos datos para presentación.

Evo Shell no recalcula metadata ni genera index.

## Futuro

No se generaliza todavía `iteration_presenter` a DB scopes, URL scopes o Web API scopes.

No se crea:

- `IterationPresenter<T>`;
- `Table<T>`;
- `Row` genérico.

Si futuros scopes producen resultados tabulares compatibles, esta abstracción podrá revisarse.

US-004 es exclusivamente filesystem.

## Fuera de alcance

- operador `index 0`
- pipelines
- filter
- select
- first
- to-value
- to-text
- Evo Script
- sorting
- pagination
- recursion
- recursive directory size
- total byte summary
- inode
- Windows file ID
- permissions
- owner/group
- DB scopes
- URL scopes
- generic table engine
- generic `IterationItem<T>`
- new presentation crate
- implementación Rust

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
