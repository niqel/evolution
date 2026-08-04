# Evo Shell Engine

`evo-shell-engine` es el motor operativo basado en scopes del ecosistema Evo.

Proporciona las capacidades fundamentales utilizadas por interfaces superiores. Está diseñado para ser consumido por Evo Shell, Evo Script y potencialmente otros componentes del ecosistema Evo.

Separación conceptual:

- Evo Shell Engine: scopes, operaciones, dominio, casos de uso y providers.
- Evo Shell: futura interfaz interactiva y command language.
- Evo Script: futuro lenguaje de scripting.

El engine no depende conceptualmente de prompt, terminal, lexer, parser, AST, sintaxis de Evo Script ni UI.

Documentación:

- [functional_documentation/](functional_documentation/)
- [technical_documentation/](technical_documentation/)
