# Evo-Script Engine — AST Function Definitions

Status: CLOSED

Este documento cierra las identidades AST relacionadas con Function Definitions y Function Body de archivos `.efn`.

La autoridad sintáctica deriva de Evo-Script v0.1, `AST_DATA.md` y la política `Earliest Responsible Failure`.

## 1. Closed Identities

Representaciones Rust cerradas:

```rust
struct FunctionDefinition<'source> {
    visibility: Visibility,
    name: Identifier<'source>,
    parameters: Vec<Parameter<'source>>,
    result_type: Identifier<'source>,
    satisfaction: Option<QualifiedName<'source>>,
    body: FunctionBody<'source>,
}

enum Parameter<'source> {
    Value(TypedBinding<'source>),
    SignatureDependency {
        signature: QualifiedName<'source>,
        name: Identifier<'source>,
    },
}

struct FunctionBody<'source> {
    statements: Vec<BodyStatement<'source>>,
    result: Expression<'source>,
}

enum BodyStatement<'source> {
    Let(LetBinding<'source>),
    Operation(OperationStatement<'source>),
}

struct LetBinding<'source> {
    binding: TypedBinding<'source>,
    value: Expression<'source>,
}
```

`OperationStatement` queda cerrado conceptualmente como una alternativa entre Function Call y Pipeline, pero sus payloads Rust concretos se completarán junto con `Expression`, `FunctionCall` y `Pipeline` porque dependen de la representación recursiva del AST.

```text
OperationStatement
├── FunctionCall
└── Pipeline
```

## 2. FunctionDefinition

Forma estructural:

```text
FunctionDefinition
├── Visibility
├── name
├── Parameter 0..N
├── result_type
├── satisfaction 0..1
└── FunctionBody
```

Reglas:

1. `visibility` es siempre explícita y utiliza `Visibility::Public | Visibility::Private`.
2. `name` permanece como Identifier sintáctico.
3. `parameters` conserva una única lista posicional común; no se separan Value Parameters y Signature Dependency Parameters en colecciones distintas.
4. `result_type` permanece como Identifier sintáctico; Parser no resuelve Native Type, Struct o Enum.
5. `satisfaction` utiliza directamente `Option<QualifiedName>`; no se introduce wrapper `SignatureSatisfaction` sin responsabilidad adicional.
6. Function Definition no conserva FunctionId, resolved Signature ni otra identidad semántica.

## 3. Parameter

`Parameter` representa exactamente las dos formas sintácticas v0:

```text
Value Parameter
    tipo nombre

Signature Dependency Parameter
    qualifier::signature nombre_local
```

La variante `Value` reutiliza `TypedBinding` porque introduce un Value binding tipado.

La variante `SignatureDependency` conserva únicamente:

```text
signature: QualifiedName
name: Identifier
```

Parser no valida que `signature` exista, haya sido importada o corresponda realmente a una Signature publicada. Esa resolución pertenece a Semantic Analyzer.

La colección `parameters: Vec<Parameter>` conserva estrictamente el orden posicional escrito, incluso cuando ambas formas aparecen intercaladas.

## 4. FunctionBody

La gramática v0 posee la invariante estructural:

```text
Body Statements 0..N
Final Return     exactly 1 and final
```

El AST la representa como:

```text
FunctionBody
├── statements: BodyStatement 0..N
└── result: Expression exactly 1
```

### `return` no sobrevive como AST Node

`return expression;` es sintaxis estructural consumida por Parser. Una vez validada su obligatoriedad y posición final, el AST conserva directamente la Expression resultante en `FunctionBody.result`.

Por tanto no existen:

```text
ReturnStatement
ReturnNode
EarlyReturn
ReturnKind
```

La forma de `FunctionBody` hace imposible representar como AST válido:

```text
zero returns
multiple returns
early return
statement after return
return without expression
```

Estas invalideces se rechazan en Parser conforme a Earliest Responsible Failure.

## 5. BodyStatement

`BodyStatement` representa exactamente las dos categorías previas al resultado final:

```text
Let Binding
Operation Statement
```

No existe Statement universal que admita cualquier Expression.

Explícitamente no existen en v0:

```text
ExpressionStatement
ReturnStatement
AssignmentStatement
IfStatement
LoopStatement
```

## 6. LetBinding

Forma:

```text
let tipo nombre = expression;
```

AST:

```text
LetBinding
├── binding: TypedBinding
└── value: Expression
```

Los tokens `let`, `=`, y `;` son absorbidos por Parser una vez que su efecto estructural quedó expresado.

`value` conserva la Expression sintáctica sin type resolution ni materialización semántica.

## 7. OperationStatement

Evo-Script v0.1 permite únicamente:

```text
function_call;
pipeline_expression;
```

No se modela como un wrapper general sobre `Expression`, porque eso permitiría estados AST que la gramática prohíbe, como literals, binary expressions o field access usados como statements.

La identidad conceptual queda cerrada:

```text
OperationStatement
├── FunctionCall
└── Pipeline
```

La representación física de ambas variantes se cerrará junto con el modelo recursivo de Expressions.

## 8. Source Span Policy

No se agrega Source Span completo por costumbre a FunctionDefinition, FunctionBody, Parameter, BodyStatement o LetBinding.

Los Identifiers y Expressions contenidos conservan los Source Spans necesarios para diagnostics. Un span adicional solo se agregará si aparece una necesidad diagnóstica concreta que no pueda derivarse de los datos existentes.

## 9. Explicitly Excluded

```text
ReturnStatement
Statement universal
ExpressionStatement
ParameterKind separado
SignatureSatisfaction wrapper
separate Value Parameter list
separate Dependency Parameter list
FunctionId
resolved result Type
resolved Signature
LocalSlot
ParameterSlot
```

## 10. Closure

```text
FunctionDefinition        ✅ CLOSED
Parameter                 ✅ CLOSED
FunctionBody              ✅ CLOSED
BodyStatement             ✅ CLOSED
LetBinding                ✅ CLOSED
return parser-only        ✅ CLOSED
OperationStatement shape  ✅ CLOSED
Operation payload layout  → closes with Expression/Pipeline model
```
