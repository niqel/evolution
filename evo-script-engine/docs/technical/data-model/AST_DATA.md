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

La frontera `.efn` / Host elimina cualquier representación de sesión interactiva dentro del AST.

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

## AD-005 — `this` es Parser-only contextual syntax

Status: CLOSED

`this` permanece como Structural Keyword porque participa en la sintaxis de Pipeline, pero no necesita sobrevivir como AST Node o Expression independiente.

```text
value |> combine(this, other)
        ↓ Parser
Pipeline Call Stage
├── callee: combine
└── additional argument: other
```

El transported first argument es una propiedad estructural del Pipeline Stage.

Por tanto no existen:

```text
ThisExpression
ThisNode
ResolvedThisValue
```

## AD-006 — Pipeline AST representa composición de datos

Status: CLOSED at structural level

Dentro de `.efn`, Pipeline representa composición de Pipeline Data y no navegación de Host state.

```text
Pipeline
├── Source Expression
└── Pipeline Stages 1..N
```

No existe un segundo channel de Active Scope dentro del Pipeline AST.

Reglas:

- Pipeline preserva el orden textual de stages;
- Pipeline no contiene Use Stage;
- Pipeline no contiene Scope transition;
- `this` desaparece una vez expresada su función en la estructura del stage;
- la compatibilidad de tipos y la resolución de callees pertenecen a Semantic Analyzer.

## AD-007 — Preliminary AST Inventory

Status: IN ANALYSIS

Inventario estructural actual:

```text
Program
│
├── Imports 0..N
└── Declarations 1..N
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

`Program` solo existe como AST exitoso cuando sus declaraciones contienen exactamente una `Function Definition` con `Visibility::Public`. La validación de esta cardinalidad estructural pertenece a Parser conforme a AD-009.

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

Supporting syntax data candidatos:

```text
Identifier
Qualified Name
Visibility
Typed Binding
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

```text
Body Statements 0..N
Final Return     exactly 1 and final
```

AST puede ser semánticamente inválido, pero no debe representar como válido un Function Body que viole su gramática.

### Enum Variant structural forms

```text
Simple Variant
Associated Variant
Structured Variant
```

### Function Parameter structural forms

AST conserva la diferencia entre Value Parameter y la forma calificada de Signature Dependency Parameter sin resolver todavía la Signature.

## AD-008 — Foundational AST Syntax Identities

Status: CLOSED

Las identidades sintácticas fundamentales del AST v0 son `Identifier`, `QualifiedName`, `Visibility` y `TypedBinding`.

### Identifier

Representación Rust cerrada:

```rust
struct Identifier<'source> {
    lexeme: &'source str,
    span: SourceSpan,
}
```

`Identifier` representa una ocurrencia sintáctica concreta de un identificador. Conserva el texto borrowed del Source Text y su ubicación, pero no expresa todavía si el nombre corresponde a Type, Function, Signature, Parameter, Local Binding, Enum Variant u otra identidad semántica.

No se conserva `TokenKind::Identifier` dentro del AST porque Parser ya absorbió esa clasificación léxica.

### QualifiedName

Representación Rust cerrada:

```rust
struct QualifiedName<'source> {
    qualifier: Identifier<'source>,
    name: Identifier<'source>,
}
```

Representa exactamente la forma sintáctica v0:

```text
qualifier::name
```

No clasifica todavía la relación como Module::Symbol, Enum::Variant u otra identidad resuelta.

No contiene un `SourceSpan` adicional: el rango total se deriva de `qualifier.span.start` hasta `name.span.end`.

La gramática v0 no introduce qualified paths arbitrarios de tres o más segmentos; si una versión futura lo requiere, esta identidad deberá reabrirse.

### Visibility

Representación Rust cerrada:

```rust
enum Visibility {
    Public,
    Private,
}
```

La visibilidad es explícita en toda Function Implementation `.efn`; no existe estado `Absent` y no se modela mediante `bool`.

### TypedBinding

Representación Rust cerrada:

```rust
struct TypedBinding<'source> {
    type_name: Identifier<'source>,
    name: Identifier<'source>,
}
```

Representa la construcción sintáctica `tipo nombre` cuando dicha forma introduce un Value binding tipado, por ejemplo Value Parameters, Let Bindings y extracciones tipadas de `when`.

`type_name` permanece como Identifier sintáctico. Parser no decide si corresponde a Native Type, local Struct, local Enum o imported shared Type.

No contiene un `SourceSpan` adicional: puede derivarse desde `type_name.span.start` hasta `name.span.end`.

`TypedBinding` no sustituye a `FieldDefinition`: un field de datos y un lexical Value binding mantienen responsabilidades sintácticas distintas aunque ambos usen una forma textual `tipo nombre`.

### Foundational relationship

```text
Source Text
    ▲
    │ borrow
Identifier<'source>
├── lexeme
└── SourceSpan

QualifiedName
├── qualifier ──► Identifier
└── name      ──► Identifier

Visibility
├── Public
└── Private

TypedBinding
├── type_name ──► Identifier
└── name      ──► Identifier
```

## AD-009 — Parser aplica Earliest Responsible Failure a invariantes estructurales de `.efn`

Status: CLOSED

Regla canónica:

> Cada fase del compiler debe rechazar una invalidez en la primera fase que disponga de toda la información necesaria y sea responsable de esa regla. Una invalidez no se transporta artificialmente hacia una fase posterior.

Para `Parser`, esto significa que solo produce `AST` cuando la estructura completa del `.efn` es sintácticamente válida.

Al finalizar el parseo del Program, Parser valida las invariantes estructurales completas que no requieren identity resolution ni type resolution.

En v0, Parser debe detectar directamente al menos:

