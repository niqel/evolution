# Arquitectura del Proyecto Evolution

Este documento especifica la arquitectura del proyecto **Evolution**, distinguiendo la topología de ejecución alojada en `evo-runtime`, los límites entre crates y la organización interna de `evo-shell`.

La arquitectura actual es deliberadamente orientada a funciones: los Use Cases definen firmas completas mediante punteros de función, los Agents implementan exactamente esas firmas, los Requesters transportan la capacidad de responder y los Contracts expresan las operaciones técnicas que deben realizar los Providers.

---

## 1. Topología de Ejecución vs Dependencias de Crates

Evolution distingue formalmente dos dimensiones diferentes:

1. **Topología de ejecución**: quién aloja, mantiene el ciclo de vida y ejecuta una aplicación.
2. **Dependencias de código / crates**: qué proyecto conoce a cuál durante la compilación.

Estas dimensiones no deben confundirse. Una relación de ejecución no implica necesariamente una dependencia directa de código, y una dependencia de crate no implica IPC, red ni un servicio externo.

---

## 2. Topología de Ejecución

`evo-runtime` es el **Execution Host** de las aplicaciones Evolution.

```text
                    evo-runtime
                  execution host
                        │
                     app.evo
                        │
                        ▼
                   evo-script
        ┌─────────────────────────────┐
        │ language                    │
        │ syntax / tokenization       │
        │ parser                      │
        │ types                       │
        │ expressions                 │
        │ operators                   │
        │ predicates                  │
        │ filter / select / new       │
        │ to-value / append / take    │
        │ pipes (|>)                  │
        │ lazy iteration semantics    │
        └─────────────────────────────┘
                        │
         utiliza operaciones del entorno
                        ▼
                    evo-shell
        ┌─────────────────────────────┐
        │ scope                       │
        │ filesystem                  │
        │ copy / move / rename        │
        │ create / delete / trash     │
        │ terminal / processes        │
        │ network                     │
        │ system environment          │
        └─────────────────────────────┘
                        │
                        ▼
                    Providers
                        │
                        ▼
                        OS
```

`evo-runtime` aloja el contexto de ejecución; no absorbe las responsabilidades de `evo-script`, `evo-shell` ni de los Providers.

---

## 3. Instalación Compartida y Aislamiento por Aplicación

Evolution utiliza una instalación compartida del runtime con ejecuciones aisladas por aplicación.

```text
       instalación compartida de evo-runtime
                          │
                  host / supervisor
                          │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
     ejecución A    ejecución B    ejecución C
          │              │              │
      shell.evo       music.evo       ui.evo
```

Principios:

- existe una implementación compartida de `evo-runtime`;
- cada aplicación mantiene su propio contexto de ejecución;
- compartir runtime no implica compartir estado de aplicación;
- la forma física del host/supervisor podrá evolucionar sin alterar las fronteras semánticas descritas aquí.

---

## 4. Responsabilidades de `evo-runtime`

`evo-runtime` es responsable de:

- iniciar una aplicación o script `.evo`;
- crear y mantener su contexto de ejecución;
- gestionar lifecycle y aislamiento;
- componer las operaciones requeridas por la aplicación;
- resolver las implementaciones de infraestructura disponibles;
- finalizar y liberar el contexto al terminar la aplicación.

### Principio No-God-Runtime

`evo-runtime` NO:

- define la gramática de `evo-script`;
- parsea expresiones de lenguaje;
- implementa las operaciones semánticas de `evo-shell`;
- implementa directamente filesystem, red, procesos o terminal;
- contiene lógica funcional propia de una aplicación;
- reemplaza a los Providers.

---

## 5. Modelo de Aplicaciones `.evo`

Un archivo `.evo` representa código interpretado por `evo-script` dentro de un contexto mantenido por `evo-runtime`.

```text
OS / launcher
     │
     ▼
evo-runtime
     │
     ▼
ejecución aislada de aplicación
     │
     ▼
   app.evo
```

Una aplicación puede ser desde un único script hasta un paquete con varios scripts y recursos.

