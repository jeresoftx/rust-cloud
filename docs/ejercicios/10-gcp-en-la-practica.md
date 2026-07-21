# Ejercicios: GCP en la práctica

- **Curso:** rust-cloud
- **Capítulo:** 10. GCP en la práctica
- **Estado:** implemented
- **Issue:** #40

Estos ejercicios practican GCP como traducción de fundamentos. La meta no es
crear recursos reales, sino explicar qué capacidad representa cada servicio,
qué proyecto lo gobierna y qué señales permiten operarlo sin llaves ni
credenciales reales.

## Ejercicio 1: Cloud Run gobernable `[Nivel 1]`

Construye un `GcpWorkload` para una API web:

- capacidad `CloudCapability::Compute`;
- servicio `GcpService::CloudRun`;
- proyecto de producción;
- región declarada;
- dueño y propósito explícitos;
- roles mínimos;
- identidad administrada;
- entrada pública controlada;
- límites, observabilidad, labels y ciclo de vida;
- `uses_real_credentials: false`.

**Entrada/Salida esperada:** la evaluación debe ser de bajo riesgo educativo.

<details>
<summary>Pista</summary>
Cloud Run no es solo "subir un contenedor". En el curso representa cómputo
administrado con identidad, límites, señales y costo atribuible.
</details>

Solución compilable: `examples/soluciones/gcp_practice_nivel_1.rs`.

## Ejercicio 2: runner de previews riesgoso `[Nivel 2]`

Modela un runner de previews que:

- declara capacidad serverless;
- usa `GcpService::ComputeEngine`;
- vive en desarrollo;
- no tiene dueño;
- usa permisos amplios;
- no usa identidad administrada;
- queda expuesto públicamente sin frontera;
- no declara límites, observabilidad ni labels;
- no tiene ciclo de vida;
- intenta usar credenciales reales.

**Entrada/Salida esperada:** la evaluación debe reportar servicio que no
corresponde a la capacidad, falta de dueño, permisos amplios, credenciales
permanentes o exportadas, ejemplo inseguro, red pública sin frontera, falta de
límites, observabilidad ausente, falta de labels y falta de ciclo de vida.

<details>
<summary>Pista</summary>
El problema no es Compute Engine. El problema es declararlo como serverless y
además normalizar llaves o credenciales reales en un ejemplo educativo.
</details>

Solución compilable: `examples/soluciones/gcp_practice_nivel_2.rs`.

## Ejercicio 3: matriz por proyecto y capacidad `[Nivel 3]`

Construye tres workloads:

- API web con Cloud Run;
- assets publicados con Cloud Storage;
- presupuesto de staging con Budgets.

Compara sus evaluaciones.

**Entrada/Salida esperada:** los tres deben quedar sin hallazgos si cada
servicio representa su capacidad y declara proyecto, región, dueño, identidad,
límites, observabilidad, labels y ciclo de vida.

<details>
<summary>Pista</summary>
En GCP el proyecto es parte del diseño: agrupa permisos, facturación, cuotas y
gobierno. No lo trates como un detalle administrativo.
</details>

Solución compilable: `examples/soluciones/gcp_practice_nivel_3.rs`.

## Ejercicio 4: diseño GCP para Jeresoft Academy `[Nivel 4]`

Diseña una matriz para una versión mínima de Jeresoft Academy en GCP:

- frontend estático;
- API de cursos;
- assets públicos;
- base de datos;
- cola de publicación;
- observabilidad;
- presupuesto por ambiente.

Para cada componente declara capacidad, servicio candidato, proyecto, región,
ambiente, dueño, propósito, responsabilidad retenida, señal operativa y señal
de costo.

**Entrada/Salida esperada:** una tabla con diez columnas: componente,
capacidad, servicio, proyecto, región, ambiente, dueño, responsabilidad
retenida, señal operativa y señal de costo.

<details>
<summary>Pista</summary>
Evita responder con equivalencias AWS/GCP. El valor está en explicar qué
capacidad representa cada servicio y qué frontera conserva el equipo.
</details>

Discusión sugerida:

- Cloud Run puede servir una API si se declaran límites, identidad y señales.
- Cloud Storage puede guardar assets, pero el acceso público debe ser
  intencional.
- Pub/Sub ayuda a enseñar eventos si se declara idempotencia y destino.
- Cloud SQL representa estado relacional, pero exige recuperación y costo
  visible.
- Budgets y labels vuelven visible la responsabilidad; no corrigen una mala
  arquitectura por sí solos.
