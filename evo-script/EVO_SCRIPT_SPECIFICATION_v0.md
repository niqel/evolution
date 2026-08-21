# Evo-Script Language Specification v0

Evo-Script v0 define formalmente el núcleo mínimo y autocontenido del lenguaje Evo-Script.

En esta versión:
- Un programa en Evo-Script v0 vive y se ejecuta dentro de un único archivo fuente `.efn`.
- `.efn` constituye el único artefacto fuente operativo del lenguaje en v0.
- Los conceptos de `struct` y `enum` forman parte del lenguaje como construcciones locales declaradas dentro del mismo archivo `.efn` que las utiliza.
- Cada archivo `.efn` contiene exactamente una función pública (`public fn`) y puede contener cero o más funciones privadas (`fn` o `private fn`).
- No existe en v0 sistema de módulos, imports, archivos externos de tipos, firmas abstractas (`.esig`), composición (`.root`, `.main`, `.elib`) ni integración con capacidades externas o Evo-Shell.
- El propósito exclusivo de la especificación v0 es consolidar y cerrar formalmente el núcleo computacional y semántico del lenguaje antes de introducir mecanismos de modularidad y composición entre archivos.

---

## Índice de contenidos

1. Propósito y alcance de Evo-Script v0
2. Archivo fuente .efn
3. Reglas léxicas
4. Identificadores y convenciones de nombres
5. Sistema de tipos
6. Literales y Values
7. Structs locales
8. Enums locales
9. Bindings inmutables con let
10. Expresiones y operadores
11. Conversión explícita
12. Expresión when
13. Functions
14. Function Calls
15. Pipelines
16. Evaluación y errores
17. Gramática consolidada de Evo-Script v0
18. Programa canónico autocontenido
19. Elementos explícitamente fuera de v0

---

## 1. Propósito y alcance de Evo-Script v0

Evo-Script v0 define el núcleo mínimo y autocontenido del lenguaje Evo-Script. El objetivo de esta versión es establecer de forma completa y precisa las reglas fundamentales necesarias para escribir, analizar y ejecutar un script Evo-Script.

Un programa en Evo-Script v0 se encuentra completamente contenido dentro de un único archivo `.efn`. El archivo `.efn` constituye la unidad completa del programa y puede contener:
- cero o más declaraciones `struct`;
- cero o más declaraciones `enum`;
- cero o más funciones privadas;
- exactamente una función pública.

La función pública constituye la única operación expuesta por el archivo y se declara siempre mediante la palabra clave `public fn`. No se utiliza `pub fn`.

Las funciones privadas pueden declararse de forma explícita mediante `private fn` o simplemente mediante `fn`. Toda función declarada únicamente mediante `fn` es una función privada.


### 1.1 Script autocontenido

Todo elemento necesario para interpretar y ejecutar un programa Evo-Script v0 se encuentra declarado dentro del mismo archivo `.efn`. Las declaraciones `struct`, las declaraciones `enum` y las funciones auxiliares pertenecen al mismo programa y pueden ser utilizadas por las funciones del archivo de acuerdo con las reglas establecidas por esta especificación.

El archivo `.efn` constituye una unidad autocontenida de:
- análisis léxico;
- parsing;
- validación semántica;
- evaluación.


### 1.2 Modelo del lenguaje

Evo-Script v0 trabaja mediante Values y transformaciones.

Las funciones reciben Values mediante sus parámetros, realizan operaciones sobre ellos y producen un Value como resultado. Los Values son inmutables.

Las transformaciones pueden expresarse mediante:
- llamadas a funciones;
- pipelines.


### 1.3 Alcance del núcleo

Evo-Script v0 define formalmente los siguientes componentes del núcleo del lenguaje:
- tipos nativos;
- aliases de tipos;
- literales;
- Values;
- declaraciones `struct`;
- declaraciones `enum`;
- bindings inmutables mediante `let`;
- expresiones;
- operadores;
- conversiones explícitas;
- expresión `when`;
- funciones;
- llamadas a funciones;
- `return`;
- pipelines;
- reglas de evaluación;
- errores del lenguaje.


### 1.4 Criterio de definición completa

Evo-Script v0 se considera completamente definido cuando esta especificación permita implementar de forma determinista la totalidad de la cadena de procesamiento del lenguaje:

```text
source text
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic analyzer
    ↓
evaluator
```

sin requerir decisiones adicionales sobre la sintaxis o la semántica del lenguaje. Este criterio formal establece el cierre técnico normativo de la especificación v0.
