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
- `to-args`;
- `filter`;
- operadores comparativos básicos;
- operadores lógicos básicos.

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

## Filter

`filter` evalúa propiedades estructuradas de cada elemento de una secuencia.

Conserva únicamente los elementos que cumplen la expresión.

No convierte los elementos en texto.

No modifica el filesystem.

No cambia el scope.

No reindexa necesariamente en esta regla.

## Propiedades filtrables

Para el scope-fs actual, las propiedades filtrables iniciales son:

- `index`;
- `created`;
- `modified`;
- `type`;
- `size`;
- `name`.

Interpretación conceptual:

- `index`: índice estructurado del elemento dentro de la iteración;
- `created`: fecha y hora de creación cuando existe;
- `modified`: fecha y hora de modificación cuando existe;
- `type`: tipo de filesystem entry;
- `size`: tamaño cuando aplica;
- `name`: nombre del filesystem entry.

No se agregan propiedades nuevas en esta regla.

## index vs filter index

`index 0` y `filter index < 10` no representan la misma capacidad.

`index 0` usa `0` como argumento del comando `index` para seleccionar por posición.

`filter index < 10` evalúa una condición sobre la propiedad `index` de cada elemento.

Ejemplo conceptual:

```text
iter
|> filter index < 10
```

La expresión conserva conceptualmente todos los elementos cuyo índice sea menor que 10.

## Operadores comparativos

El conjunto inicial aprobado de operadores comparativos es:

- `equals`;
- `not-equals`;
- `>`;
- `<`;
- `at-least`;
- `at-most`;
- `between`;
- `not-between`.

## equals

La igualdad se expresa únicamente con `equals`.

Ejemplo conceptual:

```text
filter name equals "README.md"
```

No se aprueban como equivalentes:

- `=`;
- `==`;
- `===`.

## not-equals

La desigualdad se expresa con `not-equals`.

Ejemplo conceptual:

```text
filter type not-equals "directory"
```

No se aprueban como equivalentes:

- `!=`;
- `<>`;
- `diff`;
- `differs-from`.

## Mayor y menor

Se mantienen los símbolos simples:

- `>`;
- `<`.

Ejemplos conceptuales:

```text
filter size > 10kb
```

```text
filter index < 10
```

No se aprueban como equivalentes:

- `greater-than`;
- `less-than`.

## at-least

`at-least` representa conceptualmente mayor o igual.

Ejemplo conceptual:

```text
filter size at-least 50kb
```

Semánticamente equivale a `>=`, pero `>=` no es sintaxis aprobada.

## at-most

`at-most` representa conceptualmente menor o igual.

Ejemplo conceptual:

```text
filter size at-most 100kb
```

Semánticamente equivale a `<=`, pero `<=` no es sintaxis aprobada.

## between

`between` representa un rango inclusivo.

Ejemplo conceptual:

```text
filter size between 10kb, 100kb
```

Semánticamente equivale a:

```text
10kb <= size <= 100kb
```

Los límites pertenecen al predicado `between`.

La coma en `between 10kb, 100kb` separa los dos límites requeridos por ese predicado.

No se interpreta como dos condiciones independientes de `filter`.

## not-between

`not-between` es la negación exacta de `between`.

Ejemplo conceptual:

```text
filter size not-between 10kb, 100kb
```

Si `between` incluye ambos extremos, `not-between` conserva valores menores que `10kb` o mayores que `100kb`.

Los extremos `10kb` y `100kb` no cumplen `not-between`.

## Operadores lógicos

Los únicos operadores lógicos básicos aprobados en esta versión son:

- `and`;
- `or`.

Ejemplos conceptuales:

```text
filter type equals "file" and size > 10kb
```

```text
filter type equals "directory" or type equals "symlink"
```

No se aprueban como alternativas:

- `&`;
- `&&`;
- `|`;
- `||`;
- `!`.

## Encadenamiento

Se permiten cadenas del mismo operador lógico sin paréntesis.

Ejemplo conceptual:

```text
filter type equals "file"
    and size > 10kb
    and size < 1mb
```

Otro ejemplo conceptual:

```text
filter ext equals "txt"
    or ext equals "exe"
    or ext equals "md"
```

La indentación sigue siendo visual.

## Mezcla de and y or

