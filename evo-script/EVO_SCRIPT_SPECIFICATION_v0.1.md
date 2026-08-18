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
| `string` | `&str` / `String` según requerimiento técnico de ownership |
| `dynamic` | Representación numérica dinámica interna |
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

El tipo `dynamic` es un tipo numérico especial nativo de Evo-Script. Su propósito
es permitir cómputos numéricos donde el programador no fija de antemano el tamaño
de representación del resultado. En Evo-Script v0.1 `dynamic` es exclusivamente
un tipo numérico; no representa "cualquier objeto", ni tipos heterogéneos, ni
introduce dynamic dispatch, reflexión o runtime member lookup estilo C#, ni tiene
relación con `dyn` de Rust. `dynamic` utiliza una representación numérica interna
dinámica: para enteros, dicha representación puede expandirse para conservar
exactamente el resultado matemático; para valores de punto flotante, continúan
aplicando las reglas y limitaciones de `float`, `float32` y `float64`.


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


### 7.4 Enum

`enum` define un tipo suma en Evo-Script.

Un valor de un tipo `enum` representa exactamente una de las variantes declaradas
en ese enum.

Conceptualmente:

- `struct`: representa un **AND** de datos (composición de campos).
- `enum`: representa un **OR** de variantes (alternativas de datos).
- `fn`: representa **comportamiento** (funciones que operan sobre datos).

Ejemplo:

    enum Estado {
        Activo
        Inactivo
        Suspendido
    }

Un valor de `Estado` puede ser:

    Estado::Activo

o:

    Estado::Inactivo

o:

    Estado::Suspendido

pero exclusivamente una variante a la vez.

Las variantes de un enum **no representan números enteros** dentro de la semántica
visible del lenguaje (no existe `Activo = 0`, `Inactivo = 1`, etc.). La implementación
técnica interna puede emplear discriminantes en bajo nivel, pero éstos no forman
parte del significado del lenguaje: `Estado::Activo` es una variante formal del tipo
`Estado`, no un número entero.


#### 7.4.1 Forma general y declaración de variantes

La forma textual general para definir un `enum` es:

    enum NombreTipo {
        Variante
        Variante
        Variante
    }

Ejemplo canónico:

    enum Dias {
        Lunes
        Martes
        Miercoles
        Jueves
        Viernes
        Sabado
        Domingo
    }


#### 7.4.2 Convenciones de nombres

Evo-Script establece las siguientes convenciones para `enum`:

- **Tipos definidos**: se nombran en `PascalCase` (`Dias`, `Estado`, `Resultado`, `Evento`, `EstadoUsuario`).
- **Variantes**: se nombran en `PascalCase` (`Lunes`, `Activo`, `Correcto`, `NoEncontrado`, `Movimiento`).
- **Campos en variantes estructuradas**: se nombran en `snake_case` (`error_message`, `user_id`, `last_name`, `x`, `y`, `text`).


#### 7.4.3 Referencia y construcción: Tipo::Variante

La forma canónica oficial para referenciar y construir una variante es:

    Tipo::Variante

Ejemplos:

    Dias::Lunes
    Estado::Activo
    Estado::Suspendido

La variante siempre se referencia mediante el nombre calificado de su tipo (`NombreEnum::NombreVariante`).
No se permite el uso de la variante aislada (`Lunes`, `Activo`) como forma canónica,
evitando cualquier ambigüedad entre enums distintos que compartan nombres de variantes:

    enum EstadoUsuario {
        Activo
    }

    enum EstadoServicio {
        Activo
    }

Los valores `EstadoUsuario::Activo` y `EstadoServicio::Activo` son formalmente
distintos y no ambiguos.

No existe la palabra clave `new` para instanciar enums.


#### 7.4.4 Variantes simples

Una variante simple no transporta datos asociados:

    enum Estado {
        Activo
        Inactivo
        Suspendido
    }

Construcción:

    Estado::Activo

No requiere paréntesis `()`, llaves `{}` ni palabras clave adicionales.


#### 7.4.5 Variantes con un valor asociado

Una variante puede transportar un dato asociado cuyo tipo sea cualquier tipo válido
de Evo-Script:

    enum Resultado {
        Correcto(string)
        Error(string)
    }

Construcción:

    Resultado::Correcto("Guardado")
    Resultado::Error("No se pudo guardar")

Conceptualmente:

    Resultado
    ├── Correcto(string)
    └── Error(string)

La variante identifica la alternativa y transporta el valor correspondiente.


#### 7.4.6 Variantes con tipos definidos

Una variante puede transportar un tipo definido previamente en el programa (como un `struct`):

    struct Trabajador {
        int id;
        string name;
    }

    enum Busqueda {
        Encontrado(Trabajador)
        NoEncontrado
    }

Construcción:

    Busqueda::Encontrado(
        Trabajador {
            id: 10
            name: "Juan"
        }
    )

o:

    Busqueda::NoEncontrado

Una variante puede transportar cualquier tipo válido de Evo-Script sin requerir
reglas especiales para structs.


#### 7.4.7 Variantes estructuradas

