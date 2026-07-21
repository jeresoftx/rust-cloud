# Rust Cloud

Repositorio del camino troncal de Jeresoft Academy para estudiar Cloud en
Rust. Pertenece al Semestre 5 del plan de estudios junto con
`rust-software-architecture` (RFC-0001 §10).

El objetivo no es memorizar catálogos de AWS o GCP. El objetivo es aprender a
razonar sobre plataformas: modelos de servicio, cómputo, almacenamiento, red,
identidad, servicios manejados, serverless, costos y operación con decisiones
explícitas.

## Qué contiene

- Capítulos en Markdown compatibles con publicación posterior.
- Modelos Rust pequeños para representar decisiones de plataforma.
- Ejemplos progresivos: básico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integración y doctests.
- Benchmarks cuando una decisión tenga costo observable.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar en el camino

Este curso vive en el Semestre 5. Recibe fundamentos de
`rust-networking`, `rust-operating-systems`, `rust-database-internals`,
`rust-concurrency`, `rust-distributed-systems`, `rust-system-design` y
`rust-software-architecture`.

Alimenta `rust-ai-engineering`, `rust-travel`, dominios aplicados y cursos
complementarios como `rust-devops`, `rust-api-design` y
`software-engineering-handbook`.

## Estado editorial

El primer corte completo del curso está en estado `implemented`: los diez
capítulos tienen explicación, modelo Rust, pruebas, ejemplos, diagramas,
ejercicios y lectura de costos. Esto no significa que el curso esté publicado.
La revisión humana de Joel sigue siendo obligatoria antes de usar `reviewed` o
`published`.

## Capítulos del corte implementado

| # | Capítulo | Módulo sugerido | Estado |
|---|----------|-----------------|--------|
| 01 | Modelos de servicio | `src/service_models.rs` | implemented |
| 02 | Cómputo | `src/compute.rs` | implemented |
| 03 | Almacenamiento | `src/storage.rs` | implemented |
| 04 | Redes y VPC | `src/networking.rs` | implemented |
| 05 | Identidad y accesos | `src/iam.rs` | implemented |
| 06 | Servicios manejados | `src/managed_services.rs` | implemented |
| 07 | Serverless | `src/serverless.rs` | implemented |
| 08 | Costos y FinOps | `src/finops.rs` | implemented |
| 09 | AWS en la práctica | `src/aws_practice.rs` | implemented |
| 10 | GCP en la práctica | `src/gcp_practice.rs` | implemented |

Estados posibles: `planned`, `draft`, `implemented`, `tested`, `benchmarked`,
`reviewed`, `published`. En este repositorio, `implemented` significa listo
para revisión humana diferida; no equivale a aprobación editorial.

## Estructura

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
course.manifest.json
docs/
src/
examples/
tests/
benches/
diagrams/
assets/
```

## Cómo usarlo

Ejecutar pruebas:

```bash
cargo test
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --all-targets
node scripts/verify-course-links.mjs
node scripts/verify-manifest-consistency.mjs
node scripts/verify-editorial-status.mjs
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `course.manifest.json` expone el mapa estructurado del curso para
  `academy-web`.
- `docs/SUMMARY.md` contiene la navegación del curso.
- `docs/navegacion-y-enlaces.md` registra el cierre de navegación y rutas
  internas antes de revisión humana.
- `docs/publicacion-candidata.md` registra el corte candidato interno y sus
  verificaciones sin publicarlo automáticamente.
- `docs/00-introduccion.md` define la frontera conceptual de Cloud frente a
  DevOps y arquitectura.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha
  límite.
- El GitHub Project del curso vive en
  `https://github.com/users/jeresoftx/projects/10`.
- Ese Project debe tener su vista principal agrupada por `Milestone`; si no se
  ve agrupado así, la configuración de GitHub no está completa.
- Antes de tocar código de curso, el plan completo debe existir como milestones
  e issues de GitHub.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
