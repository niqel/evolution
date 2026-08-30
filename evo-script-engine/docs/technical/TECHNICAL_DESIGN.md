# Evo-Script Engine — Technical Design

Status: TECHNICAL DESIGN — CLOSED / REVALIDATED

Este documento registra las decisiones estructurales de Technical Design para `evo-script-engine` v0.

El diseño deriva del modelo funcional cerrado y de la frontera normativa `.efn` / Host definida por `evo-script/EFN_HOST_BOUNDARY_v0.1.md`.

## 1. Canonical Processing Pipeline

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

`Execute Source` reutiliza semánticamente el mismo pipeline de compilación y ejecución; no introduce intérprete o runtime alternativo.

## 2. Closed Structural Decisions

### TD-001 — Stack-based VM

Status: CLOSED

`evo-script-engine` v0 utiliza una **Stack-based Bytecode Virtual Machine**.

Invariantes:

- `Compiled Program` contiene bytecode ejecutable por una Stack VM;
- Operand Stack es mecanismo técnico de evaluación;
- `Pipeline Data` no se redefine como sinónimo de Operand Stack;
- la VM no contiene `Active Scope` ni estado interactivo del Host;
- una futura representación de VM diferente requiere reabrir Technical Design, no el modelo funcional mientras preserve la misma semántica pública.

### TD-002 — Semantic Program como identidad técnica propia

Status: CLOSED

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
- `Semantic Program` solo existe después de semantic analysis exitoso;
- representa el significado resuelto necesario para generar bytecode;
- es la única Semantic IR de v0;
- no se introduce otra IR entre Semantic Program y bytecode sin reabrir esta decisión.

### TD-003 — Internal Functions se resuelven durante compilación

Status: CLOSED

Las referencias a funciones internas se resuelven durante compilación hacia una identidad técnica estable dentro del `Compiled Program`.

```text
internal function name
        ↓ semantic resolution
     FunctionId
        ↓
   bytecode CALL
```

`FunctionId` expresa la identidad conceptual; su representación Rust concreta pertenece al Technical Data Model.

Separación obligatoria:

```text
Internal Function
    → resolved at Compile
    → direct technical identity

External Symbol
    → remains symbolic in Compiled Program
    → resolved during execution through explicit capability binding
```

La VM no realiza búsqueda dinámica por nombre para funciones internas ya resueltas.

### TD-004 — Forma arquitectónica de Compiled Program

Status: CLOSED

```text
Compiled Program
├── Functions
├── Entry Point
├── Constant Pool
├── External Symbols
└── Diagnostic / Source Mapping Data
```

Cada compiled function conserva la información necesaria para ejecutar su bytecode y participar en llamadas internas.

El `Compiled Program` no contiene `Active Scope`, Host Session State ni un Current Provider.

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

- un Compiled Program válido puede sobrevivir al Source Text que lo produjo;
- bytecode no conserva referencias borrowed al Source Text como almacenamiento permanente;
- bytecode puede referenciar constants mediante identities/indices técnicos;
- durante ejecución pueden materializarse borrowed views sobre datos owned por Compiled Program cuando sus lifetimes lo permitan.

### TD-006 — Shared Operand Stack con ventanas lógicas por Call Frame

Status: CLOSED

La VM utiliza conceptualmente un único `Shared Operand Stack` perteneciente a la ejecución. Cada `Call Frame` delimita su región lógica mediante un lower bound técnico.

```text
VM Execution
│
├── Shared Operand Stack
│
└── Call Frames
      ├── Frame A → operand lower bound A
      ├── Frame B → operand lower bound B
      └── Frame C → operand lower bound C
```

```text
Shared Operand Stack != Vec<Value>
```

Invariantes:

- existe un único almacenamiento lógico de operands por ejecución;
- cada Call Frame posee una ventana lógica propia;
- una instrucción no puede consumir operands situados debajo del lower bound del frame activo;
- argumentos y resultados pueden utilizar el mismo almacenamiento lógico;
- Call Frame no posee un container de operands independiente;
- `Pipeline Data` continúa siendo concepto semántico distinto del Shared Operand Stack;
- la representación física se decide en Technical Data Model.