La UI, CLI o una interfaz para agentes de IA son **superficies de interacción**. No deben duplicar la lógica funcional de la aplicación.

```text
                 evo-runtime
                      │
                 music.evo
                      │
               lógica de evo-script
                 /         \
                /           \
              UI          CLI / AI
                \           /
                 \         /
                  ▼       ▼
              misma operación semántica
```

> La interacción visual es una superficie; el comportamiento funcional permanece scriptable.

---

## 6. Separación `evo-script` / `evo-shell`

### `evo-script` es dueño del lenguaje

Incluye:

- sintaxis;
- tokenización;
- parsing;
- tipos;
- expresiones;
- operadores;
- predicados;
- `filter`;
- `select`;
- `new`;
- `to-value`;
- transformaciones como `append` y `take`;
- pipes `|>`;
- semántica de iteración lazy.

### `evo-shell` es dueño de las operaciones semánticas del entorno

Incluye conceptos como:

- scope;
- filesystem;
- create;
- copy;
- move;
- rename;
- delete;
- trash;
- procesos;
- red;
- otras operaciones del entorno del sistema.

`evo-shell` no contiene parser, gramática ni operadores del lenguaje.

---

## 7. Organización Interna de `evo-shell`

La estructura conceptual actual es:

```text
evo-shell/src/
├── agents/
├── collaborators/
├── definitions/
│   ├── contracts/
│   ├── requesters/
│   ├── structs/
│   └── use_cases/
├── resolvers/
└── tools/
```

Las antiguas categorías `handlers` y `continuations` fueron eliminadas. No forman parte de la arquitectura vigente.

---

## 8. Definitions

`definitions/` contiene tipos y firmas semánticas. No implementa la operación.

### 8.1 Use Cases

Un Use Case define la **firma completa** de una operación.

Puede incluir:

- argumentos semánticos;
- Requesters;
- Contracts requeridos;
- resultado de control cuando corresponda;
- Error semántico propio.

Ejemplo conceptual:

```rust
pub type Create = for<'target> fn(
    &'target str,
    create_requester::Request,
    create_contract::Create,
);
```

El Use Case no es documentación informal: su tipo es una restricción de compilación real.

### 8.2 Requesters

Un Requester es un puntero de función que define **cómo entregar una respuesta** al consumidor final.

```rust
pub type Request = fn(Result<(), create_file::Error>);
```

Para vistas prestadas se utiliza un HRTB cuando el materializador debe elegir el lifetime:

```rust
pub type Request = for<'a> fn(View<'a>);
```

Principio:

> Los componentes intermedios transportan la capacidad de responder (`Requester`), no la respuesta.

### 8.3 Contracts

Un Contract define la operación técnica que `evo-shell` espera de infraestructura externa.

```rust
pub type Delete = for<'target> fn(
    &'target str,
) -> Result<(), Error>;
```

Un Contract:

- pertenece a las definiciones de `evo-shell`;
- no es un Provider;
- no posee el recurso externo;
- expresa únicamente la firma mínima requerida.

### 8.4 Structs

Los structs representan datos semánticos, entidades o vistas.

Cuando un valor puede ser prestado, se prefiere expresar explícitamente el lifetime en lugar de crear ownership intermedio innecesario.

---

## 9. Agent = Implementación Exacta del Use Case

Todo Use Case tiene un Agent como punto de entrada.

El Agent debe implementar exactamente la firma definida por el Use Case.

La relación se hace explícita con un binding tipado de producción:

```rust
pub fn delete(
    target: &str,
    request: delete_requester::Request,
    delete_operation: delete::Delete,
) {
    delete_resolver::resolve(
        delete_operation,
        target,
        request,
    );
}

pub const DELETE: delete_use_case::Delete = delete;
```

La constante tipada obliga al compilador a verificar cantidad de parámetros, orden, tipos, lifetimes y retorno.

### Responsabilidad del Agent

El Agent coordina, transporta argumentos, Requesters y Contracts, y decide qué Resolver o Collaborator participa.

