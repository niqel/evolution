# Evo-Script Engine — Compiled Scalar Equality

Status: CLOSED

Este documento cierra las instructions escalares no numéricas requeridas por Evo-Script v0 para boolean negation/equality y string equality.

La autoridad deriva de:

- `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`, secciones de operators/comparisons;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_NUMERIC_INSTRUCTIONS.md`;
- `COMPILED_CONTROL_FLOW.md`.

## 1. Equality families

Evo-Script define equality semántica sobre varias familias:

```text
numeric     ✅ closed in COMPILED_NUMERIC_INSTRUCTIONS.md
bool        ← this document
string      ← this document
struct      PENDING Composite Layout

enum        PENDING Composite Layout

dynamic     ❌ prohibited by language
```

Este documento no adelanta structural equality.

## 2. Boolean negation

Unary logical `!` se representa mediante:

```rust
Instruction::NotBoolean
```

Stack effect:

```text
1 bool → 1 bool
```

Semántica:

```text
true  → false
false → true
```

Bytecode Compiler solo produce `NotBoolean` para operandos semanticamente `bool`.

No existe runtime type dispatch.

`NotBoolean` es distinto de short-circuit `&&` / `||`: negation evalúa exactamente un operand y no requiere branching.

## 3. Boolean equality

Representación:

```rust
Instruction::EqualBoolean
Instruction::NotEqualBoolean
```

Stack effect común:

```text
2 bool → 1 bool
```

Contrato:

```text
before
... left(bool) right(bool)

after
... result(bool)
```

La VM consume `right`, luego `left`, y produce el resultado lógico correspondiente.

No se introduce ordering boolean:

```text
LessBoolean
LessEqualBoolean
GreaterBoolean
GreaterEqualBoolean
```

porque Evo-Script no define orden relacional sobre `bool`.

## 4. String equality

Representación:

```rust
Instruction::EqualString
Instruction::NotEqualString
```

Stack effect:

```text
2 string → 1 bool
```

La igualdad de `string` compara contenido textual UTF-8 completo.

Invariantes:

1. no compara addresses;
2. no compara ownership identity;
3. borrowed y owned backing representations que materialicen el mismo contenido deben producir equality semántica;
4. no depende de locale, culture o collation ambiental;
5. la VM no realiza ordering lexicográfico mediante estos operators.

No existen:

```text
LessString
LessEqualString
GreaterString
GreaterEqualString
```

porque Evo-Script v0.1 no define ordering operators sobre `string`.

## 5. Why `NotEqual*` remains explicit

Aunque `!=` podría lowered conceptualmente a:

```text
Equal*
NotBoolean
```

v0 conserva instructions explícitas:

```text
NotEqualNumeric
NotEqualBoolean
NotEqualString
```

por simetría con el modelo semántico cerrado y para evitar introducir instrucciones adicionales en una comparación simple cuando el backend puede ejecutarla directamente.

Esto no impide que una optimización futura reescriba internamente una forma equivalente si preserva semántica.

## 6. Dynamic equality remains absent

No se introducen:

```text
EqualDynamic
NotEqualDynamic
```

porque Evo-Script prohíbe comparación directa sobre `dynamic`.

El programa debe convertir explícitamente a un tipo concreto antes de comparar.

## 7. Structural equality remains pending

Este cierre no define todavía:

```text
EqualStruct
EqualEnum
StructuralEqual
RuntimeTypeEquality
```

La especificación exige igualdad estructural para `struct` y `enum`, pero la estrategia física depende de Composite Layout:

```text
field representation
field position
runtime enum discriminant
payload representation
```

Después de cerrar Composite Layout se decidirá si Bytecode Compiler expande structural equality hacia instrucciones más básicas o si se justifica una instruction compuesta.

## 8. Closure

```text
NotBoolean                    ✅ CLOSED
EqualBoolean                  ✅ CLOSED
NotEqualBoolean               ✅ CLOSED
bool ordering                 ❌ EXCLUDED
EqualString                   ✅ CLOSED
NotEqualString                ✅ CLOSED
string equality by UTF-8 data ✅ CLOSED
string ordering               ❌ EXCLUDED
numeric equality              ✅ already CLOSED
dynamic equality              ❌ language-prohibited
struct equality               PENDING Composite Layout
enum equality                 PENDING Composite Layout
```