Una variante puede declarar una estructura propia de campos con nombre:

    enum Evento {
        Inicio

        Movimiento {
            int x;
            int y;
        }

        Mensaje {
            string text;
        }
    }

Construcción:

    Evento::Inicio

    Evento::Movimiento {
        x: 10
        y: 20
    }

    Evento::Mensaje {
        text: "Hola"
    }


#### 7.4.8 Reglas de campos en variantes estructuradas

Las variantes estructuradas reutilizan exactamente las mismas reglas definidas
para los campos de `struct`:

- **En definición**: se declara `tipo nombre;` (con punto y coma).
- **En construcción**: se inicializa `nombre: valor` (con dos puntos).
- Todos los campos declarados son obligatorios durante la construcción.
- No existen valores por defecto implícitos ni existe `null`.
- No pueden proporcionarse campos duplicados ni inexistentes.
- El orden de inicialización no altera la identidad semántica del valor; los campos
  se identifican por nombre.


#### 7.4.9 Separación entre datos y comportamiento

Un `enum` solo define alternativas de datos. No contiene funciones, métodos,
constructores, herencia, interfaces ni lógica asociada.

- `struct`: define una composición de datos.
- `enum`: define alternativas de datos.
- `fn`: define comportamiento.


#### 7.4.10 Conceptos no incluidos en v0.1

Evo-Script v0.1 delimita formalmente el alcance de `enum`:

1. **Variables**: No se define aún sintaxis de declaración de variables (`let`, `var`, `mut`).
2. **Funciones como valores**: Variantes que transporten tipos función o clausuras quedan pendientes.
3. **Inspección de variantes**: Mecanismos como `match`, pattern matching, desestructuración o guards pertenecen a especificaciones posteriores y no forman parte de esta sección.
4. **Discriminantes explícitos**: No se permite asignar valores numéricos explícitos a variantes (`Activo = 1`).
5. **Generic enums**: Los enums genéricos (`enum Tipo<T>`) no forman parte de v0.1.
6. **Conceptos ajenos**: No se introducen métodos, `impl`, `self`, `this`, `new`, `Option`, traits, `dyn`, punteros ni sintaxis de ownership/borrowing.


### 7.5 Result

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


## 8. Valores y bindings

Evo-Script v0.1 no posee variables mutables.

El lenguaje utiliza la palabra clave `let` para asociar un nombre a un valor.
Semánticamente, `let` crea un **binding inmutable**:

    nombre -> valor

Una vez establecido el binding, el identificador representa ese mismo valor
durante todo su ciclo de vida dentro del ámbito correspondiente. `let` no es
un mecanismo de mutabilidad ni define variables reasignables.


### 8.1 Declaración oficial con `let`

La sintaxis oficial y canónica para crear un binding inmutable es:

    let tipo nombre = valor;

Ejemplos:

    let int edad = 43;
    let string name = "Gustavo";
    let Dias dia = Dias::Lunes;

Esta forma mantiene la regla uniforme de Evo-Script: **tipo primero, nombre después**.
No se utiliza la sintaxis invertida con dos puntos (`let edad: int = 43;`).


### 8.2 Tipo explícito obligatorio

En Evo-Script v0.1 todo binding creado mediante `let` debe declarar su tipo
de forma explícita.

Válido:

    let int edad = 43;

La inferencia de tipos no forma parte de Evo-Script v0.1. Por tanto, formas sin
tipo explícito quedan fuera del lenguaje:

    let edad = 43;

No se introducen palabras clave como `auto`, `var`, `infer` ni operadores como `:=`.


### 8.3 Inicialización obligatoria y compatibilidad de tipos

Todo binding debe inicializarse obligatoriamente en la misma sentencia en que se declara.

Válido:

    let int edad = 43;

Inválido:

    let int edad;

No existen bindings sin valor inicial ni valores por defecto implícitos.

Asimismo, el valor asignado debe ser directamente compatible con el tipo declarado:

    let int edad = 43;     // Válido

    let int edad = "43";   // Inválido: incompatibilidad de tipos

Evo-Script no realiza coerciones ni conversiones implícitas de tipos. Si se requiere
adaptar un valor a otro tipo, debe utilizarse una conversión explícita mediante la
familia `to_tipo`.


### 8.4 Inmutabilidad absoluta y ausencia de reasignación

Un binding creado mediante `let` no puede ser reasignado bajo ninguna circunstancia.

Ejemplo:

    let int edad = 43;

Tras esta declaración, cualquier intento de reasignación es inválido:

    edad = 44; // Inválido

La inmutabilidad es una propiedad intrínseca y estructural de Evo-Script. No existen
modificadores ni sintaxis para declarar bindings mutables (`mut`, `mutable`, `var`,
`ref mut`, `set`, etc.).


### 8.5 Ausencia de shadowing y ámbito de visibilidad

Evo-Script no permite shadowing (ocultamiento de nombres en un mismo ámbito de visibilidad).

Ejemplo inválido:

    let int edad = 43;
    let int edad = 44; // Inválido: el identificador 'edad' ya se encuentra visible

Mientras un binding continúe visible, su identificador no puede ser reutilizado
por otra declaración `let`. Debe emplearse un identificador distinto:

    let int edad = 43;
    let int nueva_edad = 44; // Válido

