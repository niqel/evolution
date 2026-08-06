# Documentación funcional

Esta carpeta contiene la documentación funcional de Evo Shell.

Las historias de usuario describen el comportamiento observable desde la perspectiva del usuario de la shell, incluyendo:

* los comandos disponibles;
* la sintaxis observable de los comandos;
* los argumentos requeridos;
* los resultados esperados;
* los errores observables;
* el estado que debe conservarse entre comandos cuando sea necesario.

Las reglas de lenguaje describen convenciones sintácticas transversales reutilizables por múltiples comandos.

Evo Shell proporciona una interfaz de comandos sobre las capacidades de Evo Shell Engine.

La documentación funcional de Evo Shell describe **qué solicita el usuario mediante comandos y qué comportamiento espera obtener**, pero no duplica la definición de las capacidades operativas proporcionadas por Evo Shell Engine.

Los detalles internos de implementación pertenecen a la documentación técnica y se documentan únicamente cuando sean necesarios para implementar una historia de usuario.

Esto incluye, entre otros:

* lexer;
* parser;
* AST;
* representación interna del estado;
* ejecución de comandos;
* integración técnica con Evo Shell Engine.

## Historias de usuario

* [US-001 — Establecer un filesystem scope mediante un comando](user_stories/US-001-set-filesystem-scope-command.md)
* [US-002 — Iterar el filesystem scope activo mediante el comando iter](user_stories/US-002-iterate-filesystem-scope-command.md)
* [US-003 — Entrar en una ubicación del scope activo mediante el comando enter](user_stories/US-003-enter-active-scope-location.md)
* [US-004 — Mostrar información estructurada de los elementos mediante iter](user_stories/US-004-show-structured-filesystem-iteration.md)
* [US-005 — Mejorar el contexto y la legibilidad visual de iter](user_stories/US-005-improve-iter-context-and-visual-readability.md)
* [US-006 — Mostrar fecha de creación de elementos en iter](user_stories/US-006-show-filesystem-created-time.md)
* [US-007 — Unificar la identidad visual de la tabla, los archivos y el prompt](user_stories/US-007-unify-shell-visual-identity.md)
* [US-008 — Limpiar la terminal](user_stories/US-008-clear-terminal.md)
* [US-009 — Iniciar Evo Shell con una presentación de bienvenida](user_stories/US-009-start-shell-with-welcome.md)
* [US-010 — Terminar Evo Shell mediante el comando `exit`](user_stories/US-010-exit-shell.md)
* [US-011 — Ejecutar un pipeline estructurado](user_stories/US-011-execute-structured-pipeline.md)

## Reglas de lenguaje

* [LR-001 — Command Arguments and Options](language_rules/LR-001-command-arguments-and-options.md)
* [LR-002 — Pipeline Syntax, Grouping and Argument Expansion](language_rules/LR-002-pipeline-syntax-grouping-and-argument-expansion.md)
