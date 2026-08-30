# Evo-Script Engine — Compiled Storage Data

Status: CLOSED — REVALIDATED AFTER FINAL AUDIT

Este documento cierra las identidades físicas mínimas de storage persistente de `Compiled Program / Bytecode Data` de `evo-script-engine` v0.

La autoridad deriva de `COMPILED_PROGRAM_DATA.md`, `TECHNICAL_DESIGN.md` y de la auditoría final que cerró la familia `Core Load / Store + Calls`.

## 1. ParameterSlot

Representación cerrada:

```rust
struct ParameterSlot(usize);
```

`ParameterSlot` identifica la posición lógica de un Value Parameter dentro de una `CompiledFunction`.

Namespace:

```text
CompiledFunction
└── ParameterSlot namespace
```

Relación física conceptual:

```text
absolute position
    = frame_base + ParameterSlot
```

Invariantes:

1. `ParameterSlot` es local a una `CompiledFunction`.
2. No es un índice absoluto dentro del Shared Value Storage.
3. Solo existen slots para Value Parameters físicos.
4. Signature Dependency Parameters fueron erased durante Bytecode lowering y no poseen `ParameterSlot`.
5. Parameters son inmutables desde la semántica de Evo-Script; no se define `StoreParameter` en v0.

## 2. LocalSlot

Representación cerrada:

```rust
struct LocalSlot(usize);
```

`LocalSlot` identifica la posición lógica de un Value binding estable no-parameter dentro de una `CompiledFunction`.

Puede representar storage para:

```text
Let binding
Associated when extraction
Structured when extraction
```

Relación física conceptual:

```text
absolute position
    = frame_base
    + parameter_count
    + LocalSlot
```

Invariantes:

1. `LocalSlot` es local a una `CompiledFunction`.
2. No es un índice absoluto de Shared Value Storage.
3. `LocalSlot` y `ParameterSlot` son namespaces e identidades lógicas distintas.
4. El hecho de compartir backing storage no los convierte en `FrameSlot` único.

## 3. Operand Base Derivation

Con `parameter_count` y `local_count` cerrados en `CompiledFunction`:

```text
operand_base
    = frame_base
    + parameter_count
    + local_count
```

La región posterior pertenece al Operand Window temporal del frame.

```text
frame_base
    ↓
[parameters][locals][temporaries...]
                     ↑
                 operand_base
```

No se introduce `OperandSlot` persistente en Compiled Program. Los operands son stack temporaries y su estado pertenece a VM Execution Data.

## 4. BindingId → Slot mapping

La traducción:

```text
BindingId
    → ParameterSlot | LocalSlot
```

es Compilation Working State temporal del Bytecode Compiler.

Ejemplo:

```text
BindingId(0) → ParameterSlot(0)
BindingId(1) → LocalSlot(0)
BindingId(2) → LocalSlot(1)
```

Una vez emitidas las Instructions que contienen slots físicos, esta tabla no forma parte de `CompiledProgram`.

No se introducen:

```text
CompiledBindingId
BindingTable
Persistent BindingLocation
Generic FrameSlot
```

## 5. ExternalSymbol

Representación cerrada y corregida por la auditoría final:

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
}
```

`ExternalSymbolId(n)` referencia:

```text
CompiledProgram.external_symbols[n]
```

`SignatureSymbol` conserva la identidad contractual canónica `module::signature` y es owned por el `CompiledProgram` a través de `ExternalSymbol`.

`parameter_count` expresa la aridad física de Value Parameters requerida por `CallExternal(ExternalSymbolId)`.

Se calcula desde la `SemanticSignature` origen:

```text
SemanticSignatureParameter::Value(...)
    → counts 1

SemanticSignatureParameter::SignatureDependency(...)
    → counts 0
    → erased from physical calling convention
```

No contiene:

```text
Provider
runtime function pointer
runtime binding
root path
implementation .efn path
Current Provider
parameter TypeId list
result TypeId
```

La resolución hacia una implementación concreta ocurre únicamente mediante explicit Application Bindings durante ejecución/composición.

`parameter_count` no es metadata semántica redundante: es mecanismo ejecutable mínimo para conocer cuántos operand Values consume una external call sin repetir la aridad en cada call site.

## 6. Constant — canonical physical representation

`Constant` representa datos persistentes ya materializables por la VM. No reutiliza `SemanticLiteral`, porque Semantic Program conserva significado mientras Constant Pool conserva representación ejecutable owned.

La auditoría final eliminó las variantes redundantes `Int(i32)` y `Float(f64)` porque el Compiled Program ya había cerrado la canonicalización física:

```text
NativeType::Int
NativeType::Int32
    → NumericKind::Int32

