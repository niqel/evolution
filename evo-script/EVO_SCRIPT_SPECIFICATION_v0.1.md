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


## 6. Scopes y contexto de ejecución

### 6.1 Definición de Scope

El Scope es una pieza fundamental de Evo-Script.

Un Scope es un contexto semántico de ejecución que identifica un entorno
de operación, mantiene una ubicación cuando el contexto la requiere
y determina las capacidades disponibles para las operaciones de Evo-Script.

Un Scope no es:

- una ruta o path técnico
- una colección de datos
- un conjunto de resultados en memoria
- una conexión técnica o socket
- un objeto orientado a objetos
- un Provider
- una tecnología concreta

El Scope proporciona una frontera semántica entre el lenguaje y las
capacidades externas. Permite a Evo-Script operar dentro de un entorno
sin necesidad de conocer la tecnología concreta que lo implementa.


### 6.2 Contexto

El contexto identifica el entorno semántico dentro del cual opera Evo-Script.

Representa la naturaleza del dominio operativo en el que se ejecutan
las operaciones del lenguaje.

Ejemplos de contextos:

- filesystem
- terminal
- database
- network
- UI
- otros contextos futuros

Un Scope puede asumir distintos roles dentro de una composición:

- fuente de datos (produce datos para el flujo)
- destino de datos (consume datos del flujo)
- contexto de interacción (entrada y salida interactiva)
- combinación de estos roles


### 6.3 Ubicación

La ubicación representa la posición actual dentro del contexto cuando
dicho contexto posee una noción de navegación o jerarquía.

Ejemplos conceptuales:

- Filesystem: `/home/gustavo/documents`
- Database: `company/workers`

En los contextos que admiten navegación, el Scope mantiene el registro
de la posición actual dentro del espacio accesible.

Existen contextos donde una noción de ubicación navegable no es aplicable
o no es necesaria (como una terminal). En esos casos, el Scope opera sin
requerir una posición espacial o jerárquica.

Se distinguen tres conceptos:

- **Scope**: la representación del contexto semántico de ejecución.
- **Scope activo**: el Scope actualmente establecido para ejecutar operaciones.
- **Ubicación actual**: la posición relativa dentro del Scope activo, cuando aplica.


### 6.4 Capacidades

Las capacidades representan las operaciones que pueden realizarse dentro
de un contexto específico.

Cada contexto determina qué operaciones están disponibles semánticamente:

- **Filesystem** (ejemplos conceptuales):
  `enumerate`, `enter`, `create`, `copy`, `move`, `delete`
- **Terminal** (ejemplos conceptuales):
  `print`, `read`, `clear`
- **Database** (ejemplos conceptuales):
  `enter`, `query`, `insert`, `update`, `delete`
- **UI** (ejemplos conceptuales):
  `display`, `notify`, `dialog`

Estos ejemplos son ilustrativos y no constituyen una lista cerrada.

Las operaciones disponibles dependen directamente de las capacidades
proporcionadas por el contexto activo, y no de comprobaciones del lenguaje
basadas en nombres de tecnologías concretas.


### 6.5 Obtención de Scopes

Un Scope puede obtenerse mediante operaciones de creación o materialización
de contexto.

Ejemplos conceptuales:

    let documents = scope-fs("/documents")
    let terminal = scope-terminal

Estas operaciones retornan un Scope que encapsula el contexto y la
configuración inicial correspondiente, listo para ser utilizado como
entorno de ejecución.


### 6.6 Activación mediante `use`

La operación `use` es la instrucción semántica que establece un Scope
como el contexto activo de ejecución.

Ejemplo conceptual:

    let documents = scope-fs("/documents")

    use documents

La semántica de `use` implica:

- cambiar el Scope activo de ejecución
- hacer disponibles las capacidades correspondientes a ese contexto

`use` no realiza navegación interna dentro del Scope actual, no enumera
datos ni materializa elementos en memoria.


### 6.7 Navegación mediante `enter`

La operación `enter(target)` modifica la ubicación actual dentro del
Scope activo, siempre que el contexto soporte navegación.

