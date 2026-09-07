# Evo-Script Engine — D2 Sequence Diagrams

Status: D2 SEQUENCE DIAGRAMS — CLOSED

Este directorio contiene las secuencias dinámicas canónicas de `evo-script-engine` v0 derivadas exclusivamente de Rust Signatures, Participants, Module Signature Diagrams y la reconciliación cerrada con `evo-values v0.1`.

```text
HISTORICAL SEQUENCE SUITE
CLOSED / PRESERVED
4 views

CURRENT RECONCILED SEQUENCE SUITE
CLOSED
9 views
```

## Canonical suite

```text
00-compile.d2                         ✅ BUILT / PRESERVED
01-execute-compiled.d2                ✅ BUILT / PRESERVED
02-call-external.d2                   ✅ BUILT / PRESERVED
03-execute-source.d2                  ✅ BUILT / PRESERVED

04-direct-scalar-delegation.d2        ✅ BUILT
05-scalar-comparison-delegation.d2    ✅ BUILT
06-conversion-delegation.d2           ✅ BUILT
07-dynamic-numeric-delegation.d2      ✅ BUILT
08-structural-equality-delegation.d2  ✅ BUILT
```

Total: `9 / 9`

## 00 — Compile

Representa la secuencia exacta:

```text
Consumer
  ↓ COMPILE
compiler Agent
  ↓ LEX_SOURCE
lexer
  ↓ PARSE_TOKENS
parser
  ↓ ANALYZE_PROGRAM
semantic_analyzer
  ↓ LOWER_PROGRAM
bytecode_compiler
  ↓
CompiledProgram
```

La primera `Err(CompileFailure)` de Lexer, Parser o Semantic Analyzer termina el flujo. Lowering no posee failure normal.

## 01 — ExecuteCompiled

Representa:

```text
Consumer
  ↓ EXECUTE_COMPILED
compiled_program_executor
  ↓ INITIALIZE_EXECUTION
execution_initializer
  ↓
VmExecution
  ↓ execution loop
instruction_executor
  ↓ entry Return
OwnedValue
```

Incluye el uso explícito de:

```text
matches_value_shape
materialize_value
own_runtime_value
```

`CallExternal` se separa deliberadamente en la vista `02-call-external.d2` porque cruza una frontera técnica externa.

## 02 — CallExternal

Representa la única frontera técnica externa de ejecución:

```text
Agent
  ↓ RESOLVE_EXTERNAL_CALL
external_call_resolver
  ↓ observe RuntimeValues
observe_runtime_value
  ↓
Value<'call> arguments
  ↓
ExternalCapability
  ↓
OwnedValue / ExternalCapabilityFailure
```

En success:

```text
matches_owned_value_shape
    ↓
materialize_owned_value
    ↓
RuntimeValue
    ↓
N → 1 commit
    ↓
ip += 1
```

En failure se reutiliza `locate_source_span`; no hay stack commit y el IP permanece en `CallExternal`.

## 03 — ExecuteSource

Representa composición directa:

```text
Consumer
  ↓ EXECUTE_SOURCE
source_executor
  ├── Compile participants
  ├── contextualize_compile_failure cuando corresponda
  └── Execution participants
```

No existe:

```text
source_executor → Compile Agent
source_executor → ExecuteCompiled Agent
```

`ExecuteSource` reutiliza las mismas firmas internas y conserva un `CompiledProgram` local durante la `VmExecution`.

## 04 — Direct Scalar Delegation

Representa la delegación directa de operaciones escalares fijas desde `instruction_executor` hacia las funciones públicas de `evo-values`:

```text
instruction_executor
  ├── Fixed Integer Arithmetic: evo-values::numeric (ADD_I32, DIVIDE_I32, etc.)
  │     ↳ Zero-copy pop/peek de escalares directos de la pila
  │     ↳ Mapeo explícito: NumericFailure::Overflow → EvaluationFailure::Overflow
  │     ↳ Mapeo explícito: NumericFailure::DivisionByZero → EvaluationFailure::DivisionByZero
  ├── Fixed Float Arithmetic: IEEE 754 (DIVIDE_F64, etc.)
  │     ↳ División por ±0.0 produce ±inf / NaN según IEEE 754
  │     ↳ Sin conversión automática a Overflow ni DivisionByZero
  └── Boolean NOT: evo-values::boolean::NOT
        ↳ Pop de Boolean directo y push de resultado sin allocation
```

## 05 — Scalar Comparison Delegation

Representa la delegación de igualdad y ordenamiento escalar hacia `evo-values::comparison`:

```text
instruction_executor
  ├── Numeric Equality: EQUAL (EqualNumeric, NotEqualNumeric)
  │     ↳ Pop de escalares numéricos directos
  ├── Relational Ordering: LESS, LESS_EQUAL, GREATER, GREATER_EQUAL
  │     ↳ LessNumeric / GreaterNumeric delegan a sus funciones dedicadas de ordering
  │     ↳ Estrictamente no derivan de EQUAL
  ├── Boolean Equality: EQUAL
  │     ↳ Pop de Boolean y evaluación de igualdad canónica
  └── String Equality: observe_runtime_value → EQUAL
        ↳ Zero-copy borrow de ambos Strings en la pila vía OBSERVE_RUNTIME_VALUE
        ↳ Value::String(&str) como vistas eficientes sin clonar buffers
```

En todas las comparaciones escalares, dado que Semantic Analyzer ya garantizó la compatibilidad estática de tipos:
```text
ComparisonFailure tras análisis estático válido → INTERNAL INVARIANT VIOLATION
```

## 06 — Conversion Delegation

Representa las cinco familias de conversión tipadas explícitas hacia `evo-values::conversion`:

