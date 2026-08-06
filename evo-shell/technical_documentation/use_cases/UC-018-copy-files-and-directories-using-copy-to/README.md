# UC-018 — Copiar archivos y directorios mediante copy-to

## Descripción técnica

Este caso de uso documenta la arquitectura, el modelo de dominio, la secuencia de ejecución y las firmas técnicas para el comando `copy-to` en Evo Shell.

## Sintaxis soportada

- `copy-to source, path: destination`
- `copy-to source1, source2, path: destination`
- `copy-to (pipeline |> to-args), path: destination`

## Componentes y Flujo

1. **Parser Agent & Command Resolver (`resolvers/command.rs`):** Parsea tokens posicionales y la opción nombrada `path:`, retornando `Command::CopyTo { sources, destination }`.
2. **Executor Agent (`resolvers/execution.rs`):** Evalúa los argumentos posicionales (resolviendo expresiones agrupadas escalares o secuencias `Arguments` derivadas de `to-args`) y la ubicación destino.
3. **Engine Integration (`copier::copy`):** Invoca la capacidad operativa de copia de `evo-shell-engine` sobre el `filesystem_scope` actual sin mutar el scope.

## Diagramas

- [Architecture Diagram](architecture.d2)
- [Domain Model Diagram](domain-model.d2)
- [Sequence Diagram](sequence.d2)
- [Use Case Diagram](use-case.d2)
