# Evo-Script Engine — Semantic Expressions

Status: CLOSED

Este documento cierra el significado ejecutable interno de `SemanticFunction` para `evo-script-engine` v0.

La autoridad deriva de `SEMANTIC_PROGRAM_DATA.md`, `SEMANTIC_PROGRAM_STRUCTURE.md`, `TECHNICAL_DESIGN.md` y la especificación Evo-Script v0.1.

## 1. SemanticExpression

```rust
struct SemanticExpression {
    type_id: TypeId,
    kind: SemanticExpressionKind,
    span: SourceSpan,
}
```

Invariantes:

1. Toda `SemanticExpression` posee `TypeId` completamente resuelto.
2. Bytecode Compiler no realiza type inference ni name resolution.
3. `span` conserva ubicación para Source Mapping / diagnostics.
4. Semantic Program solo existe después de semantic analysis exitoso.

## 2. SemanticLiteral

Representación cerrada después de revisión:

```rust
enum SemanticLiteral {
    Integer(String),
    Floating(f64),
    Boolean(bool),
    String(String),
}
```

### Integer

`Integer(String)` contiene la magnitud decimal canónica ya validada por Semantic Analyzer. No es el lexeme fuente sin procesar.

La elección de `String` evita limitar Semantic Program a `u128`, porque Evo-Script `dynamic` admite enteros de precisión arbitraria. La representación física runtime de esos enteros pertenece a VM / Value Data posterior.

```text
SemanticLiteral::Integer("100")
+
SemanticExpression.type_id
    → int / int64 / uint128 / dynamic / ... según resolución
```

Un literal negativo continúa representándose mediante `UnaryOperator::Negate` aplicado sobre la magnitud Integer cuando corresponda; no se introduce aquí una representación física signed.

### Floating / Boolean / String

`Floating(f64)` conserva el valor semántico flotante ya validado. `Boolean(bool)` conserva el valor lógico. String literals se materializan como `String` owned; escapes ya fueron interpretados.

Bytecode Compiler puede transformar/copiar estos datos hacia el Constant Pool persistente.

## 3. SemanticExpressionKind

```rust
enum SemanticExpressionKind {
    Literal(SemanticLiteral),

    Binding(BindingId),

    Unary {
        operator: UnaryOperator,
        operand: Box<SemanticExpression>,
    },

    Binary {
        left: Box<SemanticExpression>,
        operator: BinaryOperator,
        right: Box<SemanticExpression>,
    },

    Conversion {
        operand: Box<SemanticExpression>,
    },

    FieldAccess {
        receiver: Box<SemanticExpression>,
        field: FieldId,
    },

    Call(SemanticCall),

    StructConstruction {
        fields: Vec<SemanticFieldValue>,
    },

    EnumConstruction {
        variant: VariantId,
        payload: SemanticEnumPayload,
    },

    When(SemanticWhen),
}
```

No existe `SemanticIdentifier`, `SemanticQualifiedName` ni `SemanticPipeline`.

## 4. Value reference

Una referencia local resuelta se representa directamente mediante:

```rust
Binding(BindingId)
```

El nombre textual ya cumplió su responsabilidad durante semantic resolution.

## 5. Unary and Binary Expressions

Se reutilizan `UnaryOperator` y `BinaryOperator` cerrados en AST porque la operación conserva el mismo significado.

No se crean `SemanticUnaryOperator` ni `SemanticBinaryOperator`.

## 6. Conversion

Las operaciones oficiales `to_tipo` pertenecen a la semántica del lenguaje y no son Internal Functions, External Signatures ni Signature Dependencies.

Representación:

```rust
Conversion {
    operand: Box<SemanticExpression>,
}
```

La conversión queda completamente determinada por:

```text
source type = operand.type_id
target type = enclosing SemanticExpression.type_id
```

Por tanto no se introduce `ConversionKind`, `BuiltinFunctionId` ni una variant por cada `to_int64`, `to_float32`, etc.

Ejemplo:

