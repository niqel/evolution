# Arquitectura del Proyecto Evolution

Este documento especifica la arquitectura del proyecto **Evolution**, definiendo los límites conceptuales entre los proyectos/crates (`evo-script`, `evo-shell`, `evo-runtime`, `providers`, `evo-shell-cli`, `evo-ui`, `evo-apps`), el modelo de aplicaciones `.evo` y la organización interna del motor semántico `evo-shell`.

---

## 1. Visión General y Modelo de Ejecución del Sistema

Evolution separa estrictamente las superficies de interacción, la coordinación de ejecución, la interpretación del lenguaje y el motor de capacidades semánticas.

### Topología Macro de Ejecución

```text
                 shared evo-runtime installation
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
        App Process A  App Process B  App Process C
              │             │             │
            *.evo         *.evo         *.evo
              │             │             │
              └──── use evo-script ───────┘
                            │
                      semantic API
                            │
                        evo-shell
                            │
                       capabilities
                            │
                        Providers
```

### Principios Fundamentales del Modelo de Ejecución

1. **Un solo `evo-runtime` instalado**: Existe una única instalación/implementación compartida del runtime en el sistema. Las aplicaciones `.evo` reutilizan esa instalación común y **no empaquetan una copia completa del runtime**.
2. **Aislamiento por aplicación**: Reutilizar una instalación común de `evo-runtime` **no significa** ejecutar todas las aplicaciones en el mismo proceso del sistema operativo ni compartir estado entre ellas. La arquitectura asigna a cada aplicación su propio contexto de ejecución e aislamiento de proceso.
3. **Host / Supervisor**: `evo-runtime` incluye conceptualmente una faceta de host/supervisor encargada de descubrir aplicaciones, gestionar su ciclo de vida, lanzar instancias aisladas y resolver capacidades. *(Su topología física concreta —daemon, servicio de sistema, proceso residente o launcher— se diseñará posteriormente).*

---

## 2. Modelo de Aplicaciones `.evo`

Un archivo `.evo` contiene código/script ejecutable interpretado por `evo-script` y ejecutado dentro del entorno proporcionado por `evo-runtime`.

### Naturaleza de `.evo`:
- No requiere compilación a ejecutable binario nativo (ELF/EXE) para ser considerado una aplicación ejecutable de Evolution.
- Puede ser un archivo trivial (ej. `calculator.evo`) o formar parte de un paquete de aplicación con múltiples scripts y recursos (ej. un directorio con `app.evo`, `player.evo`, `resources/`).

> *Decisión de diseño pendiente:* El formato físico final de `.evo` (fuente textual, representación intermedia, bytecode, compresión, cifrado, firmas) y la extensión o formato del paquete de aplicación (*application package*) se definirán en commits posteriores.

---

## 3. Lógica Funcional Única y Superficies UI / CLI

Evolution establece como principio rígido que **las aplicaciones poseen una única lógica funcional**, y que la interfaz gráfica (UI) y la interfaz de consola (CLI) son **superficies de interacción diferentes sobre las mismas capacidades**.

### Diagrama de Aplicación

```text
        UI                    CLI / AI
         │                       │
         └──────────┬────────────┘
                    ▼
               evo-script
                    │
               evo-runtime
                    │
              evo-shell API
                    │
              capabilities
                    │
                Providers
```

### Principios de Superficie:
1. **`evo-script` es común a UI y CLI**: No existen "lenguajes de UI" ni "lenguajes de CLI" separados. Ambos convergen en la misma lógica expresada en `evo-script` y las mismas capacidades semánticas de `evo-shell`.
2. **No duplicación de lógica**: Una aplicación visual **no debe duplicar** su implementación funcional para ofrecer soporte CLI. Tanto el botón de una UI como un comando de consola invocan la misma capacidad funcional subyacente.
   - *Ejemplo conceptual (Music):* El botón `[Play]` en la UI y el comando `music play "song.flac"` en la CLI invocan exactamente la misma capacidad de reproducción.
