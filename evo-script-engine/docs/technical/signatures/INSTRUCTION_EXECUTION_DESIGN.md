# Evo-Script Engine — Instruction Execution Design

Status: INSTRUCTION EXECUTION DESIGN — CLOSED

Este documento cierra los Participants internos y Tools requeridos por `execute_instruction`, cuya firma raíz está cerrada en `EXECUTION_PARTICIPANT_DESIGN.md`.

Firma raíz:

```rust
pub type ExecuteInstruction =
    for<'compiled, 'bindings> fn(
        &mut VmExecution<'compiled, 'bindings>,
    ) -> Result<
        Option<OwnedValue>,
        ExecutionFailure,
    >;
```

`CallExternal` está explícitamente fuera de esta responsabilidad y pertenece a `resolve_external_call`.

## RSD-030 — `own_runtime_value` es Tool de materialización de Outcome

Status: CLOSED

`RuntimeValue` es un descriptor relativo a una `VmExecution` y no puede escapar como resultado autónomo. La transformación hacia `OwnedValue` representa una responsabilidad genérica de adaptación entre el modelo privado de VM y el modelo owned de intercambio.

Firma cerrada:

```rust
pub type OwnRuntimeValue =
    fn(
        RuntimeValue,
        &CompiledProgram,
        &ExecutionBackingStore,
    ) -> OwnedValue;
```

Regla:

```text
RuntimeValue
+ CompiledProgram backing
+ ExecutionBackingStore backing
        ↓
own_runtime_value
        ↓
OwnedValue
```

Invariantes:

- fixed scalars se materializan directamente;
- `StringBackingRef::Compiled` observa el Constant Pool y produce String owned;
- `StringBackingRef::Execution` observa execution backing y produce String owned;
- Dynamic Integer produce su representación neutral owned canónica;
- Struct y Enum se materializan recursivamente como `OwnedValue` autónomo;
- no escapan backing IDs, `RuntimeValue`, `FunctionId`, frame state ni references a `VmExecution`;
- no existe failure normal: los handles observados pertenecen a una `VmExecution` válida;
- la Tool no decide cuándo termina la ejecución; únicamente materializa el dato.

`execute_instruction` utiliza esta Tool únicamente cuando un `Return` corresponde al entry frame.

## RSD-031 — `locate_source_span` es Tool diagnóstica interna

Status: CLOSED

Materializar la provenance de la instruction activa es una operación interna pequeña e independiente de la familia de failure que la solicita.

Firma cerrada:

```rust
pub type LocateSourceSpan =
    fn(
        &CompiledProgram,
        &CallFrame,
    ) -> SourceSpan;
```

Resolución:

```text
CallFrame.function
+ CallFrame.instruction_pointer
        ↓
CompiledProgram.source_map
        ↓
SourceSpan
```

Invariantes:

- un `CompiledProgram` válido posee dense SourceMap para cada instruction;
- el active `InstructionPointer` siempre identifica una instruction válida;
- no retorna `Option` ni `Result` por estados que serían invariant violations;
- no conoce `ExecutionFailureKind`;
- no formatea line/column/snippet;
- puede ser reutilizada por `execute_instruction` y `resolve_external_call`.

## RSD-032 — Inventario exacto de `execute_instruction`

Status: CLOSED

Las 47 Instructions no externas se mantienen dentro de una sola responsabilidad arquitectónica:

```text
execute_instruction
├── core data movement
├── internal Call
├── fixed numeric
├── dynamic numeric
├── control flow
├── conversions
├── bool/string operations
├── composite mechanics
├── structural equality
└── Return
```

Estas familias no se convierten en Collaborators distintos porque todas expresan mecanismos internos de la misma responsabilidad: ejecutar la instruction activa de la VM.

Pueden existir funciones privadas como:

```text
execute_numeric
execute_dynamic_numeric
execute_conversion
execute_composite
execute_equality
push_operand / pop_operand
observe_parameter / observe_local
create_call_frame
return_from_frame
```

sin adquirir identidad arquitectónica por ese hecho.

Tools arquitectónicas demostradas para esta rama:

```text
execute_instruction
├── own_runtime_value   Tool
└── locate_source_span  Tool
```

`locate_source_span` se utiliza cuando una operación produce uno de los cuatro `EvaluationFailure` normales:

```text
Overflow
DivisionByZero
Conversion
DynamicNumericType
```

El Collaborator construye directamente:

```text
ExecutionFailure {
    kind: Evaluation(...),
    source_span: Some(span),
}
```

No existe `InstructionError`, `VmError` o error intermedio sin semántica propia.

Para entry `Return`:

```text
RuntimeValue result
    ↓ own_runtime_value
OwnedValue
    ↓
Ok(Some(result))
```

Para internal `Return`:

```text
RuntimeValue descriptor
    ↓
truncate callee region
remove callee frame
push descriptor for caller
advance caller IP
    ↓
Ok(None)
```

Inventario arquitectónico de la rama:

```text
Collaborator   1  execute_instruction
Contract       0
Resolver       0
Requester      0
Tool           2  own_runtime_value, locate_source_span
```

## Closure

```text
RSD-030 own_runtime_value Tool              ✅ CLOSED
RSD-031 locate_source_span Tool             ✅ CLOSED
RSD-032 execute_instruction exact inventory ✅ CLOSED

Instruction Execution Design                ✅ CLOSED
resolve_external_call internals             ← NEXT
```