```text
to_int64(value)
```

```text
SemanticExpression
├── type_id = TypeId(int64)
└── Conversion
    └── operand
        └── type_id = TypeId(source)
```

Semantic Analyzer ya comprobó que la conversión está permitida por Evo-Script. Bytecode/VM decide en runtime si una conversión potencialmente fallable produce `ConversionError`.

## 7. FieldAccess

```rust
FieldAccess {
    receiver: Box<SemanticExpression>,
    field: FieldId,
}
```

El owner se obtiene del `type_id` del receiver y el tipo resultante del `type_id` de la expresión exterior.

No se conserva el nombre textual del field.

## 8. SemanticCallTarget

```rust
enum SemanticCallTarget {
    Internal(FunctionId),
    DirectSignature(SignatureId),
    SignatureDependency(SignatureBindingId),
}
```

```text
Internal Function
    != Signature Definition
    != local Signature Dependency
```

Conversions no utilizan `SemanticCallTarget`.

## 9. SemanticArgument

```rust
enum SemanticArgument {
    Value(SemanticExpression),
    SignatureDependency(SignatureBindingId),
}
```

Signature Dependencies pueden reenviarse posicionalmente sin convertirse en Values de primer orden.

## 10. SemanticCall

```rust
struct SemanticCall {
    target: SemanticCallTarget,
    arguments: Vec<SemanticArgument>,
}
```

Invariantes:

1. `target` está completamente resuelto.
2. `arguments` tienen aridad, orden y compatibilidad válidas.
3. Bytecode Compiler no vuelve a consultar names, aliases o imports.
4. El orden de `arguments` conserva evaluación izquierda-a-derecha.

## 11. Pipeline semantic lowering

Status: CLOSED / CORRECTED

`Pipeline` es sintaxis/AST de composición y no sobrevive como identidad propia en Semantic Program.

La regla corregida es:

> Semantic Analyzer reduce Pipeline a **Semantic Expression Composition** ya resuelta.

Stages ordinarios producen `SemanticCall`; stages `to_tipo` producen `Conversion`.

Ejemplo de calls:

```text
worker
|> validate
|> save
```

```text
Call save
└── Value argument
    └── Call validate
        └── Value argument
            └── Binding worker
```

Ejemplo con conversiones:

```text
source
|> to_int64
|> to_string
```

```text
Conversion → string
└── Conversion → int64
    └── source expression
```

Consecuencias:

```text
No SemanticPipeline
No SemanticPipelineStage
No SemanticThis
No Pipeline-specific lowering in Bytecode Compiler
```

## 12. SemanticFieldValue

```rust
struct SemanticFieldValue {
    field: FieldId,
    value: SemanticExpression,
}
```

Se reutiliza en Struct Construction y Structured Enum Construction.

## 13. Struct Construction

```rust
StructConstruction {
    fields: Vec<SemanticFieldValue>,
}
```

El `TypeId` de la `SemanticExpression` identifica el Struct construido. Semantic Analyzer ya validó field completeness, duplicates, existence y types.

## 14. SemanticEnumPayload

```rust
enum SemanticEnumPayload {
    Simple,
    Associated {
        value: Box<SemanticExpression>,
    },
    Structured {
        fields: Vec<SemanticFieldValue>,
    },
}
```

## 15. Enum Construction

```rust
EnumConstruction {
    variant: VariantId,
    payload: SemanticEnumPayload,
}
```

El `TypeId` exterior identifica el Enum owner y `VariantId` la variante resuelta.

## 16. SemanticWhen

```rust
struct SemanticWhen {
    subject: Box<SemanticExpression>,
    branches: Vec<SemanticWhenBranch>,
}
```

`when` sí sobrevive porque representa branching semántico real.

## 17. SemanticWhenBranch

```rust
struct SemanticWhenBranch {
    variant: VariantId,
    extraction: SemanticVariantExtraction,
    result: SemanticExpression,
}
```

