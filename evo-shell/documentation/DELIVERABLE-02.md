# Evo Shell — Deliverable 02

> **Structured pipelines, expression composition and filesystem copy**

Este documento constituye el **informe formal de cierre del Entregable 02** de Evo Shell y funciona como el **manual de usuario consolidado** de todas las capacidades funcionales y técnicas implementadas hasta la fecha.

---

## SECCIÓN 1 — PROPÓSITO

El objetivo central del Entregable 02 de Evo Shell no fue simplemente agregar comandos independientes, sino construir una **infraestructura completa de pipelines estructurados y composición de expresiones**.

A lo largo de este entregable se han establecido las bases para:
- La iteración estructurada de elementos del sistema de archivos.
- El filtrado y proyección fuertemente tipados dentro de la pipeline.
- La evaluación prioritaria y composición de expresiones mediante paréntesis `(...)`.
- La conversión explícita de datos proyectados hacia valores escalares (`to-value`), colecciones (`to-values`) o secuencias de argumentos posicionales (`to-args`).
- El consumo real de expresiones agrupadas y expandidas como argumentos de comandos posicionales.
- La ejecución de operaciones sobre el sistema de archivos mediante el comando de copia recursiva `copy-to`.

---

## SECCIÓN 2 — CAPACIDADES DISPONIBLES

Resumen consolidado de todas las capacidades realmente disponibles en Evo Shell al cierre del Entregable 02:

| Categoría | Elemento / Sintaxis | Descripción |
|---|---|---|
| **Comandos de Ámbito y Navegación** | `scope-fs "<path>"` | Establece de forma explícita el `filesystem_scope` activo. |
| | `enter <path>` / `enter (...)` | Navega hacia un directorio hijo o padre alterando el scope activo. |
| **Comandos de Inspección y Control** | `iter` | Produce una iteración estructurada de las entradas del scope activo. |
| | `clear` | Limpia la pantalla completa del terminal (pantalla visible y scrollback). |
| | `exit` | Finaliza la sesión interactiva de Evo Shell. |
| **Operaciones de Pipeline** | `filter <expr>` | Filtra entradas estructuradas evaluando predicados lógicos. |
| | `select <prop1>, <prop2>` | Proyecta propiedades específicas manteniendo el formato estructurado. |
| | `index <N>` | Selecciona un único elemento por su índice cero-basado (0..N). |
| | `take <N>` | Limita el resultado del pipeline a un máximo de N elementos. |
| **Conversores de Salida** | `to-value` | Convierte una proyección de una sola fila y propiedad en un escalar. |
| | `to-values` | Convierte una proyección de una sola propiedad en una colección. |
| | `to-args` | Transforma una proyección en una secuencia de argumentos posicionales. |
| **Composición y Sintaxis** | `pipeline \|>` | Conecta etapas consecutivas en una sola línea. |
| | `pipeline multilínea (...)` | Permite continuar la entrada textual en múltiples líneas usando `\|>`. |
| | `grouping (...)` | Evalúa de forma prioritaria una expresión encerrada entre paréntesis. |
| | `nested grouping ((...))` | Permite anidar agrupaciones de expresiones recursivamente. |
| | `grouped as argument` | Pasa el resultado de una expresión agrupada como argumento de comando. |
| **Operaciones de Sistema de Archivos** | `copy-to <sources>, path: <dst>` | Copia archivos o directorios de forma recursiva hacia un destino. |

---

## SECCIÓN 3 — MODELO MENTAL

Evo Shell no es una shell tradicional basada en procesamiento de texto plano o piping de caracteres sin estructura (Unix string piping).

### Conceptos Fundamentales

1. **COMMAND (Comando):**
   Instrucción de nivel superior que interactúa con la shell o realiza operaciones de efectos laterales (e.g. `scope-fs`, `enter`, `clear`, `exit`, `copy-to`).

