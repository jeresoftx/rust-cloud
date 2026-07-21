# Costos: cómputo

- **Curso:** rust-cloud
- **Capítulo:** 02. Cómputo
- **Estado:** implemented
- **Issue:** #8

Este capítulo no usa precios de proveedor. Los precios y límites de AWS, GCP u
otros proveedores cambian; cuando aparezcan en capítulos prácticos deberán
fecharse y revisarse. Aquí el costo se expresa como capacidad reservada,
elasticidad, operación, arranque, cuotas y riesgo de sobredimensionar.

## Costos que sí se pueden comparar aquí

| Modo | Costo de capacidad | Costo operativo | Costo por límites |
|------|--------------------|-----------------|-------------------|
| Máquina virtual | Alto si queda encendida o sobredimensionada | Alto: parches, runtime, reinicios y escalado manual | Medio: límites más visibles, pero el equipo opera más |
| Contenedor | Medio: depende de réplicas, nodos o plataforma | Medio: empaquetar ayuda, operar sigue existiendo | Medio: red, health checks, almacenamiento y scheduling importan |
| Contenedor administrado | Variable: escala mejor, pero puede ocultar cuotas | Medio-bajo: parte de despliegue y escalado se delega | Alto si arranque, límites o concurrencia no se entienden |
| Función | Bajo en reposo, variable por invocación y duración | Bajo en operación base | Alto si eventos, cold starts, tiempo máximo o concurrencia dominan |
| Trabajo batch | Bajo si corre solo cuando hay trabajo | Medio: reintentos, colas y ventanas requieren diseño | Medio-alto si el volumen excede la ventana o la cola crece |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara formas de ejecutar cargas y sus costos
operativos/capacidad.

**Problema:** medir el tiempo de ejecución de `recommend_compute` solo mediría
comparaciones triviales y no enseñaría el costo real de cómputo cloud.

**Alternativas:** agregar `criterion`, escribir un simulador de capacidad o
documentar costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando el curso tenga simulaciones de colas, capacidad,
latencia o FinOps. En este punto la evidencia correcta es declarar recursos,
ciclo de vida, señal de escalado y tradeoff.

## Checklist de costo antes de elegir cómputo

- ¿La carga consume capacidad todo el tiempo o solo por eventos?
- ¿Qué pasa si tarda en arrancar?
- ¿Cuál es la señal real de escalado: CPU, memoria, cola, evento u horario?
- ¿Qué límite rompe primero: memoria, concurrencia, duración, red o cuota?
- ¿Qué operación conserva el equipo aunque delegue plataforma?
- ¿Qué cambiaría si el volumen crece diez veces?
