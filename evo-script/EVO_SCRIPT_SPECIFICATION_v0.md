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

---

## 4. Identificadores y convenciones de nombres

Un identificador (`Identifier`) representa un nombre definido por el programa. Los identificadores se utilizan para nombrar:
- tipos `struct`;
- tipos `enum`;
- variantes de `enum`;
- funciones;
- parámetros;
- bindings;
- campos.

Evo-Script v0 establece dos niveles diferenciados de validez:

```text
forma léxica
    ↓
convención nominal requerida por su contexto
```

La forma léxica determina si una secuencia de caracteres puede ser reconocida por el lexer como un token `Identifier`. La convención nominal determina si ese `Identifier` es válido para la clase de símbolo específica que nombra en el programa.


### 4.1 Gramática de Identifier

La gramática formal de un identificador general se define como:

```text
LowercaseLetter
    ::= "a".."z"

UppercaseLetter
    ::= "A".."Z"

Letter
    ::= LowercaseLetter
     |  UppercaseLetter

Digit
    ::= "0".."9"

Identifier
    ::= Letter IdentifierCharacter*

IdentifierCharacter
    ::= Letter
     |  Digit
     |  "_"
```

Reglas normativas:
1. Todo identificador debe comenzar obligatoriamente con una letra ASCII mayúscula o minúscula (`a-z`, `A-Z`).
2. Tras el primer carácter, un identificador puede contener cualquier combinación de letras ASCII, dígitos (`0-9`) y guiones bajos (`_`).

Ejemplos léxicamente válidos:
- `worker`
- `worker2`
- `worker_id`
- `SearchResult`
- `Value32`
- `HTTPResult`

Ejemplos léxicamente inválidos:
- `2worker` (comienza con un dígito)
- `_worker` (comienza con un guion bajo)
- `__worker` (comienza con un guion bajo)
- `_` (no contiene una letra inicial)
- `worker-name` (contiene el carácter `-`, que no forma parte de Identifier)


### 4.2 Caracteres permitidos

Los identificadores utilizan exclusivamente el subconjunto de caracteres ASCII:
- `A-Z`
- `a-z`
- `0-9`
- `_`

Los caracteres Unicode no ASCII no forman parte de la gramática de `Identifier`.

Ejemplos inválidos como identificadores:
- `niño`
- `México`
- `búsqueda`
- `日本`

Esta restricción aplica estrictamente a los identificadores del lenguaje y no afecta a las cadenas de texto (`StringLiteral`) ni a los comentarios (`//`), donde los caracteres Unicode están permitidos según sus propias reglas.


### 4.3 Sensibilidad a mayúsculas y minúsculas

Los identificadores son estrictamente sensibles a mayúsculas y minúsculas (`case-sensitive`).

Por consiguiente:
- `worker`
- `Worker`
- `WORKER`

representan tres identificadores léxicos distintos. El lenguaje no realiza transformaciones, normalizaciones ni correcciones automáticas de mayúsculas o minúsculas.


### 4.4 Identificadores reservados

Una secuencia de caracteres que coincida exactamente con:
- una palabra reservada estructural (`Structural Keyword`);
- un token literal booleano reservado (`Boolean Literal Token`);
- un nombre de tipo reservado (`Reserved Type Name`);

no puede utilizarse como un identificador definido por el programa.

Ejemplos reservados:
- `let`
- `return`
- `true`
- `false`
- `int`
- `string`
- `float64`

La reserva de identificadores es sensible a mayúsculas y minúsculas:
- `return` es una palabra reservada del lenguaje y no puede usarse como identificador.
- `Return` no coincide con el token reservado `return` y es léxicamente un `Identifier` (su validez dependerá de la convención nominal requerida en el contexto donde se utilice).


### 4.5 Convenciones nominales

Evo-Script v0 define exactamente dos convenciones nominales para los identificadores semánticos del código:
- `PascalCase`
- `snake_case`

Estas convenciones son reglas normativas de validez y no meras recomendaciones de estilo. Un identificador léxicamente válido que no cumpla con la convención nominal requerida por su clase de símbolo hace que el programa sea semánticamente inválido.


### 4.6 PascalCase

Los nombres de las siguientes construcciones deben utilizar estrictamente la convención `PascalCase`:
- tipos `struct`;
- tipos `enum`;
- variantes de `enum`.

La gramática formal de `PascalCaseIdentifier` se define como:

```text
PascalCaseIdentifier
    ::= UppercaseLetter PascalCharacter*

PascalCharacter
    ::= Letter
     |  Digit
```

Reglas normativas:
1. Comienza obligatoriamente con una letra ASCII mayúscula (`A-Z`).
2. Los caracteres posteriores pueden ser letras ASCII (`A-Z`, `a-z`) o dígitos (`0-9`).
3. No contiene guiones bajos (`_`).

Ejemplos válidos:
- `Worker`
- `SearchResult`
- `ApplicationState`
- `Value32`
- `HTTPResult`

Ejemplos inválidos como PascalCase:
- `worker` (no inicia con mayúscula)
- `search_result` (inicia con minúscula y contiene guiones bajos)
- `_Worker` (inicia con guion bajo)
- `Worker_Result` (contiene guion bajo)


### 4.7 snake_case

Los nombres de las siguientes construcciones deben utilizar estrictamente la convención `snake_case`:
- funciones;
- parámetros;
- bindings;
- campos.

La gramática formal de `SnakeCaseIdentifier` se define como:

```text
SnakeCaseIdentifier
    ::= SnakeSegment ("_" SnakeSegment)*

SnakeSegment
    ::= LowercaseLetter SnakeCharacter*

SnakeCharacter
    ::= LowercaseLetter
     |  Digit
```

Reglas normativas:
1. Utiliza exclusivamente letras ASCII minúsculas (`a-z`), dígitos (`0-9`) y guiones bajos (`_`).
2. Comienza obligatoriamente con una letra ASCII minúscula.
3. No puede comenzar con un guion bajo (`_`).
4. No puede terminar con un guion bajo (`_`).
5. No puede contener guiones bajos consecutivos (`__`).
6. Cada segmento separado por guion bajo debe iniciar obligatoriamente con una letra minúscula (`LowercaseLetter`).

Consecuencias normativas:
- `version2` es válido (los dígitos forman parte del segmento que inició con letra).
- `version_2` es inválido (el segmento tras el guion bajo comienza con un dígito).
- `value32` es válido (los dígitos forman parte del segmento que inició con letra).
- `value_32` es inválido (el segmento tras el guion bajo comienza con un dígito).
- `to_int32` es válido (el segmento `int32` comienza con la letra `i` y contiene dígitos posteriormente).

Ejemplos válidos:
- `worker`
- `worker_id`
- `search_worker`
- `calculate_total`
- `value32`
- `to_int32`

Ejemplos inválidos como snake_case:
- `Worker` (contiene mayúsculas)
- `workerId` (contiene mayúsculas)
- `worker__id` (contiene guiones bajos consecutivos)
- `worker_` (termina con guion bajo)
- `_worker` (inicia con guion bajo)
- `version_2` (segmento numérico aislado)
- `value_32` (segmento numérico aislado)
- `search-Worker` (contiene guion medio y mayúscula)


### 4.8 Convención según clase de símbolo

La correspondencia entre la clase de símbolo y su convención nominal se define normativamente en la siguiente tabla:

| Clase de símbolo | Convención nominal obligatoria |
|---|---|
| Tipo `struct` | `PascalCase` |
| Tipo `enum` | `PascalCase` |
| Variante de `enum` | `PascalCase` |
| Función (`fn`, `private fn`, `public fn`) | `snake_case` |
| Parámetro de función | `snake_case` |
| Binding (`let`) | `snake_case` |
| Campo de `struct` | `snake_case` |

Ejemplo ilustrativo:

```text
struct Worker
{
    int worker_id;
    string first_name;
}

enum SearchResult
{
    Found(Worker)
    NotFound
}

fn describe_worker(Worker worker) -> string
{
    let string worker_name = worker.first_name;
    return worker_name;
}
```

Clasificación nominal del ejemplo:
- `Worker`: `PascalCase` (tipo `struct`)
- `SearchResult`: `PascalCase` (tipo `enum`)
- `Found`: `PascalCase` (variante de `enum`)
- `NotFound`: `PascalCase` (variante de `enum`)
- `describe_worker`: `snake_case` (función)
- `worker`: `snake_case` (parámetro)
- `worker_id`: `snake_case` (campo de `struct`)
- `first_name`: `snake_case` (campo de `struct`)
- `worker_name`: `snake_case` (binding local)


### 4.9 Validez léxica y validez nominal

Existe una distinción formal entre validez léxica y validez nominal:

1. **Validez léxica**: determinada por el lexer según la producción `Identifier`. Establece si una secuencia de caracteres constituye un identificador general válido.
2. **Validez nominal**: determinada por el analizador semántico (`semantic analyzer`) según la clase de símbolo declarada. Establece si el identificador satisface la convención `PascalCase` o `snake_case` correspondiente.

Ejemplos:
- La secuencia `search_result` es un `Identifier` léxicamente válido. Sin embargo, la declaración `struct search_result { ... }` es inválida porque los tipos `struct` requieren obligatoriamente `PascalCase`.
- La secuencia `CalculateTotal` es un `Identifier` léxicamente válido. Sin embargo, la declaración `fn CalculateTotal() -> int { ... }` es inválida porque las funciones requieren obligatoriamente `snake_case`.


### 4.10 Nombre físico del archivo .efn

En Evo-Script v0 existe un único tipo de artefacto fuente operativo: el archivo `.efn`.

El nombre físico del archivo `.efn` utiliza exclusivamente la convención `kebab-case`. La convención `kebab-case` aplica únicamente al sistema de archivos y no constituye una convención válida para identificadores dentro del código fuente.

#### 4.10.1 Identidad del archivo

El nombre físico del archivo `.efn` está determinado obligatoriamente por la única función pública (`public fn`) declarada en el programa. La función pública constituye la identidad semántica del script.