3. **Orientación a Automatización y Agentes de IA**: Exponer superficies textuales/scriptables equivalentes a las capacidades visuales permite que:
   - Usuarios humanos utilicen la UI gráfica.
   - Usuarios avanzados utilicen scripts y la CLI.
   - **Agentes de IA y automatización** invoquen comandos deterministas directamente, evitando la automatización frágil basada en coordenadas visuales o clicks.
   - *Principio:* *"Visual interaction is a surface; functional capability remains scriptable."*

---

## 4. Capacidades Bajo Demanda (*Capabilities On-Demand*)

Una aplicación Evolution no necesita cargar todas las capacidades del sistema al iniciar; consume únicamente las capacidades que requiere.

### Resulución Dinámica de Capacidades:
```text
app inicia → contexto mínimo → app solicita Capability X → runtime resuelve
                                                               ├── disponible   → continúa
                                                               └── no disponible → CapabilityUnavailable (error)
```

- **Principio *Capability Unavailable*:** Si un entorno carece de una capacidad externa (ej. un entorno sin terminal), la ejecución no se invalida globalmente; simplemente la solicitud de esa capacidad responde con una indisponibilidad o error semántico.
- **Ejemplos conceptuales:**
  - `shell.evo`: requiere `terminal`, `filesystem`, `process`, `scope`.
  - `calculator.evo`: requiere `arithmetic`.
  - `music.evo`: requiere `filesystem`, `audio`, `UI`.

> *Decisión de diseño pendiente:* El mecanismo concreto de resolución de capacidades se diseñará posteriormente. No se inventan prematuramente contenedores de servicios, inyectores de dependencias ni mapas dinámicos.

---

## 5. Frontends y Capa `evo-runtime`

```text
                     FRONTENDS

           evo-shell-cli       evo-ui
                 │               │
                 └───────┬───────┘
                         │
                         ▼
                    evo-runtime
                         │
               ┌─────────┴─────────┐
               ▼                   ▼
           evo-script           evo-shell
           language             semantics
                                   │
                                Contracts
                                   │
                              capabilities /
                               Providers
                                   │
                            external world
```

### Responsabilidades de Capas

- **Frontends (`evo-shell-cli`, `evo-ui`)**: Interfaces de entrada/salida. Capturan la interacción del usuario y la envían a `evo-runtime`.
  - `evo-shell-cli`: Frontend de terminal interactivo/scriptable.
  - `evo-ui`: Frontend gráfico que no requiere terminal física para operar.
  - Regla: `Frontend → evo-runtime` (Nunca `evo-runtime → Frontend`).
- **`evo-runtime` (Runtime / Execution Host)**: Capa común de coordinación de ejecución (*execution composition/orchestration layer*).
  - Coordina la interpretación sintáctica en `evo-script`.
  - Coordina el acceso a Use Cases públicos en `evo-shell`.
  - Entrega eventos/resultados al frontend emisor.
  - *Principio No God Runtime:* No absorbe la sintaxis del lenguaje, ni la semántica de Use Cases, ni la infraestructura física.

---

## 6. Visión Futura: `evo-apps` (Repositorio / Launcher / Store)

> *Nota:* Esta sección define una visión arquitectónica futura y **no representa código ni módulos actualmente implementados**.

`evo-apps` conceptualiza el sistema de gestión de aplicaciones Evolution (launcher gráfico, catálogo, instalación, actualización y descubrimiento).

```text
evo-apps (UI / CLI)
    ↓
repository / store
    ↓
descarga paquetes / scripts .evo
    ↓
ejecución en evo-runtime compartido (procesos aislados)
```

### Características de `evo-apps`:
1. **Distribución liviana**: El repositorio distribuye principalmente scripts `.evo`, recursos y metadatos; no redistribuye copias del runtime.
2. **Superficie dual UI / CLI**: `evo-apps` contará con una interfaz gráfica y una superficie textual equivalente (ej. `evo-apps install ...`), permitiendo que tanto usuarios como **agentes de IA** puedan buscar, inspeccionar e instalar aplicaciones Evolution de manera determinista.

---

## 7. Responsabilidades de `evo-script`

