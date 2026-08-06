# UC-014 — Interpretar expresiones textuales de `filter`

## Objetivo

Este caso de uso documenta cómo Evo Shell interpreta una etapa `filter` dentro de un pipeline textual y la convierte en una `PipelineOperation::Filter(FilterExpression)` tipada sin ejecutar el filtrado.

Ejemplo canónico:

```text
iter |> filter name equals "file.txt" |> select name |> to-values
```

La ejecución del pipeline pertenece a `UC-011 — Ejecutar un pipeline estructurado`.

UC-014 pertenece completamente a Evo Shell.

Evo Shell Engine no participa como sistema de interpretación textual. Participa únicamente de forma indirecta al aportar los tipos semánticos reutilizados por el modelo estructurado.

## Naturaleza del caso de uso

`parse` continúa siendo la frontera pública de interpretación textual.

`Parser Agent` sigue coordinando la interpretación de tokens.

`Command Resolver` continúa resolviendo comandos simples y puede delegar una secuencia de pipeline a un resolver específico de pipeline.

`Pipeline Resolver` delega la interpretación de la etapa `filter` a un resolver especializado de expresión de filter.

No existe un parser clásico separado.

No existe un AST textual independiente.

La arquitectura se extiende con un resolver de expresión porque existe una transformación real:

```text
tokens estructurados
    ↓
FilterExpression
```

## Modelo conceptual

UC-014 interpreta `filter` como parte del modelo ya existente:

```text
Command::Pipeline(Pipeline)
└── PipelineOperation::Filter(FilterExpression)
```

`FilterExpression` es el modelo tipado existente del engine y no se duplica como AST paralelo.

Expresiones soportadas en esta primera versión:

- `Comparison(FilterComparison)`;
- `And(Vec<FilterExpression>)`;
- `Or(Vec<FilterExpression>)`.

El contrato textual aprobado reutiliza:

- `FilterProperty`;
- `FilterOperator`;
- `FilterValue`;
- `FilterOperand`;
- `FilterExpression`.

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
Filter Expression Resolver
    ↓
FilterExpression
    ↓
PipelineOperation::Filter
    ↓
Command::Pipeline(Pipeline)
```

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
- preserva el orden escrito;
- delega la interpretación textual de `filter` al resolver especializado;
- devuelve `Command::Pipeline(Pipeline)`.

### Filter Expression Resolver

- consume la expresión textual del `filter`;
- reconoce comparaciones, agrupación y operadores lógicos;
- construye `FilterExpression` directamente;
- no ejecuta filtros;
- no toca filesystem;
- no conoce `Shell`.

## Separación entre interpretación y ejecución

UC-014 solo construye estructura.

No evalúa si un archivo cumple la expresión.

No ejecuta `filter`.

No valida la compatibilidad semántica de la expresión con el engine.

La evaluación real pertenece a `UC-004 — Filtrar una iteración estructurada`.

## Error handling

UC-014 debe producir error de interpretación para:

- expresión vacía;
- propiedad desconocida;
- operador desconocido;
- operador faltante;
- valor faltante;
- valor inválido;
- límite superior faltante en `between` / `not-between`;
- paréntesis abierto sin cerrar;
- paréntesis de cierre inesperado;
- mezcla ambigua de `and` y `or`.

Estos errores pertenecen a Evo Shell, no al engine.

## Relación con Tokenize

UC-014 reutiliza el tokenizador existente.

La representación exacta de `(`, `)`, `|>`, `>` y `<` dentro del tokenizador no queda fijada por este caso de uso, pero la arquitectura debe poder distinguirlos como tokens para construir la expresión tipada.

UC-014 no define un lexer nuevo.

## Relación con Evo Shell Engine

Evo Shell no reimplementa:

- evaluación de `filter`;
- operadores comparativos;
- operadores lógicos;
- recorridos de filesystem;
- providers del engine.

UC-014 solo construye la expresión que después consumirá `UC-004`.

## Sintaxis textual básica

La primera versión soporta, entre otros, estos ejemplos:

```text
filter name equals "README.md"
filter type not-equals "directory"
filter index < 10
filter size > 10kb
filter size at-least 50kb
filter size at-most 5mb
filter size between 10kb, 100kb
filter size not-between 10kb, 100kb
filter type equals "file" and size > 10kb
filter (type equals "file" or type equals "directory") and size > 10kb
```

La interpretación conserva la forma escrita de la expresión.

## Compatibilidad

Los comandos simples existentes siguen funcionando sin `filter`.

La interpretación de pipelines básicos sigue funcionando.

## Decisiones diferidas

- evaluación de `filter`;
- `created` y `modified` cuando no existe un literal textual aprobado;
- multilinea;
- subpipelines;
- pipelines como argumentos;
- presentación final;
- aliases sintácticos no aprobados;
- lexer clásico;
- AST textual paralelo.
