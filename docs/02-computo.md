# Cómputo

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 02. Cómputo
- **Issues:** #5, #6, #7
- **Módulo Rust:** `src/compute.rs`
- **Diagrama:** `diagrams/02-computo.mmd`
- **Ejemplo:** `examples/compute.rs`

## Concepto

Cómputo es la capacidad de ejecutar trabajo: procesos, servicios, tareas,
contenedores, funciones y cargas programadas. En cloud, esa capacidad no se
entiende solo como "una máquina", sino como una frontera entre recursos,
aislamiento, ciclo de vida, elasticidad y responsabilidad operativa.

La pregunta central del capítulo no es "¿qué instancia uso?", sino "¿qué forma
de ejecución necesita esta carga y qué operación implica sostenerla?".

Para este curso, los modos canónicos de cómputo son:

- **Máquina virtual:** control amplio sobre sistema operativo y runtime, con
  carga operativa alta.
- **Contenedor:** empaquetado reproducible y frontera de proceso más portable,
  pero todavía requiere una plataforma que lo ejecute.
- **Servicio administrado de contenedores:** el equipo entrega imágenes y
  configuración; la plataforma administra parte del despliegue y escalado.
- **Función/evento:** el equipo entrega unidades pequeñas de lógica; la
  plataforma decide capacidad y ejecución.
- **Trabajo batch o programado:** la carga corre por ventanas, colas o eventos,
  y la decisión gira alrededor de duración, concurrencia y reintentos.

Estos modos no compiten por moda. Cada uno expresa un contrato distinto entre
control, aislamiento y carga operativa.

## Imagen mental

Piensa en un taller donde llegan distintos tipos de trabajo.

- Algunas piezas necesitan una mesa completa, herramientas propias y control
  fino: se parecen a una **máquina virtual**.
- Otras llegan empacadas en una caja reproducible: se parecen a un
  **contenedor**.
- Algunas cajas pueden entregarse a una línea de producción que escala con la
  demanda: se parecen a un **contenedor administrado**.
- Otras son instrucciones pequeñas que se ejecutan cuando ocurre algo: se
  parecen a una **función**.
- Otras se acumulan y se procesan por turnos: se parecen a un **trabajo batch**.

La decisión no empieza por la herramienta. Empieza por la forma del trabajo.

## Problema

El cómputo cloud suele enseñarse como una lista de productos: instancias,
contenedores, funciones, jobs o clusters. Ese enfoque es práctico, pero puede
ocultar la decisión duradera: cómo corre la carga, quién la reinicia, cómo
escala, qué se aísla, qué falla y qué costo aparece cuando cambia el volumen.

Sin esta base, un estudiante puede creer que "contenedor" significa
automáticamente "arquitectura moderna" o que "serverless" significa
automáticamente "barato". Ambas ideas son peligrosas: la forma de cómputo debe
elegirse por restricciones de ejecución, no por etiqueta.

## Alternativas

### Empezar por máquinas virtuales

Es concreto y permite entender CPU, memoria, disco, red y sistema operativo.
Pero si el capítulo se queda ahí, Cloud parece una sala de servidores remota.

### Empezar por contenedores y funciones

Conecta con prácticas actuales, pero puede saltarse fundamentos: scheduling,
aislamiento, límites, arranque, reintentos y capacidad.

### Definir cargas y restricciones primero

El capítulo empieza por el trabajo que debe ejecutarse y después compara formas
de cómputo. Esta alternativa separa el fundamento estable de los productos que
lo materializan.

## Justificación

Se elige definir cargas y restricciones primero porque conserva la regla de
RFC-0001 §2: concepto, problema, alternativas, justificación y código. Además
sigue RFC-0001 §10: los proveedores aparecen después del fundamento. EC2, Cloud
Run, Lambda, Cloud Functions o Kubernetes pueden servir como ejemplos, pero no
como fuente de verdad del capítulo.

## Invariantes del capítulo

- Toda carga consume CPU, memoria, red y tiempo de ejecución.
- Toda forma de cómputo tiene una frontera de aislamiento.
- Toda carga necesita política de ciclo de vida: iniciar, detener, reiniciar,
  reemplazar o reintentar.
- La elasticidad no elimina límites; mueve la conversación hacia cuotas,
  arranque, concurrencia, capacidad y costo.
- Más control operativo no implica mejor arquitectura.
- Menos operación directa no implica menor costo ni menor complejidad.
- La forma de cómputo debe elegirse por restricciones de carga, no por marca.
- Los ejemplos de proveedor deben estar fechados o tratarse como revisables
  cuando dependan de límites, precios o capacidades actuales.

## Requisitos funcionales del modelo Rust

El módulo `src/compute.rs` representa, en una primera versión:

- formas de cómputo;
- recursos solicitados por una carga;
- límites y cuotas educativas;
- estrategia de ciclo de vida;
- señales de elasticidad;
- errores explícitos cuando una carga no declara requisitos mínimos.

## Requisitos no funcionales

- Sin dependencias externas.
- Sin `unsafe`.
- API pública documentada con doctests.
- Pruebas para límites, clasificación de carga y errores de decisión.
- Código pequeño: el objetivo es razonar cómputo, no simular un scheduler real.

## Comparación educativa

| Modo | Cuándo encaja | Qué controla el equipo | Riesgo si se elige por moda |
|------|---------------|------------------------|------------------------------|
| Máquina virtual | Cargas que requieren sistema operativo, runtime o configuración fina | Sistema operativo, runtime, parches, escalado y operación | Convertir Cloud en servidores remotos administrados a mano |
| Contenedor | Cargas empacadas y reproducibles que necesitan portabilidad | Imagen, proceso, runtime de aplicación y parte del despliegue | Creer que empaquetar resuelve operación, red y capacidad |
| Contenedor administrado | Servicios HTTP o workers con imagen de contenedor y deseo de menor operación | Imagen, variables, límites, health checks y reglas de despliegue | Delegar sin entender cuotas, arranque, observabilidad y costo |
| Función | Cargas cortas, orientadas a eventos y con baja operación base | Código, handler, permisos y contrato de evento | Fragmentar lógica o depender demasiado del modelo de eventos |
| Trabajo batch | Procesamiento por lotes, horario, cola o ventana de ejecución | Entradas, reintentos, concurrencia y límites de recursos | Tratar un trabajo temporal como servicio permanente |

La tabla no reemplaza una evaluación real. Solo ordena el razonamiento para que
la conversación empiece por restricciones de carga.

## Cómo leer el módulo Rust

El módulo `compute` empieza por una carga:

```rust
use rust_cloud::compute::{ResourceRequest, WorkloadRequirements};

let workload = WorkloadRequirements {
    resources: Some(ResourceRequest::new(1, 512).unwrap()),
    event_driven: true,
    wants_low_operational_load: true,
    ..WorkloadRequirements::default()
};
```

Después se pide una recomendación educativa:

```rust
use rust_cloud::compute::{ComputeMode, recommend_compute};

# let workload = rust_cloud::compute::WorkloadRequirements {
#     resources: Some(rust_cloud::compute::ResourceRequest::new(1, 512).unwrap()),
#     event_driven: true,
#     wants_low_operational_load: true,
#     ..rust_cloud::compute::WorkloadRequirements::default()
# };
let mode = recommend_compute(workload);
assert_eq!(mode, Ok(ComputeMode::Function));
```

La API obliga a declarar recursos mínimos. Si no hay CPU y memoria, el modelo
no recomienda. Esto refleja una regla sencilla: no se elige plataforma sin
nombrar lo que la carga necesita para existir.

## Diagrama

El diagrama del capítulo vive en `diagrams/02-computo.mmd`. Resume la lectura
principal:

```text
carga -> restricciones -> modo de cómputo -> tradeoff operativo
```

## Ejemplo ejecutable

El ejemplo `examples/compute.rs` compara perfiles y recomienda modos para tres
cargas pequeñas:

```bash
cargo run --example compute
```

El ejemplo no contacta proveedores ni calcula precios. Su intención es mostrar
que cada forma de cómputo nace de supuestos explícitos.

## Decisiones registradas en el modelo Rust

- Las formas de cómputo del modelo son: máquina virtual, contenedor,
  contenedor administrado, función y trabajo batch.
- Contenedor administrado y función se modelan como variantes separadas porque
  enseñan fronteras distintas de ciclo de vida, escalado y control.
- Elasticidad, control y carga operativa se representan como puntajes
  educativos (`Low`, `Medium`, `High`), no como métricas reales.
- Una recomendación sin recursos mínimos devuelve error explícito.
- Una carga que exige control de sistema operativo y baja carga operativa a la
  vez devuelve conflicto: el modelo no decide el tradeoff por el estudiante.

## Práctica sugerida

Antes de escoger una forma de cómputo, escribe:

1. Recursos mínimos: CPU, memoria, duración y concurrencia esperada.
2. Ciclo de vida: servicio permanente, petición/respuesta, evento o batch.
3. Frontera de aislamiento: máquina, contenedor, proceso o runtime manejado.
4. Señal de escalado: manual, CPU/memoria, cola, evento u horario.
5. Tradeoff aceptado: control, operación, portabilidad, costo o límites.

Si la respuesta no menciona recursos y ciclo de vida, todavía no es una decisión
de cómputo; es una preferencia por una herramienta.

## Estado editorial

Este capítulo está en `implemented`. No está marcado como `reviewed` ni
`published`.
