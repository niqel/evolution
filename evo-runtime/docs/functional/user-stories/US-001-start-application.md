# US-001 — Iniciar una Aplicación Evo

## Historia

Como un caller/host,
quiero iniciar una Evo Application proporcionando su acción Run,
para que Evo Runtime la mantenga activa hasta que termine y me entregue su Result.

## Contexto

Evo Runtime Model A tiene una responsabilidad mínima y acotada: iniciar una Evo
Application a partir de la acción Run que dicha aplicación proporciona.

Evo Runtime no administra la lógica interna de la aplicación, no resuelve
operaciones, no determina ni selecciona engines, no administra providers ni
capacidades, no transporta Values entre operaciones y no mantiene un Context ni
una entidad Execution propia.

El flujo de control es directo:

1. El Host solicita a Evo Runtime iniciar la aplicación proporcionando su
   acción Run.
2. Evo Runtime invoca la acción Run.
3. La llamada Start permanece activa mientras la acción Run continúe ejecutándose.
4. Cuando la acción Run concluye y entrega un Result, Evo Runtime retorna dicho
   Result al Host.

Múltiples llamadas a Start pueden ejecutarse de forma independiente sin compartir
estado ni interferir funcionalmente entre sí.

## Criterios de Aceptación

- Evo Runtime acepta una acción Run proporcionada por la Evo Application.
- Evo Runtime invoca la acción Run recibida.
- La ejecución de Start permanece activa mientras la acción Run esté activa.
- Cuando la acción Run retorna un Result, Start retorna dicho Result al Host.
- Cada invocación de Start es completamente independiente de otras invocaciones.
- Múltiples invocaciones de Start pueden coexistir funcionalmente.
- El Failure de una invocación de Start no implica ni produce el Failure de otra.
- Evo Runtime no participa ni conoce las operaciones internas, engines o
  providers que la aplicación utilice tras ser iniciada.
- No se requiere una entidad Context.
- No se requiere una entidad Execution.
- No se requiere un Use Case separado de Finalize (la conclusión de Run finaliza
  la llamada Start).

## Fuera de Alcance

Esta historia no define:

- Mecanismos de concurrencia física (hilos del SO, tareas asíncronas, procesos).
- Estructura o lógica interna de la aplicación ejecutada.
- Definición interna de los tipos Result o Failure (pertenecientes a `evo-values`).
- Carga dinámica de extensiones o engines.
- Formatos de paquetes, ejecutables o manifests.
- APIs técnicas o firmas concretas en Rust.