La correspondencia nominal entre la función pública y el nombre del archivo es obligatoria y se deriva mediante el siguiente algoritmo determinista:

```text
public function name
    ↓
reemplazar cada "_" por "-"
    ↓
agregar ".efn"
    ↓
physical filename
```

Las letras minúsculas y los dígitos se conservan sin modificación durante la transformación.

Ejemplos obligatorios de correspondencia:
- `public fn search_worker(...)` -> `search-worker.efn`
- `public fn calculate_total(...)` -> `calculate-total.efn`
- `public fn value32(...)` -> `value32.efn`
- `public fn convert_int32(...)` -> `convert-int32.efn`
- `public fn to_int32(...)` -> `to-int32.efn`

#### 4.10.2 Regla de validez nominal del archivo

La correspondencia entre el nombre físico del archivo y la función pública es una regla de validez obligatoria.

Ejemplo válido:
- Archivo en disco: `calculate-total.efn`
- Contenido del archivo:
  ```text
  public fn calculate_total(int value) -> int
  {
      return value;
  }
  ```

Ejemplo inválido:
- Archivo en disco: `calculator.efn`
- Contenido del archivo:
  ```text
  public fn calculate_total(int value) -> int
  {
      return value;
  }
  ```
  Este programa es inválido porque el nombre físico del archivo no corresponde a la función pública `calculate_total`.

#### 4.10.3 Gramática del nombre físico

El nombre base (*basename*) del archivo físico derivado utiliza exclusivamente los caracteres:
- `a-z`
- `0-9`
- `-`

El nombre base físico no puede:
- comenzar con `-`;
- terminar con `-`;
- contener guiones consecutivos (`--`);
- contener guiones bajos (`_`);
- contener letras mayúsculas.

La extensión `.efn` forma parte del nombre físico del archivo en el sistema de archivos, pero no forma parte del identificador semántico de la función dentro del lenguaje (`search_worker` es el identificador semántico de la función; `search-worker.efn` es el nombre físico del archivo).

La derivación nominal opera en un único sentido normativo:

```text
public fn
    ↓
physical .efn filename
```

No se derivan nombres físicos a partir de tipos `struct` o `enum`, ni se permite que un nombre de archivo físico arbitrario determine el nombre de la función pública.

---

## 5. Sistema de tipos

Evo-Script v0 posee un sistema de tipos semántico propio, estático y determinista.

Todo Value producido por una evaluación correcta posee exactamente un tipo semántico. Los tipos del lenguaje se organizan en la siguiente jerarquía conceptual:

```text
SemanticType
    ├── NativeType
    └── ProgramDefinedType

ProgramDefinedType
    ├── StructType
    └── EnumType
```

Un nombre de tipo representa una identidad semántica formal dentro del lenguaje. La representación binaria o en memoria utilizada internamente por una implementación concreta no forma parte del sistema de tipos observable ni de la semántica del lenguaje.


### 5.1 Modelo de tipos

El sistema de tipos de Evo-Script v0 se caracteriza por:
1. **Verificación estática**: la compatibilidad de tipos y la validez de las operaciones se comprueban durante el análisis semántico.
2. **Ausencia de tipos implícitos**: todo parámetro, campo y binding posee un tipo declarado explícitamente.
3. **Inmutabilidad inherente**: los tipos describen Values inmutables; no existen tipos puntero, tipos referencia ni tipos mutables.
4. **Identidad nominal**: los tipos definidos por el programa se diferencian por su nombre unívoco y no por su estructura interna.


### 5.2 Tipos nativos

Evo-Script v0 define exactamente 17 tipos nativos (`NativeType`), clasificados conceptualmente en las siguientes categorías:

```text
Boolean
    bool

Text
    string

SignedInteger
    int
    int8
    int16
    int32
    int64
    int128

UnsignedInteger
    uint8
    uint16
    uint32
    uint64
    uint128

FloatingPoint
    float
    float32
    float64

DynamicNumeric
    dynamic
```

El tipo `dynamic` es exclusivamente un tipo numérico. No representa un tipo genérico, un objeto arbitrario, un contenedor heterogéneo ni habilita despacho dinámico o reflexión.


### 5.3 Aliases `int` y `float`

Evo-Script v0 define exactamente dos aliases semánticos de tipos nativos:

```text
CanonicalType(int)   = int32
CanonicalType(float) = float64
```

Reglas normativas:
1. `int` e `int32` representan exactamente el mismo tipo semántico canónico.
2. `float` y `float64` representan exactamente el mismo tipo semántico canónico.
3. En cualquier contexto del lenguaje, escribir `int` es completamente intercambiable e indistinguible de escribir `int32`.
4. En cualquier contexto del lenguaje, escribir `float` es completamente intercambiable e indistinguible de escribir `float64`.
5. La equivalencia entre `int` e `int32`, y entre `float` y `float64`, no constituye una conversión ni una promoción implícita, sino una identidad canónica absoluta.


### 5.4 Tipos enteros de tamaño fijo

Los tipos enteros de tamaño fijo definen dominios numéricos con límites matemáticos exactos:

#### Tipos enteros con signo (SignedInteger)

- `int8`: $-2^7 \dots 2^7 - 1$ (rango: $-128 \dots 127$)
- `int16`: $-2^{15} \dots 2^{15} - 1$ (rango: $-32\,768 \dots 32\,767$)
- `int32`: $-2^{31} \dots 2^{31} - 1$ (rango: $-2\,147\,483\,648 \dots 2\,147\,483\,647$)
- `int64`: $-2^{63} \dots 2^{63} - 1$ (rango: $-9\,223\,372\,036\,854\,775\,808 \dots 9\,223\,372\,036\,854\,775\,807$)
- `int128`: $-2^{127} \dots 2^{127} - 1$ (rango: $-170\,141\,183\,460\,469\,231\,731\,687\,303\,715\,884\,105\,728 \dots 170\,141\,183\,460\,469\,231\,731\,687\,303\,715\,884\,105\,727$)

El tipo `int` posee exactamente el mismo dominio y rango que `int32`.

#### Tipos enteros sin signo (UnsignedInteger)

- `uint8`: $0 \dots 2^8 - 1$ (rango: $0 \dots 255$)
- `uint16`: $0 \dots 2^{16} - 1$ (rango: $0 \dots 65\,535$)
- `uint32`: $0 \dots 2^{32} - 1$ (rango: $0 \dots 4\,294\,967\,295$)
- `uint64`: $0 \dots 2^{64} - 1$ (rango: $0 \dots 18\,446\,744\,073\,709\,551\,615$)
- `uint128`: $0 \dots 2^{128} - 1$ (rango: $0 \dots 340\,282\,366\,920\,938\,463\,463\,374\,607\,431\,768\,211\,455$)


### 5.5 Tipos de punto flotante

Los tipos de punto flotante representan números reales aproximados según el estándar IEEE 754:
- `float32`: corresponde semánticamente al formato de precisión simple IEEE 754 *binary32*.
- `float64`: corresponde semánticamente al formato de precisión doble IEEE 754 *binary64*.
- `float`: alias semántico exacto de `float64`.

La referencia a los formatos IEEE 754 define el comportamiento y dominio semántico de los tipos reales en el lenguaje, sin condicionar los mecanismos de optimización internos que una implementación pueda emplear.


### 5.6 Tipos `bool` y `string`

#### 5.6.1 Tipo `bool`
El tipo `bool` representa el dominio de valores lógicos del lenguaje, compuesto exclusivamente por los dos valores:
```text
true
false
```

#### 5.6.2 Tipo `string`
El tipo `string` representa una secuencia finita e inmutable de texto Unicode formada por valores escalares Unicode (*Unicode Scalar Values*).

El tipo `string` es un tipo de primer nivel en el lenguaje; no expone punteros, búferes mutables ni detalles de representación en memoria.


### 5.7 Tipo `dynamic`

`dynamic` es un único tipo numérico semántico propio que no equivale a ninguno de los tipos enteros o de punto flotante de tamaño fijo:

```text
dynamic != int8
dynamic != int16
dynamic != int32
dynamic != int64
dynamic != int128

dynamic != uint8
dynamic != uint16
dynamic != uint32
dynamic != uint64
dynamic != uint128

dynamic != float32
dynamic != float64
```

El tipo `dynamic` es exclusivamente numérico. No representa un tipo genérico, un objeto arbitrario, un contenedor heterogéneo ni habilita despacho dinámico o reflexión.

#### 5.7.1 Dominio numérico de `dynamic`

Dentro del tipo `dynamic`, un Value puede representar dos clases de valores numéricos:

```text
dynamic
    ├── integral value
    └── floating value
```

No existen dos tipos visibles independientes; existe un único tipo en el lenguaje denominado `dynamic`. Los términos *integral value* y *floating value* describen la naturaleza concreta del Value contenido dentro de este tipo:

1. **Valores enteros (`integral value`)**: representan números enteros con precisión arbitraria. Su magnitud no está limitada por los rangos finitos de los tipos enteros de tamaño fijo (`int8` a `int128`, `uint8` a `uint128`).
2. **Valores de punto flotante (`floating value`)**: representan números reales cuya semántica numérica corresponde al formato IEEE 754 *binary64*. La coincidencia en el modelo numérico no convierte a `dynamic` en un alias de `float64`; se mantiene estrictamente que `dynamic != float64` y `Compatible(dynamic, float64) = false`.

#### 5.7.2 Independencia de evaluación de expresiones

En Evo-Script v0, el tipo esperado en el destino de una asignación o binding no altera de forma retroactiva el tipo ni la semántica de evaluación de los operandos de una expresión.

Por ejemplo, dados:
```text
int8 a
int8 b
```
la expresión `a + b` se evalúa estrictamente como una operación sobre operandos de tipo `int8`. Si se escribe:
```text
let dynamic result = a + b;
```
el hecho de que la variable destino `result` sea de tipo `dynamic` **no** convierte los operandos `a` y `b` a `dynamic`, **no** altera la semántica de la suma y **no** evita un eventual desbordamiento propio del tipo `int8`. La asignación requiere que el resultado producido sea directamente compatible con `dynamic`.

