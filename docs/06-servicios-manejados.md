# Servicios manejados

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 06. Servicios manejados
- **Issues:** #21, #22, #23, #24
- **Módulo Rust:** `src/managed_services.rs`
- **Diagrama:** `diagrams/06-servicios-manejados.mmd`
- **Ejemplo:** `examples/managed_services.rs`
- **Ejercicios:** `docs/ejercicios/06-servicios-manejados.md`
- **Costos:** `docs/costos/06-servicios-manejados.md`

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

## Imagen mental

Piensa en un taller que renta máquinas especializadas.

- El **proveedor** mantiene la máquina, repuestos, energía y monitoreo base.
- El **equipo** decide qué produce, con qué insumos, bajo qué permisos y cómo
  valida que el producto final sirve.
- El **contrato operativo** dice qué pasa si la máquina falla, cuándo se le da
  mantenimiento y qué límites no debe cruzar.
- La **recuperación** no es tener una copia guardada: es saber restaurar y
  comprobar que el sistema vuelve.

La metáfora separa comodidad de responsabilidad. Usar una máquina administrada
puede ser una excelente decisión, pero no elimina diseño, permisos, datos,
observabilidad ni costos.

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

## Comparación educativa

| Elemento | Qué decide | Qué no decide por sí solo | Riesgo común |
|----------|------------|----------------------------|--------------|
| Capacidad | Qué problema resuelve el servicio | Producto comercial correcto | Elegir por marca antes que por necesidad |
| Delegación | Qué opera el proveedor | Responsabilidad total | Creer que manejado significa sin operación |
| Responsabilidad retenida | Qué conserva el equipo | Implementación automática | Olvidar permisos, datos, costo y observabilidad |
| Criticidad | Qué tanto duele una falla | Estrategia de recuperación | Tratar servicios críticos como accesorios |
| Recuperación | Cómo se vuelve después de fallar | Que el restore funcione | Tener backup sin prueba de restauración |
| Dueño operativo | Quién cuida el servicio | Que todo esté bien configurado | Servicios sin responsable claro |

## Cómo leer el módulo Rust

El módulo `managed_services` empieza con requerimientos explícitos:

```rust
use rust_cloud::managed_services::{
    Criticality, ManagedCapability, ManagedServiceProfile,
    ManagedServiceRequirements, RecoveryStrategy,
};

let profile = ManagedServiceProfile::new(
    "academy-db",
    ManagedServiceRequirements {
        capability: ManagedCapability::Database,
        durable_state: true,
        criticality: Criticality::High,
        recovery: RecoveryStrategy::TestedRestore,
        has_owner: true,
        purpose: "guardar progreso y evaluaciones del estudiante",
    },
)
.unwrap();

assert!(profile.evaluate().is_low_risk());
```

Un servicio crítico con estado durable y sin restauración probada produce un
hallazgo educativo:

```rust
use rust_cloud::managed_services::{
    Criticality, ManagedCapability, ManagedServiceFinding,
    ManagedServiceProfile, ManagedServiceRequirements, RecoveryStrategy,
};

let profile = ManagedServiceProfile::new(
    "academy-db",
    ManagedServiceRequirements {
        capability: ManagedCapability::Database,
        durable_state: true,
        criticality: Criticality::High,
        recovery: RecoveryStrategy::BackupOnly,
        has_owner: true,
        purpose: "guardar progreso y evaluaciones del estudiante",
    },
)
.unwrap();

assert!(profile.evaluate().findings().contains(
    &ManagedServiceFinding::CriticalServiceWithoutTestedRecovery("academy-db"),
));
```

El objetivo del modelo no es decir qué servicio comprar. El objetivo es obligar
a nombrar la capacidad, la responsabilidad delegada, la responsabilidad
retenida, la criticidad y la recuperación antes de convertir la decisión en
infraestructura.

## Diagrama

El diagrama del capítulo vive en `diagrams/06-servicios-manejados.mmd`. Resume
la lectura principal:

```text
capacidad -> delegación -> responsabilidad retenida -> recuperación -> hallazgos
```

## Ejemplo ejecutable

El ejemplo `examples/managed_services.rs` compara una base de datos administrada
con restauración probada contra un índice de búsqueda durable sin estrategia de
recuperación.

```bash
cargo run --example managed_services
```

El ejemplo no contacta proveedores ni consulta precios. Su intención es mostrar
qué preguntas operativas deben quedar visibles antes de elegir un servicio real.

## Ejercicios y costos

Los ejercicios viven en `docs/ejercicios/06-servicios-manejados.md` y tienen
soluciones compilables en `examples/soluciones/managed_services_nivel_*.rs`.

El análisis de costos vive en `docs/costos/06-servicios-manejados.md`. No usa
precios de proveedor ni agrega herramientas de benchmark: compara delegación,
operación retenida, recuperación, observabilidad, consumo y dependencia.

## Práctica sugerida

Antes de adoptar un servicio manejado, escribe:

1. Capacidad: qué problema resuelve.
2. Estado: si conserva datos durables o solo procesa flujo temporal.
3. Delegación: qué operación queda en manos del proveedor.
4. Responsabilidad retenida: qué sigue cuidando el equipo.
5. Criticidad: qué pasa si falla.
6. Recuperación: cómo se respalda, restaura y prueba.
7. Dueño: quién responde por configuración, costo y revisión.
8. Límites: cuotas, throughput, tamaño, latencia y región.
9. Salida: qué tan difícil sería migrar o extraer datos.

Si el servicio es crítico y nadie ha probado restaurarlo, todavía no está listo
como decisión de producción.

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
