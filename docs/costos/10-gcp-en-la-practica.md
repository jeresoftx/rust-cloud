# Costos: GCP en la práctica

- **Curso:** rust-cloud
- **Capítulo:** 10. GCP en la práctica
- **Estado:** implemented
- **Issue:** #40

Este capítulo no usa precios de GCP. Los precios, cuotas y nombres exactos son
material vivo y deben revisarse con fecha cuando el curso los publique. Aquí el
costo se expresa como decisión educativa: proyecto, servicio, identidad, red,
límites, observabilidad, labels y ciclo de vida.

## Costos que sí se pueden comparar aquí

| Decisión | Costo visible | Riesgo si se ignora |
|----------|---------------|---------------------|
| Cloud Run | Contenedores administrados y escalado | Escalar sin límites o sin señales |
| Compute Engine | Capacidad estable y operación retenida | Pagar VMs sin carga suficiente |
| GKE | Orquestación y control | Complejidad operativa innecesaria |
| Cloud Functions | Invocaciones event-driven | Retries sin idempotencia |
| Cloud Storage | Objetos, operaciones y salida de datos | Retención infinita o acceso público accidental |
| Cloud SQL | Estado relacional administrado | Sobredimensionamiento y backups no probados |
| Pub/Sub | Mensajería y fan-out | Backlogs, duplicados y consumidores sin límites |
| Cloud Operations | Logs, métricas y trazas | Retención y cardinalidad sin control |
| Budgets | Señal temprana de gasto | Alertas ignoradas si no tienen dueño |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara costos GCP como consecuencias de una decisión
de plataforma: proyecto, servicio, identidad, red, límites, observabilidad,
labels y ciclo de vida.

**Problema:** medir `GcpWorkload::evaluate` solo mediría lógica local trivial.
Ese número no enseña el costo real de GCP: tráfico, regiones, capacidad,
peticiones, retención, cuotas, compromisos, descuentos o exportaciones de
facturación.

**Alternativas:** agregar `criterion`, simular facturas sintéticas o documentar
costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando exista una simulación fechada de tráfico,
almacenamiento, retención, eventos, billing export o forecast. Por ahora, la
evidencia correcta es declarar señales que vuelven gobernable la decisión.

## Checklist de costo antes de desplegar en GCP

- ¿Qué capacidad del curso representa el servicio?
- ¿Qué proyecto agrupa recursos, IAM y facturación?
- ¿Qué región o zona explica el despliegue?
- ¿Quién es dueño del gasto?
- ¿Qué propósito humano justifica el recurso?
- ¿Qué roles mínimos e identidad administrada protegen la operación?
- ¿Qué parte queda pública y qué parte queda interna?
- ¿Qué límite existe para capacidad, concurrencia, retención o ciclo de vida?
- ¿Qué señal operativa permite investigar incidentes?
- ¿Qué labels permiten atribuir costo?

## Lectura de tradeoffs

| Optimización | Puede ahorrar | Puede romper |
|--------------|---------------|--------------|
| Cambiar Compute Engine por Cloud Run | Operación de VMs | Control fino de infraestructura |
| Cambiar cómputo estable por Cloud Functions | Capacidad ociosa | Latencia, límites y acoplamiento a eventos |
| Bajar retención de logs | Logs acumulados | Diagnóstico histórico |
| Agregar lifecycle en Cloud Storage | Almacenamiento viejo | Recuperación de objetos antiguos |
| Separar proyectos por ambiente | Claridad de costo y permisos | Más gobierno y configuración |
| Consolidar recursos | Costos fijos | Aislamiento de riesgos |

GCP en la práctica no se mide por cuántos servicios se usan, sino por cuántas
decisiones quedan explicadas, observables y atribuibles.
