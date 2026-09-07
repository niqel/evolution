# Evo-Script Engine — Compiled Conversions

Status: CLOSED (reconciled with evo-values v0.1)
Authority: [`../EVO_VALUES_V0_1_RECONCILIATION.md`](../EVO_VALUES_V0_1_RECONCILIATION.md)

Este documento cierra la representación de bytecode para las conversiones explícitas `to_tipo` definidas por Evo-Script v0.1 reconciliadas con `evo-values v0.1`.

La autoridad deriva de:

- `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`, sección 9;
- `evo-script/DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_NUMERIC_INSTRUCTIONS.md`;
- `COMPILED_STORAGE_DATA.md`;
- `../EVO_VALUES_V0_1_RECONCILIATION.md`.

## 1. Principle & Responsibility Split

```text
conversion availability
target selection
lowering
opcode selection
    → evo-script-engine

exact conversion semantics
    → evo-values

failure translation
    → evo-script-engine
```

Semantic Program representa conversiones mediante:

```rust
SemanticExpressionKind::Conversion {
    operand: Box<SemanticExpression>,
}
```

con source/target `TypeId` ya resueltos.

Bytecode Compiler transforma esa información a mecanismos físicos y elimina `TypeId`.

Regla canónica:

> La VM ejecuta una conversión explícitamente descrita por bytecode; no realiza type inference, coercion implícita ni selección dinámica de un target semántico. La semántica matemática exacta de conversión delega en las operaciones universales de `evo-values`.

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
    → ConversionFailure::NotExactlyRepresentable → EvaluationFailure::Conversion
```

La semántica de exact representability y la detección de fallas pertenecen a las operaciones de conversión de `evo-values`. El engine delega la conversión a `evo-values` y traduce `ConversionFailure::NotExactlyRepresentable` a `EvaluationFailure::Conversion` (históricamente documentado como `ConversionError`). El engine no duplica algoritmos de rango ni de exactitud matemática.

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

La VM puede implementar la misma operación exacta delegando en `evo-values`; una conversión garantizada simplemente no alcanza la ruta `EvaluationFailure::Conversion` para ningún Value válido del source kind.

Bytecode Compiler puede eliminar una conversión físicamente identidad cuando demuestre que source y target requieren exactamente la misma representación y la operación no puede fallar.

Ejemplo físico:

```text
semantic `int` → `int32`
NumericKind::Int32 → NumericKind::Int32
```

Puede no requerir instruction runtime.

Esta eliminación es optimization/lowering válido, no cambio de semántica visible.

## 6. Integer conversions

Entre integer kinds, `ConvertNumeric` delega en `evo-values`, que conserva el valor matemático exacto cuando pertenece al rango destino.

Ejemplos:

```text
Int8 → Int16
    guaranteed

Int128 → Int64
    runtime range check via evo-values

Int32 → Uint32
    requires source >= 0 and within target range via evo-values

Uint128 → Int128
    requires source <= Int128::MAX mathematical value via evo-values
```

Failure:

```text
ConversionFailure::NotExactlyRepresentable → EvaluationFailure::Conversion
```

No existe bit reinterpretation.

## 7. Floating conversions

Las conversiones que involucran floating kinds delegan en `evo-values` y exigen exact representability (`ConversionFailure::NotExactlyRepresentable`).

```text
integer → float
    exact or EvaluationFailure::Conversion

float → integer
    exact integer value + in range
    otherwise EvaluationFailure::Conversion

Float64 → Float32
    exact representation required
    otherwise EvaluationFailure::Conversion
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

La instrucción delega directamente en los Use Cases por target de `evo-values` sobre la familia runtime activa del dynamic numeric:

```text
Dynamic Integer
Dynamic Float32
Dynamic Float64
```

Si el valor concreto de la familia activa puede representarse exactamente en el target especificado, produce el fixed Value correspondiente.

En cualquier otro caso:

```text
ConversionFailure::NotExactlyRepresentable → EvaluationFailure::Conversion
```

