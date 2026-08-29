# US-003 — Execute Evo-Script Source

Status: REVALIDATED — FUNCTIONAL CLOSED

## Historia

```text
Como Consumer,
quiero proporcionar el Source Text completo de un programa Evo-Script
y los Invocation Values requeridos para su ejecución
al Evo-Script Engine,
para ejecutar el programa conforme a Evo-Script
y obtener su Result.
```

---

## Contexto

El Evo-Script Engine es el componente responsable de compilar y ejecutar programas Evo-Script.

Bajo **US-003 (Execute Source)**, el Consumer proporciona el **Source Text completo** de un programa Evo-Script y los **Invocation Values** ordenados (`0..N`) directamente al Engine. El Consumer solicita compilación y ejecución como una sola operación pública y no necesita invocar previamente `Compile` ni administrar externamente un `Compiled Program`.

```text
Consumer
   │
   ├── Source Text completo de Evo-Script
   └── Invocation Values 0..N
           │
           ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  Compile semantics                     │
│           ↓                            │
│  Compiled Program (bytecode)           │
│           ↓                            │
│  Execute Compiled semantics            │
└──────────────────┬─────────────────────┘
                   │
                   ▼
                 Result
```

`Execute Source` es funcionalmente una operación compuesta. Su comportamiento debe ser semánticamente equivalente a compilar el mismo `Source Text` mediante `Compile` y ejecutar posteriormente el `Compiled Program` resultante mediante `Execute Compiled`, bajo los mismos `Invocation Values` y las mismas capacidades externas disponibles.

---

## Distinciones de Input en la Frontera

- **Source Text != File Path**: el Engine no realiza I/O de archivos ni resolución de rutas; la lectura de un archivo físico `.efn` pertenece al Consumer o a una capacidad externa responsable de ello.
- **Source Text != AST / Token Stream**: el Consumer entrega texto fuente, no representaciones técnicas internas del compilador.
- **Source Text != Individual Function**: el input representa la unidad completa de programa definida por la especificación vigente de Evo-Script.
- **Source Text != Compiled Program**: el Consumer proporciona código fuente; `Execute Source` no exige un artefacto previamente compilado.
- **Invocation Values != Command-Line Strings**: el Engine recibe `Values` estructurados; interpretar argumentos de terminal pertenece a la superficie que consume el Engine.
- **Sin Compile previo requerido**: el Consumer no está obligado a invocar públicamente `Compile` antes de `Execute Source`.

---

## Semántica Funcional de Execute Source

1. **Operación Compuesta**: `Execute Source` acepta `Source Text` e `Invocation Values`, aplica las mismas reglas funcionales de compilación definidas para `Compile` y, si la compilación tiene éxito, aplica las mismas reglas funcionales de ejecución definidas para `Execute Compiled`.
2. **Compilación a Bytecode**: el programa debe producir una representación ejecutable basada en bytecode antes de iniciar su ejecución.
3. **Source Inválido**: si el `Source Text` viola reglas léxicas, sintácticas o semánticas pertenecientes a Evo-Script, `Execute Source` produce un `Result` fallido y no inicia la ejecución del programa.
4. **Sin Retorno de Compiled Program**: el `Compiled Program` producido durante la operación no forma parte del outcome público de `Execute Source`.
5. **Sin Persistencia ni Caché Implícita**: el Engine no persiste, registra ni cachea el `Compiled Program` producido como responsabilidad funcional propia.
6. **Equivalencia Semántica**: para el mismo `Source Text`, los mismos `Invocation Values` y las mismas capacidades externas disponibles, `Execute Source` debe comportarse de manera semánticamente equivalente a `Compile` seguido de `Execute Compiled`.

Conceptualmente:

```text
Execute Source(source, values)

        ≡

Execute Compiled(
    Compile(source),
    values
)
```

La equivalencia anterior expresa semántica funcional. No obliga todavía a una composición concreta de funciones Rust ni a que la implementación invoque literalmente las Public Capabilities entre sí.

---

## Invocation Values

