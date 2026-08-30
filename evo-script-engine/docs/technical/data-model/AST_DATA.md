# Evo-Script Engine — AST Data

Status: AST DATA — IN ANALYSIS

Este documento define el bloque de `AST Data` del Technical Data Model de `evo-script-engine` v0.

La autoridad sintáctica pertenece a `evo-script`. El Parser reconoce la representación textual conforme a esa especificación y produce `AST`; el `Semantic Analyzer` consume ese `AST` para resolver significado.

```text
Token Sequence
    ↓
Parser
    ↓
AST
    ↓
Semantic Analyzer
    ↓
Semantic Program
```

## AD-001 — AST representa estructura sintáctica abstracta completa

Status: CLOSED

`AST` representa exclusivamente la estructura sintáctica abstracta completa de un `Evo-Script Program` sintácticamente válido.

Debe conservar la información necesaria para que el `Semantic Analyzer` pueda resolver posteriormente el significado del programa sin volver a reconstruir la gramática desde Tokens.

El AST conserva, cuando corresponda:

- jerarquía sintáctica;
- orden sintácticamente significativo;
- nombres e identificadores;
- literales en su forma textual necesaria para análisis posterior;
- formas sintácticas reconocidas;
- `Source Span` para identidades y construcciones que puedan originar diagnostics.

El AST no resuelve significado semántico.

Separación canónica:

```text
AST
    = qué estructura escribió el programa

Semantic Program
    = qué significa el programa después del análisis semántico
```

Invariantes:

1. El AST solo existe después de parsing sintácticamente exitoso.
2. El AST puede existir aunque el programa sea semánticamente inválido.
3. La precedencia, asociatividad y agrupación sintáctica ya deben estar representadas en la jerarquía del AST.
4. El Semantic Analyzer no vuelve a interpretar Tokens para reconstruir precedencia o estructura gramatical.
5. Delimitadores como paréntesis, llaves, comas, punto y coma y otros símbolos no requieren nodos propios cuando su significado estructural ya quedó representado por el AST.
6. El AST no es un Concrete Syntax Tree ni una copia decorada de `Token Sequence`.
7. Nombres y literales pueden continuar borrowing desde `Source Text` durante Compile cuando el Technical Data Model concreto lo permita; el AST no necesita sobrevivir al `Source Text`.
8. Un literal sintáctico no se materializa automáticamente como `evo-values::Value`; su tipado y significado pertenecen al Semantic Analyzer.
9. El AST no contiene `FunctionId`, slots, identidades semánticas resueltas, bytecode, Opcodes, direcciones físicas de funciones, Providers ni capacidades concretas.

## AD-002 — AST preserva ocurrencias; no deduplica ni normaliza semántica

Status: CLOSED

El AST conserva todas las ocurrencias sintácticas reconocidas aunque posteriormente resulten duplicadas, incompatibles o inválidas semánticamente.

Ejemplo conceptual:

```text
private fn calculate(...) { ... }
private fn calculate(...) { ... }
```

Debe producir dos declaraciones sintácticas distinguibles en el AST. Corresponde al Semantic Analyzer detectar posteriormente `DuplicateFunctionError`.

La misma regla aplica a otros casos donde la semántica debe comparar múltiples ocurrencias, como nombres de campos, variantes, bindings, imports o declaraciones.

Invariantes:

1. El Parser no elimina duplicados.
2. El Parser no reemplaza una declaración previa por una posterior.
3. El Parser no convierte secuencias sintácticas en mapas semánticos cuya clave pueda destruir información duplicada.
4. La normalización que requiera identidad, resolución de símbolos, tipos o reglas de unicidad pertenece al Semantic Analyzer.
5. El orden se conserva cuando la sintaxis o semántica posterior depende del orden textual, como Function Body Statements, arguments y pipeline stages.

## AD-003 — Parser / Semantic Analyzer boundary

Status: IN ANALYSIS

La siguiente etapa es definir el inventario exacto de identidades sintácticas que deben existir en el AST.

Criterio de inclusión:

> Una identidad pertenece al AST cuando representa una diferencia estructural observable en la gramática que el Semantic Analyzer necesita conocer posteriormente.

Criterio de exclusión:

> Una identidad no pertenece al AST cuando solo expresa resolución, tipo determinado, identidad semántica, binding, slot, dependencia concreta o cualquier significado que requiera conocer otras declaraciones o símbolos.

Inventario preliminar sujeto a análisis:

```text
Program
Imports / Qualified Names
Struct Definitions / Fields
Enum Definitions / Variants
Function Definitions / Parameters / Function Body
Let Binding
Operation Statement
Final Return
Expressions
When Correspondences
Pipeline Stages
```

No se ha cerrado todavía:

- `AST Node Kind` exacto;
- structs/enums Rust concretos;
- `Vec`, `NodeId`, árbol jerárquico u otra representación física;
- cardinalidades Rust concretas;
- derives;
- visibilidad;
- firmas de Parser o Semantic Analyzer.
