# Evo-Script Engine — Technical Design

Status: TECHNICAL DESIGN — CLOSED

Este documento registra las decisiones estructurales de Technical Design para `evo-script-engine` v0.

El diseño técnico deriva exclusivamente del modelo funcional cerrado y no redefine la semántica de `evo-script` ni de los Functional Use Cases.

## 1. Canonical Processing Pipeline

El pipeline técnico canónico de `evo-script-engine` v0 es:

```text
Source Text
    ↓
Lexer
    ↓
Tokens
    ↓
Parser
    ↓
AST
    ↓
Semantic Analyzer
    ↓
Semantic Program
    ↓
Bytecode Compiler
    ↓
Compiled Program
    ↓
Stack VM
    ↓
Result
```

`Execute Source` reutiliza semánticamente el mismo pipeline de compilación y el mismo pipeline de ejecución; no introduce un intérprete o runtime alternativo.

## 2. Closed Structural Decisions

### TD-001 — Stack-based VM

Status: CLOSED

`evo-script-engine` v0 utiliza una **Stack-based Bytecode Virtual Machine**.

La elección de Stack VM es una decisión técnica interna y no modifica las Public Capabilities ni los Functional Use Cases.

Invariantes:

- el `Compiled Program` contiene bytecode ejecutable por una Stack VM;
- el operand stack es un mecanismo técnico de evaluación;
- `Pipeline Data` no se redefine como sinónimo de operand stack;
- `Active Scope` permanece como estado semántico separado del mecanismo de operand stack;
- una futura representación de VM diferente requeriría reabrir Technical Design, no el modelo funcional mientras conserve la misma semántica pública.

### TD-002 — Semantic Program como identidad técnica propia

Status: CLOSED

`Semantic Program` es una representación técnica con identidad propia situada entre `AST` y `Bytecode Compiler`.

```text
AST
 ↓
Semantic Analyzer
 ↓
Semantic Program
 ↓
Bytecode Compiler
```

Invariantes:

- `AST` representa estructura sintáctica y puede existir aunque el programa sea semánticamente inválido;
- `Semantic Program` solo existe cuando el análisis semántico concluye exitosamente;
- `Semantic Program` representa el significado ya resuelto necesario para generar bytecode;
- `Semantic Program` es la única IR semántica de v0;
- no se introduce otra IR entre `Semantic Program` y bytecode sin reabrir esta decisión.

### TD-003 — Internal Functions se resuelven durante compilación

Status: CLOSED

Las referencias a funciones internas del mismo Evo-Script Program se resuelven durante compilación hacia una identidad técnica estable dentro del `Compiled Program`.

Conceptualmente:

```text
internal function name
        ↓ semantic resolution
     FunctionId
        ↓
   bytecode CALL
```

El nombre `FunctionId` expresa la identidad conceptual; su representación Rust concreta se define posteriormente en el Technical Data Model.

Separación obligatoria:

```text
Internal Function
    → resuelta durante compilación
    → identidad técnica directa dentro de Compiled Program

External Symbol
    → permanece simbólico en Compiled Program
    → se resuelve durante ejecución mediante capacidad explícita
```

La VM no realiza búsqueda dinámica por nombre para funciones internas ya conocidas por el compiler.

### TD-004 — Forma arquitectónica de Compiled Program

Status: CLOSED

`Compiled Program` contiene conceptualmente, como mínimo:

```text
Compiled Program
├── Functions
├── Entry Point
├── Constant Pool
├── External Symbols
└── Diagnostic / Source Mapping Data
```

La representación concreta, cardinalidades técnicas, ownership y tipos Rust pertenecen al Technical Data Model.

Cada función compilada conserva la información necesaria para ejecutar su bytecode y participar en llamadas internas.

### TD-005 — Constant Pool owned por Compiled Program

Status: CLOSED

Los literales y datos constantes que deban sobrevivir al `Source Text` pertenecen al `Compiled Program` mediante un `Constant Pool` owned.

```text
Source Text
    ↓
Compiler
    ↓
Compiled Program
    └── Constant Pool
          └── owns persistent constant data
```

Invariantes:

- un `Compiled Program` válido puede sobrevivir al lifetime del `Source Text` utilizado para producirlo;
- el bytecode no conserva punteros o referencias prestadas hacia el `Source Text` como almacenamiento permanente de constantes;
- el bytecode puede referenciar constantes mediante identidades/índices técnicos cuya representación se definirá en el Technical Data Model;
- durante ejecución pueden materializarse vistas sobre datos owned por el `Compiled Program` cuando sus lifetimes lo permitan.

