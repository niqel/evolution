# Evo Shell Engine

`evo-shell-engine` es el motor operativo basado en scopes del ecosistema Evo.

Proporciona las capacidades fundamentales utilizadas por interfaces superiores. Está diseñado para ser consumido por Evo Shell, Evo Script y potencialmente otros componentes del ecosistema Evo.

Separación conceptual:

- Evo Shell Engine: scopes, operaciones, dominio, casos de uso y providers.
- Evo Shell: futura interfaz interactiva y command language.
- Evo Script: futuro lenguaje de scripting.

Frontera pública:

- los use cases son contratos de entrada públicos del engine;
- los contracts son capacidades internas que el engine necesita de infraestructura;
- los providers son implementaciones internas de esos contracts;
- un consumidor externo no debe proporcionar contracts ni providers internos.

Para iteración de filesystem, la frontera pública conceptual se compone de:

- `Iter(&FilesystemScope)`: inicia una `FilesystemIteration`;
- `Advance(&mut FilesystemIteration)`: avanza la iteración lazy y produce el siguiente `FilesystemEntry`, fin de iteración o error.

Para filtrado estructurado de resultados ya materializados, la frontera pública conceptual incluye:

- `Filter(&[FilesystemIterationItem], &FilterExpression)`: conserva únicamente los elementos que cumplen un predicado estructurado.

Para proyección estructurada de propiedades ya materializadas, la frontera pública conceptual incluye:

- `Select(&[FilesystemIterationItem], &[SelectProperty])`: proyecta propiedades estructuradas de cada elemento sin eliminar filas.

Para navegación dentro de un filesystem scope, la frontera pública conceptual incluye:

- `Enter(&FilesystemScope, &Path)`: resuelve una ubicación relativa desde el scope actual y produce un nuevo `FilesystemScope` válido o error.

El engine no depende conceptualmente de prompt, terminal, lexer, parser, AST, sintaxis de Evo Script ni UI.

Documentación:

- [functional_documentation/](functional_documentation/)
- [technical_documentation/](technical_documentation/)
