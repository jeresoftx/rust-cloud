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

## Evidencia técnica

Las validaciones usadas durante el corte fueron:

- `node scripts/verify-course-links.mjs`;
- `node scripts/verify-editorial-status.mjs`;
- `cargo fmt --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --all-targets`;
- `cargo test --doc`;
- `cargo bench --all-targets`;
- `git diff --check`;
- check remoto `rust` en GitHub Actions.

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
