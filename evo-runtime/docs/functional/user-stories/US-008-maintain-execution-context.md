# US-008 — Mantener un Contexto de Ejecución

## Historia

Como una Aplicación Evo,
quiero que Evo Runtime mantenga un contexto común durante una ejecución,
para que las operaciones que participan en el mismo trabajo puedan ejecutarse
como parte de una misma ejecución sin tener que reconstruir individualmente
la información contextual necesaria.

## Contexto

Una ejecución iniciada en Evo Runtime puede involucrar múltiples operaciones.

Una operación puede requerir otra operación, esa segunda operación puede
requerir nuevas operaciones y el trabajo puede continuar transitivamente
durante la misma ejecución.

Evo Runtime debe poder reconocer que esas operaciones forman parte del mismo
trabajo en curso y mantener el contexto necesario mientras dicho trabajo
continúa.

Cuando una operación provoca nuevas resoluciones, cargas, selecciones de engine
o invocaciones, esas actividades pueden continuar dentro del contexto de la
ejecución que las originó.

Las unidades participantes no necesitan crear ni administrar directamente el
contexto utilizado por Evo Runtime.

La información concreta contenida en ese contexto y su representación interna
se definirán por separado.

## Criterios de Aceptación

- Una ejecución puede tener un contexto asociado mientras permanece activa.
- Las operaciones que participan en el mismo trabajo pueden continuar dentro
  del contexto de la ejecución que las originó.
- Una operación que requiere nuevas operaciones puede hacerlo sin crear por sí
  misma un nuevo contexto de ejecución independiente.
- Las resoluciones requeridas durante una ejecución pueden realizarse dentro
  del contexto de esa ejecución.
- Las implementaciones requeridas durante una ejecución pueden hacerse
  disponibles dentro del contexto del trabajo en curso.
- La selección de engines requerida durante una ejecución puede continuar
  dentro del mismo contexto.
- Las invocaciones transitivas pueden permanecer asociadas con la ejecución
  que las originó.
- Los Values intercambiados durante el trabajo pueden participar en operaciones
  pertenecientes a esa misma ejecución.
- Los failures producidos durante el trabajo pueden propagarse dentro de la
  ejecución que los originó.
- Las unidades participantes no necesitan conocer cómo Evo Runtime representa
  internamente el contexto.
- Las unidades participantes no necesitan crear, almacenar ni destruir
  directamente el contexto de Runtime.
- Una ejecución no debe depender accidentalmente del contexto perteneciente a
  otra ejecución distinta.

## Fuera de Alcance

Esta historia no define:

- la representación concreta de un Execution Context;
- qué campos contiene un contexto;
- identificadores concretos de ejecución;
- scopes;
- variables de scope;
- almacenamiento global;
- thread-local storage;
- task-local storage;
- ownership;
- borrowing;
- lifetimes de Rust;
- referencias o punteros;
- concurrencia;
- ejecución asíncrona;
- threads;
- tasks;
- cancelación;
- timeouts;
- seguridad o permisos;
- identidad de usuario;
- tracing;
- logging;
- telemetría;
- persistencia del contexto;
- serialización del contexto;
- transporte del contexto entre procesos;
- transporte del contexto por red;
- lifecycle concreto del contexto;
- estructuras, enums o tipos Rust utilizados para implementarlo.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