No existe `DifferentFamily` como semántica o error de conversión: la conversión entre familias numéricas dinámicas y tipos fijos es una operación de exact representability (o se puede convertir exactamente al target o falla con `EvaluationFailure::Conversion`).

Tampoco existe:

```text
EvaluationFailure::DynamicNumericType
```

para una conversión explícita, porque la semántica de la operación es precisamente intentar convertir el valor. `EvaluationFailure::DynamicNumericType` pertenece exclusivamente a dynamic numeric arithmetic cross-family sin conversión solicitada.

## 9. Fixed → dynamic is not a language conversion instruction

Evo-Script v0.1 no define `to_dynamic`.

Cuando un fixed numeric operand debe participar desde el origen en una arithmetic subtree cuyo resultado semántico es `dynamic`, Bytecode Compiler utiliza la instruction técnica ya cerrada:

```rust
Instruction::LiftDynamic(NumericKind)
```

Separación:

```text
LiftDynamic
    = internal bytecode lowering mechanism (usa construcción/conversión universal de valores de evo-values)

ConvertDynamic
    = explicit Evo-Script `dynamic → fixed` conversion (delega en Use Cases por target de evo-values)
```

No son operaciones inversas visibles del mismo API de lenguaje. `LiftDynamic` no representa una función o conversión explícita del lenguaje, sino un mecanismo interno del compilador de bytecode que utiliza la construcción universal de `evo-values` para inyectar un valor numérico fijo en una representación dinámica.

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

Responsabilidad:
- `evo-values` posee la semántica neutral de `ToString` y el formatting canónico para valores numéricos del modelo universal.
- `evo-script-engine` mantiene únicamente la disponibilidad de la operación en el lenguaje, el opcode de bytecode (`NumericToString`), la adaptación del resultado hacia `RuntimeValue` y la propiedad del buffer/string backing (gestión de memoria del string producido).

No existe parsing inverso desde string hacia numeric en v0.1.

El bytecode no conserva locale, culture, format string ni formatting provider; `to_string` es una operación determinista del lenguaje delegada en la semántica neutral de `evo-values` y no depende del Host.

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

Delega directamente en la semántica neutral de `ToString` provista por `evo-values` sobre el Dynamic Numeric Value activo. No se realiza primero una conversión intermedia a un fixed numeric kind.

Al igual que en `NumericToString`, `evo-script-engine` conserva únicamente el opcode, la adaptación a `RuntimeValue` y la propiedad de la asignación del string resultante.

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
EvaluationFailure::Conversion (históricamente ConversionError)
```

cuando `evo-values` retorna `ConversionFailure::NotExactlyRepresentable` (la representación exacta es imposible).

`EvaluationFailure::Conversion`:

```text
is not a Value
is not part of normal expression type
is not returned as Result
is not catchable inside Evo-Script v0.1
propagates to the outer execution boundary
```

La representación técnica exacta del error pertenece a Outcome / Diagnostic Data (`EvaluationFailure::Conversion`).

## 15. Closure

```text
fixed numeric → fixed numeric representation ✅ CLOSED
ConvertNumeric                              ✅ CLOSED (delegates exactness to evo-values)
guaranteed/fallible shared instruction      ✅ CLOSED
integer exact conversion                    ✅ CLOSED (delegates to evo-values)
floating exact conversion                   ✅ CLOSED (delegates to evo-values)
dynamic → fixed numeric                     ✅ CLOSED (delegates to evo-values target UCs)
ConvertDynamic                              ✅ CLOSED
fixed numeric → string                      ✅ CLOSED (evo-values ToString, engine owns buffer)
NumericToString                             ✅ CLOSED
dynamic → string                            ✅ CLOSED (evo-values ToString, engine owns buffer)
DynamicToString                             ✅ CLOSED
fixed → dynamic                             ✅ CLOSED via LiftDynamic
string → numeric parsing                    ❌ EXCLUDED v0
implicit conversion                         ❌ EXCLUDED
bool/struct/enum → string                    ❌ NOT INTRODUCED by current spec
EvaluationFailure::Conversion boundary      ✅ CLOSED
```
