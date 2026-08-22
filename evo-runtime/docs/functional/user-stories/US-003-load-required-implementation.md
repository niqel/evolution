# US-003 — Cargar una Implementación Requerida

## Historia

Como una Aplicación Evo,
quiero que Evo Runtime cargue una implementación únicamente cuando sea necesaria
para la ejecución,
para evitar preparar o cargar componentes que no participan en el trabajo
solicitado.

## Contexto

Después de que Evo Runtime determina qué implementación satisface una operación
requerida, esa implementación debe estar disponible para poder participar en
la ejecución.

Una Aplicación Evo puede conocer muchas operaciones e implementaciones
disponibles sin que todas ellas sean necesarias durante una ejecución concreta.

Evo Runtime es responsable de hacer disponible una implementación requerida
cuando la ejecución realmente la necesita.

Las implementaciones que no forman parte de la ejecución solicitada no deben
ser cargadas únicamente por estar disponibles dentro de la aplicación.

La forma concreta mediante la cual una implementación se localiza físicamente,
se carga o se prepara para su ejecución se definirá por separado.

## Criterios de Aceptación

- Una implementación correctamente resuelta puede hacerse disponible para
  participar en la ejecución.
- Evo Runtime carga una implementación cuando la ejecución realmente la
  requiere.
- Una implementación disponible dentro de la aplicación no se carga únicamente
  por existir o estar registrada.
- Las implementaciones que no forman parte de la ejecución solicitada permanecen
  sin cargar.
- Una operación puede requerir nuevas implementaciones durante el desarrollo
  de la ejecución.
- Evo Runtime puede cargar esas nuevas implementaciones cuando pasan a ser
  necesarias.
- La unidad solicitante no necesita localizar físicamente la implementación.
- La unidad solicitante no necesita realizar directamente la carga.
- La unidad solicitante no necesita conocer el mecanismo utilizado para hacer
  disponible la implementación.
- Si una implementación requerida no puede hacerse disponible, la operación que
  depende de ella no se ejecuta y Evo Runtime reporta un fallo.
- Una implementación cargada puede quedar disponible para participar en la
  ejecución sin exponer su mecanismo de carga a la unidad solicitante.

## Fuera de Alcance

Esta historia no define:

- cómo se localiza físicamente una implementación;
- el formato o semántica de `.elib`;
- el formato o semántica de `.emod`;
- cómo se representa físicamente una unidad ejecutable;
- si una implementación procede de un archivo, librería, proceso, engine,
  servicio u otro mecanismo;
- cómo se selecciona un engine;
- cómo se prepara internamente una implementación antes de ejecutarla;
- cuánto tiempo permanece cargada una implementación;
- si una implementación cargada puede reutilizarse;
- el lifecycle de providers;
- scopes;
- cómo se invoca finalmente una operación cargada;
- cómo se transportan Values;
- la representación concreta de los fallos de carga;
- estructuras, enums o tipos Rust utilizados para implementar la carga.

Estas responsabilidades se definirán mediante historias separadas,
el diccionario de datos, casos de uso, documentación técnica o capítulos
normativos de Evo Runtime.
