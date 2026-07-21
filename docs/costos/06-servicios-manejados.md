# Costos: servicios manejados

- **Curso:** rust-cloud
- **Capítulo:** 06. Servicios manejados
- **Estado:** implemented
- **Issue:** #24

Este capítulo no usa precios de proveedor. En servicios manejados, los precios
cambian por región, tamaño, almacenamiento, operaciones, retención, tráfico,
réplicas, backups y nivel de soporte. Aquí el costo se expresa como delegación,
operación retenida, recuperación, observabilidad, consumo y dependencia.

## Costos que sí se pueden comparar aquí

| Decisión | Costo visible | Riesgo si se ignora |
|----------|---------------|---------------------|
| Base de datos manejada | Menos operación base | Esquema, índices y restore siguen siendo del equipo |
| Cola manejada | Menos infraestructura propia | Reintentos, orden, DLQ y duplicados mal entendidos |
| Caché manejado | Menos latencia en lecturas | Invalidation, memoria y datos obsoletos |
| Secretos manejados | Rotación y almacenamiento más controlados | Permisos amplios o rotación no probada |
| Observabilidad manejada | Menos operación del stack | Eventos caros, ruido y alertas inútiles |
| Búsqueda manejada | Menos operación de índices | Costo por almacenamiento, consultas y reconstrucción |
| Réplicas y backups | Mejor recuperación potencial | Falsa confianza si nunca se prueba restore |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara costos de delegación: qué opera el proveedor,
qué conserva el equipo, qué recuperación se prueba y qué riesgo se introduce.

**Problema:** medir `ManagedServiceProfile::evaluate` solo mediría operaciones
locales triviales. Ese número no enseña el costo real de servicios manejados:
consumo, tráfico, almacenamiento, réplicas, backups, dependencia y operación
retenida.

**Alternativas:** agregar `criterion`, simular cientos de perfiles o documentar
costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando el curso tenga simulaciones de carga, throughput,
latencia, retención o FinOps. En este punto la evidencia correcta es declarar
responsabilidad, criticidad, recuperación, dueño y propósito.

## Checklist de costo antes de elegir un servicio manejado

- ¿Qué capacidad se necesita realmente?
- ¿El servicio guarda estado durable o procesa flujo temporal?
- ¿Qué responsabilidades quedan delegadas al proveedor?
- ¿Qué responsabilidades siguen siendo del equipo?
- ¿Qué pasa si el servicio falla una hora?
- ¿Existe backup, restore probado o failover probado?
- ¿Quién es dueño operativo de configuración, costo y revisión?
- ¿Qué límites de cuota, throughput, tamaño, latencia o región importan?
- ¿Qué tan difícil sería migrar datos o comportamiento a otro servicio?
