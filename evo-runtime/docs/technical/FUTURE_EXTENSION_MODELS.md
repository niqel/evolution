# Modelos Futuros de Extensión de Evo Runtime

Status: DESIGN NOTE — NOT CLOSED

Este documento preserva direcciones conceptuales acordadas para trabajo futuro.

No forma parte todavía de la especificación normativa de Evo Runtime v0.

No define implementación cerrada.

El Modelo A del Core base se documenta funcionalmente mediante US-011 y se
desarrollará técnicamente por separado.

======================================================================
Modelo A — Core Estático
======================================================================

El Modelo A representa el Core base conocido en tiempo de composición por
Rust/Cargo.

Los cuatro componentes base son:

    evo-runtime
        → Evo Runtime

    evo-values
        → EvoV

    evo-query-engine
        → EvoQ

    evo-script-engine
        → EvoS

Conceptualmente:

                         Evo Runtime
                              │
                  ┌───────────┴───────────┐
                  ▼                       ▼
                    EvoQ                 EvoS
             evo-query-engine     evo-script-engine
                  │                       │
                  └───────────┬───────────┘
                              ▼
                             EvoV
                         evo-values

EvoQ y EvoS son engines del Core conocidos por Evo Runtime.

EvoV no es un engine.

EvoV constituye la base común de Values compartida por los componentes del
Core.

Mantener la distinción:

    Evo-Script
        = lenguaje

    EvoS
        = engine del Core capaz de trabajar con Evo-Script

El Modelo A no necesita discovery dinámico de estos componentes.

La composición física concreta mediante Cargo, crates y linking se documentará
en la arquitectura técnica correspondiente y no queda cerrada por esta nota.

======================================================================
Modelo B — Extensiones Rust
======================================================================

El Modelo B representa componentes adicionales al Core base de Evo.

Estos componentes se implementarán en Rust y podrán distribuirse o instalarse
de forma independiente al Core.

Ejemplos futuros pueden incluir:

- filesystem;
- SQL;
- UI;
- audio;
- network;
- otras capacidades no pertenecientes al Core base.

Conceptualmente:

    Rust Extension
        ↓
    Provider o Engine
        ↓
    proporciona Capabilities
        ↓
    Evo Runtime

Los componentes del Modelo B no forman parte automáticamente del Core base
compuesto por:

    Evo Runtime
    EvoV
    EvoQ
    EvoS

El mecanismo mediante el cual una extensión Rust se conecta dinámicamente con
Evo Runtime todavía NO está definido.

Antes de cerrar el Modelo B deberán estudiarse, entre otros:

- plugin boundary;
- ABI;
- discovery;
- loading;
- unloading;
- versionado;
- compatibilidad;
- publicación de Capabilities;
- reutilización de dependencias comunes;
- lifecycle;
- manejo de failures en la frontera del plugin.

No asumir todavía:

- `cdylib` como solución definitiva;
- ABI Rust estable entre plugins;
- ABI C como solución definitiva;
- procesos separados;
- IPC;
- registry concreto;
- archivo de manifiesto concreto;
- `.elib`;
- `.esig`;
- `.emod`;
- Scope concreto.

La implementación del Modelo B se decidirá después de cerrar el Core base.

======================================================================
Modelo C — Composición mediante Evo
======================================================================

El Modelo C representa una etapa posterior en la que Evo-Script y artefactos
propios de Evo puedan participar en la definición o composición de una
Aplicación Evo para Evo Runtime.

Conceptualmente:

    Evo-Script / Evo artifacts
        ↓
    composition description
        ↓
    Evo Runtime

En este modelo se revisarán nuevamente, con problemas concretos que resolver,
los artefactos históricos:

    .main
    .root
    .elib
    .esig
    .emod

Ninguno de estos artefactos queda definido ni confirmado por esta nota.

Cada uno deberá justificar una responsabilidad que Rust, Cargo o el propio
Runtime no resuelvan ya de forma suficiente.

El objetivo futuro del Modelo C puede llegar a incluir:

- composición de aplicaciones mediante Evo;
- descripción de dependencias de una aplicación;
- utilización de Providers o Engines desde Evo-Script;
- preparación de una aplicación para Evo Runtime;
- generación de un artefacto propio de Evo;
- eventualmente compilación o transformación a una extensión o formato propio
  de Evo.

La sintaxis, semántica, formato físico y proceso de compilación de esos
artefactos todavía NO están definidos.

======================================================================
Scope — Relación Futura
======================================================================

Scope todavía NO está definido.

La dirección conceptual que debe preservarse es:

    Provider / Engine
        ↓
    Capability
        ↓
    Scope
        ↓
    Evo-Script

Scope se estudiará como la frontera mediante la cual Evo-Script utiliza
capacidades proporcionadas por un Provider o Engine.

Esta nota NO define todavía:

- sintaxis de Scope;
- lifecycle de Scope;
- ownership;
- identidad;
- nesting;
- visibilidad;
- aislamiento;
- sharing;
- si Scope pertenece al Provider;
- si Provider pertenece al Scope;
- cómo Scope se representa en Rust;
- cómo Scope aparece en Evo-Script.

======================================================================
Regla de Separación
======================================================================

Mantener separados:

    MODEL A
        Core estático base:
        Evo Runtime + EvoV + EvoQ + EvoS

    MODEL B
        extensiones adicionales implementadas en Rust

    MODEL C
        composición futura mediante Evo-Script y artefactos Evo

No utilizar los Modelos B o C para explicar responsabilidades que Rust y Cargo
ya resuelven dentro del Modelo A.

No utilizar esta nota como especificación normativa.

Los Modelos B y C se retomarán después de cerrar el Core base de Evo Runtime.
