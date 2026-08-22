# Evo Runtime Definition Naming Conventions

Este documento establece formalmente el vocabulario, las responsabilidades y las convenciones de nombres utilizadas para estructurar las **definitions** e **implementaciones** de la arquitectura técnica en Rust de **Evo Runtime**.

> [!NOTE]
> Este documento técnico pertenece exclusivamente a la arquitectura de implementación en Rust de `evo-runtime`. **No** forma parte de la sintaxis ni semántica de Evo-Script, ni altera el contenido de `EVO_SCRIPT_SPECIFICATION_v0.md` o `EVO_RUNTIME_SPECIFICATION_v0.md`. La especificación normativa define *qué* debe hacer el Runtime; este documento define *cómo* nombramos las piezas técnicas de su implementación.


---

## 1. Principio central: Definitions como Function Pointers

La arquitectura de Evo Runtime desacopla la declaración de interfaces y contratos de sus implementaciones concretas mediante **function pointers** tipados, evitando el uso de traits, dynamic dispatch (`dyn`), trait objects o interfaces simuladas con polimorfismo genérico como mecanismo arquitectónico de frontera.

Se establecen las siguientes tres categorías de definitions:

- **Use Case**: definición mediante function pointer que describe una acción que un **Agent** debe implementar.
- **Contract**: definición mediante function pointer que describe una capacidad externa que un **Provider** debe implementar.
- **Requester**: definición mediante function pointer que describe la función a través de la cual un consumidor solicita una acción.

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                            ARCHITECTURAL BOUNDARY                           │
├───────────────────────────────┬─────────────────────────────────────────────┤
│ Definitions (Function Pointer)│ Implementations (Concrete Functions)        │
├───────────────────────────────┼─────────────────────────────────────────────┤
│ Use Case  (action.rs)         │ Agent    (agents/<subject>/<action>.rs)     │
│ Contract  (action_provide.rs) │ Provider (providers/<provider>/provide.rs)  │
│ Requester (action_request.rs) │ Consumer / Requester implementation         │
└───────────────────────────────┴─────────────────────────────────────────────┘
```

> **Regla fundamental**: `Use Case`, `Contract` y `Requester` son *definitions* expresadas mediante function pointers. `Agent` y `Provider` son *implementaciones concretas* de funciones cuyas firmas satisfacen dichas definiciones.


---

## 2. Regla general de nombres de archivos: Acción primero (*Action-First*)

La organización de archivos en las definiciones técnicas prioriza la **agrupación alfabética por acción**. La acción principal aparece siempre al principio del nombre, seguida de sus calificadores:

```text
action_qualifier.rs
```

### Ejemplos canónicos
- `copy.rs`
- `copy_full.rs`
- `copy_metadata.rs`
- `copy_recursive.rs`
- `rename.rs`

Esta convención es deliberada: permite que al listar o explorar el árbol de archivos alfabéticamente, todas las variantes de una misma familia de acción permanezcan agrupadas de forma contigua. Nombres como `copy_full.rs` son intencionales y **no** deben alterarse a `full_copy.rs`.


---

## 3. Use Cases

Los Use Cases residen conceptualmente bajo el directorio:

```text
definitions/use_cases/
```

Cada archivo representa **una única acción**.

### Reglas de Use Cases
1. **Nombre de archivo**: coincide con la acción o `action_qualifier.rs` (por ejemplo, `copy.rs`, `copy_full.rs`, `rename.rs`).
2. **Firma arquitectónica**: el archivo declara la firma completa mediante un alias público de tipo function pointer (`pub type ... = fn(...);`).
3. **Nombre del tipo**: utiliza `PascalCase` derivado directamente del nombre de la acción.

### Ejemplos conceptuales
```rust
// definitions/use_cases/copy.rs
pub type Copy = fn(...);

