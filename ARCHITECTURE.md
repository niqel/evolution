# Arquitectura del Proyecto Evolution

Este documento especifica la arquitectura del proyecto **Evolution**, definiendo la topología de ejecución alojada en `evo-runtime`, los límites conceptuales entre los proyectos/crates (`evo-script`, `evo-shell`, `evo-runtime`, `providers`, `evo-shell-cli`, `evo-ui`, `evo-apps`), el modelo de aplicaciones `.evo` y la organización interna del motor semántico de capacidades `evo-shell`.

---

## 1. Topología de Ejecución vs Dependencias de Crates

Para evitar ambigüedades arquitectónicas, este documento distingue formalmente entre:

1. **Topología de Ejecución**: Quién aloja, mantiene el ciclo de vida y ejecuta a quién en tiempo de ejecución.
2. **Dependencias de Código / Crates**: Qué módulo conoce o depende conceptualmente del código de cuál durante la compilación.

---

## 2. Topología de Ejecución (`evo-runtime` como Execution Host)

`evo-runtime` **no es una biblioteca o servicio externo llamado por las aplicaciones**, ni tampoco un paso lineal posterior en un pipeline. `evo-runtime` es el **entorno de ejecución (*Execution Host*)** que rodea y mantiene viva la ejecución de una aplicación Evolution.

### Diagrama de Topología de Ejecución por Aplicación

```text
                    evo-runtime
                 execution host
                       │
                   app.evo
                       │
                       ▼
                  evo-script
        ┌─────────────────────────────┐
        │ language                    │
        │ syntax                      │
        │ types                       │
        │ operators (+ - * / %)       │
        │ expressions                 │
        │ filter / select / new       │
        │ to-value                    │
        │ pipes (|>)                  │
        │ lazy iteration semantics    │
        └─────────────────────────────┘
                       │
                       │ uses capabilities
                       ▼
                   evo-shell
        ┌─────────────────────────────┐
        │ scope                       │
        │ filesystem                  │
        │ terminal                    │
        │ copy / move / rename        │
        │ delete / trash              │
        │ processes                   │
        │ network                     │
        │ system capabilities         │
        └─────────────────────────────┘
                       │
                       ▼
                   Providers
```

> *Nota:* Este diagrama representa responsabilidades conceptuales dentro del entorno mantenido por el runtime. No significa que `evo-runtime` contenga físicamente el código de la UI, CLI, `evo-script` o `evo-shell`, sino que **aloja y mantiene el contexto** en el cual esas piezas colaboran.

---

## 3. Instalación Compartida de Runtime vs Procesos de Aplicación

Evolution utiliza una estrategia de instalación de runtime compartida combinada con aislamiento de ejecución por aplicación.

```text
               shared evo-runtime installation
                         │
              host / supervisor
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
     execution A     execution B     execution C
          │              │              │
      shell.evo       music.evo       ui.evo
```

### Principios de Aislamiento e Instalación:
1. **Un solo `evo-runtime` instalado**: Existe una única instalación/implementación compartida del runtime en el sistema. Las aplicaciones `.evo` reutilizan esa instalación común y **no empaquetan una copia completa del runtime**.
2. **Aislamiento por proceso**: Reutilizar una instalación común de `evo-runtime` **no significa** ejecutar todas las aplicaciones en el mismo proceso del sistema operativo ni compartir estado entre ellas. Cada aplicación dispone de su propia ejecución y proceso aislado.
3. **Host / Supervisor**: `evo-runtime` incluye conceptualmente una faceta de host/supervisor encargada de descubrir aplicaciones, gestionar su ciclo de vida, lanzar instancias aisladas y resolver capacidades. *(Su topología física concreta —daemon, servicio de sistema, proceso residente o launcher— se diseñará posteriormente).*

---

## 4. Responsabilidades de `evo-runtime` como Host

`evo-runtime` no es simplemente una API consumida externamente; es el entorno que mantiene el contexto de vida de una aplicación Evolution.

