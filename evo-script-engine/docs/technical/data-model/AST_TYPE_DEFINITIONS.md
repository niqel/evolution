# Evo-Script Engine — AST Type Definitions

Status: CLOSED

Este documento complementa `AST_DATA.md` y registra las identidades AST cerradas para definiciones locales de tipos `.efn`.

La cardinalidad sintáctica se rige por `evo-script/EFN_TYPE_CARDINALITY_v0.1.md`.

## StructDefinition

Representación Rust cerrada:

```rust
struct StructDefinition<'source> {
    name: Identifier<'source>,
    fields: Vec<FieldDefinition<'source>>,
}
```

Invariantes:

- `fields` preserva orden y duplicados sintácticos;
- cardinalidad `0..N`;
- Parser no resuelve tipos ni deduplica fields;
- no requiere `SourceSpan` adicional.

## FieldDefinition

Representación Rust cerrada:

```rust
struct FieldDefinition<'source> {
    type_name: Identifier<'source>,
    name: Identifier<'source>,
}
```

`FieldDefinition` representa un field estructural de datos y permanece separado de `TypedBinding`, aunque ambos utilicen la forma textual `tipo nombre`.

La misma identidad `FieldDefinition` se reutiliza tanto en `StructDefinition` como en una Structured `EnumVariant`, porque ambas construcciones comparten exactamente la misma sintaxis y reglas de fields.

## EnumDefinition

Representación Rust cerrada:

```rust
struct EnumDefinition<'source> {
    name: Identifier<'source>,
    variants: Vec<EnumVariant<'source>>,
}
```

Invariantes:

- `variants` preserva orden y duplicados sintácticos;
- cardinalidad `1..N`;
- un `enum` vacío produce Syntax Failure en Parser conforme a Earliest Responsible Failure;
- no requiere `SourceSpan` adicional.

## EnumVariant

Representación Rust cerrada:

```rust
enum EnumVariant<'source> {
    Simple {
        name: Identifier<'source>,
    },
    Associated {
        name: Identifier<'source>,
        type_name: Identifier<'source>,
    },
    Structured {
        name: Identifier<'source>,
        fields: Vec<FieldDefinition<'source>>,
    },
}
```

Las tres variants representan exactamente las formas sintácticas v0:

```text
Simple       Variante
Associated   Variante(Tipo)
Structured   Variante { tipo campo; ... }
```

Una Structured Variant admite `0..N FieldDefinition`.

No existe `EnumVariantKind` separado: la variant Rust expresa directamente la alternativa sintáctica.

## Semantic Boundary

Los `type_name` permanecen como `Identifier` sintáctico. Parser no decide si representan Native Type, local Struct, local Enum o imported shared Type.

Semantic Analyzer conserva la responsabilidad de:

```text
Type resolution
DuplicateFieldError
DuplicateVariantError
RecursiveTypeCycleError
other semantic type validation
```

## Closed Relationship

```text
StructDefinition
├── name: Identifier
└── FieldDefinition 0..N
      ├── type_name: Identifier
      └── name: Identifier

EnumDefinition
├── name: Identifier
└── EnumVariant 1..N
      ├── Simple
      │    └── name
      ├── Associated
      │    ├── name
      │    └── type_name
      └── Structured
           ├── name
           └── FieldDefinition 0..N
```

## Closure

```text
StructDefinition       ✅ CLOSED
FieldDefinition        ✅ CLOSED
EnumDefinition         ✅ CLOSED
EnumVariant            ✅ CLOSED
Struct fields 0..N     ✅ CLOSED
Enum variants 1..N     ✅ CLOSED
Structured fields 0..N ✅ CLOSED
```
