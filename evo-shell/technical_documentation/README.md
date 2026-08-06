# Documentación técnica

Esta carpeta contiene la documentación técnica de Evo Shell.

La documentación técnica describe cómo Evo Shell implementa el comportamiento definido por sus historias de usuario y cómo se integra con las capacidades proporcionadas por Evo Shell Engine.

A partir de las historias de usuario se documentarán, cuando sean necesarios:

* casos de uso técnicos;
* diagramas de arquitectura;
* diagramas de secuencia;
* modelos de datos;
* representación interna de comandos;
* interpretación de instrucciones y argumentos;
* estado necesario entre comandos;
* ejecución de comandos;
* integración con los use cases de frontera de Evo Shell Engine;
* presentación de resultados al usuario.

Evo Shell es responsable de la interpretación y ejecución de su lenguaje de comandos.

Evo Shell Engine es responsable de las capacidades operativas que Evo Shell consume.

La documentación técnica de Evo Shell no debe duplicar la arquitectura interna de Evo Shell Engine. Cuando un comando requiera una capacidad del engine, debe documentarse la interacción con su use case de frontera correspondiente.

Los componentes internos de Evo Shell deben surgir de las necesidades reales de los casos de uso.

Por esta razón, elementos como:

* lexer;
* parser;
* AST;
* representación del estado;
* executor;
* renderer;

se documentarán únicamente cuando sean necesarios para implementar una historia de usuario concreta.

Las decisiones técnicas deben mantener separada la sintaxis y la interacción propias de Evo Shell de las capacidades operativas proporcionadas por Evo Shell Engine.

## Inicialización de Evo Shell

Evo Shell toma el directorio actual del proceso al iniciar y lo usa como filesystem scope inicial.

La inicialización construye una instancia operativa de `Shell`.

Una instancia operativa de Evo Shell siempre posee un `FilesystemScope` válido.

Relación conceptual:

```text
Shell
└── owns FilesystemScope
```

`Shell` representa una instancia operativa de Evo Shell.

La vida del `FilesystemScope` está ligada a la vida de la instancia de `Shell`.

No se utiliza `Option<FilesystemScope>` como estado operativo normal.

### Flujo

```text
InitializeShell
        ↓
shell_initializer::initialize
        ↓
shell::resolve
        ↓
CurrentDirectory
        ↓
current_directory::provide
        ↓
std::env::current_dir()
        ↓
PathBuf
        ↓
SetFilesystemScope(&Path)
        ↓
evo-shell-engine
        ↓
FilesystemScope
        ↓
Shell
```

### Responsabilidades

`InitializeShell` es el use case de Evo Shell para construir una instancia operativa.

`shell_initializer::initialize` es el agent que coordina el flujo de inicialización.

`shell::resolve` es el resolver que materializa una `Shell` a partir de las capacidades necesarias.

`CurrentDirectory` es un contract interno de Evo Shell y debe definirse mediante function pointer.

`current_directory::provide` es el provider interno que usa `std::env::current_dir()`.

Flujo del resolver:

```text
CurrentDirectory
        ↓
PathBuf
        ↓
SetFilesystemScope
        ↓
FilesystemScope
        ↓
Shell
```

Evo Shell obtiene `PathBuf` mediante infraestructura propia y consume directamente `SetFilesystemScope(&Path)` de Evo Shell Engine.

Evo Shell Engine resuelve:

```text
Path
    ↓
FilesystemScope
```

Evo Shell no duplica validación de directorio, providers de filesystem, resolvers de filesystem ni `std::fs` interno del engine.

El `FilesystemScope` recibido desde Evo Shell Engine representa una ubicación filesystem resuelta.

Evo Shell no corrige, normaliza ni canonicaliza ese path.

### Entidad Shell

`Shell`:

* owns `FilesystemScope`;
* representa una instancia operativa de Evo Shell;
* vive mientras la shell está operativa;
* mantiene vivo su `FilesystemScope` mientras vive la `Shell`.

Puede existir conceptualmente un constructor interno mínimo `Shell::new(FilesystemScope)`, pero no representa la API principal de inicialización, no obtiene infraestructura, no valida filesystem y solo materializa una entidad ya válida.

El consumidor conceptual debe usar `shell_initializer::initialize`.

## Prompt y presentación del scope activo

El prompt de Evo Shell debe mostrar una representación compacta del filesystem scope activo.

Formato aprobado:

```text
scope-fs …/src >
```

El prompt incluye explícitamente el tipo de scope:

```text
scope-fs
```

Esto prepara conceptualmente la presentación para futuros tipos de scope, como `scope-db` o `scope-url`, sin diseñarlos ni implementarlos todavía.

La parte:

```text
…/
```

significa que existen componentes anteriores del path que no se muestran en el prompt.

`…/` no forma parte del `FilesystemScope`.

`…/` no es sintaxis que pueda introducir el usuario.

