# Evo-Script Engine — Compiled Program / Bytecode Data

Status: COMPILED PROGRAM / BYTECODE DATA — IN ANALYSIS

Este documento define el producto persistente producido por Bytecode Compiler y consumido directamente por la Stack VM de `evo-script-engine` v0.

La autoridad deriva de `TECHNICAL_DESIGN.md`, especialmente TD-003, TD-004, TD-005, TD-007, TD-009, TD-010 y TD-011, y de `SEMANTIC_PROGRAM_DATA.md`.

```text
Semantic Program
    ↓ Bytecode Compiler
Compiled Program
    ↓ Stack VM
Execution Result
```

## CD-001 — Compiled Program representa mecanismo ejecutable

Status: CLOSED

Regla canónica:

> `Semantic Program` representa significado resuelto; `Compiled Program` representa el mecanismo ejecutable persistente que la VM consume sin volver al AST ni al Semantic Program.

Consecuencias:

1. `Compiled Program` puede sobrevivir al Source Text y al Compilation Working State.
2. La VM no realiza name resolution, type inference ni semantic validation.
3. Semantic identities pueden conservarse solamente cuando siguen siendo una identity técnica útil en el producto compilado.
4. Semantic information que ya fue lowered a layout, constants, symbols o Instructions no se conserva por costumbre.
5. No se introducen Active Scope, Host Session State, Current Provider ni provider lookup ambiental.

## CD-002 — FunctionId se preserva

Status: CLOSED

`FunctionId` se reutiliza como identidad de Internal Function desde Semantic Program hacia Compiled Program.

Invariante de ordering:

```text
SemanticProgram.functions[n]
    ↓ Bytecode Compiler preserves function identity ordering
CompiledProgram.functions[n]

FunctionId(n)
    = misma Internal Function dentro de la compilación
```

No se introduce `CompiledFunctionId`.

`FunctionId` continúa siendo:

```text
not stable between compilations
not ABI identity
not physical function address
```

Una llamada interna compilada puede referenciar directamente `FunctionId`.

## CD-003 — ConstantId

Status: CLOSED

Representación:

```rust
struct ConstantId(usize);
```

`ConstantId` identifica una constante owned por el `CompiledProgram`:

```text
ConstantId(n)
    → CompiledProgram.constants[n]
```

Invariantes:

1. namespace local al `CompiledProgram`;
2. no estable entre compilaciones;
3. no es address de memoria;
4. puede utilizarse como operand de una Instruction para cargar una constante persistente.

## CD-004 — ExternalSymbolId

Status: CLOSED

Representación:

```rust
struct ExternalSymbolId(usize);
```

`ExternalSymbolId` identifica un external capability symbol persistente dentro de `CompiledProgram`:

```text
ExternalSymbolId(n)
    → CompiledProgram.external_symbols[n]
```

Flujo:

```text
SignatureId / SignatureBindingId
        ↓ Bytecode Compiler
SignatureSymbol
        ↓
ExternalSymbolId
        ↓
CompiledProgram.external_symbols
```

Invariantes:

1. no identifica Provider;
2. no representa runtime binding;
3. no es `SignatureId`;
4. Runtime lo resolverá únicamente mediante explicit Application Bindings.

## CD-005 — Signature Dependency Erasure

Status: CLOSED

`SignatureBindingId` es una identity semántica necesaria durante Semantic Program, pero no necesita slot ni representación como Value durante runtime v0.

Regla:

> Signature Dependencies no son Values de primer orden y se eliminan como parámetros físicos durante Bytecode lowering.

Ejemplo:

```text
fn process(
    int id,
    values::search search,
    string filter
)
```

Semantic parameter count:

```text
3
```

Physical compiled Value parameters:

```text
2
├── id
└── filter
```

La capability `search` ya fue lowered hacia `ExternalSymbolId`.

No se introducen:

```text
SignatureSlot
LoadSignature
PassSignature Value
Function Value
Closure for dependency transport
```

## CD-006 — Signature Dependency Forwarding se resuelve en compilation

Status: CLOSED

El forwarding semántico de una Signature Dependency no obliga a transportar una función/capability como dato en runtime.

```text
Semantic call argument
SignatureDependency(SignatureBindingId)
    ↓ Bytecode Compiler
validated dependency relationship
    ↓
no physical Value argument
```

Una función interna llamada ya fue compilada con la misma Signature requirement lowered hacia el `ExternalSymbolId` correspondiente.

