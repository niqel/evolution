# UC-011 — Ejecutar un pipeline estructurado

## Objetivo

Este caso de uso documenta cómo Evo Shell ejecuta un pipeline ya estructurado:

```text
Iter
Take(1)
Select(Name)
ToValue
```

según:

[US-011 — Ejecutar un pipeline estructurado](../../../functional_documentation/user_stories/US-011-execute-structured-pipeline.md)

UC-011 pertenece completamente a Evo Shell.

Evo Shell Engine no participa como sistema de coordinación: participa como proveedor de semántica estructurada para cada operación.

## Naturaleza del caso de uso

`pipeline_executor` es un caso de uso compuesto.

No tiene resolver propio.

No tiene provider propio.

Su responsabilidad es coordinar el flujo entre operaciones estructuradas y delegar la semántica real a Evo Shell Engine.

La composición conceptual es:

```text
Pipeline
└── PipelineOperation[]

ExecutePipeline
├── Iter
├── Filter
├── Index
├── Take
├── Select
├── ToValue
├── ToValues
└── ToArgs
```

El ejecutor consume tipos estructurados y conserva el estado intermedio tipado.

## Modelo conceptual

UC-011 introduce un modelo estructurado del pipeline:

```text
Pipeline
└── operations: ordered PipelineOperation[]
```

Operaciones iniciales aprobadas:

- `Iter`
- `Filter(FilterExpression)`
- `Index(usize)`
- `Take(usize)`
- `Select(Vec<SelectProperty>)`
- `ToValue`
- `ToValues`
- `ToArgs`

La historia no convierte operaciones en strings.

La historia no define un AST textual.

La historia no fija la representación exacta en Rust.

## Estado intermedio del pipeline

El ejecutor necesita conservar explícitamente el estado que produce cada operación.

Conceptualmente, ese estado puede representarse como:

```text
PipelineValue
├── StructuredItems
├── StructuredProjection
├── ProjectedValue
├── Values
└── Arguments
```

Las operaciones de filas trabajan sobre `StructuredItems`:

- `Iter`
- `Filter`
- `Index`
- `Take`

`Select` transforma `StructuredItems` en `StructuredProjection`.

Las operaciones de conversión trabajan sobre `StructuredProjection`:

- `ToValue`
- `ToValues`
- `ToArgs`

La validación de compatibilidad entre etapas debe ser explícita.

## Relación con Evo Shell Engine

UC-011 no duplica la semántica de:

- `iter`
- `filter`
- `index`
- `take`
- `select`
- `to-value`
- `to-values`
- `to-args`

Evo Shell coordina esas capacidades mediante los use cases de frontera del engine.

El engine conserva la semántica estructurada.

Evo Shell conserva la orquestación del flujo.

Para la primera operación del pipeline, `Iter` necesita acceso al scope activo de la shell.
`PipelineExecutor` actúa como puente de coordinación entre ese contexto de shell y el use case del engine.

## Pipeline Executor

El agent conceptual es:

```text
pipeline_executor
```

expresando la acción:

```text
execute
```

Responsabilidades:

1. recibir un `Pipeline` estructurado;
2. conservar el estado intermedio;
3. ejecutar las operaciones en orden;
4. delegar cada operación al use case correspondiente del engine;
5. propagar el resultado final;
6. detenerse ante el primer error;
7. rechazar transiciones incompatibles entre tipos intermedios.

## Flujo

El pipeline principal aprobado para esta primera integración conceptual es:

```text
Iter
Take(1)
Select(Name)
ToValue
```

Flujo conceptual:

```text
Iter
  ↓ StructuredItems
Take(1)
  ↓ StructuredItems
Select(Name)
  ↓ StructuredProjection
ToValue
  ↓ ProjectedValue
```

UC-011 también debe poder representar:

```text
Iter
Take(10)
Select(Name)
ToValues
```

y:

```text
Iter
Select(Name)
ToArgs
```

## Error handling

UC-011 debe fallar si una operación no puede consumirse desde el estado intermedio actual.

Ejemplos conceptuales:

- `ToValue` sin `StructuredProjection` previa;
- `Select` sin `StructuredItems` previos;
- `Iter` seguido por `ToValue` sin transición intermedia válida.

Los errores producidos por el engine se propagan.

Los errores de compatibilidad entre etapas también son explícitos.

La ejecución sigue fail-fast:

si una etapa falla, las etapas posteriores no se ejecutan.

## Relación con el startup

UC-011 no cambia el comportamiento de `starter`.

Starter sigue siendo un precedente arquitectónico de un agent que coordina use cases sin resolver propio.

La diferencia es que `pipeline_executor` coordina dinámicamente según la secuencia de operaciones del `Pipeline`.

## Tests

La implementación futura debe cubrir:

- ejecución ordenada de operaciones;
- transporte del resultado intermedio tipado;
- fail-fast ante errores;
- compatibilidad entre `Iter`, `Take`, `Select`, `ToValue`, `ToValues` y `ToArgs`;
- rechazo de combinaciones incompatibles;
- uso de `StructuredItems` y `StructuredProjection` como fronteras de tipo;
- ausencia de resolver propio;
- ausencia de provider propio;
- `Iter -> Take -> Select -> ToValue` como flujo canónico;
- `Iter -> Select -> ToValues` como flujo válido;
- `Iter -> Select -> ToArgs` como flujo válido.

## Fuera de alcance

- parser textual;
- lexer/tokenizer;
- lectura multilinea;
- representación exacta de `PipelineOperation` en Rust;
- representación exacta de `PipelineValue` en Rust;
- agrupaciones/subpipelines;
- argumentos con pipelines anidados;
- integración textual de `filter`;
- mensajes finales de errores de pipeline;
- pipeline vacío;
- optimización o reordenamiento;
- presentación final según tipo de resultado;
- cambios en Evo Shell Engine.

## Diseño técnico

- [use-case.d2](use-case.d2)
- [architecture.d2](architecture.d2)
- [sequence.d2](sequence.d2)
- [domain-model.d2](domain-model.d2)
