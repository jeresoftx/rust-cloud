# Serverless

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** draft
- **Milestone:** 07. Serverless
- **Issue:** #25
- **Módulo Rust:** `src/serverless.rs`

## Concepto

Serverless es un modelo de ejecución donde el equipo diseña unidades de trabajo
activadas por eventos y delega al proveedor gran parte de la provisión,
escalado, disponibilidad base y administración de servidores. No significa que
no existan servidores; significa que el contrato de operación cambia.

En este curso, serverless se estudia como una decisión de acoplamiento entre
evento, función, límite, estado y operación. La pregunta central no es "¿qué
función puedo subir?", sino "¿qué evento dispara qué trabajo, con qué límites,
qué estado toca, qué pasa si falla y qué costo aparece al escalar?".

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

## Requisitos para `src/serverless.rs`

El módulo Rust mínimo deberá modelar, sin dependencias externas:

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

## Decisiones pendientes

- Definir si una carga serverless se modela como función, workflow o workload
  genérico.
- Nombrar errores y hallazgos públicos antes de escribir ejemplos.
- Definir umbrales educativos de timeout y concurrencia.
- Decidir cómo representar idempotencia sin sobreprometer seguridad real.

## Estado editorial

Este capítulo queda en `draft`. No está marcado como `reviewed` ni `published`.
