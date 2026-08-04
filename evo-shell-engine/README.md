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

El engine no depende conceptualmente de prompt, terminal, lexer, parser, AST, sintaxis de Evo Script ni UI.

Documentación:

- [functional_documentation/](functional_documentation/)
- [technical_documentation/](technical_documentation/)