2. **PIPELINE STAGE (Etapa de Pipeline):**
   Operación de transformación de datos (e.g. `iter`, `filter`, `select`, `index`, `take`, `to-value`, `to-values`, `to-args`).

3. **GROUPED EXPRESSION (Expresión Agrupada `(...)`):**
   Evaluación prioritaria de una sub-expresión que devuelve un resultado de dominio tipado (`ExecutionResult` / `PipelineValue`).

4. **ARGUMENT (Argumento Posicional):**
   Valor (literal o derivado de expresión agrupada) que se entrega ordenadamente a un comando.

5. **NAMED ARGUMENT (Argumento Nombrado):**
   Opción requerida por un comando mediante la sintaxis `<key>: <value>` (e.g. `path: backup`).

6. **STRUCTURED VALUE (Valor Estructurado):**
   Durante la ejecución de un pipeline, las entidades de dominio (`FilesystemIteration`, `FilesystemEntry`, `ProjectedRow`, `ProjectedValue`) se transportan directamente en memoria como estructuras fuertemente tipadas sin serialización a texto.

---

## SECCIÓN 4 — SCOPE

El `filesystem_scope` representa la ubicación activa dentro del sistema de archivos.

- **Definición explícita:**
  ```text
  scope-fs "/home/user/documents"
  ```
- **Comportamiento:**
  - `iter` consulta las entradas contenidas en el `filesystem_scope` activo sin alterarlo.
  - `enter` resuelve la ruta destino respecto al `filesystem_scope` activo y lo reemplaza si el destino es un directorio válido.
  - `copy-to` resuelve rutas relativas respecto al `filesystem_scope` activo, pero **NO** modifica el scope activo al finalizar la copia.

---

## SECCIÓN 5 — ITER

El comando `iter` inicia la secuencia de iteración estructurada sobre el `filesystem_scope` activo.

```text
scope-fs …/evo-shell > iter
```

### Comportamiento

`iter` produce elementos de tipo `FilesystemIterationItem` que contienen:
- Índice (0-indexed)
- Nombre de la entrada
- Tipo (`File`, `Directory` o `Symlink`)
- Tamaño en bytes (si aplica)
- Fecha de creación y modificación

Presentación en pantalla:
```text
┌───┬─────────────┬───────────┬──────────┬────────────────────────┐
│ # │ name        │ type      │ size     │ created                │
├───┼─────────────┼───────────┼──────────┼────────────────────────┤
│ 0 │ Cargo.toml  │ File      │ 228 B    │ 2026-08-06 05:00:00    │
│ 1 │ src         │ Directory │ -        │ 2026-08-06 05:00:00    │
└───┴─────────────┴───────────┴──────────┴────────────────────────┘
```

---

## SECCIÓN 6 — FILTER

La etapa `filter` permite filtrar elementos estructurados según condiciones lógicas sobre sus propiedades.

### Sintaxis

```text
iter |> filter <propiedad> <operador> <valor>
```

### Operadores Soportados

- `equals`: Igualdad sintáctica o de valor.
- `not-equals`: Desigualdad.
- `>` / `less-than`: Mayor / menor numérico.
- `at-least` / `at-most`: Mayor o igual / menor o igual.
- `between <low> and <high>`: Rango inclusivo.
- `not-between <low> and <high>`: Fuera de rango.
- `and` / `or`: Operadores lógicos de combinación.

### Agrupación Lógica

Se permite agrupar predicados booleanos internos entre paréntesis:

```text
iter |> filter (name equals "a.txt" or name equals "b.txt") and size > 100b
```

---

## SECCIÓN 7 — SELECT

La etapa `select` proyecta una o varias propiedades específicas de los elementos iterados.

### Sintaxis

```text
iter |> select name
iter |> select name, type, size
```

### Propiedades Proyectables

