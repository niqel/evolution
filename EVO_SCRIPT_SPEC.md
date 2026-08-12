# Especificación Semántica y Sintáctica de Evo-Script

Este documento especifica la arquitectura, reglas semánticas y sintaxis oficial de **Evo-Script**, el lenguaje de programación, automatización y consulta de Evolution.

---

## 1. Visión General y Principios de Diseño

Evolution establece una separación rígida de responsabilidades entre el lenguaje y las capacidades del entorno:

* **Evo-Script es dueño del lenguaje**: Responsable de la sintaxis, tokenización, parsing, sistema de tipos, operadores, evaluación de expresiones, predicados (`equals`, `between`, `contains`), conectivos lógicos (`and`, `or`), proyectores (`select`), construcción de campos (`new`), transformaciones (`append`, `take`), conversión a escalares (`to-value`), tuberías (`|>`) y la semántica lazy de iteración.
* **Evo-Shell expone capacidades del sistema**: Responsable del contexto de trabajo (`Scope`), operaciones de sistema de archivos (`copy-to`, `move-to`, `rename`, `delete`, `trash`), terminal, procesos y red.

---

## 2. Ejemplo Canónico de Pipeline y Expresiones

El siguiente script ilustra el modelo sintáctico y semántico oficial de Evo-Script:

```text
scope ../documents>

filter size equals(85)
    and (
        modified between(date_start, date_end)
        or name contains("report")
    )
|> select name,
          size,
          modified,
          new full-name(
              "Melendez Villarreal "
              |> append(name |> to-value)
          )
|> take(5)
|> iter
|> print
```

### Explicación Paso por Paso del Ejemplo Canónico:

1. **`scope ../documents>`**: Establece `../documents` como el contexto de trabajo activo (*current working context*).
2. **`filter ...`**: Aplica un filtro lazy que requiere que `size` sea igual a `85` **Y** que simultáneamente se cumpla la agrupación entre paréntesis: que `modified` esté entre las fechas `date_start` y `date_end`, **O** que `name` contenga el texto `"report"`.
3. **`select ...`**: Proyecta los campos `name`, `size` y `modified` para cada registro.
4. **`new full-name(...)`**: Declara explícitamente un nuevo campo calculado llamado `full-name`.
5. **`name |> to-value`**: Extrae el valor escalar del campo `name`.
6. **`"Melendez Villarreal " |> append(...)`**: Pasa como entrada implícita la cadena `"Melendez Villarreal "` a la transformación `append`, la cual concatena el escalar extraído de `name`.
7. **`|> take(5)`**: Limita el flujo de forma lazy a los primeros 5 elementos.
8. **`|> iter`**: Emite e itera los registros transformados elemento a elemento (*item by item*).
9. **`|> print`**: Consume cada elemento emitido e imprime su contenido.

---

## 3. Reglas Sintácticas Generales

### A. Argumentos Siempre Delimitados con Paréntesis `()`

Toda operación que reciba argumentos explícitos escritos en el código debe utilizar **siempre** paréntesis.

* **Correcto**:
  ```text
  equals(85)
  between(date_start, date_end)
  contains("report")
  append(value)
  take(5)
  range(1, 10)
  ```
* **Inválido / No permitido**:
  ```text
  equals 85
  between date_start, date_end
  contains "report"
  take 5
  ```

### B. Uso de Comas

Las comas `,` se utilizan exclusivamente:
1. Para separar argumentos dentro de una lista delimitada por paréntesis: `between(date_start, date_end)`, `range(1, 10)`.
2. Para separar los campos proyectados dentro de una declaración `select`: `select name, size, modified`.

Las comas **NO** representan operadores lógicos. Nunca debe escribirse `filter A, B` queriendo significar `A and B`. Las condiciones lógicas se combinan explícitamente mediante `and` y `or`.

---

## 4. Predicados y Operadores Lógicos (`and` / `or`)

### A. Estructura de un Predicado

Un predicado evalúa una condición sobre un sujeto y produce conceptualmente un valor booleano (`bool`).

```text
subject predicate(arguments)
```

* **Ejemplos**:
  ```text
  size equals(85)
  modified between(date_start, date_end)
  name contains("report")
  ```
* **Descomposición conceptual**:
  - `subject`: `size`
  - `predicate`: `equals`
  - `argument`: `85`
  - `result`: `bool`

*Nota*: Los predicados como `equals`, `between` y `contains` son semántica propia de Evo-Script y no llamadas a Evo-Shell. La sintaxis oficial es la forma declarativa `sujeto predicado(argumentos)`, no funciones infix como `equals(size, 85)`.

### B. Conectivos Lógicos `and` y `or`

* `and`: Conjunción lógica.
* `or`: Disyunción lógica.

Es válido encadenar múltiples condiciones usando exclusivamente `and`:
```text
filter size equals(85)
    and ext equals("txt")
    and name contains("report")
```

O encadenar múltiples condiciones usando exclusivamente `or`:
```text
filter ext equals("txt")
    or ext equals("md")
    or ext equals("evo")
```

### C. Regla Estricta de Precedencia Lógica (Agrupación Obligatoria)

> **Regla de Diseño**: Evo-Script **NO** define prioridad de precedencia implícita entre `and` y `or`.

Cuando una expresión combina operadores `and` y `or`, la prioridad debe declararse **obligatoriamente mediante paréntesis**.

* **Válido (Conjunción con Disyunción agrupada)**:
  ```text
  filter size equals(85)
      and (
          modified between(date_start, date_end)
          or name contains("report")
      )
  ```
  *Semántica*: `A and (B or C)`

