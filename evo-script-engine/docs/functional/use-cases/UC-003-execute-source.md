# UC-003 — Execute Source

Status: FUNCTIONAL CLOSED

## 1. Propósito

Definir funcionalmente cómo un `Consumer` solicita al `Evo-Script Engine` ejecutar directamente el `Source Text` completo de un programa Evo-Script junto con sus `Invocation Values`, sin requerir una invocación pública previa de `Compile` ni la administración externa de un `Compiled Program`.

```text
Source Text + Invocation Values (0..N)
                │
                ▼
          Execute Source
                │
                ▼
              Result
```

`Execute Source` es una operación funcional compuesta cuyo comportamiento debe ser semánticamente equivalente a aplicar `Compile` y, cuando la compilación es exitosa, aplicar `Execute Compiled` sobre el `Compiled Program` resultante.

---

## 2. Trazabilidad

- **Deriva de**: [`US-003 — Execute Evo-Script Source`](../user-stories/US-003-execute-evo-script-source.md)
- **Utiliza conceptos de**: [`Evo-Script Engine v0 — Functional Data Dictionary`](../DATA_DICTIONARY.md)
- **Aplica las reglas funcionales de**:
  - [`UC-001 — Compile`](UC-001-compile.md)
  - [`UC-002 — Execute Compiled`](UC-002-execute-compiled.md)
- **Aplica normativamente**: `Evo-Script Language Specification` vigente.

---

## 3. Operación Funcional

La operación funcional provista es:

```text
Execute Source(
    Source Text,
    Invocation Values
) -> Result
```

Esta notación representa la frontera funcional pública. No define todavía una firma Rust, ownership, borrowing, lifetimes, estructuras, enums, function pointers auxiliares ni Participants técnicos.

---

## 4. Consumer

El `Consumer` es el rol funcional externo que solicita `Execute Source`:

- proporciona exactamente 1 `Source Text`;
- proporciona `0..N Invocation Values`;
- recibe exactamente 1 `Result` cuando concluye la operación;
- no necesita invocar públicamente `Compile` antes de `Execute Source`;
- no necesita recibir ni administrar el `Compiled Program` producido dentro de la operación.

El `Consumer` no representa una estructura técnica concreta ni implica una superficie específica como CLI, UI o API.

---

## 5. Inputs Funcionales

### 5.1 Source Text

`Execute Source` recibe exactamente 1 `Source Text` completo.

Invariantes:

- `Source Text != File Path`;
- `Source Text != AST`;
- `Source Text != Token Stream`;
- `Source Text != Compiled Program`;
- representa la unidad completa de programa definida por Evo-Script;
- el Engine no realiza filesystem I/O para obtenerlo.

### 5.2 Invocation Values

`Execute Source` recibe `0..N Invocation Values` ordenados.

Invariantes:

- `Invocation Values` contiene `Values` estructurados;
- `Invocation Values != Command-Line Strings`;
- la aridad debe coincidir exactamente con los `Parameters` del entry point determinado conforme a Evo-Script;
- el binding es estrictamente posicional;
- cada `Invocation Value` debe ser semánticamente compatible con su `Parameter` correspondiente;
- no existen coerciones implícitas para reparar incompatibilidades.

---

## 6. Output Funcional

Toda invocación concluida de `Execute Source` produce exactamente 1 `Result`.

```text
Result
├── success ──► preserva Value
└── failure ──► expresa Failure
```

`Execute Source` no expone públicamente:

- `Compile Outcome`;
- `Compiled Program`;
- bytecode;
- AST;
- Tokens;
- ninguna representación intermedia de compilación o ejecución.

---

## 7. Equivalencia Semántica

La regla central de `Execute Source` es:

> Para el mismo `Source Text`, los mismos `Invocation Values` y las mismas `External Capabilities` disponibles, `Execute Source` debe comportarse semánticamente de manera equivalente a ejecutar `Compile` y posteriormente `Execute Compiled` cuando la compilación es exitosa.

Conceptualmente:

```text
Execute Source(source, values)

        ≡

Compile(source)
    │
    ├── Failure
    │      │
    │      ▼
    │  Result.failure
    │
    └── Success(compiled)
             │
             ▼
      Execute Compiled(
          compiled,
          values
      )
```

Esta equivalencia no obliga a que la futura implementación Rust invoque literalmente las Public Capabilities entre sí. La composición técnica se decidirá posteriormente.

---

## 8. Fase Funcional de Compilación

`Execute Source` aplica las mismas reglas funcionales cerradas para `Compile`.

La fase de compilación:

1. recibe el `Source Text`;
2. valida las reglas léxicas de Evo-Script;
3. valida las reglas sintácticas de Evo-Script;
4. valida las reglas semánticas pertenecientes al lenguaje;
5. produce bytecode como representación ejecutable del `Compiled Program`;
6. puede preservar `External Symbols` válidos sin resolver;
7. no requiere Providers concretos ni bindings de ejecución para que la compilación sea exitosa;
8. no ejecuta el programa.

