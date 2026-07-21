# Serverless

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 07. Serverless
- **Issues:** #25, #26, #27, #28
- **Módulo Rust:** `src/serverless.rs`
- **Diagrama:** `diagrams/07-serverless.mmd`
- **Ejemplo:** `examples/serverless.rs`
- **Ejercicios:** `docs/ejercicios/07-serverless.md`
- **Costos:** `docs/costos/07-serverless.md`

## Concepto

Serverless es un modelo de ejecución donde el equipo diseña unidades de trabajo
activadas por eventos y delega al proveedor gran parte de la provisión,
escalado, disponibilidad base y administración de servidores. No significa que
no existan servidores; significa que el contrato de operación cambia.

En este curso, serverless se estudia como una decisión de acoplamiento entre
evento, función, límite, estado y operación. La pregunta central no es "¿qué
función puedo subir?", sino "¿qué evento dispara qué trabajo, con qué límites,
qué estado toca, qué pasa si falla y qué costo aparece al escalar?".

## Imagen mental

Piensa en una cocina de pedidos rápidos.

- El **evento** es la orden que entra.
- La **función** es la estación que prepara una parte del pedido.
- El **timeout** es el tiempo máximo antes de cancelar la preparación.
- La **concurrencia** es cuántas órdenes se aceptan al mismo tiempo.
- El **retry** es volver a intentar una orden fallida.
- La **idempotencia** evita cobrar, guardar o enviar dos veces por el mismo
  evento.

La metáfora sirve para recordar que serverless no es ausencia de operación. Es
operación movida a otro contrato: eventos, límites, reintentos y observabilidad.

## Problema

Serverless permite construir flujos pequeños, elásticos y orientados a eventos
sin administrar máquinas de forma directa. Esa elasticidad también puede ocultar
deuda: cold starts, timeouts, reintentos duplicados, límites de concurrencia,
observabilidad fragmentada, dependencia de proveedor, costo por invocación y
dificultad para probar flujos distribuidos.

La enseñanza de serverless suele caer en tres errores:

- presentarlo como "sin servidores" y no como cambio de responsabilidad;
- enseñar funciones aisladas sin hablar de eventos, idempotencia y reintentos;
- vender escalado automático sin explicar límites, cuotas, latencia y costo.

Sin una base clara, el estudiante puede creer que una función pequeña no tiene
arquitectura, que un retry es inocuo, o que pagar por uso siempre es más barato
que mantener capacidad estable.

## Alternativas consideradas

1. **Empezar por funciones como producto.** Es rápido para una demo, pero
   reduce serverless a subir handlers.
2. **Empezar por eventos y colas.** Explica bien el flujo, pero puede dejar de
   lado límites de ejecución, cold starts y costo.
3. **Empezar por contrato de ejecución event-driven.** Permite unir evento,
   handler, estado, límites, reintentos, observabilidad y costo antes de hablar
   de un proveedor.

## Justificación

Este capítulo adopta la tercera alternativa. Primero se modela serverless como
un contrato entre evento, unidad de ejecución, límite temporal, concurrencia,
estado, retry e idempotencia. Después se podrán comparar funciones, workflows,
colas, schedulers y APIs serverless en proveedores concretos.

La decisión conserva RFC-0001 §2: concepto antes de implementación, problema
antes de herramienta, alternativas antes de elección. También mantiene
RFC-0001 §10: proveedor después de fundamentos.

## Invariantes del capítulo

- Serverless no elimina servidores; cambia quién administra capacidad base.
- Toda función serverless debe tener disparador, propósito y límite temporal.
- Los retries requieren idempotencia o una compensación explícita.
- Escalado automático no significa escalado infinito.
- El estado durable debe vivir fuera de la función o quedar justificado.
- Cold start, timeout y concurrencia son parte del diseño, no detalles finales.
- Observabilidad debe cubrir evento, ejecución, error, retry y resultado.
- Pagar por uso favorece cargas variables, pero puede penalizar flujos masivos o
  constantes.
- La dependencia de proveedor crece cuando eventos, permisos y workflows quedan
  acoplados a servicios específicos.
- Los nombres, precios y límites de proveedor son material vivo y deben
  revisarse cuando se usen ejemplos fechados.

## Modelo Rust mínimo

El módulo Rust mínimo vive en `src/serverless.rs` y modela, sin dependencias
externas:

- tipos de disparador: HTTP, cola, storage, scheduler y evento interno;
- unidad serverless con nombre, propósito y límites;
- duración máxima y concurrencia máxima educativas;
- política de retry e idempotencia;
- relación con estado durable externo;
- evaluación de hallazgos cuando falten propósito, límite, idempotencia o
  observabilidad;
- señales de riesgo para retries no idempotentes, timeouts altos y concurrencia
  sin límite.

El módulo no debe intentar simular una plataforma serverless real. Su función es
pedagógica: hacer visible el contrato de ejecución antes de hablar de AWS Lambda,
Cloud Functions, Cloud Run, workflows u otros productos.

## Comparación educativa

