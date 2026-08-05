# UC-009 — Iniciar Evo Shell con una presentación de bienvenida

## Objetivo

Este caso de uso documenta el inicio compuesto de Evo Shell según:

[US-009 — Iniciar Evo Shell con una presentación de bienvenida](../../../functional_documentation/user_stories/US-009-start-shell-with-welcome.md)

UC-009 pertenece completamente a Evo Shell.

Evo Shell Engine no participa directamente porque el startup de bienvenida es una composición de use cases de shell.

## Naturaleza del caso de uso

`starter` es un caso de uso compuesto.

No tiene resolver propio.

No tiene provider propio.

Su responsabilidad es coordinar otros use cases existentes de Evo Shell.

La composición conceptual es:

```text
Start
├── InitializeShell
├── TerminalClearer
└── WelcomePresenter
```

Starter consume function pointers de esos use cases.

No depende conceptualmente de agentes concretos como mecanismo de acoplamiento interno.

Los agentes concretos implementan los use cases.

## Orden de startup

El orden aprobado es:

```text
1. InitializeShell
2. TerminalClearer(Visible)
3. WelcomePresenter
4. return Shell
```

Si `InitializeShell` falla:

- no se limpia la terminal;
- no se muestra la bienvenida;
- startup falla.

Si `TerminalClearer` falla:

- no se muestra la bienvenida;
- startup falla.

Si `WelcomePresenter` falla:

- startup falla.

Solo después de completar las tres operaciones el use case devuelve `Shell`.

## Frontera técnica

La frontera de UC-009 se apoya en tres capacidades internas de Evo Shell:

- `InitializeShell` construye un `Shell` válido;
- `TerminalClearer` limpia la pantalla visible;
- `WelcomePresenter` presenta la bienvenida.

Starter orquesta esas capacidades y luego deja el control al loop normal de la shell.

## Welcome Presenter

La bienvenida es una capacidad propia de presentación.

Por eso se modela con su propia cadena de capas:

```text
welcome_presenter Agent
  ↓
Resolve function pointer
  ↓
welcome_presenter Resolver
  ↓
Provide function pointer
  ↓
welcome_presenter Provider
  ↓
stdout / terminal
```

La bienvenida mostrada es:

```text
CatarinaSoft
evo-shell {version}
evo-shell is a life :)
```

La versión procede de la versión de paquete de Evo Shell.

No se usa ASCII art.

No se usan emojis Unicode.

## Startup y loop

Flujo conceptual:

```text
main
  ↓
starter::start
  ↓
Shell
  ↓
run_loop
  ↓
prompt
```

Starter devuelve `Shell` ya inicializado, con la pantalla visible limpia y la bienvenida ya presentada.

`main` no limpia la terminal directamente.

`main` no imprime la bienvenida directamente.

`main` no obtiene la versión directamente.

## Error handling

UC-009 debe preservar la causa original del fallo cuando:

- falla `InitializeShell`;
- falla `TerminalClearer`;
- falla `WelcomePresenter`.

No se oculta el error fuente con cadenas genéricas.

## Tests

La implementación debe cubrir:

- orden initialize -> clear visible -> welcome;
- clear recibe `TerminalClearMode::Visible`;
- welcome se ejecuta después de clear;
- startup devuelve `Shell`;
- si initialize falla, clear no se ejecuta;
- si clear falla, welcome no se ejecuta;
- si welcome falla, startup falla;
- welcome presenter escribe las tres líneas esperadas;
- la versión corresponde a `env!("CARGO_PKG_VERSION")`;
- no se usa terminal real en tests unitarios;
- `clear --all` no se usa en startup.

## Fuera de alcance

- opciones de inicio;
- `--no-banner`;
- detección de capacidades de terminal;
- animaciones;
- retrasos;
- `sleep`;
- sonidos;
- archivos de configuración;
- banner del entorno;
- cambios en Evo Shell Engine.

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
