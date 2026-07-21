# Costos y FinOps

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 08. Costos y FinOps
- **Issues:** #29, #30, #31, #32
- **Módulo Rust:** `src/finops.rs`
- **Diagrama:** `diagrams/08-costos-y-finops.mmd`
- **Ejemplo:** `examples/finops.rs`
- **Ejercicios:** `docs/ejercicios/08-costos-y-finops.md`
- **Costos:** `docs/costos/08-costos-y-finops.md`

## Concepto

Costos y FinOps es la disciplina que convierte el gasto cloud en una decisión
visible de ingeniería, producto y operación. No se limita a ahorrar dinero: busca
entender qué uso genera costo, qué valor produce, qué señales permiten corregir
rumbo y qué responsabilidades tiene cada equipo.

En este curso, FinOps se estudia como un ciclo de **visibilidad, atribución,
decisión y ajuste**. La pregunta central no es "¿cuánto cuesta AWS o GCP?",
sino "¿qué decisión técnica generó este costo, qué valor sostiene y qué cambio
responsable podemos hacer sin romper el sistema?".

## Imagen mental

Piensa en una cocina profesional que vende comidas por pedido.

- El **ticket promedio** dice cuánto valor entra por venta.
- La **receta** define qué ingredientes, equipo y pasos consume cada plato.
- La **merma** muestra desperdicio de insumos.
- El **horno encendido** cuesta aunque no haya pedidos.
- El **reparto** agrega costo por distancia, urgencia y volumen.
- El **tablero diario** permite corregir antes de cerrar el mes.

Cloud funciona de forma parecida. Una arquitectura no solo debe responder bien;
debe explicar qué consume, por qué lo consume y cuándo ese consumo deja de
comprar valor real.

## Problema

Cloud permite crear recursos elásticos, pagar por uso, probar rápido y delegar
operación. Esa misma flexibilidad puede convertir el costo en sorpresa: recursos
olvidados, logs sin retención, tráfico entre regiones, bases sobredimensionadas,
concurrencia sin límite, almacenamiento acumulado, entornos duplicados y
servicios que nadie siente propios.

La enseñanza de costos suele caer en tres errores:

- reducir FinOps a "apagar recursos";
- presentar precios de proveedor sin explicar las decisiones que los producen;
- tratar costo y arquitectura como conversaciones separadas.

Sin una base clara, el estudiante puede creer que optimizar costo es un acto
contable posterior, no una parte del diseño de sistemas. También puede creer que
una arquitectura barata es automáticamente buena, o que una factura alta siempre
es mala, sin distinguir desperdicio, inversión deliberada y riesgo aceptado.

## Alternativas consideradas

1. **Empezar por calculadoras de proveedor.** Son útiles para estimaciones, pero
   cambian rápido y pueden ocultar el modelo de consumo.
2. **Empezar por reglas de ahorro.** Es práctico, pero vuelve el capítulo una
   lista de consejos sin criterio transferible.
3. **Empezar por unidad de costo, atribución y tradeoff operativo.** Permite
   razonar sobre recursos, tráfico, datos, observabilidad y capacidad antes de
   aterrizar precios fechados.

## Justificación

Este capítulo adopta la tercera alternativa. Primero se modela el costo como una
relación entre unidad técnica, uso, dueño, criticidad, valor, señales y decisión
de optimización. Después se podrán usar precios de proveedor como material vivo,
fechado y revisado.

La decisión conserva RFC-0001 §2: concepto antes de implementación, problema
antes de herramienta, alternativas antes de elección. También mantiene
RFC-0001 §10: proveedor después de fundamentos.

## Invariantes del capítulo

- Todo costo cloud debe tener dueño operativo o de producto.
- Un costo no atribuido es una deuda de observabilidad y gobierno.
- Optimizar costo sin entender valor puede degradar el sistema.
- El costo se diseña: capacidad, datos, red, logs, reintentos y regiones importan.
- La unidad de costo debe poder nombrarse: invocación, GiB, petición, hora,
  transferencia, índice, transición o retención.
- La elasticidad necesita límites; sin límites, el costo también escala.
- Los ambientes no productivos también tienen costo y ciclo de vida.
- Los ahorros deben medirse contra riesgo, latencia, operación y experiencia.
- FinOps no reemplaza arquitectura; la vuelve económicamente observable.
- Los precios de proveedor son material vivo y deben fecharse cuando se usen.