`…/` no modifica el path.

Es exclusivamente presentación.

Ejemplos:

```text
FilesystemScope real:
/home/user/repos/evolution/evo-shell/src

Prompt:
scope-fs …/src >
```

```text
FilesystemScope real:
/home/user/repos/evolution/evo-shell/src/agents

Prompt:
scope-fs …/agents >
```

Después de:

```text
enter ..
```

si el `FilesystemScope` resuelto es:

```text
/home/user/repos/evolution/evo-shell/src
```

el prompt debe ser:

```text
scope-fs …/src >
```

Para la raíz del filesystem, el prompt debe usar una representación compacta apropiada sin inventar un segmento.

No se usa `~` como sustituto arbitrario del path.

El formato aprobado para paths truncados es:

```text
…/<último-segmento>
```

La presentación no debe corregir paths ni resolver componentes como `..`.

Evo Shell Engine mantiene la invariante del `FilesystemScope`.

Evo Shell obtiene el `FilesystemScope` actual desde `Shell` y muestra únicamente una representación compacta.

Un cambio exitoso de scope no debe imprimir adicionalmente:

```text
Scope activo: /home/...
```

El prompt ya representa el estado activo.

Interacción conceptual:

```text
scope-fs …/evo-shell >
enter src
scope-fs …/src >
```

No:

```text
scope-fs …/evo-shell >
enter src
Scope activo: /home/user/repos/evolution/evo-shell/src
scope-fs …/src >
```

Lo mismo aplica a `scope-fs`.

`ExecutionResult::ScopeChanged` sigue existiendo conceptualmente como resultado de ejecución.

La capa de presentación decide no producir una línea adicional para ese resultado.

Una capacidad futura equivalente a `pwd` podría mostrar la ubicación completa, pero esta documentación no define ese comando, no crea una historia de usuario y no fija su nombre definitivo.

### Errores de inicialización

Errores conceptuales:

1. Error al obtener el directorio actual.
2. Error al convertirlo en `FilesystemScope` mediante Evo Shell Engine.

En ambos casos, no existe una `Shell` operativa.

Esta decisión no define todavía mensaje final de error, proceso/binario, `main.rs`, exit codes, shell loop, renderer, estructura de estado ni ownership Rust exacto.

Evo Shell debe apoyarse en la abstracción multiplataforma de Rust `std` para obtener el directorio actual y representar rutas con `Path`/`PathBuf`.

No se asumen rutas Linux como `/home/...`, rutas Windows como `C:\Users\...`, ni variables de entorno como `HOME` o `USERPROFILE`.

Casos de uso técnicos:

* [UC-001 — Establecer un filesystem scope mediante un comando](use_cases/UC-001-set-filesystem-scope-command/)
* [UC-002 — Iterar el filesystem scope activo mediante el comando iter](use_cases/UC-002-iterate-filesystem-scope-command/)
* [UC-003 — Entrar en una ubicación del scope activo mediante el comando enter](use_cases/UC-003-enter-active-scope-location/)
* [UC-004 — Presentar una iteración filesystem estructurada](use_cases/UC-004-present-structured-filesystem-iteration/)
* [UC-005 — Mejorar el contexto y la legibilidad visual de iter](use_cases/UC-005-improve-iter-context-and-visual-readability/)
* [UC-007 — Unificar la identidad visual de la tabla y el prompt](use_cases/UC-007-unify-shell-visual-identity/)
* [UC-008 — Limpiar la terminal](use_cases/UC-008-clear-terminal/)
* [UC-009 — Iniciar Evo Shell con una presentación de bienvenida](use_cases/UC-009-start-shell-with-welcome/)
* [UC-010 — Terminar Evo Shell mediante el comando `exit`](use_cases/UC-010-exit-shell/)
* [UC-011 — Ejecutar un pipeline estructurado](use_cases/UC-011-execute-structured-pipeline/)
* [UC-012 — Interpretar un pipeline textual básico](use_cases/UC-012-interpret-basic-textual-pipeline/)
* [UC-013 — Presentar el resultado de un pipeline ejecutado](use_cases/UC-013-present-pipeline-execution-result/)
* [UC-014 — Interpretar expresiones textuales de `filter`](use_cases/UC-014-interpret-textual-filter-expressions/)
* [UC-015 — Recolectar entrada textual multilínea de pipeline](use_cases/UC-015-collect-multiline-pipeline-input/)
* [UC-016 — Agrupar y evaluar una expresión mediante paréntesis](use_cases/UC-016-group-and-evaluate-expression-using-parentheses/)
* [UC-017 — Usar el resultado de una expresión agrupada como argumento de un comando](use_cases/UC-017-use-grouped-result-as-command-argument/)
* [UC-018 — Copiar archivos y directorios mediante copy-to](use_cases/UC-018-copy-files-and-directories-using-copy-to/)
