# Evo-Script Engine — External Call Resolution Design

Status: EXTERNAL CALL RESOLUTION DESIGN — CLOSED

Este documento cierra los Participants internos y Tools requeridos por `resolve_external_call`, cuya firma raíz está cerrada en `EXECUTION_PARTICIPANT_DESIGN.md`.

Firma raíz:

```rust
pub type ResolveExternalCall =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<(), ExecutionFailure>;
```

Precondición arquitectónica:

```text
current instruction
    = CallExternal(ExternalSymbolId)
```

## RSD-033 — `observe_runtime_value` es Tool de observación de interoperabilidad

Status: CLOSED

`RuntimeValue` es un descriptor privado relativo a `VmExecution` y no puede cruzar directamente la frontera de `ExternalCapability`.

Observarlo como `evo_values::Value<'value>` es una responsabilidad pequeña, reusable e independiente del Resolver concreto.

Firma cerrada:

```rust
pub type ObserveRuntimeValue =
    for<'value> fn(
        RuntimeValue,
        &'value CompiledProgram,
        &'value ExecutionBackingStore,
    ) -> Value<'value>;
```

Regla:

```text
RuntimeValue
+ CompiledProgram backing
+ ExecutionBackingStore backing
        ↓
observe_runtime_value
        ↓
Value<'value>
```

Invariantes:

- fixed scalars se observan inline;
- `StringBackingRef::Compiled` borrowea el string del Constant Pool;
- `StringBackingRef::Execution` borrowea el string del `ExecutionBackingStore`;
- Dynamic Integer se expone mediante la representación neutral canónica de `evo-values`;
- Struct y Enum pueden materializar árboles temporales owned de descriptors `Value<'value>` mientras el backing pesado permanece borrowed;
- no se copia backing completo salvo la canonicalización temporal expresamente permitida por el interchange model;
- no escapan `RuntimeValue`, backing IDs ni handles de VM a `ExternalCapability`;
- no produce failure normal: los handles provienen de una `VmExecution` válida;
- no conoce `ApplicationBindings`, `SignatureSymbol`, Resolver, Provider ni failure semantics.

## RSD-034 — `matches_owned_value_shape` es Tool de validación del resultado externo

Status: CLOSED

La validación del `OwnedValue` retornado por una `ExternalCapability` contra la shape compilada esperada es una operación de frontera independiente del lookup y de la invocación externa.

Firma cerrada:

```rust
pub type MatchesOwnedValueShape =
    fn(
        &OwnedValue,
        CompiledValueShapeId,
        &[CompiledValueShape],
    ) -> bool;
```

Invariantes:

- realiza exact recursive match;
- no realiza coerciones;
- valida family, numeric width, cardinalidad, canonical order, enum ordinal y payload shape;
- `Dynamic` solo coincide con `OwnedValue::Dynamic`;
- no conoce `ExternalSymbol`, `SignatureSymbol`, `ApplicationBindings` ni `ExecutionFailure`;
- responde únicamente `true/false`;
- el Resolver conserva la responsabilidad de traducir `false` a `ExternalExecutionFailure::ResultContractMismatch`.

No se fuerza una abstracción genérica común con `matches_value_shape`: `Value<'a>` y `OwnedValue` son tipos canónicos distintos y no se introduce trait/genérico de storage únicamente para compartir implementación.

## RSD-035 — `materialize_owned_value` es Tool de transferencia hacia RuntimeValue

Status: CLOSED

Un `OwnedValue` externo validado debe convertirse a la representación privada de ejecución y transferir ownership al `ExecutionBackingStore` cuando corresponda.

Firma cerrada:

```rust
pub type MaterializeOwnedValue =
    fn(
        OwnedValue,
        &mut ExecutionBackingStore,
    ) -> RuntimeValue;
```

Regla:

```text
OwnedValue
    ↓ ownership transfer
materialize_owned_value
    ↓
RuntimeValue
+ ExecutionBackingStore when required
```

Invariantes:

- fixed scalars permanecen inline;
- String transfiere su `Box<str>` al store de ejecución;
- Dynamic Integer transfiere/materializa su magnitud hacia el backing arithmetic owned de la ejecución;
- Struct y Enum se materializan recursivamente a backing de ejecución;
- no conserva borrow hacia el `OwnedValue` consumido;
- no vuelve a validar shape; el caller debe haber validado antes;
- no produce failure normal después de recibir un `OwnedValue` válido conforme al contrato compilado;
- no conoce `ApplicationBindings`, Provider ni failure translation.

## RSD-036 — Inventario exacto de `resolve_external_call`

Status: CLOSED

El Resolver utiliza cuatro Tools arquitectónicas, tres propias de adaptación de frontera y una ya cerrada/reutilizada para provenance:

```text
resolve_external_call
├── observe_runtime_value       Tool
├── matches_owned_value_shape   Tool
├── materialize_owned_value     Tool
└── locate_source_span          Tool reutilizada
```

Flujo cerrado:

```text
current CallExternal(ExternalSymbolId)
        ↓
ExternalSymbol
        ↓
SignatureSymbol + parameter_count + result_shape
        ↓
ApplicationBindings lookup
        │
        ├── missing
        │      ↓
        │   locate_source_span
        │      ↓
        │   External(MissingBinding)
        │
        └── capability found
               ↓
          observe top N RuntimeValues
               ↓
          Vec<Value<'call>> temporary descriptors
               ↓
          ExternalCapability(&arguments)
               │
               ├── Err(ExternalCapabilityFailure)
               │      ↓
               │   locate_source_span
               │      ↓
               │   External(CapabilityFailure)
               │
               └── Ok(OwnedValue)
                      ↓
                 matches_owned_value_shape
                      │
                      ├── false
                      │      ↓
                      │   locate_source_span
                      │      ↓
                      │   External(ResultContractMismatch)
                      │
                      └── true
                             ↓
                        materialize_owned_value
                             ↓
                        RuntimeValue
                             ↓
                        truncate/remove N argument cells
                        push result
                        ip += 1
```

Las siguientes operaciones permanecen privadas del Resolver y no se promueven a Participants:

```text
leer current ExternalSymbolId
resolver ExternalSymbol por índice
lookup HashMap<SignatureSymbol, ExternalCapability>
calcular slice lógico top-N
construir Vec temporal de Value descriptors
aplicar commit N → 1
avanzar IP después de success
construir ExternalExecutionFailure contextualizado
```

Inventario arquitectónico de la rama:

```text
Resolver       1  resolve_external_call
Contract       0 adicional
Requester      0
Collaborator   0
Tool           4 usadas
               3 nuevas
               1 reutilizada
```

La frontera física ejecutable sigue siendo exactamente `ExternalCapability`; no se crea un wrapper `Contract` paralelo.

## Closure

```text
RSD-033 observe_runtime_value Tool               ✅ CLOSED
RSD-034 matches_owned_value_shape Tool           ✅ CLOSED
RSD-035 materialize_owned_value Tool             ✅ CLOSED
RSD-036 resolve_external_call exact inventory    ✅ CLOSED

External Call Resolution Design                  ✅ CLOSED
Execution Participant Design root                READY FOR FINAL CLOSURE
```
