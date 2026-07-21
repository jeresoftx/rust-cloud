# Modelos de servicio

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 01. Modelos de servicio
- **Issues:** #1, #2, #3
- **Módulo Rust:** `src/service_models.rs`
- **Diagrama:** `diagrams/01-modelos-de-servicio.mmd`
- **Ejemplo:** `examples/service_models.rs`
- **Ejercicios:** `docs/ejercicios/01-modelos-de-servicio.md`
- **Costos:** `docs/costos/01-modelos-de-servicio.md`

## Concepto

Un modelo de servicio cloud define qué responsabilidad se delega al proveedor y
qué responsabilidad conserva el equipo que construye el sistema. La pregunta
central no es "¿qué producto compro?", sino "¿quién opera cada capa y qué
control pierdo o gano al delegarla?".

Para este curso, los modelos canónicos son:

- **IaaS:** el proveedor entrega infraestructura flexible; el equipo conserva
  control operativo amplio sobre sistema operativo, runtime y despliegue.
- **PaaS:** el proveedor opera parte del runtime y de la plataforma; el equipo
  se concentra más en aplicación y datos.
- **SaaS:** el proveedor entrega una aplicación completa; el equipo consume una
  capacidad de negocio o colaboración.
- **Serverless:** el proveedor administra capacidad y escalado de manera más
  granular; el equipo entrega funciones, eventos o unidades pequeñas de lógica.

Estos nombres no son marcas ni recetas. Son contratos de responsabilidad.

## Imagen mental

Imagina que un equipo construye una cafetería dentro de un edificio.

- En **IaaS**, el edificio existe, pero el equipo decide cómo instalar cocina,
  máquinas, flujo de trabajo y operación diaria.
- En **PaaS**, el edificio ya trae cocina funcional; el equipo diseña el menú,
  prepara recetas y cuida su producto.
- En **SaaS**, el equipo renta una cafetería operada por alguien más; consume
  una capacidad completa y gobierna datos, reglas y uso.
- En **Serverless**, el equipo entrega recetas pequeñas que se activan por
  pedidos; la capacidad aparece y desaparece según la demanda.

La metáfora no busca ser perfecta. Sirve para recordar que cada modelo cambia
la pregunta central: qué queremos controlar y qué estamos dispuestos a delegar.

## Problema

Cloud suele enseñarse como una lista de servicios de proveedor. Ese enfoque
envejece rápido y esconde la decisión importante: qué se delega, qué se opera,
qué se paga, qué se puede cambiar después y qué límites quedan fuera de nuestro
control.

Sin esta base, un estudiante puede comparar una máquina virtual, una base de
datos manejada y una función serverless como si fueran elementos de un catálogo
equivalente. En realidad, cada opción cambia la frontera entre aplicación,
plataforma, operación, costo, seguridad y portabilidad.

## Alternativas

### Empezar por productos concretos

Por ejemplo, abrir con EC2, Cloud Run, Lambda o una base de datos manejada.
Tiene una ventaja práctica inmediata, pero amarra el capítulo a nombres,
pantallas y límites que cambian.

### Evitar proveedores por completo

Permite preservar conceptos, pero corre el riesgo de volverse demasiado
abstracto. Cloud se aprende mejor cuando el concepto puede aterrizarse después
en decisiones reales.

### Definir responsabilidades primero

El capítulo empieza por frontera de responsabilidad y después conecta esos
modelos con ejemplos de AWS y GCP. Esta alternativa separa el fundamento
duradero del aterrizaje específico por proveedor.

## Justificación

Se elige definir responsabilidades primero porque sigue la decisión de
RFC-0001 §10: concepto primero, proveedor después. El capítulo debe poder
seguirse aunque cambien nombres comerciales, consolas o precios. Los
proveedores aparecen como ejemplos, no como fuente de verdad.

## Invariantes del capítulo

- Todo modelo de servicio reparte responsabilidad entre proveedor y equipo.
- Delegar operación reduce ciertas cargas, pero también reduce cierto control.
- Mayor abstracción no significa automáticamente menor costo.
- Menor abstracción no significa automáticamente mejor arquitectura.
- La decisión depende de carga operativa, portabilidad, requisitos de seguridad,
  madurez del equipo, presupuesto y ritmo de cambio.
- Ningún ejemplo de proveedor debe presentarse como recomendación universal.
- Los costos y límites dependientes del proveedor deben fecharse o tratarse
  como ejemplos revisables.

## Requisitos funcionales del modelo Rust

