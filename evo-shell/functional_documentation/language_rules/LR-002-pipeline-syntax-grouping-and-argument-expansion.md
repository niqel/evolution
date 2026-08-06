# LR-002 — Pipeline Syntax, Grouping and Argument Expansion

## Propósito

Esta regla formaliza una convención sintáctica transversal de Evo Shell.

Su objetivo es describir conceptualmente:

- pipelines con `|>`;
- continuación multilínea;
- agrupación mediante paréntesis;
- argumentos posicionales;
- argumentos nombrados;
- coma como separador de argumentos;
- resultados estructurados dentro de pipelines;
- `to-value`;
- `to-values`;
- `to-args`.

Esta regla no define la gramática completa de Evo Shell.

Tampoco define la implementación interna de los pipelines.

## Pipeline

La sintaxis aprobada para conectar etapas es:

```text
command |> command |> command
```

Ejemplo conceptual:

```text
iter |> index 0 |> select name |> to-value
```

Interpretación conceptual:

- `iter` produce un resultado estructurado;
- `index 0` recibe la salida anterior;
- `select name` recibe la salida anterior;
- `to-value` transforma el resultado final en un valor escalar.

El resultado de una etapa entra conceptualmente como entrada de la siguiente.

Los pipelines transportan datos estructurados, no texto sintáctico.

## Continuación multilínea

Una expresión no debe ejecutarse mientras esté sintácticamente incompleta.

Si una línea termina en `|>`, Evo Shell debe considerar que la expresión continúa.

Ejemplo:

```text
iter |>
index 0 |>
select name |>
to-value
```

No se necesita un marcador manual de continuación.

La propia sintaxis del pipeline indica que falta otra etapa.

## Agrupación

Un paréntesis abierto también mantiene la expresión incompleta.

Ejemplo:

```text
enter (
    iter |>
    index 0 |>
    select name |>
    to-value
)
```

Mientras exista un `(` sin cerrar, Evo Shell debe continuar leyendo.

La expresión se considera completa cuando:

- no existe un pipeline pendiente;
- los delimitadores abiertos están cerrados;
- la expresión es sintácticamente válida.

Los paréntesis significan evaluar primero la expresión interna y usar su resultado en la expresión externa.

No representan sintaxis de llamada estilo función.

## Indentación

La indentación es exclusivamente visual.

Estos ejemplos son conceptualmente equivalentes:

```text
enter (
    iter |>
    index 0 |>
    select name |>
    to-value
)
```

y:

```text
enter (
iter |>
index 0 |>
select name |>
to-value
)
```

No se definen bloques semánticos por indentación.

## Argumentos posicionales

Un comando puede recibir argumentos posicionales.

Ejemplo conceptual:

```text
index 0
```

Aquí:

- `index` es el comando;
- `0` es el argumento posicional.

Otro ejemplo conceptual:

```text
copy-to file1.txt, file2.txt, file3.txt, path: "~/repos/documents"
```

Los valores `file1.txt`, `file2.txt` y `file3.txt` son argumentos posicionales.

La posición y el contrato del comando determinan su función semántica.

## Argumentos nombrados

Un argumento puede ser nombrado mediante:

```text
name: value
```

Ejemplo conceptual:

```text
path: "~/repos/documents"
```

`path` es el nombre del argumento.

`"~/repos/documents"` es su valor.

Un argumento nombrado es distinto de una opción.

Según LR-001:

- argumento: dato necesario para el comando;
- opción: modifica cómo se comporta el comando.

## Coma

La coma separa argumentos del comando actual.

Ejemplo conceptual:

```text
copy-to file1.txt, file2.txt, file3.txt, path: "~/repos/documents"
```

Conceptualmente, la coma permite listar múltiples argumentos homogéneos.

La coma no significa:

- AND;
- OR;
- pipeline;
- cambio automático de tipo;
- cambio automático de parámetro lógico.

La asignación de argumentos al contrato del comando depende de la firma o semántica del comando.

## Resultados estructurados

Los pipelines transportan resultados estructurados.

No transportan texto para volver a parsearlo.

En particular, una colección producida por un pipeline no contiene comas sintácticas.

La representación explicativa de una colección puede verse así:

```text
["1.txt", "2.txt", "3.txt"]
```

Pero esa coma es solo una convención descriptiva.

No se fija aquí si la implementación interna será una lista, un iterador, un stream o cualquier otra estructura.

## to-value

`to-value` tiene semántica escalar.

Conceptualmente:

- una fila;
- una columna;

pueden transformarse en un único valor.

Ejemplo conceptual:

```text
iter
|> index 0
|> select name
|> to-value
```

Resultado conceptual:

```text
"file1.txt"
```

`to-value` no debe interpretarse como una transformación de colección.

## to-values

`to-values` representa la transformación de una selección de una columna con múltiples filas en una colección estructurada de valores.

Ejemplo conceptual:

```text
iter
|> select name
|> to-values
```

Resultado conceptual:

