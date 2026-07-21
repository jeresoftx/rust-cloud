# Costos: serverless

- **Curso:** rust-cloud
- **Capítulo:** 07. Serverless
- **Estado:** implemented
- **Issue:** #28

Este capítulo no usa precios de proveedor. En serverless, el costo cambia por
invocaciones, duración, memoria, concurrencia, tráfico, eventos, logs, colas,
workflows, almacenamiento y región. Aquí el costo se expresa como elasticidad,
límites, retries, observabilidad, dependencia y carga downstream.

## Costos que sí se pueden comparar aquí

| Decisión | Costo visible | Riesgo si se ignora |
|----------|---------------|---------------------|
| Función HTTP | Pago por invocación y duración | Cold start, timeout y latencia variable |
| Handler de cola | Escalado por backlog | Duplicados si falta idempotencia |
| Concurrencia sin límite | Menos configuración inicial | Saturar bases, APIs o servicios dependientes |
| Retry automático | Recuperación ante fallas temporales | Multiplicar efectos secundarios y costo |
| Timeout alto | Menos fallas por tiempo | Ocultar procesos largos en runtime incorrecto |
| Observabilidad detallada | Mejor diagnóstico | Logs y métricas pueden crecer mucho |
| Workflow serverless | Orquestación visible | Costo por transición y acoplamiento al proveedor |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara costos de ejecución por eventos: invocación,
duración, concurrencia, retries, observabilidad y carga sobre dependencias.

**Problema:** medir `ServerlessWorkload::evaluate` solo mediría operaciones
locales triviales. Ese número no enseña el costo real de serverless: cold
starts, duración, memoria, tráfico, transiciones, logs, escalado y saturación
de servicios downstream.

**Alternativas:** agregar `criterion`, simular millones de eventos o documentar
costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando el curso tenga simulaciones de tráfico, latencia,
colas, memoria, cold starts o FinOps. En este punto la evidencia correcta es
declarar límites, retries, idempotencia, estado y observabilidad.

## Checklist de costo antes de elegir serverless

- ¿El flujo es variable o constante?
- ¿Qué dispara la ejecución?
- ¿Cuánto dura y cuánta memoria necesita?
- ¿Qué límite de concurrencia protege a dependencias?
- ¿Qué pasa si el evento se procesa dos veces?
- ¿Cuánto cuesta observar cada ejecución?
- ¿Qué servicios downstream se saturan al escalar?
- ¿Qué partes quedan acopladas a eventos o workflows de proveedor?
- ¿Qué alternativa con capacidad estable sería más simple o barata?
