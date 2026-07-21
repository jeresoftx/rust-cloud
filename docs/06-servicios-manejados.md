# Servicios manejados

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 06. Servicios manejados
- **Issues:** #21, #22
- **Módulo Rust:** `src/managed_services.rs`

## Concepto

Un servicio manejado es una capacidad de plataforma donde el proveedor asume
parte importante de la operación diaria: aprovisionamiento, parches, respaldo,
replicación, salud, escalado o recuperación. El equipo consumidor sigue tomando
decisiones de arquitectura, datos, permisos, disponibilidad, costos y límites de
responsabilidad.

En este curso, los servicios manejados se estudian como una frontera de
delegación. La pregunta central no es "¿qué servicio ofrece el proveedor?",
sino "¿qué operación dejamos en manos de la plataforma y qué responsabilidad
seguimos conservando?".

## Problema

Cloud permite consumir bases de datos, colas, cachés, buscadores, secretos,
mensajería y observabilidad sin operar toda la infraestructura subyacente. Esa
abstracción acelera mucho, pero también puede ocultar decisiones críticas:
versiones, respaldos, ventanas de mantenimiento, límites de throughput,
aislamiento, región, costo por uso, dependencia de proveedor y salida de datos.

La enseñanza de servicios manejados suele caer en tres errores:

- presentar cada servicio como un producto aislado;
- asumir que "manejado" significa "sin operación";
- comparar servicios por lista de features, no por responsabilidades delegadas.

Sin una base clara, el estudiante puede creer que elegir una base de datos
manejada elimina la necesidad de diseñar esquema, índices, backups, permisos,
observabilidad o estrategia de recuperación.

## Alternativas consideradas

1. **Empezar por catálogos de AWS o GCP.** Es práctico para consola, pero ata
   el aprendizaje a nombres comerciales y cambia rápido.
2. **Empezar por bases de datos solamente.** Aterriza un caso común, pero deja
   fuera colas, cachés, secretos y otros servicios donde la delegación también
   importa.
3. **Empezar por capacidad, responsabilidad y contrato operativo.** Permite
   comparar servicios distintos desde un criterio común: qué capacidad entrega,
   qué opera el proveedor, qué conserva el equipo y qué riesgos aparecen.

## Justificación

Este capítulo adopta la tercera alternativa. Primero se modela un servicio
manejado por capacidad, responsabilidad delegada, responsabilidad retenida,
estado, criticidad, recuperación y costo operativo. Después se podrán mapear
estos conceptos a servicios concretos de proveedor.

La decisión conserva RFC-0001 §2: concepto antes de implementación, problema
antes de herramienta, alternativas antes de elección. También mantiene
RFC-0001 §10: proveedor después de fundamentos.

## Invariantes del capítulo

- "Manejado" no significa "sin responsabilidad".
- Todo servicio manejado tiene una frontera explícita entre proveedor y equipo.
- La capacidad técnica debe nombrarse antes que el producto comercial.
- Los datos, permisos, configuración y patrones de uso siguen siendo decisiones
  del equipo.
- La recuperación se diseña: backup, restore, réplica y prueba de restauración
  no son lo mismo.
- Los límites de cuota, throughput, latencia y tamaño forman parte del diseño.
- La dependencia de proveedor es una decisión de producto y operación, no solo
  una preferencia técnica.
- El costo combina consumo, almacenamiento, tráfico, operación, observabilidad y
  riesgo de cambio.
- Un servicio manejado debe tener propósito humano y dueño operativo.
- Los nombres, precios y límites de proveedor son material vivo y deben
  revisarse cuando se usen ejemplos fechados.

## Modelo Rust mínimo

El módulo Rust mínimo vive en `src/managed_services.rs` y modela, sin
dependencias externas:

- tipos de capacidad manejada: base de datos, cola, caché, secretos,
  observabilidad y búsqueda;
- responsabilidades delegadas al proveedor;
- responsabilidades retenidas por el equipo;
- requerimientos mínimos de datos, criticidad y recuperación;
- recomendación educativa de servicio manejado;
- hallazgos cuando falten propósito, dueño, respaldo o estrategia de recuperación;
- señales de riesgo cuando el servicio sea crítico sin restauración probada o
  use estado duradero sin backup.

El módulo no debe intentar reemplazar catálogos de proveedor. Su función es
pedagógica: hacer visible qué se delega, qué se conserva y qué costo operativo
queda.

## Decisiones registradas

- La capacidad manejada se modela como enum cerrado (`ManagedCapability`) para
  enseñar criterio antes de productos concretos.
- Las responsabilidades delegadas viven en `DelegatedResponsibility`; las
  responsabilidades retenidas viven en `RetainedResponsibility`.
- Los requerimientos mínimos se agrupan en `ManagedServiceRequirements`:
  capacidad, estado durable, criticidad, recuperación, dueño y propósito.
- Un `ManagedServiceProfile` produce `ManagedServiceEvaluation` con hallazgos
  educativos (`ManagedServiceFinding`).
- La criticidad no produce una fórmula automática: solo vuelve visible si falta
  restauración probada o dueño operativo.
- Los errores públicos viven en `ManagedServiceDecisionError`.

## Estado editorial

Este capítulo queda en `implemented`. No está marcado como `reviewed` ni
`published`.
