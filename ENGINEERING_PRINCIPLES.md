# ENGINEERING_PRINCIPLES

## Propósito

Este documento define principios de diseño reutilizables para todos los proyectos.

No describe una tecnología específica ni un framework. Describe una forma de pensar, nombrar y estructurar software.

## Principio Base

Todo proyecto debe construirse sobre principios SOLID.

De todos ellos, el Principio de Responsabilidad Única es indispensable:

- cada módulo debe tener una responsabilidad dominante
- cada función debe expresar una acción clara
- cada tipo debe representar una entidad comprensible del dominio
- si una pieza intenta hacer demasiadas cosas, debe dividirse

## Responsabilidad Única

Una pieza de software tiene responsabilidad única cuando su motivo de cambio principal es uno solo.

Esto implica:

- un módulo no debe mezclar lógica de dominio con interfaz, persistencia, red o coordinación externa si no es su función principal
- una función no debe ocultar varias transformaciones independientes
- un tipo no debe existir como contenedor genérico de comportamientos inconexos

La responsabilidad única no es solo una regla de arquitectura. También es una regla de lenguaje.

Si el nombre de una pieza no deja claro qué hace, probablemente su responsabilidad no está bien definida.

## Sujeto Agente

Se adopta el concepto de sujeto agente como criterio de diseño.

En lingüística, un nombre de agente puede derivarse de un verbo para designar al sujeto que realiza una acción.

Ejemplos:

- licuar -> licuadora
- correr -> corredor
- secar -> secadora
- profesar -> profesor
- construir -> constructor
- to blend -> blender
- to print -> printer
- to generate -> generator

En software, este concepto se usa para nombrar módulos, tipos, comandos y componentes como sujetos conceptuales definidos por la acción dominante que realizan.

Regla:

- si una pieza recibe un nombre de sujeto agente, debe tener una acción principal clara y defendible

Ejemplos:

- `Generator` genera
- `Printer` imprime
- `Disipador` disipa
- `VolumeController` controla volumen

Si una pieza no puede justificar su nombre por una acción dominante, su diseño debe revisarse.

El sujeto agente no obliga a crear una `struct`, una clase o un objeto con estado.

Puede expresarse como módulo, tipo, comando, script o componente, según la frontera real que necesite el sistema.

## Relación Entre Objeto, Acción y Sujeto Agente

Se considera útil la siguiente relación semántica:

```text
objeto + acción -> sujeto agente
```

Ejemplos conceptuales:

- ropa + lavar -> lavadora
- audio + enrutar -> router de audio
- stream + mover -> movedor de streams
- sink + seleccionar -> selector de sink

Esta relación ayuda a diseñar componentes cuyo nombre revele:

- qué recibe
- qué transforma
- cuál es su función principal

En este documento, el objeto de esa relación se entiende primero como una pieza semántica del dominio, no necesariamente como una entidad poseída por el sistema.

En algunos proyectos, ese objeto podrá ser una entidad propia. En otros, podrá ser un préstamo externo, una vista o una capacidad mínima definida por contrato o especificación.

## Inspiración Lingüística

Parte de la inspiración de este enfoque proviene del uso de compuestos funcionales en persa, donde la relación entre objeto y acción puede formar una identidad semántica clara para el sujeto agente o herramienta.

En este documento, esa idea se adopta como referencia conceptual para mejorar la claridad de nombres, módulos y responsabilidades en software, no como una regla lingüística estricta.

## Principio de Transparencia Semántica

Los nombres del sistema deben acercarse al dominio real y no solo a la mecánica técnica.

Se prefiere:

- nombres que expresen función
- módulos pequeños y semánticamente cerrados
- verbos claros para las operaciones
- tipos que representen entidades del dominio

Se evita:

- nombres genéricos como `Manager`, `Helper`, `Utils`, `Thing`, `DataProcessor`
- módulos que solo agrupan código sin identidad semántica
- estructuras creadas solo por costumbre si un módulo basta

## Módulo Como Sujeto Agente

No toda responsabilidad necesita representarse como una `struct`.

Cuando no existe estado propio, una responsabilidad puede vivir correctamente en un módulo.

El módulo puede actuar como sujeto agente semántico del dominio.

Ejemplo:

```rust
mod lavadora {
    pub struct Ropa {
        pub limpia: bool,
        pub material: String,
    }

    pub fn lavar(prenda: &mut Ropa) {
        prenda.limpia = true;
    }
}
```

Lectura semántica:

- `Ropa` es la entidad que recibe la transformación
- `lavar` es la acción
- `lavadora` es el contexto funcional que da identidad al acto

La existencia del módulo ya establece una frontera de responsabilidad.

