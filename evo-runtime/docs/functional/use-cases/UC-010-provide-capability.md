# UC-010 — Provide Capability

Status: FUNCTIONAL

## Goal

Un Provider pone una o más Capabilities individuales a disposición funcional de
Evo Runtime para que puedan participar durante una Execution cuando sean
necesarias.

## Trigger

Un Provider proporciona una Capability utilizable por Evo Runtime.

## Preconditions

- Existe un Provider capaz de ofrecer una Capability.

## Main Flow

1. Existe un Provider que ofrece una Capability individual.
2. El Provider pone dicha Capability a disposición funcional de Evo Runtime.
3. La Capability queda funcionalmente disponible para Evo Runtime.
4. Una unidad participante que posteriormente requiera funcionalidad no necesita
   conocer ni administrar directamente el Provider original.

## Successful Outcome

La Capability queda funcionalmente disponible para ser utilizada en ejecuciones
coordinadas por Evo Runtime.

## Failure Outcomes

Si una Capability necesaria no está disponible, el trabajo dependiente no
puede continuar como si dicha Capability estuviera disponible.

## Invariants

- `Provider != Capability`
- `Required Operation != Capability`
- `Capability != catalog`
- `Capability != module`
- `Capability != namespace`
- `Capability != group of operations`
- La relación conceptual exacta entre Required Operation y Capability permanece
  sin definir normativamente en Model A.
- EvoQ y EvoS son componentes del Core estático y no se modelan como Providers
  adicionales en Model A.

## Related User Stories

- US-010 (Provide a Capability)
- US-011 (Provide the Evo Base Core)

## Related Data Dictionary Terms

- Provider
- Capability
- Required Operation
- Evo Runtime
- Failure

## Out of Scope

- Protocolos dinámicos de descubrimiento de plugins o proveedores.
- Mecanismos de registro o manifests en tiempo de ejecución.
- Ciclo de vida técnico de carga/descarga de extensiones.
- Modelo B (Extensiones Rust dinámicas).
- Mapeo técnico entre Required Operation y Capability.
