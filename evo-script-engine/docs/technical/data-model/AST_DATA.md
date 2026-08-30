# Evo-Script Engine — AST Data

Status: AST DATA — IN ANALYSIS

Este documento define `AST Data` del Technical Data Model de `evo-script-engine` v0.

La autoridad sintáctica pertenece a Evo-Script v0.1 y a `evo-script/EFN_HOST_BOUNDARY_v0.1.md` para la frontera `.efn` / Host.

```text
Token Sequence
    ↓
Parser
    ↓
AST
    ↓
Semantic Analyzer
    ↓
Semantic Program
```

## AD-001 — AST representa estructura sintáctica abstracta completa

Status: CLOSED

`AST` representa exclusivamente la estructura sintáctica abstracta completa de un Evo-Script Program sintácticamente válido.

Debe conservar lo necesario para que Semantic Analyzer resuelva posteriormente el significado sin reconstruir gramática desde Tokens.

Conserva, cuando corresponda:

- jerarquía sintáctica;
- orden sintácticamente significativo;
- nombres e identifiers;
- literales en forma textual necesaria;
- formas sintácticas reconocidas;
- Source Span para identidades y construcciones diagnosticables.

Separación canónica:

```text
AST
    = qué estructura escribió el programa

Semantic Program
    = qué significa después del análisis semántico
```

Invariantes:

1. AST solo existe tras parsing exitoso.
2. Puede existir aunque el programa sea semánticamente inválido.
3. Precedence, associativity y grouping ya están representados en su jerarquía.
4. Semantic Analyzer no reinterpreta Tokens para reconstruir gramática.
5. Delimiters no requieren nodos cuando su información estructural ya fue absorbida.
6. AST no es Concrete Syntax Tree ni Token Sequence decorada.
7. Names y literals pueden continuar borrowing de Source Text durante Compile.
8. Un literal sintáctico no se materializa automáticamente como `evo-values::Value`.
9. AST no contiene FunctionId, slots, resolved identities, bytecode, Opcodes, physical function addresses, Providers ni concrete capabilities.

## AD-002 — AST preserva ocurrencias; no deduplica ni normaliza semántica

Status: CLOSED

AST conserva todas las ocurrencias sintácticas aunque posteriormente resulten duplicadas, incompatibles o semánticamente inválidas.

```text
private fn calculate(...) { ... }
private fn calculate(...) { ... }
```

produce dos declaraciones distinguibles. Semantic Analyzer detecta posteriormente `DuplicateFunctionError`.

Invariantes:

1. Parser no elimina duplicados.
2. Parser no reemplaza una declaración previa por otra posterior.
3. Parser no convierte secuencias sintácticas en semantic maps que destruyan ocurrencias duplicadas.
4. Identity resolution, type resolution y uniqueness pertenecen a Semantic Analyzer.
5. El orden se preserva donde sea significativo: Body Statements, arguments, field initializers, when correspondences y Pipeline Stages.

## AD-003 — Parser / Semantic Analyzer boundary

Status: CLOSED

Criterio de inclusión:

> Una identidad pertenece al AST cuando representa una diferencia estructural observable de la gramática que Semantic Analyzer necesita conocer posteriormente.

Criterio de exclusión:

> Una identidad no pertenece al AST cuando solo expresa resolución, tipo determinado, semantic identity, binding, slot, Provider o significado que requiere conocer otras declaraciones o símbolos.

Ejemplo:

```text
AST Identifier("worker")
        ↓
Semantic Analyzer
        ↓
resolved Parameter / Local / other symbol
```

Parser conserva la forma `worker`; no decide qué símbolo representa.

## AD-004 — `.efn` AST no contiene Host Scope ni `use`

Status: CLOSED

La nueva frontera `.efn` / Host elimina cualquier representación de sesión interactiva dentro del AST.

Explícitamente no existen en AST `.efn`:

```text
Use Node
Use Stage
Scope Node
Active Scope Node
Host Context Node
Current Provider Node
```

`use` ya no es keyword `.efn` y por tanto no genera una construcción AST especial.

La semántica histórica de `enter` como navegación del Active Scope tampoco genera una identidad AST especial. Si `enter` aparece como Identifier de una función/capability explícita, se representa mediante las mismas formas normales de Identifier / Function Call que cualquier otra operación.

Regla canónica:

```text
Interactive Host State
    !=
AST of reusable `.efn`
```

## AD-005 — `this` es Parser-only contextual syntax

Status: CLOSED

`this` permanece como Structural Keyword porque participa en la sintaxis de Pipeline, pero no necesita sobrevivir como AST Node o Expression independiente.

Ejemplo conceptual:

```text
value
|> combine(this, other)
```