```text
colección:
- "file1.txt"
- "file2.txt"
- "file3.txt"
```

`to-values` no implica expansión como argumentos.

## to-args

`to-args` representa una transformación distinta.

Su propósito es convertir una selección homogénea de valores en argumentos posicionales para el comando consumidor.

Ejemplo conceptual:

```text
copy-to (
    iter
    |> select full_name
    |> to-args
), path: "~/repos/documents"
```

Conceptualmente equivale a:

```text
copy-to file1.txt, file2.txt, file3.txt, path: "~/repos/documents"
```

`to-args` no genera texto fuente ni comas sintácticas.

Produce expansión semántica de argumentos.

`to-args` puede operar directamente después de una selección apropiada.

No es obligatorio pasar antes por `to-values`.

## Relación entre to-value, to-values y to-args

La relación conceptual aprobada es:

- `to-value`:
  - una fila + una columna;
  - valor escalar.
- `to-values`:
  - múltiples filas + una columna;
  - colección de valores.
- `to-args`:
  - múltiples filas + una columna;
  - expansión de valores como argumentos posicionales.

`to-args` no es un alias de `to-values`.

`to-values` no es un requisito previo de `to-args`.

## Homogeneidad de to-args

Los valores expandidos por `to-args` deben pertenecer al mismo argumento variádico lógico del comando y ser homogéneos según el contrato esperado.

Ejemplo conceptual:

```text
copy-to
```

puede entenderse conceptualmente como:

```text
sources: Path...
path: Path
```

Esto solo explica la semántica de un contrato conceptual.

No define una sintaxis de funciones.

## Misma semántica: manual vs pipeline

Estas dos formas deben representar conceptualmente la misma entrada para el comando consumidor:

Forma manual:

```text
copy-to file1.txt, file2.txt, file3.txt, path: "~/repos/documents"
```

Forma derivada:

```text
copy-to (
    iter
    |> select full_name
    |> to-args
), path: "~/repos/documents"
```

El comando consumidor no debe necesitar saber si los argumentos fueron escritos manualmente o producidos por un pipeline.

## Ejemplos principales

Ejemplo principal con `enter`:

```text
enter (iter |> index 0 |> select name |> to-value)
```

Versión multilínea:

```text
enter (
    iter |>
    index 0 |>
    select name |>
    to-value
)
```

Ejemplo principal con `copy-to`:

```text
copy-to (
    iter
    |> filter ext = "txt"
    |> select full_name
    |> to-args
), path: "~/repos/documents"
```

`filter ext = "txt"` aparece aquí solo como expresión ilustrativa simple.

## Filtrado fuera de alcance

LR-002 no resuelve la semántica completa de `filter`.

No se define todavía:

- AND;
- OR;
- múltiples condiciones;
- precedencia lógica;
- coma dentro de expresiones de filtrado;
- agrupación booleana compleja.

## Alcance diferido

Quedan diferidos al menos los siguientes puntos:

- implementación del parser multilinea;
- AST;
- lexer concreto;
- estructura interna del pipeline;
- Vec vs iterator vs stream;
- ejecución lazy o eager;
- filter avanzado;
- AND / OR;
- precedencia;
- short circuit;
- redirecciones;
- stdin/stdout pipes tradicionales;
- `|` estilo Unix;
- pipelines asíncronos;
- pipelines paralelos;
- propagación de errores;
- comportamiento de Result / Option dentro del pipeline;
- coerción de tipos;
- variables;
- almacenamiento de `to-values`;
- declaración de funciones;
- firmas formales;
- splat o spread syntax;
- wildcard o glob;
- quoting avanzado;
- escaping;
- implementación de `index`;
- implementación de `select`;
- implementación de `to-value`;
- implementación de `to-values`;
- implementación de `to-args`;
- implementación de `copy-to`;
- implementación de `filter`.

## Principios de diseño

1. Los pipelines transportan datos estructurados, no strings de comandos.
2. La sintaxis debe indicar naturalmente cuándo una expresión está incompleta.
3. No se requieren marcadores manuales de continuación de línea.
4. Los paréntesis agrupan y fuerzan evaluación previa.
5. Las comas separan argumentos del comando actual.
6. Los argumentos pueden ser posicionales o nombrados.
7. Los resultados de pipelines pueden convertirse explícitamente en escalares, colecciones o argumentos.
8. Las transformaciones deben ser explícitas: `to-value`, `to-values`, `to-args`.
9. El comando consumidor no debe distinguir entre argumentos escritos manualmente y argumentos derivados mediante `to-args`.

## Fuera de alcance

LR-002 no define:

- la gramática completa de Evo Shell;
- la implementación interna;
- el modelo de tipos Rust;
- la ejecución concreta de `index`, `select`, `to-value`, `to-values`, `to-args`, `copy-to` o `filter`;
- la semántica avanzada de filtrado;
- los detalles de errores concretos para casos futuros;
- el sistema completo de pipelines.
