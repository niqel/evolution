# US-018 — Copiar archivos y directorios mediante copy-to

## Historia de usuario

Como usuario de Evo Shell,
quiero copiar archivos y directorios hacia un directorio destino utilizando el comando `copy-to`,
para duplicar elementos individuales, múltiples o arboles completos de directorios, incluyendo la expansión estructurada de argumentos mediante `to-args`, sin alterar el ámbito activo (`filesystem_scope`).

## Descripción

Evo Shell ya permite iterar, filtrar, proyectar y expandir argumentos posicionales con `to-args` (US-009, US-012, US-014), así como agrupar expresiones con `(...)` (US-016, US-017).
Esta historia incorpora el comando real `copy-to`, el cual actúa como consumidor de argumentos posicionales (directos o expandidos estructuradamente) y delega la operación de copia real al motor `evo-shell-engine`.

## Sintaxis

1. **Copia de fuente única:**
   ```text
   copy-to README.md, path: "backup"
   ```

2. **Copia de múltiples fuentes:**
   ```text
   copy-to a.txt, b.txt, folder, path: "/backup"
   ```

3. **Copia con argumento expandido estructuradamente (`to-args`):**
   ```text
   copy-to (
       iter
       |> filter type equals "file"
       |> select name
       |> to-args
   ), path: backup
   ```

4. **Copia recursiva de directorio:**
   ```text
   copy-to documents, path: backup
   ```

## Reglas semánticas

1. **Argumentos posicionales (fuentes):** Una o más rutas fuente (`sources`). Se aceptan literales o expresiones agrupadas que produzcan un `ProjectedValue::Name(name)` (como escalar) o una secuencia `Arguments` de nombres (mediante `to-args`).
2. **Argumento nombrado obligatorio (`path: destination`):** Especifica el directorio destino. Debe resolverse como un directorio existente.
3. **Persistencia de Scope:** `copy-to` no modifica el `filesystem_scope` activo.
4. **Sin sobrescritura silenciosa:** Si un archivo o directorio destino ya existe en la ubicación objetivo, la shell retorna un error tipado.
5. **Rechazo de Self-Copy:** Copiar un directorio sobre sí mismo o dentro de una subcarpeta descendiente es rechazado con error tipado.
6. **Symlinks:** Los symlinks son rechazados con un error tipado sin seguir sus destinos.
7. **Fail-Fast:** La copia se detiene ante el primer error encontrado.

## Fuera de alcance

- Move-to, rename o delete.
- Opciones de sobrescritura (`--force` / `--overwrite`).
- Preservación especial de permisos avanzadas o atributos extendidos.