Ejemplo conceptual:

    use documents
    |> enter("reports")

Al ejecutar `enter`, el contexto semántico permanece inalterado
(por ejemplo, filesystem continúa siendo filesystem); únicamente se
actualiza la posición relativa dentro de dicho contexto.

La distinción entre ambas operaciones es fundamental:

- `use` cambia el contexto activo (por ejemplo, de database a filesystem).
- `enter` cambia la ubicación dentro del contexto activo existente.


### 6.8 Scopes sin navegación

No todos los Scopes poseen una estructura jerárquica o navegable.

Por ejemplo, un Scope de tipo terminal (`scope-terminal`) proporciona
capacidades como `print`, `read` o `clear`, pero no requiere una noción
de ruta o navegación interna.

Por tanto, `enter` no es una operación universal ni obligatoria para
todos los Scopes, sino una capacidad específica de aquellos contextos
cuya semántica incluye navegación espacial o jerárquica.


### 6.9 Independencia tecnológica

Evo-Script no conoce las tecnologías concretas que implementan los Scopes.

- Un Scope de filesystem no impone conocimiento de Linux, Windows, ext4,
  NTFS ni APIs de sistema operativo.
- Un Scope de base de datos no impone conocimiento de PostgreSQL, SQL Server,
  SQLite u Oracle.
- Un Scope de terminal no impone conocimiento de stdout, secuencias ANSI,
  Windows Console, Wayland o X11.

La relación arquitectónica conceptual se define en capas:

    Provider
        ↓
    Evo-Shell
        ↓
    Scope / capacidades
        ↓
    Evo-Script

Evo-Shell expone las capacidades semánticas y los Providers realizan la
interacción técnica concreta con el entorno externo.


### 6.10 Scope y flujo de datos

Un Scope no es un contenedor de resultados ni una colección de datos.

El Scope establece el entorno desde el cual las operaciones pueden
generar, transformar o consumir flujos de datos.

Separar el concepto de Scope del de colección materializada permite que
Evo-Script opere bajo un modelo de evaluación lazy: activar un Scope no
implica leer, transferir ni materializar previamente todos sus elementos.

Asimismo, operaciones de salida como `print` no pertenecen de forma global
e intrínseca al lenguaje, sino que constituyen capacidades ofrecidas por
un contexto compatible (como una terminal, una interfaz de usuario o una
sesión de pruebas).


### 6.11 Cambio de Scope dentro de pipelines

Cambiar el Scope activo durante una composición no destruye ni invalida
los datos que ya se encuentran fluyendo a través del pipeline.

Los datos producidos por una operación permanecen en el flujo y pueden
atravesar transiciones de contexto para ser consumidos por capacidades
proporcionadas por un nuevo Scope activo.


### 6.12 Flujo de datos entre múltiples Scopes

Evo-Script permite componer operaciones que involucran múltiples Scopes
dentro de un mismo pipeline.

Ejemplo conceptual (Filesystem hacia Terminal):

    let files = scope-fs("/documents")
    let terminal = scope-terminal

    use files
    |> filter ext equals("txt")
    |> select name, size
    |> use terminal
    |> print

Ejemplo conceptual (Filesystem hacia Database):

    let files = scope-fs("/documents")
    let database = scope-db(...)

    use files
    |> filter ext equals("txt")
    |> select name
    |> use database
    |> enter("documents")
    |> insert(column name)
    |> iter

En estos flujos:

1. El Scope inicial (`files`) actúa como fuente y produce datos.
2. Las operaciones funcionales intermedias (`filter`, `select`) transforman
   los datos en tránsito.
3. La activación de un segundo Scope (`use terminal` o `use database`)
   establece un nuevo contexto activo sin interrumpir el flujo de datos.
4. Las capacidades del nuevo Scope (`print` o `enter` / `insert`) consumen
   los datos transformados.

Scope permite así que Evo-Script componga operaciones sobre distintas fuentes,
destinos y entornos sin incorporar dichas tecnologías directamente al lenguaje.
