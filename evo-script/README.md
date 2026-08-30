# Evo-Script — Normative Documents

La autoridad vigente de Evo-Script v0.1 se compone de:

1. [`EVO_SCRIPT_SPECIFICATION_v0.1.md`](./EVO_SCRIPT_SPECIFICATION_v0.1.md) — especificación base.
2. [`EFN_HOST_BOUNDARY_v0.1.md`](./EFN_HOST_BOUNDARY_v0.1.md) — amendment normativo posterior para la frontera `.efn` / Host.
3. [`EFN_TYPE_CARDINALITY_v0.1.md`](./EFN_TYPE_CARDINALITY_v0.1.md) — amendment normativo para cardinalidades estructurales de `struct` y `enum` en `.efn`.
4. [`DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md`](./DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md) — amendment normativo para arithmetic entre payload families runtime de `dynamic` y `DynamicNumericTypeError`.

Cuando exista contradicción sobre `.efn`, `Scope`, `Active Scope`, `use`, navegación ambiental o neutralidad respecto del Consumer, `EFN_HOST_BOUNDARY_v0.1.md` tiene precedencia por ser la decisión arquitectónica posterior.

Cuando la especificación base no fije explícitamente cardinalidades mínimas para fields o variants de tipos `.efn`, `EFN_TYPE_CARDINALITY_v0.1.md` tiene precedencia normativa.

Cuando exista ambigüedad específica sobre arithmetic entre valores `dynamic`, compatibilidad entre sus payload families runtime o el fallo correspondiente por incompatibilidad, `DYNAMIC_NUMERIC_ARITHMETIC_v0.1.md` tiene precedencia normativa por ser la decisión posterior.

Regla de lenguaje/documentación:

> Nombres de artifacts, fases y conceptos técnicos canónicos en English; explicaciones, decisiones, reglas e invariantes en español.
