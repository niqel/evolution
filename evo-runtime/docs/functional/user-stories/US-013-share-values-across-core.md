# US-013 — Compartir Values entre los Componentes del Core

## Historia

Como una Aplicación Evo,
quiero que los Values puedan participar entre Evo Runtime, EvoQ y EvoS
conservando su significado,
para que los componentes del Core puedan interoperar sobre una base común de
información proporcionada por EvoV.

## Contexto

EvoV proporciona la base común de Values del Core de Evo.

Evo Runtime, EvoQ y EvoS pueden trabajar con Values sobre esa misma base
semántica.

Un Value producido durante el trabajo de un componente del Core puede continuar
participando en la ejecución a través de otros componentes del Core sin dejar de
representar funcionalmente la misma información.

Por ejemplo, un Value producido por una implementación ejecutada mediante EvoS
puede regresar a Evo Runtime y posteriormente participar en trabajo realizado
mediante EvoQ.

De la misma forma, un Value que participe en trabajo realizado mediante EvoQ
puede continuar a través de Evo Runtime y participar posteriormente en una
implementación ejecutada mediante EvoS.

Compartir una base común de Values no implica que todos los componentes tengan
las mismas responsabilidades.

EvoV proporciona la base común de Values.

Evo Runtime continúa coordinando la ejecución.

EvoQ continúa siendo el engine base de consultas.

EvoS continúa siendo el engine base para Evo-Script.

Un Failure continúa siendo distinto de un Value y no forma parte del modelo de
Values exitosos únicamente por atravesar componentes del Core.

## Criterios de Aceptación

- EvoV proporciona la base común de Values del Core.
- Evo Runtime puede trabajar con Values proporcionados sobre la base de EvoV.
- EvoQ puede trabajar con Values proporcionados sobre la base de EvoV.
- EvoS puede trabajar con Values proporcionados sobre la base de EvoV.
- Un Value puede pasar de EvoS a Evo Runtime conservando su significado
  funcional.
- Un Value puede pasar de Evo Runtime a EvoQ conservando su significado
  funcional.
- Un Value puede pasar de EvoQ a Evo Runtime conservando su significado
  funcional.
- Un Value puede pasar de Evo Runtime a EvoS conservando su significado
  funcional.
- Un Value puede continuar participando en distintas operaciones del Core sin
  requerir un modelo semántico de Value diferente para cada componente.
- Compartir Values no cambia las responsabilidades funcionales de Evo Runtime,
  EvoQ, EvoS o EvoV.
- EvoV continúa siendo la base común de Values y no un engine.
- EvoQ continúa siendo el engine base de consultas.
- EvoS continúa siendo el engine base para Evo-Script.
- Evo Runtime continúa siendo responsable de coordinar la ejecución.
- Un Failure permanece distinguible de un Value correcto.

## Fuera de Alcance

Esta historia no define:

- la representación Rust de Value;
- structs Rust para Value;
- enums Rust para Value;
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
- copia física de Values;
- movimiento físico de Values;
- conversión física entre representaciones;
- optimizaciones de Value;
- almacenamiento de Values;
- persistencia de Values;
- caching de Values;
- identidad física de un Value;
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
