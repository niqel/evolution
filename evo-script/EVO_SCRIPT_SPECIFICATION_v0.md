# Evo-Script Language Specification v0

Evo-Script v0 define formalmente el núcleo mínimo y autocontenido del lenguaje Evo-Script.

En esta versión:
- Un programa en Evo-Script v0 vive y se ejecuta dentro de un único archivo fuente `.efn`.
- `.efn` constituye el único artefacto fuente operativo del lenguaje en v0.
- Los conceptos de `struct` y `enum` forman parte del lenguaje como construcciones locales declaradas dentro del mismo archivo `.efn` que las utiliza.
- Cada archivo `.efn` contiene exactamente una función pública (`public fn`) y puede contener cero o más funciones privadas (`fn` o `private fn`).
- El propósito de la especificación v0 es consolidar y cerrar formalmente el núcleo computacional y semántico del lenguaje.

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

Toda declaración propia de un programa Evo-Script v0 se encuentra dentro del mismo archivo `.efn`. El programa no requiere otros archivos fuente para definir sus structs, enums o funciones.

Las declaraciones `struct`, las declaraciones `enum` y las funciones auxiliares pertenecen al mismo programa y pueden ser utilizadas por las funciones del archivo de acuerdo con las reglas establecidas por esta especificación.

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

---

## 2. Archivo fuente .efn

Un archivo `.efn` representa un programa completo de Evo-Script v0. Su contenido está formado exclusivamente por declaraciones de nivel superior (`TopLevelDeclaration`).

Las declaraciones permitidas en un archivo `.efn` son:
- declaraciones `struct`;
- declaraciones `enum`;
- declaraciones de función (`fn`, `private fn`, `public fn`).

Un archivo `.efn` válido contiene:
- cero o más declaraciones `struct`;
- cero o más declaraciones `enum`;
- cero o más funciones privadas;
- exactamente una función pública.

Estructuralmente, un archivo `.efn` se compone de:

```text
.efn
├── struct      0..N
├── enum        0..N
├── private fn  0..N
└── public fn   exactamente 1
```

La única función pública constituye la operación expuesta por el programa.


### 2.1 Declaraciones de nivel superior

Únicamente las siguientes construcciones sintácticas pueden aparecer directamente en el nivel superior de un archivo `.efn`:
- `struct`
- `enum`
- `fn`
- `private fn`
- `public fn`

Los bindings, expresiones y sentencias propios del cuerpo de una función no pueden aparecer directamente en el nivel superior. Por consiguiente, no son válidos en el nivel superior del archivo:
- declaraciones de binding `let`;
- llamadas a funciones (`Function Calls`);
- pipelines (`|>`);
- sentencias `return`;
- expresiones ejecutables.

Ejemplo inválido:

```text
let int value = 10;

public fn calculate() -> int
{
    return value;
}
```

El programa anterior es inválido porque el binding `let` aparece en el nivel superior del archivo fuera del cuerpo de una función.


### 2.2 Orden de las declaraciones

Las declaraciones de nivel superior pueden escribirse en cualquier orden dentro del archivo `.efn`. El orden físico en el código fuente no determina si una declaración puede ser referenciada por otra declaración del mismo programa.

Antes de validar las referencias entre tipos y funciones, todas las declaraciones de nivel superior del archivo `.efn` deben ser reconocidas como pertenecientes al mismo programa.

Esto permite que una función invoque o haga referencia a otra función o tipo que aparezca posteriormente en el archivo:

```text
public fn calculate(int value) -> int
{
    return double(value);
}

fn double(int value) -> int
{
    return value * 2;
}
```

En este ejemplo válido, la posición posterior de la función `double` no impide que `calculate` pueda referenciarla e invocarla.


### 2.3 Función pública del programa

Todo archivo `.efn` válido debe contener exactamente una función declarada mediante `public fn`.

No puede existir un archivo `.efn` válido con:
- cero funciones públicas;
- más de una función pública.

La única función pública constituye la operación expuesta por el programa.

Ejemplo válido:

```text
fn double(int value) -> int
{
    return value * 2;
}

public fn calculate(int value) -> int
{
    return double(value);
}
```

Ejemplo inválido:

```text
public fn first() -> int
{
    return 1;
}

public fn second() -> int
{
    return 2;
}
```

El ejemplo anterior es inválido porque el archivo contiene más de una función pública.


### 2.4 Funciones privadas

Un archivo `.efn` puede contener cero o más funciones privadas.

Las siguientes dos formas de declaración son semánticamente equivalentes respecto a su visibilidad:

```text
fn calculate() -> int
{
    return 10;
}
```

y:

```text
private fn calculate() -> int
{
    return 10;
}
```

La palabra clave `private` hace explícita una visibilidad que ya está implícita cuando se utiliza únicamente la palabra clave `fn`. Por lo tanto:
- `fn` define una función privada.
- `private fn` define una función privada explícita.

La visibilidad pública se declara siempre de forma completa mediante `public fn`. No se utiliza ninguna abreviación como `pub`.


### 2.5 Estructura sintáctica general

La estructura sintáctica general de nivel superior de un archivo `.efn` se define como:

```text
SourceFile
    ::= TopLevelDeclaration* EndOfFile

TopLevelDeclaration
    ::= StructDeclaration
     |  EnumDeclaration
     |  FunctionDeclaration
```

Esta estructura sintáctica expresa las categorías de declaraciones que pueden existir directamente dentro del archivo fuente. Las reglas detalladas para `StructDeclaration`, `EnumDeclaration` y `FunctionDeclaration` se definen formalmente en sus respectivos capítulos.

Adicionalmente a la estructura sintáctica, se aplica la siguiente restricción semántica sobre `SourceFile`:

```text
cantidad de PublicFunctionDeclaration = 1
```

La gramática permite una secuencia de declaraciones de nivel superior, mientras que la regla de contener exactamente una función pública constituye una restricción semántica obligatoria sobre la validez del programa `SourceFile`.

---

## 3. Reglas léxicas

El análisis léxico transforma el contenido textual de un archivo `.efn` en una secuencia ordenada de tokens según la cadena de procesamiento:

```text
source text
    ↓
lexer
    ↓
tokens
```

El lexer reconoce las siguientes categorías léxicas:
- palabras reservadas (`Structural Keywords`);
- tokens literales reservados (`Boolean Literal Tokens`);
- nombres de tipos reservados (`Reserved Type Names`);
- identificadores (`Identifier`);
- literales (`BooleanLiteral`, `NumericLiteral`, `StringLiteral`);
- delimitadores;
- símbolos estructurales y operadores;
- comentarios;
- espacios en blanco (`whitespace`).

Los espacios en blanco y los comentarios participan en el análisis léxico como separadores y delimitadores, pero no producen tokens evaluables en la secuencia de salida entregada al parser.


### 3.1 Codificación del archivo fuente

Todo archivo `.efn` de Evo-Script v0 debe utilizar estrictamente codificación UTF-8 sin marca de orden de bytes (BOM).

Reglas normativas:
1. Un archivo que contenga secuencias de bytes que no representen texto UTF-8 válido es léxicamente inválido y produce un error léxico (`lexical error`).
2. La presencia de una marca de orden de bytes UTF-8 (BOM, secuencia de bytes `0xEF, 0xBB, 0xBF`) al inicio del archivo es inválida y produce un error léxico.
3. Caracteres Unicode arbitrarios pueden existir válidamente dentro de construcciones léxicas cuya gramática lo permita expresamente, en particular dentro de cadenas de texto (`StringLiteral`) y comentarios (`//`).


### 3.2 Espacios en blanco y saltos de línea

Evo-Script v0 reconoce exactamente cuatro caracteres como espacio en blanco (`whitespace`):
- Espacio en blanco (`space`): `U+0020`
- Tabulación horizontal (`tab`): `U+0009`
- Salto de línea (`Line Feed`, `LF`): `U+000A`
- Retorno de carro (`Carriage Return`, `CR`): `U+000D`

La secuencia formada por un retorno de carro seguido inmediatamente por un salto de línea (`CRLF`, `\r\n`) se procesa como espacio en blanco.

