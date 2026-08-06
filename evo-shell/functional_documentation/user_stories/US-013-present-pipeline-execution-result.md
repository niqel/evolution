# US-013 — Presentar el resultado de un pipeline ejecutado

## Historia de usuario

Como usuario de Evo Shell,
quiero ver en terminal el resultado de un pipeline ya ejecutado,
para obtener una salida humana y legible sin perder el tipo semántico del resultado durante la ejecución.

## Descripción

Cuando un pipeline produce un resultado tipado, la shell debe presentarlo al usuario.

La presentación debe cubrir:

- un valor escalar;
- una colección de valores;
- una colección de argumentos;
- una proyección estructurada;
- una colección estructurada de filas.

La shell no debe mostrar representaciones internas de depuración.
La shell no debe convertir el resultado a `null` o `None`.
La shell no debe modificar el scope ni el filesystem para presentarlo.

## Criterios de aceptación

1. Un `PipelineValue` final puede mostrarse al usuario.
2. `Value` se muestra como valor escalar.
3. `Values` se muestra en orden, un valor por línea.
4. `Arguments` se muestra en orden, un argumento por línea.
5. `StructuredProjection` se muestra de forma estructurada.
6. `StructuredItems` usa la presentación existente de iteración cuando es posible.
7. No se muestran nombres internos de enums.
8. No se usa salida de depuración.
9. No se muestra `null`.
10. Los valores opcionales ausentes usan la convención existente.
11. Una colección `Values` vacía es éxito.
12. Una colección `Arguments` vacía es éxito.
13. La presentación no ejecuta operaciones.
14. La presentación no modifica el scope.
15. La presentación no modifica el filesystem.

## Fuera de alcance

Esta historia no define:

- ejecución de pipelines;
- parsing textual;
- multilinea;
- subpipelines;
- pipelines como argumentos;
- semántica del engine;
- nuevos tipos de resultado.
