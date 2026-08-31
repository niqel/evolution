# Evo-Script Engine — ExternalCapabilityFailure

Status: CLOSED

Este documento cierra la representación técnica exacta de `ExternalCapabilityFailure` para `evo-script-engine` v0.

`ExternalCapabilityFailure` es el único failure que una `ExternalCapability` puede devolver directamente al Engine. Expresa una failure normalizada de la capability externa sin introducir errores específicos de Provider, ubicación del script, estado VM o presentación humana dentro del ABI.

## Canonical shape

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}
```

Inventario exacto:

```text
ExternalCapabilityFailure = 1 technical identity
fields                    = 1
```

## ECF-001 — Exactly one owned struct with one field

Status: CLOSED

`ExternalCapabilityFailure` es exactamente una estructura owned con un único field:

```rust
struct ExternalCapabilityFailure {
    code: Box<str>,
}
```

No se introduce una jerarquía de errors externos ni un enum global de dominios/providers.

## ECF-002 — code is a stable symbolic identifier

Status: CLOSED

`code` representa una identidad simbólica estable de failure producida por la capability.

Debe ser:

```text
non-empty
machine-readable
stable dentro del contrato de la capability
```

No es un mensaje humano preformateado.

Ejemplos conceptuales:

```text
not_found
permission_denied
unavailable
conflict
invalid_resource
```

## ECF-003 — Canonical code form is lowercase snake_case

Status: CLOSED

La forma canónica v0 es:

```text
[a-z][a-z0-9_]*
```

Esto evita que `code` se convierta en texto de presentación libre y mantiene una representación interoperable simple.

## ECF-004 — External codes are scoped by SignatureSymbol context

Status: CLOSED

`evo-script-engine` no posee un catálogo universal de external failure codes.

El significado contractual completo aparece al contextualizar:

```text
SignatureSymbol + ExternalCapabilityFailure.code
```

Ejemplo:

```text
fs::read + not_found

db::find + not_found
```

El mismo code puede existir en distintas Signatures sin obligar al Engine a unificar dominios externos.

`SignatureSymbol` no se duplica dentro de `ExternalCapabilityFailure`; el Engine ya conoce la Signature ejecutada a través del `ExternalSymbol` de `CallExternal`.

## ECF-005 — Application adapter normalizes Provider/vendor failures

Status: CLOSED

Una implementation/application adapter traduce cualquier error físico o específico del Provider a `ExternalCapabilityFailure` antes de cruzar el ABI del Engine.

```text
Provider/vendor-specific error
        ↓
application adapter
        ↓ normalize
ExternalCapabilityFailure { code }
        ↓
evo-script-engine
```

No cruzan directamente:

```text
std::io::Error
OS errno
HTTP client error objects
database vendor errors
Provider-specific enums
Box<dyn Error>
dyn Error
```

El Engine depende del failure normalizado, no del proveedor físico.

## ECF-006 — No unrelated context or presentation payload

Status: CLOSED

`ExternalCapabilityFailure` no contiene en v0:

```text
SourceSpan
SignatureSymbol
Provider identity
FunctionId
InstructionPointer
CompiledProgram
SourceMap
human message
retry policy
arbitrary details payload
OwnedValue details
```

La ubicación del script pertenece al Engine y se materializa posteriormente al construir `ExecutionFailure` conforme a `DIAGNOSTIC_PROVENANCE.md`.

Un message humano pertenece a una capa de presentation.

Un payload adicional de detalles requerirá una futura User Story explícita antes de reabrir esta frontera.

## ECF-007 — Engine-owned external failures stay outside

Status: CLOSED

Las siguientes condiciones nunca se representan como `ExternalCapabilityFailure`:

```text
missing ApplicationBindings entry
successful external result does not match ExternalSymbol.result_shape
```

Razón:

```text
Missing Binding
    → capability was never invoked

Result Contract Mismatch
    → capability returned Success; Engine rejected the returned contract shape
```

Ambas pertenecen a `ExecutionFailure` del Engine.

## ECF-008 — Exact ExternalCapability ABI is complete

Status: CLOSED

La firma Rust arquitectónica exacta queda cerrada como:

```rust
type ExternalCapability =
    for<'value> fn(
        &'value [Value<'value>],
    ) -> Result<
        OwnedValue,
        ExternalCapabilityFailure,
    >;
```

La semántica de argumentos/result, ownership, stack commit y `InstructionPointer` permanece cerrada por `EXTERNAL_CAPABILITY_ABI.md`.

## Contextualization into ExecutionFailure

La capability produce solamente:

```text
ExternalCapabilityFailure { code }
```

El Engine conserva el contexto de ejecución:

```text
CallExternal(ExternalSymbolId)
        ↓
ExternalSymbol
├── SignatureSymbol
└── expected result shape
```

Y, si la capability falla:

```text
ExternalCapabilityFailure
+
SignatureSymbol known by Engine
+
CallExternal SourceSpan resolved by Engine
        ↓
ExecutionFailure
```

`ExternalCapabilityFailure` permanece Consumer-neutral y no necesita conocer ese contexto.

## Explicitly not introduced

```text
universal ExternalError enum
ProviderError in Engine ABI
Box<dyn Error>
dyn Error
std::io::Error across boundary
vendor error objects
SourceSpan inside ExternalCapabilityFailure
SignatureSymbol duplicated in failure
human message
retry metadata
arbitrary details payload
missing binding as capability failure
result contract mismatch as capability failure
```

## Closure

```text
ECF-001 exactly one owned struct / one field                    ✅ CLOSED
ECF-002 code = stable symbolic failure identifier               ✅ CLOSED
ECF-003 lowercase snake_case code form                           ✅ CLOSED
ECF-004 codes scoped by SignatureSymbol context                  ✅ CLOSED
ECF-005 adapter normalizes Provider/vendor failures              ✅ CLOSED
ECF-006 no source/provider/presentation/details context          ✅ CLOSED
ECF-007 Engine-owned external failures stay outside              ✅ CLOSED
ECF-008 exact ExternalCapability ABI complete                    ✅ CLOSED

ExternalCapabilityFailure                                       ✅ CLOSED — 1 identity
ExternalCapability exact Rust ABI                               ✅ CLOSED

ExecutionFailure exact family                                   ← NEXT
Outcome exact inventory                                         PENDING
```