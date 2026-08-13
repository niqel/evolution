# ENGINEERING_PRINCIPLES

## Propósito

Este documento define principios de diseño reutilizables para los proyectos Evolution.

No describe un framework específico. Describe una forma de pensar, nombrar, dividir responsabilidades y expresar dependencias en código.

---

## 1. Principio Base

Todo diseño debe buscar responsabilidades claras, dependencias explícitas y semántica legible.

El Principio de Responsabilidad Única es indispensable:

- cada módulo debe tener una responsabilidad dominante;
- cada función debe expresar una acción clara;
- cada tipo debe representar una entidad, vista, resultado o contrato comprensible;
- si una pieza tiene varios motivos principales de cambio, debe dividirse.

La responsabilidad única también es una regla de lenguaje: si el nombre de una pieza no permite explicar qué hace, probablemente su frontera no está bien definida.

---

## 2. Transparencia Semántica

Los nombres deben acercarse al dominio real y no únicamente a la mecánica técnica.

Se prefieren nombres funcionales, módulos pequeños, verbos precisos y firmas que permitan reconstruir el flujo del sistema.

Se evitan nombres como `Manager`, `Helper`, `Utils`, `Thing` o contenedores genéricos sin responsabilidad defendible.

---

## 3. Sujeto Agente

Se adopta el concepto de **sujeto agente** como criterio de diseño.

```text
objeto + acción → sujeto agente
```

Ejemplos:

- ropa + lavar → lavadora;
- audio + enrutar → router de audio;
- archivo + copiar → copier;
- archivo + renombrar → renamer.

Si una pieza recibe un nombre de sujeto agente, debe tener una acción principal clara y defendible.

Un sujeto agente no obliga a crear una clase o `struct`. Puede expresarse como módulo, función, comando, script o componente.

---

## 4. Módulo Antes que Objeto Artificial

Cuando una responsabilidad no posee estado propio ni identidad de instancia, un módulo puede ser suficiente.

No debe crearse una `struct` únicamente para imitar una clase de servicio.

Un tipo con estado se justifica cuando el estado, ownership o invariantes pertenecen realmente a esa entidad.

---

## 5. Firmas como Contratos Arquitectónicos

Una firma de función puede ser una frontera arquitectónica completa.

Cuando una operación puede expresarse mediante un puntero de función, se prefiere esa forma antes que introducir una jerarquía de interfaces ficticia.

```rust
pub type Operation = fn(Input, Request, Contract);
```

Esto permite que el compilador verifique cantidad de argumentos, orden, tipos, lifetimes y retorno.

---

## 6. Use Case como Firma Completa

Un Use Case define la operación completa que el sistema ofrece.

Debe declarar todo lo necesario para ejecutar esa operación:

- parámetros semánticos;
- Requesters;
- Contracts requeridos;
- resultado de control cuando corresponda;
- Error semántico propio cuando la operación pueda fallar.

No debe existir una dependencia escondida que el Agent necesite pero que el Use Case omita.

---

## 7. Agent Implementa Exactamente el Use Case

El Agent es el punto de entrada de una operación y debe implementar exactamente la firma del Use Case.

```rust
pub fn execute(/* exact use case signature */) {
    // orchestration only
}

pub const EXECUTE: use_case::Execute = execute;
```

El binding es un invariante arquitectónico comprobado por el compilador.

---

## 8. Agent = Orchestration Only

Un Agent coordina; no absorbe responsabilidades de otras capas.

Puede transportar argumentos, Requesters y Contracts, y decidir qué Collaborator o Resolver participa.

No debe implementar infraestructura, traducir errores técnicos que pertenecen al Resolver, materializar datos de otro owner, interpretar sintaxis ni convertirse en un `Manager` disfrazado.

---

## 9. Requester = Capacidad de Responder

Un Requester representa la capacidad de entregar una respuesta al consumidor.

```rust
pub type Request = fn(Result<(), Error>);
```

Para vistas prestadas:

```rust
pub type Request = for<'a> fn(View<'a>);
```

> Los componentes intermedios transportan la capacidad de responder, no la respuesta.

---

## 10. Materialization Ownership

