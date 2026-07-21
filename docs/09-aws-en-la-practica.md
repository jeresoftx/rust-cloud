# AWS en la práctica

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 09. AWS en la práctica
- **Issues:** #33, #34
- **Módulo Rust:** `src/aws_practice.rs`

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
- El modelo Rust mínimo deberá representar una carga AWS educativa sin
  depender de SDKs.
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

## Nota editorial

Este capítulo queda en `implemented`. No está marcado como `reviewed` ni
`published`.