### TD-007 — Shared Frame Region para Parameters, Locals y Operands

Status: CLOSED

Cada `Call Frame` posee lógicamente una `Shared Frame Region` con slots estables de Parameters y Locals seguidos por una región temporal de operands.

```text
frame_base
    ↓
[parameters][locals][temporaries...]
                     ↑
                 operand_base
```

Separación obligatoria:

```text
Parameters / Locals
    = stable frame slots

Operands
    = temporary evaluation region
```

Invariantes:

- cada Call Frame delimita una Shared Frame Region dentro del Value storage de ejecución;
- Parameter Slots y Local Slots permanecen estables durante la vida del frame;
- Operand Window contiene evaluación temporal, argumentos, resultados intermedios y materialización técnica de Pipeline Data cuando aplique;
- Parameters, Locals y Operands son identidades lógicas distintas aunque compartan backing storage;
- argumentos pueden convertirse en Parameter Slots del callee sin almacenamiento per-frame adicional;
- al retornar, la región completa puede liberarse/truncarse lógicamente desde `frame_base`;
- representación de storage, bases, slots y capacities pertenece al Technical Data Model;
- esta decisión no prescribe `Vec<Value>`;
- Bytecode Compiler puede calcular `parameter_count`, `local_count` y `max_operand_depth` si el Data Model demuestra su necesidad.

### TD-008 — Ownership de Values producidos por External Capabilities

Status: CLOSED

Regla canónica:

```text
borrow mientras alcance
ownership cuando deba sobrevivir
```

Los resultados de External Capabilities utilizan borrowing mientras sean consumidos dentro del lifetime válido de su materializador.

Cuando un resultado debe sobrevivir a la invocación externa inmediata, su backing data debe transferir ownership hacia almacenamiento de la ejecución. Desde ese momento la ejecución es owner y materializador de las futuras views borrowed.

```text
Provider / materializer
        │
        ├── owns data
        ├── materializes Value<'a>
        └── Requester(Value<'a>)
                │
                ▼
          immediate consumption
```

Si debe sobrevivir:

```text
Provider
   │ ownership transfer
   ▼
VM Execution
   │
   ├── owns backing data
   └── materializes Value<'execution>
```

Invariantes:

- un borrowed Value externo nunca escapa al lifetime de su materializer;
- no se crea ownership artificial para observación inmediata;
- ownership se justifica cuando el dato debe sobrevivir;
- VM Execution es owner lógico natural del backing data externo que necesita execution lifetime;
- Call Frame, Shared Frame Region y Shared Value Storage no se convierten automáticamente en owners de ese backing data;
- Value view y owned backing representation son conceptos técnicos diferentes;
- Requesters continúan viajando hasta owner/materializer cuando el dato permanece borrowed;
- esta decisión no prescribe Vec, arena, heap, Box, Arc o Rc.

### TD-009 — VM dirigida por Evo-Script e implementada en Rust

Status: CLOSED

La VM y su bytecode se diseñan desde la semántica ejecutable de Evo-Script. Rust determina seguridad, ownership, borrowing, memoria y rendimiento de implementación, pero sus construcciones no se convierten automáticamente en conceptos de VM.

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

- Evo-Script determina las capacidades requeridas de VM;
- Semantic Program representa significado resuelto;
- bytecode representa mecanismos ejecutables, no una reproducción de Rust;
- ownership/borrowing/lifetimes pertenecen a implementación Rust;
- una construcción sintáctica no requiere opcode propio si puede reducirse correctamente a mecanismos más fundamentales;
- `Pipeline`, `when`, `let` y demás sintaxis válida `.efn` pueden lowered a load/store/call/compare/branch/return cuando se preserve semántica;
- no se introducen capacidades de VM únicamente porque Rust posea una construcción equivalente.

### TD-010 — Compilation Working State es temporal y no redefine Runtime Materialization

Status: CLOSED

