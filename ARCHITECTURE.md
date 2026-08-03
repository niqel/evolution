# ARCHITECTURE

## Propósito

Este documento define una arquitectura genérica para resolver proyectos de forma consistente.

La meta no es crear capas por costumbre. La meta es separar responsabilidades de manera defendible.

---

## Principio General

La arquitectura sigue una dirección simple:

```text
agent
  ↓
resolver
  ↓
contract
  ↓
provider

(tool → opcional)
```

El flujo debe conservar esa dirección.

La filosofía general es:

- operar sobre préstamos cuando sea suficiente
- evitar paquetes intermedios innecesarios
- materializar solo cuando agrega capacidad real
- mantener responsabilidades pequeñas y defendibles

---

## Agent

Un `agent` representa un caso de uso específico del sistema.

Sigue la regla del sujeto agente.

Ejemplos:

- lavadora → lava
- licuadora → licua
- report_generator → genera reportes

Un `agent`:

- coordina un caso de uso
- organiza el flujo internamente como pipeline
- solo trabaja con `resolvers`
- no implementa resolución fina
- no usa `providers` directamente

Puede:

- iniciar flujo
- seleccionar resolvers
- terminar ejecución

No puede:

- consultar infraestructura
- transformar datos arbitrariamente
- resolver por sí mismo

---

## Use Cases Entre Proyectos

Cuando dos proyectos necesitan comunicarse, no deben compartir `resolvers` internos.

La comunicación entre proyectos debe hacerse mediante `use_cases`.

Un `use_case` representa una capacidad pública específica del sistema.

Regla:

- un `use_case` define una sola acción pública
- un `agent` implementa ese `use_case`
- otros proyectos dependen del `use_case`, no del `agent` concreto
- los `resolvers` permanecen internos al proyecto

Ejemplo conceptual:

```rust
pub trait UseCaseLavadora {
    fn lavar(&self, ropa: &mut Ropa);
}
```

Implementación:

```rust
pub struct Lavadora;

impl UseCaseLavadora for Lavadora {
    fn lavar(&self, ropa: &mut Ropa) {
        // flujo interno con resolvers
    }
}
```

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

## Resolver

Un `resolver` decide si existe una forma operable para continuar.

Para resolver:

- usa contratos
- consume datos del provider mediante contratos
- evita copias innecesarias
- puede usar `tools`

Resultado esperado:

```text
resolved
not_resolved
```

Puede:

- inspeccionar
- resolver
- devolver imposibilidad explícita

No puede:

- coordinar casos de uso
- convertirse en provider
- crear ownership innecesario

Forma esperada:

```text
resolver.resolve(...)
```


### Regla de Alcance

Un `resolver` debe resolver una sola condición o transición necesaria para que el caso de uso pueda continuar.

Su responsabilidad no es ejecutar todo el caso de uso, sino responder una pregunta operativa concreta, por ejemplo:

- ¿el origen es válido y accesible?
- ¿el destino permite la operación?
- ¿la copia pudo realizarse?
- ¿el origen puede eliminarse después de una copia confirmada?

El nombre del módulo expresa qué resuelve. Dentro del módulo, la operación pública puede conservar una forma uniforme:

```rust
resolver_origen::resolve(...)
resolver_destino::resolve(...)
resolver_copia::resolve(...)
```

La identidad semántica pertenece al módulo. La función `resolve` expresa la acción común de todos los resolvers.

Un resolver:

- habilita o rechaza un paso del flujo
- devuelve un resultado explícito
- usa contratos para solicitar capacidades externas
- no coordina resolvers anteriores o posteriores

Distinción:

```text
agent      → coordina el caso de uso
resolver   → habilita o rechaza un paso
contract   → define una capacidad externa
provider   → ejecuta contra infraestructura real
tool       → transforma o calcula
```

### Ejemplo: Copiar un Archivo

El caso de uso público es copiar un archivo desde un origen hacia un destino.

El sujeto agente es el módulo `copiador`:

```text
copiador::copiar(origen, destino)
```

El agente no consulta directamente el sistema de archivos ni realiza por sí mismo cada operación. Coordina una cadena de resolvers:

```text
copiador::copiar(origen, destino)
    ↓
resolver_origen::resolve(origen)
    ↓
resolver_destino::resolve(destino)
    ↓
resolver_copia::resolve(origen, destino)
```

Responsabilidades:

- `resolver_origen` confirma que el origen representa un archivo operable
- `resolver_destino` confirma que el destino acepta la operación
- `resolver_copia` solicita la copia y confirma su resultado

Cada resolver usa contratos del sistema de archivos. Un provider implementa esos contratos mediante la infraestructura real, por ejemplo `std::fs`.

```text
copiador
   ↓
resolvers
   ↓
file_system contract
   ↓
std_file_system provider
   ↓
std::fs
```

La operación de copiar conserva el archivo original.

Si el caso de uso también elimina el origen después de confirmar la copia, entonces semánticamente ya no es un `copiador`, sino un `movedor`:

```text
movedor::mover(origen, destino)
    ↓
resolver_origen::resolve(origen)
    ↓
resolver_destino::resolve(destino)
    ↓
resolver_copia::resolve(origen, destino)
    ↓
resolver_eliminacion::resolve(origen)
```

El resolver de eliminación solo puede ejecutarse después de que la copia haya sido confirmada. El agente conserva la responsabilidad de coordinar ese orden.

---

## Contract

Un `contract` define qué se espera del exterior.

No implementa.

No coordina.

Solo define capacidades externas.

Se agrupa en:

- inputs
- outputs
- actions
- events

Puede:

- definir comportamiento esperado
- expresar capacidades

No puede:

- ejecutar lógica
- hablar con tecnología real

---

## Provider

Un `provider` implementa contratos.

Es quien habla con el mundo real.

Puede:

- leer
- escribir
- ejecutar
- emitir

No puede:

- decidir flujo
- definir reglas del dominio
- coordinar casos de uso

---

## Tool (Opcional)

Una `tool` es una herramienta puntual.

Solo existe cuando una operación específica merece identidad propia.

Puede:

- transformar
- calcular
- normalizar

No puede:

- coordinar
- resolver flujo

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

Para Evo-script, estas definiciones pueden expresarse mediante firmas de funciones y function pointers, sin requerir `trait`, `dyn` ni genéricos de comportamiento.

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

- el `use_case` expone una acción pública entre proyectos
- el `agent` coordina e implementa el caso de uso
- el `resolver` decide
- el `contract` define
- el `provider` implementa
- la `tool` ayuda
