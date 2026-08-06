# UC-012 — Interpretar un pipeline textual básico

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta una entrada textual con `|>` y la convierte en un `Command::Pipeline(Pipeline)` estructurado sin ejecutar el pipeline.

Ejemplo canónico:

```text
iter |> take 1 |> select name |> to-value
```

La ejecución del pipeline pertenece a `UC-011 — Ejecutar un pipeline estructurado`.

UC-012 pertenece completamente a Evo Shell.

Evo Shell Engine no participa como sistema de interpretación textual. Participa únicamente de forma indirecta al aportar los tipos semánticos reutilizados por el modelo estructurado.

## Naturaleza del caso de uso

`parse` continúa siendo la frontera pública de interpretación textual.

`Parser Agent` sigue coordinando la interpretación de tokens.

`Command Resolver` continúa resolviendo comandos simples y ahora también puede delegar una secuencia de pipeline a un resolver específico de pipeline.

No existe un parser clásico separado.

No existe un AST textual independiente.

La arquitectura se extiende con un resolver de pipeline porque existe una transformación real:

```text
tokens estructurados
    ↓
Pipeline estructurado
```

## Modelo conceptual

UC-012 introduce conceptualmente una evolución de `Command`:

```text
Command
├── ScopeFs(&str)
├── Iter
├── Enter(&str)
├── Clear(TerminalClearMode)
├── Exit
└── Pipeline(Pipeline)
```

`Pipeline` conserva una secuencia ordenada de operaciones tipadas:

```text
Pipeline
└── operations: ordered PipelineOperation[]
```

Operaciones soportadas en esta primera versión de interpretación:

- `Iter`
- `Index(usize)`
- `Take(usize)`
- `Select(Vec<SelectProperty>)`
- `ToValue`
- `ToValues`
- `ToArgs`

`Filter(FilterExpression)` existe como operación estructurada del dominio del engine, pero su interpretación textual queda fuera de este caso de uso.

## Flujo conceptual

```text
entrada textual
    ↓
Tokenize
    ↓
TokenStream
    ↓
Parser Agent
    ↓
Parse Use Case
    ↓
Command Resolver
    ↓
Pipeline Resolver
    ↓
Command::Pipeline(Pipeline)
```

`Parser Agent` y `Parse Use Case` se reutilizan.

`PipelineExecutor` no participa.

## Responsabilidades

### Parser Agent

- recibe `TokenStream`;
- coordina `Tokenize`;
- delega resolución;
- devuelve `Command`.

### Parse Use Case

- expresa la capacidad pública de interpretación textual;
- conserva la firma conceptual existente;
- devuelve un comando ya resuelto.

### Command Resolver

- reconoce comandos simples existentes;
- detecta cuando la entrada contiene una secuencia de pipeline;
- delega la construcción del pipeline al resolver correspondiente;
- no ejecuta el pipeline;
- no duplica la semántica de `PipelineExecutor`.

### Pipeline Resolver

- consume tokens ya producidos por la infraestructura existente;
- reconoce etapas separadas por `|>`;
- construye `PipelineOperation` tipadas;
- construye `Pipeline`;
- preserva el orden escrito;
- devuelve `Command::Pipeline(Pipeline)`.

## Separación entre interpretación y ejecución

UC-012 solo construye estructura.

No valida compatibilidad de tipos entre etapas.

No ejecuta `Iter`, `Take`, `Select`, `ToValue`, `ToValues` ni `ToArgs`.

La compatibilidad semántica y la ejecución pertenecen a `UC-011`.

Ejemplo:

```text
iter |> to-value
```

puede convertirse en un `Command::Pipeline(Pipeline [...])` aunque después `PipelineExecutor` lo rechace como transición inválida.

## Error handling

UC-012 debe producir error de interpretación para:

- etapa vacía;
- separador final sin etapa siguiente;
- operación desconocida;
- argumento faltante;
- argumento inválido;
- lista vacía de propiedades en `select`;
- argumento adicional en `to-value`, `to-values` o `to-args`.

Estos errores pertenecen a Evo Shell, no al engine.

## Relación con Tokenize

UC-012 reutiliza el tokenizador existente.

La representación exacta de `|>` dentro del tokenizador no queda fijada por este caso de uso, pero la arquitectura debe poder distinguirlo como separador de etapas para construir el pipeline estructurado.

UC-012 no define un lexer nuevo.

## Relación con Evo Shell Engine

Evo Shell no reimplementa:

- `iter`;
- `filter`;
- `index`;
- `take`;
- `select`;
- `to-value`;
- `to-values`;
- `to-args`.

UC-012 solo construye el `Pipeline` que después consumirá `UC-011`.

Tipos públicos del engine reutilizados por esta historia:

- `SelectProperty`;
- `FilterExpression` como capacidad estructural existente del modelo de pipeline.

## Pipeline textual básico

La primera versión soporta la secuencia:

```text
iter |> take 1 |> select name |> to-value
```

El orden se conserva exactamente como fue escrito.

La historia no inserta etapas implícitas.

La historia no reordena operaciones.

La historia no transforma operaciones en texto serializado.

## Compatibilidad

Los comandos simples existentes siguen funcionando sin `|>`:

- `scope-fs`
- `iter`
- `enter`
- `clear`
- `exit`

Si la entrada no contiene pipeline, la resolución sigue el camino normal de comando simple.

## Decisiones diferidas

- `filter` textual;
- operadores de `filter`;
- agrupación con paréntesis;
- subpipelines;
- pipelines como argumentos;
- lectura multilinea;
- continuation prompt;
- validación de compatibilidad de etapas;
- mensajes finales de presentación de errores;
- representación exacta en Rust de `PipelineResolver` y `ParseError` ampliado;
- representación exacta del token correspondiente a `|>`.

## Tests

La implementación futura debe cubrir:

- interpretación de `iter |> take 1 |> select name |> to-value`;
- interpretación de `iter |> select name, size |> to-values`;
- interpretación de `iter |> select name |> to-args`;
- rechazo de etapas vacías;
- rechazo de operación desconocida;
- rechazo de argumentos faltantes;
- rechazo de argumentos inválidos;
- preservación del orden de etapas;
- preservación de comandos simples existentes;
- ausencia de ejecución del pipeline durante el parse;
- ausencia de `PipelineExecutor` en esta frontera.

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
