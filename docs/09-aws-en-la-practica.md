# AWS en la práctica

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 09. AWS en la práctica
- **Issues:** #33, #34, #35, #36
- **Módulo Rust:** `src/aws_practice.rs`
- **Diagrama:** `diagrams/09-aws-en-la-practica.mmd`
- **Ejemplo:** `examples/aws_practice.rs`
- **Ejercicios:** `docs/ejercicios/09-aws-en-la-practica.md`
- **Costos:** `docs/costos/09-aws-en-la-practica.md`

## Concepto

AWS en la práctica es el aterrizaje de los fundamentos del curso en una
plataforma concreta. No es un catálogo de servicios ni una guía de consola. Es
un ejercicio de traducción: tomar una necesidad de producto y expresarla como
decisiones AWS explícitas sobre cómputo, almacenamiento, red, identidad,
operación, costo y frontera de responsabilidad.

La pregunta central no es "¿qué servicio de AWS uso?", sino "¿qué contrato del
sistema necesito y qué servicio de AWS lo representa con menos deuda para este
caso?".

## Imagen mental

Piensa en AWS como una ciudad con infraestructura compartida.

- **VPC** define calles, zonas y fronteras de tráfico.
- **IAM** define credenciales, permisos y llaves de acceso.
- **EC2, ECS, Lambda y servicios administrados** son formas distintas de operar
  trabajo.
- **S3, EBS, EFS y bases administradas** son formas distintas de guardar datos.
- **CloudWatch, CloudTrail y métricas** permiten investigar qué ocurrió.
- **Budgets, tags y Cost Explorer** ayudan a explicar gasto y responsabilidad.

La ciudad ofrece muchas herramientas, pero la ingeniería sigue siendo elegir
rutas, límites, dueños y señales que sostengan un sistema real.

## Problema

AWS es amplio y cambia con frecuencia. Enseñarlo como lista de productos
produce estudiantes que reconocen nombres, pero no necesariamente entienden la
decisión detrás de esos nombres. Además, las demos rápidas pueden ocultar temas
centrales: permisos excesivos, redes públicas por comodidad, recursos sin tags,
costos invisibles, datos sin ciclo de vida, dependencia innecesaria de servicios
específicos y operación sin señales.

La enseñanza de AWS suele caer en cuatro errores:

- empezar por consola y no por contrato de arquitectura;
- presentar servicios como equivalentes sin hablar de responsabilidad operativa;
- usar credenciales amplias para avanzar rápido;
- ignorar costo, tags, regiones, límites y observabilidad hasta el final.

Sin una base clara, el estudiante puede creer que AWS se aprende memorizando
nombres de servicios, copiando diagramas de referencia o desplegando recursos
sin explicar sus invariantes.

## Alternativas consideradas

1. **Curso guiado por consola.** Es vistoso y rápido, pero envejece pronto y
   puede enseñar clics antes que criterios.
2. **Curso guiado por certificación.** Ordena vocabulario, pero prioriza
   cobertura de examen sobre diseño de sistemas.
3. **Curso guiado por patrones transferibles.** Parte de cargas, datos, red,
   identidad, observabilidad y costo; después nombra servicios AWS como
   implementaciones posibles.

## Justificación

Este capítulo adopta la tercera alternativa. AWS aparece como proveedor
concreto, pero no reemplaza el canon del curso. Cada servicio se estudia como
representación de una decisión ya trabajada: modelo de servicio, cómputo,
almacenamiento, VPC, IAM, managed services, serverless y FinOps.

La decisión conserva RFC-0001 §2: concepto antes de implementación, problema
antes de herramienta, alternativas antes de elección. También mantiene
RFC-0001 §10: proveedor después de fundamentos.

## Invariantes del capítulo

- AWS se enseña como traducción de fundamentos, no como catálogo.
- Todo recurso debe declarar región, ambiente, dueño y propósito.
- IAM debe preferir permisos mínimos y credenciales temporales cuando aplique.
- Una VPC debe distinguir tráfico público, privado y dependencias externas.
- La elección de cómputo debe explicar operación delegada, escalado y límites.
- Los datos deben declarar durabilidad, recuperación, ciclo de vida y frontera
  de acceso.
