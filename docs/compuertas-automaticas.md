# Compuertas automáticas

Este documento registra las compuertas automáticas del corte candidato de
`rust-cloud`. Su objetivo es proteger consistencia, compilación y trazabilidad
antes de revisión humana diferida. No publica el curso, no aprueba capítulos y
no cambia estados editoriales a `reviewed` ni `published`.

Las compuertas son evidencia, no criterio final. La revisión humana de Joel
sigue decidiendo si el contenido representa bien el curso y si puede avanzar a
un estado editorial superior.

## Matriz de compuertas

| Compuerta | Qué valida | Señal de falla | Frontera humana |
| --- | --- | --- | --- |
| `node scripts/verify-course-links.mjs` | Que `docs/SUMMARY.md` apunte a rutas existentes y que los capítulos esperados estén enlazados. | Enlace roto, capítulo fuera de navegación o archivo esperado sin ruta. | Joel decide si la navegación cuenta bien la progresión pedagógica. |
| `node scripts/verify-manifest-consistency.mjs` | Que `course.manifest.json` mantenga identidad, orden, unicidad y rutas derivadas del número y slug de cada capítulo. | Número duplicado, slug inválido, documento mal nombrado o ruta inconsistente. | Joel decide si el manifiesto representa el contrato correcto para `academy-web`. |
| `node scripts/verify-cargo-examples.mjs` | Que todos los ejemplos `.rs` de `examples/` estén declarados explícitamente en `Cargo.toml` y que cada ruta declarada exista. | Ejemplo sin declarar, ruta inexistente, nombre duplicado o ruta duplicada. | Joel decide si el ejemplo enseña el concepto con claridad suficiente. |
| `node scripts/verify-chapter-anatomy.mjs` | Que cada capítulo conserve la anatomía editorial esperada antes de revisión humana. | Falta concepto, problema, alternativas, justificación, invariantes, modelo Rust, ejemplo, práctica o cierre editorial. | Joel decide si la explicación es correcta, suficiente y aprobable. |
| `node scripts/verify-review-packet-coverage.mjs` | Que el paquete de revisión humana cubra todos los capítulos y sus artefactos principales. | Falta un capítulo, ruta, comando de compuerta o frontera editorial. | Joel decide el resultado de la revisión; la compuerta solo evita omisiones. |
| `node scripts/verify-editorial-status.mjs` | Que ningún capítulo se marque como `reviewed` ni `published` por automatización. | Estado editorial elevado sin revisión humana explícita. | Solo Joel puede aprobar revisión o publicación de capítulos. |
| `node scripts/verify-gate-sync.mjs` | Que README, publicación candidata, guía de revisión, checklist por capítulo, paquete humano, bitácora diferida y esta matriz mencionen las mismas compuertas obligatorias. | Documento operativo con una lista incompleta o desactualizada. | Joel decide si una nueva compuerta pertenece al flujo; el script solo evita deriva documental. |
| `node scripts/verify-deferred-review-log.mjs` | Que la bitácora de revisión diferida conserve secciones, PRs recientes, compuertas clave y frontera editorial. | Bitácora incompleta, PR reciente omitido o pérdida de la frontera `reviewed`/`published`. | Joel decide la revisión; el script solo protege evidencia histórica mínima. |
| `cargo fmt --check` | Que el código Rust conserve formato estándar. | Archivo Rust fuera de formato. | No sustituye criterio sobre legibilidad pedagógica. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Que biblioteca, pruebas y ejemplos no acumulen advertencias relevantes. | Warning de Clippy o patrón de código riesgoso. | Joel decide si una excepción futura requiere explicación curricular. |
| `cargo test --all-targets` | Que modelos, ejemplos compilables y pruebas de integración mantengan invariantes. | Prueba fallida, ejemplo que no compila o contrato roto. | No prueba exactitud conceptual completa del capítulo. |
| `cargo test --doc` | Que los ejemplos de documentación pública compilen como parte del contrato educativo. | Doctest roto o ejemplo documental desactualizado. | Joel decide si el ejemplo comunica bien el porqué, no solo el cómo. |
| `cargo bench --all-targets` | Que el corte candidato pueda ejecutar targets de benchmark presentes sin romper la suite. | Benchmark que no compila o falla en ejecución. | Los benchmarks de cloud requieren escenarios educativos fechados; no se inventan métricas locales como verdad de costos reales. |
| `git diff --check` | Que el cambio no introduzca espacios finales ni errores básicos de parche. | Espacios finales o líneas problemáticas para aplicar. | No revisa ortografía, tono ni alineación con RFC-0001. |
| Check remoto `rust` en GitHub Actions | Que la suite anterior también pase fuera de la máquina local del agente. | Job fallido, pendiente indefinido o diferencia de entorno. | Joel decide si una falla externa amerita issue de infraestructura o corrección del curso. |

## Uso en PRs autónomos

En modo de avance autónomo con revisión diferida, cada PR debe declarar qué
compuertas pasaron. Si una compuerta falla, el PR no se fusiona hasta corregir
la causa o hasta crear un issue explícito si la falla revela una decisión de
arquitectura, currículo o gobernanza.

La higiene de ramas vive en `docs/higiene-ramas-prs.md`. Antes de abrir un PR,
el agente debe confirmar que `git rev-list --count origin/main..HEAD` devuelve
`1`. Antes de fusionarlo, debe verificar que GitHub también muestra un solo
commit en el PR. Si una rama arrastra commits ajenos, el PR se corrige antes de
fusionarse.

El texto del PR debe usar `Closes #N` para cerrar el issue correspondiente en
GitHub. Puede incluir también la explicación en español, pero el cierre
automático depende de la palabra clave en inglés.

## Uso en revisión humana

Durante la revisión semanal, Joel puede leer esta matriz como inventario de
evidencia. Una compuerta verde significa que el repo conserva consistencia
mecánica; no significa que el capítulo esté aprobado editorialmente.

Si durante la revisión humana aparece una corrección, se crea un nuevo issue con
milestone, labels y asignación a `jeresoftx`, y se resuelve con un PR separado.
