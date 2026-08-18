# Evo-Script Specification v0.1

## 1. Propósito

Este documento define los principios, la semántica y los elementos
fundamentales de Evo-Script.

La especificación describe qué representa una construcción del lenguaje
y cómo debe comportarse, independientemente de la implementación concreta
del parser, intérprete o runtime.

Evo-Script convive dentro de un ecosistema de dominios independientes
con responsabilidades claramente delimitadas:

- **Evo-Script** (`evo-script`): define el lenguaje general, la estructura
  de programas y la composición funcional.
- **EvoQ** (`evo-query`): define la semántica formal de consultas, operaciones
  de iteración, proyecciones, condiciones y expresiones sobre datos.
- **Evo-Shell** (`evo-shell`): define capacidades semánticas para interactuar
  con el entorno y el sistema (gestión de scopes, operaciones de sistema).
- **Evo-Values** (`evo-values`): define valores puros y operaciones universales
  independientes de tecnología.
- **Providers**: implementan contratos técnicos concretos para respaldar
  las capacidades semánticas de EvoQ, Evo-Shell u otros dominios.

Esta especificación guía la implementación del lenguaje. La implementación
no define retrospectivamente la semántica; la semántica definida aquí
rige la implementación.


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
- permitir que nuevas capacidades sean proporcionadas mediante contratos
  por EvoQ, Evo-Shell y Providers
- evitar abstracciones orientadas a objetos
- evitar estructuras imperativas tradicionales cuando exista una
  representación funcional equivalente
- permitir composición entre distintas fuentes de datos y capacidades
- mantener separada la semántica del lenguaje de su representación textual
  e implementación técnica


## 4. Paradigma

Evo-Script es un lenguaje funcional y declarativo.

El lenguaje no utiliza programación orientada a objetos como modelo
de composición.

Evo-Script no basa su flujo de control en estructuras imperativas
tradicionales como:

    for
    while

La transformación de colecciones o flujos se expresa mediante operaciones
funcionales y declarativas.

Por ejemplo, en lugar de recorrer manualmente una colección,
se expresa una transformación:

    filter ...
    |> select ...
    |> take(...)
    |> iter

La sintaxis del pipeline forma parte del programa en Evo-Script, mientras
que las operaciones de consulta (`Filter`, `Select`, `Take`, `Iteration`, etc.)
se representan semánticamente mediante **EvoQ** (`evo-query`).

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

Estas capacidades pertenecen a dominios independientes como Evo-Shell
o proveedores especializados de EvoQ.

Evo-Script expresa intención semántica y coordina flujos.

Por ejemplo:

    use scope
    |> enter("documents")
    |> ...

Evo-Shell proporciona las capacidades semánticas necesarias para interactuar
con el entorno (como scopes). EvoQ proporciona la semántica para consultar
e iterar datos.

Los Providers implementan los contratos técnicos correspondientes
para comunicarse con la tecnología concreta.

Conceptualmente:

                     Evo-Script
                    /          \
                   ▼            ▼
                EvoQ         Evo-Shell
              evo-query      evo-shell
                   │             │
                   ▼             ▼
               Contracts     Contracts
                   \             /
                    ▼           ▼
                      Providers
                          │
                          ▼
                 External Technology

                  Evo Values (evo-values)

Un Provider implementa contratos de la capacidad semántica correspondiente.
Por ejemplo:

    EvoQ Contract
        ↓
    Filesystem Provider

o:

    Evo-Shell Contract
        ↓
    Filesystem Provider

Evo-Script no necesita conocer si una capacidad está implementada mediante:

- Linux
- Windows
- PostgreSQL
- SQL Server
- HTTP
- almacenamiento remoto
- cualquier otra tecnología

La tecnología concreta pertenece al Provider. Esto permite que nuevas
tecnologías puedan incorporarse mediante contratos sin introducir
su semántica dentro del lenguaje.