El shadowing no se admite como alternativa para simular mutabilidad ni como excepción.
Un binding posee una región delimitada de visibilidad, y al salir de dicho ámbito
finaliza su ciclo de vida.


### 8.6 Convenciones de nombres

Evo-Script distingue formalmente las siguientes convenciones de nombres:

- **Tipos nativos**: conservan exactamente los nombres definidos por Evo-Script (`int`, `float`, `bool`, `string`, `dynamic`, `int8`, `int16`, `int32`, `int64`, `int128`, `uint8`, `uint16`, `uint32`, `uint64`, `uint128`, `float32`, `float64`).
- **Tipos definidos por el programa**: se nombran en `PascalCase` (`Trabajador`, `Dias`, `Resultado`, `Evento`).
- **Bindings**: se nombran en `snake_case` (`edad`, `first_name`, `last_name`, `id_colonia`, `current_user`). No se utiliza `camelCase`.


### 8.7 `let` con distintos tipos de valores

`let` opera de manera uniforme sobre todos los tipos de datos de Evo-Script:

#### 8.7.1 Tipos nativos

    let int edad = 43;
    let float precio = 10.5;
    let bool activo = true;
    let string name = "Gustavo";

#### 8.7.2 Structs

El valor asignado puede ser la construcción directa de un `struct`:

    let Trabajador trabajador = Trabajador {
        edad: 43
        name: "Gustavo"
        last_name: "Melendez"
    };

La expresión `Trabajador { ... }` produce el valor del struct y `let` asocia el nombre
`trabajador` a dicho valor. No existe `new` ni instanciación orientada a objetos.

#### 8.7.3 Enums

El valor asignado puede ser cualquier variante válida de un `enum`:

- Variante simple:
  ```text
  let Dias dia = Dias::Lunes;
  ```
- Variante con valor asociado:
  ```text
  let Resultado resultado = Resultado::Correcto("Guardado");
  ```
- Variante estructurada:
  ```text
  let Evento evento = Evento::Movimiento {
      x: 10
      y: 20
  };
  ```

#### 8.7.4 Tipo dynamic

`dynamic` permite asociar un valor numérico cuya representación no se restringe
anticipadamente a un tamaño entero de representación:

    let dynamic result = a + b;
    let dynamic value = 100;

Los bindings de tipo `dynamic` son **completamente inmutables** y siguen todas
las reglas de `let` (inicialización obligatoria, sin reasignación, sin shadowing,
nombres en `snake_case`). `dynamic` describe la representación del valor numérico,
no constituye un modificador de mutabilidad (`mut`, `var`).



### 8.8 Regla de terminación con punto y coma (`;`)

En Evo-Script v0.1 aplica la siguiente regla sintáctica general:

    Toda declaración u operación completa termina con punto y coma (`;`).

Ejemplos:

    let int edad = 43;
    guardar(trabajador);
    print(name);

Esta regla no aplica a las definiciones estructurales delimitadas por bloques:

- Las definiciones de `struct`, `enum` y `fn` concluyen con su llave de cierre `}` sin requerir un punto y coma posterior (no se escribe `};`).
- Los campos internos de un `struct` o de una variante estructurada concluyen con punto y coma (`tipo nombre;`).
- Las declaraciones `let` y las operaciones concluyen con punto y coma (`;`).


### 8.9 Separación entre Tipo, Valor y Binding

Evo-Script distingue tres conceptos fundamentales:

1. **Tipo**: define la clase y el contrato de los datos válidos (`int`, `string`, `Trabajador`, `Dias`).
2. **Valor**: representa la instancia concreta de datos (`43`, `"Gustavo"`, `Trabajador { ... }`, `Dias::Lunes`).
3. **Binding**: asocia de forma inmutable un nombre a un valor específico mediante `let`.

Ejemplo:

| Concepto | Ejemplo | Significado |
| :--- | :--- | :--- |
| **Tipo** | `int` | Define el dominio de enteros de 32 bits. |
| **Valor** | `43` | Instancia concreta de dato numérico. |
| **Binding** | `let int edad = 43;` | Asociación inmutable del nombre `edad` al valor `43`. |

Asimismo, se mantiene una clara distinción entre construcciones del lenguaje:

- **Argumento de función**: `int edad` (sin `let`, sin punto y coma).
- **Campo de struct**: `int edad;` (sin `let`, con punto y coma).
- **Binding**: `let int edad = 43;` (con `let`, con inicialización y punto y coma).


### 8.10 Conceptos no incluidos en v0.1

Evo-Script v0.1 delimita estrictamente la semántica de valores y bindings:

1. **Variables mutables**: No existen `mut`, `var`, `mutable` ni reasignaciones (`edad = 44`).
2. **Asignación independiente**: El operador `=` solo forma parte de la sintaxis de `let`, no constituye una sentencia de asignación separada.
3. **Constantes globales**: No se introducen `const` ni `static`.
4. **Desestructuración**: No se admite pattern binding ni desestructuración en `let` (`let (a, b) = ...`).
5. **Inferencia de tipos**: No se admite omisión de tipo ni palabras clave de inferencia (`auto`).
6. **Funciones como valores**: No se definen bindings a funciones (`let funcion = guardar;`).
7. **Referencias y punteros**: No se introducen punteros, referencias, `&mut`, `Box`, `Rc`, `Arc` ni mutabilidad interior (`Cell`, `RefCell`).


