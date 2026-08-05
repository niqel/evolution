# UC-005 — Mejorar el contexto y la legibilidad visual de `iter`

## Objetivo

Este caso de uso documenta cómo Evo Shell debe presentar el contexto completo y mejorar la legibilidad visual de la salida de `iter`, según:

[US-005 — Mejorar el contexto y la legibilidad visual de `iter`](../../../functional_documentation/user_stories/US-005-improve-iter-context-and-visual-readability.md)

US-005 no cambia la sintaxis del comando:

```text
iter
```

Tampoco cambia la semántica de la iteración definida por US-002 y enriquecida por US-004.

La evolución técnica está en dos superficies de presentación:

- `iteration_presenter::present`, para la salida de `iter`;
- el render actual del prompt, para diferenciar visualmente el tipo de scope y la ubicación compacta.

No se crea un nuevo presenter, crate de presentación, renderer genérico ni agent para obtener el path.

## Frontera con Evo Shell Engine

Evo Shell Engine conserva la fuente de verdad del path en el filesystem scope.

Cuando `Iter(&FilesystemScope)` crea una `FilesystemIteration`, la iteración conserva su propio path completo del scope de origen.

Evo Shell consume ese contexto mediante:

```text
FilesystemIteration::path()
```

Conceptualmente:

```text
Evo Shell Engine
    FilesystemScope.path
        ↓
    FilesystemIteration.path()

Evo Shell
    iteration_presenter::present
        ↓
    Path: <full path>
```

El engine entrega un path estructurado.

El prefijo visible `Path:` pertenece exclusivamente a Evo Shell.

Evo Shell no debe reconstruir el path desde:

- `FilesystemEntry`;
- el primer item producido;
- el prompt;
- el directorio actual del proceso;
- strings de presentación;
- el estado mutable de `Shell` después de iniciar la presentación.

## Entrada

Este caso de uso parte del resultado de ejecución existente:

```text
ExecutionResult::FilesystemIteration(iteration)
```

La `FilesystemIteration` ya contiene:

- estado lazy de lectura;
- `next_index`;
- path completo del scope que originó la iteración.

## Presenter

El agent existente:

```text
iteration_presenter::present
```

debe asumir la presentación completa de `iter`.

Responsabilidades añadidas por US-005:

- renderizar una línea vacía inicial;
- renderizar el header de la tabla inmediatamente después de esa separación inicial;
- conservar el consumo lazy mediante `Advance`;
- renderizar una línea vacía después de la tabla;
- renderizar el summary;
- renderizar `Path: <full path>` inmediatamente después del summary;
- renderizar una línea vacía final;
- aplicar la diferenciación visual de directorios de forma consistente entre `Type` y `Name`.

No debe:

- obtener metadata del filesystem;
- inventar index;
- obtener el path desde `Shell`;
- consultar el filesystem para resolver el path;
- ordenar resultados;
- almacenar toda la iteración;
- crear un total acumulado de bytes;
- modificar `Shell`;
- modificar `FilesystemScope`.

## Secuencia de presentación

Secuencia conceptual:

```text
iteration_presenter::present(iteration)

1. render blank line
2. render table header

3. loop lazy:
   - Advance
   - render row
   - update files/directories counters

4. Advance -> None

5. render blank line
6. render summary
7. render Path footer immediately after the summary
8. render blank line
9. return
```

Después de que el presenter retorna, el main/shell loop vuelve a renderizar el prompt.

## Regla de blank lines

`iter` comienza su presentación con una línea vacía y termina su presentación con una línea vacía.

La responsabilidad de estas líneas pertenece a `iteration_presenter::present`.

`main.rs` no debe insertar líneas vacías específicas de la presentación de `iter`.

Secuencia visible:

```text
command
<blank line>

table
<blank line>

summary
Path
<blank line>

prompt
```

## Path footer

El footer `Path:` debe salir de la propia iteración recibida:

```text
iteration.path()
```

La presentación describe la iteración recibida.

No debe consultar posteriormente el scope activo mutable de `Shell`.

Esto evita que la salida de una iteración dependa de otro estado después de que la iteración ya fue creada.

Formato visible:

```text
5 directories
7 files
Path: /home/user/repos/evolution/evo-shell/target/release
```

`Path:` es texto de presentación.

El path mostrado debe representar la ubicación completa iterada.

No debe ser el prompt compacto ni una forma abreviada como:

```text
…/target
```

La conversión a texto debe respetar la representación de paths del sistema operativo y evitar pérdida innecesaria de información.

No debe existir una línea vacía entre `N files` y `Path:`.

Debe existir una línea vacía después del footer `Path:` antes de que el main/shell loop renderice el siguiente prompt.

## Identidad visual del footer

El footer reutiliza la identidad visual del prompt.

Prompt conceptual:

```text
scope-fs …/src >
^^^^^^^ ^^^^^^^
style A style B
```

Footer:

```text
Path: /home/user/repos/evolution/evo-shell/src
^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
style A                  style B
```

Reglas técnicas:

- `Path:` usa el mismo estilo visual que `scope-fs`;
- el full path usa el mismo estilo visual que la ubicación compacta del prompt;
- cuando sea razonable con el código actual, ambas superficies deben reutilizar las mismas definiciones mínimas de estilo;
- no se duplican colores mágicos distintos para prompt y footer;
- no se crea una entidad de dominio para estilos.

