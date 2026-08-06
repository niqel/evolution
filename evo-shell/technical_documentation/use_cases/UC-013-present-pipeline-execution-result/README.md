# UC-013 — Presentar el resultado de un pipeline ejecutado

## Objetivo

Este caso de uso documenta cómo Evo Shell presenta al usuario el `PipelineValue` producido por la ejecución de un pipeline estructurado.

La capacidad pertenece a Evo Shell y no modifica la semántica de Evo Shell Engine.

## Naturaleza del caso de uso

`pipeline_result_presenter` es un caso de uso de presentación.

Tiene resolver y provider propios porque separa:

- la conversión de un `PipelineValue` a una representación visible;
- la escritura de esa representación en stdout.

El agent conceptual coordina la cadena:

```text
PipelineValue
  ↓
Resolver
  ↓
Provider
  ↓
stdout
```

## Relación con la ejecución

UC-013 recibe el resultado de:

```text
ExecutionResult::Pipeline(PipelineValue)
```

UC-013 no ejecuta pipeline.

UC-013 no modifica el `Shell`.

UC-013 no modifica el filesystem.

## Responsabilidades

### Pipeline Result Presenter Agent

- recibe `PipelineValue` junto con el contexto necesario para presentar la variante estructurada;
- coordina resolver y provider;
- no ejecuta operaciones;
- no modifica estado de dominio.

### Resolver

- convierte `PipelineValue` en una representación visible;
- preserva el orden de filas y valores;
- reutiliza la convención visual existente cuando el resultado es estructurado;
- no escribe en stdout.

### Provider

- escribe la representación producida;
- propaga errores de IO.

## Modelos conceptuales

`PipelineValue` puede ser:

- `StructuredItems`
- `StructuredProjection`
- `Value`
- `Values`
- `Arguments`

La presentación debe cubrir todas las variantes.

## Decisiones de presentación

- `Value` se presenta como un valor escalar.
- `Values` y `Arguments` se presentan una línea por elemento.
- `StructuredProjection` se presenta como tabla.
- `StructuredItems` reutiliza la tabla estructurada de iteración cuando es posible.
- Los valores opcionales ausentes no se muestran como `null` ni `None`.

## Decisiones diferidas

- presentación final exacta de tablas;
- iconografía o estilo final;
- mensajes de error visuales;
- salida multilinea enriquecida;
- exportación a otros destinos distintos de stdout.
