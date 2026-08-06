# US-017 — Usar el resultado de una expresión agrupada como argumento de un comando

## Historia de usuario

Como usuario de Evo Shell,
quiero pasar el resultado de una expresión agrupada `(...)` como argumento de un comando (como `enter`),
para que la shell evalúe primero la expresión interior de forma tipada y utilice su valor producido como el argumento de ejecución del comando exterior.

## Descripción

Evo Shell ya permite agrupar y evaluar expresiones entre paréntesis `(...)` (US-016) y navegar entre directorios con el comando `enter` (US-005).
Esta historia amplía el parser y el ejecutor para permitir que un comando posicional reciba una expresión agrupada como su argumento.

Ejemplo canónico:

```text
enter (
    iter
    |> filter type equals "directory"
    |> filter name equals "child"
    |> select name
    |> to-value
)
```

## Flujo semántico

1. **Reconocimiento del argumento agrupado (`command.rs`):** El parser de `enter` identifica un token de apertura `(` en la posición del argumento y resuelve recursivamente la expresión agrupada como `CommandArgument::Grouped(Box<Command>)`.
2. **Evaluación prioritaria (`execution.rs`):** Antes de ejecutar `enter`, el ejecutor resuelve la expresión interior utilizando el `filesystem_scope` actual y obtiene su `PipelineValue` resultante.
3. **Conversión y validación de tipo:**
   - Si la expresión interior produce un valor escalar tipado (`PipelineValue::Value(...)`), se extrae su representación de ruta y se entrega a `enter`.
   - Si la expresión interior produce un resultado incompatible (múltiples valores `PipelineValue::Values` o argumentos variádicos `PipelineValue::Arguments` sin comando receptor variádico), la shell retorna un error tipado `ExecuteError::IncompatibleGroupedArgument`.
4. **Ejecución y Presentación:**
   - El resultado intermedio de la expresión interior NO se imprime ni se presenta en stdout.
   - `enter` se ejecuta con el argumento resuelto y altera el `filesystem_scope` según su comportamiento estándar.

## Reglas de multilínea y comillas

- Una expresión agrupada como argumento mantiene la solicitud de lectura multilínea `... > ` mientras el paréntesis `(` no haya sido cerrado.
- Las expresiones literales existentes como `enter child` o `enter "child"` continúan funcionando sin alteraciones.

## Fuera de alcance

- Creación de nuevos comandos artificiales como `copy-to`.
- Reparsing sintáctico a texto intermedio (la composición se realiza exclusivamente a nivel de dominio y ejecutor).