#### 5.7.3 Operaciones directas y tipado de literales

El tipo `dynamic` puede utilizarse directamente en declaraciones y expresiones cuyos operandos sean de tipo `dynamic`:
```text
let dynamic a = ...;
let dynamic b = ...;
let dynamic result = a + b;
```

Asimismo, de conformidad con las reglas de tipado contextual definidas en el Capítulo 6, un literal numérico en un contexto que requiera `dynamic` producirá directamente un Value de tipo `dynamic` (sea entero o de punto flotante), sin mediar conversiones implícitas desde `int` o `float`.

Las reglas operativas detalladas de las operaciones y combinaciones sobre `dynamic` se definen en el Capítulo 10. Las operaciones de conversión explícita entre `dynamic` y otros tipos numéricos se definen en el Capítulo 11.


### 5.8 Tipos definidos por el programa

El programa puede introducir nuevos tipos de datos mediante declaraciones en el nivel superior del archivo `.efn`:

```text
ProgramDefinedType
    ::= StructType
     |  EnumType
```

- Una declaración `struct Worker` introduce un tipo semántico denominado `Worker`.
- Una declaración `enum SearchResult` introduce un tipo semántico denominado `SearchResult`.

Todos los tipos definidos por el programa pertenecen al mismo archivo `.efn` autocontenido. Dado que las declaraciones de nivel superior pueden escribirse en cualquier orden, las referencias adelantadas entre tipos son plenamente válidas:

```text
struct Node
{
    Element value;
}

struct Element
{
    int id;
}
```

En este ejemplo válido, la referencia a `Element` dentro de `Node` es correcta independientemente de su orden físico en el código fuente.


### 5.9 Identidad nominal de tipos

Los tipos `struct` y `enum` definidos por el programa poseen identidad estrictamente nominal. Dos tipos con nombres distintos representan tipos semánticos diferentes e incompatibles, aun cuando su estructura interna o sus campos sean exactamente idénticos.

Ejemplo:
```text
struct Point
{
    int x;
    int y;
}

struct Coordinate
{
    int x;
    int y;
}
```
Formalmente:
```text
Point != Coordinate
```
A pesar de que `Point` y `Coordinate` declaran campos idénticos con los mismos tipos, constituyen dos tipos completamente independientes en el sistema de tipos.

De igual manera:
```text
enum StateA
{
    Active
    Inactive
}

enum StateB
{
    Active
    Inactive
}
```
produce formalmente:
```text
StateA != StateB
```

Evo-Script v0 no admite equivalencia estructural de tipos.


### 5.10 Type Space y resolución de tipos

Cada archivo `.efn` posee un único espacio de tipos (`Type Space`) que agrupa la totalidad de los tipos reconocidos en el programa:

```text
Type Space = Native Types ∪ Program Defined Types
```

Toda mención de un tipo en firmas de funciones, parámetros, tipos de retorno, campos de structs o bindings locales debe resolver exactamente a una entrada unívoca dentro del Type Space.

#### Unicidad de nombres en el Type Space
Dentro del Type Space de un archivo `.efn`, dos tipos definidos por el programa no pueden compartir el mismo nombre.

Ejemplo inválido:
```text
struct Worker
{
    int id;
}

enum Worker
{
    Empty
}
```
El código anterior es semánticamente inválido porque ambas declaraciones intentan registrar el identificador `Worker` en el mismo Type Space. Las categorías `struct` y `enum` comparten el mismo espacio de nombres de tipos.


### 5.11 Compatibilidad exacta de tipos

La compatibilidad entre dos tipos en Evo-Script v0 se rige por el principio de identidad canónica exacta:

> Dos tipos $A$ y $B$ son directamente compatibles si y solo si su tipo semántico canónico es idéntico.

```text
Compatible(A, B) := CanonicalType(A) == CanonicalType(B)
```

Evaluación de compatibilidad en casos representativos:

```text
Compatible(int, int32)       = true
Compatible(float, float64)   = true

Compatible(int8, int16)      = false
Compatible(int32, int64)     = false
Compatible(int32, uint32)    = false
Compatible(int32, float32)   = false
Compatible(float32, float64) = false

Compatible(Worker, Worker)   = true
Compatible(Worker, Customer) = false

Compatible(dynamic, dynamic) = true
Compatible(dynamic, int32)   = false
Compatible(dynamic, int64)   = false
Compatible(dynamic, float64) = false
```

Evo-Script v0 no incorpora subtipado, covarianza, contravarianza, tipos unión ni tipado estructural.


### 5.12 Ausencia de promociones y conversiones implícitas

Evo-Script v0 no realiza promociones numéricas implícitas, conversiones implícitas ni coerciones automáticas de ningún tipo.

No existen conversiones implícitas entre:
- tipos enteros de diferente tamaño (ej. `int8` $\to$ `int16`, `int32` $\to$ `int64`);
- tipos enteros con signo y sin signo (ej. `int32` $\to$ `uint32`);
- tipos enteros y tipos de punto flotante (ej. `int32` $\to$ `float32`, `int32` $\to$ `float64`);
- tipos de punto flotante de diferente precisión (ej. `float32` $\to$ `float64`);
- tipos numéricos fijos y `dynamic` (ej. `int32` $\to$ `dynamic`, `dynamic` $\to$ `int32`);
- ningún tipo definido por el programa y otro tipo, aun con estructura equivalente.

Toda transformación entre tipos semánticamente distintos debe ser explícita en el código fuente mediante las operaciones normativas definidas en el Capítulo 11 (*Conversión explícita*).

#### Aliases frente a conversiones
Las identidades `int == int32` y `float == float64` no constituyen conversiones implícitas porque ambos identificadores resuelven exactamente al mismo tipo canónico.

#### Tipado de literales numéricos
La asignación de tipo a los literales numéricos (`NumericLiteral`) se rige por las reglas de tipado contextual definidas en el Capítulo 6. El hecho de que un literal como `10` pueda utilizarse para inicializar un `int8` o un `int64` no constituye una conversión implícita entre Values, sino la determinación estática del tipo del propio literal al ser analizado.

---

## 6. Literales y Values

En Evo-Script v0, la evaluación de expresiones y el manejo de datos se rigen por la distinción formal entre tres conceptos fundamentales:

```text
forma textual del literal
    ↓ determinación de su tipo semántico
Value producido
    ↓ asociación de identificador (opcional)
Binding
```

Un literal (`Literal`) pertenece exclusivamente al código fuente. Un valor (`Value`) es el dato semántico inmutable producido por la evaluación correcta de una expresión. Un enlace (`Binding`) asocia posteriormente un identificador a un Value. Por consiguiente, se establece formalmente:

```text
Literal != Value != Binding
```


### 6.1 Modelo de Value

Un Value es un dato semántico inmutable producido durante la evaluación correcta de un programa Evo-Script.

Todo Value posee exactamente un tipo semántico (`SemanticType`), de conformidad con el sistema de tipos definido en el Capítulo 5. Los Values se clasifican conceptualmente en:

```text
Value
    ├── NativeValue
    └── ProgramDefinedValue

ProgramDefinedValue
    ├── StructValue
    └── EnumValue
```

Reglas normativas:
1. **Inmutabilidad inherente**: todo Value en Evo-Script v0 es conceptualmente inmutable desde la semántica observable del lenguaje.
2. **Independencia de Bindings**: un Value existe independientemente de si está asociado a un identificador mediante `let` o si es el resultado directo de una función o pipeline. Por ejemplo, una llamada `calculate()` produce un Value de retorno sin requerir un binding intermedio.
3. **Diferenciación conceptual**:
   - `int` es un tipo (`SemanticType`).
   - `43` es un literal en el código fuente (`IntegerLiteral`).
   - `Value(int, 43)` es el dato semántico producido tras la evaluación.
   - `let int age = 43;` es la sentencia que establece un binding entre el nombre `age` y el Value producido.


### 6.2 Literal Expression

Un literal es una expresión sintáctica constante escrita directamente en el código fuente:

```text
Literal
    ↓ evaluación semántica correcta
Value
```

Las expresiones literales reconocidas en Evo-Script v0 son:

```text
LiteralExpression
    ::= BooleanLiteral
     |  StringLiteral
     |  NumericLiteral
```


### 6.3 BooleanLiteral

Los literales booleanos representan valores lógicos constantes. Su gramática formal está constituida por los tokens reservados:

```text
BooleanLiteral
    ::= "true"
     |  "false"
```

Reglas normativas:
1. `true` produce un Value de tipo `bool` con valor lógico verdadero.
2. `false` produce un Value de tipo `bool` con valor lógico falso.
3. Los literales booleanos **no** poseen tipado contextual hacia otros tipos: producen invariablemente un Value de tipo `bool`.
4. No existen conversiones implícitas de `BooleanLiteral` a tipos numéricos o de texto (por ejemplo, nunca `true -> int`, `true -> string` ni `true -> dynamic`).


### 6.4 StringLiteral

Los literales de cadena de texto representan secuencias constantes de caracteres Unicode. En Evo-Script v0 se delimitan exclusivamente mediante comillas dobles (`"`).

La gramática formal de `StringLiteral` se define como:

```text
StringLiteral
    ::= '"' StringElement* '"'

StringElement
    ::= StringCharacter
     |  EscapeSequence
```