El Agent NO implementa infraestructura, no traduce errores técnicos, no materializa vistas ajenas, no introduce DTOs de transporte innecesarios y no interpreta sintaxis.

---

## 10. Collaborators

Un Collaborator realiza trabajo interno de `evo-shell` que no cruza una frontera técnica externa.

```text
Use Case
   ↓
Agent
   ↓
Collaborator
   ├─ materializa valor/vista
   ├─ opcionalmente usa Tools
   └─ Requester(value)
```

Ejemplo actual: About.

```text
respond_about::Respond
        ↓
about_responder::respond
        ↓
about_collaborator::collaborate
        ↓
shell_information::get
        ↓
about_requester::Request
```

Un Collaborator no llama a otro Collaborator. Si una operación requiere coordinar varios colaboradores, esa coordinación pertenece al Agent.

---

## 11. Resolvers

Un Resolver existe únicamente cuando hay una **frontera técnica externa**.

Su responsabilidad es invocar el Contract, adaptar la llamada técnica si es necesario, traducir errores técnicos a errores semánticos, transportar Requesters hacia el materializador y entregar mediante Requester el resultado semántico cuando corresponda.

> Result no implica Resolver.

Los Resolvers no necesitan un Error intermedio propio cuando únicamente traducen `Contract::Error → UseCase::Error`.

---

## 12. Providers

Un Provider posee o controla infraestructura externa concreta y expone funciones compatibles con los Contracts definidos por `evo-shell`.

```text
Contract definition
       ▲
       │ función compatible
Provider implementation
```

Un Provider puede poseer estado técnico. Ese estado no debe filtrarse como dependencia conceptual hacia el Use Case.

---

## 13. Tools

Un Tool es una operación interna reutilizable y semánticamente pequeña.

Un Tool no conoce Requesters, Agents, Providers ni infraestructura externa. Puede ser usado por Collaborators cuando una operación pura o reutilizable merece una identidad separada.

---

## 14. Flujo de Operación Externa con Resultado Final

```text
Use Case(target, Request, Contract)
          │
          ▼
        Agent
          │
          ▼
       Resolver
          │
          ▼
       Contract
          │
          ▼
       Provider
          │
          ▼
    Result técnico
          │
          ▼
       Resolver
          │
  Contract Error → Use Case Error
          │
          └────────────► Requester(Result semántico)
```

---

## 15. Flujo de Respuesta Prestada Externa

```text
respond_scope Use Case
         │
         ▼
scope_responder Agent
         │
         ▼
scope_resolver
         │
         ▼
provide_scope Contract
         │
         ▼
Provider
    ├─ materializa Scope<'a>
    └─ request(scope) ─────────► consumer
```

El Provider conserva ownership de los datos de los cuales se forma la vista prestada.

El pequeño `Result<(), respond_scope::Error>` representa estado de control de la operación, no transporte de la vista `Scope`.

---

## 16. Materialization Ownership

> El componente que posee los datos materializa la vista prestada y ejecuta el Requester dentro del lifetime válido.

```text
owner/materializer
    │
    ├─ crea View<'a>
    ├─ request(view)
    └─ termina borrow
```

El Requester viaja hasta el materializador; la vista no regresa a través de las capas para buscar a su consumidor.

---

## 17. Respuestas Múltiples: Transfer Progress

Copy y Move comparten:

```rust
pub struct TransferProgress {
    pub total_bytes: Option<u64>,
    pub transferred_bytes: u64,
}
```

Y un Requester de progreso reutilizable.

```text
Copy / Move Use Case
        │
        ▼
      Agent
        │
        ▼
     Resolver
        │
        ▼
     Contract
        │
        ▼
     Provider
       ├────► TransferProgress Requester (0..N veces)
       │
       └────► Result técnico
                    │
                    ▼
                 Resolver
                    │
                    └────► Requester final (1 vez)
```

Copy y Move comparten únicamente el concepto de progreso. Conservan separados Use Cases, Contracts, Errors, Agents, Resolvers y Requesters finales.

---

