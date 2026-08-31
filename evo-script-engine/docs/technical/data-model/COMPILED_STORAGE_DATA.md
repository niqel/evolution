# Evo-Script Engine — Compiled Storage Data

Status: CLOSED — REVALIDATED AFTER BOUNDARY SHAPE CORRECTION

Este documento cierra las identities físicas mínimas de storage persistente de `Compiled Program / Bytecode Data` de `evo-script-engine` v0.

La autoridad deriva de:

- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_BOUNDARY_VALUE_SHAPE.md`;
- `TECHNICAL_DESIGN.md`;
- la auditoría de Core Load / Store + Calls.

## 1. ParameterSlot

```rust
struct ParameterSlot(usize);
```

`ParameterSlot` identifica la posición lógica de un Value Parameter dentro de una `CompiledFunction`.

```text
absolute position
    = frame_base + ParameterSlot
```

Invariantes:

1. local a una `CompiledFunction`;
2. no es índice absoluto del Shared Value Storage;
3. solo existe para Value Parameters físicos;
4. Signature Dependency Parameters fueron erased;
5. no existe `StoreParameter` en v0.

## 2. LocalSlot

```rust
struct LocalSlot(usize);
```

Representa un Value binding estable no-parameter:

```text
Let binding
Associated when extraction
Structured when extraction
```

```text
absolute position
    = frame_base
    + parameter_count
    + LocalSlot
```

`LocalSlot` y `ParameterSlot` permanecen identities distintas.

## 3. Operand Base Derivation

```text
operand_base
    = frame_base
    + parameter_count
    + local_count
```

No existe `OperandSlot` persistente.

## 4. BindingId → Slot mapping

El mapping:

```text
BindingId → ParameterSlot | LocalSlot
```

es Compilation Working State temporal y desaparece después de emitir bytecode.

No se introducen:

```text
CompiledBindingId
BindingTable
Persistent BindingLocation
Generic FrameSlot
```

## 5. ExternalSymbol

Representación corregida y cerrada:

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
    result_shape: CompiledValueShapeId,
}
```

Owner:

```text
ExternalSymbolId(n)
    → CompiledProgram.external_symbols[n]
```

`symbol` conserva la identity contractual `module::signature`.

`parameter_count` cuenta únicamente:

```text
SemanticSignatureParameter::Value → +1
SemanticSignatureParameter::SignatureDependency → +0
```

Signature Dependencies continúan erased de la calling convention física.

`result_shape` representa el expected boundary Value shape del resultado devuelto por la `ExternalCapability` uniforme.

Flujo:

```text
ExternalCapability Success(OwnedValue)
    ↓
validate against ExternalSymbol.result_shape
    ├── mismatch → execution Failure; no N→1 commit
    └── match → materialize RuntimeValue → commit N→1
```

No persisten external argument shapes en v0 porque los argumentos provienen de bytecode ya semanticamente validado.

No contiene:

```text
Provider
runtime function pointer
runtime binding
parameter TypeId list
result TypeId
parameter CompiledValueShapeId list
Current Provider
```

## 6. Constant — canonical physical representation

```rust
enum Constant {
    Boolean(bool),
    String(String),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),

    Float32(f32),
    Float64(f64),

    Dynamic(DynamicConstant),
}
```

Canonical lowering:

```text
int / int32     → Constant::Int32
float / float64 → Constant::Float64
```

No existen `Constant::Int` ni `Constant::Float` separados.

`Constant::String(String)` owns su contenido y puede sobrevivir al Source Text.

## 7. DynamicConstant

```rust
enum DynamicConstant {
    Integer {
        negative: bool,
        magnitude: Vec<u8>,
    },
    Float32(f32),
    Float64(f64),
}
```

Dynamic Integer usa:

```text
minimal unsigned big-endian magnitude
zero = negative false + empty magnitude
```

No prescribe BigInt crate ni runtime arithmetic representation.

## 8. Signed minimum literal lowering

Bytecode Compiler puede combinar Unary Negate + literal magnitude para producir directamente el mínimo signed válido.

Ejemplo:

```text
Negate(Integer("128")) under int8
    → Constant::Int8(-128)
```

No se materializa primero un positivo fuera de rango.

## 9. Constant Pool policy

```text
ConstantId(n)
    → CompiledProgram.constants[n]
```

Constant interning/deduplication es optimización opcional, no invariante de validez.

## 10. Boundary Value Shape ownership

Cerrado en `COMPILED_BOUNDARY_VALUE_SHAPE.md`:

```rust
struct CompiledValueShapeId(usize);
```

```text
CompiledValueShapeId(n)
    → CompiledProgram.value_shapes[n]
```

`CompiledProgram` conserva solo shapes transitivamente necesarias para entry Value Parameters y external results.

Esta tabla es boundary executable contract metadata y no runtime reflection general.

## 11. Core storage instruction boundary

Estas identities participan en:

```text
LoadConstant(ConstantId)
LoadParameter(ParameterSlot)
LoadLocal(LocalSlot)
StoreLocal(LocalSlot)
Call(FunctionId)
CallExternal(ExternalSymbolId)
```

El Instruction Set no cambia por la corrección de boundary shape.

## Closure

```text
ParameterSlot                                  ✅ CLOSED
LocalSlot                                      ✅ CLOSED
Parameter / Local logical separation           ✅ CLOSED
operand_base derivation                        ✅ CLOSED
BindingId → Slot mapping temporary             ✅ CLOSED
ExternalSymbol                                 ✅ CLOSED — corrected
ExternalSymbol.parameter_count                 ✅ CLOSED
ExternalSymbol.result_shape                    ✅ CLOSED
external argument shapes                       ❌ NOT PERSISTED
Signature Dependency physical erasure          ✅ CLOSED
Constant                                       ✅ CLOSED
DynamicConstant                                ✅ CLOSED
arbitrary integer binary magnitude             ✅ CLOSED
signed-min literal lowering                    ✅ CLOSED
Constant Pool ownership policy                 ✅ CLOSED
CompiledValueShapeId owner relation             ✅ CLOSED

Core Load / Store + Calls                      ✅ CLOSED
Compiled Program exact inventory               ✅ CLOSED — 21 identities
```