El módulo `src/service_models.rs` representa, en una primera versión:

- modelos de servicio;
- capas de responsabilidad;
- reparto de responsabilidad entre proveedor y equipo;
- comparación educativa de control, carga operativa y portabilidad;
- errores explícitos cuando una decisión no declara sus supuestos mínimos.

## Requisitos no funcionales

- Sin dependencias externas.
- Sin `unsafe`.
- API pública documentada con doctests.
- Pruebas para las reglas pequeñas del reparto de responsabilidad.
- Código pequeño: el objetivo es enseñar el contrato, no simular un proveedor.

## Comparación educativa

| Modelo | Qué conserva el equipo | Qué delega más claramente | Tradeoff principal |
|--------|-------------------------|---------------------------|--------------------|
| IaaS | Sistema operativo, runtime, aplicación, datos y escalado | Instalaciones físicas, red física, cómputo físico y virtualización | Mayor control, mayor carga operativa |
| PaaS | Código y datos de aplicación | Sistema operativo, runtime y parte de la plataforma | Menos operación, menos control de plataforma |
| Serverless | Código orientado a eventos y parte de los datos | Runtime, capacidad y escalado | Menos operación, más dependencia del modelo de ejecución |
| SaaS | Uso, datos y reglas de negocio alrededor del producto | Aplicación, runtime, plataforma e infraestructura | Menor carga técnica, menor control de implementación |

Esta tabla no recomienda un modelo universal. Solo organiza la conversación.
La decisión real depende de seguridad, presupuesto, madurez del equipo,
portabilidad, disponibilidad, latencia y ritmo de cambio.

## Cómo leer el módulo Rust

El módulo `service_models` representa cada modelo como un perfil:

```rust
use rust_cloud::service_models::{ResponsibilityLayer, ServiceModel};

let profile = ServiceModel::Iaas.profile();
let owner = profile.owner_of(ResponsibilityLayer::OperatingSystem);
```

La API fuerza una lectura explícita: primero eliges el modelo, luego preguntas
quién opera una capa. Eso evita enseñar Cloud como un menú de productos y lo
convierte en un reparto de responsabilidades.

También existe una recomendación pequeña:

```rust
use rust_cloud::service_models::{DecisionContext, recommend_model};

let context = DecisionContext {
    wants_low_operational_load: true,
    event_driven_workload: true,
    ..DecisionContext::default()
};

let model = recommend_model(context);
```

La recomendación devuelve error si no hay supuestos mínimos. Eso es deliberado:
una decisión cloud sin contexto no es una decisión de ingeniería, es una
preferencia sin argumento.

## Diagrama

El diagrama del capítulo vive en
`diagrams/01-modelos-de-servicio.mmd`. Resume la frontera de responsabilidad
como un continuo de abstracción:

```text
IaaS -> PaaS -> Serverless -> SaaS
control alto                         carga operativa baja
```

## Ejemplo ejecutable

El ejemplo `examples/service_models.rs` imprime una comparación breve y una
recomendación educativa:

```bash
cargo run --example service_models
```

El ejemplo no contacta ningún proveedor, no calcula precios y no promete
arquitecturas de producción. Solo muestra cómo convertir una conversación de
Cloud en datos verificables.

## Decisiones registradas en el modelo Rust

- Las capas del modelo son: instalaciones físicas, red física, cómputo físico,
  virtualización, sistema operativo, runtime, código de aplicación, datos de
  aplicación y política de escalado.
- `Serverless` se modela como modelo separado. Aunque comparte delegación de
  plataforma con PaaS, su frontera educativa cambia por eventos, capacidad y
  escalado delegados.
- Control, carga operativa y portabilidad se representan como puntajes
  educativos (`Low`, `Medium`, `High`), no como métricas reales.
- Una recomendación sin supuestos mínimos devuelve error explícito.

## Práctica sugerida

Antes de elegir un servicio concreto, escribe cuatro columnas:

1. Responsabilidad que quiero delegar.
2. Responsabilidad que debo conservar.
3. Supuesto que justifica la decisión.
4. Costo o riesgo que acepto al tomarla.

Después compara el resultado contra el módulo Rust. Si no puedes llenar la
columna de supuestos, todavía no estás listo para escoger un modelo de servicio.

Los ejercicios graduados viven en `docs/ejercicios/01-modelos-de-servicio.md`.
El análisis de costos del capítulo vive en `docs/costos/01-modelos-de-servicio.md`.

## Estado editorial

Este capítulo está en `implemented`. No está marcado como `reviewed` ni
`published`.