- `name`: Nombre de la entrada.
- `type`: Tipo de entrada (`File`, `Directory`, `Symlink`).
- `size`: Tamaño de archivo en bytes.
- `created`: Fecha/hora de creación.
- `modified`: Fecha/hora de última modificación.
- `index`: Índice numérico de la fila.

---

## SECCIÓN 8 — INDEX

La etapa `index` selecciona un único elemento basado en su índice posicional cero-basado.

```text
iter |> index 0
```

Si el índice especificado no existe dentro de la colección, la shell retorna un error tipado de índice no encontrado (`IndexError::NotFound`).

---

## SECCIÓN 9 — TAKE

La etapa `take` limita la cantidad máxima de elementos que continuarán a través del pipeline.

```text
iter |> take 1
iter |> take 5
```

Si el valor especificado es 0, produce una colección vacía.

---

## SECCIÓN 10 — TO-VALUE

`to-value` convierte una proyección estructurada de una sola fila y una sola propiedad en un valor escalar estructurado `PipelineValue::Value(ProjectedValue)`.

```text
iter |> index 0 |> select name |> to-value
```

### Salida Observable

```text
Cargo.toml
```

Si la proyección contiene múltiples filas o múltiples propiedades, `to-value` rechaza la conversión con un error tipado (`ToValueError`).

---

## SECCIÓN 11 — TO-VALUES

`to-values` convierte una proyección estructurada de una sola propiedad en una colección ordenada de valores escalares `PipelineValue::Values(Values)`.

```text
iter |> select name |> to-values
```

### Salida Observable

```text
Cargo.toml
src
README.md
```

`to-values` conserva los elementos como colección de datos en memoria, pero **no** los formatea como argumentos para comandos.

---

## SECCIÓN 12 — TO-ARGS

`to-args` es la etapa encargada de transformar una proyección estructurada en una secuencia de **argumentos posicionales** (`PipelineValue::Arguments`).

```text
iter |> filter type equals "file" |> select name |> to-args
```

### Diferencia Crítica: `to-values` vs `to-args`

- `to-values`: Produce una colección orientada a la inspección o presentación de valores.
- `to-args`: Produce una estructura de transporte de argumentos posicionales que puede ser consumida directamente por comandos como `copy-to`.

`to-args` **no concatenan cadenas con comas**, **no genera texto sintético** y **no requiere re-parsing**.

---

## SECCIÓN 13 — PIPELINES

Un pipeline conecta la salida estructurada de una etapa con la entrada de la siguiente mediante el operador `|>`.

### Pipeline Simple (Una Línea)

```text
iter |> filter type equals "file" |> select name |> to-values
```

### Pipeline Multilínea

Si una línea de entrada finaliza con el separador de pipeline `|>`, Evo Shell reconoce que la instrucción está incompleta y solicita la continuación de la entrada con el prompt `... > `:

```text
scope-fs …/evo-shell > iter |>
... > filter type equals "file" |>
... > select name |>
... > to-value
```

> **Nota de Regla Léxica:** La continuación multilínea de pipeline se activa exclusivamente cuando la línea finaliza estructuralmente en `|>`.

---

## SECCIÓN 14 — GROUPING

La sintaxis de paréntesis `(...)` permite agrupar una expresión para evaluar prioritariamente su contenido y obtener su resultado tipado.

### Ejemplo

```text
(iter |> take 1 |> select name |> to-value)
```

### Agrupación Anidada (`Nested Grouping`)

Evo Shell soporta la anidación recursiva de expresiones agrupadas:

```text
((iter |> take 1 |> select name |> to-value))
```

Cada nivel de paréntesis resuelve su balance interno y devuelve control al contexto contenedor sin exigir EOF prematuro.

---

## SECCIÓN 15 — EXPRESIÓN COMO ARGUMENTO

Una expresión agrupada `(...)` puede utilizarse directamente en la posición del argumento de un comando posicional:

```text
enter (
    iter
    |> filter type equals "directory"
    |> filter name equals "src"
    |> select name
    |> to-value
)
```

