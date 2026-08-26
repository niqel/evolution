# Evo-Script Engine — User Stories

Status: FUNCTIONAL CLOSED

Este directorio contiene las User Stories funcionales canónicas para
`evo-script-engine` v0.

En la versión v0, las responsabilidades funcionales públicas del Evo-Script Engine
están completamente definidas por **exactamente tres User Stories**:

1. **US-001 (Compile)**: Compilar el Source Text completo de un programa
   Evo-Script en un Compiled Program.
2. **US-002 (Execute Compiled)**: Ejecutar un Compiled Program con Invocation
   Values para producir un Result.
3. **US-003 (Execute Source)**: Ejecutar directamente el Source Text completo de
   un programa Evo-Script con Invocation Values para producir un Result.

No existen User Stories funcionales adicionales en v0.

---

## Relación Conceptual de Ejecución

Las relaciones funcionales a través de las operaciones de compilación y ejecución
se estructuran de la siguiente manera:

```text
                     Source Text completo
                     /                  \
                    /                    \
                   ▼                      ▼
             Compile (US-001)       Execute Source (US-003)
                   │                (+ Invocation Values)
                   ▼                      │
            Compiled Program              ▼
                   │                    Result
                   │ + Invocation Values
                   ▼
         Execute Compiled (US-002)
                   │
                   ▼
                 Result
```

### Distinciones Funcionales Clave
- **Compile y Execute Source son operaciones distintas**: `Compile` produce un
  `Compiled Program` sin ejecutar; `Execute Source` ejecuta el Source Text
  directamente y produce un `Result`.
- **Compile no ejecuta automáticamente**: La compilación genera una unidad
  compilada para su ejecución posterior.
- **Execute Source no requiere una llamada previa a Compile**: El Consumer
  proporciona el Source Text directamente sin necesidad de gestionar artefactos
  intermedios de compilación.
- **Execute Compiled opera sobre un Compiled Program**: Evalúa una unidad de
  programa ya compilada y no recibe Source Text.
- **Execute Source opera sobre Source Text**: Evalúa Source Text directamente y
  no recibe un Compiled Program.
- **Retención y Reejecución**: Un `Compiled Program` producido por `Compile` puede
  ser conservado externamente por el Consumer y ejecutarse múltiples veces a
  través de `Execute Compiled`.

---

## Catálogo

| ID | Título | Estado |
| --- | --- | --- |
| [US-001](US-001-compile-evo-script-source.md) | Compile Evo-Script Source | FUNCTIONAL CLOSED |
| [US-002](US-002-execute-compiled-evo-script-program.md) | Execute Compiled Evo-Script Program | FUNCTIONAL CLOSED |
| [US-003](US-003-execute-evo-script-source.md) | Execute Evo-Script Source | FUNCTIONAL CLOSED |
