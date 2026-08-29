# Evo-Script Engine — D2 Sequence Diagrams

Status: TECHNICAL DESIGN — NOT STARTED

Este directorio contendrá los **D2 Sequence Diagrams** de `evo-script-engine`.

Los diagramas de secuencia representan el comportamiento dinámico de una operación ya diseñada.

Responden:

> ¿Qué firma invoca a qué firma, en qué orden y con qué datos o capacidades?

Las lifelines representan Consumers, módulos participantes o fronteras técnicas reales. Las flechas deben identificar la acción o firma invocada.

Regla:

> Toda interacción mostrada en un D2 Sequence Diagram debe corresponder a una firma explícita o a una llamada técnica explícitamente definida por el Technical Design.

Los diagramas no pueden inventar dependencias ocultas ni pasos que no puedan rastrearse hacia Rust Signatures y Participants cerrados.

Los archivos fuente `.d2` son documentación versionable y forman parte del diseño técnico.
