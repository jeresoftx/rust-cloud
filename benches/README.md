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