NativeType::Float
NativeType::Float64
    → NumericKind::Float64
```

La misma canonicalización aplica al Constant Pool.

Representación cerrada:

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

Lowering canónico:

```text
semantic int literal / value constant
    → Constant::Int32

semantic int32 literal / value constant
    → Constant::Int32

semantic float literal / value constant
    → Constant::Float64

semantic float64 literal / value constant
    → Constant::Float64
```

Esto no convierte `int` e `int32`, ni `float` y `float64`, en aliases semánticos. La distinción ya cumplió su responsabilidad antes del producto compilado; ambos pares comparten mecanismo físico.

`Constant` no conserva `TypeId`; la variant física expresa la representación necesaria para ejecución.

`String(String)` owns su contenido y puede sobrevivir al Source Text.

## 7. DynamicConstant

Representación cerrada:

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

Para integer dynamic:

```text
magnitude
    = minimal unsigned big-endian magnitude
```

Representación canónica:

```text
zero     → empty Vec
non-zero → minimal big-endian bytes, sin leading zero bytes
```

`negative` solo tiene efecto para magnitud non-zero. Zero posee representación canónica no-negativa:

```text
negative = false
magnitude = []
```

Esto garantiza una única representación persistente para cero y evita parsear decimal durante cada ejecución.

La estrategia runtime concreta de arbitrary-precision arithmetic se define después en VM / Value Data; `DynamicConstant` no prescribe BigInt crate, limb size ni allocator.

## 8. Signed minimum literal lowering

Semantic Program representa el signo negativo mediante Unary Negate sobre una magnitud Integer. Bytecode Compiler puede combinar una forma literal negada hacia una Constant signed válida para representar correctamente los mínimos del dominio.

Ejemplo conceptual:

```text
Semantic
Negate(Integer("128"))
+ target int8

        ↓ Bytecode Compiler

Constant::Int8(-128)
```

La misma regla puede aplicar a `int16`, `int32`, `int64`, `int128` y dynamic integer negativo.

No se materializa primero un valor positivo fuera de rango para después negarlo.

## 9. Constant Pool policy

`CompiledProgram.constants: Vec<Constant>` es el Constant Pool owned de v0.

```text
ConstantId(n)
    → CompiledProgram.constants[n]
```

Constant interning/deduplication es una optimización permitida del Bytecode Compiler, no una invariante de validez del `CompiledProgram`.

Por tanto dos `ConstantId` distintos pueden referenciar valores iguales sin invalidar el programa compilado.

## 10. Core storage Instructions boundary

Las Instructions que consumen estas identities se cierran en `COMPILED_CORE_CALL_INSTRUCTIONS.md`:

```text
LoadConstant(ConstantId)
LoadParameter(ParameterSlot)
LoadLocal(LocalSlot)
StoreLocal(LocalSlot)
Call(FunctionId)
CallExternal(ExternalSymbolId)
```

Separación:

```text
Compiled Storage Data
    = persistent identities / data

Core / Call Instructions
    = executable operations over those identities
```

## 11. Closure

```text
ParameterSlot                         ✅ CLOSED
LocalSlot                             ✅ CLOSED
Parameter / Local logical separation ✅ CLOSED
operand_base derivation               ✅ CLOSED
BindingId → Slot mapping temporary    ✅ CLOSED
ExternalSymbol                        ✅ CLOSED — corrected
ExternalSymbol.parameter_count        ✅ CLOSED
Signature Dependency physical erasure ✅ CLOSED
Constant                              ✅ CLOSED — canonicalized
Int/Int32 constant duplication        ❌ REMOVED
Float/Float64 constant duplication    ❌ REMOVED
DynamicConstant                       ✅ CLOSED
arbitrary integer binary magnitude    ✅ CLOSED
signed-min literal lowering           ✅ CLOSED
Constant Pool ownership policy        ✅ CLOSED
constant interning optional           ✅ CLOSED

Core Load / Store + Calls             ✅ CLOSED in COMPILED_CORE_CALL_INSTRUCTIONS.md
Compiled Program exact inventory      ← NEXT after final audit corrections
```
