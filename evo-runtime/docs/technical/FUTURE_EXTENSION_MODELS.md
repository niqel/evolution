# Modelos de Extensión Futuros — Evo Runtime

Status: DESIGN NOTE — NOT CLOSED

Esta nota registra consideraciones arquitectónicas futuras respecto a extensiones
de engines compilados y descargables para el ecosistema Evo.

> [!IMPORTANT]
> **Fuera de Alcance para Model A**
>
> Los modelos arquitectónicos discutidos en este documento **NO** forman parte de
> Evo Runtime Model A. Evo Runtime Model A no resuelve, no carga ni administra
> engines o providers.

---

## 1. Futura Extensión de Engines Compilados

En fases de evolución futuras más allá de Model A, podría existir el
requerimiento de agregar, instalar o cargar engines compilados dinámicamente
(por ejemplo, `evo-api-rest-engine` o drivers especializados de bases de datos)
sin requerir la recompilación de Evo Runtime ni de toda la aplicación anfitriona.

### Temas Arquitectónicos Abiertos

Los siguientes temas permanecen abiertos para el diseño técnico futuro:

1. **ABI Estable de Engines**: Definición de compatibilidad de interfaz binaria
   entre versiones del compilador y plataformas.
2. **Formato de Paquete y Distribución**: Definición de empaquetado, manifiestos y
   metadatos para la distribución de engines compilados.
3. **Engine Loader / Host**: Diseño de un mecanismo dedicado de carga de engines
   independiente de la coordinación del runtime central.
4. **Binarios por Plataforma**: Gestión de artefactos binarios específicos por
   arquitectura y sistema operativo.
5. **Versionado y Compatibilidad**: Negociación de versiones entre aplicaciones y
   engines dinámicos.
6. **Instalación y Descubrimiento**: Rutas del sistema de archivos y registros
   para la búsqueda de engines.

---

## 2. Aclaración de Frontera

Los modelos de extensión futuros deben preservar el invariante central de diseño:
- Evo Runtime no se convierte en un registro de engines ni en un service locator.
- La carga dinámica de engines, si se implementa, pertenecerá a un subsistema
  dedicado de extensiones y no complicará la frontera mínima de runtime de Model A.