Semantic Analyzer ya garantizó enum owner, exhaustiveness, no duplicates, result compatibility y extraction correctness.

## 18. SemanticVariantExtraction

```rust
enum SemanticVariantExtraction {
    Simple,
    Associated {
        binding: BindingId,
    },
    Structured {
        fields: Vec<SemanticFieldBinding>,
    },
}
```

No sobreviven AST `WhenPattern`, `TypedBinding` o `PatternField`.

## 19. SemanticFieldBinding

```rust
struct SemanticFieldBinding {
    field: FieldId,
    binding: BindingId,
}
```

El type del binding vive en `SemanticFunction.bindings[BindingId].type_id`.

## 20. SemanticFunctionBody

```rust
struct SemanticFunctionBody {
    statements: Vec<SemanticStatement>,
    result: SemanticExpression,
}
```

`return` no reaparece como nodo semántico.

## 21. SemanticStatement

```rust
enum SemanticStatement {
    Bind {
        binding: BindingId,
        value: SemanticExpression,
    },

    Operation(SemanticExpression),
}
```

`Bind` representa `let` ya resuelto.

Un `Operation` válido proviene de un Operation Statement sintácticamente válido. Después de semantic lowering, su outer kind puede ser:

```text
Call
Conversion
```

No se introduce `SemanticOperationStatement` adicional.

## 22. SourceSpan Policy

`SemanticExpression.span` se conserva para que Bytecode Compiler produzca Source Mapping sin volver al AST.

No se agregan spans completos por costumbre a todas las identidades semánticas.

## 23. Semantic reduction from AST

```text
AST Identifier
    → BindingId / resolved Call Target

AST QualifiedName
    → TypeId / VariantId / SignatureId

AST integer literal lexeme
    → canonical SemanticLiteral::Integer(String)

AST other literal lexeme
    → materialized SemanticLiteral

AST to_tipo(...)
    → Conversion

AST Field name
    → FieldId

AST Enum variant syntax
    → TypeId + VariantId + SemanticEnumPayload

AST Pipeline
    → Semantic Expression Composition

AST WhenPattern
    → VariantId + SemanticVariantExtraction

AST typed extraction
    → BindingId / SemanticFieldBinding
```

## 24. Explicitly Excluded

```text
SemanticIdentifier
SemanticQualifiedName
SemanticPipeline
SemanticPipelineStage
SemanticWhenPattern
SemanticTypedBinding
SemanticFieldName lookup
BuiltinFunctionId for conversions
ConversionKind per target name
Function Value
Closure generated for Signature Dependency
Provider identity
Runtime binding
ExternalSymbolId
ParameterSlot / LocalSlot
FieldOffset
runtime discriminant
bytecode / Opcode
```

## 25. Closure

```text
SemanticExpression                  ✅ CLOSED
SemanticExpression.type_id          ✅ CLOSED
SemanticExpression.span             ✅ CLOSED
SemanticExpressionKind              ✅ CLOSED
SemanticLiteral                     ✅ CLOSED — arbitrary integer safe
Conversion                          ✅ CLOSED
Binding reference                   ✅ CLOSED
Unary / Binary semantic reuse       ✅ CLOSED
FieldAccess                         ✅ CLOSED
SemanticCallTarget                  ✅ CLOSED
SemanticArgument                    ✅ CLOSED
SemanticCall                        ✅ CLOSED
Pipeline semantic lowering          ✅ CLOSED — expression composition
SemanticFieldValue                  ✅ CLOSED
Struct Construction                 ✅ CLOSED
SemanticEnumPayload                 ✅ CLOSED
Enum Construction                   ✅ CLOSED
SemanticWhen                        ✅ CLOSED
SemanticWhenBranch                  ✅ CLOSED
SemanticVariantExtraction           ✅ CLOSED
SemanticFieldBinding                ✅ CLOSED
SemanticFunctionBody                ✅ CLOSED
SemanticStatement                   ✅ CLOSED
SourceSpan propagation              ✅ CLOSED
```