Si las definiciones de estilo viven cerca del render actual del prompt y `iteration_presenter` necesita usarlas, la evolución debe ser la mínima coherente con el código existente.

No se crea:

- `Theme`;
- `PromptTheme`;
- `PathTheme`;
- `Style`;
- crate de presentación.

## Directorio vacío

El path pertenece a la iteración, no a sus filas.

Por eso una iteración vacía puede presentar el footer:

```text
scope-fs …/empty > iter

#   Modified             Type    Size      Name

0 directories
0 files
Path: /home/user/empty

scope-fs …/empty >
```

El presenter debe renderizar el footer `Path:` aunque `Advance` devuelva `None` en la primera llamada.

No se renderiza `Path:` antes del header.

## Tabla

US-005 conserva la tabla aprobada por US-004:

```text
#   Modified             Type    Size      Name
```

No cambia:

- `index`;
- `Modified`;
- `Type`;
- `Size`;
- `Name`;
- `format_size`;
- `format_modified`;
- consumo lazy;
- counters;
- summary.

## Directory Color

Para una fila de directorio, `Type` y `Name` deben compartir la misma diferenciación visual.

Conceptualmente:

```text
dir       target/
^^^^      ^^^^^^^
same directory style
```

No es necesario colorear `index`, `Modified` ni `Size` para indicar que la fila es un directorio.

Para archivos, `file` y el nombre pueden permanecer en presentación default/neutra.

Symlink y Other conservan su diferenciación visual actual. Si la presentación ya puede aplicar un estilo coherente a `Type` y `Name`, puede hacerlo sin cambiar el dominio.

## Colores como presentación

Los colores pertenecen exclusivamente a Evo Shell.

Evo Shell Engine no conoce colores.

No se agregan propiedades de presentación a:

- `FilesystemEntry`;
- `FilesystemEntryKind`;
- `FilesystemIteration`;
- `FilesystemIterationItem`.

Evo Shell puede reutilizar el mecanismo ligero de secuencias de terminal ya usado por `iteration_presenter`.

No se agrega crate externo de terminal.

No se crea un sistema de themes.

No se documenta una paleta configurable.

El color del footer `Path:` pertenece a esta misma frontera de presentación.

## Prompt

US-005 también mejora visualmente el prompt.

El formato funcional permanece compacto:

```text
scope-fs …/target >
```

El prompt no vuelve a mostrar el path completo.

Responsabilidades:

- `scope-fs`: tipo de scope;
- `…/target`: ubicación compacta;
- `>`: separador para entrada del usuario.

`scope-fs` y `…/target` deben renderizarse con estilos visuales distintos.

El símbolo `>` puede permanecer neutral/default.

El footer `Path:` reutiliza esos estilos:

- `Path:` comparte estilo con `scope-fs`;
- el full path comparte estilo con la ubicación compacta.

Si el prompt actual se genera en `main.rs`, la evolución mínima es mantener esa responsabilidad donde ya vive la presentación del prompt.

No se crea un agent nuevo si el componente actual puede asumir el estilo visual.

No se crea una arquitectura genérica de themes.

## Flujo principal

1. El usuario ejecuta `iter`.
2. `executor::execute` devuelve `ExecutionResult::FilesystemIteration(iteration)`.
3. El main/shell loop delega en `iteration_presenter::present(iteration)`.
4. El presenter renderiza línea vacía y header.
5. El presenter consume la iteración de forma lazy mediante `Advance`.
6. Cada item se presenta como una fila.
7. El presenter actualiza solo contadores locales de files/directories.
8. Al terminar, renderiza summary.
9. El presenter lee el path completo de la propia iteración.
10. El presenter renderiza el footer `Path:` inmediatamente después del summary.
11. El presenter renderiza una línea vacía final.
12. El main/shell loop renderiza el siguiente prompt compacto.

## Errores

US-005 no introduce errores de dominio nuevos para path.

El path ya existe en una `FilesystemIteration` válida.

Mostrarlo es responsabilidad de presentación.

Se mantienen los errores existentes:

- errores de `Iter`;
- errores de `Advance`;
- errores de escritura/salida.

No se introduce `PathPresentationError`.

## Relación con futuro `pwd`

Una capacidad futura equivalente a `pwd` podrá reutilizar la misma fuente de verdad del path del `FilesystemScope`.

US-005 no requiere `pwd`.

Esta documentación no define:

- comando `pwd`;
- `Command::Pwd`;
- agent de `pwd`;
- resolver de path;
- provider de path;
- sintaxis;
- arquitectura de `pwd`.

## Fuera de alcance

- comando `pwd`
- `Command::Pwd`
- agent de `pwd`
- use case para obtener path
- agent para obtener path
- resolver o provider para obtener path
- pipelines
- index operator
- filter
- select
- first
- to-value
- to-text
- sorting
- pagination
- iteración recursiva
- detección de ancho de terminal
- themes
- colores configurables
- renderer genérico de tablas
- presenter genérico
- scopes DB
- scopes URL
- código Rust