El componente que posee los datos debe materializar cualquier vista prestada derivada de ellos.

```text
owner
  │
  ├─ materialize View<'a>
  ├─ request(view)
  └─ borrow ends
```

> El Requester viaja hasta el materializador.

---

## 11. Borrowing Antes que Ownership Artificial

Cuando un consumidor únicamente necesita observar un dato durante un lifetime válido, se prefiere borrowing explícito.

Un valor owned se justifica cuando necesita sobrevivir al owner original, almacenarse, transferirse realmente o representar ownership semánticamente real.

---

## 12. Contract = Frontera Técnica Requerida

Un Contract define la operación mínima que se espera de infraestructura externa.

Un Contract define una firma; no implementa infraestructura, no posee recursos externos y no es un service locator.

La palabra *capability* puede describir conceptualmente una operación disponible, pero no obliga a crear un objeto genérico `Capability`.

---

## 13. Provider = Realización Física

El Provider implementa la operación técnica concreta y posee, cuando corresponde, el recurso o estado externo.

```text
semantic Contract
       ▲
       │ compatible function
technical Provider
```

Los detalles técnicos del Provider no deben filtrarse hacia el Use Case.

---

## 14. Resolver Solo para Fronteras Técnicas

Un Resolver existe cuando una operación cruza una frontera técnica externa.

Puede invocar el Contract, adaptar datos técnicos, traducir Error técnico → Error semántico, transportar un Requester hacia el Provider y entregar el resultado semántico final mediante Requester.

> Result does not imply Resolver.

---

## 15. Evitar Errors Intermedios sin Semántica

Si un Resolver solo transforma:

```text
Contract::Error
      ↓
UseCase::Error
```

no debe introducir además `Resolver::Error` sin información semántica nueva.

---

## 16. Collaborator = Trabajo Interno

Un Collaborator realiza trabajo interno que no cruza una frontera técnica externa.

Puede materializar una vista, usar Tools, ejecutar lógica interna e invocar el Requester cuando él es el materializador.

Un Collaborator no debe invocar a otro Collaborator directamente. La coordinación pertenece al Agent.

---

## 17. Tool = Operación Interna Reutilizable

Un Tool debe ser pequeño, reusable y ajeno a la orquestación.

No conoce Agents, Requesters, Providers, Use Cases ni infraestructura externa.

Un Tool no debe convertirse en un `Utils` genérico.

---

## 18. Control y Datos Viajan en Direcciones Diferentes

```text
CONTROL
caller → Use Case → Agent → Resolver / Collaborator
```

```text
DATOS
materializer / boundary → Requester → consumer
```

No es obligatorio que la respuesta recorra en sentido inverso todas las capas por las que llegó el control.

---

## 19. Una Operación Puede Tener Varias Rutas de Respuesta

Una transferencia puede tener progreso 0..N veces y un resultado final exactamente una vez.

```text
Provider
   ├────► progress Requester
   ├────► progress Requester
   │
   └────► Result
              │
              ▼
           Resolver
              │
              └────► final Requester
```

Cada ruta debe tener una semántica clara.

---

## 20. Tipos Compartidos Solo Cuando el Concepto es Compartido

Dos operaciones pueden reutilizar un tipo únicamente cuando comparten realmente el mismo concepto semántico.

```text
Copy ─┐
      ├─► TransferProgress
Move ─┘
```

No debe compartirse un tipo solo porque tenga los mismos campos si su nombre pertenece semánticamente a otro flujo.

---

## 21. No Duplicar Objetivos Semánticos

Si dos Use Cases, Agents y Collaborators producen exactamente el mismo dato con la misma intención, debe revisarse si realmente existen dos operaciones distintas.

Nombres distintos no prueban responsabilidades distintas.

---

## 22. No Crear Abstracciones por Costumbre

No deben introducirse automáticamente traits, `dyn`, jerarquías de interfaces, wrappers de servicios, factories, service locators o genéricos de comportamiento.

Una abstracción se justifica cuando representa una variación real que el sistema necesita modelar.

Para operaciones estáticas y conocidas, un puntero `fn` puede ofrecer una frontera más pequeña y explícita.

---

