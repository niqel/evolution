# Evo-Script Engine — AST Expressions

Status: CLOSED

Este documento cierra la representación AST de Expressions de Evo-Script v0.1. El modelo interno de `when` se detalla en `AST_WHEN.md`.

La representación recursiva se rige por `AST_EXPRESSION_REPRESENTATION.md`: typed nested tree, `Box<Expression>` únicamente cuando rompe recursión directa y `Vec<...>` cuando la colección ya aporta indirection.

## 1. Expression

Representación Rust cerrada:

```rust
struct Expression<'source> {
    kind: ExpressionKind<'source>,
    span: SourceSpan,
}
```

`Expression.span` identifica la construcción evaluable completa. Se conserva porque puede abarcar más texto que los Identifiers internos, incluyendo grouping absorbido por Parser.

Ejemplo:

```text
((worker))
```

puede producir un `ExpressionKind::Identifier`, mientras `Expression.span` conserva el rango completo de la expresión y `Identifier.span` únicamente el rango de `worker`.

No existe `ParenthesizedExpression` ni `GroupingExpression`: Parser absorbe grouping y precedence en la jerarquía AST.

## 2. ExpressionKind

Forma cerrada:

```rust
enum ExpressionKind<'source> {
    Literal {
        kind: LiteralKind,
        lexeme: &'source str,
    },
    Identifier(Identifier<'source>),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression<'source>>,
    },
    Binary {
        left: Box<Expression<'source>>,
        operator: BinaryOperator,
        right: Box<Expression<'source>>,
    },
    FieldAccess {
        receiver: Box<Expression<'source>>,
        field: Identifier<'source>,
    },
    FunctionCall(FunctionCall<'source>),
    StructConstruction {
        type_name: Identifier<'source>,
        fields: Vec<FieldInitializer<'source>>,
    },
    EnumConstruction(EnumConstruction<'source>),
    Pipeline(Pipeline<'source>),
    When(WhenExpression<'source>),
}
```

`WhenExpression` y sus patterns se definen en `AST_WHEN.md`.

## 3. LiteralKind

```rust
enum LiteralKind {
    Integer,
    Floating,
    String,
    Boolean,
}
```

El AST conserva clasificación sintáctica + lexeme. Parser no materializa `evo_values::Value` ni fija anticipadamente el tipo semántico contextual de un literal numérico.

No se introduce un wrapper `Literal` independiente porque no agrega responsabilidad propia.

## 4. UnaryOperator

```rust
enum UnaryOperator {
    Not,
    Negate,
}
```

Parser transforma `TokenKind::Minus` en `UnaryOperator::Negate` cuando su posición gramatical es unary. El AST no reutiliza TokenKind como operador semántico-sintáctico.

## 5. BinaryOperator

```rust
enum BinaryOperator {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
}
```

`Pipeline` y `FieldAccess` no son variants de `BinaryOperator`; poseen estructura AST propia.

## 6. FieldAccess

```rust
FieldAccess {
    receiver: Box<Expression<'source>>,
    field: Identifier<'source>,
}
```

`receiver` puede ser cualquier Expression, por lo que formas como:

```text
country.state.name
find_worker(id).name
```

se representan naturalmente por composición recursiva. No se reduce Field Access a `Vec<Identifier>`.

## 7. FunctionCall

```rust
struct FunctionCall<'source> {
    callee: Identifier<'source>,
    arguments: Vec<Expression<'source>>,
}
```

`FunctionCall` posee identidad propia porque la misma construcción sintáctica participa tanto como Expression como `OperationStatement`.

`callee` es un nombre local resoluble. La resolución hacia Internal Function o Signature Dependency pertenece a Semantic Analyzer.

## 8. FieldInitializer

```rust
struct FieldInitializer<'source> {
    name: Identifier<'source>,
    value: Expression<'source>,
}
```

Se reutiliza en Struct Construction y Structured Enum Construction porque ambas comparten la misma construcción sintáctica `field_name: expression`.