Reglas normativas:
1. El espacio en blanco separa tokens contiguos cuando sea necesario para evitar ambigüedades léxicas.
2. El espacio en blanco no produce Values ni posee semántica en tiempo de ejecución.
3. Evo-Script v0 no es sensible a la indentación (`indentation-insensitive`). La indentación no abre, cierra ni determina bloques de código; los bloques se delimitan exclusivamente mediante `{` y `}`.
4. Los saltos de línea no terminan sentencias. Toda construcción sintáctica que requiera terminación explícita utiliza el punto y coma (`;`).

Ejemplo de equivalencia léxica:

```text
let int value = 10;
```

y:

```text
let
    int
    value
    =
    10
    ;
```

ambas formas producen exactamente la misma secuencia ordenada de tokens.


### 3.3 Comentarios

Evo-Script v0 define una única forma léxica de comentario mediante la secuencia `//`.

Reglas normativas:
1. Toda secuencia `//` que aparezca fuera de un literal de texto inicia un comentario de línea.
2. El comentario iniciado mediante `//` termina ante cualquiera de las siguientes condiciones:
   - un salto de línea individual `LF` (`U+000A`);
   - un retorno de carro individual `CR` (`U+000D`);
   - una secuencia `CRLF` (`\r\n`), la cual constituye una única terminación física de línea para este propósito;
   - el fin de archivo (`EndOfFile`) si el archivo concluye antes de encontrar una terminación de línea.
3. El contenido textual del comentario es ignorado por el lexer y no produce tokens en la secuencia de salida.
4. La secuencia `//` ubicada dentro de un literal de texto (`StringLiteral`) no inicia un comentario. Por ejemplo, en `"https://example.com"`, la secuencia `//` forma parte del valor del texto.
5. Evo-Script v0 no define comentarios multilínea (`/* ... */`).
6. Evo-Script v0 no define sintaxis especiales para comentarios de documentación. Una secuencia como `/// comentario` se interpreta normalmente como un comentario de línea estándar cuyo contenido textual comienza con el carácter `/`, sin constituir una categoría léxica distinta ni producir un error léxico.


### 3.4 Palabras reservadas

Una palabra es una palabra reservada (`Structural Keyword`) si y solo si forma parte activa de la gramática formal de Evo-Script v0.

La lista oficial y exhaustiva de palabras reservadas estructurales en Evo-Script v0 está constituida exactamente por:

```text
struct
enum
fn
public
private
let
return
when
this
```

Reglas normativas:
1. Las palabras reservadas no pueden utilizarse como identificadores definidos por el usuario (nombres de funciones, parámetros, bindings, structs, enums, variantes o campos).
2. Las palabras reservadas son estrictamente sensibles a mayúsculas y minúsculas (`case-sensitive`). La palabra `return` es una keyword reservada, mientras que secuencias como `Return` o `RETURN` no corresponden al token keyword `return`.


### 3.5 Tokens literales reservados

Evo-Script v0 define exactamente dos tokens literales booleanos reservados:

```text
true
false
```

Reglas normativas:
1. Los tokens `true` y `false` están léxicamente reservados y no pueden utilizarse como identificadores definidos por el usuario.
2. Estos tokens no se clasifican como palabras reservadas estructurales (`Structural Keywords`), sino como literales booleanos cuyo valor y tipado en el sistema de tipos se definen formalmente en el Capítulo 6.


### 3.6 Nombres de tipos reservados

Evo-Script v0 define la categoría léxica `Reserved Type Names`. Los nombres de tipos nativos y aliases del lenguaje están reservados a nivel léxico y no pueden utilizarse como identificadores definidos por el usuario en ninguna posición (funciones, parámetros, bindings, structs, enums, variantes o campos).

La lista oficial y exhaustiva de nombres de tipos reservados en Evo-Script v0 es:

```text
bool
string
dynamic

int
int8
int16
int32
int64
int128

uint8
uint16
uint32
uint64
uint128

float
float32
float64
```


### 3.7 Delimitadores