Los `Invocation Values` son los `Values` ordenados proporcionados por el Consumer para satisfacer los parámetros de la `Public Function` determinada conforme a la semántica vigente de Evo-Script.

### Reglas de Invocation Values

1. **Cardinalidad**: `Invocation Values` contiene cero o más `Values` (`0..N`).
2. **Cero Parámetros**: una `Public Function` sin parámetros requiere cero `Invocation Values`.
3. **Aridad Exacta**: una `Public Function` con `N` parámetros requiere exactamente `N Invocation Values`.
4. **Mapeo Posicional Estricto**:

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

5. **Orden de Declaración**: el orden de los `Invocation Values` corresponde al orden de declaración de los parámetros.
6. **Compatibilidad Semántica**: cada `Invocation Value` debe ser semánticamente compatible con el tipo de su parámetro correspondiente.
7. **Sin Conversiones Implícitas**: el Engine no realiza coerciones implícitas para reparar incompatibilidades.
8. **Desajuste de Aridad**: produce un `Result` fallido.
9. **Incompatibilidad de Tipo**: produce un `Result` fallido.
10. **Representación Técnica Diferida**: la representación Rust concreta de `Invocation Values`, slices, ownership o borrowing pertenece al Technical Design.

### Ejemplo Conceptual

```text
public fn sum(int left, int right) -> int
{
    return left + right;
}
```

El Consumer proporciona:

```text
Source Text
Invocation Values: [10, 20]
```

Binding posicional:

```text
InvocationValue[0] (10) ──► left
InvocationValue[1] (20) ──► right
```

La cantidad y selección de `Public Functions` disponibles en una unidad de programa pertenecen a la especificación de Evo-Script y no son definidas por esta User Story.

---

## Compilation Failure y Execution Failure

`Execute Source` puede fallar antes o después de producir internamente un `Compiled Program`.

```text
Source Text
    │
    ▼
Compilation
    │
    ├── Failure ─────────► Result fallido
    │
    ▼
Compiled Program
    │
    ▼
Execution
    │
    ├── Failure ─────────► Result fallido
    │
    ▼
Result exitoso
```

Hacia el Consumer existe un único outcome funcional público: `Result`.

Las categorías, códigos y representaciones concretas de `Compilation Failure` y `Execution Failure` deben definirse posteriormente en el Functional Data Dictionary y, cuando corresponda, en el Technical Data Model.

---

## External Symbols y Capacidades Externas

La fase de compilación puede conservar `External Symbols` sin resolver dentro del `Compiled Program`.

La resolución de esos símbolos ocurre durante la ejecución y únicamente mediante capacidades o bindings explícitamente suministrados por la composición de la aplicación.

```text
Source Text
    │
    ▼
Compile semantics
    │
    └── External Symbol
            │
            ▼
      Execute semantics
            │
            ▼
   explicit application binding
            │
      ┌─────┴─────┐
      ▼           ▼
 disponible    no disponible
      │           │
    invoke     Result fallido
```

Reglas:

- la ausencia de un Provider concreto durante compilación no invalida por sí misma el `Source Text`;
- la ausencia durante ejecución de una capacidad externa requerida produce un `Result` fallido;
- el Engine no descubre Providers;
- el Engine no posee Providers concretos;
- el Engine no utiliza registries globales, Service Locator, reflection ni mecanismos equivalentes de descubrimiento oculto;
- las dependencias requeridas por la ejecución deberán hacerse explícitas posteriormente en el diseño técnico y en las Rust Signatures.

---

## Active Scope y Estado Local de Ejecución

Cada invocación de `Execute Source` mantiene únicamente el estado local requerido por esa ejecución.

Una ejecución comienza sin heredar implícitamente un `Active Scope` de otra ejecución, de una sesión de terminal o de una superficie de presentación.

```text
Execute Source
      │
      ▼
Local Execution State
      │
      ├── Pipeline Data
      └── Active Scope = none inicialmente
```

Si el programa ejecutado utiliza semántica de Scope definida por Evo-Script, puede establecer o cambiar su `Active Scope` durante la ejecución.

