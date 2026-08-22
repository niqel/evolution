# User Stories de Evo Runtime

Este directorio contiene las historias de usuario funcionales que describen las necesidades, capacidades y comportamiento observable de **Evo Runtime** desde la perspectiva del usuario o consumidor del sistema.

## Convenciones de documentación

1. **Un archivo por historia de usuario**: cada historia de usuario se documenta de forma independiente en su propio archivo Markdown.
2. **Convención de nombres**:
   ```text
   US-001-start-application.md
   US-002-<descripcion-kebab-case>.md
   ```
3. **Enfoque funcional y de comportamiento observable**: las historias describen *qué* necesita el usuario y *cómo se comporta* el sistema ante diferentes escenarios.
4. **Independencia técnica**: las historias de usuario **no** deben contener detalles de implementación en Rust, ni definir Agents, function pointers, Providers, estructuras internas ni decisiones de bajo nivel.
5. **Trazabilidad con casos de uso**: posteriormente, cada historia de usuario podrá relacionarse formalmente con uno o más casos de uso de la arquitectura técnica.
