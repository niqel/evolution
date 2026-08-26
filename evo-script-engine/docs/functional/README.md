# Evo-Script Engine — Documentación Funcional

Status: FUNCTIONAL DESIGN — NOT CLOSED

Este directorio contiene la documentación de diseño funcional para el componente
`evo-script-engine`.

## Secuencia de Diseño Arquitectónico

El desarrollo formal y la especificación de `evo-script-engine` sigue esta
secuencia de diseño canónica:

1. **User Stories**: Captura de objetivos funcionales desde la perspectiva de los Consumers.
2. **Data Dictionary**: Consolidación del vocabulario funcional canónico.
3. **Use Cases**: Derivación de acciones funcionales discretas individuales.
4. **Functional Coverage**: Verificación de trazabilidad completa desde historias a use cases.
5. **Functional Model Closed**: Cierre formal del modelo funcional.
6. **Technical Interfaces**: Definición de function pointers de frontera y puntos de entrada.
7. **Technical Data Model**: Definición de representaciones técnicas y ownership.
8. **Sequence Diagrams**: Documentación de flujos de interacción en runtime.
9. **Structure Diagrams**: Documentación de relaciones modulares y de componentes.
10. **Technical Mapping**: Mapeo de definiciones a archivos y tipos concretos.
11. **Technical Model Closed**: Cierre formal del diseño técnico.
12. **Evo-Script Engine Specification v0**: Documento de especificación normativa.
13. **Rust Implementation**: Código concreto del crate y verificación automatizada.

---

## Organización del Directorio

- [`user-stories/`](user-stories/README.md): User Stories funcionales candidatas y cerradas (*FUNCTIONAL CLOSED*).
- [`DATA_DICTIONARY.md`](DATA_DICTIONARY.md): Vocabulario funcional canónico para evo-script-engine v0 (*FUNCTIONAL CLOSED*).
- [`use-cases/`](use-cases/README.md): Use Cases funcionales candidatos y cerrados (*FUNCTIONAL DESIGN — IN PROGRESS*).