Si `and` y `or` aparecen al mismo nivel lógico sin agrupación explícita, la expresión es inválida o ambigua.

Ejemplo no válido:

```text
A or B and C
```

Evo Shell no debe asumir automáticamente:

- `A or (B and C)`;
- `(A or B) and C`.

El usuario debe expresar su intención con paréntesis.

## Agrupación lógica

Se reutiliza la regla general de `(...)`:

la expresión interna se evalúa primero.

Ejemplos válidos:

```text
filter (A or B) and C
```

```text
filter A or (B and C)
```

Cuando se mezclan operadores lógicos distintos al mismo nivel, los paréntesis son obligatorios.

## Ejemplos de filter

```text
filter name equals "README.md"
```

```text
filter name not-equals "temp.txt"
```

```text
filter index < 10
```

```text
filter size > 10kb
```

```text
filter size at-least 50kb
```

```text
filter size at-most 5mb
```

```text
filter size between 10kb, 100kb
```

```text
filter size not-between 10kb, 100kb
```

```text
filter type equals "file" and size > 10kb
```

```text
filter (
    type equals "file"
    and size between 10kb, 5mb
)
```

```text
filter (
    type equals "directory"
    or type equals "symlink"
)
```

## filter en pipeline

Ejemplo conceptual:

```text
iter
|> filter index < 10
```

Otro ejemplo conceptual:

```text
iter
|> filter (
    type equals "file"
    and size between 10kb, 5mb
)
|> select full_name
|> to-args
```

Interpretación conceptual:

- `iter` produce elementos estructurados;
- `filter` evalúa el predicado para cada elemento;
- solo los elementos que cumplen continúan por el pipeline;
- `select` transforma posteriormente esa salida;
- `to-args` puede convertir una selección apropiada en argumentos para un consumidor.

## filter + copy-to

Puede mantenerse este ejemplo conceptual:

```text
copy-to (
    iter
    |> filter (
        type equals "file"
        and size between 10kb, 5mb
    )
    |> select full_name
    |> to-args
), path: "~/repos/documents"
```

No implica que `copy-to` esté implementado.

Solo demuestra composición conceptual.

## Valores no disponibles

La semántica de `filter` sobre valores opcionales queda diferida.

No se define todavía el comportamiento de:

- `created` cuando no existe;
- `modified` cuando no existe;
- `size` cuando no aplica.

## Operadores diferidos

Quedan diferidos al menos los siguientes puntos:

- `xor`;
- `contains`;
- `not-contains`;
- `starts-with`;
- `ends-with`;
- `matches`;
- `regex`;
- `glob`;
- `in`;
- `not-in`;
- short-circuit;
- reindexado después de `filter`;
- coerción de tipos;
- comparación Path/String;
- validación de unidades;
- formato formal de fechas;
- implementación concreta de `filter`.

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
    |> filter ext equals "txt"
    |> select full_name
    |> to-args
), path: "~/repos/documents"
```

`filter ext equals "txt"` aparece aquí solo como expresión ilustrativa simple.

## Alcance diferido

Quedan diferidos al menos los siguientes puntos:

- implementación del parser multilinea;
- AST;
- lexer concreto;
- estructura interna del pipeline;
- Vec vs iterator vs stream;
- ejecución lazy o eager;
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
10. `filter` trabaja sobre propiedades estructuradas.
11. Una comparación debe usar la forma aprobada cuando exista.
12. Evitar múltiples operadores equivalentes para el mismo significado.
13. `equals` y `not-equals` reemplazan familias como `==`, `===`, `!=`, `<>`.
14. `<` y `>` se conservan por claridad.
15. `at-least`, `at-most`, `between` y `not-between` expresan límites de forma legible.
16. `and` y `or` son palabras, no símbolos.
17. Mezclar `and` y `or` requiere paréntesis explícitos.
18. El lenguaje prioriza intención visible sobre precedencia implícita.

## Fuera de alcance

LR-002 no define:

- la gramática completa de Evo Shell;
- la implementación interna;
- el modelo de tipos Rust;
- la ejecución concreta de `index`, `select`, `to-value`, `to-values`, `to-args`, `copy-to` o `filter`;
- la semántica avanzada de filtrado;
- los detalles de errores concretos para casos futuros;
- el sistema completo de pipelines.
