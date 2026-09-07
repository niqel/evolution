# Evo-Script Engine — Compiled Structural Equality

Status:
- HISTORICAL DESIGN: CLOSED / PRESERVED (SUPERSEDED)
- CURRENT RECONCILED MODEL: CLOSED
- Authority: [`../EVO_VALUES_V0_1_RECONCILIATION.md`](../EVO_VALUES_V0_1_RECONCILIATION.md)

Este documento define el mecanismo de bytecode para Structural Equality de `struct` y `enum` en `evo-script-engine` v0 reconciliado con las decisiones de `evo-values v0.1`.

## 1. Current Reconciled Responsibility Split

```text
Semantic Analyzer
    → decide EqualityComparable
    → valida compatibilidad estática
    → same TypeId donde corresponde

Bytecode Compiler
    → emite EqualComposite / NotEqualComposite
    → NO construye equality plan

InstructionExecutor
    → observa ambos RuntimeValue mediante
      OBSERVE_RUNTIME_VALUE

    → obtiene Value trees

    → EqualComposite:
      evo_values::comparison::EQUAL

    → NotEqualComposite:
      evo_values::comparison::NOT_EQUAL
```

### Regla fundamental de delegación:

`NotEqualComposite` debe delegar directamente en:

```text
evo_values::comparison::NOT_EQUAL
```

y **NO** implementarse como `!EQUAL(...)`.

## 2. Runtime Semantics via evo-values Comparison Kernel

La comparación recursiva de:

```text
Struct
Enum Simple
Enum Associated
Enum Structured
```

pertenece al kernel de Comparison de `evo-values` (`evo_values::comparison::EQUAL` y `evo_values::comparison::NOT_EQUAL` sobre `Value`).

El engine **NO** recorre fields ni variants para definir la semántica de equality: esa semántica universal está centralizada en `evo-values`.

---

## 3. Value Materialization for Equality

```text
RuntimeValue composite
    ↓
OBSERVE_RUNTIME_VALUE
    ↓
temporary Value tree
    ↓
EQUAL / NOT_EQUAL
```

- La creación temporal de containers `Value` para `Struct` y `Enum` puede requerir allocation (`Box` o containers temporales de `Value<'a>`). Esto está formalmente aceptado en esta versión para preservar una única autoridad semántica universal sin duplicar algoritmos de comparación.
- Strings permanecen borrowed cuando sea posible.
- Dynamic Integer debe poder permanecer borrowed mediante el backing reconciliado posteriormente (`OwnedDynamicInteger.as_borrowed()`).

---

## 4. Current Instructions

```rust
Instruction::EqualComposite
Instruction::NotEqualComposite
```

Ambas variantes son **payloadless**.

Stack effect común:

```text
2 composite Values → 1 bool
```

Los operandos se evalúan de izquierda a derecha antes de ejecutar la instrucción de comparación.

---

## 5. Comparison Failure Handling

Dado que `Semantic Analyzer` ya validó la compatibilidad estática (`EqualityComparable` y coherencia de tipos):

```text
ComparisonFailure::DifferentFamily
ComparisonFailure::NotComparable
```

son considerados:

```text
INTERNAL INVARIANT VIOLATION
```

No se introducen nuevas variantes de `EvaluationFailure`. Si ocurriese un failure de comparación tras un análisis semántico exitoso, se trata como una violación invariante interna del motor.

---

## 6. Explicit Exclusions

La reconciliación técnica prohíbe introducir:

```text
EqualValue instruction
RuntimeTypeEquality
EqualityPlanId
CompiledProgram.equality_plans
ValueSemanticBridge
reflection metadata
CompiledValueShape runtime equality dispatch
```

`CompiledValueShape` conserva su responsabilidad exclusiva de boundary validation y **NO** se reutiliza para dispatch de structural equality en runtime.

Dynamic comparison continúa prohibida por Evo-Script: un composite que contenga `dynamic` directa o transitivamente no es `EqualityComparable` y es rechazado estáticamente por Semantic Analyzer.

---

## 7. Historical Design (CLOSED / PRESERVED — SUPERSEDED)

> [!NOTE]
> **HISTORICAL DESIGN — SUPERSEDED BY evo-values v0.1 RECONCILIATION**: El diseño histórico requería que el compiler generase planes explícitos de igualdad recorridos recursivamente por la VM:
> - `EqualityRule`
> - `CompositeEqualityPlan`
> - `EnumEqualityPayloadPlan`
> - generación de planes en `BytecodeCompiler`
> - recorrido de planes en `InstructionExecutor`
>
> Ese mecanismo ya **NO** es la autoridad vigente. Se preserva a continuación únicamente con fines de trazabilidad histórica.

```rust
enum EqualityRule {
    Numeric(NumericKind),
    Boolean,
    String,
    Composite(CompositeEqualityPlan),
}

enum CompositeEqualityPlan {
    Struct {
        fields: Vec<EqualityRule>,
    },
    Enum {
        variants: Vec<EnumEqualityPayloadPlan>,
    },
}

enum EnumEqualityPayloadPlan {
    Simple,
    Associated(EqualityRule),
    Structured {
        fields: Vec<EqualityRule>,
    },
}

// Históricas instrucciones con payload:
Instruction::EqualComposite(CompositeEqualityPlan)
Instruction::NotEqualComposite(CompositeEqualityPlan)
```

Dichas 3 identidades (`EqualityRule`, `CompositeEqualityPlan`, `EnumEqualityPayloadPlan`) han sido eliminadas del modelo técnico de Compiled Program reconciliado.

---

## 8. Closure

```text
EqualComposite payloadless                   ✅ CLOSED (reconciled)
NotEqualComposite payloadless                ✅ CLOSED (reconciled)
Direct delegation to EQUAL / NOT_EQUAL       ✅ CLOSED (reconciled)
Kernel Comparison in evo-values              ✅ CLOSED (reconciled)
Temporary Value tree via OBSERVE             ✅ CLOSED (reconciled)
No EqualityRule / CompositeEqualityPlan      ✅ CLOSED (reconciled — removed from active model)
No EqualValue generic instruction            ❌ EXCLUDED
No reflection metadata in engine             ❌ EXCLUDED
Dynamic equality prohibited                  ❌ EXCLUDED by language spec
```
