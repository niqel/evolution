# US-001 — Iniciar una Aplicación Evo

## Historia

Como Host,
quiero iniciar una Evo Application proporcionando su acción Run,
para que Evo Runtime inicie su ejecución
y mantenga activa la llamada Start
hasta que Run termine.

## Contexto

Evo Runtime Model A tiene una responsabilidad mínima y acotada: iniciar una Evo
Application a partir de la acción Run que dicha aplicación proporciona.

Evo Runtime no administra la lógica interna de la aplicación, no resuelve
operaciones, no determina ni selecciona engines, no administra providers ni
capacidades, no transporta Values entre operaciones, no transporta outcomes y no
mantiene un Context ni una entidad Execution propia.

El flujo de control es directo:

1. El Host solicita a Evo Runtime iniciar la aplicación proporcionando su
   acción Run.
2. Evo Runtime invoca la acción Run.
3. La llamada Start permanece activa mientras la acción Run continúe ejecutándose.
4. Cuando la acción Run concluye naturalmente, la llamada Start retorna
   naturalmente al Host.

Múltiples llamadas a Start pueden ejecutarse de forma independiente sin compartir
estado ni interferir funcionalmente entre sí.

## Criterios de Aceptación

- Evo Runtime acepta una acción Run proporcionada por la Evo Application.
- Evo Runtime invoca la acción Run recibida.
- La ejecución de Start permanece activa mientras la acción Run esté activa.
- Cuando la acción Run termina, Start termina naturalmente.
- Cada invocación de Start es completamente independiente de otras invocaciones.
- Múltiples invocaciones de Start pueden coexistir funcionalmente.
- La terminación de una invocación de Start no termina ni altera otra invocación.
- Evo Runtime no participa ni conoce las operaciones internas, engines o
  providers que la aplicación utilice tras ser iniciada.
- No se requiere una entidad Context.
- No se requiere una entidad Execution.
- No se requiere un Use Case separado de Finalize (la conclusión de Run finaliza
  naturalmente la llamada Start).

## Fuera de Alcance

Esta historia no define:

- Mecanismos de concurrencia física (hilos del SO, tareas asíncronas, procesos).
- Estructura o lógica interna de la aplicación ejecutada.
- Semántica de outcomes propios de la Evo Application.
- Carga dinámica de extensiones o engines.
- Formatos de paquetes, ejecutables o manifests.
- APIs técnicas o firmas concretas en Rust.
