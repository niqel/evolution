# Evo-Script Engine — Compiled Structural Equality

Status: CLOSED

Este documento cierra el mecanismo de bytecode para Structural Equality de `struct` y `enum` en `evo-script-engine` v0.

La autoridad deriva de:

- `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`;
- `evo-script/COMPOSITE_EQUALITY_COMPARABILITY_v0.1.md`;
- `SEMANTIC_PROGRAM_STRUCTURE.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_SCALAR_EQUALITY.md`;
- `COMPILED_COMPOSITE_LAYOUT.md`;
- `COMPILED_COMPOSITE_INSTRUCTIONS.md`.

## 1. Principle

Regla canónica:

> Semantic Analyzer decide si un composite type es `EqualityComparable`; Bytecode Compiler transforma esa decisión y el grafo de tipos ya resuelto en un plan ejecutable de igualdad; la VM ejecuta dicho plan sin `TypeId`, reflection, type inference ni igualdad dinámica oculta.

Structural Equality solo se compila para composite types que ya satisfacen la regla normativa `EqualityComparable`.

## 2. No bytecode expansion through destructive field access

No se expande por defecto:

```text
struct == struct
    → GetField(0) / compare
    → GetField(1) / compare
    → ...
```

porque `GetField` transforma/consume el composite temporal y una expansión de múltiples fields requeriría introducir machinery adicional como:

```text
Duplicate
stack shuffling
compiler-generated locals solely for equality
owner/payload aliasing
```

Structural Equality merece un mecanismo compilado propio en lugar de deformar las instructions de acceso ya cerradas.

## 3. No generic runtime `EqualValue`

No se introduce:

```text
EqualValue
NotEqualValue
RuntimeTypeEquality
```

La VM no selecciona igualdad preguntando dinámicamente qué clase de `Value` recibió.

El conocimiento de tipos ya fue resuelto durante Semantic Analysis y lowered por Bytecode Compiler.

## 4. EqualityRule

Representación cerrada:

```rust
enum EqualityRule {
    Numeric(NumericKind),
    Boolean,
    String,
    Composite(CompositeEqualityPlan),
}
```

No existe:

```text
EqualityRule::Dynamic
```

porque un composite que contenga `dynamic` directa o transitivamente no puede llegar válidamente a Structural Equality.

## 5. CompositeEqualityPlan

Representación cerrada:

```rust
enum CompositeEqualityPlan {
    Struct {
        fields: Vec<EqualityRule>,
    },

    Enum {
        variants: Vec<EnumEqualityPayloadPlan>,
    },
}
```

El orden de `fields` es el canonical `FieldIndex` ordering ya cerrado.

La posición `variants[n]` corresponde al `VariantDiscriminant(n)` canónico.

No se almacenan nuevamente `FieldIndex` ni `VariantDiscriminant` cuando la posición del Vec ya expresa la misma relación física de forma no ambigua.

## 6. EnumEqualityPayloadPlan

Representación cerrada:

```rust
enum EnumEqualityPayloadPlan {
    Simple,

    Associated(
        EqualityRule,
    ),

    Structured {
        fields: Vec<EqualityRule>,
    },
}
```

La forma corresponde exactamente al payload físico de la variant compilada.

## 7. Struct equality plan generation

Para un Struct semanticamente comparable:

```text
SemanticType::Struct(fields)
        ↓ Bytecode Compiler
CompositeEqualityPlan::Struct
        └── one EqualityRule per canonical field
```

Ejemplo conceptual:

```text
struct Address {
    string city;
    int zip;
}

struct Person {
    int id;
    string name;
    Address address;
}
```

produce conceptualmente:

```text
Struct
├── Numeric(Int32)
├── String
└── Composite(
      Struct
      ├── String
      └── Numeric(Int32)
   )
```

No contiene names, `TypeId` ni runtime layout lookup.

## 8. Enum equality plan generation

Para un Enum semanticamente comparable:

```text
SemanticType::Enum(variants)
        ↓ Bytecode Compiler
CompositeEqualityPlan::Enum
        └── one EnumEqualityPayloadPlan per canonical variant
```

Ejemplo conceptual:

```text
enum Result {
    Empty
    Found(Person)
    Error {
        int code;
        string message;
    }
}
```

produce:

```text
Enum
├── Simple
├── Associated(Person structural plan)
└── Structured
    ├── Numeric(Int32)
    └── String
```

## 9. Instructions

Representación cerrada:

