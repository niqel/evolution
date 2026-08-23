# US-014 — Ejecutar una Consulta mediante EvoQ

## Historia

Como una Aplicación Evo,
quiero que el trabajo de consulta requerido durante una ejecución pueda ser
realizado por Evo Runtime mediante EvoQ,
para que las consultas puedan participar en mi ejecución sin que la aplicación
necesite administrar directamente el engine de consultas.

## Contexto

Durante una ejecución, una operación o implementación participante puede
necesitar realizar trabajo de consulta sobre Values.

EvoQ es el engine base de consultas del Core de Evo.

Cuando durante una ejecución se necesita realizar trabajo de consulta, Evo
Runtime puede utilizar EvoQ para realizarlo.

La unidad que necesita la consulta no necesita conocer, crear ni administrar
directamente EvoQ.

El trabajo realizado mediante EvoQ permanece asociado con la ejecución que lo
originó.

EvoQ puede trabajar con Values sobre la base común proporcionada por EvoV y
puede producir Values que regresen a Evo Runtime para continuar participando en
la ejecución.

Si el trabajo de consulta no puede completarse correctamente, puede producirse
un Failure que continúa siendo distinto de un Value producido correctamente.

EvoQ realiza trabajo de consulta, pero Evo Runtime continúa siendo responsable
de coordinar la ejecución general.

Si después del trabajo de consulta se requieren nuevas operaciones, esas
necesidades continúan siendo atendidas a través de Evo Runtime.

## Criterios de Aceptación

- Durante una ejecución puede requerirse trabajo de consulta.
- EvoQ es el engine base de consultas del Core.
- Evo Runtime puede utilizar EvoQ cuando se necesita realizar trabajo de
  consulta.
- EvoQ no necesita ser descubierto como un Provider adicional para formar parte
  del Core.
- La unidad que necesita la consulta no necesita conocer directamente EvoQ.
- La unidad que necesita la consulta no necesita crear directamente EvoQ.
- La unidad que necesita la consulta no necesita administrar directamente EvoQ.
- EvoQ puede recibir Values como información sobre la cual realizar trabajo de
  consulta.
- Los Values utilizados por EvoQ participan sobre la base común proporcionada
  por EvoV.
- El trabajo realizado mediante EvoQ puede producir un Value como resultado.
- Un Value producido mediante EvoQ puede regresar a Evo Runtime y continuar
  participando en la ejecución.
- Un Failure producido durante el trabajo de consulta permanece distinguible de
  un Value producido correctamente.
- El trabajo mediante EvoQ puede continuar dentro del mismo contexto de
  ejecución que originó la necesidad de consulta.
- EvoQ no asume la coordinación general de la ejecución.
- Las nuevas operaciones requeridas después o durante el trabajo de consulta
  continúan siendo atendidas a través de Evo Runtime.

## Fuera de Alcance

Esta historia no define:

- la sintaxis de EvoQ;
- un lenguaje concreto de consultas;
- query operators concretos;
- `filter`;
- `map`;
- `select`;
- `where`;
- `group`;
- `join`;
- `order`;
- `aggregate`;
- la representación concreta de una consulta;
- AST de consultas;
- parser de consultas;
- lexer de consultas;
- planner;
- optimizer;
- query plan;
- ejecución lazy o eager de consultas;
- materialización de resultados;
- iteradores;
- enumeradores;
- streaming;
- caching;
- índices;
- almacenamiento;
- persistencia;
- consultas SQL;
- traducción a SQL;
- providers de base de datos;
- providers de filesystem;
- providers externos;
- la implementación interna de EvoQ;
- la API Rust de `evo-query-engine`;
- function pointers utilizados por EvoQ;
- structs o enums Rust;
- traits;
- generics;
- ownership;
- borrowing;
- lifetimes;
- representación Rust de Value;
- serialización;
- ABI;
- memory layout;
- Cargo;
- crate layout;
- linking;
- lifecycle técnico de EvoQ;
- creación física de EvoQ;
- destrucción de EvoQ;
- threading;
- async;
- scheduling;
- plugins;
- Providers dinámicos;
- Modelo B;
- Modelo C;
- Scope;
- `.efn`;
- `.esig`;
- `.emod`;
- `.elib`;
- `.root`;
- `.main`;
- cómo EvoQ utiliza Providers adicionales;
- cómo Evo-Script expresa o construye consultas.

Estas responsabilidades se definirán mediante documentación técnica,
especificaciones propias de EvoQ o etapas posteriores de Evo Runtime.
