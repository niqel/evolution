# US-001 — Compile Evo-Script Source

Status: FUNCTIONAL CLOSED

## Historia

```text
Como Consumer,
quiero proporcionar el Source Text completo de un programa Evo-Script
al Evo-Script Engine,
para que el Source Text sea compilado conforme a
Evo-Script Language Specification v0 y reciba un Compiled Program.
```

---

## Contexto

El Evo-Script Engine es el componente responsable de compilar y ejecutar
programas Evo-Script.

En Evo-Script v0, un programa completo está contenido dentro de un único archivo
fuente `.efn` y declara exactamente una función pública (`public fn`), junto con
funciones privadas, structs y enums opcionales locales a ese archivo.

Bajo **US-001 (Compile)**, el Consumer proporciona el **Source Text completo** del
programa directamente al Engine. El Engine procesa y valida el Source Text
conforme a la especificación del lenguaje y produce un **Compiled Program** listo
para su ejecución futura.

```text
Consumer
   │
   │ Source Text completo de Evo-Script
   ▼
┌────────────────────────────────────────┐
│ Evo-Script Engine                      │
│                                        │
│  Procesa y valida Source Text          │
│  Compila según especificación v0       │
└──────────────────┬─────────────────────┘
                   │
                   │ compilación exitosa
                   ▼
            Compiled Program
```

### Distinciones de Input en la Frontera
- **Source Text != File Path**: El Engine no realiza I/O de archivos ni
  resolución de rutas; la lectura del archivo físico `.efn` es responsabilidad
  del Consumer o de un cargador externo.
- **Source Text != AST / Token Stream**: El Consumer entrega texto plano, no un
  árbol sintáctico intermedio o preprocesado.
- **Source Text != Individual Function**: El input es el Source Text completo y
  autocontenido de la unidad de programa.

---

## Concepto de Compiled Program

Un **Compiled Program** es la representación ejecutable producida por el Engine de
un programa Evo-Script que ha sido procesado exitosamente conforme a la
Evo-Script Language Specification v0 y es adecuado para su posterior ejecución por
el Evo-Script Engine.

### Características Conceptuales
- **Compiled Program != Source Text**: Representa un artefacto de compilación ya
  procesado y validado.
- **Formato Abierto**: La representación técnica interna de un Compiled Program
  no está congelada en esta fase funcional (por ejemplo, bytecode, IR, AST
  validado o formato binario permanecen como candidatos técnicos abiertos).
- **Sin Persistencia en el Engine**: El Engine produce el Compiled Program y lo
  entrega al Consumer. Persistir, cachear o escribir el Compiled Program en
  almacenamiento es responsabilidad del Consumer o de componentes externos.

---

## Reglas Funcionales de Compile

1. **Unidad Completa**: El Consumer proporciona el Source Text completo de un
   programa Evo-Script v0.
2. **Conformidad con la Especificación**: El Engine compila el código conforme a
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md).
3. **Outcome en Éxito**: Una compilación exitosa produce un Compiled Program
   válido.
4. **Outcome en Error**: Si el Source Text viola las reglas léxicas, sintácticas
   o semánticas de Evo-Script v0, Compile falla y no produce un Compiled
   Program.
5. **Sin Ejecución**: Compile no ejecuta la Public Function ni evalúa
   expresiones.
6. **Sin Invocation Values**: Compile no acepta ni requiere Invocation Values.
7. **Sin Persistencia de Archivos**: Compile no escribe archivos en disco ni
   administra almacenamiento.

---

## Criterios de Aceptación

1. El Consumer puede proporcionar el Source Text completo de un programa
   Evo-Script v0 al Evo-Script Engine.
2. El Engine trata dicho Source Text como un programa Evo-Script completo.
3. El Engine procesa y compila el programa conforme a:
   [`evo-script/EVO_SCRIPT_SPECIFICATION_v0.md`](../../../../evo-script/EVO_SCRIPT_SPECIFICATION_v0.md).
4. Una compilación exitosa produce un Compiled Program que representa la unidad
   compilada.
5. Si el Source Text contiene errores léxicos, sintácticos o semánticos según
   Evo-Script v0, la compilación falla y no produce un Compiled Program válido.
6. El Consumer no necesita parsear ni preprocesar el Source Text antes de
   proporcionarlo al Engine.
7. El Consumer no necesita conocer la estructura interna del compilador del
   Engine, su AST ni sus representaciones intermedias.
8. Compile no ejecuta la Public Function ni evalúa expresiones del programa.
9. Compile no acepta ni requiere Invocation Values.
10. Compile no persiste, no escribe en disco ni almacena el Compiled Program
    resultante.
11. La invocación Compile se completa tras la producción exitosa de un Compiled
    Program o tras una compilación fallida.

---

## No Responsabilidades y Fuera de Alcance

Para el alcance de US-001:
- Lectura de archivos `.efn` del sistema de archivos o resolución de rutas.
- Ejecución del programa compilado o evaluación de expresiones en runtime
  (cubierto por US-002).
- Aceptación o binding de Invocation Values.
- Persistencia, almacenamiento en caché o serialización de Compiled Programs.
- Interacción con la terminal, formateo o presentación de UI.
- Inicio, detención o gestión del ciclo de vida de aplicaciones de Evo Runtime.
- Parseo de consultas o semántica de ejecución pertenecientes a EvoQ.
- Decisiones de arquitectura interna del compilador (por ejemplo, crates o
  módulos separados para lexer, parser o AST).
