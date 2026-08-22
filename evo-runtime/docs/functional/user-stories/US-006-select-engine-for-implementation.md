# US-006 — Seleccionar un Engine para una Implementación

## Historia

Como una Aplicación Evo,
quiero que Evo Runtime determine qué engine puede ejecutar una implementación
que lo requiera,
para que las operaciones puedan ejecutarse sin que la aplicación tenga que
conocer o seleccionar directamente el engine responsable.

## Contexto

Las implementaciones que participan en una Aplicación Evo pueden requerir
distintos mecanismos de ejecución.

Cuando una implementación necesita un engine para ser ejecutada, la unidad
solicitante no necesita conocer qué engine es compatible con ella ni cómo
debe seleccionarse.

Evo Runtime es responsable de determinar un engine válido para la implementación
antes de que la operación sea invocada mediante ese mecanismo.

La selección debe basarse en la capacidad del engine para ejecutar la
implementación requerida y no en una elección arbitraria realizada durante
la ejecución.

La forma concreta mediante la cual los engines se identifican, se registran,
declaran compatibilidad o son localizados se definirá por separado.

## Criterios de Aceptación

- Una implementación que requiera un engine puede ser asociada con un engine
  capaz de ejecutarla.
- Evo Runtime determina qué engine puede ejecutar la implementación requerida.
- La unidad solicitante no necesita seleccionar directamente el engine.
- La unidad solicitante no necesita conocer cómo Evo Runtime determina la
  compatibilidad entre una implementación y un engine.
- Un engine seleccionado debe ser compatible con la implementación que será
  ejecutada.
- La selección de engine debe producir una decisión inequívoca antes de la
  invocación mediante ese engine.
- Evo Runtime no debe seleccionar arbitrariamente entre engines cuando no
  exista una regla suficiente para determinar cuál corresponde.
- Si ningún engine válido puede ejecutar una implementación que requiere uno,
  la operación no se invoca y Evo Runtime reporta un fallo.
- Si existen múltiples engines candidatos y Evo Runtime no puede determinar
  inequívocamente cuál corresponde, la operación no se invoca y Evo Runtime
  reporta un fallo.
- Determinar el engine requerido no obliga a preparar o inicializar engines
  que no participan en la ejecución solicitada.
- Una vez determinado el engine correcto, la implementación puede continuar
  hacia su invocación sin exponer la selección del engine a la unidad
  solicitante.

## Fuera de Alcance

Esta historia no define:

- cómo se identifica un engine;
- cómo se registra un engine;
- si existe un engine registry;
- cómo se representa la compatibilidad entre engine e implementación;
- cómo una implementación declara qué engine necesita;
- cómo se localiza físicamente un engine;
- cómo se carga un engine;
- cómo se prepara o inicializa un engine;
- el lifecycle de engines;
- el formato o semántica de `.emod`;
- el formato o semántica de `.elib`;
- engine priorities;
- reglas concretas de desempate entre engines;
- el ABI utilizado por un engine;
- cómo un engine ejecuta internamente una implementación;
- cómo un engine representa internamente Values;
- scopes;
- concurrencia;
- ejecución asíncrona;
- estructuras, enums o tipos Rust utilizados para implementar la selección.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
