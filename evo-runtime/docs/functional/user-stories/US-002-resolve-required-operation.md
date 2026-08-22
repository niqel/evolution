# US-002 — Resolver una Operación Requerida

## Historia

Como una Aplicación Evo,
quiero poder requerir una operación sin conocer su implementación concreta,
para que Evo Runtime determine qué implementación debe satisfacerla.

## Contexto

Durante la ejecución de una Aplicación Evo, una operación puede necesitar
utilizar otra operación cuya implementación no forma parte de la unidad
ejecutable que realiza la solicitud.

La aplicación expresa qué operación necesita, pero no necesita conocer
dónde se encuentra físicamente, qué componente la implementa ni cómo debe
ser cargada.

Evo Runtime es responsable de resolver la operación requerida hacia una
implementación válida disponible dentro de la composición de la aplicación.

La forma concreta mediante la cual una operación requerida se identifica,
se declara y se relaciona con una implementación se definirá por separado.

## Criterios de Aceptación

- Una operación en ejecución puede requerir otra operación sin conocer su
  implementación concreta.
- La operación requerida debe poder identificarse de forma inequívoca.
- Evo Runtime determina qué implementación satisface la operación requerida.
- La unidad que realiza la solicitud no necesita conocer la ubicación física
  de la implementación.
- La unidad que realiza la solicitud no necesita cargar la implementación
  directamente.
- La unidad que realiza la solicitud no necesita conocer qué provider, engine
  o componente interno satisface finalmente la operación.
- Resolver una operación no requiere cargar implementaciones que no formen
  parte de la ejecución solicitada.
- La resolución debe producir una única implementación válida.
- Si ninguna implementación válida puede satisfacer la operación requerida,
  Evo Runtime reporta un fallo y la operación requerida no se ejecuta.
- Si la operación requerida puede resolverse de forma ambigua hacia más de una
  implementación válida, Evo Runtime reporta un fallo y no selecciona una
  implementación arbitrariamente.
- Una vez resuelta correctamente, la implementación puede participar en la
  ejecución sin exponer sus detalles internos al solicitante.

## Fuera de Alcance

Esta historia no define:

- la sintaxis o formato de `.root`;
- la sintaxis o formato de `.esig`;
- si una operación requerida se representa mediante una signature, contract
  u otro artefacto;
- cómo se declara una relación entre operación e implementación;
- cómo se localiza físicamente una implementación;
- el formato de `.elib`;
- el formato de `.emod`;
- cómo se carga una implementación;
- el lifecycle de providers;
- scopes;
- la representación concreta de los fallos de resolución;
- estructuras, enums o tipos Rust utilizados para implementar la resolución.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