### Flujo de Ejecución

1. Se evalúa prioritariamente la expresión interior contenida en `(...)`.
2. Se obtiene el `PipelineValue` resultante de la sub-expresión.
3. Se realiza la validación estricta de compatibilidad semántica (para `enter`, se acepta exclusivamente `ProjectedValue::Name`).
4. El resultado intermedio de la sub-expresión **NO** se imprime en stdout.
5. El comando principal (`enter`) se ejecuta con la ruta resuelta.

---

## SECCIÓN 16 — COPY-TO

El comando `copy-to` realiza la copia de archivos y directorios desde rutas fuente hacia un directorio destino.

### Sintaxis

1. **Copia de un solo archivo:**
   ```text
   copy-to README.md, path: backup
   ```
2. **Copia de múltiples elementos:**
   ```text
   copy-to a.txt, b.txt, documents, path: backup
   ```
3. **Copia recursiva de directorios:**
   ```text
   copy-to documents, path: backup
   ```

### Reglas Operativas de `copy-to`

- **Rutas Relativas:** Se resuelven con respecto al `filesystem_scope` activo.
- **Invariante de Scope:** `copy-to` **NO** altera el `filesystem_scope` activo.
- **Destino Obligatorio:** Se especifica mediante la opción nombrada `path: <directorio>`. El destino debe existir y debe ser un directorio.
- **Sin Sobrescritura Silenciosa:** Si la ubicación destino ya contiene un archivo o directorio con el mismo nombre, retorna un error tipado (`DestinationAlreadyExists`).
- **Rechazo de Self-Copy:** Copiar un directorio sobre sí mismo o dentro de una subcarpeta descendiente (e.g. `copy-to docs, path: docs/sub`) es rechazado con error tipado (`RecursiveSelfCopy`).
- **Symlinks:** Los enlaces simbólicos son rechazados con error tipado (`UnsupportedSourceType`) sin seguir el destino.
- **Fail-Fast:** La copia valida las condiciones y se detiene ante el primer fallo. No existe mecanismo de rollback.

---

## SECCIÓN 17 — COPY-TO + TO-ARGS

La integración de `copy-to` con `to-args` constituye la demostración central del Entregable 02, permitiendo que la salida de un pipeline filtrado expanda sus argumentos posicionales hacia el comando de copia.

```text
copy-to (
    iter
    |> filter type equals "file"
    |> select name
    |> to-args
), path: backup
```

### Desglose de Ejecución

```text
1. iter
   └─► Recupera las entradas del scope activo de forma estructurada.

2. filter type equals "file"
   └─► Conserva únicamente los archivos regulares.

3. select name
   └─► Proyecta la propiedad 'name'.

4. to-args
   └─► Encapsula las filas proyectadas en un 'PipelineValue::Arguments'.

5. (...)
   └─► Evalúa la expresión agrupada y retorna la secuencia de argumentos.

6. copy-to ..., path: backup
   └─► Recibe los argumentos posicionales expandidos y ejecuta la copia recursiva en el motor.
```

---

## SECCIÓN 18 — EJEMPLOS PRÁCTICOS

### Ejemplo A: Listar solo directorios

```text
iter |> filter type equals "directory"
```

### Ejemplo B: Obtener el nombre del archivo más antiguo / primero

```text
iter |> filter type equals "file" |> index 0 |> select name |> to-value
```

### Ejemplo C: Entrar al directorio `src` dinámicamente

```text
enter (iter |> filter name equals "src" |> select name |> to-value)
```

### Ejemplo D: Copiar un archivo específico a una carpeta de respaldo

```text
copy-to Cargo.toml, path: backup
```

### Ejemplo E: Copiar todo un árbol de directorios

```text
copy-to docs, path: backup
```

### Ejemplo F: Copiar todos los archivos filtrados por tipo mediante `to-args`