| Elemento | Qué decide | Qué no decide por sí solo | Riesgo común |
|----------|------------|----------------------------|--------------|
| Trigger | Qué inicia la ejecución | Idempotencia | Tratar todo evento como único |
| Runtime | Cómo se empaqueta el trabajo | Diseño del flujo | Usar función para procesos largos |
| Timeout | Cuánto puede durar | Latencia aceptable | Ocultar lentitud subiendo el límite |
| Concurrencia | Cuánto escala a la vez | Capacidad downstream | Saturar bases, APIs o colas |
| Retry | Cómo reacciona a falla | Seguridad de repetir | Duplicar efectos secundarios |
| Estado | Qué se lee o escribe fuera | Consistencia real | Guardar estado implícito en la función |
| Observabilidad | Qué se puede investigar | Corrección automática | Ver logs sin correlación de evento |

## Cómo leer el módulo Rust

El módulo `serverless` empieza con requerimientos explícitos:

```rust
use rust_cloud::serverless::{
    IdempotencyStrategy, ObservabilityPlan, RetryPolicy, RuntimeProfile,
    ServerlessRequirements, ServerlessWorkload, StateAccess, TriggerKind,
};

let workload = ServerlessWorkload::new(
    "process-publication",
    ServerlessRequirements {
        trigger: TriggerKind::Queue,
        runtime: RuntimeProfile::Function,
        timeout_seconds: Some(30),
        max_concurrency: Some(50),
        retry_policy: RetryPolicy::Backoff { attempts: 3 },
        idempotency: IdempotencyStrategy::EventKey,
        state_access: StateAccess::ExternalWrite,
        observability: ObservabilityPlan::standard(),
        purpose: "procesar eventos de publicación de contenido",
    },
)
.unwrap();

assert!(workload.evaluate().is_low_risk());
```

Una función que reintenta, escribe estado y no declara idempotencia produce
hallazgos:

```rust
use rust_cloud::serverless::{
    IdempotencyStrategy, ObservabilityPlan, RetryPolicy, RuntimeProfile,
    ServerlessFinding, ServerlessRequirements, ServerlessWorkload,
    StateAccess, TriggerKind,
};

let workload = ServerlessWorkload::new(
    "charge-payment",
    ServerlessRequirements {
        trigger: TriggerKind::Http,
        runtime: RuntimeProfile::Function,
        timeout_seconds: Some(120),
        max_concurrency: None,
        retry_policy: RetryPolicy::Fixed { attempts: 2 },
        idempotency: IdempotencyStrategy::None,
        state_access: StateAccess::ExternalWrite,
        observability: ObservabilityPlan::none(),
        purpose: "registrar pago de estudiante",
    },
)
.unwrap();

assert!(workload.evaluate().findings().contains(
    &ServerlessFinding::RetryWithoutIdempotency("charge-payment"),
));
```

El objetivo del modelo no es imitar Lambda, Cloud Run o Workflows. El objetivo
es obligar a nombrar evento, límite, retry, idempotencia, estado y señales de
observabilidad antes de convertir el flujo en infraestructura.

## Diagrama

El diagrama del capítulo vive en `diagrams/07-serverless.mmd`. Resume la
lectura principal:

```text
trigger -> workload -> límites -> retry/idempotencia -> evaluación
```

## Ejemplo ejecutable

El ejemplo `examples/serverless.rs` compara un handler de cola con idempotencia
contra una función HTTP riesgosa que reintenta, escribe estado y no declara
observabilidad suficiente.

```bash
cargo run --example serverless
```

El ejemplo no contacta proveedores ni ejecuta funciones reales. Su intención es
mostrar qué decisiones deben quedar visibles antes de elegir un runtime
serverless.

## Ejercicios y costos

Los ejercicios viven en `docs/ejercicios/07-serverless.md` y tienen soluciones
compilables en `examples/soluciones/serverless_nivel_*.rs`.

El análisis de costos vive en `docs/costos/07-serverless.md`. No usa precios de
proveedor ni agrega benchmarks: compara elasticidad, límites, retries,
observabilidad, dependencia y carga downstream.

## Práctica sugerida

Antes de diseñar un flujo serverless, escribe:

1. Evento: qué dispara la ejecución.
2. Propósito: qué trabajo legítimo realiza.
3. Timeout: cuánto puede durar antes de fallar.
4. Concurrencia: qué límite protege a dependencias downstream.
5. Retry: cuántos intentos existen y con qué backoff.
6. Idempotencia: cómo se evita duplicar efectos secundarios.
7. Estado: qué se lee o escribe fuera de la función.
8. Observabilidad: logs, métricas y correlación por evento.
9. Costo: qué pasa si llegan miles o millones de eventos.

Si un flujo reintenta y escribe estado sin idempotencia, todavía no debería
considerarse listo para producción.

Los ejercicios graduados viven en `docs/ejercicios/07-serverless.md`. El
análisis de costos del capítulo vive en `docs/costos/07-serverless.md`.

## Decisiones registradas

- La unidad serverless se modela como `ServerlessWorkload` para cubrir función,
  contenedor y workflow sin atarse a un proveedor.
- Los disparadores viven en `TriggerKind`; el runtime educativo en
  `RuntimeProfile`.
- Los límites viven en `timeout_seconds` y `max_concurrency`.
- Retries e idempotencia se modelan por separado con `RetryPolicy` e
  `IdempotencyStrategy`.
- `StateAccess` distingue funciones sin estado de lecturas o escrituras en
  estado externo.
- `ServerlessFinding` vuelve visibles retries sin idempotencia, concurrencia sin
  límite, timeout alto, escritura de estado sin idempotencia y observabilidad
  incompleta.

## Estado editorial

Este capítulo queda en `implemented`. No está marcado como `reviewed` ni
`published`.