Según la arquitectura del proyecto, ese objeto puede representarse como entidad propia o como recurso prestado bajo una forma mínima suficiente para la operación.

No se debe crear una `struct Lavadora` si no existe estado, coordinación o comportamiento persistente que la justifique.

## Regla de Diseño

Antes de crear una pieza nueva, deben responderse estas preguntas:

1. ¿Cuál es la acción dominante?
2. ¿Qué entidad recibe la acción?
3. ¿El sujeto agente necesita estado o solo un contexto semántico?
4. ¿El nombre refleja claramente su responsabilidad?
5. ¿La pieza tiene un solo motivo principal de cambio?

Si estas preguntas no tienen respuesta clara, el diseño todavía no está listo.

## Convenciones Recomendadas

- usar nombres con carga semántica real
- preferir módulos pequeños antes que contenedores genéricos
- crear tipos solo cuando agreguen identidad o invariantes reales
- usar funciones con verbos precisos
- evitar abstracciones vacías
- revisar si cada nombre puede explicarse desde el verbo que le da origen

## Criterio de Revisión

Una pieza de código debe revisarse si ocurre cualquiera de estos casos:

- su nombre no explica lo que hace
- hace más de una cosa principal
- mezcla responsabilidades de distintos niveles
- usa una `struct` donde bastaría un módulo
- usa un módulo donde en realidad hace falta estado explícito
- el dominio no puede reconstruirse leyendo nombres y firmas

## Reglas de Composición y Organización

### Subject y Sujeto Agente

- **Subject:** Representa aquello sobre lo cual se realiza una acción o que participa en el caso de uso. No debe convertirse en una clase de servicio. Los datos y el estado permanecen representados mediante las estructuras del dominio apropiadas (`entities`, `value_objects`, vistas prestadas `borrowed`).
- **Sujeto Agente:** Asigna identidad semántica a módulos, tipos y componentes en función de la acción principal que realizan.

### Agent

- **Responsabilidad:** Coordina un caso de uso específico del sistema.
- **Acciones:**
  - Coordinar el flujo del caso de uso.
  - Tomar decisiones de coordinación de alto nivel.
  - Delegar resoluciones puntuales a los Resolvers.
  - Delegar o componer otros casos de uso invocando directamente a otros Agents (`Agent -> Agent`).
  - Propagar resultados y errores tipados mediante `Result<T, E>` y el operador `?`.
  - Asegurar que el caso de uso se complete exitosamente o reportar explícitamente por qué no pudo completarse.
- **Prohibiciones:**
  - No implementa infraestructura.
  - No consume directamente Providers (`Agent -> Provider` PROHIBIDO).
  - No realiza pequeñas transformaciones que corresponden a Resolvers.
  - No invoca `Tools` ni `Collaborators` directamente (`Agent -> Tool` / `Agent -> Collaborator` PROHIBIDO).
  - No se convierte en un "Manager" o contenedor genérico de lógica.
- **Agent -> Agent (Acíclico):** Un Agent puede llamar directamente a otro Agent cuando un caso de uso necesita la ejecución de otro caso de uso completo. No debe colocarse un Resolver artificial entre dos Agents. Las dependencias entre Agents deben ser estrictamente **acíclicas** (`Agent A -> Agent B -> Agent C`).

### Resolver

- **Responsabilidad:** Es la frontera responsable de determinar si una necesidad puntual del caso de uso queda resuelta o no (`Result<ResolvedValue, ResolveError>`).
- **Cadena Obligatoria:** Se ubica obligatoriamente entre el Agent y el Provider:
  ```text
  Agent → Resolver → Contract → Provider
  ```
- **Acciones:**
  - Recibe la necesidad puntual (`input`).
  - Solicita capacidades al exterior mediante Contracts.
  - Recibe el resultado técnico nativo retornado por el Provider.
  - Interpreta dicho resultado técnico y lo traduce a tipos/errores del dominio (`Result<T, E>`).
  - Se auxilia internamente de `Tools` (funciones puras pequeñas) y `Collaborators` (lógica de resolución interna cohesionada).
- **Prohibiciones:**
  - No invoca Agents (`Resolver -> Agent` PROHIBIDO).
  - No realiza I/O o trabajo de infraestructura por sí mismo sin un Contract/Provider.

### Provider y Contract

- **Provider:** Representa la comunicación con la infraestructura real y el exterior (filesystem, terminal, OS, red, temporizadores, procesos). Ejecuta la operación externa y retorna el resultado técnico nativo (p. ej., `io::Result<ReadDir>`) sin decidir su significado para el dominio.
- **Contract:** Define la firma de las capacidades que el Resolver requiere del exterior. Se prefieren **function pointers** (`type Provide = fn(...) -> Result<...>`) sobre traits o structs artificiales cuando la capacidad representa una sola acción stateless.