`Pipeline Data` y `Active Scope` son canales semánticos distintos. Cambiar el `Active Scope` no destruye ni reemplaza automáticamente los datos que fluyen por el pipeline.

Al concluir `Execute Source`, su estado local termina. El Engine no mantiene una sesión implícita persistente entre invocaciones.

---

## Criterios de Aceptación

1. El Consumer puede proporcionar el `Source Text` completo de un programa Evo-Script al Engine.
2. El Consumer puede proporcionar cero o más `Invocation Values`.
3. El Consumer no necesita invocar `Compile` previamente.
4. El Engine procesa el `Source Text` conforme a la especificación vigente de Evo-Script.
5. El programa se compila a bytecode antes de su ejecución.
6. Un error léxico, sintáctico o semántico de Evo-Script produce un `Result` fallido sin iniciar la ejecución.
7. `Execute Source` no retorna públicamente el `Compiled Program` producido durante la operación.
8. `Execute Source` no persiste ni cachea ese `Compiled Program` como responsabilidad propia.
9. Los `Invocation Values` se enlazan posicionalmente con los parámetros definidos por la `Public Function` determinada conforme a Evo-Script.
10. La aridad debe coincidir exactamente.
11. Cada `Invocation Value` debe ser semánticamente compatible con su parámetro correspondiente.
12. El Engine no realiza coerciones implícitas para reparar incompatibilidades.
13. Un desajuste de aridad produce un `Result` fallido.
14. Una incompatibilidad de tipo produce un `Result` fallido.
15. Cada invocación mantiene estado local independiente.
16. Cada invocación comienza sin heredar implícitamente un `Active Scope` externo.
17. `Pipeline Data` y `Active Scope` permanecen semánticamente separados.
18. Los `External Symbols` requeridos se resuelven durante ejecución mediante bindings explícitos.
19. La ausencia de una capacidad externa requerida produce un `Result` fallido.
20. El Engine no descubre ni conoce Providers concretos como parte de su modelo funcional.
21. Una ejecución exitosa preserva el `Value` producido mediante un `Result` exitoso.
22. Una falla de compilación o de ejecución produce un `Result` fallido.
23. `Execute Source` no realiza filesystem I/O, presentación de terminal, UI ni serialización HTTP como responsabilidad propia.
24. Al concluir la invocación no queda una sesión o contexto implícito persistente dentro del Engine.
25. Bajo las mismas entradas y capacidades externas, `Execute Source` debe ser semánticamente equivalente a `Compile` seguido de `Execute Compiled`.

---

## Concepto de Outcome de Ejecución — Result

El outcome funcional de `Execute Source` es `Result`:

- **Outcome Exitoso**: preserva el `Value` producido por el programa.
- **Outcome Fallido**: representa una falla ocurrida durante compilación o ejecución.

`Result != Value` y `Result != Failure`.

Las representaciones concretas en Rust, parámetros de tipo, estructuras o variantes pertenecen al Functional Data Dictionary y al Technical Design correspondiente; no se deciden en esta User Story.

---

## No Responsabilidades y Fuera de Alcance

Para el alcance de US-003:

- leer archivos `.efn` desde filesystem o resolver rutas físicas;
- devolver un `Compiled Program` como outcome público;
- persistir, serializar o cachear artefactos compilados como responsabilidad propia;
- interpretar command-line strings;
- imprimir en terminal o stdout;
- construir UI;
- serializar HTTP/JSON como responsabilidad propia;
- administrar el ciclo de vida de una Evo Application;
- descubrir Providers o capacidades mediante registries ocultos;
- definir la semántica de EvoQ, Scope u otras capacidades externas;
- definir todavía la arquitectura concreta de la VM, stack frames, opcodes, AST, lexer, parser u otras representaciones técnicas internas.

## Closure

Esta User Story ha sido revalidada contra el `Purpose`, las `Public Capabilities` y la arquitectura superior vigente de Evolution.

**US-003 queda `REVALIDATED — FUNCTIONAL CLOSED`.**