```rust
Instruction::EqualComposite(
    CompositeEqualityPlan,
)

Instruction::NotEqualComposite(
    CompositeEqualityPlan,
)
```

Stack effect común:

```text
2 composite Values → 1 bool
```

Los operands se evalúan antes de ejecutar la comparison instruction, conservando la regla general izquierda-a-derecha de Evo-Script.

## 10. Struct runtime semantics

Para `EqualComposite(StructPlan)`:

```text
compare field 0 using rule 0
compare field 1 using rule 1
...
```

Primer field desigual:

```text
→ false
```

Todos iguales:

```text
→ true
```

Un struct vacío contiene cero fields y por igualdad estructural:

```text
Empty {} == Empty {}
    → true
```

La terminación temprana en primer field desigual es válida porque ambos operandos completos ya fueron evaluados antes de comenzar la comparison.

## 11. Enum runtime semantics

Para `EqualComposite(EnumPlan)`:

```text
left.discriminant != right.discriminant
    → false

same discriminant
    ↓
select variants[discriminant]
```

Según payload plan:

```text
Simple
    → true

Associated(rule)
    → compare associated payload using rule

Structured(fields)
    → compare corresponding payload fields recursively
```

La VM no interpreta el discriminante como número visible de Evo-Script; únicamente lo usa como mecanismo físico interno ya cerrado.

## 12. `NotEqualComposite`

`NotEqualComposite(plan)` produce la negación lógica exacta de la Structural Equality correspondiente.

Se conserva como instruction explícita, siguiendo la misma política ya cerrada para:

```text
NotEqualNumeric
NotEqualBoolean
NotEqualString
```

No obliga a emitir `EqualComposite + NotBoolean`.

## 13. Totality and failures

Una Structural Equality compilada es total sobre dos Values válidos del composite type para el que fue generada:

```text
Composite × Composite → bool
```

No produce:

```text
ComparisonTypeError runtime
DynamicNumericTypeError
ConversionError
```

Si el type no era EqualityComparable, Semantic Analyzer debía fallar antes con `ComparisonTypeError` y Bytecode Compiler nunca recibe una comparison válida para bajar.

## 14. No hidden dynamic equality

El plan no puede contener `Dynamic`.

La regla normativa transitiva garantiza:

```text
composite contains dynamic directly or transitively
    → composite not EqualityComparable
    → comparison rejected statically
```

Por tanto la VM nunca necesita inventar semántica de igualdad para Dynamic Numeric Value dentro de Structural Equality.

## 15. Plan ownership

`CompositeEqualityPlan` se almacena directamente como operand data de `EqualComposite` / `NotEqualComposite` en v0.

No se introduce:

```text
EqualityPlanId
CompiledProgram.equality_plans
EqualityPlanTable
```

Motivo:

1. no existe necesidad demostrada de una tabla persistente nueva;
2. evita reabrir la forma arquitectónica de `CompiledProgram`;
3. mantiene el plan cerca de la instruction que lo consume;
4. una futura optimización puede internar/deduplicar plans si profiling demuestra beneficio suficiente.

Duplicación de plans equivalentes entre distintas instructions es válida en v0.

## 16. No runtime type metadata

Structural Equality no requiere:

```text
TypeId
RuntimeTypeId
StructLayoutId
EnumLayoutId
field names
variant names
reflection metadata
```

El plan expresa únicamente mecanismos ejecutables de igualdad.

## 17. Closure

```text
EqualityRule                               ✅ CLOSED
CompositeEqualityPlan                     ✅ CLOSED
EnumEqualityPayloadPlan                   ✅ CLOSED
EqualComposite                            ✅ CLOSED
NotEqualComposite                         ✅ CLOSED
struct structural equality                ✅ CLOSED
enum structural equality                  ✅ CLOSED
empty struct equality                     ✅ CLOSED
same/different enum variant semantics      ✅ CLOSED
recursive equality over composite DAG      ✅ CLOSED
no EqualityRule::Dynamic                   ✅ CLOSED
static EqualityComparable boundary         ✅ CLOSED
no generic EqualValue runtime dispatch      ✅ CLOSED
no TypeId/runtime reflection requirement    ✅ CLOSED
plan stored directly in instruction         ✅ CLOSED
EqualityPlanId/table                        ❌ NOT NEEDED v0

Struct / Enum Structural Equality           ✅ CLOSED
SourceMap                                   ← NEXT
Compiled Program exact inventory            PENDING
```
