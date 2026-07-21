# Costos: almacenamiento

- **Curso:** rust-cloud
- **Capítulo:** 03. Almacenamiento
- **Estado:** implemented
- **Issue:** #12

Este capítulo no usa precios de proveedor. Los precios, clases y límites de
almacenamiento cambian; cuando aparezcan en capítulos de AWS o GCP deberán
fecharse. Aquí el costo se expresa como retención, recuperación, frecuencia de
lectura, operación, consistencia y riesgo de perder o mover datos.

## Costos que sí se pueden comparar aquí

| Forma | Costo de retención | Costo de recuperación | Costo operativo |
|-------|--------------------|-----------------------|-----------------|
| Objeto | Bajo-medio según volumen y retención | Bajo si se lee con frecuencia razonable | Bajo: no se opera disco, pero sí ciclo de vida y permisos |
| Bloque | Medio-alto si queda provisionado junto a cargas | Bajo para la carga que lo monta | Medio: snapshots, tamaño, zona y recuperación importan |
| Archivo | Medio: capacidad compartida y rendimiento pueden pesar | Medio: depende de concurrencia y patrón de rutas | Medio-alto: permisos, bloqueo y coordinación son relevantes |
| Efímero | Bajo si se destruye con la carga | Alto si se perdió algo que no era derivable | Bajo si se usa solo como scratch |
| Archivo histórico | Bajo en retención relativa | Alto: recuperar puede ser lento o costoso | Bajo-medio: retención, auditoría y restauración deben probarse |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara promesas del dato y costos de almacenamiento:
retención, recuperación, consistencia y operación.

**Problema:** medir `recommend_storage` solo mediría ramas triviales y no
enseñaría costos reales de storage cloud.

**Alternativas:** agregar `criterion`, escribir un simulador de retención o
documentar costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando el curso tenga simulaciones de latencia, patrones de
lectura/escritura, recuperación o FinOps. En este punto la evidencia correcta es
declarar fuente de verdad, patrón de acceso y ciclo de vida.

## Checklist de costo antes de elegir almacenamiento

- ¿El dato es fuente de verdad, derivable o temporal?
- ¿Cuánto tiempo debe retenerse?
- ¿Qué pasa si tarda en recuperarse?
- ¿Qué consistencia necesita el consumidor?
- ¿Qué costo aparece por moverlo entre regiones, clases o sistemas?
- ¿Qué operación se conserva aunque el proveedor administre la infraestructura?
