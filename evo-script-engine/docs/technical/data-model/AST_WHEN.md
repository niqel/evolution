# Evo-Script Engine — AST When

Status: CLOSED

Este documento cierra la representación AST de la expresión exhaustiva `when` de Evo-Script v0.1.

La autoridad sintáctica deriva de Evo-Script v0.1, `AST_DATA.md`, `AST_EXPRESSIONS.md` y la política `Earliest Responsible Failure`.

## 1. Closed Identities

Representaciones Rust cerradas:

```rust
struct WhenExpression<'source> {
    subject: Box<Expression<'source>>,
    correspondences: Vec<WhenCorrespondence<'source>>,
}

struct WhenCorrespondence<'source> {
    pattern: WhenPattern<'source>,
    result: Expression<'source>,
}

enum WhenPattern<'source> {
    Simple {
        variant: QualifiedName<'source>,
    },
    Associated {
        variant: QualifiedName<'source>,
        binding: TypedBinding<'source>,
    },
    Structured {
        variant: QualifiedName<'source>,
        fields: Vec<PatternField<'source>>,
    },
}

struct PatternField<'source> {
    field: Identifier<'source>,
    binding: TypedBinding<'source>,
}
```

`ExpressionKind` completa su variante `When` así:

```rust
When(WhenExpression<'source>)
```

## 2. WhenExpression

Forma estructural:

```text
WhenExpression
├── subject: Expression
└── correspondences: WhenCorrespondence 1..N
```

`subject` puede ser cualquier Expression cuyo tipo semántico resulte posteriormente ser un enum.

`subject` utiliza `Box<Expression>` porque existe recursión directa:

```text
Expression
→ WhenExpression
→ Expression
```

`correspondences` utiliza `Vec` porque conserva ocurrencias y orden textual y la colección ya aporta indirection física.

Parser no produce un `when` sin correspondencias.

## 3. WhenCorrespondence

Cada correspondencia representa exactamente:

```text
pattern => expression
```

AST:

```text
WhenCorrespondence
├── pattern: WhenPattern
└── result: Expression
```

`result` no requiere `Box` adicional porque la recursión ya está interrumpida por `WhenExpression.correspondences: Vec<WhenCorrespondence>`.

El marcador `=>` es consumido por Parser una vez expresada la relación estructural entre pattern y result.

## 4. WhenPattern

`WhenPattern` representa exactamente las tres formas sintácticas de inspección de enum permitidas en v0.1:

```text
Simple
Associated
Structured
```

No es un framework general de pattern matching.

### 4.1 Simple

Forma:

```text
EnumType::Variant
```

Representación:

```rust
Simple {
    variant: QualifiedName<'source>,
}
```

### 4.2 Associated

Forma:

```text
EnumType::Variant(Type local_name)
```

Representación:

```rust
Associated {
    variant: QualifiedName<'source>,
    binding: TypedBinding<'source>,
}
```

`TypedBinding` se reutiliza porque la extracción introduce exactamente un Value binding tipado local a la correspondencia.

Parser conserva el tipo y nombre escritos, pero no valida que el payload real de la variante posea ese tipo. Esa comparación pertenece a Semantic Analyzer.

### 4.3 Structured

Forma:

```text
EnumType::Variant {
    field_name: Type local_name;
    ...
}
```

Representación:

```rust
Structured {
    variant: QualifiedName<'source>,
    fields: Vec<PatternField<'source>>,
}
```

La colección preserva orden y duplicados. Parser reconoce la estructura; Semantic Analyzer valida que los fields correspondan exactamente a la variante resuelta y que la extracción sea completa.

## 5. PatternField

`PatternField` representa dos identidades diferentes presentes en una extracción estructurada:

```text
message: string error_message;
```

se modela como:

```text
PatternField
├── field: message
└── binding
    ├── type_name: string
    └── name: error_message
```

Representación:

```rust
struct PatternField<'source> {
    field: Identifier<'source>,
    binding: TypedBinding<'source>,
}
```

`field` identifica un field estructural existente que se intenta extraer. `binding` introduce el nuevo Value binding local.

`PatternField` no reutiliza `FieldDefinition`: `FieldDefinition` define estructura de datos; `PatternField` referencia un field existente e introduce un binding local.

## 6. Qualified Variant Syntax

Las tres formas de `WhenPattern` utilizan:

```rust
variant: QualifiedName<'source>
```

porque la sintaxis v0.1 exige la referencia completa:

```text
EnumType::Variant
```

Parser no resuelve todavía si `EnumType` es realmente un enum ni si `Variant` pertenece a ese enum.

## 7. Parser / Semantic Analyzer Boundary

Parser valida únicamente lo que puede determinar por estructura sintáctica:

```text
when syntax
subject Expression syntax
correspondence syntax
Simple / Associated / Structured shape
qualified variant syntax
typed extraction syntax
```

Semantic Analyzer valida significado resuelto:

```text
subject type is enum
EnumType::Variant exists
variant belongs to subject enum
exhaustiveness
no duplicate variant correspondence
Associated payload shape/type compatibility
Structured field existence
Structured field completeness
Structured field duplication
extracted binding type compatibility
binding visibility and no-shadowing rules
common semantic result type for all correspondences
```

En particular, exhaustividad no pertenece a Parser porque requiere conocer la definición resuelta del enum inspeccionado.

## 8. Correspondence-local Bindings

Los bindings introducidos por `Associated` y `Structured` existen únicamente durante el análisis/evaluación del `result` de su propia `WhenCorrespondence`.

No se introduce ningún AST node de scope:

```text
WhenScope
PatternScope
BindingScope
```

La visibilidad de esos bindings pertenece al semantic environment temporal del Semantic Analyzer.

Conceptualmente:

```text
outer semantic environment
        │
        ▼
WhenCorrespondence
├── pattern introduces local bindings
└── result analyzed with those bindings
```

## 9. Source Span Policy

`WhenExpression` no agrega SourceSpan propio porque ya está contenido por:

```rust
Expression {
    kind: ExpressionKind::When(...),
    span: SourceSpan,
}
```

`WhenCorrespondence`, `WhenPattern` y `PatternField` tampoco agregan span completo por costumbre. Los Identifiers, QualifiedNames, TypedBindings y result Expressions contenidos proporcionan ubicaciones diagnosticables suficientes para v0.

## 10. Explicitly Excluded

```text
General Pattern framework
Wildcard pattern
Default / otherwise / else pattern
Guard pattern
Range pattern
Nested pattern
Or pattern
Tuple pattern
Pattern alias / @ binding
WhenScope AST node
PatternScope AST node
FieldDefinition reuse for PatternField
WhenPatternKind separate enum
```

## 11. Closure

```text
WhenExpression                  ✅ CLOSED
WhenCorrespondence              ✅ CLOSED
WhenPattern                      ✅ CLOSED
PatternField                     ✅ CLOSED
Simple pattern                   ✅ CLOSED
Associated pattern               ✅ CLOSED
Structured pattern               ✅ CLOSED
Correspondence-local binding     ✅ CLOSED
Parser / Semantic boundary       ✅ CLOSED
ExpressionKind::When payload     ✅ CLOSED
```
