# US-001 — Compile Evo-Script Source

Status: FUNCTIONAL CLOSED — REVALIDATED

## User Story

```text
Como Consumer,
quiero proporcionar el Source Text completo de un programa Evo-Script
al Evo-Script Engine,
para obtener un Compiled Program conforme a la Evo-Script Specification
que pueda conservar y ejecutar posteriormente.
```

---

## Contexto

`evo-script-engine` es el componente responsable de implementar operativamente la especificación de Evo-Script.

Bajo **US-001 (Compile)**, el Consumer proporciona el **Source Text completo** de una unidad de programa Evo-Script directamente al Engine. El Engine procesa y valida el Source Text conforme a la especificación vigente del lenguaje y produce un **Compiled Program** adecuado para su ejecución posterior.

La estructura válida de una unidad de programa —incluyendo las reglas sobre Public Functions, Private Functions, structs, enums u otras construcciones— pertenece exclusivamente a `evo-script` y no es definida por esta User Story.

```text
Consumer
   │
   │ Source Text completo de Evo-Script
   ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  valida reglas del lenguaje            │
│  compila a bytecode                    │
└──────────────────┬─────────────────────┘
                   │
                   │ compilación exitosa
                   ▼
            Compiled Program
```

### Distinciones de Input en la Frontera

- **Source Text != File Path**: el Engine no realiza I/O de archivos ni resolución de rutas; la lectura del archivo físico `.efn` corresponde al Consumer o a una capacidad externa.
- **Source Text != AST / Token Stream**: el Consumer entrega texto fuente, no una representación interna previamente procesada.
- **Source Text = unidad completa de programa**: el Engine recibe la unidad textual completa definida por la Evo-Script Specification vigente.

---

## Compiled Program

Un **Compiled Program** es el artefacto producido por `Compile` cuando una unidad de programa Evo-Script ha sido procesada exitosamente conforme a la especificación vigente del lenguaje.

### Características Funcionales

- **Compiled Program != Source Text**: es un artefacto ya procesado y validado.
- **Representación ejecutable = Bytecode**: la representación ejecutable del Compiled Program es bytecode de Evo-Script.
- **Representaciones internas permitidas**: tokens, AST u otras estructuras intermedias pueden existir durante la compilación, pero no sustituyen al bytecode como representación ejecutable del Compiled Program.
- **External Symbols**: el Compiled Program puede conservar símbolos externos todavía no vinculados a una implementación concreta.
- **Sin Persistencia en el Engine**: el Engine produce el Compiled Program y lo entrega al Consumer; persistirlo, almacenarlo o cachearlo no es responsabilidad del Engine.

---

## Reglas Funcionales de Compile

1. **Unidad Completa**: el Consumer proporciona el Source Text completo de una unidad de programa Evo-Script válida según la especificación vigente.
2. **Conformidad con la Especificación**: el Engine valida y compila el Source Text conforme a `evo-script`.
3. **Validación del Lenguaje**: `Compile` valida las reglas léxicas, sintácticas y semánticas que pertenecen al lenguaje Evo-Script.
4. **Outcome en Éxito**: una compilación exitosa produce un Compiled Program válido.
5. **Outcome en Error**: si el Source Text viola reglas del lenguaje que deben resolverse durante compilación, `Compile` falla y no produce un Compiled Program válido.
6. **Bytecode**: una compilación exitosa produce bytecode como representación ejecutable del Compiled Program.
7. **External Symbols sin Binding**: los símbolos externos pueden permanecer sin resolver dentro del Compiled Program hasta la fase de ejecución/binding.
8. **Provider Ausente != Compile Failure**: la ausencia de un Provider concreto o de un binding de aplicación no constituye por sí sola un error de compilación cuando el símbolo externo es válido como construcción del lenguaje.
9. **Sin Ejecución**: `Compile` no ejecuta la Public Function ni evalúa el programa.
10. **Sin Invocation Values**: `Compile` no acepta ni requiere Invocation Values.
11. **Sin Filesystem I/O**: `Compile` no lee ni escribe archivos físicos.
12. **Sin Persistencia**: `Compile` no administra almacenamiento, caché ni persistencia del Compiled Program.

---

## Criterios de Aceptación

1. El Consumer puede proporcionar Source Text completo al Evo-Script Engine.
2. El Engine trata dicho Source Text como una unidad completa de programa conforme a la Evo-Script Specification vigente.
3. El Consumer no necesita parsear, tokenizar ni preprocesar el Source Text antes de proporcionarlo al Engine.
4. El Engine valida las reglas léxicas, sintácticas y semánticas del lenguaje que corresponden a compilación.
5. Una compilación exitosa produce un Compiled Program.
6. El Compiled Program utiliza bytecode como representación ejecutable.
7. El Compiled Program puede conservar External Symbols todavía no vinculados.
8. La ausencia de un Provider o binding concreto no invalida por sí misma la compilación de un símbolo externo válido.
9. Si el Source Text viola reglas del lenguaje que deben resolverse durante Compile, la compilación falla y no produce un Compiled Program válido.
10. `Compile` no ejecuta el programa.
11. `Compile` no acepta Invocation Values.
12. `Compile` no realiza filesystem I/O.
13. `Compile` no persiste ni cachea el Compiled Program resultante.
14. La operación termina con un Compiled Program válido o con un Compilation Failure.

---

## Public Entry Point y Regla del Lenguaje

La cantidad y semántica de las Public Functions de una unidad Evo-Script pertenecen a la Evo-Script Specification y no a esta User Story.

Las Public Capabilities actuales de `evo-script-engine` asumen que, al momento de ejecutar un Compiled Program o Source Text, la especificación vigente proporciona una forma inequívoca de determinar qué entry point público debe ejecutarse.

Si una versión futura de Evo-Script permite múltiples Public Functions seleccionables externamente dentro de una misma unidad de programa, deberá revisarse la frontera funcional de `Execute Compiled` y `Execute Source` para definir cómo el Consumer selecciona el entry point. Esa evolución no modifica la responsabilidad de `Compile`: compilar la unidad completa conforme a la especificación del lenguaje.

---

## Non-Responsibilities

Para el alcance de US-001 quedan fuera:

- definir cuántas Public Functions puede declarar una unidad Evo-Script;
- definir la gramática o semántica del lenguaje;
- leer archivos `.efn` del filesystem o resolver rutas;
- ejecutar el programa compilado;
- aceptar o enlazar Invocation Values;
- resolver Providers concretos durante Compile;
- persistir, almacenar en caché o serializar Compiled Programs;
- interactuar con terminal, UI o HTTP;
- gestionar el ciclo de vida de una Evo Application;
- definir la arquitectura interna concreta de lexer, parser, AST, compiler o VM.

---

## Closure

US-001 ha sido revalidada contra `Purpose` y `Public Capabilities` actuales y se considera `FUNCTIONAL CLOSED`.

La User Story define únicamente la necesidad funcional de `Compile`; las estructuras internas necesarias para implementarla se definirán posteriormente en el Functional Data Dictionary y en el Technical Design según corresponda.
