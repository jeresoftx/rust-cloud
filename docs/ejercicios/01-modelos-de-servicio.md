# Ejercicios: modelos de servicio

- **Curso:** rust-cloud
- **Capítulo:** 01. Modelos de servicio
- **Estado:** implemented
- **Issue:** #4

Estos ejercicios convierten el capítulo en práctica. El objetivo no es adivinar
marcas ni memorizar servicios, sino razonar qué responsabilidad queda en el
equipo y qué responsabilidad se delega.

## Ejercicio 1: ubicar responsabilidades `[Nivel 1]`

Una aplicación corre en IaaS. Identifica quién opera estas capas:

- instalaciones físicas;
- sistema operativo;
- runtime;
- código de aplicación;
- política de escalado.

**Entrada/Salida esperada:** escribe una tabla con capa y responsable
principal (`Provider`, `Team` o `Shared`).

<details>
<summary>Pista</summary>
En IaaS el proveedor entrega infraestructura flexible, pero el equipo conserva
la mayor parte de la operación por encima de la virtualización.
</details>

Solución compilable: `examples/soluciones/service_models_nivel_1.rs`.

## Ejercicio 2: recomendar con supuestos explícitos `[Nivel 2]`

Un equipo quiere reducir carga operativa y su carga se activa por eventos. Usa
`DecisionContext` y `recommend_model` para obtener una recomendación educativa.

Después cambia el contexto: ahora el equipo necesita controlar el sistema
operativo. ¿Qué cambia?

**Entrada/Salida esperada:** dos llamadas a `recommend_model`; una debe
recomendar `Serverless` y la otra `IaaS`.

<details>
<summary>Pista</summary>
No empieces por un servicio. Declara primero el supuesto que empuja la decisión.
</details>

Solución compilable: `examples/soluciones/service_models_nivel_2.rs`.

## Ejercicio 3: detectar un tradeoff contradictorio `[Nivel 3]`

Construye un contexto donde el equipo exige controlar el sistema operativo y al
mismo tiempo quiere baja carga operativa. Ejecuta la recomendación y explica por
qué el modelo devuelve error en lugar de escoger por ti.

**Entrada/Salida esperada:** `DecisionError::ConflictingAssumptions`.

<details>
<summary>Pista</summary>
La función no intenta resolver decisiones humanas; solo evita ocultar el
tradeoff detrás de una recomendación automática.
</details>

Solución compilable: `examples/soluciones/service_models_nivel_3.rs`.

## Ejercicio 4: diseñar una frontera de responsabilidad `[Nivel 4]`

Diseña la frontera de responsabilidad para una plataforma educativa que tendrá:

- sitio público;
- autenticación con GitHub;
- progreso de estudiantes;
- ejercicios ejecutables;
- publicación gradual de capítulos.

Decide qué partes podrían empezar como SaaS, PaaS, Serverless o IaaS. No hay
respuesta única: justifica qué control conservas, qué operación delegas, qué
costo aceptas y qué decisión dejarías revisable.

**Entrada/Salida esperada:** una propuesta corta con cuatro columnas:
responsabilidad, modelo elegido, supuesto y tradeoff.

<details>
<summary>Pista</summary>
Una buena respuesta puede mezclar modelos. Por ejemplo, autenticación SaaS,
frontend PaaS o serverless, y ejercicios ejecutables en una frontera más
controlada.
</details>

Discusión sugerida:

- Si el objetivo inicial es publicar contenido, conviene delegar hosting,
  autenticación y analítica para reducir operación.
- Si después aparecen laboratorios ejecutables, conviene aislarlos y tratarlos
  como una decisión distinta: seguridad, costos y límites cambian.
- La decisión debe registrarse como supuesto revisable, no como dogma.
