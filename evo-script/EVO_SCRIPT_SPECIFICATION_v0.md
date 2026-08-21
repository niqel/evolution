# Evo-Script Language Specification v0

Evo-Script v0 define formalmente el núcleo mínimo y autocontenido del lenguaje Evo-Script.

En esta versión:
- Un programa en Evo-Script v0 vive y se ejecuta dentro de un único archivo fuente `.efn`.
- `.efn` constituye el único artefacto fuente operativo del lenguaje en v0.
- Los conceptos de `struct` y `enum` forman parte del lenguaje como construcciones locales declaradas dentro del mismo archivo `.efn` que las utiliza.
- Cada archivo `.efn` contiene exactamente una función pública (`public fn`) y puede contener cero o más funciones privadas (`fn` o `private fn`).
- No existe en v0 sistema de módulos, imports, archivos externos de tipos, firmas abstractas (`.esig`), composición (`.root`, `.main`, `.elib`) ni integración con capacidades externas o Evo-Shell.
- El propósito exclusivo de la especificación v0 es consolidar y cerrar formalmente el núcleo computacional y semántico del lenguaje antes de introducir mecanismos de modularidad y composición entre archivos.