## Comparación educativa

| Elemento | Qué decide | Qué no decide por sí solo | Riesgo común |
|----------|------------|----------------------------|--------------|
| Unidad de costo | Qué recurso explica el gasto | Valor producido | Medir consumo sin contexto |
| Atribución | Quién entiende el gasto | Optimización automática | Facturas huérfanas |
| Propósito | Qué valor compra el consumo | Precio correcto | Defender gasto inútil |
| Límite | Hasta dónde puede crecer | Uso saludable | Confundir elasticidad con infinito |
| Señal de uso | Cuándo investigar | Acción adecuada | Alertar tarde o sin dueño |
| Tradeoff | Qué riesgo se acepta | Ahorro garantizado | Romper confiabilidad por ahorrar |

## Modelo Rust mínimo

El módulo Rust mínimo vive en `src/finops.rs` y modela, sin dependencias
externas:

- unidades educativas de costo: cómputo, almacenamiento, red, observabilidad,
  invocaciones y servicios manejados;
- dueño, propósito y ambiente del gasto;
- frecuencia o intensidad de uso;
- criticidad y elasticidad;
- hallazgos cuando falten dueño, propósito, presupuesto, unidad económica o
  visibilidad suficiente;
- evaluación de riesgos por gasto no atribuido, elasticidad sin límite,
  observabilidad excesiva o recursos constantes sobredimensionados.

El módulo no debe intentar calcular precios reales. Su función es pedagógica:
hacer visibles las variables que explican por qué aparece un costo.

## Lectura del modelo

`FinOpsProfile` junta cuatro preguntas que suelen quedar separadas:

1. **Qué se consume:** categoría de costo, ambiente y patrón de uso.
2. **Por qué se consume:** propósito, criticidad e intención de optimización.
3. **Quién lo cuida:** dueño y control de presupuesto.
4. **Cómo se observa:** visibilidad, unidad económica y límite de elasticidad.

El modelo prefiere nombres explícitos sobre fórmulas. Un estudiante no necesita
memorizar precios para detectar que un gasto sin dueño, sin presupuesto, sin
unidad económica y sin límite de elasticidad todavía no está gobernado.

## Cómo leer el módulo Rust

El módulo `finops` empieza con un perfil explícito:

```rust
use rust_cloud::finops::{
    BudgetControl, CostCategory, CostVisibility, ElasticityLimit, Environment,
    FinOpsCriticality, FinOpsProfile, FinOpsRequirements, OptimizationIntent,
    UsagePattern,
};

let profile = FinOpsProfile::new(
    "academy-api",
    FinOpsRequirements {
        category: CostCategory::Compute,
        environment: Environment::Production,
        usage_pattern: UsagePattern::Steady,
        criticality: FinOpsCriticality::High,
        elasticity: ElasticityLimit::Bounded { max_units: 20 },
        visibility: CostVisibility::UnitEconomics,
        budget_control: BudgetControl::ForecastAndReview,
        optimization_intent: OptimizationIntent::ReduceWaste,
        owner: "equipo academy",
        purpose: "servir rutas de aprendizaje a estudiantes",
        unit_economics: "costo por estudiante activo",
    },
)
.unwrap();

assert!(profile.evaluate().is_low_risk());
```

Un costo sin dueño, sin unidad económica y con elasticidad sin límite produce
hallazgos:

```rust
use rust_cloud::finops::{
    BudgetControl, CostCategory, CostVisibility, ElasticityLimit, Environment,
    FinOpsCriticality, FinOpsFinding, FinOpsProfile, FinOpsRequirements,
    OptimizationIntent, UsagePattern,
};

let profile = FinOpsProfile::new(
    "preview-workers",
    FinOpsRequirements {
        category: CostCategory::Invocations,
        environment: Environment::Development,
        usage_pattern: UsagePattern::Growing,
        criticality: FinOpsCriticality::Medium,
        elasticity: ElasticityLimit::Unbounded,
        visibility: CostVisibility::Aggregate,
        budget_control: BudgetControl::None,
        optimization_intent: OptimizationIntent::None,
        owner: "",
        purpose: "ejecutar previews automáticos",
        unit_economics: "",
    },
)
.unwrap();

assert!(profile.evaluate().findings().contains(
    &FinOpsFinding::UnboundedElasticity("preview-workers"),
));
```

