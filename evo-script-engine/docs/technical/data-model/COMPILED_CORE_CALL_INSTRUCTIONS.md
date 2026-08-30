# Evo-Script Engine — Compiled Core Data Movement and Calls

Status: CLOSED

Este documento cierra las Instructions base de movimiento de Values y la familia de calls de `evo-script-engine` v0 detectadas durante la auditoría final de `Compiled Program / Bytecode Data`.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`;
- `SEMANTIC_PROGRAM_STRUCTURE.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_STORAGE_DATA.md`;
- `COMPILED_CONTROL_FLOW.md`.

## 1. Principle

Regla canónica:

> Bytecode Compiler transforma bindings y call targets semánticos ya resueltos en mecanismos físicos directos. La VM no reconstruye BindingId, SignatureId, aliases ni nombres locales para cargar Values o invocar funciones.

## 2. Core data movement Instructions

Representación cerrada:

```rust
Instruction::LoadConstant(ConstantId)
Instruction::LoadParameter(ParameterSlot)
Instruction::LoadLocal(LocalSlot)
Instruction::StoreLocal(LocalSlot)
```

Stack contracts:

```text
LoadConstant    0 → 1
LoadParameter   0 → 1
LoadLocal       0 → 1
StoreLocal      1 → 0
```

## 3. LoadConstant

`LoadConstant(ConstantId)` materializa sobre el Operand Window el Value ejecutable correspondiente a `CompiledProgram.constants[id]`.

```text
ConstantId
    ↓ Constant Pool
Constant
    ↓ runtime materialization
Value
```

La forma concreta de borrowing/ownership de strings, dynamic integers u otros backing data pertenece a `VM Execution Data`.

No se introduce `PushInt`, `PushString`, `PushBoolean` u otra familia duplicada mientras `ConstantId + Constant` expresen completamente la responsabilidad.

## 4. LoadParameter

```rust
Instruction::LoadParameter(ParameterSlot)
```

Lee el Value Parameter estable del frame activo y materializa su Value sobre el Operand Window.

```text
ParameterSlot
    → stable frame parameter
    → operand Value
```

Los Parameters son inmutables en Evo-Script v0; no existe `StoreParameter`.

## 5. LoadLocal

```rust
Instruction::LoadLocal(LocalSlot)
```

Lee un Value binding estable no-parameter ya inicializado y materializa su Value sobre el Operand Window.

`LocalSlot` puede representar:

```text
let binding
Associated when extraction
Structured when extraction
```

## 6. StoreLocal

```rust
Instruction::StoreLocal(LocalSlot)
```

Consume el Value superior del Operand Window y materializa la inicialización física del `LocalSlot` indicado.

No representa mutabilidad semántica ni reassignment del lenguaje.

Usos canónicos:

```text
SemanticStatement::Bind
    expression
    → StoreLocal

when extraction binding
    extracted Value
    → StoreLocal
```

Invariante de Compiled Program válido:

> Para todo execution path que alcance `LoadLocal(slot)`, la inicialización física correspondiente de ese `LocalSlot` ya ocurrió en ese path.

Bytecode Compiler garantiza esta propiedad a partir de un Semantic Program válido; la VM no realiza semantic definite-assignment analysis.

## 7. Internal Call

Representación cerrada:

```rust
Instruction::Call(FunctionId)
```

`FunctionId` ya fue resuelto durante Semantic Analysis y se preserva dentro de `CompiledProgram.functions`.

La aridad física se obtiene de:

```text
CompiledProgram.functions[target].parameter_count
```

Si el target posee `N` Value Parameters físicos:

```text
Call(FunctionId)

stack effect:
N argument Values → 1 result Value
```

Los argumentos se evalúan y colocan en orden semántico izquierda-a-derecha antes de la call.

Signature Dependency Parameters no forman Values físicos y no participan en `parameter_count`.

La mecánica exacta de creación/reutilización de Call Frame, frame_base y transferencia del result pertenece a `VM Execution Data`.

## 8. External Call

Representación cerrada:

```rust
Instruction::CallExternal(ExternalSymbolId)
```

Direct Signature y Signature Dependency calls ya convergen durante compilation al mismo `ExternalSymbolId`.

La aridad física se obtiene de:

```text
CompiledProgram.external_symbols[target].parameter_count
```

Si el External Symbol posee `N` Value Parameters físicos:

```text
CallExternal(ExternalSymbolId)

stack effect:
N argument Values → 1 result Value
```

La VM resuelve `ExternalSymbol.symbol` mediante explicit Application Bindings. No existe Provider lookup ambiental ni Current Provider.

## 9. ExternalSymbol physical parameter_count

Representación persistente requerida:

```rust
struct ExternalSymbol {
    symbol: SignatureSymbol,
    parameter_count: usize,
}
```

`parameter_count` cuenta exclusivamente parámetros físicos `Value` de la `SemanticSignature` origen.

```text
SemanticSignatureParameter::Value(...)
    → counts 1

SemanticSignatureParameter::SignatureDependency(...)
    → counts 0
    → erased from physical Value calling convention
```

Esto no reintroduce semantic parameter type metadata. Es únicamente el dato ejecutable mínimo que permite a `CallExternal(ExternalSymbolId)` conocer cuántos Values consumir.

La aridad pertenece al External Symbol compilado y no se duplica en cada call site.

No se elige:

```rust
CallExternal {
    symbol: ExternalSymbolId,
    parameter_count: usize,
}
```

porque repetiría el mismo contrato físico en cada occurrence.

## 10. Signature Dependency erasure remains closed

Una `SemanticArgument::SignatureDependency` no genera operand Value.

```text
SemanticArgument::Value(expr)
    → evaluate expr
    → one physical argument Value

SemanticArgument::SignatureDependency(...)
    → no Value
    → no ParameterSlot
    → no Operand
```

Por tanto:

```text
semantic argument count
    may differ from
physical Value argument count
```

El forwarding contractual ya fue resuelto por Bytecode Compiler.

## 11. No call-site semantic metadata

No se introducen en call Instructions:

```text
argument TypeId list
result TypeId
SignatureId
SignatureBindingId
parameter names
function name
provider identity
```

La VM ejecuta el mecanismo ya resuelto.

## 12. SourceMap consequence

Cada occurrence de:

```text
Call(FunctionId)
CallExternal(ExternalSymbolId)
```

posee su propio `SourceSpan` mediante `SourceMap`, normalmente el span de la Semantic Call expression responsable.

La ubicación pertenece a la Instruction occurrence; no se agrega un source mapping separado a `FunctionId` o `ExternalSymbolId`.

## 13. Closure

```text
LoadConstant                              ✅ CLOSED
LoadParameter                             ✅ CLOSED
LoadLocal                                 ✅ CLOSED
StoreLocal                                ✅ CLOSED
StoreParameter                            ❌ NOT NEEDED v0
local initialization path invariant       ✅ CLOSED
Call(FunctionId)                          ✅ CLOSED
CallExternal(ExternalSymbolId)            ✅ CLOSED
internal physical arity                   ✅ CLOSED via CompiledFunction.parameter_count
external physical arity                   ✅ CLOSED via ExternalSymbol.parameter_count
Signature Dependency physical erasure     ✅ CLOSED / PRESERVED
call-site semantic metadata               ❌ NOT NEEDED

Core Load / Store                         ✅ CLOSED
Internal / External Calls                 ✅ CLOSED
```
