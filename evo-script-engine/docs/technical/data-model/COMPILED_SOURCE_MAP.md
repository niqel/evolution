# Evo-Script Engine — Compiled Source Map

Status: CLOSED

Este documento cierra `SourceMap` para `Compiled Program / Bytecode Data` en `evo-script-engine` v0.

La autoridad deriva de:

- `LEXICAL_DATA.md`;
- `SEMANTIC_EXPRESSIONS.md`;
- `COMPILED_PROGRAM_DATA.md`;
- `COMPILED_CONTROL_FLOW.md`;
- `COMPILED_COMPOSITE_INSTRUCTIONS.md`.

## 1. Responsibility

Regla canónica:

> `SourceMap` relaciona cada Instruction persistente de un `CompiledProgram` con el `SourceSpan` de la construcción semántica responsable de producirla, sin conservar el Source Text ni convertirse en sistema de presentación diagnóstica.

Resolución conceptual:

```text
(FunctionId, InstructionIndex)
        ↓
     SourceMap
        ↓
    SourceSpan
```

## 2. Representation

Representación cerrada:

```rust
struct SourceMap {
    functions: Vec<Vec<SourceSpan>>,
}
```

La estructura es posicional y densa.

```text
SourceMap.functions[f][i]
        ↕
CompiledProgram.functions[f].instructions[i]
```

No se introduce `SourceMapEntry` con `FunctionId` e `InstructionIndex` duplicados porque esas coordenadas ya están expresadas por el owner/index structure del `CompiledProgram`.

## 3. Cardinality invariants

Para todo `CompiledProgram` válido:

```text
source_map.functions.len()
    == compiled_program.functions.len()
```

Para cada función `f`:

```text
source_map.functions[f].len()
    == compiled_program.functions[f].instructions.len()
```

Por tanto:

```text
(FunctionId(f), InstructionIndex(i))
    → exactly one SourceSpan
```

Todo Instruction persistente posee exactamente un source anchor.

No se utiliza:

```text
Option<SourceSpan>
sparse SourceMap
range search
```

## 4. SourceSpan reuse

Se reutiliza directamente la identidad ya cerrada:

```rust
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}
```

Convención:

```text
[start, end)
```

`SourceSpan` continúa expresando byte offsets relativos al mismo Source Text que originó el `CompiledProgram`.

No se introducen:

```text
CompiledSourceSpan
RuntimeSourceSpan
InstructionSpan
```

## 5. One source coordinate space per CompiledProgram v0

Regla v0:

> Todos los `SourceSpan` de un `CompiledProgram` pertenecen a un único source coordinate space: el Source Text que produjo ese programa compilado.

Por tanto en v0 no se requieren:

```text
SourceId
SourcePath
SourceName
SourceLocation
```

Un path, editor-buffer identity, in-memory label o cualquier identidad externa del source pertenece al Host/Consumer mientras exista un único coordinate space ejecutable por `CompiledProgram`.

Si una versión futura permite que un único `CompiledProgram` contenga Instructions originadas por múltiples Source Text, esta decisión debe reabrirse y puede evolucionar naturalmente a:

```rust
struct SourceLocation {
    source: SourceId,
    span: SourceSpan,
}
```

sin modificar la semántica de `Instruction`, `CompiledFunction` ni VM execution.

## 6. SourceMap encapsulation boundary

Regla arquitectónica cerrada:

> La nested storage shape de `SourceMap` pertenece exclusivamente al subsistema de Source Mapping. Los consumidores no deben acoplarse directamente a `Vec<Vec<SourceSpan>>` fuera de su frontera de resolución.

Conceptualmente, los consumidores solicitan una ubicación mediante:

```text
FunctionId + InstructionIndex
        ↓ Source Mapping boundary
SourceSpan
```

La firma técnica concreta de esa resolución se define posteriormente junto con Participants / VM / Diagnostics.

Esta encapsulación prepara una futura migración a `SourceLocation { source, span }` sin propagar el cambio por todo el Engine.

## 7. Most-specific responsible span policy

Regla:

> Una Instruction recibe el `SourceSpan` de la `SemanticExpression` más específica cuya compilación produjo directamente esa Instruction.

Ejemplo conceptual:

```text
a + (b * c)

Load a       → span(a)
Load b       → span(b)
Load c       → span(c)
Multiply     → span(b * c)
Add          → span(a + (b * c))
```

## 8. Compiler-generated supporting Instructions

Una Instruction técnica puede no corresponder a una expresión textual individual.

En ese caso utiliza el span de la construcción semántica responsable más cercana.

Ejemplos:

```text
JumpIfFalse generado por a && b
    → span(a && b)

LoadConstant(false) sintético de short-circuit
    → span(a && b)

TestVariant / extraction machinery de when
    → span(when) cuando no existe un span semántico más específico

StoreLocal generado por let binding
    → span(initializer expression)

Return final generado por SemanticFunctionBody.result
    → span(result expression)
```

No se fabrican spans artificiales y no se requiere volver al AST.

