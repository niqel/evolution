# US-001 — Establecer un scope de sistema de archivos

## Historia de usuario

Como usuario de Evo Shell,
quiero establecer una ubicación del sistema de archivos como mi `scope` activo,
para que las operaciones posteriores trabajen dentro de ese ámbito.

## Descripción

Evo Shell utiliza el concepto de `scope` para determinar el ámbito sobre el que se ejecutan las operaciones.

Para el scope de sistema de archivos, el usuario debe poder indicar una ubicación que se convertirá en el ámbito activo de trabajo.

Ejemplo conceptual:

```text
scope fs "/home/user/documents"
