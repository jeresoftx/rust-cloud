# Ejercicios: costos y FinOps

- **Curso:** rust-cloud
- **Capítulo:** 08. Costos y FinOps
- **Estado:** implemented
- **Issue:** #32

Estos ejercicios practican costo como señal de diseño. La meta no es calcular
precios reales, sino declarar unidad económica, dueño, propósito, visibilidad,
presupuesto, elasticidad y tradeoffs antes de optimizar.

## Ejercicio 1: costo de producción gobernable `[Nivel 1]`

Construye un `FinOpsProfile` para una API de la academia:

- categoría `CostCategory::Compute`;
- ambiente `Environment::Production`;
- patrón `UsagePattern::Steady`;
- criticidad alta;
- elasticidad acotada;
- visibilidad por unidad económica;
- presupuesto con forecast y revisión;
- intención de reducir desperdicio;
- dueño, propósito y unidad económica explícitos.

**Entrada/Salida esperada:** la evaluación debe ser de bajo riesgo educativo.

<details>
<summary>Pista</summary>
Un costo sano no es necesariamente bajo. Es atribuible, explicable y gobernable.
</details>

Solución compilable: `examples/soluciones/finops_nivel_1.rs`.

## Ejercicio 2: detectar costo sin atribución `[Nivel 2]`

Modela workers de previews en desarrollo con:

- categoría de invocaciones;
- patrón de uso creciente;
- elasticidad sin límite;
- visibilidad agregada;
- sin presupuesto;
- sin dueño;
- sin unidad económica.

**Entrada/Salida esperada:** la evaluación debe reportar dueño ausente, unidad
económica ausente, presupuesto ausente, elasticidad sin límite y baja
visibilidad.

<details>
<summary>Pista</summary>
La elasticidad ayuda cuando existe límite y señal. Sin eso, solo hace que el
problema pueda crecer más rápido.
</details>

Solución compilable: `examples/soluciones/finops_nivel_2.rs`.

## Ejercicio 3: comparar inversión, desperdicio y riesgo `[Nivel 3]`

Construye tres perfiles:

- observabilidad de producción que compra confiabilidad;
- índice heredado casi sin uso en producción;
- ambiente efímero de experimentación con límite explícito.

Compara sus evaluaciones y explica cuál gasto es inversión, cuál parece
desperdicio y cuál es riesgo aceptado.

**Entrada/Salida esperada:** la observabilidad con intención y visibilidad debe
ser de bajo riesgo; el índice heredado debe revelar recurso ocioso en
producción; el ambiente efímero acotado debe quedar gobernable.

<details>
<summary>Pista</summary>
FinOps no consiste en apagar todo. A veces gastar más compra resiliencia,
aprendizaje o velocidad de entrega.
</details>

Solución compilable: `examples/soluciones/finops_nivel_3.rs`.

## Ejercicio 4: matriz FinOps de una plataforma educativa `[Nivel 4]`

Diseña una matriz para `academy-web` con:

- API pública;
- procesamiento de eventos;
- almacenamiento de assets;
- logs y métricas;
- base de datos manejada;
- ambientes de preview;
- jobs programados;
- tráfico entre regiones.

Para cada fila, declara unidad económica, dueño, propósito, ambiente,
visibilidad, presupuesto, elasticidad, riesgo y acción sugerida.

**Entrada/Salida esperada:** una tabla que distinga inversión, desperdicio,
riesgo aceptado y decisiones que requieren más información.

<details>
<summary>Pista</summary>
La pregunta más importante no es "¿qué apagamos?", sino "¿qué consumo ya no
compra valor, quién puede corregirlo y qué se rompe si lo tocamos?".
</details>

Discusión sugerida:

- Un costo sin dueño debe tratarse como deuda operativa.
- Un ambiente efímero necesita ciclo de vida, no solo una etiqueta.
- La observabilidad excesiva puede costar, pero observabilidad insuficiente
  también encarece incidentes.
- Una unidad económica buena conecta consumo técnico con valor humano.
- Los precios reales deben fecharse y revisarse; el criterio arquitectónico debe
  permanecer.
