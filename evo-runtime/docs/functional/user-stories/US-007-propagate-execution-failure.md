# US-007 — Propagar un Failure de Ejecución

## Historia

Como una Aplicación Evo,
quiero que Evo Runtime preserve y propague los failures que ocurren durante
la ejecución,
para que una operación fallida no sea presentada como exitosa y el solicitante
pueda conocer que el trabajo requerido no pudo completarse correctamente.

## Contexto

Durante la ejecución de una Aplicación Evo pueden ocurrir failures en distintas
etapas del trabajo realizado por Evo Runtime.

Un failure puede originarse, por ejemplo, al resolver una operación, hacer
disponible una implementación, seleccionar un engine o invocar una operación.

Cuando ocurre un failure, Evo Runtime debe conservar la diferencia entre una
ejecución correcta y una ejecución fallida mientras ese resultado atraviesa
las fronteras de la ejecución.

Una unidad que depende de una operación fallida no debe recibir un Value como
si dicha operación hubiera completado correctamente.

El failure debe poder regresar hacia el solicitante a través de Evo Runtime
sin requerir que las unidades intermedias conozcan cómo se representa
internamente.

La representación concreta de failures y la información que contienen se
definirá por separado.

## Criterios de Aceptación

- Una operación o responsabilidad del Runtime puede terminar con un failure.
- Evo Runtime distingue funcionalmente entre una ejecución correcta y una
  ejecución fallida.
- Un failure puede atravesar las fronteras de Evo Runtime hacia la unidad que
  depende del trabajo que falló.
- Un failure no debe convertirse silenciosamente en un Value producido
  correctamente.
- Un failure no debe ser presentado como una ejecución exitosa.
- Una operación que depende de un resultado que no pudo producirse
  correctamente no debe continuar utilizando un Value inexistente como si
  fuera válido.
- Un failure producido durante resolución puede impedir continuar con la
  operación que dependía de esa resolución.
- Un failure producido al hacer disponible una implementación puede impedir
  invocar la operación dependiente.
- Un failure producido durante la selección de un engine puede impedir la
  invocación que requería dicho engine.
- Un failure producido durante una invocación puede regresar hacia su
  solicitante.
- La propagación de un failure puede atravesar más de una operación cuando la
  ejecución contiene dependencias entre operaciones.
- Las unidades intermedias no necesitan conocer la representación interna del
  failure para que Evo Runtime pueda propagarlo.

## Fuera de Alcance

Esta historia no define:

- la representación concreta de un failure;
- categorías concretas de failures;
- códigos de error;
- mensajes de error;
- stack traces;
- excepciones;
- panics;
- la representación Rust de failures;
- `Result<T, E>`;
- enums de error;
- recuperación automática;
- retry;
- fallback;
- compensación;
- logging;
- tracing;
- telemetría;
- presentación de errores al usuario final;
- persistencia de failures;
- serialización de failures;
- transporte de failures entre procesos;
- transporte de failures por red;
- concurrencia;
- ejecución asíncrona;
- estructuras, enums o tipos Rust utilizados para implementar la propagación.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
