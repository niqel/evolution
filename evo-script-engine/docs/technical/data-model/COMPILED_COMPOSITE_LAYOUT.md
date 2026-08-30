# Evo-Script Engine — Compiled Composite Layout

Status: CLOSED

Este documento cierra el layout físico conceptual de composite Values requerido por `Compiled Program / Bytecode Data` antes de definir las instructions específicas de `struct`, `enum`, `when` y structural equality.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`;
- `SEMANTIC_PROGRAM_STRUCTURE.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_CONTROL_FLOW.md`.

## 1. Principle

Regla canónica:

> Semantic Program conserva identities y significado estructural; Bytecode Compiler reduce ese significado a posiciones físicas y discriminantes que la VM puede consumir sin volver a `TypeId`, field names, variant names ni runtime type lookup.

Composite Layout describe mecanismo físico ejecutable. No introduce reflection metadata ni una segunda IR de tipos.

## 2. No persistent runtime type-layout table in v0

No se introducen por defecto:

```text
StructLayout
EnumLayout
StructLayoutId
EnumLayoutId
CompositeTypeId
RuntimeTypeId
Runtime Type Table
Reflection Metadata
```

La ausencia de estas identities es intencional: Semantic Analyzer ya resolvió tipos, fields y variants, y Bytecode Compiler puede materializar directamente las posiciones/discriminantes que necesitan las Instructions.

Si una necesidad técnica futura demuestra que Runtime requiere metadata persistente adicional, esta decisión debe reabrirse explícitamente.

## 3. FieldIndex

Representación cerrada:

```rust
struct FieldIndex(usize);
```

`FieldIndex` identifica una posición lógica física dentro del composite owner actual.

No es:

```text
FieldId
byte offset
memory address
field name
TypeId
```

Separación:

```text
FieldId
    = semantic identity inside structural owner

FieldIndex
    = compiled physical position inside composite Value
```

## 4. Canonical field ordering

Bytecode Compiler utiliza como canonical physical ordering el mismo ordering ya definido por el semantic structural owner.

```text
semantic owner fields[0] → FieldIndex(0)
semantic owner fields[1] → FieldIndex(1)
semantic owner fields[2] → FieldIndex(2)
...
```

Por tanto el mapping `FieldId → FieldIndex` puede ser numéricamente idéntico en v0 sin convertir ambas identities en el mismo concepto.

El mapping pertenece al lowering del compiler y no requiere una tabla persistente adicional dentro de `CompiledProgram`.

## 5. Struct Value layout

Layout conceptual cerrado:

```text
Struct Value
└── ordered fields
    ├── FieldIndex(0) → Value
    ├── FieldIndex(1) → Value
    ├── FieldIndex(2) → Value
    └── ...
```

Un Struct Value no necesita conservar para ejecución ordinaria:

```text
TypeId
struct name
field names
SemanticField
```

La representación Rust concreta del runtime Value pertenece a `VM Execution Data`; este documento únicamente fija el contrato físico observable por bytecode.

## 6. FieldIndex reuse in structured enum payloads

`FieldIndex` se reutiliza para campos de una Structured Enum Variant porque la responsabilidad es exactamente la misma: posición dentro del composite owner actual.

Ejemplo conceptual:

```text
Variant Movimiento {
    int x;
    int y;
}

Structured Payload
├── FieldIndex(0) → x
└── FieldIndex(1) → y
```

No se introducen:

```text
StructFieldIndex
EnumFieldIndex
VariantFieldIndex
```

mientras no exista una responsabilidad distinta.

## 7. VariantDiscriminant

Representación cerrada:

```rust
struct VariantDiscriminant(usize);
```

`VariantDiscriminant` representa la identity física de la alternativa activa dentro de un Enum Value compilado.

Separación:

```text
VariantId
    = semantic identity inside SemanticType::Enum

VariantDiscriminant
    = compiled/runtime physical alternative identity
