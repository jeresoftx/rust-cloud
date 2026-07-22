# Paquete de revisión humana

- **Curso:** rust-cloud
- **Estado:** implemented
- **Milestone:** 14. Paquete de revisión humana
- **Propósito:** reunir evidencia para revisión humana diferida sin aprobar ni
  publicar capítulos.

Este paquete ayuda a Joel a revisar el corte candidato del curso capítulo por
capítulo. No cambia ningún estado a `reviewed` ni `published`; solo reúne rutas,
preguntas y compuertas para que la revisión humana pueda concentrarse en
criterio técnico, claridad y continuidad.

## Frontera de decisión

La IA puede preparar evidencia, detectar inconsistencias y mantener la suite en
verde. Joel decide:

- si el capítulo explica bien el concepto antes de aterrizarlo en servicios;
- si el problema, alternativas y justificación son técnicamente correctos;
- si los ejercicios realmente prueban comprensión y no memorización;
- si el material puede pasar de `implemented` a `reviewed`;
- si más adelante puede prepararse para `published`.

## Compuertas antes de revisar

Antes de abrir revisión humana, ejecutar:

```bash
node scripts/verify-course-links.mjs
node scripts/verify-manifest-consistency.mjs
node scripts/verify-cargo-examples.mjs
node scripts/verify-chapter-anatomy.mjs
node scripts/verify-review-packet-coverage.mjs
node scripts/verify-deferred-review-log.mjs
node scripts/verify-editorial-status.mjs
node scripts/verify-gate-sync.mjs
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
cargo bench --all-targets
git diff --check
```

El check remoto `rust` en GitHub Actions también debe pasar antes de cerrar la
revisión del corte.

## Preguntas por capítulo

Para cada capítulo, revisar:

- ¿El concepto se entiende sin depender de una marca o consola?
- ¿El problema justifica por qué existe esta estructura de cloud?
- ¿Las alternativas muestran trade-offs reales?
- ¿El modelo Rust representa invariantes, no detalles accidentales?
- ¿El ejemplo ejecutable aterriza el concepto sin ocultar riesgos?
- ¿Los ejercicios suben de dificultad de forma razonable?
- ¿La lectura de costos evita números actuales sin fecha o revisión humana?
- ¿El diagrama ayuda a razonar o solo adorna?
- ¿El cierre editorial conserva `implemented` y evita `reviewed`/`published`?

## Capítulos

### 01. Modelos de servicio

- Documento: `docs/01-modelos-de-servicio.md`
- Ejercicios: `docs/ejercicios/01-modelos-de-servicio.md`
- Costos: `docs/costos/01-modelos-de-servicio.md`
- Módulo Rust: `src/service_models.rs`
- Ejemplo: `examples/service_models.rs`
- Tests: `tests/service_models.rs`
- Diagrama: `diagrams/01-modelos-de-servicio.mmd`
- Revisión humana: confirmar que la frontera IaaS, PaaS, SaaS y serverless se
  explica como responsabilidad operativa, no como catálogo.

### 02. Cómputo

- Documento: `docs/02-computo.md`
- Ejercicios: `docs/ejercicios/02-computo.md`
- Costos: `docs/costos/02-computo.md`
- Módulo Rust: `src/compute.rs`
- Ejemplo: `examples/compute.rs`
- Tests: `tests/compute.rs`
- Diagrama: `diagrams/02-computo.mmd`
- Revisión humana: confirmar que máquina virtual, contenedor, función y batch
  se comparan por control, operación, límites y elasticidad.

### 03. Almacenamiento

- Documento: `docs/03-almacenamiento.md`
- Ejercicios: `docs/ejercicios/03-almacenamiento.md`
- Costos: `docs/costos/03-almacenamiento.md`
- Módulo Rust: `src/storage.rs`
- Ejemplo: `examples/storage.rs`
- Tests: `tests/storage.rs`
- Diagrama: `diagrams/03-almacenamiento.mmd`
- Revisión humana: confirmar que objeto, bloque, archivo, efímero y archivo
  frío se entienden por acceso, durabilidad y recuperación.

### 04. Redes y VPC