Si la compilación falla:

```text
Compilation Failure
        │
        ▼
      Failure
        │
        ▼
  Result.failure
```

La fase de ejecución no comienza.

El `Compile Outcome` es conceptualmente utilizado durante la operación compuesta, pero no forma parte de la frontera pública de `Execute Source`.

---

## 9. Compiled Program Interno

Cuando la compilación es exitosa se produce conceptualmente un `Compiled Program`.

```text
Source Text
    │
    ▼
Compile semantics
    │
    ▼
Compiled Program
    │
    └── executable representation: bytecode
```

Reglas:

- el `Compiled Program` existe como artefacto funcional necesario para la ejecución;
- contiene bytecode;
- puede preservar `External Symbols`;
- no se devuelve públicamente al `Consumer`;
- el Engine no lo persiste, registra o cachea implícitamente como responsabilidad funcional propia;
- no se establece en esta etapa su representación Rust, ownership o layout de memoria.

---

## 10. Fase Funcional de Ejecución

Cuando la compilación es exitosa, `Execute Source` aplica las mismas reglas funcionales de `Execute Compiled`.

La fase de ejecución:

1. utiliza el `Compiled Program` producido;
2. determina el entry point conforme a la semántica vigente de Evo-Script;
3. valida la aridad de los `Invocation Values`;
4. realiza binding posicional estricto;
5. valida compatibilidad semántica de los `Values`;
6. no realiza conversiones implícitas;
7. ejecuta el bytecode;
8. mantiene estado local independiente para esa invocación;
9. resuelve `External Symbols` únicamente mediante capacidades explícitamente disponibles;
10. produce `Result`.

---

## 11. External Symbols y External Capabilities

La fase de compilación puede conservar `External Symbols` dentro del `Compiled Program` sin resolverlos.

La resolución ocurre durante ejecución:

```text
Compiled Program
      │
      ▼
    Bytecode
      │
      ▼
External Symbol
      │
      ▼
explicit application binding
      │
      ▼
External Capability
```

Reglas funcionales:

1. La inexistencia de un Provider concreto durante compilación no constituye por sí sola un `Compilation Failure`.
2. Si durante ejecución se requiere una `External Capability`, esta debe estar disponible mediante composición explícita de la aplicación.
3. El Engine no descubre Providers ni bindings mediante registries globales, Service Locator, reflection o mecanismos equivalentes.
4. Si una capacidad requerida no está disponible, `Execute Source` produce `Result.failure`.
5. La forma técnica concreta mediante la que las capacidades se incorporan a la Rust Signature pertenece al Technical Design.

---

## 12. Active Scope y Estado Local de Ejecución

Cada invocación de `Execute Source` posee estado local independiente.

Conceptualmente:

```text
Execute Source
      │
      ▼
Local Execution State
      │
      ├── Active Scope = none inicialmente
      ├── Pipeline Data
      └── evaluation state
```

Reglas:

1. La ejecución no hereda implícitamente `Active Scope` de una CLI, UI, otra ejecución o una sesión anterior.
2. El programa puede establecer o cambiar `Active Scope` conforme a la semántica de Evo-Script.
3. `Pipeline Data` y `Active Scope` son canales semánticos diferentes.
4. Cambiar `Active Scope` no destruye ni reemplaza automáticamente `Pipeline Data`.
5. El estado local termina al concluir la invocación.
6. El Engine no mantiene una `Session` implícita persistente entre invocaciones.

---

## 13. Flujos de Failure

Todo fallo público de `Execute Source` termina expresado mediante `Result.failure` y un `Failure`.

### 13.1 Compilation Failure

Puede originarse cuando el `Source Text` viola reglas del lenguaje durante la fase funcional equivalente a `Compile`.

Ejemplos funcionales de origen:

- regla léxica inválida;
- regla sintáctica inválida;
- regla semántica del lenguaje inválida.

Comportamiento:

- no comienza la ejecución;
- no se expone ningún `Compiled Program`;
- se produce `Result.failure`.

### 13.2 Invocation / Execution Failure

Puede originarse después de una compilación exitosa.

Ejemplos funcionales de origen:

- desajuste de aridad;
- incompatibilidad semántica de tipo;
- fallo de evaluación runtime definido por Evo-Script.

Comportamiento:

- se produce `Result.failure`;
- el `Compiled Program` temporal no se expone públicamente.

### 13.3 External Capability Failure

Ocurre cuando la ejecución requiere una capacidad externa que no puede satisfacerse mediante los bindings explícitos disponibles o cuando dicha invocación externa falla funcionalmente.

Comportamiento:

