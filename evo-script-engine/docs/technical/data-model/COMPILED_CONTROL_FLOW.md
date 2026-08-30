# Evo-Script Engine — Compiled Control Flow

Status: CLOSED

Este documento cierra las identities y reglas de bytecode para control flow, short-circuit boolean evaluation, discard y return de `evo-script-engine` v0.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`;
- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_STORAGE_DATA.md`;
- `COMPILED_NUMERIC_INSTRUCTIONS.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `evo-script/EVO_SCRIPT_SPECIFICATION_v0.1.md`.

## 1. Instruction representation

`Instruction` es un enum tipado. Cada variant expresa el opcode conceptual junto con exactamente los operands persistentes que necesita.

No se introduce un `Opcode` separado acompañado por operands genéricos, porque permitiría combinaciones técnicamente representables pero inválidas.

Forma parcial acumulativa:

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

    // calls, numeric, conversions, composite data, etc.
}
```

El enum completo se cierra cuando terminen todas las familias del Instruction Set.

## 2. InstructionIndex

Representación:

```rust
struct InstructionIndex(usize);
```

`InstructionIndex` identifica una posición persistente dentro del bytecode de una única `CompiledFunction`.

Namespace:

```text
CompiledFunction
└── instructions
    └── InstructionIndex namespace
```

Regla:

```text
InstructionIndex(n)
    → CompiledFunction.instructions[n]
```

Invariantes:

1. es local a una `CompiledFunction`;
2. todo branch target referencia una instruction válida de esa misma función;
3. no es un byte offset;
4. no es una address física;
5. no es `InstructionPointer`.

Separación canónica:

```text
InstructionIndex
    = persistent position in CompiledFunction bytecode

InstructionPointer
    = mutable VM Execution state
```

`InstructionPointer` pertenece a VM Execution Data.

## 3. Absolute branch targets

v0 utiliza targets absolutos dentro de `CompiledFunction.instructions`:

```rust
Jump(InstructionIndex)
JumpIfFalse(InstructionIndex)
```

No se utilizan relative byte offsets en el Technical Data Model v0.

La elección simplifica generation, patching, validation, inspection y diagnostics mientras `CompiledProgram` continúe siendo una estructura Rust persistente y no un formato binario serializado.

Bytecode Compiler puede utilizar labels temporales durante emission y resolverlos finalmente a `InstructionIndex`; dichos labels pertenecen a Compilation Working State y no sobreviven dentro de `CompiledProgram`.

No existe `Label` como instruction runtime.

## 4. Jump

```rust
Instruction::Jump(InstructionIndex)
```

Semántica:

```text
operand stack effect: 0 → 0
next instruction := target
```

`Jump` no inspecciona ni modifica Values.

En lowering de Evo-Script v0, los jumps generados por `&&`, `||` y `when` representan control flow derivado de expresiones finitas; no se introduce sintaxis de loop ni un loop opcode.

## 5. JumpIfFalse

```rust
Instruction::JumpIfFalse(InstructionIndex)
```

Stack contract:

```text
before
... condition(bool)

JumpIfFalse(target)

after
...
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
    → continue with following instruction
```

La condición se consume en ambos casos.

Bytecode Compiler solo produce `JumpIfFalse` después de semantic validation de una expresión `bool`; la VM no realiza type inference.

No se introduce `JumpIfTrue` en v0 porque `Jump` + `JumpIfFalse` expresan completamente las necesidades actuales.

## 6. Short-circuit `&&`

`&&` no posee eager binary instruction.

Lowering canónico conceptual:

```text
evaluate left
JumpIfFalse(false_branch)

evaluate right
Jump(end)

false_branch:
LoadConstant(false)

end:
```

El resultado deja exactamente un `bool` sobre el Operand Window.

Consecuencias:

1. `left` se evalúa primero;
2. `right` se evalúa únicamente si `left == true`;
3. si `left` produce `EvaluationError`, `right` no se evalúa;
4. no existe `AndBoolean` eager.

`false` se materializa mediante el Constant Pool (`Constant::Boolean(false)`) y `LoadConstant`; no se introduce `PushFalse` mientras no exista una necesidad independiente.

## 7. Short-circuit `||`

`||` tampoco posee eager binary instruction.

Lowering canónico conceptual:

```text
evaluate left
JumpIfFalse(evaluate_right)

LoadConstant(true)
Jump(end)

evaluate_right:
evaluate right

end:
```

El resultado deja exactamente un `bool` sobre el Operand Window.

Consecuencias:

1. `left` se evalúa primero;
2. `right` se evalúa únicamente si `left == false`;
3. si `left` produce `EvaluationError`, `right` no se evalúa;
4. no existe `OrBoolean` eager.

## 8. Discard

```rust
Instruction::Discard
```

Stack effect:

```text
1 → 0
```

`Discard` elimina el Value superior del Operand Window.

Su responsabilidad principal es cerrar correctamente un `Operation Statement` cuyo resultado normal no se utiliza.

Ejemplo conceptual:

```text
CALL / CALL_EXTERNAL
DISCARD
```

Después de un statement correctamente lowered no quedan temporaries semánticamente abandonados en el Operand Window.

## 9. Return

```rust
Instruction::Return
```

Toda `CompiledFunction` de Evo-Script v0 retorna exactamente un Value normal cuando la ejecución tiene éxito.

Contrato conceptual:

```text
before
... result

Return

→ result transferred to caller
```

Para la `entry_point`, el Value se convierte en el resultado normal exterior de la ejecución.

La mecánica física exacta para transferir/reutilizar storage entre caller/callee pertenece a VM Execution Data.

No existen:

```text
ReturnVoid
multiple return values
exception return value
Result wrapper
```

Un `EvaluationError` detiene la evaluación antes de completar `Return` y se propaga por la frontera de ejecución definida por Evo-Script.

## 10. `when` uses the same branch infrastructure

`when` no introduce:

```text
Instruction::When
Match opcode
Pattern opcode
```

Su control flow utilizará `Jump` / `JumpIfFalse` junto con las futuras instructions de enum discriminant/payload inspection.

Por tanto:

```text
branch infrastructure for `when` ✅ CLOSED
specific enum inspection          PENDING Composite Layout / Enum Mechanics
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

Control flow del Compiled Program deriva exclusivamente de la semántica reusable del `.efn`.

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
when enum inspection                       PENDING Composite Layout
InstructionPointer separation              ✅ CLOSED
```
