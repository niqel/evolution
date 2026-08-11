# Arquitectura del Proyecto Evolution

Este documento especifica la arquitectura del proyecto **Evolution**, definiendo los límites conceptuales entre los proyectos/crates (`evo-script`, `evo-shell`, `providers`, `evo-shell-cli`) y la organización interna del motor semántico `evo-shell`.

---

## 1. Visión General y Fronteras del Sistema

El sistema Evolution procesa solicitudes desde el texto escrito por el usuario hasta su ejecución en infraestructura física siguiendo un flujo unidireccional y desacoplado:

```text
                         USER SOURCE
                              │
                              ▼
                         evo-script
                syntax / rules / language
                              │
                         semantic intent
                              │
                              ▼
                     evo-shell Use Cases
                              │
                              ▼
                           Agents
                         /         \
                        /           \
             pure internal        needs external
                  │                    │
                  ▼                    ▼
             Collaborator          Resolver
                                       │
                                    Contract
                                       │
                                    Provider
                                       │
                                external world
```

### Roles de los Componentes en la Frontera

- **`evo-script`**: Es el dueño del lenguaje de programación Evo. Responsable de la gramática, sintaxis, parsing, tokenización, operadores, funciones sintácticas, reglas de asociatividad, precedencia de operadores y agrupación sintáctica.
- **`evo-shell`**: Es el motor semántico de ejecución `no_std`. No conoce la gramática textual ni el parsing del lenguaje. Recibe intenciones semánticas puras expresadas mediante Use Cases públicos e independizadas de la infraestructura externa.
- **`Contracts` / `Providers`**: Los Contracts definen la firma de las capacidades externas necesarias (filesystem, terminal, base de datos, tiempo, procesos). Los Providers implementan la interacción concreta contra la infraestructura física.
- **`evo-shell-cli`**: Es el *Composition Root* ejecutable que ensambla la entrada/salida del usuario, invoca a `evo-script`, conecta los Use Cases públicos de `evo-shell` con los Providers concretos e inicia la ejecución.

---

## 2. Responsabilidades de `evo-script`

`evo-script` representa exclusivamente el lenguaje de programación Evo.

### Alcance Exclusivo de `evo-script`:
- **Sintaxis y Gramática**: Análisis léxico y sintáctico del código fuente.
- **Identificadores y Literales**: Reconocimiento textual de constantes y variables (ej. `"42"`, `"2.5f32"`, `"quantity"`, `"price"`).
- **Operadores y Simbología**: Definición de símbolos del lenguaje (ej. `"+"`, `"-"`, `*`, `"/"`, `"%"`, `"|>"`, `"( )"`).
- **Nombres de Comandos y Pipes**: Palabras clave como `"copy-to"`, `"move-to"`, `"select"`, `"filter"`, `"iter"`, `"rename"`.
- **Precedencia y Reglas de Expresión**: Determinación del orden operacional de evaluación (ej. en `quantity * price + tax`, decide que la multiplicación se evalúa antes que la suma).
- **Errores Sintácticos**: Diagnósticos del lenguaje como token inesperado, fin de expresión inesperado o paréntesis sin cerrar.

### Aislamiento de `evo-script`:
- `evo-script` **NO debe conocer la estructura interna** de `evo-shell` (`agents`, `resolvers`, `collaborators`, `contracts`, `providers`, `handlers`).
- `evo-script` interactúa exclusivamente con `evo-shell` invocando sus **Use Cases públicos** (`definitions/use_cases`).
- `evo-script` delega la ejecución semántica de operaciones físicas o matemáticas puras hacia los Use Cases correspondientes.

---

## 3. Responsabilidades de `evo-shell`

`evo-shell` es la frontera y motor semántico de ejecución del sistema.

### Características de `evo-shell`:
- **Core `#![no_std]`**: Diseñado con cero asignación dinámica en heap (`no alloc`, `no Vec`, `no String`, `no Box`, `no dyn`).
- **Independiente de la Sintaxis**: `evo-shell` no contiene parsers, tokenizadores ni AST.
- **Traducción Semántica**: Mientras `evo-script` interpreta el texto `"+"` o `"copy-to"`, `evo-shell` recibe y ejecuta el caso de uso `add(left, right)` o `copy_to(origin, destination)`.

---

## 4. Arquitectura Interna de `evo-shell`

### Mapeo Definición $\rightarrow$ Implementación

| Capa de Definición (`definitions/`) | Capa de Implementación | Responsabilidad Principal |
|---|---|---|
| `definitions/use_cases/` | `agents/` | Frontera pública y coordinación del caso de uso. |
| `definitions/contracts/` | `providers/` | Firma de capacidad externa e implementación de infraestructura. |
| `definitions/continuations/` | `handlers/` | Firma de trabajo prestado y su procesador. |
| *Interno* | `resolvers/` | Resolución determinista de límites técnicos. |
| *Interno* | `collaborators/` | Lógica de dominio pura interna y reutilizable. |
| *Interno* | `tools/` | Operaciones puras pequeñas y utilitarias. |

---

