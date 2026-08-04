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

## Inicialización y scope inicial

Evo Shell toma el directorio actual del proceso al iniciar.

Esa ubicación se solicita a Evo Shell Engine mediante `SetFilesystemScope`.

Evo Shell solo entra en estado operativo si obtiene un `FilesystemScope` válido.

Durante la operación normal siempre existe un filesystem scope activo.

`scope-fs` reemplaza ese scope únicamente cuando el nuevo scope puede resolverse correctamente.

`iter` presta el scope activo, pero no lo modifica.

La decisión usa el directorio actual desde el que se lanzó Evo Shell, no el directorio home del usuario.

Ejemplo conceptual:

```text
~/repos/evolution> evo-shell
```

Scope inicial:

```text
~/repos/evolution
```

Esta documentación no fija una sintaxis específica de prompt.

La obtención del directorio actual debe apoyarse en la abstracción multiplataforma de Rust `std`.

Evo Shell no construye manualmente rutas Linux o Windows, no asume `/home/...`, no asume `C:\Users\...` y no utiliza variables de entorno como `HOME` o `USERPROFILE` para esta decisión.

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
