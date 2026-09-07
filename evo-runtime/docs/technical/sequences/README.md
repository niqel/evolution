# Evo Runtime Model A — D2 Sequence Diagrams

Status: D2 SEQUENCE DIAGRAMS — CLOSED

Este directorio contiene la secuencia dinámica canónica de `evo-runtime Model A`, derivada exclusivamente de las firmas canónicas de Rust y los participantes técnicos cerrados.

## Canonical Suite

```text
00-start-application.d2       ✅ BUILT
```

Total: `1 / 1`

## 00 — Start Application

Representa la secuencia canónica del ciclo de vida de Model A:

```text
Host
  → Starter (Agent):
      START(run)
Starter
  → Run Requester:
      run()
Run Requester
  → Evo Application:
      implementación del function pointer
Evo Application
  → Run Requester:
      return ()
Run Requester
  → Starter:
      return ()
Starter
  → Host:
      return ()
```

El Agent `Starter` invoca directamente al Requester `Run`. No existen Collaborators, Resolvers, Contracts, Tools, `Result` ni `Failure`.
