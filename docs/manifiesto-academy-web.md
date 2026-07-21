# Manifiesto para academy-web

Este documento describe el contrato inicial para que `academy-web` pueda
descubrir el curso sin asumir que los capítulos ya son publicables.

## Principio

El sitio puede leer metadatos de capítulos en estado `planned`, `draft`,
`implemented`, `tested` o `benchmarked`, pero solo debe presentar como contenido
publicado lo que tenga revisión humana y estado editorial compatible.

## Fuente estructurada

El archivo `course.manifest.json` contiene:

- identidad del curso;
- semestre;
- repositorio;
- capítulos;
- estado editorial;
- rutas esperadas de documento, ejercicios, costos, módulo, ejemplo, tests,
  benchmark y diagrama.

La navegación humana vive en `docs/SUMMARY.md`. La verificación editorial de
rutas vive en `docs/navegacion-y-enlaces.md`. `academy-web` debe tratar esas
rutas como referencias internas del curso, no como garantía de publicación.

## Regla de publicación

Ningún capítulo pasa a `reviewed` o `published` por automatización. La revisión
humana de Joel es obligatoria según RFC-0001 §20.