La compilación es un proceso finito cuyo producto persistente es `Compiled Program`. Las representaciones intermedias pertenecen al `Compilation Working State` y pueden utilizar ownership, colecciones dinámicas y allocation cuando representan naturalmente el trabajo del compiler y simplifican corrección, validación, inspección o mantenibilidad.

```text
Compile
├── Source Text
├── Tokens                 temporary
├── AST                    temporary
├── Semantic Program       temporary
├── compiler indexes       temporary when not product data
└── Compiled Program       persistent product
```

Invariantes:

- Vec, Box y allocation no están prohibidos por principio en Compilation Working State;
- tampoco se introducen automáticamente por costumbre;
- un container temporal no se convierte en identidad arquitectónica solo para imitar transporte por capas;
- no se crean DTOs, entities, wrappers o copias completas sin responsabilidad real;
- Token Sequence, AST y Semantic Program pueden materializarse cuando simplifiquen el compiler;
- working state termina cuando deja de ser necesario;
- al concluir Compile, working state no forma parte del Compiled Program salvo datos explícitamente transformados en producto persistente;
- durante Runtime continúa aplicando `borrow mientras alcance; ownership cuando deba sobrevivir` y la prohibición de materialización artificial.

Regla de revisión:

> En Compilation se optimiza primero por corrección, claridad y determinismo; en Runtime se revisa además el costo recurrente de cada materialización.

### TD-011 — `.efn` no posee Active Scope ni Host Session State

Status: CLOSED

La ejecución de un `.efn` es reusable y Consumer-neutral. La VM no modela la sesión interactiva del Host.

```text
Host / Consumer
    │ Invocation Values + explicit capability bindings
    ▼
VM Execution
    ├── function/frame state
    ├── Values
    ├── Pipeline Data
    └── external capability interaction state when required

Host Active Scope
    ╳ does not cross this boundary implicitly
```

Invariantes:

- VM State no contiene `Active Scope`;
- `Compiled Program` no contiene Scope activation metadata;
- `use` no produce AST, Semantic Program node, bytecode instruction u Opcode para `.efn`;
- no existe `SET_SCOPE` o mecanismo equivalente en la Evo-Script v0 VM por necesidad de `.efn`;
- el Engine no consulta Scope del Host para resolver External Symbols;
- External Symbols se satisfacen únicamente mediante explicit Application Bindings;
- no existe Current Provider ambiental: diferentes capabilities pueden utilizar diferentes bindings durante una misma ejecución;
- la semántica histórica de `enter` como mutación de Active Scope no pertenece a `.efn`; un Identifier `enter` puede resolver a una función/capability ordinaria si está definido explícitamente;
- `Pipeline Data` es composición de datos y no state de Host;
- `this` es sintaxis contextual de Pipeline; puede desaparecer durante parsing cuando la forma AST preserve completamente la posición del transported value;
- CLI, UI y API pueden invocar el mismo Compiled Program sin alterar su semántica;
- presentación o reacción específica al Result permanece fuera de la VM.

Regla canónica:

```text
Interactive Host State
    belongs to Host / evo-shell

Reusable `.efn` State
    belongs to evo-script-engine execution

Host State
    !=
VM Execution State
```

## 3. Technical Design Closure

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
Compilation Working State policy          ✅ CLOSED
`.efn` / Host State separation            ✅ CLOSED

Technical Design                          ✅ CLOSED
Technical Data Model                      ← IN PROGRESS
```

Reabrir una decisión requiere identificar explícitamente la necesidad funcional o técnica nueva que invalida el diseño cerrado.

## 4. Boundary Toward Technical Data Model

El Technical Data Model puede definir, cuando corresponda:

```text
Token
Token Kind
AST
AST Nodes / supporting syntax data
Semantic Program
resolved identities
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

Explícitamente no debe introducir por la frontera `.efn` actual:

```text
Active Scope
Host Session State
Current Provider
Use Node
Use Instruction
SET_SCOPE Opcode
```

No se decidirán Participants ni Rust function-pointer signatures antes de que los datos necesarios estén definidos.