## 23. Traits y Despacho Dinámico Deben Tener una Razón Real

Antes de introducir `trait` o `dyn` debe existir una razón concreta, como polimorfismo real en runtime o implementaciones heterogéneas con identidad/estado propio.

Si una firma `fn` expresa completamente la operación, se prefiere la solución más pequeña.

---

## 24. Evitar Transporte Innecesario

No se deben crear objetos únicamente para atravesar capas.

Evitar ownership, clonados y allocations artificiales cuando un Requester y borrowing expresan correctamente el flujo.

---

## 25. Errores Pertenecen a su Semántica

- el Contract define errores técnicos de su frontera;
- el Use Case define errores semánticos de la operación;
- el Resolver traduce entre ambos cuando corresponde.

No deben compartirse errores globales entre operaciones independientes solo por conveniencia.

---

## 26. Presentación No Pertenece al Dominio

La respuesta semántica no debe incorporar texto ya formateado para terminal, widgets, píxeles, estilos o detalles de una UI concreta.

La presentación final pertenece al consumidor.

---

## 27. Separar Lenguaje de Operaciones del Sistema

La capa de lenguaje posee sintaxis, parser, operadores, expresiones, tipos y transformaciones del lenguaje.

La capa de operaciones posee filesystem, red, procesos, scope y otras acciones del entorno.

Un operador del lenguaje no debe convertirse en Use Case de infraestructura.

---

## 28. Dependencias Apuntan hacia Definiciones, no Implementaciones Físicas

Una operación semántica puede conocer el tipo de un Contract requerido, pero no el Provider concreto.

```text
Use Case
   │
   └── Contract type

Provider
   └── compatible implementation
```

---

## 29. Testing como Verificación de Arquitectura

Los tests también pueden verificar invariantes arquitectónicos.

```rust
let operation: use_case::Operation = agent::operation;
let operation_const: use_case::Operation = agent::OPERATION;
```

También deben comprobar transporte de argumentos, traducción de errores, entrega por Requester y eventos de progreso cuando corresponda.

---

## 30. Validación Proporcional al Cambio

Una modificación pequeña no requiere ejecutar siempre toda la suite si existen filtros confiables.

Se recomienda:

- compilación global del workspace;
- formatting global;
- tests filtrados del flujo afectado;
- suite completa cuando el alcance o riesgo lo justifiquen.

La validación debe ser suficiente, no ritual.

---

## 31. Criterio de Diseño Antes de Crear una Pieza

Antes de crear una nueva pieza deben responderse estas preguntas:

1. ¿Cuál es su acción dominante?
2. ¿Qué entidad o vista recibe esa acción?
3. ¿Quién posee los datos?
4. ¿Quién materializa la respuesta?
5. ¿Existe una frontera técnica externa?
6. ¿Necesita Resolver o solo Collaborator?
7. ¿Necesita ownership o basta borrowing?
8. ¿La operación requiere un Requester?
9. ¿Hay una variación real que justifique una abstracción adicional?
10. ¿El nombre permite explicar la responsabilidad sin leer la implementación?

---

## 32. Criterio de Revisión

Una pieza debe revisarse si:

- su nombre no explica su función;
- mezcla coordinación e infraestructura;
- un Agent contiene lógica ajena;
- un Resolver existe sin frontera técnica;
- un Collaborator llama a otro Collaborator;
- aparece un Error intermedio sin semántica propia;
- una vista borrowed se vuelve owned solo para atravesar capas;
- dos flujos tienen el mismo objetivo con nombres diferentes;
- un tipo compartido pertenece semánticamente a solo uno de sus consumidores;
- una abstracción existe únicamente por costumbre.

---

## 33. Síntesis

```text
Use Case
   defines the complete operation
       │
       ▼
Agent
   orchestrates exactly that operation
       │
       ├──► Collaborator ───► Requester
       │
       └──► Resolver ─► Contract ─► Provider
                       │             │
                       │             └─ materialized response/progress
                       │
                       └─ semantic result ─► Requester
```

> Definir explícitamente la operación, mantener ownership donde pertenece, transportar la capacidad de responder y crear solo las abstracciones que representan una diferencia real.
