# UC-001 — Compile

Status: REVALIDATED — FUNCTIONAL CLOSED

---

## 1. Purpose

Definir funcionalmente la operación `Compile` mediante la cual un Consumer proporciona un `Source Text` completo al `evo-script-engine` para obtener un `Compile Outcome` conforme a la especificación vigente de Evo-Script.

```text
Consumer
    │
    │ Source Text
    ▼
  Compile
    │
    ▼
Compile Outcome
    ├── Success ──► Compiled Program
    └── Failure ──► Failure
```

`Compile` es el nombre semántico canónico del Use Case. Las descripciones funcionales pueden explicar que compila Source Text de Evo-Script, pero el nombre arquitectónico de la operación permanece `Compile`.

---

## 2. Traceability

- **Deriva de**: [`US-001 — Compile Evo-Script Source`](../user-stories/US-001-compile-evo-script-source.md)
- **Utiliza conceptos de**: [`Functional Data Dictionary`](../DATA_DICTIONARY.md)
- **Aplica normativamente**: la especificación vigente de Evo-Script.
- **Corresponde a Public Capability**: `Compile`.

---

## 3. Primary Actor

- **Actor**: `Consumer`
- **Definición**: Rol externo que ya dispone del `Source Text` completo y solicita su compilación. El rol no prescribe una implementación concreta del Consumer.

---

## 4. Actor Goal

Obtener un `Compiled Program` ejecutable cuando el `Source Text` cumple las reglas de Evo-Script, o recibir un `Failure` funcional cuando la compilación no puede concluir exitosamente.

---

## 5. Trigger

El Use Case comienza cuando el Consumer solicita `Compile` proporcionando exactamente un `Source Text` completo.

La frontera funcional comienza cuando el texto ya está disponible. Localizar, abrir o leer físicamente un archivo `.efn` ocurre fuera de `evo-script-engine`.

---

## 6. Preconditions

1. El Consumer dispone de exactamente un `Source Text` para proporcionar a `Compile`.
2. No se exige que el `Source Text` sea válido como precondición; determinar su conformidad con Evo-Script es responsabilidad de `Compile`.
3. No se exige que Providers, bindings o implementaciones concretas de capacidades externas estén disponibles durante compilación.

---

## 7. Functional Inputs

### Source Text

`Compile` consume exactamente un `Source Text` completo.

Invariantes:

- `Source Text != File Path`.
- `Source Text != AST`.
- `Source Text != Token Stream`.
- `Source Text != Compiled Program`.
- `Source Text != Individual Function`.
- `Compile` no recibe `Invocation Values`.

---

## 8. Functional Output

`Compile` produce exactamente un `Compile Outcome` cuando la operación concluye.

```text
Compile Outcome
├── Success
│      └── Compiled Program
│
└── Failure
       └── Failure
```

Esta definición es funcional. No determina todavía si la representación Rust será un `enum`, un `Result`, callbacks, Requesters u otro mecanismo técnico.

---

## 9. Main Success Flow

1. El Consumer proporciona un `Source Text` completo y solicita `Compile`.
2. `evo-script-engine` trata el `Source Text` como candidato a representar un `Evo-Script Program` completo.
3. El Engine aplica las reglas léxicas definidas por Evo-Script.
4. El Engine aplica las reglas sintácticas definidas por Evo-Script.
5. El Engine aplica las reglas semánticas del lenguaje definidas por Evo-Script.
6. El Engine determina que el programa puede compilarse exitosamente.
7. El Engine genera bytecode como representación ejecutable del programa compilado.
8. Cuando el programa contiene referencias a capacidades externas válidas conforme a Evo-Script, el Engine preserva dichas referencias como `External Symbols` sin ligarlas a una implementación concreta.
9. El Engine produce un `Compiled Program` completo y válido.
10. `Compile` concluye con `Compile Outcome::Success`, que transporta el `Compiled Program`.

La mención a `Compile Outcome::Success` es notación conceptual; no prescribe todavía una variante Rust concreta.

---

## 10. Semantic Validation Boundary

`Compile` valida semántica perteneciente al lenguaje, pero no valida la disponibilidad runtime de infraestructura externa.

```text
Compile valida
├── reglas léxicas
├── reglas sintácticas
├── reglas semánticas de Evo-Script
└── validez de referencias externas según el lenguaje

Compile NO exige
├── Provider concreto disponible
├── binding de Application disponible
├── dirección de function pointer
└── capacidad externa materializada en runtime
```

