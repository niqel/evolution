# UC-002 — Execute Compiled Evo-Script Program

Status: FUNCTIONAL CLOSED

## 1. Propósito

El Use Case **UC-002 (Execute Compiled)** define funcionalmente cómo un `Consumer` solicita al `Evo-Script Engine` ejecutar un `Compiled Program` existente, proporcionando los `Invocation Values` requeridos por su única `Public Function` para obtener un `Result`.

```text
Compiled Program + Invocation Values (0..N) ──► Execute Compiled ──► Result
```

---

## 2. Trazabilidad

- **Deriva de**: [US-002 — Execute Compiled Evo-Script Program](../user-stories/US-002-execute-compiled-evo-script-program.md)
- **Utiliza conceptos de**: [Evo-Script Engine v0 — Data Dictionary Funcional](../DATA_DICTIONARY.md)
- **Aplica normativamente**: [`Evo-Script Language Specification v0`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md)

---

## 3. Operación Funcional

La operación funcional provista es:

```text
Execute Compiled(
    Compiled Program,
    Invocation Values
) -> Result
```

Esta notación establece la frontera conceptual del Use Case. No define firmas técnicas de Rust, modificadores de ownership, borrowing, lifetimes, genéricos ni wrappers de tipo concretos.

---

## 4. Consumer

El `Consumer` es el actor/rol funcional externo que invoca la operación `Execute Compiled`:

- Proporciona exactamente **1 `Compiled Program`**.
- Proporciona los **`Invocation Values` (0..N)** requeridos por la `Public Function` del programa.
- Recibe exactamente **1 `Result`** al concluir la invocación.

El `Consumer` representa un rol funcional (como una CLI, test suite, runner o aplicación anfitriona) y no un componente físico o estructura de datos concreta.

---

## 5. Inputs Funcionales

La operación recibe exactamente dos inputs funcionales:

1. **`Compiled Program`**:
   - Cardinalidad: exactamente 1.
   - Representa la unidad de programa Evo-Script ya procesada y validada por `Compile`.
   - Invariantes de frontera:
     - `Compiled Program != Source Text`
     - `Execute Compiled` **no** recibe Source Text.
     - `Execute Compiled` **no** recibe File Path ni archivos `.efn`.
     - `Execute Compiled` **no** recibe AST ni Token Stream.

2. **`Invocation Values`**:
   - Cardinalidad: `0..N` `Values` ordenados.
   - Suministrados para satisfacer los parámetros formales de la `Public Function`.
   - Invariantes de frontera:
     - `Invocation Values != Command-Line Strings` (valores de datos estructurados, no texto plano de terminal).

---

## 6. Output Funcional

Toda invocación concluida de `Execute Compiled` produce exactamente **1 `Result`**:

```text
Result
├── success ──► Preserva el Value producido por la Public Function
└── failure ──► Expresa un Failure
```

Invariantes de output:
- `Result != Value`
- `Result != Failure`
- El outcome pertenece al modelo conceptual de outcomes de `evo-values`.

---

## 7. Reglas Funcionales

### 7.1 Ejecución de la Public Function
El `Compiled Program` representa un programa Evo-Script v0 que contiene exactamente una `Public Function`. `Execute Compiled` ejecuta esa única `Public Function`.

### 7.2 Correspondencia Exacta de Aridad
- Si la `Public Function` declara $0$ `Parameters`, la invocación requiere exactamente $0$ `Invocation Values`.
- Si la `Public Function` declara $N$ `Parameters`, la invocación requiere exactamente $N$ `Invocation Values`.
- No se admiten argumentos opcionales, valores por defecto, argumentos nombrados ni parámetros variádicos.

### 7.3 Binding Posicional Estricto
El mapeo entre `Invocation Values` y `Parameters` es estrictamente posicional:

```text
InvocationValue[0]     ──► Parameter[0]
InvocationValue[1]     ──► Parameter[1]
...
InvocationValue[N - 1] ──► Parameter[N - 1]
```

El orden de los `Invocation Values` corresponde directamente al orden de declaración de los parámetros en la firma de la `Public Function`.

### 7.4 Compatibilidad Semántica de Tipos
Cada `Invocation Value` debe ser semánticamente compatible con el tipo declarado de su `Parameter` correspondiente (incluyendo tipos nativos, structs y enums definidos en el programa conforme a `Evo-Script Language Specification v0`).

### 7.5 Ausencia de Conversiones Implícitas
El Engine no realiza conversiones, coerciones ni adaptaciones automáticas de tipo. Un `Value` incompatible produce un fallo de invocación.

### 7.6 Evaluación y Conclusión Exitosa
Una vez validada la invocación, el Engine ejecuta la `Public Function` según la semántica de `Evo-Script Language Specification v0`. Si la ejecución concluye sin errores, el `Value` resultante se preserva en la rama exitosa de `Result`.

### 7.7 Reutilización e Independencia del Compiled Program
Un `Compiled Program` puede ser ejecutado múltiples veces a través de sucesivas invocaciones independientes de `Execute Compiled`:

```text
Compiled Program P
    ├── Execute Compiled(P, Values A) ──► Result A
    ├── Execute Compiled(P, Values B) ──► Result B
    └── Execute Compiled(P, Values C) ──► Result C
```

