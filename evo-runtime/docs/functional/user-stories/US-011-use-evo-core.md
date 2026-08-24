# US-011 — Disponer del Core Base de Evo

## Historia

Como una Aplicación Evo,
quiero disponer de EvoV y de los engines EvoQ y EvoS como componentes base de
Evo Runtime,
para poder utilizar las capacidades fundamentales de Evo sin depender de
Providers adicionales instalados o descubiertos durante la ejecución.

## Contexto

Evo Runtime dispone de un conjunto mínimo de componentes que forman parte del
Core base de Evo.

Ese Core está compuesto funcionalmente por:

- EvoV, como base común de Values y Outcomes;
- EvoQ, como engine base de consultas;
- EvoS, como engine base para Evo-Script;
- Evo Runtime, como plataforma responsable de coordinar la ejecución.

EvoQ y EvoS son los engines base conocidos por Evo Runtime dentro del Core.

EvoV no es un engine. EvoV proporciona la base común de Values y Outcomes
(Result y Failure) compartida por los componentes del Core.

Estos componentes forman parte del entorno base de Evo y no son Providers
adicionales.

Una Aplicación Evo no necesita descubrir, instalar ni seleccionar dinámicamente
EvoV, EvoQ o EvoS como si fueran extensiones externas para poder utilizar las
responsabilidades fundamentales proporcionadas por el Core.

La composición técnica mediante la cual estos componentes forman el Core se
define por separado.

## Criterios de Aceptación

- Evo Runtime forma parte del Core base de Evo.
- EvoV forma parte del Core base de Evo.
- EvoQ forma parte del Core base de Evo.
- EvoS forma parte del Core base de Evo.
- EvoV proporciona la base común de Values y Outcomes utilizada por el Core de Evo.
- EvoV no es un engine.
- EvoQ proporciona el engine base de consultas de Evo.
- EvoS proporciona el engine base capaz de trabajar con Evo-Script.
- Evo Runtime conoce a EvoQ y EvoS como engines base del Core.
- Los componentes del Core no necesitan ser descubiertos dinámicamente como
  Providers adicionales durante una ejecución.
- Una Aplicación Evo no necesita instalar EvoV, EvoQ o EvoS como Providers
  externos para disponer del Core.
- Una Aplicación Evo no necesita seleccionar entre múltiples Providers para
  obtener los componentes fundamentales del Core.
- Los Providers adicionales pueden extender posteriormente las capacidades
  disponibles sin convertirse por ello en componentes del Core base.
- La ausencia de Providers adicionales no elimina la disponibilidad funcional
  del Core base de Evo.

## Fuera de Alcance

Esta historia no define:

- la estructura concreta de los crates Rust del Core;
- los nombres concretos de crates o packages;
- `Cargo.toml`;
- workspace organization;
- static linking;
- dynamic linking;
- `rlib`;
- `cdylib`;
- ABI;
- símbolos exportados;
- function pointers utilizados internamente entre componentes del Core;
- la API técnica entre Evo Runtime y EvoV;
- la API técnica entre Evo Runtime y EvoQ;
- la API técnica entre Evo Runtime y EvoS;
- el modelo de plugins;
- discovery de Providers;
- loading dinámico de Providers;
- versionado de Providers;
- providers de filesystem;
- providers de SQL;
- providers de UI;
- otros Providers adicionales;
- `.elib`;
- `.esig`;
- `.emod`;
- `.root`;
- `.main`;
- Scope;
- cómo Evo-Script utiliza Providers;
- compilación de aplicaciones Evo;
- artefactos binarios propios de Evo;
- el Modelo B de extensiones Rust;
- el Modelo C de composición mediante Evo.

Estas responsabilidades se definirán mediante historias posteriores,
documentación técnica, el diccionario de datos, casos de uso o capítulos
normativos de Evo Runtime.