### Tool y Collaborator

- **Tool:** Función interna pequeña, puramente matemática, algorítmica o de formato, genérica, stateless, determinista, sin I/O y sin conocimiento del caso de uso completo.
- **Collaborator:** Pieza interna de lógica cohesionada y más compleja que ayuda al Resolver en la interpretación o estructuración de una resolución. No realiza I/O directo ni coordina un caso de uso completo.

### Mapa de Relaciones Arquitectónicas

Se aplica la regla central:

> *"Agents coordinate Agents and Resolvers. Resolvers resolve using Contracts/Providers, Tools, and Collaborators."*

- **Permitidas (VALID):**
  - `Agent -> Agent` (Delegación acíclica de casos de uso)
  - `Agent -> Resolver` (Solicitud de resolución puntual)
  - `Resolver -> Contract` (Solicitud de capacidad externa)
  - `Resolver -> Tool` (Asistencia en cálculos o trasformaciones puras pequeñas)
  - `Resolver -> Collaborator` (Asistencia en lógica de resolución interna cohesionada)
  - `Contract -> Provider` (Implementación de la capacidad externa)
- **Prohibidas (INVALID):**
  - `Agent -> Provider` (Violación de aislamiento de infraestructura)
  - `Agent -> Tool` (Violación de responsabilidad)
  - `Agent -> Collaborator` (Violación de responsabilidad)
  - `Resolver -> Agent` (Violación de dirección de dependencias)
  - `Provider -> Agent` (Violación de dirección de dependencias)

### Function Pointer Preference

Para dependencias stateless que representan una sola acción, se prefieren function pointers sobre traits cuando sea suficiente:

```rust
type Provide = fn(...) -> Result<..., ...>;
```

Esto permite:
- módulos y funciones puras;
- composición directa;
- testing sencillo sin mocks pesados;
- evitar structs artificiales usadas como clases.

Se usará `trait` únicamente cuando exista una necesidad real que justifique:
- múltiples operaciones relacionadas;
- estado asociado al comportamiento;
- abstracción o dispatch genérico significativo.

### Structs vs Acciones

- **Structs / Entities / Value Objects:** representan datos, estado e invariantes.
- **Acciones:** se expresan preferentemente mediante módulos y funciones con identidad de sujeto agente.

Esta convención evita la creación innecesaria de clases contenedoras en Rust.

### Testing Ownership y Tester (Responsabilidad Transversal)

`Tester` representa la responsabilidad conceptual de verificar el comportamiento e invariantes de cualquier pieza (`Agent`, `Resolver`, `Provider`, `Tool`, `Collaborator`, `Domain`).

- **Naturaleza:** Es una responsabilidad conceptual **exclusivamente de testing**, NO de producción. NUNCA crear `struct Tester`, `trait Tester` ni módulos de producción llamados `tester`.
- **Ubicación en Rust:**
  - Unit Tests: `#[cfg(test)] mod tests { use super::*; }` colocado junto al componente propietario.
  - Integration Tests: Viven exclusivamente en `tests/` del crate (e.g. `evo-shell/tests/`).
- **Provider Tester:** Prueban la correcta adaptación de los Providers con la infraestructura física real o mockeada según necesidad real (principio de no ceremonia).

`lib.rs` nunca debe utilizarse como depósito global de tests.

### Crate Root / lib.rs Rule

`lib.rs` es el mapa público del crate. Debe contener principalmente:

- declaración de módulos (`mod ...`);
- reexports (`pub use ...`);
- documentación indispensable del crate.

No debe contener lógica de negocio, helpers, resolvers, providers, mocks o suites globales de tests. `lib.rs` debe poder leerse como un índice limpio del crate.

### Principio de No Ceremonia

Las piezas de la arquitectura (Tool, Collaborator, Provider, Resolver, Agent) **NUNCA deben crearse por simetría o ceremonia**.

- No todo flujo requiere una `Tool` o un `Collaborator`.
- Sin embargo, todo acceso de un `Agent` a infraestructura DEBE respetar la cadena `Agent -> Resolver -> Contract -> Provider`.
- Todo caso de uso DEBE tener su `Agent` correspondiente.
- Se debe crear una pieza únicamente cuando exista una responsabilidad real y defendible en el sistema.

## Aplicación General

Estos principios aplican a:

- scripts
- crates
- CLIs
- herramientas internas
- automatización
- utilidades de sistema

## Síntesis

La arquitectura no solo se organiza con tipos y dependencias. También se organiza con gramática.

Un buen diseño puede leerse como una relación clara entre:

- entidad
- acción
- sujeto agente

Cuando el nombre, la responsabilidad y el comportamiento coinciden, el sistema gana claridad, mantenibilidad y coherencia.
