# Evo-Script Engine — Semantic Expressions

Status: CLOSED

Este documento cierra el significado ejecutable interno de `SemanticFunction` para `evo-script-engine` v0.

La autoridad deriva de `SEMANTIC_PROGRAM_DATA.md`, `SEMANTIC_PROGRAM_STRUCTURE.md`, `TECHNICAL_DESIGN.md` y la especificación Evo-Script v0.1.

## 1. SemanticExpression

Representación cerrada:

```rust
struct SemanticExpression {
    type_id: TypeId,
    kind: SemanticExpressionKind,
    span: SourceSpan,
}
```

Invariantes:

1. Toda `SemanticExpression` posee un `TypeId` completamente resuelto.
2. Bytecode Compiler no realiza type inference ni name resolution sobre Semantic Program.
3. `span` conserva la ubicación de la expresión fuente para Source Mapping / diagnostics posteriores.
4. El Semantic Program solo existe después de semantic analysis exitoso; por tanto sus expresiones ya satisfacen las reglas semánticas de tipos, aridad, access, construction y `when`.

## 2. SemanticLiteral

Representación cerrada:

```rust
enum SemanticLiteral {
    Integer(u128),
    Floating(f64),
    Boolean(bool),
    String(String),
}
```

El `TypeId` de la `SemanticExpression` determina el tipo semántico concreto del literal.

Ejemplo:

```text
SemanticLiteral::Integer(10)
+
SemanticExpression.type_id
    → int / int8 / int64 / uint32 / ... según resolución semántica
```

Reglas:

1. El lexeme del AST deja de ser mecanismo de compilación.
2. String literals se materializan como `String` owned en Semantic Program; escapes y significado textual ya fueron interpretados por Semantic Analyzer.
3. El Bytecode Compiler puede transferir/copiar los datos persistentes necesarios al Constant Pool del Compiled Program.
4. Un literal negativo continúa representándose mediante `UnaryOperator::Negate` aplicado sobre una magnitud Integer cuando corresponda; no se introduce una segunda forma signed dentro de `SemanticLiteral`.

## 3. SemanticExpressionKind

Representación cerrada:

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

`BindingId` se interpreta exclusivamente dentro del namespace de la `SemanticFunction` owner.

## 5. Unary and Binary Expressions

Se reutilizan las identidades cerradas del AST:

```text
UnaryOperator
BinaryOperator
```

No se crean `SemanticUnaryOperator` ni `SemanticBinaryOperator` porque la alternativa de operación ya está resuelta y conserva el mismo significado.

La validez de operandos y el tipo resultante ya fueron comprobados antes de producir Semantic Program.

## 6. FieldAccess

Representación:

```rust
FieldAccess {
    receiver: Box<SemanticExpression>,
    field: FieldId,
}
```

El owner semántico del field se obtiene a partir del `type_id` del receiver.

El tipo resultante se obtiene de `SemanticExpression.type_id`.

No se conserva el nombre textual del field como mecanismo de resolución.

```text
AST
worker.name
    ↓ Semantic Analyzer
receiver → BindingId(...)
field    → FieldId(...)
    ↓
Semantic FieldAccess
```

## 7. SemanticCallTarget

Una llamada resuelta posee exactamente uno de tres targets:

```rust
enum SemanticCallTarget {
    Internal(FunctionId),
    DirectSignature(SignatureId),
    SignatureDependency(SignatureBindingId),
}
```

### Internal

Identifica una Function Implementation interna del mismo Semantic Program.

### DirectSignature

Identifica una Signature importada utilizada mediante llamada directa. El nombre o alias textual ya fue resuelto hacia `SignatureId`.

### SignatureDependency

Identifica una Signature Dependency Parameter concreta de la función actual mediante `SignatureBindingId`.

Esta separación conserva:

```text
Internal Function
    != Signature Definition
    != local Signature Dependency
```

No se introduce Provider ni runtime external binding.

## 8. SemanticArgument

Signature Dependencies pueden reenviarse posicionalmente sin convertirse en Values de primer orden.

Representación cerrada:

```rust
enum SemanticArgument {
    Value(SemanticExpression),
    SignatureDependency(SignatureBindingId),
}
```

La colección de argumentos preserva el orden posicional de la invocación.

No existe Function Value ni closure sintética para transportar una Signature Dependency.

## 9. SemanticCall

Representación cerrada:

```rust
struct SemanticCall {
    target: SemanticCallTarget,
    arguments: Vec<SemanticArgument>,
}
```

Invariantes:

1. `target` ya está completamente resuelto.
2. `arguments` ya poseen aridad, orden y compatibilidad semántica válidas respecto al target.
3. Bytecode Compiler no vuelve a consultar nombres, aliases, imports o Signature declarations para decidir el target.
4. La evaluación izquierda-a-derecha continúa expresada por el orden de `arguments`.

## 10. Pipeline semantic lowering

Status: CLOSED

`Pipeline` es una construcción sintáctica/AST de composición. No sobrevive como identidad propia dentro de Semantic Program.

Ejemplo:

```text
worker
|> validate
|> save
```

AST:

```text
Pipeline
├── source: worker
├── validate
└── save
```

Semantic Program:

```text
Call save
└── Value argument
    └── Call validate
        └── Value argument
            └── Binding worker
```

Regla:

> Semantic Analyzer resuelve Pipeline stages, valida tipos, inserta el transported value en la posición semántica correspondiente y produce composición de `SemanticCall`.

Consecuencias:

```text
No SemanticPipeline
No SemanticPipelineStage
No `this` semantic node
No Pipeline-specific lowering in Bytecode Compiler
```

Bytecode Compiler recibe call composition ya resuelta.

## 11. SemanticFieldValue

