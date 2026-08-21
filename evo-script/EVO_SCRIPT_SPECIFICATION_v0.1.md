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

Por ejemplo (conceptual):

    use documents
    |> enter("reports")
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

### 6.1 Definición y naturaleza de Scope

El **Scope** es una pieza fundamental de Evo-Script y del ecosistema Evo para modelar la interacción con el entorno operativo, provisto semánticamente por **Evo-Shell**.

1. **Contexto operativo semántico**: Un Scope representa un **contexto operativo semántico** (`semantic operational context`) que identifica un entorno de ejecución, determina las capacidades operativas disponibles para las operaciones de Evo-Script y mantiene una ubicación interna cuando el contexto soporta navegación.
2. **Naturaleza de dato prestado (`Borrowed Data`)**: Técnicamente, un Scope es una estructura de datos contextual prestada (`borrowed semantic context data`), alineada con la representación arquitectónica interna:
   ```text
   Scope {
       scope_type,
       server,
       user,
       source,
       item
   }
   ```
   Scope describe y transporta información contextual (`scope_type`, `server`, `user`, `source`, `item`), pero **no posee necesariamente dichos datos**. La propiedad y el tiempo de vida técnico pertenecen a la implementación/Provider que suministra dicha información. La semántica de préstamo (*borrowing*) es un detalle interno de arquitectura y no introduce sintaxis visible de lifetimes (`'scope`), referencias (`&`) ni genéricos (`Scope<T>`) en Evo-Script.
3. **Frontera semántica**: Un Scope no es la implementación del Provider en sí misma, ni una colección materializada de datos, ni una ruta técnica del sistema operativo, ni un socket, ni un objeto de programación orientada a objetos, ni un módulo (`.emod`), ni un archivo `.elib`, ni una consulta EvoQ, ni estado global del lenguaje. El Scope actúa como la vista semántica prestada del contexto proporcionado por un Provider.


### 6.2 Relación Provider -> Scope y provisión de contexto

Existe una dependencia necesaria entre un Scope utilizable y el Provider que lo suministra:

```text
Provider (implementación técnica concreta)
    ↓
provides (provide_scope)
    ↓
Scope (vista contextual prestada)
```

1. **El Provider como origen del Scope**: Un Scope no puede existir de forma autónoma sin el Provider que lo proporciona. Si no hay Provider disponible, no existe Scope suministrable (`No Provider -> no Scope available`).
2. **Distinción estructural**:
   - **Provider**: Es el componente de comportamiento e implementación técnica concreta que interactúa con el entorno exterior (filesystem, base de datos, terminal, etc.).
   - **Scope**: Es la vista semántica prestada del contexto provisto por dicho Provider.
3. **Independencia tecnológica**: Evo-Script interactúa con el Scope sin conocer las tecnologías de bajo nivel del Provider (como APIs POSIX, Win32, SQL Server o secuencias ANSI).


### 6.3 Separación entre Provisión y Activación (`provide Scope != activate Scope`)

Evo-Script y Evo-Shell distinguen formalmente entre dos operaciones fundamentales:

```text
provide Scope  !=  activate Scope
```

1. **Provisión (`provide_scope`)**: Obtiene o expone una vista prestada de Scope a partir de un Provider. Obtener o recibir un Scope **NO lo activa automáticamente**.
2. **Activación (`activate_scope` / `use`)**: Establece un Scope previamente provisto como el `Active Scope` para el contexto de ejecución.
3. **Flujo operativo**:
   ```text
   Provider
       ↓
   provides Scope (provide_scope)
       ↓
   Scope provisto para la operación
       ↓
   use (activate_scope)
       ↓
   Active Scope
   ```


### 6.4 Concepto único: Active Scope

Evo-Script v0.1 define exactamente un único concepto formal para el Scope en uso:

> **`Active Scope`**: Es el Scope semántico actualmente activo para el contexto de ejecución que está siendo evaluado.

No existe un segundo tipo semántico de Scope ni entidades formales separadas para la terminal (no existen términos como `Prompt Scope`, `Session Scope`, `Host Scope` ni `current_scope`).


### 6.5 Comportamiento de Active Scope en la Terminal Interactiva

En un entorno de terminal interactiva provisto por Evo-Shell:

1. **Garantía de Active Scope válido**: La terminal interactiva de Evo opera siempre con exactamente un `Active Scope` válido disponible (`interactive terminal always operates with one valid Active Scope`). La terminal interactiva no inicia normalmente en un estado sin Scope activo.
2. **Persistencia conceptual entre comandos**: Al ejecutar comandos interactivos sucesivos ($N \to N+1$), el `Active Scope` permanece conceptualmente seleccionado para el siguiente comando:
   ```text
   // Ejemplo conceptual de sesión interactiva en terminal:
   use documents;
   enter("reports");
   iter |> filter ...;
   ```
3. **Sustitución explícita**: El usuario puede cambiar el `Active Scope` en cualquier momento ejecutando `use otro_scope`.
4. **Independencia de la estrategia técnica de retención**: La especificación no fija si el host interactivo conserva la misma instancia física prestada de `Scope` o si preserva el estado contextual para re-solicitar un borrow al Provider en cada comando. Dicha decisión queda abierta como detalle interno del runtime/host (en `evo-shell`, `evo-cli` u otro entorno).


### 6.6 Comportamiento de Active Scope en ejecución de `.efn`

La ejecución de funciones y scripts `.efn` sigue reglas estrictas de determinismo y aislamiento:

1. **Inicio sin Active Scope**: Toda ejecución de un `.efn` comienza **SIN Active Scope** (`Active Scope = absent`). Una función `.efn` no asume ningún Scope activo por defecto.
2. **Prohibición de herencia del Active Scope de la terminal**: Un `.efn` **NUNCA hereda implícitamente el `Active Scope` de la terminal interactiva** que lo invocó.
3. **Aislamiento total de contextos**:
   - El contexto de ejecución de la terminal interactiva tiene su propio `Active Scope`.
   - El contexto de ejecución del `.efn` inicia sin `Active Scope`.
   - La ejecución del `.efn` no consume, no altera y no reemplaza el `Active Scope` de la terminal exterior.
4. **Activación local dentro de `.efn`**:
   ```text
   .efn inicia (sin Active Scope)
       ↓
   Provider proporciona Scope
       ↓
   use activa el Scope localmente
       ↓
   evaluación de la composición
       ↓
   conclusión de la composición (Active Scope local finaliza)
   ```
5. **Finalización sin restauración mágica**: Al concluir la composición o la función `.efn`, el `Active Scope` local cesa de estar activo por simple terminación de su ámbito. No existe mecanismo de "guardar y restaurar" (*save/restore*) del Scope de la terminal, porque ambos contextos estuvieron completamente aislados desde el inicio. No existen palabras clave como `deactivate`, `unuse`, `restore_scope` ni `close_scope`.
6. **Ausencia de estado ambiental oculto**: No existen parámetros implícitos, inyección oculta de Scope ni variables globales ambientales en `.efn`.
7. **Operaciones que requieren Scope**: En `.efn`, la ausencia de Scope activo es un estado normal previo a su activación explícita. Aquellas operaciones que requieran un contexto semántico no pueden ejecutarse válidamente hasta que un Scope haya sido activado.


### 6.7 Activación y sustitución de Scope mediante `use`

La palabra clave estructural `use` es la instrucción que activa o sustituye el `Active Scope`:

1. **Semántica de `use`**: Activa un Scope provisto por un Provider correspondiente para el contexto de la composición actual (o en la terminal interactiva).
2. **Sustitución en pipelines**: Dentro de una misma composición, una cláusula posterior `|> use nuevo_scope` sustituye el `Active Scope` previo por el nuevo contexto para las operaciones subsiguientes:
   ```text
   use files
   |> filter ...
   |> select ...
   |> use terminal
   |> print
   ```
3. **Lo que `use` NO hace**:
   - `use` **no crea un Provider** ni crea un Scope de la nada.
   - `use` **no abre bases de datos, no monta discos ni crea sockets**.
   - `use` **no es `import`** (que resuelve símbolos estáticos antes de la evaluación).
   - `use` **no es inyección de dependencias (DI)** ni selección de implementaciones (`.root`).


### 6.8 Navegación interna mediante `enter`

La operación `enter(target)` modifica la ubicación actual dentro del `Active Scope`, siempre que el contexto semántico soporte navegación:

```text
use documents
|> enter("reports")
```

Distinción fundamental entre `use` y `enter`:
- **`use`**: Activa o sustituye el `Active Scope` (por ejemplo, transiciona de filesystem a terminal o a database).
- **`enter`**: Modifica la posición relativa dentro del `Active Scope` existente sin cambiar el contexto ni sus capacidades.
- **Navegabilidad no universal**: `enter` no es una operación universal; solo está disponible en aquellos Scopes cuya semántica admite navegación jerárquica o espacial (como filesystem o database), y no en contextos sin ubicación (como terminal o UI).


### 6.9 Separación formal entre Scope y EvoQ (`Scope != EvoQ`)

Evo-Script distingue formalmente entre el contexto operativo (**Scope**) y las operaciones de consulta sobre flujos de datos (**EvoQ**):

```text
Scope != EvoQ
Scope != 'from' / query source syntax
```

1. **Uso de Scope sin consultas EvoQ**: Un Scope puede activarse para ejecutar operaciones directas (como `print` en terminal o `copy` en filesystem) sin requerir ninguna consulta EvoQ.
2. **Flujo de datos y transformaciones**:
   ```text
   Provider
       ↓
   provides Scope
       ↓
   use (Active Scope)
       ↓
   operación fuente produce datos
       ↓
   EvoQ transforma datos (filter, select, take, iter)
       ↓
   operación destino consume datos
   ```
3. **Ortogonalidad con los datos del pipeline**: Sustituir el `Active Scope` dentro de un pipeline mediante `|> use otro_scope` **no destruye ni descarta los datos** en tránsito. EvoQ opera sobre el flujo de datos producido, pero EvoQ no proporciona ni posee el Scope.


### 6.10 Relación con .elib y resolución física de Providers

En proyectos estructurados:

1. **Membresía física del Provider en `.elib`**: Todo Provider utilizado para suministrar un Scope dentro de una aplicación estructurada debe pertenecer físicamente al `Physical Artifact Universe` declarado por el archivo `.elib` activo.
2. **`.elib` NO almacena Scope**:
   - `.elib` **hace disponibles físicamente** los artefactos del Provider.
   - `.elib` **NO almacena el Scope ni el Active Scope**.
   - `.elib` **NO posee estado en tiempo de ejecución**.
   - `.elib` **NO retiene valores de Scope prestados**.
3. **Membresía != Activación**: Que un Provider esté registrado en `.elib` permite su resolución en el universo físico del proyecto, pero no implica activación de Scope; la activación requiere la instrucción explícita `use`.


### 6.11 Resumen canónico de ciclo de vida y comportamiento de Scope

```text
Active .elib
    ↓ (hace disponible físicamente al Provider)
Provider disponible
    ↓ (provide_scope)
Scope provisto (borrowed context)
    ↓ (use / activate_scope)
Active Scope
    ↓
Capacidades operativas / Datos producidos
    ↓
Transformaciones EvoQ (filter, select, etc.)
```

| Dimensión | Terminal interactiva | Ejecución de archivo `.efn` |
| :--- | :--- | :--- |
| **Estado inicial de Active Scope** | Siempre posee exactamente un `Active Scope` válido | Inicia **sin Active Scope** (`Active Scope = absent`) |
| **Herencia de contexto** | Mantiene conceptualmente el contexto seleccionado entre comandos | **No hereda** el Active Scope de la terminal interactiva |
| **Activación de Scope** | Mediante `use <scope>` en el prompt | Mediante `use <scope>` explícito dentro de sus composiciones |
| **Ámbito y vigencia** | Persiste conceptualmente entre comandos interactivos | Confinado estrictamente a la composición local en evaluación |
| **Finalización** | Por sustitución (`use`) o cierre de sesión del terminal | Al concluir la composición; se descarta sin alterar la terminal |
| **Interacción con el entorno** | Opera sobre el Scope provisto por el Provider correspondiente | Requiere que el Provider esté disponible físicamente en el proyecto |


### 6.12 Invalidación externa y retención de contexto (Candidatos futuros / v0.2)

1. **Invalidación externa de Scope (`External Scope Invalidation`)**:
   - Un Provider proporciona una vista de Scope prestada (`borrowed Scope`).
   - Posteriormente, el recurso o servidor externo subyacente puede ser modificado, movido o desconectado por un agente externo.
   - Una provisión futura de Scope describirá el nuevo estado del recurso.
   - Evo-Script v0.1 no define observadores activos (*watchers*), reconexión automática (`reconnect`, `refresh`) ni errores de sistema especiales (no existen `StaleScopeError` ni `ScopeInvalidatedError` en v0.1).
2. **Retención / Readquisición de Scope a largo plazo (`Long-lived Scope Retention/Reacquisition`)**:
   - La estrategia técnica mediante la cual un host interactivo preserva o readquiere un borrow válido de Scope a lo largo de una sesión queda diferida como detalle de implementación para versiones futuras del runtime/host (sin determinar si reside en `evo-shell`, `evo-cli` o `evo-runtime`).


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

Los tipos definidos pueden originarse como tipos locales dentro de un archivo `.efn` o como tipos compartidos declarados en artefactos especializados (`.estc` y `.enum`), publicados por un módulo `.emod` e importados explícitamente al `Type Space` local mediante cláusulas `import modulo::Tipo;` (o con alias mediante `import modulo::Tipo as Alias;`).

Una función o estructura que hace referencia a un tipo definido no necesita conocer internamente si el identificador corresponde a un `struct` o a un `enum`. La definición correspondiente determina su naturaleza.



### 7.3 Struct

`struct` define exclusivamente una estructura inmutable de datos (`AND data`).

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


#### 7.3.4 Construcción de valores struct (Struct Construction Expression)

1. **Construcción como expresión**:
   - La construcción de un valor struct (`Struct Construction Expression`) es una `Expression` que produce un nuevo valor inmutable del tipo struct correspondiente.
   - Evo-Script no utiliza la palabra clave `new`, constructores de objetos ni métodos `init`.
   - La construcción se realiza directamente especificando el nombre del tipo seguido de sus campos y valores entre llaves:
     ```text
     Trabajador {
         edad: 43
         name: "Gustavo"
         last_name: "Melendez"
     }
     ```
2. **Semántica de datos conjuntos (`AND Data`)**:
   - Un struct representa la conjunción obligatoria de todos sus campos declarados. Para construir un valor válido, **todos los campos declarados son estrictamente obligatorios**.
   - No existen valores por defecto implícitos (no se asume `0`, `""` ni `false`) ni existe `null` como mecanismo para omitir campos. La omisión de cualquier campo es un error de validación estática.
3. **Unicidad de campos en la construcción**:
   - Cada campo declarado debe aparecer **exactamente una vez** dentro de la construcción.
   - No se permiten campos duplicados ni campos desconocidos que no formen parte de la definición del tipo.
4. **Correspondencia exacta de tipos**:
   - Cada expresión asignada a un campo (`nombre: expresion`) debe producir exactamente el tipo declarado para dicho campo en la definición del struct. No aplican conversiones implícitas. Si se requiere una conversión, debe realizarse explícitamente (`to_tipo`).
5. **Identificación por nombre**:
   - El orden textual en que se inicializan los campos durante la construcción no altera la identidad semántica del valor, ya que los campos se vinculan por su nombre:
     ```text
     Trabajador {
         edad: 43
         name: "Gustavo"
         last_name: "Melendez"
     }
     ```
     y:
     ```text
     Trabajador {
         name: "Gustavo"
         last_name: "Melendez"
         edad: 43
     }
     ```
     producen valores equivalentes.


#### 7.3.5 Acceso a campos de struct (Field Access)

1. **Sintaxis formal**:
   Evo-Script v0.1 define formalmente el acceso a campos (`Field Access Expression`) mediante la sintaxis:

   ```text
   field_access
       := expression "." field_name
   ```

   El operador oficial de acceso a campos es el punto (`.`). No se admiten operadores alternativos como `->`, `::`, `?.` ni `[]` para acceder a campos de structs.
2. **Evaluación y tipado de Field Access**:
   - `Field Access` es una `Expression` que evalúa la expresión del receptor (*receiver*) y proyecta exactamente el valor del campo solicitado.
   - El tipo resultante de `expression.field_name` es **exactamente el tipo declarado para dicho campo** en la definición del struct. No existe conversión implícita durante el acceso a campos:
     ```text
     let Trabajador trabajador = Trabajador {
         edad: 43
         name: "Gustavo"
         last_name: "Melendez"
     };

     let int edad = trabajador.edad;       // Produce int
     let string nombre = trabajador.name;   // Produce string
     ```
3. **Restricción de tipo de receptor (Receiver Struct-Only)**:
   - El acceso a campos solo es válido sobre expresiones cuyo tipo semántico sea un `struct`.
   - **Tipos nativos**: No se permite acceso a campos sobre tipos nativos (`int`, `float`, `bool`, `string`, `dynamic`). Expresiones como `10.value`, `true.value` o `"texto".length` son inválidas y producen el error estático **`FieldAccessTypeError`**.
   - **Enums**: No se permite acceso directo a campos sobre valores de tipo `enum` (por ejemplo `resultado.trabajador` es inválido). Los enums representan alternativas disjuntas (`OR alternatives`) y deben seleccionarse obligatoriamente mediante `when` antes de acceder a los campos del struct contenido.
4. **Validación de campos inexistentes**:
   - Si el receptor es un struct válido pero el campo solicitado no existe en su definición (por ejemplo `trabajador.salario`), el programa es inválido y produce el error estático **`FieldNotFoundError`**.
   - Tanto `FieldNotFoundError` como `FieldAccessTypeError` pertenecen a la categoría de **errores de validación del sistema** (`SystemError`); se detectan antes de la evaluación normal y **no** constituyen errores de evaluación (`EvaluationError`).
5. **Encadenamiento y asociatividad (Chaining)**:
   - El acceso a campos puede encadenarse para navegar a través de composiciones de structs.
   - El encadenamiento se asocia estrictamente de **izquierda a derecha**:
     ```text
     colonia.pais.name
     ```
     equivale semánticamente a:
     ```text
     (colonia.pais).name
     ```
6. **Receptor como llamada a función**:
   - El receptor de un acceso a campo puede ser cualquier expresión que produzca un struct, incluyendo una llamada a función:
     ```text
     buscar_trabajador(id).name
     buscar_colonia(id).pais.name
     ```
7. **Contextos de uso de Field Access**:
   - `Field Access` es una `Expression` ordinaria que produce un valor y puede utilizarse en cualquier contexto donde se espera un `Value`:
     - Inicializador de `let`: `let string nombre = trabajador.name;`
     - Argumento de llamada: `procesar(trabajador.name);`
     - Retorno de función: `return trabajador.name;`
     - Entrada de pipeline: `trabajador.name |> normalizar |> guardar;`
     - Expresiones aritméticas / lógicas / comparaciones: `trabajador.edad + 10`, `trabajador.name == "Gustavo"`
8. **Prohibición de Field Access como Operation Statement**:
   - Un acceso a campo es una expresión de valor, **NO** un `Operation Statement`. Sentencias como `trabajador.name;` son sintácticamente inválidas.
9. **Precedencia de Field Access**:
   - El acceso a campos es una operación postfix de la más alta precedencia, agrupándose antes de operadores aritméticos, comparaciones, operadores lógicos y pipelines:
     - `trabajador.edad + 10` equivale a `(trabajador.edad) + 10`.
     - `trabajador.name |> normalizar` equivale a `(trabajador.name) |> normalizar`.


#### 7.3.6 Inmutabilidad y ausencia de mutación en structs

1. **Ausencia de asignación a campos (No Field Assignment)**:
   - Los campos de un `struct` son estrictamente de solo lectura una vez construido el valor.
   - Evo-Script **NO define asignación a campos**. Expresiones como las siguientes son inválidas:
     ```text
     trabajador.name = "Juan";   // Inválido: no existe asignación a campos
     trabajador.edad = 44;       // Inválido: no existe asignación a campos
     ```
   - El carácter `=` solo se utiliza en declaraciones de binding inmutable `let` y no adquiere semántica de operador de asignación.
2. **Ausencia de setters y propiedades mutables**:
   - No existen métodos setter, propiedades modificables ni sintaxis de actualización destructiva.
3. **Reconstrucción inmutable de valores**:
   - Para representar una versión modificada de los datos, se construye un nuevo valor struct explícito:
     ```text
     let Trabajador trabajador_actualizado = Trabajador {
         edad: trabajador.edad
         name: "Juan"
         last_name: trabajador.last_name
     };
     ```
   - `trabajador` permanece intacto e inmutable como el valor original; `trabajador_actualizado` constituye un nuevo valor estructurado independiente.
   - Evo-Script v0.1 no define sintaxis abreviada de actualización destructiva ni operadores copy-update (como `..trabajador` o `with`).
4. **Ausencia de métodos y orientación a objetos**:
   - `Field Access` (`.`) sirve única y exclusivamente para proyectar campos de datos de structs.
   - **NO se admiten métodos de miembros**: llamadas como `trabajador.guardar()`, `trabajador.get_name()` o `trabajador.name()` son inválidas.
   - El comportamiento se organiza exclusivamente en funciones y firmas independientes (`guardar_trabajador(trabajador)` o `trabajador |> guardar_trabajador`).


#### 7.3.7 Composición estructural finita y grafo acíclico de dependencias de tipos

Evo-Script v0.1 establece normativamente el principio de **composición estructural finita**:

1. **Valores estructurales finitos**:
   - Todo `struct` representa una conjunción finita de datos (`AND data`).
   - Todo `enum` representa una disyunción finita de alternativas de datos (`OR alternatives`).
   - Todo valor estructural instanciado en Evo-Script v0.1 debe ser estrictamente finito en memoria y representación.
2. **Grafo de dependencias de tipos estructurales (`Type Dependency Graph`)**:
   - Cada tipo estructural declarado por el usuario (`struct` o `enum`) constituye un nodo en el `Type Dependency Graph`.
   - Se genera una arista de dependencia estructural dirigida $A \to B$ cuando un valor de tipo $A$ contiene estructuralmente un valor de tipo $B$:
     - El tipo declarado de un campo dentro de un `struct`.
     - El tipo de carga asociada transportado por una variante de un `enum` (`Variante(Tipo)`).
     - El tipo declarado de un campo dentro de una variante estructurada de un `enum` (`Variante { Tipo campo; }`).
   - Los tipos nativos escalares (`int`, `int8`..`int128`, `uint8`..`uint128`, `float`, `float32`, `float64`, `bool`, `string`, `dynamic`) no son tipos recursivos de usuario y no generan aristas hacia otros structs o enums.
3. **Exigencia normativa de Grafo Acíclico Dirigido (DAG)**:
   - El `Type Dependency Graph` de todo proyecto o artefacto en Evo-Script v0.1 debe ser estrictamente un **Grafo Acíclico Dirigido (DAG)** (*Directed Acyclic Graph*).
   - No se exige que el grafo sea un árbol: se admiten dependencias convergentes ($A \to D$ y $B \to D$) y cadenas profundas de composición ($A \to B \to C \to D \to E$), siempre y cuando ninguna ruta genere un ciclo de vuelta.
   - Las dependencias convergentes representan uso compartido del tipo en distintas estructuras, **sin implicar** compartición de instancias mutables en tiempo de ejecución (se mantiene la semántica de valores inmutables puros).


#### 7.3.8 Prohibición formal de tipos recursivos y ciclos estructurales

Queda estrictamente prohibida cualquier forma de recursión en el grafo de dependencias de tipos estructurales:

1. **Recursión directa en structs**:
   - Un `struct` no puede contenerse estructuralmente a sí mismo:
     ```text
     struct Node {
         int value;
         Node next;   // Inválido: dependencia directa Node -> Node
     }
     ```
   - Invalidez: `RecursiveTypeCycleError`.
2. **Recursión indirecta entre structs**:
   - Dos o más structs no pueden formar un ciclo de contención mutua:
     ```text
     struct A {
         B b;
     }

     struct B {
         A a;         // Inválido: ciclo A -> B -> A
     }
     ```
   - Invalidez: `RecursiveTypeCycleError`.
3. **Ciclos de longitud arbitraria**:
   - Rutas como $A \to B \to C \to A$ son igualmente detectadas y rechazadas con `RecursiveTypeCycleError`.
4. **Recursión directa e indirecta en enums**:
   - Una variante de un `enum` no puede contener como carga su propio tipo ni un ciclo indirecto de enums:
     ```text
     enum Node {
         End
         Next(Node)   // Inválido: dependencia directa Node -> Node
     }
     ```
   - Invalidez: `RecursiveTypeCycleError`.
5. **Ciclos mixtos entre structs y enums**:
   - Las dependencias entre structs y variantes de enums no pueden cerrarse cíclicamente:
     ```text
     struct Worker {
         int id;
         WorkerResult result;
     }

     enum WorkerResult {
         Empty
         Found(Worker) // Inválido: ciclo mixto Worker -> WorkerResult -> Worker
     }
     ```
   - Invalidez: `RecursiveTypeCycleError`.
6. **Independencia del orden de declaración**:
   - La presencia de un ciclo invalida el conjunto de tipos con independencia del orden en que aparezcan declarados o resueltos en el código fuente.
7. **Error del Sistema: `RecursiveTypeCycleError`**:
   - Pertenece a la categoría de **errores de validación del sistema** (`SystemError`).
   - Se detecta durante el análisis y validación estática del proyecto, **antes de la evaluación normal en runtime**.
   - No es un `EvaluationError`, no es un `Value`, no es capturable y no se modela como un *runtime stack overflow* ni como agotamiento de memoria.
   - **Distinción formal con `FunctionCallCycleError`**:
     - `FunctionCallCycleError` valida el grafo de llamadas entre funciones (evita recursión de ejecución).
     - `RecursiveTypeCycleError` valida el grafo de dependencias de tipos estructurales (garantiza valores finitos).


#### 7.3.9 Composición estructural unidireccional y navegación de relaciones

Evo-Script v0.1 define una separación conceptual estricta entre la **composición estructural de datos** y la **navegación de relaciones del dominio**:

1. **Composición estructural unidireccional**:
   - La composición de datos opera en una única dirección acíclica elegida por el diseñador:
     ```text
     struct Pais {
         int id;
         string name;
     }

     struct Estado {
         int id;
         string name;
         Pais pais;    // Válido: dependencia unidireccional Estado -> Pais
     }
     ```
   - `Estado` contiene a `Pais`; `Pais` **no contiene estructuralmente** a `Estado`.
2. **Relación inversa mediante comportamiento (Funciones / Queries)**:
   - Una relación conceptual bidireccional del dominio **no se modela mediante contención estructural mutua**, sino mediante funciones explícitas:
     - Composición estructural: `Estado -> Pais` (datos inmutables).
     - Navegación inversa: función independiente `states_by_country(int country_id)` o capability que consulta o proyecta los estados asociados.
3. **Identificadores sin semántica mágica**:
   - Modelar relaciones mediante claves numéricas (como `int pais_id`) no altera la naturaleza del campo:
     ```text
     struct Estado {
         int id;
         string name;
         int pais_id;  // Campo escalar de tipo int ordinario
     }
     ```
   - `pais_id` es un valor `int` simple; el sufijo `_id` no introduce llaves foráneas automáticas, relaciones implícitas de base de datos ni punteros en v0.1.
4. **Prohibición de funciones y Signature Dependencies en structs**:
   - Los campos de un `struct` son **exclusivamente Values de datos**.
   - No se permite almacenar punteros a funciones, métodos, clausuras, firmas (`.esig`) ni `Signature Dependency Parameters` dentro de un `struct`.
   - Las `Signature Dependency Parameters` son contratos de capacidades resolubles en funciones, **no son valores de datos** de primer orden y no pueden almacenarse como campos.
5. **Ausencia de modelos de objetos y referencias**:
   - Evo-Script v0.1 no introduce tipos referencia, punteros, `Box`, `Rc`, `Arc`, `GC`, valores `null`, `lazy loading` ni propiedades de navegación de tipo ORM para enlazar estructuras.


#### 7.3.10 Notas de diseño prospectivas para Evo-Script v0.2

Se registran exclusivamente como **candidatos conceptuales de diseño futuro para una versión v0.2**, sin carácter operativo en Evo-Script v0.1:

1. **Candidato futuro: Colección homogénea (`Homogeneous Collection`)**:
   - Idea conceptual para representar secuencias o conjuntos materializados de cardinalidad uno-a-muchos sobre un tipo concreto, evitando la exposición de genéricos generales.
   - **Principio de preservación del DAG**: Si en una versión futura se introduce una colección homogénea, una contención como `Pais -> Collection[Estado]` junto con `Estado -> Pais` seguirá constituyendo un ciclo estructural ($Pais \to Estado \to Pais$) y será igualmente rechazada con `RecursiveTypeCycleError`. Las colecciones no constituyen un mecanismo para eludir la prohibición de ciclos estructurales.
   - **Alcance en v0.1**: En Evo-Script v0.1 **no existen arrays ni colecciones** (`Estado[]`, `[Estado]`, `Array<Estado>`, `List<Estado>`). No se define sintaxis de colección, alocación, indexación, mutación ni tamaño fijo/dinámico.
2. **Candidato futuro: Definición de relaciones (`Relationship Definition`)**:
   - Idea conceptual para declarar metadatos de relación entre identidades de tipos fuera de la definición de campos de structs, permitiendo que mecanismos de consulta y navegación (como EvoQ) recorran relaciones en ambas direcciones.
   - **Alcance en v0.1**: En Evo-Script v0.1 **no existen palabras clave de relación** (`relationship`, `relation`, `foreign`, `references`, `has_many`, `belongs_to`), ni restricciones relacionales de base de datos, ni integración operativa con EvoQ.


#### 7.3.11 Separación entre datos y comportamiento

Evo-Script mantiene una separación total entre la estructura de datos y las
operaciones que actúan sobre ella:

