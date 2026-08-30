# Evo-Script Engine — AST Expression Representation

Status: CLOSED

Este documento cierra la representación física v0 de relaciones recursivas dentro de `Expression` para `evo-script-engine`.

La decisión aplica exclusivamente al `Compilation Working State` del AST. No cambia la semántica de Evo-Script ni reabre las decisiones previamente cerradas de `FunctionBody`, `LetBinding`, `Program` o definiciones de tipos.

## ER-001 — Typed Nested Tree

Status: CLOSED

El AST v0 utiliza un árbol tipado anidado para representar `Expression`.

```text
Expression
    ↓
Expression Kind
    ↓
child Expressions when required
```

No se introduce un `AstNode` universal ni un `NodeId` genérico.

## ER-002 — Direct Recursive Relationship usa `Box<Expression>`

Status: CLOSED

Cuando una forma de Expression contiene directamente otra `Expression`, la indirection física se representa mediante:

```rust
Box<Expression<'source>>
```

Ejemplos conceptuales:

```text
Unary
└── operand: Box<Expression>

Binary
├── left: Box<Expression>
└── right: Box<Expression>

FieldAccess
└── receiver: Box<Expression>
```

`Box` existe únicamente para romper la recursión de tamaño físico requerida por Rust. No representa ownership semántico de Evo-Script ni introduce recursividad funcional o de tipos en el lenguaje.

## ER-003 — Ordered Collections usan `Vec<...>` sin `Box` adicional

Status: CLOSED

Cuando una construcción contiene una secuencia ordenada de Expressions, `Vec` ya aporta la indirection necesaria y no se agrega `Box` por elemento sin necesidad.

Ejemplos:

```rust
Vec<Expression<'source>>
Vec<FieldInitializer<'source>>
Vec<PipelineStage<'source>>
```

No:

```rust
Vec<Box<Expression<'source>>>
```

salvo que una necesidad técnica futura demuestre lo contrario.

## ER-004 — No ExpressionId / Expression Store / AST Arena en v0

Status: CLOSED

No se introducen en v0:

```text
ExpressionId
Expression Store
AST Arena
Node Store
Generic NodeId
```

Motivo:

- no existe necesidad demostrada de stable node identities;
- no existe incremental compilation en v0;
- no existe sharing de AST nodes;
- no existe cross-tree mutation que justifique indexed storage;
- TD-010 prioriza corrección, claridad y determinismo durante Compilation Working State.

Una futura necesidad técnica puede reabrir esta representación sin cambiar la semántica del lenguaje.

## ER-005 — Decisiones previas permanecen intactas

Status: CLOSED

Las representaciones ya cerradas continúan válidas:

```rust
struct FunctionBody<'source> {
    statements: Vec<BodyStatement<'source>>,
    result: Expression<'source>,
}

struct LetBinding<'source> {
    binding: TypedBinding<'source>,
    value: Expression<'source>,
}
```

No se sustituyen por `ExpressionId`.

## Canonical Rule

```text
AST v0
    = typed nested tree

direct recursive Expression relationship
    = Box<Expression>

ordered child collections
    = Vec<...>

no indexed expression storage
    unless a demonstrated future need reopens this decision
```

## Closure

```text
Typed nested Expression tree        ✅ CLOSED
Box for direct recursion             ✅ CLOSED
Vec for ordered child collections    ✅ CLOSED
No ExpressionId                      ✅ CLOSED
No AST Arena                         ✅ CLOSED
Previous AST decisions unchanged     ✅ CLOSED
```
