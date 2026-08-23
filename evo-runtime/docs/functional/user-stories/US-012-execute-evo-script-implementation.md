# US-012 — Ejecutar una Implementación Evo-Script

## Historia

Como una Aplicación Evo,
quiero que una implementación escrita en Evo-Script pueda ser ejecutada por
Evo Runtime mediante EvoS,
para que pueda participar en mi ejecución sin que la aplicación necesite
administrar directamente el engine que la ejecuta.

## Contexto

Durante una ejecución, Evo Runtime puede resolver una operación requerida hacia
una implementación escrita en Evo-Script.

EvoS es el engine base del Core capaz de trabajar con Evo-Script.

Cuando una implementación Evo-Script debe participar en una ejecución, Evo
Runtime puede utilizar EvoS para ejecutarla.

La unidad que requirió originalmente la operación no necesita conocer ni
administrar directamente EvoS para que la implementación pueda ejecutarse.

La ejecución de una implementación Evo-Script permanece dentro del contexto de
la ejecución que originó el trabajo.

Una implementación Evo-Script puede recibir Values como información de entrada
y puede producir un Value como resultado.

Si la ejecución de la implementación no puede completarse correctamente, puede
producirse un Failure que continúa siendo distinto de un Value producido
correctamente.

Una implementación Evo-Script puede requerir transitivamente otras operaciones
durante su ejecución. Estas necesidades continúan siendo atendidas a través de
Evo Runtime.

## Criterios de Aceptación

- Una operación requerida puede resolverse hacia una implementación escrita en
  Evo-Script.
- EvoS es el engine base del Core capaz de ejecutar una implementación
  Evo-Script.
- Evo Runtime puede utilizar EvoS cuando una implementación Evo-Script debe
  participar en una ejecución.
- EvoS no necesita ser descubierto como un Provider adicional para formar parte
  del Core.
- La unidad solicitante no necesita conocer directamente EvoS.
- La unidad solicitante no necesita crear directamente EvoS.
- La unidad solicitante no necesita administrar directamente EvoS.
- Una implementación Evo-Script puede recibir Values como entrada.
- Una implementación Evo-Script puede producir un Value como resultado.
- Un Value producido puede regresar a Evo Runtime y continuar participando en
  la ejecución.
- Un Failure producido durante la ejecución permanece distinguible de un Value
  producido correctamente.
- Una implementación Evo-Script puede requerir otras operaciones durante su
  ejecución.
- Las operaciones requeridas transitivamente continúan siendo resueltas a
  través de Evo Runtime.
- La ejecución mediante EvoS puede continuar dentro del mismo contexto de
  ejecución que originó el trabajo.

## Fuera de Alcance

Esta historia no define:

- la sintaxis de Evo-Script;
- el parser de Evo-Script;
- lexer;
- AST;
- análisis semántico;
- bytecode;
- máquina virtual;
- interpretación interna de Evo-Script;
- compilación de Evo-Script;
- optimizaciones;
- representación interna de una función Evo-Script;
- representación física de archivos Evo-Script;
- `.efn`;
- `.esig`;
- `.emod`;
- `.elib`;
- `.root`;
- `.main`;
- la API Rust de `evo-script-engine`;
- function pointers utilizados por EvoS;
- structs o enums Rust;
- Cargo;
- crate layout;
- linking;
- lifecycle técnico de EvoS;
- creación física de EvoS;
- destrucción de EvoS;
- threading;
- async;
- scheduling;
- caching;
- hot reload;
- plugins;
- Providers dinámicos;
- Modelo B;
- Modelo C;
- Scope;
- cómo Evo-Script accede a Providers o Engines adicionales.

Estas responsabilidades se definirán en historias posteriores, documentación
técnica o en la especificación propia de Evo-Script y EvoS.
