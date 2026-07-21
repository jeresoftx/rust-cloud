# Benchmarks

Los benchmarks se agregan solo cuando una decisión tenga costo observable. En
Cloud, esto puede aplicar a simulaciones educativas de costo, latencia,
capacidad o selección de plataforma.

## Decisiones actuales

- `01. Modelos de servicio`: no agrega benchmark. El capítulo razona costos
  operativos y de decisión; medir el tiempo de ejecución del modelo Rust daría
  una señal falsa.
- `02. Cómputo`: no agrega benchmark. El capítulo razona capacidad, límites,
  ciclo de vida y elasticidad; medir `recommend_compute` no representaría costo
  real de plataforma.
- `03. Almacenamiento`: no agrega benchmark. El capítulo razona retención,
  recuperación, consistencia y fuente de verdad; medir `recommend_storage` no
  representaría costo real de almacenamiento cloud.
- `04. Redes y VPC`: no agrega benchmark. El capítulo razona segmentación,
  rutas y exposición; medir validaciones locales no representa latencia,
  transferencia ni costo real de red.
- `05. Identidad y accesos`: no agrega benchmark. El capítulo razona límites de
  confianza, duración y auditoría; medir construcción de permisos no enseña
  costo operativo de IAM.
- `06. Servicios manejados`: no agrega benchmark. El capítulo razona estado,
  recuperación y responsabilidad delegada; medir el modelo Rust no representa
  costo real de disponibilidad ni backups.
- `07. Serverless`: no agrega benchmark. El capítulo razona evento,
  idempotencia, retries y límites; medir reglas locales no representa
  concurrencia ni facturación real.
- `08. Costos y FinOps`: no agrega benchmark. El capítulo enseña atribución,
  presupuesto y unidad económica; el número útil vendrá de una simulación
  fechada de consumo, no de tiempo de CPU local.
- `09. AWS en la práctica`: no agrega benchmark. El capítulo aterriza
  capacidades en AWS sin precios vivos; cualquier benchmark debe nacer de una
  simulación fechada de tráfico, retención, región y billing export.
- `10. GCP en la práctica`: no agrega benchmark. El capítulo aterriza
  capacidades en GCP sin precios vivos; cualquier benchmark debe nacer de una
  simulación fechada de proyectos, cuotas, tráfico, retención y billing export.

## Verificación de este corte

Para la publicación candidata interna, `cargo test --all-targets` cubre los
ejemplos registrados en `Cargo.toml`. También se ejecuta
`cargo bench --all-targets`: compila el perfil bench y pasa correctamente, pero
no reporta métricas porque aún no existen targets de benchmark ejecutables en
`benches/`.
