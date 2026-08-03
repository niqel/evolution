# Evo Shell

Evo Shell es una shell basada en ambitos (`scope`) que busca ofrecer un modelo comun de operaciones sobre diferentes tipos de recursos.

Un `scope` define el ambito o dominio sobre el que trabajan las operaciones. El mismo modelo de shell puede trabajar posteriormente con diferentes ambitos, como filesystem, database, URL/resources o Web API.

Estos ejemplos representan la direccion del diseno y no deben entenderse todavia como funcionalidades implementadas.

Una operacion puede conservar el mismo significado conceptual aunque cambie el `scope`. Por ejemplo, `iter` es la operacion fundamental prevista para iterar sobre los elementos disponibles en el scope activo.

Ejemplo conceptual:

```text
scope fs "/home/user"
iter
```

Evo Shell todavia esta en una etapa inicial de diseno y desarrollo.

Documentacion:

- [functional_documentation/](functional_documentation/)
- [technical_documentation/](technical_documentation/)
