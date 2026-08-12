# Especificación Semántica de Evo-Script

Este documento especifica la arquitectura y reglas semánticas de **Evo-Script**, el lenguaje de programación, automatización y consulta de Evolution.

---

## 1. Visión General y Principios de Diseño

Evolution establece una separación rígida de responsabilidades entre el lenguaje y las capacidades del entorno:

* **Evo-Script es dueño del lenguaje**: Responsable de la sintaxis, tokenización, parsing, sistema de tipos, operadores, evaluación de expresiones, proyectores (`select`), filtros (`filter`), construcción de campos (`new`), conversión a escalares (`to-value`), tuberías (`|>`) y la semántica lazy de iteración.
* **Evo-Shell expone capacidades del sistema**: Responsable del contexto de trabajo (`Scope`), operaciones de sistema de archivos (`copy-to`, `move-to`, `rename`, `delete`, `trash`), terminal, procesos y red.

---

## 2. Ejemplo Canónico de Pipeline

El siguiente script ilustra el modelo semántico central de Evo-Script:

```text
scope ../documents>

filter ext equals "txt"
|> select name, size, new size_big(15 + (size |> to-value))
|> iter
|> print
```

---

## 3. Semántica del Pipeline de Ejecución

### A. Contexto Activo (`Scope`)

La instrucción `scope ../documents>` establece el contexto o ámbito de trabajo activo (*current working context*).

```text
scope
  ↓
current working context
```

* En este ejemplo, `../documents` define el origen de datos sobre el que operan las expresiones siguientes.
* **Diferencia conceptual con SQL (`FROM`)**: Mientras `FROM` debe especificarse en cada consulta individual, `Scope` pertenece al contexto persistente de ejecución. Establecido un `Scope`, las operaciones subsecuentes operan sobre él sin recalificar constantemente la ruta o recurso.
* `Scope` es una abstracción general y no está restringido exclusivamente a carpetas de sistema de archivos.

### B. Filtrado Lazy (`filter`)

La declaración `filter ext equals "txt"` define una transformación condicional lazy.

* **Propósito**: Filtra el flujo dejando pasar únicamente aquellos elementos cuya propiedad `ext` sea igual a `"txt"`.
* **Procesamiento Lazy**: `filter` **no materializa el Scope al declararse**. No lee el directorio completo, no crea colecciones intermediate ni asigna vectores en memoria. Simplemente añade una etapa de filtrado a la composición del pipeline.

```text
Scope → Filter(ext equals "txt")
```

### C. Proyección de Campos (`select`) y la Regla `new`

La expresión `|> select name, size, new size_big(15 + (size |> to-value))` define la forma proyectada de cada registro que fluye por el pipeline.

* Mantiene campos existentes del origen (`name`, `size`).
* **Regla de `new`**: Para crear un nuevo campo derivado en la proyección, se requiere la palabra clave explícita `new`:

```text
new nombre_campo(expresión)
```

```text
new
 ├── nombre de campo = size_big
 └── expresión de valor = 15 + (size |> to-value)
```

* **Intención de diseño**: Evitar alias ambiguos o implícitos (ej. `expresión as campo`). La creación de un nuevo campo debe ser explícita y proporcionar tanto el nombre del campo como la expresión delimitada que calcula su contenido. No se permiten campos proyectados sin nombre.
* `select` forma parte de la composición lazy y no fuerza la carga completa de datos.

### D. Distinción `Selection != Scalar` y Operador `to-value`

Evo-Script establece como regla formal que **seleccionar un campo no equivale automáticamente a obtener su valor escalar**.

```text
select size
```

* La selección de `size` produce conceptualmente una estructura/columna proyectada. Aun cuando una selección contenga 1 fila y 1 columna, sigue siendo una estructura de datos y no un valor numérico/escalar directo.
* Para realizar operaciones aritméticas o de comparación con valores literales, se requiere la transición explícita a escalar mediante `to-value`:

```text
size |> to-value
```

```text
selection / field
       ↓
   to-value
       ↓
 scalar value
```

* Esta regla preserva la distinción formal entre:
  1. **Colección** (*Collection*)
  2. **Registro** (*Row / Item*)
  3. **Selección / Campo** (*Column / Field Selection*)
  4. **Valor Escalar** (*Scalar Value*)

