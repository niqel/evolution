# Evo-Script Engine — Exact AST Inventory

Status: CLOSED

Este documento consolida el inventario exacto de identidades AST de `evo-script-engine` v0 una vez cerrados los modelos de Program, types, functions, Expressions y `when`.

Este inventario sustituye el carácter preliminar de `AD-007 — Preliminary AST Inventory` de `AST_DATA.md`. Las representaciones detalladas permanecen distribuidas en los documentos especializados del Technical Data Model.

## 1. Root and Top-level Syntax

```text
Program
ImportDeclaration
Declaration
```

Representaciones cerradas en `AST_DATA.md`.

## 2. Foundational Syntax Data

```text
Identifier
QualifiedName
Visibility
TypedBinding
```

`SourceSpan` es reutilizado desde Lexical Data y no se redefine como identidad AST paralela.

## 3. Local Type Definitions

```text
StructDefinition
FieldDefinition
EnumDefinition
EnumVariant
```

Representaciones cerradas en `AST_TYPE_DEFINITIONS.md`.

## 4. Functions and Body

```text
FunctionDefinition
Parameter
FunctionBody
BodyStatement
LetBinding
OperationStatement
```

Representaciones cerradas en `AST_FUNCTION_DEFINITIONS.md` y completadas por `AST_EXPRESSIONS.md` para los payloads de OperationStatement.

## 5. Expressions

```text
Expression
ExpressionKind
LiteralKind
UnaryOperator
BinaryOperator
FunctionCall
FieldInitializer
EnumConstruction
Pipeline
PipelineStage
```

Representaciones cerradas en `AST_EXPRESSIONS.md`.

Las siguientes construcciones existen como variants/fields dentro de `ExpressionKind` y no requieren identidad struct independiente:

```text
Literal
Identifier Expression
Unary Expression
Binary Expression
FieldAccess
StructConstruction
```

Esta decisión evita wrappers sin responsabilidad adicional.

## 6. When

```text
WhenExpression
WhenCorrespondence
WhenPattern
PatternField
```

Representaciones cerradas en `AST_WHEN.md`.

## 7. Exact Identity Count

El inventario AST v0 contiene exactamente **31 identidades técnicas propias**, sin contar `SourceSpan` porque pertenece a Lexical Data y es reutilizado por AST.

```text
Foundational                    4
Top-level                       3
Local type definitions          4
Functions / body                6
Expressions                    10
When                            4
                               ──
Total                           31
```

Lista completa:

```text
01 Identifier
02 QualifiedName
03 Visibility
04 TypedBinding
05 Program
06 ImportDeclaration
07 Declaration
08 StructDefinition
09 FieldDefinition
10 EnumDefinition
11 EnumVariant
12 FunctionDefinition
13 Parameter
14 FunctionBody
15 BodyStatement
16 LetBinding
17 OperationStatement
18 Expression
19 ExpressionKind
20 LiteralKind
21 UnaryOperator
22 BinaryOperator
23 FunctionCall
24 FieldInitializer
25 EnumConstruction
26 Pipeline
27 PipelineStage
28 WhenExpression
29 WhenCorrespondence
30 WhenPattern
31 PatternField
```

## 8. Parser-consumed Syntax

Las siguientes formas son sintácticamente relevantes pero no sobreviven como identidad AST independiente porque su efecto ya quedó expresado estructuralmente:

```text
return keyword
this pipeline placeholder
parentheses / grouping
commas
semicolons
colons
braces / delimiters
arrow / correspondence markers
import / as punctuation structure
fn / struct / enum declaration keywords
```

En particular:

```text
return expression;
    → FunctionBody.result

this
    → transported Pipeline Data position

(parenthesized expression)
    → Expression hierarchy + Expression.span
```

## 9. Explicitly Excluded Identities

No forman parte del AST v0:

```text
AstNode
AstNodeKind
Generic NodeId
ExpressionId
AST Arena
TypeReference
NativeType AST enum
ImportKind
EnumVariantKind
ParameterKind
ReturnStatement
ExpressionStatement
ParenthesizedExpression
GroupingExpression
ThisExpression
PipelineStageKind
SignatureSatisfaction wrapper
WhenScope / PatternScope
General Pattern framework
FunctionId
TypeId
LocalSlot
ParameterSlot
Resolved Signature
Resolved External Symbol
Provider binding
Bytecode
Opcode
Active Scope
Host Session State
```

## 10. Physical Representation Closure

La representación física v0 es:

```text
typed nested tree

Direct Expression recursion
    → Box<Expression>

Ordered variable-size collections
    → Vec<...>

No ExpressionId
No generic NodeId
No AST Arena
```

La decisión detallada se encuentra en `AST_EXPRESSION_REPRESENTATION.md`.

## 11. AST Data Closure

Con este inventario quedan cerrados para v0:

```text
AST syntactic responsibility      ✅ CLOSED
Parser / Semantic boundary        ✅ CLOSED
Occurrence preservation           ✅ CLOSED
Top-level Program model           ✅ CLOSED
Local type definitions            ✅ CLOSED
Function/body model               ✅ CLOSED
Expression representation         ✅ CLOSED
Expression inventory              ✅ CLOSED
When model                        ✅ CLOSED
Exact AST identity inventory      ✅ CLOSED
Physical recursive representation ✅ CLOSED

AST Data                          ✅ CLOSED
```

`derives`, Rust module visibility y Parser/Semantic Analyzer behavioral signatures no agregan nuevas identidades al AST Data Model y pertenecen a las fases técnicas posteriores de signatures/module design/implementation.
