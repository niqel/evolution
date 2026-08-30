# Evo-Script Engine — Compiled Conversions

Status: CLOSED

Este documento cierra la representación de bytecode para las conversiones explícitas `to_tipo` definidas por Evo-Script v0.1.

La autoridad deriva de:

- `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`, sección 9;
- `evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_NUMERIC_INSTRUCTIONS.md`;
- `COMPILED_STORAGE_DATA.md`.

## 1. Principle

Semantic Program representa conversiones mediante:

```rust
SemanticExpressionKind::Conversion {
    operand: Box<SemanticExpression>,
}
```

con source/target `TypeId` ya resueltos.

Bytecode Compiler transforma esa información a mecanismos físicos y elimina `TypeId`.

Regla canónica:

> La VM ejecuta una conversión explícitamente descrita por bytecode; no realiza type inference, coercion implícita ni selección dinámica de un target semántico.

## 2. Conversion instruction family

No se introduce un `ConversionKind` general ni un `BuiltinFunctionId`.

Las necesidades físicas de v0 se expresan directamente mediante variants de `Instruction`:

```rust
Instruction::ConvertNumeric {
    source: NumericKind,
    target: NumericKind,
}

Instruction::ConvertDynamic(NumericKind)

Instruction::NumericToString(NumericKind)

Instruction::DynamicToString
```

Estas variants cubren los dominios de conversión explícitamente establecidos por la especificación v0.1 sin reintroducir `NativeType` o `TypeId` dentro del Compiled Program.

## 3. Semantic native type lowering

Para conversion execution:

```text
int     → NumericKind::Int32
int32   → NumericKind::Int32

float   → NumericKind::Float64
float64 → NumericKind::Float64
```

La pérdida de la distinción semántica `int`/`int32` y `float`/`float64` en esta operación es válida porque semantic validation ya ocurrió y ambos pares comparten el mismo mecanismo físico de referencia.

No implica que los nombres sean aliases semánticos durante parsing o semantic analysis.

## 4. ConvertNumeric

```rust
Instruction::ConvertNumeric {
    source: NumericKind,
    target: NumericKind,
}
```

Stack effect:

```text
1 fixed numeric → 1 fixed numeric
```

La instruction implementa la regla universal de Evo-Script:

```text
source value exactly representable in target
    → produce target value

otherwise
    → ConversionError
```

Esto cubre:

```text
signed → signed
unsigned → unsigned
signed ↔ unsigned
integer → floating
floating → integer
floating → floating
```

No existe reinterpretación de bits, wrapping, saturation, truncation silenciosa ni rounding silencioso.

## 5. Guaranteed vs potentially fallible conversions

La especificación distingue conversiones garantizadas y potencialmente fallables, pero esa diferencia no requiere dos instruction identities.

```text
guaranteed conversion
potentially fallible conversion
        ↓
ConvertNumeric { source, target }
```

La VM puede implementar la misma operación exacta; una conversión garantizada simplemente no alcanza la ruta `ConversionError` para ningún Value válido del source kind.

Bytecode Compiler puede eliminar una conversión físicamente identidad cuando demuestre que source y target requieren exactamente la misma representación y la operación no puede fallar.

Ejemplo físico:

```text
semantic `int` → `int32`
NumericKind::Int32 → NumericKind::Int32
```

Puede no requerir instruction runtime.

Esta eliminación es optimization/lowering válido, no cambio de semántica visible.

## 6. Integer conversions

Entre integer kinds, `ConvertNumeric` conserva el valor matemático exacto cuando pertenece al rango destino.

Ejemplos:

```text
Int8 → Int16
    guaranteed

Int128 → Int64
    runtime range check

Int32 → Uint32
    requires source >= 0 and within target range

Uint128 → Int128
    requires source <= Int128::MAX mathematical value
```

Failure:

```text
ConversionError
```

No existe bit reinterpretation.

## 7. Floating conversions

Las conversiones que involucran floating kinds exigen exact representability.

```text
integer → float
    exact or ConversionError

float → integer
    exact integer value + in range
    otherwise ConversionError

Float64 → Float32
    exact representation required
    otherwise ConversionError
```

La VM no redondea ni trunca silenciosamente para satisfacer una conversión Evo-Script.

## 8. ConvertDynamic

