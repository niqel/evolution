# US-003 — Execute Evo-Script Source

Status: FUNCTIONAL CLOSED

## Historia

```text
Como Consumer,
quiero proporcionar el Source Text completo de un programa Evo-Script
y los Invocation Values requeridos por su Public Function
al Evo-Script Engine,
para que el programa fuente sea ejecutado conforme a
Evo-Script Language Specification v0 y reciba su Result.
```

---

## Contexto

El Evo-Script Engine es el componente responsable de compilar y ejecutar
programas Evo-Script.

Bajo **US-003 (Execute Source)**, el Consumer proporciona el **Source Text
completo** de un programa Evo-Script y los **Invocation Values** ordenados
(0..N) directamente al Engine, ejecutando el programa en una única operación
pública sin requerir un paso previo separado de Compile ni la gestión externa de
un Compiled Program.

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
│  Execute Source (especificación v0)    │
│  Enlaza Invocation Values a parámetros │
│  Ejecuta la única Public Function      │
└──────────────────┬─────────────────────┘
                   │
                   │ outcome de ejecución
                   ▼
                 Result
```

### Distinciones de Input en la Frontera
- **Source Text != File Path**: El Engine no realiza I/O de archivos ni
  resolución de rutas; la lectura del archivo físico `.efn` es responsabilidad
  del Consumer o de un cargador externo.
- **Source Text != AST / Token Stream**: El Consumer entrega texto plano, no un
  árbol sintáctico abstracto o flujo de tokens preprocesado.
- **Source Text != Individual Function**: El input es el Source Text completo y
  autocontenido de la unidad de programa.
- **Source Text != Compiled Program**: El Consumer proporciona código fuente;
  Execute Source no requiere ni espera un artefacto previamente compilado.
- **Invocation Values != Command-Line Strings**: El Engine recibe Values
  estructurados, no cadenas de texto de terminal que requieran parseo implícito.
- **Sin Compile Previo Requerido**: El Consumer no está obligado a llamar a
  Compile ni a gestionar representaciones intermedias antes de invocar Execute
  Source.

---

## Semántica Funcional de Execute Source

1. **Ejecución Directa**: Execute Source acepta Source Text e Invocation Values,
   procesa el programa conforme a
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md),
   ejecuta su única Public Function y retorna `Result`.
2. **Manejo de Source Inválido**: Si el Source Text viola las reglas léxicas,
   sintácticas o semánticas de Evo-Script v0, Execute Source produce un `Result`
   fallido sin ejecutar la Public Function.
3. **Sin Retorno de Compiled Program**: Execute Source no retorna un Compiled
   Program como parte de su contrato funcional público; su único outcome público
   es `Result`.
4. **Sin Persistencia ni Caché**: Execute Source no guarda, no registra, no
   persiste ni cachea un Compiled Program como responsabilidad funcional pública.
5. **Estrategia Interna Abierta**: La estrategia de implementación interna de
   Execute Source (por ejemplo, interpretación directa tree-walk, compilación a
   bytecode temporal o evaluación de representaciones intermedias) permanece
   intencionalmente abierta y no está restringida por esta User Story.

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
- Source Text que contiene `sum`
- Invocation Values: `[10, 20]`

Binding posicional:
- `InvocationValue[0]` (`10`) $\rightarrow$ enlazado al parámetro `left`
- `InvocationValue[1]` (`20`) $\rightarrow$ enlazado al parámetro `right`

---

## Criterios de Aceptación

1. El Consumer puede proporcionar el Source Text completo de un programa
   Evo-Script v0 al Evo-Script Engine.
2. El Consumer puede proporcionar cero o más Invocation Values.
3. El Consumer no está obligado a invocar Compile ni a obtener un Compiled
   Program antes de invocar Execute Source.
4. El Engine trata el Source Text como un programa Evo-Script completo y lo
   procesa conforme a:
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md).
5. Si el Source Text viola las reglas léxicas, sintácticas o semánticas de
   Evo-Script v0, Execute Source produce un Result fallido sin ejecutar la
   Public Function.
6. El Engine determina los parámetros declarados por la Public Function.
7. Los Invocation Values se emparejan con los parámetros estrictamente por
   posición.
8. La cantidad de Invocation Values debe ser igual a la cantidad de parámetros de
   la Public Function ($N$).
9. Cada Invocation Value debe ser semánticamente compatible con su correspondiente
   tipo de parámetro.
10. El Engine no realiza conversiones implícitas para reparar Invocation Values
    incompatibles.
11. Las Public Functions de cero parámetros requieren cero Invocation Values.
12. Un desajuste de aridad produce un Result fallido.
13. Una incompatibilidad de tipo produce un Result fallido.
14. El Engine ejecuta la Public Function conforme a la semántica de Evo-Script
    v0.
15. Una ejecución exitosa preserva el Value producido por la Public Function en
    el Result exitoso.
16. Un procesamiento o ejecución fallida produce un Result fallido en lugar de
    tratarse silenciosamente como éxito.
17. Execute Source no retorna un Compiled Program como parte de su contrato
    público.
18. Execute Source no persiste, no escribe en disco ni cachea un Compiled
    Program como responsabilidad funcional pública.
19. El Consumer no necesita conocer la arquitectura interna de procesamiento,
    AST, VM o intérprete del Engine.
20. Una vez que se retorna el Result al Consumer, esa invocación de Execute
    Source concluye.
21. No se requiere ningún objeto explícito de sesión o de contexto de ejecución
    del Engine en la frontera funcional.

---

## Concepto de Outcome de Ejecución (Result)

El outcome funcional de ejecutar código fuente directamente se representa
conceptualmente como `Result`:
- **Outcome Exitoso**: Preserva el `Value` producido por el programa.
- **Outcome Fallido**: Representa una falla de ejecución (por ejemplo, error
  léxico/sintáctico, error semántico, desajuste de aridad, incompatibilidad de
  tipo o error de evaluación en runtime).

> [!NOTE]
> `Result` es un concepto de outcome funcional alineado con el modelo compartido
> de Evo (`Result != Value`, `Result != Failure`).
> Las representaciones concretas en Rust, parámetros de tipo, genéricos
> (`Result<T, E>`), estructuras de error y variantes de enum deliberadamente
> **no** se deciden en esta User Story.

---

## No Responsabilidades y Fuera de Alcance

Para el alcance de US-003 y Evo-Script Engine v0:
- Lectura de archivos `.efn` del sistema de archivos o resolución de rutas.
- Producir o retornar un Compiled Program (cubierto por US-001).
- Persistencia, almacenamiento en caché o serialización de artefactos
  compilados.
- Interacción con la terminal, formateo o presentación de UI.
- Inicio, detención o gestión del ciclo de vida de aplicaciones de Evo Runtime.
- Parseo de consultas o semántica de ejecución pertenecientes a EvoQ.
- Decisiones de arquitectura interna del motor de ejecución (por ejemplo,
  intérprete AST vs VM de bytecode temporal).
- Efectos laterales externos, salida a consola (`print`), stdout, I/O de sistema
  de archivos, Requesters, Providers o callbacks intermedios durante la
  ejecución.
