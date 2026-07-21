# Publicación candidata interna

- **Curso:** rust-cloud
- **Estado:** implemented
- **Issue:** #43
- **Fecha del corte:** 2026-07-21

Este documento registra el primer corte candidato interno del curso. No publica
el curso y no cambia ningún capítulo a `reviewed` ni `published`. Su función es
dejar evidencia para la revisión humana de Joel.

La bitácora de PRs fusionados en modo autónomo vive en
`docs/bitacora-revision-diferida.md`.

La guía para revisar el corte completo vive en `docs/guia-revision-corte.md`.

## Alcance del corte

El corte incluye:

- diez capítulos canónicos en `docs/`;
- ejercicios graduados en `docs/ejercicios/`;
- lectura de costos y decisiones de benchmark en `docs/costos/`;
- modelos Rust en `src/`;
- ejemplos ejecutables y soluciones en `examples/`;
- pruebas unitarias, integración y doctests;
- diagramas Mermaid en `diagrams/`;
- manifiesto estructurado para `academy-web`.

## Verificaciones requeridas

Antes de considerar este corte como listo para revisión humana, deben pasar:

- `node scripts/verify-course-links.mjs`;
- `cargo fmt --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --all-targets`;
- `cargo test --doc`;
- `cargo bench --all-targets`;
- `node scripts/verify-editorial-status.mjs`;
- `git diff --check`.

## Benchmarks aplicables

Se ejecuta `cargo bench --all-targets` en este corte y pasa correctamente. El
resultado no contiene benchmarks medidos porque el repositorio todavía no tiene
targets de benchmark ejecutables. `benches/README.md` registra por capítulo por
qué medir la lógica local del modelo Rust daría señales falsas sobre costos
reales de cloud.

Cuando exista una simulación educativa fechada de tráfico, almacenamiento,
retención, concurrencia o billing export, se deberá crear un issue específico,
un PR y una explicación de qué decisión enseña el benchmark.

## Criterios para revisión humana

Joel debe revisar:

- continuidad entre capítulos;
- exactitud conceptual y técnica;
- ortografía en español es-MX;
- claridad de diagramas;
- coherencia entre ejemplos, ejercicios y soluciones;
- separación entre fundamentos, AWS, GCP y DevOps;
- ausencia de credenciales reales, `unsafe` y dependencias externas no
  justificadas;
- límites de publicación para `academy-web`.

## Resultado esperado

Si la revisión humana aprueba un capítulo, el cambio de estado a `reviewed`
debe hacerse en un issue y PR separados. La automatización puede preparar la
evidencia, pero no toma la decisión editorial final.
