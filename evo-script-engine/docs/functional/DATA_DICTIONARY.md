# Evo-Script Engine v0 — Data Dictionary Funcional

Status: FUNCTIONAL CLOSED

Este documento consolida el vocabulario arquitectónico y funcional canónico
para `evo-script-engine` v0, derivado de las User Stories cerradas US-001 a
US-003 y de la Evo-Script Language Specification v0.

---

## 1. Propósito y Alcance

El propósito de este Data Dictionary es definir formalmente los conceptos de datos
canónicos, roles y fronteras utilizados por `evo-script-engine` v0.

El Evo-Script Engine proporciona tres operaciones funcionales públicas distintas:

```text
1. Compile
   Source Text ──► Compiled Program (en compilación exitosa)

2. Execute Compiled
   Compiled Program + Invocation Values (0..N) ──► Result

3. Execute Source
   Source Text + Invocation Values (0..N) ──► Result
```

Este documento establece definiciones y restricciones funcionales.
Deliberadamente **no** define structs, enums, traits, genéricos, layouts de
memoria ni formatos binarios concretos de Rust.

---

## 2. Boundary Data

### Source Text

- **Categoría**: Boundary Input Data
- **Definición**: La representación textual completa de exactamente un
  Evo-Script Program v0 suministrado al Evo-Script Engine.
- **Características**:
  - Representa un Evo-Script Program completo y autocontenido.
  - Cuando se persiste externamente como artefacto de archivo fuente, Evo-Script
    v0 utiliza la extensión de archivo canónica `.efn`.
  - El Engine recibe el contenido textual plano, **no** un archivo físico o ruta.
  - `Source Text != File Path` (el Engine no realiza resolución de rutas ni I/O
    de sistema de archivos).
  - `Source Text != AST / Token Stream` (el Consumer suministra texto plano sin
    preprocesar).
  - `Source Text != Compiled Program` (Source Text es código fuente sin
    compilar).
  - `Source Text != Individual Function` (contiene la unidad completa del
    programa).
  - El Consumer es el único responsable de leer u obtener el texto desde el
    almacenamiento externo antes de invocar el Engine.
- **Cardinalidad**:
  - Exactamente 1 `Source Text` por invocación de `Compile`.
  - Exactamente 1 `Source Text` por invocación de `Execute Source`.
- **Fuentes**: US-001, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Compiled Program

- **Categoría**: Boundary Data (Output de Compile / Input de Execute Compiled)
- **Definición**: La representación ejecutable producida por el Engine de un
  Evo-Script Program que ha sido procesado exitosamente conforme a la
  Evo-Script Language Specification v0 y es adecuada para su posterior ejecución
  por el Evo-Script Engine.
- **Características**:
  - `Compiled Program != Source Text` (representa un artefacto de compilación ya
    procesado y validado).
  - Producido únicamente como resultado exitoso de `Compile`.
  - Puede suministrarse posteriormente como input a `Execute Compiled`.
  - Reutilizable a lo largo de múltiples invocaciones independientes de
    `Execute Compiled`.
  - La persistencia, caché y almacenamiento pertenecen al Consumer o a
    componentes externos; el Engine no administra almacenamiento físico ni
    registros de programas en v0.
  - Su representación técnica interna permanece **abierta** (por ejemplo,
    bytecode, IR, AST validado o flujo binario son candidatos técnicos
    diferidos al diseño técnico).
  - No se asigna ninguna extensión de archivo a Compiled Program en v0.
- **Garantía de Ubicación Fuente (Source Location Guarantee)**:
  - Un `Compiled Program` debe preservar suficiente relación con el `Source Text`
    original para que, cuando ocurra un fallo de ejecución originado en el
    programa, el Evo-Script Engine pueda identificar la línea fuente
    correspondiente durante `Execute Compiled`.
- **Cardinalidad**:
  - Producido en invocaciones exitosas de `Compile` (0..1 por compilación).
  - Exactamente 1 `Compiled Program` por invocación de `Execute Compiled`.
- **Fuentes**: US-001, US-002

### Invocation Values

- **Categoría**: Boundary Input Data
- **Definición**: Los Values ordenados suministrados por el Consumer para los
  parámetros declarados por la única Public Function del programa.
