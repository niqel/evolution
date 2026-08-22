# US-004 — Invocar una Operación

## Historia

Como una Aplicación Evo,
quiero que Evo Runtime invoque una operación que ya está disponible para la
ejecución,
para que el trabajo solicitado pueda realizarse sin que la unidad solicitante
necesite conocer cómo se ejecuta internamente su implementación.

## Contexto

Después de que Evo Runtime determina qué implementación satisface una operación
requerida y hace disponible dicha implementación, la operación debe poder
participar efectivamente en la ejecución.

La unidad solicitante conoce la operación que necesita utilizar, pero no
necesita conocer el mecanismo interno mediante el cual su implementación
será ejecutada.

Evo Runtime es responsable de realizar la invocación y de mantener la frontera
entre la unidad solicitante y la implementación invocada.

Una invocación puede completar correctamente y producir un resultado, o puede
fallar.

La forma concreta mediante la cual se representan los argumentos, el resultado,
los failures y el mecanismo físico de invocación se definirá por separado.

## Criterios de Aceptación

- Una operación cuya implementación está disponible puede ser invocada durante
  la ejecución.
- Evo Runtime realiza la invocación de la operación solicitada.
- La unidad solicitante no necesita conocer cómo se ejecuta internamente la
  implementación.
- La unidad solicitante no necesita conocer si la implementación es ejecutada
  por un engine, provider u otro mecanismo interno.
- La invocación puede recibir información necesaria para realizar la operación.
- Una invocación completada correctamente puede producir un resultado.
- El resultado producido puede regresar a la unidad solicitante a través de
  Evo Runtime.
- Si la invocación falla, el fallo puede regresar a través de Evo Runtime sin
  convertirse en un resultado válido.
- Evo Runtime no debe ocultar un fallo de invocación presentándolo como una
  ejecución exitosa.
- Una operación no debe ser invocada si su implementación requerida no está
  disponible para participar en la ejecución.
- La implementación invocada puede requerir nuevas operaciones durante su
  ejecución, continuando el mismo modelo de resolución y carga bajo demanda.

## Fuera de Alcance

Esta historia no define:

- la representación concreta de los argumentos;
- la representación concreta de los resultados;
- la representación concreta de los Values;
- la representación concreta de los failures;
- el ABI utilizado para realizar una invocación;
- function pointers;
- cómo se selecciona un engine;
- cómo un engine ejecuta internamente una unidad;
- cómo un provider implementa una operación;
- cómo se crea o mantiene un Execution Context;
- scopes;
- pipelines;
- callbacks;
- concurrencia;
- ejecución asíncrona;
- lifecycle de implementaciones;
- estructuras, enums o tipos Rust utilizados para implementar la invocación.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
