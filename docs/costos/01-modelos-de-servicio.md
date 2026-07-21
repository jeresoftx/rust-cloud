# Costos: modelos de servicio

- **Curso:** rust-cloud
- **Capítulo:** 01. Modelos de servicio
- **Estado:** implemented
- **Issue:** #4

Este capítulo no usa precios de proveedor. Los precios cambian y deben
fecharse cuando aparezcan en capítulos de AWS o GCP. Aquí el costo se trata
como una conversación de ingeniería: operación, control, portabilidad,
capacidad y riesgo.

## Costos que sí se pueden comparar aquí

| Modelo | Costo operativo del equipo | Costo de control | Costo de cambio |
|--------|----------------------------|------------------|-----------------|
| IaaS | Alto: el equipo opera sistema operativo, runtime y escalado | Bajo: conserva más control técnico | Medio: algunas decisiones son portables, otras dependen del proveedor |
| PaaS | Medio: parte de la plataforma queda delegada | Medio: se pierden detalles del runtime/plataforma | Medio: APIs y límites de plataforma pueden pesar |
| Serverless | Bajo en operación base, variable por eventos y límites | Medio: se controla código, no el entorno completo | Alto si el diseño se ata al modelo de eventos del proveedor |
| SaaS | Bajo para operación técnica directa | Alto: se controla uso, no implementación | Alto cuando datos, flujos o permisos quedan muy acoplados al producto |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara modelos de responsabilidad, no algoritmos ni
latencia de una implementación.

**Problema:** un benchmark local del módulo mediría iteraciones sobre arreglos
pequeños y daría una falsa sensación de precisión.

**Alternativas:** agregar `criterion`, escribir un simulador de costos o
documentar costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Los
benchmarks serán útiles en capítulos donde haya costo observable: simulación de
capacidad, latencia, colas, almacenamiento, redes o FinOps. Aquí la evidencia
correcta es la claridad del supuesto, no el tiempo de ejecución del código.

## Checklist de costo antes de elegir modelo

- ¿Qué operación concreta estoy delegando?
- ¿Qué control concreto estoy perdiendo?
- ¿Qué límite del proveedor puede afectar el diseño?
- ¿Qué dato o flujo sería costoso de migrar después?
- ¿Qué supuesto debería revisar si cambia el volumen, presupuesto o equipo?
