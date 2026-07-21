# Ejercicios: serverless

- **Curso:** rust-cloud
- **Capítulo:** 07. Serverless
- **Estado:** implemented
- **Issue:** #28

Estos ejercicios practican serverless como contrato de ejecución por eventos.
La meta no es subir una función a un proveedor, sino declarar trigger, timeout,
concurrencia, retry, idempotencia, estado, observabilidad y costo.

## Ejercicio 1: handler de cola acotado `[Nivel 1]`

Construye un `ServerlessWorkload` para procesar eventos de publicación:

- `TriggerKind::Queue`;
- `RuntimeProfile::Function`;
- timeout de 30 segundos;
- concurrencia máxima de 50;
- retry con backoff;
- idempotencia por clave de evento;
- escritura en estado externo;
- observabilidad estándar.

**Entrada/Salida esperada:** la evaluación debe ser de bajo riesgo educativo.

<details>
<summary>Pista</summary>
El retry no es el problema por sí solo. El riesgo aparece cuando puede repetir
efectos secundarios sin idempotencia.
</details>

Solución compilable: `examples/soluciones/serverless_nivel_1.rs`.

## Ejercicio 2: detectar función riesgosa `[Nivel 2]`

Modela una función HTTP de pago que:

- tarda hasta 120 segundos;
- no declara concurrencia máxima;
- reintenta dos veces;
- no tiene idempotencia;
- escribe estado externo;
- no declara observabilidad.

**Entrada/Salida esperada:** la evaluación debe reportar retry sin
idempotencia, escritura de estado sin idempotencia, concurrencia sin límite,
timeout alto y observabilidad ausente.

<details>
<summary>Pista</summary>
Una función pequeña puede tener un efecto enorme si escribe estado y se repite.
</details>

Solución compilable: `examples/soluciones/serverless_nivel_2.rs`.

## Ejercicio 3: comparar scheduler y workflow `[Nivel 3]`

Construye dos workloads:

- un scheduler de limpieza sin estado, con concurrencia baja y sin retry;
- un workflow de publicación con retry, compensación y observabilidad estándar.

Compara sus evaluaciones.

**Entrada/Salida esperada:** ambos deben quedar sin hallazgos educativos si
declaran timeout, concurrencia, propósito y observabilidad completa.

<details>
<summary>Pista</summary>
Serverless no es solo funciones HTTP. Schedulers y workflows también deben
tener límites y señales observables.
</details>

Solución compilable: `examples/soluciones/serverless_nivel_3.rs`.

## Ejercicio 4: diseñar matriz serverless `[Nivel 4]`

Escribe una matriz para una plataforma educativa con:

- webhook de inscripción;
- procesador de publicaciones;
- generación programada de reportes;
- workflow de publicación;
- función de limpieza;
- evento de almacenamiento para assets.

Para cada flujo, declara evento, runtime, timeout, concurrencia, retry,
idempotencia, estado, observabilidad y costo/riesgo.

**Entrada/Salida esperada:** una tabla con nueve columnas: flujo, evento,
runtime, timeout, concurrencia, retry, idempotencia, estado y costo/riesgo.

<details>
<summary>Pista</summary>
El mejor diseño no es "todo serverless". Es saber qué flujos se benefician de
eventos elásticos y cuáles necesitan capacidad estable, latencia predecible o
menor acoplamiento.
</details>

Discusión sugerida:

- Los handlers de cola deben ser idempotentes.
- Las funciones HTTP de baja latencia deben cuidar cold start y timeout.
- Los schedulers deben declarar qué pasa si una ejecución se solapa.
- Los workflows ayudan a orquestar, pero pueden acoplarse fuerte al proveedor.
- La observabilidad debe permitir seguir un evento, no solo ver logs sueltos.
