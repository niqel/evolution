# Evo-Script — Dynamic Numeric Arithmetic v0.1

Status: CLOSED — NORMATIVE AMENDMENT

Este documento cierra la semántica que la especificación base v0.1 no definía de forma suficiente para operaciones aritméticas entre valores cuyo tipo semántico es `dynamic`.

Este amendment complementa `EVO_SCRIPT_SPECIFICATION_v0.1.md`. En caso de ambigüedad específica sobre arithmetic entre payloads runtime de `dynamic`, este documento tiene precedencia por representar la decisión posterior.

## 1. Principle

`dynamic` es exclusivamente un tipo numérico. No representa any-value, reflection, object dispatch ni coerción general.

Regla canónica:

> El contexto `dynamic` participa desde el origen de la evaluación numérica; no se evalúa primero bajo un tipo fijo para convertir el resultado después.

Para integer arithmetic, `dynamic` conserva el resultado matemático exacto mediante representación suficiente, incluyendo precisión arbitraria cuando sea necesaria.

Para floating-point, `dynamic` no introduce precisión arbitraria ni `float128`; conserva las familias `float32` y `float64`.

## 2. Runtime Dynamic Numeric Families

Un valor `dynamic` numérico pertenece en runtime a exactamente una de estas familias conceptuales:

```text
Dynamic Numeric Value
├── Integer
├── Float32
└── Float64
```

### Integer

La familia `Integer` representa un entero signed de precisión suficiente. El origen `int`, `int8`..`int128` o `uint8`..`uint128` deja de imponer width o signedness una vez elevado a `dynamic`; se conserva únicamente su valor matemático exacto.

### Float32

Conserva semántica de precisión `float32`.

### Float64

Conserva semántica de precisión `float64`. Los tipos semánticos `float` y `float64` utilizan esta familia física.

## 3. Compatible Dynamic Arithmetic

Para `+`, `-`, `*` y `/`, dos operands `dynamic` son compatibles únicamente cuando pertenecen a la misma familia runtime:

```text
Integer + Integer      ✅
Float32 + Float32      ✅
Float64 + Float64      ✅

Integer + Float32      ❌
Integer + Float64      ❌
Float32 + Float64      ❌
```

La misma matriz aplica a `-`, `*` y `/`.

Para `%`:

```text
Integer % Integer      ✅
Float32 % Float32      ❌
Float64 % Float64      ❌
```

`dynamic` no habilita remainder flotante.

## 4. No Implicit Cross-Family Conversion

Evo-Script no introduce coerciones implícitas entre familias `dynamic`.

Por tanto:

```text
Dynamic Integer + Dynamic Float64
```

no convierte automáticamente Integer a Float64 ni Float64 a Integer.

Tampoco existe promoción implícita:

```text
Dynamic Float32 → Dynamic Float64
```

Para cambiar representación, el programa debe usar una conversión explícita `to_tipo` hacia un tipo concreto cuando la especificación la permita.

## 5. DynamicNumericTypeError

Cuando una operación aritmética entre operands semánticamente `dynamic` alcanza runtime con familias incompatibles, la evaluación falla con:

```text
DynamicNumericTypeError
```

`DynamicNumericTypeError` es un `EvaluationError` del lenguaje.

Invariantes:

1. no es un Value;
2. no forma parte del tipo normal `dynamic`;
3. no produce un enum implícito ni `Result`;
4. no es capturable dentro de Evo-Script v0.1;
5. se propaga al límite exterior de evaluación igual que `OverflowError`, `DivisionByZeroError` y `ConversionError`;
6. no es `ComparisonTypeError`, porque la incompatibilidad concreta de payload families no necesariamente es conocible durante static semantic analysis;
7. no es `ConversionError`, porque la operación no solicitó una conversión.

## 6. Integer Dynamic Arithmetic

Para Dynamic Integer:

```text
Add / Subtract / Multiply
    → mathematical exact result
    → no OverflowError caused by representation width

Divide
    → truncation toward zero

Remainder
    → same quotient/remainder identity as Evo integer arithmetic

Divide / Remainder by zero
    → DivisionByZeroError
```

La representación puede expandirse según sea necesario para preservar el valor exacto.

## 7. Floating Dynamic Arithmetic

Para Dynamic Float32 y Dynamic Float64 se conservan las reglas normativas de la familia flotante correspondiente.

En particular:

```text
x / 0.0
x / -0.0
    → DivisionByZeroError
```

Evo-Script no produce Infinity o NaN silenciosamente por división entre cero.

## 8. Dynamic Negation

Unary `-` sobre `dynamic` opera sobre la familia runtime actual:

```text
Dynamic Integer  → exact signed negation
Dynamic Float32  → Float32 negation
Dynamic Float64  → Float64 negation
```

Dynamic Integer no produce `OverflowError` por width durante negation.

## 9. Dynamic Comparisons Remain Prohibited

Este amendment no introduce comparison directa para `dynamic`.

Continúan inválidas:

```text
dynamic_a == dynamic_b
dynamic_a != dynamic_b
dynamic_a < dynamic_b
dynamic_a <= dynamic_b
dynamic_a > dynamic_b
dynamic_a >= dynamic_b
```

El programador debe convertir explícitamente a un tipo concreto antes de comparar.

## 10. Dynamic Context and Function Boundaries

El contexto `dynamic` no atraviesa silenciosamente contratos de funciones o Signatures con tipos concretos.

Ejemplo conceptual:

```text
private fn calculate(int8 a, int8 b) -> int8 {
    return a + b;
}

let dynamic result = calculate(a, b);
```

`calculate` conserva arithmetic `int8` y puede producir `OverflowError`. Solo después de producir exitosamente su `int8` puede el resultado elevarse a `dynamic` para una expresión exterior.

La misma regla aplica a explicit conversions hacia tipos concretos.

## 11. Closure

```text
dynamic numeric families                 ✅ CLOSED
Integer arbitrary precision semantics     ✅ CLOSED
Float32 / Float64 family preservation      ✅ CLOSED
same-family arithmetic                     ✅ CLOSED
cross-family implicit conversion forbidden ✅ CLOSED
DynamicNumericTypeError                    ✅ CLOSED
integer dynamic divide/remainder semantics ✅ CLOSED
dynamic comparison remains prohibited      ✅ CLOSED
dynamic context function-boundary rule     ✅ CLOSED
```
