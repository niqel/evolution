# UC-001 — Compilar Source Text de Evo-Script

Status: FUNCTIONAL CLOSED

---

## 1. Propósito

Definir funcionalmente cómo un Consumer solicita al Evo-Script Engine la
compilación de un Source Text completo correspondiente a un programa Evo-Script
v0 para obtener un Compiled Program.

```text
Consumer
    │
    │ Source Text
    ▼
Evo-Script Engine
    │
    │ Compile
    ▼
┌─────────────────────────────┐
│ Compilación Exitosa         │
│             ↓               │
│      Compiled Program       │
└─────────────────────────────┘

o

┌─────────────────────────────┐
│ Compilación Fallida         │
│             ↓               │
│     Failure diagnostic      │
└─────────────────────────────┘
```

---

## 2. Trazabilidad

- **Deriva de**: [`US-001 — Compile Evo-Script Source`](../user-stories/US-001-compile-evo-script-source.md)
- **Utiliza conceptos de**: [`Evo-Script Engine v0 — Functional Data Dictionary`](../DATA_DICTIONARY.md)
- **Aplica normativamente**: [`Evo-Script Language Specification v0`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md)

---

## 3. Actor Principal

- **Actor**: `Consumer`
- **Definición**: Rol externo que ya dispone de un Source Text completo y solicita
  su compilación al Evo-Script Engine. No asume una implementación técnica
  específica (puede ser un CLI, runner, test suite, o entorno anfitrión).

---

## 4. Objetivo del Actor

Obtener un `Compiled Program` válido a partir de un `Source Text` que cumpla
todas las reglas de la `Evo-Script Language Specification v0`.

---

## 5. Trigger

El Use Case se inicia cuando el `Consumer` proporciona exactamente un
`Source Text` completo al `Evo-Script Engine` y solicita la operación `Compile`.

> [!NOTE]
> La frontera funcional comienza cuando el Source Text **ya está disponible**.
> La localización de archivos en disco, apertura o lectura de archivos `.efn`
> ocurren previamente fuera del Engine.

---

## 6. Precondiciones

1. El `Consumer` dispone de exactamente un `Source Text` completo para
   proporcionarlo a la operación `Compile`.

> [!IMPORTANT]
> **No se exige como precondición que el Source Text sea válido**.
> Determinar si el código cumple las reglas léxicas, sintácticas y semánticas es
> precisamente la responsabilidad funcional de `Compile`. Por lo tanto, un
> Source Text inválido constituye una entrada válida a la operación `Compile`.

---

## 7. Inputs Funcionales

- **Input**: Exactamente 1 `Source Text` completo.

### Invariantes del Input
- `Source Text != File Path` (el Engine no realiza resolución de rutas ni lectura
  de archivos).
- `Source Text != AST / Token Stream` (el Consumer entrega texto plano sin
  preprocesar).
- `Source Text != Compiled Program` (es código fuente sin compilar).
- `Source Text != Individual Function` (representa la unidad de compilación
  completa).
- `Compile` **no** recibe `Invocation Values`.

---

## 8. Flujo Principal Exitoso

1. El `Consumer` proporciona exactamente un `Source Text` completo al
   `Evo-Script Engine` y solicita la operación `Compile`.
2. El `Evo-Script Engine` acepta el `Source Text` como candidato a representar
   un `Evo-Script Program` completo.
3. El `Evo-Script Engine` procesa el `Source Text` conforme a
   `Evo-Script Language Specification v0`.
4. El `Evo-Script Engine` determina que el `Source Text` cumple las reglas
   léxicas de Evo-Script v0.
5. El `Evo-Script Engine` determina que el `Source Text` cumple las reglas
   sintácticas de Evo-Script v0.
6. El `Evo-Script Engine` determina que el programa cumple las reglas
   semánticas de Evo-Script v0.
7. El `Evo-Script Engine` produce un `Compiled Program`.
8. El `Compiled Program` queda disponible para el `Consumer` como outcome
   exitoso de `Compile`.
9. La invocación `Compile` concluye exitosamente.

> [!NOTE]
> La validación de reglas léxicas, sintácticas y semánticas representa
> condiciones **funcionales** normativas del lenguaje. No imponen una división
> modular física concreta (como crates o structs separados de `Lexer`, `Parser`
> o `SemanticAnalyzer`).

