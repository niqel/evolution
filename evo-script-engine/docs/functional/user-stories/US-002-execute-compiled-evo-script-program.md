# US-002 — Execute Compiled Evo-Script Program

Status: REVALIDATED — FUNCTIONAL CLOSED

## Historia

```text
Como Consumer,
quiero proporcionar un Compiled Program y los Invocation Values
requeridos para su ejecución al Evo-Script Engine,
para ejecutar el programa compilado conforme a Evo-Script
y obtener su Result.
```

---

## Contexto

El Evo-Script Engine ejecuta programas Evo-Script previamente compilados.

Bajo **US-002 (Execute Compiled)**, el Consumer proporciona un **Compiled Program** y los **Invocation Values** ordenados (`0..N`) requeridos para la invocación definida por la semántica vigente de Evo-Script.

`Execute Compiled` no acepta Source Text y no realiza compilación. Ejecuta el bytecode contenido por el Compiled Program y produce un `Result`.

```text
Consumer
   │
   ├── Compiled Program
   └── Invocation Values 0..N
           │
           ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  Enlaza Invocation Values a parámetros │
│  Ejecuta el bytecode                   │
│  Mantiene estado local de ejecución    │
│  Resuelve capacidades externas         │
│  mediante bindings explícitos          │
└──────────────────┬─────────────────────┘
                   │
                   ▼
                 Result
```

La cantidad y selección de Public Functions pertenecen a la especificación de Evo-Script y no son definidas por esta User Story. Mientras la especificación vigente determine un único entry point público, la ejecución es inequívoca. Si en el futuro Evo-Script permitiera múltiples Public Functions seleccionables externamente, las Public Capabilities de ejecución deberán reabrirse para definir cómo seleccionar el entry point.

---

## Invocation Values

Los Invocation Values son Values ordenados proporcionados por el Consumer para satisfacer los parámetros de la Public Function determinada conforme a la semántica vigente de Evo-Script.

### Reglas de Invocation Values

1. **Cardinalidad**: Invocation Values contiene cero o más Values (`0..N`).
2. **Cero Parámetros**: Una Public Function sin parámetros requiere cero Invocation Values.
3. **Aridad Exacta**: Una Public Function con `N` parámetros requiere exactamente `N` Invocation Values.
4. **Mapeo Posicional Estricto**: El mapeo de Invocation Values a parámetros es estrictamente posicional:

   ```text
   InvocationValue[0]     ──► Parameter[0]
   InvocationValue[1]     ──► Parameter[1]
   ...
   InvocationValue[N - 1] ──► Parameter[N - 1]
   ```

5. **Orden de Declaración**: El orden de los Invocation Values corresponde directamente al orden de declaración de los parámetros en la firma de la Public Function.
6. **Compatibilidad Semántica**: Cada Invocation Value debe ser semánticamente compatible con el tipo del parámetro correspondiente.
7. **Sin Conversiones Implícitas**: El Engine no realiza coerciones implícitas para reparar un Value incompatible.
8. **Desajuste de Aridad**: Una cantidad incorrecta de Invocation Values produce un Result fallido.
9. **Incompatibilidad de Tipo**: Un Invocation Value incompatible con su parámetro produce un Result fallido.
10. **Failure Diferido**: Las categorías concretas de Failure, códigos y variantes se definen posteriormente en el Functional Data Dictionary y el diseño técnico.
11. **Representación Técnica Diferida**: La representación Rust concreta de Invocation Values se define en el diseño técnico.

### Ejemplo Conceptual

Dada una Public Function en Evo-Script:

```text
public fn sum(int left, int right) -> int
{
    return left + right;
}
```

El Consumer proporciona:

- Compiled Program que contiene el entry point correspondiente;
- Invocation Values: `[10, 20]`.

Binding posicional:

- `InvocationValue[0]` (`10`) → `left`;
- `InvocationValue[1]` (`20`) → `right`.

---

## Execution State

Cada invocación de `Execute Compiled` mantiene su propio estado local de ejecución.

### Reglas de estado

1. Una ejecución no comparte implícitamente su estado local con otra ejecución.
2. Una ejecución `.efn` inicia sin heredar implícitamente el Active Scope de una terminal, UI, API u otra ejecución.
3. El programa puede establecer o cambiar su Active Scope conforme a la semántica de Evo-Script.
4. Pipeline Data y Active Scope son canales semánticos distintos.
5. Cambiar el Active Scope no destruye ni reemplaza automáticamente el Pipeline Data.
6. Finalizada la invocación, su estado local de ejecución deja de pertenecer a esa operación.
7. El Consumer no necesita proporcionar un objeto genérico de sesión, Context o Service Locator en la frontera funcional.

Conceptualmente:

```text
Compiled Program
      │
      ├── Execute(values A) ──► Execution State A ──► Result A
      ├── Execute(values B) ──► Execution State B ──► Result B
      └── Execute(values C) ──► Execution State C ──► Result C
```

---

## External Symbols and Capabilities

Un Compiled Program puede contener External Symbols conservados durante `Compile`.

Cuando la ejecución alcanza una operación que requiere una capacidad externa:

```text
External Symbol
      │
      ▼
explicit application binding
      │
      ├── capacidad disponible ──► continuar ejecución
      └── capacidad ausente ─────► Result fallido
```

### Reglas funcionales

1. Los External Symbols se resuelven durante la ejecución, no mediante descubrimiento oculto durante Compile.
2. Las capacidades externas requeridas deben haber sido suministradas explícitamente por la composición de la aplicación.
3. El Engine no conoce ni descubre Providers concretos.
4. El Engine no mantiene registries globales de Providers o capacidades.
5. La ausencia de una capacidad externa requerida produce un Result fallido.
6. El mecanismo técnico exacto de binding, Requesters, Contracts y Providers se define posteriormente en el diseño técnico.

---

## Criterios de Aceptación

1. El Consumer puede proporcionar un Compiled Program válido al Evo-Script Engine.
2. El Consumer puede proporcionar cero o más Invocation Values.
3. Execute Compiled no recibe Source Text.
4. Execute Compiled no realiza compilación de código fuente.
5. El Engine ejecuta el bytecode contenido por el Compiled Program.
6. Los Invocation Values se emparejan con los parámetros estrictamente por posición.
7. La cantidad de Invocation Values debe coincidir exactamente con la cantidad de parámetros requeridos.
8. Cada Invocation Value debe ser semánticamente compatible con su parámetro correspondiente.
9. El Engine no realiza conversiones implícitas para reparar incompatibilidades.
10. Las Public Functions de cero parámetros requieren cero Invocation Values.
11. Un desajuste de aridad produce un Result fallido.
12. Una incompatibilidad de tipo produce un Result fallido.
13. Cada ejecución mantiene estado local independiente.
14. La ejecución inicia sin heredar implícitamente un Active Scope externo.
15. Pipeline Data y Active Scope permanecen semánticamente separados.
16. Si el programa requiere una capacidad externa disponible mediante binding explícito, el Engine puede utilizarla durante la ejecución.
17. Si una capacidad externa requerida no está disponible, la ejecución produce un Result fallido.
18. El Engine no descubre ni selecciona Providers concretos mediante mecanismos ocultos.
19. Una ejecución exitosa preserva el Value producido por el programa en el Result exitoso.
20. Una ejecución fallida produce un Result fallido en lugar de tratarse silenciosamente como éxito.
21. El mismo Compiled Program puede ejecutarse múltiples veces con distintos Invocation Values.
22. Una ejecución no contamina el estado local de otra ejecución.
23. Cuando se retorna el Result al Consumer, esa invocación de Execute Compiled concluye.

---

## Concepto de Outcome de Ejecución (Result)

El outcome funcional de ejecutar un programa compilado se representa conceptualmente como `Result`:

- **Outcome Exitoso**: preserva el `Value` producido por el programa.
- **Outcome Fallido**: representa una falla de ejecución, como desajuste de aridad, incompatibilidad de tipo, falla de evaluación o capacidad externa requerida no disponible.

`Result` es un concepto funcional compartido de Evo y no debe confundirse con `Value` ni con `Failure`.

La representación técnica concreta de `Result`, `Failure` y sus variantes se define posteriormente.

---

## No Responsabilidades y Fuera de Alcance

Para el alcance de US-002:

- compilación o parseo de Source Text de Evo-Script, cubierto por US-001;
- carga de Compiled Programs desde almacenamiento físico o filesystem;
- persistencia del estado de ejecución después de concluir la invocación;
- interacción con terminal, stdout, UI o protocolos de presentación;
- gestión del ciclo de vida de una Evo Application;
- implementación de filesystem, database, network u otros Providers;
- descubrimiento dinámico u oculto de Providers;
- definición de la semántica de EvoQ;
- definición de la arquitectura interna concreta de la VM, por ejemplo Stack VM o Register VM;
- definición técnica de Requesters, Contracts, Resolvers, Collaborators o Tools.

---

## Revalidation

Esta User Story fue revalidada contra el Purpose y las Public Capabilities cerradas de `evo-script-engine`, además de las decisiones arquitectónicas actuales sobre bytecode, External Symbols, bindings explícitos, Active Scope y separación entre Pipeline Data y Active Scope.

US-002 se considera `REVALIDATED — FUNCTIONAL CLOSED`.