### TD-006 — Shared Operand Stack con ventanas lógicas por Call Frame

Status: CLOSED

La Stack VM utiliza conceptualmente un único **Shared Operand Stack** perteneciente a la ejecución. Cada `Call Frame` delimita su región lógica mediante un `stack_base`.

```text
VM Execution
│
├── Shared Operand Stack
│
└── Call Frames
      ├── Frame A → stack_base A
      ├── Frame B → stack_base B
      └── Frame C → stack_base C
```

La decisión es arquitectónica y no prescribe una representación Rust concreta del almacenamiento.

En particular:

```text
Shared Operand Stack != Vec<Value>
```

`Vec`, arrays, slices, buffers propios, almacenamiento preasignado u otras representaciones se evaluarán únicamente en el Technical Data Model si son necesarias. No se introduce una colección genérica por costumbre.

Invariantes:

- existe un único almacenamiento lógico de operandos por ejecución;
- cada `Call Frame` posee una ventana lógica delimitada por su `stack_base`;
- una instrucción del frame activo no puede consumir operandos situados por debajo de su `stack_base`;
- los argumentos y resultados de llamadas pueden utilizar el mismo almacenamiento lógico sin requerir un operand stack independiente por frame;
- `Call Frame` no posee su propio contenedor de operandos;
- `Pipeline Data` continúa siendo un concepto semántico distinto del Shared Operand Stack;
- la representación física del almacenamiento queda diferida al Technical Data Model.

### TD-007 — Shared Frame Region para Parameters, Locals y Operands

Status: CLOSED

Cada `Call Frame` posee lógicamente una **Shared Frame Region** compuesta por slots estables para Parameters y Locals, seguidos por una región temporal de operands.

```text
Shared Value Storage

┌──────────────────────────────┐
│ Frame B Operand Window       │
├──────────────────────────────┤ ← B.operand_base
│ Frame B Local Slots          │
│ Frame B Parameter Slots      │
├──────────────────────────────┤ ← B.frame_base
│ Frame A Operand Window       │
├──────────────────────────────┤ ← A.operand_base
│ Frame A Local Slots          │
│ Frame A Parameter Slots      │
├──────────────────────────────┤ ← A.frame_base
│ initial execution values     │
└──────────────────────────────┘
```

Separación obligatoria:

```text
Parameters / Locals
    = stable frame slots

Operands
    = temporary evaluation region
```

Aunque ambas regiones puedan compartir físicamente el mismo almacenamiento de `Value`, no son semánticamente equivalentes.

Invariantes:

- cada `Call Frame` delimita una `Shared Frame Region` propia dentro del almacenamiento de Values de la ejecución;
- los Parameter Slots y Local Slots son estables durante la vida del frame;
- el Operand Window contiene únicamente valores temporales de evaluación, argumentos de llamadas, resultados intermedios y materialización técnica de Pipeline Data cuando corresponda;
- Parameters, Locals y Operands conservan identidades lógicas distintas aunque compartan backing storage;
- los argumentos de una llamada pueden convertirse en Parameter Slots del callee sin requerir un almacenamiento independiente por frame;
- al retornar o abandonar un frame, su región completa puede liberarse/truncarse lógicamente desde `frame_base`;
- la representación Rust concreta de `Shared Value Storage`, `frame_base`, `operand_base` y slots se define posteriormente en el Technical Data Model;
- esta decisión no prescribe `Vec<Value>` ni otra colección genérica específica;
- el Bytecode Compiler puede calcular o preservar información como `parameter_count`, `local_count` y `max_operand_depth` cuando el Technical Data Model demuestre que es necesaria.

### TD-008 — Ownership de Values producidos por External Capabilities

Status: CLOSED

Los resultados producidos por una `External Capability` utilizan borrowing mientras el dato sea consumido dentro del lifetime válido de su materializador.

Cuando la semántica de ejecución exige que un resultado sobreviva a la invocación inmediata de la `External Capability`, el backing data debe transferir ownership hacia almacenamiento perteneciente a la ejecución. Desde ese momento la ejecución se convierte en owner y materializador de las vistas `Value` borrowed posteriores.

Regla canónica:

```text
borrow mientras alcance

ownership cuando deba sobrevivir
```

Flujo borrowed:

```text
Provider / materializer
        │
        ├── owns data
        ├── materializa Value<'a>
        └── Requester(Value<'a>)
                │
                ▼
          consumo inmediato
                │
                ▼
          termina el borrow
```

Flujo con transferencia real de ownership:

```text
Provider
   │
   │ transfer ownership
   ▼
VM Execution
   │
   ├── owns backing data
   └── materializa Value<'execution>
            │
            ▼
      uso posterior por VM
```

