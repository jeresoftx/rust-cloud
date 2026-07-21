# Costos y FinOps

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** draft
- **Milestone:** 08. Costos y FinOps
- **Issues:** #29
- **Módulo Rust:** `src/finops.rs`

## Concepto

Costos y FinOps es la disciplina que convierte el gasto cloud en una decisión
visible de ingeniería, producto y operación. No se limita a ahorrar dinero: busca
entender qué uso genera costo, qué valor produce, qué señales permiten corregir
rumbo y qué responsabilidades tiene cada equipo.

En este curso, FinOps se estudia como un ciclo de **visibilidad, atribución,
decisión y ajuste**. La pregunta central no es "¿cuánto cuesta AWS o GCP?",
sino "¿qué decisión técnica generó este costo, qué valor sostiene y qué cambio
responsable podemos hacer sin romper el sistema?".

## Imagen mental

Piensa en una cocina profesional que vende comidas por pedido.

- El **ticket promedio** dice cuánto valor entra por venta.
- La **receta** define qué ingredientes, equipo y pasos consume cada plato.
- La **merma** muestra desperdicio de insumos.
- El **horno encendido** cuesta aunque no haya pedidos.
- El **reparto** agrega costo por distancia, urgencia y volumen.
- El **tablero diario** permite corregir antes de cerrar el mes.

Cloud funciona de forma parecida. Una arquitectura no solo debe responder bien;
debe explicar qué consume, por qué lo consume y cuándo ese consumo deja de
comprar valor real.

## Problema

Cloud permite crear recursos elásticos, pagar por uso, probar rápido y delegar
operación. Esa misma flexibilidad puede convertir el costo en sorpresa: recursos
olvidados, logs sin retención, tráfico entre regiones, bases sobredimensionadas,
concurrencia sin límite, almacenamiento acumulado, entornos duplicados y
servicios que nadie siente propios.

La enseñanza de costos suele caer en tres errores:

- reducir FinOps a "apagar recursos";
- presentar precios de proveedor sin explicar las decisiones que los producen;
- tratar costo y arquitectura como conversaciones separadas.

Sin una base clara, el estudiante puede creer que optimizar costo es un acto
contable posterior, no una parte del diseño de sistemas. También puede creer que
una arquitectura barata es automáticamente buena, o que una factura alta siempre
es mala, sin distinguir desperdicio, inversión deliberada y riesgo aceptado.

## Alternativas consideradas

1. **Empezar por calculadoras de proveedor.** Son útiles para estimaciones, pero
   cambian rápido y pueden ocultar el modelo de consumo.
2. **Empezar por reglas de ahorro.** Es práctico, pero vuelve el capítulo una
   lista de consejos sin criterio transferible.
3. **Empezar por unidad de costo, atribución y tradeoff operativo.** Permite
   razonar sobre recursos, tráfico, datos, observabilidad y capacidad antes de
   aterrizar precios fechados.

## Justificación

Este capítulo adopta la tercera alternativa. Primero se modela el costo como una
relación entre unidad técnica, uso, dueño, criticidad, valor, señales y decisión
de optimización. Después se podrán usar precios de proveedor como material vivo,
fechado y revisado.

La decisión conserva RFC-0001 §2: concepto antes de implementación, problema
antes de herramienta, alternativas antes de elección. También mantiene
RFC-0001 §10: proveedor después de fundamentos.

## Invariantes del capítulo

- Todo costo cloud debe tener dueño operativo o de producto.
- Un costo no atribuido es una deuda de observabilidad y gobierno.
- Optimizar costo sin entender valor puede degradar el sistema.
- El costo se diseña: capacidad, datos, red, logs, reintentos y regiones importan.
- La unidad de costo debe poder nombrarse: invocación, GiB, petición, hora,
  transferencia, índice, transición o retención.
- La elasticidad necesita límites; sin límites, el costo también escala.
- Los ambientes no productivos también tienen costo y ciclo de vida.
- Los ahorros deben medirse contra riesgo, latencia, operación y experiencia.
- FinOps no reemplaza arquitectura; la vuelve económicamente observable.
- Los precios de proveedor son material vivo y deben fecharse cuando se usen.

## Comparación educativa

| Elemento | Qué decide | Qué no decide por sí solo | Riesgo común |
|----------|------------|----------------------------|--------------|
| Unidad de costo | Qué recurso explica el gasto | Valor producido | Medir consumo sin contexto |
| Atribución | Quién entiende el gasto | Optimización automática | Facturas huérfanas |
| Propósito | Qué valor compra el consumo | Precio correcto | Defender gasto inútil |
| Límite | Hasta dónde puede crecer | Uso saludable | Confundir elasticidad con infinito |
| Señal de uso | Cuándo investigar | Acción adecuada | Alertar tarde o sin dueño |
| Tradeoff | Qué riesgo se acepta | Ahorro garantizado | Romper confiabilidad por ahorrar |

## Requisitos para `src/finops.rs`

El módulo Rust mínimo deberá modelar, sin dependencias externas:

- unidades educativas de costo: cómputo, almacenamiento, red, observabilidad,
  invocaciones y servicios manejados;
- dueño, propósito y ambiente del gasto;
- frecuencia o intensidad de uso;
- criticidad y elasticidad;
- hallazgos cuando falten dueño, propósito, límite, señal de uso o ambiente;
- evaluación de riesgos por gasto no atribuido, elasticidad sin límite,
  observabilidad excesiva o recursos constantes sobredimensionados.

El módulo no debe intentar calcular precios reales. Su función es pedagógica:
hacer visibles las variables que explican por qué aparece un costo.

## Decisiones pendientes

- Definir si la unidad de costo será enum cerrado o perfil extensible.
- Definir cómo representar intensidad de uso sin prometer estimaciones reales.
- Nombrar hallazgos públicos antes de escribir ejemplos.
- Decidir qué umbrales educativos son suficientes sin depender de proveedor.

## Práctica sugerida

Antes de optimizar una arquitectura cloud, escribe:

1. Unidad económica: por estudiante, transacción, curso, documento, minuto o
   evento.
2. Recurso dominante: cómputo, almacenamiento, red, base de datos,
   observabilidad u operación humana.
3. Dueño: quién puede interpretar y corregir la señal.
4. Valor: qué capacidad legítima compra ese gasto.
5. Desperdicio probable: qué consumo no compra valor.
6. Riesgo: qué se puede romper al optimizar.
7. Control: presupuesto, alerta, reporte o revisión periódica.

Si no puedes explicar qué valor compra un costo, todavía no deberías
optimizarlo: primero debes entenderlo.

## Estado editorial

Este capítulo queda en `draft`. No está marcado como `reviewed` ni `published`.