Por tanto una internal CALL transporta únicamente sus Value arguments físicos.

## CD-007 — Direct Signature y Signature Dependency convergen

Status: CLOSED

Ambas formas semánticas:

```text
SemanticCallTarget::DirectSignature(SignatureId)
SemanticCallTarget::SignatureDependency(SignatureBindingId)
```

se reducen a una llamada externa contra:

```text
ExternalSymbolId
```

El origen sintáctico/semántico diferente no necesita opcode distinto después de lowering.

## CD-008 — CompiledProgram root

Status: CLOSED — shell

Representación raíz:

```rust
struct CompiledProgram {
    functions: Vec<CompiledFunction>,
    entry_point: FunctionId,
    constants: Vec<Constant>,
    external_symbols: Vec<ExternalSymbol>,
    source_map: SourceMap,
}
```

Las identities `Constant`, `ExternalSymbol`, `SourceMap` e `Instruction` están requeridas por esta raíz pero su representación interna se cierra en bloques posteriores.

Relaciones:

```text
CompiledProgram
├── functions: Vec<CompiledFunction>      1..N
├── entry_point: FunctionId               exactly 1 valid FunctionId
├── constants: Vec<Constant>              0..N
├── external_symbols: Vec<ExternalSymbol> 0..N
└── source_map: SourceMap                  exactly 1
```

No se introducen wrappers `FunctionTable`, `ConstantPool` o `ExternalSymbolTable` mientras no agreguen responsabilidad distinta de poseer la colección.

El concepto arquitectónico `Constant Pool` se materializa mediante `CompiledProgram.constants`.

## CD-009 — CompiledFunction shell

Status: CLOSED

Representación:

```rust
struct CompiledFunction {
    parameter_count: usize,
    local_count: usize,
    max_operand_depth: usize,
    instructions: Vec<Instruction>,
}
```

### parameter_count

Cuenta exclusivamente Value Parameters físicos.

Signature Dependency Parameters están erased antes de runtime y no cuentan como Parameter Slots.

### local_count

Cuenta Value bindings con storage estable que no son Value Parameters, incluyendo cuando aplique:

```text
Let bindings
Associated when extraction bindings
Structured when extraction bindings
```

La asignación exacta `BindingId → LocalSlot` pertenece al siguiente bloque.

### max_operand_depth

Representa la profundidad temporal máxima requerida por la función compilada sobre su Operand Window.

Bytecode Compiler la calcula a partir del instruction sequence resultante.

La VM puede usarla para preparar/delimitar la Shared Frame Region sin descubrir la profundidad máxima semántica durante ejecución.

### instructions

`Vec<Instruction>` conserva el bytecode ordenado de la función. `Instruction` se cierra posteriormente.

## CD-010 — Semantic data lowered away

Status: CLOSED

Por defecto no sobreviven dentro de `CompiledFunction`:

```text
TypeId
BindingId
FieldId
VariantId
SignatureId
SignatureBindingId
SemanticExpression
SemanticStatement
SemanticFunction.satisfaction
parameter type metadata
local type metadata
```

Lowering esperado:

```text
TypeId              → selected executable mechanism / instruction semantics
BindingId           → ParameterSlot / LocalSlot
FieldId             → physical field position
VariantId           → runtime discriminant
SignatureId         → ExternalSymbolId
SignatureBindingId  → erased / ExternalSymbolId
SemanticLiteral     → ConstantId / immediate when explicitly justified later
SemanticExpression  → Instructions
```

No se conserva información semántica únicamente para duplicar una decisión ya materializada en bytecode.

## CD-011 — Current closure

```text
Compiled Program responsibility          ✅ CLOSED
FunctionId preservation                  ✅ CLOSED
ConstantId                               ✅ CLOSED
ExternalSymbolId                         ✅ CLOSED
Signature Dependency Erasure             ✅ CLOSED
Signature Dependency Forwarding lowering ✅ CLOSED
Direct/Dependency external convergence   ✅ CLOSED
CompiledProgram root shell               ✅ CLOSED
CompiledFunction shell                   ✅ CLOSED
Semantic data lowering boundary          ✅ CLOSED

ParameterSlot / LocalSlot                ← NEXT
Constant                                 ← NEXT
ExternalSymbol                           ← NEXT
Instruction / Instruction Set            PENDING
SourceMap                                PENDING
Compiled Program exact inventory         PENDING
```