Invariantes:

- un `Value` borrowed producido por una External Capability nunca puede escapar del lifetime de su materializador;
- no se crea ownership artificial cuando el dato solo necesita observación inmediata;
- cuando el dato debe sobrevivir a la invocación externa, existe una necesidad real de ownership y la transferencia queda justificada;
- `VM Execution` es el owner natural del backing data externo cuyo lifetime debe extenderse durante la ejecución;
- después de una transferencia de ownership, la ejecución es responsable de materializar futuras vistas borrowed sobre ese dato;
- `Call Frame`, `Shared Frame Region` y `Shared Value Storage` no se convierten automáticamente en owners del backing data externo;
- una vista `Value` y su representación owned de respaldo son conceptos técnicos distintos;
- la representación Rust concreta del almacenamiento owned y de la relación entre owned backing data y `Value<'a>` se define en el Technical Data Model;
- los Requesters continúan viajando hasta el owner/materializador cuando el dato permanece borrowed;
- esta decisión no prescribe `Vec`, arena, heap, `Box`, `Arc`, `Rc` ni otro mecanismo concreto de almacenamiento.

### TD-009 — VM dirigida por Evo-Script e implementada en Rust

Status: CLOSED

La Stack VM y su bytecode se diseñan a partir de la semántica ejecutable de `Evo-Script`. Rust constituye el lenguaje de implementación y determina las decisiones técnicas necesarias para expresar esa semántica de forma segura y eficiente, pero las construcciones propias de Rust no se convierten automáticamente en conceptos del bytecode ni de la VM.

Regla canónica:

```text
Evo-Script
    define QUÉ debe ejecutarse

Rust
    define CÓMO se implementa
```

Flujo de autoridad:

```text
Evo-Script semantics
        ↓
Semantic Program
        ↓
Bytecode Compiler
        ↓
Evo Bytecode
        ↓
Stack VM
        ↓
Rust implementation
```

Invariantes:

- la semántica de `Evo-Script` determina las capacidades necesarias de la VM;
- `Semantic Program` representa el significado ya resuelto que será reducido a bytecode;
- el bytecode representa mecanismos ejecutables necesarios para `Evo-Script`, no una reproducción de las construcciones de Rust;
- ownership, borrowing, lifetimes y representación de memoria pertenecen a la implementación Rust del Engine y no se exponen automáticamente como semántica de `Evo-Script`;
- una construcción sintáctica de `Evo-Script` no requiere un opcode dedicado cuando puede reducirse correctamente a operaciones de VM más fundamentales;
- `Pipeline`, `when`, `let` u otras construcciones pueden compilarse a mecanismos como load, store, call, compare, branch y return cuando su semántica quede preservada;
- la VM debe ser específica de `Evo-Script` en las capacidades que necesita ejecutar y pequeña en sus mecanismos computacionales;
- no se introducen capacidades de VM únicamente porque Rust posea una construcción equivalente o más compleja.

## 3. Technical Design Closure

Las decisiones estructurales necesarias para comenzar el Technical Data Model de `evo-script-engine` v0 están cerradas.

```text
Stack VM                                  ✅ CLOSED
Semantic Program identity                 ✅ CLOSED
Semantic Program as only semantic IR      ✅ CLOSED
Internal Function resolution at compile   ✅ CLOSED
Compiled Program architectural shape      ✅ CLOSED
Owned Constant Pool                       ✅ CLOSED
Shared Operand Stack + frame windows      ✅ CLOSED
Shared Frame Region                       ✅ CLOSED
External Value ownership                  ✅ CLOSED
Evo-Script-driven VM                      ✅ CLOSED

Technical Design                          ✅ CLOSED
Technical Data Model                      ← NEXT
```

Reabrir una de estas decisiones requiere identificar explícitamente qué nueva necesidad técnica o funcional invalida el diseño cerrado.

## 4. Boundary Toward Technical Data Model

El Technical Data Model puede ahora definir de forma concreta los datos demostrados por el diseño, incluyendo cuando corresponda:

```text
Token
Token Kind
AST
AST Nodes
Semantic Program
Semantic nodes / resolved identities
FunctionId
Compiled Program
Compiled Function
Constant Pool
External Symbol
Instruction
Opcode
Instruction Pointer
Call Frame
Shared Value Storage / VM State
Parameter Slots
Local Slots
Operand Window
Owned external backing data
Source Mapping
Failure representations
```

No se decidirán Participants ni Rust function-pointer signatures antes de que estos datos estén definidos.
