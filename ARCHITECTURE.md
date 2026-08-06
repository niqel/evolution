# ARCHITECTURE

## Propósito

Este documento define una arquitectura genérica para resolver proyectos de forma consistente.

La meta no es crear capas por costumbre. La meta es separar responsabilidades de manera defendible.

---

## Principio General

La arquitectura sigue un flujo de responsabilidades y dirección de dependencias claro:

```text
                         ┌──────────── Agent B
                         │
Subject → Agent A ───────┤
                         │
                         └──────────── Resolver
                                          │
                    ┌─────────────────────┼─────────────────────┐
                    ↓                     ↓                     ↓
                  Tool              Collaborator             Contract
                                                               ↓
                                                            Provider
```

El flujo debe conservar esa dirección.

La filosofía general es:

- operar sobre préstamos cuando sea suficiente
- evitar paquetes intermedios innecesarios
- materializar solo cuando agrega capacidad real
- mantener responsabilidades pequeñas y defendibles

---

## Subject (Sujeto y Sujeto Agente)

Se mantiene la arquitectura basada en sujeto/acción.

El **Subject** representa aquello sobre lo cual se realiza una acción o aquello que participa en el caso de uso.

Reglas del Subject:

- No convertir el Subject en una clase de servicio.
- Los datos y el estado deben representarse mediante las estructuras del dominio apropiadas (`entities`, `value_objects`, vistas prestadas `borrowed`).
- Las acciones no se empaquetan en métodos artificiales de clases de servicio; se expresan mediante funciones y módulos con identidad de sujeto agente.

---

## Agent — Responsabilidad

Un `agent` representa y coordina un caso de uso específico del sistema.

Sigue la regla del sujeto agente (ejemplos: `copier` coordina la copia, `executor` coordina la ejecución de comandos).

### Responsabilidades del Agent:

- coordinar el flujo completo del caso de uso;
- tomar decisiones de coordinación del flujo;
- delegar resoluciones puntuales a los `resolvers` correspondientes;
- coordinar otros casos de uso invocando a otros `agents` cuando sea necesario (`Agent -> Agent`);
- propagar resultados y errores tipados mediante `Result<T, E>` y el operador `?`;
- asegurar que el caso de uso se complete exitosamente o reporte explícitamente por qué no pudo completarse.

### El Agent NO debe:

- implementar infraestructura directa;
- hablar directamente con `providers` (`Agent -> Provider` está estrictamente prohibido);
- realizar pequeñas transformaciones que pertenecen a `resolvers`;
- invocar `tools` o `collaborators` directamente (`Agent -> Tool` / `Agent -> Collaborator` prohibido);
- convertirse en un contenedor genérico de lógica (no es un "Manager" o "Service").

El código de un Agent debe poder leerse principalmente como una secuencia de coordinación limpia:

```rust
let source = source_resolver::resolve(...)?;
let result = another_agent::execute(source)?;
let output = output_resolver::resolve(result)?;

Ok(output)
```

---

## Agent → Agent (Delegación de Casos de Uso)

Un Agent puede llamar directamente a otro Agent (`Agent A -> Agent B`).

Esto representa la **composición o delegación directa de casos de uso**. Si Agent A necesita ejecutar una capacidad funcional que ya pertenece conceptualmente al caso de uso coordinado por Agent B, la llamada directa es totalmente válida.

Reglas:

- **VÁLIDO:** `Agent A -> Agent B` cuando un caso de uso necesita otro caso de uso completo.
- **INCORRECTO:** No debe introducirse artificialmente un Resolver entre ambos (`Agent A -> Resolver -> Agent B`). El Resolver no existe para envolver Agents.

---

## Dependencias entre Agents (Acíclicas)

Las dependencias entre Agents deben ser estrictamente **acíclicas**.

- **Permitido:** `Agent A -> Agent B -> Agent C`
- **No permitido:** `Agent A -> Agent B -> Agent C -> Agent A` (Ciclos)

Si surge una dependencia circular entre Agents, es una señal inequívoca de que la división de responsabilidades de los casos de uso debe revisarse.

---

## Use Cases Entre Proyectos

Cuando dos proyectos necesitan comunicarse, no deben compartir `resolvers` internos.

La comunicación entre proyectos debe hacerse mediante `use_cases`.

Un `use_case` representa una capacidad pública específica del sistema.

Regla:

