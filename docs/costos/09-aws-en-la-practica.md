# Costos: AWS en la práctica

- **Curso:** rust-cloud
- **Capítulo:** 09. AWS en la práctica
- **Estado:** implemented
- **Issue:** #36

Este capítulo no usa precios de AWS. Los precios, cuotas y nombres exactos son
material vivo y deben revisarse con fecha cuando el curso los publique. Aquí el
costo se expresa como decisión educativa: qué se delega, qué se retiene, qué se
expone, qué se observa y qué se puede atribuir.

## Costos que sí se pueden comparar aquí

| Decisión | Costo visible | Riesgo si se ignora |
|----------|---------------|---------------------|
| EC2 | Capacidad estable y operación retenida | Pagar por hosts sin carga suficiente |
| ECS Fargate | Contenedores sin operar hosts | Menos control fino sobre infraestructura base |
| Lambda | Invocaciones y duración | Picos, retries y falta de idempotencia |
| S3 | Objetos, peticiones y transferencia | Retención infinita o acceso público accidental |
| CloudFront | Distribución y salida de datos | Cache mal entendida y exposición innecesaria |
| VPC | Aislamiento y tráfico entre zonas | Red compleja sin propósito claro |
| RDS | Estado relacional administrado | Sobredimensionamiento y backups no probados |
| CloudWatch | Logs, métricas y alarmas | Retención y cardinalidad sin límites |
| Budgets | Señal temprana de gasto | Alertas ignoradas si no tienen dueño |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara costos AWS como consecuencias de una decisión
de plataforma: servicio, responsabilidad retenida, credenciales, red, límites,
observabilidad, tags y ciclo de vida.

**Problema:** medir `AwsWorkload::evaluate` solo mediría lógica local trivial.
Ese número no enseña el costo real de AWS: transferencia, regiones, capacidad,
peticiones, retención, cardinalidad, descuentos, créditos, cuotas o reservas.

**Alternativas:** agregar `criterion`, simular facturas sintéticas o documentar
costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando exista una simulación fechada de tráfico,
almacenamiento, retención, eventos o forecast. Por ahora, la evidencia correcta
es declarar señales que vuelven gobernable la decisión.

## Checklist de costo antes de desplegar en AWS

- ¿Qué concepto del curso representa el servicio?
- ¿Qué región y ambiente explican el despliegue?
- ¿Quién es dueño del gasto?
- ¿Qué propósito humano justifica el recurso?
- ¿Qué permisos mínimos y credenciales temporales protegen la operación?
- ¿Qué parte queda pública y qué parte queda privada?
- ¿Qué límite existe para capacidad, concurrencia, retención o ciclo de vida?
- ¿Qué señal operativa permite investigar incidentes?
- ¿Qué tags permiten atribuir costo?
- ¿Qué recurso se puede apagar, reducir o mover si la señal cambia?

## Lectura de tradeoffs

| Optimización | Puede ahorrar | Puede romper |
|--------------|---------------|--------------|
| Cambiar EC2 por Fargate | Operación de hosts | Control fino de infraestructura |
| Cambiar cómputo estable por Lambda | Capacidad ociosa | Latencia, límites y acoplamiento a eventos |
| Bajar retención de CloudWatch | Logs acumulados | Diagnóstico histórico |
| Agregar lifecycle en S3 | Almacenamiento viejo | Recuperación de objetos antiguos |
| Cerrar red pública | Superficie de ataque | Integraciones que esperaban entrada directa |
| Consolidar cuentas o ambientes | Costos fijos | Aislamiento de riesgos |

AWS en la práctica no se mide por cuántos servicios se usan, sino por cuántas
decisiones quedan explicadas y gobernables.
