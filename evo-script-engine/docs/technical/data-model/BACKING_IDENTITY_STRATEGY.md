# Evo-Script Engine — Backing Identity Strategy

Status: CLOSED

Este documento cierra la estrategia de identity utilizada por `RuntimeValue` para referenciar backing data variable o composite sin introducir persistent self-borrows dentro de `VmExecution`.

La autoridad deriva de:

- `TECHNICAL_DESIGN.md`, especialmente TD-005, TD-007 y TD-008;
- `COMPILED_STORAGE_DATA.md`;
- `VM_EXECUTION_DATA.md`;
- `RUNTIME_VALUE_MODEL.md`.

## BI-001 — Backing identities are typed by backing kind

Status: CLOSED

Las identities de backing de ejecución son tipadas por la clase de dato que identifican.

```rust
struct StringBackingId(usize);
struct DynamicIntegerBackingId(usize);
struct StructBackingId(usize);
struct EnumBackingId(usize);
```

Aunque las cuatro puedan utilizar `usize` como representación v0, no son identities intercambiables.

Regla:

> Distintas responsabilidades de backing conservan distintas identities técnicas aunque compartan representación física.

Esto impide representar por accidente relaciones como `String` apuntando a Struct backing sin requerir runtime type guessing.

## BI-002 — No universal RuntimeBackingId in v0

Status: CLOSED

No se introduce:

```rust
struct RuntimeBackingId(usize);
```

ni una tabla universal que obligue a resolver después qué clase de backing vive en cada posición.

No existe en v0:

```text
RuntimeBackingId
GenericObjectId
RuntimeObjectId
runtime backing type tag lookup
```

El Runtime Value ya expresa qué categoría de backing espera.

## BI-003 — String distinguishes compiled-backed and execution-backed data

Status: CLOSED

String puede tener dos owners legítimos:

```text
CompiledProgram
    └── Constant::String

VmExecution
    └── runtime-produced String backing
```

La referencia cerrada es:

```rust
enum StringBackingRef {
    Compiled(ConstantId),
    Execution(StringBackingId),
}
```

Invariante:

```text
StringBackingRef::Compiled(id)
    → CompiledProgram.constants[id]
    → must be Constant::String
```

`ConstantId` se reutiliza; no se introduce `CompiledStringId` duplicado.

## BI-004 — Dynamic Integer distinguishes compiled-backed and execution-backed data

Status: CLOSED

Dynamic Integer también puede provenir de Constant Pool o producirse durante ejecución.

```rust
enum DynamicIntegerBackingRef {
    Compiled(ConstantId),
    Execution(DynamicIntegerBackingId),
}
```

Invariante:

```text
DynamicIntegerBackingRef::Compiled(id)
    → CompiledProgram.constants[id]
    → must be Constant::Dynamic(DynamicConstant::Integer { ... })
```

No se introduce `CompiledDynamicIntegerId`.

## BI-005 — Struct and Enum use execution-owned typed IDs

Status: CLOSED

En Evo-Script Engine v0, Struct y Enum runtime Values se construyen durante ejecución y no existen como composite constants persistentes dentro del Constant Pool cerrado.

Por tanto utilizan directamente:

```rust
StructBackingId
EnumBackingId
```

Conceptualmente:

```text
RuntimeValue::Struct(StructBackingId)
    → VmExecution-owned Struct backing

RuntimeValue::Enum(EnumBackingId)
    → VmExecution-owned Enum backing
```

No se introduce una discriminación `Compiled | Execution` que v0 no necesita para estas categorías.

## BI-006 — Backing IDs are stable for the full VmExecution lifetime

Status: CLOSED

Todo backing ID válido conserva la misma identity durante la vida completa de la `VmExecution` que lo creó.

Regla v0:

```text
allocate backing id
    → identity remains stable
    → id is not reused during the same VmExecution
    → all execution backing is released when VmExecution ends
```

Esto evita requerir en v0:

```text
generation counters
handle reuse detection
reference counting
GC
per-object liveness tracking
```

La reclamación temprana de backing puede reabrirse únicamente si profiling demuestra una necesidad real.

## BI-007 — Backing identity does not prescribe physical storage strategy

Status: CLOSED

Las identities anteriores expresan relación estable, no allocator/layout físico.

Este bloque NO prescribe:

```text
Vec
Arena
Slab
Box
Rc
Arc
raw pointer
offset
segmented storage
custom allocator
```

Una implementación futura puede cambiar el container físico mientras preserve:

```text
valid typed ID
    → same backing object
    for the full VmExecution lifetime
```

La elección del container pertenece al bloque posterior de backing representation / Shared Value Storage.

## Closed Conceptual Shape

```text
RuntimeValue
│
├── String
│   └── StringBackingRef
│       ├── Compiled(ConstantId)
│       └── Execution(StringBackingId)
│
├── Dynamic Integer
│   └── DynamicIntegerBackingRef
│       ├── Compiled(ConstantId)
│       └── Execution(DynamicIntegerBackingId)
│
├── Struct
│   └── StructBackingId
│
└── Enum
    └── EnumBackingId
```

## Explicitly Not Introduced

```text
RuntimeBackingId
RuntimeObjectId
CompiledStringId
CompiledDynamicIntegerId
runtime reflection
runtime type lookup for backing
Rc / Arc requirement
raw/self references
```

## Closure

```text
BI-001 typed backing identities                         ✅ CLOSED
BI-002 no universal RuntimeBackingId                    ✅ CLOSED
BI-003 String compiled/execution reference              ✅ CLOSED
BI-004 Dynamic Integer compiled/execution reference     ✅ CLOSED
BI-005 Struct / Enum execution-owned typed IDs          ✅ CLOSED
BI-006 IDs stable and non-reused per VmExecution        ✅ CLOSED
BI-007 identity independent of physical container       ✅ CLOSED

Backing Identity Strategy                              ✅ CLOSED

RuntimeValue exact representation                      ← NEXT
Dynamic Value exact representation                     PENDING
String / Dynamic Integer backing representation        PENDING
Struct / Enum backing representation                   PENDING
Shared Value Storage exact representation              PENDING
```
