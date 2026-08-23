# US-010 — Proporcionar una Capability

## Historia

Como una Aplicación Evo,
quiero que Evo Runtime pueda utilizar una Capability proporcionada por un
Provider,
para que una operación requerida pueda ser satisfecha sin que la aplicación
necesite conocer directamente qué Provider la proporciona.

## Contexto

Durante una ejecución, una Aplicación Evo puede requerir una operación cuya
realización depende de una capacidad proporcionada por un Provider.

Un Provider puede poner una o más Capabilities a disposición de Evo Runtime.

Cada Capability representa una capacidad funcional individual proporcionada
por ese Provider.

Cuando una operación requerida necesita una Capability disponible, Evo Runtime
debe poder utilizar la capacidad proporcionada sin exigir que la unidad
solicitante conozca directamente el Provider que la ofrece.

La relación concreta mediante la cual una operación requerida se corresponde
con una Capability se definirá por separado.

La forma concreta mediante la cual un Provider declara, registra o implementa
sus Capabilities también se definirá por separado.

## Criterios de Aceptación

- Un Provider puede proporcionar una o más Capabilities.
- Cada Capability representa una capacidad funcional individual.
- Una Capability proporcionada puede participar durante una ejecución.
- Una operación requerida puede necesitar una Capability para poder realizarse.
- Evo Runtime puede utilizar una Capability proporcionada por un Provider
  cuando sea necesaria para satisfacer trabajo requerido.
- La unidad solicitante no necesita conocer directamente qué Provider
  proporciona la Capability.
- La unidad solicitante no necesita localizar directamente el Provider.
- La unidad solicitante no necesita crear directamente el Provider.
- La unidad solicitante no necesita administrar directamente el Provider.
- Diferentes Providers pueden existir sin que la aplicación deba conocer sus
  detalles internos.
- La disponibilidad de una Capability no implica que todas las Capabilities
  de todos los Providers deban participar en la ejecución.
- Si una Capability necesaria no puede obtenerse de ningún Provider válido,
  el trabajo dependiente no puede continuar como si dicha Capability estuviera
  disponible.

## Fuera de Alcance

Esta historia no define:

- cómo se identifica un Provider;
- cómo se registra un Provider;
- cómo se localiza un Provider;
- cómo se crea un Provider;
- el lifecycle de un Provider;
- cómo se destruye un Provider;
- cómo se representa una Capability;
- cómo una Capability se corresponde con una Required Operation;
- signatures de Capabilities;
- function pointers;
- Contracts;
- Requesters;
- Agents;
- implementación concreta de Providers;
- si una Capability corresponde directamente a una función;
- si una Capability tiene estado;
- scopes;
- lifecycle de scopes;
- prioridad entre Providers;
- selección entre múltiples Providers;
- fallback entre Providers;
- providers por defecto;
- providers globales;
- providers por ejecución;
- providers por operación;
- caching de Providers;
- concurrencia;
- ejecución asíncrona;
- estructuras, enums o tipos Rust utilizados para implementar Providers o
  Capabilities.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
