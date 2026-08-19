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

    enum GuardarColoniaResult {
        Guardado(Colonia)
        Error(GuardarError)
    }

    fn guardar(Colonia colonia) -> GuardarColoniaResult {
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

1. **Mutabilidad**: Evo-Script v0.1 no define `var`, `mut` ni variables reasignables. Los valores con nombre se representan mediante bindings inmutables declarados con `let`.
2. **Funciones como valores**: Variantes que transporten tipos función o clausuras quedan pendientes.
3. **Pattern matching general**: La inspección de variantes se realiza exclusivamente mediante la expresión exhaustiva `when` (Sección 10.14). Mecanismos de pattern matching general (`match`, wildcards `_`, guards, patrones anidados o rangos) no forman parte de v0.1.
4. **Discriminantes explícitos**: No se permite asignar valores numéricos explícitos a variantes (`Activo = 1`).
5. **Generic enums**: Los enums genéricos (`enum Tipo<T>`) no forman parte de v0.1.
6. **Conceptos ajenos**: No se introducen métodos, `impl`, `self`, `this` orientado a objetos, `new`, `Option`, traits, `dyn`, punteros ni sintaxis de ownership/borrowing.


### 7.5 Ausencia de tipos genéricos

Evo-Script v0.1 **no posee tipos genéricos**. No existe parametrización de tipos ni
sintaxis general como `Tipo<T>` o `Tipo<T, E>`.

Reglas normativas:

1. **Sin tipos genéricos generales**: No se introducen type parameters, constraints, cláusulas `where`, traits, `dyn` ni colecciones genéricas integradas (`Option<T>`, `Either<T, E>`, `Outcome<T, E>`, `List<T>`, `Vec<T>`, `Map<K, V>`).
2. **Eliminación de result<T, E>**: `result<T, E>` no existe como tipo especial ni como abstracción genérica del lenguaje. No existen tipos unión como `T | E`.
3. **Alternativas del dominio mediante enum**: Cuando una función requiere retornar una de varias alternativas semánticas de su dominio, se define explícitamente un tipo `enum`:
   ```text
   struct Trabajador {
       int id;
       string name;
   }

   enum BuscarTrabajadorResult {
       Encontrado(Trabajador)
       NoEncontrado
       Error(BuscarError)
   }

   fn buscar(int id) -> BuscarTrabajadorResult {
       ...
   }
   ```
4. **Variantes sin semántica mágica**: La variante `Error` dentro de un enum de dominio es una variante normal definida por el usuario; el lenguaje no le asigna ningún comportamiento intrínseco especial. Los enums de dominio pueden modelar tantas alternativas como el caso requiera (por ejemplo, `Encontrado(Trabajador)`, `NoEncontrado`, `ServicioNoDisponible`).
5. **Separación entre dominio y evaluación**: Las alternativas del dominio son valores normales producidos por el programa (`enum`); los errores de evaluación del lenguaje (`ConversionError`, `OverflowError`, `DivisionByZeroError`) representan fallos durante la evaluación y no forman parte del tipo normal retornado por una expresión o función.



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

Cuando una conversión puede implicar pérdida de rango numérico, imposibilidad de representación exacta o pérdida de precisión, la operación no altera ni distorsiona silenciosamente el valor.

1. **Tipo semántico normal**: El tipo normal de una conversión explícita es estrictamente el tipo destino declarado (`T`).
2. **Evaluación válida**: Si el valor concreto de origen puede representarse de forma exacta y completa en el tipo destino, la evaluación produce directamente el valor en el tipo destino.
3. **Fallo de evaluación**: Si el valor de origen no puede representarse exactamente en el destino, la evaluación falla con:
   ```text
   ConversionError
   ```
4. **Naturaleza de ConversionError**: `ConversionError` es un error de evaluación del lenguaje que representa el fracaso de una conversión de tipo. No constituye un valor, no forma parte del tipo normal (`T | ConversionError` no existe), no se envuelve en un tipo genérico `Result`, no puede asociarse a un binding ni capturarse desde dentro de Evo-Script v0.1.


### 9.5 Conversiones entre enteros

- **Ampliación garantizada**:
  ```text
  to_int128(int64_value) -> int128
  ```
- **Reducción potencialmente fallable**:
  ```text
  to_int64(int128_value) -> int64
  ```
  La conversión valida en tiempo de ejecución si el valor concreto cabe dentro del rango del tipo destino. Si cabe, produce `int64`; si no cabe, la evaluación falla con `ConversionError`.


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
  - `int32` $\rightarrow$ `uint32`: los valores negativos de `int32` no pueden representarse en `uint32`. `to_uint32(value)` produce `uint32` si el valor es positivo o cero; si es negativo, la evaluación falla con `ConversionError`.
  - `uint32` $\rightarrow$ `int32`: los valores de `uint32` mayores a $2^{31}-1$ no caben en `int32`. `to_int32(value)` produce `int32` si cabe; si no cabe, la evaluación falla con `ConversionError`.
  - `int8` $\rightarrow$ `uint8`: los valores negativos no pueden representarse en `uint8`. `to_uint8(value)` produce `uint8` si no es negativo; si es negativo, la evaluación falla con `ConversionError`.
  - `uint128` $\rightarrow$ `int128`: los valores de `uint128` mayores a $2^{127}-1$ no caben en `int128`. `to_int128(value)` produce `int128` si cabe; si no cabe, la evaluación falla con `ConversionError`.

Evo-Script no realiza reinterpretación de bits ni comportamiento de wrapping silencioso.


### 9.7 Conversiones de punto flotante

Las conversiones que involucran números de punto flotante consideran tanto el rango
como la precisión y la exactitud de la representación:

- **Entero a Float**: `to_float64(int64_value)` produce `float64` cuando el valor entero es exactamente representable en formato flotante. Si existe pérdida de información, la evaluación falla con `ConversionError`. No se realiza redondeo silencioso.
- **Float a Entero**: `to_int64(float_value)` produce `int64` cuando el valor cabe exactamente en el entero. Si no cabe o no es entero exacto, la evaluación falla con `ConversionError`. No se realiza truncamiento ni redondeo silencioso.
- **Float a Float**: `to_float32(float64_value)` produce `float32` si no existe pérdida de precisión; si existe pérdida, la evaluación falla con `ConversionError`.


### 9.8 Conversión a string (`to_string`)

La operación `to_string` permite convertir explícitamente valores a su representación textual:

    let string text = to_string(43);

Ningún valor se convierte automáticamente a texto. En Evo-Script v0.1 no se define parsing inverso desde texto hacia números (`parse_int`, `parse_float`).


### 9.9 Conversiones desde dynamic

Toda conversión explícita desde `dynamic` hacia cualquier tipo numérico de tamaño fijo
es **semánticamente potencialmente fallable**. Su tipo semántico normal es el tipo destino:

    to_int(dynamic_value)    -> int
    to_int8(dynamic_value)   -> int8
    to_int16(dynamic_value)  -> int16
    to_int32(dynamic_value)  -> int32
    to_int64(dynamic_value)  -> int64
    to_int128(dynamic_value) -> int128

    to_uint8(dynamic_value)   -> uint8
    to_uint16(dynamic_value)  -> uint16
    to_uint32(dynamic_value)  -> uint32
    to_uint64(dynamic_value)  -> uint64
    to_uint128(dynamic_value) -> uint128

    to_float(dynamic_value)   -> float
    to_float32(dynamic_value) -> float32
    to_float64(dynamic_value) -> float64

Reglas:

1. **Evaluación exitosa**: Si el valor concreto almacenado en `dynamic` puede representarse exactamente en el tipo destino según las reglas de rango y precisión, la evaluación produce directamente el valor en dicho tipo:
   ```text
   let dynamic value = 10;
   let int64 fixed_value = to_int64(value); // Produce int64 normalmente
   ```
   No existe ningún contenedor `Result` que deba ser desempaquetado.
2. **Evaluación fallida**: Si el valor concreto no puede representarse con exactitud en el tipo destino, la evaluación falla con `ConversionError`.
3. **Conversión a string**: `to_string(dynamic_value)` produce directamente `string` como representación textual explícita del valor dinámico. No se introducen métodos como `dynamic.as_int64` ni operadores de casteo (`as`, `cast`).


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
3. **Redondeo o truncamiento automático**: Las conversiones fallan explícitamente con `ConversionError` en lugar de distorsionar datos.
4. **Parsing inverso**: No se definen `parse_int` ni `parse_float`.
5. **Tipos no existentes**: No existe `float128` ni `to_float128`.
6. **Mecanismos alternativos**: No existen `convert_to_*` ni generic conversion functions.
7. **Genéricos y desempaquetado de Result**: Result no existe como tipo del lenguaje; no existen genéricos, `?`, `unwrap`, `expect` ni tipos unión (`T | E`).


## 10. Expresiones y operadores

### 10.1 Expresiones

Una **expresión** (`Expression`) es una construcción sintáctica y semántica que posee un tipo semántico normal y, al evaluarse correctamente, produce exactamente un valor de dicho tipo.

Ejemplos:

- Aritméticas: `price + tax`, `count * 2`
- Comparaciones: `age >= 18`, `first == second`
- Lógicas: `active && authorized`, `!disabled`
- Unarias: `-temperature`, `!ready`
- Conversiones: `to_int64(value)`

Reglas fundamentales de evaluación:

1. **Producción de valor normal**: Una evaluación válida produce exactamente un valor coincidente con el tipo semántico de la expresión.
2. **Fallo de evaluación**: Una expresión puede fallar durante su evaluación según las reglas normativas del lenguaje (por ejemplo, `ConversionError`, `OverflowError`, `DivisionByZeroError`).
3. **Ausencia de tipos unión o Result**: Un posible fallo de evaluación no altera el tipo normal de la expresión, no produce un segundo valor, no genera tipos unión (`T | Error`) ni envuelve el resultado en `Result<T, E>`.

Una expresión puede utilizarse directamente como el valor en una declaración `let`:

    let int total = price + tax;
    let bool allowed = active && age >= 18;


### 10.2 Tipado contextual y sintaxis de literales numéricos

Los literales numéricos en Evo-Script no se definen como valores previamente tipados que posteriormente deban convertirse. Un literal numérico expresa un valor textual en el código fuente y adquiere su tipo semántico directamente a partir del contexto numérico explícitamente requerido.

Ejemplo:

    let int64 value = 100;

En este caso, el literal `100` nace semánticamente como `int64`. No ocurre una conversión implícita `int -> int64` ni una promoción de tipos.


#### 10.2.1 Gramática conceptual de literales numéricos

Evo-Script v0.1 clasifica los literales numéricos en tres formas textuales:

```text
digit
    := "0".."9"

digits
    := digit+

integer_literal
    := digits

decimal_literal
    := digits "." digits

scientific_literal
    := (digits | decimal_literal) ("e" | "E") ("+" | "-")? digits
```

Reglas lexicales:
1. **Separador decimal**: El separador decimal es exclusivamente el punto (`.`). No se admite la coma (`,`) ni separadores dependientes del locale o configuración regional del entorno.
2. **Ausencia de separadores de dígitos**: No se admite el carácter guión bajo (`_`) dentro de literales numéricos (`1_000`, `1_000.25` o `1e1_000` son inválidos).
3. **Signo negativo y unario**: El signo negativo situado antes de un número (`-10.5` o `-1.5e10`) no forma parte de la gramática lexical del literal; constituye la aplicación del operador unario `-` sobre el literal (`-(10.5)`, `-(1.5e10)`).
4. **Ausencia de operador unario `+`**: El lenguaje no define un operador unario `+` (`+10.5` es inválido). El signo `+` solo es válido como marcador opcional dentro del exponente de un literal científico (`1e+10`).


#### 10.2.2 Literales enteros

Un literal entero está compuesto por una o más cifras decimales (`digits`):

    let int8 level = 5;
    let int64 population = 100;
    let int128 total = 500;
    let uint8 percentage = 100;
    let uint64 identifier = 1000;

Reglas:
1. **Representabilidad obligatoria**: El valor entero debe ser exactamente representable por el tipo requerido (por ejemplo, `let uint8 x = 100;` es válido; `let uint8 x = 300;` es inválido por exceder el rango de `uint8`).
2. **Sin contexto**: Un literal entero sin contexto de tipo explícito produce `int` (`i32`).


#### 10.2.3 Literales decimales de punto flotante

La forma decimal canónica de un literal de punto flotante requiere dígitos obligatorios tanto antes como después del punto decimal (`digits "." digits`):

    0.0
    0.5
    1.0
    10.5
    123.456
    1000.25

Reglas normativas:
1. **Dígito inicial obligatorio**: Formas decimales sin dígito entero previo al punto (como `.5`) son sintácticamente inválidas en Evo-Script v0.1; la forma canónica es `0.5`.
2. **Dígito posterior obligatorio**: Formas decimales sin dígitos tras el punto (como `5.`) son sintácticamente inválidas en Evo-Script v0.1; la forma canónica es `5.0`.
3. **Tipado contextual de punto flotante**: El literal decimal nace directamente con el tipo del contexto requerido:
   - `let float a = 10.5;` $\rightarrow$ nace directamente como `float` (`f64`).
   - `let float32 b = 10.5;` $\rightarrow$ nace directamente como `float32` (`f32`).
   - `let float64 c = 10.5;` $\rightarrow$ nace directamente como `float64` (`f64`).
   No existe una representación decimal intermedia ni conversión implícita posterior.
4. **Sin contexto**: Un literal decimal sin contexto explícito adopta el tipo por defecto `float` (`f64`).


#### 10.2.4 Literales de notación científica

Evo-Script v0.1 admite notación científica para literales de punto flotante mediante la sintaxis:

    mantissa ("e" | "E") ["+" | "-"] digits

donde la mantisa puede ser una secuencia entera de dígitos (`digits`) o un literal decimal canónico (`digits "." digits`).

Ejemplos válidos:

    1e10
    1E10
    1e+10
    1e-10

    1.5e10
    1.5E10
    1.5e+10
    1.5e-10

Reglas normativas:
1. **Naturaleza de punto flotante**: Todo literal en notación científica constituye semánticamente un literal de punto flotante, incluso cuando su mantisa no incluya punto decimal (por ejemplo, `1e10` es un literal de punto flotante, no un entero).
2. **Tipado contextual científico**: El literal científico adopta directamente el tipo requerido por el contexto:
   - `let float a = 1.5e10;` $\rightarrow$ nace directamente como `float` (`f64`).
   - `let float32 b = 1.5e-3;` $\rightarrow$ nace directamente como `float32` (`f32`).
   - `let float64 c = 1.5e10;` $\rightarrow$ nace directamente como `float64` (`f64`).
3. **Sin contexto**: Un literal en notación científica sin contexto explícito produce `float` (`f64`).
4. **Incompatibilidad con contexto entero**: Un literal científico no se asocia directamente a tipos enteros (`let int64 x = 1e10;` es inválido). Si se requiere convertir un valor flotante a entero, debe utilizarse una conversión explícita `to_tipo`.
5. **Exponentes completos obligatorios**: El exponente debe contener al menos un dígito posterior a `e`, `E` o al signo opcional (formas como `1e`, `1E`, `1e+`, `1e-`, `1.5e`, `1.5e+`, `1.5e-` son sintácticamente inválidas).
6. **Mantisas decimales canónicas**: Formas con mantisa incompleta como `.5e10` o `5.e10` son sintácticamente inválidas (las formas válidas son `0.5e10` y `5.0e10`).


#### 10.2.5 Formas y conceptos excluidos en literales

1. **Ausencia de sufijos de tipo**: Evo-Script v0.1 no admite sufijos en literales (`10.5f`, `10.5d`, `10.5f32`, `10.5f64`, `1e10f` son inválidos).
2. **Ausencia de literales especiales NaN e Infinity**: No existen identificadores o literales especiales como `NaN`, `nan`, `Infinity`, `infinity`, `inf` ni `-inf`.
3. **Ausencia de bases no decimales en flotantes**: No existen literales de punto flotante hexadecimales (`0x1.2p3`), binarios ni octales.
4. **Independencia de algoritmos internos de parsing**: La especificación define la sintaxis textual aceptada y la semántica de tipado, pero no exige un algoritmo interno particular de parsing binario ni se acopla a librerías específicas.


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

1. **Conservación de tipo normal**: La operación conserva como tipo normal el tipo de tamaño fijo declarado en sus operandos.
2. **Sin promoción automática**: El lenguaje nunca cambia silenciosamente el tipo para evitar un error (`int8 + int8` no se transforma en `int16`).
3. **Fallo con OverflowError**: Cuando una operación bajo un tipo fijo produce un valor fuera del rango representable, la evaluación falla con `OverflowError`.
4. **Sin wrapping modular**: Evo-Script no realiza wrapping silencioso (`127 + 1` en `int8` no produce `-128`).
5. **Sin saturación**: No se realiza saturación automática (`127 + 1` en `int8` no produce `127`).
6. **Negación unaria y rango**: La negación numérica `-value` sobre tipos fijos también produce `OverflowError` si el valor resultante no cabe en el tipo (por ejemplo, negar el valor mínimo representable en un entero con signo).
7. **Naturaleza de OverflowError**: `OverflowError` es un fallo de evaluación aritmética; no es un valor normal, no forma parte del tipo normal de la expresión (`int8 | OverflowError` no existe), no se envuelve en `Result` y no es capturable desde dentro de Evo-Script v0.1.


### 10.5 División y residuo entre cero (DivisionByZeroError)

Dividir entre cero o calcular el residuo con un divisor igual a cero no constituye una operación válida en Evo-Script. Cuando el segundo operando de una operación `/` o `%` es numéricamente igual a cero, la evaluación aritmética falla con:

    DivisionByZeroError

Reglas:

1. **Conservación de tipo normal**: Las operaciones `/` y `%` conservan su tipo semántico normal (`int`, `float64`, etc.). `DivisionByZeroError` no forma parte del tipo normal de retorno.
2. **Operaciones cubiertas**: Aplica de manera uniforme tanto a la división (`a / b`) como al residuo (`a % b`). No existen errores separados como `RemainderByZeroError` ni `ModuloByZeroError`.
3. **Tipos enteros**: Aplica a todos los tipos enteros (`int`, `int8`..`int128`, `uint8`..`uint128`).
   ```text
   let int64 value = 100;
   let int64 divisor = 0;
   let int64 result = value / divisor; // Falla la evaluación con DivisionByZeroError
   ```
4. **Punto flotante**: Aplica a todos los tipos flotantes (`float`, `float32`, `float64`). Evo-Script no produce silenciosamente `Infinity`, `+Infinity`, `-Infinity` ni `NaN` como resultado normal de una división entre cero.
   - Divisores `0.0` y `-0.0` se consideran numéricamente cero:
     ```text
     10.0 / 0.0  // Produce DivisionByZeroError
     10.0 / -0.0 // Produce DivisionByZeroError
     0.0 / 0.0   // Produce DivisionByZeroError
     0 / 0       // Produce DivisionByZeroError
     ```
5. **Tipo dynamic**: La evaluación bajo contexto `dynamic` no valida la división entre cero; la evaluación termina con `DivisionByZeroError` antes de producir un valor dynamic:
   ```text
   let dynamic result = 100 / 0; // Falla la evaluación con DivisionByZeroError
   ```
6. **Naturaleza de DivisionByZeroError**: Es un fallo de evaluación aritmética en tiempo de ejecución. No es un valor, no pasa por `return`, no genera tipos unión y no es capturable desde dentro de Evo-Script v0.1.


### 10.6 Evaluación numérica dinámica con dynamic

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
7. **Errores de evaluación en dynamic**: Si una operación dentro de una expresión `dynamic` resulta matemáticamente inválida (como división entre cero), la evaluación falla con el error correspondiente (`DivisionByZeroError`) antes de producir un valor `dynamic`.


### 10.7 Errores de evaluación y ausencia de captura

Evo-Script v0.1 establece una distinción conceptual estricta entre **alternativas de dominio** y **errores de evaluación del lenguaje**.

```
Categorías de Resultado
├── Alternativas de dominio (Domain Alternatives)
│   ├── Modeladas explícitamente mediante enum
│   ├── Constituyen valores normales (Values)
│   ├── Retornables con return e inspeccionables con when
│   └── Ejemplo: BuscarTrabajadorResultado::Error(string)
│
└── Errores de evaluación (Evaluation Errors)
    ├── Fallos de evaluación del lenguaje
    ├── No son valores normales ni tipos de datos
    ├── Detienen la evaluación y se propagan al host exterior
    └── Ejemplos: ConversionError, OverflowError, DivisionByZeroError
```


#### 10.7.1 Alternativas de dominio vs errores de evaluación

1. **Alternativas de dominio como valores**: Toda situación esperada y manejable por la lógica del programa debe modelarse como parte del contrato de tipos del dominio mediante un `enum` definido explícitamente:
   ```text
   enum BuscarTrabajadorResultado {
       Encontrado(Trabajador)
       NoEncontrado
       Error(string)
   }
   ```
   En este caso, la variante `Error(string)` es un **valor normal** del enum. El nombre `Error` no posee magia ni captura fallos del lenguaje; es simplemente una variante de datos que puede almacenarse en bindings, retornarse mediante `return` e inspeccionarse exhaustivamente mediante `when`.
2. **Errores de evaluación como fallos**: Los errores definidos por el lenguaje (`ConversionError`, `OverflowError`, `DivisionByZeroError`) representan **fallos de evaluación** causados por operaciones inválidas. No son valores, no son tipos de datos declarables y no forman parte del tipo normal de las expresiones.


#### 10.7.2 Naturaleza no tipable de los errores de evaluación

1. **Sin tipos visibles**: No existen tipos de datos como `EvaluationError`, `ArithmeticError`, `ConversionError` ni jerarquías de errores en el sistema de tipos de Evo-Script.
2. **Prohibición en declaraciones**:
   - No es válido declarar bindings de error: `let DivisionByZeroError err = ...;` (inválido).
   - No es válido recibir errores como argumentos: `fn manejar(DivisionByZeroError err) -> ...` (inválido).
   - No es válido declarar retornos de error: `fn test() -> DivisionByZeroError` o `return DivisionByZeroError;` (inválido).
3. **Sin tipos unión ni Result**: Una expresión como `a / b` posee como tipo semántico normal únicamente `int`. No posee tipo `int | DivisionByZeroError` ni `Result<int, DivisionByZeroError>`.


#### 10.7.3 Propagación del fallo en expresiones, bindings y funciones

Como Evo-Script v0.1 no define mecanismos de captura de errores de evaluación, un fallo ocurrido durante la evaluación detiene la producción de valores normales y se propaga hacia el exterior:

1. **Expresiones compuestas**: Si una subexpresión falla, la expresión completa que depende de ella no produce su valor normal. En `(a / b) + 10`, si `b == 0`, `a / b` falla con `DivisionByZeroError` y la suma exterior no se evalúa ni produce `int`.
2. **Declaraciones let**: En `let int value = a / b;`, si `a / b` falla con `DivisionByZeroError`, el binding `value` no llega a crearse con valores parciales, nulos o por defecto. La inicialización completa del binding falla.
3. **Declaraciones return**: `return expresion;` solo declara como resultado el valor producido exitosamente por `expresion`. Si la evaluación de `expresion` falla, el fallo ocurre **antes** de que `return` pueda completarse; la función no retorna un valor y el fallo se propaga al contexto que invocó la función.
4. **Llamadas entre funciones**: Si una función invocada falla durante su evaluación, la función llamadora tampoco produce su valor normal:
   ```text
   fn dividir(int a, int b) -> int {
       return a / b;
   }

   fn calcular(int a, int b) -> int {
       return dividir(a, b) + 10;
   }
   ```
   Si `b == 0`, `dividir(a, 0)` falla con `DivisionByZeroError` y `calcular(a, 0)` no produce un `int`.


#### 10.7.4 Propagación en pipelines y when

1. **Pipelines**: En una cadena monovalor `valor |> operacion_a |> operacion_b`, si `operacion_a` falla con un error de evaluación (por ejemplo `ConversionError`), ese stage no produce un valor, `operacion_b` no recibe entrada y el pipeline completo no produce un resultado normal. El placeholder contextual `this` no intercepta ni transforma errores.
2. **Expresiones when**: `when` inspecciona exclusivamente valores de tipo `enum`. Si la expresión asociada a la variante activa falla durante su evaluación (por ejemplo `Resultado::Numero(0) => 100 / value`), la expresión `when` completa falla con `DivisionByZeroError`. `when` no busca otras variantes como respaldo ni captura errores del lenguaje.
3. **Ausencia de enum de captura automática**: Definir un enum como:
   ```text
   enum DivisionResultado {
       Correcto(int)
       DivisorInvalido
   }
   ```
   no captura automáticamente un `DivisionByZeroError` producido por `/`. El lenguaje no transforma fallos de evaluación en variantes de enum a posteriori.


#### 10.7.5 Ausencia de mecanismos de captura y control de excepciones

Evo-Script v0.1 **no define mecanismos de captura ni manejo de excepciones**:

1. **Sin try / catch / throw**: No existen palabras clave `try`, `catch`, `throw`, `finally` ni sintaxis de lanzamiento/captura manual.
2. **Sin recuperación ni fallback**: No existen operadores ni funciones como `recover`, `rescue`, `fallback`, `or_else`, `on_error` o `otherwise_error` para errores de evaluación.
3. **Sin inspección booleana**: Los errores de evaluación no pueden convertirse a `bool` ni inspeccionarse mediante APIs como `has_error` o `is_error`.
4. **Modelado en el origen**: Si una operación requiere que un resultado no exitoso sea manejable por el programa, dicha operación debe diseñar su contrato semántico desde el origen mediante un tipo `enum` de dominio, en lugar de intentar capturar un fallo del lenguaje a posteriori.


#### 10.7.6 Límite exterior de evaluación e independencia de implementación

1. **Límite exterior**: Al no existir captura interna en el lenguaje, todo error de evaluación se propaga hasta el límite exterior del entorno que inició la ejecución de Evo-Script (el host o runtime), el cual es responsable de registrar o reportar el diagnóstico correspondiente.
2. **Independencia de implementación Rust**: La ausencia de tipos `Result` y excepciones en la semántica visible de Evo-Script no prohíbe que la implementación interna del compilador, parser, runtime o crates Rust (`evo-values`, `evo-query`, `evo-shell`, providers) utilice `Result<T, E>` u otros patrones técnicos de Rust para implementar el comportamiento del lenguaje.


### 10.8 Operadores de comparación

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


### 10.9 Operadores lógicos

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


### 10.10 Operadores unarios

Evo-Script v0.1 define dos operadores unarios prefijos:

| Operador | Significado | Tipo aplicable | Ejemplo |
| :--- | :--- | :--- | :--- |
| `!` | Negación lógica | `bool` | `!valid` |
| `-` | Negación numérica | Numéricos con signo | `-10`, `-delta` |

Ejemplo:

    let int temperature = -10;

No se incluyen operadores unarios de incremento (`++`), decremento (`--`), complemento bit a bit (`~`), referencias (`&`) ni desreferencia (`*`).


### 10.11 Agrupación y precedencia

Los paréntesis `( )` permiten agrupar expresiones para controlar explícitamente el orden de evaluación:

    (a + b) * c

El uso de paréntesis tiene como único propósito la agrupación sintáctica; no define tuplas ni tipos compuestos.

Evo-Script v0.1 define la jerarquía completa de precedencia y asociatividad de operadores de la siguiente manera (de mayor a menor precedencia):

1. **Operadores unarios prefijos** (`!`, `-`) — asociatividad por la derecha.
2. **Multiplicativos** (`*`, `/`, `%`) — asociatividad por la izquierda.
3. **Aditivos** (`+`, `-`) — asociatividad por la izquierda.
4. **Comparaciones** (`<`, `<=`, `>`, `>=`, `==`, `!=`) — no encadenables.
5. **Conjunción lógica** (`&&`) — asociatividad por la izquierda.
6. **Disyunción lógica** (`||`) — asociatividad por la izquierda.
7. **Pipeline** (`|>`) — asociatividad por la izquierda (menor precedencia de todos los operadores).

Ejemplos:

- `a + b * c` equivale semánticamente a `a + (b * c)`.
- `a > 10 && b < 20` equivale semánticamente a `(a > 10) && (b < 20)`.
- `a + b |> to_string` equivale semánticamente a `(a + b) |> to_string`.
- `a > b |> to_string` equivale semánticamente a `(a > b) |> to_string`.


### 10.12 Pipeline (`|>`) y placeholder contextual `this`

Evo-Script define la sintaxis y la semántica universal del operador de composición secuencial de pipelines:

    |>

y del placeholder contextual:

    this

El pipeline en Evo-Script es estrictamente **monovalor** y opera mediante composición secuencial de izquierda a derecha.


#### 10.12.1 Principio de pipeline monovalor

1. **Un solo valor transportado**: Un pipeline transporta exactamente un valor en cada etapa. Cada stage consume exactamente un valor proveniente de la izquierda del pipe y produce exactamente un valor que se convierte en la entrada del siguiente stage.
2. **Valores estructurados**: El principio monovalor aplica a todo tipo de valor semántico en Evo-Script (tipos nativos, `struct`, `enum`, `dynamic`). Un `struct Trabajador` o un `enum BuscarTrabajadorResultado` transportado por un pipeline cuenta como exactamente un valor.
3. **Ausencia de pipes multivalor**: El pipeline no transporta múltiples parámetros, tuplas implícitas, listas de argumentos ni valores múltiples (`(a, b) |> funcion` no existe como mecanismo de paso múltiple).


#### 10.12.2 Stage de un solo argumento (Aridad 1)

Para operaciones cuya firma requiere exactamente un argumento:

    fn to_string(int value) -> string

la forma canónica y obligatoria en el pipeline es:

    valor |> operacion

Ejemplo:

    100 |> to_string

Reglas:
- No se permite `valor |> operacion(this)` ni `valor |> operacion()`. Evo-Script define una única forma canónica para operaciones de aridad 1.
- Semánticamente, el stage recibe el valor transportado y lo aplica como único argumento de la operación.


#### 10.12.3 Stage de dos o más argumentos (Aridad >= 2) y placeholder `this`

Para operaciones cuya firma requiere dos o más argumentos, el valor transportado por el pipeline debe declararse explícitamente mediante el placeholder contextual `this`:

    valor |> operacion(this, argumento2, ..., argumentoN)

Ejemplo:

    fn sumar(int a, int b) -> int {
        return a + b;
    }

    let int resultado = 10 |> sumar(this, 20);

Reglas normativas de `this`:

1. **Obligatoriedad en aridad >= 2**: Para operaciones de dos o más argumentos, el uso de `this` es obligatorio. La forma implícita `10 |> sumar(20)` es inválida.
2. **Exclusivamente en el primer argumento**: `this` solo puede ocupar la primera posición de la lista de argumentos del stage. Las formas `sumar(20, this)` o `concat(a, this, b)` son inválidas.
3. **Exactamente una aparición**: `this` debe aparecer exactamente una vez en el stage. Las formas `sumar(this, this)` o `concat(this, " ", this)` son inválidas.
4. **Primer argumento completo**: `this` debe ser directamente el primer argumento completo del stage. No se permiten subexpresiones que contengan `this` (como `sumar(this + 1, 20)` o `funcion(to_string(this), 20)`). Para transformar previamente el valor transportado, debe utilizarse un stage previo en el pipeline.
5. **Naturaleza de placeholder contextual**: `this` no es una variable, no es un binding, no es un parámetro, no es un campo y no representa un objeto actual ni referencia `self`/`this` de programación orientada a objetos. Su significado existe exclusivamente dentro de `operacion(this, ...)` como placeholder del valor transportado por el pipeline inmediato. Fuera de este contexto (`let int x = this;`, `return this;`), `this` es inválido.
6. **Argumentos adicionales**: Los argumentos posteriores a `this` (`argumento2`, `argumento3`, etc.) son expresiones normales del lenguaje evaluadas en su propio contexto.


#### 10.12.4 Pipelines anidados y scopes de `this`

Un argumento adicional en un stage de pipeline puede ser a su vez una expresión pipeline:

```text
nombre
|> concat(
    this,
    " ",
    apellido |> limpiar
)
```

O agrupado explícitamente con paréntesis:

```text
nombre
|> concat(
    this,
    " ",
    (apellido |> limpiar)
)
```

Reglas:
- Cada pipeline posee su propio contexto de valor transportado.
- En pipelines anidados, cada aparición de `this` se resuelve estrictamente contra el pipeline inmediato al que pertenece. No existe captura de `this` entre pipelines externos e internos.


#### 10.12.5 Asociatividad y composición secuencial

1. **Asociatividad por la izquierda**: El operador `|>` es asociativo de izquierda a derecha. Una cadena como:
   ```text
   valor
   |> operacion_a
   |> operacion_b
   |> operacion_c
   ```
   se evalúa secuencialmente alimentando la salida de `operacion_a` hacia `operacion_b`, y la salida de esta hacia `operacion_c`.
2. **Equivalencia semántica**: Para funciones ordinarias de un argumento, `a |> f |> g` equivale conceptualmente a `g(f(a))`.


#### 10.12.6 Compatibilidad de tipos en stages

1. **Validación estricta de tipos**: El tipo del valor producido por la parte izquierda de `|>` debe ser exactamente compatible con el tipo esperado por el primer parámetro del stage derecho.
2. **Ausencia de conversiones implícitas**: El pipeline no efectúa coerciones ni conversiones automáticas entre stages. Si se requiere transformar el tipo, debe intercalarse un stage explícito de la familia `to_tipo`.
3. **Stage de cero argumentos inválido**: Una operación que no recibe argumentos no puede participar como stage de pipeline, ya que no posee un parámetro para consumir el valor transportado.


#### 10.12.7 Composición con let, return y when

1. **Con `let`**: Un pipeline puede inicializar un binding inmutable:
   ```text
   let string texto = 100 |> to_int64 |> to_string;
   ```
2. **Con `return`**: Una función puede declarar una expresión pipeline como su resultado:
   ```text
   fn calcular_texto(int a, int b) -> string {
       return a + b
           |> to_string;
   }
   ```
   Debido a la menor precedencia de `|>`, `return a + b |> to_string;` evalúa `(a + b) |> to_string` antes de entregarlo a `return`.
3. **Con `when`**: Las ramas de una correspondencia `when` pueden producir expresiones pipeline:
   ```text
   return when resultado {
       Resultado::Numero(int value) => value |> to_string
       Resultado::Texto(string value) => value
   };
   ```


#### 10.12.8 Relación arquitectónica con EvoQ

1. **Independencia semántica**: Evo-Script define la semántica universal y el mecanismo de composición de `|>`, `this` y pipelines. EvoQ define operaciones semánticas de consulta que pueden participar en pipelines, pero no redefine el operador ni el lenguaje.
2. **Operaciones conceptuales**: Nombres como `filter`, `select`, `take`, `skip`, `first`, `last`, `count` o `concat` no constituyen palabras clave reservadas de Evo-Script en v0.1; pertenecen a sus respectivos sistemas semánticos o funciones del programa.


#### 10.12.9 Ejemplo canónico completo

```text
fn sumar(int value, int amount) -> int {
    return value + amount;
}

fn multiplicar(int value, int factor) -> int {
    return value * factor;
}

fn calcular(int value) -> string {
    return value
        |> sumar(this, 20)
        |> multiplicar(this, 2)
        |> to_string;
}
```

Flujo conceptual para `calcular(10)`:
$$\text{10} \xrightarrow{\text{sumar(10, 20)}} \text{30} \xrightarrow{\text{multiplicar(30, 2)}} \text{60} \xrightarrow{\text{to\_string(60)}} \text{"60"}$$


### 10.13 Ausencia de operadores de asignación y mutación

Debido a la inmutabilidad intrínseca de los bindings, Evo-Script no posee operadores generales de asignación ni mutación:

- No existen operadores de asignación ni asignación compuesta (`=`, `+=`, `-=`, `*=`, `/=`, `%=`).
- El símbolo `=` aparece exclusivamente dentro de la sentencia `let tipo nombre = valor;` como ligadura inicial; no constituye una sentencia de asignación.
- No existen operadores de incremento ni decremento (`++`, `--`).

Ejemplos inválidos:

    age = 44;     // Inválido
    age += 1;     // Inválido
    age++;        // Inválido
    ++age;        // Inválido


### 10.14 Correspondencia exhaustiva de enums con when

Evo-Script no utiliza estructuras imperativas de control de flujo (`if`, `else`, `switch`, `case`, `for`, `while`, `loop`). La inspección y el consumo de valores de un tipo `enum` se modelan como una correspondencia declarativa y exhaustiva entre el conjunto cerrado de alternativas del enum y expresiones que producen un valor, mediante la palabra clave:

    when

`when` es una **expresión** (`Expression`) que evalúa exactamente un valor de tipo `enum` definido por el programa y produce exactamente un valor.

Forma general:

    when valor_enum {
        TipoEnum::VarianteA
            => expresion_a

        TipoEnum::VarianteB(Tipo valor)
            => expresion_b
    }

El símbolo `=>` actúa exclusivamente como **marcador de correspondencia** dentro de `when`. No constituye un operador aritmético, lógico, de comparación, de asignación ni de pipeline, y no forma parte de la jerarquía de precedencia de operadores. Fuera de `when`, el símbolo `=>` no posee significado en Evo-Script v0.1.


#### 10.14.1 Reglas de exhaustividad y correspondencia

1. **Exhaustividad obligatoria**: Una expresión `when` debe cubrir todas las variantes declaradas en el tipo enum inspeccionado. Si falta alguna variante, el programa es semánticamente inválido.
2. **Sin duplicados**: Cada variante del enum debe aparecer exactamente una vez. No pueden repetirse correspondencias para una misma variante.
3. **Ausencia de default y comodines**: No existen palabras clave `default`, `otherwise`, `else` ni comodines de captura general `_`. Todas las correspondencias deben declarar explícitamente la variante que atienden.
4. **Referencia canónica obligatoria**: Cada correspondencia debe utilizar el calificador completo `TipoEnum::Variante`. No se admiten nombres de variantes no calificados dentro de `when`.
5. **Valor producido y consistencia de tipos**: `when` produce exactamente un valor. Todas las expresiones asociadas mediante `=>` deben producir el mismo tipo semántico. Si una rama produce `string` y otra `int`, la expresión es inválida. No existen tipos unión (`T | E`) ni promociones automáticas para ocultar incompatibilidades.
6. **Conversiones explícitas**: Si las correspondencias necesitan producir un tipo común a partir de tipos distintos, deben utilizar explícitamente la familia `to_tipo`.
7. **Tipado contextual de literales**: Si el resultado de `when` se asigna a un binding explícito (`let int64 value = when ...`), los literales numéricos en las expresiones de correspondencia adquieren dicho tipo contextual.


#### 10.14.2 Inspección y extracción según tipo de variante

1. **Variantes simples**: Se corresponden directamente sin paréntesis ni parámetros:
   ```text
   Estado::Activo => "activo"
   ```
2. **Variantes con valor asociado**: Extraen el valor asociado mediante la sintaxis oficial `tipo primero, nombre después`:
   ```text
   BuscarTrabajadorResultado::Encontrado(Trabajador trabajador)
       => describir_trabajador(trabajador)

   BuscarTrabajadorResultado::Error(string message)
       => message
   ```
   El identificador (`trabajador`, `message`) crea un binding inmutable estrictamente local a la expresión correspondiente a esa variante. Fuera de esa correspondencia, el binding no está visible.
3. **Inmutabilidad y prohibición de shadowing en extracciones**: Los bindings extraídos por `when` son inmutables y no pueden reasignarse ni sombrear nombres ya visibles en el ámbito exterior.
4. **Ausencia de patrones anidados o alias**: No se admite desestructuración anidada (como `Encontrado(Trabajador { ... })`) ni sintaxis de captura adicional (`as`, `@`, `bind`).
5. **Variantes estructuradas**: Extraen explícitamente todos sus campos mediante la sintaxis de campos con bindings tipados:
   ```text
   OperacionResultado::Error {
       message: string error_message;
       code: int error_code;
   }
       => error_message
   ```
   En Evo-Script v0.1, una correspondencia sobre una variante estructurada debe extraer todos sus campos declarados. No se permite extracción parcial ni patrones de resto (`..`, `_`).


#### 10.14.3 Ausencia de control condicional y guards

1. **Solo tipos enum**: `when` solo acepta expresiones cuyo tipo sea un `enum` definido. No acepta valores booleanos (`when active { ... }`), numéricos ni cadenas.
2. **Sin guards condicionales**: No se permite sintaxis de guards (`Variante if condicion => ...` ni `Variante when condicion => ...`). La selección se realiza exclusivamente por la variante activa del enum.
3. **No sustituye a comparaciones booleanas**: La inspección semántica del enum se realiza mediante `when`, no mediante comparaciones de igualdad (`resultado == Tipo::Variante`).


#### 10.14.4 Expresión when y resultado de función

1. **Expresión when en return**: `when` es una `Expression` y no retorna implícitamente de una función. Para constituir el resultado de una función, debe declararse explícitamente mediante `return when ...;`.
2. **Ausencia de return en ramas**: No se permite utilizar `return` dentro de las ramas individuales de `when` (`Variante => return valor;` es inválido). `when` produce el valor y `return` declara que ese valor es el resultado de la función.
3. **Terminación con punto y coma**: Cuando `when` forma parte de una declaración `let` o de una sentencia `return`, la declaración completa termina con `;` (`let string mensaje = when ... ;` o `return when ... ;`). No se coloca `;` al final de cada correspondencia individual `Variante => expresion`.


#### 10.14.5 Ejemplo completo canónico

```text
struct Trabajador {
    int id;
    string name;
}

enum BuscarTrabajadorResultado {
    Encontrado(Trabajador)
    NoEncontrado
    Error(string)
}

fn buscar_trabajador(int id) -> BuscarTrabajadorResultado {
    correspondencia
}

fn describir_trabajador(Trabajador trabajador) -> string {
    correspondencia
}

fn obtener_mensaje(BuscarTrabajadorResultado resultado) -> string {
    return when resultado {
        BuscarTrabajadorResultado::Encontrado(Trabajador trabajador)
            => describir_trabajador(trabajador)

        BuscarTrabajadorResultado::NoEncontrado
            => "Trabajador no encontrado"

        BuscarTrabajadorResultado::Error(string message)
            => message
    };
}

let BuscarTrabajadorResultado resultado =
    buscar_trabajador(10);

let string mensaje =
    obtener_mensaje(resultado);
```


### 10.15 Operadores y conceptos excluidos de v0.1

Quedan formalmente fuera de Evo-Script v0.1:

1. **Operaciones Bitwise**: No existen `&`, `|`, `^`, `<<`, `>>` ni funciones como `bit_and`, `bit_or`, `shift_left`.
2. **Mutación y asignación**: No existen `=`, `+=`, `++`, `--`.
3. **Casteo**: No existen `as` ni `cast` (se utiliza exclusivamente la familia `to_tipo`).
4. **Punteros y referencias**: No existen operadores `&` ni `*`.
5. **Tipos no numéricos en dynamic**: `dynamic` no soporta structs, enums ni dispatch polimórfico dinámico.
6. **Tipos enteros artificiales visibles**: No existen `bigint`, `int256` ni `int512`.
7. **Control condicional imperativo y pattern matching general**: No existen `if`, `else`, `switch`, `case`, `for`, `while`, `loop`, `match`, guards, wildcards `_`, rangos ni destructuring anidado.
8. **Conceptos orientados a objetos y tuplas**: No existen clases, métodos, constructores, interfaces, herencia, `self`, `this` orientado a objetos, tuplas ni pipes multivalor.
9. **Funciones como valores y closures**: No existen lambdas, funciones anónimas, clausuras ni tipos función como valores de primer orden.
10. **Manejo de excepciones y captura de errores**: No existen `try`, `catch`, `throw`, `finally`, `recover`, tipos de excepción ni captura de errores de evaluación dentro del lenguaje.
11. **Sintaxis no canónica en literales numéricos**: No existen sufijos de tipo (`10.5f`, `10.5f32`), separadores `_` en números (`1_000`), decimales incompletos (`.5`, `5.`), mantisas científicas incompletas (`.5e10`, `5.e10`), exponentes incompletos (`1e`), flotantes hexadecimales (`0x1.2p3`) ni literales especiales `NaN` o `Infinity`.


### 10.16 Semántica pendiente

Dentro del alcance delimitado para Evo-Script v0.1, no existen temas semánticos pendientes en el subsistema de expresiones y operadores.


## 11. Funciones

La unidad semántica fundamental de ejecución y cómputo se denomina `Function`.

En Evo-Script v0.1 existen dos representaciones para funciones:

1. **Implementación de función (`Function Implementation`)**: declarada dentro de archivos `.efn`, con visibilidad explícita (`public fn` o `private fn`), cuerpo delimitado por `{ ... }` y un único `return expresion;` obligatorio.
2. **Firma de función (`Function Signature`)**: declarada dentro de archivos `.esig`, con la forma `fn nombre(argumentos) -> Tipo;`, sin cuerpo ni modificadores de visibilidad.

La forma textual general de una implementación de función en Evo-Script v0.1 es:

    public fn nombre(tipo argumento, tipo argumento) -> tipo {
        cero_o_mas_bindings_let
        return expresion;
    }

o para funciones auxiliares privadas:

    private fn nombre(tipo argumento, tipo argumento) -> tipo {
        cero_o_mas_bindings_let
        return expresion;
    }

Ejemplo canónico:

    public fn sumar(int numero, int numero2) -> int {
        return numero + numero2;
    }


### 11.1 Declaración y visibilidad

La palabra clave `fn` inicia textualmente la declaración de una función.

En archivos de implementación `.efn`, la visibilidad de cada función debe ser explícita:

- `public`: declara la única función pública principal del archivo `.efn`.
- `private`: declara funciones auxiliares de uso estrictamente local dentro del mismo archivo `.efn`.

No existe visibilidad implícita. En firmas públicas `.esig`, la función es pública por naturaleza y se declara directamente como `fn nombre(...) -> Tipo;` sin modificadores de visibilidad.


### 11.2 Nombre

En la declaración:

    public fn guardar(...)

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

    public fn ejemplo(int id, float amount, Trabajador trabajador) -> ...


### 11.4 Tipo de resultado

La cláusula:

    -> tipo

declara explícitamente el contrato de tipo producido por la función. Toda función en Evo-Script v0.1 declara su tipo de resultado de forma explícita. No existe inferencia del tipo de resultado de una función.


### 11.5 Declaración explícita de resultado con return

Evo-Script v0.1 exige que toda función declare explícitamente cuál es la expresión que produce su resultado mediante la palabra clave:

    return

Sintaxis oficial:

    return expresion;

Reglas normativas:

1. **Declaración explícita de resultado**: `return expresion;` declara formalmente que `expresion` constituye el valor producido por la función.
2. **Ausencia de retorno implícito**: Evo-Script no infiere el resultado a partir de la última expresión del cuerpo. Una función sin `return` (por ejemplo, `{ a + b }`) es semánticamente inválida.
3. **Exactamente un return por función**: Toda función debe contener exactamente un `return`. No se permiten cero ni múltiples sentencias `return`.
4. **Último elemento de la correspondencia**: `return expresion;` debe ser estrictamente el último elemento dentro del cuerpo de la función. No se admiten declaraciones, bindings ni expresiones posteriores al `return`.
5. **Ausencia de early return y control imperativo**: `return` no constituye una sentencia de bifurcación, salto o interrupción temprana (`early exit`/`jump`). No existen múltiples puntos de retorno ni sentencias de control condicional (`if`, `else`, `switch`).
6. **Compatibilidad estricta de tipos**: El tipo producido por la expresión en `return expresion;` debe coincidir exactamente con el tipo declarado en `-> tipo`. Si los tipos no coinciden (por ejemplo, `return a + b;` con `-> string`), el programa es inválido. Si se requiere convertir el tipo, debe utilizarse explícitamente `to_tipo`.
7. **Terminación obligatoria con punto y coma**: La sentencia `return` debe terminar obligatoriamente con punto y coma (`;`).
8. **Ámbito de return**: `return` solo tiene significado dentro de la correspondencia de una `Function`. No es un operador ni puede utilizarse a nivel global.


### 11.6 Estructura de la correspondencia (Correspondence)

La correspondencia de una función está delimitada por llaves `{ ... }` y posee la siguiente estructura:

    Function Implementation (.efn)
    ├── visibility (public | private)
    ├── name
    ├── arguments
    ├── result type (-> Tipo)
    └── correspondence
        ├── cero o más let bindings inmutables
        └── exactamente un return expresion;

Ejemplos válidos:

- **Función pública directa**:
  ```text
  public fn multiplicar(int a, int b) -> int {
      return a * b;
  }
  ```
- **Función con bindings intermedios**:
  ```text
  public fn calcular_total(int precio, int impuesto) -> int {
      let int subtotal = precio + impuesto;
      let int resultado = subtotal * 2;

      return resultado;
  }
  ```
- **Función con pipeline**:
  ```text
  public fn sumar_texto(int a, int b) -> string {
      return a + b
          |> to_string;
  }
  ```
- **Función con when**:
  ```text
  public fn obtener_mensaje(BuscarTrabajadorResultado resultado) -> string {
      return when resultado {
          BuscarTrabajadorResultado::Encontrado(Trabajador trabajador)
              => trabajador.name

          BuscarTrabajadorResultado::NoEncontrado
              => "Trabajador no encontrado"

          BuscarTrabajadorResultado::Error(string message)
              => message
      };
  }
  ```


### 11.7 Sintaxis y semántica

Existe una separación estricta entre la representación textual y la semántica del lenguaje:

| Sintaxis | Semántica |
| :--- | :--- |
| `public fn` | `Public principal function in .efn` |
| `private fn` | `File-local helper function in .efn` |
| `fn ... -> Tipo;` | `Function signature in .esig` |
| `tipo nombre` | `Argument` |
| `-> tipo` | `Result type declaration` |
| `return expresion;` | `Explicit function result declaration` |
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
| `ConversionError` | `Explicit type conversion evaluation error` |
| `valor \|> to_tipo` | `Composed explicit conversion` |
| `literal numérico` | `Contextual numeric literal` |
| `10.5` | `Decimal floating-point literal` |
| `1.5e10` | `Scientific floating-point literal` |
| `let int64 x = 100;` | `Literal typed as int64 by context` |
| `let dynamic x = expresión;` | `Dynamic numeric binding` |
| `OverflowError` | `Fixed-width arithmetic overflow` |
| `DivisionByZeroError` | `Zero-divisor arithmetic evaluation error` |
| `a + b` | `Addition expression` |
| `a - b` | `Subtraction expression` |
| `a * b` | `Multiplication expression` |
| `a / b` | `Division expression` |
| `a % b` | `Remainder expression` |
| `a / 0` | `Division by zero evaluation error` |
| `a % 0` | `Division/remainder by zero evaluation error` |
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
| `valor \|> operacion` | `Single-input pipeline stage` |
| `valor \|> operacion(this, arg)` | `Explicit pipeline input with additional arguments` |
| `this` | `Contextual pipeline input placeholder` |
| `when valor { ... }` | `Exhaustive enum correspondence expression` |
| `Tipo::Variante => expresion` | `Variant-to-value correspondence` |
| `Tipo::Variante(Tipo binding)` | `Associated enum value extraction` |
| `Tipo::Variante { campo: Tipo binding; }` | `Structured variant field extraction` |
| `=>` | `Correspondence marker inside when` |


## 12. Modelo de archivos, módulos y ejecución

Evo-Script v0.1 define formalmente el modelo de archivos, artefactos modulares y fronteras de ejecución guiado por un principio fundamental:

> **Principio de distribución estructural**: La complejidad estructural aparece únicamente cuando aparece distribución estructural.

El lenguaje permite dos modalidades de uso:

1. **Script autocontenido**: Un único archivo `.efn` que contiene toda su lógica, tipos locales y funciones auxiliares, ejecutable de forma directa sin requerir infraestructura de proyecto.
2. **Proyecto estructurado**: Un conjunto de artefactos especializados con responsabilidades delimitadas (`.root`, `.main` / `.elib`, `.emod`, `.esig`, `.estc`, `.enum`, `.efn`).


### 12.1 Extensiones oficiales y responsabilidades

Cada extensión de archivo en Evo-Script expresa su responsabilidad semántica principal:

| Extensión | Nombre | Responsabilidad semántica principal |
| :--- | :--- | :--- |
| `.efn` | Evo Function | Implementación de función o script autocontenido ejecutable |
| `.esig` | Evo Signature | Contrato público de una función (firma sin cuerpo) |
| `.estc` | Evo Struct | Definición compartible de struct |
| `.enum` | Enum | Definición compartible de enum |
| `.emod` | Evo Module | Módulo, frontera semántica y catálogo de firmas públicas |
| `.root` | Evo Project Root | Raíz de resolución de un proyecto estructurado |
| `.main` | Evo Application Entry | Selección del punto de entrada ejecutable de una aplicación |
| `.elib` | Evo Library | Agrupación semántica reutilizable de módulos de librería |
| `.evo` | Evo Package | Artefacto distribuible / paquete del ecosistema Evo |

Regla de exclusión de extensiones alternativas:
- La extensión oficial para módulos es estrictamente `.emod` (no se admite `.mod`).
- No existen extensiones no oficiales como `.evo-script`, `.evostruct`, `.evoenum` ni `.efun`.


### 12.2 Script autocontenido (.efn)

Un archivo `.efn` puede constituir por sí mismo un programa Evo-Script completamente ejecutable.

Reglas normativas:

1. **Autocontención**: Un script `.efn` no requiere `.root`, `.main`, `.emod`, `.esig`, `.estc`, `.enum`, `.elib` ni `.evo` si toda la semántica requerida está contenida dentro del propio archivo.
2. **Exactamente una función pública**: Un archivo `.efn` contiene exactamente una función `public fn` principal. No se permiten múltiples funciones `public` en el mismo `.efn`.
3. **Cero o más funciones privadas**: Un archivo `.efn` puede declarar cero o más funciones `private fn` auxiliares.
4. **Visibilidad explícita obligatoria**: Toda función dentro de un `.efn` debe declarar explícitamente `public` o `private`. No existe visibilidad implícita ni por defecto.
5. **Tipos locales**: Un `.efn` puede declarar definiciones locales de `struct` y `enum`. Estos tipos pertenecen exclusivamente al archivo y no requieren `.estc` ni `.enum` mientras no crucen fronteras hacia otros archivos Evo-Script.
6. **Participación de tipos locales en la frontera con el host**: Los tipos locales pueden utilizarse libremente como argumentos o tipo de retorno de la `public fn` principal, en bindings intermedios, en `return` y en `when`. El host/runtime que ejecuta el script recibe el valor semántico resultante.
7. **Ausencia de `.esig` en scripts directos**: La `public fn` de un script `.efn` ejecutado directamente por el host no requiere una firma `.esig`.


### 12.3 Visibilidad de funciones dentro de .efn

Los modificadores de visibilidad `public` y `private` aplican exclusivamente a funciones declaradas dentro de archivos `.efn`:

- `public fn`: Declara la operación principal del archivo. Constituye el punto de interacción externo frente al host que ejecuta el script o frente a la firma `.esig` que implementa.
- `private fn`: Declara una función auxiliar interna. Solo puede ser invocada por otras funciones dentro del mismo archivo `.efn`. No puede ser accedida desde otro archivo, no puede registrarse en un `.emod` ni puede referenciarse mediante `.esig`.

En Evo-Script v0.1 no se aplican modificadores `public`/`private` a structs, enums ni signatures en archivos externos; su visibilidad se rige por su presencia modular.


### 12.4 Tipos locales vs tipos compartidos (.estc y .enum)

Evo-Script distingue formalmente entre tipos locales a un archivo y tipos compartidos entre múltiples archivos:

1. **Tipo local**: Declarado dentro de un archivo `.efn`. Solo existe en el ámbito léxico de ese archivo. Ningún otro archivo Evo-Script puede referenciarlo ni nombrarlo (`let TipoLocal x = ...` en otro archivo es inválido).
2. **Tipo compartido**: Cuando un tipo de datos necesita participar en comunicaciones entre distintos archivos Evo-Script (por ejemplo, en los argumentos o resultado de una `.esig`), debe extraerse a su propio archivo especializado:
   - Struct compartido $\rightarrow$ archivo `.estc` (contiene una única definición `struct Nombre { ... }`).
   - Enum compartido $\rightarrow$ archivo `.enum` (contiene una única definición `enum Nombre { ... }`).
3. **Principio de frontera**:
   - Tipo utilizado únicamente dentro del mismo `.efn` $\rightarrow$ permanece local.
   - Tipo que cruza una frontera entre archivos Evo-Script $\rightarrow$ debe residir en `.estc` o `.enum`.


### 12.5 Firmas públicas de funciones (.esig)

Un archivo `.esig` (Evo Signature) declara formal y exclusivamente el contrato público de una función:

```text
fn nombre(tipo argumento, tipo argumento) -> Tipo;
```

Reglas normativas:

1. **Sintaxis de firma**: La declaración consiste en `fn`, nombre, lista de argumentos tipados, cláusula `-> Tipo` y punto y coma final (`;`).
2. **Ausencia de cuerpo**: Un archivo `.esig` no posee cuerpo `{ ... }`, correspondencia ni sentencias `return`.
3. **Pública por naturaleza**: Toda firma en un `.esig` es intrínsecamente pública. No admite modificadores `public` ni `private`.
4. **Contrato de acción, no interfaz de objeto**: `.esig` modela directamente la acción requerida, no una interfaz de clase u objeto. Evo-Script no define `interface`, `trait` ni `dyn`.
5. **No crea funciones como valores**: `.esig` define un contrato invocable, no un valor de primer orden de tipo función. No introduce lambdas, clausuras ni tipos función manipulables como datos.
6. **Tipos permitidos**: Una `.esig` solo puede utilizar tipos nativos o tipos compartidos (`.estc`, `.enum`). No puede utilizar tipos locales de un `.efn`.


### 12.6 Comunicación entre archivos y satisfacción de contratos

Evo-Script prohíbe el acoplamiento directo entre implementaciones:

1. **Prohibición de acceso directo `.efn` $\rightarrow$ `.efn`**: Un archivo de implementación `.efn` nunca puede depender ni invocar directamente a otro archivo `.efn`.
2. **Canal de comunicación formal**: La comunicación entre archivos distintos ocurre exclusivamente a través de firmas `.esig` catalogadas en módulos `.emod`:
   ```text
   consumer.efn
        │
        │ requiere una operación
        ▼
   module.emod
        │
        │ ofrece
        ▼
   operation.esig
        │
        ├──► input.estc
        │
        ├──► result.enum
        │
        │ satisfecha por
        ▼
   implementation.efn
   ```
3. **Satisfacción de contrato**: Un archivo `.efn` satisface una `.esig` cuando su única `public fn` coincide exactamente en:
   - Nombre de la función.
   - Cantidad y orden de los argumentos.
   - Tipos de los argumentos.
   - Tipo de resultado.
   No se permiten conversiones implícitas para satisfacer una firma.
4. **Desacoplamiento de implementación**: El consumidor conoce exclusivamente la `.esig` y sus tipos asociados. No conoce el nombre del archivo `.efn`, su ubicación física ni sus funciones privadas.
5. **Modelado explícito de resultados**: Los resultados esperados y manejables de una firma se modelan mediante tipos `enum` de dominio (por ejemplo, variantes `Ok` y `Error(string)` como valores normales), manteniéndolos estrictamente separados de los errores de evaluación del lenguaje (`ConversionError`, `OverflowError`, `DivisionByZeroError`).


### 12.7 Módulos (.emod) y selección granular de dependencias

Un archivo `.emod` (Evo Module) define la identidad semántica y la frontera de capacidades de un módulo:

1. **Frontera semántica vs carpeta física**: Una carpeta física organiza archivos en disco; un `.emod` delimita la superficie modular semántica del conjunto. La presencia de un archivo en una carpeta no lo hace automáticamente público si no está registrado en el `.emod`.
2. **Catálogo de capacidades**: El `.emod` registra las firmas públicas `.esig` que ofrece el módulo.
3. **Ausencia de `namespace`**: Evo-Script no introduce la palabra clave `namespace`.
4. **Selección por firma individual**: Un consumidor selecciona una capacidad concreta de un módulo por su firma (`signature`), no importando automáticamente todo el contenido del módulo. Utilizar una operación `modulo::operacion` no hace visibles el resto de operaciones del módulo.
5. **Cierre transitivo de tipos limitado**: Al seleccionar una `.esig`, se resuelven exclusivamente los tipos compartidos (`.estc`, `.enum`) directamente requeridos por los argumentos y resultado de dicha firma, junto con los tipos anidados por estos. Esto no expone otros tipos no relacionados del módulo.
6. **Independencia del mecanismo `use`**: La palabra clave `use` conserva estrictamente su semántica de activación de `Scope`. La sintaxis textual para la selección de dependencias modulares se definirá en especificaciones posteriores.


### 12.8 Raíz de proyecto (.root)

Un archivo `.root` (Evo Project Root) establece la raíz estructural y el límite superior de resolución semántica de un proyecto estructurado:

1. **Obligatoriedad en proyectos estructurados**: Todo proyecto Evo-Script multi-archivo que utilice comunicaciones entre artefactos (`.emod`, `.esig`, `.estc`, `.enum`) debe poseer exactamente un archivo `.root` en su nivel superior.
2. **Innecesario en scripts autocontenidos**: Un script simple `.efn` ejecutado directamente no requiere `.root`.
3. **Naturaleza no ejecutable**: El archivo `.root` no ejecuta código, no es un namespace y no importa módulos de forma automática; actúa como ancla de resolución del árbol del proyecto.


### 12.9 Puntos de entrada de aplicación (.main) y librerías (.elib)

1. **Aplicaciones ejecutables (`.main`)**:
   - Un proyecto estructurado que se ejecuta como aplicación define su punto de entrada mediante un archivo `.main` (Evo Application Entry).
   - El `.main` identifica y selecciona la operación inicial que arranca la aplicación.
   - El `.main` no implementa la lógica de la función ni contiene código ejecutable de negocio; la implementación reside en un `.efn`.
   - La función seleccionada no está obligada a llamarse literalmente `main`.
   - Un script `.efn` autocontenido no requiere `.main`.
2. **Librerías reutilizables (`.elib`)**:
   - Una agrupación reutilizable de módulos se declara mediante un archivo `.elib` (Evo Library).
   - Una librería agrupa módulos `.emod` para su consumo estructurado.
   - Un proyecto de librería posee `.root` y `.elib`, pero no requiere `.main` al no constituir una aplicación directamente ejecutable.
   - El consumo de capacidades desde una librería continúa siendo por firma individual (`.esig`).


### 12.10 Artefacto distribuible (.evo)

La extensión `.evo` está reservada exclusivamente para el artefacto distribuible o paquete empaquetado del ecosistema Evo:

1. **Naturaleza del paquete**: `.evo` representa el paquete empaquetado final de una aplicación o librería (contenedor de proyecto, módulos, metadatos y código preparado).
2. **No es código fuente**: `.evo` no es una extensión para archivos de código fuente general (las fuentes utilizan `.efn`, `.esig`, `.estc`, `.enum`, `.emod`, `.root`, `.main`, `.elib`).
3. **Ortogonalidad entre empaquetado y compilación**: El empaquetado `.evo` no impone ni presupone un modelo específico de compilación (como bytecode o binario nativo AOT).
4. **Formato físico desacoplado**: El formato físico interno del archivo `.evo` (compresión, manifiestos binarios) queda fuera del alcance de v0.1.
5. **No obligatorio para scripts simples**: Un archivo `.efn` autocontenido se ejecuta directamente sin necesidad de empaquetarse en un `.evo`.


### 12.11 Frontera con el entorno de ejecución (Host / Runtime)

Existe una separación conceptual estricta entre la semántica interna de Evo-Script y el entorno exterior que inicia y hospeda la ejecución:

1. **Producción de valores semánticos puros**: Una función pública de un `.efn` produce como resultado un valor semántico del sistema de tipos de Evo-Script (`Value`).
2. **Materialización del resultado**: El valor semántico producido no es inherentemente JSON, XML, texto plano, bytes ni stdout. El host/runtime es el único responsable de materializar dicho valor según el medio exterior correspondiente (CLI, API HTTP, log, etc.).
3. **Suministro de entradas desde el host**: El host/runtime proporciona las entradas requeridas por la función pública convirtiendo sus representaciones externas en valores semánticos de Evo-Script compatibles con la firma.
4. **Ausencia de interfaces de serialización en el lenguaje**: Evo-Script no incluye interfaces ni traits como `Serializable`, `Serialize` o `Deserialize`. Los tipos del lenguaje no requieren implementar codecs para participar en la frontera con el host.
5. **Conocimiento de tipos locales por el host**: Aunque un tipo local a un `.efn` no puede ser nombrado por otros archivos Evo-Script, el runtime que ejecuta el script conoce su definición semántica y puede materializar sus valores hacia el host.
6. **Propagación de fallos de evaluación hacia el host**:
   - **Evaluación válida**: El host recibe el `Value` semántico producido.
   - **Alternativa de dominio**: Valores como `Result::Error("mensaje")` son valores normales de dominio entregados como `Value`.
   - **Fallo de evaluación**: Errores del lenguaje (`ConversionError`, `OverflowError`, `DivisionByZeroError`) detienen la evaluación y se propagan directamente hasta el límite exterior del host/runtime como fallos de ejecución.


### 12.12 Ejemplos canónicos y modelos arquitectónicos

#### 12.12.1 Script autocontenido completo (`washer.efn`)

```text
struct Clothes {
    string name;
}

enum WashResult {
    Ok(Clothes)
    Error(string)
}

private fn validate(Clothes clothes) -> bool {
    return clothes.name != "";
}

public fn washes_clothes(Clothes clothes) -> WashResult {
    let bool valid = validate(clothes);

    return when valid {
        true  => WashResult::Ok(clothes)
        false => WashResult::Error("Ropa inválida")
    };
}
```

Características:
- `Clothes` y `WashResult` son tipos locales a `washer.efn`.
- `validate` es una función auxiliar privada (`private fn`).
- `washes_clothes` es la única función pública (`public fn`).
- No requiere `.esig`, `.estc`, `.enum`, `.root` ni `.main`.
- Se ejecuta directamente por un host/runtime Evo-Script.

#### 12.12.2 Proyecto estructurado completo (`laundry/`)

Estructura física de archivos:

```text
laundry/
├── application.root
├── application.main
│
├── laundry.emod
│
├── clothes.estc
├── washes_clothes_result.enum
├── washes_clothes.esig
└── washer.efn
```

Contenido y responsabilidades de cada artefacto:

- **`clothes.estc`** (Struct compartido):
  ```text
  struct Clothes {
      string name;
  }
  ```

- **`washes_clothes_result.enum`** (Enum compartido):
  ```text
  enum WashesClothesResult {
      Ok(Clothes)
      Error(string)
  }
  ```

- **`washes_clothes.esig`** (Firma pública / Contrato):
  ```text
  fn washes_clothes(Clothes clothes) -> WashesClothesResult;
  ```

- **`washer.efn`** (Implementación):
  ```text
  private fn validate(Clothes clothes) -> bool {
      return clothes.name != "";
  }

  public fn washes_clothes(Clothes clothes) -> WashesClothesResult {
      let bool valid = validate(clothes);

      return when valid {
          true  => WashesClothesResult::Ok(clothes)
          false => WashesClothesResult::Error("Ropa inválida")
      };
  }
  ```

- **`laundry.emod`**: Registra y ofrece la capacidad `washes_clothes.esig`.
- **`application.root`**: Declara la raíz de resolución del proyecto `laundry`.
- **`application.main`**: Identifica la operación de inicio de la aplicación.

#### 12.12.3 Diagrama de responsabilidades semánticas de proyecto

```text
                      structured project
                              │
                            .root
                              │
                 ┌────────────┴────────────┐
                 ▼                         ▼
        application (.main)        library (.elib)
                 │                         │
                 └────────────┬────────────┘
                              ▼
                            .emod
                              │
                            .esig
                            /   \
                        .estc   .enum
                            \   /
                            .efn
```

#### 12.12.4 Organización arquitectónica por responsabilidades

Los proyectos pueden organizar sus artefactos por responsabilidades semánticas (por ejemplo, `use_cases/`, `agents/`, `domain/`):
- `use_cases/`: Aloja firmas `.esig` que modelan las acciones requeridas.
- `agents/`: Aloja implementaciones `.efn` que satisfacen las firmas.
- `domain/`: Aloja definiciones de datos y alternativas `.estc` y `.enum`.

Estos nombres de carpetas representan patrones organizacionales sugeridos y no constituyen palabras reservadas del lenguaje.