Un `External Symbol` válido puede permanecer sin resolver dentro del `Compiled Program`. Su resolución corresponde a ejecución.

---

## 11. Failure Flow — Compilation Failure

1. Durante el procesamiento del `Source Text`, el Engine determina que no se cumple una regla léxica, sintáctica o semántica perteneciente a Evo-Script.
2. La compilación no produce públicamente un `Compiled Program` parcial o inválido.
3. El Engine genera un `Failure` conforme al Functional Data Dictionary.
4. Cuando el fallo puede asociarse a una ubicación fuente, el `Failure` puede referenciar un `Source Location`.
5. `Compile` concluye con `Compile Outcome::Failure`, que transporta el `Failure`.

La ausencia de un Provider o binding concreto para un `External Symbol` válido no constituye por sí misma un Compilation Failure.

---

## 12. Postconditions

### Success

- Existe un `Compiled Program` completo y funcionalmente válido.
- Su representación ejecutable es bytecode.
- Puede conservar `External Symbols` todavía no resueltos.
- Puede utilizarse posteriormente mediante `Execute Compiled`.
- Puede reutilizarse en múltiples ejecuciones independientes.
- Conserva la información funcional necesaria para diagnósticos de ejecución cuando corresponda.

### Failure

- No se expone públicamente ningún `Compiled Program` parcial.
- El `Compile Outcome` contiene un `Failure`.

---

## 13. Functional Invariants and Atomicity

1. **Canonical Name**: el nombre arquitectónico de la operación es `Compile`.
2. **Public Atomicity**: success expone un `Compiled Program` completo; failure no expone un artefacto parcial.
3. **Bytecode**: la representación ejecutable del `Compiled Program` es bytecode.
4. **No Execution**: `Compile` no ejecuta el programa ni produce un `Result` de ejecución.
5. **No Invocation Values**: `Compile` no recibe ni enlaza `Invocation Values`.
6. **External Symbols**: pueden preservarse sin resolución concreta durante compilación.
7. **No Provider Discovery**: `Compile` no descubre, selecciona ni posee Providers concretos.
8. **No Runtime Binding Requirement**: la disponibilidad de bindings externos se valida durante ejecución, no como requisito general de compilación.
9. **No Filesystem I/O**: `Compile` no abre, lee, escribe ni resuelve rutas físicas.
10. **No Persistence**: el Engine no persiste ni cachea el `Compiled Program` como responsabilidad funcional de este Use Case.
11. **No Technical Representation Yet**: este Use Case no decide structs, enums, ownership, lifetimes, Requesters, Collaborators, Agents ni mecanismos concretos de retorno.

---

## 14. Out of Scope

- Lectura o escritura de archivos físicos.
- Resolución de rutas.
- Binding de `Invocation Values`.
- Ejecución de bytecode.
- Resolución runtime de `External Symbols` contra capacidades externas.
- Descubrimiento o selección de Providers.
- Persistencia o caché del `Compiled Program`.
- Terminal, stdout, UI o presentación.
- Ciclo de vida de Evo Applications.
- Decisiones sobre Lexer, Parser, AST, Instruction Set, VM, stack/register model u otras estructuras técnicas internas.

---

## 15. Summary Model

```text
UC-001 — Compile

Input
    Source Text

Functional Processing
    Evo-Script rules
    ├── lexical
    ├── syntactic
    └── semantic

Compilation Product
    Compiled Program
    ├── executable representation = Bytecode
    └── may preserve External Symbols

Output
    Compile Outcome
    ├── Success ──► Compiled Program
    └── Failure ──► Failure
                     └── Source Location 0..1

Key Invariants
    Compile does NOT execute
    Compile does NOT receive Invocation Values
    Compile does NOT require concrete Providers
    Compile does NOT resolve runtime bindings
    Compile does NOT perform filesystem I/O
    Compile does NOT persist artifacts
```

---

## Closure Rule

Los niveles técnicos posteriores pueden definir la representación Rust, firmas y Participants necesarios para implementar `Compile`, pero no pueden cambiar sus inputs, outcome funcional, semántica de bytecode o frontera de resolución externa sin reabrir explícitamente este Functional Use Case.