## 9. Conversiones de tipos

### 9.1 Principio fundamental: ausencia de conversiones implícitas

Evo-Script **no realiza conversiones implícitas de tipos**. Un valor nunca cambia
silenciosamente de tipo en ninguna circunstancia.

Esto aplica incluso cuando técnicamente el tipo destino pueda albergar sin pérdida
todos los valores del tipo origen. No existen promociones numéricas automáticas como:

- `int8` $\rightarrow$ `int16` $\rightarrow$ `int32` $\rightarrow$ `int64` $\rightarrow$ `int128`
- `uint8` $\rightarrow$ `uint16` $\rightarrow$ `uint32` $\rightarrow$ `uint64` $\rightarrow$ `uint128`
- `int` $\rightarrow$ `float`
- `float32` $\rightarrow$ `float64`

Cualquier cambio de tipo debe ser explícito y visible en el código del programa.


### 9.2 Familia oficial de conversión: `to_tipo`

Evo-Script define una única familia oficial y normativa de operaciones de conversión:

    to_tipo

La lista completa de operaciones de conversión en Evo-Script v0.1 es:

- **Enteros con signo**: `to_int` (convierte a `int` / `i32`), `to_int8`, `to_int16`, `to_int32`, `to_int64`, `to_int128`.
- **Enteros sin signo**: `to_uint8`, `to_uint16`, `to_uint32`, `to_uint64`, `to_uint128`.
- **Punto flotante**: `to_float` (convierte a `float` / `f64`), `to_float32`, `to_float64`.
- **Texto**: `to_string`.

No existe una familia alternativa como `convert_to_tipo`, ni palabras clave de casteo como `cast` o `as`. No existe el tipo `float128` ni la operación `to_float128`.


### 9.3 Conversiones garantizadas

Una conversión se considera **garantizada** cuando todos los valores posibles del
tipo origen tienen representación exacta dentro del tipo destino sin riesgo de
pérdida de rango ni precisión (por ejemplo: `int8` $\rightarrow$ `int16`,
`int64` $\rightarrow$ `int128`, `uint8` $\rightarrow$ `uint16`, `uint64` $\rightarrow$ `int128`).

En estos casos, la operación produce directamente el tipo destino:

    let int64 source = 100;
    let int128 target = to_int128(source); // Produce int128 directamente

Aun siendo una conversión garantizada, la operación **sigue siendo estrictamente explícita**:

    let int128 target = source; // Inválido: no hay conversión automática


### 9.4 Conversiones potencialmente fallables y ConversionError

Cuando una conversión puede implicar pérdida de rango numérico, imposibilidad de
representación exacta o pérdida de precisión, la operación **no altera ni trunca silenciosamente
el valor**. En su lugar, expresa la posibilidad de fallo produciendo:

    result<T, ConversionError>

`ConversionError` se introduce conceptualmente como el tipo semántico de error que
representa el fracaso de una conversión de tipo. En Evo-Script v0.1 no se definen
variantes internas cerradas de `ConversionError` ni mecanismos finales de captura/propagación
de Result (`?`, `unwrap`, `match`).


### 9.5 Conversiones entre enteros

- **Ampliación garantizada**:
  ```text
  to_int128(int64_value) -> int128
  ```
- **Reducción potencialmente fallable**:
  ```text
  to_int64(int128_value) -> result<int64, ConversionError>
  ```
  La conversión valida en tiempo de ejecución si el valor concreto cabe dentro del rango del tipo destino.


### 9.6 Conversiones entre signed y unsigned

Las conversiones entre enteros con signo y sin signo se rigen estrictamente por la
regla universal de representación exacta del dominio completo del tipo origen dentro
del tipo destino. La clasificación no depende únicamente de la presencia o ausencia
de signo, sino de la relación de rangos entre ambos tipos:

- **Conversiones signed/unsigned garantizadas**:
  Cuando el tipo destino tiene capacidad suficiente para albergar exactamente todo
  el rango no negativo del tipo origen sin signo:
  - `uint8` $\rightarrow$ `int16`
  - `uint16` $\rightarrow$ `int32`
  - `uint32` $\rightarrow$ `int64`
  - `uint64` $\rightarrow$ `int128`

  Ejemplo:
  ```text
  let uint64 source = 100;
  let int128 target = to_int128(source); // Produce int128 directamente
  ```
  Aun siendo garantizada, sigue requiriendo la invocación explícita de `to_tipo`
  (`let int128 target = source;` es inválido).