## 9. `when` source mapping

`SemanticWhenBranch` no conserva un `SourceSpan` propio en v0.

Por tanto las Instructions técnicas de branch matching/extraction pueden anclarse a la `SemanticExpression` exterior del `when`, mientras las Instructions emitidas para cada branch result usan los spans específicos de sus propias `SemanticExpression`.

Esta granularidad es suficiente para v0 y no justifica reabrir Semantic Program únicamente para agregar pattern spans.

## 10. Instruction occurrence owns execution provenance

La ubicación pertenece a la occurrence ejecutable, no a los objetos compartidos que una Instruction referencia.

No se introducen mappings persistentes:

```text
ConstantId       → SourceSpan
ExternalSymbolId → SourceSpan
FunctionId       → declaration SourceSpan
```

Ejemplo:

```text
ConstantId(5) = 100
```

puede ser usado por múltiples `LoadConstant(ConstantId(5))`; cada occurrence posee su propio entry dentro de `SourceMap`.

## 11. Source Text lifetime boundary

`CompiledProgram` puede sobrevivir al Source Text porque `SourceMap` no contiene borrowed references.

Esto significa:

```text
CompiledProgram
    does not borrow Source Text
```

No significa que deba almacenar todo lo necesario para reconstruir por sí mismo una presentación humana completa del source.

Si Host conserva o vuelve a resolver el Source Text:

```text
SourceSpan
    → line / column / snippet / highlight
```

Si no lo posee, `SourceSpan` continúa siendo provenance técnica mediante byte offsets.

La forma final de presentation pertenece a `Outcome / Diagnostic Data`.

## 12. No line/column duplication

No se almacenan:

```text
line
column
end_line
end_column
```

`LEXICAL_DATA.md` ya establece que line/column son datos derivables a partir del Source Text y `SourceSpan`.

Esto evita duplicación y mantiene fuera de `SourceMap` decisiones de display como Unicode columns, graphemes o tab widths.

## 13. SourceMap remains outside CompiledFunction

Se conserva la raíz ya cerrada:

```rust
struct CompiledProgram {
    functions: Vec<CompiledFunction>,
    entry_point: FunctionId,
    constants: Vec<Constant>,
    external_symbols: Vec<ExternalSymbol>,
    source_map: SourceMap,
}
```

`CompiledFunction.instructions` representa hot executable mechanism.

`CompiledProgram.source_map` representa diagnostic/provenance metadata y no necesita consultarse durante ejecución normal de cada Instruction.

## 14. Persistence != portable serialization

Regla de alcance:

> Que `CompiledProgram` sea un producto persistente respecto del Compilation Working State no significa que Evo-Script v0 ya haya definido un formato binario portable, estable o cross-architecture.

El uso actual de `usize` en `SourceSpan`, `InstructionIndex`, `FieldIndex` y otras identities internas no constituye un ABI o serialized bytecode format.

Si posteriormente se define un formato portable de bytecode, su encoding estable será una responsabilidad distinta y puede transformar estas representaciones internas.

No se reabre `SourceSpan` por una necesidad de serialización todavía no definida.

## 15. Runtime failure location consequence

Una futura estructura de execution failure puede conservar una coordenada técnica mínima:

```text
FunctionId
InstructionIndex
```

y resolver después:

```text
runtime execution location
        ↓
SourceMap
        ↓
SourceSpan
```

La forma concreta del Failure/Outcome permanece para `VM Execution Data` y `Outcome / Diagnostic Data`.

## 16. Explicit exclusions

```text
SourceMapEntry with duplicated coordinates
Option<SourceSpan> per Instruction
sparse SourceMap
SourceId v0
SourcePath v0
SourceName v0
Source Text ownership / borrowing
line-column duplication
ConstantId source map
ExternalSymbolId source map
runtime TypeId
portable bytecode serialization format
```

## 17. Closure

```text
SourceMap responsibility                         ✅ CLOSED
SourceMap dense positional representation        ✅ CLOSED
FunctionId + InstructionIndex → SourceSpan       ✅ CLOSED
one SourceSpan per persistent Instruction        ✅ CLOSED
cardinality invariants                           ✅ CLOSED
SourceSpan reuse                                 ✅ CLOSED
one source coordinate space per program v0       ✅ CLOSED
SourceId / SourcePath / SourceName                ❌ NOT NEEDED v0
SourceMap encapsulation boundary                 ✅ CLOSED
future SourceLocation migration seam             ✅ CLOSED
most-specific responsible span policy            ✅ CLOSED
compiler-generated Instruction span fallback     ✅ CLOSED
when source-map fallback                         ✅ CLOSED
Instruction occurrence provenance                ✅ CLOSED
no Source Text borrowing                         ✅ CLOSED
no line/column duplication                       ✅ CLOSED
SourceMap separated from hot instructions        ✅ CLOSED
persistent != portable serialization             ✅ CLOSED

SourceMap                                        ✅ CLOSED
Compiled Program exact inventory                 ← NEXT
```