`evo-script` representa exclusivamente el lenguaje de programación Evo.

### Alcance Exclusivo de `evo-script`:
- **Sintaxis y Gramática**: Análisis léxico y sintáctico del código fuente.
- **Identificadores y Literales**: Reconocimiento textual de constantes y variables (ej. `"42"`, `"2.5f32"`, `"quantity"`, `"price"`).
- **Operadores y Simbología**: Definición de símbolos del lenguaje (ej. `"+"`, `"-"`, `*`, `"/"`, `"%"`, `"|>"`, `"( )"`).
- **Nombres de Comandos y Pipes**: Palabras clave como `"copy-to"`, `"move-to"`, `"select"`, `"filter"`, `"iter"`, `"rename"`.
- **Precedencia y Reglas de Expresión**: Determinación del orden operacional de evaluación.
- **Errores Sintácticos**: Diagnósticos del lenguaje como token inesperado, fin de expresión inesperado o paréntesis sin cerrar.

---

## 8. Responsabilidades de `evo-shell`

`evo-shell` es la frontera y motor semántico de ejecución `no_std` del sistema.

### Características de `evo-shell`:
- **Core `#![no_std]`**: Diseñado con cero asignación dinámica en heap (`no alloc`, `no Vec`, `no String`, `no Box`, `no dyn`).
- **Independiente de la Sintaxis**: `evo-shell` no contiene parsers, tokenizadores ni AST.
- **Traducción Semántica**: Recibe intenciones semánticas ya interpretadas (ej. `add(left, right)`, `copy_to(origin, destination)`).

---

## 9. Arquitectura Interna de `evo-shell`

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

### Componentes Internos
- **Use Case**: Definición pública de una operación semántica (`definitions/use_cases`).
- **Agent**: Coordinador e implementador de un Use Case (`agents/`).
- **Resolver**: Paso determinista que ejecuta contratos y traduce errores de infraestructura (`resolvers/`).
- **Contract & Provider**: Firma stateless (`definitions/contracts`) e implementación física (`providers/`).
- **Continuation & Handler**: Procesamiento con ventana de préstamo (*borrowing window*).

---

## 10. Direcciones de Dependencias Conceptuales entre Crates

```text
evo-shell-cli ─────┐
                   │
evo-ui ────────────┼──► evo-runtime
                   │
future frontend ───┘

evo-runtime ─────────► evo-script
evo-runtime ─────────► evo-shell public API
```

### Portabilidad (Linux / Windows)
La semántica de las aplicaciones y de `evo-script` permanece común e idéntica entre sistemas operativos. Las diferencias físicas específicas del SO corresponden exclusivamente a los hosts y Providers concretos (`Linux Provider` / `Windows Provider`).

### Prohibiciones de Dependencias:
- `evo-runtime → Frontends`: PROHIBIDO.
- `evo-shell → evo-script`: PROHIBIDO.
- `evo-shell → evo-runtime`: PROHIBIDO.

---

## 11. Diseño Orientado a Funciones (*Function-Oriented Design*)

Evolution aplica un paradigma enfocado en funciones puras, punteros de función (`fn`), enums y estructuras de datos. Se eliminan clases de servicio, administradores (*managers*), objetos con estado dinámico y despacho dinámico (`dyn`).

---

## 12. Aclaraciones y Notas de Migración Pendiente

### Interpretación de Expresiones vs Dominio Numérico
- **En `evo-shell`**: `Number` y las funciones de `collaborators::arithmetic` (`add`, `subtract`, `multiply`, etc.) pertenecen al motor semántico.
- **Pendiente de Migración**: `expression_evaluator` y `NumberBinding` residen temporalmente en `evo-shell` debido al proceso de descubrimiento incremental, pero pertenecen a `evo-script` y serán migrados en una etapa posterior.

---

## 13. Estrategia de Testing

- **Código de Producción Limpio**: Todos los archivos en `src/` están exentos de bloques `#[cfg(test)] mod tests { ... }`.
- **Verificación Externa**: Todos los unit tests e integration tests residen en la suite externa `tests/`.
