# Evo-Script Specification v0.1

## 1. Propósito

Este documento define los principios, la semántica y los elementos
fundamentales de Evo-Script.

La especificación describe qué representa una construcción del lenguaje
y cómo debe comportarse, independientemente de la implementación concreta
del parser, intérprete o runtime.

Evo-Script se diseña en conjunto con Evo-Shell, pero ambos componentes
mantienen responsabilidades distintas:

- Evo-Script define el lenguaje, la composición y el procesamiento de datos.
- Evo-Shell define capacidades semánticas para interactuar con el entorno.
- Los Providers implementan las capacidades técnicas requeridas por Evo-Shell.

Esta especificación debe guiar posteriormente la implementación del lenguaje.

La implementación no debe definir retrospectivamente la semántica del lenguaje;
la semántica definida aquí debe guiar la implementación.


## 2. Descripción general

Evo-Script es un lenguaje funcional orientado a la composición,
consulta y transformación de datos.

Su modelo está inspirado en operaciones sobre conjuntos de datos,
álgebra relacional y pipelines funcionales.

El lenguaje está diseñado alrededor del flujo de valores y datos entre
operaciones, en lugar de secuencias imperativas de instrucciones.

Evo-Script utiliza como elementos fundamentales:

- valores
- funciones
- operaciones
- scopes
- registros
- flujos de datos
- pipelines
- módulos

Las operaciones se componen principalmente mediante pipelines:

    source
    |> operation
    |> operation
    |> consumer

El resultado producido por una operación puede convertirse en la entrada
de la siguiente operación.

Un principio general del lenguaje es:

    Los datos fluyen; las operaciones se componen.

Evo-Script no debe depender del conocimiento de tecnologías concretas.

Otro principio general es:

    Evo-Script no conoce tecnologías. Conoce datos y capacidades.


## 3. Objetivos de diseño

Evo-Script busca:

- favorecer programación funcional y composición
- mantener una sintaxis pequeña y legible
- representar operaciones sobre datos de forma declarativa
- permitir evaluación lazy cuando la operación lo permita
- evitar materializaciones innecesarias de colecciones completas
- evitar null como mecanismo de ausencia o error
- separar el lenguaje de las tecnologías y recursos externos
- permitir que nuevas capacidades sean proporcionadas por Evo-Shell y Providers
- evitar abstracciones orientadas a objetos
- evitar estructuras imperativas tradicionales cuando exista una
  representación funcional equivalente
- permitir composición entre distintas fuentes de datos y capacidades
- mantener separada la semántica del lenguaje de su implementación técnica


## 4. Paradigma

Evo-Script es un lenguaje funcional y declarativo.

El lenguaje no utiliza programación orientada a objetos como modelo
de composición.

Evo-Script no basa su flujo de control en estructuras imperativas
tradicionales como:

    for
    while

La transformación de colecciones o flujos se expresa mediante operaciones
funcionales.

Por ejemplo, en lugar de recorrer manualmente una colección,
se expresa una transformación:

    filter ...
    |> select ...
    |> take(...)
    |> iter

Los datos son el elemento que fluye entre las operaciones.

Las funciones transforman, consultan, consumen o producen datos.

Evo-Script favorece composición sobre mutación explícita y operaciones
declarativas sobre control imperativo.


## 5. Separación entre lenguaje y entorno

Evo-Script no implementa directamente acceso a:

- filesystem
- databases
- network
- processes
- operating system resources

Estas capacidades pertenecen a Evo-Shell.

Evo-Script expresa intención semántica.

Por ejemplo:

    use scope
    |> enter("documents")
    |> ...

Evo-Shell proporciona las capacidades semánticas necesarias para realizar
esas operaciones.

Los Providers realizan la interacción técnica con la tecnología concreta.

Conceptualmente:

    Evo-Script
        ↓
    Evo-Shell
        ↓
    Contract
        ↓
    Provider
        ↓
    External Technology

Evo-Script no debe necesitar conocer si una capacidad está implementada
mediante:

- Linux
- Windows
- PostgreSQL
- SQL Server
- HTTP
- almacenamiento remoto
- cualquier otra tecnología

La tecnología concreta pertenece al Provider.

Esto permite que nuevas tecnologías puedan incorporarse sin introducir
su semántica dentro del lenguaje.
