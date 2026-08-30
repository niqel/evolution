# Evo-Script Engine — Compiled Scalar Equality

Status: CLOSED — REVALIDATED

Este documento cierra las instructions escalares no numéricas requeridas por Evo-Script v0 para boolean negation/equality y string equality.

La autoridad deriva de `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`, `SEMANTIC_EXPRESSIONS.md`, `COMPILED_NUMERIC_INSTRUCTIONS.md` y `COMPILED_CONTROL_FLOW.md`.

## 1. Equality families

Estado vigente:

```text
numeric     ✅ CLOSED in COMPILED_NUMERIC_INSTRUCTIONS.md
bool        ✅ CLOSED here
string      ✅ CLOSED here
struct      ✅ CLOSED in COMPILED_STRUCTURAL_EQUALITY.md
enum        ✅ CLOSED in COMPILED_STRUCTURAL_EQUALITY.md
dynamic     ❌ prohibited by language
```

Este documento posee únicamente la familia scalar bool/string; no duplica el mecanismo structural posterior.

## 2. Boolean negation

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

No existe runtime type dispatch.

`NotBoolean` es distinto de `&&` / `||`, que requieren branching de short-circuit.

## 3. Boolean equality

```rust
Instruction::EqualBoolean
Instruction::NotEqualBoolean
```

Stack effect:

```text
2 bool → 1 bool
```

No existen ordering instructions booleanas:

```text
LessBoolean
LessEqualBoolean
GreaterBoolean
GreaterEqualBoolean
```

## 4. String equality

```rust
Instruction::EqualString
Instruction::NotEqualString
```

Stack effect:

```text
2 string → 1 bool
```

La igualdad compara contenido textual UTF-8 completo.

Invariantes:

1. no compara addresses;
2. no compara ownership identity;
3. borrowed y owned backing con el mismo contenido son iguales;
4. no depende de locale, culture o collation ambiental;
5. no existe ordering lexicográfico mediante operadores.

No existen:

```text
LessString
LessEqualString
GreaterString
GreaterEqualString
```

## 5. Why explicit `NotEqual*` remains

Aunque `!=` podría reducirse conceptualmente a:

```text
Equal*
NotBoolean
```

v0 conserva variants explícitas:

```text
NotEqualNumeric
NotEqualBoolean
NotEqualString
NotEqualComposite
```

Esto mantiene simetría con el modelo semántico y evita una Instruction adicional en el camino directo.

## 6. Dynamic equality remains absent

No existen:

```text
EqualDynamic
NotEqualDynamic
```

Evo-Script requiere conversión explícita a un tipo concreto antes de comparar `dynamic`.

Además, `COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md` cierra que un composite que contiene `dynamic` directa o transitivamente tampoco adquiere igualdad escondida.

## 7. Structural equality boundary

Struct/Enum equality ya está cerrada en `COMPILED_STRUCTURAL_EQUALITY.md` mediante:

```text
EqualityRule
CompositeEqualityPlan
EnumEqualityPayloadPlan
EqualComposite
NotEqualComposite
```

Este documento no redefine ese mecanismo.

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
numeric equality              ✅ CLOSED elsewhere
dynamic equality              ❌ language-prohibited
struct equality               ✅ CLOSED elsewhere
enum equality                 ✅ CLOSED elsewhere
```
