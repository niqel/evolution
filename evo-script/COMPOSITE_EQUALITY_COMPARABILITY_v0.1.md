# Evo-Script `.efn` — Composite Equality Comparability v0.1

Status: NORMATIVE — CLOSED

Este documento complementa `EVO_SCRIPT_SPECIFICATION_v0.1.md` y cierra la regla transitiva de comparabilidad por igualdad para `struct` y `enum` cuando sus campos o payloads contienen tipos que no admiten `==` / `!=`.

La especificación base ya establece que:

- `==` / `!=` requieren tipos semánticos exactos;
- `struct` utiliza Structural Equality campo por campo;
- `enum` compara discriminante/variante y payload estructuralmente;
- `dynamic` no admite comparación directa;
- `ComparisonTypeError` es un `SystemError` estático.

Este amendment define cómo se combinan esas reglas dentro de composites.

## 1. EqualityComparable

Se define la propiedad semántica estática:

```text
EqualityComparable(Type)
```

Un tipo puede participar en `==` / `!=` únicamente cuando esta propiedad es verdadera y ambos operandos poseen exactamente el mismo tipo semántico.

## 2. Native types

```text
fixed signed numeric    → EqualityComparable = true
fixed unsigned numeric  → EqualityComparable = true
fixed floating numeric  → EqualityComparable = true
bool                    → EqualityComparable = true
string                  → EqualityComparable = true
dynamic                 → EqualityComparable = false
```

`int`/`float` y sus variantes fixed conservan sus reglas existentes de identidad exacta de tipos; esta propiedad no introduce promociones ni aliases semánticos.

## 3. Struct

Para:

```text
struct S {
    field_0: T0
    field_1: T1
    ...
}
```

la regla es:

```text
EqualityComparable(S)
    iff
EqualityComparable(T0)
AND EqualityComparable(T1)
AND ...
```

Por tanto, un `struct` admite `==` / `!=` si y solo si todos sus fields son equality-comparable.

La regla es transitiva: si un field contiene otro composite no comparable, el owner tampoco es comparable.

Ejemplo:

```text
struct Measurement {
    dynamic value;
}
```

produce:

```text
EqualityComparable(dynamic)     = false
EqualityComparable(Measurement) = false
```

Entonces:

```text
measurement_a == measurement_b
```

es inválido estáticamente con `ComparisonTypeError`.

## 4. Enum

Un enum es equality-comparable si y solo si todos los payloads posibles de todas sus variants son equality-comparable.

Reglas por shape:

```text
Simple
    → comparable

Associated(T)
    → EqualityComparable(T)

Structured(fields...)
    → all fields EqualityComparable
```

Para el Enum completo:

```text
EqualityComparable(Enum)
    iff
all variants are equality-comparable
```

Ejemplo:

```text
enum Result {
    Empty
    Value(dynamic)
}
```

produce:

```text
EqualityComparable(Result) = false
```

Aunque `Result::Empty` no transporte payload, `Result` como tipo completo no admite `==` / `!=` porque otra variante puede transportar `dynamic`.

La validez del operador no depende de la variante runtime actual.

## 5. Static failure boundary

Cuando ambos operandos son del mismo composite type pero dicho type no satisface `EqualityComparable`, Semantic Analyzer rechaza la expresión con:

```text
ComparisonTypeError
```

No se produce Semantic Program exitoso para esa comparación.

No se difiere la decisión a runtime y no se introduce:

```text
DynamicEqualityError
CompositeEqualityRuntimeTypeError
variant-dependent equality validity
```

Esto aplica `Earliest Responsible Failure`.

## 6. Structural Equality when comparable

Si un Struct es EqualityComparable:

```text
left == right
    → corresponding fields compared recursively
```

Si un Enum es EqualityComparable:

```text
different variants
    → false

same Simple variant
    → true

same Associated / Structured variant
    → payload compared recursively
```

`!=` es la negación semántica de la igualdad estructural correspondiente.

## 7. No hidden dynamic equality

Regla canónica:

> `dynamic` no adquiere igualdad implícita por estar contenido dentro de un `struct` o `enum`.

Por tanto no existe una ruta donde Structural Equality llegue a un `dynamic` y decida compararlo en runtime.

Si se desea comparar información dinámica, el programa debe convertir explícitamente a un tipo concreto antes de construir/usar una forma comparable adecuada.

## 8. DAG consequence

Evo-Script v0.1 ya exige un Type Dependency Graph acíclico para composites. Por tanto `EqualityComparable` puede calcularse de forma finita durante Semantic Analysis mediante recorrido del DAG.

No requiere runtime recursion metadata ni cycle detection durante equality.

## 9. Closure

```text
EqualityComparable property                   ✅ CLOSED
fixed numeric equality comparability          ✅ CLOSED
bool equality comparability                   ✅ CLOSED
string equality comparability                 ✅ CLOSED
dynamic equality comparability                ❌ FALSE by language
Struct recursive comparability                ✅ CLOSED
Enum recursive comparability                  ✅ CLOSED
transitive dynamic prohibition                ✅ CLOSED
ComparisonTypeError boundary                  ✅ CLOSED
runtime variant-dependent validity            ❌ EXCLUDED
hidden dynamic equality inside composites     ❌ EXCLUDED
```
