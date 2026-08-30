# Evo-Script `.efn` — Type Cardinality v0.1

Status: NORMATIVE — CLOSED

Este documento complementa `EVO_SCRIPT_SPECIFICATION_v0.1.md` y tiene precedencia cuando la especificación base no fija explícitamente la cardinalidad mínima de fields o variants en definiciones de tipos `.efn`.

## Struct Definition

Un `struct` puede declarar cero o más fields:

```text
StructDefinition
├── name
└── FieldDefinition 0..N
```

Por tanto, una definición vacía es sintácticamente válida:

```text
struct Completed {
}
```

Un `struct` vacío representa una composición de datos sin fields. Esta regla no introduce `void`, `Unit`, `null` ni comportamiento especial.

## Enum Definition

Un `enum` debe declarar una o más variants:

```text
EnumDefinition
├── name
└── EnumVariant 1..N
```

Una definición sin variants es sintácticamente inválida:

```text
enum Impossible {
}
```

Evo-Script v0.1 no introduce mediante `enum {}` un tipo `Never`, `Bottom` o uninhabited type implícito.

Conforme a Earliest Responsible Failure, Parser rechaza un `enum` vacío sin producir AST exitoso.

## Structured Enum Variant

Una Structured Enum Variant puede declarar cero o más fields:

```text
Structured Variant
├── name
└── FieldDefinition 0..N
```

Los fields reutilizan exactamente la misma forma sintáctica y reglas de `FieldDefinition` que los fields de `struct`.

## Semantic Validation Boundary

Parser conserva todas las ocurrencias sintácticamente válidas de fields y variants. Unicidad de nombres, resolución de tipos, `RecursiveTypeCycleError` y demás validaciones que requieren significado pertenecen a Semantic Analyzer.

## Closure

```text
Struct fields                 0..N  ✅ CLOSED
Enum variants                 1..N  ✅ CLOSED
Structured variant fields     0..N  ✅ CLOSED
Empty enum rejected by Parser       ✅ CLOSED
```