## 18. Control vs Datos

```text
CONTROL
caller / composition
       │ semantic args + Requesters + Contracts
       ▼
     Use Case
       ▼
      Agent
       ▼
Resolver / Collaborator
```

```text
DATOS / RESPUESTA
materializador / frontera
       │
       ├─ vista prestada
       ├─ Result semántico
       └─ evento de progreso
               │
               ▼
            Requester
               │
               ▼
        consumidor final
```

---

## 19. Use Cases Actuales de `evo-shell`

```text
copy_to::Copy
    → copier::copy
    → copier::COPY

create_dir::Create
    → directory_creator::create
    → directory_creator::CREATE

create_file::CreateFile
    → file_creator::create_file
    → file_creator::CREATE

delete::Delete
    → deleter::delete
    → deleter::DELETE

move_to::Move
    → mover::move_to
    → mover::MOVE

rename::Rename
    → renamer::rename
    → renamer::RENAME

respond_about::Respond
    → about_responder::respond
    → about_responder::RESPOND

respond_scope::Respond
    → scope_responder::respond
    → scope_responder::RESPOND

respond_welcome::Respond
    → welcome_responder::respond
    → welcome_responder::RESPOND

trash::Trash
    → trasher::trash
    → trasher::TRASH
```

About es el único flujo para `ShellInformation`.

---

## 20. Dependencias de Código y Crates

```text
evo-shell-cli ─────┐
                   │
evo-ui ────────────┼──► evo-runtime
                   │
future frontend ───┘

evo-runtime ─────────► evo-script
evo-runtime ─────────► evo-shell public API
```

Prohibiciones:

- `evo-runtime → frontends`;
- `evo-shell → evo-script`;
- `evo-shell → evo-runtime`.

---

## 21. Diseño Orientado a Funciones

La arquitectura favorece funciones, punteros `fn`, enums, structs de datos, borrowing explícito, lifetimes y composición estática.

No se introducen traits, `dyn`, wrappers de servicio o genéricos de comportamiento para simular interfaces cuando una firma de función expresa suficientemente el contrato arquitectónico.

---

## 22. Nombres y Sujeto Agente

Los nombres deben expresar responsabilidad semántica.

Ejemplos:

- `copier` copia;
- `mover` mueve;
- `renamer` renombra;
- `deleter` elimina;
- `directory_creator` crea directorios;
- `about_responder` responde About.

Se evitan nombres genéricos como `Manager`, `Helper`, `Utils` o un objeto contenedor genérico `Capability`.

---

## 23. Estrategia de Testing

El código de producción vive en `src/`; las pruebas viven en `tests/`.

Se prueba comportamiento semántico, transporte de argumentos, traducción técnica → semántica, entrega por Requester y bindings de Agent contra Use Case.

Para cambios pequeños se prefieren tests filtrados del flujo afectado, acompañados por una comprobación global de compilación del workspace.

---

## 24. Visión Futura: `evo-apps`

`evo-apps` podrá actuar como catálogo, launcher, instalador y repositorio de aplicaciones Evolution.

```text
evo-apps
   │
   ▼
repositorio / catálogo
   │
   ▼
descarga de paquete .evo
   │
   ▼
evo-runtime
```

---

## 25. Síntesis

```text
evo-runtime
    host / lifecycle / aislamiento / composición

        ↓

evo-script
    semántica del lenguaje

        ↓

evo-shell
    operaciones semánticas del entorno

        ↓

Providers
    realización física / técnica
```

Dentro de `evo-shell`:

```text
Use Case
    define la firma completa de la operación
        │
        ▼
Agent
    implementación exacta en tiempo de compilación
        │
        ├─────────────► Collaborator ─────► Requester
        │
        └─────────────► Resolver ─► Contract ─► Provider
                                  │             │
                                  │             └─ respuesta prestada/de progreso
                                  │
                                  └─ resultado final traducido ─► Requester
```

> La firma define la operación, el Agent la coordina, el materializador conserva ownership y el Requester lleva la respuesta directamente al consumidor.
