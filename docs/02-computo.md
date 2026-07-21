# Cómputo

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 02. Cómputo
- **Issues:** #5, #6
- **Módulo Rust:** `src/compute.rs`

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

## Estado editorial

Este capítulo está en `implemented`. No está marcado como `reviewed` ni
`published`.
