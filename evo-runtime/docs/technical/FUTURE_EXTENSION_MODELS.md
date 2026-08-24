# Future Extension Models — Evo Runtime

Status: DESIGN NOTE — NOT CLOSED

This note records future architectural considerations regarding downloadable and
compiled engine extensions for the Evo ecosystem.

> [!IMPORTANT]
> **Out of Scope for Model A**
>
> The architectural models discussed in this document are **NOT** part of Evo
> Runtime Model A. Evo Runtime Model A does not resolve, load, or manage engines
> or providers.

---

## 1. Future Compiled Engine Extension

In future evolution phases beyond Model A, there may be a requirement to add,
install, or load compiled engines dynamically (for example, `evo-api-rest-engine`
or specialized database drivers) without requiring the recompilation of Evo
Runtime or the entire host application.

### Open Architectural Topics

The following topics remain open for future technical design:

1. **Stable Engine ABI**: Defining binary interface compatibility across
   compiler versions and platforms.
2. **Package and Distribution Format**: Defining packaging, manifests, and
   metadata for compiled engine distribution.
3. **Engine Loader / Host**: Designing a dedicated engine loader mechanism
   independent of the core runtime coordination.
4. **Platform Binaries**: Managing architecture-specific and OS-specific binary
   artifacts.
5. **Versioning and Compatibility**: Version negotiation between applications
   and dynamic engines.
6. **Installation and Discovery**: Filesystem paths and registries for engine
   lookup.

---

## 2. Boundary Clarification

Future extension models must preserve the core design invariant:
- Evo Runtime does not become an engine registry or service locator.
- Dynamic engine loading, if implemented, will belong to a dedicated extension
  subsystem and will not complicate the minimal Model A runtime boundary.
