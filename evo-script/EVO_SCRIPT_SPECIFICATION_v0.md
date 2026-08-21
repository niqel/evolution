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
