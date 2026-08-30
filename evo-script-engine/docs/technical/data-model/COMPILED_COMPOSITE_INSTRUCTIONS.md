# Evo-Script Engine — Compiled Composite Instructions

Status: CLOSED

Este documento cierra las instructions de bytecode para construcción, acceso, inspección y extracción de composite Values (`struct` / `enum`) en `evo-script-engine` v0.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_CONTROL_FLOW.md`;
- `COMPILED_COMPOSITE_LAYOUT.md`;
- `evo-script/EFN_TYPE_CARDINALITY_v0.1.md`.

## 1. Principle

Regla canónica:

> Bytecode opera sobre composite Values ya semanticamente validados mediante `FieldIndex` y `VariantDiscriminant`, sin reintroducir `TypeId`, field names, variant names, pattern objects ni runtime type-layout lookup.

Las instructions deben preservar simultáneamente:

```text
source evaluation order
+
canonical composite storage order
```

La extracción de payloads de `enum` consume el Enum una vez confirmada la variante; no mantiene simultáneamente el composite owner y Values interiores extraídos cuando no existe una necesidad semántica para hacerlo.

## 2. Struct construction

Representación cerrada:

```rust
Instruction::ConstructStruct {
    field_order: Vec<FieldIndex>,
}
```

Bytecode Compiler evalúa las field expressions en source order. `field_order[i]` indica el `FieldIndex` físico al que pertenece el i-ésimo Value producido.

Ejemplo:

```text
Trabajador {
    name: get_name()
    edad: get_age()
}

canonical:
edad → FieldIndex(0)
name → FieldIndex(1)
```

Lowering conceptual:

```text
evaluate get_name()   // Value 0
evaluate get_age()    // Value 1

ConstructStruct {
    field_order: [
        FieldIndex(1),
        FieldIndex(0),
    ]
}
```

Resultado conceptual:

```text
Struct Value
├── FieldIndex(0) = edad
└── FieldIndex(1) = name
```

Stack effect para N fields:

```text
N → 1
```

### Construction invariants

Para un composite de N fields:

```text
field_order.len() == N
```

`field_order` contiene exactamente una vez cada `FieldIndex` válido del composite.

Conceptualmente:

```text
field_order = permutation of [0 .. N)
```

No son válidos como producto compilado:

```text
duplicated FieldIndex
missing FieldIndex
out-of-range FieldIndex
```

Semantic Analyzer ya impide dichas construcciones; esta es la invariante física equivalente de bytecode.

Un struct vacío utiliza:

```text
ConstructStruct { field_order: [] }
```

con stack effect:

```text
0 → 1
```

## 3. Struct field access

Representación cerrada:

```rust
Instruction::GetField(FieldIndex)
```

Stack effect:

```text
1 composite → 1 field Value
```

Conceptualmente:

```text
before:
... StructValue

GetField(FieldIndex(n))

after:
... StructValue.fields[n]
```

La instruction transforma el composite temporal en el field requerido. No conserva por obligación una copia adicional del Struct Value.

Ejemplo anidado:

```text
worker.address.city
```

puede reducirse a:

```text
Load...
GetField(address_index)
GetField(city_index)
```

No requiere:

```text
TypeId
FieldId
field name
StructLayout
Duplicate
```

## 4. Simple enum construction

Representación cerrada:

```rust
Instruction::ConstructEnumSimple(
    VariantDiscriminant,
)
```

Stack effect:

```text
0 → 1
```

Produce conceptualmente:

```text
Enum Value
├── discriminant
└── payload: Simple
```

## 5. Associated enum construction

Representación cerrada:

```rust
Instruction::ConstructEnumAssociated(
    VariantDiscriminant,
)
```

La associated expression se evalúa antes de la instruction.

Stack effect:

```text
1 associated Value → 1 Enum Value
```

Resultado conceptual:

```text
Enum Value
├── discriminant
└── payload: Associated(Value)
```

## 6. Structured enum construction

Representación cerrada:

```rust
Instruction::ConstructEnumStructured {
    variant: VariantDiscriminant,
    field_order: Vec<FieldIndex>,
}
```

Aplica la misma separación entre source evaluation order y canonical storage order que `ConstructStruct`.

Stack effect para N fields:

```text
N → 1
```

`field_order` debe ser una permutación completa y válida de los fields de la Structured Variant.

Una Structured Variant con cero fields es válida y utiliza:

```text
ConstructEnumStructured {
    variant,
    field_order: [],
}
```

con stack effect:

```text
0 → 1
```

No se colapsa obligatoriamente a `Simple`; ambas formas continúan siendo semanticamente distintas aunque una implementación runtime futura pueda compartir representación física cuando preserve la semántica.

## 7. Variant testing

Representación cerrada:

```rust
Instruction::TestVariant(
    VariantDiscriminant,
)
```

`TestVariant` inspecciona únicamente el discriminante y preserva el Enum Value para permitir probar otra correspondencia cuando la actual no coincide.

Stack effect:

```text
1 Enum → 1 Enum + 1 bool
```

Conceptualmente:

```text
before:
... Enum

