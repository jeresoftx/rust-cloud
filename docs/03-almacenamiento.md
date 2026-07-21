# Almacenamiento

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 03. Almacenamiento
- **Issues:** #9, #10, #11
- **Módulo Rust:** `src/storage.rs`
- **Diagrama:** `diagrams/03-almacenamiento.mmd`
- **Ejemplo:** `examples/storage.rs`
- **Ejercicios:** `docs/ejercicios/03-almacenamiento.md`
- **Costos:** `docs/costos/03-almacenamiento.md`

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

## Imagen mental

Piensa en una biblioteca técnica.

- Algunos documentos se piden por clave: "dame este manual". Eso se parece a
  almacenamiento de **objetos**.
- Algunos cuadernos están sobre la mesa de trabajo de una sola persona. Eso se
  parece a almacenamiento de **bloque**.
- Algunos archivos están en un archivero compartido por varios equipos. Eso se
  parece a almacenamiento de **archivo**.
- Algunas notas se escriben en una pizarra y se borran al terminar. Eso se
  parece a almacenamiento **efímero**.
- Algunos expedientes se guardan en bodega y tardan más en recuperarse. Eso se
  parece a **archivo histórico**.

La metáfora sirve para recordar que no todo dato merece la misma promesa. Hay
datos que se consumen, datos que se montan, datos que se comparten, datos que se
derivan y datos que se conservan por años.

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

## Comparación educativa

| Forma | Cuándo encaja | Promesa principal | Riesgo si se elige por costumbre |
|-------|---------------|-------------------|----------------------------------|
| Objeto | Assets, respaldos, contenido estático, datasets y archivos por clave | Durabilidad y acceso por clave | Usarlo como disco local o depender de operaciones que no son de objeto |
| Bloque | Volúmenes para máquinas, bases de datos o workloads que necesitan montaje | Disco persistente cercano a una carga | Acoplar dato a una máquina o zona sin revisar recuperación |
| Archivo | Rutas compartidas entre varias cargas | Vista común con semántica de archivos | Convertir coordinación de archivos en cuello de botella |
| Efímero | Scratch, cache derivable, temporales de build o procesamiento | Velocidad local sin promesa de permanencia | Tratarlo como fuente de verdad y perder datos |
| Archivo histórico | Retención, auditoría, respaldo frío o recuperación rara | Conservación con lectura infrecuente | Olvidar tiempos/costos de recuperación cuando ocurre un incidente |

Esta tabla no recomienda marcas. Ordena la pregunta: qué promesa debe cumplir
el dato y qué costo aparece si la promesa se rompe.

## Cómo leer el módulo Rust

El módulo `storage` empieza por un dato y sus supuestos:

```rust
use rust_cloud::storage::{AccessFrequency, DataRequirements};

let assets = DataRequirements {
    size_gib: Some(10),
    source_of_truth: true,
    accessed_by_key: true,
    read_frequency: AccessFrequency::Warm,
    ..DataRequirements::default()
};
```

Después se pide una recomendación educativa:

```rust
use rust_cloud::storage::{StorageMode, recommend_storage};

# let assets = rust_cloud::storage::DataRequirements {
#     size_gib: Some(10),
#     source_of_truth: true,
#     accessed_by_key: true,
#     read_frequency: rust_cloud::storage::AccessFrequency::Warm,
#     ..rust_cloud::storage::DataRequirements::default()
# };
let mode = recommend_storage(assets);
assert_eq!(mode, Ok(StorageMode::Object));
```

La API obliga a declarar tamaño y fuente de verdad. Si el dato no dice si es
permanente o temporal, el modelo no recomienda. Esa negativa es parte de la
enseñanza: antes de almacenar algo, hay que saber si perderlo rompe el sistema.

## Diagrama

El diagrama del capítulo vive en `diagrams/03-almacenamiento.mmd`. Resume la
lectura principal:

```text
dato -> promesa -> patrón de acceso -> forma de almacenamiento
```

## Ejemplo ejecutable

El ejemplo `examples/storage.rs` compara perfiles y recomienda formas para
datos comunes de una academia:

```bash
cargo run --example storage
```

El ejemplo no escribe archivos ni contacta proveedores. Su intención es mostrar
que cada dato necesita declarar su promesa antes de escoger almacenamiento.

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

## Práctica sugerida

Antes de elegir almacenamiento, escribe:

1. Fuente de verdad: permanente, temporal o derivable.
2. Tamaño aproximado y crecimiento esperado.
3. Patrón de acceso: clave, montaje, archivos compartidos, scratch o histórico.
4. Consistencia necesaria: fuerte, eventual o acotada por frontera.
5. Ciclo de vida: activo, retenido, temporal o archivado.
6. Costo aceptado: latencia, recuperación, operación, retención o migración.

Si una decisión no declara fuente de verdad, todavía no es una decisión de
almacenamiento; es solo un lugar donde se puso un dato.

Los ejercicios graduados viven en `docs/ejercicios/03-almacenamiento.md`. El
análisis de costos del capítulo vive en `docs/costos/03-almacenamiento.md`.

## Estado editorial

Este capítulo está en `implemented`. No está marcado como `reviewed` ni
`published`.