```text
copy-to (
    iter
    |> filter type equals "file"
    |> select name
    |> to-args
), path: backup
```

### Ejemplo G: Copiar un directorio específico filtrado dinámicamente

```text
copy-to (
    iter
    |> filter type equals "directory"
    |> filter name equals "docs"
    |> select name
    |> to-args
), path: backup
```

---

## SECCIÓN 19 — ERRORES Y LIMITACIONES

Evo Shell reporta errores tipados comprensibles ante situaciones anómalas:

- **Origen no encontrado (`SourceNotFound`):** Cuando la ruta fuente especificada no existe.
- **Destino no encontrado (`DestinationNotFound`):** Cuando la ruta destino indicada en `path:` no existe.
- **Destino no es directorio (`DestinationNotDirectory`):** Cuando la ruta en `path:` apunta a un archivo regular.
- **Target existente (`DestinationAlreadyExists`):** Cuando el archivo/directorio ya existe dentro del destino.
- **Copia circular (`RecursiveSelfCopy`):** Intento de copiar un directorio dentro de sí mismo.
- **Symlink no soportado (`UnsupportedSourceType`):** Intento de copiar un enlace simbólico.
- **Argumento incompatible (`IncompatibleGroupedArgument`):** Pasar tipos no escalares o escalares no representables como ruta (e.g. `Index`, `Size`, `Type`) a `enter` o `copy-to`.
- **Falta de fuente (`MissingSource`):** `copy-to` ejecutado sin fuentes o cuando `to-args` produce 0 elementos.

---

## SECCIÓN 20 — QUÉ NO EXISTE TODAVÍA

Para definir con precisión la frontera del Entregable 02, se aclara que **NO forman parte de la versión actual**:

- Comando `move-to` (mover elementos).
- Comando `delete` / `rm` (eliminar elementos).
- Comando `rename` (renombrar elementos).
- Opciones de sobrescritura (`--force` / `--overwrite`).
- Mecanismo de rollback o transacciones en copias.
- Barra o indicador de progreso en copias.
- Preservación especial de atributos extendidos o ACLs.

---

## SECCIÓN 21 — ARQUITECTURA RESUMIDA

La arquitectura mantiene una estricta separación de responsabilidades:

```text
                       EVO SHELL (Frontend/CLI)
  ┌──────────────────────────────────────────────────────────────┐
  │ Tokenizer  ─►  Parser Agent  ─►  Command Resolvers           │
  │                                           │                  │
  │ Pipeline Executor  ◄──  Executor Agent ◄──┘                  │
  └─────────────────────────────┬────────────────────────────────┘
                                │ Invoca Use Cases
                                ▼
                    EVO SHELL ENGINE (Backend)
  ┌──────────────────────────────────────────────────────────────┐
  │ Use Cases: SetFilesystemScope | Iter | Enter | Copy | ...    │
  │ Agents:    scope_setter | iterator | enterer | copier | ...   │
  │ Providers: std::fs (read_dir, copy, create_dir, metadata)    │
  └──────────────────────────────────────────────────────────────┘
```

---

## SECCIÓN 22 — CRITERIO DE CIERRE

El criterio técnico y funcional que marca la culminación del Entregable 02 se satisface mediante la ejecución exitosa de la instrucción:

```text
copy-to (
    iter
    |> filter type equals "file"
    |> select name
    |> to-args
), path: backup
```

Esta instrucción valida la integración completa de:
`Iteración estructurada ─► Filtrado ─► Proyección ─► Expansión a to-args ─► Evaluación agrupada ─► Opción nombrada path: ─► Operación real de copia en el motor`.

---

## SECCIÓN 23 — ESTADO FINAL

```text
Deliverable: 02
Status: COMPLETE

Closing Commit:
56c82af90aa3bef219dc92af2a6cc2b9c09ae06e
feat: add recursive copy-to command
```

Este commit constituye la frontera funcional y técnica del Entregable 02.
