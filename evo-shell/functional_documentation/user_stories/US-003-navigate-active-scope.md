# US-003 — Navegar dentro del scope activo mediante el comando in

## Historia de usuario

Como usuario de Evo Shell,
quiero entrar en una ubicación desde mi scope activo mediante el comando `in`,
para que los comandos posteriores operen desde esa nueva ubicación.

## Descripción

Evo Shell permite navegar dentro del scope activo mediante el comando:

```text
in
```

La intención conceptual de `in <location>` es entrar en una ubicación desde la ubicación activa actual.

Por ahora, esta historia define únicamente el comportamiento observable cuando el scope activo es un filesystem scope.

Evo Shell distingue conceptualmente tres operaciones:

- `scope-fs` establece explícitamente un filesystem scope.
- `in` navega desde la ubicación actual dentro del filesystem scope.
- `iter` enumera el contenido de la ubicación activa.

Ejemplo conceptual:

```text
scope-fs "/home/user/projects/evo-shell"
in src
in agents
iter
```

Después de:

```text
in src
```

la ubicación activa pasa conceptualmente a:

```text
/home/user/projects/evo-shell/src
```

Después de:

```text
in agents
```

la ubicación activa pasa conceptualmente a:

```text
/home/user/projects/evo-shell/src/agents
```

## Sintaxis

La sintaxis funcional del comando es:

```text
in <location>
```

Una ubicación simple puede escribirse sin comillas:

```text
in agents
```

También puede usarse una ubicación relativa compuesta:

```text
in src/agents
```

Las comillas pueden utilizarse cuando la ubicación contiene espacios:

```text
in "Mis Documentos"
```

El espacio separa la instrucción de su argumento.

Esta historia no define `in..` sin espacio.

## Navegación hacia arriba

`..` se interpreta funcionalmente como la ubicación padre de la ubicación activa.

Ejemplo:

Ubicación activa:

```text
/home/user/projects/evo-shell/src/agents
```

Entrada:

```text
in ..
```

Resultado conceptual:

```text
/home/user/projects/evo-shell/src
```

También se permite navegación relativa compuesta.

Entrada:

```text
in ../..
```

Desde:

```text
/home/user/projects/evo-shell/src/agents
```

Resultado conceptual:

```text
/home/user/projects/evo-shell
```

`in ..` no representa historial ni significa volver al lugar visitado anteriormente.

Su significado es navegación estructural hacia la ubicación padre.

No se introduce un comando `back` en esta historia.

## Comportamiento exitoso

Si la ubicación solicitada es válida y puede utilizarse como nueva ubicación activa:

- Evo Shell entra en ella.
- Esa ubicación pasa a ser la ubicación activa.
- Los comandos posteriores operan desde ella.

Ejemplo:

```text
scope-fs "/home/user/projects/evo-shell"
in src
iter
```

`iter` debe observar conceptualmente el contenido de:

```text
/home/user/projects/evo-shell/src
```

## Comportamiento no exitoso

Si el usuario solicita una ubicación que no puede utilizarse:

```text
in directory-that-does-not-exist
```

Evo Shell debe:

- informar que no pudo entrar en la ubicación solicitada.
- conservar la ubicación activa anterior.

Los mensajes mostrados son conceptuales.

Esta historia no fija todavía el formato definitivo de errores.

## Relación con scope-fs

`scope-fs "<path>"` establece explícitamente el ámbito filesystem.

`in <location>` navega desde la ubicación actual de ese ámbito.

`in` no es un alias de `scope-fs`.

`scope-fs` no se convierte en `cd`.

Ambos comandos expresan intenciones diferentes desde la perspectiva del usuario.

## Relación con iter

`in` entra o navega.

`iter` enumera.

`in` no debe:

- enumerar automáticamente;
- leer archivos;
- abrir archivos;
- ejecutar `iter` implícitamente.

Ejemplo:

```text
in agents
```

solo cambia la ubicación activa.

Después el usuario puede ejecutar:

```text
iter
```

para observar su contenido.

## Criterios de aceptación

1. El usuario puede introducir `in` seguido de una ubicación.
2. `in` requiere una ubicación.
3. Una ubicación simple puede escribirse sin comillas:

   ```text
   in agents
   ```

4. Una ubicación con espacios puede escribirse entre comillas:

   ```text
   in "Mis Documentos"
   ```

5. La ubicación se interpreta relativamente a la ubicación activa.
6. `in ..` navega a la ubicación padre.
7. `in ../..` permite navegar dos niveles hacia arriba.
8. Una navegación exitosa cambia la ubicación activa.
9. Los comandos posteriores operan desde la nueva ubicación.
10. Una navegación fallida conserva la ubicación anterior.
11. `in` no enumera contenido.
12. `in` no abre archivos.
13. `in` no representa historial de navegación.
14. Una instrucción que no cumple la sintaxis requerida no se ejecuta como navegación válida.

## Ejemplos

### A. Entrar en un directorio hijo

Ubicación activa:

```text
/home/user/projects/evo-shell/src
```

Entrada:

```text
in agents
```

Resultado conceptual:

```text
/home/user/projects/evo-shell/src/agents
```

### B. Entrar usando una ubicación relativa compuesta

Ubicación activa:

```text
/home/user/projects/evo-shell
```

Entrada:

```text
in src/agents
```

Resultado conceptual:

```text
/home/user/projects/evo-shell/src/agents
```

### C. Subir un nivel

Ubicación activa:

```text
/home/user/projects/evo-shell/src/agents
```

Entrada:

```text
in ..
```

Resultado conceptual:

```text
/home/user/projects/evo-shell/src
```

### D. Subir dos niveles

Ubicación activa:

```text
/home/user/projects/evo-shell/src/agents
```

Entrada:

```text
in ../..
```

Resultado conceptual:

```text
/home/user/projects/evo-shell
```

### E. Ubicación con espacios

Ubicación activa:

```text
/home/user
```

Entrada:

```text
in "Mis Documentos"
```

Resultado conceptual:

```text
/home/user/Mis Documentos
```

### F. Ubicación inexistente

Ubicación activa:

```text
/home/user/projects/evo-shell
```

Entrada:

```text
in directory-that-does-not-exist
```

Resultado conceptual:

```text
No se pudo entrar en la ubicación solicitada.
```

La ubicación activa permanece:

```text
/home/user/projects/evo-shell
```

### G. Falta de argumento

Entrada:

```text
in
```

Resultado conceptual:

```text
El comando requiere una ubicación.
```

### H. Argumentos adicionales inválidos

Entrada:

```text
in agents extra
```

Resultado conceptual:

```text
El comando requiere una única ubicación.
```

## Futuro

`in` está pensado conceptualmente como navegación dentro del scope activo.

Esta historia no define todavía qué significa `in` para otros tipos de scope.

Ese comportamiento deberá emerger de historias funcionales independientes.

## Fuera de alcance

Esta historia no define todavía:

- navegación en database scopes;
- navegación en URL scopes;
- navegación en Web API scopes;
- expansión de `~`;
- variables;
- wildcards/globbing;
- autocomplete;
- history;
- comando `back`;
- aliases;
- pipes;
- filtros;
- apertura de archivos;
- formato visual definitivo;
- detalles técnicos de interpretación;
- detalles de implementación.