- se produce `Result.failure`;
- el Engine no intenta descubrir automáticamente otro Provider.

Las categorías técnicas concretas, enums, códigos y variantes de error se definen posteriormente.

---

## 14. Postcondiciones

### Postcondición Exitosa

- la compilación interna fue exitosa;
- el bytecode fue ejecutado conforme a Evo-Script;
- el `Value` producido queda preservado en `Result.success`;
- no queda una sesión implícita persistente;
- el `Compiled Program` temporal no se convierte automáticamente en estado persistente del Engine.

### Postcondición Fallida

- el fallo se expresa mediante `Result.failure`;
- no se expone un `Compiled Program` parcial o temporal;
- la invocación concluye sin dejar estado local compartido con futuras ejecuciones.

---

## 15. Invariantes Funcionales

1. El nombre canónico del Use Case es `Execute Source`.
2. Consume exactamente 1 `Source Text`.
3. Consume `0..N Invocation Values`.
4. Produce exactamente 1 `Result`.
5. El Consumer no necesita invocar públicamente `Compile` antes de usarlo.
6. No recibe un `Compiled Program` como input.
7. Aplica las mismas reglas funcionales de `Compile`.
8. Produce bytecode antes de comenzar la ejecución.
9. Si la compilación falla, la ejecución no comienza.
10. Un fallo de compilación se expresa públicamente mediante `Result.failure`.
11. Si la compilación es exitosa, aplica las mismas reglas funcionales de `Execute Compiled`.
12. Los `Invocation Values` requieren aridad exacta.
13. El binding de `Invocation Values` es posicional.
14. No existen conversiones implícitas para reparar incompatibilidades.
15. El entry point se determina conforme a Evo-Script y no se redefine en este Use Case.
16. El `Compiled Program` puede contener `External Symbols`.
17. Las `External Capabilities` requeridas durante ejecución deben suministrarse explícitamente.
18. El Engine no descubre Providers.
19. Una capacidad externa requerida que no está disponible produce `Result.failure`.
20. Cada invocación mantiene estado local independiente.
21. Cada invocación comienza sin `Active Scope` heredado implícitamente.
22. El programa puede modificar su `Active Scope` conforme a Evo-Script.
23. `Pipeline Data != Active Scope`.
24. Una invocación no contamina el estado local de otra.
25. El `Compiled Program` producido internamente no se devuelve al Consumer.
26. El Engine no persiste ni cachea implícitamente ese `Compiled Program`.
27. El Engine no realiza filesystem I/O para obtener `Source Text`.
28. El Engine no realiza presentación de terminal, UI o HTTP como responsabilidad propia.
29. La invocación no deja una `Session` o contexto persistente implícito.
30. `Execute Source` debe ser semánticamente equivalente a `Compile` seguido de `Execute Compiled` bajo las mismas entradas y capacidades externas.
31. Este Use Case no decide todavía Agents, Collaborators, Requesters, Contracts, Resolvers, Providers concretos, Tools ni firmas Rust.

---

## 16. No Responsabilidades y Fuera de Alcance

- Resolución o lectura de rutas `.efn` desde filesystem.
- Interpretación de command-line strings.
- Exposición pública del `Compiled Program` generado durante la operación.
- Persistencia o caché de artefactos compilados como responsabilidad propia del Engine.
- Descubrimiento automático de Providers o capacidades.
- Gestión del ciclo de vida de Evo Applications.
- Presentación mediante terminal, UI, stdout o HTTP/JSON.
- Definición de la semántica propia de `Scope`, Query u otras capacidades externas.
- Decidir arquitectura interna de VM, frames, opcodes, AST, Lexer, Parser o representaciones técnicas.
- Decidir Participants técnicos o Rust Signatures.

---

## 17. Modelo Resumido

```text
UC-003 — Execute Source

Consumer
   │
   ├── Source Text
   └── Invocation Values 0..N
           │
           ▼
      Execute Source
           │
           ├── Compile semantics
           │       │
           │       ├── Failure ─────────────► Result.failure
           │       │
           │       ▼
           │  Compiled Program
           │       └── bytecode
           │       └── External Symbols 0..N
           │
           └── Execute Compiled semantics
                   │
                   ├── local execution state
                   │    ├── Active Scope
                   │    └── Pipeline Data
                   │
                   ├── External Symbol
                   │       ↓
                   │  explicit binding
                   │       ↓
                   │  External Capability
                   │
                   ▼
                 Result
                /      \
         success        failure
            │              │
          Value          Failure

Invariante central:

Execute Source
    ≡
Compile + Execute Compiled

bajo las mismas entradas y capacidades externas.
```

## Closure

`UC-003 — Execute Source` queda `FUNCTIONAL CLOSED` y completa el conjunto de Functional Use Cases públicos de `evo-script-engine` v0.