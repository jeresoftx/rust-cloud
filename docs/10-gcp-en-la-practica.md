# GCP en la práctica

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 10. GCP en la práctica
- **Issues:** #37, #38, #39
- **Módulo Rust:** `src/gcp_practice.rs`
- **Diagrama:** `diagrams/10-gcp-en-la-practica.mmd`
- **Ejemplo:** `examples/gcp_practice.rs`

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
- El modelo Rust mínimo representa una carga GCP educativa sin depender de SDKs.
- Las recomendaciones dependientes de precios, cuotas o límites concretos se
  consideran material vivo y no se publican sin fecha y revisión humana.

## Modelo Rust mínimo

El módulo Rust mínimo vive en `src/gcp_practice.rs` y modela, sin dependencias
externas:

- capacidades cloud del curso que se quieren aterrizar;
- servicios GCP candidatos para cada capacidad;
- proyecto, región, ambiente, dueño y propósito;
- permisos mínimos, identidad administrada y exposición de red;
- límites, observabilidad, labels de costo y ciclo de vida;
- hallazgos cuando un servicio no corresponde a la capacidad, faltan señales o
  el ejemplo intenta usar credenciales reales.

El módulo no usa SDKs, `gcloud` ni contacta GCP. Su función es pedagógica:
practicar la traducción de capacidad a servicio antes de tocar una cuenta real.

## Lectura del modelo

`GcpWorkload` junta cinco preguntas que deben quedar claras antes de abrir una
consola o ejecutar `gcloud`:

1. **Qué capacidad existe:** cómputo, almacenamiento, red, identidad, managed
   service, serverless, observabilidad o FinOps.
2. **Qué servicio candidato la representa:** Cloud Run, Compute Engine, GKE,
   Cloud Functions, Cloud Storage, VPC, Service Account, Cloud SQL, Pub/Sub,
   Cloud Operations, Budgets o BigQuery.
3. **Qué frontera organiza la responsabilidad:** proyecto, región, ambiente,
   dueño y propósito.
4. **Qué conserva el equipo:** roles mínimos, identidad administrada, red,
   límites, observabilidad, labels y ciclo de vida.
5. **Qué no debe normalizar el ejemplo:** llaves exportadas, credenciales
   reales, red pública sin frontera o costos sin labels.

El modelo no decide si GCP es mejor que AWS. Decide si una capacidad del curso
fue traducida a un servicio GCP con suficiente claridad para enseñarse.

## Cómo leer el módulo Rust

Un mapeo gobernable declara capacidad, servicio, proyecto, región, dueño,
identidad administrada, límites y señales:

```rust
use rust_cloud::gcp_practice::{
    CloudCapability, GcpEnvironment, GcpNetworkExposure, GcpPracticeRequirements,
    GcpService, GcpWorkload,
};

let workload = GcpWorkload::new(
    "academy-api",
    GcpPracticeRequirements {
        capability: CloudCapability::Compute,
        service: GcpService::CloudRun,
        project: "academy-prod",
        region: "us-central1",
        environment: GcpEnvironment::Production,
        owner: "equipo academy",
        purpose: "servir rutas de aprendizaje a estudiantes",
        least_privilege: true,
        managed_identity: true,
        network_exposure: GcpNetworkExposure::PublicEntrypoint,
        has_limit: true,
        observability: true,
        cost_labels: true,
        lifecycle_policy: true,
        uses_real_credentials: false,
    },
)
.unwrap();

assert!(workload.evaluate().is_low_risk());
```

Una decisión riesgosa mezcla capacidad, servicio e identidad sin límites:

```rust
use rust_cloud::gcp_practice::{
    CloudCapability, GcpEnvironment, GcpFinding, GcpNetworkExposure,
    GcpPracticeRequirements, GcpService, GcpWorkload,
};

let workload = GcpWorkload::new(
    "preview-runner",
    GcpPracticeRequirements {
        capability: CloudCapability::Serverless,
        service: GcpService::ComputeEngine,
        project: "academy-dev",
        region: "us-central1",
        environment: GcpEnvironment::Development,
        owner: "",
        purpose: "ejecutar previews",
        least_privilege: false,
        managed_identity: false,
        network_exposure: GcpNetworkExposure::PublicUnbounded,
        has_limit: false,
        observability: false,
        cost_labels: false,
        lifecycle_policy: false,
        uses_real_credentials: true,
    },
)
.unwrap();

assert!(workload.evaluate().findings().contains(
    &GcpFinding::ServiceDoesNotMatchCapability("preview-runner"),
));
```

## Diagrama

El diagrama del capítulo vive en `diagrams/10-gcp-en-la-practica.mmd`. Resume
la lectura principal:

```text
necesidad -> capacidad -> servicio GCP -> proyecto -> identidad/red -> señales -> decisión practicable
```

La ruta sana obliga a justificar el servicio con una capacidad del curso y un
proyecto responsable. La ruta riesgosa aparece cuando el servicio se elige por
equivalencia superficial, cuando el ejemplo usa credenciales reales o cuando
faltan límites, observabilidad y costo atribuible.

## Ejemplo ejecutable

El ejemplo `examples/gcp_practice.rs` compara una API en Cloud Run con labels,
service account y límites contra un runner de previews que mezcla serverless
con Compute Engine, identidad no administrada, red pública sin frontera y
señales incompletas.

```bash
cargo run --example gcp_practice
```

Salida esperada:

```text
academy-api: mapeo GCP gobernable
preview-runner: 10 hallazgos educativos
```

El ejemplo no usa `gcloud`, SDKs ni credenciales reales. Sirve para practicar
qué preguntas deben quedar respondidas antes de tocar infraestructura real.

## Ejemplos progresivos

| Nivel | Escenario | Señal principal | Aprendizaje |
|-------|-----------|-----------------|-------------|
| Básico | API en Cloud Run | Mapeo capacidad-servicio | Compute no es solo una VM; puede delegar runtime y escalado |
| Intermedio | Runner de previews mal modelado | Identidad y red inseguras | La práctica en GCP no debe normalizar llaves exportadas |
| Avanzado | Mapa por proyecto y capacidad | Responsabilidad retenida | Cada servicio debe declarar proyecto, labels y señales |

## Nota editorial

Este capítulo queda en `implemented`. No está marcado como `reviewed` ni
`published`.