TestVariant(D)

after:
... Enum bool
```

`JumpIfFalse` puede consumir el `bool` y dejar el mismo Enum disponible para la siguiente branch.

No se materializa el discriminante como Value normal del lenguaje.

No se introducen:

```text
GetDiscriminant Value
CompareDiscriminant
Match opcode
Pattern runtime object
```

## 8. Associated payload extraction — corrected final design

La propuesta provisional:

```text
GetEnumAssociated
1 → 2
```

queda RECHAZADA.

Conservar simultáneamente el Enum y exponer un Value contenido podría forzar cloning, interior borrowing o aliasing antes de que `VM Execution Data` defina una representación que lo justifique.

Representación cerrada corregida:

```rust
Instruction::ExtractEnumAssociated
```

Precondición de bytecode válido:

```text
la branch ya confirmó la VariantDiscriminant correcta
```

Stack effect:

```text
1 Enum → 1 associated Value
```

La instruction consume el Enum y transfiere/materializa el payload como Value de evaluación.

Lowering conceptual:

```text
TestVariant(D)
JumpIfFalse(next)

ExtractEnumAssociated
StoreLocal(binding_slot)
```

Después de `ExtractEnumAssociated` no queda un Enum residual que requiera `Discard`.

## 9. Structured payload extraction — corrected final design

La propuesta provisional de extraer un field individual preservando el Enum:

```text
GetEnumField(FieldIndex)
1 → 2
```

queda RECHAZADA por la misma razón de ownership/aliasing.

Semantic Analyzer ya garantiza que Structured extraction es válida y completa para la correspondencia resuelta.

Representación cerrada:

```rust
Instruction::ExtractEnumStructured {
    fields: Vec<FieldIndex>,
}
```

Stack effect para N extracted fields:

```text
1 Enum → N Values
```

La instruction consume el Enum y produce los Values requeridos en el orden descrito por `fields`.

Ejemplo:

```text
Evento::Movimiento {
    y: int local_y;
    x: int local_x;
}
```

con canonical layout:

```text
x → FieldIndex(0)
y → FieldIndex(1)
```

puede compilar:

```text
ExtractEnumStructured {
    fields: [
        FieldIndex(1),
        FieldIndex(0),
    ]
}
```

produciendo conceptualmente:

```text
... y x
```

para permitir después, bajo disciplina LIFO:

```text
StoreLocal(local_x)
StoreLocal(local_y)
```

Una Structured Variant con cero extracted fields utiliza:

```text
ExtractEnumStructured { fields: [] }
```

con stack effect:

```text
1 → 0
```

## 10. Simple `when` branch

Una Simple Variant no posee payload que extraer.

Lowering conceptual:

```text
TestVariant(D)
JumpIfFalse(next_branch)
Discard
<evaluate branch result>
Jump(end)
```

`Discard` consume el Enum subject después de confirmar que la simple variant corresponde.

## 11. Associated `when` branch

Lowering conceptual:

```text
<evaluate subject>              // [enum]

TestVariant(D)                  // [enum bool]
JumpIfFalse(next_branch)        // [enum]

