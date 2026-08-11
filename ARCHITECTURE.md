# Arquitectura del Proyecto Evolution

Este documento especifica la arquitectura del proyecto **Evolution**, definiendo los límites conceptuales entre los proyectos/crates (`evo-script`, `evo-shell`, `evo-runtime`, `providers`, `evo-shell-cli`, `evo-ui`) y la organización interna del motor semántico `evo-shell`.

---

## 1. Visión General y Fronteras del Sistema

El sistema Evolution procesa solicitudes desde el frontend del usuario hasta su ejecución en infraestructura física siguiendo un flujo unidireccional y desacoplado:

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

### Roles de los Componentes en la Frontera

- **Frontends (`evo-shell-cli`, `evo-ui`)**: Interfaces de interacción con el usuario (terminal, interfaz gráfica, servidores, editores). Dependen de `evo-runtime` para iniciar y visualizar ejecuciones de Evolution.
- **`evo-runtime`**: Runtime / *execution host* de Evolution. Coordina y permite la colaboración entre los componentes del sistema (`evo-script`, `evo-shell`, capacidades/providers) durante una ejecución.
- **`evo-script`**: Es el dueño del lenguaje de programación Evo. Responsable de la gramática, sintaxis, parsing, tokenización, operadores, funciones sintácticas, reglas de asociatividad, precedencia de operadores y agrupación sintáctica.
- **`evo-shell`**: Es el motor semántico de ejecución `no_std`. No conoce la gramática textual ni el parsing del lenguaje. Recibe intenciones semánticas puras expresadas mediante Use Cases públicos e independizadas de la infraestructura externa.
- **`Contracts` / `Providers`**: Los Contracts definen la firma de las capacidades externas necesarias (filesystem, terminal, base de datos, tiempo, procesos). Los Providers implementan la interacción concreta contra la infraestructura física.

---

## 2. Frontends

Un **Frontend** es la interfaz de entrada y salida entre el usuario final (o sistema externo) y Evolution.

### Ejemplos de Frontends:
- **`evo-shell-cli`**: Frontend de terminal interactivo/scriptable.
- **`evo-ui`**: Frontend gráfico.
- **Futuros Frontends**: `evo-server`, `evo-editor`, plugins de IDE, etc.

### Responsabilidades de un Frontend:
- Capturar la interacción del usuario o entorno.
- Enviar solicitudes o fuentes de código a `evo-runtime`.
- Presentar resultados o eventos de ejecución entregados por `evo-runtime`.
- Proporcionar o configurar las capacidades de infraestructura específicas de su entorno cuando corresponda.

### Regla Fundamental de Frontera:
```text
Frontend → evo-runtime
```
*Nunca:*
```text
evo-runtime → Frontend
```

#### `evo-shell-cli` (Frontend de Terminal)
`evo-shell-cli` es un frontend específico para entornos de consola. No es el composition root global del sistema, sino un cliente ejecutable de terminal que utiliza `evo-runtime`. No parsea sintaxis de `evo-script`, no evalúa semántica de `evo-shell` ni decide precedencias.

#### `evo-ui` (Frontend Gráfico)
`evo-ui` es un frontend gráfico que utiliza `evo-runtime` para ejecutar Evolution. No requiere de una terminal física para funcionar ni depende de `evo-shell-cli`. Esto demuestra que la terminal no es un requisito de ejecución para Evolution.

---

## 3. `evo-runtime` (Runtime / Execution Host)

`evo-runtime` es la capa común de ejecución (*execution composition/orchestration layer*) sobre la que operan los distintos frontends.

### Responsabilidad Principal:
Permitir y coordinar una ejecución completa utilizando los componentes especializados del sistema. Decir que `evo-runtime` "corre todo" significa que **hace posible y coordina la ejecución**, no que implemente todas las responsabilidades internamente.

### Responsabilidades Conceptuales de `evo-runtime`:
- Iniciar una ejecución a solicitud de un frontend.
- Recibir el código fuente o intención enviado por el frontend.
- Coordinar la participación de `evo-script` para la interpretación sintáctica.
- Coordinar el acceso a las capacidades semánticas públicas de `evo-shell`.
- Mantener el contexto necesario durante el ciclo de vida de una ejecución.
- Transportar y conectar las capacidades de infraestructura requeridas.
- Entregar los resultados o eventos de ejecución de vuelta al frontend emisor.
- Servir como base común reutilizable para múltiples frontends (`evo-shell-cli`, `evo-ui`, etc.).

### Lo que `evo-runtime` NO debe hacer (Principio *No God Runtime*):
- NO es responsable de sintaxis, gramática, parsing, tokenización ni reglas del lenguaje.
- NO es responsable de implementar los Use Cases semánticos de `evo-shell`.
- NO implementa infraestructura física (filesystem, terminal, base de datos, red).
- NO debe convertirse en un *God Crate* que absorba las responsabilidades de otros módulos.

---

## 4. Coordinación de `evo-runtime` con otros Módulos

### `evo-runtime` y `evo-script`
`evo-runtime` invoca a `evo-script` para interpretar la sintaxis y obtener las intenciones semánticas. `evo-script` conserva la titularidad exclusiva sobre gramática, sintaxis, operadores y precedencia.

### `evo-runtime` y `evo-shell`
`evo-runtime` coordina el acceso a la frontera pública de `evo-shell` (`definitions/use_cases`). Los componentes internos de `evo-shell` (`agents`, `resolvers`, `collaborators`) permanecen encapsulados.
> *Decisión de diseño pendiente:* El mecanismo concreto mediante el cual `evo-runtime` recibe y conecta las implementaciones de los Use Cases se diseñará en un commit posterior. No se inventan estructuras ni contenedores de servicios prematuros.

