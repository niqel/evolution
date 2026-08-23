# US-009 — Finalizar una Ejecución

## Historia

Como un host,
quiero que Evo Runtime finalice una ejecución cuando el trabajo iniciado haya
concluido,
para poder conocer que la ejecución terminó y recibir su resultado o failure
sin tener que administrar internamente su ciclo de finalización.

## Contexto

Una ejecución iniciada por Evo Runtime puede involucrar múltiples operaciones,
resoluciones, implementaciones, engines, Values y failures antes de concluir.

Mientras el trabajo permanece activo, sus actividades pueden continuar
asociadas con el contexto de ejecución que las originó.

Cuando el trabajo iniciado ya no requiere continuar ejecutándose, Evo Runtime
debe poder considerar concluida esa ejecución y comunicar su resultado al host
que la inició.

Una ejecución puede concluir correctamente o puede concluir con un failure.

Finalizar una ejecución no implica definir todavía cómo se liberan físicamente
recursos, cómo se destruye el contexto ni cómo se administran internamente
engines, providers u otras implementaciones.

La representación concreta del resultado final y del estado interno utilizado
para determinar la finalización se definirá por separado.

## Criterios de Aceptación

- Una ejecución iniciada puede llegar a una condición de finalización.
- Evo Runtime puede reconocer cuando el trabajo correspondiente a una ejecución
  ha concluido.
- Una ejecución que concluye correctamente puede producir un resultado final.
- El resultado final puede regresar al host que inició la ejecución.
- Una ejecución que no puede concluir correctamente puede finalizar con un
  failure.
- El failure final puede regresar al host que inició la ejecución.
- Una ejecución finalizada no debe continuar como si permaneciera activa.
- Las operaciones transitivas pertenecientes al trabajo deben poder concluir
  antes de que Evo Runtime considere finalizada la ejecución cuando dichas
  operaciones todavía sean necesarias para completar el trabajo.
- La finalización de una ejecución no debe finalizar accidentalmente otra
  ejecución distinta.
- El host no necesita conocer cómo Evo Runtime determina internamente que una
  ejecución ha concluido.
- El host no necesita destruir directamente el contexto interno utilizado por
  Evo Runtime.
- Finalizar una ejecución no convierte un failure en un resultado exitoso.

## Fuera de Alcance

Esta historia no define:

- la representación concreta del resultado final;
- la representación concreta del failure final;
- un estado concreto `Completed`, `Finished` o equivalente;
- una máquina de estados de ejecución;
- cómo se detecta físicamente que no queda trabajo pendiente;
- reference counting;
- contadores de operaciones;
- event loops;
- schedulers;
- threads;
- tasks;
- concurrencia;
- ejecución asíncrona;
- cancelación;
- timeouts;
- cleanup físico;
- liberación de memoria;
- destrucción concreta del Execution Context;
- lifecycle de engines;
- lifecycle de providers;
- unload de implementaciones;
- persistencia de resultados;
- logging;
- tracing;
- telemetría;
- estructuras, enums o tipos Rust utilizados para implementar la finalización.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
