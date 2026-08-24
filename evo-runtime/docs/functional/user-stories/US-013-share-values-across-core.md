# US-013 — Compartir Values y Outcomes entre los Componentes del Core

## Historia

Como una Aplicación Evo,
quiero que los Values y Outcomes (Result y Failure) puedan participar entre
Evo Runtime, EvoQ y EvoS conservando su significado,
para que los componentes del Core puedan interoperar sobre una base común de
información y resultados proporcionada por EvoV.

## Contexto

EvoV proporciona la base común de Values y Outcomes del Core de Evo.

Evo Runtime, EvoQ y EvoS pueden trabajar con Values, Results y Failures sobre
esa misma base semántica.

Un Value o un Outcome producido durante el trabajo de un componente del Core
puede continuar participando en la ejecución a través de otros componentes del
Core sin dejar de representar funcionalmente la misma información o estado de
resultado.

Por ejemplo, un Value producido por una implementación ejecutada mediante EvoS
puede regresar a Evo Runtime y posteriormente participar en trabajo realizado
mediante EvoQ. De igual modo, un Result o Failure producido por EvoS o EvoQ
puede ser recibido y propagado por Evo Runtime preservando su distinción
semántica.

Compartir una base común de Values y Outcomes no implica que todos los
componentes tengan las mismas responsabilidades:

- EvoV proporciona la base común de Values y Outcomes;
- Evo Runtime continúa coordinando la ejecución;
- EvoQ continúa siendo el engine base de consultas;
- EvoS continúa siendo el engine base para Evo-Script.

Se preservan estrictamente las distinciones:

- `Result != Value`
- `Result != Failure`
- `Value != Failure`

## Criterios de Aceptación

- EvoV proporciona la base común de Values y Outcomes (Result y Failure) del Core.
- Evo Runtime puede trabajar con Values, Results y Failures proporcionados sobre
  la base de EvoV.
- EvoQ puede trabajar con Values, Results y Failures proporcionados sobre la base
  de EvoV.
- EvoS puede trabajar con Values, Results y Failures proporcionados sobre la base
  de EvoV.
- Un Value puede pasar entre componentes del Core (EvoS, Evo Runtime, EvoQ)
  conservando su significado funcional.
- Un Result o Failure puede ser transmitido entre componentes del Core
  conservando su significado funcional.
- Los Outcomes y Values pueden participar en distintas operaciones del Core sin
  requerir un modelo semántico diferente para cada componente.
- Compartir Values y Outcomes no cambia las responsabilidades funcionales de
  Evo Runtime, EvoQ, EvoS o EvoV.
- EvoV continúa siendo la base común de Values y Outcomes y no un engine.
- EvoQ continúa siendo el engine base de consultas.
- EvoS continúa siendo el engine base para Evo-Script.
- Evo Runtime continúa siendo responsable de coordinar la ejecución.
- `Result != Value`, `Result != Failure` y `Value != Failure` se preservan en
  todo el Core.

## Fuera de Alcance

Esta historia no define:

- la representación Rust de Value, Result o Failure;
- structs Rust para Value, Result o Failure;
- enums Rust para Value, Result o Failure;
- traits;
- generics;
- ownership;
- borrowing;
- lifetimes;
- referencias;
- punteros;
- `Box`;
- `Rc`;
- `Arc`;
- `Copy`;
- `Clone`;
- memory layout;
- ABI;
- serialización;
- deserialización;
- formato binario;
- representación en memoria;
- copia física de Values o Outcomes;
- movimiento físico de Values u Outcomes;
- conversión física entre representaciones;
- optimizaciones de Value u Outcomes;
- almacenamiento de Values u Outcomes;
- persistencia de Values u Outcomes;
- caching de Values u Outcomes;
- identidad física de un Value u Outcome;
- query operators concretos;
- sintaxis de EvoQ;
- implementación interna de EvoQ;
- sintaxis de Evo-Script;
- implementación interna de EvoS;
- parser;
- lexer;
- AST;
- bytecode;
- máquina virtual;
- compilación;
- `.efn`;
- `.esig`;
- `.emod`;
- `.elib`;
- `.root`;
- `.main`;
- Providers dinámicos;
- plugins;
- Modelo B;
- Modelo C;
- Scope;
- cómo Evo-Script accede a Providers;
- cómo EvoQ accede a Providers;
- APIs Rust entre los componentes del Core;
- crate dependencies;
- Cargo;
- linking.

Estas responsabilidades se definirán mediante documentación técnica,
especificaciones de los componentes correspondientes o etapas posteriores de
Evo Runtime.