### Use Case
Definición pública de una operación semántica pura. Es el único punto de entrada expuesto a otros crates como `evo-script`.
- **Forma:** Expresado mediante firmas de función o punteros de función (`fn`).
- **Ejemplos:** `copy_to`, `move_to`, `rename`, `delete`, `trash`, `create_file`, `create_dir`, `add`, `subtract`, `multiply`, `divide`, `remainder`, `negate`.

### Agent
Coordinador e implementador de un Use Case.
- Recibe la intención semántica del Use Case.
- Coordina el flujo operacional.
- Invoca un `Resolver` si se requiere interactuar con una frontera técnica o infraestructura externa.
- Invoca un `Collaborator` o `Tool` directamente cuando ejecuta lógica interna pura sin fronteras externas.
- **Reglas:**
  - **No `Agent -> Agent`**: Los agentes no dependen de otros agentes; comparten colaboraciones y capacidades.
  - **`Agent -> Collaborator` permitido**: Cuando el flujo es puramente interno y no requiere resolución externa (ej. `add` Use Case $\rightarrow$ `calculator` Agent $\rightarrow$ `arithmetic` Collaborator).

### Resolver
Paso determinista que ejecuta y traduce operaciones a través de una frontera técnica.
- Consume una capacidad definida por un `Contract`.
- Ejecuta la capacidad técnica del Provider.
- Traduce los resultados técnicos o errores de infraestructura a respuestas semánticas del dominio.
- **Regla:** No se crean Resolvers para operaciones puramente matemáticas o internas (ej. `add(2, 2)` no usa Resolver porque no hay infraestructura externa).

### Contract
Firma que especifica una capacidad requerida del mundo exterior.
- Declarado mediante punteros de función (`fn`) sin estado (*stateless*).
- No implementa lógica de infraestructura.

### Provider
Implementación concreta de un `Contract` contra infraestructura real fuera del core.
- Ejemplos de dominios de Providers: `filesystem`, `database`, `network`, `operating_system`, `clock`, `process`, `terminal`.
- **Terminal como Provider**: La terminal es infraestructura física externa. Se modela conceptualmente como `terminal Contract` $\rightarrow$ `terminal Provider`.

### Collaborator
Lógica interna cohesionada, pura y reutilizable.
- Cero I/O físico, cero infraestructura, cero Provider/Contract.
- Puede ser consumido directamente por Agents o por Resolvers.
- Ejemplo: `collaborators::arithmetic` (ejecuta operaciones numéricas escalares checked).

### Continuation & Handler
Mecanismo para procesamiento con ventana de préstamo (*borrowing window*).
- **Continuation** (`definitions/continuations/`): Define el puntero de función para procesar datos sin transferir ownership.
- **Handler** (`handlers/`): Implementación concreta que procesa la vista prestada durante la llamada del Provider.
- Ejemplos: `consume_scope` $\rightarrow$ `scope_handler`, `report_copy_progress` $\rightarrow$ `copy_progress_handler`.

---

## 5. Diseño Orientado a Funciones (*Function-Oriented Design*)

Evolution aplica un paradigma estrictamente enfocado en funciones puras y tipos de datos:
- **Estructuras de Datos y Enums**: Contienen datos y estados semánticos (ej. `Number`, `NumberBinding`, `Scope<'a>`).
- **Funciones y Pointers (`fn`)**: Expresan comportamiento, contratos y transferencias de control.
- **Construcciones Evitadas**: Se eliminan clases de servicio, administradores (*managers*), patrones orientados a objetos, `traits` de un solo método por ceremonia, objetos con estado dinámico y despacho dinámico (`dyn`).

---

## 6. Manejo de Datos, Ownership y Borrowed Structs

1. **Ownership de backing data**: La propiedad del dato reside en el componente que lo crea o en el Provider de infraestructura.
2. **Borrowed Structs**: Estructuras del dominio cuyos datos son referencias con lifetimes explícitos (ej. `Scope<'a>`, `NumberBinding<'b>`).
3. **Immutability por Defecto**: Préstamos inmutables (`&T`) para lectura e inspección; préstamos mutables (`&mut T`) solo si se modifica el recurso prestado para devolverlo al dueño.

---

## 7. Aclaraciones y Notas de Migración Pendiente

### Dominio Numérico vs Interpretación de Expresiones
- **`Number` y `arithmetic` en `evo-shell`**: `evo-shell` posee la representación escalar en ejecución `Number` y las funciones puras `add`, `subtract`, `multiply`, `divide`, `remainder`, `negate`.
- **`expression_evaluator` y `NumberBinding`**: La interpretación de expresiones escritas (`"2 + 3 * 4"`), parsing léxico, reglas de precedencia, evaluación de paréntesis y resolución de identificadores textuales corresponden a `evo-script`. El código de `expression_evaluator` y `NumberBinding` reside temporalmente en `evo-shell` por razones de descubrimiento incremental y será migrado a `evo-script` en una etapa posterior.

---

## 8. Estrategia de Testing

- **Código de Producción Limpio**: Todos los archivos en `src/` están exentos de bloques `#[cfg(test)] mod tests { ... }`.
- **Verificación Externa**: Todos los unit tests e integration tests residen en la suite externa `tests/`.
- **Principios del Tester**: Las pruebas verifican la validez de los contratos y las invariantes del sistema sin introducir ceremonia ni código de test dentro de la entrega de producción.
