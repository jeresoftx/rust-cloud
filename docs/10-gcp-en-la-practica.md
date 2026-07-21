# GCP en la práctica

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** draft
- **Milestone:** 10. GCP en la práctica
- **Issue:** #37
- **Módulo Rust:** `src/gcp_practice.rs`

## Concepto

GCP en la práctica es el aterrizaje de los fundamentos del curso en Google
Cloud. No se estudia como una lista de productos, sino como una forma concreta
de expresar decisiones sobre proyectos, regiones, cómputo, datos, red,
identidad, observabilidad y costo.

La pregunta central no es "¿qué servicio de GCP se parece a uno de AWS?", sino
"¿qué capacidad necesita el sistema y cómo la representa GCP con responsabilidad
clara, señales visibles y costo atribuible?".

## Imagen mental

Piensa en GCP como un campus técnico organizado por proyectos.

- **Proyecto** agrupa recursos, permisos, facturación y cuotas.
- **IAM** define quién puede hacer qué sobre recursos concretos.
- **Cloud Run, Compute Engine, GKE y Cloud Functions** expresan distintas
  formas de ejecutar trabajo.
- **Cloud Storage, Persistent Disk, Filestore, Cloud SQL, Spanner y BigQuery**
  expresan distintas formas de conservar o analizar datos.
- **VPC, subredes, firewall y balanceadores** delimitan exposición y tráfico.
- **Cloud Logging, Monitoring, Trace y Audit Logs** hacen investigable la
  operación.
- **Labels, budgets y billing export** ayudan a explicar costo y responsabilidad.

El campus no enseña por sí solo. El trabajo de ingeniería consiste en ubicar
cada capacidad en el proyecto correcto, con permisos, red, señales y límites
explícitos.

## Problema

GCP puede parecer más homogéneo que otros proveedores por su énfasis en
proyectos, servicios administrados y APIs consistentes. Esa sensación puede
ocultar decisiones importantes: proyectos mal separados, roles amplios,
service accounts permanentes, redes expuestas, datos sin ciclo de vida,
observabilidad sin retención, cuotas ignoradas y costos que no se pueden
atribuir.

La enseñanza de GCP suele caer en cuatro errores:

- comparar servicio por servicio contra AWS sin explicar el contrato;
- empezar por despliegues rápidos sin hablar de proyecto, IAM y facturación;
- tratar Cloud Run, Cloud Functions y GKE como opciones intercambiables;
- dejar labels, budgets, auditoría y límites para el final.

Sin una base clara, el estudiante puede creer que GCP se aprende memorizando
equivalencias o siguiendo comandos, no entendiendo qué responsabilidad conserva
el equipo y qué responsabilidad delega a la plataforma.

## Alternativas consideradas

1. **Curso guiado por equivalencias AWS/GCP.** Ayuda a orientarse, pero puede
   reducir la nube a traducción de nombres.
2. **Curso guiado por consola y CLI.** Produce resultados visibles rápido, pero
   envejece y puede esconder fundamentos.
3. **Curso guiado por capacidades transferibles.** Parte de cómputo, datos,
   red, IAM, observabilidad y costo; después aterriza servicios GCP como
   representaciones concretas.

## Justificación

Este capítulo adopta la tercera alternativa. GCP aparece como proveedor
concreto, pero el canon sigue siendo el del curso: concepto, problema,
alternativas, decisión, implementación y costos. La comparación con AWS sirve
solo cuando aclara responsabilidades; no se vuelve el eje del capítulo.

La decisión conserva RFC-0001 §2: concepto antes de implementación, problema
antes de herramienta, alternativas antes de elección. También mantiene
RFC-0001 §10: proveedor después de fundamentos.

## Invariantes del capítulo

- GCP se enseña como traducción de fundamentos, no como catálogo.
- Todo recurso debe declarar proyecto, región o zona, ambiente, dueño y
  propósito.
- IAM debe preferir roles mínimos, service accounts explícitas y credenciales
  no permanentes.
- La red debe distinguir entrada pública, tráfico interno y dependencias
  externas.
- La elección de cómputo debe explicar operación delegada, escalado, límites y
  portabilidad razonable.
- Los datos deben declarar durabilidad, recuperación, ciclo de vida, frontera
  de acceso y tipo de consulta.
- Los flujos event-driven deben declarar evento, retry, idempotencia y destino.
- Los servicios administrados no eliminan responsabilidad: cambian qué se
  delega y qué sigue siendo del equipo.
- Toda decisión GCP debe tener señal operativa: logs, métricas, auditoría o
  trazas según el caso.
- El costo debe ser atribuible mediante labels, presupuesto y unidad económica.
- Los nombres, precios, cuotas y límites de GCP son material vivo y deben
  revisarse cuando se usen ejemplos fechados.

## Comparación educativa inicial

| Necesidad | Pregunta de ingeniería | Ejemplo GCP posible | Riesgo común |
|-----------|------------------------|---------------------|--------------|
| Ejecutar API web | ¿Quién opera runtime, escalado y empaquetado? | Cloud Run, GKE o Compute Engine | Elegir por moda, no por contrato |
| Publicar assets | ¿Qué acceso y ciclo de vida necesitan los objetos? | Cloud Storage y Cloud CDN | Hacer público lo que debía ser privado |
| Separar recursos | ¿Qué frontera organiza permisos y facturación? | Proyectos, folders y labels | Mezclar ambientes sin responsabilidad |
| Acceso de workloads | ¿Qué identidad usa el servicio? | Service Account | Llaves permanentes descargadas |
| Procesar eventos | ¿Qué pasa si el evento se repite? | Pub/Sub, Cloud Functions, Eventarc | Retry sin idempotencia |
| Observar operación | ¿Qué señal permite investigar incidentes? | Cloud Logging, Monitoring, Trace | Logs sin retención ni correlación |
| Controlar costo | ¿Quién entiende el gasto y contra qué unidad? | Labels, Budgets, Billing Export | Factura por proyecto sin dueño real |

## Decisiones registradas

- El capítulo no empieza por comandos de `gcloud` ni por consola.
- GCP se modela como una traducción de fundamentos a servicios concretos.
- La comparación con AWS es secundaria y solo se usa para aclarar decisiones.
- El modelo Rust mínimo deberá representar una carga GCP educativa sin depender
  de SDKs.
- Las recomendaciones dependientes de precios, cuotas o límites concretos se
  consideran material vivo y no se publican sin fecha y revisión humana.

## Nota editorial

Este capítulo está en borrador. No está marcado como `reviewed` ni `published`.
