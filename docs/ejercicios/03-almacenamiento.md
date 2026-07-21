# Ejercicios: almacenamiento

- **Curso:** rust-cloud
- **Capítulo:** 03. Almacenamiento
- **Estado:** implemented
- **Issue:** #12

Estos ejercicios practican almacenamiento como promesa del dato. La meta no es
adivinar servicios de proveedor, sino declarar tamaño, fuente de verdad, patrón
de acceso, ciclo de vida y costo aceptado.

## Ejercicio 1: clasificar un dato por clave `[Nivel 1]`

Un sitio de academia publica assets estáticos. El conjunto pesa `10` GiB, es
fuente de verdad para publicación y se accede por clave.

Construye `DataRequirements` y usa `recommend_storage`.

**Entrada/Salida esperada:** la recomendación debe ser `StorageMode::Object`.

<details>
<summary>Pista</summary>
Un dato accedido por clave no necesita montarse como volumen. Empieza por
declarar tamaño y fuente de verdad.
</details>

Solución compilable: `examples/soluciones/storage_nivel_1.rs`.

## Ejercicio 2: comparar datos de una academia `[Nivel 2]`

Modela tres datos:

- assets públicos del sitio;
- scratch temporal de generación de assets;
- retención histórica de entregables.

Usa `recommend_storage` para obtener una recomendación educativa para cada uno.

**Entrada/Salida esperada:** `Object`, `Ephemeral` y `Archive`.

<details>
<summary>Pista</summary>
La diferencia no está en el nombre del archivo. Está en si el dato es fuente de
verdad, temporal o histórico.
</details>

Solución compilable: `examples/soluciones/storage_nivel_2.rs`.

## Ejercicio 3: detectar una promesa contradictoria `[Nivel 3]`

Construye un dato que sea fuente de verdad y temporal al mismo tiempo. Ejecuta
`recommend_storage` y explica por qué el modelo devuelve error.

**Entrada/Salida esperada:** `StorageDecisionError::ConflictingRequirements`.

<details>
<summary>Pista</summary>
Un dato puede ser derivable o permanente, pero no debe prometer ambas cosas al
mismo tiempo sin una frontera nueva.
</details>

Solución compilable: `examples/soluciones/storage_nivel_3.rs`.

## Ejercicio 4: diseñar almacenamiento para una academia `[Nivel 4]`

Diseña la estrategia inicial de almacenamiento para:

- imágenes del sitio;
- progreso de estudiantes;
- entregables de ejercicios;
- archivos temporales de generación visual;
- respaldos históricos del curso.

Para cada dato, decide si usarías objeto, bloque, archivo, efímero, archivo
histórico u otra frontera futura. Justifica fuente de verdad, patrón de acceso,
consistencia, ciclo de vida y costo aceptado.

**Entrada/Salida esperada:** una propuesta breve con seis columnas: dato,
forma, fuente de verdad, patrón de acceso, ciclo de vida y tradeoff.

<details>
<summary>Pista</summary>
No mezcles datos por comodidad. El progreso de estudiantes, un asset público y
un archivo temporal de render tienen promesas distintas.
</details>

Discusión sugerida:

- Los assets públicos suelen encajar con objeto: clave, durabilidad y lectura
  amplia.
- Los temporales de generación visual pueden ser efímeros si se pueden
  reconstruir.
- El progreso de estudiantes probablemente no se resuelve solo con storage; se
  conecta con bases de datos, consistencia y control de identidad en capítulos
  posteriores.
- Los respaldos históricos deben explicitar recuperación lenta, retención y
  costo de restauración.
