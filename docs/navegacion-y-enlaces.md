# Navegación y enlaces

- **Curso:** rust-cloud
- **Estado:** implemented
- **Issue:** #41

Este documento registra el cierre de navegación del primer corte completo del
curso. No convierte ningún capítulo en `reviewed` ni `published`: solo deja
visible qué rutas deben existir antes de que Joel haga la revisión humana.

## Fuentes de navegación

| Fuente | Uso | Regla |
|--------|-----|-------|
| `docs/SUMMARY.md` | Lectura tipo mdBook | Debe listar introducción, manifiesto, mapa, verificación y capítulos. |
| `course.manifest.json` | Ingestión estructurada para `academy-web` | Debe declarar rutas internas por capítulo. |
| `docs/manifiesto-academy-web.md` | Contrato editorial | Debe separar contenido implementado de contenido publicable. |
| `docs/checklist-revision-capitulo.md` | Revisión humana | Debe impedir estados `reviewed` o `published` automáticos. |

## Rutas obligatorias por capítulo

Cada capítulo canónico debe tener estas rutas:

- documento principal en `docs/`;
- ejercicios en `docs/ejercicios/`;
- costos o decisión de benchmark en `docs/costos/`;
- módulo Rust en `src/`;
- ejemplo ejecutable en `examples/`;
- pruebas en `tests/`;
- diagrama Mermaid en `diagrams/`;
- entrada correspondiente en `course.manifest.json`.

## Revisión de enlaces internos

Para cada capítulo, la revisión humana debe confirmar:

- el título en `docs/SUMMARY.md` coincide con el título del documento;
- el documento principal declara su diagrama, ejercicios y costos;
- el manifiesto apunta a archivos existentes;
- los ejemplos de soluciones referenciados en ejercicios existen o se declaran
  como ejercicio de diseño sin archivo compilable;
- los diagramas Mermaid no sustituyen la explicación conceptual;
- ningún enlace promete publicación antes de revisión humana.

## Estado de este corte

El curso tiene diez capítulos en estado `implemented`. Todos tienen documento,
ejercicios, costos, módulo, ejemplo, tests y diagrama declarados. Este estado
significa que el material está listo para revisión humana diferida, no que ya
esté aprobado para publicación.

## Comando sugerido de verificación local

```bash
node scripts/verify-course-links.mjs
```

Si el repositorio aún no tiene ese script, la revisión equivalente es comprobar
que cada ruta declarada en `course.manifest.json` existe en el árbol de trabajo
y que `docs/SUMMARY.md` lista los documentos visibles del curso.