### Responsabilidades Conceptuales del Host Runtime:
- Iniciar la ejecución de un script o paquete `.evo`.
- Crear, alojar y mantener el contexto de ejecución aislado de la aplicación.
- Gestionar el ciclo de vida (*lifecycle*) de la aplicación.
- Proporcionar acceso y resolver capacidades de infraestructura bajo demanda.
- Mantener el aislamiento estricto respecto a otras aplicaciones en ejecución.
- Finalizar y liberar el contexto cuando la aplicación termina.

### Lo que `evo-runtime` NO implementa (Principio *No God Runtime*):
- NO es responsable de gramática, sintaxis, parsing ni reglas de `evo-script`.
- NO implementa la lógica funcional propia de la aplicación.
- NO implementa las capacidades semánticas de `evo-shell` (ej. no realiza directamente operaciones de filesystem o procesos).
- NO implementa interfaces físicas (UI, terminal) ni Providers de infraestructura física.

---

## 5. Modelo de Aplicaciones `.evo`

Un archivo `.evo` representa código/script ejecutable interpretado por `evo-script` y ejecutado dentro del contexto proporcionado por `evo-runtime`.

### Topología de Alojamiento:
```text
OS / launcher → evo-runtime → isolated application execution → app.evo
```

### Estructura de Aplicación:
- Una aplicación puede ser un script trivial (ej. `script.evo`) o un paquete de aplicación con múltiples scripts y recursos (ej. directorio con `app.evo`, `player.evo`, `resources/`).
- **El caso de `evo-shell-cli.evo`**: La consola interactiva de Evolution se define como una aplicación Evolution (`evo-shell-cli.evo`) alojada por `evo-runtime`, que hace uso de la superficie CLI, interpreta `evo-script` y ejecuta capacidades semánticas mediante `evo-shell`.

---

## 6. Lógica Funcional Única y Superficies UI / CLI / AI

Dentro de una aplicación ejecutándose bajo `evo-runtime`, la interfaz gráfica (UI) y la interfaz de consola (CLI) pertenecen a la superficie de interacción de la aplicación.

```text
                 evo-runtime
                      │
                 music.evo
                      │
             shared app logic
                  evo-script
                 /         \
                /           \
              UI            CLI / AI
              │              │
           [Play]      music play ...
              │              │
              └──────┬───────┘
                     ▼
             same capability
```

### Principios de Superficie:
1. **Misma Lógica Funcional**: La UI gráfica y la CLI textual no tienen implementaciones de negocio separadas; ambas invocan exactamente la misma capacidad funcional expresada en `evo-script` y provista por `evo-shell`.
2. **Orientación a Automatización y Agentes de IA**: Exponer capacidades funcionales en superficies textuales/scriptables permite que usuarios humanos, scripts y **agentes de IA** invoquen comandos deterministas.
   - *Principio:* *"Visual interaction is a surface; functional capability remains scriptable."*

---

## 7. Resolución Dinámica de Capacidades (*Capabilities On-Demand*)

El runtime resuelve y proporciona las capacidades necesarias a la aplicación de forma dinámica según su necesidad.

### Flujo de Capacidades Semánticas y de Infraestructura:

**Operaciones del Sistema / Entorno (ej. Copia):**
```text
CopyTo Use Case → Copier Agent → Copy Resolver → Copy Contract → Copy Provider
```
*(El Resolver cruza la frontera técnica con la infraestructura externa implementada por el Provider).*

**Operaciones Puras Internas de Dominio en evo-shell:**
```text
Use Case → Agent → Collaborator → (optional pure Tools)
```
*(El Use Case define la firma y su propio tipo Error. El Agent coordina. El Collaborator ejecuta la lógica).*

> **Nota de Separación Arquitectónica:**
> Las responsabilidades de lenguaje relativas a operadores aritméticos (`+`, `-`, `*`, `/`, `%`), expresiones sintácticas y tipos numéricos de la arquitectura anterior han sido completamente removidas de `evo-shell`. Pertenecen exclusivamente a `evo-script`.

- **Principio *Capability Unavailable*:** Si una capacidad requerida no está disponible en el entorno (ej. ausencia de terminal en un entorno puramente gráfico), la solicitud responde con un error semántico de indisponibilidad sin invalidar el entorno global de `evo-runtime`.
- **Ownership de Estado Externo**: Las aplicaciones y el runtime no duplican persistentemente el estado visual que pertenece a la infraestructura física (ej. la terminal física mantiene su propio render/scrollback).

