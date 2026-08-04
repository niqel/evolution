# Evolution (Evo)

Evolution (Evo) es un ecosistema de software construido principalmente en Rust.

Su objetivo es construir componentes pequenos, eficientes, multiplataforma e independientes que puedan utilizarse por separado o integrarse como parte del ecosistema Evo.

El proyecto prioriza binarios pequenos, dependencias controladas, separacion clara de responsabilidades y composicion.

Los principios generales de ingenieria estan definidos en [ENGINEERING_PRINCIPLES.md](ENGINEERING_PRINCIPLES.md).

La arquitectura utilizada por los componentes de Evo esta definida en [ARCHITECTURE.md](ARCHITECTURE.md).

Componente inicial:

- `evo-shell-engine`: motor operativo basado en scopes, diseñado para ser compartido por el futuro `evo-shell`, el futuro `evo-script` y otros componentes del ecosistema Evo.

Relación conceptual:

```text
evo-shell
        │
        ▼
evo-shell-engine
        ▲
        │
evo-script
```