- un `use_case` define una sola acción pública;
- un `agent` implementa ese `use_case`;
- otros proyectos dependen del `use_case`, no del `agent` concreto;
- los `resolvers` permanecen internos al proyecto.

Relación:

```text
otro proyecto
    ↓
use_case
    ↓
agent
    ↓
resolver
    ↓
contract
    ↓
provider
```

No es correcto:

```text
otro proyecto -> resolver interno
```

Porque el `resolver` forma parte de la mecánica interna del sistema y no de su frontera pública.

---

## Resolver — Responsabilidad

Un `resolver` es la frontera responsable de determinar si una necesidad puntual del caso de uso queda resuelta o no.

### El Resolver:

- recibe la necesidad puntual (`input`);
- solicita capacidades externas mediante `contracts`;
- recibe el resultado técnico proporcionado por un `provider`;
- interpreta ese resultado técnico;
- determina si la necesidad quedó resuelta;
- convierte errores técnicos de bajo nivel (p. ej., `io::Error`) a errores tipados con significado explícito para el dominio o caso de uso (`IterError`, `ScopeError`, etc.);
- retorna un `Result<ResolvedValue, ResolveError>` tipado.

### El Resolver NO debe:

- realizar trabajo de infraestructura o I/O por sí mismo sin un `contract`/`provider`;
- coordinar casos de uso completos (eso pertenece al `agent`);
- invocar a un `agent` (`Resolver -> Agent` está prohibido);
- crear ownership innecesario.

Forma conceptual del flujo del Resolver:

```text
input
    ↓
Resolver
    ↓
Contract
    ↓
Provider
    ↓
resultado técnico
    ↓
Resolver
    ↓
Result<ResolvedValue, ResolveError>
```

---

## Resolver Siempre entre Agent y Provider

Se establece como regla arquitectónica obligatoria: **Un Agent NUNCA consume directamente un Provider.**

- **INCORRECTO:** `Agent -> Provider`
- **CORRECTO:** `Agent -> Resolver -> Contract -> Provider`

Aunque la responsabilidad del Resolver sea pequeña o directa, sigue siendo indispensable porque constituye la frontera encargada de interpretar si la respuesta técnica enviada por el Provider satisface la necesidad del caso de uso. Esto mantiene al Agent completamente aislado de detalles técnicos y errores de infraestructura.

---

## Provider — Responsabilidad

Un `provider` representa la comunicación con el mundo real y la infraestructura externa.

Ejemplos de infraestructura:
- sistema de archivos (`std::fs`);
- terminal (`stdout`, `stdin`, secuencias ANSI);
- sistema operativo / entorno;
- red / sockets;
- reloj externo / temporizadores de sistema;
- procesos externos.

### El Provider:

- provee información externa o ejecuta operaciones físicas sobre la infraestructura;
- devuelve el resultado técnico nativo correspondiente (p. ej., `io::Result<ReadDir>`).

### El Provider NO debe:

- decidir qué significa ese resultado técnico para el caso de uso o para el dominio.

Ejemplo conceptual:

- **Provider:** `std::fs::read_dir(path) -> io::Result<ReadDir>` (resultado técnico)
- **Resolver:** `io::Result<ReadDir> -> Result<FilesystemIteration, IterError>` (interpretación del dominio)

La interpretación del resultado técnico pertenece exclusivamente al Resolver.

---

## Contract

Un `contract` define qué se espera del exterior sin implementarlo ni coordinarlo.

- El Resolver solicita capacidades al exterior expresadas formalmente en el Contract: `Resolver -> Contract -> Provider`.
- **Preferencia por Function Pointers:** Cuando una capacidad representa una sola acción stateless, se prefiere definir la firma mediante un `function pointer`:
  ```rust
  type ReadDirectory = fn(&Path) -> io::Result<ReadDir>;
  ```
- No se deben crear `traits` ni `structs` artificiales únicamente para envolver una sola operación stateless.

---

## Tool (Herramienta Interna)

Una `tool` es una pieza interna, pequeña, genérica, stateless, determinista, sin I/O y sin conocimiento del caso de uso completo.

- **Propósito:** Ayuda a un `resolver` a realizar una operación interna pequeña, puramente matemática, algorítmica o de formateo.
- **Ejemplos conceptuales:** `normalize_path`, `format_size`, `compare_strings`, `calculate_offset`.
- **Fronteras:** Una Tool no es un Agent, ni un Resolver, ni un Provider, ni un caso de uso.
- **Regla:** No se deben crear Tools por costumbre si una función privada local dentro del Resolver expresa suficientemente la operación. La categoría existe para funciones genéricas reutilizables de pequeña responsabilidad.

