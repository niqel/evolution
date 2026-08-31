# Evo-Script Engine — Compiled Boundary Value Shape

Status: IN ANALYSIS

Este documento abre el bloque correctivo detectado durante el cierre de `VmExecution`.

No modifica todavía `CompiledProgram`, `CompiledFunction` ni `ExternalSymbol`.

Su propósito es determinar la metadata mínima persistente que debe sobrevivir al lowering para validar correctamente los Values que cruzan dos fronteras ejecutables:

```text
Consumer → Execute Compiled entry Parameters
ExternalCapability → CallExternal result
```

## Why this block exists

El Functional Use Case `Execute Compiled` exige que los Invocation Values tengan:

```text
exact arity
exact semantic compatibility
no implicit coercion
```

antes de comenzar una ejecución válida.

La aridad del entry puede resolverse actualmente mediante:

```text
CompiledFunction.parameter_count
```

pero `CompiledFunction` no conserva parameter Value shapes/types.

Por tanto, con el modelo compilado vigente, dos Invocation Values con la aridad correcta pero shapes incompatibles no pueden validarse únicamente desde el persistent executable artifact.

Segundo problema:

```text
CallExternal
    ↓
ExternalCapability
    ↓ Success
OwnedValue
```

`ExternalSymbol` conserva actualmente:

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
}
```

pero no conserva el expected result Value shape.

Como todas las External Capabilities convergen al ABI uniforme, Rust ya no puede demostrar por la firma específica del Provider que el `OwnedValue` retornado coincide con el result contractual esperado por el bytecode.

## Architectural boundary

Este problema pertenece al `CompiledProgram`, no a `VmExecution`.

Regla:

> El artifact compilado debe conservar suficiente contrato ejecutable para validar Values que entran desde una frontera no tipada por el bytecode específico, sin reintroducir Semantic Program ni runtime reflection general.

No se propone almacenar en `VmExecution`:

```text
TypeId
SemanticType
entry parameter type copies
external result type copies
runtime reflection tables
```

## Questions to close

### CB-QUESTION-001 — What is the minimum executable Value-shape identity?

Debemos definir una representación compilada mínima capaz de expresar las familias de Value que pueden cruzar una frontera:

```text
Boolean
Int8 .. Int128
Uint8 .. Uint128
Float32 / Float64
String
Dynamic
Struct
Enum
```

Para composites deberá determinarse si basta una shape estructural recursiva o si se requiere otra identity compilada reutilizable.

No se asumirá `TypeId` por defecto.

### CB-QUESTION-002 — Where does entry parameter shape live?

Debemos decidir la forma mínima que permita validar:

```text
InvocationValue[n]
    ↔ entry Parameter[n]
```

antes de iniciar una `VmExecution` válida.

La solución puede afectar `CompiledFunction`, el root `CompiledProgram` o una tabla compilada dedicada, pero no se decide todavía.

### CB-QUESTION-003 — What does ExternalSymbol preserve?

Debemos determinar qué información adicional mínima necesita:

```text
ExternalSymbol
```

para validar el `OwnedValue` retornado por su `ExternalCapability`.

La aridad ya permanece como:

```text
parameter_count
```

La question nueva es el expected result Value shape.

### CB-QUESTION-004 — Are external argument shapes required at runtime?

Los argumentos de `CallExternal` provienen de bytecode ya semanticamente validado y de `RuntimeValue` internos producidos por ese mismo programa.

Debemos revisar si conservar sus parameter shapes en `ExternalSymbol` agrega una responsabilidad runtime real o sería metadata redundante.

No se conservarán por simetría únicamente.

### CB-QUESTION-005 — Validation failure ownership

Debemos separar:

```text
invocation boundary mismatch
external capability contract mismatch
internal compiler/VM invariant violation
```

El tipo exacto de Failure pertenece a Outcome / Diagnostic Data, pero este bloque debe identificar qué validaciones son normales de frontera y cuáles son invariantes internas.

## Constraints already closed

La solución debe preservar:

```text
no AST in CompiledProgram
no SemanticProgram in VM
no runtime type inference
no general reflection
no ambient Provider lookup
no TypeId in RuntimeValue
no TypeId in composite backing
ExternalCapability remains uniform fn ABI
Value<'a> / OwnedValue remain neutral interchange Values
```

## Current hypothesis — NOT CLOSED

La dirección mínima probablemente requerirá una representación compilada de `ValueShape` distinta de `SemanticType` y de `RuntimeValue`.

Conceptualmente:

```text
Semantic Type
    ↓ lowering
Compiled Value Shape
    ↓ boundary validation only
Value<'a> / OwnedValue
```

Esto es únicamente una hipótesis de trabajo.

No quedan cerrados todavía:

```text
ValueShape name
exact enum variants
composite shape representation
storage owner
entry parameter storage shape
ExternalSymbol result field
sharing/interning strategy
identity vs inline structural shape
```

## Next

El siguiente paso arquitectónico es comparar alternativas para una **Compiled Value Shape** mínima y decidir si la forma debe ser:

```text
A. inline recursive structural shape
B. owner-indexed compiled shape table
C. another smaller executable contract representation
```

Ninguna alternativa está CLOSED todavía.
