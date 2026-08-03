# Evo Shell

Evo Shell es una shell basada en ámbitos (`scope`) que busca ofrecer un modelo común de operaciones sobre diferentes tipos de recursos.

Un `scope` define el ámbito o dominio sobre el que trabajan las operaciones. El mismo modelo de shell puede trabajar posteriormente con diferentes ámbitos, como filesystem, database, URL/resources o Web API.

Estos ejemplos representan la dirección del diseño y no deben entenderse todavía como funcionalidades implementadas.

Una operación puede conservar el mismo significado conceptual aunque cambie el `scope`. Por ejemplo, `iter` es la operación fundamental prevista para iterar sobre los elementos disponibles en el scope activo.

Ejemplo conceptual:

```text
scope fs "/home/user"
iter
```

Evo Shell todavía está en una etapa inicial de diseño y desarrollo.

Documentación:

- [functional_documentation/](functional_documentation/)
- [technical_documentation/](technical_documentation/)