- `struct`: define tipos de datos (`AND data`).
- `NombreTipo { ... }`: construye un nuevo valor estructurado inmutable.
- `valor.campo`: proyecta un campo del valor estructurado.
- `fn`: define comportamiento y funciones de transformación.
- `when`: selecciona alternativas en tipos enumerados (`OR alternatives`).

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

    public fn guardar(Colonia colonia) -> GuardarColoniaResult {
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

   public fn buscar(int id) -> BuscarTrabajadorResult {
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
    let bool allowed = active && (age >= 18);


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

| Operador | Operación semántica | Resultado producido | Ejemplo |
| :--- | :--- | :--- | :--- |
| `+` | Suma | Suma numérica | `a + b` |
| `-` | Resta | Diferencia numérica | `a - b` |
| `*` | Multiplicación | Producto numérico | `a * b` |
| `/` | División | Cociente numérico (*Quotient*) | `a / b` |
| `%` | Residuo | Residuo entero (*Remainder*) | `a % b` |

Reglas normativas generales:

1. **Expresiones de valor (Value Expressions)**: Toda operación aritmética es una `Value Expression` que produce exactamente un valor normal (`Value`) del tipo numérico correspondiente cuando su evaluación tiene éxito. No constituyen `Operation Statements` (`a + b;` o `a / b;` como sentencias aisladas son inválidas).
2. **Conservación de tipo semántico**: Una operación entre operandos del mismo tipo numérico de tamaño fijo produce como resultado ese mismo tipo semántico (por ejemplo, `int32 + int32` produce `int32`, `float64 / float64` produce `float64`).
3. **Ausencia de conversión o promoción automática**: No existe widening, coerción ni promoción silenciosa de tipos (por ejemplo, `int32 + int32` no se convierte automáticamente en `int64`).
4. **Compatibilidad estricta**: Los operandos izquierdo y derecho deben ser exactamente del mismo tipo numérico semántico (`int == int`, `int64 == int64`, `uint32 == uint32`, `float64 == float64`). Operaciones con tipos heterogéneos (`int / int64`, `int / float`, `int32 % int64`) son estáticamente inválidas y requieren conversión explícita mediante la familia `to_tipo`.


#### 10.3.1 Semántica de división (`/`) y truncamiento hacia cero

El operador de división `/` produce exclusivamente el **cociente numérico** (`Quotient`):

1. **División entera con signo (`Integer Division`)**:
   - Aplica a todos los tipos enteros con signo (`int`, `int8`, `int16`, `int32`, `int64`, `int128`).
   - El cociente entero se calcula mediante **truncamiento estricto hacia cero** (*Truncation Toward Zero*), descartando la parte fraccionaria en dirección a cero.
   - Evo-Script **no utiliza división de piso** (*floor division* hacia $-\infty$), redondeo al entero más cercano ni techo hacia $+\infty$.
   - **Ejemplos normativos con signo**:
     ```text
     10 / 3    ->  3
     -10 / 3   -> -3
     10 / -3   -> -3
     -10 / -3  ->  3
     ```
     En particular, `-10 / 3` produce `-3` (y no `-4`).
2. **División entera sin signo (`Unsigned Integer Division`)**:
   - Aplica a todos los tipos enteros sin signo (`uint8`, `uint16`, `uint32`, `uint64`, `uint128`).
   - Produce el cociente entero no negativo exacto:
     ```text
     let uint32 dividendo = 10;
     let uint32 divisor = 3;
     let uint32 cociente = dividendo / divisor; // Produce 3
     ```
3. **División de punto flotante (`Floating Division`)**:
   - Aplica a los tipos flotantes definidos (`float`, `float32`, `float64`) cuando ambos operandos son del mismo tipo:
     ```text
     10.0 / 4.0   -> 2.5
     ```
   - Produce un cociente flotante del mismo tipo semántico.


#### 10.3.2 Semántica de residuo entero (`%`)

El operador `%` produce exclusivamente el **residuo entero** (`Remainder`):

1. **Definición matemática formal**:
   Para toda división entera válida, el residuo satisface invariablemente la identidad:
   $$\text{dividend} = (\text{quotient} \times \text{divisor}) + \text{remainder}$$
   donde $\text{quotient} = \text{dividend} / \text{divisor}$ según la regla de truncamiento hacia cero.
2. **Residuo entero con signo**:
   - Cuando $\text{remainder} \ne 0$, el signo del residuo coincide **con el signo del dividendo** (no con el del divisor).
   - **Ejemplos normativos con signo**:
     ```text
     10 % 3    ->  1
     -10 % 3   -> -1
     10 % -3   ->  1
     -10 % -3  -> -1
     ```
   - **Comprobación de la identidad**:
     - `-10 / 3` produce `-3`.
     - `-10 % 3` produce `-1`.
     - Identidad: $(-3 \times 3) + (-1) = -9 - 1 = -10$.
3. **Magnitud del residuo**:
   Para cualquier divisor distinto de cero sobre enteros con signo, la magnitud absoluta del residuo es estrictamente menor que la del divisor: $|\text{remainder}| < |\text{divisor}|$.
4. **División exacta y residuo cero**:
   Cuando la división es exacta (`12 / 3` $\rightarrow$ `4`), el residuo es exactamente `0` (`12 % 3` $\rightarrow$ `0`). No existe signo observable para el cero entero.
5. **Residuo entero sin signo**:
   Para enteros unsigned (`uint8`..`uint128`), el residuo satisface $0 \le \text{remainder} < \text{divisor}$ para todo divisor no nulo (`10 % 3` $\rightarrow$ `1` en contexto unsigned).


#### 10.3.3 Prohibición de residuo (`%`) sobre punto flotante en v0.1

Evo-Script v0.1 establece formalmente que el operador `%` **no existe sobre tipos de punto flotante**:

1. **Exclusividad para enteros**: El operador `%` está definido única y exclusivamente para tipos enteros (`int`, `int8`..`int128`, `uint8`..`uint128`).
2. **Invalidez estática en flotantes**:
   - `float % float` $\rightarrow$ Inválido.
   - `float32 % float32` $\rightarrow$ Inválido.
   - `float64 % float64` $\rightarrow$ Inválido.
   - No se definen operaciones de *floating remainder*, *floating modulo* ni funciones IEEE `fmod` para el operador `%`.
3. **Rechazo en validación estática**:
   - Expresiones como `10.0 % 4.0` o `10.0 % 0.0` son rechazadas durante la **validación estática** (`System / Validation Error`) antes de cualquier ejecución en runtime por constituir una combinación no admitida de operador y tipo.
   - Por tanto, `10.0 % 0.0` **no alcanza la evaluación runtime** ni produce `DivisionByZeroError`, sino que invalida el programa antes de la ejecución.


#### 10.3.4 Matriz normativa de operaciones de división y residuo

| Operación | ¿Válida estáticamente? | Tipo resultante / Fallo semántico |
| :--- | :---: | :--- |
| `int / int` (enteros con signo) | Sí | Cociente entero (`int`) con truncamiento hacia cero |
| `int % int` (enteros con signo) | Sí | Residuo entero (`int`) con signo del dividendo |
| `uint / uint` (enteros sin signo) | Sí | Cociente entero (`uint`) |
| `uint % uint` (enteros sin signo) | Sí | Residuo entero (`uint`) |
| `float / float` (flotantes) | Sí | Cociente flotante (`float`) |
| `float % float` (flotantes) | **No** | Invalidez en validación estática (`System / Validation Error`) |
| `integer / 0` | Sí (estática) | Fallo en runtime con `DivisionByZeroError` |
| `integer % 0` | Sí (estática) | Fallo en runtime con `DivisionByZeroError` |
| `float / 0.0` (o `-0.0`) | Sí (estática) | Fallo en runtime con `DivisionByZeroError` |
| `float % 0.0` | **No** | Invalidez en validación estática (`System / Validation Error`) |
| `signed_fixed_min / -1` | Sí (estática) | Fallo en runtime con `OverflowError` |
| `signed_fixed_min % -1` | Sí (estática) | Fallo en runtime con `OverflowError` |


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
7. **Caso extremo de división `MIN_VALUE / -1`**: Para los tipos enteros con signo de tamaño fijo (`int8`, `int16`, `int32`, `int64`, `int128`), dividir el valor mínimo representable entre `-1` genera un cociente matemático positivo que excede la capacidad máxima del tipo. La operación produce `OverflowError`.
8. **Caso extremo de residuo `MIN_VALUE % -1`**: Para los tipos enteros con signo de tamaño fijo, calcular el residuo del valor mínimo representable entre `-1` también produce `OverflowError`. Aunque el residuo matemático sería cero, Evo-Script define formalmente que `/` y `%` comparten el mismo límite de desbordamiento en enteros con signo de tamaño fijo para garantizar consistencia en sus dominios operacionales.
9. **Inaplicabilidad a enteros sin signo**: Los casos de desbordamiento por divisor `-1` no aplican a los tipos sin signo (`uint8`..`uint128`), ya que el valor `-1` no pertenece al tipo unsigned correspondiente.
10. **Naturaleza de OverflowError**: `OverflowError` es un fallo de evaluación aritmética; no es un valor normal, no forma parte del tipo normal de la expresión (`int8 | OverflowError` no existe), no se envuelve en `Result` y no es capturable desde dentro de Evo-Script v0.1.


### 10.5 División y residuo entre cero (DivisionByZeroError)

Dividir entre cero o calcular el residuo con un divisor igual a cero no constituye una operación válida en Evo-Script. Cuando el segundo operando de una operación `/` o de una operación válida `%` es numéricamente igual a cero, la evaluación aritmética falla con:

    DivisionByZeroError

Reglas:

1. **Conservación de tipo normal**: La operación `/` conserva el tipo semántico numérico de sus operandos (`int`, `float64`, etc.). La operación `%` conserva el tipo semántico entero de sus operandos dentro de su dominio válido (`int`, `uint32`, etc.). `DivisionByZeroError` no forma parte del tipo normal de retorno de ninguna de las dos expresiones.
2. **Operaciones cubiertas**: Aplica de manera uniforme tanto a la división (`a / b`) en enteros y flotantes, como al residuo (`a % b`) en su dominio entero válido. No existen errores separados como `RemainderByZeroError` ni `ModuloByZeroError`.
3. **Tipos enteros**: Aplica a todos los tipos enteros (`int`, `int8`..`int128`, `uint8`..`uint128`) tanto para `/` como para `%`:
   ```text
   let int64 value = 100;
   let int64 divisor = 0;
   let int64 result = value / divisor; // Falla la evaluación con DivisionByZeroError
   let int64 rem = value % divisor;    // Falla la evaluación con DivisionByZeroError
   ```
4. **Punto flotante en división (`/`)**: Aplica a todos los tipos flotantes (`float`, `float32`, `float64`) para el operador `/`. Evo-Script no produce silenciosamente `Infinity`, `+Infinity`, `-Infinity` ni `NaN` como resultado normal de una división entre cero.
   - Divisores `0.0` y `-0.0` se consideran numéricamente cero:
     ```text
     10.0 / 0.0  // Produce DivisionByZeroError
     10.0 / -0.0 // Produce DivisionByZeroError
     0.0 / 0.0   // Produce DivisionByZeroError
     0 / 0       // Produce DivisionByZeroError
     ```
   - Recordatorio: `10.0 % 0.0` no alcanza `DivisionByZeroError` porque `%` sobre flotantes es rechazado antes de la evaluación por validación estática (Sección 10.3.3).
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
3. **División entera bajo dynamic**: Cuando una operación de división entera se evalúa bajo contexto `dynamic`, conserva exactamente la semántica matemática de cociente truncado hacia cero. La evaluación dinámica utiliza representación suficiente sin convertir la operación en división flotante.
4. **Residuo entero bajo dynamic**: Cuando una operación `%` sobre enteros se evalúa bajo contexto `dynamic`, conserva la identidad $\text{dividend} = (\text{quotient} \times \text{divisor}) + \text{remainder}$ con truncamiento hacia cero.
5. **dynamic no habilita `%` sobre punto flotante**: El contexto `dynamic` no altera las reglas de compatibilidad de operadores; una expresión como `let dynamic r = 10.0 % 4.0;` sigue siendo inválida estáticamente porque `%` no admite tipos flotantes.
6. **Conservación exacta de enteros y precisión arbitraria**: Para operaciones enteras, `dynamic` garantiza la conservación exacta del resultado matemático. Si el resultado excede el tamaño de `int128`/`uint128`, utiliza internamente una representación de precisión arbitraria. No se introducen tipos visibles adicionales como `bigint`, `int256` ni `int512`.
7. **dynamic no significa imprecisión**: Para enteros, `dynamic` garantiza exactitud matemática absoluta, no aproximación.
8. **dynamic y punto flotante**: `dynamic` no introduce precisión arbitraria para flotantes ni el tipo `float128`. Las operaciones flotantes se rigen por las reglas de los tipos flotantes definidos (`float`, `float32`, `float64`).
9. **Sin conversiones implícitas de operandos**: Declarar un resultado como `dynamic` no vuelve válidas operaciones entre operandos incompatibles. Por ejemplo, operar `int32` con `int64` requiere conversión explícita:
   ```text
   let dynamic result = to_int64(a) + b;
   ```
10. **Errores de evaluación en dynamic**: Si una operación dentro de una expresión `dynamic` resulta matemáticamente inválida (como división o residuo entre cero), la evaluación falla con `DivisionByZeroError` antes de producir un valor `dynamic`.


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
   private fn dividir(int a, int b) -> int {
       return a / b;
   }

   public fn calcular(int a, int b) -> int {
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


#### 10.7.7 Nota de diseño futura para Evo-Script v0.2: operación `divide(...)`

Se registra exclusivamente con carácter de **nota de diseño prospectiva NO operativa para una posible versión v0.2** la idea conceptual de una operación funcional:

    divide(...)

1. **Motivación conceptual**:
   - En Evo-Script v0.1, el operador aritmético `/` produce un cociente numérico y falla con `DivisionByZeroError` ante divisores nulos, mientras que `%` produce el residuo entero.
   - En una versión futura (v0.2), podría explorarse la introducción de una operación `divide(...)` que modele explícitamente a nivel de dominio un resultado compuesto mediante un `enum`, conteniendo información de cociente, residuo y una alternativa de dominio ante divisiones no realizables.
2. **Delimitación estricta en Evo-Script v0.1**:
   - En Evo-Script v0.1, `divide` **NO es palabra clave**, **NO es función estándar**, **NO es builtin**, **NO es operador** ni constituye una firma o capability reservada.
   - No se define una forma oficial ni variantes para dicho enum prospectivo.
   - No se introducen tipos `Result<T, E>`, tuplas, retornos múltiples ni genéricos.
   - Definir un enum de usuario en v0.1 **no captura automáticamente** el fallo `DivisionByZeroError` originado por el operador `/`.


### 10.8 Operadores de comparación

Evo-Script v0.1 define seis operadores de comparación divididos en dos familias formales:

1. **Operadores de igualdad (`Equality Operators`)**:
   - `==` (Igual a)
   - `!=` (Diferente de)
2. **Operadores de orden (`Ordering Operators`)**:
   - `<` (Menor que)
   - `<=` (Menor o igual que)
   - `>` (Mayor que)
   - `>=` (Mayor o igual que)

Toda expresión de comparación (`Comparison Expression`) produce como resultado normal exactamente un valor de tipo `bool`.


#### 10.8.1 Principio general de tipado en comparaciones

Evo-Script v0.1 **no realiza conversiones implícitas, promociones ni coerciones** entre tipos durante una comparación.

1. **Regla de identidad exacta de tipos**:
   Para cualquier operador de comparación `left OP right`, el tipo semántico exacto del operando izquierdo (`left_type`) debe coincidir exactamente con el tipo semántico del operando derecho (`right_type`).
2. **Rechazo por discrepancia de tipos (`ComparisonTypeError`)**:
   Si los tipos de los operandos no coinciden exactamente, o si el tipo no es compatible con el operador específico (por ejemplo comparar `int` con `int64`, `int` con `float`, `string` con operadores de orden, o `dynamic` directamente), el programa es inválido y se rechaza estáticamente con el error:

   ```text
   ComparisonTypeError
   ```

   `ComparisonTypeError` pertenece a la categoría de **errores de validación del sistema** (`SystemError`); se detecta antes de la evaluación normal y **no** es un error de evaluación (`EvaluationError`).
3. **Conversión explícita requerida**:
   Para comparar valores de representaciones distintas, el programador debe convertir explícitamente uno de los operandos mediante la familia `to_tipo`:
   ```text
   to_int64(a) == b
   ```


#### 10.8.2 Semántica de operadores de igualdad (`==`, `!=`)

Los operadores `==` y `!=` determinan la equivalencia semántica de valores del mismo tipo exacto:

1. **Numéricos concretos**:
   - Se permite la igualdad entre operandos del mismo tipo numérico concreto:
     - Enteros con signo: `int`, `int8`, `int16`, `int32`, `int64`, `int128`.
     - Enteros sin signo: `uint8`, `uint16`, `uint32`, `uint64`, `uint128`.
     - Flotantes: `float`, `float32`, `float64`.
   - Comparaciones entre tipos numéricos distintos (como `int == int64` o `int32 == uint32`) son inválidas y producen `ComparisonTypeError`.
2. **Booleanos (`bool`)**:
   - `bool == bool` y `bool != bool` comparan el valor de verdad.
   - Ejemplos: `active == true`, `ready != false`.
3. **Cadenas de texto (`string`)**:
   - `string == string` y `string != string` comparan la igualdad por el **contenido textual completo** en codificación UTF-8.
   - No evalúan identidad de memoria, dirección de punteros ni identidad de objetos.
   - Ejemplo: `"hola" == "hola"` produce `true`.
4. **Estructuras (`struct`) - Igualdad estructural (`Structural Equality`)**:
   - `struct_a == struct_b` es válido únicamente si ambos operandos son exactamente del mismo tipo `struct`.
   - La igualdad de structs es estrictamente **estructural**: dos valores de struct son iguales si y solo si todos sus campos correspondientes son iguales entre sí.
   - Si un campo contiene otro `struct`, la igualdad se aplica recursivamente sobre los campos del struct anidado.
   - No existe igualdad por referencia ni identidad de objeto; dos instancias independientes con campos idénticos producen `true`.
5. **Enumeraciones (`enum`) - Igualdad de variantes y cargas**:
   - `enum_a == enum_b` es válido únicamente si ambos operandos pertenecen exactamente al mismo tipo `enum`.
   - La igualdad de enums se evalúa según la regla:
     1. Si representan **variantes distintas**, producen `false` (no error).
     2. Si representan la **misma variante simple**, producen `true`.
     3. Si representan la **misma variante con valores asociados o estructurados**, comparan sus cargas (*payloads*) aplicando recursivamente la igualdad estructural de sus respectivos tipos.
   - `==` determina únicamente la equivalencia de dos valores enum; no sustituye la descomposición ni correspondencia exhaustiva proporcionada por `when`.
6. **Prohibición sobre `dynamic`**:
   - `dynamic` no admite comparación directa con `==` ni `!=` (`dynamic_a == dynamic_b` produce `ComparisonTypeError`).
   - Para comparar valores dinámicos, deben convertirse explícitamente a un tipo concreto (`to_int(dynamic_a) == to_int(dynamic_b)`).
7. **Prohibición sobre Signatures y Funciones**:
   - Las firmas de función (`Signature Dependency Parameters`) y las funciones no son valores de primer orden y no pueden compararse con `==` ni `!=`.


#### 10.8.3 Semántica de operadores de orden (`<`, `<=`, `>`, `>=`)

Los operadores `<`, `<=`, `>`, `>=` evalúan relaciones de orden estricto o no estricto:

1. **Restricción exclusiva a tipos numéricos concretos**:
   - Los operadores de orden solo están permitidos entre operandos del **mismo tipo numérico concreto**:
     - `int < int`, `int64 >= int64`, `uint32 > uint32`, `float64 <= float64`.
2. **Prohibición de orden en otros tipos**:
   - **Cadenas (`string`)**: No se admite orden lexicográfico mediante operadores (`"a" < "b"` es inválido y produce `ComparisonTypeError`).
   - **Booleanos (`bool`)**: No poseen orden (`false < true` es inválido y produce `ComparisonTypeError`).
   - **Estructuras (`struct`)**: No poseen orden natural (`worker_a < worker_b` produce `ComparisonTypeError`).
   - **Enumeraciones (`enum`)**: No poseen orden por discriminante u ordinal (`resultado_a < resultado_b` produce `ComparisonTypeError`).
   - **Dinámicos (`dynamic`)**: Requieren conversión explícita previa (`to_int(dynamic_a) < to_int(dynamic_b)`).


#### 10.8.4 Matriz normativa de tipos comparables

| Tipo | `==` `!=` (Igualdad) | `<` `<=` `>` `>=` (Orden) | Observaciones |
| :--- | :---: | :---: | :--- |
| **Enteros con signo** (`int`, `int8`, `int16`, `int32`, `int64`, `int128`) | Sí | Sí | Mismo tipo exacto. Sin promoción automática. |
| **Enteros sin signo** (`uint8`, `uint16`, `uint32`, `uint64`, `uint128`) | Sí | Sí | Mismo tipo exacto. Sin coerción de signo. |
| **Flotantes** (`float`, `float32`, `float64`) | Sí | Sí | Mismo tipo exacto. |
| **Booleano** (`bool`) | Sí | No | Sin orden relacional. |
| **Cadena** (`string`) | Sí | No | Igualdad por contenido UTF-8. Sin orden lexicográfico por operador. |
| **Estructura** (`struct`) | Sí | No | Igualdad estructural campo por campo. Mismo tipo struct. |
| **Enumeración** (`enum`) | Sí | No | Misma variante y cargas iguales. Mismo tipo enum. |
| **Dinámico** (`dynamic`) | No | No | Requiere conversión explícita (`to_int`, `to_float64`). |
| **Signature Dependency** | No | No | No es un valor de primer orden. |
| **Function** | No | No | No es un valor de primer orden. |


#### 10.8.5 Prohibición de encadenamiento de comparaciones (No Comparison Chaining)

Los operadores de comparación **no son encadenables**:

- Expresiones como `a < b < c`, `a == b == c` o `a <= b >= c` son **sintácticamente inválidas**.
- Evo-Script no realiza reescritura implícita de encadenamientos. Para expresar rangos o condiciones conjuntas, deben escribirse explícitamente como operaciones lógicas con paréntesis obligatorios:
  ```text
  (a < b) && (b < c)
  ```


#### 10.8.6 Evaluación de izquierda a derecha en operadores binarios

1. **Orden de evaluación en operadores binarios ordinarios**:
   Para los operadores binarios que no aplican cortocircuito (aritméticos y de comparación), la evaluación procede estrictamente de izquierda a derecha:
   1. Se evalúa el operando izquierdo (`left`).
   2. Si la evaluación de `left` concluye exitosamente con un valor normal, se evalúa el operando derecho (`right`).
   3. Se aplica el operador sobre ambos valores normales.
2. **Operadores lógicos y cortocircuito**:
   En los operadores lógicos `&&` y `||`, la evaluación también comienza siempre por el operando izquierdo, pero la evaluación del operando derecho queda sujeta a la semántica formal de cortocircuito (Sección 10.9).
3. **Propagación de `EvaluationError` en operando izquierdo**:
   Si la evaluación de `left` produce un error de evaluación (`EvaluationError`, como `DivisionByZeroError`, `OverflowError` o `ConversionError`), la evaluación del operando derecho `right` **no llega a ejecutarse** y el fallo se propaga inmediatamente hacia afuera. Esta detención deriva de la propagación universal de errores de evaluación y no constituye cortocircuito lógico.
4. **Propagación de `EvaluationError` en operando derecho**:
   Si `left` se evalúa exitosamente pero `right` produce un `EvaluationError`, la operación binaria no produce su valor normal y el error se propaga hacia el exterior.


#### 10.8.7 Regla de paréntesis obligatorios con operadores lógicos (`&&`, `||`)

Evo-Script v0.1 establece normativamente la regla de **agrupación explícita mediante paréntesis**:

1. **Comparaciones aisladas**:
   Una expresión de comparación individual no requiere paréntesis cuando se utiliza en declaraciones `let`, argumentos o `return`:
   ```text
   let bool adulto = age >= 18;
   let bool mismo_nombre = worker.name == other_worker.name;
   let bool mismo_trabajador = worker == other_worker;
   return worker.age >= 18;
   ```
2. **Comparaciones como operandos de `&&` o `||`**:
   Cuando una expresión de comparación participa como operando de una conjunción (`&&`) o disyunción (`||`), **debe estar explícitamente agrupada entre paréntesis**:
   - **Válidos**:
     ```text
     (age >= 18) && active
     active && (age >= 18)
     (age >= 18) && (status == 1)
     (name == "Juan") || active
     active || (name != "Pedro")
     (name == "Juan") || (name != "Pedro")
     (worker.age >= 18) && worker.active
     ```
   - **Inválidos**:
     ```text
     age >= 18 && active              // Inválido: falta agrupación explícita
     active && age >= 18              // Inválido: falta agrupación explícita
     age >= 18 && status == 1         // Inválido: falta agrupación explícita
     name == "Juan" || active         // Inválido: falta agrupación explícita
     name == "Juan" || name != "Pedro" // Inválido: falta agrupación explícita
     ```
3. **Ausencia de resolución por precedencia implícita**:
   El lenguaje rechaza expresiones como `a == b && c != d` sin permitir que se resuelvan por precedencia implícita. La estructura lógica debe quedar visualmente inequívoca mediante `(a == b) && (c != d)`.
4. **Valores booleanos simples**:
   Los identificadores o expresiones booleanas simples (como `active`, `!ready`, `worker.active`) pueden participar directamente como operandos lógicos sin paréntesis obligatorios: `active && authorized`, `ready || cached`.
5. **Interacción con el cortocircuito booleano**:
   La regla de paréntesis define la estructura sintáctica obligatoria; la evaluación en runtime de los operandos lógicos aplica la semántica formal de cortocircuito definida en la Sección 10.9.


### 10.9 Operadores lógicos y cortocircuito (Short-Circuit Evaluation)

Evo-Script v0.1 define tres operadores lógicos:

| Operador | Significado | Tipado formal | Ejemplo |
| :--- | :--- | :--- | :--- |
| `&&` | AND lógico con cortocircuito | `bool × bool -> bool` | `active && authorized` |
| `\|\|` | OR lógico con cortocircuito | `bool × bool -> bool` | `active \|\| administrator` |
| `!` | Negación lógica (NOT) | `bool -> bool` | `!disabled` |

Reglas normativas generales:

1. **Tipado booleano estricto (Sin Truthiness)**:
   - Los operadores lógicos operan **única y exclusivamente sobre valores de tipo `bool`** y producen exactamente un valor de tipo `bool`.
   - Evo-Script no implementa conversiones implícitas a booleano (*truthiness*). Expresiones como `1 && true`, `"texto" || false` o `Worker { ... } && active` son sintáctica y semánticamente inválidas (`System / Validation Error`).
2. **Operador unario `!`**:
   - Actúa exclusivamente como negación booleana sobre un valor `bool`. No cumple funciones de unwrapping, aserción ni propagación de errores. No aplica cortocircuito.
3. **Naturaleza de expresión**:
   - Las operaciones lógicas son `Expressions` monovalor. No constituyen sentencias de control de flujo (`if`, `else`) ni `Operation Statements` (`a && b;` es inválido como sentencia).


#### 10.9.1 Semántica de cortocircuito de `&&` (Logical AND)

Para una expresión `left && right`:

1. Se evalúa el operando izquierdo (`left`).
2. `left` debe evaluarse exitosamente a un valor de tipo `bool`.
3. **Condición de cortocircuito (`false`)**:
   - Si `left == false`, el resultado final de la expresión es inmediatamente `false` y el operando derecho (`right` / `RHS`) **NO se evalúa**.
4. **Evaluación de operando derecho (`true`)**:
   - Si `left == true`, se evalúa el operando derecho (`right`), el cual debe producir un valor de tipo `bool`. El resultado final de la expresión es exactamente el valor producido por `right`.

Resumen:
- `false && RHS` $\rightarrow$ `false` (`RHS` no se evalúa).
- `true && RHS` $\rightarrow$ se evalúa `RHS` $\rightarrow$ resultado = valor de `RHS`.


#### 10.9.2 Semántica de cortocircuito de `||` (Logical OR)

Para una expresión `left || right`:

1. Se evalúa el operando izquierdo (`left`).
2. `left` debe evaluarse exitosamente a un valor de tipo `bool`.
3. **Condición de cortocircuito (`true`)**:
   - Si `left == true`, el resultado final de la expresión es inmediatamente `true` y el operando derecho (`right` / `RHS`) **NO se evalúa**.
4. **Evaluación de operando derecho (`false`)**:
   - Si `left == false`, se evalúa el operando derecho (`right`), el cual debe producir un valor de tipo `bool`. El resultado final de la expresión es exactamente el valor producido por `right`.

Resumen:
- `true || RHS` $\rightarrow$ `true` (`RHS` no se evalúa).
- `false || RHS` $\rightarrow$ se evalúa `RHS` $\rightarrow$ resultado = valor de `RHS`.


#### 10.9.3 Tabla normativa de evaluación en runtime

| Expresión | ¿Se evalúa RHS en runtime? | Resultado final |
| :--- | :---: | :--- |
| `false && RHS` | **No** | `false` |
| `true && RHS` | **Sí** | Valor de `RHS` |
| `true \|\| RHS` | **No** | `true` |
| `false \|\| RHS` | **Sí** | Valor de `RHS` |

> [!NOTE]
> `RHS` (*Right-Hand Side*) debe ser siempre una expresión estáticamente tipada como `bool` y válida según las reglas del lenguaje, con independencia de que en runtime su evaluación pueda ser omitida por cortocircuito.


#### 10.9.4 Separación entre validación estática y evaluación en runtime

Evo-Script distingue formalmente dos niveles en el procesamiento de expresiones lógicas:

1. **Validación estática (`Static Validation`)**:
   - Se ejecuta antes del runtime y valida exhaustivamente **ambos operandos** (`left` y `right`).
   - Ambos operandos deben ser expresiones de tipo `bool` válidas en el sistema de tipos.
   - Los errores estructurales y de resolución (como `FunctionNotFoundError`, `FunctionArityError`, `FunctionArgumentTypeError`, `ComparisonTypeError`, `FieldNotFoundError`, `FieldAccessTypeError`) invalidan y rechazan el programa antes de la ejecución, **sin importar** si en runtime el operando derecho hubiese sido omitido por cortocircuito:
     ```text
     false && funcion_inexistente()   // Inválido: FunctionNotFoundError antes de ejecutar
     false && 10                      // Inválido: tipo int no admitido en operador lógico
     true || "texto"                  // Inválido: tipo string no admitido en operador lógico
     ```
2. **Evaluación en runtime (`Runtime Evaluation`)**:
   - Durante la ejecución, el evaluador procesa de izquierda a derecha y aplica las reglas de cortocircuito para inhibir la evaluación de `RHS` cuando el resultado queda determinado por `left`.


#### 10.9.5 Interacción con EvaluationError y efectos externos

1. **`EvaluationError` en operando izquierdo**:
   - Si `left` produce un error de evaluación (`EvaluationError`, por ejemplo `DivisionByZeroError`), la evaluación actual aborta y `RHS` **no se evalúa**. Esta omisión no es cortocircuito lógico booleano, sino la consecuencia de la propagación del fallo de evaluación.
2. **`EvaluationError` en `RHS` cuando es evaluado**:
   - Si las reglas de cortocircuito requieren evaluar `RHS` (`true && RHS` o `false || RHS`) y `RHS` produce un `EvaluationError`, el fallo detiene la evaluación y se propaga hacia el exterior.
3. **Inexistencia de `EvaluationError` en `RHS` omitido**:
   - Si `RHS` es omitido por cortocircuito, cualquier fallo de evaluación que hubiese ocurrido exclusivamente durante su ejecución en runtime **no ocurre**:
     ```text
     false && ((10 / 0) > 1)          // Válido: División entre cero en RHS no ocurre en runtime
     true || (overflow_val + 1 > 0)   // Válido: Overflow en RHS no ocurre en runtime
     ```
   - Regla fundamental: *lo que no se evalúa en runtime no puede producir EvaluationError*.
4. **Efectos externos en `RHS` omitido**:
   - Si `RHS` invoca una función o Signature Dependency Parameter con efectos externos y `RHS` es omitido por cortocircuito, dicha función **no se ejecuta** y sus efectos externos no tienen lugar:
     ```text
     autorizado || solicitar_autorizacion()
     ```
     Si `autorizado == true`, `solicitar_autorizacion()` no se ejecuta.
   - Evo-Script no incorpora anotaciones de pureza ni sistemas de efectos (`IO`); la ausencia de efectos es el resultado natural de la no evaluación de la subexpresión.


#### 10.9.6 Encadenamiento homogéneo de operadores lógicos

Evo-Script permite encadenar múltiples instancias del **mismo operador lógico** sin requerir paréntesis adicionales:

1. **Encadenamiento de `&&`**:
   - `a && b && c` se asocia estructuralmente por la izquierda como `(a && b) && c`.
   - **Evaluación secuencial con cortocircuito**:
     1. Se evalúa `a`. Si `a == false` $\rightarrow$ resultado `false`; `b` y `c` **no se evalúan**.
     2. Si `a == true` $\rightarrow$ se evalúa `b`. Si `b == false` $\rightarrow$ resultado `false`; `c` **no se evalúa**.
     3. Si `b == true` $\rightarrow$ se evalúa `c` y el resultado final es el valor de `c`.
2. **Encadenamiento de `||`**:
   - `a || b || c` se asocia estructuralmente por la izquierda como `(a || b) || c`.
   - **Evaluación secuencial con cortocircuito**:
     1. Se evalúa `a`. Si `a == true` $\rightarrow$ resultado `true`; `b` y `c` **no se evalúan**.
     2. Si `a == false` $\rightarrow$ se evalúa `b`. Si `b == true` $\rightarrow$ resultado `true`; `c` **no se evalúa**.
     3. Si `b == false` $\rightarrow$ se evalúa `c` y el resultado final es el valor de `c`.


#### 10.9.7 Exigencia estricta de paréntesis al mezclar `&&` y `||`

Evo-Script v0.1 **prohíbe la mezcla directa de operadores `&&` y `||` sin paréntesis explícitos**:

1. **Prohibición de precedencia implícita entre `&&` y `||`**:
   - A pesar de que la tabla general define una jerarquía de operadores, el lenguaje **no permite** que expresiones que combinan conjunciones y disyunciones se resuelvan por precedencia implícita.
   - La intención sintáctica debe quedar visual y estructuralmente inequívoca mediante paréntesis obligatorios.
2. **Formas inválidas**:
   ```text
   a && b || c              // Inválido: mezcla && y || sin agrupación explícita
   a || b && c              // Inválido: mezcla || y && sin agrupación explícita
   a && b || c && d         // Inválido: mezcla sin agrupación explícita
   a || b && c || d         // Inválido: mezcla sin agrupación explícita
   (age >= 18) && active || admin // Inválido: mezcla sin paréntesis lógicos
   ```
3. **Formas válidas**:
   ```text
   (a && b) || c            // Válido: conjunción agrupada explícitamente
   a && (b || c)            // Válido: disyunción agrupada explícitamente
   a || (b && c)            // Válido: conjunción agrupada explícitamente
   (a || b) && c            // Válido: disyunción agrupada explícitamente
   ((age >= 18) && active) || admin // Válido: agrupación lógica completa
   ```


#### 10.9.8 Composición canónica con comparaciones y bindings

- **Conjunción con comparación agrupada**:
  ```text
  let bool permitido = (age >= 18) && active;
  ```
- **Conjunción de comparaciones encadenadas**:
  ```text
  let bool en_rango = (a < b) && (b < c);
  ```
- **Conjunción múltiple con comparación**:
  ```text
  let bool acceso_completo = (age >= 18) && active && authorized;
  ```
- **Disyunción con comparación y bindings**:
  ```text
  let bool encontrado = (name == "Juan") || cached || local;
  ```
- **Retorno con composición lógica**:
  ```text
  return (worker.age >= 18) && worker.active;
  ```


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

Los paréntesis `( )` permiten establecer explícitamente la agrupación sintáctica de una expresión:

    (a + b) * c

El uso de paréntesis tiene como único propósito la agrupación sintáctica; no define tuplas ni tipos compuestos.

Evo-Script v0.1 define la jerarquía completa de precedencia y asociatividad de operadores de la siguiente manera (de mayor a menor precedencia):

1. **Acceso a campos** (`.`) — asociatividad por la izquierda (operación postfix de mayor precedencia).
2. **Operadores unarios prefijos** (`!`, `-`) — asociatividad por la derecha.
3. **Multiplicativos** (`*`, `/`, `%`) — asociatividad por la izquierda.
4. **Aditivos** (`+`, `-`) — asociatividad por la izquierda.
5. **Comparaciones** (`<`, `<=`, `>`, `>=`, `==`, `!=`) — no encadenables.
6. **Conjunción lógica** (`&&`) — asociatividad por la izquierda.
7. **Disyunción lógica** (`||`) — asociatividad por la izquierda.
8. **Pipeline** (`|>`) — asociatividad por la izquierda (menor precedencia de todos los operadores).

> [!IMPORTANT]
> A pesar de la jerarquía de precedencia general, la sintaxis de Evo-Script exige **agrupación explícita obligatoria con paréntesis** en los siguientes casos:
> 1. Cuando una expresión de comparación participa como operando de `&&` o `||` (Sección 10.8.7).
> 2. Cuando se mezclan operadores `&&` y `||` en una misma expresión lógica (Sección 10.9.7).
> No se admite confiar en precedencia implícita en estas combinaciones.

Ejemplos:

- `worker.age + 10` equivale semánticamente a `(worker.age) + 10`.
- `worker.name |> to_string` equivale semánticamente a `(worker.name) |> to_string`.
- `a + b * c` equivale semánticamente a `a + (b * c)`.
- `(a > 10) && (b < 20)` define la conjunción explícita de dos comparaciones agrupadas.
- `(a && b) || c` define la disyunción explícita de una conjunción agrupada con un tercer operando.
- `a + b |> to_string` equivale semánticamente a `(a + b) |> to_string`.
- `(a > b) |> to_string` equivale semánticamente a `(a > b) |> to_string`.


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

    esig to_string(int value) -> string;

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

    public fn sumar(int a, int b) -> int {
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
   public fn calcular_texto(int a, int b) -> string {
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
private fn sumar(int value, int amount) -> int {
    return value + amount;
}

private fn multiplicar(int value, int factor) -> int {
    return value * factor;
}

public fn calcular(int value) -> string {
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

private fn buscar_trabajador(int id) -> BuscarTrabajadorResultado {
    correspondencia
}

private fn describir_trabajador(Trabajador trabajador) -> string {
    correspondencia
}

public fn obtener_mensaje(BuscarTrabajadorResultado resultado) -> string {
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

Dentro del subsistema de expresiones y operadores en Evo-Script v0.1, no existen especificaciones semánticas pendientes. Todos los aspectos léxicos, sintácticos, de tipado, precedencia, evaluación, cortocircuito, división, residuo y propagación de errores se encuentran formalmente definidos y cerrados.


## 11. Funciones

La unidad semántica fundamental de ejecución y cómputo se denomina `Function`.

En Evo-Script v0.1 existen dos representaciones para funciones:

1. **Implementación de función (`Function Implementation`)**: declarada dentro de archivos `.efn` mediante la palabra clave `fn`, con visibilidad explícita (`public fn` o `private fn`), cuerpo delimitado por `{ ... }` y un único `return expresion;` obligatorio.
2. **Firma de función (`Function Signature`)**: declarada dentro de archivos `.esig` mediante la palabra clave `esig`, con la forma `esig nombre(argumentos) -> Tipo;`, sin cuerpo ni modificadores de visibilidad.

La forma textual general de una implementación de función en Evo-Script v0.1 es:

    public fn nombre(tipo argumento, tipo argumento) -> tipo {
        cero_o_mas_body_statements
        return expresion;
    }

o para funciones auxiliares privadas:

    private fn nombre(tipo argumento, tipo argumento) -> tipo {
        cero_o_mas_body_statements
        return expresion;
    }

Ejemplo canónico:

    public fn sumar(int numero, int numero2) -> int {
        return numero + numero2;
    }


### 11.1 Declaración y visibilidad

La palabra clave `fn` inicia textualmente la declaración de una implementación de función dentro de un archivo `.efn`. La palabra clave `esig` inicia textualmente la declaración de una firma en un archivo `.esig`.

En archivos de implementación `.efn`, la visibilidad de cada función debe ser explícita:

- `public`: declara la única función pública principal del archivo `.efn`.
- `private`: declara funciones auxiliares de uso estrictamente local dentro del mismo archivo `.efn`.

No existe visibilidad implícita. En firmas públicas `.esig`, la firma es pública por naturaleza y se declara directamente como `esig nombre(...) -> Tipo;` sin modificadores de visibilidad (`public` y `private` no aplican a `.esig`).


### 11.2 Nombre

En la declaración:

    public fn guardar(...)

el identificador `guardar` define el nombre de la función dentro del programa.


### 11.3 Parámetros de funciones (Function Parameters)

Evo-Script v0.1 distingue formalmente dos clases de parámetros dentro de la lista posicional de una función:

1. **Parámetros de datos (`Value Parameters`)**: Representan valores de datos evaluables que participan en el cómputo de la función.
2. **Parámetros de dependencia de firma (`Signature Dependency Parameters`)**: Declaran capacidades funcionales requeridas por la función, tipadas mediante una Evo Signature importada.

Ambas clases de parámetros se escriben dentro de la misma y única lista posicional de la función, separados por comas. No existen múltiples listas de parámetros (como `fn foo(a)(b)`) ni bloques especiales (`dependencies { ... }`, `inject { ... }`, `requires { ... }`).

#### 11.3.1 Parámetros de datos (Value Parameters)

La regla oficial para la declaración de parámetros de datos es:

    tipo primero, nombre después

Ejemplos:

    int count
    float value
    Trabajador trabajador

No se utiliza la sintaxis invertida con dos puntos (`trabajador: Trabajador`).

Ejemplo de función con múltiples parámetros de datos:

    public fn ejemplo(int id, float amount, Trabajador trabajador) -> ...

#### 11.3.2 Parámetros de dependencia de firma (Signature Dependency Parameters)

Una Function Implementation puede declarar explícitamente que requiere una capacidad externa especificando una firma importada en su lista de parámetros:

    modulo::firma nombre_local

Ejemplo canónico:

    import values::SearchResult;
    import values::search;

    public fn process(int id, values::search search) -> SearchResult {
        return search(id);
    }

Interpretación del ejemplo:
- `int id`: Parámetro de datos (`Value Parameter`) que recibe un valor normal de tipo `int`.
- `values::search search`: Parámetro de dependencia de firma (`Signature Dependency Parameter`), donde:
  - `values::search`: Identifica formalmente la Signature requerida por la función.
  - `search`: Define el nombre local de la capacidad dentro del cuerpo de la función.

Reglas normativas:

1. **Obligatoriedad de `import`**: Toda Signature utilizada como parámetro de dependencia debe estar explícitamente importada al inicio del archivo `.efn` mediante `import modulo::firma;`. Declarar `values::search search` no sustituye a la cláusula `import`.
2. **Prohibición de `esig` en parámetros**: La palabra clave `esig` está reservada exclusivamente para la declaración de contratos dentro de archivos `.esig`. La sintaxis `esig values::search search` es inválida. La forma canónica es directamente `values::search search`.
3. **No es un tipo de datos ordinario (`Value`)**: Una Signature no forma parte del sistema de valores normales del lenguaje. No puede utilizarse como tipo de un binding (`let values::search x = ...` es inválido), como tipo de retorno de una función (`-> values::search` es inválido), como campo de una estructura (`struct Service { values::search s; }` es inválido) ni como dato asociado de un enum (`enum E { V(values::search) }` es inválido).
4. **Ausencia de funciones de primer orden y reenvío estricto de dependencias**: Un parámetro de dependencia no es un valor de primer orden (`first-class function`). No puede asignarse a variables (`let x = search;` es inválido), retornarse desde funciones (`return search;` es inválido) ni almacenarse en campos de estructuras o variantes de enums. Sin embargo, un `Signature Dependency Parameter` **SÍ puede ser reenviado / transportado (`forwarded`)** como argumento hacia otra invocación funcional o de firma que declare formalmente en su lista de parámetros exactamente la misma Signature (`values::search`). Este transporte de dependencias de capacidad no convierte a las firmas en valores de datos ordinarios.
5. **Invocación de la capacidad**: Dentro del cuerpo de la función, la capacidad requerida se invoca mediante su nombre local (`search(id)`). La llamada obedece estrictamente las reglas de validación de la Signature declarada: aridad exacta, correspondencia exacta de tipos de argumentos, evaluación de argumentos de izquierda a derecha y ausencia de conversiones implícitas.
6. **Autonomía del nombre local**: El identificador local (`search`) es exclusivo del ámbito interno de esa función. La identidad formal y contractual de la dependencia continúa siendo `values::search`. Declarar `values::search employee_search` es válido y se invoca internamente como `employee_search(id)`.
7. **Distinción con alias de importación**: Un alias de importación (`import values::search as imported_search;`) opera a nivel de archivo para llamadas directas, mientras que el nombre local de un Signature Dependency Parameter (`values::search search`) opera exclusivamente en el ámbito léxico de esa Function Implementation.
8. **Asistencia de tooling (Nota no normativa)**: La presencia previa de `import values::search;` permite que editores y servidores de lenguaje (LSP) conozcan la firma y asistan al desarrollador con autocompletado y validación de tipos al declarar parámetros de dependencia e invocar sus capacidades.
9. **Reenvío estricto de dependencias (`Signature Dependency Forwarding`)**: Una función que recibe un `Signature Dependency Parameter` puede pasarlo como argumento posicional a otra función o firma que espere exactamente la misma dependencia de firma (`modulo::firma`).
   ```text
   import values::SearchResult;
   import values::search;
   import workers::lookup;

   public fn process(int id, values::search search) -> SearchResult {
       return lookup(id, search);
   }
   ```
   donde `lookup` declara `values::search search` en sus parámetros. El transporte de la capacidad es estático y no involucra closures ni punteros a función como datos.


### 11.3.3 Nota de diseño prospectiva para Evo-Script v0.2: dependencia por nombre importado

Se registra exclusivamente con carácter de **nota de diseño prospectiva NO operativa para una posible versión v0.2** la idea conceptual de una simplificación sintáctica en la declaración de `Signature Dependency Parameters`.

1. **Estado normativo y obligatorio en Evo-Script v0.1**:
   En Evo-Script v0.1, la forma normativa y obligatoria continúa siendo la especificación explícita y calificada de la firma:

   ```text
   import callbacks::completed;

   public fn execute(string value, callbacks::completed completed) -> ResultType {
       ...
   }
   ```

   La forma oficial de un Signature Dependency Parameter en v0.1 sigue siendo:

   ```text
   modulo::firma nombre_local
   ```

   Por ejemplo:

   ```text
   callbacks::completed completed
   ```

   El `import` hace conocida la Signature en el archivo, pero no elimina en v0.1 la necesidad de declarar explícitamente su identidad formal en el parámetro.

2. **Candidato conceptual para v0.2 (Dependencia por nombre importado)**:
   Para una futura Evo-Script v0.2 se registra como candidato conceptual una forma abreviada donde una Signature previamente importada pueda utilizarse directamente por su nombre local dentro de la lista de parámetros.

   Ejemplo prospectivo:

   ```text
   import callbacks::completed;

   public fn execute(string value, completed) -> ResultType {
       ...
   }
   ```

   En esta propuesta futura, `completed` seguiría siendo semánticamente un `Signature Dependency Parameter`.

   NO sería:
   - un Value Parameter;
   - una función como Value;
   - una closure;
   - una lambda;
   - un function pointer de primer orden;
   - un parámetro con tipo dinámico;
   - inferencia general de tipos.

   La identidad formal del parámetro se resolvería estáticamente a partir de la Signature importada con ese nombre local.

   Conceptualmente:

   ```text
   import callbacks::completed;
                  │
                  └── introduce `completed` en Signature Space

   public fn execute(string value, completed) -> ResultType
                                   │
                                   └── dependencia de callbacks::completed
   ```

3. **Preservación estricta de la separación entre Signature Space y Value Space**:
   La propuesta preserva la separación total entre `Signature Space` y `Value Space`. Una dependencia abreviada seguiría sin pertenecer al espacio de valores y no podría utilizarse como dato.

   Continuarían siendo inválidas construcciones conceptuales como:

   ```text
   let x = completed;
   return completed;
   ```

   y tampoco podría almacenarse en structs, enums o bindings.

4. **Integración natural con alias de importación (`as`)**:
   La propuesta se integra naturalmente con los aliases `as` que ya existen en Evo-Script para desambiguar firmas homónimas importadas:

   ```text
   import callbacks_a::completed as first_completed;
   import callbacks_b::completed as second_completed;

   public fn execute(string value, first_completed, second_completed) -> ResultType {
       ...
   }
   ```

   Aquí:
   - `first_completed` representaría una dependencia cuya identidad formal continúa siendo `callbacks_a::completed`.
   - `second_completed` representaría `callbacks_b::completed`.

   El alias solo define el nombre local en `Signature Space`; no altera la identidad formal de la Signature.

5. **Motivación de eliminación de redundancia visual**:
   Esta propuesta futura tiene como motivación eliminar redundancia visual.

   La forma v0.1:

   ```text
   import requesters::completed;

   public fn execute(string value, requesters::completed completed) -> ResultType {
       ...
   }
   ```

   podría expresarse en una futura v0.2 como:

   ```text
   import requesters::completed;

   public fn execute(string value, completed) -> ResultType {
       ...
   }
   ```

   sin modificar la arquitectura ni la semántica de dependencia funcional.

6. **Preservación del transporte composicional (`Signature Dependency Forwarding`)**:
   El `Signature Dependency Forwarding` seguiría funcionando exactamente como transporte estructural de capacidades. La abreviación no convertiría las Signatures en Values.

   Por ejemplo, conceptualmente:

   ```text
   import callbacks::completed;

   public fn execute(string value, completed) -> ResultType {
       return process(value, completed);
   }
   ```

   seguiría significando que `completed` transporta una dependencia cuya identidad formal fue resuelta estáticamente desde `callbacks::completed`.

7. **Delimitación formal y carácter NO operativo en v0.1**:
   Se deja expresamente indicado que esta sintaxis abreviada es:
   - un candidato conceptual para Evo-Script v0.2;
   - no normativa;
   - no operativa;
   - no implementada;
   - inválida en Evo-Script v0.1.

   No se formaliza todavía su gramática definitiva ni se agregan errores nuevos del sistema en v0.1; se registra únicamente como idea conceptual para consideración futura.

8. **Restricciones absolutas de alcance**:
   Esta nota prospectiva **NO introduce**:
   - closures;
   - lambdas;
   - first-class functions;
   - function pointers como Values;
   - inferencia general de tipos;
   - Value Parameters sin tipo explícito;
   - resolución dinámica;
   - nuevas keywords;
   - nuevas extensiones;
   - cambios a `import`;
   - cambios a `as`;
   - cambios a `.root`.


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


### 11.6 Estructura de la correspondencia y cuerpo de función (Function Body)

La correspondencia o cuerpo de una función (`Function Body`) está delimitada por llaves `{ ... }` y posee la siguiente estructura:

    Function Implementation (.efn)
    ├── visibility (public | private)
    ├── name
    ├── parameters (Value Parameters / Signature Dependency Parameters)
    ├── result type (-> Tipo)
    └── Function Body / Correspondence ({ ... })
        ├── zero or more Body Statements
        │   ├── Let Binding Statement (`let Tipo nombre = expression;`)
        │   └── Operation Statement
        │       ├── Function Call (`nombre(args...);`)
        │       └── Pipeline Expression (`source |> op1 |> op2;`)
        └── exactly one Final Return (`return expression;`)

#### 11.6.1 Sentencias del cuerpo (Body Statements)

Evo-Script v0.1 reconoce **exactamente dos clases** de sentencias en el cuerpo de una función previas al `return` final:

1. **Sentencia de binding (`Let Binding Statement`)**:
   - Declara un binding inmutable asociando un nombre tipado con el valor producido por una expresión:
     ```text
     let Tipo nombre = expresion;
     ```
   - El valor resultante se conserva en el ámbito léxico de la función para ser utilizado por sentencias posteriores o por el `return` final.
   - Preserva todas las reglas de inmutabilidad, ausencia de reasignación y tipado estricto (Sección 8).

2. **Sentencia de operación (`Operation Statement`)**:
   - Representa una operación evaluable ejecutada por sus efectos o capacidad de cómputo dentro de la secuencia de una función, cuyo valor normal resultante **no se conserva en un binding ni constituye el resultado final** de la función.
   - Las únicas dos formas válidas de `Operation Statement` en Evo-Script v0.1 son:
     ```text
     function_call;
     pipeline_expression;
     ```

#### 11.6.2 Semántica del Operation Statement y descarte del valor normal

1. **Evaluación normal y descarte intencional**:
   - Una sentencia de operación se evalúa normalmente según las reglas estándar de Function Calls o Pipelines.
   - El valor tipado normal (`Value`) producido por la operación es **descartado deliberadamente** por el llamador al no asociarse a un binding:
     ```text
     Function Call / Pipeline
               ↓
     produces typed Value
               ↓
       Operation Statement
               ↓
     normal Value discarded
               ↓
       continue evaluation
     ```
2. **El descarte de valor NO es `void` ni `Unit`**:
   - Descartar el valor normal de una llamada no altera el tipo declarado de la función invocada. Si una firma declara `esig open() -> WindowResult;`, la llamada `open_window();` produce normalmente `WindowResult`, pero el llamador elige no utilizar dicho valor.
   - Evo-Script v0.1 **NO** introduce tipos `void`, `Unit`, `()`, ni procedimientos sin retorno. Toda función y firma continúa declarando un tipo de retorno obligatorio.
3. **Ausencia de sintaxis especial para descarte**:
   - No se requieren palabras clave ni comodines como `_`, `discard`, `ignore`, `let _` ni `drop`.
4. **Descarte de alternativas de dominio (Domain Alternatives)**:
   - Si una operación retorna un enum de dominio (por ejemplo `SaveResult::Saved` o `SaveResult::Error("fallo")`), el llamador puede utilizarla como `Operation Statement` si decide no inspeccionar el resultado.
5. **Los errores de evaluación (EvaluationError) NUNCA se descartan**:
   - Si un `Operation Statement` produce un error de evaluación (`DivisionByZeroError`, `OverflowError`, `ConversionError`), dicho error **no puede ser descartado**:
     ```text
     operation_a(); // Falla con EvaluationError
     operation_b(); // NO se evalúa
     return result; // NO se evalúa
     ```
   - La evaluación actual aborta inmediatamente y el fallo se propaga hacia afuera hasta el Evo Runtime (Sección 10.7 y 12.9.7).
6. **Los errores estáticos / de validación (System / Validation Errors) NUNCA se descartan**:
   - Invalideces como firmas no encontradas, discordancia de aridad o tipos incompatibles invalidan el programa antes de la evaluación.

#### 11.6.3 Pipelines como Operation Statements

1. **Pipeline completo con terminación en punto y coma**:
   - Un pipeline completo puede utilizarse como `Operation Statement` terminando la expresión con punto y coma (`;`):
     ```text
     worker
     |> validate
     |> save;
     ```
2. **Operador oficial de pipeline**:
   - El operador de pipeline es única y exclusivamente `|>`.
   - Para operaciones de aridad 1: `valor |> operacion`
   - Para operaciones de aridad $\ge 2$: `valor |> operacion(this, arg)`
3. **Punto y coma al final del pipeline**:
   - El punto y coma (`;`) finaliza la sentencia completa del pipeline; no se coloca `;` en cada stage intermedio.

#### 11.6.4 Prohibición de Expression Statements generales

Evo-Script v0.1 **NO define Expression Statements generales**. Las expresiones arbitrarias terminadas en punto y coma que no sean una Function Call ni una Pipeline Expression son **inválidas**:

- `10 + 20;` (Inválido: expresión aritmética como statement)
- `true;` (Inválido: literal booleano como statement)
- `worker;` (Inválido: binding como statement)
- `"hello";` (Inválido: literal de texto como statement)
- `SearchResult::NotFound;` (Inválido: variante de enum como statement)
- `worker.name;` (Inválido: acceso a campo como statement)
- `active && ready;` (Inválido: expresión lógica como statement)

Las expresiones aritméticas, lógicas y literales solo son válidas donde se espera un `Value` (inicializador de `let`, argumento de llamada, expresión de `return`, entrada de pipeline).

#### 11.6.5 Orden secuencial estricto de evaluación (Top-to-Bottom)

1. **Garantía de orden secuencial**:
   - Las sentencias del cuerpo de la función (`Body Statements`) se evalúan estrictamente en orden textual de arriba hacia abajo (*top-to-bottom*).
   - El runtime / evaluador **no puede reordenar** estas operaciones:
     ```text
     let Config config = load_config(); // 1. Se evalúa load_config() y se crea config
     validate_config(config);          // 2. Se evalúa validate_config
     open_window();                     // 3. Se evalúa open_window
     return InitResult::Ready;          // 4. Se evalúa el return final
     ```
2. **Coordinación de capacidades con efectos**:
   - En operaciones con efectos externos sobre el entorno o interfaz, el orden textual garantiza la secuencia de invocación requerida (`load_config();` precede estrictamente a `open_window();`).

#### 11.6.6 El retorno final (Final Return)

1. **Obligatoriedad y posición final estricta**:
   - Toda función concluye obligatoriamente con exactamente una sentencia `return expresion;`.
   - `return` debe ser estrictamente la última sentencia del cuerpo. No se admiten sentencias posteriores a `return`.
2. **Ausencia de retornos tempranos (No Early Return)**:
   - No existen retornos tempranos ni condicionales (`early return`). No se admiten múltiples sentencias `return`.
3. **Independencia frente al ciclo de vida**:
   - `Function return` finaliza la evaluación de la función; no es un `Application Exit Request` ni termina el Application Main Loop.

#### 11.6.7 Ejemplos canónicos válidos

- **Función pura directa**:
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
- **Función con sentencias de operación secuenciales**:
  ```text
  public fn initialize(window::open open_window, config::load load_config) -> InitResult {
      load_config();
      open_window();

      return InitResult::Ready;
  }
  ```
- **Función con let y operation statement**:
  ```text
  public fn process(
      Source source,
      source::read read_source,
      destination::save save
  ) -> ProcessResult {
      let Data data = read_source(source);

      save(data);

      return ProcessResult::Completed;
  }
  ```
- **Función con pipeline como Operation Statement**:
  ```text
  public fn update_worker(
      Worker worker,
      worker::validate validate,
      storage::save save
  ) -> UpdateResult {
      worker
      |> validate
      |> save;

      return UpdateResult::Success;
  }
  ```
- **Función con return pipeline**:
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


### 11.7 Llamadas de funciones (Function Calls)

Una llamada de función (`Function Call`) es una `Expression` de Evo-Script que invoca una función resoluble en el contexto local, evalúa sus argumentos y produce un único valor tipado correspondiente al tipo de resultado declarado en `-> Tipo`.

#### 11.7.1 Forma general

La sintaxis canónica de una llamada de función es:

    nombre(argumento1, argumento2, ..., argumentoN)

Ejemplos:

    sumar(10, 20)

    prepare(clothes)

    process(to_int64(value))

Si una función auxiliar declara:

    private fn double(int value) -> int {
        return value * 2;
    }

la expresión `double(10)` produce un valor de tipo `int` (en este caso `20`).

#### 11.7.2 Ejemplo canónico de llamadas locales

```text
private fn double(int value) -> int {
    return value * 2;
}

private fn add(int value, int amount) -> int {
    return value + amount;
}

public fn calculate(int value) -> int {
    let int doubled = double(value);

    return add(doubled, 10);
}
```

En este ejemplo canónico:
- `double` y `add` son funciones auxiliares privadas (`private fn`).
- `calculate` es la única función pública principal (`public fn`).
- La llamada directa `double(value)` asigna el valor resultante al binding inmutable `doubled`.
- La llamada directa `add(doubled, 10)` evalúa sus argumentos posicionales con tipos exactos y produce el resultado retornado por `calculate`.

#### 11.7.3 Invocación de aridad cero

Toda función declarada sin argumentos debe invocarse obligatoriamente utilizando paréntesis vacíos:

    operacion()

Ejemplo:

```text
private fn get_value() -> int {
    return 10;
}

public fn run() -> int {
    return get_value();
}
```

Reglas normativas:
1. **Obligatoriedad de los paréntesis**: No se permite invocar una función sin argumentos omitiendo los paréntesis (`let int x = get_value;` es inválido).
2. **Diferenciación estricta**: Los paréntesis vacíos `()` garantizan la distinción inequívoca entre una invocación de función y la referencia a un identificador o binding.

#### 11.7.4 Argumentos posicionales

La correspondencia entre los argumentos suministrados en una llamada y los parámetros declarados por la función es estrictamente posicional:

```text
private fn subtract(int left, int right) -> int {
    return left - right;
}
```

En la llamada:

    subtract(a, b)

el primer argumento `a` se asocia con `left` y el segundo argumento `b` se asocia con `right`. El orden de los argumentos es determinante.

En Evo-Script v0.1 quedan formalmente excluidos:
- Argumentos con nombre (`named arguments` o `keyword arguments`).
- Argumentos por defecto (`default arguments`).
- Argumentos de longitud variable (`variadic arguments`).

#### 11.7.5 Llamadas anidadas y participación en expresiones

Una llamada de función puede participar en cualquier contexto sintáctico donde una expresión de su tipo de resultado sea válida:

1. **Llamadas anidadas (`Nested Calls`)**:
   ```text
   process(normalize(value))
   ```
2. **Inicialización de bindings**:
   ```text
   let int x = calculate(value);
   ```
3. **Sentencias return**:
   ```text
   return calculate(value);
   ```
4. **Expresiones aritméticas y lógicas**:
   ```text
   calculate(value) + 10
   ```
5. **Stages de pipeline**:
   ```text
   value |> calculate(this, 10)
   ```

#### 11.7.6 Relación con el pipeline

La invocación directa y la composición en pipeline responden a las reglas semánticas ya establecidas:
- **Aridad 1**: En un pipeline, `value |> normalize` constituye la forma canónica de composición monovalor. No se transforma textualmente en `normalize(value)`, aunque semánticamente ambos entreguen `value` a la operación. La forma `value |> normalize()` no está permitida como stage de pipeline.
- **Aridad >= 2**: En un pipeline, `value |> operation(this, arg)` requiere obligatoriamente el placeholder contextual `this` en la primera posición.


### 11.8 Resolución de llamadas, espacios semánticos y desambiguación

Dentro del ámbito de un archivo `.efn`, la resolución de identificadores y llamadas opera bajo reglas estrictas de separación semántica, identidad y unicidad local.

#### 11.8.1 Espacios semánticos y clases de símbolos

Evo-Script v0.1 distingue formalmente distintas clases de símbolos en espacios semánticos separados:

- **Function Implementations**: Funciones locales (`public fn`, `private fn`) definidas dentro del archivo `.efn`.
- **Signatures**: Contratos públicos (`esig`) disponibles mediante declaraciones `import modulo::firma`.
- **Types**: Estructuras y enumeraciones locales o compartidas (`struct`, `enum`, tipos nativos).
- **Modules**: Módulos lógicos (`module`) que agrupan y publican firmas y tipos.
- **Bindings**: Identificadores locales inmutables asociados a valores (`let`, parámetros).

Esta separación semántica garantiza una resolución determinista e inequívoca sin introducir conceptos orientados a objetos como namespaces de clases o interfaces con despacho dinámico.

#### 11.8.2 Identidad de función local

La identidad de una Function Implementation dentro de un archivo `.efn` está determinada única y exclusivamente por su **nombre**:

$$\text{Function Identity} = \text{function name}$$

No forman parte de la identidad de una función:
- El tipo de resultado (`-> Tipo`).
- La cantidad de argumentos (aridad).
- Los tipos de los argumentos.
- El modificador de visibilidad (`public` o `private`).

Por tanto, cada nombre de función local debe ser estrictamente único dentro del archivo `.efn`.

#### 11.8.3 Prohibición de sobrecarga y unicidad de funciones locales

Evo-Script v0.1 **no admite sobrecarga de funciones** (`function overloading`). Declarar múltiples funciones locales con el mismo nombre dentro del mismo `.efn` es semánticamente inválido y produce un error de sistema `DuplicateFunctionError`.

`DuplicateFunctionError` aplica exclusivamente a colisiones entre múltiples Function Implementations locales dentro del mismo archivo `.efn`. La declaración `import modulo::firma;` introduce una Signature en el espacio de dependencias, **no crea una Function Implementation local adicional**, y por tanto no produce `DuplicateFunctionError` frente a una función local homónima.

Ejemplo inválido por variación de tipos (funciones locales):
```text
private fn calculate(int value) -> int {
    return value;
}

private fn calculate(string value) -> string {
    return value;
}
```
*Inválido*: Ambas funciones locales declaran el mismo nombre `calculate`, violando la unicidad de identidad (`DuplicateFunctionError`).

Ejemplo inválido por variación de aridad (funciones locales):
```text
private fn calculate(int value) -> int {
    return value;
}

private fn calculate(int value, int other) -> int {
    return value + other;
}
```
*Inválido*: Variar la cantidad de parámetros no crea una función distinta (`DuplicateFunctionError`).

Ejemplo inválido por variación de tipo de resultado (funciones locales):
```text
private fn calculate(int value) -> int {
    return value;
}

private fn calculate(int value) -> string {
    return to_string(value);
}
```
*Inválido*: El tipo de retorno no participa en la identidad ni en la resolución (`DuplicateFunctionError`).

#### 11.8.4 Resolución de llamadas a funciones no calificadas

Al encontrar una expresión de llamada `nombre(args...)` dentro de un archivo `.efn`, el sistema resuelve la operación considerando los siguientes espacios:

1. **Function Implementation local**: Se busca una función declarada localmente en el archivo `.efn` cuyo nombre coincida exactamente con `nombre`.
2. **Signature importada**: Se busca una firma importada mediante `import` cuyo nombre local visible coincida con `nombre` (sea su nombre original o su alias local asignado con `as`).

Los posibles resultados de la búsqueda son:

- **Solo existe función local**: Se resuelve como una llamada a la Function Implementation local del archivo.
- **Solo existe firma importada**: Se resuelve como una llamada a la capacidad provista por la Signature importada.
- **No existe ninguna coincidencia**: La llamada no puede resolverse y produce un error de resolución `FunctionNotFoundError`.
- **Existen simultáneamente función local y firma importada con el mismo nombre**: Se produce un conflicto semántico por ambigüedad. Evo-Script v0.1 **no utiliza precedencia implícita** (no existe regla de *"local gana"*, *"import gana"*, *"más cercano gana"* ni orden de declaración). Esta colisión hace el programa inválido y requiere obligatoriamente desambiguación mediante alias explícito en la cláusula `import`.

Una vez resuelto el destino de la llamada:
1. **Validación de aridad**: Se verifica que la cantidad de argumentos provistos coincida exactamente con la aridad de la función o firma resuelta (Sección 11.9).
2. **Validación de tipos**: Se verifica que los tipos de los argumentos coincidan exactamente con los tipos de los parámetros (Sección 11.9).
3. **Evaluación de argumentos**: Se evalúan los argumentos de izquierda a derecha (Sección 11.10).
4. **Ejecución / Delegación**: Se ejecuta el cuerpo de la función local o se delega la ejecución de la capacidad importada, produciendo el resultado tipado correspondiente.

El contexto receptor (por ejemplo, el tipo de un binding `let int x = calculate(...)`) nunca participa en la resolución de la función; la función se resuelve siempre de forma determinista y exclusiva por su nombre.

#### 11.8.5 Desambiguación obligatoria mediante alias ante colisión de llamadas

Cuando un archivo `.efn` contiene una función local y simultáneamente importa una firma con el mismo nombre, la llamada no calificada es ambigua. La situación debe resolverse asignando un alias explícito a la firma importada mediante la cláusula `as`:

Ejemplo inválido por colisión no resuelta:
```text
import values::SearchResult;
import values::search;

private fn search(int id) -> SearchResult {
    return SearchResult::NotFound;
}

public fn execute(int id) -> SearchResult {
    return search(id); // Inválido: llamada ambigua entre Function local y Signature importada
}
```

Ejemplo corregido mediante alias explícito:
```text
import values::SearchResult;
import values::search as external_search;

private fn search(int id) -> SearchResult {
    return SearchResult::NotFound;
}

public fn execute(int id) -> SearchResult {
    let SearchResult local_result = search(id);
    let SearchResult imported_result = external_search(id);

    return imported_result;
}
```

En este diseño:
- `search(id)` invoca de forma inequívoca la Function Implementation local.
- `external_search(id)` invoca de forma inequívoca la Signature importada.
- No existe ambigüedad y la resolución permanece determinista sin reglas mágicas de prioridad.


### 11.9 Validación de aridad y tipos de argumentos

#### 11.9.1 Validación de aridad

La cantidad de argumentos provistos en una llamada debe coincidir exactamente con la cantidad de parámetros declarados en la función:

```text
private fn add(int a, int b) -> int {
    return a + b;
}
```

- `add(10, 20)`: Válido (aridad exacta = 2).
- `add(10)`: Inválido $\rightarrow$ `FunctionArityError`.
- `add(10, 20, 30)`: Inválido $\rightarrow$ `FunctionArityError`.

#### 11.9.2 Validación exacta de tipos

Cada expresión de argumento debe producir exactamente el tipo declarado por el parámetro correspondiente en la firma de la función:

```text
private fn process(int64 value) -> int64 {
    return value;
}
```

Si se dispone de un valor de tipo `int`:
```text
let int value = 10;

process(value); // Inválido -> FunctionArgumentTypeError
```

Evo-Script no realiza conversiones implícitas, promociones ni widening automático entre tipos numéricos ni de ninguna otra clase (`int != int64`). Para invocar la función, la conversión debe ser explícita:

```text
process(to_int64(value)); // Válido
```


### 11.10 Orden de evaluación de argumentos

#### 11.10.1 Evaluación estricta de izquierda a derecha

Los argumentos de una llamada de función se evalúan de forma estrictamente secuencial de izquierda a derecha (`izquierda -> derecha`):

```text
operation(
    first(),
    second(),
    third()
)
```

Orden de evaluación garantizado:
1. Se evalúa la expresión `first()`.
2. Se evalúa la expresión `second()`.
3. Se evalúa la expresión `third()`.
4. Se ejecuta `operation(...)` con los valores producidos.

La implementación técnica no puede reordenar semánticamente la evaluación de los argumentos.

#### 11.10.2 Fallo durante la evaluación de argumentos

Si la evaluación de un argumento produce un error de evaluación (`EvaluationError`, tales como `DivisionByZeroError`, `OverflowError` o `ConversionError`):
1. La ejecución de la función invocada **no comienza**.
2. Los argumentos posteriores aún no evaluados **no se evalúan**.
3. El `EvaluationError` se propaga inmediatamente hacia el exterior según las reglas de propagación de la Sección 10.7.

Ejemplo conceptual:
```text
operation(
    first(),
    failing(),
    third()
)
```
Si la evaluación de `failing()` produce `DivisionByZeroError`:
- La expresión `third()` **no se evalúa**.
- La función `operation` **no llega a ejecutarse**.
- El fallo `DivisionByZeroError` se propaga al contexto llamador.


### 11.11 Forward references y visibilidad interna

#### 11.11.1 Declaración sin dependencia del orden textual (Forward References)

La posición textual de una función dentro de un archivo `.efn` no restringe su disponibilidad. Un archivo `.efn` se analiza y resuelve como una unidad semántica completa:

```text
public fn run(int value) -> int {
    return double(value);
}

private fn double(int value) -> int {
    return value * 2;
}
```

Este código es completamente válido a pesar de que `double` se declara textualmente después de `run`. Todas las funciones declaradas en el archivo son conocidas por el resolvedor antes de iniciar la evaluación. No se exige la regla de "declaración previa al uso" (*declaration before use*) ni se describe esto como hoisting imperativo de código.

#### 11.11.2 Reglas de llamadas entre funciones del mismo archivo

Dentro de un mismo archivo `.efn`, todas las combinaciones de llamadas entre funciones son válidas siempre que no formen ciclos:

- `private -> private`: Una función auxiliar puede invocar a otra función auxiliar local.
- `private -> public`: Una función auxiliar puede invocar a la función pública principal del mismo archivo.
- `public -> private`: La función pública principal puede invocar libremente cualquier función auxiliar local.
- `public -> public`: La función pública puede invocarse a sí misma conceptualmente (sujeto a la prohibición de ciclos de v0.1).

Aclaraciones normativas:
1. `private fn` significa no accesible desde el exterior del archivo `.efn`; no implica que sea inaccesible para la función `public fn` del mismo archivo.
2. `public fn` puede ser llamada internamente por funciones del mismo archivo, pero su modificador `public` no habilita llamadas directas desde otros archivos `.efn` (la comunicación entre archivos requiere obligatoriamente una firma `.esig` registrada en un módulo `.emod`).


### 11.12 Grafo de llamadas y prohibición de recursión

Evo-Script v0.1 **no permite recursión**.

El grafo de llamadas local formado por las funciones de un archivo `.efn` debe ser un **grafo dirigido acíclico** (*Directed Acyclic Graph* o *DAG*).

#### 11.12.1 Prohibición de recursión directa

Una función no puede invocarse directamente a sí misma:

```text
public fn run(int value) -> int {
    return run(value); // Inválido -> FunctionCallCycleError
}
```

```text
private fn calculate(int value) -> int {
    return calculate(value); // Inválido -> FunctionCallCycleError
}
```

#### 11.12.2 Prohibición de recursión indirecta (Ciclos)

No se permiten ciclos de llamadas indirectas entre dos o más funciones, independientemente de la longitud del ciclo:

```text
private fn first(int value) -> int {
    return second(value);
}

private fn second(int value) -> int {
    return first(value); // Inválido -> FunctionCallCycleError
}
```

Igualmente, cadenas como $A \rightarrow B \rightarrow C \rightarrow A$ son semánticamente inválidas.

#### 11.12.3 Detección y FunctionCallCycleError

El grafo de llamadas se valida semánticamente antes de la ejecución. Si se detecta cualquier ciclo directo o indirecto, se produce:

    FunctionCallCycleError

No se denomina `StackOverflowError` porque el programa se rechaza estáticamente/semánticamente antes de ejecutar el ciclo. No se denomina simplemente `RecursionError` para dejar explícito que cubre tanto auto-invocaciones directas como ciclos complejos entre múltiples funciones.


### 11.13 Errores de validación y resolución del sistema

Los errores de resolución y validación local de funciones pertenecen a la categoría conceptual de **errores del sistema** (`SystemError`).

#### 11.13.1 Naturaleza de SystemError

1. **No es un tipo del lenguaje**: `SystemError` no es un tipo de datos en Evo-Script. No existe `enum SystemError`, `result<T, SystemError>` ni `Result<T, E>`.
2. **No son valores**: Los errores del sistema no son valores (`Value`), no poseen variantes, no pueden almacenarse en bindings (`let`), no pueden pasarse como argumentos ni declararse en cláusulas de retorno (`-> Tipo`).
3. **No son capturables**: Evo-Script no posee estructuras de captura o manejo de excepciones (`try`, `catch`, `throw`, `recover`, `otherwise`). Un programa que contiene un error de sistema es rechazado antes de alcanzar la evaluación normal.

#### 11.13.2 Errores de resolución y validación de este bloque

| Error del Sistema | Condición semántica |
| :--- | :--- |
| `FunctionNotFoundError` | Se invoca un nombre de función que no existe en el archivo local ni en las firmas importadas. |
| `FunctionArityError` | La cantidad de argumentos suministrados no coincide con la aridad declarada. |
| `FunctionArgumentTypeError` | El tipo producido por un argumento no coincide exactamente con el tipo del parámetro correspondiente. |
| `DuplicateFunctionError` | Dos funciones declaran el mismo nombre dentro del mismo archivo `.efn`. |
| `FunctionCallCycleError` | Se detecta recursión directa o un ciclo indirecto en el grafo de llamadas. |
| `FieldNotFoundError` | Se intenta acceder a un campo que no existe en la definición del struct receptor. |
| `FieldAccessTypeError` | Se intenta acceder a un campo mediante `.` sobre un receptor cuyo tipo no es struct. |
| `ComparisonTypeError` | Los tipos de los operandos en una comparación no coinciden exactamente o no son compatibles con el operador. |
| `RecursiveTypeCycleError` | Se detecta un ciclo de dependencias estructurales directas o indirectas entre structs y/o enums. |
| `TypeNameCollisionError` | Dos declaraciones de tipos o importaciones intentan registrar el mismo identificador local dentro del Type Space. |
| `LibraryArtifactPathError` | La ruta declarada en un `artifact` es inválida (absoluta, URL, comodín o escapa del Library Base Directory). |
| `LibraryArtifactNotFoundError` | El archivo físico declarado mediante `artifact` en `.elib` no existe en el sistema de archivos. |
| `DuplicateLibraryArtifactError` | Dos declaraciones `artifact` registran la misma ruta física normalizada en la Active Library. |
| `ModuleBoundaryError` | Un artefacto modular no puede asignarse a exactamente una Physical Module Boundary o existen múltiples `.emod` en el mismo directorio. |
| `DuplicateModuleError` | Dos archivos `.emod` registrados en la Active Library declaran la misma Module Identity lógica. |
| `ModuleNotFoundError` | Se referencia un módulo lógico que no existe entre los módulos registrados en la Active Library. |
| `ModuleSymbolNotFoundError` | Se intenta importar o publicar un símbolo que no existe en el módulo correspondiente. |
| `DuplicateModuleSymbolError` | Más de un artefacto registrado dentro de la misma Physical Module Boundary declara el mismo símbolo semántico. |

#### 11.13.3 Distinción formal de categorías de error

Evo-Script distingue formalmente tres categorías ortogonales de fallos o alternativas:

| Categoría | Naturaleza | Momento de detección | Ejemplo | Manejo en Evo-Script |
| :--- | :--- | :--- | :--- | :--- |
| **System / Validation Error** | Invalidez estructural o de resolución del programa | Antes de la evaluación | `FunctionNotFoundError`, `FunctionArityError`, `DuplicateFunctionError`, `FunctionCallCycleError`, `FieldNotFoundError`, `FieldAccessTypeError`, `ComparisonTypeError`, `RecursiveTypeCycleError`, `TypeNameCollisionError`, `LibraryArtifactPathError`, `LibraryArtifactNotFoundError`, `DuplicateLibraryArtifactError`, `ModuleBoundaryError`, `DuplicateModuleError`, `ModuleNotFoundError`, `ModuleSymbolNotFoundError`, `DuplicateModuleSymbolError` | El programa se rechaza; no es evaluable. |
| **Evaluation Error** | Fallo en la evaluación de una expresión en un programa válido | Durante la evaluación | `DivisionByZeroError`, `OverflowError`, `ConversionError` | Detiene la evaluación y se propaga al host/runtime exterior. |
| **Domain Alternative** | Resultado o caso normal esperado del dominio del programa | Durante la evaluación | `BuscarTrabajadorResult::Error(string)`, `SearchResult::NotFound` | Valor normal `Value` de tipo `enum`; inspeccionable con `when`. |


### 11.14 Sintaxis y semántica

Existe una separación estricta entre la representación textual y la semántica del lenguaje:

| Sintaxis | Semántica |
| :--- | :--- |
| `public fn` | `Public principal function in .efn` |
| `private fn` | `File-local helper function in .efn` |
| `esig nombre(args...) -> Tipo;` | `Evo Signature declaration in .esig` |
| `import modulo::simbolo;` | `Granular published symbol import declaration (.efn, .estc, .enum, .esig)` |
| `import modulo::simbolo as alias;` | `Granular published symbol import with local alias` |
| `artifact "ruta/archivo";` | `Physical artifact manifest entry in .elib` |
| `modulo::simbolo` | `Qualified modular symbol reference` |
| `modulo::firma nombre_local` | `Signature Dependency Parameter` |
| `public fn ... -> Tipo : modulo::firma` | `Function Implementation satisfying Signature` |
| `:` | `Signature satisfaction marker / field init` |
| `bind modulo::firma to "ruta/archivo.efn";` | `Functional Composition Binding in .root` |
| `to` | `Binding target delimiter in .root` |
| `entry "ruta/archivo.efn";` | `Application Entry Selection in .main` |
| `module Nombre { ... }` | `Evo Module declaration in .emod` |
| `publish Simbolo;` | `Public modular surface entry in .emod` |
| `tipo nombre` | `Value Parameter` |
| `-> tipo` | `Result type declaration` |
| `return expresion;` | `Explicit function result declaration (Final Return)` |
| `nombre(args...)` | `Function call expression` |
| `nombre()` | `Zero-arity function call expression` |
| `nombre(args...);` | `Operation Statement (Function call with discarded normal Value)` |
| `pipeline;` | `Operation Statement (Pipeline with discarded normal Value)` |
| `expresion.campo` | `Field Access Expression (Projection of struct field)` |
| `{ ... }` | `Function Body / Correspondence` |
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
| `let tipo nombre = valor;` | `Let Binding Statement (Immutable binding)` |
| `=` | `Value association in let` |
| `;` | `End of declaration / Body Statement delimiter` |
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
2. **Proyecto estructurado**: Un conjunto de artefactos especializados con responsabilidades delimitadas (`.root`, `.main`, `.emod`, `.esig`, `.estc`, `.enum`, `.efn`).


### 12.1 Extensiones oficiales y responsabilidades

Cada extensión de archivo en Evo-Script expresa su responsabilidad semántica principal:

| Extensión | Nombre | Responsabilidad semántica principal |
| :--- | :--- | :--- |
| `.efn` | Evo Function | Implementación de función o script autocontenido ejecutable |
| `.esig` | Evo Signature | Contrato público de una función (firma sin cuerpo) |
| `.estc` | Evo Struct | Definición compartible de struct |
| `.enum` | Enum | Definición compartible de enum |
| `.emod` | Evo Module | Módulo, frontera semántica y catálogo de firmas y tipos públicos |
| `.root` | Evo Project Root | Raíz de resolución y Functional Composition Root de un proyecto estructurado |
| `.main` | Evo Application Main | Selección de la operación inicial, Application Main Loop y ciclo de vida de la aplicación |
| `.elib` | Evo Library | Manifiesto de artefactos físicos (Physical Artifact Manifest) y unidad de resolución física del proyecto |
| `.evo` | Evo Package | Extensión reservada para paquetes y artefactos distribuibles (no especificada operativamente en v0.1) |

Regla de exclusión y estado de extensiones:
- La extensión oficial para módulos es estrictamente `.emod` (no se admite `.mod`).
- La extensión `.elib` se encuentra formalmente cerrada y especificada operativamente en Evo-Script v0.1 como manifiesto físico de artefactos (`Physical Artifact Manifest`) y unidad de resolución física.
- La extensión `.evo` queda expresamente **reservada** para especificaciones futuras y no posee semántica operativa en Evo-Script v0.1 (`reserved != usable`, `reserved != partially specified`).
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

1. **Tipo local**: Declarado dentro de un archivo `.efn`. Solo existe en el ámbito léxico de ese archivo. Ningún otro archivo Evo-Script puede referenciarlo ni nombrarlo (`let TipoLocal x = ...` en otro archivo es inválido). Los tipos locales no son publicables por módulos `.emod` ni importables por otros artefactos.
2. **Tipo compartido**: Cuando un tipo de datos necesita participar en comunicaciones entre distintos archivos Evo-Script (por ejemplo, como campo de otro struct en `.estc`, como carga de una variante en `.enum`, o en los argumentos o resultado de una `.esig` o `.efn`), debe extraerse a su propio archivo especializado:
   - Struct compartido $\rightarrow$ archivo `.estc` (contiene una única definición `struct Nombre { ... }`, precedida opcionalmente por cláusulas `import`).
   - Enum compartido $\rightarrow$ archivo `.enum` (contiene una única definición `enum Nombre { ... }`, precedida opcionalmente por cláusulas `import`).
3. **Publicación e importación de tipos compartidos**: Para que un tipo compartido pueda ser consumido por otros artefactos, debe ser publicado por un módulo `.emod` (`publish Nombre;`) e importado explícitamente en el artefacto consumidor mediante `import modulo::Nombre;` (o con alias `import modulo::Nombre as Alias;`).
4. **Principio de frontera**:
   - Tipo utilizado únicamente dentro del mismo `.efn` $\rightarrow$ permanece local.
   - Tipo que cruza una frontera entre archivos Evo-Script $\rightarrow$ debe residir en `.estc` o `.enum`.


### 12.5 Firmas públicas de funciones (.esig)

Un archivo `.esig` (Evo Signature) declara formal y exclusivamente el contrato público de una función mediante la palabra clave `esig`:

```text
esig nombre(tipo argumento1, tipo argumento2) -> Tipo;
```

O cuando la firma requiere dependencias de capacidad funcional:

```text
esig nombre(tipo argumento, modulo::firma dependencia) -> Tipo;
```

Reglas normativas:

1. **Sintaxis oficial de firma**: La declaración consiste exclusivamente en la palabra clave `esig`, el identificador de la firma, la lista de parámetros tipados posicionales, la cláusula `-> Tipo` y el punto y coma final (`;`). Puede estar precedida por cero o más cláusulas `import modulo::Tipo;` o `import modulo::firma;` al inicio del archivo para importar los tipos compartidos o firmas requeridos por sus parámetros o resultado.
2. **Diferenciación estricta entre `esig` y `fn`**: La palabra clave `esig` declara exclusivamente contratos de firma sin cuerpo. La palabra clave `fn` (`public fn` / `private fn`) declara exclusivamente implementaciones de función dentro de `.efn`.
3. **Ausencia de cuerpo y sentencias**: Un archivo `.esig` no posee cuerpo `{ ... }`, correspondencia, bindings locales ni sentencias `return`.
4. **Pública por naturaleza**: Toda firma en un `.esig` es intrínsecamente pública. No admite modificadores de visibilidad (`public esig` y `private esig` son inválidos).
5. **Contrato de acción, no interfaz de objeto**: `.esig` modela directamente la acción requerida, no una interfaz de clase u objeto. Evo-Script no define `interface`, `trait`, `class` ni despacho dinámico `dyn`.
6. **No crea funciones como valores**: `.esig` define un contrato invocable, no un valor de primer orden de tipo función (`Function Value`). No introduce lambdas, clausuras, function pointers como valores ni tipos función manipulables como datos.
7. **Parámetros permitidos en `.esig`**: Una `.esig` puede declarar dos clases de parámetros en su lista posicional:
   - **Parámetros de datos (`Value Parameters`)**: Declarados como `tipo nombre` utilizando tipos nativos (`int`, `float`, `string`, etc.) o tipos compartidos (`.estc`, `.enum`) explícitamente importados mediante `import modulo::Tipo;`.
   - **Parámetros de dependencia de firma (`Signature Dependency Parameters`)**: Declarados como `modulo::firma nombre_local`, donde `modulo::firma` es una Signature pública importada mediante `import modulo::firma;`. No se utiliza la palabra `esig` dentro de la lista de parámetros (`esig modulo::firma dep` es inválido).
   - No puede utilizar tipos locales de un `.efn`.
8. **Desacoplamiento total**: Una `.esig` no contiene ninguna referencia a archivos `.efn` ni a implementaciones concretas.


### 12.6 Comunicación entre archivos y satisfacción de contratos

Evo-Script prohíbe el acoplamiento directo entre implementaciones y organiza la comunicación mediante dependencias granulares de firmas y satisfacción explícita de contratos:

1. **Prohibición de acceso directo `.efn` $\rightarrow$ `.efn`**: Un archivo de implementación `.efn` nunca puede depender ni invocar directamente a otro archivo `.efn`.
2. **Canal de comunicación formal**: La comunicación entre archivos distintos ocurre exclusivamente a través de firmas `.esig` catalogadas en módulos `.emod`:
   ```text
   consumer.efn
        │
        │ importa la firma granular
        ▼
   import module::operation;
        │
        │ ofrecida por
        ▼
   module.emod ──► operation.esig
                        │
                        ├──► input.estc
                        │
                        ├──► result.enum
                        │
                        │ satisfecha por
                        ▼
   public fn operation(...) : module::operation { ... }
        │
   implementation.efn
   ```
3. **Declaración de dependencias mediante `import`**: Para utilizar o satisfacer una firma publicada por un módulo, el archivo debe declararla explícitamente mediante `import`:
   ```text
   import module::signature;
   ```
   Opcionalmente con un alias local para evitar colisiones de nombres en llamadas:
   ```text
   import module::signature as alias_local;
   ```
4. **Posición de las declaraciones `import`**: Las cláusulas `import` son declaraciones estructurales de nivel superior y deben ubicarse estrictamente al inicio del archivo `.efn`, antes de cualquier definición de struct local, enum local, `private fn` o `public fn`. No se permiten declaraciones `import` dentro del cuerpo de una función ni como expresiones evaluables.
5. **Satisfacción explícita mediante `:`**: Una `Function Implementation` en un archivo `.efn` declara formalmente qué firma satisface utilizando el carácter `:` seguido del nombre calificado del módulo y firma:
   ```text
   public fn search(int id) -> SearchResult
       : values::search
   {
       return SearchResult::NotFound;
   }
   ```
   - El carácter `:` en este contexto significa estrictamente *"satisface / implementa la firma"*.
   - No representa herencia, subtipado, tipo de parámetro ni interfaz orientada a objetos.
   - La implementación referencia exclusivamente el nombre calificado de la firma (`: values::search`) y **no duplica** los parámetros ni la sintaxis de la firma después de `:`.
6. **Resolución exclusiva de Signatures tras `:`**: El marcador `:` resuelve identificadores **únicamente en el espacio semántico de Signatures** calificadas publicadas por módulos `.emod`. Nunca puede resolverse como una función local, un binding, un valor ni una variante de enum.
7. **No redundancia entre `import` y `:` en el implementador**:
   - `import values::search;` declara la dependencia estructural y hace conocida la Signature y su cierre transitivo de tipos al archivo.
   - `: values::search` declara que la `public fn` local satisface ese contrato.
   - Ambos son requeridos en la forma canónica del implementador y cumplen funciones ortogonales.
8. **Ausencia de colisión en el implementador**: La Signature importada (`values::search`) y la Function Implementation local (`search`) pertenecen a clases semánticas distintas. La cláusula `import values::search;` no reserva el nombre `search` como una función local, por lo que declarar `public fn search(...) : values::search` es completamente válido y no produce `DuplicateFunctionError`.
9. **Invarianza de la identidad de la Signature frente a alias**: Si un archivo declara `import values::search as imported_search;`, el alias `imported_search` define únicamente el nombre local disponible para llamadas dentro de ese archivo. La identidad formal de la firma continúa siendo `values::search`. Por consiguiente, la satisfacción contractual tras `:` debe referenciar siempre la identidad real (`: values::search`) y nunca el alias (`: imported_search` es inválido).
10. **Comprobación exacta de coincidencia contractual**: La `public fn` que declara satisfacer `: modulo::firma` debe coincidir exactamente con la declaración en `.esig` en:
   - Nombre de la función.
   - Cantidad de parámetros (aridad de Value Parameters y Signature Dependency Parameters).
   - Orden posicional de los parámetros.
   - Tipos exactos de los parámetros (tanto de datos como de dependencias de firma).
   - Tipo exacto de resultado.
   No se permiten conversiones implícitas, promociones ni widening para satisfacer una firma.
11. **Error de discordancia contractual (`SignatureMismatchError`)**: Si una función declara satisfacer `: modulo::firma` pero su firma implementada difiere en nombre, aridad, tipos o retorno respecto de la `.esig` publicada, el programa es inválido y produce un error de validación `SignatureMismatchError` (categoría `SystemError`).
12. **Modalidades de consumo e Inversión de Control Funcional**:
    Evo-Script v0.1 admite dos formas de consumir una capacidad importada:
    - **Consumo directo**: El archivo importa la firma (`import values::search;`) y la invoca directamente (`search(id)`). La capacidad está disponible a nivel de archivo.
    - **Consumo mediante Signature Dependency Parameter**: Una Function Implementation declara formalmente que requiere una capacidad específica en su lista de parámetros (`modulo::firma nombre_local`):
      ```text
      import values::SearchResult;
      import values::search;

      public fn process(int id, values::search search) -> SearchResult {
          return search(id);
      }
      ```
      En este modo, `process` no decide qué implementación concreta ejecutará `values::search`; declara la necesidad funcional del contrato y delega la vinculación a la resolución externa del proyecto. Esto constituye **Inversión de Control Funcional** (`Functional Inversion of Control`).
13. **Distinción fundamental entre requerir y satisfacer una Signature**:
    - `modulo::firma nombre_local` en parámetros $\rightarrow$ La función **REQUIERE** la Signature para operar (`values::search search`).
    - `: modulo::firma` en la declaración $\rightarrow$ La función **SATISFACE** la Signature (`public fn search(...) : values::search`).
    Ambas declaraciones son conceptualmente opuestas y no deben confundirse.
14. **Ausencia de conceptos de IoC orientado a objetos**:
    La inyección de capacidades mediante Signature Dependency Parameters no introduce conceptos de Programación Orientada a Objetos:
    - No existen `new`, constructores, instancias ni objetos de servicio.
    - No existen contenedores IoC con estado ni ciclos de vida (`Singleton`, `Scoped`, `Transient`).
    - La ejecución es puramente funcional: cada invocación de la capacidad ejecuta directamente la Function Implementation vinculada.
15. **Desacoplamiento total del consumidor inyectado**: El archivo `process.efn` conoce exclusivamente la firma `values::search` y sus tipos asociados (`SearchResult`, `Worker`). No conoce el archivo `search.efn`, su ubicación física ni sus funciones privadas.
16. **Ausencia de llamadas calificadas en v0.1**: En Evo-Script v0.1, las llamadas a capacidades importadas o inyectadas se invocan exclusivamente mediante su nombre local no calificado (`search(id)`) o alias asignado (`alias(id)`). No se introduce sintaxis de llamada calificada tipo `values::search(id)` en expresiones evaluables.
17. **Asistencia para herramientas y editores (Nota no normativa)**: La presencia explícita de `import values::search;` al inicio del archivo permite que herramientas de desarrollo, servidores de lenguaje (LSP) y entornos integrados (IDEs) conozcan de antemano el conjunto de firmas disponibles para el archivo, facilitando el autocompletado y la sugerencia de firmas válidas al declarar parámetros de dependencia o escribir `:`.
18. **Composición de dependencias mediante `.root`**: El enlace formal entre una Signature pública requerida (`values::search`) y la Function Implementation concreta que la satisface (`: values::search`) se declara formalmente en el archivo `.root` del proyecto mediante sentencias `bind values::search to "ruta/archivo.efn";` (ver Sección 12.8).
19. **Reenvío estricto de dependencias de firma (`Signature Dependency Forwarding`)**: Una Function Implementation o Signature que declara un `Signature Dependency Parameter` puede transportarlo/reenviarlo como argumento a otra función o firma que declare exactamente la misma dependencia de firma (`modulo::firma`). Este transporte composicional de capacidades no convierte a las Signatures en datos de primer orden. Permanece estrictamente prohibido almacenar dependencias en structs/enums, asignarlas a variables locales con `let` o retornarlas desde funciones.


### 12.7 Módulos (.emod) y selección granular de dependencias

Un archivo `.emod` (Evo Module) define la identidad semántica y la superficie modular pública de un módulo:

#### 12.7.1 Declaración y superficie pública (`module` y `publish`)

La sintaxis oficial de un archivo `.emod` define el nombre lógico del módulo y los símbolos que conforman su superficie pública mediante la palabra clave `publish`:

```text
module values {
    publish Worker;
    publish SearchResult;
    publish search;
}
```

Reglas normativas:
1. **Identidad del módulo**: La cláusula `module Nombre { ... }` establece el identificador lógico del módulo utilizado en referencias calificadas (`values::search`, `values::Worker`).
2. **Declaración `publish`**: Cada cláusula `publish Simbolo;` incluye explícitamente un artefacto en la superficie pública del módulo. `publish` es una declaración estructural modular; no ejecuta operaciones ni es una llamada de función.
3. **Artefactos publicables**: Un `.emod` puede publicar exclusivamente:
   - Firmas de funciones (`.esig`).
   - Structs compartidos (`.estc`).
   - Enums compartidos (`.enum`).
4. **Prohibición estricta de publicar `.efn`**: Un `.emod` **nunca publica archivos `.efn`**, funciones privadas ni tipos locales a una implementación. Las implementaciones permanecen completamente ocultas detrás de los contratos públicos.
5. **Ausencia de `namespace`**: Evo-Script no introduce la palabra clave `namespace`. La pertenencia modular se expresa mediante nombres calificados con `::`.

#### 12.7.2 Importación universal de símbolos publicados (`import` y `as`)

Evo-Script v0.1 define una **sintaxis única y universal** para la importación explícita de cualquier símbolo publicado por un módulo:

```text
import modulo::simbolo;
```

o con alias local explícito:

```text
import modulo::simbolo as nombre_local;
```

Reglas normativas:

1. **Sintaxis uniforme para firmas y tipos compartidos**: La misma construcción textual `import modulo::simbolo;` se utiliza para importar cualquier categoría de símbolo publicado:
   - Firmas de funciones (`.esig`): `import workers::search;`
   - Structs compartidos (`.estc`): `import workers::Worker;`
   - Enums compartidos (`.enum`): `import workers::SearchResult;`
2. **Prohibición de palabras clave diferenciadas de importación**: No existen palabras clave ni formas alternativas como `import type`, `import struct` ni `import enum`. El parser reconoce la forma única `import modulo::simbolo;`, y la categoría semántica del símbolo importado determina su espacio de nombres local.
3. **Condición de publicación previa**: Un símbolo solo puede importarse si ha sido explícitamente incluido en la cláusula `publish` del módulo `.emod` correspondiente. Intentar importar un símbolo no publicado o privado es un error de análisis estático.
4. **Importación estrictamente granular**: Toda declaración `import` importa exactamente un símbolo calificado. Quedan prohibidos los comodines (`import modulo::*;`, `import * from modulo;`) y la apertura implícita de todo el espacio del módulo (`import modulo;`).
5. **Declaración estructural top-level**: Las cláusulas `import` son declaraciones estructurales de nivel superior. Deben ubicarse estrictamente al inicio de los archivos que las admiten (`.efn`, `.estc`, `.enum`, `.esig`), antes de cualquier declaración que utilice los nombres locales introducidos. No se permiten declaraciones `import` anidadas dentro de funciones, structs, ramas de `when` ni expresiones evaluables.
6. **Naturaleza estática no ejecutable**: `import` no es una sentencia ejecutable (`Operation Statement` o `Body Statement`), no ejecuta código en runtime ni produce valores. Se procesa exclusivamente durante el análisis semántico de dependencias.
7. **Separación estricta entre `import` y `use`**:
   - `import`: declara dependencias estructurales y semánticas de símbolos publicados en tiempo de análisis.
   - `use`: activa un `Scope` de capacidades en tiempo de ejecución.
   No se utiliza `use` para importar tipos o firmas ni `import` para activar Scopes.


#### 12.7.3 Separación de espacios semánticos (`Signature Space` y `Type Space`)

Evo-Script organiza los símbolos locales disponibles en un archivo en espacios semánticos formalmente diferenciados:

1. **Registro en Semantic Spaces según la categoría del símbolo**:
   - **`Signature Space`**: Al importar una firma (`import workers::search;`), el símbolo entra al `Signature Space` local. Se utiliza para llamadas a capacidades funcionales directas o como tipo formal en `Signature Dependency Parameters` (`workers::search search`).
   - **`Type Space`**: Al importar un struct (`import workers::Worker;`) o un enum (`import workers::SearchResult;`), el símbolo entra al `Type Space` local. Se utiliza en todas las posiciones sintácticas donde se espera un nombre de tipo.
2. **Unificación en `Type Space`**: Dentro de un mismo archivo, el `Type Space` aloja tanto los tipos definidos localmente (structs y enums locales en `.efn`) como los tipos compartidos importados.
3. **Uso no calificado del nombre local de tipo**: Una vez importado un tipo en `Type Space` (por ejemplo `import workers::Worker;`), su uso local se realiza **exclusivamente mediante su nombre local no calificado** (`Worker`) o su alias (`Employee`).
4. **Prohibición de uso calificado de tipos**: No se admite el uso de nombres calificados en posiciones ordinarias de tipos. Escribir `let workers::Worker w = ...;`, `workers::Worker { ... }` o `public fn f(workers::Worker w) -> ...` es sintácticamente inválido. La forma obligatoria y canónica es importar previamente el tipo (`import workers::Worker;`) y utilizar su nombre local simple (`Worker`).
   - *Excepción normativa de diseño*: En `Signature Dependency Parameters`, la identidad formal de la firma se escribe calificada (`workers::search search`) para distinguir formalmente el contrato requerido del identificador local del parámetro.


#### 12.7.4 Uso de tipos compartidos importados en artefactos (`.efn`, `.estc`, `.enum`, `.esig`)

Los tipos compartidos importados en `Type Space` pueden utilizarse en cualquier artefacto que admita declaraciones `import`:

1. **En implementaciones de funciones (`.efn`)**:
   - Tipo de parámetros: `public fn save(Worker worker) -> SaveResult { ... }`
   - Tipo de resultado: `public fn current_worker() -> Worker { ... }`
   - Declaraciones de binding `let`: `let Worker worker = ...;`
   - Expresiones de construcción de struct: `Worker { id: 10, name: "Juan" }`
   - Referencias y variantes de enum: `SearchResult::Found(worker)`, `SearchResult::NotFound`
   - Ramas de selección exhaustiva en `when`: `when (result) { SearchResult::Found(w) => w.name, ... }`
2. **En definiciones de structs compartidos (`.estc`)**:
   - Un archivo `.estc` puede declarar cero o más cláusulas `import` al inicio para importar otros tipos compartidos requeridos por sus campos:
     ```text
     import geography::Pais;

     struct Estado {
         int id;
         string name;
         Pais pais;
     }
     ```
   - El archivo sigue definiendo exactamente una única estructura compartida. Las cláusulas `import` no constituyen definiciones adicionales de tipos.
3. **En definiciones de enums compartidos (`.enum`)**:
   - Un archivo `.enum` puede declarar cero o más cláusulas `import` al inicio para importar los tipos de datos requeridos por sus variantes:
     ```text
     import workers::Worker;

     enum SearchResult {
         Found(Worker)
         NotFound
     }
     ```
   - Las variantes estructuradas también pueden utilizar tipos importados:
     ```text
     import workers::Worker;

     enum SearchResult {
         NotFound
         Found {
             Worker worker;
         }
     }
     ```
   - El archivo sigue definiendo exactamente un único enum compartido.
4. **En firmas públicas de funciones (`.esig`)**:
   - Un archivo `.esig` puede declarar cero o más cláusulas `import` al inicio para importar los tipos compartidos requeridos por sus parámetros o su retorno:
     ```text
     import workers::SearchResult;

     esig search(int id) -> SearchResult;
     ```
   - El archivo sigue conteniendo exactamente una única declaración de firma (`esig`).
5. **Artefactos que no admiten `import`**:
   - `.main`: Contiene exclusivamente la sentencia de selección de entrada `entry`.
   - `.root`: Contiene exclusivamente sentencias de vinculación `bind ... to ...`.
   - `.emod`: Contiene exclusivamente sentencias `publish`.


#### 12.7.5 Cierre de dependencias de tipos (`Type Dependency Closure`) y de firmas (`Signature Type Closure`)

Evo-Script define formalmente la distinción entre **resolución semántica transitiva** e **importación local en el espacio de nombres**:

1. **Cierre de dependencias de tipo (`Type Dependency Closure`)**:
   - Para cualquier tipo compartido $T$, su `Type Dependency Closure(T)` es el conjunto transitivo de todos los tipos de usuario necesarios para conocer de forma completa su estructura de campos y variantes.
   - Si `Worker` compone `Address`, y `Address` compone `Country`, entonces $\text{Type Dependency Closure}(\text{Worker}) = \{\text{Worker}, \text{Address}, \text{Country}\}$.
2. **Resolución transitiva no implica importación local**:
   - Al declarar `import workers::Worker;`, el compilador/analizador resuelve transitivamente todo el cierre para validar tipos, tamaños y accesos a campos anidados.
   - Sin embargo, **solo `Worker` ingresa al `Type Space` local** del archivo consumidor. Los tipos transitivos (`Address`, `Country`) **no se importan automáticamente** ni están disponibles como nombres locales.
3. **Acceso a campos anidados sin importación local**:
   - Gracias a la resolución semántica del cierre, una expresión de acceso a campos anidados es completamente válida sin requerir importar los tipos intermedios:
     ```text
     import workers::Worker;

     public fn city_of(Worker worker) -> string {
         return worker.address.city; // Válido: el tipo de address se conoce transitivamente
     }
     ```
4. **Nombrar o construir directamente tipos transitivos exige `import` explícito**:
   - Si un archivo desea nombrar directamente `Address` en un binding `let` o construirlo directamente mediante `Address { ... }`, debe importar explícitamente dicho tipo:
     ```text
     import workers::Worker;
     import workers::Address; // Requerido para nombrar 'Address' localmente

     let Address addr = Address { city: "Monterrey" };
     ```
5. **Cierre de tipos en firmas (`Signature Type Closure`)**:
   - Al importar una firma (`import workers::search;`), el compilador resuelve el cierre de tipos requerido por su contrato (`SearchResult`, `Worker`, etc.).
   - No obstante, la declaración `import workers::search;` **no introduce `SearchResult` ni `Worker` en el `Type Space` local**. Si el archivo consumidor utiliza `SearchResult` como tipo de retorno o en bindings `let`, debe declarar adicionalmente `import workers::SearchResult;`.


#### 12.7.6 Cierre público de tipos (`Public Type Closure`)

Todo módulo que exponga símbolos públicos debe garantizar la integridad de su superficie:

1. **Resolubilidad del cierre público**: Si un módulo publica una firma o un tipo compartido, todos los tipos de usuario que formen parte de su `Type Dependency Closure` o `Signature Type Closure` deben ser semánticamente accesibles y resolubles a través de interfaces publicadas.
2. **Prohibición de contratos incompletos o inaccesibles**: Un módulo no puede publicar una firma o un tipo que dependa de un tipo local no publicado o inaccesible externamente.
3. **Cierre a través de módulos lógicos**: El cierre de tipos puede componer tipos compartidos procedentes de distintos módulos lógicos registrados en la Active Library (`.elib`), resolviéndose deterministamente a través de sus respectivas Physical Module Boundaries (véase la Sección 12.10).


#### 12.7.7 Aliases de tipos con `as`, convenciones y colisiones (`TypeNameCollisionError`)

1. **Uso de alias local con `as`**: La cláusula `as` permite asignar un nombre local alternativo a un tipo importado:
   ```text
   import hr::Worker as HrWorker;
   import sales::Worker as SalesWorker;
   ```
2. **Invarianza de la identidad formal del tipo**:
   - El alias define únicamente el identificador local visible en el `Type Space` de ese archivo.
   - La **identidad formal y semántica** del tipo sigue siendo `hr::Worker` o `sales::Worker`.
   - El alias no crea un tipo nuevo, no es un `typedef`, no introduce un subtipo ni crea un wrapper. La igualdad estructural (`==`, `!=`) y la compatibilidad de tipos operan exclusivamente sobre la identidad semántica real.
3. **Convenciones de nombres para aliases**:
   - Todo alias asignado a un tipo (`struct` o `enum`) ingresa a `Type Space` y debe nombrarse obligatoriamente en **`PascalCase`** (`HrWorker`, `SalesWorker`, `WorkerResult`).
   - Todo alias asignado a una `Signature` ingresa a `Signature Space` y debe nombrarse obligatoriamente en **`snake_case`** (`search_employee`).
4. **Error del Sistema: `TypeNameCollisionError`**:
   - Si dos declaraciones intentan registrar el mismo identificador dentro del `Type Space` del mismo archivo, el programa es inválido y produce **`TypeNameCollisionError`**:
     - Dos importaciones de tipos que comparten el mismo nombre local:
       ```text
       import hr::Worker;
       import sales::Worker; // Error: TypeNameCollisionError (Worker ya existe en Type Space)
       ```
     - Un tipo local y un tipo importado con el mismo nombre:
       ```text
       import hr::Worker;
       struct Worker { ... } // Error: TypeNameCollisionError
       ```
     - Dos aliases de tipos que asignan el mismo identificador:
       ```text
       import hr::Worker as Employee;
       import sales::Worker as Employee; // Error: TypeNameCollisionError
       ```
     - Importación duplicada exacta del mismo tipo:
       ```text
       import hr::Worker;
       import hr::Worker; // Error: TypeNameCollisionError
       ```
     - Un struct y un enum que comparten el mismo nombre local dentro del mismo archivo.
   - `TypeNameCollisionError` pertenece a la categoría de **errores de validación del sistema** (`SystemError`); se detecta durante el análisis estático antes de la evaluación normal en runtime.


#### 12.7.8 Preservación del Type Dependency Graph y detección de ciclos intermodulares (`RecursiveTypeCycleError`)

1. **Importación no crea dependencia estructural automática**: Una cláusula `import modulo::Tipo;` hace disponible el símbolo en `Type Space`, pero **no genera una arista en el Type Dependency Graph** a menos que el tipo importado se utilice estructuralmente como campo de un struct o carga de un enum.
2. **Inclusión de tipos compartidos en el Grafo Global de Tipos**: Cuando un struct o enum utiliza un tipo importado en su composición, dicha arista dirigida participa plenamente en el `Type Dependency Graph` global del proyecto.
3. **Detección de ciclos a través de fronteras de módulos**: La exigencia de que el grafo de dependencias de tipos sea un **DAG** aplica universalmente a todo el proyecto. Un ciclo estructural no se evade distribuyendo los tipos en distintos archivos o módulos lógicos:
   ```text
   // En modulo_a / Estado.estc
   import modulo_b::Pais;
   struct Estado { Pais pais; } // Arista Estado -> Pais

   // En modulo_b / Pais.estc
   import modulo_a::Estado;
   struct Pais { Estado estado; } // Arista Pais -> Estado
   ```
   El grafo global contiene el ciclo $Estado \to Pais \to Estado$. El proyecto se rechaza en validación estática con **`RecursiveTypeCycleError`**.
4. **Invalidez de aliases para eludir ciclos**: Los aliases no alteran la identidad formal de los tipos, por lo que su uso no oculta ni evita la detección de un ciclo estructural.


#### 12.7.9 Calificación mediante `::` y delimitación del alcance

El operador `::` denota calificación y pertenencia lógica de un símbolo dentro de un contexto nombrado:

- `modulo::simbolo` (por ejemplo, `values::search`, `values::Worker` en declaraciones `import`, `publish` o `Signature Dependency Parameters`).
- `TipoEnum::Variante` (por ejemplo, `SearchResult::Found`, `SearchResult::NotFound`).


### 12.8 Raíz de proyecto (.root) y Functional Composition Root

Un archivo `.root` (Evo Project Root) cumple dos responsabilidades arquitectónicas fundamentales en un proyecto estructurado:

1. **Raíz estructural del proyecto**: Establece el límite superior de resolución semántica y física del árbol de artefactos del proyecto.
2. **Functional Composition Root**: Actúa como raíz de composición funcional, seleccionando mediante declaraciones `bind` qué Function Implementation concreta (`.efn`) satisface cada Evo Signature pública (`.esig`) requerida por el proyecto.

#### 12.8.1 Naturaleza del Functional Composition Root

- **Composición funcional de contratos e implementaciones**: `.root` vincula identidades lógicas de contratos con archivos de implementación física:
  ```text
  Signature (Evo Signature) ──► Function Implementation (.efn)
  ```
- **No es un contenedor IoC orientado a objetos**: `.root` no es un Object Container, Service Container ni DI Container orientado a objetos. No administra clases, objetos, constructores, métodos virtuales ni ciclos de vida de instancias (`Singleton`, `Scoped`, `Transient`).
- **Naturaleza puramente estructural y no ejecutable**: Un archivo `.root` no contiene lógica de negocio, no implementa funciones, no declara firmas, no define structs ni enums, y no publica módulos. Es una declaración estructural procesada antes de la evaluación; no produce valores (`Value`), no puede importarse ni invocarse.

#### 12.8.2 Sintaxis oficial de vinculación (`bind ... to ...`)

La sintaxis oficial de una declaración de vinculación en un archivo `.root` es:

```text
bind modulo::firma to "ruta/relativa/archivo.efn";
```

Ejemplo canónico:

```text
bind values::search to "providers/search_database.efn";
```

Interpretación semántica:
- `bind`: Palabra clave declarativa estructural de `.root`. No es una expresión, asignación ni sentencia evaluable en `.efn`.
- `values::search`: Identificador lógico calificado de la Signature pública requerida, publicada por un `.emod`.
- `to`: Delimitador estructural exclusivo de la gramática de `.root` que asocia la Signature con el archivo de implementación. No es un operador de expresiones generales.
- `"providers/search_database.efn"`: Literal de texto que especifica la ruta relativa al archivo `.efn` seleccionado como implementación.

#### 12.8.3 Reglas normativas de `.root`

1. **Resolución física relativa al directorio de `.root`**: La ruta textual indicada en `to "..."` se interpreta siempre de forma relativa al directorio que contiene el archivo `.root`. Además, en un proyecto estructurado activo, el archivo `.efn` objetivo debe pertenecer obligatoriamente al `Physical Artifact Universe` definido por el `.elib` activo. No se admiten URLs, rutas absolutas remotas, comodines (`*`), registros de paquetes ni variables de entorno en v0.1.
2. **Ausencia de bloque envolvente**: El archivo `.root` no requiere un bloque envolvente `root nombre { ... }`. Las declaraciones `bind` se escriben directamente a nivel de archivo.
3. **Múltiples bindings independientes**: Un archivo `.root` puede contener tantas declaraciones `bind` independientes como requiera el proyecto:
   ```text
   bind values::search to "providers/search_database.efn";
   bind filesystem::read to "providers/read_std.efn";
   bind terminal::write to "providers/write_terminal.efn";
   ```
4. **Exactamente una implementación seleccionada por Signature**: Dentro de un mismo archivo `.root`, una Signature calificada puede tener exactamente una declaración `bind`. Múltiples bindings para la misma Signature producen un error de validación del sistema:
   ```text
   // Inválido: colisión de múltiples bindings para la misma Signature en un .root
   bind values::search to "providers/search_database.efn";
   bind values::search to "providers/search_memory.efn";
   ```
   No existe selección automática ni prioridad implícita.
5. **Coexistencia de múltiples implementaciones físicas**: En el sistema de archivos pueden coexistir múltiples archivos `.efn` que satisfagan la misma Signature (`: values::search`):
   ```text
   providers/
   ├── search_database.efn
   ├── search_memory.efn
   └── search_remote.efn
   ```
   Cada archivo `.efn` declara formalmente `: values::search`. El archivo `.root` selecciona exactamente cuál de ellos se activa para la composición concreta del proyecto.
6. **Sustitución de implementaciones sin alterar consumidores**:
   - Cambiar la implementación activa en el proyecto (por ejemplo, para producción, pruebas o entornos alternativos) se realiza modificando exclusivamente la sentencia `bind` en `.root`:
     - Composición A (Base de datos):
       ```text
       bind values::search to "providers/search_database.efn";
       ```
     - Composición B (Memoria / Pruebas):
       ```text
       bind values::search to "providers/search_memory.efn";
       ```
     - Composición C (Remota):
       ```text
       bind values::search to "providers/search_remote.efn";
       ```
   - Los archivos consumidores permanecen completamente intactos e invariantes:
     ```text
     import values::search;

     public fn process(int id, values::search search) -> SearchResult {
         return search(id);
     }
     ```
7. **Desacoplamiento total del consumidor**:
   - Una Function Implementation consumidora (`process.efn` o llamadas directas) conoce exclusivamente la Signature (`values::search`).
   - Desconoce por completo qué archivo `.efn` la satisface, su ubicación física y sus funciones privadas auxiliares.
8. **Requisito de satisfacción explícita en el implementador**:
   - El archivo `.efn` objetivo debe declarar explícitamente la cláusula contractual `: modulo::firma` (por ejemplo, `public fn search(...) : values::search`).
   - `.root` no puede enlazar arbitrariamente un `.efn` que no declare satisfacer dicha Signature.
9. **Condiciones de validación del binding**: Una declaración `bind modulo::firma to "ruta.efn";` es válida si y solo si:
   - `modulo::firma` existe como Signature pública publicada por un `.emod` accesible.
   - El archivo objetivo `.efn` existe en la ruta relativa especificada.
   - El archivo `.efn` resuelto pertenece al `Physical Artifact Universe` de la Active Library (`.elib` activo).
   - El archivo `.efn` posee su única función pública (`public fn`).
   - Dicha `public fn` declara explícitamente `: modulo::firma`.
   - La `public fn` coincide exactamente en nombre, aridad, orden y tipos de parámetros, y tipo de resultado con la declaración `.esig`. Si difiere, se produce `SignatureMismatchError`.
   - No existe otro binding para la misma Signature en ese archivo `.root`.
10. **Validación estática previa a la evaluación**: Todos los bindings declarados en `.root` se validan y resuelven estáticamente antes de iniciar la evaluación del programa. Un proyecto con bindings inválidos es rechazado íntegramente; no se descubren errores de composición durante la evaluación de llamadas.
11. **Ejecución puramente funcional**: Cuando se invoca una capacidad vinculada (sea mediante llamada directa `search(id)` o parámetro de dependencia `search(id)`), el runtime despacha la ejecución directamente a la `public fn` del `.efn` seleccionado por `.root`. Cada invocación constituye una ejecución funcional normal que evalúa argumentos y produce un resultado tipado (`Value`).
12. **Alcance uniforme de composición en el proyecto**: Una declaración `bind` en `.root` aplica uniformemente a todas las necesidades de esa Signature en todo el proyecto, tanto para llamadas directas como para Signature Dependency Parameters. No existen bindings locales por función en v0.1.
13. **Prohibición de `bind` en otros artefactos**: La declaración `bind` no puede utilizarse dentro de archivos `.efn` ni `.emod`.
14. **Independencia de `.esig` y `.efn` frente a `.root`**: Las firmas `.esig` y las implementaciones `.efn` no contienen referencias a `.root` ni a rutas de composición. La relación es estrictamente unidireccional: `.root` referencia a `.esig` y a `.efn`.
15. **Prohibición de `publish` en `.root`**: `.root` no publica símbolos; la superficie modular pública pertenece exclusivamente a `.emod`.
16. **Unicidad de `.root` en proyectos estructurados**: Todo proyecto estructurado posee exactamente un archivo `.root`. No se admiten raíces anidadas, encadenadas ni herencia de raíces (`include root`, `extends root` son inválidos).
17. **Innecesario en scripts autocontenidos**: Un script simple `.efn` sin dependencias externas se ejecuta directamente sin necesidad de `.root`.
18. **Inaplicabilidad a structs y enums**: `bind` aplica exclusivamente a Evo Signatures. Declarar `bind modulo::MiStruct to ...` o `bind modulo::MiEnum to ...` es inválido.
19. **Destino exclusivo a archivos `.efn`**: El lado derecho de `bind` referencia siempre la ruta al archivo `.efn`, nunca una función calificada (`"archivo.efn::funcion"` es inválido).
20. **Orden textual irrelevante**: El orden físico de las sentencias `bind` en `.root` no tiene relevancia semántica ni define prioridades (no existe regla de *"primer bind gana"* ni *"último bind gana"*).
21. **Ausencia de condicionales en `.root` en v0.1**: No existen cláusulas condicionales (`when`, `if`, `platform`, `fallback`). Cada binding es incondicional dentro del archivo `.root`.
22. **Separación entre composición (.root) y punto de entrada (.main)**: `.root` define la composición de dependencias de todo el proyecto, mientras que `.main` identifica la operación inicial que arranca una aplicación ejecutable.


### 12.9 Aplicación ejecutable (.main), Application Main Loop y ciclo de vida

Un archivo `.main` (Evo Application Main) define el punto de inicio y la semántica de ciclo de vida (`Application Lifetime`) de una aplicación Evo-Script estructurada:

#### 12.9.1 Responsabilidad oficial y Application Main Loop

1. **Ciclo de vida de la aplicación (`Application Lifetime`)**: El archivo `.main` posee conceptualmente el **Application Main Loop** de una aplicación estructurada.
2. **Equivalencia fundamental**:
   - Mientras el **Application Main Loop** permanece activo, la aplicación está viva.
   - Cuando el **Application Main Loop** termina, la aplicación termina.
   ```text
   Main Loop lifetime = Application lifetime
   ```
3. **Loop administrado por el runtime**: El Application Main Loop es una responsabilidad estructural del runtime de Evo-Script. **No es un constructo de código escrito por el desarrollador**.
4. **Ausencia de loops imperativos en el lenguaje**: El Application Main Loop **no introduce** al lenguaje palabras clave como `while`, `loop`, `for`, `break` ni `continue`. El desarrollador nunca escribe un bucle para mantener viva una aplicación.
5. **No es recursión ni ciclo de llamadas**: El Application Main Loop no constituye recursión de funciones (se preserva estrictamente la regla de "sin recursión en v0.1") ni forma parte del grafo de llamadas ordinario de funciones, por lo que no genera `FunctionCallCycleError`.
6. **No es un valor ni constructo de primer orden**: El Application Main Loop no es un tipo de datos (`Value`), no posee variantes, no se almacena en variables (`let`), no se retorna ni se pasa como argumento. No existen `MainLoop Value`, `LoopHandle` ni `ApplicationHandle`.
7. **Ausencia de estado u objetos de aplicación OO**: No existen clases de aplicación (`MainApplication`, `App`), constructores, instancias (`new`), ni ciclos de vida de servicios orientados a objetos (`Singleton`, `Scoped`, `Transient`).

#### 12.9.2 Secuencia de arranque y relación entre `.root` y `.main`

Existe una estricta separación de responsabilidades entre `.root` y `.main`:
- **`.root`**: Define **CÓMO** se compone la aplicación (selecciona qué Function Implementation satisface cada Signature mediante declaraciones `bind`).
- **`.main`**: Define **CÓMO** arranca la aplicación y **CUÁNTO TIEMPO** permanece viva (selecciona la Function Implementation inicial mediante `entry` y posee el Application Main Loop).

Flujo conceptual de arranque, ejecución y ciclo de vida de una aplicación estructurada:

```text
    structured project
            ↓
       resolve .root
            ↓
validate composition bindings
            ↓
       resolve .main
            ↓
    validate entry target
            ↓
start Application Main Loop
            ↓
invoke initial public fn / evaluations
            ↓
    application alive
            │
            ├─► normal evaluation ──────► produces Value (evaluation completed)
            │
            ├─► failing evaluation ─────► EvaluationError propagates outward to Evo Runtime
            │                             (Application Main Loop remains active)
            │
            ▼
Application Exit Request (e.g. Super + Q, UI action, etc.)
            ↓
 Application Main Loop terminates
            ↓
   application terminates
```

Reglas normativas del proceso de arranque:
1. **Validación de `.root` previa a `.main`**: Antes de iniciar el Application Main Loop, el archivo `.root` debe ser cargado, resuelto y validado en su totalidad. Si existe cualquier error de composición (firmas no resueltas, implementaciones faltantes, discordancias `SignatureMismatchError` o bindings duplicados), el proyecto es rechazado y el Application Main Loop **nunca se inicia**.
2. **`.root` no controla el ciclo de vida**: El archivo `.root` finaliza su responsabilidad cuando la composición queda validada; no inicia ni detiene el Application Main Loop.
3. **`.main` no realiza composición**: El archivo `.main` no contiene declaraciones `bind` ni sustituye la composición de `.root`.
4. **Validación de `entry` previa al Main Loop**: La declaración `entry` de `.main` se valida completamente antes de iniciar el Application Main Loop. Si el archivo destino no existe, no es un `.efn`, carece de `public fn`, declara Value Parameters, o sus dependencias de firma no pueden resolverse mediante `.root`, el Application Main Loop **nunca se inicia**.

#### 12.9.3 Sintaxis oficial de selección de entrada (`entry ...`)

La sintaxis oficial y normativa para declarar la selección de la operación inicial en un archivo `.main` es:

```text
entry "ruta/relativa/archivo.efn";
```

Ejemplo canónico:

```text
entry "functions/application.efn";
```

Reglas normativas de `entry`:

1. **Declaración estructural**: `entry` es una declaración estructural de alto nivel exclusiva de archivos `.main`. No es una Function Call, no es una expresión evaluable, no es un runtime statement, no es un `Value` ni una asignación de variable.
2. **Unicidad de `entry` en `.main`**: Todo archivo `.main` debe contener **exactamente una** declaración `entry "..."`. Declarar múltiples `entry` en el mismo `.main` es inválido y produce un error estático de validación. No existe prioridad ni reglas de *"primer entry gana"* ni *"último entry gana"*.
   - **Ejemplo válido**:
     ```text
     entry "functions/application.efn";
     ```
   - **Ejemplo inválido**:
     ```text
     entry "functions/application.efn";
     entry "functions/other.efn"; // Inválido: múltiples declaraciones entry
     ```
3. **Destino exclusivo a archivos `.efn`**: El literal de texto de `entry` debe apuntar única y exclusivamente a un archivo con extensión `.efn`. Apuntar a `.esig`, `.emod`, `.root`, `.elib` u otras extensiones es inválido:
   - **Válido**: `entry "functions/application.efn";`
   - **Inválido**: `entry "values/search.esig";`, `entry "values/values.emod";`, `entry "application.root";`, `entry "library.elib";`
4. **Destino a archivo, no a función calificada**: Dado que todo archivo `.efn` posee exactamente una `public fn`, `entry` referencia la ruta al archivo `.efn`, nunca un nombre de función calificado. La única `public fn` del archivo seleccionado es la operación inicial que ejecuta el runtime.
   - **Válido**: `entry "functions/application.efn";`
   - **Inválido**: `entry "functions/application.efn"::initialize;`, `entry application::initialize;`, `entry initialize;`
5. **Libertad en el nombre de la función inicial**: La `public fn` principal del `.efn` seleccionado no está obligada a llamarse literalmente `main`. Puede nombrarse libremente según las reglas de Function Identity (por ejemplo, `initialize`, `start_application`, `boot`, `execute`):
   ```text
   public fn initialize() -> InitResult {
       return InitResult::Ready;
   }
   ```
6. **Resolución de ruta relativa a Project Root**: La ruta textual indicada en `entry "..."` se resuelve de forma relativa al directorio que contiene el archivo `.root` del proyecto estructurado (Project Root). Además, en un proyecto estructurado activo, el archivo `.efn` de entrada debe pertenecer obligatoriamente al `Physical Artifact Universe` definido por el `.elib` activo.
7. **Ausencia de bloque envolvente**: El archivo `.main` no requiere ni admite bloques envolventes (`main application { ... }` ni `application { ... }` son inválidos). La sentencia `entry` se declara directamente a nivel de archivo.
8. **Ausencia de lógica y otras construcciones en `.main`**: El archivo `.main` no contiene funciones (`public fn`, `private fn`), variables (`let`), sentencias de control (`return`, `when`), llamadas a funciones, ni declaraciones `bind`, `import`, `esig`, `struct`, `enum` o `publish`.
9. **Parámetros permitidos en Application Entry (Restricción de Value Parameters en v0.1)**:
   - `.root` proporciona resolución estructural exclusivamente para **Signature Dependency Parameters** (`modulo::firma nombre_local`) mediante sentencias `bind`.
   - `.root` **NO** proporciona ni suministra **Value Parameters** (`int width`, `string title`, `Worker worker`).
   - En funciones normales del lenguaje, los Value Parameters son proporcionados por el código que realiza la invocación (`resize(800)`).
   - En el Application Entry (`entry "..."`), la función inicial es invocada directamente por el runtime al arrancar la aplicación. Dado que Evo-Script v0.1 no define mecanismos para suministrar argumentos de datos a la entrada de la aplicación (no existen argumentos CLI / `argv`, valores por defecto en parámetros, valores implícitos ni inyección mágica desde el host), la `public fn` seleccionada como Application Entry **debe declarar exactamente CERO Value Parameters**.
   - La `public fn` seleccionada como Application Entry puede declarar **cero o más Signature Dependency Parameters**, los cuales son resueltos a través de la composición de `.root`.
   - **Forma permitida para Application Entry**:
     ```text
     Entry Parameters :=
         cero Signature Dependency Parameters
         |
         uno o más Signature Dependency Parameters
     ```
   - **Ejemplo válido sin parámetros**:
     ```text
     public fn initialize() -> InitResult {
         return InitResult::Ready;
     }
     ```
   - **Ejemplo válido con Signature Dependency Parameters**:
     ```text
     import window::open;
     import config::load;

     public fn initialize(window::open open_window, config::load load_config) -> InitResult {
         load_config();
         open_window();
         return InitResult::Ready;
     }
     ```
   - **Ejemplo inválido como Application Entry**:
     ```text
     import window::open;

     public fn initialize(int width, window::open open_window) -> InitResult {
         ...
     }
     ```
     Si este `.efn` es seleccionado mediante `entry "functions/application.efn";`, el proyecto es inválido estáticamente y es rechazado antes de iniciar el Application Main Loop porque nadie suministra el valor para `width`.
   - **Alcance restringido a Application Entry**: Esta restricción aplica exclusivamente a la función seleccionada como Application Entry en `.main`. Las funciones normales de Evo-Script continúan pudiendo declarar Value Parameters con total libertad:
     ```text
     public fn resize(int width) -> ResizeResult { ... } // Válido como función normal
     public fn process(int id, values::search search) -> SearchResult { ... } // Válido como función normal
     ```
     Si `process.efn` fuera seleccionado mediante `entry "functions/process.efn";`, sería inválido como Application Entry debido a `int id` (no debido a `values::search search`).
10. **Condiciones estáticas de validación**: Una declaración `entry "ruta/archivo.efn";` es válida únicamente si:
    - Existe exactamente un archivo `.main` en el proyecto estructurado.
    - El archivo `.main` contiene exactamente una declaración `entry`.
    - La ruta relativa existe y es accesible desde el Project Root.
    - El archivo destino es un `.efn` válido.
    - El `.efn` contiene exactamente una `public fn`.
    - La `public fn` seleccionada declara **CERO** Value Parameters.
    - La `public fn` seleccionada declara cero o más Signature Dependency Parameters (`modulo::firma nombre_local`), y todas sus dependencias de firma pueden satisfacerse mediante la composición resuelta en `.root`.
    - Cumple todas las demás reglas normales de Function Implementation.
11. **Inyección funcional en la función inicial**: La Function Implementation inicial puede declarar Signature Dependency Parameters (`modulo::firma nombre_local`). Estas dependencias no se inyectan en `.main`, sino que son resueltas por el runtime a través de los bindings declarados en `.root`:
    ```text
    public fn initialize(window::open open_window, config::load load_config) -> InitResult
    ```
12. **Distinción entre `entry` y `bind`**:
    - `bind`: Asocia una Signature requerida (`.esig`) con su implementación (`.efn`) en `.root`.
    - `entry`: Selecciona la Function Implementation inicial (`.efn`) que arranca la aplicación en `.main`.
13. **`entry` no satisface firmas**: `entry` no declara satisfacción contractual (`: modulo::firma`). La función seleccionada puede o no implementar una firma según su propia definición, pero `.main` no interviene en contratos modulares.
14. **Operación inicial única e incondicional**: En Evo-Script v0.1 existe exactamente una operación inicial por aplicación. No se admiten cadenas de inicialización múltiples (`pre-main`, `post-main`) ni entradas condicionales (`entry ... when linux` es inválido).
15. **Universalidad de `.main`**: Un único formato `.main` sirve para cualquier tipo de aplicación estructurada (gráfica, consola, interactiva o servicio). No existen modos especiales (`gui main`, `server main`).

#### 12.9.4 Ejemplo canónico completo de aplicación estructurada

A continuación se ilustra la interacción canónica entre `.root`, `.main`, los proveedores de dependencias y la función inicial de entrada:

Estructura física del proyecto:

```text
project/
├── application.root
├── application.main
├── functions/
│   └── application.efn
└── providers/
    ├── window_native.efn
    └── config_file.efn
```

Contenido de los artefactos:

- **`application.root`** (Functional Composition Root):
  ```text
  bind window::open to "providers/window_native.efn";
  bind config::load to "providers/config_file.efn";
  ```

- **`application.main`** (Evo Application Main):
  ```text
  entry "functions/application.efn";
  ```

- **`functions/application.efn`** (Implementación inicial de la aplicación):
  ```text
  import window::open;
  import config::load;

  public fn initialize(window::open open_window, config::load load_config) -> InitResult {
      load_config();
      open_window();

      return InitResult::Ready;
  }
  ```

En este modelo:
- `application.root` define qué implementaciones concretas satisfacen las firmas `window::open` y `config::load`.
- `application.main` selecciona `functions/application.efn` como punto de arranque mediante `entry`.
- `functions/application.efn` requiere sus dependencias mediante Signature Dependency Parameters (`window::open open_window`, `config::load load_config`) sin conocer los archivos proveedores concretos.
- `functions/application.efn` no declara ningún Value Parameter, cumpliendo la regla de entrada de aplicación de v0.1.
- `application.main` desconoce por completo los proveedores `window_native.efn` y `config_file.efn`.
- El runtime valida `.root` y `.main`, inicia el Application Main Loop e invoca `initialize`.

#### 12.9.5 Semántica de terminación (Application Exit Request)

La terminación del Application Main Loop y el cierre de la aplicación se rigen formalmente por el concepto de **Application Exit Request**:

1. **Definición de Application Exit Request**: Un `Application Exit Request` es una solicitud procesada por el runtime para finalizar el Application Main Loop de una aplicación activa.
2. **Flujo unificado de terminación**:
   ```text
   Application Exit Request
           ↓
   Application Main Loop terminates
           ↓
      application terminates
   ```
3. **Responsabilidad del runtime (no es keyword del lenguaje)**:
   - La terminación de una aplicación pertenece al ciclo de vida administrado por el runtime, no a sentencias imperativas de control del lenguaje.
   - Evo-Script **NO** introduce keywords como `exit`, `quit`, `stop`, `shutdown` ni `terminate`. No existe la sentencia `exit;`.
   - `Application Exit Request` **NO** es un tipo de datos (`Value`), enum de dominio, `bool`, `string` ni resultado de función. No existen `ExitValue`, `SystemExit Value`, `ApplicationExit enum`, `MainResult` ni `ExitCode`.
   - Una función no requiere retornar ningún valor especial para terminar la aplicación (no existe `return Exit;`).
4. **Combinación estándar del runtime (`Super + Q`)**:
   - Evo Runtime establece la combinación **`Super + Q`** como vía de entrada estándar universal para solicitar el cierre de la aplicación activa.
   - **Terminología `Super`**: Se utiliza formalmente `Super` como término multiplataforma para designar la tecla Super (comúnmente identificada físicamente como tecla Windows en teclados estándar).
   - **Naturaleza en el runtime**: `Super + Q` es un input binding / política de ciclo de vida del runtime, **no es sintaxis ni palabra clave del lenguaje Evo-Script**.
   - Flujo ante el atajo:
     ```text
     user presses Super + Q
             ↓
     runtime receives shortcut
             ↓
     runtime processes Application Exit Request
             ↓
     Application Main Loop terminates
             ↓
        application terminates
     ```
5. **Unificación semántica: Múltiples fuentes (`Exit Sources`), una sola semántica**:
   - Existe una separación conceptual formal entre la fuente que origina la solicitud (`Exit Source`) y la semántica de terminación (`Exit Semantics`):
     ```text
     Exit Source != Exit Semantics
     ```
   - Diversas fuentes convergen en el mismo `Application Exit Request`:
     ```text
     Super + Q ──────────┐
                         │
     botón [X] ──────────┤
                         ├──> Application Exit Request
     menú Exit ──────────┤
                         │
     comando terminal ───┘
                                 ↓
                       Application Main Loop terminates
                                 ↓
                         application terminates
     ```
   - **Botón cerrar [X]**: El comportamiento de los controles de ventana es definido por la lógica de la aplicación o su entorno de interfaz. Cerrar una ventana puede o no emitir un `Application Exit Request` según determine la aplicación (cerrar una ventana no equivale universalmente a terminar la aplicación).
   - **Comando de terminal**: Si una aplicación interactiva de terminal procesa el comando de texto `"exit"`, se trata de una entrada textual interpretada por dicha aplicación, no de una keyword del lenguaje.
   - **Garantía del runtime**: Aunque una aplicación no implemente botones de cierre, menús o comandos específicos, el runtime garantiza la disponibilidad universal de `Super + Q` para emitir el `Application Exit Request`.
6. **Ciclo de vida unívoco y sin reinicio**:
   - El Application Main Loop termina una sola vez. No existe reinicio automático (`automatic restart`), recreación de loop ni resurrección de la aplicación dentro de la misma ejecución.
   - La terminación del Main Loop no introduce `break` ni `continue` (no se modela como un `break` imperativo).
   - Preserva estrictamente la ausencia de recursión y la detección de `FunctionCallCycleError` en el grafo de llamadas de funciones.
   - No se diseñan APIs concretas de interfaz en este bloque (no Window API, Button API, Menu API, Terminal Command API, Event API, ni Signatures oficiales como `application::exit` o `runtime::exit`).
   - No se introducen callbacks ni hooks de terminación (`on_exit`, `before_exit`, `shutdown handler`, cleanup callbacks, destructores) ni modelos de confirmación/veto ("Are you sure?", veto exit).

#### 12.9.6 Distinción formal entre Function return y Application exit

Evo-Script distingue formalmente entre finalizar la evaluación de una función y terminar la ejecución global de la aplicación:

1. **`Function return`**: La sentencia `return expresion;` finaliza estrictamente la correspondencia de la Function Implementation evaluada y produce su valor tipado (`Value`).
2. **`Application exit`**: Ocurre única y exclusivamente cuando finaliza el **Application Main Loop** tras procesar un **Application Exit Request**.
3. **Independencia del retorno de la función inicial**: La terminación de la función inicial no implica el cierre de la aplicación:
   ```text
   Function return != Application Exit Request
   Function return != Application exit
   ```
   Ejemplo:
   ```text
   public fn initialize(...) -> InitResult {
       // Inicializa componentes y abre interfaz
       return InitResult::Ready;
   }
   ```
   Cuando `initialize` ejecuta su `return InitResult::Ready;`, la evaluación de `initialize` concluye, pero si el Application Main Loop permanece activo (por ejemplo, atendiendo eventos de ventana, interacción del usuario o solicitudes del host), la aplicación permanece viva.
4. **Las alternativas de dominio no son señales de salida**: Retornar un valor como `SearchResult::NotFound` o `InitResult::Error("fallo")` constituye la entrega normal de un `Value` de tipo enum y no detiene automáticamente el Application Main Loop.

#### 12.9.7 Interacción entre EvaluationError, Application Main Loop y ciclo de vida

Evo-Script v0.1 define con precisión formal la relación entre los fallos de evaluación (`EvaluationError`) y el ciclo de vida de la aplicación (`Application Lifetime`):

1. **Origen y naturaleza del `EvaluationError`**: Un `EvaluationError` ocurre exclusivamente durante la evaluación de una expresión u operación concreta dentro de una Function Implementation (`ConversionError`, `OverflowError`, `DivisionByZeroError`). Pertenece al proceso de cómputo funcional y no al ciclo de vida global de la aplicación.
2. **Propagación hacia afuera hasta el runtime**:
   - Cuando ocurre un `EvaluationError`, la evaluación actual se aborta inmediatamente y no se produce el `Value` esperado.
   - El fallo se propaga hacia afuera a través de la cadena de llamadas (`Function Call Chain`) hasta alcanzar el límite del **Evo Runtime / Host** que solicitó la evaluación:
     ```text
     failing expression
             ↑
        current Function
             ↑
         caller Function
             ↑
         caller Function
             ↑
        Evo Runtime / Host
     ```
3. **Naturaleza no capturable en Evo-Script v0.1**: Conforme a la Sección 10.7, el `EvaluationError` no es capturable dentro del código Evo-Script (no existen `try`, `catch`, `throw`, `recover`, `rescue` ni tipos de error en el lenguaje).
4. **Independencia fundamental frente al Application Main Loop**:
   - Un `EvaluationError` por sí mismo **NO** termina automáticamente el Application Main Loop.
   - El Application Main Loop no forma parte del grafo de llamadas funcionales por el que se propaga el error.
   - Mientras el **Application Main Loop** continúe activo en el runtime, la aplicación permanece viva:
     ```text
     failed evaluation != dead application
     EvaluationError != Application Exit Request
     EvaluationError != Application Main Loop termination
     EvaluationError != Application exit
     ```
5. **Diferenciación estricta de tres resultados semánticos**:
   - **`Function return`**: Produce un resultado tipado (`Value`) tras completar la evaluación normalmente.
   - **`EvaluationError`**: Fallo de evaluación que aborta la computación actual y se propaga hacia afuera al Evo Runtime sin terminar el Application Main Loop.
   - **`Application Exit Request`**: Solicitud de ciclo de vida (vía `Super + Q` o acciones de aplicación) que finaliza el Application Main Loop y provoca la terminación de la aplicación.
   ```text
   Function return
         ↓
       Value
         ↓
   evaluation completed


   EvaluationError
         ↓
   current evaluation aborted
         ↓
   failure propagated to Evo Runtime (Main Loop remains active)


   Application Exit Request
         ↓
   Application Main Loop terminates
         ↓
   application terminates
   ```
6. **Las alternativas de dominio y fallos de evaluación no son solicitudes de salida**:
   - Valores como `SearchResult::NotFound` o `InitResult::Error("fallo")` son `Values` normales de enum de dominio.
   - Ni los valores de dominio ni los `EvaluationError` se transforman implícitamente en `Application Exit Request`.
7. **Error durante la función de entrada inicial**: Si durante la evaluación de la función inicial seleccionada por `entry` ocurre un `EvaluationError`, dicha evaluación aborta y el fallo se propaga al Evo Runtime; no se genera automáticamente un `Application Exit Request` ni se introducen políticas especiales de startup failure en v0.1.
8. **Ausencia de estado de error en el Application Main Loop**: El runtime no crea estados de error en el sistema de tipos (no existen `MainLoop::Failed`, `Application::Failed` ni `RuntimeState::Error`). No se diseñan en v0.1 interfaces de diagnóstico/error (`Error UI`), trazabilidad gráfica ni mecanismos de reintento automático (`retry`).

#### 12.9.8 Ámbito de aplicación del modelo

1. **Universalidad para aplicaciones estructuradas**: El Application Main Loop modela el ciclo de vida de cualquier aplicación estructurada (gráfica con ventanas, de terminal interactiva o tipo servicio/servidor). Las aplicaciones gráficas no requieren implementar bucles infinitos para mantener viva su interfaz.
2. **Scripts autocontenidos**: Un script simple `.efn` ejecutado directamente no requiere `.root`, `.main` ni `entry`. Su evaluación concluye inmediatamente cuando su única `public fn` ejecuta su `return`, entregando el resultado directamente al host exterior sin Application Main Loop.
3. **Unicidad de `.main`**: Toda aplicación estructurada posee exactamente un archivo `.main`. No se admiten archivos `.main` múltiples, anidados, incluidos ni heredados.
4. **Manifiesto físico de artefactos (.elib)**: En un proyecto estructurado activo, el archivo `.main` y el archivo `.root` forman parte del `Physical Artifact Universe` registrado por el manifiesto `.elib` activo (véase la Sección 12.10).

#### 12.9.9 Cierre normativo de .main en v0.1

Todos los aspectos arquitectónicos y semánticos fundamentales de `.main`, Application Main Loop, Application Lifetime, Application Exit Request y la relación con EvaluationError quedan plenamente formalizados y cerrados para la especificación oficial de Evo-Script v0.1.


### 12.10 Manifiesto físico de artefactos (.elib) y resolución física

Un archivo `.elib` (Evo Library) define el **manifiesto físico de artefactos** (`Physical Artifact Manifest`) y constituye la **unidad de resolución física** (`Physical Resolution Unit`) de un proyecto estructurado en Evo-Script v0.1.

#### 12.10.1 Responsabilidad oficial de .elib y separación de responsabilidades

La responsabilidad normativa exclusiva de un archivo `.elib` es declarar formal y explícitamente **qué artefactos físicos pertenecen a la unidad estructurada activa**:

```text
.elib  ──►  WHAT PHYSICAL ARTIFACTS BELONG
```

Evo-Script v0.1 establece una separación estricta e inviolable entre cuatro responsabilidades arquitectónicas ortogonales:

1. **`.elib` (Physical Membership)**: Declara explícitamente la totalidad de archivos físicos que componen el universo de artefactos del proyecto estructurado activo (`Physical Artifact Universe`). No define publicación semántica, no selecciona implementaciones, no arranca la aplicación ni define dependencias de paquetes.
2. **`.emod` (Semantic Publication)**: Declara la frontera lógica de un módulo y qué firmas y tipos de datos conforman su superficie pública accesible externamente (`publish`). No contiene rutas de archivos ni realiza descubrimiento físico.
3. **`.root` (Functional Composition)**: Selecciona qué implementación física concreta (`.efn`) satisface cada contrato de firma pública (`.esig`) requerida (`bind ... to ...`).
4. **`.main` (Application Entry and Lifetime)**: Selecciona la operación inicial del proyecto (`entry "..."`) y administra el ciclo de vida de ejecución (`Application Main Loop`).

En Evo-Script v0.1, `.elib` **no es un gestor de paquetes** (`package manager`), no define versiones ni metadatos de distribución, no gestiona repositorios remotos ni declara dependencias inter-librería.


#### 12.10.2 Sintaxis y estructura de .elib (`artifact "relative/path";`)

La gramática de un archivo `.elib` consta exclusivamente de cero o más declaraciones de membresía física de artefactos mediante la palabra clave estructural `artifact`:

```text
artifact "ruta/relativa/archivo.extension";
```

Ejemplo normativo:

```text
artifact "application.root";
artifact "application.main";

artifact "definitions/use_cases/use-cases.emod";
artifact "definitions/use_cases/copy-file.esig";

artifact "definitions/requesters/requesters.emod";
artifact "definitions/requesters/copy-completed.esig";

artifact "definitions/contracts/contracts.emod";
artifact "definitions/contracts/read-file.esig";
artifact "definitions/contracts/write-file.esig";

artifact "definitions/domain/domain.emod";
artifact "definitions/domain/file-view.estc";
artifact "definitions/domain/copy-result.enum";

artifact "agents/copier.efn";

artifact "resolvers/origin-resolver.efn";
artifact "resolvers/destination-resolver.efn";
artifact "resolvers/copy-resolver.efn";

artifact "collaborators/copy-buffer.efn";

artifact "providers/std-file-system.efn";
```

Reglas normativas:

1. **Palabra clave `artifact`**: `artifact` es una palabra clave estructural reservada exclusiva de archivos `.elib`. No puede utilizarse como identificador.
2. **Ausencia de bloque envolvente**: El archivo `.elib` no posee ni admite bloques envolventes (`library Nombre { ... }` o `elib { ... }` son estrictamente inválidos). Las sentencias `artifact` se declaran directamente a nivel de archivo.
3. **El nombre físico del archivo no define identidad semántica**: Nombres como `application.elib`, `project.elib` o `core.elib` son identificadores físicos del archivo en el sistema de archivos. El basename no introduce namespaces, nombres de módulo, identidades de paquete ni nombres lógicos en el lenguaje.
4. **Prohibición de sentencias de otros artefactos**: Un archivo `.elib` no puede contener declaraciones `publish`, `bind`, `entry`, `import`, `module`, definiciones de `struct`, `enum` o funciones (`fn`), variables (`let`), expresiones ni sentencias de ejecución en runtime.
5. **No es una sentencia ejecutable**: `artifact` se procesa exclusivamente durante el análisis físico y semántico estático; no ejecuta operaciones ni produce valores en runtime.


#### 12.10.3 Active Library, Library Base Directory y Physical Artifact Universe

1. **Librería activa (`Active Library`)**: En un proyecto estructurado, el host/tooling inicia el análisis proporcionando explícitamente la ruta al archivo `.elib` que actuará como la **Active Library**. No existe auto-descubrimiento heurístico ni búsqueda implícita por directorios padres para adivinar el archivo `.elib`.
2. **Directorio base de la librería (`Library Base Directory`)**: Es el directorio físico del sistema de archivos que contiene al archivo `.elib` activo. Si la Active Library es `/home/user/my-app/application.elib`, su `Library Base Directory` es `/home/user/my-app/`.
3. **Rutas de `artifact` relativas al Library Base Directory**: Toda ruta declarada en `artifact "..."` se resuelve de forma estrictamente relativa al `Library Base Directory`. Nunca se interpreta respecto del directorio de trabajo actual (`current working directory`), ni del directorio de `.root`, `.main` o `.emod`.
4. **Prohibición de rutas no relativas y comodines**:
   - Solo se admiten rutas relativas locales en el sistema de archivos (`relative filesystem paths`).
   - Quedan estrictamente prohibidas rutas absolutas (`/usr/...`, `C:\...`), URLs (`http://...`, `https://...`), variables de entorno (`$HOME/...`), expansión de shell (`~`), patrones glob/comodines (`*.esig`, `**/*`) y referencias a registros de paquetes.
5. **Prohibición de escape del Library Base Directory**: Una ruta de artefacto no puede escapar del `Library Base Directory` mediante secuencias de navegación padre (`artifact "../../fuera/archivo.esig";` es inválido). Toda ruta normalizada debe resolver estrictamente dentro del árbol del `Library Base Directory`.
6. **Universo físico de artefactos (`Physical Artifact Universe`)**: La Active Library define el `Physical Artifact Universe` completo y cerrado del proyecto. Solamente los archivos explícitamente registrados mediante `artifact "..."` forman parte del proyecto.
   - `exists on filesystem != belongs to Active Library`: Que un archivo exista en el disco dentro del subárbol no lo convierte en artefacto del proyecto si no está registrado en el `.elib` activo.
7. **Ausencia de Directory Scanning**: Evo-Script no realiza escaneo de directorios (`directory scanning`) ni descubrimiento recursivo de archivos de código fuente. No se inspecciona el disco para buscar archivos `.estc`, `.enum`, `.esig` o `.efn` no registrados.
8. **Scripts autocontenidos**: Un script autocontenido `.efn` ejecutado directamente por el host no requiere `.elib` ni distribución física previa. La complejidad estructural solo aparece cuando existe distribución estructural de artefactos.
9. **Libertad en subdirectorios**: Evo-Script no impone una estructura rígida de carpetas ni reserva nombres de directorios como `modules/` o `src/`. El desarrollador puede organizar sus carpetas libremente (`definitions/`, `domain/`, `contracts/`, `agents/`, `providers/`, `custom/`).


#### 12.10.4 Library Artifact Table y errores de validación de .elib

1. **Tabla interna de artefactos (`Library Artifact Table`)**: Durante el análisis estático, el compilador/analizador construye en memoria una tabla a partir de las declaraciones `artifact` del `.elib` activo. Esta tabla es una estructura interna en memoria; no genera archivos de índice, manifiestos binarios ni bases de datos de caché en v0.1.
2. **Extensiones admitidas por `artifact`**: En Evo-Script v0.1, las únicas extensiones admitidas en declaraciones `artifact` son:
   - `.root`
   - `.main`
   - `.emod`
   - `.esig`
   - `.estc`
   - `.enum`
   - `.efn`
3. **Prohibición de registro de `.elib` y `.evo`**: No se permite registrar archivos `.elib` ni `.evo` como artefactos dentro de un `.elib` (no existen librerías anidadas ni inclusión de múltiples librerías en v0.1). El archivo `.elib` activo tampoco se lista a sí mismo.
4. **Errores de validación de manifiesto**:
   - **`LibraryArtifactPathError`**: Se produce si la ruta declarada en un `artifact` es absoluta, contiene una URL, utiliza comodines o intenta escapar del `Library Base Directory`.
   - **`LibraryArtifactNotFoundError`**: Se produce si el archivo físico declarado en una sentencia `artifact` no existe en el sistema de archivos.
   - **`DuplicateLibraryArtifactError`**: Se produce si dos o más declaraciones `artifact` dentro del mismo `.elib` registran la misma ruta física normalizada.
   Todos estos errores pertenecen a la categoría de **System / Validation Errors**; impiden la evaluación y se detectan estáticamente antes de iniciar la ejecución.


#### 12.10.5 Frontera física de módulo (Physical Module Boundary)

1. **Definición de Physical Module Boundary**: Cada archivo `.emod` registrado en la Active Library establece una frontera física de módulo anclada en el **directorio que contiene dicho `.emod`**.
2. **Desacoplamiento entre nombres de archivo/directorio e identidad de módulo**:
   - El nombre del directorio físico **NO** define la identidad del módulo. Un directorio `definitions/contracts/` cuyo `.emod` declare `module contracts { ... }` tiene como identidad lógica `contracts` (no `definitions/contracts` ni `definitions`).
   - El nombre del archivo `.emod` tampoco define la identidad lógica. La identidad del módulo proviene **exclusivamente** de la cláusula declarativa `module Nombre { ... }` en el contenido del archivo.
3. **Membresía física de artefactos modulares (`Nearest Registered Ancestor .emod`)**:
   - Todo artefacto registrado `.esig`, `.estc` o `.enum` pertenece físicamente al módulo cuyo `.emod` registrado sea su **ancestro físico más cercano** en el árbol de rutas.
   - *Regla de resolución*: Para un artefacto registrado, se parte de su directorio y se asciende por los directorios padres; el primer directorio que contenga un `.emod` registrado en la Active Library es el módulo propietario del artefacto.
4. **Subdirectorios internos y módulos anidados**:
   - Un módulo puede contener subdirectorios internos (por ejemplo `definitions/contracts/io/read-file.esig` pertenece al módulo `contracts` si `contracts.emod` es su ancestro registrado más cercano).
   - Se admiten fronteras anidadas (`definitions/domain/domain.emod` y `definitions/domain/geography/geography.emod`), donde los artefactos bajo `geography/` pertenecen al módulo `geography` por ser su ancestro más cercano.
5. **Errores de frontera modular (`ModuleBoundaryError`)**:
   - Se produce si un directorio contiene dos o más archivos `.emod` registrados en la misma Active Library.
   - Se produce si un artefacto registrado `.esig`, `.estc` o `.enum` no posee ningún ancestro `.emod` registrado en el `Physical Artifact Universe`.
   - `ModuleBoundaryError` pertenece a la categoría `System / Validation Error`.
6. **Independencia de implementaciones (.efn)**: Los archivos `.efn` representan implementaciones y **no requieren** pertenecer a una `Physical Module Boundary`. Pueden residir en carpetas independientes (`agents/`, `resolvers/`, `providers/`). Si un `.efn` reside físicamente bajo un directorio con un `.emod`, **nunca es publicado por dicho `.emod`** ni se convierte en símbolo público.
7. **Exclusión de `.root` y `.main`**: `.root` y `.main` se registran en `.elib` para formar parte del universo del proyecto, pero no pertenecen a ninguna tabla de módulo ni son símbolos publicables.


#### 12.10.6 Module Artifact Table y Public Symbol Table

1. **Tabla de artefactos de módulo (`Module Artifact Table`)**: Para cada módulo registrado, el compilador construye una tabla interna que asocia los símbolos semánticos declarados dentro de los archivos `.esig`, `.estc` y `.enum` pertenecientes a su `Physical Module Boundary`.
2. **La identidad del símbolo proviene del contenido, NO del nombre del archivo**:
   - Un archivo que contiene `struct Worker { ... }` define el símbolo `Worker`.
   - Un archivo que contiene `enum SearchResult { ... }` define el símbolo `SearchResult`.
   - Un archivo que contiene `esig read_file(...) -> ReadResult;` define el símbolo `read_file`.
   - El nombre del archivo físico (ej. `worker.estc`, `read-file.esig`) no determina por sí mismo el símbolo semántico. No existe un algoritmo de conversión de mayúsculas/minúsculas ni transformación automática `kebab-case` $\leftrightarrow$ `camelCase`/`snake_case`. La convención `kebab-case` en nombres de archivo es una convención humana de estilo, no una regla de resolución semántica.
3. **Colisión de símbolos en el módulo (`DuplicateModuleSymbolError`)**: Si dos o más artefactos registrados dentro de la misma `Physical Module Boundary` declaran el mismo símbolo semántico (por ejemplo, dos archivos `.estc` que declaran `struct Worker`), la validación falla con `DuplicateModuleSymbolError` (System / Validation Error).
4. **Unicidad de identidades de módulo (`DuplicateModuleError`)**: Dos archivos `.emod` registrados en la Active Library no pueden declarar la misma identidad lógica (`module Nombre`). Una colisión de nombres de módulo produce `DuplicateModuleError` (System / Validation Error).
5. **Tabla de símbolos públicos (`Public Symbol Table`)**:
   - A partir del archivo `.emod`, el compilador procesa las sentencias `publish Simbolo;` para construir la `Public Symbol Table` del módulo.
   - `publish` no descubre archivos en disco ni recibe rutas; valida y expone un símbolo ya presente en la `Module Artifact Table`.
   - Cada cláusula `publish Simbolo;` debe corresponder exactamente a un símbolo existente en la `Module Artifact Table` de ese módulo. Si no existe, se produce `ModuleSymbolNotFoundError`.
   - Publicar el mismo símbolo múltiples veces en el mismo `.emod` es inválido y se rechaza en validación estática.
   - Solo pueden publicarse `.esig`, `.estc` y `.enum`. Queda estrictamente prohibido publicar `.efn`, `.root`, `.main`, `.elib` o `.evo`.
6. **Manejo de símbolos privados / no publicados**: Un artefacto modular presente en la `Module Artifact Table` que no haya sido publicado mediante `publish` permanece privado al módulo. Si un consumidor externo intenta importarlo (`import modulo::no_publicado;`), la resolución falla con **`ModuleSymbolNotFoundError`**, sin exponer externamente si el símbolo existe de forma privada o no.


#### 12.10.7 Flujo determinista de resolución física y modular

Evo-Script v0.1 define un flujo de resolución determinista y unívoco para importaciones de dependencias:

```text
import modulo::simbolo;
    ↓
Active .elib
    ↓
Localizar .emod cuya Module Identity sea 'modulo' (falla con ModuleNotFoundError si no existe)
    ↓
Consultar la Public Symbol Table del módulo (falla con ModuleSymbolNotFoundError si 'simbolo' no está publicado)
    ↓
Identificar en la Module Artifact Table el artefacto registrado (.esig, .estc o .enum)
    ↓
Cargar la definición semántica del artefacto
    ↓
Registrar el símbolo importado en el espacio semántico local correspondiente:
    - Signature ──► Signature Space
    - Struct / Enum ──► Type Space
```

Principios de resolución:

1. **Resolución exacta sin heurísticas**: Cada referencia calificada `modulo::simbolo` resuelve a exactamente un artefacto físico registrado y publicado, o el programa se rechaza con un error de validación del sistema. No existen reglas de búsqueda heurística, prioridades por orden de declaración, ni alternativas de respaldo (*fallback paths*).
2. **Invarianza semántica frente a reubicación física**: Si un artefacto físico se traslada a otro subdirectorio dentro de la misma frontera modular (por ejemplo, de `definitions/contracts/read-file.esig` a `definitions/contracts/io/read-file.esig`), y se actualiza su registro en el `.elib` activo, la identidad lógica formal (`contracts::read_file`) permanece inalterada y los consumidores no requieren modificación.
3. **Ausencia de magia de reubicación**: Evo-Script no busca automáticamente archivos movidos. Si el `.elib` activo conserva la ruta antigua, se produce `LibraryArtifactNotFoundError`.
4. **Mismo nombre de símbolo en módulos distintos**: Es plenamente válido que dos módulos independientes publiquen símbolos homónimos (ej. `use_cases::search` y `contracts::search`). No existe conflicto modular global porque cada uno pertenece a una `Module Identity` formalmente diferenciada.
5. **Distinción entre errores de colisión**:
   - `DuplicateModuleSymbolError`: Conflicto de definición física donde dos archivos del mismo módulo definen el mismo símbolo.
   - `TypeNameCollisionError`: Conflicto en el consumidor local cuando dos importaciones o declaraciones locales intentan ocupar el mismo nombre en su `Type Space`.


#### 12.10.8 Orden de validación del proyecto estructurado

La validación de un proyecto estructurado bajo una Active Library sigue una secuencia estricta previa a cualquier ejecución en runtime:

```text
1. Proporcionar la Active Library (.elib)
      ↓
2. Validar sintaxis y rutas de artifact en .elib ──► (falla con LibraryArtifactPathError,
                                                    LibraryArtifactNotFoundError, DuplicateLibraryArtifactError)
      ↓
3. Construir el Physical Artifact Universe y la Library Artifact Table
      ↓
4. Identificar las Physical Module Boundaries (.emod) ──► (falla con ModuleBoundaryError, DuplicateModuleError)
      ↓
5. Construir las Module Artifact Tables (.esig, .estc, .enum) ──► (falla con DuplicateModuleSymbolError)
      ↓
6. Construir las Public Symbol Tables a partir de publish ──► (falla con ModuleSymbolNotFoundError)
      ↓
7. Resolver importaciones, Signature Type Closures y Type Dependency Closures ──► (falla con ModuleNotFoundError,
                                                                                  TypeNameCollisionError, RecursiveTypeCycleError)
      ↓
8. Validar la composición funcional en .root ──► (falla si el .efn objetivo no pertenece
                                                 al Physical Artifact Universe o hay discordancia de firma)
      ↓
9. Validar la entrada de aplicación en .main ──► (falla si el .efn de entry no pertenece al Physical Artifact Universe)
      ↓
10. Iniciar el Application Main Loop y evaluar la entrada inicial
```

Si se produce cualquier error de validación en cualquiera de los pasos, el proyecto estructurado es **rechazado íntegramente** y el `Application Main Loop` nunca se inicia.


### 12.11 Artefacto distribuible (.evo) - Estado reservado

La extensión `.evo` (Evo Package / artefacto distribuible) queda expresamente **reservada** para especificaciones futuras del ecosistema Evo:

1. **Naturaleza del concepto**: `.evo` representa conceptualmente el artefacto empaquetado final para la distribución de aplicaciones o módulos preparados para ejecución o consumo.
2. **Fuera de alcance en Evo-Script v0.1**: En Evo-Script v0.1:
   - **NO se define el formato físico interno** de `.evo` (formatos de archivo, contenedores, compresión ni metadatos binarios).
   - **NO se define el sistema de empaquetado** (no se definen procesos de empaquetado, manifiestos ni herramientas de construcción/compilación de paquetes).
   - **NO se define instalación ni distribución** de archivos `.evo` (no existen registros de paquetes, servidores de publicación ni clientes de distribución).
3. **No es código fuente**: `.evo` no es una extensión para archivos de código fuente (las fuentes utilizan estrictamente `.efn`, `.esig`, `.estc`, `.enum`, `.emod`, `.root` y `.main`).
4. **Relación futura desacoplada con `.elib`**: Una especificación futura podrá vincular manifiestos de fuentes (`.elib`) con artefactos empaquetados (`.evo`), pero dicha correspondencia no se prejuzga ni se cierra de forma prematura en v0.1 (no se asume que todo `.evo` es una librería ni que todo `.evo` es una aplicación).
5. **Innecesario para scripts autocontenidos y aplicaciones v0.1**: Los scripts `.efn` y los proyectos estructurados con `.elib`, `.root` y `.main` se ejecutan directamente en v0.1 sin requerir contenedor ni empaquetado `.evo`.


### 12.12 Frontera con el entorno de ejecución (Host / Runtime)

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


### 12.13 Ejemplos canónicos y modelos arquitectónicos

#### 12.13.1 Script autocontenido completo (`washer.efn`)

```text
struct Clothes {
    string name;
}

enum WashResult {
    Ok(Clothes)
    Error(string)
}

private fn prepare(Clothes clothes) -> Clothes {
    return clothes;
}

public fn washes_clothes(Clothes clothes) -> WashResult {
    let Clothes prepared = prepare(clothes);

    return WashResult::Ok(prepared);
}
```

Características:
- `Clothes` y `WashResult` son tipos locales a `washer.efn`.
- `prepare` es una función auxiliar privada (`private fn`).
- `washes_clothes` es la única función pública (`public fn`).
- No requiere `.esig`, `.estc`, `.enum`, `.root` ni `.main`.
- Se ejecuta directamente por un host/runtime Evo-Script.

#### 12.13.2 Proyecto canónico estructurado con Functional Composition Root (`project/`)

A continuación se presenta un ejemplo canónico completo de un proyecto estructurado con manifiesto físico de artefactos (`application.elib`), tipos compartidos, contrato público, módulo, múltiples implementaciones físicas, consumidor inyectado, consumidor directo y el Functional Composition Root (`application.root`):

Estructura física de archivos:

```text
project/
├── application.elib
├── application.root
│
├── values/
│   ├── values.emod
│   ├── search.esig
│   ├── SearchResult.enum
│   └── Worker.estc
│
├── providers/
│   ├── search_database.efn
│   ├── search_memory.efn
│   └── search_remote.efn
│
└── functions/
    ├── process.efn
    └── consumer.efn
```

Contenido de los artefactos:

- **`application.elib`** (Manifiesto físico de artefactos / Active Library):
  ```text
  artifact "application.root";
  artifact "values/values.emod";
  artifact "values/search.esig";
  artifact "values/SearchResult.enum";
  artifact "values/Worker.estc";
  artifact "providers/search_database.efn";
  artifact "providers/search_memory.efn";
  artifact "providers/search_remote.efn";
  artifact "functions/process.efn";
  artifact "functions/consumer.efn";
  ```

- **`Worker.estc`** (Struct compartido):
  ```text
  struct Worker {
      int id;
      string name;
  }
  ```

- **`SearchResult.enum`** (Enum compartido):
  ```text
  import values::Worker;

  enum SearchResult {
      Found(Worker)
      NotFound
      Error(string)
  }
  ```

- **`search.esig`** (Firma pública / Contrato):
  ```text
  import values::SearchResult;

  esig search(int id) -> SearchResult;
  ```

- **`values.emod`** (Módulo y catálogo público):
  ```text
  module values {
      publish Worker;
      publish SearchResult;
      publish search;
  }
  ```

- **`providers/search_database.efn`** (Implementación A que satisface la firma):
  ```text
  import values::SearchResult;
  import values::search;

  public fn search(int id) -> SearchResult
      : values::search
  {
      return SearchResult::NotFound;
  }
  ```

- **`providers/search_memory.efn`** (Implementación B que satisface la firma):
  ```text
  import values::SearchResult;
  import values::search;

  public fn search(int id) -> SearchResult
      : values::search
  {
      return SearchResult::NotFound;
  }
  ```

- **`providers/search_remote.efn`** (Implementación C que satisface la firma):
  ```text
  import values::SearchResult;
  import values::search;

  public fn search(int id) -> SearchResult
      : values::search
  {
      return SearchResult::NotFound;
  }
  ```

- **`functions/process.efn`** (Consumidor inyectado que requiere la capacidad):
  ```text
  import values::SearchResult;
  import values::search;

  public fn process(int id, values::search search) -> SearchResult {
      return search(id);
  }
  ```

- **`functions/consumer.efn`** (Consumidor directo que utiliza la capacidad):
  ```text
  import values::SearchResult;
  import values::search;

  public fn execute(int id) -> SearchResult {
      return search(id);
  }
  ```

- **`application.root`** (Functional Composition Root):
  ```text
  bind values::search to "providers/search_database.efn";
  ```

En este modelo:
- `application.elib` delimita el universo físico cerrado de artefactos activos.
- `values.emod` establece la frontera física y publica exclusivamente `Worker`, `SearchResult` y `search`.
- Coexisten tres implementaciones físicas distintas que satisfacen `: values::search` (`search_database.efn`, `search_memory.efn`, `search_remote.efn`).
- `application.root` compone el proyecto seleccionando una única implementación concreta (`bind values::search to "providers/search_database.efn";`).
- `functions/process.efn` requiere `values::search` mediante un Signature Dependency Parameter (`values::search search`) sin conocer la implementación concreta.
- `functions/consumer.efn` importa y utiliza directamente `values::search` a nivel de archivo sin conocer la implementación concreta.
- **Sustitución de implementación sin alterar consumidores**: Para cambiar el proveedor a memoria (por ejemplo, para pruebas automáticas), se modifica exclusivamente `application.root`:
  ```text
  bind values::search to "providers/search_memory.efn";
  ```
  Tanto `process.efn` como `consumer.efn` y los proveedores permanecen completamente inalterados. Esto demuestra Inversión de Control Funcional pura sin constructores ni instancias de objetos.

#### 12.13.3 Proyecto estructurado completo (`laundry/`)

Estructura física de archivos:

```text
laundry/
├── application.elib
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

- **`application.elib`** (Manifiesto físico de artefactos / Active Library):
  ```text
  artifact "application.root";
  artifact "application.main";
  artifact "laundry.emod";
  artifact "clothes.estc";
  artifact "washes_clothes_result.enum";
  artifact "washes_clothes.esig";
  artifact "washer.efn";
  ```

- **`clothes.estc`** (Struct compartido):
  ```text
  struct Clothes {
      string name;
  }
  ```

- **`washes_clothes_result.enum`** (Enum compartido):
  ```text
  import laundry::Clothes;

  enum WashesClothesResult {
      Ok(Clothes)
      Error(string)
  }
  ```

- **`washes_clothes.esig`** (Firma pública / Contrato):
  ```text
  import laundry::Clothes;
  import laundry::WashesClothesResult;

  esig washes_clothes(Clothes clothes) -> WashesClothesResult;
  ```

- **`laundry.emod`** (Módulo y catálogo público):
  ```text
  module laundry {
      publish Clothes;
      publish WashesClothesResult;
      publish washes_clothes;
  }
  ```

- **`washer.efn`** (Implementación que satisface la firma):
  ```text
  import laundry::Clothes;
  import laundry::WashesClothesResult;
  import laundry::washes_clothes;

  private fn prepare(Clothes clothes) -> Clothes {
      return clothes;
  }

  public fn washes_clothes(Clothes clothes) -> WashesClothesResult
      : laundry::washes_clothes
  {
      let Clothes prepared = prepare(clothes);

      return WashesClothesResult::Ok(prepared);
  }
  ```

- **`application.root`** (Functional Composition Root):
  ```text
  bind laundry::washes_clothes to "washer.efn";
  ```
- **`application.main`** (Evo Application Main):
  ```text
  entry "washer.efn";
  ```

#### 12.13.4 Diagrama de responsabilidades de proyecto estructurado

```text
                      Active .elib
                           │
                [Physical Artifact Universe]
                           │
                ┌──────────┴──────────┐
                ▼                     ▼
              .main                 .root
        (Application Entry)   (Composition Root)
                │                     │
                └──────────┬──────────┘
                           ▼
                         .emod
                   (Module Boundary)
                           │
                         .esig
                   (Public Signature)
                         /   \
                     .estc   .enum
                 (Shared User Types)
                         \   /
                         .efn
                    (Implementation)
```

#### 12.13.5 Arquitectura modular desacoplada completa (`my-app/`)

Evo-Script no impone una jerarquía rígida de carpetas (no existen carpetas reservadas por el lenguaje como `modules/` o `src/`). A continuación se presenta un ejemplo canónico de una aplicación con arquitectura limpia desacoplada:

Estructura física de archivos:

```text
my-app/
├── application.elib
├── application.root
├── application.main
│
├── definitions/
│   ├── use_cases/
│   │   ├── use-cases.emod
│   │   └── copy-file.esig
│   │
│   ├── requesters/
│   │   ├── requesters.emod
│   │   └── copy-completed.esig
│   │
│   ├── contracts/
│   │   ├── contracts.emod
│   │   ├── read-file.esig
│   │   └── write-file.esig
│   │
│   └── domain/
│       ├── domain.emod
│       ├── file-view.estc
│       └── copy-result.enum
│
├── agents/
│   └── copier.efn
│
├── resolvers/
│   ├── origin-resolver.efn
│   ├── destination-resolver.efn
│   └── copy-resolver.efn
│
├── collaborators/
│   └── copy-buffer.efn
│
└── providers/
    └── std-file-system.efn
```

Manifiesto físico (`application.elib`):

```text
artifact "application.root";
artifact "application.main";

artifact "definitions/use_cases/use-cases.emod";
artifact "definitions/use_cases/copy-file.esig";

artifact "definitions/requesters/requesters.emod";
artifact "definitions/requesters/copy-completed.esig";

artifact "definitions/contracts/contracts.emod";
artifact "definitions/contracts/read-file.esig";
artifact "definitions/contracts/write-file.esig";

artifact "definitions/domain/domain.emod";
artifact "definitions/domain/file-view.estc";
artifact "definitions/domain/copy-result.enum";

artifact "agents/copier.efn";

artifact "resolvers/origin-resolver.efn";
artifact "resolvers/destination-resolver.efn";
artifact "resolvers/copy-resolver.efn";

artifact "collaborators/copy-buffer.efn";

artifact "providers/std-file-system.efn";
```

Módulos y superficies públicas (`.emod`):

- **`definitions/use_cases/use-cases.emod`**:
  ```text
  module use_cases {
      publish copy_file;
  }
  ```
- **`definitions/requesters/requesters.emod`**:
  ```text
  module requesters {
      publish copy_completed;
  }
  ```
- **`definitions/contracts/contracts.emod`**:
  ```text
  module contracts {
      publish read_file;
      publish write_file;
  }
  ```
- **`definitions/domain/domain.emod`**:
  ```text
  module domain {
      publish FileView;
      publish CopyResult;
  }
  ```

Implementaciones y desacoplamiento:
- Los directorios `agents/`, `resolvers/`, `collaborators/` y `providers/` alojan implementaciones `.efn`. No requieren archivos `.emod` porque no definen contratos públicos compartidos.
- Nombres de carpetas como `definitions`, `use_cases`, `requesters`, `contracts`, `domain`, `agents`, `resolvers`, `collaborators` y `providers` son convenciones organizacionales del usuario y **no son palabras clave** ni conceptos normativos impuestos por Evo-Script.
- Cada consumidor y componente interactúa exclusivamente a través de identidades formales (`contracts::read_file`, `domain::FileView`, `use_cases::copy_file`), permitiendo una composición determinista mediante `application.root` y un inicio de aplicación mediante `application.main`.


## 13. Elementos léxicos y convenciones de nombres

Esta sección formaliza las decisiones léxicas, la codificación del código fuente, los delimitadores de comentarios, el principio de palabras reservadas, las gramáticas de identificadores y literales, y las convenciones de nombres para Evo-Script v0.1.


### 13.1 Comentarios

Evo-Script v0.1 define exactamente una única forma oficial de comentario:

1. **Comentarios de una línea (`//`)**:
   - La secuencia `//` inicia un comentario que se extiende hasta el final de la línea física actual.
   - El contenido posterior a `//` en la misma línea es ignorado por el análisis léxico y no forma parte de los tokens evaluables del programa.
   - **Ejemplos válidos**:
     ```text
     let int age = 43; // edad actual

     // comentario de línea completa
     let bool active = true;
     ```
2. **Ausencia de comentarios multilínea**:
   - Evo-Script v0.1 **NO** define ni soporta delimitadores de comentarios multilínea (`/* ... */`).
   - Construcciones como las siguientes son **inválidas**:
     ```text
     /* comentario */ // Inválido

     /*
        comentario
        multilinea
     */ // Inválido
     ```
   - No se admiten sintaxis alternativas como `#`, `##`, `<!-- -->`, `///` ni `/** */`.
3. **Múltiples líneas de comentarios**:
   - Para documentar o comentar múltiples líneas consecutivas, cada línea debe prefijarse individualmente con `//`:
     ```text
     // primera línea de documentación
     // segunda línea de contexto
     // tercera línea de explicación
     ```
4. **Interacción con literales de texto**:
   - La secuencia `//` situada dentro de un string literal (`"https://example.com"`) no inicia un comentario; el reconocimiento del contenido del string tiene precedencia.
   - Una vez iniciado un comentario mediante `//` fuera de un string literal, todo el contenido hasta el salto de línea es ignorado como comentario, incluso si contiene comillas (`// "esto sigue siendo comentario"`).


### 13.2 Palabras reservadas (Keywords) y tokens literales reservados

1. **Principio de reserva efectiva**:
   > Una palabra es una *keyword* reservada en Evo-Script v0.1 única y exclusivamente si forma parte activa de la gramática y construcciones semánticas definidas en esta versión.
   - No se reservan palabras de forma preventiva para características hipotéticas o futuras provenientes de otros lenguajes o términos semánticos (palabras como `class`, `interface`, `trait`, `async`, `await`, `package`, `library`, `dependency`, `version`, `install`, `include`, `from`, `require`, `scope`, `prompt`, `context`, `active`, `session`, `with` o `refresh` **NO** son keywords en v0.1).
   - La extensión de archivo reservada (`.evo`) no constituye palabra clave del lenguaje (las palabras `library` o `package` no son keywords en v0.1).
2. **Catálogo de palabras estructurales reservadas (Structural Keywords)**:
   Las siguientes palabras constituyen las palabras reservadas estructurales oficiales de Evo-Script v0.1:
   - `artifact`: Declaración de membresía física de artefactos en manifiestos de librería (`.elib`).
   - `let`: Declaración de bindings locales inmutables.
   - `struct`: Definición de tipos de estructura de datos.
   - `enum`: Definición de tipos de enumeración y variantes.
   - `fn`: Declaración de funciones.
   - `public`: Modificador de visibilidad pública para funciones.
   - `private`: Modificador de visibilidad privada para funciones.
   - `return`: Sentencia de retorno de valor en funciones.
   - `when`: Expresión de correspondencia y selección de variantes de enum.
   - `esig`: Declaración de contratos de firmas públicas (`.esig`).
   - `import`: Declaración de dependencia estructural modular.
   - `as`: Declaración de alias local para importaciones.
   - `module`: Declaración de catálogo y frontera de módulo (`.emod`).
   - `publish`: Declaración de publicación de firmas y tipos en módulos (`.emod`).
   - `bind`: Declaración de correspondencia de composición funcional (`.root`).
   - `to`: Delimitador de destino en declaraciones `bind ... to ...` (`.root`).
   - `entry`: Declaración de selección de entrada inicial (`.main`).
   - `this`: Marcador de posición contextual en pipelines.
   - `use`: Activación y cambio de contexto semántico (Scope) en composiciones y pipelines.
3. **Tokens literales reservados (Reserved Literal Tokens)**:
   - `true`: Literal booleano de valor verdadero.
   - `false`: Literal booleano de valor falso.
   - Los tokens `true` y `false` son **Boolean Literal Tokens**, no Structural Keywords; sin embargo, están lexicalmente reservados y no pueden emplearse como identificadores.
4. **Prohibición de uso como identificadores**:
   - Ninguna palabra estructural reservada ni token literal reservado puede utilizarse como identificador de función, firma, binding, parámetro, campo, tipo, variante de enum o módulo.
   - **Ejemplos conceptuales inválidos**:
     ```text
     let int return = 10; // Inválido: 'return' es una palabra reservada
     let int true = 10;   // Inválido: 'true' es un token literal reservado
     ```


### 13.3 Codificación del código fuente y espacios en blanco (Whitespace)

1. **Codificación oficial del código fuente (Source Encoding)**:
   - Todo archivo de código fuente Evo-Script v0.1 (`.efn`, `.esig`, `.estc`, `.enum`, `.emod`, `.root`, `.main`) se interpreta de forma canónica como texto codificado en **UTF-8 sin marca de orden de bytes (UTF-8 without BOM)**.
   - La presencia de BOM no forma parte del formato canónico del lenguaje.
2. **Espacios en blanco reconocidos (Whitespace)**:
   - Evo-Script v0.1 reconoce formalmente como caracteres de espacio en blanco para separación léxica:
     - Espacio (`space`, ASCII `0x20`)
     - Tabulación horizontal (`tab`, `\t`, ASCII `0x09`)
     - Salto de línea (`line feed`, `LF`, `\n`, ASCII `0x0A`)
     - Retorno de carro (`carriage return`, `CR`, `\r`, ASCII `0x0D`)
3. **Función del espacio en blanco**:
   - El espacio en blanco actúa exclusivamente como separador de tokens contiguos cuando sea necesario para evitar ambigüedades.
   - No produce valores semánticos (`Values`) ni posee significado en tiempo de ejecución.
   - Las siguientes dos declaraciones son semánticamente equivalentes:
     ```text
     let int age = 43;
     ```
     ```text
     let
         int
         age
         =
         43
         ;
     ```
4. **Insensibilidad a la indentación**:
   - Evo-Script **NO es sensible a la indentación** (*not indentation-sensitive*).
   - Los espacios o tabulaciones al inicio de línea no abren ni cierran scopes, no crean bloques, no alteran la precedencia y no influyen en la correspondencia estructural.
   - Todos los bloques estructurales se delimitan explícitamente mediante llaves (`{ ... }`).
5. **Comportamiento de saltos de línea (Newlines)**:
   - Fuera de comentarios de una línea (`//`) y literales de texto (`"..."`), los saltos de línea funcionan como espacios en blanco ordinarios.
   - En comentarios de línea, el salto de línea físico finaliza el comentario.
   - En literales de texto, un salto de línea físico entre comillas es inválido.


### 13.4 Gramática de identificadores (Identifier Grammar)

1. **Definición formal**:
   La gramática léxica de identificadores en Evo-Script v0.1 se define formalmente como:

   ```text
   ascii_lowercase_letter
       := "a".."z"

   ascii_uppercase_letter
       := "A".."Z"

   ascii_letter
       := ascii_lowercase_letter
        | ascii_uppercase_letter

   digit
       := "0".."9"

   identifier
       := ascii_letter (ascii_letter | digit | "_")*
   ```

2. **Primer carácter obligatorio**:
   - Un identificador **debe** comenzar obligatoriamente con una letra ASCII (`a-z` o `A-Z`).
   - No se admite el carácter guion bajo (`_`) ni dígitos numéricos (`0-9`) como primer carácter.
   - **Ejemplos válidos**: `worker`, `worker2`, `worker_id`, `SearchResult`.
   - **Ejemplos inválidos**:
     ```text
     2worker      // Inválido: inicia con dígito
     _worker      // Inválido: no existe leading underscore
     __worker     // Inválido: no existe leading underscore
     _            // Inválido: no se admite guion bajo solitario
     ```
3. **Uso de guion bajo (`_`)**:
   - El carácter guion bajo (`_`) solo puede aparecer a partir de la segunda posición del identificador (`worker_id`, `current_user`, `Search_Result`).
4. **Exclusividad ASCII**:
   - Los identificadores en Evo-Script v0.1 se limitan estrictamente a letras ASCII.
   - **NO se admiten caracteres Unicode** dentro de identificadores.
   - **Ejemplos inválidos como identificadores**:
     ```text
     niño       // Inválido: carácter Unicode 'ñ'
     año        // Inválido: carácter Unicode 'ñ'
     México     // Inválido: carácter Unicode 'é'
     búsqueda   // Inválido: carácter Unicode 'ú'
     χρήστης    // Inválido: alfabeto griego
     日本       // Inválido: caracteres CJK
     ```
   - La codificación UTF-8 del archivo fuente permite caracteres Unicode en el contenido de strings o comentarios, pero **no** en los identificadores del programa.
5. **Sensibilidad a mayúsculas y minúsculas (Case-Sensitivity)**:
   - Los identificadores en Evo-Script son estrictamente **case-sensitive**.
   - `worker`, `Worker` y `WORKER` constituyen tres identificadores distintos y no intercambiables.
   - No existe resolución insensible a mayúsculas ni normalización automática de casing.
6. **Separación entre gramática léxica y convenciones de nombres**:
   - La gramática de identificadores define qué secuencias de caracteres son léxicamente válidas para el lenguaje.
   - Las convenciones de nombres (Sección 13.7) definen el estilo nominal requerido para cada clase de símbolo semántico (`PascalCase`, `snake_case`).


### 13.5 Literales booleanos (Boolean Literals)

1. **Definición formal**:
   Evo-Script v0.1 define exactamente dos literales booleanos:

   ```text
   boolean_literal
       := "true"
        | "false"
   ```

2. **Tipado estricto**:
   - `true` produce un valor de tipo semántico `bool`.
   - `false` produce un valor de tipo semántico `bool`.
   - No existe tipado contextual que permita a un literal booleano producir `int`, `string` o `dynamic`.
3. **Minúsculas obligatorias**:
   - Los literales booleanos deben escribirse estrictamente en minúsculas (`true`, `false`).
   - Debido a la sensibilidad a mayúsculas (*case-sensitivity*), formas como `True`, `False`, `TRUE`, `FALSE` o `TrUe` **NO** son literales booleanos.
4. **Ausencia de literales alternativos**:
   - No existen en Evo-Script literales como `yes`, `no`, `on`, `off`, ni valores numéricos `1` o `0` interpretados como booleanos.
   - No existe conversión implícita entre números y booleanos.


### 13.6 Literales de texto (String Literals) y secuencias de escape

1. **Forma canónica**:
   Evo-Script v0.1 posee exactamente una forma oficial de literal de texto:

   ```text
   "texto"
   ```

   El literal se delimita exclusivamente mediante comillas dobles (`"`).
2. **Ausencia de sintaxis alternativas**:
   - **No existen comillas simples**: `'texto'` no es un literal de texto válido (Evo-Script v0.1 no define tipo `char` ni literales de carácter).
   - **No existen cadenas crudas (raw strings)**: No se admite sintaxis como `r"..."`, `r#"..."#` ni `` `...` ``.
   - **No existe interpolación de cadenas**: No se admite sintaxis como `$"..."`, `f"..."` ni `"${...}"`. Toda concatenación o formato debe realizarse mediante funciones y capacidades normales.
3. **Contenido Unicode directo en UTF-8**:
   - Un string literal puede contener directamente cualquier carácter Unicode válido codificado en UTF-8:
     ```text
     let string pais = "México";
     let string autor = "José";
     let string termino = "niño";
     let string saludo = "你好";
     let string pais_origen = "日本";
     ```
4. **Secuencias de escape oficiales**:
   Evo-Script v0.1 soporta **única y exclusivamente** las siguientes 5 secuencias de escape:

   | Secuencia | Representación semántica |
   | :--- | :--- |
   | `\"` | Comilla doble (`"`) |
   | `\\` | Barra invertida (`\`) |
   | `\n` | Salto de línea (`LF`, `0x0A`) |
   | `\r` | Retorno de carro (`CR`, `0x0D`) |
   | `\t` | Tabulación horizontal (`TAB`, `0x09`) |

   **Ejemplos válidos**:
   ```text
   let string saludo = "Hola\nMundo";
   let string cita = "Dijo \"hola\" a todos";
   let string ruta = "C:\\documents\\file.txt";
   let string columnas = "uno\tdos\ttres";
   ```
5. **Escapes desconocidos inválidos**:
   - Toda secuencia de escape no reconocida (por ejemplo, `\q`, `\z`, `\a`, `\e`) es **inválida** y produce un error léxico.
   - Los escapes desconocidos no se preservan literalmente ni se ignoran silenciosamente.
6. **Ausencia de sintaxis de escape Unicode numérico**:
   - Evo-Script v0.1 **no define** secuencias de escape numéricas como `\u0041`, `\u{0041}`, `\x41` ni `\U00000041`.
   - Los caracteres Unicode se escriben directamente como texto UTF-8 dentro del literal.
7. **Prohibición de saltos de línea físicos en el código fuente**:
   - Un string literal **no puede** contener saltos de línea físicos entre sus comillas delimitadoras.
   - El siguiente código es **inválido**:
     ```text
     let string texto = "Hola
     Mundo"; // Inválido: salto de línea físico en string literal
     ```
   - Para representar un salto de línea en el valor del string, debe utilizarse la secuencia de escape `\n`:
     ```text
     let string texto = "Hola\nMundo"; // Válido
     ```
8. **Delimitación y terminación**:
   - Un string literal comienza con una comilla doble `"` y concluye en la siguiente comilla doble `"` no escapada.
   - La secuencia `\"` no finaliza el string literal; representa una comilla doble dentro del contenido del texto.
9. **Tipo semántico y string vacío**:
   - Todo literal de texto produce un valor de tipo semántico **`string`**.
   - La cadena vacía `""` es un string literal válido de longitud cero de tipo `string`. No representa ausencia ni valor `null` (Evo-Script no posee `null`).


### 13.7 Convenciones de nombres (Naming Conventions)

Evo-Script distingue formalmente entre los identificadores semánticos del código y los nombres físicos de los archivos en el sistema de archivos:

```text
Semantic identifiers (código) != Physical artifact filenames (archivos)
```

#### 13.7.1 Identificadores semánticos en el código

1. **`PascalCase` para tipos de datos**:
   - Todo tipo definido por el usuario o programa (`struct`, `enum`) se nombra en `PascalCase`.
   - **Ejemplos**: `Worker`, `SearchResult`, `ApplicationState`, `UserAddress`, `InitResult`.
2. **`PascalCase` para variantes de enumeraciones**:
   - Toda variante de un `enum` se nombra en `PascalCase`.
   - **Ejemplos**: `Found`, `NotFound`, `Ready`, `InvalidRequest`, `ServiceUnavailable`, `Ok`, `Error`.
3. **`snake_case` para funciones y firmas**:
   - Toda Function Implementation (`public fn`, `private fn`) se nombra en `snake_case`.
   - Toda Signature (`.esig`) se nombra en `snake_case`.
   - **Ejemplos**: `search_worker`, `save_file`, `process_request`, `calculate_total`, `washes_clothes`.
4. **`snake_case` para bindings locales**:
   - Todo binding declarado mediante `let` se nombra en `snake_case`.
   - **Ejemplos**: `worker`, `worker_id`, `current_user`, `search_result`, `application_state`.
5. **`snake_case` para parámetros de funciones**:
   - **Value Parameters**: Los nombres locales de parámetros de datos se declaran en `snake_case`:
     ```text
     public fn search_worker(int worker_id) -> SearchResult
     ```
   - **Signature Dependency Parameters**: Los nombres locales de parámetros de dependencia de firma se declaran en `snake_case`:
     ```text
     public fn initialize(window::open open_window, config::load load_config) -> InitResult
     ```
   - **Distinción terminológica entre Parámetro y Argumento**:
     - `worker_id` en la cabecera `fn search_worker(int worker_id)` es el nombre del **Parámetro** (`snake_case`).
     - `current_id` en la llamada `search_worker(current_id)` es la expresión/binding utilizada como **Argumento** (`snake_case`).
     - Evo-Script v0.1 no soporta argumentos con nombre (`named arguments`).
6. **`snake_case` para campos**:
   - Campos de estructuras `struct`: se nombran en `snake_case` (`id`, `worker_id`, `first_name`, `last_name`, `creation_date`).
   - Campos nombrados en variantes estructuradas de `enum`: se nombran en `snake_case` (`error_message`, `error_code`, `user_id`).
7. **`snake_case` para módulos**:
   - El identificador lógico de un módulo (`module nombre { ... }`) se declara en `snake_case`.
   - **Ejemplos**: `values`, `employee_values`, `file_system`, `laundry`.

#### 13.7.2 Nombres físicos de artefactos (`kebab-case`)

1. **Convención `kebab-case` para el sistema de archivos**:
   - Los nombres físicos de los archivos de artefactos Evo-Script utilizan **`kebab-case`** (palabras en minúsculas separadas por guiones `-` cuando constan de múltiples términos).
   - Los archivos de una sola palabra (`worker.estc`, `washer.efn`, `application.root`, `application.main`) son conformes a la convención kebab-case al no requerir separador.
   - **Ejemplos**:
     - `search-result.enum`
     - `worker.estc`
     - `search-worker.esig`
     - `search-database.efn`
     - `employee-values.emod`
     - `application.root`
     - `application.main`
2. **Restricción estricta de `kebab-case` al sistema de archivos**:
   - `kebab-case` aplica **única y exclusivamente a nombres de archivos físicos**.
   - **NO** se utiliza `kebab-case` para identificadores evaluables dentro del código Evo-Script, ya que el carácter `-` está reservado como operador aritmético de resta.
   - **Ejemplo**: `search_worker` es el identificador semántico válido de la función; `search-worker.efn` es el nombre del archivo físico.
3. **Correspondencia nominal entre símbolos semánticos y artefactos físicos**:
   A continuación se ilustra la correspondencia entre la convención de nombres semánticos y físicos:

   | Símbolo semántico | Convención semántica | Artefacto físico | Convención física |
   | :--- | :--- | :--- | :--- |
   | `SearchResult` (enum) | `PascalCase` | `search-result.enum` | `kebab-case` |
   | `Worker` (struct) | `PascalCase` | `worker.estc` | `kebab-case` |
   | `search_worker` (signature) | `snake_case` | `search-worker.esig` | `kebab-case` |
   | `search_database` (function) | `snake_case` | `search-database.efn` | `kebab-case` |
   | `employee_values` (module) | `snake_case` | `employee-values.emod` | `kebab-case` |
   | `application` (root) | `snake_case` | `application.root` | `kebab-case` |
   | `application` (main) | `snake_case` | `application.main` | `kebab-case` |

4. **Ausencia de algoritmo automático de resolución física en v0.1**:
   - Esta correspondencia expresa la **convención oficial de nombrado**, pero **NO** define un algoritmo automático de transformación (`PascalCase -> kebab-case` o `snake_case -> kebab-case`).
   - Las reglas exactas de localización y resolución física de módulos en el sistema de archivos permanecen delimitadas para el bloque de resolución física de módulos.


### 13.8 Cierre formal del bloque léxico en Evo-Script v0.1

Con las formalizaciones establecidas en este capítulo y en las secciones numéricas correspondientes, el modelo léxico fundamental de Evo-Script v0.1 queda plenamente definido y cerrado:

1. **Aspectos formalizados y cerrados**:
   - Delimitación de comentarios (`//`) y ausencia de comentarios multilínea.
   - Principio de palabras reservadas efectivas y catálogo normativo de keywords estructurales.
   - Tokens literales reservados (`true`, `false`).
   - Codificación obligatoria del código fuente en UTF-8 sin BOM.
   - Caracteres de espacio en blanco reconocidos e insensibilidad a la indentación.
   - Gramática formal y exclusividad ASCII de identificadores con sensibilidad a mayúsculas/minúsculas.
   - Gramática y tipado estricto de literales booleanos.
   - Gramática, secuencias de escape exactas y delimitación de literales de texto (`string`).
   - Gramática de literales numéricos (`int`, `float`).
   - Convenciones de nombres semánticos (`PascalCase`, `snake_case`) y físicos (`kebab-case`).
2. **Delimitación de alcance**:
   - El cierre formal del bloque léxico no constituye la congelación total de la especificación general de Evo-Script v0.1, continuando el desarrollo de los bloques semánticos y modulares restantes conforme a la metodología establecida.


## 14. Ejemplos canónicos oficiales de Evo-Script v0.1 (Canonical v0.1 Examples)

Esta sección consolida el ejemplo oficial de referencia canónica para proyectos estructurados en Evo-Script v0.1, denominado **Canonical Copy Application (`copy-app`)**, junto con sus recorridos de resolución, llamadas semánticas, modelos de desacoplamiento arquitectónico y microejemplos complementarios.

El propósito de estos ejemplos es servir como prueba integral de coherencia formal del lenguaje, demostrando el funcionamiento armónico y conjunto de todos los artefactos, extensiones y reglas cerradas en la especificación.


### 14.1 Arquitectura canónica de aplicación Evo

El ecosistema Evo y Evo-Script modelan las aplicaciones mediante una arquitectura limpia funcional rigurosamente desacoplada:

```text
Use Case (Contrato de aplicación)
    ↓
  Agent (Orquestador)
    ↓
Resolver (Adaptación y resolución)
    ↓
 Contract (Frontera hacia infraestructura)
    │
    ├── Provider (Materializa la operación)
    │       ↓
    └── Requester (Puerto de salida)
            ↓
        Responder (Implementaciones concretas)
```

1. **Responsabilidades arquitectónicas**:
   - **Use Case**: Define formalmente la operación de aplicación (`input port`).
   - **Agent**: Orquesta la ejecución de la operación y transporta las dependencias funcionales hacia el Resolver.
   - **Resolver**: Conecta la intención semántica de la operación con los contratos técnicos requeridos, invocando el Contract y reenviando el Requester.
   - **Contract**: Frontera formal de salida hacia la infraestructura técnica (`infrastructure port`). Recibe el Requester, materializa la operación y ejecuta la notificación de salida.
   - **Provider**: Implementación técnica concreta del Contract (filesystem, database, network, etc.).
   - **Requester**: Frontera formal de salida hacia consumidores de la aplicación (`output port`).
   - **Responder**: Implementación concreta del Requester (UI, notificaciones, logs, persistencia de eventos, etc.).
2. **Relación entre Use Case, Requesters y Contracts**:
   Los Requesters y Contracts son dependencias intrínsecas de la arquitectura del Use Case. En la arquitectura Rust de Evo (`evo-shell`), un puntero de función de Use Case recibe conceptualmente sus parámetros de datos junto con los punteros de función de sus Requesters y Contracts:
   ```text
   UseCase(data, requester_fn, contract_fn)
   ```
3. **Diferencia fundamental entre Rust y Evo-Script**:
   - En Rust, los punteros de función son valores ordinarios que pueden pasarse como argumentos de función.
   - En Evo-Script v0.1, las firmas (`Signatures`) **NO son valores (`Values`)**, no son funciones de primer orden y no pueden transportarse como datos genéricos.
   - Evo-Script representa estas capacidades explícitamente mediante **`Signature Dependency Parameters`** (`requesters::copy_completed request`, `contracts::copy copy`), permitiendo el reenvío estricto de dependencias (`dependency forwarding`) entre capas funcionales.
   - El archivo `application.root` resuelve estáticamente qué implementación física satisface cada Signature en todo el proyecto.
   - El grafo arquitectónico completo se preserva fielmente sin introducir closures, lambdas ni transporte manual de punteros a función como datos.


### 14.2 Árbol físico del proyecto canónico (`copy-app`)

A continuación se presenta la estructura física oficial del proyecto:

```text
copy-app/
├── application.elib
├── application.root
├── application.main
├── main.efn
│
├── definitions/
│   ├── use_cases/
│   │   ├── use-cases.emod
│   │   └── copy-file.esig
│   │
│   ├── requesters/
│   │   ├── requesters.emod
│   │   └── copy-completed.esig
│   │
│   ├── resolvers/
│   │   ├── resolvers.emod
│   │   └── resolve-copy.esig
│   │
│   ├── contracts/
│   │   ├── contracts.emod
│   │   └── copy.esig
│   │
│   └── domain/
│       ├── domain.emod
│       ├── file-view.estc
│       └── copy-result.enum
│
├── agents/
│   └── copier.efn
│
├── resolvers/
│   └── copy-resolver.efn
│
├── providers/
│   └── std-copy.efn
│
└── responders/
    └── copy-completed.efn
```

**Métricas y notas estructurales sobre la organización física**:
1. **Conteo exacto de artefactos**:
   - El árbol del proyecto contiene exactamente **19 archivos físicos** (incluyendo el manifiesto `application.elib`).
   - El manifiesto `application.elib` declara unívocamente exactamente **18 sentencias `artifact`** registradas.
2. **Entrada de aplicación (`main.efn`)**:
   La regla normativa de `application.main` estipula que la función de entrada no debe recibir Value Parameters (`zero Value Parameters`), pero puede recibir **cero o más Signature Dependency Parameters**. `main.efn` recibe las dependencias `requesters::copy_completed` y `contracts::copy` (resueltas por `.root`) y las transporta hacia el caso de uso `use_cases::copy_file` junto con las rutas literales de datos.
3. **Complejidad estructural bajo demanda**:
   Las firmas `.esig` intermedias (como `resolvers::resolve_copy`) aparecen exclusivamente cuando existe distribución entre archivos separados (`.efn` distintos). Si dos componentes residieran en el mismo archivo, no se requeriría una firma ceremonial.
4. **Nombres de directorios como arquitectura de usuario**:
   Los nombres de carpetas (`definitions`, `use_cases`, `requesters`, `resolvers`, `contracts`, `domain`, `agents`, `providers`, `responders`) son elecciones organizacionales de la aplicación y **no son palabras clave ni conceptos normativos impuestos por Evo-Script**.


### 14.3 Manifiesto físico de artefactos (`application.elib`)

El archivo `application.elib` declara exhaustiva y unívocamente los 18 artefactos que componen el `Physical Artifact Universe`:

```text
artifact "application.root";
artifact "application.main";
artifact "main.efn";

artifact "definitions/use_cases/use-cases.emod";
artifact "definitions/use_cases/copy-file.esig";

artifact "definitions/requesters/requesters.emod";
artifact "definitions/requesters/copy-completed.esig";

artifact "definitions/resolvers/resolvers.emod";
artifact "definitions/resolvers/resolve-copy.esig";

artifact "definitions/contracts/contracts.emod";
artifact "definitions/contracts/copy.esig";

artifact "definitions/domain/domain.emod";
artifact "definitions/domain/file-view.estc";
artifact "definitions/domain/copy-result.enum";

artifact "agents/copier.efn";
artifact "resolvers/copy-resolver.efn";
artifact "providers/std-copy.efn";
artifact "responders/copy-completed.efn";
```


### 14.4 Módulos lógicos (`.emod`)

Cada archivo `.emod` declara la identidad lógica formal de su módulo y su catálogo público de publicación (publicando exclusivamente `.esig`, `.estc` y `.enum`, nunca `.efn`):

- **`definitions/use_cases/use-cases.emod`**:
  ```text
  module use_cases {
      publish copy_file;
  }
  ```

- **`definitions/requesters/requesters.emod`**:
  ```text
  module requesters {
      publish copy_completed;
  }
  ```

- **`definitions/resolvers/resolvers.emod`**:
  ```text
  module resolvers {
      publish resolve_copy;
  }
  ```

- **`definitions/contracts/contracts.emod`**:
  ```text
  module contracts {
      publish copy;
  }
  ```

- **`definitions/domain/domain.emod`**:
  ```text
  module domain {
      publish FileView;
      publish CopyResult;
  }
  ```


### 14.5 Tipos compartidos de dominio (`.estc` y `.enum`)

- **`definitions/domain/file-view.estc`** (Estructura inmutable de archivo):
  ```text
  struct FileView {
      string path;
      string content;
  }
  ```

- **`definitions/domain/copy-result.enum`** (Resultado de dominio):
  ```text
  import domain::FileView;

  enum CopyResult {
      Success(FileView)
      Failed(string)
  }
  ```


### 14.6 Firmas públicas (`.esig`)

- **`definitions/use_cases/copy-file.esig`** (Firma del caso de uso / Input Port):
  ```text
  import domain::CopyResult;
  import requesters::copy_completed;
  import contracts::copy;

  esig copy_file(
      string source_path,
      string destination_path,
      requesters::copy_completed request,
      contracts::copy copy
  ) -> CopyResult;
  ```

- **`definitions/requesters/copy-completed.esig`** (Firma del Requester / Output Port):
  ```text
  import domain::CopyResult;

  esig copy_completed(CopyResult result) -> CopyResult;
  ```
  *(Nota: Al carecer Evo-Script v0.1 de tipo `void`/`Unit`, el Requester retorna `CopyResult`, permitiendo al invocador ejecutarlo como Operation Statement y descartar el valor).*

- **`definitions/resolvers/resolve-copy.esig`** (Firma de resolución intermedia):
  ```text
  import domain::CopyResult;
  import requesters::copy_completed;
  import contracts::copy;

  esig resolve_copy(
      string source_path,
      string destination_path,
      requesters::copy_completed request,
      contracts::copy copy
  ) -> CopyResult;
  ```

- **`definitions/contracts/copy.esig`** (Firma del contrato técnico de infraestructura):
  ```text
  import domain::CopyResult;
  import requesters::copy_completed;

  esig copy(
      string source_path,
      string destination_path,
      requesters::copy_completed request
  ) -> CopyResult;
  ```


### 14.7 Orquestador / Agente (`agents/copier.efn`)

El agente `copier.efn` orquesta el caso de uso satisfaciendo `use_cases::copy_file` y transportando las dependencias funcionales requeridas hacia el Resolver:

```text
import domain::CopyResult;
import use_cases::copy_file;
import requesters::copy_completed;
import contracts::copy;
import resolvers::resolve_copy;

public fn copy_file(
    string source_path,
    string destination_path,
    requesters::copy_completed request,
    contracts::copy copy
) -> CopyResult
    : use_cases::copy_file
{
    let CopyResult result = resolve_copy(
        source_path,
        destination_path,
        request,
        copy
    );

    return result;
}
```


### 14.8 Implementación del Resolver (`resolvers/copy-resolver.efn`)

El resolver `copy-resolver.efn` implementa la frontera de adaptación invocando el contrato de infraestructura `contracts::copy` y reenviándole el Requester:

```text
import domain::CopyResult;
import resolvers::resolve_copy;
import requesters::copy_completed;
import contracts::copy;

public fn resolve_copy(
    string source_path,
    string destination_path,
    requesters::copy_completed request,
    contracts::copy copy
) -> CopyResult
    : resolvers::resolve_copy
{
    let CopyResult result = copy(
        source_path,
        destination_path,
        request
    );

    return result;
}
```


### 14.9 Proveedor de infraestructura (`providers/std-copy.efn`)

El proveedor satisface el contrato técnico `contracts::copy`. Materializa la copia, construye `CopyResult::Success`, ejecuta el Requester y finaliza retornando el resultado:

```text
import domain::FileView;
import domain::CopyResult;
import contracts::copy;
import requesters::copy_completed;

public fn copy(
    string source_path,
    string destination_path,
    requesters::copy_completed request
) -> CopyResult
    : contracts::copy
{
    let FileView copied_file = FileView {
        path: destination_path,
        content: "canonical copied content"
    };
    let CopyResult result = CopyResult::Success(copied_file);

    request(result);

    return result;
}
```


### 14.10 Responder de aplicación (`responders/copy-completed.efn`)

El responder satisface la firma de salida `requesters::copy_completed`, implementando la reacción concreta al evento de finalización:

```text
import domain::CopyResult;
import requesters::copy_completed;

public fn copy_completed(CopyResult result) -> CopyResult
    : requesters::copy_completed
{
    return result;
}
```


### 14.11 Functional Composition Root (`application.root`)

El archivo `application.root` compone la aplicación vinculando de forma unívoca y estática cada firma abstracta a su implementación física:

```text
bind use_cases::copy_file to "agents/copier.efn";
bind resolvers::resolve_copy to "resolvers/copy-resolver.efn";
bind contracts::copy to "providers/std-copy.efn";
bind requesters::copy_completed to "responders/copy-completed.efn";
```


### 14.12 Entrada de aplicación (`application.main` y `main.efn`)

- **`application.main`**:
  ```text
  entry "main.efn";
  ```

- **`main.efn`** (Punto de entrada de la aplicación):
  ```text
  import domain::CopyResult;
  import use_cases::copy_file;
  import requesters::copy_completed;
  import contracts::copy;

  public fn main(
      requesters::copy_completed request,
      contracts::copy copy
  ) -> CopyResult {
      let string source = "/documents/source.txt";
      let string destination = "/documents/copy.txt";
      let CopyResult result = copy_file(
          source,
          destination,
          request,
          copy
      );

      return result;
  }
  ```


### 14.13 Recorrido de resolución física (Physical Resolution Walkthrough)

La secuencia determinista de análisis y resolución estática ejecutada por el host/compilador para `copy-app` es la siguiente:

1. **Carga de la Active Library**: El host selecciona `copy-app/application.elib` como la librería activa (`Active Library`). Su directorio base es `copy-app/` (`Library Base Directory`).
2. **Construcción del Physical Artifact Universe**: Se validan sintáctica y físicamente las 18 sentencias `artifact "..."`, registrándolas en la `Library Artifact Table`.
3. **Determinación de Fronteras Modulares (`Physical Module Boundaries`)**:
   - `definitions/use_cases/use-cases.emod` ancla la frontera modular de `definitions/use_cases/`.
   - `definitions/requesters/requesters.emod` ancla la frontera modular de `definitions/requesters/`.
   - `definitions/resolvers/resolvers.emod` ancla la frontera modular de `definitions/resolvers/`.
   - `definitions/contracts/contracts.emod` ancla la frontera modular de `definitions/contracts/`.
   - `definitions/domain/domain.emod` ancla la frontera modular de `definitions/domain/`.
4. **Construcción de Module Artifact Tables y Public Symbol Tables**:
   - `copy-file.esig` se asocia al módulo `use_cases`, registrando `copy_file` como símbolo público.
   - `copy-completed.esig` se asocia al módulo `requesters`, registrando `copy_completed` como símbolo público.
   - `resolve-copy.esig` se asocia al módulo `resolvers`, registrando `resolve_copy` como símbolo público.
   - `copy.esig` se asocia al módulo `contracts`, registrando `copy` como símbolo público.
   - `file-view.estc` y `copy-result.enum` se asocian al módulo `domain`, registrando `FileView` y `CopyResult` como símbolos públicos.
5. **Validación del Grafo de Tipos y Cierres**: Se verifica que `domain::CopyResult` y `domain::FileView` sean acíclicos y cumplan `Public Type Closure`.
6. **Resolución de Bindings de `.root`**: Se validan los 4 bindings de `application.root`. Cada target `.efn` existe en el universo registrado, tiene exactamente una función pública y satisface (`:`) la firma vinculada.
7. **Validación de la Entrada (`.main`)**: Se comprueba que `main.efn` pertenezca al `Physical Artifact Universe` y su función pública declare cero Value Parameters y dependencias de firma válidas satisfechas por `.root`.
8. **Inicio del Application Main Loop**: Al completarse la validación estática sin errores, el host ejecuta la entrada e inicia el ciclo de vida de la aplicación.


### 14.14 Recorrido de llamadas semánticas (Semantic Call Walkthrough)

Evo-Script distingue formalmente entre la resolución composicional estática previa a la evaluación y el flujo de llamadas durante la ejecución:

#### 14.14.1 Pre-evaluación composicional (Estática)
1. El compilador/runtime procesa `application.root` y verifica que cada `bind` (`use_cases::copy_file`, `resolvers::resolve_copy`, `contracts::copy`, `requesters::copy_completed`) resuelva unívocamente a una Function Implementation `.efn` válida registrada en `application.elib`.
2. Se resuelve el grafo estático de dependencias inyectables para `main.efn` y sus llamadas descendentes. `application.root` **no actúa como nodo ejecutable ni service locator en tiempo de ejecución**.

#### 14.14.2 Evaluación en tiempo de ejecución (Runtime)
El flujo directo de invocación funcional opera de la siguiente manera:

```text
Host / Runtime (inicia Application Main Loop con dependencias inyectadas estáticamente)
    ↓
main.efn (public fn main)
    ↓ invoca copy_file(source, destination, request, copy)
agents/copier.efn (public fn copy_file)
    ↓ invoca resolve_copy(source, destination, request, copy)
resolvers/copy-resolver.efn (public fn resolve_copy)
    ↓ invoca copy(source, destination, request)
providers/std-copy.efn (public fn copy)
    ├─► materializa la copia y construye CopyResult::Success
    │
    └─► invoca request(result)
            ↓
        responders/copy-completed.efn (public fn copy_completed)
            ↓ notifica evento y retorna
    providers/std-copy.efn recibe retorno y finaliza
    ↓ retorna CopyResult::Success
resolvers/copy-resolver.efn recibe CopyResult::Success y finaliza
    ↓ retorna CopyResult::Success
agents/copier.efn recibe CopyResult::Success y finaliza
    ↓ retorna CopyResult::Success
main.efn retorna CopyResult::Success
    ↓
Application Main Loop recibe el resultado y administra el ciclo de vida
```


### 14.15 Recorrido conceptual de Scope y Providers (Scope Relationship Walkthrough)

> [!NOTE]
> Este apartado ilustra el flujo conceptual de interacción entre Providers y Scopes en el ecosistema Evo. No constituye sintaxis normativa de adquisición en v0.1.

```text
Active .elib
    ↓ (hace disponible físicamente al Provider en el proyecto)
Provider físico disponible (ej. providers/std-copy.efn)
    ↓ (capacidad provide_scope suministrada por Evo-Shell)
Scope provisto (vista contextual prestada: Scope<'scope>)
    ↓ (instrucción 'use' en composición o pipeline)
Active Scope (contexto activo de la composición)
    ↓
Capacidades operativas / Datos producidos
    ↓
Transformaciones de consulta (EvoQ: filter, select, etc.)
```


### 14.16 Microejemplos canónicos adicionales

#### 14.16.1 Signature Dependency Parameters vs Value Parameters

Este microejemplo ilustra la distinción formal entre un parámetro de valor ordinario y un parámetro de inyección de firma:

```text
import values::search;
import values::SearchResult;

// 'id' es un Value Parameter (int).
// 'search' es un Signature Dependency Parameter (values::search).
public fn process(int id, values::search search) -> SearchResult {
    let SearchResult result = search(id);

    return result;
}
```

- `int id`: Parámetro de valor ordinario transportado por el evaluador.
- `values::search search`: Dependencia inyectada mediante contrato de firma que no constituye un tipo función de primer orden ni un puntero genérico.

#### 14.16.2 Importación con alias local (`as`)

```text
import hr::Worker as HrWorker;
import sales::Worker as SalesWorker;

public fn compare_workers(HrWorker a, SalesWorker b) -> bool {
    let bool same_id = (a.id == b.id);

    return same_id;
}
```

#### 14.16.3 Pipeline funcional con `|>` y marcador contextual `this`

En un pipeline de Evo-Script, las operaciones unarias (aridad 1) se invocan sin paréntesis ni marcador, mientras que las operaciones con aridad $\ge 2$ utilizan `this` para posicionar explícitamente el valor transportado:

```text
import utils::format_text;
import utils::append_suffix;

public fn build_label(string name) -> string {
    let string label = name
        |> format_text
        |> append_suffix(this, "_processed");

    return label;
}
```


### 14.17 Tabla de características y reglas demostradas (Rules Demonstrated)

A continuación se resume la correspondencia entre los componentes del ejemplo canónico y las características formales de Evo-Script v0.1:

| Característica / Regla | Artefacto / Componente del ejemplo | Estado normativo |
| :--- | :--- | :--- |
| **`.elib` (Physical Artifact Manifest)** | `copy-app/application.elib` (18 artifacts) | Demostrado formalmente |
| **`.root` (Functional Composition Root)** | `copy-app/application.root` (4 bindings) | Demostrado formalmente |
| **`.main` (Application Entry)** | `copy-app/application.main` | Demostrado formalmente |
| **`.emod` (Module Surface)** | `use-cases.emod`, `requesters.emod`, `resolvers.emod`, `contracts.emod`, `domain.emod` | Demostrado formalmente |
| **`.esig` (Public Function Signature)** | `copy-file.esig`, `copy-completed.esig`, `resolve-copy.esig`, `copy.esig` | Demostrado formalmente |
| **`.esig` con Signature Dependency Parameters** | `copy_file`, `resolve_copy`, `copy` en `.esig` | Demostrado formalmente |
| **`.estc` (Shared Struct Type)** | `file-view.estc` | Demostrado formalmente |
| **`.enum` (Shared Enum Type)** | `copy-result.enum` | Demostrado formalmente |
| **`.efn` (Function Implementation)** | `main.efn`, `copier.efn`, `copy-resolver.efn`, `std-copy.efn`, `copy-completed.efn` | Demostrado formalmente |
| **Declaración `artifact`** | En `application.elib` para cada uno de los 18 artefactos | Demostrado formalmente |
| **Declaración `module` e identidades lógicas** | `module use_cases`, `module requesters`, `module resolvers`, `module contracts`, `module domain` | Demostrado formalmente |
| **Declaración `publish`** | Publicación exclusiva de `.esig`, `.estc`, `.enum` | Demostrado formalmente |
| **Declaración `import` (Tipos y Firmas)** | `import module::Symbol;` en artefactos consumidores | Demostrado formalmente |
| **Satisfacción contractual (`:`)** | `: use_cases::copy_file`, `: resolvers::resolve_copy`, `: contracts::copy`, etc. | Demostrado formalmente |
| **Vinculación en `.root` (`bind ... to ...`)** | `bind module::signature to "path.efn";` | Demostrado formalmente |
| **Use Case -> Agent** | `use_cases::copy_file` vinculado a `agents/copier.efn` | Demostrado formalmente |
| **Agent -> Resolver** | `resolvers::resolve_copy` invocado por `copier.efn` reenviando dependencias | Demostrado formalmente |
| **Resolver -> Contract** | `contracts::copy` invocado por `copy-resolver.efn` reenviando Requester | Demostrado formalmente |
| **Contract/Provider -> Requester** | `requesters::copy_completed` invocado por `providers/std-copy.efn` | Demostrado formalmente |
| **Requester -> Responder** | `requesters::copy_completed` vinculado a `responders/copy-completed.efn` | Demostrado formalmente |
| **Reenvío estricto de dependencias (`Forwarding`)** | Reenvío de `request` y `copy` en `main` $\rightarrow$ `copier` $\rightarrow$ `resolver` $\rightarrow$ `provider` | Demostrado formalmente |
| **Entrada con Signature Dependency Parameters** | `main.efn` con `public fn main(request, copy) -> CopyResult` | Demostrado formalmente |
| **Construcción de Struct inmutable** | `FileView { path: ..., content: ... }` | Demostrado formalmente |
| **Variantes calificadas de Enum** | `CopyResult::Success(...)` | Demostrado formalmente |
| **Signature Dependency Parameters** | `process(int id, values::search search)` en microejemplo | Demostrado formalmente |
| **Import alias (`as`)** | `import hr::Worker as HrWorker;` en microejemplo | Demostrado formalmente |
| **Pipeline funcional (`|>`, unario y `this`)** | `name \|> format_text \|> append_suffix(this, "_processed")` | Demostrado formalmente |
| **Relación Provider -> Scope** | Modelo Provider $\to$ Scope $\to$ use $\to$ Active Scope | Recorrido conceptual (sin sintaxis de adquisición en v0.1) |