- Documento: `docs/04-redes-y-vpc.md`
- Ejercicios: `docs/ejercicios/04-redes-y-vpc.md`
- Costos: `docs/costos/04-redes-y-vpc.md`
- Módulo Rust: `src/networking.rs`
- Ejemplo: `examples/networking.rs`
- Tests: `tests/networking.rs`
- Diagrama: `diagrams/04-redes-y-vpc.mmd`
- Revisión humana: confirmar que VPC, subredes, rutas y reglas se explican
  como límites de comunicación y exposición.

### 05. Identidad y accesos

- Documento: `docs/05-identidad-y-accesos.md`
- Ejercicios: `docs/ejercicios/05-identidad-y-accesos.md`
- Costos: `docs/costos/05-identidad-y-accesos.md`
- Módulo Rust: `src/iam.rs`
- Ejemplo: `examples/iam.rs`
- Tests: `tests/iam.rs`
- Diagrama: `diagrams/05-identidad-y-accesos.mmd`
- Revisión humana: confirmar que identidad, rol, permiso, condición y auditoría
  se entienden como control de riesgo, no solo como credenciales.

### 06. Servicios manejados

- Documento: `docs/06-servicios-manejados.md`
- Ejercicios: `docs/ejercicios/06-servicios-manejados.md`
- Costos: `docs/costos/06-servicios-manejados.md`
- Módulo Rust: `src/managed_services.rs`
- Ejemplo: `examples/managed_services.rs`
- Tests: `tests/managed_services.rs`
- Diagrama: `diagrams/06-servicios-manejados.mmd`
- Revisión humana: confirmar que delegar operación no borra responsabilidad
  sobre configuración, restauración, costo y evidencia.

### 07. Serverless

- Documento: `docs/07-serverless.md`
- Ejercicios: `docs/ejercicios/07-serverless.md`
- Costos: `docs/costos/07-serverless.md`
- Módulo Rust: `src/serverless.rs`
- Ejemplo: `examples/serverless.rs`
- Tests: `tests/serverless.rs`
- Diagrama: `diagrams/07-serverless.mmd`
- Revisión humana: confirmar que eventos, tiempo de ejecución, reintentos e
  idempotencia aparecen antes de hablar de productos.

### 08. Costos y FinOps

- Documento: `docs/08-costos-y-finops.md`
- Ejercicios: `docs/ejercicios/08-costos-y-finops.md`
- Costos: `docs/costos/08-costos-y-finops.md`
- Módulo Rust: `src/finops.rs`
- Ejemplo: `examples/finops.rs`
- Tests: `tests/finops.rs`
- Diagrama: `diagrams/08-costos-y-finops.mmd`
- Revisión humana: confirmar que costo se enseña como diseño, atribución,
  elasticidad y control, no como tabla de precios.

### 09. AWS en la práctica

- Documento: `docs/09-aws-en-la-practica.md`
- Ejercicios: `docs/ejercicios/09-aws-en-la-practica.md`
- Costos: `docs/costos/09-aws-en-la-practica.md`
- Módulo Rust: `src/aws_practice.rs`
- Ejemplo: `examples/aws_practice.rs`
- Tests: `tests/aws_practice.rs`
- Diagrama: `diagrams/09-aws-en-la-practica.mmd`
- Revisión humana: confirmar que AWS aterriza los conceptos sin convertir el
  capítulo en tutorial de consola ni recomendación comercial.

### 10. GCP en la práctica

- Documento: `docs/10-gcp-en-la-practica.md`
- Ejercicios: `docs/ejercicios/10-gcp-en-la-practica.md`
- Costos: `docs/costos/10-gcp-en-la-practica.md`
- Módulo Rust: `src/gcp_practice.rs`
- Ejemplo: `examples/gcp_practice.rs`
- Tests: `tests/gcp_practice.rs`
- Diagrama: `diagrams/10-gcp-en-la-practica.mmd`
- Revisión humana: confirmar que GCP aterriza capacidades comparables con AWS
  sin borrar diferencias de identidad, red, operación y costos.

## Resultado de la revisión

Si un capítulo queda aprobado, crear un issue y PR separado para moverlo a
`reviewed`. Si requiere correcciones, crear issues puntuales. Si más adelante
se prepara publicación, usar otro PR separado para no mezclar revisión editorial
con preparación de distribución.