```

No es un user-defined ordinal ni una identity estable entre compilaciones.

## 8. Canonical discriminant ordering

Evo-Script v0.1 no permite discriminantes explícitos definidos por el programador.

Por tanto Bytecode Compiler fija canónicamente:

```text
semantic enum variants[0] → VariantDiscriminant(0)
semantic enum variants[1] → VariantDiscriminant(1)
semantic enum variants[2] → VariantDiscriminant(2)
...
```

No se requiere una tabla persistente `VariantId → VariantDiscriminant` después de compilation.

## 9. Enum Value layout

Layout conceptual cerrado:

```text
Enum Value
├── VariantDiscriminant
└── Payload
    ├── Simple
    ├── Associated(Value)
    └── Structured(ordered fields)
```

### Simple

No transporta payload Value.

### Associated

Transporta exactamente un Value asociado.

### Structured

Transporta fields ordenados canónicamente mediante `FieldIndex`.

La representación Rust concreta del payload se decide después en `VM Execution Data`.

## 10. Semantic type identity may disappear after valid lowering

Dos composite types semánticamente diferentes pueden compartir la misma forma física.

Ejemplo:

```text
struct A { int value; }
struct B { int other; }
```

Ambos pueden materializar físicamente un composite de un único `Int32` sin convertirse por ello en el mismo tipo semántico.

Semantic Analyzer ya impide mezclar `A` y `B`; Bytecode Compiler emite únicamente operations válidas para el programa ya resuelto.

Regla:

> La pérdida de `TypeId` en runtime no borra la corrección semántica; dicha corrección ya fue materializada en las Instructions válidas que el compiler produce.

## 11. Source evaluation order != canonical storage order

La identificación de fields por nombre permite que el source construction order difiera del canonical physical storage order.

Ejemplo conceptual:

```text
Trabajador {
    name: get_name()
    edad: get_age()
}
```

Puede corresponder físicamente a:

```text
name → FieldIndex(1)
edad → FieldIndex(0)
```

pero Bytecode Compiler no puede reordenar arbitrariamente la evaluación de `get_name()` y `get_age()` únicamente para producir storage canónico.

Regla:

```text
source evaluation order
    must remain observable evaluation order

canonical storage order
    defines final composite field positions
```

Las futuras construction instructions deben preservar simultáneamente ambas propiedades.

Esto importa porque una field expression puede contener:

```text
External Call
Internal Call
EvaluationError
```

y cambiar su orden sería semánticamente observable.

## 12. Structural equality consequence

El layout cerrado permite que structural equality pueda definirse sin `TypeId` runtime:

```text
Struct equality
    → compare corresponding ordered fields

Enum equality
    → compare discriminants
    → if equal, compare payload structurally
```

La estrategia exacta de instructions para structural equality se cierra después de `Struct / Enum Instructions`.

Este documento no introduce todavía:

```text
EqualStruct
EqualEnum
StructuralEqual
```

## 13. Explicit exclusions

```text
StructLayoutId
EnumLayoutId
CompositeTypeId
RuntimeTypeId
runtime field names
runtime variant names
runtime type lookup table
reflection metadata
byte offsets for fields
persistent FieldId
persistent VariantId
```

`FieldOffset` no se introduce porque el contrato físico v0 se expresa en posiciones lógicas de Values, no en offsets de bytes crudos.

## 14. Closure

```text
FieldIndex                              ✅ CLOSED
FieldId / FieldIndex separation         ✅ CLOSED
canonical field ordering                ✅ CLOSED
Struct Value conceptual layout          ✅ CLOSED
FieldIndex reuse for structured payload ✅ CLOSED
VariantDiscriminant                     ✅ CLOSED
VariantId / discriminant separation     ✅ CLOSED
canonical discriminant ordering         ✅ CLOSED
Enum Value conceptual layout            ✅ CLOSED
no persistent type-layout table v0      ✅ CLOSED
TypeId erased after valid lowering      ✅ CLOSED
source evaluation order preservation    ✅ CLOSED
canonical storage order distinction     ✅ CLOSED
structural equality layout basis        ✅ CLOSED

Struct / Enum Instructions              ← NEXT
Struct / Enum equality                  PENDING
SourceMap                               PENDING
Compiled Program exact inventory        PENDING
```