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

Casos de uso técnicos:

* [UC-001 — Establecer un filesystem scope mediante un comando](use_cases/UC-001-set-filesystem-scope-command/)
* [UC-002 — Iterar el filesystem scope activo mediante el comando iter](use_cases/UC-002-iterate-filesystem-scope-command/)
