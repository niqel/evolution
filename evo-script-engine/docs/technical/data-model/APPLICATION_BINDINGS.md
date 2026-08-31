# Evo-Script Engine — ApplicationBindings

Status: CLOSED

Este documento cierra el modelo v0 de `ApplicationBindings` utilizado por `VmExecution` para resolver capabilities externas explícitamente suministradas por la composición de la aplicación.

La autoridad deriva de:

- `evo-script/EFN_HOST_BOUNDARY_v0.1.md`;
- `docs/functional/CAPABILITIES.md`;
- `docs/functional/DATA_DICTIONARY.md`;
- `docs/functional/use-cases/UC-002-execute-compiled.md`;
- `SEMANTIC_PROGRAM_STRUCTURE.md`;
- `COMPILED_CORE_CALL_INSTRUCTIONS.md`;
- `VM_EXECUTION_DATA.md`;
- `ENGINEERING_PRINCIPLES.md`.

Este bloque define la composición y lookup de bindings. La firma ABI exacta de `ExternalCapability` se cierra en el bloque posterior.

## AB-001 — Explicit application capability composition

Status: CLOSED

`ApplicationBindings` representa la composición explícita de External Capabilities suministrada por la aplicación para una ejecución Evo-Script.

No es estado de Host interactivo y no se descubre ambientalmente.

```text
Application composition
        ↓ supplies
ApplicationBindings
        ↓ borrowed by
VmExecution
```

## AB-002 — Application-oriented and reusable

Status: CLOSED

`ApplicationBindings` pertenece a la composición de la aplicación y puede reutilizarse para ejecutar distintos `CompiledProgram` compatibles.

No está indexado por `ExternalSymbolId`, porque `ExternalSymbolId` es una identity local a un `CompiledProgram` concreto.

```text
CompiledProgram A: ExternalSymbolId(0) !=
CompiledProgram B: ExternalSymbolId(0)
```

Por tanto una misma instancia de `ApplicationBindings` no depende del orden interno de `external_symbols` de un programa específico.

## AB-003 — SignatureSymbol is the lookup key

Status: CLOSED

La identity contractual utilizada para lookup es:

```rust
struct SignatureSymbol {
    module: String,
    name: String,
}
```

`SignatureSymbol` representa la identity canónica `module::signature` que sobrevive desde Semantic Program hacia `ExternalSymbol` compilado.

Separación:

```text
ExternalSymbolId
    = program-local compiled identity

SignatureSymbol
    = cross-boundary contractual identity

Provider identity
    = not part of Engine binding lookup
```

## AB-004 — Exact base representation

Status: CLOSED

Representación base v0:

```rust
struct ApplicationBindings {
    capabilities: HashMap<SignatureSymbol, ExternalCapability>,
}
```

`ExternalCapability` es una única function-pointer identity compatible con el ABI uniforme del Engine. Su firma exacta permanece pendiente del siguiente bloque.

Consecuencia técnica: `SignatureSymbol` debe soportar las propiedades de key requeridas por `HashMap`; esto no cambia su semántica contractual.

## AB-005 — At most one capability per SignatureSymbol

Status: CLOSED

Dentro de una instancia de `ApplicationBindings`, cada `SignatureSymbol` resuelve a lo sumo una `ExternalCapability`.

```text
SignatureSymbol
    → 0..1 ExternalCapability
```

No existe selección runtime entre múltiples Providers para el mismo símbolo dentro de una ejecución.

Una composición diferente puede suministrar otra capability en otra instancia de `ApplicationBindings`.

## AB-006 — Borrowed and immutable during invocation

Status: CLOSED

`VmExecution` referencia exactamente un `ApplicationBindings` que permanece inmutable durante toda la invocation.

```text
compose bindings
    ↓
create VmExecution borrowing bindings
    ↓
execute
    ↓
VmExecution ends
```

No existen durante `.efn` execution:

```text
bind
unbind
switch provider
activate capability
Current Provider
```

## AB-007 — Superset bindings are allowed

Status: CLOSED

`ApplicationBindings` puede contener capabilities que el `CompiledProgram` actual no requiere o no alcanza.

```text
ApplicationBindings capabilities
    may be a strict superset of
CompiledProgram.external_symbols
```

Capabilities extra no producen failure.

## AB-008 — Lazy resolution at CallExternal

Status: CLOSED

La resolución ocurre cuando la VM alcanza:

```rust
CallExternal(ExternalSymbolId)
```

Flujo canónico:

```text
ExternalSymbolId
    ↓
CompiledProgram.external_symbols[id]
    ↓
ExternalSymbol.symbol : SignatureSymbol
    ↓
ApplicationBindings.capabilities lookup
    ├── found   → invoke ExternalCapability
    └── missing → execution Failure
```

Una capability ausente no causa failure durante Compile ni por el simple hecho de crear `VmExecution`. Falla únicamente cuando el `ExternalSymbol` correspondiente es realmente alcanzado.

No se introduce un `ResolvedExternalBindings` cache en v0 sin evidencia de profiling.

## AB-009 — No provider/session/service-locator state

Status: CLOSED

`ApplicationBindings` no contiene ni realiza lookup mediante:

```text
Provider identity
Current Provider
Active Scope
Host Session State
reflection
Service Locator
global registry
ExternalSymbolId
runtime aliases
runtime semantic type lookup
```

El Engine conoce la capability uniforme suministrada por la aplicación, no el Provider físico que puede existir detrás de su adapter/composition.

## Exact Closed Shape

```rust
type ExternalCapability = fn(/* exact uniform Engine ABI — NEXT */);

struct ApplicationBindings {
    capabilities: HashMap<SignatureSymbol, ExternalCapability>,
}
```

Conceptualmente:

```text
CallExternal(ExternalSymbolId)
        ↓
ExternalSymbol
        ↓ SignatureSymbol
ApplicationBindings
        ↓
ExternalCapability
        ↓
application adapter/composition
        ↓
Agent / Resolver / Contract / Provider
```

El último tramo pertenece a la aplicación y sus crates. `evo-script-engine` no conoce Provider identity.

## Explicitly Not Introduced

```text
ApplicationBinding wrapper per entry
ExternalSymbolId-keyed application table
program-specific resolved binding table
Provider object
Provider identity
Current Provider
Service Locator
reflection
binding mutation during execution
preflight requirement that every unused ExternalSymbol be bound
```

## Closure

```text
AB-001 explicit application capability composition       ✅ CLOSED
AB-002 application-oriented / reusable                    ✅ CLOSED
AB-003 SignatureSymbol lookup key                         ✅ CLOSED
AB-004 HashMap<SignatureSymbol, ExternalCapability>       ✅ CLOSED
AB-005 at most one capability per symbol                  ✅ CLOSED
AB-006 immutable borrowed bindings per invocation         ✅ CLOSED
AB-007 superset bindings allowed                          ✅ CLOSED
AB-008 lazy resolution at CallExternal                    ✅ CLOSED
AB-009 no provider/session/service-locator state          ✅ CLOSED

ApplicationBindings exact model                           ✅ CLOSED
ExternalCapability uniform ABI                            ← NEXT
external argument/result materialization                  PENDING
VmExecution exact Rust root                               PENDING
VM Execution exact inventory                              PENDING
```