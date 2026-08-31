# Evo-Script Engine — Semantic Program / CompilationCatalog Relation

Status: CLOSED — CORRECTIVE RELATION

Este documento sincroniza `Semantic Program Data` con la dependencia técnica `CompilationCatalog` definida en [`COMPILATION_DEPENDENCY_MODEL.md`](./COMPILATION_DEPENDENCY_MODEL.md).

La corrección no modifica la forma cerrada de `SemanticProgram` ni su inventario exacto de 33 identities propias.

## Canonical relation

```text
AST
+
&CompilationCatalog
        ↓
Semantic Analyzer
        ↓
SemanticProgram
```

`CompilationCatalog` participa únicamente durante semantic resolution de imports y sus cierres transitivos de shared Types / Signatures.

## SCR-001 — Catalog is semantic-analysis input, not SemanticProgram state

Status: CLOSED

`SemanticProgram` no conserva una referencia al catálogo.

```text
Semantic Analyzer
    borrows CompilationCatalog

SemanticProgram
    owns compilation-local resolved meaning
```

No se agrega a `SemanticProgram`:

```text
catalog: &CompilationCatalog
catalog_id
module registry
import resolver
```

## SCR-002 — Imported shared Type resolution

Status: CLOSED

Una referencia fuente importada se resuelve así:

```text
AST QualifiedName / local alias
        ↓ import environment
canonical TypeSymbol
        ↓ CompilationCatalog.types
CatalogType
        ↓ semantic lowering
TypeId + SemanticType
```

El alias local se consume durante resolution y no modifica `TypeSymbol`.

`TypeId` continúa siendo identity owner-index local al `SemanticProgram` resultante.

## SCR-003 — Imported Signature resolution

Status: CLOSED

Una Signature importada se resuelve así:

```text
AST QualifiedName / local alias
        ↓ import environment
canonical SignatureSymbol
        ↓ CompilationCatalog.signatures
CatalogSignature
        ↓ semantic lowering
SignatureId + SemanticSignature
```

`SemanticSignature.symbol` conserva el `SignatureSymbol` canónico porque esa identity contractual debe sobrevivir hacia Compiled Program.

## SCR-004 — Transitive contract closure is materialized on demand

Status: CLOSED

Semantic Analyzer materializa únicamente los shared Types y Signatures transitivamente requeridos por el Source Text compilado.

El catálogo completo no se copia a `SemanticProgram`.

Un cache temporal de lowering como:

```text
TypeSymbol      → TypeId
SignatureSymbol → SignatureId
```

puede existir dentro de Compilation Working State para compartir identities y evitar materialización duplicada.

Ese cache no sobrevive al `SemanticProgram`.

## SCR-005 — Semantic success still guarantees full resolution

Status: CLOSED

Después de semantic success:

```text
all TypeId references resolve
all FunctionId references resolve
all SignatureId references resolve
all BindingId / FieldId / VariantId references resolve
all SemanticCall targets and argument types are valid
```

Bytecode Compiler recibe significado completamente resuelto y no necesita `CompilationCatalog`.

## SCR-006 — Exact Semantic Program inventory remains 33

Status: CLOSED

Las 8 identities de Compilation Dependency Data pertenecen a su propia familia técnica y no se cuentan como identities de Semantic Program.

```text
Compilation Dependency Data = 8 own identities
Semantic Program Data       = 33 own identities
```

No se agrega ninguna nueva identity persistente a la forma de `SemanticProgram`.

## SCR-007 — Engine semantic failures vs catalog-construction failures

Status: CLOSED

Semantic Analyzer puede producir failure cuando el Source Text, evaluado contra un catálogo válido, no puede resolver o satisfacer un contrato.

Ejemplos conceptuales:

```text
import requests a canonical symbol absent from catalog
local alias/name collision
function/signature call ambiguity
arity mismatch
type mismatch
signature satisfaction mismatch
shared Type misuse
```

En cambio no produce como `SemanticFailure` normal del Source Text:

```text
filesystem path invalid
artifact file missing
invalid module boundary
duplicate physical module
malformed catalog internal reference
```

Los primeros pertenecen al constructor externo de `CompilationCatalog`; el último es una integración/invariant violation.

## Closure

```text
catalog borrowed only by Semantic Analyzer                  ✅ CLOSED
TypeSymbol → TypeId lowering                                ✅ CLOSED
SignatureSymbol → SignatureId lowering                     ✅ CLOSED
transitive closure materialized on demand                  ✅ CLOSED
no catalog relation in Bytecode Compiler                   ✅ CLOSED
SemanticProgram exact inventory remains 33                 ✅ CLOSED
failure ownership boundary synchronized                    ✅ CLOSED

NEXT
    SemanticFailure exact family
```