El objetivo del modelo no es decidir si un recurso es caro o barato. El objetivo
es obligar a explicar qué valor compra, qué señal lo vuelve atribuible y qué
tradeoff aparece antes de optimizarlo.

## Diagrama

El diagrama del capítulo vive en `diagrams/08-costos-y-finops.mmd`. Resume la
lectura principal:

```text
recurso -> unidad de costo -> dueño/proposito -> visibilidad -> presupuesto -> tradeoff -> decision
```

La rama crítica del diagrama aparece cuando una señal no puede atribuirse o
cuando un recurso elástico no tiene límite. En ambos casos, el problema no es
solo económico: también es de arquitectura y operación.

El diagrama separa tres tipos de lectura:

1. **Lectura normal:** el costo tiene unidad, dueño, propósito, señal,
   presupuesto y decisión explícita.
2. **Lectura de gobierno:** si el costo no se puede atribuir, el primer trabajo
   no es ahorrar; es volverlo explicable.
3. **Lectura de elasticidad:** si un recurso puede crecer sin límite, la
   arquitectura debe declarar qué protege a la factura, a las dependencias y al
   producto.

## Ejemplo ejecutable

El ejemplo `examples/finops.rs` compara un costo de producción atribuido y
gobernable contra invocaciones de desarrollo que crecen sin dueño, presupuesto
ni unidad económica. También incluye un recurso productivo casi sin uso para
mostrar que "barato" y "gobernado" no son sinónimos.

```bash
cargo run --example finops
```

Salida esperada:

```text
academy-api: costo atribuido y gobernable
preview-workers: 5 hallazgos educativos
legacy-worker: 1 hallazgos educativos
```

El ejemplo no consulta proveedores ni precios actuales. Su intención es mostrar
qué señales deben existir antes de abrir una calculadora o una factura real.

## Ejemplos progresivos

El capítulo usa tres niveles de lectura:

| Nivel | Escenario | Señal principal | Aprendizaje |
|-------|-----------|-----------------|-------------|
| Básico | API de producción con presupuesto y unidad económica | Sin hallazgos | El costo puede defenderse porque tiene dueño, valor y control |
| Intermedio | Workers de preview con invocaciones crecientes | Elasticidad sin límite | La elasticidad necesita presupuesto, atribución y límite explícito |
| Avanzado | Worker productivo casi inactivo | Recurso idle | Un recurso puede ser estable y aun así revelar desperdicio |

Estos escenarios son deliberadamente pequeños. La intención no es simular una
factura completa, sino entrenar la pregunta correcta antes de comparar
proveedores: qué decisión técnica explica el consumo y qué acción responsable
permite tomar.

## Decisiones registradas

- El perfil principal se llama `FinOpsProfile`.
- Las categorías educativas viven en `CostCategory`.
- La elasticidad se modela con `ElasticityLimit` para distinguir recursos no
  elásticos, recursos acotados y crecimiento sin límite visible.
- La visibilidad se modela con `CostVisibility`, separando gasto agregado de
  atribución y unidad económica.
- `FinOpsFinding` hace visibles costos sin dueño, propósito, unidad económica,
  presupuesto, límite o visibilidad suficiente.

## Ejercicios y costos

Los ejercicios viven en `docs/ejercicios/08-costos-y-finops.md` y tienen
soluciones compilables en `examples/soluciones/finops_nivel_*.rs`.

El análisis de costos vive en `docs/costos/08-costos-y-finops.md`. No usa
precios de proveedor ni agrega benchmarks: compara atribución, unidad
económica, elasticidad, observabilidad, presupuestos, ambientes y desperdicio.

## Práctica sugerida

Antes de optimizar una arquitectura cloud, escribe:

1. Unidad económica: por estudiante, transacción, curso, documento, minuto o
   evento.
2. Recurso dominante: cómputo, almacenamiento, red, base de datos,
   observabilidad u operación humana.
3. Dueño: quién puede interpretar y corregir la señal.
4. Valor: qué capacidad legítima compra ese gasto.
5. Desperdicio probable: qué consumo no compra valor.
6. Riesgo: qué se puede romper al optimizar.
7. Control: presupuesto, alerta, reporte o revisión periódica.

Si no puedes explicar qué valor compra un costo, todavía no deberías
optimizarlo: primero debes entenderlo.

## Estado editorial

Este capítulo queda en `implemented`. No está marcado como `reviewed` ni
`published`.
