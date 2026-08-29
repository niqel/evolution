# Evo-Script Engine — Technical Design

Status: TECHNICAL DESIGN — IN PROGRESS

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

## 3. Required Open Decisions Before Technical Data Model Closure

Los siguientes puntos forman parte del plan técnico acordado y deben resolverse antes de cerrar el Technical Data Model.

### TD-007 — Parameters / Locals vs Operand Stack

Status: OPEN — NEXT

Debe definirse dónde viven los Parameters y los bindings locales de una función respecto al operand stack.

Hipótesis a evaluar:

```text
Parameters / Locals
    → slots estables del Call Frame

Operand Stack
    → valores temporales de evaluación,
      argumentos de llamada,
      resultados intermedios y pipeline values
```

Esta hipótesis no queda cerrada hasta revisar el modelo de Call Frame.

### TD-008 — Ownership de Values producidos por External Capabilities

Status: OPEN

Debe definirse cómo una ejecución conserva un `Value` producido por una External Capability cuando dicho valor necesita sobrevivir a la invocación inmediata del Provider.

La decisión debe respetar:

- `Value` pertenece semánticamente a `evo-values`;
- Materialization Ownership;
- borrowing antes que ownership artificial;
- Requesters viajan al owner/materializer cuando un dato prestado no puede escapar de su lifetime;
- Pipeline Data puede necesitar permanecer disponible para operaciones posteriores.

No se asumirá que todo resultado externo puede almacenarse arbitrariamente como borrowed `Value` después de retornar del Provider.

## 4. Current Technical Design State

```text
Stack VM                                  ✅ CLOSED
Semantic Program identity                 ✅ CLOSED
Semantic Program as only semantic IR      ✅ CLOSED
Internal Function resolution at compile   ✅ CLOSED
Compiled Program architectural shape      ✅ CLOSED
Owned Constant Pool                       ✅ CLOSED
Shared Operand Stack + frame windows      ✅ CLOSED

Parameters / Locals model                 ← NEXT
External Value ownership                  PENDING

Technical Data Model                      BLOCKED until above decisions close
```

## 5. Boundary Toward Technical Data Model

Una vez cerradas TD-007 y TD-008, el Technical Data Model podrá definir de forma concreta los datos demostrados por el diseño, incluyendo cuando corresponda:

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
Shared Operand Stack / VM State
Parameter / Local slots
Source Mapping
Failure representations
```

No se decidirán Participants ni Rust function-pointer signatures antes de que estos datos estén definidos.