- Una invocación no consume ni invalida funcionalmente el `Compiled Program`.
- Un fallo durante la ejecución no degrada ni invalida el `Compiled Program` para futuras ejecuciones.

---

## 8. Flujos de Failure

Todo fallo en `Execute Compiled` se expresa como un `Result.failure` que contiene un `Failure`:

```text
Failure
├── description: obligatoria
└── source line: 0..1 (1-based, según origen)
```

Se distinguen funcionalmente dos orígenes de fallo:

### 8.1 Failure de Frontera de Invocación
Ocurre cuando la invocación incumple las reglas de frontera:
- **Desajuste de Aridad**: la cantidad de `Invocation Values` no coincide con la cantidad de `Parameters` ($N$).
- **Incompatibilidad de Tipo**: un `Invocation Value` no es semánticamente compatible con su `Parameter` correspondiente.

**Comportamiento**:
- Se produce `Result.failure`.
- El `Failure` contiene una `description` explicativa.
- El campo `source line` permanece **ausente** (no corresponde a una línea interna del código fuente del programa; no se generan líneas artificiales como línea 0 o línea 1 arbitraria).
- La `Public Function` **no** se ejecuta.

### 8.2 Failure Originado en el Programa
Ocurre durante la evaluación interna de la `Public Function` (por ejemplo, errores de runtime o violaciones semánticas en tiempo de ejecución conforme a `Evo-Script Language Specification v0`).

**Comportamiento**:
- Se produce `Result.failure`.
- El `Failure` contiene una `description` explicativa.
- El campo `source line` está **presente** (1-based) cuando el fallo corresponde a una ubicación del código fuente original, gracias a la garantía de ubicación preservada por el `Compiled Program`.

> [!NOTE]
> `Execute Compiled` recibe un `Compiled Program` ya validado, por lo que **no realiza compilación** ni produce errores de compilación léxicos o sintácticos (`Compilation Failure`).

---

## 9. Invariantes Funcionales

1. `Execute Compiled` recibe exactamente 1 `Compiled Program`.
2. `Execute Compiled` recibe `0..N` `Invocation Values`.
3. `Execute Compiled` **no** recibe `Source Text`.
4. `Execute Compiled` **no** recibe `File Path`.
5. `Execute Compiled` **no** realiza `Compile` público.
6. Los `Invocation Values` se enlazan estrictamente por posición.
7. La aridad de los `Invocation Values` debe coincidir exactamente con la de los `Parameters`.
8. No existen conversiones implícitas de tipos.
9. Toda invocación concluida produce exactamente 1 `Result`.
10. `Result.success` preserva el `Value` producido.
11. `Result.failure` expresa un `Failure`.
12. `Result != Value` y `Result != Failure`.
13. El `Compiled Program` es reutilizable a través de múltiples invocaciones.
14. Cada invocación de `Execute Compiled` es funcionalmente independiente.
15. Un fallo en una invocación no invalida funcionalmente el `Compiled Program`.
16. No se requiere objeto de sesión del Engine (`Session`).
17. No se requiere contexto de ejecución público (`Execution Context`).
18. `Execute Compiled` no administra persistencia ni almacenamiento.
19. `Execute Compiled` no administra operaciones de sistema de archivos.

---

## 10. No Responsabilidades y Fuera de Alcance

Para el alcance de UC-002 v0:
- Lectura de archivos `.efn`, resolución de rutas o acceso al sistema de archivos.
- Parseo o compilación de `Source Text` (responsabilidad de UC-001).
- Carga, serialización, deserialización, caché o persistencia física de `Compiled Programs`.
- Formato binario interno o representación en memoria del `Compiled Program`.
- Gestión de ciclo de vida de aplicaciones de Evo Runtime (`Start` / `Run`).
- Objetos de sesión de Engine o contextos de ejecución explícitos en la frontera.
- Definición de `Requesters`, `Contracts` o `Providers` (no requeridos en esta frontera funcional).
- Interacción con la terminal, UI, salida estándar (`stdout`, `print`) o efectos laterales externos.
- Decisiones técnicas sobre la arquitectura de la VM, intérprete o evaluador interno.

---

## 11. Modelo Resumido

```text
UC-002 — Execute Compiled

Consumer
    │
    ├── Compiled Program
    │       cardinalidad: exactamente 1
    │
    └── Invocation Values
            cardinalidad: 0..N
            ordenados
            │
            ▼
    Execute Compiled
            │
            ├── valida aridad
            ├── realiza binding posicional
            ├── valida compatibilidad semántica
            └── ejecuta la única Public Function
                    │
                    ▼
                  Result
                 /      \
                /        \
         success          failure
            │                │
          Value            Failure
                          ├── description: obligatoria
                          └── source line: 0..1 (1-based)

Failure de frontera (aridad / incompatibilidad de tipo):
    source line = ausente

Failure originado en el programa (evaluación runtime):
    source line = presente (si existe ubicación fuente)

Invariantes Clave:
    NO recibe Source Text
    NO realiza Compile
    NO realiza conversiones implícitas
    NO administra filesystem ni persistencia
    NO requiere Engine Session ni Context
    NO define Requester, Contract ni Provider
    Compiled Program es reutilizable
```
