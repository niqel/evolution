# US-001 — Iniciar una Aplicación Evo

## Historia

Como host,
quiero que Evo Runtime inicie una Aplicación Evo desde su punto de entrada declarado,
para poder ejecutar la aplicación sin conocer su implementación interna.

## Contexto

Una Aplicación Evo declara dónde comienza su ejecución.

El host solicita la ejecución de la aplicación, pero no necesita conocer
qué unidad ejecutable, engine, provider o implementación interna realiza
el trabajo de la aplicación.

Evo Runtime es responsable de interpretar el punto de entrada declarado
por la aplicación e iniciar la ejecución desde ese punto.

El mecanismo utilizado para declarar el punto de entrada forma parte del
modelo de aplicación de Evo Runtime y se definirá por separado.

## Criterios de Aceptación

- Un host puede solicitar a Evo Runtime que inicie una Aplicación Evo.
- La aplicación tiene un único punto de entrada inicial declarado.
- Evo Runtime determina el punto de entrada declarado sin requerir que el host
  conozca su implementación interna.
- Evo Runtime inicia la ejecución desde ese punto de entrada.
- El host no necesita resolver por sí mismo las dependencias de la aplicación.
- El host no necesita localizar ni cargar por sí mismo las unidades ejecutables
  internas.
- El host no necesita conocer qué engine ejecutará finalmente el punto de entrada.
- Iniciar una aplicación no requiere cargar capabilities o implementaciones
  que no estén relacionadas con la ejecución solicitada.
- Si la aplicación no puede proporcionar un punto de entrada declarado válido,
  la ejecución no comienza y Evo Runtime reporta un fallo.
- El resultado de la ejecución se devuelve a través de la frontera de Evo Runtime.

## Fuera de Alcance

Esta historia no define:

- la sintaxis o formato de archivo de `.main`;
- cómo `.root` realiza la composición;
- cómo se resuelven las dependencias;
- cómo se localizan físicamente las unidades ejecutables;
- el registro de engines;
- el ciclo de vida de providers;
- los scopes;
- la representación concreta de los fallos del Runtime;
- la representación concreta de los Values devueltos.

Estas responsabilidades se definirán mediante historias separadas,
casos de uso o capítulos normativos de Evo Runtime.