- **Características**:
  - Cardinalidad: `0..N` Values.
  - Una Public Function con cero parámetros requiere cero Invocation Values.
  - Una Public Function con $N$ parámetros requiere exactamente $N$ Invocation
    Values.
  - El mapeo a parámetros es estrictamente posicional:
    ```text
    InvocationValue[0]     ──► Parameter[0]
    InvocationValue[1]     ──► Parameter[1]
    ...
    InvocationValue[N - 1] ──► Parameter[N - 1]
    ```
  - El orden de los Invocation Values corresponde directamente al orden de
    declaración de los parámetros en la firma de la Public Function.
  - Cada Invocation Value debe ser semánticamente compatible con su tipo de
    parámetro correspondiente (incluyendo tipos nativos, structs y enums
    definidos por el programa).
  - El Engine no realiza conversiones ni coerciones implícitas.
  - Utilizado por `Execute Compiled` y `Execute Source`.
  - **No** utilizado por `Compile`.
  - `Invocation Values != Command-Line Strings` (representa valores de datos
    estructurados, no argumentos de texto plano de terminal).
- **Fuentes**: US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Result

- **Categoría**: Boundary Output Data
- **Definición**: El outcome funcional de una operación de ejecución realizada
  por el Evo-Script Engine.
- **Características**:
  - Representa el outcome completado de `Execute Source` o `Execute Compiled`.
  - **No** asignado como outcome de `Compile` en esta fase funcional.
  - Estructura conceptual:
    ```text
    Result
    ├── success ──► preserva el Value producido
    └── failure ──► expresa Failure
    ```
  - `Result != Value`
  - `Result != Failure`
  - Alineado con el modelo compartido de outcomes de `evo-values`.
  - No asume genéricos de Rust concretos (`Result<T, E>`) ni
    `std::result::Result`.
- **Cardinalidad**:
  - Exactamente 1 `Result` por invocación completada de `Execute Source`.
  - Exactamente 1 `Result` por invocación completada de `Execute Compiled`.
- **Fuentes**: US-002, US-003

---

## 3. Shared Data Concepts

### Value

- **Categoría**: Shared Data Concept
- **Definición**: El concepto de datos compartido de Evo utilizado para
  transportar valores semánticos entre el Consumer, los parámetros de función y
  los outcomes de ejecución.
- **Características**:
  - Pertenece conceptualmente al modelo de datos compartido `evo-values`.
  - `Invocation Values` contiene `0..N` Values.
  - Un `Result` exitoso preserva el `Value` producido por la Public Function.
  - Los tipos de valores concretos soportados están definidos por la
    Evo-Script Language Specification v0 (primitivos, structs, enums).
  - Este Data Dictionary no redefine la semántica interna completa ni el layout
    de memoria de Value.
  - No introduce semántica de ownership, lifetimes o punteros inteligentes de
    Rust.
- **Fuentes**: US-002, US-003, `evo-values`, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Failure

- **Categoría**: Shared Outcome Concept
- **Definición**: El diagnóstico funcional mínimo que describe por qué el
  procesamiento o ejecución de Evo-Script no concluyó exitosamente.
- **Características**:
  - Concepto compartido único en todo el Engine; sin conceptos separados de
    `Error`, `CompileError` o `ExecutionError`.
  - Elementos de datos mínimos en v0:
    ```text
    Failure
    ├── description: exactly 1
    └── source line: 0..1 (número de línea 1-based en Source Text, si aplica)
    ```
  - **Description**: Explicación textual del fallo (siempre presente).
  - **Source Line**: Índice de línea 1-based (`line 1` = primera línea de
    Source Text) asociado al fallo cuando existe una ubicación fuente.
  - **Presencia de Source Line**:
    - *Requerida* para fallos léxicos, sintácticos, semánticos y de evaluación
      en runtime originados en el programa.
    - *Ausente* para fallos de frontera de invocación que no corresponden a una
      línea interna del programa (por ejemplo, desajuste de aridad o
      incompatibilidad de tipos en la frontera). No se generan líneas
      artificiales (como línea 0).
  - **Relación con Compile**: Una compilación fallida produce un diagnóstico
    Failure (el wrapper técnico o mecanismo de retorno para Compile permanece
    abierto).
  - **Relación con Ejecución**: Un `Result` fallido de `Execute Source` o
    `Execute Compiled` expresa un Failure.
  - **Excluido en v0**: No exige columnas, códigos de error, categorías,
    severidades, stack traces, spans ni byte offsets.
- **Fuentes**: US-001, US-002, US-003

---

## 4. Referenced Evo-Script Language Concepts

### Evo-Script Program

- **Categoría**: Language Domain Concept
- **Definición**: La unidad de programa completa y autocontenida definida por
  la Evo-Script Language Specification v0.
- **Características**:
  - Contenida enteramente dentro de un único archivo fuente (`.efn`).
  - Representada textualmente por `Source Text`.
  - Representada ejecutablemente por un `Compiled Program` tras una compilación
    exitosa.
  - Declara exactamente 1 Public Function (`public fn`).
  - Puede declarar `0..N` funciones privadas, structs y enums locales al
    archivo.
  - No representa un struct técnico de Rust.
- **Fuentes**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Public Function