### `evo-runtime`, `Providers` y Capacidades
`evo-runtime` no posee por obligación todos los Providers concretos existentes. Las capacidades disponibles dependen del host o frontend de ejecución.
- **Principio de Indisponibilidad de Capacidad (*Capability Unavailable*)**: Si un entorno no proporciona una capacidad determinada (ej. un entorno gráfico sin terminal), la ausencia de dicha capacidad se expresa como una indisponibilidad o error semántico correspondiente, sin invalidar a `evo-runtime`.
- **La terminal no es obligatoria**: La terminal es un Provider de infraestructura externa y no constituye un requisito para la existencia o ejecución de `evo-runtime`.

---

## 5. Responsabilidades de `evo-script`

`evo-script` representa exclusivamente el lenguaje de programación Evo.

### Alcance Exclusivo de `evo-script`:
- **Sintaxis y Gramática**: Análisis léxico y sintáctico del código fuente.
- **Identificadores y Literales**: Reconocimiento textual de constantes y variables (ej. `"42"`, `"2.5f32"`, `"quantity"`, `"price"`).
- **Operadores y Simbología**: Definición de símbolos del lenguaje (ej. `"+"`, `"-"`, `*`, `"/"`, `"%"`, `"|>"`, `"( )"`).
- **Nombres de Comandos y Pipes**: Palabras clave como `"copy-to"`, `"move-to"`, `"select"`, `"filter"`, `"iter"`, `"rename"`.
- **Precedencia y Reglas de Expresión**: Determinación del orden operacional de evaluación.
- **Errores Sintácticos**: Diagnósticos del lenguaje como token inesperado, fin de expresión inesperado o paréntesis sin cerrar.

---

## 6. Responsabilidades de `evo-shell`

`evo-shell` es la frontera y motor semántico de ejecución `no_std` del sistema.

### Características de `evo-shell`:
- **Core `#![no_std]`**: Diseñado con cero asignación dinámica en heap (`no alloc`, `no Vec`, `no String`, `no Box`, `no dyn`).
- **Independiente de la Sintaxis**: `evo-shell` no contiene parsers, tokenizadores ni AST.
- **Traducción Semántica**: Recibe intenciones semánticas ya interpretadas (ej. `add(left, right)`, `copy_to(origin, destination)`).

---

## 7. Arquitectura Interna de `evo-shell`

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
Definición pública de una operación semántica. Es el único punto de entrada expuesto externamente a través de `definitions/use_cases`.
- **Forma:** Expresado mediante firmas de función o punteros de función (`fn`).
- **Ejemplos:** Operaciones con efectos (`copy_to`, `move_to`, `rename`, `delete`, `trash`, `create_file`, `create_dir`) y operaciones puras (`add`, `subtract`, `multiply`, `divide`, `remainder`, `negate`).

### Agent
Coordinador e implementador de un Use Case.
- Coordina el flujo operacional e invoca un `Resolver` cuando se requiere infraestructura externa, o un `Collaborator`/`Tool` para lógica pura interna.
- **Reglas:** No `Agent -> Agent`. `Agent -> Collaborator` es permitido cuando no hay resolución externa involucrada.

### Resolver
Paso determinista que ejecuta una capacidad de `Contract` y traduce errores de infraestructura a respuestas semánticas del dominio.

### Contract y Provider
- **Contract**: Firma que especifica una capacidad requerida (`fn` stateless).
- **Provider**: Implementación concreta de un Contract contra infraestructura externa.

### Continuation & Handler
Mecanismo para procesamiento con ventana de préstamo (*borrowing window*).

---

## 8. Direcciones de Dependencias Conceptuales entre Crates

```text
evo-shell-cli ─────┐
                   │
evo-ui ────────────┼──► evo-runtime
                   │
future frontend ───┘

evo-runtime ─────────► evo-script
evo-runtime ─────────► evo-shell public API
```

### Prohibiciones de Dependencias:
- **`evo-runtime` $\rightarrow$ `frontends`**: PROHIBIDO. El runtime no conoce los clientes.
- **`evo-shell` $\rightarrow$ `evo-script`**: PROHIBIDO. El motor semántico no conoce el lenguaje.
- **`evo-shell` $\rightarrow$ `evo-runtime`**: PROHIBIDO. El motor semántico no conoce el runtime.

---

## 9. Diseño Orientado a Funciones (*Function-Oriented Design*)

Evolution aplica un paradigma enfocado en funciones puras, punteros de función (`fn`), enums y estructuras de datos. Se eliminan clases de servicio, administradores (*managers*), objetos con estado dinámico y despacho dinámico (`dyn`).

---

## 10. Aclaraciones y Notas de Migración Pendiente

### Interpretación de Expresiones vs Dominio Numérico
- **En `evo-shell`**: `Number` y las funciones de `collaborators::arithmetic` (`add`, `subtract`, `multiply`, etc.) pertenecen al motor semántico.
- **Pendiente de Migración**: `expression_evaluator` y `NumberBinding` residen temporalmente en `evo-shell` debido al proceso de descubrimiento incremental, pero pertenecen a `evo-script` y serán migrados en una etapa posterior.

---

## 11. Estrategia de Testing

- **Código de Producción Limpio**: Todos los archivos en `src/` están exentos de bloques `#[cfg(test)] mod tests { ... }`.
- **Verificación Externa**: Todos los unit tests e integration tests residen en la suite externa `tests/`.
