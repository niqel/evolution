# Evo-Script Engine — ExternalCapability ABI

Status: CLOSED — ABI SEMANTICS; EXACT `evo-values` TYPES PENDING

Este documento cierra las reglas v0 de la frontera uniforme entre `CallExternal` y una `ExternalCapability` suministrada mediante `ApplicationBindings`.

La autoridad deriva de:

- `APPLICATION_BINDINGS.md`;
- `RUNTIME_VALUE_MODEL.md`;
- `RUNTIME_VALUE_REPRESENTATION.md`;
- `SHARED_VALUE_STORAGE.md`;
- `INSTRUCTION_POINTER_STEPPING.md`;
- `TECHNICAL_DESIGN.md`, especialmente TD-008;
- `ENGINEERING_PRINCIPLES.md`;
- el modelo actual de `evo-values`.

Este bloque cierra la responsabilidad, dirección de borrowing/ownership y commit semantics del ABI. La firma Rust textual final permanece pendiente únicamente de completar en `evo-values` las representaciones borrowed/owned necesarias para todo el lenguaje.

## EC-001 — One uniform function-pointer ABI

Status: CLOSED

`ExternalCapability` es una única identity técnica de function pointer utilizada por `ApplicationBindings` para todas las capabilities externas ejecutables desde bytecode.

Conceptualmente:

```rust
type ExternalCapability = fn(/* uniform Engine ABI */);
```

Las signatures Evo-Script pueden ser heterogéneas semánticamente, pero `ApplicationBindings` necesita un único tipo Rust almacenable en:

```rust
HashMap<SignatureSymbol, ExternalCapability>
```

No se introduce `trait`, `dyn Fn`, Provider object ni jerarquía de interfaces por defecto.

## EC-002 — Arguments cross as borrowed interchange Values

Status: CLOSED

Los argumentos de una `ExternalCapability` cruzan la frontera como views prestadas de interoperabilidad.

`RuntimeValue` no cruza la frontera pública/application porque sus handles son relativos a una `VmExecution`:

```text
RuntimeValue::String(StringBackingRef)
RuntimeValue::Struct(StructBackingId)
RuntimeValue::Enum(EnumBackingId)
```

La VM observa/materializa temporalmente cada argumento como una representación borrowed equivalente a `Value<'a>`.

```text
RuntimeValue
    ↓ observe/materialize view
Value<'call>
    ↓
ExternalCapability
```

La capability no recibe `CompiledProgram`, `ExecutionBackingStore` ni handles internos para poder interpretar sus argumentos.

## EC-003 — Exact argument source is the top N operands

Status: CLOSED

Para:

```rust
CallExternal(ExternalSymbolId)
```

la cantidad de argumentos físicos es:

```text
N = CompiledProgram.external_symbols[id].parameter_count
```

Los argumentos son exactamente las `N` cells superiores del Operand Window activo, preservando orden posicional.

```text
[..., arg0, arg1, ..., argN-1]
      └───────────────┘
      arguments for CallExternal
```

No se crea un Parameter region ni `CallFrame` externo.

## EC-004 — Argument RuntimeValues remain in SharedValueStorage during invocation

Status: CLOSED

Las cells de argumentos permanecen en `SharedValueStorage` mientras la capability externa se ejecuta.

La VM puede prestar views derivadas de esos `RuntimeValue` sin hacer commit anticipado del stack effect `N → 1`.

```text
before/during external call
[..., arg0, arg1, ..., argN-1]
```

No se requiere remover los argumentos antes de conocer el outcome externo.

## EC-005 — Successful external result crosses as owned interchange Value

Status: CLOSED

Un resultado exitoso de `CallExternal` debe sobrevivir a la invocation externa porque se convierte en un operand normal de la VM.

Por tanto cruza hacia el Engine mediante una representación owned de interoperabilidad, conceptualmente `OwnedValue`.

```text
ExternalCapability
    ↓ Success
OwnedValue
    ↓ transfer/materialize
VmExecution-owned runtime representation
```

Esta ownership no es artificial: representa datos que deben sobrevivir al owner/materializer externo.

## EC-006 — No Requester for the normal one-result CallExternal boundary

Status: CLOSED

El `CallExternal` normal de v0 produce exactamente un resultado semántico que debe sobrevivir a la llamada.

Por tanto no se introduce un Requester únicamente para prestar un resultado que inmediatamente tendría que ser copiado/materializado dentro de `VmExecution`.

Regla aplicada:

```text
borrow mientras alcance
ownership cuando deba sobrevivir
```

Los Requesters siguen siendo válidos en otras fronteras del ecosistema cuando el consumidor solo necesita observar una vista durante el lifetime del materializador. Esta decisión es específica del one-result ABI de `CallExternal`.

## EC-007 — Success materializes OwnedValue before stack replacement commit