```text
malformed declaration
malformed expression
invalid import placement
missing final return
multiple / non-final return shape
missing public function
more than one public function
```

La cardinalidad estructural de la función pública queda cerrada así:

```text
Program
├── Imports 0..N
└── Declarations 1..N
      └── exactly one FunctionDefinition
          with Visibility::Public
```

Un `.efn` vacío o compuesto solo por imports, structs, enums o private functions no produce AST exitoso: Parser retorna Syntax Failure al confirmar el final del Program sin una Public Function.

Una segunda Public Function también produce Syntax Failure tan pronto como Parser dispone de evidencia suficiente para confirmar la violación estructural.

Esta regla no mueve validaciones semánticas hacia Parser. Continúan perteneciendo a Semantic Analyzer, entre otras:

```text
DuplicateFunctionError
UnknownTypeError
UnknownSymbolError
argument arity/type compatibility
RecursiveTypeCycleError
FunctionCallCycleError
Signature resolution / satisfaction
```

Parser puede utilizar working state temporal, por ejemplo un contador de Public Functions, sin transportar ese contador al AST.

Invariante resultante:

```text
Parser success
    ⇒ structurally valid Evo-Script AST

Semantic Analyzer success
    ⇒ semantically valid Semantic Program
```

## AD-010 — Program, ImportDeclaration y Declaration

Status: CLOSED

El nivel superior de un `.efn` sintácticamente válido separa imports estáticos de declaraciones locales y conserva el orden textual común de las declaraciones.

Representaciones Rust cerradas:

```rust
struct Program<'source> {
    imports: Vec<ImportDeclaration<'source>>,
    declarations: Vec<Declaration<'source>>,
}

struct ImportDeclaration<'source> {
    symbol: QualifiedName<'source>,
    alias: Option<Identifier<'source>>,
}

enum Declaration<'source> {
    Struct(StructDefinition<'source>),
    Enum(EnumDefinition<'source>),
    Function(FunctionDefinition<'source>),
}
```

### Program

`Program` conserva dos secuencias ordenadas:

```text
Program
├── Imports 0..N
└── Declarations 1..N
```

La colección `declarations` conserva exactamente el orden textual compartido entre Struct Definitions, Enum Definitions y Function Definitions. No se separan prematuramente en colecciones por categoría porque hacerlo destruiría información estructural del Source Text y constituiría una normalización anticipada.

`Program` no conserva:

```text
entry point index
public function index
symbol table
type table
function table
resolved imports
```

Esos datos son working state o identidades semánticas posteriores.

`Program` no requiere `SourceSpan`: el rango del programa corresponde al Source Text completo y no se duplica como dato AST.

### ImportDeclaration

`ImportDeclaration` representa exclusivamente las dos formas sintácticas v0:

```text
import qualifier::name;
import qualifier::name as alias;
```

`symbol` conserva la forma calificada; `alias` conserva únicamente la presencia textual opcional del nombre local.

No existe `ImportKind` en AST. Parser no decide si el símbolo importado es Struct, Enum, Signature u otra identidad semántica publicada. Esa clasificación pertenece a Semantic Analyzer.

`ImportDeclaration` tampoco requiere `SourceSpan` adicional: los Source Spans de sus Identifiers permiten diagnosticar las identidades relevantes sin conservar punctuation o delimiters ya absorbidos por Parser.

### Declaration

`Declaration` representa exactamente las tres clases de definición top-level permitidas dentro de `.efn` después de imports:

```text
Struct Definition
Enum Definition
Function Definition
```

`ImportDeclaration` no es variante de `Declaration`. Esta separación expresa directamente la regla gramatical de que los imports ocurren al inicio y evita que el AST pueda representar imports intercalados con definiciones.

Invariantes:

1. `Program.imports` preserva ocurrencias y orden de imports.
2. `Program.declarations` preserva ocurrencias y orden textual común de definitions.
3. Parser no deduplica imports ni declarations por nombre.
4. `Declaration` no contiene categoría `Import`.
5. No existen declaration maps en AST.
6. Exactly one Public Function continúa garantizado por AD-009 antes de producir `Program`.
7. La representación física mediante `Vec` queda cerrada para estas dos secuencias porque expresa orden, duplicados y cardinalidad variable dentro del Compilation Working State.

## 10. Explicitly Excluded from AST

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
Parser public-function counter
ImportKind
Entry Point Index
AST Symbol Tables
```

## 11. Representation Still Open

Todavía no se cierra:

- exact Rust enum/struct inventory restante;
- tree with nested owned nodes vs indexed storage;
- NodeId u otra representación física para recursive Expression data;
- cardinalidades Rust concretas salvo las ya fijadas por sintaxis;
- derives;
- visibility Rust;
- Parser / Semantic Analyzer signatures.

TD-010 permite utilizar Vec/Box/allocation dentro del Compilation Working State cuando simplifiquen de forma real la corrección y mantenibilidad.

## 12. Current Closure

```text
AST syntactic responsibility              ✅ CLOSED
Occurrence preservation                   ✅ CLOSED
Parser / Semantic Analyzer boundary       ✅ CLOSED
No Host Scope / no `use` in AST           ✅ CLOSED
`this` consumed by Parser                 ✅ CLOSED
Pipeline = data composition               ✅ CLOSED
Identifier                                ✅ CLOSED
QualifiedName                             ✅ CLOSED
Visibility                                ✅ CLOSED
TypedBinding                              ✅ CLOSED
Earliest Responsible Failure              ✅ CLOSED
Exactly one Public Function in Parser     ✅ CLOSED
Program                                   ✅ CLOSED
ImportDeclaration                         ✅ CLOSED
Declaration                               ✅ CLOSED
Exact remaining AST inventory             ← IN ANALYSIS
Recursive AST representation              PENDING
```
