# Evo-Script Engine — Compiled Control Flow

Status: CLOSED — REVALIDATED

Este documento cierra las identities y reglas de bytecode para control flow, short-circuit boolean evaluation, discard y return de `evo-script-engine` v0.

La autoridad deriva de `TECHNICAL_DESIGN.md`, `COMPILED_PROGRAM_DATA.md`, `COMPILED_STORAGE_DATA.md`, `COMPILED_NUMERIC_INSTRUCTIONS.md`, `SEMANTIC_EXPRESSIONS.md` y la especificación Evo-Script v0.1.

## 1. Instruction representation

`Instruction` es un enum tipado. Cada variant expresa el opcode conceptual junto con exactamente los operands persistentes que necesita.

No se introduce `Opcode + generic operands`.

La familia base visible en este documento es:

```rust
enum Instruction {
    LoadConstant(ConstantId),
    LoadParameter(ParameterSlot),
    LoadLocal(LocalSlot),
    StoreLocal(LocalSlot),

    Discard,

    Jump(InstructionIndex),
    JumpIfFalse(InstructionIndex),

    Return,

    // otras familias cerradas en sus documentos especializados
}
```

`Load*`, `StoreLocal`, `Call` y `CallExternal` están cerrados en `COMPILED_CORE_CALL_INSTRUCTIONS.md`. Numeric, conversions, composite y equality families están cerradas en sus documentos correspondientes. El enum completo se consolida en el Exact Compiled Inventory.

## 2. InstructionIndex

```rust
struct InstructionIndex(usize);
```

`InstructionIndex` identifica una posición persistente dentro del bytecode de una única `CompiledFunction`.

```text
InstructionIndex(n)
    → CompiledFunction.instructions[n]
```

Invariantes:

1. local a una `CompiledFunction`;
2. todo branch target referencia una Instruction válida de esa misma función;
3. no es byte offset;
4. no es physical address;
5. no es `InstructionPointer`.

```text
InstructionIndex
    = persistent bytecode position

InstructionPointer
    = mutable VM Execution state
```

## 3. Absolute branch targets

v0 usa:

```rust
Jump(InstructionIndex)
JumpIfFalse(InstructionIndex)
```

No usa relative byte offsets.

Bytecode Compiler puede utilizar labels temporales durante emission y resolverlos finalmente a `InstructionIndex`. Esos labels son Compilation Working State y no sobreviven.

No existe `Label` runtime.

## 4. Jump

```rust
Instruction::Jump(InstructionIndex)
```

Stack effect:

```text
0 → 0
```

Semántica:

```text
next instruction := target
```

Los jumps de v0 derivan de `&&`, `||` y `when`; no existe sintaxis de loop ni loop opcode.

## 5. JumpIfFalse

```rust
Instruction::JumpIfFalse(InstructionIndex)
```

Stack effect:

```text
1 bool → 0
```

Semántica:

```text
condition == false
    → next instruction := target

condition == true
    → continue
```

La condición se consume siempre.

No existe `JumpIfTrue` porque `Jump + JumpIfFalse` cubren las necesidades de v0.

## 6. Short-circuit `&&`

No existe eager `AndBoolean`.

Lowering conceptual:

```text
evaluate left
JumpIfFalse(false_branch)

evaluate right
Jump(end)

false_branch:
LoadConstant(false)

end:
```

Resultado: exactamente un bool.

`right` solo se evalúa cuando `left == true`.

## 7. Short-circuit `||`

No existe eager `OrBoolean`.

Lowering conceptual:

```text
evaluate left
JumpIfFalse(evaluate_right)

LoadConstant(true)
Jump(end)

evaluate_right:
evaluate right

end:
```

Resultado: exactamente un bool.

`right` solo se evalúa cuando `left == false`.

## 8. Discard

```rust
Instruction::Discard
```

Stack effect:

```text
1 → 0
```

Uso principal: cerrar un `Operation Statement` cuyo Value normal no se utiliza.

```text
CALL / CALL_EXTERNAL
DISCARD
```

No quedan temporaries semánticamente abandonados después del statement.

## 9. Return

```rust
Instruction::Return
```

Toda `CompiledFunction` retorna exactamente un Value normal en ejecución exitosa.

```text
... result
Return
→ result transferred to caller
```

Para entry point, el Value se convierte en resultado exterior.

La transferencia física caller/callee pertenece a `VM Execution Data`.

No existen:

```text
ReturnVoid
multiple return values
exception return value
Result wrapper
```

## 10. `when` uses the same branch infrastructure

`when` no introduce:

```text
Instruction::When
Match opcode
Pattern opcode
```

Su control flow usa `Jump` / `JumpIfFalse` junto con la familia ya cerrada en `COMPILED_COMPOSITE_INSTRUCTIONS.md`:

```text
TestVariant
ExtractEnumAssociated
ExtractEnumStructured
```

Por tanto:

```text
when branch infrastructure   ✅ CLOSED here
when enum inspection         ✅ CLOSED in composite instructions
```

## 11. No hidden Host control flow

No existen instructions para:

```text
Active Scope
Host Session State
SET_SCOPE
use
Current Provider
try/catch
throw
```

## 12. Closure

```text
Instruction typed-enum representation      ✅ CLOSED
InstructionIndex                           ✅ CLOSED
absolute branch targets                    ✅ CLOSED
compiler temporary labels                  ✅ CLOSED — not product data
Jump                                       ✅ CLOSED
JumpIfFalse                                ✅ CLOSED
JumpIfTrue                                 ❌ NOT NEEDED v0
&& short-circuit lowering                  ✅ CLOSED
|| short-circuit lowering                  ✅ CLOSED
And/Or eager instructions                  ❌ EXCLUDED
Discard                                    ✅ CLOSED
Return                                     ✅ CLOSED
when branch infrastructure                 ✅ CLOSED
when enum inspection                       ✅ CLOSED elsewhere
InstructionPointer separation              ✅ CLOSED
Core Load / Store + Calls                  ✅ CLOSED elsewhere
```