Parser valida la posición sintáctica de `this` y produce una forma de Pipeline Stage que ya expresa que el transported Pipeline Data ocupa la posición definida por la gramática.

Conceptualmente:

```text
Source syntax
combine(this, other)
        ↓ Parser
Pipeline Call Stage
├── callee: combine
└── additional argument: other

transported first argument
    = structural property of the stage
```

Por tanto no existen:

```text
ThisExpression
ThisNode
ResolvedThisValue
```

Invariantes:

1. `this` no es binding local.
2. `this` no es Parameter.
3. `this` no es Value.
4. `this` no representa Scope ni Consumer.
5. Parser valida sus restricciones de posición conforme a la gramática.
6. Semantic Analyzer recibe la relación estructural ya expresada por Pipeline Stage y no necesita resolver un símbolo llamado `this`.

## AD-006 — Pipeline AST representa composición de datos

Status: CLOSED at structural level

Dentro de `.efn`, Pipeline representa composición de Pipeline Data y no navegación de Host state.

Forma conceptual:

```text
Pipeline
├── Source Expression
└── Pipeline Stages 1..N
```

La `Source Expression` representa la forma sintáctica que inicia el dato de la composición. Cada Pipeline Stage conserva la operación y sus argumentos sintácticos necesarios.

No existe un segundo channel de Active Scope dentro del Pipeline AST.

Reglas:

- Pipeline preserva el orden textual de stages;
- Pipeline no contiene Use Stage;
- Pipeline no contiene Scope transition;
- `this` puede desaparecer porque su función queda expresada por la estructura del stage;
- la validez semántica del tipo producido/consumido por cada stage pertenece a Semantic Analyzer;
- la resolución del callee como local Function o External Symbol pertenece a Semantic Analyzer.

La forma Rust exacta de `Pipeline`, `PipelineStage` y sus variants sigue pendiente.

## AD-007 — Preliminary AST Inventory

Status: IN ANALYSIS

Inventario estructural actual:

```text
Program
│
├── Import Declaration
├── Struct Definition
├── Enum Definition
└── Function Definition
       │
       └── Function Body
              ├── Body Statements 0..N
              │      ├── Let Binding
              │      └── Operation Statement
              └── Final Return 1
```

Expressions candidatas:

```text
Expression
├── Literal
├── Identifier
├── Unary Expression
├── Binary Expression
├── Field Access
├── Function Call
├── Struct Construction
├── Enum Construction
├── Pipeline
└── When
```

Supporting syntax data candidatos, que no necesariamente son AST Nodes universales:

```text
Identifier
Qualified Name
Type Reference
Visibility
Field Definition
Field Initializer
Parameter
Enum Variant
When Pattern
When Correspondence
Pipeline Stage
Unary Operator
Binary Operator
Literal Kind
Source Span
```

### Function Body structural invariant

La forma AST debe representar que una Function Body sintácticamente válida contiene:

```text
Body Statements 0..N
Final Return     exactly 1 and final
```

AST puede ser semánticamente inválido, pero no debe representar como válido un Function Body que viole su gramática.

### Enum Variant structural forms

El AST debe distinguir sintácticamente:

```text
Simple Variant
Associated Variant
Structured Variant
```

sin resolver todavía los Types referenciados.

### Function Parameter structural forms

El AST debe conservar la diferencia textual entre una Type Reference simple y una forma calificada cuando la gramática la distingue, sin afirmar todavía que la forma calificada resuelve a una Signature.

## 8. Explicitly Excluded from AST

```text
FunctionId
TypeId / resolved type identity
LocalSlot / ParameterSlot
Resolved Signature
Resolved External Symbol
Provider binding
Bytecode
Opcode
Physical function address
Active Scope
Host Session State
Use Node / Use Stage
This Expression
```

## 9. Representation Still Open

Todavía no se cierra:

- exact Rust enum/struct inventory;
- `AstNodeKind` exacto, si se necesita;
- tree with nested owned nodes vs indexed storage;
- `Vec`, NodeId u otra representación física;
- cardinalidades Rust concretas;
- derives;
- visibility;
- Parser / Semantic Analyzer signatures.

TD-010 permite utilizar Vec/Box/allocation dentro del Compilation Working State cuando simplifiquen de forma real la corrección y mantenibilidad; AST Data no debe optimizar estas allocations como dogma antes de decidir la representación más clara.

## 10. Current Closure

```text
AST syntactic responsibility              ✅ CLOSED
Occurrence preservation                   ✅ CLOSED
Parser / Semantic Analyzer boundary       ✅ CLOSED
No Host Scope / no `use` in AST           ✅ CLOSED
`this` consumed by Parser                 ✅ CLOSED
Pipeline = data composition               ✅ CLOSED
Exact AST inventory / Rust representation ← IN ANALYSIS
```