El lexer de Evo-Script v0 reconoce los siguientes delimitadores sintácticos individuales:

```text
(
)
{
}
,
;
.
:
::
```

Reglas normativas:
1. El lexer reconoce estos caracteres y secuencias como tokens delimitadores individuales.
2. La secuencia `::` se reconoce léxicamente como un único token compuesto indivisible.
3. El significado sintáctico y el rol de cada delimitador se determinan por las construcciones gramaticales en las que participan.


### 3.8 Símbolos estructurales y operadores

El lexer reconoce los siguientes símbolos estructurales y operadores como tokens individuales:

Símbolos estructurales compuestos:
```text
->
=>
|>
```

Operadores y símbolos de expresiones y bindings:
```text
=
+
-
*
/
%
!
==
!=
<
<=
>
>=
&&
||
```

Reglas normativas:
1. Los símbolos `->`, `=>` y `|>` son reconocidos como tokens individuales indivisibles.
2. El símbolo `=` es un token reconocido por el lenguaje cuya utilización válida se restringe a las construcciones gramaticales que lo requieran (como bindings inmutables).
3. La semántica de evaluación, precedencia, asociatividad y tipado de cada operador se define formalmente en el Capítulo 10.


### 3.9 Reconocimiento de tokens compuestos

El lexer aplica formalmente el principio de coincidencia del token más largo (*longest valid token match*):

> Cuando varios tokens válidos comienzan con el mismo prefijo de caracteres, el lexer selecciona siempre el token válido más largo que inicia en la posición actual del flujo de lectura.

Esta regla determina unívocamente la tokenización de símbolos con prefijos compartidos:
- Ante `::`, el lexer produce el token único `::` (y no dos tokens `:` sucesivos).
- Ante `==`, el lexer produce el token único `==` (y no dos tokens `=` sucesivos).
- Ante `=>`, el lexer produce el token único `=>` (y no `=` seguido de `>`).
- Ante `>=`, el lexer produce el token único `>=` (y no `>` seguido de `=`).
- Ante `<=`, el lexer produce el token único `<=` (y no `<` seguido de `=`).
- Ante `!=`, el lexer produce el token único `!=` (y no `!` seguido de `=`).
- Ante `&&`, el lexer produce el token único `&&`.
- Ante `||`, el lexer produce el token único `||`.
- Ante `|>`, el lexer produce el token único `|>` (y no un carácter `|` seguido de `>`).
- Ante `->`, el lexer produce el token único `->` (y no `-` seguido de `>`).


### 3.10 Identificadores y literales

El lexer reconoce como categorías léxicas fundamentales:
- `Identifier`: nombres definidos por el usuario para funciones, parámetros, bindings, structs, enums, variantes y campos.
- `BooleanLiteral`: literales booleanos (`true`, `false`).
- `NumericLiteral`: literales numéricos enteros y de punto flotante.
- `StringLiteral`: literales de texto delimitados por comillas dobles (`"`).

La gramática formal detallada, reglas de caracteres y convenciones de `Identifier` se definen en el Capítulo 4. La gramática formal, secuencias de escape y representación de los literales se definen en el Capítulo 6.


### 3.11 Caracteres no reconocidos

Todo carácter o secuencia de bytes que no pueda formar válidamente un espacio en blanco, comentario, palabra reservada, token literal reservado, nombre de tipo reservado, delimitador, símbolo estructural, operador, identificador o literal válido según las reglas de esta especificación produce un error léxico (`lexical error`).

El lexer:
1. No ignora silenciosamente caracteres no reconocidos.
2. No intenta realizar corrección o recuperación automática de errores léxicos.
3. No produce tokens aproximados o ambiguos.


### 3.12 Fin de archivo

Al consumir completamente la totalidad del contenido del archivo fuente en codificación UTF-8, el lexer produce el token conceptual de fin de archivo:

```text
EndOfFile
```

Este token señala la conclusión formal de la secuencia de tokens y satisface la regla de producción sintáctica del nivel superior definida en el Capítulo 2:

```text
SourceFile
    ::= TopLevelDeclaration* EndOfFile
```
