# US-005 — Transportar Values entre Operaciones

## Historia

Como una Aplicación Evo,
quiero que Evo Runtime permita que los Values producidos por una operación
puedan ser utilizados por otras operaciones,
para que las distintas partes de la aplicación puedan colaborar sin conocer
la implementación interna unas de otras.

## Contexto

Durante la ejecución de una Aplicación Evo, una operación puede necesitar
información producida previamente por otra operación.

Esa información debe poder cruzar las fronteras entre las unidades que
participan en la ejecución sin requerir que el consumidor conozca cómo fue
producida internamente.

Evo Runtime es responsable de permitir que los Values utilizados como entrada
y producidos como resultado puedan pasar entre operaciones durante la ejecución.

El productor de un Value no necesita conocer qué implementación lo consumirá
posteriormente, y el consumidor no necesita conocer cómo fue producido.

La representación concreta de un Value y los mecanismos físicos utilizados
para transportarlo se definirán por separado.

## Criterios de Aceptación

- Una operación puede recibir Values como parte de la información necesaria
  para su ejecución.
- Una operación puede producir Values como resultado de una ejecución correcta.
- Un Value producido por una operación puede ser utilizado como entrada de otra
  operación.
- Evo Runtime permite que los Values crucen la frontera entre una operación
  solicitante y una operación invocada.
- La operación productora no necesita conocer la implementación que consumirá
  posteriormente el Value.
- La operación consumidora no necesita conocer cómo fue producido internamente
  el Value.
- El transporte de un Value no requiere que las operaciones conozcan el engine,
  provider o mecanismo interno utilizado por la otra parte.
- Un Value que atraviesa la frontera del Runtime debe conservar su significado
  durante el intercambio.
- Un failure no debe transportarse como si fuera un Value producido
  correctamente.
- El transporte de Values puede ocurrir repetidamente durante una misma
  ejecución a medida que unas operaciones producen información que otras
  necesitan.

## Fuera de Alcance

Esta historia no define:

- la representación concreta de un Value;
- los tipos concretos que puede contener un Value;
- la representación Rust de un Value;
- ownership o borrowing de Rust;
- serialización;
- deserialización;
- memory layout;
- ABI;
- copia física de datos;
- referencias físicas o punteros;
- mutabilidad;
- persistencia de Values;
- transporte entre procesos;
- transporte por red;
- pipelines;
- concurrencia;
- ejecución asíncrona;
- la representación concreta de failures;
- cómo un engine representa internamente sus propios datos;
- estructuras, enums o tipos Rust utilizados para implementar el transporte.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