* **Válido (Disyunción con Conjunción agrupada)**:
  ```text
  filter (
          size equals(85)
          and modified between(date_start, date_end)
      )
      or name contains("report")
  ```
  *Semántica*: `(A and B) or C`

* **Inválido / Ambiguo (Prohibido)**:
  ```text
  filter A and B or C
  filter A or B and C
  ```

---

## 5. Operaciones de Tubería (`|>`) y Distinción Predicado vs Transformación

### A. Diferencia entre Predicado y Transformación

* **Predicado**: Evalúa una condición sobre un sujeto y produce `bool`.
  ```text
  subject predicate(arguments)  → bool
  ```
* **Transformación**: Modifica, proyecta o procesa una entrada enviada a través de la tubería `|>`. Produce un nuevo valor o flujo transformado.
  ```text
  value |> transformation
  value |> transformation(arguments)
  ```

### B. Tuberías Sin Argumentos Extra

Una operación que no requiere argumentos explícitos adicionales y consume únicamente la entrada del pipe **NO utiliza paréntesis**:

```text
name |> to-value
items |> iter
items |> print
```

* **Forma correcta**: `|> to-value`, `|> iter`, `|> print`
* **Forma incorrecta**: `|> to-value()`, `|> iter()`, `|> print()`

### C. Tuberías Con Argumentos Extra

Si una operación consume la entrada del pipe Y ADEMÁS requiere argumentos explícitos adicionales, dichos argumentos **deben encerrarse entre paréntesis**:

```text
items |> take(5)

"Melendez Villarreal " |> append(name |> to-value)
```

### D. Regla Oficial de `append`

`append` es una **TRANSFORMACIÓN**, no un predicado.

* **Sintaxis Oficial**:
  ```text
  "Melendez Villarreal "
  |> append(name |> to-value)
  ```
* **Entrada implícita del pipe**: `"Melendez Villarreal "`
* **Argumento explícito**: `name |> to-value` (que evalúa al escalar `"Gustavo"`)
* **Resultado**: `"Melendez Villarreal Gustavo"`

---

## 6. Predicados Específicos y Construcciones

### A. `between(lower, upper)`

`between` es un **PREDICADO** que comprueba pertenencia a un intervalo y devuelve `bool`.

```text
subject between(lower, upper) → bool
```

* **Ejemplo**:
  ```text
  modified between(date_start, date_end)
  size between(100, 5000)
  ```
* *(Nota: La semántica detallada de límites inclusivos/exclusivos o rangos abiertos se definirá en especificaciones posteriores).*

### B. `range(from, to)`

`range` es una **CONSTRUCCIÓN** que genera un rango de elementos. **NO es un predicado** y no devuelve `bool`.

```text
range(from, to) → Range
```

* **Ejemplo**:
  ```text
  range(1, 10)
  |> iter
  |> print
  ```
* **Diferencia clave**:
  - `size between(1, 10)` $\rightarrow$ evalúa si `size` está en el rango $\rightarrow$ produce `bool`.
  - `range(1, 10)` $\rightarrow$ genera una secuencia/rango $\rightarrow$ produce objeto `Range`.
* *(Nota: La semántica inclusiva/exclusiva y representación interna de `Range` se definirán en posteriores commits).*

### C. `take(count)`

`take` es una transformación de flujo lazy que limita la salida a los primeros `count` elementos.

```text
stream |> take(5)
```

---

## 7. Proyección y Creación de Campos (`new`)

Para crear un nuevo campo derivado en una proyección `select`, se requiere la sintaxis explícita `new`:

```text
new nombre-campo(expresión)
```

* **Ejemplo**:
  ```text
  new full-name(
      "Melendez Villarreal "
      |> append(name |> to-value)
  )
  ```
* No se admite la palabra clave `as` ni la creación de campos calculados anónimos/sin nombre.

---

## 8. Semántica del Pipeline de Ejecución

### A. Contexto Activo (`Scope`)

La instrucción `scope ../documents>` establece el contexto de trabajo activo (*current working context*).

```text
scope
  ↓
current working context
```

* `Scope` pertenece al entorno persistente de ejecución y no está restringido al sistema de archivos.

### B. Filtrado Lazy (`filter`)

La declaración `filter ...` define una transformación condicional lazy. No lee todo el origen ni asigna memoria masiva al declararse.

### C. Distinción `Selection != Scalar` y Operador `to-value`

Seleccionar un campo (`select size`) produce una estructura proyectada. Para obtener el valor escalar usable en expresiones o transformaciones se exige la extracción explícita:

```text
size |> to-value
```

### D. Iteración (`iter`) y Consumidor (`print`)

* **`iter`**: Emite e itera elementos uno a uno (*item by item*). No realiza presentación gráfica ni materialización.
* **`print`**: Consumidor final independiente. Puede consumir elementos de `iter` o escalares directos (`print "Gustavo"`).

---

## 9. Separación Clara Evo-Script vs Evo-Shell

```text
                   Evo-Script
────────────────────────────────────────
language semantics
syntax
types
operators (+ - * / %)
predicates (equals, between, contains)
logical connectors (and, or, grouping)
expressions
filter / select / new
to-value / append / take / range
pipes (|>)
lazy iteration semantics (iter)

                     │
                     │ uses capabilities
                     ▼

                  Evo-Shell
────────────────────────────────────────
system capabilities
scope
filesystem
terminal
process
network
copy / move / rename / delete / trash
etc.
```
