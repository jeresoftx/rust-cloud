# Costos: identidad y accesos

- **Curso:** rust-cloud
- **Capítulo:** 05. Identidad y accesos
- **Estado:** implemented
- **Issue:** #20

Este capítulo no usa precios de proveedor. En IAM, el costo más importante no
siempre aparece en una factura: aparece como riesgo operativo, fricción,
auditoría, rotación, incidentes, permisos vivos y tiempo de revisión.

## Costos que sí se pueden comparar aquí

| Decisión | Costo visible | Riesgo si se ignora |
|----------|---------------|---------------------|
| Credenciales permanentes | Menos fricción inicial | Filtración, rotación tardía y acceso olvidado |
| Sesiones temporales | Más integración y automatización | Complejidad operativa si nadie entiende expiración |
| Alcance por recurso | Más diseño inicial | Menor daño ante error o abuso |
| Alcance global | Menos trabajo de configuración | Radio de daño alto y auditoría difícil |
| MFA para humanos | Fricción humana pequeña | Menor riesgo de cuenta comprometida |
| Condiciones contextuales | Más reglas que revisar | Menos uso indebido fuera de contexto |
| Eventos auditables | Más almacenamiento y revisión | Evidencia real durante incidentes |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara costos de IAM: fricción, radio de daño,
rotación, auditoría, acceso externo y operación durante incidentes.

**Problema:** medir `AccessPlan::evaluate` solo mediría recorrido de vectores y
no enseñaría costos reales de identidad en cloud.

**Alternativas:** agregar `criterion`, simular miles de grants o documentar
costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando el curso tenga un simulador de políticas, evaluación
masiva o FinOps. En este punto la evidencia correcta es declarar principal,
acción, recurso, alcance, duración, propósito y auditoría.

## Checklist de costo antes de conceder acceso

- ¿El principal es humano, workload, automatización o externo?
- ¿La acción es la mínima necesaria?
- ¿El alcance puede bajar de cuenta a recurso?
- ¿La duración caduca por diseño?
- ¿Hay condición explícita para fronteras cruzadas?
- ¿Existe evento auditable para permisos privilegiados?
- ¿Quién revisará accesos vivos y con qué frecuencia?
- ¿Qué pasa si esta credencial se filtra hoy?