// definitions/use_cases/copy_full.rs
pub type CopyFull = fn(...);
```


---

## 4. Agents

Un **Agent** representa el **sujeto** que ejecuta uno o más Use Cases.

```text
Agent    = Sujeto
Use Case = Acción
```

Conceptualmente: `Subject.action` (por ejemplo, `Copier.copy` o `Copier.copy_full`).

### Reglas de Agents
1. **Organización modular preferida**: cuando un sujeto posee múltiples acciones relacionadas, se organiza como un módulo bajo el nombre del sujeto:
   ```text
   agents/
   └── copier/
       ├── copy.rs
       └── copy_full.rs
   ```
2. **Función de implementación**: la función pública que implementa el Use Case se nombra exactamente con la **acción**:
   ```rust
   // agents/copier/copy.rs
   pub fn copy(...) {
       // Implementación que satisface definitions::use_cases::copy::Copy
   }

   // agents/copier/copy_full.rs
   pub fn copy_full(...) {
       // Implementación que satisface definitions::use_cases::copy_full::CopyFull
   }
   ```
3. **Preservación de firma**: el Agent **no** redefine la firma arquitectónica; debe satisfacer la firma declarada por la definition del Use Case correspondiente.


---

## 5. Contracts

Los Contracts residen conceptualmente bajo el directorio:

```text
definitions/contracts/
```

Un **Contract** describe una capacidad requerida por el sistema que será satisfecha por un Provider de infraestructura o servicio externo.

### Reglas de Contracts
1. **Nombre de archivo**: utiliza la convención `action_provide.rs` (o `action_qualifier_provide.rs`), manteniendo la acción al inicio para agrupación alfabética:
   - `copy_provide.rs`
   - `copy_full_provide.rs`
   - `read_provide.rs`
2. **Nombre del tipo function pointer**: el tipo exportado utiliza **siempre** el nombre local uniforme `Provide`:
   ```rust
   // definitions/contracts/copy_provide.rs
   pub type Provide = fn(...);

   // definitions/contracts/copy_full_provide.rs
   pub type Provide = fn(...);
   ```
3. **Diferenciación por namespace**: `copy_provide::Provide` y `copy_full_provide::Provide` son Contracts distintos; el contexto semántico de la acción lo aporta el módulo/archivo contenedor, no el identificador del tipo.


---

## 6. Providers

Un **Provider** es una implementación concreta de infraestructura externa (sistema operativo, filesystem, librerías Rust, servicios externos, red, dispositivos o runtimes externos) que implementa uno o más Contracts.

### Reglas de Providers
1. **Función de implementación**: la función concreta que satisface un Contract se llama **siempre** `provide`:
   ```rust
   // providers/fs_copier/provide.rs
   pub fn provide(...) {
       // Implementación concreta que satisface copy_full_provide::Provide
   }
   ```
2. **Aislamiento de infraestructura**: el Provider encapsula las dependencias externas o de bajo nivel respecto al caso de uso que lo consume.


---

## 7. Requesters

Un **Requester** reside conceptualmente bajo el directorio:

```text
definitions/requesters/
```

Representa la frontera de solicitud a través de la cual un consumidor invoca una acción o servicio.

### Reglas de Requesters
1. **Nombre de archivo**: utiliza la convención `action_request.rs` (o `action_qualifier_request.rs`):
   - `copy_request.rs`
   - `copy_full_request.rs`
   - `rename_request.rs`
2. **Nombre del tipo function pointer**: el tipo exportado utiliza **siempre** el nombre local uniforme `Request`:
   ```rust
   // definitions/requesters/copy_request.rs
   pub type Request = fn(...);
   ```
3. **Función de implementación**: la función concreta que satisface la firma de solicitud se llama `request`:
   ```rust
   pub fn request(...) { ... }
   ```
4. **Principio de necesidad**: un Requester solo se define cuando existe un consumidor que realmente requiere esa frontera explícita; no se crean capas vacías por mera simetría arquitectónica.


---

## 8. Resumen de Convenciones

| Concepto | Convención de archivo | Nombre del tipo (Function Pointer) | Función de implementación |
|---|---|---|---|
| **Use Case** | `action.rs` o `action_qualifier.rs` | PascalCase de la acción (`Copy`, `CopyFull`) | Nombre de la acción (`copy`, `copy_full`) |
| **Contract** | `action_provide.rs` | `Provide` | `provide` |
| **Requester** | `action_request.rs` | `Request` | `request` |
| **Agent** | Módulo por sujeto (`agents/<subject>/<action>.rs`) | Implementa Use Case | Nombre de la acción (`copy`, `copy_full`) |
| **Provider** | Módulo por provider (`providers/<provider>/...`) | Implementa Contract | `provide` |

- **Use Case / Contract / Requester**: definiciones abstractas expresadas exclusivamente mediante *function pointers*.
- **Agent / Provider**: implementaciones concretas expresadas como *funciones regulares* en Rust.


---

## 9. Convenciones sobre Estructuras de Datos

Las definiciones de estructuras de datos técnicas de Evo Runtime se mantendrán conceptualmente separadas de las definiciones de acciones. 

La arquitectura distinguirá formalmente entre estructuras de datos con propiedad (*owned*) y con préstamo (*borrowed*). Su diseño, nomenclatura y ubicación dentro del árbol técnico se formalizarán en el Data Dictionary y en la especificación técnica correspondiente cuando se desarrollen las necesidades concretas de datos.


---

## 10. Reglas y Directrices de Diseño

1. **Organización orientada a la acción**: la acción principal organiza los nombres y aparece siempre primero (`copy_full`, no `full_copy`).
2. **Use Cases describen acciones del sistema**: formalizan qué operaciones ejecuta el Runtime.
3. **Agents son sujetos ejecutores**: implementan las acciones declaradas por los Use Cases.
4. **Contracts formalizan capacidades requeridas**: declaran la interfaz que un Provider debe satisfacer.
5. **Providers implementan Contracts**: exponen una función `provide` que conecta con infraestructura o servicios.
6. **Requesters formalizan solicitudes**: exponen una función `request` cuando un consumidor lo requiere.
7. **Function Pointers como frontera**: las fronteras arquitectónicas se expresan mediante function pointers, no mediante traits ni `dyn`.
8. **Justificación de responsabilidad**: no se crea ninguna definition, Agent, Provider ni Requester sin una responsabilidad real e inmediata que la justifique.
9. **Composición explícita**: se favorece la composición explícita de funciones sobre jerarquías de herencia o contenedores opacos.
10. **Ámbito estrictamente técnico**: estas convenciones son de implementación en Rust y son totalmente independientes de las reglas del lenguaje Evo-Script.