---

## Collaborator (Colaborador Interno)

Un `collaborator` es una pieza interna que ayuda a un `resolver` a completar una resolución más compleja.

- **Características:**
  - Trabaja internamente (sin I/O ni contacto directo con infraestructura externa);
  - Posee una responsabilidad cohesionada y más significativa que una simple Tool;
  - Ayuda al Resolver a interpretar o estructurar una resolución;
  - No coordina un caso de uso completo (no es un Agent).
- **Ejemplos conceptuales:** `recursive copier`, `argument expander`, `expression evaluator`, `path classifier`.

### Diferencia Fundamental entre Piezas:

```text
Tool          → Pequeña, genérica y sin estado.
Collaborator  → Lógica interna más significativa y cohesionada.
Provider      → Comunicación con infraestructura externa.
Agent         → Coordinador del caso de uso completo.
Resolver      → Evaluador determinista de una necesidad puntual.
```

---

## Mapa de Relaciones Arquitectónicas

La arquitectura se sintetiza bajo la siguiente regla central:

> *"Agents coordinate Agents and Resolvers. Resolvers resolve using Contracts/Providers, Tools, and Collaborators."*

### Relaciones Permitidas (VALID)

| Origen | Destino | Naturaleza de la Relación |
|---|---|---|
| `Agent` | `Agent` | Composición / delegación directa de caso de uso |
| `Agent` | `Resolver` | Solicitud de resolución de un paso del flujo |
| `Resolver` | `Contract` | Invocación de capacidad externa requerida |
| `Resolver` | `Tool` | Asistencia en cálculos/operaciones puras pequeñas |
| `Resolver` | `Collaborator` | Asistencia en lógica interna cohesionada |
| `Contract` | `Provider` | Implementación concreta de la capacidad externa |

### Relaciones Prohibidas (INVALID)

| Origen | Destino | Motivo de Prohibición |
|---|---|---|
| `Agent` | `Provider` | Violación de aislamiento de infraestructura (requiere Resolver) |
| `Agent` | `Tool` | Violación de frontera (el Agent solo coordina Resolvers y Agents) |
| `Agent` | `Collaborator` | Violación de frontera (el Collaborator asiste al Resolver) |
| `Resolver` | `Agent` | Violación de dirección (un Resolver no invoca casos de uso) |
| `Provider` | `Agent` | Violación de dirección (la infraestructura no invoca casos de uso) |

### Nota sobre Composición Resolver → Resolver

Si en flujos específicos existe composición puntual de un Resolver sobre otro, se trata como una composición interna puntual y no como una regla rígida de nivel superior.

---

## Tester (Responsabilidad Transversal de Pruebas)

`Tester` representa la responsabilidad conceptual de verificar el contrato, comportamiento e invariantes de cualquier pieza del sistema.

### Naturaleza del Tester:

- Es una responsabilidad conceptual **exclusivamente de testing**, NO de producción.
- **REGLA:** NUNCA crear un `struct Tester`, `trait Tester` ni módulos de producción llamados `tester`.
- Existe conceptualmente para probar: `Agent`, `Resolver`, `Provider`, `Tool`, `Collaborator` y estructuras del dominio.

### Roles Conceptuales del Tester:

- **Agent Tester:** Verifica la correcta secuencia de coordinación, toma de decisiones y propagación de errores del caso de uso.
- **Resolver Tester:** Verifica la interpretación del input, la toma de decisión determinista y la conversión de resultados a `Ok`/`Err`.
- **Provider Tester:** Verifica la adaptación e interacción adecuada con la infraestructura real o mediante mocks controlados de sistema, asegurando que los errores técnicos se reporten fielmente.
- **Tool Tester:** Verifica operaciones deterministas y casos borde de funciones puras.
- **Collaborator Tester:** Verifica la lógica de resolución interna cohesionada.

### Representación Física en Rust (Testing Ownership):

- **Unit Tests:** Viven en el módulo `#[cfg(test)] mod tests { use super::*; }` colocado en el mismo archivo del componente propietario.
- **Integration Tests:** Viven exclusivamente en el directorio `tests/` del crate correspondiente.

### Regla para Provider Tester:

Un Provider también debe probarse para verificar su adaptación e interacción con la infraestructura física. No obstante, se aplica el principio de no ceremonia: no se exigen pruebas artificiales para Providers triviales si no aportan valor real.