Asimismo, existe una estricta separación entre el lenguaje y el parser:

- `evo-script` define el lenguaje, los tipos y el modelo semántico.
- `evo-script-parser` es responsable únicamente de reconocer e interpretar
  la representación textual (tokens, delimitadores, identificadores),
  sin definir el significado semántico de las construcciones.


## 6. Scopes y contexto de ejecución

### 6.1 Definición de Scope

El Scope es una pieza fundamental de Evo-Script para la interacción
con el entorno, provista semánticamente por Evo-Shell.

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
capacidades externas de sistema. Permite a Evo-Script operar dentro de un
entorno sin necesidad de conocer la tecnología concreta que lo implementa.


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
de contexto provistas por Evo-Shell.

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

La relación arquitectónica conceptual se define mediante contratos:

    Provider
        ↓
    Contrato semántico (Evo-Shell / EvoQ)
        ↓
    Scope / capacidades
        ↓
    Evo-Script

Evo-Shell expone las capacidades de entorno, EvoQ expone las capacidades
de consulta, y los Providers realizan la interacción técnica concreta
con el entorno externo.


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
2. Las operaciones intermedias de consulta (`filter`, `select`) son expresadas
   en la sintaxis de Evo-Script pero representadas y ejecutadas según la semántica
   de **EvoQ** sobre el Provider correspondiente.
3. La activación de un segundo Scope (`use terminal` o `use database`)
   establece un nuevo contexto activo sin interrumpir el flujo de datos.
4. Las capacidades del nuevo Scope (`print` o `enter` / `insert`) consumen
   los datos transformados.

Scope permite así que Evo-Script componga operaciones sobre distintas fuentes,
destinos y entornos sin incorporar dichas tecnologías directamente al lenguaje.


## 7. Sistema de tipos

Evo-Script posee su propio sistema de tipos formal.

Los nombres de tipos del lenguaje no son meros aliases textuales ni sustituciones
realizadas por el parser; representan tipos semánticos propios de Evo-Script.


### 7.1 Tipos nativos

Evo-Script v0.1 define exactamente la siguiente tabla de tipos nativos:

| Evo Script | Representación Rust |
| :--- | :--- |
| `int` | `i32` |
| `float` | `f64` |
| `bool` | `bool` |
| `string` | `str` / `String` según requerimiento técnico de ownership |
| `int8` | `i8` |
| `int16` | `i16` |
| `int32` | `i32` |
| `int64` | `i64` |
| `int128` | `i128` |
| `uint8` | `u8` |
| `uint16` | `u16` |
| `uint32` | `u32` |
| `uint64` | `u64` |
| `uint128` | `u128` |
| `float32` | `f32` |
| `float64` | `f64` |

El tipo `int` es un tipo nativo de Evo-Script cuya representación técnica
de referencia en Rust es `i32`.

El tipo `float` es un tipo nativo de Evo-Script cuya representación técnica
de referencia en Rust es `f64`.

El tipo `string` es un tipo semántico nativo único de Evo-Script. Su representación
técnica de referencia en Rust puede ser prestada (`&str`) o propietaria (`String`)
según las necesidades técnicas de la implementación y de su runtime. La distinción
entre referencias prestadas y memoria propietaria no forma parte de la semántica
visible del lenguaje: el programador de Evo-Script utiliza exclusivamente `string`.
Aspectos como ownership, borrowing y lifetimes del texto son detalles internos de
implementación.

Estos tipos nativos no se definen como macros textuales ni reemplazos del parser.


### 7.2 Tipos definidos

Un tipo también puede ser un tipo definido por el programa.

En Evo-Script v0.1 existen conceptualmente dos mecanismos para definir tipos:

- `struct`
- `enum`

Una función o estructura que hace referencia a un tipo definido no necesita
conocer internamente si el identificador corresponde a un `struct` o a un `enum`.
La definición correspondiente determina su naturaleza.


### 7.3 Struct

