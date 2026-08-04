# Documentación funcional

Esta carpeta contiene la documentación funcional de Evo Shell.

Las historias de usuario describen el comportamiento observable desde la perspectiva del usuario de la shell, incluyendo:

* los comandos disponibles;
* la sintaxis observable de los comandos;
* los argumentos requeridos;
* los resultados esperados;
* los errores observables;
* el estado que debe conservarse entre comandos cuando sea necesario.

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