- **Categoría**: Language Domain Concept
- **Definición**: La única función pública de entrada declarada por un
  Evo-Script Program v0 (`public fn`) y ejecutada durante las operaciones de
  ejecución.
- **Características**:
  - Exactamente 1 por Evo-Script Program v0.
  - Declara `0..N` Parameters.
  - Recibe Invocation Values mediante binding posicional estricto.
  - Evalúa expresiones y produce un Value conforme a Evo-Script v0.
  - `Public Function != main` (no implica punto de entrada de proceso del SO).
  - `Public Function != Runtime startup / Run` (independiente de Evo Runtime).
- **Fuentes**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

### Parameter

- **Categoría**: Language Domain Concept
- **Definición**: Un parámetro formal tipado declarado en la firma de la Public
  Function.
- **Características**:
  - Cardinalidad: `0..N` por Public Function.
  - Posee una posición declarada y un tipo declarado conforme a Evo-Script v0.
  - Recibe exactamente un Invocation Value correspondiente a su índice de
    declaración durante una ejecución válida.
  - En v0, los parámetros son bindings inmutables; no hay argumentos nombrados,
    valores por defecto, parámetros opcionales, parámetros variádicos ni
    modificadores de referencia/mutabilidad.
- **Fuentes**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

---

## 5. Roles y Componentes

### Consumer

- **Categoría**: External Functional Role
- **Definición**: El invocador externo o componente del sistema que utiliza las
  capacidades públicas del Evo-Script Engine.
- **Responsabilidades**:
  - Suministra `Source Text` a `Compile` o `Execute Source`.
  - Suministra `Compiled Program` a `Execute Compiled`.
  - Suministra `Invocation Values` a `Execute Source` o `Execute Compiled`.
  - Recibe un `Compiled Program` tras un `Compile` exitoso.
  - Recibe un `Result` tras completar `Execute Source` o `Execute Compiled`.
  - Administra la lectura externa de archivos, persistencia o caché de
    programas si lo requiere.
- **Invariantes / Distinciones**:
  - `Consumer` es un rol funcional (por ejemplo, CLI, runner, test suite o
    aplicación anfitriona) y no es una estructura de datos técnica.
- **Fuentes**: US-001, US-002, US-003

### Evo-Script Engine

- **Categoría**: Core Engine / Component
- **Definición**: El componente de la plataforma que implementa las reglas de
  compilación y ejecución de la Evo-Script Language Specification v0.
- **Capacidades Públicas**:
  1. **`Compile`**: `Source Text` $\longrightarrow$ `Compiled Program`
  2. **`Execute Source`**: `Source Text` $+$ `Invocation Values` $\longrightarrow$ `Result`
  3. **`Execute Compiled`**: `Compiled Program` $+$ `Invocation Values` $\longrightarrow$ `Result`
- **Invariantes / Distinciones**:
  - `Evo-Script Engine != Evo Runtime` (no coordina aplicaciones, no administra
    providers ni maneja Runtime Start/Run).
  - No administra I/O de terminal, UI, descubrimiento de sistema de archivos ni
    efectos laterales en v0.
- **Fuentes**: US-001, US-002, US-003, `EVO_SCRIPT_SPECIFICATION_v0.md`

---

## 6. Relaciones Canónicas

Las relaciones canónicas a través de los términos del vocabulario se resumen a
continuación:

```text
Source Text
  └── represents ──────────────────────────► Evo-Script Program

Compiled Program
  └── represents (executable) ─────────────► Successfully compiled Evo-Script Program
  └── preserves source-line relationship ──► Source Text (for failure reporting)

Evo-Script Program
  └── declares ────────────────────────────► Exactly 1 Public Function

Public Function
  └── declares ────────────────────────────► 0..N Parameters

Invocation Values
  └── contains ────────────────────────────► 0..N Values
  └── maps positionally (0..N-1) ──────────► Parameters (0..N-1)

Result
  ├── success branch ──────────────────────► Preserves produced Value
  └── failure branch ──────────────────────► Expresses Failure

Failure
  ├── description ─────────────────────────► Exactly 1 (mandatory textual description)
  └── source line ─────────────────────────► 0..1 (1-based, present when line exists)

Compile
  ├── consumes ────────────────────────────► Source Text
  ├── success outcome ─────────────────────► Compiled Program
  └── failure outcome ─────────────────────► Failure diagnostic available

Execute Source
  ├── consumes ────────────────────────────► Source Text
  ├── consumes ────────────────────────────► Invocation Values (0..N)
  └── produces ────────────────────────────► Result

Execute Compiled
  ├── consumes ────────────────────────────► Compiled Program
  ├── consumes ────────────────────────────► Invocation Values (0..N)
  └── produces ────────────────────────────► Result
```
