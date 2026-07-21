# Modelos de servicio

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 01. Modelos de servicio
- **Issues:** #1, #2
- **Módulo Rust:** `src/service_models.rs`

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

## Estado editorial

Este capítulo está en `implemented`. No está marcado como `reviewed` ni
`published`.