- **Conversiones signed/unsigned potencialmente fallables**:
  Cuando existe al menos un valor válido del tipo origen que no cabe en el tipo destino:
  - `int32` $\rightarrow$ `uint32`: los valores negativos de `int32` no pueden representarse en `uint32`. Requiere `to_uint32(value)` y produce `result<uint32, ConversionError>`.
  - `uint32` $\rightarrow$ `int32`: los valores de `uint32` mayores a $2^{31}-1$ no caben en `int32`. Requiere `to_int32(value)` y produce `result<int32, ConversionError>`.
  - `int8` $\rightarrow$ `uint8`: los valores negativos no pueden representarse en `uint8`. Requiere `to_uint8(value)` y produce `result<uint8, ConversionError>`.
  - `uint128` $\rightarrow$ `int128`: los valores de `uint128` mayores a $2^{127}-1$ no caben en `int128`. Requiere `to_int128(value)` y produce `result<int128, ConversionError>`.

Evo-Script no realiza reinterpretación de bits ni comportamiento de wrapping silencioso.


### 9.7 Conversiones de punto flotante

Las conversiones que involucran números de punto flotante consideran tanto el rango
como la precisión y la exactitud de la representación:

- **Entero a Float**: `to_float64(int64_value)` produce `result<float64, ConversionError>` cuando el valor entero no pueda representarse con exactitud en el formato flotante sin pérdida de información. No se realiza redondeo silencioso.
- **Float a Entero**: `to_int64(float_value)` produce `result<int64, ConversionError>`. No se realiza truncamiento ni redondeo silencioso.
- **Float a Float**: `to_float32(float64_value)` produce `result<float32, ConversionError>` si existe pérdida de precisión.


### 9.8 Conversión a string (`to_string`)

La operación `to_string` permite convertir explícitamente valores a su representación textual:

    let string text = to_string(43);

Ningún valor se convierte automáticamente a texto. En Evo-Script v0.1 no se define parsing inverso desde texto hacia números (`parse_int`, `parse_float`).


### 9.9 Conversiones desde dynamic

Toda conversión explícita desde `dynamic` hacia cualquier tipo numérico de tamaño fijo
es **semánticamente potencialmente fallable** y posee una firma de retorno estable:

    result<T, ConversionError>

Dado que el dominio representable por `dynamic` no está acotado por los límites
de ningún tipo numérico fijo particular, la operación no puede garantizar estáticamente
que el valor concreto quepa en el tipo destino. Por tanto, su firma no varía según
el valor en tiempo de ejecución:

    to_int(dynamic_value)    -> result<int, ConversionError>
    to_int8(dynamic_value)   -> result<int8, ConversionError>
    to_int16(dynamic_value)  -> result<int16, ConversionError>
    to_int32(dynamic_value)  -> result<int32, ConversionError>
    to_int64(dynamic_value)  -> result<int64, ConversionError>
    to_int128(dynamic_value) -> result<int128, ConversionError>

    to_uint8(dynamic_value)   -> result<uint8, ConversionError>
    to_uint16(dynamic_value)  -> result<uint16, ConversionError>
    to_uint32(dynamic_value)  -> result<uint32, ConversionError>
    to_uint64(dynamic_value)  -> result<uint64, ConversionError>
    to_uint128(dynamic_value) -> result<uint128, ConversionError>

    to_float(dynamic_value)   -> result<float, ConversionError>
    to_float32(dynamic_value) -> result<float32, ConversionError>
    to_float64(dynamic_value) -> result<float64, ConversionError>

Incluso cuando un binding `dynamic` contenga un valor pequeño que cabe en el tipo destino:

    let dynamic value = 10;
    to_int64(value) // Produce result<int64, ConversionError> (con éxito en esa ejecución)

La operación produce una variante de éxito que contiene el valor `int64`, pero el tipo
estático de la expresión continúa siendo `result<int64, ConversionError>`. Si el valor
no puede representarse exactamente en el destino por rango o precisión, produce una
variante de error con `ConversionError`.

Asimismo, `to_string(dynamic_value)` produce directamente `string` como representación
textual explícita del valor dinámico. No se introducen métodos como `dynamic.as_int64`
ni operadores de casteo (`as`, `cast`).



### 9.10 Composición en pipelines

Las operaciones de conversión pueden componerse de forma natural dentro de pipelines:

    source
    |> to_int128

o:

    source
    |> to_int64


### 9.11 Conceptos no incluidos en v0.1

Quedan formalmente fuera de la especificación de conversiones de Evo-Script v0.1:

1. **Casts implícitos o estilo C / Rust**: No existen `as`, `cast`, `transmute` ni reinterpretación de memoria.
2. **Promociones automáticas**: No existen widening implícito ni conversiones silenciosas.
3. **Redondeo o truncamiento automático**: Las conversiones fallan explícitamente en lugar de distorsionar datos.
4. **Parsing inverso**: No se definen `parse_int` ni `parse_float`.
5. **Tipos no existentes**: No existe `float128` ni `to_float128`.
6. **Mecanismos alternativos**: No existen `convert_to_*` ni generic conversion functions.
7. **Desempaquetado de Result**: No se introducen `?`, `unwrap`, `expect` ni `match`.


## 10. Expresiones y operadores

### 10.1 Expresiones

Una **expresión** (`Expression`) es una construcción sintáctica y semántica que se
evalúa para producir un valor.

Ejemplos:

