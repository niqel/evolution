# Evo Runtime Specification v0

Evo Runtime es la plataforma de ejecución y composición del ecosistema Evo. Su propósito es definir de manera formal y determinista cómo se inicia, compone, prepara y ejecuta una aplicación Evo.

El Runtime establece la frontera operativa entre unidades ejecutables, capabilities, contracts, providers y contextos de ejecución. Su diseño permite la composición modular y la carga bajo demanda, garantizando que una aplicación solo cargue y prepare los recursos estrictamente necesarios para cada ejecución.

Esta especificación mantiene una separación arquitectónica estricta:
- **Evo-Script**: lenguaje de scripting autocontenido cuyo comportamiento en archivos `.efn` se encuentra formalizado de manera independiente en su propia especificación.
- **Evo Runtime**: plataforma que interpreta la composición y selección de entrada de una aplicación para preparar, resolver y ejecutar unidades ejecutables y capabilities.
- **Engines y productos (Evo-Shell, Evo-CLI, Evo-UI, EvoQ)**: herramientas e interfaces construidas sobre Evo Runtime que reutilizan su infraestructura de ejecución.

---

## Índice

1. Propósito y alcance de Evo Runtime v0
2. Modelo de aplicación Evo
3. Archivo .main
4. Archivo .root
5. Unidades ejecutables
6. Resolución y composición
7. Dependency Closure
8. Carga y preparación bajo demanda
9. Values en la frontera del Runtime
10. Invocation y Execution Context
11. Capabilities
12. Contracts
13. Providers
14. Scopes
15. Pipelines entre capabilities y providers
16. Failures y fronteras de error
17. Lifecycle del Runtime
18. Determinismo y reglas de resolución
19. Gramática y formato consolidado de .main y .root
20. Aplicación canónica autocontenida

---

## Mapa inicial de la especificación

### 1. Propósito y alcance de Evo Runtime v0
Define la frontera del Runtime respecto a Evo-Script, hosts, engines y providers.

### 2. Modelo de aplicación Evo
Define conceptualmente qué elementos forman una aplicación ejecutable por Evo Runtime.

### 3. Archivo .main
Formalizará cómo una aplicación identifica su operación inicial.

### 4. Archivo .root
Formalizará el composition root funcional de una aplicación.

### 5. Unidades ejecutables
Definirá qué elementos puede preparar e invocar el Runtime y cómo se relacionan con unidades como `.efn`.

### 6. Resolución y composición
Definirá cómo una dependencia o capability requerida se resuelve contra la composición declarada.

### 7. Dependency Closure
Definirá cómo se determina el conjunto transitivo de elementos requeridos por una ejecución.

### 8. Carga y preparación bajo demanda
Formalizará la diferencia entre elementos conocidos, resueltos, cargados, preparados e inicializados.

### 9. Values en la frontera del Runtime
Definirá cómo argumentos y resultados atraviesan la frontera entre Runtime, unidades ejecutables y capabilities.

### 10. Invocation y Execution Context
Formalizará cómo se invoca una operación y qué contexto runtime participa durante su evaluación.

### 11. Capabilities
Definirá qué representa una capacidad disponible para una ejecución.

### 12. Contracts
Definirá la descripción abstracta de operaciones que pueden ser satisfechas por implementaciones externas.

### 13. Providers
Definirá cómo infraestructura, librerías Rust, sistema operativo o servicios externos implementan capabilities/contracts.

### 14. Scopes
Definirá los contextos runtime asociados a capabilities/providers sin confundir Scope con prompt, Provider o lenguaje Evo-Script.

### 15. Pipelines entre capabilities y providers
Definirá cómo Values pueden fluir entre operaciones pertenecientes a diferentes capabilities/providers.

### 16. Failures y fronteras de error
Distinguirá errores de composición, carga, resolución, provider e `EvaluationFailure`.

### 17. Lifecycle del Runtime
Definirá creación, preparación, ejecución, reutilización y finalización de recursos/runtime contexts.

### 18. Determinismo y reglas de resolución
Definirá cuándo una composición es válida y cómo evitar resolución ambigua.

### 19. Gramática y formato consolidado de .main y .root
Consolidará únicamente después de cerrar la semántica previa la sintaxis normativa de ambos formatos.

### 20. Aplicación canónica autocontenida
Servirá como prueba integral de la especificación del Runtime, de forma análoga al programa canónico de Evo-Script v0.

---

## Principios rectores de diseño

Los siguientes principios arquitectónicos guían la dirección de la futura especificación:

1. **Carga bajo demanda (On-demand / Lazy execution)**: Evo Runtime debe poder ejecutar una aplicación sin cargar ni inicializar capacidades que dicha ejecución no necesita.
2. **Diferenciación de estados de ciclo de vida**: se distingue conceptualmente entre los estados de un elemento dentro del runtime:
   ```text
   known != resolved != loaded != prepared != initialized
   ```
3. **Composición abstracta desacoplada**: la composición completa de una aplicación puede ser conocida y validada sin necesidad de materializar todas sus implementaciones en memoria.
4. **Desacoplamiento de artefactos de configuración**:
   - `.root` representa la declaración de composición; **no** es el Runtime.
   - `.main` representa la selección de inicio; **no** es el Runtime.
   - **Evo Runtime** es la plataforma que interpreta y utiliza ambos para preparar y orquestar una ejecución.
5. **Independencia del lenguaje Evo-Script**: Evo-Script define formalmente el comportamiento de programas `.efn` y permanece completamente independiente de la semántica del Runtime, de Evo-Shell y de providers concretos.
6. **Reutilización transversal por engines**: Evo Runtime proporciona la infraestructura base sobre la cual engines como Evo-CLI, Evo-UI o EvoQ, así como hosts externos, pueden integrarse de manera homogénea.
7. **Providers como frontera de infraestructura**: los providers representan adaptadores de infraestructura o implementaciones externas, cuyo modelo exacto se formalizará en el Capítulo 13.
8. **Claridad conceptual de Scope**: el concepto de Scope designa un contexto runtime de resolución y no debe confundirse con prompts, providers ni construcciones léxicas del lenguaje Evo-Script.

---

## Estado inicial

Documento creado como scaffold de diseño y mapa de trabajo.
Ningún capítulo normativo ha sido desarrollado todavía.