Status: CLOSED

En success, la VM convierte/transfiere el `OwnedValue` recibido a su representación runtime.

```text
OwnedValue
    ↓
RuntimeValue
    +
ExecutionBackingStore insertion when required
```

Solo después de obtener un `RuntimeValue` válido se aplica el stack effect:

```text
remove N argument cells
push one result cell
```

El resultado runtime puede usar inline scalar data o un typed backing ID según las reglas ya cerradas.

## EC-008 — External failure does not commit stack replacement

Status: CLOSED

Si la capability externa concluye con failure:

```text
argument cells remain physically present
IP remains on CallExternal
execution produces Failure
```

No se aplica el reemplazo `N → 1` y no se avanza el `InstructionPointer`.

Esto preserva la regla commit-after-success de `InstructionPointer`.

No implica rollback de side effects externos ni resumability de la ejecución.

## EC-009 — `evo-values` must provide complete borrowed and owned interchange representations

Status: CLOSED

La firma Rust textual final de `ExternalCapability` depende de que `evo-values` pueda representar todo Value que el lenguaje cerrado permite cruzar por esta frontera.

El `Value<'a>` histórico actual:

```rust
pub enum Value<'value> {
    Text(&'value str),
    Unsigned(u64),
    Signed(i64),
    Boolean(bool),
}
```

no cubre todavía todas las familias runtime cerradas por `RuntimeValue`.

Por tanto `evo-values` debe definir conceptualmente dos responsabilidades complementarias:

```text
Value<'a>
    = borrowed/interchange view

OwnedValue
    = owned/interchange representation for ownership transfer
```

La definición exacta de ambos tipos es el siguiente bloque y no reabre EC-001..EC-008.

## EC-010 — Plain fn means statically composed behavior in v0

Status: CLOSED

Una `ExternalCapability` expresada como plain Rust `fn` no captura estado de closure o instancia.

La representación v0 cierra por tanto una composición ejecutable estática mediante function pointers.

No se afirma que un `fn` pueda representar por sí solo cualquier Provider stateful.

Si una futura necesidad exige ligar una capability a estado runtime por instancia, deberá reabrirse explícitamente esta frontera y justificar una forma como:

```text
function pointer + explicit state
```

u otra abstracción con responsabilidad real.

No se introduce preventivamente `dyn Fn`, trait object, closure box o Provider object.

## Canonical CallExternal flow

```text
CallExternal(ExternalSymbolId)
        ↓
ExternalSymbol
        ↓ SignatureSymbol
ApplicationBindings lookup
        ↓
ExternalCapability
        ↓
borrow top N RuntimeValues
        ↓
Value<'call> argument views
        ↓
invoke external function pointer
        │
        ├── Failure
        │      ├── no N→1 stack commit
        │      └── IP remains on CallExternal
        │
        └── Success(OwnedValue)
               ↓
        materialize/transfer into VmExecution
               ↓
        RuntimeValue
               ↓
        replace N arguments with 1 result
               ↓
        ip += 1
```

## Architectural signature shape

La forma arquitectónica cerrada es:

```rust
type ExternalCapability =
    for<'value> fn(
        /* borrowed ordered argument views */
    ) -> Result<
        /* one owned interchange value */,
        /* external capability failure */,
    >;
```

Los tipos exactos de arguments/result/failure no se fijan en este documento antes de cerrar el `evo-values` interchange model y Outcome/Diagnostic Data.

## Explicitly Not Introduced

```text
RuntimeValue across application boundary
ExecutionBackingStore handle exposed externally
CallFrame for external calls
Requester for normal one-result CallExternal
dyn Fn
trait object Provider
capturing closure ABI
implicit Provider state
pre-pop of arguments before external success
rollback guarantee
resume-after-failure guarantee
```

## Closure

```text
EC-001 one uniform fn-pointer ABI                         ✅ CLOSED
EC-002 borrowed interchange arguments                     ✅ CLOSED
EC-003 top-N operand argument source                      ✅ CLOSED
EC-004 arguments remain during invocation                 ✅ CLOSED
EC-005 owned interchange success result                   ✅ CLOSED
EC-006 no Requester for normal one-result external call   ✅ CLOSED
EC-007 success materializes then commits N→1              ✅ CLOSED
EC-008 failure does not commit stack replacement          ✅ CLOSED
EC-009 complete borrowed + owned evo-values required      ✅ CLOSED
EC-010 plain fn = statically composed behavior v0         ✅ CLOSED

ExternalCapability ABI semantics                          ✅ CLOSED
Exact Rust ABI value types                                PENDING — depends on evo-values

evo-values borrowed / owned interchange model             ← NEXT
external failure exact representation                     PENDING — Outcome / Diagnostic Data
VmExecution exact Rust root                               PENDING
VM Execution exact inventory                              PENDING
```
