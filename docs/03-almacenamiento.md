# Almacenamiento

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 03. Almacenamiento
- **Issues:** #9, #10
- **Módulo Rust:** `src/storage.rs`

## Concepto

Almacenamiento es la decisión de dónde vive un dato, cómo se accede a él, qué
durabilidad necesita, qué consistencia acepta y cuánto cuesta conservarlo. En
cloud, almacenar no significa solo "guardar archivos"; significa elegir una
frontera entre forma de acceso, ciclo de vida, recuperación, latencia,
replicación, seguridad y costo.

La pregunta central del capítulo no es "¿qué servicio de storage uso?", sino
"¿qué promesa debe cumplir este dato y qué tradeoff acepto para cumplirla?".

Para este curso, las formas canónicas de almacenamiento son:

- **Objeto:** datos direccionados por clave, útiles para archivos, assets,
  respaldos, datasets y contenido estático.
- **Bloque:** volúmenes montados por una máquina o workload, útiles cuando el
  sistema operativo o una base de datos necesita disco persistente.
- **Archivo:** sistema compartido con jerarquía y semántica de archivos, útil
  cuando varias cargas necesitan una vista común.
- **Efímero:** almacenamiento temporal ligado a una instancia, contenedor,
  función o job; rápido, pero no fuente de verdad.
- **Archivo frío o histórico:** datos con baja frecuencia de lectura y mayor
  tolerancia a recuperación lenta.

Estos nombres son modelos de acceso y responsabilidad, no marcas.

## Problema

El almacenamiento cloud suele enseñarse como una lista de productos: buckets,
volúmenes, discos, snapshots, archivos compartidos o clases de almacenamiento.
Ese enfoque ayuda a empezar, pero esconde la decisión importante: qué dato se
puede perder, qué dato debe sobrevivir, qué tan rápido debe leerse, qué tan
consistente debe ser y qué costo aparece por conservarlo o moverlo.

Sin esta base, un estudiante puede usar almacenamiento de objetos como si fuera
un disco local, montar volúmenes cuando bastaba contenido estático, o tratar un
cache efímero como fuente de verdad. En los tres casos el error no es de marca:
es de modelo mental.

## Alternativas

### Empezar por buckets y discos concretos

Es práctico y aterriza rápido, pero ata el capítulo a nombres de proveedor y
puede convertir una decisión conceptual en tutorial de consola.

### Empezar por bases de datos

Las bases de datos son familiares, pero pertenecen a un capítulo distinto del
camino. Aquí la pregunta previa es la forma física o lógica donde vive el dato,
no todavía el modelo relacional, documental o distribuido.

### Definir forma de acceso y promesa del dato primero

El capítulo empieza por cómo se accede al dato, qué durabilidad y consistencia
requiere, y qué ciclo de vida tendrá. Después se conectará con ejemplos de
proveedor sin convertirlos en fuente de verdad.

## Justificación

Se elige definir forma de acceso y promesa del dato primero porque conserva
RFC-0001 §2: concepto, problema, alternativas, justificación y código. También
sigue RFC-0001 §10: los conceptos cloud se enseñan antes que AWS o GCP. Los
servicios concretos serán ejemplos revisables; la fuente de verdad del capítulo
serán las responsabilidades del dato.

## Invariantes del capítulo

- Todo dato necesita una fuente de verdad o debe declararse explícitamente como
  efímero.
- Toda decisión de almacenamiento implica forma de acceso: objeto, bloque,
  archivo, temporal o histórico.
- Durabilidad y disponibilidad no son lo mismo.
- Consistencia fuerte, eventual o limitada cambia cómo se diseña la aplicación.
- Menor latencia suele aumentar costo, acoplamiento u operación.
- Replicar datos mejora ciertas fallas, pero puede complicar consistencia,
  privacidad y costo.
- Borrar, retener y recuperar datos son decisiones de diseño, no tareas
  administrativas posteriores.
- Los límites y precios de proveedor deben fecharse o tratarse como ejemplos
  revisables.

## Requisitos funcionales del modelo Rust

El módulo `src/storage.rs` representa, en una primera versión:

- formas de almacenamiento;
- patrones de acceso;
- durabilidad y consistencia educativas;
- ciclo de vida del dato;
- señales de costo como frecuencia de lectura, retención y recuperación;
- errores explícitos cuando un dato no declara su fuente de verdad o sus
  requisitos mínimos.

## Requisitos no funcionales

- Sin dependencias externas.
- Sin `unsafe`.
- API pública documentada con doctests.
- Pruebas para clasificación de almacenamiento, requisitos mínimos y errores de
  decisión.
- Código pequeño: el objetivo es razonar sobre promesas de datos, no simular un
  sistema de almacenamiento real.

## Decisiones registradas en el modelo Rust

- Las formas del modelo Rust son: objeto, bloque, archivo, efímero y archivo
  histórico.
- La consistencia se representa como enum educativo (`Strong`, `Eventual`,
  `Bounded`) para explicar fronteras sin prometer semánticas de proveedor.
- Ciclo de vida, frecuencia de lectura y costo de recuperación se separan para
  que el modelo pueda distinguir datos activos, retenidos, temporales e
  históricos.
- Una recomendación sin tamaño o sin declaración de fuente de verdad devuelve
  error explícito.
- Un dato no puede ser fuente de verdad y temporal al mismo tiempo.

## Estado editorial

Este capítulo está en `implemented`. No está marcado como `reviewed` ni
`published`.
