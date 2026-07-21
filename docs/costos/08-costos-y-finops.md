# Costos: costos y FinOps

- **Curso:** rust-cloud
- **Capítulo:** 08. Costos y FinOps
- **Estado:** implemented
- **Issue:** #32

Este capítulo no usa precios de proveedor. FinOps cambia con región, descuentos,
volumen, soporte, arquitectura, patrón de uso y fecha de consulta. Aquí el costo
se expresa como señales estables: atribución, unidad económica, límite,
visibilidad, presupuesto, desperdicio, inversión y riesgo.

## Costos que sí se pueden comparar aquí

| Decisión | Costo visible | Riesgo si se ignora |
|----------|---------------|---------------------|
| Cómputo constante | Capacidad encendida | Pagar por ociosidad o sobredimensionamiento |
| Cómputo elástico | Crecimiento por demanda | Escalar gasto sin límite operativo |
| Almacenamiento retenido | Datos, backups e índices | Guardar información sin ciclo de vida |
| Tráfico de red | Transferencia entre zonas, regiones o internet | Diseñar flujos caros por ubicación |
| Observabilidad | Logs, métricas, trazas y retención | Diagnóstico caro o insuficiente |
| Ambientes no productivos | Recursos duplicados | Previews o staging sin apagado |
| Servicios manejados | Delegación operativa | Pagar comodidad sin entender responsabilidad |
| Operación humana | Tiempo de investigación y corrección | Optimizar recursos y olvidar trabajo manual |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara señales económicas, no rendimiento local.

**Problema:** medir `FinOpsProfile::evaluate` solo mediría operaciones triviales
en memoria. Ese número no enseña costos cloud reales: consumo, región, tráfico,
retención, descuentos, soporte, cuotas, hábitos del equipo y volumen.

**Alternativas:** agregar un benchmark local, simular una factura falsa o
documentar costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos porque el aprendizaje correcto
es distinguir gasto atribuible, desperdicio, inversión y riesgo aceptado. Un
benchmark será útil cuando el curso tenga simulaciones de uso, escenarios de
facturación sintética o datos fechados revisados por una persona.

## Checklist FinOps antes de optimizar

- ¿Qué unidad económica explica el costo?
- ¿Qué dueño puede interpretar la señal?
- ¿Qué propósito humano o de producto compra el gasto?
- ¿Qué parte del costo es desperdicio y cuál es inversión deliberada?
- ¿Qué límite evita que la elasticidad crezca sin control?
- ¿Qué ambiente puede apagarse, reducirse o expirar?
- ¿Qué retención de logs, métricas, backups o datos sigue siendo necesaria?
- ¿Qué riesgo aparece si se reduce capacidad, redundancia u observabilidad?
- ¿Qué precio real debe consultarse con fecha antes de publicar una recomendación?

## Señales estables para revisar una factura

1. **Atribución:** cada gasto debe poder mapearse a producto, equipo, ambiente o
   iniciativa.
2. **Unidad económica:** cada gasto importante debe tener una unidad de valor:
   estudiante, curso, petición, documento, evento, transacción o minuto.
3. **Límite:** cada recurso elástico debe declarar hasta dónde puede crecer.
4. **Ciclo de vida:** cada recurso no productivo debe tener regla de expiración.
5. **Retención:** cada dato, backup, log o métrica debe justificar cuánto tiempo
   vive.
6. **Tradeoff:** cada ahorro debe declarar qué riesgo, latencia, confiabilidad o
   trabajo humano cambia.