- Aritméticas: `price + tax`, `count * 2`
- Comparaciones: `age >= 18`, `first == second`
- Lógicas: `active && authorized`, `!disabled`
- Unarias: `-temperature`, `!ready`
- Conversiones: `to_int64(value)`

Una expresión puede utilizarse directamente como el valor en una declaración `let`:

    let int total = price + tax;
    let bool allowed = active && age >= 18;


### 10.2 Tipado contextual de literales numéricos

Los literales numéricos en Evo-Script no se definen como valores previamente tipados
que posteriormente deban convertirse. Un literal numérico adquiere su tipo a partir
del contexto numérico explícitamente requerido.

Ejemplo:

    let int64 value = 100;

En este caso, el literal `100` nace semánticamente como `int64`. No ocurre una
conversión implícita `int -> int64` ni una promoción de tipos.

Ejemplos canónicos de tipado contextual:

    let int8 level = 5;
    let int64 population = 100;
    let int128 total = 500;
    let uint8 percentage = 100;
    let uint64 identifier = 1000;
    let float32 price = 10.5;
    let float64 amount = 10.5;
    let dynamic count = 100;

Reglas de tipado contextual:

1. **Representabilidad obligatoria**: El literal debe ser exactamente representable por el tipo requerido.
   - `let uint8 value = 100;` es válido porque `100` cabe en `uint8`.
   - `let uint8 value = 300;` es inválido porque `300` excede el rango de `uint8`. No se realiza wrapping, truncamiento ni saturación silenciosa.
2. **Literales sin contexto**: Cuando un literal numérico no posee un contexto de tipo que determine una representación específica, adopta los tipos por defecto del lenguaje:
   - Literal entero sin contexto: produce `int` (`i32`).
   - Literal decimal sin contexto: produce `float` (`f64`).
3. **Propagación del contexto en expresiones `let`**: En una declaración como `let int64 total = value + 1;`, si la expresión opera en el contexto de `int64`, el literal `1` nace directamente como `int64`. Esto no constituye inferencia general de tipos ni relaja la prohibición de conversiones implícitas entre bindings existentes.
4. **Literales de punto flotante**: El contexto asigna el tipo `float32` o `float64`. En Evo-Script v0.1 no se extienden reglas sobre algoritmos de parsing decimal/binario o notación científica avanzada.


### 10.3 Operadores aritméticos

Evo-Script v0.1 define exactamente cinco operadores aritméticos binarios:

| Operador | Operación | Ejemplo |
| :--- | :--- | :--- |
| `+` | Suma | `a + b` |
| `-` | Resta | `a - b` |
| `*` | Multiplicación | `a * b` |
| `/` | División | `a / b` |
| `%` | Residuo | `a % b` |

Reglas:

1. **Conservación de tipo**: Una operación entre operandos del mismo tipo numérico de tamaño fijo produce como resultado ese mismo tipo semántico (por ejemplo, `int32 + int32` produce `int32`).
2. **Ausencia de promoción automática**: Las operaciones aritméticas no promocionan silenciosamente sus tipos (por ejemplo, `int32 + int32` no se convierte automáticamente en `int64` para prevenir desbordamientos).
3. **Compatibilidad estricta**: Operaciones entre tipos distintos (como `int32 + int64` o `int + float`) no realizan conversiones implícitas; requieren conversiones explícitas mediante `to_tipo`.


### 10.4 Overflow en tipos de tamaño fijo y OverflowError

Para los tipos numéricos de tamaño fijo (`int8`, `int16`, `int32`, `int64`, `int128`, `uint8`..`uint128`, `float32`, `float64`), un resultado que no puede representarse dentro del rango del tipo produce una condición de **overflow**.

Ejemplo:

    let int8 a = 127;
    let int8 b = 1;
    let int8 result = a + b; // El resultado matemático 128 no cabe en int8

Reglas de overflow:

1. **Sin promoción automática**: El lenguaje nunca cambia silenciosamente el tipo para evitar un error (`int8 + int8` no se transforma en `int16`).
2. **Fallo con OverflowError**: Cuando una operación bajo un tipo fijo produce un valor fuera del rango representable, la evaluación falla con `OverflowError`.
3. **Sin wrapping modular**: Evo-Script no realiza wrapping silencioso (`127 + 1` en `int8` no produce `-128`).
4. **Sin saturación**: No se realiza saturación automática (`127 + 1` en `int8` no produce `127`).
5. **Negación unaria y rango**: La negación numérica `-value` sobre tipos fijos también produce `OverflowError` si el valor resultante no cabe en el tipo (por ejemplo, negar el valor mínimo representable en un entero con signo).
6. **Separación semántica de errores**:
   - `OverflowError`: ocurre durante la evaluación de una operación numérica bajo un tipo fijo cuando el resultado no cabe en dicho tipo.
   - `ConversionError`: ocurre cuando se solicita una conversión explícita `to_tipo` y el valor de origen no puede representarse en el destino.
7. **No afecta firmas con Result**: `OverflowError` es un error de evaluación aritmética; no altera la firma conceptual de los operadores para envolverlos en `result<T, E>`.


### 10.5 Evaluación numérica dinámica con dynamic