Representación cerrada:

```rust
struct SemanticFieldValue {
    field: FieldId,
    value: SemanticExpression,
}
```

Se reutiliza en Struct Construction y Structured Enum Construction.

Los nombres de fields ya fueron resueltos a `FieldId`.

El owner se conoce por el tipo de la construcción y, para enum estructurado, por la variante resuelta.

## 12. Struct Construction

Representación dentro de `SemanticExpressionKind`:

```rust
StructConstruction {
    fields: Vec<SemanticFieldValue>,
}
```

El `TypeId` del `SemanticExpression` identifica el Struct construido.

No se duplica `TypeId` dentro de la variant.

El Semantic Analyzer ya comprobó fields requeridos, duplicados, existencia y compatibilidad de tipos.

## 13. SemanticEnumPayload

Representación cerrada:

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

## 14. Enum Construction

Representación:

```rust
EnumConstruction {
    variant: VariantId,
    payload: SemanticEnumPayload,
}
```

El `TypeId` del `SemanticExpression` identifica el Enum owner.

`VariantId` identifica la variante dentro de dicho enum.

No sobreviven `QualifiedName`, enum name ni variant name como mecanismo de compilación.

## 15. SemanticWhen

`when` sí sobrevive como significado semántico porque representa selección exhaustiva entre variantes y requiere branching en bytecode.

Representación cerrada:

```rust
struct SemanticWhen {
    subject: Box<SemanticExpression>,
    branches: Vec<SemanticWhenBranch>,
}
```

La colección `branches` conserva las correspondencias semánticas ya validadas.

El `TypeId` del subject identifica el Enum inspeccionado.

## 16. SemanticWhenBranch

Representación cerrada:

```rust
struct SemanticWhenBranch {
    variant: VariantId,
    extraction: SemanticVariantExtraction,
    result: SemanticExpression,
}
```

El Semantic Analyzer ya garantizó:

```text
subject is Enum
variant belongs to subject Enum
exhaustiveness
no duplicate branch
result type compatibility
extraction shape/type correctness
```

## 17. SemanticVariantExtraction

Representación cerrada:

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

No sobreviven AST `WhenPattern`, `TypedBinding`, `PatternField` o textual type names.

## 18. SemanticFieldBinding

Representación cerrada:

```rust
struct SemanticFieldBinding {
    field: FieldId,
    binding: BindingId,
}
```

Representa el significado resuelto de una extracción estructurada:

```text
semantic field identity
    → FieldId

local extracted value
    → BindingId
```

El tipo del binding está disponible mediante `SemanticFunction.bindings[BindingId].type_id`.

## 19. SemanticFunctionBody

Representación cerrada:

```rust
struct SemanticFunctionBody {
    statements: Vec<SemanticStatement>,
    result: SemanticExpression,
}
```

`return` no reaparece como nodo semántico. La función posee exactamente un resultado final semánticamente válido.

## 20. SemanticStatement

Representación cerrada:

```rust
enum SemanticStatement {
    Bind {
        binding: BindingId,
        value: SemanticExpression,
    },

    Operation(SemanticExpression),
}
```

### Bind

Representa un `let` ya resuelto. El type y nombre del binding no se repiten aquí; se obtienen mediante `SemanticFunction.bindings[BindingId]` cuando sea necesario.

### Operation

Semantic Analyzer solo produce `Operation` cuando la forma fuente fue un Operation Statement válido.

Como Pipeline ya se reduce a composición de calls, la `SemanticExpressionKind` exterior de un Operation válido es `Call`.

No se introduce `SemanticOperationStatement` separado porque no agrega significado adicional después de semantic success.

## 21. SourceSpan Policy

`SemanticExpression.span` se conserva para que el Bytecode Compiler pueda construir Source Mapping sin volver al AST.

No se agregan SourceSpan completos por costumbre a todas las identidades semánticas. Cuando una ubicación pueda derivarse de una `SemanticExpression` o no sea necesaria para producto persistente, no se duplica.

Regla:

> Semantic Program conserva Source Location cuando es necesaria para traducir significado ejecutable hacia diagnostic/source mapping, no para reproducir Concrete Syntax.

## 22. Semantic reduction from AST

Transformaciones canónicas:

```text
AST Identifier
    → BindingId / resolved Call Target

AST QualifiedName
    → TypeId / VariantId / SignatureId

AST literal lexeme
    → SemanticLiteral

AST Field name
    → FieldId

AST Enum variant syntax
    → TypeId + VariantId + SemanticEnumPayload

AST Pipeline
    → nested SemanticCall

AST WhenPattern
    → VariantId + SemanticVariantExtraction

AST typed extraction
    → BindingId / SemanticFieldBinding
```

Semantic Program conserva significado resuelto y elimina syntax-only identities una vez cumplida su responsabilidad.

## 23. Explicitly Excluded

```text
SemanticIdentifier
SemanticQualifiedName
SemanticPipeline
SemanticPipelineStage
SemanticWhenPattern
SemanticTypedBinding
SemanticFieldName lookup
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

## 24. Closure

```text
SemanticExpression                  ✅ CLOSED
SemanticExpression.type_id          ✅ CLOSED
SemanticExpression.span             ✅ CLOSED
SemanticExpressionKind              ✅ CLOSED
SemanticLiteral                     ✅ CLOSED
Binding reference                   ✅ CLOSED
Unary / Binary semantic reuse       ✅ CLOSED
FieldAccess                         ✅ CLOSED
SemanticCallTarget                  ✅ CLOSED
SemanticArgument                    ✅ CLOSED
SemanticCall                        ✅ CLOSED
Pipeline semantic lowering          ✅ CLOSED
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

Semantic Program exact inventory    ← NEXT
```
