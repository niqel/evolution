# US-011 — Ejecutar un pipeline estructurado

## Historia de usuario

Como usuario de Evo Shell,
quiero que la shell ejecute un pipeline ya estructurado,
para encadenar operaciones de forma tipada sin convertirlas a texto ni duplicar la semántica del engine.

## Descripción

Evo Shell puede recibir un pipeline estructurado compuesto por operaciones ordenadas.

La shell coordina la ejecución de esas operaciones una por una y delega su semántica a Evo Shell Engine.

El pipeline estructurado conserva:

- el orden de las operaciones;
- el tipo del resultado intermedio;
- el tipo del resultado final;
- los errores de cada etapa.

La shell no interpreta texto de pipeline en esta historia.

La shell no inserta operaciones implícitas.

La shell no convierte automáticamente valores estructurados en texto.

## Flujo observable

Ejemplo conceptual:

```text
Iter
Take(1)
Select(Name)
ToValue
```

Resultado esperado:

- `Iter` produce elementos estructurados;
- `Take(1)` limita la secuencia;
- `Select(Name)` proyecta una propiedad;
- `ToValue` obtiene un valor escalar tipado o falla si la cardinalidad no cumple el contrato.

## Semántica de ejecución

El pipeline se ejecuta en el mismo orden en que fue estructurado.

Cada operación consume el resultado de la anterior y produce un nuevo estado intermedio.

Si una operación falla:

- la ejecución se detiene inmediatamente;
- las operaciones posteriores no se ejecutan;
- el error se propaga.

## Tipos intermedios

La ejecución del pipeline conserva tipos estructurados entre etapas.

Conceptualmente, los estados intermedios pueden ser:

- `StructuredItems`;
- `StructuredProjection`;
- `ProjectedValue`;
- `Values`;
- `Arguments`.

La historia no fija el nombre exacto del tipo interno que transporta esos estados.

## Alcance del pipeline

Esta historia recibe un pipeline ya estructurado.

No define:

- lexer;
- parser;
- AST;
- lectura multilinea;
- presentación;
- agrupación/subpipeline recursivo;
- integración textual de `filter`, `select`, `index`, `take`, `to-value`, `to-values` o `to-args`.

## Criterios de aceptación

1. El pipeline contiene operaciones ordenadas.
2. Las operaciones se ejecutan en el orden recibido.
3. El resultado de una etapa alimenta a la siguiente.
4. Las operaciones conservan tipos estructurados.
5. No se convierte todo a texto.
6. No se usa `null` para representar estados intermedios.
7. Una operación incompatible con el resultado previo produce error.
8. No se insertan conversiones implícitas.
9. Un error detiene la ejecución.
10. Las operaciones posteriores al error no se ejecutan.
11. La lógica de `filter`, `select`, `index`, `take` y `to-*` pertenece al engine.
12. Evo Shell solo coordina esas operaciones.
13. Esta historia recibe un pipeline estructurado, no texto.
14. Parsing queda fuera de alcance.
15. Lectura multilinea queda fuera de alcance.
16. Presentación queda fuera de alcance.
17. La agrupación/subpipeline queda diferida.
18. Debe poder representarse el flujo `Iter -> Take -> Select -> ToValue`.

## Fuera de alcance

Esta historia no define:

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
- optimización o reordenamiento de operaciones;
- presentación final según tipo de resultado.
