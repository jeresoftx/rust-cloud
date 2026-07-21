# Guía de revisión humana por corte

- **Curso:** rust-cloud
- **Estado:** implemented
- **Issue:** #91
- **Fecha del corte:** 2026-07-21

Esta guía ayuda a revisar el curso completo como una unidad. Complementa la
checklist por capítulo, pero no reemplaza el criterio de Joel ni convierte el
material en `reviewed` o `published`.

## 1. Continuidad conceptual

Revisar si el curso se lee como una progresión:

- modelos de servicio antes de proveedor;
- cómputo, almacenamiento, red e identidad antes de servicios manejados;
- serverless y FinOps antes de AWS y GCP;
- AWS y GCP como traducciones de fundamentos, no como catálogos de consola.

Preguntas guía:

- ¿Cada capítulo explica el problema antes de nombrar servicios?
- ¿Las decisiones de un capítulo preparan el siguiente?
- ¿Hay saltos donde el estudiante necesite una definición adicional?
- ¿El curso conserva frontera clara con DevOps, arquitectura y sistemas
  distribuidos?

## 2. Exactitud técnica

Revisar que los modelos Rust representen decisiones reales sin vender promesas
falsas:

- responsabilidad delegada frente a responsabilidad retenida;
- límites de capacidad y recuperación;
- identidad temporal frente a credenciales permanentes;
- red pública controlada frente a exposición accidental;
- costos visibles frente a optimización prematura.

Preguntas guía:

- ¿Algún ejemplo normaliza credenciales reales?
- ¿Alguna recomendación depende de precios o cuotas no fechadas?
- ¿Las abstracciones sobreviven a cambios de consola o marca?
- ¿AWS y GCP se comparan por capacidad, no por equivalencias superficiales?

## 3. Ortografía y tono

Revisar español es-MX con atención a:

- acentos;
- `ñ`;
- signos de apertura cuando correspondan;
- nombres propios;
- consistencia entre "cómputo", "costos", "serverless", "FinOps", "AWS" y
  "GCP";
- frases que suenen a marketing en lugar de libro de ingeniería.

## 4. Coherencia ejecutable

Para cada capítulo, revisar que lo visible coincida:

- documento principal;
- módulo Rust;
- tests;
- ejemplo principal;
- ejercicios;
- soluciones;
- costos;
- diagrama.

Preguntas guía:

- ¿El ejemplo ejecutable enseña la misma idea que el capítulo?
- ¿Los ejercicios aumentan dificultad de forma gradual?
- ¿Las soluciones de niveles 1 a 3 compilan y son didácticas?
- ¿Los diagramas aclaran relaciones sin sustituir explicación?

## 5. Costo y benchmarks

Revisar que los capítulos traten costo como decisión educativa:

- qué se mide;
- qué no se mide;
- qué señal sería falsa;
- cuándo haría falta una simulación fechada.

Preguntas guía:

- ¿Cada capítulo explica por qué no agrega benchmark ejecutable?
- ¿`cargo bench --all-targets` se interpreta correctamente como verificación,
  no como evidencia de rendimiento real?
- ¿La lectura de costos evita precios vivos sin fecha?

## 6. Decisión editorial

Al terminar la revisión, clasificar cada capítulo:

| Resultado | Acción |
|-----------|--------|
| Necesita cambios | Crear issue y PR de corrección. |
| Listo para revisión aprobada | Crear issue y PR para mover a `reviewed`. |
| Listo para publicación | Crear issue y PR separado para preparar `published`. |

No mezclar correcciones, aprobación editorial y publicación en un solo PR. La
trazabilidad importa más que cerrar rápido.

## 7. Evidencia mínima antes de aprobar

Antes de mover cualquier capítulo a `reviewed`, deben pasar:

- `node scripts/verify-course-links.mjs`;
- `node scripts/verify-editorial-status.mjs`;
- `cargo fmt --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --all-targets`;
- `cargo test --doc`;
- `cargo bench --all-targets`;
- `git diff --check`.

La aprobación humana debe quedar registrada en el PR que cambie el estado
editorial.
