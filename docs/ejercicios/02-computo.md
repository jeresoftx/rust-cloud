# Ejercicios: cómputo

- **Curso:** rust-cloud
- **Capítulo:** 02. Cómputo
- **Estado:** implemented
- **Issue:** #8

Estos ejercicios practican la decisión de cómputo desde restricciones de carga:
recursos, ciclo de vida, aislamiento, escalado y operación. No buscan adivinar
productos de proveedor.

## Ejercicio 1: validar recursos mínimos `[Nivel 1]`

Una carga declara `2` vCPU y `2048` MiB de memoria. Construye un
`ResourceRequest` y verifica que cabe dentro de un límite educativo de `4` vCPU
y `4096` MiB.

Después prueba un límite más pequeño de `1` vCPU y explica por qué ya no cabe.

**Entrada/Salida esperada:** una validación verdadera y una validación falsa
con `fits_within`.

<details>
<summary>Pista</summary>
No empieces por el modo de cómputo. Primero demuestra que la carga declaró
recursos mínimos y que esos recursos caben en una frontera concreta.
</details>

Solución compilable: `examples/soluciones/compute_nivel_1.rs`.

## Ejercicio 2: recomendar por forma de carga `[Nivel 2]`

Modela tres cargas pequeñas:

- una API empacada como contenedor y con deseo de baja operación;
- un evento pequeño con baja operación;
- un proceso batch nocturno.

Usa `recommend_compute` para obtener una recomendación educativa para cada
carga.

**Entrada/Salida esperada:** `ManagedContainer`, `Function` y `BatchJob`.

<details>
<summary>Pista</summary>
La recomendación usa supuestos, no nombres de proveedor. Cambia booleanos en
`WorkloadRequirements` hasta que la forma de la carga sea explícita.
</details>

Solución compilable: `examples/soluciones/compute_nivel_2.rs`.

## Ejercicio 3: detectar supuestos contradictorios `[Nivel 3]`

Construye una carga que exige controlar el sistema operativo y al mismo tiempo
quiere baja carga operativa. Ejecuta `recommend_compute` y explica por qué el
modelo devuelve un error.

**Entrada/Salida esperada:** `ComputeDecisionError::ConflictingRequirements`.

<details>
<summary>Pista</summary>
El modelo no debe esconder tradeoffs. Si los supuestos chocan, debe obligarte a
decidir cuál pesa más.
</details>

Solución compilable: `examples/soluciones/compute_nivel_3.rs`.

## Ejercicio 4: diseñar capacidad para una academia `[Nivel 4]`

Diseña la forma de cómputo inicial para una academia con:

- sitio público;
- autenticación;
- progreso de estudiantes;
- generación de assets;
- laboratorios ejecutables futuros.

Propón qué partes usarían contenedor administrado, función, batch, máquina
virtual u otro modo. Justifica recursos mínimos, ciclo de vida, señal de
escalado y tradeoff operativo.

**Entrada/Salida esperada:** una propuesta breve con cinco columnas:
componente, modo de cómputo, recursos mínimos, señal de escalado y tradeoff.

<details>
<summary>Pista</summary>
No todos los componentes necesitan la misma forma de cómputo. Publicar páginas,
procesar assets y ejecutar código de estudiantes son cargas con riesgos muy
distintos.
</details>

Discusión sugerida:

- El sitio público puede empezar con cómputo muy delegado: hosting estático o
  contenedor administrado según el despliegue del ecosistema.
- La generación de assets puede ser batch o cola, porque consume recursos por
  ventanas y no necesita estar siempre encendida.
- Los laboratorios ejecutables futuros merecen una frontera separada: seguridad,
  límites, aislamiento y costos cambian de manera importante.