---

## Principio de No Ceremonia

Las piezas de la arquitectura (Tool, Collaborator, Provider, Resolver, Agent) **NUNCA deben crearse por simetría o ceremonia**.

- No todo flujo requiere una `Tool` o un `Collaborator`.
- Sin embargo, todo acceso de un `Agent` a infraestructura DEBE respetar la cadena `Agent -> Resolver -> Contract -> Provider`.
- Todo caso de uso DEBE tener su `Agent` correspondiente.
- Se debe crear una pieza únicamente cuando exista una responsabilidad real y defendible en el sistema.

---

## Ownership y Préstamo

La vida del dato pertenece a quien lo crea.

Reglas:

- si no eres dueño → usa préstamo
- no prolongues lifetimes artificialmente
- materializa solo cuando haga falta independencia real

---

## Borrowed en Rust

En esta arquitectura, `borrowed` significa una representación no dueña.

Un `borrowed` puede ser:

- `&str`
- `&[T]`
- `&T`
- `&mut T`
- una view con lifetime
- un handle con lifetime
- un slice semántico del dominio

Un slice es una forma posible de borrowed, pero no todos los borrowed son slices.

Se usa borrowed inmutable cuando el sistema solo necesita leer, inspeccionar o resolver.

Se usa borrowed mutable cuando la operación debe modificar el recurso prestado y devolverlo al mismo dueño.

Regla:

- si solo lees → `&T`
- si modificas → `&mut T`
- si necesitas conservar → ownership

---

## Failure Is Not Flow

Separar siempre:

- bug interno → se corrige
- violación externa → se rechaza
- estado válido → se modela

---

## Layout Sugerido

La estructura puede separar las definiciones de las implementaciones para que la dirección de dependencias sea visible desde el árbol del proyecto:

```text
src/
├── definitions/
│   ├── use_cases/
│   │   └── copy_file.rs
│   ├── contracts/
│   │   └── file_system.rs
│   └── domain/
│       ├── borrowed/
│       │   └── file_view.rs
│       └── entities/
├── agents/
│   └── copier.rs
├── resolvers/
│   ├── origin_resolver.rs
│   ├── destination_resolver.rs
│   └── copy_resolver.rs
├── tools/
└── providers/
    └── std_file_system.rs
```

Lectura arquitectónica:

```text
definitions/use_cases/copy_file.rs
    ↓ implementado por
agents/copier.rs
    ↓ coordina
resolvers/*
    ↓ consumen capacidades definidas en
definitions/contracts/file_system.rs
    ↓ implementado por
providers/std_file_system.rs
    ↓ usa
std::fs
```

### definitions/

Contiene aquello que el sistema necesita definir sin implementarlo:

- firmas públicas de `use_cases`
- firmas de capacidades externas o `contracts`
- representaciones del dominio
- vistas prestadas en `borrowed`
- entidades con ownership cuando son necesarias

### use_cases/

Contiene las fronteras públicas que otros componentes o proyectos pueden consumir.

Un `use_case` define una sola acción pública. El agente correspondiente implementa esa firma.

Ejemplo conceptual:

```rust
pub type CopyFile = fn(
    origin: &Path,
    destination: &Path,
) -> Result<(), CopyError>;
```

### contracts/

Contiene las firmas de las capacidades que los resolvers requieren del exterior.

El provider implementa esas firmas mediante funciones concretas.

### domain/borrowed/

Define las formas mínimas en que los datos propiedad de un provider son prestados al sistema.

Regla:

- si basta préstamo → `borrowed`
- si se necesita independencia real → `entities`

---

## Resumen

- el `subject` representa la entidad sobre la cual se actúa (mantiene datos e invariantes, no es clase de servicio)
- el `use_case` expone una acción pública entre proyectos
- el `agent` coordina e implementa el caso de uso (puede llamar a otros agents de forma acíclica)
- el `resolver` decide e interpreta resultados técnicos de providers en resultados del dominio
- el `contract` define capacidades requeridas del exterior (vía function pointers cuando son stateless)
- el `provider` habla con la infraestructura externa sin decidir semántica del dominio
- la `tool` realiza operaciones internas pequeñas, puras y stateless para un resolver
- el `collaborator` realiza lógica interna cohesionada más compleja para un resolver
- el `tester` es la responsabilidad transversal de verificación (solo en `#[cfg(test)]` o `tests/`)
