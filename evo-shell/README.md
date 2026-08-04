# Evo Shell

Evo Shell es la interfaz interactiva de comandos del ecosistema Evo.

Evo Shell consume capacidades proporcionadas por `evo-shell-engine`.

Separación conceptual:

- Evo Shell recibe instrucciones del usuario.
- Evo Shell interpreta comandos y argumentos.
- Evo Shell mantiene el estado necesario de la interacción.
- Evo Shell solicita capacidades a Evo Shell Engine.
- Evo Shell presenta resultados al usuario.
- Evo Shell Engine proporciona las capacidades operativas.
- Evo Shell Engine posee su dominio.
- Evo Shell Engine define sus use cases de frontera.
- Evo Shell Engine implementa scopes y operaciones como `iter`.
- Evo Shell Engine no conoce la sintaxis de Evo Shell.

Evo Shell todavía no define implementaciones concretas de lexer, parser o AST.

## Gramática básica de comandos

La primera regla establecida de la gramática de Evo Shell es que los nombres compuestos de instrucciones utilizan `-`.

Los espacios separan la instrucción de sus argumentos.

Ejemplo:

```text
scope-fs "/home/user/documents"
```

Interpretación conceptual:

```text
scope-fs
```

= nombre completo de una única instrucción

```text
"/home/user/documents"
```

= argumento

El espacio separa la instrucción de su argumento.

Esta regla establece la convención inicial de la gramática de Evo Shell.

Documentación:

- [functional_documentation/](functional_documentation/)
- [technical_documentation/](technical_documentation/)
