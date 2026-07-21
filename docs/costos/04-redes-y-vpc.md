# Costos: redes y VPC

- **Curso:** rust-cloud
- **Capítulo:** 04. Redes y VPC
- **Estado:** implemented
- **Issue:** #16

Este capítulo no usa precios de proveedor. Los precios de gateways,
balanceadores, tráfico entre zonas, direcciones públicas, NAT, endpoints
privados y transferencia cambian por región y fecha. Aquí el costo se expresa
como exposición, operación, observabilidad, rutas, tráfico y complejidad.

## Costos que sí se pueden comparar aquí

| Decisión | Costo visible | Riesgo si se ignora |
|----------|---------------|---------------------|
| Subred pública | Menor fricción de entrada | Exposición accidental y reglas amplias |
| Subred privada | Menor superficie pública | Necesidad de rutas de salida, DNS y operación |
| NAT o salida administrada | Salida privada controlada | Costo por tráfico y punto crítico operativo |
| Balanceador público | Entrada observable y saludable | Costo fijo, reglas de salud y configuración extra |
| Conectividad privada | Menos Internet público | Ruteo, DNS, permisos y costos de enlace |
| Tráfico entre zonas | Más tolerancia a fallos | Costo y latencia si se cruza sin necesidad |
| Reglas de firewall | Control explícito | Deuda operativa si nadie entiende el propósito |

## Decisión sobre benchmarks

No se agrega `criterion` ni otro benchmark en este issue.

**Concepto:** el capítulo compara costos de diseño de red: exposición,
aislamiento, rutas, gateways, balanceo, observabilidad y operación.

**Problema:** medir `VirtualNetwork::add_subnet` o `FirewallRule::allow`
solo mediría operaciones locales triviales y no enseñaría costos reales de red
cloud.

**Alternativas:** agregar `criterion`, simular tráfico artificial o documentar
costos educativos sin dependencia externa.

**Justificación:** se documentan costos educativos sin dependencia externa. Un
benchmark será útil cuando el curso tenga simulaciones de latencia, tráfico
entre zonas, colas, balanceo o FinOps. En este punto la evidencia correcta es
declarar rutas, exposición, reglas y propósito.

## Checklist de costo antes de elegir una red

- ¿Qué necesita entrada pública y qué puede permanecer privado?
- ¿Qué tráfico cruza zonas, regiones, cuentas o proyectos?
- ¿Qué rutas de salida necesitan NAT, endpoint privado o gateway?
- ¿Qué reglas tienen propósito humano explícito?
- ¿Dónde se observan rechazos, cambios de ruta y salud del borde?
- ¿Qué costo aparece por alta disponibilidad del balanceador o gateway?
- ¿Qué acceso administrativo queda auditado y cerrado por defecto?