`struct` define exclusivamente una estructura de datos.

Un `struct` no posee funciones, métodos, constructores, herencia, interfaces,
comportamiento ni lógica asociada. No describe clases, objetos ni entidades
orientadas a objetos.


#### 7.3.1 Forma general y declaración de campos

La forma textual general para definir un `struct` es:

    struct NombreTipo {
        tipo campo;
        tipo campo;
    }

Ejemplo canónico:

    struct Trabajador {
        int edad;
        string name;
        string last_name;
    }

Cada campo se declara siguiendo la regla oficial: `tipo nombre;` y termina con
punto y coma (`;`).


#### 7.3.2 Convenciones de nombres

Evo-Script establece normativamente las siguientes convenciones:

- **Tipos definidos**: se nombran en `PascalCase`.
  Ejemplos: `Trabajador`, `Pais`, `Colonia`, `DireccionFiscal`, `GuardarError`.
- **Campos**: se nombran en `snake_case`.
  Ejemplos: `edad`, `name`, `last_name`, `id_colonia`, `direccion_fiscal`.


#### 7.3.3 Composición de structs

Un campo dentro de un `struct` puede utilizar cualquier tipo válido de Evo-Script,
incluyendo otro tipo definido mediante `struct`.

Ejemplo:

    struct Pais {
        int id;
        string name;
    }

    struct Colonia {
        int id_colonia;
        string name;
        Pais pais;
    }

Esta relación representa exclusivamente una **composición de datos**: `Colonia`
simplemente contiene un campo de tipo `Pais`. No constituye herencia, relación
orientada a objetos ni clase contenida.


#### 7.3.4 Construcción de valores struct

Evo-Script no utiliza la palabra clave `new` ni constructores asociados para
instanciar un `struct`.

La construcción de un valor se realiza directamente mediante el nombre del tipo:

    Trabajador {
        edad: 43
        name: "Gustavo"
        last_name: "Melendez"
    }

La construcción `NombreTipo { ... }` produce directamente un valor del tipo
`NombreTipo`.


#### 7.3.5 Diferencia entre definición y construcción

Existe una distinción sintáctica y semántica fundamental:

- **Definición**: declara la estructura mediante `tipo nombre;`.
  ```text
  struct Trabajador {
      int edad;
      string name;
  }
  ```
- **Construcción**: inicializa los datos mediante `nombre: valor`.
  ```text
  Trabajador {
      edad: 43
      name: "Gustavo"
  }
  ```

El carácter dos puntos (`:`) pertenece exclusivamente a la asignación de valor a
un campo durante la construcción; no se utiliza para declarar campos en la definición.


#### 7.3.6 Reglas de construcción

Durante la construcción de un `struct` en Evo-Script v0.1 aplican las siguientes
reglas:

1. **Campos obligatorios**: Todos los campos definidos en el `struct` son
   obligatorios. No existen valores por defecto implícitos (no se asume `0`,
   `""` ni `false`) ni existe `null` como mecanismo para omitir campos.
2. **Campos desconocidos o duplicados**: No pueden proporcionarse campos no
   declarados en la definición del `struct`, ni puede repetirse un mismo campo.
3. **Orden de campos**: El orden en que se asignan los campos durante la
   construcción no altera la identidad semántica del valor. Los campos se
   identifican exclusivamente por su nombre:

       Trabajador {
           edad: 43
           name: "Gustavo"
       }

   y:

       Trabajador {
           name: "Gustavo"
           edad: 43
       }

   producen valores equivalentes.


#### 7.3.7 Structs recursivos

Los structs recursivos directos (como `struct Nodo { Nodo siguiente; }`) quedan
explícitamente fuera de la especificación de Evo-Script v0.1. El lenguaje no ha
incorporado aún mecanismos de direccionamiento, `Option`, referencias ni
representación formal de ausencia.


#### 7.3.8 Separación entre datos y comportamiento

