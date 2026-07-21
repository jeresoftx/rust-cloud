# Checklist de revisión humana por capítulo

Esta checklist se usa antes de marcar un capítulo como `reviewed` o
`published`. La IA puede acelerar borradores, diagramas, pruebas y andamiaje,
pero el criterio humano decide qué queda aprobado (RFC-0001 §20).

Para revisar el curso completo como corte editorial, usa también
`docs/guia-revision-corte.md`.

## Identificación

- Capítulo:
- Issue:
- PR:
- Milestone:
- Revisor humano:
- Fecha de revisión:

## Claridad técnica

- [ ] El capítulo explica el problema antes de presentar servicios.
- [ ] Los conceptos sobreviven a cambios de consola o marca.
- [ ] Las responsabilidades del proveedor y del equipo quedan separadas.
- [ ] Los tradeoffs de costo, disponibilidad, latencia y operación son claros.
- [ ] Las referencias a AWS o GCP están fechadas cuando pueden cambiar.

## Código Rust

- [ ] El módulo Rust es pequeño, legible y coherente con el capítulo.
- [ ] El código declara invariantes, límites y errores relevantes.
- [ ] No usa `unsafe`.
- [ ] No agrega dependencias externas sin justificación escrita.
- [ ] `cargo fmt --check` pasa.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.

## Pruebas y benchmarks

- [ ] Hay pruebas para reglas y casos límite.
- [ ] `cargo test --all-targets` pasa.
- [ ] `cargo test --doc` pasa.
- [ ] El capítulo declara si benchmark aplica.
- [ ] Los resultados no prometen costos ni rendimiento de producción.

## Publicación

- [ ] El capítulo no está marcado como `reviewed` sin revisión humana.
- [ ] El capítulo no está marcado como `published` sin aprobación explícita.
- [ ] `academy-web` solo consume el capítulo si su estado editorial lo permite.