---

## 9. Flujo de Fallo — Compilation Failure

1. Durante el procesamiento del `Source Text`, el `Evo-Script Engine` detecta que
   alguna regla léxica, sintáctica o semántica de
   `Evo-Script Language Specification v0` no se cumple.
2. La compilación no concluye exitosamente y **no** produce un `Compiled Program`
   exitoso.
3. Se genera información diagnóstica representada funcionalmente mediante
   `Failure`.
4. El `Failure` contiene:
   - `description`: explicación textual obligatoria del fallo.
   - `source line`: número de línea (1-based) cuando el fallo puede asociarse
     a una línea concreta del `Source Text`.
5. La invocación `Compile` concluye como fallida.

---

## 10. Postcondiciones

### Postcondición Exitosa
- Existe un `Compiled Program` válido desde la perspectiva funcional.
- Representa un `Evo-Script Program` procesado y validado según
  `Evo-Script Language Specification v0`.
- Queda disponible para el `Consumer` para su posterior ejecución mediante
  `Execute Compiled`.
- Conserva suficiente relación con el `Source Text` original para permitir la
  identificación de `source line` en fallos de ejecución originados en el
  programa.

### Postcondición Fallida
- **No se produce ningún Compiled Program** accesible públicamente para el
  `Consumer`.
- La información diagnóstica de la falla queda disponible expresada como
  `Failure`.

---

## 11. Invariantes Funcionales y Atomicidad

1. **Atomicidad Funcional Pública**: Desde la perspectiva del `Consumer`, la
   operación `Compile` es atómica. Una compilación exitosa produce un
   `Compiled Program` completo y válido; una compilación fallida no produce
   ningún artefacto compilado público. No existen estados intermedios ni
   programas parcialmente compilados expuestos.
2. **Sin Ejecución**: `Compile` **no** ejecuta la `Public Function`, **no**
   evalúa expresiones y **no** produce un `Result` de ejecución. La presencia
   de una `Public Function` no implica auto-ejecución.
3. **Sin Persistencia**: `Compile` **no** escribe archivos en disco, **no**
   guarda en almacenamiento, **no** gestiona caché ni registra programas en
   tablas del sistema. La retención o persistencia del `Compiled Program`
   corresponde al `Consumer` o componentes externos.
4. **Sin Formato Físico Cerrado**: El formato técnico interno del `Compiled Program`
   (bytecode, AST validado, IR, formato binario) permanece abierto para la fase
   de diseño técnico.
5. **Sin Tipo Técnico de Retorno Fijado**: No se define en esta fase el wrapper
   técnico concreto con el que `Compile` entrega su outcome exitoso o fallido
   (no se introduce `Result`, `CompileResult` ni `Option`).

---

## 12. No Responsabilidades y Fuera de Alcance

- Resolución de rutas de archivos, lectura o escritura en el sistema de archivos.
- Recepción de `Invocation Values` o binding de parámetros.
- Ejecución de código Evo-Script (cubierto por US-002 / US-003).
- Coordinación de ciclo de vida de aplicaciones de Evo Runtime.
- Efectos laterales externos, salida a consola (`print`), stdout, UI, terminal o
  red.
- Requesters, Providers o callbacks de progreso durante la compilación.

---

## 13. Modelo Resumido

```text
UC-001 — Compile

Actor
    Consumer

Trigger
    Consumer solicita Compile con exactamente 1 Source Text

Input
    Source Text (completo)

Procesamiento Funcional
    Evo-Script Language Specification v0
    ├── Reglas léxicas
    ├── Reglas sintácticas
    └── Reglas semánticas

Outcome Exitoso
    Compiled Program (válido y reutilizable)
    └── Preserva relación con Source Text para source line

Outcome Fallido
    No hay Compiled Program público
    +
    Failure diagnostic
    ├── description: obligatoria
    └── source line: 0..1 (1-based, si aplica)

Invariantes Clave
    Compile NO ejecuta la Public Function
    Compile NO recibe Invocation Values
    Compile NO produce Result
    Compile NO persiste archivos
    Compile es públicamente atómico
```
