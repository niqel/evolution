# Evo-Script Engine — Execution Initialization Design

Status: EXECUTION INITIALIZATION DESIGN — CLOSED

Este documento cierra los Participants internos y Tools requeridos por `initialize_execution`, cuya firma raíz está cerrada en `EXECUTION_PARTICIPANT_DESIGN.md`.

La responsabilidad raíz es:

```rust
pub type Initialize =
    for<'compiled, 'value, 'bindings> fn(
        &'compiled CompiledProgram,
        &'value [Value<'value>],
        &'bindings ApplicationBindings,
    ) -> Result<
        VmExecution<'compiled, 'bindings>,
        ExecutionFailure,
    >;
```

## RSD-027 — `matches_value_shape` es Tool de frontera compilada

Status: CLOSED

La validación recursiva de un `Value<'value>` neutral contra una `CompiledValueShape` no pertenece específicamente a la responsabilidad de construir frames, reservar locals o crear `VmExecution`.

Representa una operación interna pequeña, reusable y semánticamente independiente:

> determinar si un Value borrowed de intercambio satisface exactamente una shape compilada de frontera.

Firma cerrada:

```rust
pub type MatchesValueShape =
    for<'value> fn(
        &Value<'value>,
        CompiledValueShapeId,
        &[CompiledValueShape],
    ) -> bool;
```

Invariantes:

- realiza exact recursive match;
- no realiza coerciones;
- `Dynamic` solo coincide con `Value::Dynamic`;
- Struct valida cardinalidad, orden y shapes nested;
- Enum valida ordinal, payload family, cardinalidad y shapes nested;
- no conoce `VmExecution`, Agent, Resolver, Requester, Provider ni `ApplicationBindings`;
- no produce `ExecutionFailure`; únicamente responde si el dato satisface el contrato compilado;
- un `CompiledValueShapeId` se interpreta contra la tabla `CompiledProgram.value_shapes` suministrada por el caller.

`initialize_execution` conserva la responsabilidad de convertir un mismatch en:

```text
ExecutionFailure {
    kind: Invocation(ArgumentShapeMismatch { position }),
    source_span: None,
}
```

No se introduce una familia de errores dentro de la Tool.

## RSD-028 — `materialize_value` es Tool de adaptación hacia RuntimeValue

Status: CLOSED

Transformar un `Value<'value>` neutral ya validado a la representación privada de ejecución no pertenece a la orquestación del initializer.

La operación materializa ownership únicamente donde la ejecución debe conservar el dato.

Firma cerrada:

```rust
pub type MaterializeValue =
    for<'value> fn(
        &Value<'value>,
        &mut ExecutionBackingStore,
    ) -> RuntimeValue;
```

Regla:

```text
Value<'value>
    ↓ materialize_value
RuntimeValue
    + ExecutionBackingStore when required
```

Invariantes:

- scalars fixed permanecen inline;
- String borrowed se copia a backing owned por `ExecutionBackingStore`;
- Dynamic Integer se materializa en el backing arithmetic owned de la ejecución;
- Struct y Enum se materializan recursivamente a backings owned por la ejecución;
- el `RuntimeValue` resultante puede sobrevivir al lifetime del `Invocation Value`;
- no conserva borrow hacia `Value<'value>`;
- no consulta `ApplicationBindings`;
- no realiza shape validation; recibe un Value que el caller ya validó;
- no produce failure normal: una representación neutral que viola sus invariantes cerrados sería integración/invariant violation, no `ExecutionFailure` de Evo-Script.

La Tool no introduce una abstracción universal de storage ni un trait de conversión.

## RSD-029 — Inventario exacto de `initialize_execution`

Status: CLOSED

`initialize_execution` utiliza exactamente las dos Tools arquitectónicas demostradas en esta rama:

```text
initialize_execution
│
├── matches_value_shape   Tool
└── materialize_value     Tool
```

El resto pertenece a implementación privada del Collaborator:

```text
comparar aridad
iterar Invocation Values por posición
construir SharedValueStorage inicial
reservar local_count cells None
construir entry CallFrame
construir VmExecution root
opcionalmente reservar capacity usando metadata compilada
```

No se crean Participants para esas operaciones.

Flujo conceptual:

```text
Invocation Values
      │
      ├── wrong arity
      │      ↓
      │   InvocationFailure::ArityMismatch
      │
      └── exact arity
             ↓
        each Value + expected shape
             ↓
        matches_value_shape
             │
             ├── false
             │      ↓
             │   InvocationFailure::ArgumentShapeMismatch
             │
             └── true
                    ↓
              materialize_value
                    ↓
              RuntimeValue Parameter cells
                    ↓
              reserve Locals
                    ↓
              entry CallFrame
                    ↓
              VmExecution
```

Inventario arquitectónico de la rama:

```text
Collaborator   1  initialize_execution
Contract       0
Resolver       0
Requester      0
Tool           2  matches_value_shape, materialize_value
```

`ApplicationBindings` permanece borrowed dentro de `VmExecution`, pero el initializer no cruza hacia ninguna `ExternalCapability`.

## Closure

```text
RSD-027 matches_value_shape Tool              ✅ CLOSED
RSD-028 materialize_value Tool                ✅ CLOSED
RSD-029 initialize_execution exact inventory  ✅ CLOSED

Execution Initialization Design               ✅ CLOSED
execute_instruction internals                  ← NEXT
resolve_external_call internals                PENDING
```