Las colecciones de Field Initializers preservan orden y duplicados. Unknown, Missing, Duplicate Field y type compatibility pertenecen a Semantic Analyzer.

## 9. Struct Construction

Se representa inline dentro de `ExpressionKind`:

```rust
StructConstruction {
    type_name: Identifier<'source>,
    fields: Vec<FieldInitializer<'source>>,
}
```

No se introduce `StructConstruction` como struct separado porque en v0 no posee una segunda responsabilidad o contexto de reutilización.

`type_name` permanece sin resolver.

## 10. EnumConstruction

```rust
enum EnumConstruction<'source> {
    Simple {
        variant: QualifiedName<'source>,
    },
    Associated {
        variant: QualifiedName<'source>,
        value: Box<Expression<'source>>,
    },
    Structured {
        variant: QualifiedName<'source>,
        fields: Vec<FieldInitializer<'source>>,
    },
}
```

Las tres variants corresponden directamente a las tres formas sintácticas de construcción de enum.

`Associated.value` utiliza `Box` porque introduce recursión directa `Expression -> EnumConstruction -> Expression`. `Structured.fields` no necesita `Box` adicional porque `Vec` ya aporta indirection.

## 11. Pipeline

```rust
struct Pipeline<'source> {
    source: Box<Expression<'source>>,
    stages: Vec<PipelineStage<'source>>,
}

struct PipelineStage<'source> {
    callee: Identifier<'source>,
    additional_arguments: Vec<Expression<'source>>,
}
```

Invariantes:

1. `Pipeline.stages` contiene conceptualmente `1..N` stages; Parser no produce un Pipeline vacío.
2. `source` es la Expression que inicia Pipeline Data.
3. `callee` permanece como Identifier no resuelto.
4. `this` no sobrevive al AST.
5. En una stage multi-argumento, transported Pipeline Data ocupa estructuralmente la posición definida por la gramática y `additional_arguments` conserva únicamente los argumentos adicionales.
6. No existe `PipelineStageKind` en v0.

No se introduce `NonEmptyVec`; Parser garantiza la cardinalidad estructural bajo Earliest Responsible Failure.

## 12. OperationStatement — payload layout complete

La identidad pendiente de `AST_FUNCTION_DEFINITIONS.md` queda completada:

```rust
enum OperationStatement<'source> {
    FunctionCall(FunctionCall<'source>),
    Pipeline(Pipeline<'source>),
}
```

No es wrapper sobre `Expression`, porque Evo-Script v0.1 prohíbe Expression Statements generales.

## 13. When

`when` participa como Expression mediante:

```rust
When(WhenExpression<'source>)
```

Su modelo cerrado incluye:

```text
WhenExpression
WhenCorrespondence
WhenPattern
PatternField
```

Las formas de pattern son exactamente `Simple`, `Associated` y `Structured`. Exhaustividad, resolución de variantes, compatibilidad de payloads y scopes semánticos de bindings pertenecen a Semantic Analyzer.

La definición completa se encuentra en [`AST_WHEN.md`](./AST_WHEN.md).

## 14. Explicitly Excluded

```text
ParenthesizedExpression
GroupingExpression
Literal wrapper struct
TypeReference
NativeType AST enum
ExpressionStatement
PipelineStageKind
ThisExpression
ExpressionId
AST Arena
Generic AstNode
Generic NodeId
General Pattern framework
```

## 15. Closure

```text
Expression                    ✅ CLOSED
Expression.span               ✅ CLOSED
ExpressionKind inventory      ✅ CLOSED
LiteralKind                   ✅ CLOSED
UnaryOperator                 ✅ CLOSED
BinaryOperator                ✅ CLOSED
FieldAccess                   ✅ CLOSED
FunctionCall                  ✅ CLOSED
FieldInitializer              ✅ CLOSED
StructConstruction            ✅ CLOSED
EnumConstruction              ✅ CLOSED
Pipeline                      ✅ CLOSED
PipelineStage                 ✅ CLOSED
OperationStatement payloads   ✅ CLOSED
When model                    ✅ CLOSED

AST Expressions               ✅ CLOSED
```
