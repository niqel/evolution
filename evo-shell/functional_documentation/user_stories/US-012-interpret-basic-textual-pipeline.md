# US-012 — Interpretar un pipeline textual básico

## Historia de usuario

Como usuario de Evo Shell,
quiero escribir un pipeline textual básico con `|>`,
para que la shell lo convierta en un `Pipeline` estructurado sin ejecutar todavía sus operaciones.

## Descripción

Evo Shell ya recibe texto, lo tokeniza e interpreta comandos simples.
Esta historia extiende esa capacidad para reconocer una secuencia de etapas de pipeline y construir un modelo estructurado equivalente a:

```text
Command::Pipeline(
    Pipeline [
        Iter,
        Take(1),
        Select([Name]),
        ToValue
    ]
)
```

La shell no ejecuta el pipeline en esta historia.
La shell solo interpreta la entrada textual y la convierte en una representación tipada.

La semántica de ejecución pertenece a una capacidad separada de ejecución de pipelines estructurados, que ya existe como frontera independiente.

## Flujo observable

Ejemplo canónico:

```text
iter |> take 1 |> select name |> to-value
```

Resultado conceptual:

```text
Command::Pipeline(Pipeline [...])
```

con las etapas ordenadas exactamente como fueron escritas.

## Alcance de la interpretación

La primera versión de esta capacidad reconoce únicamente:

- `iter`
- `index N`
- `take N`
- `select propiedad`
- `select propiedad, propiedad, ...`
- `to-value`
- `to-values`
- `to-args`
- el separador `|>`

La interpretación conserva la secuencia de etapas y no las reduce a strings.

## Semántica observable

1. La shell puede recibir una entrada textual con `|>`.
2. Las etapas se interpretan en el orden escrito.
3. La entrada se convierte en un `Pipeline` estructurado.
4. Las operaciones no se guardan como strings.
5. `iter` puede ser una etapa.
6. `index N` puede ser una etapa.
7. `take N` puede ser una etapa.
8. `select name` puede ser una etapa.
9. `select` acepta múltiples propiedades por nombre.
10. `to-value` puede ser una etapa.
11. `to-values` puede ser una etapa.
12. `to-args` puede ser una etapa.
13. Una operación desconocida produce error.
14. Una etapa vacía produce error.
15. Los argumentos faltantes producen error.
16. Los argumentos inválidos producen error.
17. Parsing no ejecuta el pipeline.
18. Parsing no valida la compatibilidad de tipos entre etapas.
19. `filter` queda fuera de alcance.
20. Los subpipelines quedan fuera de alcance.
21. La lectura multilinea queda fuera de alcance.
22. Los comandos simples existentes siguen funcionando.

## Reglas de interpretación

- `|>` separa etapas de izquierda a derecha.
- Cada etapa debe contener una operación completa.
- `select` respeta la lista de propiedades separadas por coma.
- `take` e `index` esperan un entero no negativo.
- `to-value`, `to-values` y `to-args` no aceptan argumentos adicionales.

La validación semántica de compatibilidad entre etapas no pertenece a esta historia.

Ejemplo:

```text
iter |> to-value
```

puede interpretarse como `Command::Pipeline(Pipeline [...])` aunque después la ejecución semántica rechace la transición.

## Errores observables

Esta historia reconoce como errores de interpretación, entre otros:

- etapa vacía;
- separador final sin etapa siguiente;
- operación desconocida;
- argumento faltante;
- argumento inválido;
- lista de propiedades vacía en `select`;
- argumento adicional en `to-value`, `to-values` o `to-args`.

## Compatibilidad

Los comandos simples existentes siguen funcionando:

- `scope-fs`
- `iter`
- `enter`
- `clear`
- `exit`

Si la entrada no contiene un pipeline, Evo Shell continúa resolviendo el comando como hasta ahora.

## Fuera de alcance

- ejecución del pipeline;
- parser de `filter`;
- subpipelines;
- lectura multilinea;
- presentación del resultado del pipeline;
- validación de compatibilidad entre etapas;
- integración textual de argumentos nombrados que contengan pipelines.
