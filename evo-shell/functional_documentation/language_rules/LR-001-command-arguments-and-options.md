# LR-001 — Command Arguments and Options

## Propósito

Esta regla formaliza una convención sintáctica transversal de Evo Shell.

Su objetivo es distinguir conceptualmente entre:

- comandos;
- argumentos;
- múltiples argumentos;
- opciones;
- flags;
- valores de opciones.

Esta regla no define la gramática completa del lenguaje.

Tampoco define el comportamiento funcional completo de comandos futuros.

## Regla general

Conceptualmente, un comando puede escribirse como:

```text
command [arguments] [options]
```

Esta expresión describe la forma general de lectura.

No es todavía una especificación BNF, EBNF ni una gramática completa de Evo Shell.

## Comando

Un comando representa la acción que el usuario solicita a Evo Shell.

Ejemplos:

```text
enter src
```

Aquí:

```text
enter
```

es el comando.

```text
iter
```

también es un comando.

## Argumentos

Un argumento representa un dato que el comando consume o sobre el cual opera.

Ejemplo:

```text
enter src
```

Aquí:

```text
enter
```

es el comando.

```text
src
```

es el argumento.

Otro ejemplo conceptual:

```text
index 0
```

Aquí:

```text
index
```

es el comando.

```text
0
```

es el argumento.

Este ejemplo solo documenta la forma sintáctica de un comando con argumento.

No implementa ni especifica todavía el comando `index`.

## Múltiples argumentos

Cuando un comando recibe múltiples argumentos homogéneos, la sintaxis aprobada usa coma.

Ejemplo conceptual:

```text
select name, size
```

Aquí:

```text
select
```

es el comando.

```text
name
```

es un argumento.

```text
size
```

es otro argumento.

La coma separa elementos pertenecientes a la misma lista de argumentos.

Evo Shell prefiere una sintaxis natural dentro de pipelines:

```text
select name, size
```

No se introduce una forma de llamada estilo función:

```text
select(name, size)
```

Esta regla no establece una prohibición absoluta de paréntesis para todo Evo Shell o Evo Script.

Los paréntesis podrían tener otros usos legítimos en decisiones futuras.

## Pipelines

Los argumentos deben funcionar naturalmente dentro de pipelines.

Ejemplo conceptual:

```text
iter |> index 0 |> select name, size |> first
```

Interpretación conceptual:

- `iter` produce una iteración estructurada.
- `index 0` selecciona por índice utilizando `0` como argumento.
- `select name, size` conserva los campos `name` y `size`.
- `first` garantiza como transformación posterior un único elemento cuando corresponda según su contrato futuro.

Este ejemplo documenta sintaxis e intención.

No implementa ni especifica completamente:

- `index`;
- `select`;
- `first`;
- pipelines.

## Opciones

Una opción modifica cómo se ejecuta un comando.

Las opciones largas comienzan con:

```text
--
```

Ejemplo conceptual:

```text
clear --all
```

Aquí:

```text
clear
```

es el comando.

```text
--all
```

es una opción.

La opción no representa el objeto sobre el que opera `clear`.

Modifica el comportamiento del comando.

Este ejemplo solo motiva la distinción entre comando y opción.

No implementa ni especifica completamente:

- `clear`;
- `clear --all`.

## Flags

Una opción que no necesita un valor y representa un estado booleano puede denominarse flag.

Por tanto:

```text
--all
```

es conceptualmente un flag.

Relación:

```text
flag ⊂ option
```

Es decir:

- todo flag es una opción;
- no toda opción necesariamente será un flag.

## Opciones con valor

Una opción puede eventualmente recibir un valor.

Ejemplo gramatical hipotético:

```text
command --depth 3
```

Aquí:

```text
command
```

es un comando hipotético.

```text
--depth
```

es una opción hipotética.

```text
3
```

es el valor de la opción.

Este ejemplo no implica que `--depth` exista actualmente.

Tampoco define el comportamiento de opciones con valor para ningún comando concreto.

## Distinción fundamental

Un argumento responde conceptualmente a:

```text
¿qué dato consume el comando?
¿sobre qué opera?
```

Una opción responde conceptualmente a:

```text
¿cómo debe comportarse el comando?
```

Ejemplos:

```text
enter src
```

`src` es argumento porque representa el dato sobre el que opera `enter`.

```text
clear --all
```

`--all` es opción y flag porque modifica cómo se comporta `clear`.

## Guiones

La forma:

```text
--name
```

queda reservada para opciones largas.

No se usa `--` para argumentos normales.

Ejemplos correctos:

```text
enter src
index 0
select name, size
clear --all
```

Ejemplos no deseados:

```text
enter --src
index --0
select --name, --size
```

## Coma

La única regla aprobada actualmente para coma es:

puede separar múltiples argumentos homogéneos de un comando cuando su contrato acepta una lista.

Ejemplo:

```text
select name, size
```

La coma no se generaliza todavía como:

- `AND`;
- separador universal de expresiones;
- separador universal de parámetros;
- operador lógico.

## Opciones cortas

LR-001 no define opciones cortas.

No se asume todavía que formas como estas sean válidas:

```text
-a
-h
-r
clear -a
```

La sintaxis de opciones cortas se decidirá posteriormente.

## Filter

LR-001 no utiliza `filter` como ejemplo de múltiples argumentos.

Esta regla no decide todavía qué representa la coma dentro de expresiones de filtrado.

Quedan fuera de esta regla:

- `AND`;
- `OR`;
- listas de expresiones;
- precedencia lógica;
- agrupación;
- paréntesis;
- operadores booleanos.

## Principio de diseño

Los argumentos representan datos para el comando.

Las opciones representan modificaciones de comportamiento.

Los pipelines conectan transformaciones.

La sintaxis debe permanecer legible sin introducir símbolos innecesarios.

## Decisiones diferidas

LR-001 no resuelve todavía:

- opciones cortas;
- aliases de opciones;
- `filter`;
- `AND` / `OR`;
- precedencia de expresiones;
- agrupación;
- significado de coma dentro de expresiones;
- opciones repetibles;
- opciones con múltiples valores;
- orden entre argumentos y opciones;
- terminador de opciones `--`;
- escaping;
- quoting;
- validación de opciones desconocidas.

## Fuera de alcance

LR-001 no define ni implementa:

- `clear`;
- `clear --all`;
- `index`;
- `select`;
- `first`;
- `filter`;
- pipelines;
- Evo Script;
- lexer;
- parser;
- AST;
- implementación Rust.