---

## 8. Dependencias de Código y Crates

```text
evo-shell-cli ─────┐
                   │
evo-ui ────────────┼──► evo-runtime
                   │
future frontend ───┘

evo-runtime ─────────► evo-script
evo-runtime ─────────► evo-shell public API
```

> *Aclaración:* Una flecha de dependencia de crate representa una dependencia de código/compilación, **no que se envíe una petición de red o IPC en tiempo de ejecución**.

### Prohibiciones de Dependencias de Crates:
- `evo-runtime → frontends`: PROHIBIDO.
- `evo-shell → evo-script`: PROHIBIDO.
- `evo-shell → evo-runtime`: PROHIBIDO.

---

## 9. Visión Futura: `evo-apps` (Repositorio / Launcher / Store)

> *Nota:* Esta sección define una visión arquitectónica futura y **no representa código ni módulos actualmente implementados**.

`evo-apps` conceptualiza el sistema de catálogo, launcher, instalación, actualización y descubrimiento de aplicaciones Evolution.

```text
evo-apps (UI / CLI) → repository / store → descarga scripts .evo → ejecución en evo-runtime
```

- **Distribución liviana**: Distribuye scripts `.evo`, recursos y metadatos sin duplicar el runtime.
- **Superficie dual UI / CLI para IA**: Posee interfaz gráfica y superficie textual equivalente (ej. `evo-apps install ...`), permitiendo que agentes de IA busquen e instalen aplicaciones Evolution de forma scriptable y determinista.

---

## 10. Portabilidad y Principios Mantenedores

- **Portabilidad OS**: La semántica de las aplicaciones `.evo`, de `evo-script` y de `evo-shell` es idéntica entre Linux y Windows. Las diferencias específicas corresponden a los hosts y Providers concretos (`Linux Provider` / `Windows Provider`).
- **Diseño Orientado a Funciones**: Uso de funciones puras, punteros de función (`fn`), enums y structs de datos. Se eliminan clases de servicio, administradores (*managers*), objetos stateful y despacho dinámico (`dyn`).
- **Use Case es dueño de su resultado semántico**: Cada Use Case define su firma (`fn`) y su propio tipo `Error` cuando la acción puede fallar. No existen tipos de error globales compartidos entre Use Cases independientes.
- **Agent = Orchestration Only**: Un Agent coordina, encadena pasos, pasa argumentos/capabilities y propaga `Result`. Un Agent **NO** valida datos, implementa reglas matemáticas/dominio, ejecuta operaciones internas ni interpreta errores técnicos.
- **Semántica de Collaborator y Nombres de Sujeto**: Un Collaborator es un sujeto interno que colabora con un Agent. Cuando comparten el mismo sujeto (`agents/<subject>.rs` y `collaborators/<subject>.rs`), el Agent expone la función con el verbo de la acción pública y el Collaborator expone la función `collaborate()`. No se requieren traits ni despacho dinámico.
- **Result no implica Resolver (*Result does not imply Resolver*)**: La existencia de un `Resolver` depende de la presencia de una frontera técnica externa con un `Contract`/`Provider`, **no** de la posibilidad de que una operación devuelva `Err`.
- **Separación entre Lenguaje y Capacidades del Sistema**:
  - `evo-script` es dueño de la sintaxis, tokenización, parsing, expresiones, operadores (`+`, `-`, `*`, `/`, `%`), tipos (`i32`, `f64`, etc.), `filter`, `select`, `new`, `to-value`, tuberías (`|>`) y semántica de iteración lazy (`iter`).
  - `evo-shell` es dueño de las capacidades semánticas de entorno/sistema (`scope`, filesystem, terminal, `copy-to`, `move-to`, `rename`, `delete`, `trash`, procesos, red).
  - Los operadores del lenguaje no se modelan como Use Cases de `evo-shell`.
- **Estrategia de Testing**: Código de producción limpio en `src/` (sin `#[cfg(test)] mod tests`) y verificación externa en la suite `tests/`.
