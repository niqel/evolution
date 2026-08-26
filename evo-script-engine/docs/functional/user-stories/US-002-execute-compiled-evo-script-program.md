# US-002 — Execute Compiled Evo-Script Program

Status: FUNCTIONAL CLOSED

## Historia

```text
Como Consumer,
quiero proporcionar un Compiled Program y los Invocation Values
requeridos por su Public Function al Evo-Script Engine,
para que el programa compilado sea ejecutado y reciba su Result.
```

---

## Contexto

El Evo-Script Engine es el componente responsable de compilar y ejecutar
programas Evo-Script.

Bajo **US-002 (Execute)**, el Consumer proporciona un **Compiled Program**
previamente compilado y los **Invocation Values** ordenados (0..N) requeridos por
su única Public Function.

Execute **no** acepta Source Text y **no** realiza compilación. Evalúa el
programa compilado directamente y retorna el `Result` de ejecución.

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
│  Ejecuta la única Public Function      │
└──────────────────┬─────────────────────┘
                   │
                   │ outcome de ejecución
                   ▼
                 Result
```

---

## Invocation Values

Los Invocation Values son los Values ordenados proporcionados por el Consumer para
satisfacer los parámetros declarados por la única Public Function del programa.

### Reglas de Invocation Values

1. **Cardinalidad**: Invocation Values contiene cero o más Values (`0..N`).
2. **Cero Parámetros**: Una Public Function sin parámetros requiere cero
   Invocation Values.
3. **Aridad Exacta**: Una Public Function con $N$ parámetros requiere exactamente
   $N$ Invocation Values.
4. **Mapeo Posicional Estricto**: El mapeo de Invocation Values a parámetros es
   estrictamente posicional:
   ```text
   InvocationValue[0]     ──► Parameter[0]
   InvocationValue[1]     ──► Parameter[1]
   ...
   InvocationValue[N - 1] ──► Parameter[N - 1]
   ```
5. **Orden de Declaración**: El orden de los Invocation Values corresponde
   directamente al orden de declaración de los parámetros en la firma de la
   Public Function.
6. **Compatibilidad Semántica**: Cada Invocation Value debe ser semánticamente
   compatible con el tipo de parámetro declarado por la Public Function
   (incluyendo tipos nativos, structs y enums definidos en el programa).
7. **Sin Conversiones Implícitas**: El Engine no realiza conversiones ni
   coerciones implícitas para adaptar un Value incompatible a un tipo de
   parámetro.
8. **Desajuste de Aridad**: Una cantidad de Invocation Values que no coincida con
   la cantidad de parámetros produce un Result fallido.
9. **Incompatibilidad de Tipo**: Un Invocation Value incompatible con su tipo de
   parámetro correspondiente produce un Result fallido.
10. **Independencia de Failure**: Las categorías específicas de Failure, códigos
    de error y variantes no se definen en este nivel funcional.
11. **Representación Técnica**: Las representaciones técnicas concretas en Rust
    para Invocation Values, colecciones, slices o handles de tipo permanecen
    diferidas al diseño técnico.

### Ejemplo Conceptual

Dada una Public Function en Evo-Script:

```text
public fn sum(int left, int right) -> int
{
    return left + right;
}
```

El Consumer proporciona:
- Compiled Program que contiene `sum`
- Invocation Values: `[10, 20]`

Binding posicional:
- `InvocationValue[0]` (`10`) $\rightarrow$ enlazado al parámetro `left`
- `InvocationValue[1]` (`20`) $\rightarrow$ enlazado al parámetro `right`

---

## Criterios de Aceptación

1. El Consumer puede proporcionar un Compiled Program válido al Evo-Script
   Engine.
2. El Consumer puede proporcionar cero o más Invocation Values.
3. Execute no recibe Source Text y no realiza compilación de código fuente.
4. El Engine determina los parámetros declarados por la Public Function del
   Compiled Program.
5. Los Invocation Values se emparejan con los parámetros estrictamente por
   posición.
6. La cantidad de Invocation Values debe ser igual a la cantidad de parámetros de
   la Public Function ($N$).
7. Cada Invocation Value debe ser semánticamente compatible con su correspondiente
   tipo de parámetro.
8. El Engine no realiza conversiones implícitas para reparar Invocation Values
    incompatibles.
9. Las Public Functions de cero parámetros requieren cero Invocation Values.
10. Un desajuste de aridad produce un Result fallido.
11. Una incompatibilidad de tipo produce un Result fallido.
12. El Engine ejecuta la Public Function conforme a la semántica de Evo-Script
    v0.
13. Una ejecución exitosa preserva el Value producido por la Public Function en
    el Result exitoso.
14. Una ejecución fallida produce un Result fallido en lugar de tratarse
    silenciosamente como éxito.
15. El Consumer no necesita conocer la arquitectura interna de la VM, intérprete
    o evaluador del Engine.
16. Una vez que se retorna el Result al Consumer, esa invocación de Execute
    concluye.
17. No se requiere ningún objeto explícito de sesión o de contexto de ejecución
    del Engine en la frontera funcional.

---

## Concepto de Outcome de Ejecución (Result)

El outcome funcional de ejecutar un programa compilado se representa
conceptualmente como `Result`:
- **Outcome Exitoso**: Preserva el `Value` producido por el programa.
- **Outcome Fallido**: Representa una falla de ejecución (por ejemplo, desajuste
  de aridad, incompatibilidad de tipo, error de evaluación en runtime).

> [!NOTE]
> `Result` es un concepto de outcome funcional alineado con el modelo compartido
> de Evo (`Result != Value`, `Result != Failure`).
> Las representaciones concretas en Rust, parámetros de tipo, genéricos
> (`Result<T, E>`), estructuras de error y variantes de enum deliberadamente
> **no** se deciden en esta User Story.

---

## No Responsabilidades y Fuera de Alcance

Para el alcance de US-002 y Evo-Script Engine v0:
- Compilación o parseo de Source Text de Evo-Script (cubierto por US-001).
- Carga de Compiled Programs desde almacenamiento físico o sistemas de archivos.
- Interacción con la terminal, formateo o presentación de UI.
- Inicio, detención o gestión del ciclo de vida de aplicaciones de Evo Runtime.
- Parseo de consultas o semántica de ejecución pertenecientes a EvoQ.
- Decisiones de arquitectura interna del motor de ejecución (por ejemplo, stack
  VM, register VM o intérprete tree-walk).
- Efectos laterales externos, salida a consola (`print`), stdout, I/O de sistema
  de archivos, Requesters, Providers o callbacks intermedios durante la
  ejecución.