### E. Evaluación de Expresiones con Paréntesis

En la expresión `15 + (size |> to-value)`, los paréntesis establecen el orden de evaluación:

1. Evalúa la sub-expresión `size |> to-value` para extraer el valor escalar del campo `size`.
2. Suma el literal `15` al escalar resultante mediante el operador de adición de Evo-Script.

```text
size → to-value → scalar → 15 + scalar
```

### F. Iteración (`iter`) y Consumidor (`print`)

* **`iter` (Punto de Iteración)**: La instrucción `|> iter` habilita el recorrido e itineración secuencial elemento a elemento (*item by item*). `iter` **no tiene responsabilidad de presentación** (no implica terminal, pantalla, UI ni render) ni de materialización (`Vec` / `collect`).
* **`print` (Consumidor)**: La instrucción `|> print` actúa como un consumidor final que recibe los elementos emitidos por la iteración y los envía a la salida.
* **Separación `iter != print`**: `print` es un consumidor independiente. Si se le pasa un escalar directo (ej. `print "Gustavo"`), no requiere una tubería de iteración `iter`.

---

## 4. Operadores y Evaluación Aritmética en Evo-Script

Los operadores aritméticos (`+`, `-`, `*`, `/`, `%`) y lógicos son responsabilidad exclusiva de **Evo-Script**.

```text
Evo-Script
    ↓
interpreta la expresión sintáctica
    ↓
conoce los tipos concretos
    ↓
aplica el operador correspondiente
```

* `15 + 20` es interpretado y resuelto internamente por el motor de expresiones de Evo-Script.
* **Evo-Shell no recibe ni interpreta operadores sintácticos** como `"sum"`, `"+"`, `"divide"`. Evo-Shell solo expone capacidades semánticas de entorno y sistema.

---

## 5. Sistema de Tipos e Interoperabilidad con Rust

Evo-Script preserva tipos concretos compatibles directamente con los tipos nativos de Rust:

* **Enteros firmados**: `i8`, `i16`, `i32`, `i64`, `i128`
* **Enteros no firmados**: `u8`, `u16`, `u32`, `u64`, `u128`
* **Punto flotante**: `f32`, `f64`

### Aliases de Conveniencia (Azúcar Sintáctico):
Evo-Script proporciona dos alias básicos para simplicidad:
* `int` $\equiv$ `i32` (ejemplo: `let age: int = 25` equivale a usar `i32`).
* `float` $\equiv$ `f64` (ejemplo: `let value: float = 2.5` equivale a usar `f64`).

Usuarios que requieran precisión concreta (ej. `f32`, `u64`, `i8`) pueden especificar el tipo numérico directamente.

### Ausencia de `Number` Universal en la Arquitectura Objetivo:
* La interoperabilidad entre Evo-Script y Rust **no utiliza un enum universal `Number`** (`I8(...)`, `I32(...)`, `F64(...)`).
* La arquitectura objetivo conserva la identidad de tipos concretos de Rust.
* *(Nota: El tipo `Number` que residía temporalmente en `evo-shell` ha sido completamente removido de `evo-shell`).*
* Preservar tipos concretos permite que Evo-Script pueda interoperar de forma eficiente con motores y librerías de Rust fuertemente tipadas sin forzar empaquetado dinámico o conversiones universales.

---

## 6. Modelo de Ejecución Lazy Item por Item

Evo-Script no ejecuta pipelines mediante carga masiva en memoria (*read-all*) ni creación de vectores intermedios:

```text
source / provider
      ↓
   next item
      ↓
    filter
      ↓
    select
      ↓
     iter
      ↓
   consumer
```

### Flujo Traza por Elemento:
```text
item #1 ──► filter (pasa)      ──► select ──► iter ──► print
item #2 ──► filter (rechazado)
item #3 ──► filter (pasa)      ──► select ──► iter ──► print
```

---

## 7. Separación Clara Evo-Script vs Evo-Shell

```text
                   Evo-Script
────────────────────────────────────────
language semantics
syntax
types
operators
expressions
filter
select
new
to-value
pipes
lazy iteration semantics

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
copy / move / rename / delete
etc.
```