Evo-Script mantiene una separación total entre la estructura de datos y las
operaciones que actúan sobre ella:

- `struct`: define datos.
- `NombreTipo { ... }`: construye un valor de datos.
- `fn`: define comportamiento y funciones de transformación.

Ejemplo conceptual:

    struct Pais {
        int id;
        string name;
    }

    struct Colonia {
        int id_colonia;
        string name;
        Pais pais;
    }

    fn guardar(Colonia colonia) -> result<Colonia, GuardarError> {
        ...
    }

`Colonia` no contiene ni conoce la función `guardar`. Las funciones operan sobre
valores de datos; los structs no poseen lógica ni métodos asociados.


### 7.4 Result

Evo-Script incluye el tipo incorporado especial:

    result<T, E>

Ejemplos:

    result<int, Error>
    result<Trabajador, GuardarError>
    result<bool, Error>

Reglas:

- `T` representa el tipo producido en caso de éxito.
- `E` representa el tipo de error.
- `T` y `E` deben ser tipos válidos de Evo-Script.
- `result<T, E>` es un tipo especial incorporado y no implica soporte
  para genéricos generales.
- Los genéricos generales no forman parte de Evo-Script v0.1.


## 8. Funciones

La unidad semántica fundamental de ejecución y cómputo se denomina `Function`.

La forma textual general definida en Evo-Script v0.1 es:

    fn nombre(tipo argumento, tipo argumento) -> tipo {
        correspondencia
    }

Ejemplo:

    fn guardar(Trabajador trabajador) -> result<Trabajador, GuardarError> {
        ...
    }


### 8.1 Declaración

La palabra clave `fn` inicia textualmente la declaración de una función.

Semánticamente representa una `Function`. El parser reconoce el token `fn`,
pero el significado y modelo semántico de `Function` pertenece a Evo-Script.


### 8.2 Nombre

En la declaración:

    fn guardar(...)

el identificador `guardar` define el nombre de la función dentro del programa.


### 8.3 Argumentos

La regla oficial para la declaración de argumentos en Evo-Script es:

    tipo primero, nombre después

Ejemplo:

    Trabajador trabajador

No se utiliza la sintaxis invertida con dos puntos (`trabajador: Trabajador`).

Ejemplos válidos conceptuales:

    int count
    float value
    Trabajador trabajador

Una función puede declarar múltiples argumentos separados por comas:

    fn ejemplo(int id, float amount, Trabajador trabajador) -> ...


### 8.4 Tipo de resultado

La cláusula:

    -> tipo

declara explícitamente el tipo producido por la función.

Ejemplos:

    fn calcular(int value) -> int {
        ...
    }

    fn guardar(Trabajador trabajador) -> result<Trabajador, GuardarError> {
        ...
    }

En Evo-Script v0.1 toda función declara su tipo de resultado de forma explícita.


### 8.5 Correspondencia

Las llaves delimitadoras:

    {
        ...
    }

representan textualmente el inicio y fin de la correspondencia asociada
a la función.

Semánticamente, una `Function` posee una correspondencia asociada:

    Function
    ├── name
    ├── arguments
    ├── result type
    └── correspondence


### 8.6 Sintaxis y semántica

Existe una separación estricta entre la representación textual y la
semántica del lenguaje:

| Sintaxis | Semántica |
| :--- | :--- |
| `fn` | `Function` |
| `tipo nombre` | `Argument` |
| `-> tipo` | `Result type declaration` |
| `result<T, E>` | `Result type` |
| `{ ... }` | `Correspondence` |
| `struct Nombre { ... }` | `Struct definition` |
| `tipo nombre;` | `Field` |
| `Nombre { ... }` | `Struct construction` |
| `nombre: valor` | `Field initialization` |

El parser futuro reconocerá la representación textual y generará la estructura
correspondiente; Evo-Script define el significado y las reglas semánticas
de cada elemento.