ExtractEnumAssociated           // [payload]
StoreLocal(binding)             // []

<evaluate branch result>        // [result]
Jump(end)
```

El subject se evalúa una sola vez.

## 12. Structured `when` branch

Lowering conceptual:

```text
<evaluate subject>                  // [enum]

TestVariant(D)                      // [enum bool]
JumpIfFalse(next_branch)            // [enum]

ExtractEnumStructured { fields }    // [values...]
StoreLocal(...)
...

<evaluate branch result>            // [result]
Jump(end)
```

No existe un runtime pattern matcher.

## 13. Exhaustive `when`

Semantic Analyzer ya garantiza:

```text
subject is enum
valid variants
no duplicate branches
exhaustiveness
payload-shape correctness
binding type correctness
common result type
```

Bytecode Compiler puede utilizar la última exhaustive branch como fallback sin `TestVariant` si demuestra que todas las anteriores ya cubrieron las demás variants. Esto es una optimización permitida, no una invariante obligatoria del Compiled Program.

El mecanismo general permanece:

```text
TestVariant + JumpIfFalse
```

## 14. No forced inner-value aliasing

Regla cerrada:

> Las extraction instructions de enum consumen el composite una vez que la variant correcta fue confirmada, evitando requerir por diseño una representación donde el Enum owner y sus payload Values interiores deban coexistir como aliases independientes dentro del Operand Window.

Esta regla no prescribe todavía cómo `Value` owns/borrows internamente sus composite fields. Esa representación pertenece a `VM Execution Data`.

## 15. Final instruction family

```rust
enum Instruction {
    // ...

    ConstructStruct {
        field_order: Vec<FieldIndex>,
    },

    GetField(FieldIndex),

    ConstructEnumSimple(
        VariantDiscriminant,
    ),

    ConstructEnumAssociated(
        VariantDiscriminant,
    ),

    ConstructEnumStructured {
        variant: VariantDiscriminant,
        field_order: Vec<FieldIndex>,
    },

    TestVariant(
        VariantDiscriminant,
    ),

    ExtractEnumAssociated,

    ExtractEnumStructured {
        fields: Vec<FieldIndex>,
    },

    // ...
}
```

Stack contracts:

```text
ConstructStruct(N)            N → 1
GetField                      1 → 1

ConstructEnumSimple           0 → 1
ConstructEnumAssociated       1 → 1
ConstructEnumStructured(N)    N → 1

TestVariant                   1 → 2
ExtractEnumAssociated         1 → 1
ExtractEnumStructured(N)      1 → N
```

## 16. Explicitly rejected / excluded

```text
GetEnumAssociated preserving Enum
GetEnumField preserving Enum
Match instruction
When instruction
Pattern runtime object
Runtime TypeId
StructLayout / EnumLayout lookup
field names at runtime
variant names at runtime
forced payload cloning
forced interior aliasing
compiler-generated temporary local solely to preserve when subject
```

## 17. Closure

```text
ConstructStruct                              ✅ CLOSED
construction source evaluation order         ✅ CLOSED
construction canonical storage placement     ✅ CLOSED
construction field-order permutation         ✅ CLOSED
GetField                                     ✅ CLOSED
ConstructEnumSimple                          ✅ CLOSED
ConstructEnumAssociated                      ✅ CLOSED
ConstructEnumStructured                      ✅ CLOSED
TestVariant                                  ✅ CLOSED
GetEnumAssociated preserving owner           ❌ REJECTED
GetEnumField preserving owner                ❌ REJECTED
ExtractEnumAssociated                        ✅ CLOSED
ExtractEnumStructured                        ✅ CLOSED
Simple when lowering                         ✅ CLOSED
Associated when lowering                     ✅ CLOSED
Structured when lowering                     ✅ CLOSED
subject evaluated once                       ✅ CLOSED
no runtime pattern object                     ✅ CLOSED
no forced inner-value aliasing                ✅ CLOSED

Struct / Enum Instructions                    ✅ CLOSED
Struct / Enum Structural Equality             ← NEXT
SourceMap                                     PENDING
Compiled Program exact inventory              PENDING