`StringCharacter` representa cualquier valor escalar Unicode (*Unicode Scalar Value*) escrito directamente en el archivo fuente UTF-8, excepto:
- la comilla doble (`"`);
- la barra invertida (`\`);
- el salto de línea LF (`U+000A`);
- el retorno de carro CR (`U+000D`).

Ejemplos válidos:
- `""`
- `"Hello"`
- `"México"`
- `"niño"`
- `"日本"`
- `"😀"`

Todo `StringLiteral` produce directamente un Value de tipo `string`. No posee tipado contextual hacia otros tipos.

#### 6.4.1 Secuencias de escape
Evo-Script v0 reconoce exactamente las siguientes cinco secuencias de escape:

| Secuencia fuente | Carácter resultante | Denominación |
|---|---|---|
| `\"` | `"` (`U+0022`) | Comilla doble |
| `\\` | `\` (`U+005C`) | Barra invertida (backslash) |
| `\n` | `LF` (`U+000A`) | Salto de línea (Line Feed) |
| `\r` | `CR` (`U+000D`) | Retorno de carro (Carriage Return) |
| `\t` | `TAB` (`U+0009`) | Tabulación horizontal |

```text
EscapeSequence
    ::= '\\"'
     |  '\\\\'
     |  '\\n'
     |  '\\r'
     |  '\\t'
```

Cualquier otra secuencia iniciada por `\` (por ejemplo, `\q`, `\x`, `\z`, `\u`) no es reconocida y produce un error léxico. No se preserva silenciosamente la barra invertida de secuencias desconocidas.

#### 6.4.2 Caracteres Unicode y ausencia de escapes especiales
Dado que los archivos `.efn` se codifican estrictamente en UTF-8 sin BOM, los caracteres Unicode se escriben de forma directa en el código fuente. Evo-Script v0 no utiliza secuencias de escape numéricas o Unicode (tales como `\uXXXX` o `\UXXXXXXXX`).

#### 6.4.3 Prohibición de cadenas multilínea físicas
Un `StringLiteral` no puede contener saltos de línea físicos sin escapar. Si el analizador encuentra un carácter `LF`, `CR` o el fin de archivo `EndOfFile` antes de la comilla de cierre (`"`), el literal es inválido. Los saltos de línea en el texto deben representarse mediante la secuencia de escape `\n`.


### 6.5 NumericLiteral

Evo-Script v0 define exactamente tres formas sintácticas para los literales numéricos:

```text
Digit
    ::= "0".."9"

Digits
    ::= Digit+

IntegerLiteral
    ::= Digits

DecimalLiteral
    ::= Digits "." Digits

ScientificLiteral
    ::= (Digits | DecimalLiteral)
        ("e" | "E")
        ("+" | "-")?
        Digits

NumericLiteral
    ::= IntegerLiteral
     |  DecimalLiteral
     |  ScientificLiteral
```

Ejemplos válidos:
- Enteros: `0`, `10`, `1000`
- Decimales: `0.0`, `0.5`, `10.25`, `123.456`
- Científicos: `1e10`, `1E10`, `1e+10`, `1e-10`, `1.5e10`, `1.5E10`, `1.5e+10`, `1.5e-10`

Reglas normativas:
1. **Separador decimal único**: el punto (`.`) es el único separador decimal reconocido. La coma (`,`) no es un separador decimal válido.
2. **Ausencia de separadores de dígitos**: no se permiten guiones bajos (`_`) ni otros separadores dentro de `NumericLiteral` (ej. `1_000`, `1_000.50` y `1e1_000` son inválidos).
3. **Base decimal exclusiva**: los literales numéricos se expresan exclusivamente en base 10 (no se admiten prefijos hexadecimales `0x`, binarios `0b` u octales `0o`).
4. **Ausencia de sufijos de tipo**: los literales no admiten sufijos de tipo (ej. `10i32`, `10u64`, `10.5f32` son inválidos). La determinación del tipo se realiza mediante análisis contextual.
5. **Ausencia de literales especiales**: no existen literales numéricos intrínsecos denominados `NaN`, `Infinity` o `inf`.


### 6.6 IntegerLiteral

Un `IntegerLiteral` está formado exclusivamente por una secuencia de dígitos decimales (`Digits`) sin punto decimal, exponente ni signo integrado.

Reglas normativas:
1. **Tipado contextual entero**: según el tipo esperado (`ExpectedType`) del contexto, un `IntegerLiteral` produce directamente un Value de cualquiera de los tipos enteros del lenguaje:
   - `int` (canónicamente `int32`);
   - `int8`, `int16`, `int32`, `int64`, `int128`;
   - `uint8`, `uint16`, `uint32`, `uint64`, `uint128`;
   - `dynamic` (produciendo un `dynamic integral value`).
2. **Creación directa**: el literal se evalúa directamente en el tipo esperado; no se genera un Value intermedio de tipo por defecto para luego aplicar una conversión.
3. **Tipo por defecto**: en ausencia de un contexto que provea un `ExpectedType` numérico explícito, un `IntegerLiteral` produce un Value de tipo `int` (canónicamente `int32`).
4. **Incompatibilidad con tipos de punto flotante**: un `IntegerLiteral` **no** adquiere directamente un tipo de punto flotante (`float`, `float32`, `float64`) únicamente por contexto. Declaraciones como `let float64 value = 10;` son semánticamente inválidas; debe escribirse una forma decimal como `10.0` o científica como `1e1`.

Ejemplos:
```text
let int8 small = 10;        // Produce Value(int8, 10)
let int64 large = 10;       // Produce Value(int64, 10)
let dynamic value = 10;     // Produce Value(dynamic, integral 10)
```

En ausencia de un `ExpectedType` numérico:
```text
IntegerLiteral("10") + sin ExpectedType -> Value(int, 10) (canónicamente int32)
```


### 6.7 DecimalLiteral

Un `DecimalLiteral` está formado por una secuencia de dígitos antes y después del punto decimal (`Digits "." Digits`).

Reglas normativas:
1. **Dígitos obligatorios**: se requiere al menos un dígito antes y al menos un dígito después del punto decimal. Formas incompletas como `.5` o `5.` son sintácticamente inválidas.
2. **Naturaleza de punto flotante**: todo `DecimalLiteral` es una forma textual de punto flotante. Según el `ExpectedType`, produce directamente un Value de:
   - `float` (canónicamente `float64`);
   - `float32`;
   - `float64`;
   - `dynamic` (produciendo un `dynamic floating value`).
3. **Tipo por defecto**: en ausencia de un `ExpectedType` numérico explícito, un `DecimalLiteral` produce un Value de tipo `float` (canónicamente `float64`).
4. **Incompatibilidad con tipos enteros**: un `DecimalLiteral` no adquiere tipos enteros por contexto. Declaraciones como `let int32 value = 10.5;` son semánticamente inválidas (no se realiza truncamiento ni redondeo implícito).

Ejemplos:
```text
let float32 a = 10.5;       // Produce Value(float32, 10.5)
let float64 b = 10.5;       // Produce Value(float64, 10.5)
let float c = 10.5;         // Produce Value(float64, 10.5)
let dynamic d = 10.5;       // Produce Value(dynamic, floating 10.5)
```


### 6.8 ScientificLiteral

Un `ScientificLiteral` está formado por una mantisa entera o decimal seguida de un indicador de exponente (`e` o `E`), un signo opcional (`+` o `-`) y una secuencia de dígitos exponentes (`Digits`).

Reglas normativas:
1. **Naturaleza de punto flotante**: un `ScientificLiteral` es invariablemente una forma textual de punto flotante, incluso cuando su mantisa no contenga punto decimal o su exponente sea positivo (ej. `1e10` es un literal de punto flotante, no un entero).
2. **Tipado contextual**: según el `ExpectedType`, produce directamente un Value de `float`, `float32`, `float64` o `dynamic` (produciendo un `dynamic floating value`).
3. **Tipo por defecto**: en ausencia de un `ExpectedType` numérico explícito, produce un Value de tipo `float` (canónicamente `float64`).
4. **Incompatibilidad con tipos enteros**: un `ScientificLiteral` no puede inicializar bindings de tipos enteros (ej. `let int64 value = 1e10;` es semánticamente inválido).
5. **Forma canónica y exponente completo**: el exponente debe contener al menos un dígito (formas como `1e`, `1e+` son inválidas). La mantisa debe ser un entero o decimal canónico (formas como `.5e10` o `5.e10` son inválidas; deben escribirse como `0.5e10` o `5.0e10`).


### 6.9 Tipado contextual de literales numéricos

El analizador semántico determina el tipo de un `NumericLiteral` mediante el concepto de tipo esperado (`ExpectedType`):

```text
TypeOf(NumericLiteral, ExpectedType)
```

Cuando un literal numérico se evalúa en una posición sintáctica donde el contexto exige un tipo numérico compatible con su categoría textual, el literal adquiere **directamente** dicho `SemanticType`. No se genera un Value intermedio de tipo por defecto ni se aplica una conversión implícita posterior.

#### 6.9.1 Fuentes de ExpectedType
El `ExpectedType` proviene de las siguientes construcciones del lenguaje:
1. **Declaración de binding tipado**:
   ```text
   let int64 value = 100;
   ```
   `IntegerLiteral("100")` con `ExpectedType(int64)` produce directamente `Value(int64, 100)`.
2. **Argumento en llamada a función**:
   ```text
   fn process(int64 value) -> int64
   {
       return value;
   }

   process(100);
   ```
   El argumento `100` recibe `ExpectedType(int64)` de la firma de la función invocada.
3. **Expresión de retorno tipada**:
   ```text
   fn calculate() -> int64
   {
       return 100;
   }
   ```
   El literal `100` recibe `ExpectedType(int64)` del tipo de retorno declarado.
4. **Inicialización de campos de struct tipados**:
   El tipo declarado para el campo en la definición del `struct` proporciona el `ExpectedType` al instanciar el campo correspondiente.

#### 6.9.2 Tipado por defecto en ausencia de ExpectedType
Cuando no existe un `ExpectedType` numérico provisto por el contexto:
- `IntegerLiteral` adopta el tipo `int` (`int32`).
- `DecimalLiteral` adopta el tipo `float` (`float64`).
- `ScientificLiteral` adopta el tipo `float` (`float64`).


### 6.10 Literales `dynamic`

En concordancia con el Capítulo 5, el tipo `dynamic` es un tipo numérico propio que admite tanto valores enteros de precisión arbitraria como valores de punto flotante IEEE 754 *binary64*.

Reglas normativas:
1. **Literal entero en contexto dynamic**:
   ```text
   let dynamic integer_value = 10;
   ```
   Produce directamente un `Value(dynamic, integral 10)` con semántica de precisión arbitraria, sin conversión implícita desde `int`.
2. **Literal decimal o científico en contexto dynamic**:
   ```text
   let dynamic floating_value = 10.5;
   let dynamic scientific_value = 1e100;
   ```
   Produce directamente un `Value(dynamic, floating ...)` con semántica IEEE 754 *binary64*, sin conversión implícita desde `float`.
3. **No alteración de expresiones con operandos fijos**:
   Un contexto destino de tipo `dynamic` no modifica retroactivamente la evaluación de una expresión formada por operandos de tipos fijos:
   ```text
   int8 a
   int8 b
   let dynamic result = a + b;
   ```
   En este caso, la suma `a + b` se evalúa estrictamente bajo las reglas de `int8`.


### 6.11 Representabilidad de literales

Todo literal numérico debe poder representar un Value válido dentro del dominio del `ExpectedType` asignado. La representabilidad se valida estáticamente durante el análisis semántico.

#### 6.11.1 Literales enteros de tamaño fijo
El valor matemático denotado por un `IntegerLiteral` debe pertenecer estrictamente al rango numérico del tipo entero fijo esperado:
- `let uint8 a = 255;` es válido ($255 \in [0, 255]$).
- `let uint8 b = 256;` es semánticamente inválido ($256 \notin [0, 255]$).
- `let int8 c = 127;` es válido ($127 \in [-128, 127]$).
- `let int8 d = 128;` es semánticamente inválido ($128 \notin [-128, 127]$).

La invalidez de un literal fuera de rango se detecta durante el análisis semántico antes de la ejecución del programa.

#### 6.11.2 Literales enteros en `dynamic`
Un `IntegerLiteral` evaluado bajo `ExpectedType(dynamic)` no está acotado por los límites de los enteros fijos y puede representar cualquier magnitud entera finita.

#### 6.11.3 Literales de punto flotante y redondeo
Para `DecimalLiteral` y `ScientificLiteral`, el valor matemático denotado se mapea semánticamente al formato destino correspondiente (`float32` a IEEE 754 *binary32*, `float64`/`float`/`dynamic` a IEEE 754 *binary64*) utilizando el modo de redondeo estándar **roundTiesToEven**:
- `let float64 value = 0.1;` es válido. No se exige que la fracción decimal tenga una representación binaria exacta; el literal adopta el valor binario más cercano según `roundTiesToEven`.
- Si la magnitud matemática de un literal excede el máximo valor finito representable en el formato destino (produciendo un desbordamiento a infinito), el literal es semánticamente inválido. No se genera silenciosamente `Infinity` o `-Infinity` a partir de un literal.
- Los literales extremadamente pequeños que, bajo `roundTiesToEven`, se aproximen a un número subnormal o a cero son Values válidos.


### 6.12 Signo negativo y NumericLiteral

El carácter `-` no forma parte de la gramática de `IntegerLiteral`, `DecimalLiteral` ni `ScientificLiteral`.

A nivel léxico:
- `-10` se compone del token operador `-` y el token `IntegerLiteral("10")`.
- `-10.5` se compone del token operador `-` y el token `DecimalLiteral("10.5")`.

#### 6.12.1 Regla de representabilidad para literal directo con operador unario `-`
Para permitir la escritura del valor mínimo representable en los tipos enteros con signo (cuyo valor absoluto no es representable como entero positivo del mismo tipo), cuando el operador unario `-` se aplica directamente a un `IntegerLiteral` bajo un `ExpectedType` entero con signo, la representabilidad se comprueba sobre el valor matemático resultante completo de la negación:

```text
-(IntegerLiteral)
```

Consecuencias normativas:
- `let int8 a = -128;` es válido (el valor resultante $-128$ pertenece al dominio $[-128, 127]$ de `int8`).
- `let int8 b = -129;` es semánticamente inválido ($-129 \notin [-128, 127]$).
- `let int16 c = -32768;` es válido ($-32\,768 \in [-32\,768, 32\,767]$).
- `let int16 d = -32769;` es semánticamente inválido.

Esta excepción de representabilidad aplica única y exclusivamente a `IntegerLiteral`. Las formas `DecimalLiteral` y `ScientificLiteral` **no** participan de esta excepción y continúan sin producir tipos enteros:
- `let int8 e = -1.0;` es semánticamente inválido porque `1.0` es un `DecimalLiteral`.
- `let int8 f = -1e0;` es semánticamente inválido porque `1e0` es un `ScientificLiteral`.

Para tipos enteros sin signo (`unsigned`), la aplicación del operador unario `-` sobre un literal numérico es semánticamente inválida:
- `let uint8 value = -1;` es semánticamente inválido porque $-1$ no pertenece al dominio de `uint8`.

#### 6.12.2 Ausencia de operador unario `+`
Evo-Script v0 no define un operador unario `+`. Construcciones como `+10` o `+10.5` son sintácticamente inválidas. El carácter `+` únicamente es válido como signo explícito del exponente dentro de la gramática interna de `ScientificLiteral` (ej. `1e+10`).


### 6.13 Ausencia de `null`

Evo-Script v0 no define una expresión `NullLiteral` ni posee un valor semántico intrínseco `null`.

Reglas normativas:
1. No existe un Value representativo de ausencia o nulidad (`null`, `nil`, `none`, `undefined`).
2. La secuencia `null` no es una palabra reservada del lenguaje (no pertenece a las Structural Keywords del Capítulo 3).
3. Siguiendo las reglas léxicas y nominales, la palabra `null` puede ser utilizada como un `Identifier` ordinario definido por el usuario (por ejemplo, como nombre de parámetro o variable en convención `snake_case`) sin atribuirle ningún significado intrínseco en el sistema de tipos.

---

## 7. Structs locales

En Evo-Script v0, el mecanismo fundamental para modelar la conjunción nominal de datos es la construcción `struct`:

```text
struct = AND data
```

Un `struct` define un tipo semántico nominal (`StructType`), declara un conjunto finito de campos tipados, permite instanciar valores estructurados inmutables (`StructValue`) y proyectar dichos campos mediante el operador de acceso (`.`). Un `struct` no define comportamiento ni métodos; la lógica ejecutable pertenece exclusivamente a las funciones.


### 7.1 Modelo de struct

Un `struct` define un tipo nominal compuesto por la conjunción de todos los campos que declara:

```text
struct Worker
{
    int id;
    string name;
    bool active;
}
```

Conceptualmente:

```text
Worker = id AND name AND active
```

Relación con el sistema de tipos y valores:
- `StructType` pertenece a `ProgramDefinedType`, el cual es un `SemanticType` (Capítulo 5).
- `StructValue` pertenece a `ProgramDefinedValue`, el cual es un `Value` (Capítulo 6).

Se establece formalmente la distinción:

```text
StructType != StructValue
```

- `Worker` es el `SemanticType` nominal que describe la estructura y tipo de los datos.
- `Worker { id: 10, name: "Ana", active: true }` es la expresión sintáctica que produce un `StructValue` concreto cuyo tipo semántico es `Worker`.


### 7.2 Declaración de struct

Una declaración `struct` introduce un nuevo `StructType` en el Type Space del archivo `.efn`. Su gramática formal se define como:

```text
StructDeclaration
    ::= "struct" StructName
        "{"
        StructField*
        "}"

StructName
    ::= PascalCaseIdentifier

StructField
    ::= TypeReference FieldName ";"

FieldName
    ::= SnakeCaseIdentifier
```

Reglas normativas:
1. **Nivel superior**: una `StructDeclaration` solo puede aparecer como `TopLevelDeclaration` dentro del archivo `.efn` (Capítulo 2). No se admiten declaraciones `struct` anidadas dentro de funciones ni dentro de otros structs.
2. **Convenciones nominales**: `StructName` debe utilizar estrictamente `PascalCaseIdentifier`, mientras que cada `FieldName` debe utilizar estrictamente `SnakeCaseIdentifier` (Capítulo 4).
3. **Visibilidad local**: todos los structs declarados en un archivo `.efn` son tipos locales del script autocontenido. No existen modificadores de visibilidad para structs (no existen `public struct` ni `private struct`).
4. **Struct vacío**: la producción `StructField*` permite declaraciones `struct` sin campos:
   ```text
   struct Marker
   {
   }
   ```
   Un struct vacío es un `StructType` nominal válido. Su instanciación se realiza mediante `Marker {}` y posee exactamente un único `StructValue` representable.


### 7.3 Declaración y unicidad de campos

Cada campo dentro de un `struct` se declara especificando su tipo y su nombre, terminado obligatoriamente con un punto y coma:

```text
TypeReference FieldName;
```

Reglas normativas:
1. **Punto y coma obligatorio**: toda declaración de campo debe finalizar con el delimitador `;`.
2. **Unicidad de campos**: dentro de una misma `StructDeclaration`, cada `FieldName` debe aparecer exactamente una vez. Declarar campos con nombres duplicados dentro del mismo `struct` es semánticamente inválido:
   ```text
   // Ejemplo inválido
   struct Worker
   {
       int id;
       string id;
   }
   ```
3. **Independencia nominal entre structs**: diferentes `StructTypes` pueden declarar campos con el mismo nombre. Los campos pertenecen nominalmente a su propio tipo (`Worker.id` y `Customer.id` son campos independientes). No existe un espacio global compartido de nombres de campos.
4. **Identidad nominal de campos**: la identidad de un campo se determina exclusivamente por su nombre (`FieldName`), no por su posición u orden físico de declaración.


### 7.4 Tipos de campos y composición

El tipo declarado para un campo (`TypeReference`) puede ser cualquier `SemanticType` resoluble dentro del `Type Space` del archivo `.efn`:
- tipos nativos (`NativeType`), tales como `int`, `string`, `bool`, `float64`, `dynamic`, etc.;
- tipos struct (`StructType`);
- tipos enum (`EnumType`).

Ejemplo de composición:
```text
struct Country
{
    int id;
    string name;
}

struct Address
{
    string street;
    Country country;
}
```

La declaración `Country country;` establece que un `StructValue` de tipo `Address` contiene como valor de dicho campo un `Value` de tipo `Country`. Esto representa una composición estructural pura de datos inmutables.

#### 7.4.1 Referencias adelantadas de tipos
De conformidad con las reglas del Type Space (Capítulo 5), las declaraciones de nivel superior pueden referenciarse mutuamente sin importar su orden físico en el código fuente:
```text
struct Address
{
    Country country;
}

struct Country
{
    string name;
}
```
En este ejemplo válido, la referencia a `Country` dentro de `Address` es correcta aunque la declaración de `Country` aparezca físicamente después en el archivo.


### 7.5 Struct Construction Expression

La creación de un `StructValue` se expresa en el código fuente mediante una expresión de construcción (`StructConstructionExpression`).

La gramática formal se define como:

```text
StructConstructionExpression
    ::= StructTypeReference
        "{"
        FieldInitializerList?
        "}"

StructTypeReference
    ::= Identifier

FieldInitializerList
    ::= FieldInitializer
        ("," FieldInitializer)*

FieldInitializer
    ::= FieldName ":" Expression
```

#### 7.5.1 Declaración frente a referencia de tipo struct
Existe una distinción formal entre la declaración del nombre de un struct y su referencia en expresiones de construcción:
- `StructName` (definido en la sección 7.2 como `PascalCaseIdentifier`) se utiliza exclusivamente al **declarar** un nuevo tipo `struct`.
- `StructTypeReference` (sintácticamente un `Identifier`) se utiliza al **referenciar** un tipo ya existente para construir un `StructValue`.

A nivel léxico, la secuencia que representa el nombre del tipo (por ejemplo, `Worker`) se procesa invariablemente como un token `Identifier` general (Capítulo 4). Durante el análisis semántico, se aplica la resolución de tipos:

```text
ResolveType(StructTypeReference) -> StructType
```

El `Identifier` utilizado como `StructTypeReference` debe resolver unívocamente en el `Type Space` a un `SemanticType` cuya categoría sea estrictamente `StructType`. Si la referencia no resuelve a un `StructType` (por ejemplo, si resuelve a un `NativeType`, a un `EnumType` o a un símbolo no declarado), la `StructConstructionExpression` es semánticamente inválida.

Reglas normativas:
1. **Coma obligatoria**: los inicializadores de campo (`FieldInitializer`) deben estar separados obligatoriamente por una coma (`,`). Los saltos de línea son espacios en blanco (`whitespace`) y no sustituyen la coma delimitadora.
2. **Ausencia de coma final (trailing comma)**: no se permite una coma después del último inicializador de la lista.
   - `Worker { id: 10, name: "Ana" }` es válido.
   - `Worker { id: 10, name: "Ana", }` es inválido.
   - `Worker { id: 10 name: "Ana" }` es inválido.
3. **Ausencia de palabra clave `new`**: Evo-Script v0 no utiliza la palabra clave `new` para la construcción de valores (construcciones como `new Worker { ... }` o `new Worker(...)` son sintácticamente inválidas).
4. **Ausencia de structs anónimos**: toda construcción requiere indicar explícitamente la referencia nominal del tipo (`StructTypeReference`). Expresiones anónimas como `{ id: 10, name: "Ana" }` son sintácticamente inválidas.
5. **Struct vacío**: un struct sin campos se construye mediante la lista vacía de inicializadores:
   ```text
   Marker {}
   ```
6. **Resultado de la evaluación**: una `StructConstructionExpression` válida produce exactamente un `StructValue` correspondiente al `StructType` resuelto a partir de la `StructTypeReference`.

#### 7.5.2 Obligatoriedad y correspondencia de campos
Para que una `StructConstructionExpression` sea válida, se aplican las siguientes reglas:
1. **Todos los campos son obligatorios**: cada campo declarado en la definición del `struct` debe recibir un inicializador. No existen valores por defecto implícitos ni mecanismos de inicialización parcial o nula (`null`). Omitir un campo hace que la expresión sea semánticamente inválida.
2. **Prohibición de campos desconocidos**: especificar un inicializador para un nombre de campo que no forma parte del `struct` es semánticamente inválido.
3. **Prohibición de campos duplicados**: especificar más de un inicializador para el mismo nombre de campo es semánticamente inválido.
4. **Independencia del orden de inicializadores**: los inicializadores se asocian a los campos por coincidencia de nombre (`FieldName -> Value`) y no por posición. Los siguientes ejemplos producen el mismo `StructValue`:
   ```text
   Worker { id: 10, name: "Ana", active: true }
   Worker { active: true, id: 10, name: "Ana" }
   ```
5. **Orden determinista de evaluación**: las expresiones de los inicializadores se evalúan estrictamente de izquierda a derecha en el orden textual en que aparecen en el código fuente. La identidad nominal de los campos es independiente del orden de evaluación de sus expresiones asociadas:
   ```text
   field identity != expression evaluation order
   ```


### 7.6 Validación y ExpectedType de campos

Cada campo declarado en un `StructType` proporciona un tipo esperado (`ExpectedType`) a la expresión de su `FieldInitializer` correspondiente:

```text
struct Worker
{
    int64 id;
    string name;
}
```

En la construcción:
```text
Worker {
    id: 10,
    name: "Ana"
}
```
el inicializador de `id` recibe `ExpectedType(int64)`. Siguiendo las reglas de tipado contextual de literales del Capítulo 6, el `IntegerLiteral("10")` produce directamente un `Value(int64, 10)` sin requerir conversiones implícitas intermedias.

#### 7.6.1 Expresiones no literales en inicializadores
Cuando el valor de un inicializador es una expresión compuesta o llamada a función:
```text
FieldName: Expression
```
el tipo producido por la expresión (`TypeOf(Expression)`) debe ser directamente compatible con el tipo declarado del campo (`FieldType`):
```text
Compatible(TypeOf(Expression), FieldType) == true
```
de conformidad con la regla de compatibilidad exacta de tipos (Capítulo 5). No se realizan conversiones implícitas durante la construcción de structs; cualquier conversión de tipos debe ser explícita mediante las operaciones definidas en el Capítulo 11.


### 7.7 Field Access

El acceso a campos (`Field Access`) es la expresión que proyecta un campo específico a partir de un `StructValue`.

Su gramática formal se define como:

```text
FieldAccessExpression
    ::= Expression "." FieldName
```

El delimitador punto (`.`) es el operador de proyección de campos sobre un struct.

#### 7.7.1 Tipado del acceso a campos
Si una expresión $E$ produce un valor de tipo $S$ (`TypeOf(E) == S`), donde $S$ es un `StructType`, y $S$ contiene un campo $f$ con tipo $T$ ($S.f : T$), entonces:
```text
TypeOf(E.f) = T
```
`FieldAccessExpression` evalúa la expresión receptora y produce directamente el `Value` asociado al campo proyectado, sin transformaciones ni conversiones.

#### 7.7.2 Restricciones semánticas del receptor
1. **Receptor StructValue**: la expresión situada a la izquierda del punto (`.`) debe evaluar a un `StructValue`. Si el tipo semántico del receptor no es un `StructType`, el acceso a campo es semánticamente inválido.
2. **Campo declarado**: el `FieldName` proyectado debe existir en la declaración del `StructType` receptor. El intento de acceder a un campo no declarado es semánticamente inválido.
3. **Receptores válidos**: cualquier expresión cuyo tipo semántico sea `StructType` puede actuar como receptor de un acceso a campo (por ejemplo, un binding local, el resultado de una función, una expresión de construcción o un acceso a campo previo).
   ```text
   worker.name
   find_worker(10).name
   worker.address.country.name
   ```

#### 7.7.3 Encadenamiento de accesos
El acceso a campos puede encadenarse de forma asociativa hacia la izquierda:
```text
worker.address.country.name
```
se evalúa e interpreta formalmente como:
```text
(((worker.address).country).name)
```
Cada subexpresión intermedia debe producir un `StructValue` válido para permitir el siguiente nivel de proyección.

#### 7.7.4 Exclusividad del punto para proyección de campos
En Evo-Script v0, el operador `.` sobre un `StructValue` se utiliza exclusivamente para la proyección de campos de datos. No define invocación de métodos ni llamadas de miembro (`worker.save()` o `worker.get_name()` no son construcciones válidas). No existen operadores alternativos de acceso (tales como `?.`, `->` o `[]`).

#### 7.7.5 Precedencia
El acceso a campos es una operación posfija de alta precedencia que se resuelve antes que los operadores binarios aritméticos, lógicos y pipelines:
- En `worker.age + 10`, se evalúa primero `worker.age` como `FieldAccessExpression` antes de aplicar la suma.
- En `worker.name |> normalize`, se proyecta primero `worker.name` y su resultado se transfiere como valor de entrada al pipeline.


### 7.8 Inmutabilidad de StructValue

En concordancia con el modelo de Values inmutables definido en los Capítulos 1 y 6, todo `StructValue` es estrictamente inmutable.

Una vez construido un `StructValue`, sus campos no pueden modificarse:
```text
let Worker worker = Worker {
    id: 10,
    name: "Ana"
};

// Expresiones inválidas: no existe asignación a campos
worker.name = "Laura";
worker.id = 20;
```

#### 7.8.1 Reconstrucción de datos
Para representar una modificación en el estado o valor de los datos, se construye un nuevo `StructValue` con los valores actualizados:
```text
let Worker updated_worker = Worker {
    id: worker.id,
    name: "Laura"
};
```
El valor asociado a `worker` permanece intacto e inmutable. `updated_worker` recibe un nuevo `StructValue` independiente. Evo-Script v0 no incorpora sintaxis de actualización destructiva o copia automática por propagación (tales como `with` o `..`).


### 7.9 Composición estructural finita

Toda composición de datos definida por el programa debe poseer una estructura estáticamente finita. Un `StructType` no puede contenerse a sí mismo de manera directa o indirecta, ya que esto generaría un requerimiento de contención estructural infinito.

Ejemplo inválido (recursión estructural directa):
```text
struct Node
{
    int value;
    Node next;
}
```
La dependencia `Node -> Node` produce un ciclo estructural que hace que la declaración sea semánticamente inválida.

Composición finita válida:
```text
struct Country
{
    string name;
}

struct Address
{
    Country country;
}

struct Worker
{
    Address address;
}
```
La cadena de contención `Worker -> Address -> Country` es acíclica y termina en tipos fundamentales, constituyendo una composición estructural finita válida.


### 7.10 Type Dependency Graph y ciclos

La validez estructural de todos los tipos definidos por el programa dentro de un archivo `.efn` se verifica mediante un grafo de dependencias de tipos (`Type Dependency Graph`).

#### 7.10.1 Construcción del grafo
1. Cada `ProgramDefinedType` declarado en el archivo `.efn` constituye un nodo del grafo.
2. Si una declaración `StructType` denominada $A$ contiene un campo cuyo tipo semántico es otro `ProgramDefinedType` denominado $B$, se genera una arista dirigida en el grafo:
   ```text
   A -> B
   ```
3. Los tipos nativos (`NativeType`) no generan aristas hacia otros tipos.

#### 7.10.2 Condición de aciclicidad (DAG)
El `Type Dependency Graph` debe ser estrictamente un grafo dirigido acíclico (**DAG**, *Directed Acyclic Graph*).

Reglas normativas:
1. **Prohibición de recursión directa**: ningún nodo puede poseer una arista hacia sí mismo ($A \to A$).
2. **Prohibición de recursión indirecta**: no puede existir ningún camino dirigido cerrado de longitud arbitraria ($A \to B \to A$, $A \to B \to C \to A$).
3. **Convergencia permitida**: dependencias compartidas donde múltiples tipos dependen de un mismo tipo ($A \to C$ y $B \to C$) son válidas siempre que no formen ciclos.
4. **Independencia del orden**: la detección de ciclos opera sobre la totalidad de las declaraciones de nivel superior del archivo `.efn`. Las referencias adelantadas son válidas si y solo si el grafo global resultante es un DAG.
5. **Integración con EnumType**: el `Type Dependency Graph` es único para todos los tipos definidos por el programa. Las dependencias originadas por variantes de `EnumType` (Capítulo 8) se integran en este mismo grafo, validando que no existan ciclos mixtos entre structs y enums.

---

## 8. Enums locales

En Evo-Script v0, el mecanismo fundamental para modelar la disyunción nominal de alternativas mutuamente excluyentes es la construcción `enum`:

```text
enum = OR alternatives
```

Frente a la conjunción de datos representada por los structs (`struct = AND data`), un `enum` define un tipo semántico nominal (`EnumType`) compuesto por un conjunto cerrado y finito de variantes posibles. Un valor de tipo enum (`EnumValue`) representa exactamente una de las alternativas declaradas en un instante dado, nunca una combinación simultánea de ellas.


### 8.1 Modelo de enum

Un `enum` define un tipo nominal formado por la disyunción de sus variantes:

```text
enum Status
{
    Active,
    Inactive,
    Suspended
}
```

Conceptualmente:

```text
Status = Active OR Inactive OR Suspended
```

Relación con el sistema de tipos y valores:
- `EnumType` pertenece a `ProgramDefinedType`, el cual es un `SemanticType` (Capítulo 5).
- `EnumValue` pertenece a `ProgramDefinedValue`, el cual es un `Value` (Capítulo 6).

Se establece formalmente la distinción:

```text
EnumType != EnumValue
```

- `Status` es el `SemanticType` nominal que describe las alternativas posibles.
- `Status::Active` es la expresión sintáctica que produce un `EnumValue` concreto cuyo tipo semántico es `Status`.

Evo-Script v0 no modela los enums como máscaras de bits (`bit flags`), números enteros subyacentes ni combinaciones concurrentes de estados.


### 8.2 Declaración de enum

Una declaración `enum` introduce un nuevo `EnumType` en el Type Space del archivo `.efn`. Su gramática formal se define como:

```text
EnumDeclaration
    ::= "enum" EnumName
        "{"
        EnumVariantList
        "}"

EnumName
    ::= PascalCaseIdentifier

EnumVariantList
    ::= EnumVariant
        ("," EnumVariant)*
```

Reglas normativas:
1. **Nivel superior**: una `EnumDeclaration` solo puede aparecer como `TopLevelDeclaration` dentro del archivo `.efn` (Capítulo 2). No se admiten declaraciones `enum` anidadas dentro de funciones, structs u otros enums.
2. **Visibilidad local**: todos los enums declarados en un archivo `.efn` son tipos locales del script autocontenido. No existen modificadores de visibilidad (no existen `public enum` ni `private enum`).
3. **Coma obligatoria entre variantes**: las variantes declaradas en `EnumVariantList` deben estar separadas obligatoriamente por una coma (`,`). Los saltos de línea son espacios en blanco (`whitespace`) y no sustituyen la coma delimitadora.
4. **Ausencia de coma final (trailing comma)**: no se permite una coma después de la última variante de la lista.
   - `enum Status { Active, Inactive }` es válido.
   - `enum Status { Active, Inactive, }` es inválido.
5. **Prohibición de enums vacíos**: todo `enum` debe declarar obligatoriamente al menos una variante. Declaraciones sin variantes (`enum Empty {}`) son sintácticamente inválidas. Dado que un enum representa alternativas posibles (`enum = OR alternatives`), un enum vacío carecería de cualquier `EnumValue` representable.


### 8.3 Variantes y unicidad nominal

Cada variante dentro de un `enum` se identifica mediante un nombre de variante (`VariantName`):

```text
VariantName
    ::= PascalCaseIdentifier
```

Reglas normativas:
1. **Convención nominal**: todo `VariantName` debe utilizar estrictamente `PascalCaseIdentifier` (Capítulo 4).
2. **Unicidad de variantes**: dentro de una misma `EnumDeclaration`, cada `VariantName` debe aparecer exactamente una vez. Declarar variantes con nombres duplicados dentro del mismo `enum` es semánticamente inválido:
   ```text
   // Ejemplo inválido
   enum Status
   {
       Active,
       Active
   }
   ```
3. **Reutilización entre enums diferentes**: distintos `EnumTypes` pueden declarar variantes con el mismo nombre. Las variantes pertenecen nominalmente a su propio tipo (`UserStatus::Active` y `ServiceStatus::Active` son variantes independientes). La identidad completa de una variante está determinada por el par `EnumType + VariantName`. No existe un espacio global de variantes compartidas.


### 8.4 Formas de variante

Evo-Script v0 define exactamente tres formas sintácticas y semánticas de variante:

```text
EnumVariant
    ::= SimpleVariant
     |  AssociatedValueVariant
     |  StructuredVariant
```

#### 8.4.1 SimpleVariant
Una variante simple representa una alternativa pura sin carga ni datos adicionales asociados:

```text
SimpleVariant
    ::= VariantName
```

Ejemplo:
```text
NotFound
```

#### 8.4.2 AssociatedValueVariant
Una variante con valor asociado transporta exactamente un valor tipado anónimo:

```text
AssociatedValueVariant
    ::= VariantName "(" TypeReference ")"
```

Ejemplos:
```text
Found(Worker)
Error(string)
Success(int64)
```

Reglas normativas:
1. Transporta exactamente un `TypeReference`.
2. No se permiten variantes con paréntesis vacíos (`Found()`).
3. No se permiten múltiples tipos posicionales ni tuplas (`Found(Worker, int64)` es inválido). Si una variante requiere transportar múltiples datos, debe utilizarse una `StructuredVariant`.

#### 8.4.3 StructuredVariant
Una variante estructurada transporta un conjunto de uno o más campos nombrados y tipados:

```text
StructuredVariant
    ::= VariantName
        "{"
        EnumVariantField+
        "}"

EnumVariantField
    ::= TypeReference FieldName ";"
```

Reglas normativas:
1. Debe declarar al menos un campo (`EnumVariantField+`).
2. No se admiten variantes estructuradas vacías (`Empty {}` es inválido; una variante sin datos debe declararse como `SimpleVariant`).
3. Cada campo se declara mediante `TypeReference FieldName;` y termina obligatoriamente con punto y coma.
4. `FieldName` debe utilizar estrictamente `SnakeCaseIdentifier` y debe ser único dentro de la misma variante estructurada. Distintas variantes pueden reutilizar el mismo `FieldName`.

Ejemplo:
```text
Failed
{
    int code;
    string message;
}
```

#### 8.4.4 Ejemplo combinado de enum
```text
enum SearchResult
{
    NotFound,
    Found(Worker),
    Failed {
        int code;
        string message;
    }
}
```

Conceptualmente:
```text
SearchResult = NotFound OR Found(Worker) OR Failed(code AND message)
```


### 8.5 EnumTypeReference y EnumVariantReference

Para referenciar un tipo enum y sus variantes en expresiones, Evo-Script v0 define las siguientes producciones gramaticales:

```text
EnumTypeReference
    ::= Identifier

EnumVariantReference
    ::= EnumTypeReference
        "::"
        VariantName
```

Reglas normativas:
1. **Reconocimiento léxico**: una referencia como `SearchResult::NotFound` se compone léxicamente del token `Identifier("SearchResult")`, el delimitador `::` y el token `Identifier("NotFound")`. No existen tokens léxicos especiales para enums o variantes.
2. **Resolución del tipo enum**: durante el análisis semántico, el identificador `EnumTypeReference` debe resolver unívocamente en el `Type Space` a un `SemanticType` cuya categoría sea estrictamente `EnumType`:
   ```text
   ResolveType(EnumTypeReference) -> EnumType
   ```
3. **Resolución de la variante**: `VariantName` debe corresponder a una variante válidamente declarada dentro del `EnumType` resuelto:
   ```text
   ResolveVariant(EnumType, VariantName) -> declared variant
   ```
4. **Calificación obligatoria**: las variantes deben referenciarse siempre de forma calificada mediante `EnumType::Variant`. No se permite el uso de nombres de variantes aislados (por ejemplo, escribir `NotFound` directamente como expresión es semánticamente inválido).
5. **Correspondencia de forma entre declaración y construcción**: tras resolver `EnumVariantReference`, el analizador semántico debe validar que la forma de la expresión de construcción utilizada coincida exactamente con la forma declarada de la variante resuelta:
   ```text
   SimpleVariant          <-> SimpleVariantExpression
   AssociatedValueVariant <-> AssociatedValueVariantExpression
   StructuredVariant      <-> StructuredVariantExpression
   ```
   Cualquier discordancia entre la forma declarada y la forma de construcción es semánticamente inválida:
   - Si la variante es `SimpleVariant` (ej. `Result::Empty`), únicamente puede construirse mediante `SimpleVariantExpression` (`Result::Empty`). Intentar construirla como `Result::Empty(worker)` o `Result::Empty { value: worker }` es semánticamente inválido.
   - Si la variante es `AssociatedValueVariant` (ej. `Result::Found(Worker)`), únicamente puede construirse mediante `AssociatedValueVariantExpression` (`Result::Found(worker)`). Escribir `Result::Found` (sin valor asociado) o `Result::Found { worker: worker }` es semánticamente inválido.
   - Si la variante es `StructuredVariant` (ej. `Result::Failed { string message; }`), únicamente puede construirse mediante `StructuredVariantExpression` (`Result::Failed { message: "error" }`). Escribir `Result::Failed` (sin carga estructurada) o `Result::Failed("error")` es semánticamente inválido.


### 8.6 Construcción de variante simple

La instanciación de una `SimpleVariant` se realiza referenciando directamente la variante calificada:

```text
SimpleVariantExpression
    ::= EnumVariantReference
```

Ejemplo:
```text
Status::Active
```

Reglas normativas:
1. Produce un `EnumValue` cuyo tipo semántico es el `EnumType` correspondiente y cuya variante activa es la indicada.
2. No se utilizan paréntesis en la construcción de variantes simples (`Status::Active()` es sintácticamente inválido).


### 8.7 Construcción de variante con Value asociado

La instanciación de una `AssociatedValueVariant` se realiza suministrando exactamente una expresión entre paréntesis tras la referencia calificada:

```text
AssociatedValueVariantExpression
    ::= EnumVariantReference
        "(" Expression ")"
```

Ejemplo:
```text
SearchResult::Found(worker)
```

Reglas normativas:
1. **Cardinalidad exacta**: debe proporcionarse exactamente una expresión (`SearchResult::Found()` y `SearchResult::Found(w1, w2)` son inválidos).
2. **ExpectedType del valor asociado**: el `TypeReference` declarado en la variante proporciona el `ExpectedType` a la expresión argumento. Por ejemplo, si se declara `Success(int64)`, en `ParseResult::Success(10)` el literal `10` recibe `ExpectedType(int64)` y produce directamente un `Value(int64, 10)`.
3. **Compatibilidad exacta**: para expresiones no literales, el valor producido debe ser directamente compatible con el tipo declarado de la variante:
   ```text
   Compatible(TypeOf(Expression), PayloadType) == true
   ```
   No se aplican conversiones implícitas.


### 8.8 Construcción de variante estructurada

La instanciación de una `StructuredVariant` se realiza especificando los inicializadores de sus campos entre llaves:

```text
StructuredVariantExpression
    ::= EnumVariantReference
        "{"
        EnumFieldInitializerList
        "}"

EnumFieldInitializerList
    ::= EnumFieldInitializer
        ("," EnumFieldInitializer)*

EnumFieldInitializer
    ::= FieldName ":" Expression
```

Ejemplo:
```text
SearchResult::Failed {
    code: 500,
    message: "Internal Error"
}
```

Reglas normativas:
1. **Coma obligatoria**: los inicializadores de campo (`EnumFieldInitializer`) deben separarse obligatoriamente por una coma (`,`).
2. **Ausencia de trailing comma**: no se permite una coma tras el último inicializador.
3. **Obligatoriedad de todos los campos**: cada campo declarado en la variante estructurada debe recibir un inicializador. Omitir un campo es semánticamente inválido.
4. **Prohibición de campos desconocidos o duplicados**: no se permiten inicializadores para campos no declarados en la variante ni inicializadores duplicados para el mismo campo.
5. **Identidad nominal de campos**: la asociación de inicializadores a campos se realiza por nombre (`FieldName -> Value`) y no por posición.


### 8.9 ExpectedType y evaluación de cargas

Cada campo o carga declarado en una variante de enum proporciona un tipo esperado (`ExpectedType`) a la expresión inicializadora correspondiente:

1. **Cargas de AssociatedValueVariant**:
   ```text
   enum Result
   {
       Success(int64)
   }

   Result::Success(10) // 10 recibe ExpectedType(int64)
   ```
2. **Campos de StructuredVariant**:
   ```text
   enum Event
   {
       Movement {
           int64 x;
           int64 y;
       }
   }

   Event::Movement {
       x: 10,  // 10 recibe ExpectedType(int64)
       y: 20   // 20 recibe ExpectedType(int64)
   }
   ```
3. **Compatibilidad estricta**: las expresiones no literales deben producir valores directamente compatibles con el tipo esperado de la carga. No existen conversiones implícitas.
4. **Orden de evaluación**: en `StructuredVariantExpression`, las expresiones de los inicializadores se evalúan estrictamente de izquierda a derecha en el orden textual en que aparecen en el código fuente:
   ```text
   field identity != expression evaluation order
   ```


### 8.10 Modelo e inmutabilidad de EnumValue

Un `EnumValue` representa una instancia de datos estructurada conceptualmente como:

```text
EnumValue
    ├── EnumType
    ├── ActiveVariant
    └── VariantPayload
```

El contenido de `VariantPayload` depende de la forma de la variante activa:
- En una `SimpleVariant`, no existe payload (no se utiliza `null`, `none` ni ningún valor especial para denotar ausencia de carga).
- En una `AssociatedValueVariant`, el payload está constituido por exactamente un `Value`.
- En una `StructuredVariant`, el payload está constituido por la conjunción inmutable de los `Values` de sus campos.

Reglas normativas:
1. **Inmutabilidad inherente**: todo `EnumValue` es inmutable. No es posible reasignar campos ni modificar la variante activa de un valor existente. Para representar un estado o alternativa diferente se construye un nuevo `EnumValue`.
2. **Ausencia de discriminantes numéricos**: las variantes no poseen ordinales enteros visibles ni representaciones numéricas públicas en el lenguaje. No existe sintaxis del tipo `Variant = 0`. La identidad de cada variante es puramente nominal.


### 8.11 Acceso y selección de alternativas

Existe una distinción formal entre valores de tipo struct y valores de tipo enum:

```text
EnumValue != StructValue
```

Por consiguiente, el operador de acceso a campos (`.`) definido en el Capítulo 7 **no** aplica directamente sobre un `EnumValue`. Expresiones como `result.message` son semánticamente inválidas sobre un enum, dado que `message` solo existe cuando la variante activa es aquella que declara dicho campo.

#### 8.11.1 Selección de alternativas mediante `when`
Para inspeccionar la variante activa de un `EnumValue` y acceder a su carga de datos, el programa debe utilizar la expresión `when` (definida formalmente en el Capítulo 12).

#### 8.11.2 Distinción formal entre `::` y `.`
Evo-Script v0 diferencia estrictamente los roles sintácticos de `::` y `.`:
- `::` se utiliza exclusivamente para calificar variantes dentro de un tipo enum (`SearchResult::Found`).
- `.` se utiliza exclusivamente para proyectar campos a partir de un valor de tipo struct (`worker.name`).

No se permite el uso de `.` para calificar variantes ni el uso de `::` para proyectar campos.


### 8.12 EnumType en el Type Dependency Graph

Las dependencias de contención estructural introducidas por las declaraciones `enum` se integran en el `Type Dependency Graph` único del archivo `.efn` (definido en la sección 7.10).

#### 8.12.1 Generación de aristas
1. **SimpleVariant**: no contiene otros valores y no genera aristas en el grafo.
2. **AssociatedValueVariant**: si un `EnumType` $A$ declara una variante `Variant(B)`, donde $B$ es un `ProgramDefinedType` (`StructType` o `EnumType`), se genera la arista dirigida:
   ```text
   A -> B
   ```
   Si el tipo asociado es un `NativeType` (por ejemplo, `string` o `int32`), no se genera ninguna arista.
3. **StructuredVariant**: si una variante estructurada de un `EnumType` $A$ declara un campo de tipo $B$, donde $B$ es un `ProgramDefinedType`, se genera la arista dirigida:
   ```text
   A -> B
   ```

#### 8.12.2 Condición de aciclicidad (DAG)
El `Type Dependency Graph` conjunto (que integra todos los `StructTypes` y `EnumTypes` del archivo) debe ser estrictamente un grafo dirigido acíclico (**DAG**).

Reglas normativas:
1. **Prohibición de recursión directa**: ningún enum puede contenerse a sí mismo directamente ($Node \to Node$). La siguiente declaración es semánticamente inválida:
   ```text
   // Inválido: ciclo directo
   enum Node
   {
       End,
       Next(Node)
   }
   ```
2. **Prohibición de recursión indirecta**: no se admiten ciclos dirigidos entre enums ($A \to B \to A$).
3. **Prohibición de ciclos mixtos struct-enum**: no se admiten ciclos formados por dependencias combinadas entre structs y enums ($Worker \to WorkerResult \to Worker$). El siguiente ejemplo es semánticamente inválido:
   ```text
   // Inválido: ciclo mixto
   struct Worker
   {
       WorkerResult result;
   }

   enum WorkerResult
   {
       Empty,
       Found(Worker)
   }
   ```
4. **Referencias adelantadas válidas**: las referencias adelantadas entre enums y structs son plenamente válidas siempre que el grafo global resultante no contenga ciclos.