```text
instruction_executor
  ├── Fixed → Fixed: TO_INT32_FROM_I64
  ├── Dynamic → Fixed: TO_INT32_FROM_DYNAMIC (observe_runtime_value → borrow)
  ├── Fixed → Dynamic: TO_DYNAMIC_INTEGER_FROM_I32
  │     ↳ Crea DynamicIntegerBacking::Small (o Big) dentro de VmExecution
  │     ↳ Move semantics hacia RuntimeValue::Dynamic(DynamicValue::Integer(id))
  ├── Numeric → String: TO_STRING_FROM_I32
  │     ↳ Produce String con allocation gestionada por el engine
  └── Dynamic → String: TO_STRING_FROM_DYNAMIC
        ↳ Zero-copy borrow de DynamicValue y delegación de formateo canónico
```

Mapeo de fallos:
```text
ConversionFailure::NotExactlyRepresentable → EvaluationFailure::Conversion
```

## 07 — Dynamic Numeric Delegation

Representa la delegación de aritmética dinámica sobre `RuntimeValue::Dynamic`:

```text
instruction_executor
  ├── Observación zero-copy de operandos vía observe_runtime_value / as_borrowed()
  ├── Dynamic Negate: DYNAMIC_NEGATE
  ├── Dynamic Add / Subtract / Multiply: DYNAMIC_ADD, etc.
  │     ↳ Resultados enteros: nuevo DynamicIntegerBacking en VmExecution (sin BigInt)
  │     ↳ Resultados flotantes: RuntimeValue::Dynamic(Float32/Float64) directos
  └── Dynamic Remainder con Language Guard:
        ├── Family == Integer: delega a DYNAMIC_REMAINDER
        └── Family == Float32 | Float64: emite EvaluationFailure::DynamicNumericType
              ↳ NO CALL TO DYNAMIC_REMAINDER (guard del lenguaje)
```

Mapeo de fallos de `evo-values`:
```text
DynamicNumericFailure::DifferentFamily → EvaluationFailure::DynamicNumericType
DynamicNumericFailure::DivisionByZero  → EvaluationFailure::DivisionByZero
```

## 08 — Structural Equality Delegation

Representa la delegación de igualdad estructural compuesta (`EqualComposite`, `NotEqualComposite`):

```text
instruction_executor
  ├── Las instrucciones de igualdad compuesta son payloadless (sin equality plans)
  ├── Observación temporal de árboles Value vía observe_runtime_value
  │     ↳ Permite allocations eficientes temporales (Box<[Value]>, EnumPayload)
  │     ↳ No utiliza CompiledValueShape para comparar
  ├── EqualComposite: llama directamente a evo_values::comparison::EQUAL
  └── NotEqualComposite: llama directamente a evo_values::comparison::NOT_EQUAL
        ↳ Invocación directa sin negar !EQUAL
```

División semántica clara:
- **Semantic Analyzer**: Ya verificó estáticamente `EqualityComparable` y compatibilidad de tipos compuestos.
- **InstructionExecutor**: Adapta `RuntimeValue` a `Value` temporal.
- **evo-values Comparison**: Es la autoridad exclusiva que posee la semántica recursiva de Struct/Enum. El engine no recorre campos ni variantes.

## Sequence invariants

1. Toda lifeline conductual corresponde a un módulo/Participant cerrado, salvo `Consumer` y la frontera técnica explícita `ExternalCapability`.
2. Toda llamada entre lifelines corresponde a una firma cerrada o al ABI `ExternalCapability` ya cerrado.
3. Ningún Agent llama otro Agent.
4. Ningún Collaborator llama otro Collaborator.
5. Solo `external_call_resolver` invoca `ExternalCapability`.
6. Requesters no aparecen porque el inventario v0 es exactamente 0.
7. No se introduce un Contract duplicado para `ExternalCapability`.
8. Los helpers privados no aparecen como lifelines.
9. El resultado público exitoso de ejecución es siempre `OwnedValue`.
10. Los diagramas no reabren el Technical Data Model ni las Rust Signatures.
11. Las lifelines de `evo-values` representan Use Cases públicas de una dependencia semántica interna. No son Participants de `evo-script-engine`. No constituyen Provider/Contract boundary.
12. Las nuevas secuencias no introducen nuevos Participants en el engine.

## Audit

```text
Canonical sequence files present          9 / 9 ✅
Historical orchestration views            4 / 4 ✅
Cross-component reconciliation views      5 / 5 ✅
Public Use Cases covered                  3 / 3 ✅
Unknown engine participants introduced    0 ✅
Provider/Contract invented for values      0 ✅
```

### D2 Validation

- **Histórico**: La suite original de 4 vistas (`00` a `03`) fue diseñada sin ejecutable `d2` localmente disponible al momento de su cierre inicial.
- **Vigente (Reconciliación)**: Las cinco vistas nuevas (`04-direct-scalar-delegation.d2`, `05-scalar-comparison-delegation.d2`, `06-conversion-delegation.d2`, `07-dynamic-numeric-delegation.d2`, `08-structural-equality-delegation.d2`) fueron validadas y compiladas exitosamente utilizando el ejecutable `d2` v0.7.1 sin errores sintácticos ni de layout.

## Closure

```text
HISTORICAL / PRESERVED:
D2 Sequence Diagram suite       ✅ CLOSED — 4 views
NEXT ARCHITECTURAL STAGE
    Implementation Tasks

CURRENT RECONCILIATION SUITE:
D2 Sequence Diagram suite       ✅ CLOSED — 9 views

CURRENT RECONCILIATION NEXT:
TASK-ESE-RECON-006
Restablecer compatibilidad con la API pública de evo-values
```
