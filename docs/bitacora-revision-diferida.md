# Bitácora de revisión diferida

- **Curso:** rust-cloud
- **Estado:** implemented
- **Issue:** #89
- **Fecha del corte:** 2026-07-21

Esta bitácora registra evidencia del trabajo fusionado en modo autónomo. No es
aprobación editorial, no publica el curso y no cambia capítulos a `reviewed` ni
`published`.

## PRs del corte candidato

| PR | Issue | Resultado |
|----|-------|-----------|
| #80 | #37 | Fundamentos de GCP en la práctica. |
| #81 | #38 | Modelo Rust de GCP en la práctica. |
| #82 | #39 | Capítulo, diagrama y ejemplo de GCP. |
| #83 | #40 | Ejercicios, soluciones y costos de GCP. |
| #84 | #41 | Navegación, manifiesto ampliado y verificador de enlaces. |
| #85 | #42 | Alineación de README, ROADMAP y estados visibles. |
| #86 | #43 | Publicación candidata interna. |
| #94 | #88 | Verificador de enlaces integrado al CI. |
| #95 | #90 | Verificador de estados editoriales integrado al CI. |
| #103 | #96 | Verificador de consistencia del manifiesto integrado al CI. |
| #104 | #96 | PR duplicado sin cambios efectivos tras corrección de rama; registrado como higiene de proceso. |
| #105 | #97 | Verificador de ejemplos Cargo integrado al CI. |
| #106 | #98 | Matriz de compuertas automáticas del corte candidato. |
| #107 | #99 | Guía de revisión por corte para Joel. |
| #108 | #99 | PR duplicado sin cambios efectivos tras corrección de rama; registrado como higiene de proceso. |
| #109 | #100 | Checklist de publicación candidata ampliado con evidencia de compuertas. |
| #115 | #114 | Verificador de anatomía editorial de capítulos. |
| #116 | #110 | Paquete de revisión humana por capítulo. |
| #117 | #113 | Verificador de cobertura del paquete de revisión humana. |
| #118 | #112 | Higiene obligatoria para ramas, commits únicos y PRs autónomos. |
| #121 | #120 | Guía de revisión alineada con las compuertas actuales. |
| #122 | #119 | Verificador de sincronía entre listas de compuertas documentadas. |
| #125 | #123 | Checklist por capítulo alineada con la evidencia mínima del corte. |
| #126 | #124 | Checklist por capítulo protegida por la sincronía de compuertas. |

## Evidencia técnica

Las validaciones usadas durante el corte fueron:

- `node scripts/verify-course-links.mjs`;
- `node scripts/verify-manifest-consistency.mjs`;
- `node scripts/verify-cargo-examples.mjs`;
- `node scripts/verify-chapter-anatomy.mjs`;
- `node scripts/verify-review-packet-coverage.mjs`;
- `node scripts/verify-editorial-status.mjs`;
- `node scripts/verify-gate-sync.mjs`;
- `cargo fmt --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --all-targets`;
- `cargo test --doc`;
- `cargo bench --all-targets`;
- `git diff --check`;
- check remoto `rust` en GitHub Actions.

## Incidentes de higiene documentados

Durante el cierre del corte aparecieron PRs duplicados por correcciones de
rama. Los PRs #104 y #108 no representan avance de contenido; se registran para
mantener trazabilidad del proceso. La regla vigente queda documentada en
`docs/higiene-ramas-prs.md`: antes de abrir un PR autónomo, la rama debe tener
exactamente un commit por encima de `origin/main`, y antes de fusionarlo GitHub
debe mostrar exactamente un commit en el PR.

Estos incidentes no cambian el contenido del curso, no publican capítulos y no
marcan nada como `reviewed` ni `published`.

## Sincronía documental reciente

Los PRs #121, #122, #125 y #126 dejaron alineadas las listas de compuertas en
README, publicación candidata, guía de revisión, checklist por capítulo,
paquete humano y matriz de compuertas. La compuerta
`node scripts/verify-gate-sync.mjs` debe pasar antes de usar esta bitácora como
evidencia del corte.

Esta sincronía solo protege que la evidencia esté completa y consistente. No
aprueba contenido, no publica el curso y no reemplaza la lectura humana de
Joel.

## Decisiones que no tomó la IA

- No se marcó ningún capítulo como `reviewed`.
- No se marcó ningún capítulo como `published`.
- No se cambió el currículum de RFC-0001.
- No se agregaron dependencias externas.
- No se usó `unsafe`.
- No se decidió publicación pública en `academy-web`.

## Pendientes para revisión humana

Joel debe revisar:

- continuidad entre capítulos;
- precisión técnica de AWS y GCP;
- ortografía en español es-MX;
- coherencia entre capítulo, ejemplo, ejercicios, soluciones y costos;
- si algún capítulo puede avanzar de `implemented` a `reviewed`;
- si conviene crear nuevos issues de corrección editorial antes de publicar.

Si Joel encuentra correcciones, cada corrección debe convertirse en issue y PR
separado. La revisión diferida no elimina la revisión humana: solo mueve su
momento al cierre del bloque.