- Serverless debe declarar evento, timeout, concurrencia, retry e idempotencia.
- Los servicios administrados no eliminan responsabilidad: cambian qué se
  delega y qué sigue siendo del equipo.
- Toda decisión AWS debe tener señal operativa: logs, métricas, auditoría o
  trazas según el caso.
- El costo debe ser atribuible mediante tags, unidad económica y presupuesto.
- Los nombres, precios, cuotas y límites de AWS son material vivo y deben
  revisarse cuando se usen ejemplos fechados.

## Comparación educativa inicial

| Necesidad | Pregunta de ingeniería | Ejemplo AWS posible | Riesgo común |
|-----------|------------------------|---------------------|--------------|
| Ejecutar API estable | ¿Quién opera runtime, hosts y escalado? | EC2, ECS, App Runner o Lambda | Elegir por moda, no por contrato |
| Publicar assets | ¿Qué durabilidad y acceso necesitan los objetos? | S3 y CloudFront | Hacer público lo que debía ser privado |
| Separar red | ¿Qué debe vivir público y qué debe vivir privado? | VPC, subnets, route tables | Exponer bases o SSH por comodidad |
| Acceso humano | ¿Qué permisos y duración son aceptables? | IAM Identity Center, roles | Usuarios permanentes con permisos amplios |
| Procesar eventos | ¿Qué pasa si se repite el evento? | Lambda, SQS, EventBridge | Retry sin idempotencia |
| Observar operación | ¿Qué señal permite investigar incidentes? | CloudWatch, CloudTrail | Logs sin retención ni correlación |
| Controlar costo | ¿Quién entiende el gasto y contra qué unidad? | Tags, Budgets, Cost Explorer | Factura agregada sin dueño |

## Decisiones registradas

- El capítulo no empieza por comandos de AWS CLI ni por consola.
- AWS se modela como una traducción de fundamentos a servicios concretos.
- Los servicios se nombran solo cuando ayudan a explicar una decisión.
- El modelo Rust mínimo representa una carga AWS educativa sin depender de SDKs.
- Las recomendaciones dependientes de precios, cuotas o límites concretos se
  consideran material vivo y no se publican sin fecha y revisión humana.

## Modelo Rust mínimo

El módulo Rust mínimo vive en `src/aws_practice.rs` y modela, sin dependencias
externas:

- conceptos cloud del curso que se quieren aterrizar;
- servicios AWS candidatos para cada concepto;
- región, ambiente, dueño y propósito;
- permisos mínimos, credenciales temporales y exposición de red;
- límites, observabilidad, tags de costo y ciclo de vida;
- hallazgos cuando un servicio no corresponde al concepto, faltan señales o el
  ejemplo intenta usar credenciales reales.

El módulo no usa SDKs ni contacta AWS. Su función es pedagógica: practicar la
traducción de concepto a servicio antes de tocar una cuenta real.

## Lectura del modelo

`AwsWorkload` junta cinco preguntas que deben aparecer antes de abrir una
consola:

1. **Qué necesidad existe:** el nombre y propósito humano del workload.
2. **Qué concepto representa:** cómputo, storage, red, identidad, managed
   service, serverless, observabilidad o FinOps.
3. **Qué servicio candidato lo aterriza:** S3, VPC, Lambda, SQS, RDS, Budgets u
   otro servicio del mapa.
4. **Qué responsabilidad conserva el equipo:** permisos, red, límites,
   observabilidad, tags y ciclo de vida.
5. **Qué no debe entrar al ejemplo:** credenciales reales, permisos amplios o
   exposición pública sin frontera.

El punto pedagógico es deliberado: AWS aparece como proveedor real, pero el
modelo obliga a hablar primero de contrato, responsabilidad y señales.

## Cómo leer el módulo Rust

Un mapeo sano declara concepto, servicio, dueño, región, límites y señales:

```rust
use rust_cloud::aws_practice::{
    AwsEnvironment, AwsPracticeRequirements, AwsService, AwsWorkload,
    CloudConcept, DataLifecycle, NetworkExposure,
};

let workload = AwsWorkload::new(
    "academy-assets",
    AwsPracticeRequirements {
        concept: CloudConcept::Storage,
        service: AwsService::S3,
        environment: AwsEnvironment::Production,
        region: "us-east-1",
        owner: "equipo academy",
        purpose: "guardar assets publicados del curso",
        least_privilege: true,
        temporary_credentials: true,
        network_exposure: NetworkExposure::Private,
        has_limit: true,
        observability: true,
        cost_tags: true,
        data_lifecycle: DataLifecycle::Retained,
        uses_real_credentials: false,
    },
)
.unwrap();

assert!(workload.evaluate().is_low_risk());
```

Un ejemplo riesgoso puede usar un servicio que no representa el concepto,
credenciales permanentes, permisos amplios, red pública sin frontera y falta de
tags:

```rust
use rust_cloud::aws_practice::{
    AwsEnvironment, AwsPracticeFinding, AwsPracticeRequirements, AwsService,
    AwsWorkload, CloudConcept, DataLifecycle, NetworkExposure,
};

let workload = AwsWorkload::new(
    "preview-runner",
    AwsPracticeRequirements {
        concept: CloudConcept::Serverless,
        service: AwsService::Ec2,
        environment: AwsEnvironment::Development,
        region: "us-east-1",
        owner: "",
        purpose: "ejecutar previews",
        least_privilege: false,
        temporary_credentials: false,
        network_exposure: NetworkExposure::PublicUnbounded,
        has_limit: false,
        observability: false,
        cost_tags: false,
        data_lifecycle: DataLifecycle::Indefinite,
        uses_real_credentials: true,
    },
)
.unwrap();

assert!(workload.evaluate().findings().contains(
    &AwsPracticeFinding::RealCredentialsInExample("preview-runner"),
));
```

## Diagrama

El diagrama del capítulo vive en `diagrams/09-aws-en-la-practica.mmd`. Resume
la lectura principal:

```text
necesidad -> concepto -> servicio AWS -> responsabilidad -> permisos/red -> señales -> decisión practicable
```

La ruta sana obliga a justificar el servicio con un concepto del curso. La ruta
riesgosa aparece cuando el servicio se elige por familiaridad, cuando el ejemplo
usa credenciales reales o cuando faltan límites, observabilidad y costo
atribuible.

## Ejemplo ejecutable

El ejemplo `examples/aws_practice.rs` compara un mapeo S3 gobernable contra un
runner de previews que mezcla concepto, permisos, red, credenciales y costo sin
suficientes límites.

```bash
cargo run --example aws_practice
```

Salida esperada:

```text
academy-assets: mapeo AWS gobernable
preview-runner: 10 hallazgos educativos
```

El ejemplo no usa AWS CLI, SDKs ni credenciales reales. Sirve para practicar qué
preguntas deben quedar respondidas antes de tocar infraestructura real.

## Ejemplos progresivos

El capítulo usa tres niveles de lectura:

| Nivel | Escenario | Señal principal | Aprendizaje |
|-------|-----------|-----------------|-------------|
| Básico | Assets privados en S3 | Mapeo concepto-servicio | Storage no es solo un bucket; requiere dueño, permisos y ciclo de vida |
| Intermedio | Runner de previews mal modelado | Credenciales y red inseguras | La práctica en AWS no debe normalizar atajos peligrosos |
| Avanzado | Mapa de servicios por capacidad | Responsabilidad retenida | Cada servicio debe explicar qué delega y qué conserva el equipo |

## Ejercicios y costos

Los ejercicios viven en `docs/ejercicios/09-aws-en-la-practica.md` y tienen
soluciones compilables en `examples/soluciones/aws_practice_nivel_*.rs`.

El análisis de costos vive en `docs/costos/09-aws-en-la-practica.md`. No usa
precios de proveedor: compara exposición pública, credenciales, límites, tags,
ciclo de vida, observabilidad y responsabilidad retenida.

## Nota editorial

Este capítulo queda en `implemented`. No está marcado como `reviewed` ni
`published`.
