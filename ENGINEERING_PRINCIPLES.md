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

### Resolver y Agent

- **Resolver:**
  - Recibe valores ya disponibles;
  - Aplica una transformación, validación o decisión determinista;
  - Retorna un resultado tipado `Result<T, E>`;
  - No controla flujo mediante `if`s de negocio dispersos;
  - Permite encadenamiento mediante `Result` y `?`.

  Forma conceptual:
  ```text
  input → resolver → Result<output, error>
  ```
  El resolver favorece la composición limpia:
  ```rust
  let a = resolver_a::resolve(...)?;
  let b = resolver_b::resolve(a)?;
  let c = resolver_c::resolve(b)?;
  Ok(c)
  ```
  `Ok(value)` y `Err(error)` constituyen los dos caminos explícitos de resolución.

- **Agent:**
  - Coordina acciones;
  - Encadena resolvers, providers y capacidades;
  - Expresa el flujo del caso de uso;
  - Propaga errores tipados mediante `Result` / `?`;
  - No duplica lógica que pertenece a resolvers;
  - No se convierte en un "manager" genérico.

  Forma conceptual:
  ```text
  Agent → resolver A → provider → resolver B → Result
  ```
  El agent representa la **acción** aplicada al **sujeto** (`iterator`, `iteration_advancer`, `pipeline_executor`, `executor`).

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
- **Acciones:** se expresan preferentemente mediante módulos y funciones.

Esta convención evita la creación innecesaria de clases contenedoras en Rust.

### Testing Ownership

Los tests deben vivir junto al componente cuya responsabilidad prueban:

- **Resolver tests:** junto al resolver (en `mod tests` dentro del propio archivo).
- **Agent tests:** junto al agent (en `mod tests` dentro del propio archivo).
- **Provider tests:** junto al provider (en `mod tests` dentro del propio archivo).
- **Entity / Value Object tests:** junto al tipo correspondiente.
- **Parser / Tokenizer tests:** junto a parser, tokenizer o resolver correspondiente.
- **Tests de Integración Vertical:** viven exclusivamente en `tests/` del crate (e.g. `evo-shell/tests/`).

`lib.rs` nunca debe utilizarse como depósito global de tests.

### Crate Root / lib.rs Rule

`lib.rs` es el mapa público del crate. Debe contener principalmente:

- declaración de módulos (`mod ...`);
- reexports (`pub use ...`);
- documentación indispensable del crate.

No debe contener lógica de negocio, helpers, resolvers, providers, mocks o suites globales de tests. `lib.rs` debe poder leerse como un índice limpio del crate.

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