Evo-Script proporciona el tipo especial `dynamic` como alternativa explícita para evaluaciones numéricas donde el programador no desea restringir de antemano el tamaño de representación del resultado.

```text
let dynamic result = a + b;
```

Reglas de evaluación bajo contexto `dynamic`:

1. **Participación desde el inicio de la evaluación**: El contexto `dynamic` se aplica a la evaluación numérica de la expresión desde su origen. No ocurre una evaluación previa en un tipo fijo que cause overflow para luego intentar guardarse en `dynamic`:
   $$\text{Expresión} \longrightarrow \text{Evaluación dinámica} \longrightarrow \text{Resultado matemático exacto} \longrightarrow \text{Representación suficiente} \longrightarrow \text{dynamic}$$
2. **Comparación Fijo vs Dynamic**:
   - **Tipo fijo**:
     ```text
     let int8 fixed = a + b; // Produce OverflowError si el resultado no cabe en int8
     ```
   - **Tipo dynamic**:
     ```text
     let dynamic dynamic_result = a + b; // Conserva el valor exacto (128) en una representación suficiente sin OverflowError
     ```
3. **Conservación exacta de enteros y precisión arbitraria**: Para operaciones enteras, `dynamic` garantiza la conservación exacta del resultado matemático. Si el resultado excede el tamaño de `int128`/`uint128`, utiliza internamente una representación de precisión arbitraria. No se introducen tipos visibles adicionales como `bigint`, `int256` ni `int512`.
4. **dynamic no significa imprecisión**: Para enteros, `dynamic` garantiza exactitud matemática absoluta, no aproximación.
5. **dynamic y punto flotante**: `dynamic` no introduce precisión arbitraria para flotantes ni el tipo `float128`. Las operaciones flotantes se rigen por las reglas de los tipos flotantes definidos (`float`, `float32`, `float64`).
6. **Sin conversiones implícitas de operandos**: Declarar un resultado como `dynamic` no vuelve válidas operaciones entre operandos incompatibles. Por ejemplo, operar `int32` con `int64` requiere conversión explícita:
   ```text
   let dynamic result = to_int64(a) + b;
   ```


### 10.6 Operadores de comparación

Evo-Script v0.1 define seis operadores de comparación:

| Operador | Significado | Ejemplo |
| :--- | :--- | :--- |
| `==` | Igual a | `a == b` |
| `!=` | Diferente de | `a != b` |
| `<` | Menor que | `a < b` |
| `<=` | Menor o igual que | `a <= b` |
| `>` | Mayor que | `a > b` |
| `>=` | Mayor o igual que | `a >= b` |

Reglas:

- El resultado semántico de toda comparación es un valor de tipo `bool`.
- No se introducen identificadores textuales como `equals`, `greater_than` o `less_than` para las expresiones generales del lenguaje (las construcciones semánticas de consulta de EvoQ son independientes).

Ejemplos:

    let bool adult = age >= 18;
    let bool same = first == second;


### 10.7 Operadores lógicos

Evo-Script v0.1 define tres operadores lógicos:

| Operador | Significado | Ejemplo |
| :--- | :--- | :--- |
| `&&` | AND lógico | `active && authorized` |
| `\|\|` | OR lógico | `active \|\| administrator` |
| `!` | Negación lógica (NOT) | `!disabled` |

Reglas:

- Operan exclusivamente sobre valores de tipo `bool` y producen un resultado `bool`.
- No se introducen palabras clave alternativas como `and`, `or` o `not`.
- El operador `!` actúa exclusivamente como negación booleana; no cumple funciones de unwrapping, aserción ni propagación de errores.

Ejemplo:

    let bool allowed = active && age >= 18;


### 10.8 Operadores unarios

Evo-Script v0.1 define dos operadores unarios prefijos:

| Operador | Significado | Tipo aplicable | Ejemplo |
| :--- | :--- | :--- | :--- |
| `!` | Negación lógica | `bool` | `!valid` |
| `-` | Negación numérica | Numéricos con signo | `-10`, `-delta` |

Ejemplo:

    let int temperature = -10;

No se incluyen operadores unarios de incremento (`++`), decremento (`--`), complemento bit a bit (`~`), referencias (`&`) ni desreferencia (`*`).


### 10.9 Agrupación y precedencia

Los paréntesis `( )` permiten agrupar expresiones para controlar explícitamente el orden de evaluación:

    (a + b) * c

El uso de paréntesis tiene como único propósito la agrupación sintáctica; no define tuplas ni tipos compuestos.

Para los operadores compartidos con Rust, Evo-Script adopta la precedencia y asociatividad convencional de Rust:

1. Operadores unarios (`!`, `-`)
2. Multiplicativos (`*`, `/`, `%`)
3. Aditivos (`+`, `-`)
4. Comparaciones (`<`, `<=`, `>`, `>=`, `==`, `!=`)
5. Conjunción lógica (`&&`)
6. Disyunción lógica (`||`)

Ejemplos:

- `a + b * c` equivale semánticamente a `a + (b * c)`.
- `a > 10 && b < 20` equivale semánticamente a `(a > 10) && (b < 20)`.