```rust
Instruction::ConvertDynamic(NumericKind)
```

Stack effect:

```text
1 dynamic numeric → 1 fixed numeric
```

El target físico está completamente fijado por `NumericKind`.

La VM inspecciona únicamente la familia interna del Dynamic Numeric Value necesaria para realizar la conversión exacta:

```text
Dynamic Integer
Dynamic Float32
Dynamic Float64
```

Si el valor concreto puede representarse exactamente en el target, produce el fixed Value correspondiente.

En cualquier otro caso:

```text
ConversionError
```

No existe:

```text
DynamicNumericTypeError
```

para una conversión explícita, porque la semántica de la operación es precisamente intentar convertir el valor. `DynamicNumericTypeError` pertenece a arithmetic cross-family sin conversión solicitada.

## 9. Fixed → dynamic is not a language conversion instruction

Evo-Script v0.1 no define `to_dynamic`.

Cuando un fixed numeric operand debe participar desde el origen en una arithmetic subtree cuyo resultado semántico es `dynamic`, Bytecode Compiler utiliza la instruction técnica ya cerrada:

```rust
Instruction::LiftDynamic(NumericKind)
```

Separación:

```text
LiftDynamic
    = internal bytecode lowering mechanism

ConvertDynamic
    = explicit Evo-Script `dynamic → fixed` conversion
```

No son operaciones inversas visibles del mismo API de lenguaje.

## 10. NumericToString

Para un fixed numeric source explícitamente admitido por el sistema de conversiones:

```rust
Instruction::NumericToString(NumericKind)
```

Stack effect:

```text
1 fixed numeric → 1 string
```

Produce la representación textual definida por la semántica `to_string` de Evo-Script.

No existe parsing inverso desde string hacia numeric en v0.1.

El bytecode no conserva locale, culture, format string ni formatting provider; `to_string` es una operación determinista del lenguaje y no depende del Host.

## 11. DynamicToString

```rust
Instruction::DynamicToString
```

Stack effect:

```text
1 dynamic numeric → 1 string
```

La representación textual se produce a partir de la familia runtime actual:

```text
Dynamic Integer
Dynamic Float32
Dynamic Float64
```

No se realiza primero una conversión a un fixed numeric kind.

## 12. Conservative v0 `to_string` boundary

La especificación v0.1 establece explícitamente `to_string` dentro del capítulo de conversiones numéricas y define de forma concreta su uso sobre numeric values y `dynamic`.

El Technical Data Model no amplía silenciosamente esa regla hacia dominios no definidos explícitamente.

Por tanto v0 no introduce aquí instructions para:

```text
bool → string
string → string conversion
struct → string
enum → string
Signature/Function → string
```

Si la especificación futura declara alguna de esas conversiones, se reabre únicamente la familia necesaria.

## 13. No string → numeric parsing

No existen:

```text
StringToNumeric
ParseInt
ParseFloat
ConvertString(NumericKind)
```

porque Evo-Script v0.1 excluye parsing inverso desde texto hacia números.

## 14. Evaluation errors

Las conversion instructions pueden terminar la evaluación con:

```text
ConversionError
```

cuando la representación exacta es imposible.

`ConversionError`:

```text
is not a Value
is not part of normal expression type
is not returned as Result
is not catchable inside Evo-Script v0.1
propagates to the outer execution boundary
```

La representación técnica exacta del error pertenece a Outcome / Diagnostic Data.

## 15. Closure

```text
fixed numeric → fixed numeric representation ✅ CLOSED
ConvertNumeric                              ✅ CLOSED
guaranteed/fallible shared instruction      ✅ CLOSED
integer exact conversion                    ✅ CLOSED
floating exact conversion                   ✅ CLOSED
dynamic → fixed numeric                     ✅ CLOSED
ConvertDynamic                              ✅ CLOSED
fixed numeric → string                      ✅ CLOSED
NumericToString                             ✅ CLOSED
dynamic → string                            ✅ CLOSED
DynamicToString                             ✅ CLOSED
fixed → dynamic                             ✅ CLOSED via LiftDynamic
string → numeric parsing                    ❌ EXCLUDED v0
implicit conversion                         ❌ EXCLUDED
bool/struct/enum → string                    ❌ NOT INTRODUCED by current spec
ConversionError boundary                    ✅ CLOSED
```