### 10.10 Pipeline (`|>`)

Evo-Script preserva su operador nativo de pipeline `|>` para la composición secuencial de datos y operaciones:

    source
    |> to_int64

`|>` no es un operador bitwise ni debe confundirse con la disyunción lógica `||`.


### 10.11 Ausencia de operadores de asignación y mutación

Debido a la inmutabilidad intrínseca de los bindings, Evo-Script no posee operadores generales de asignación ni mutación:

- No existen operadores de asignación ni asignación compuesta (`=`, `+=`, `-=`, `*=`, `/=`, `%=`).
- El símbolo `=` aparece exclusivamente dentro de la sentencia `let tipo nombre = valor;` como ligadura inicial; no constituye una sentencia de asignación.
- No existen operadores de incremento ni decremento (`++`, `--`).

Ejemplos inválidos:

    age = 44;     // Inválido
    age += 1;     // Inválido
    age++;        // Inválido
    ++age;        // Inválido


### 10.12 Operadores y conceptos excluidos de v0.1

Quedan formalmente fuera de Evo-Script v0.1:

1. **Operaciones Bitwise**: No existen `&`, `|`, `^`, `<<`, `>>` ni funciones como `bit_and`, `bit_or`, `shift_left`.
2. **Mutación y asignación**: No existen `=`, `+=`, `++`, `--`.
3. **Casteo**: No existen `as` ni `cast` (se utiliza exclusivamente la familia `to_tipo`).
4. **Punteros y referencias**: No existen operadores `&` ni `*`.
5. **Tipos no numéricos en dynamic**: `dynamic` no soporta structs, enums ni dispatch polimórfico dinámico.
6. **Tipos enteros artificiales visibles**: No existen `bigint`, `int256` ni `int512`.


### 10.13 Semántica pendiente

Permanecen explícitamente pendientes para su definición en especificaciones posteriores:

1. **División por cero**: La semántica exacta y diagnóstico ante la división entre cero permanece abierta (no se introduce `DivisionByZeroError` en v0.1).
2. **Detalles avanzados de parsing en flotantes**: Algoritmos de parsing textual detallado y soporte de notación científica avanzada en literales float.
3. **Mecanismos generales de captura de errores**: No se introducen construcciones de manejo de excepciones o captura de errores de evaluación (`try`/`catch`).


## 11. Funciones

La unidad semántica fundamental de ejecución y cómputo se denomina `Function`.

La forma textual general definida en Evo-Script v0.1 es:

    fn nombre(tipo argumento, tipo argumento) -> tipo {
        correspondencia
    }

Ejemplo:

    fn guardar(Trabajador trabajador) -> result<Trabajador, GuardarError> {
        ...
    }


### 11.1 Declaración

La palabra clave `fn` inicia textualmente la declaración de una función.

Semánticamente representa una `Function`. El parser reconoce el token `fn`,
pero el significado y modelo semántico de `Function` pertenece a Evo-Script.


### 11.2 Nombre

En la declaración:

    fn guardar(...)

el identificador `guardar` define el nombre de la función dentro del programa.


### 11.3 Argumentos

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


### 11.4 Tipo de resultado

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


### 11.5 Correspondencia

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


### 11.6 Sintaxis y semántica

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
| `enum Nombre { ... }` | `Enum definition` |
| `Variante` | `Enum variant` |
| `Tipo::Variante` | `Enum variant value` |
| `Variante(tipo)` | `Variant with associated value` |
| `Tipo::Variante(valor)` | `Construction of associated-value variant` |
| `Variante { ... }` | `Structured variant` |
| `Tipo::Variante { ... }` | `Structured variant construction` |
| `let` | `Binding declaration` |
| `let tipo nombre = valor;` | `Immutable binding` |
| `=` | `Value association in let` |
| `;` | `End of declaration/operation` |
| `to_tipo(valor)` | `Explicit type conversion` |
| `valor \|> to_tipo` | `Composed explicit conversion` |
| `literal numérico` | `Contextual numeric literal` |
| `let int64 x = 100;` | `Literal typed as int64 by context` |
| `let dynamic x = expresión;` | `Dynamic numeric binding` |
| `OverflowError` | `Fixed-width arithmetic overflow` |
| `a + b` | `Addition expression` |
| `a - b` | `Subtraction expression` |
| `a * b` | `Multiplication expression` |
| `a / b` | `Division expression` |
| `a % b` | `Remainder expression` |
| `a == b` | `Equality comparison` |
| `a != b` | `Inequality comparison` |
| `a < b` | `Less-than comparison` |
| `a <= b` | `Less-or-equal comparison` |
| `a > b` | `Greater-than comparison` |
| `a >= b` | `Greater-or-equal comparison` |
| `a && b` | `Logical AND` |
| `a \|\| b` | `Logical OR` |
| `!a` | `Logical NOT` |
| `-a` | `Numeric negation` |
| `(expresión)` | `Grouped expression` |
| `valor \|> operación` | `Pipeline composition` |

El parser futuro reconocerá la representación textual y generará la estructura
correspondiente; Evo-Script define el significado y las reglas semánticas
de cada elemento.
