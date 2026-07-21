# Redes y VPC

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** draft
- **Milestone:** 04. Redes y VPC
- **Issue:** #13
- **Módulo Rust:** `src/networking.rs`

## Concepto

Una red cloud no es solo una colección de subredes, direcciones IP y pantallas
de configuración. Es el contrato que define qué cargas de trabajo existen
dentro de un límite, cómo se encuentran, qué caminos puede tomar el tráfico y
qué exposición queda permitida.

En este curso, una VPC se estudia como una frontera de alcance y confianza:
agrupa recursos, organiza subredes, rutas, reglas de tráfico, puntos de entrada,
conectividad privada y resolución de nombres. El objetivo no es memorizar la
consola de un proveedor, sino razonar sobre alcance, aislamiento,
disponibilidad y operación.

## Problema

La enseñanza de redes cloud suele caer en dos extremos:

- Convertirse en una lista de servicios de proveedor sin explicar el modelo
  mental.
- Repetir redes tradicionales sin mostrar qué cambia cuando la infraestructura
  se vuelve programable.

Ambos caminos ocultan las preguntas de ingeniería que importan: ¿qué puede
hablar con qué?, ¿por dónde entra y sale el tráfico?, ¿qué rutas existen?, ¿qué
reglas autorizan el flujo?, ¿qué nombres resuelven los destinos?, ¿qué pasa si
una zona falla? y ¿qué costo operativo introduce cada nivel de aislamiento?

## Alternativas consideradas

1. **Empezar desde una consola de proveedor.** Es práctico, pero ata el
   aprendizaje a nombres comerciales y deja débil el criterio transferible.
2. **Empezar desde redes clásicas.** Refuerza fundamentos, pero puede ignorar
   decisiones propias de cloud como VPC, subredes privadas, gateways, reglas
   administradas y conectividad entre cuentas o proyectos.
3. **Empezar desde alcanzabilidad y límites de confianza.** Permite explicar
   productos concretos más adelante sin perder la idea rectora.

## Justificación

Este capítulo adopta la tercera alternativa. Primero se define el criterio:
límites, rutas, reglas, exposición y operación. Después se podrán mapear esos
conceptos a AWS, GCP u otro proveedor sin que el curso dependa de una interfaz
específica.

La decisión sigue RFC-0001 §2: concepto antes de implementación, problema antes
de herramienta, alternativas antes de elección. También conserva RFC-0001 §10:
el currículum debe formar criterio de ingeniería, no solo memoria operativa.

## Invariantes del capítulo

- Toda carga de trabajo vive dentro de una frontera de red explícita o implícita.
- El direccionamiento define identidad y alcanzabilidad, pero no debe tratarse
  como autorización suficiente.
- Una subred expresa ubicación, alcance de rutas y dominio de fallo; no es una
  política de seguridad por sí sola.
- Las rutas determinan caminos posibles; las reglas de tráfico determinan qué
  comunicación se permite.
- Todo ingreso público debe ser explícito, justificable y observable.
- La conectividad privada reduce exposición pública, pero agrega decisiones de
  ruteo, DNS, operación y costo.
- El balanceo de carga cambia el modelo de entrada, salud y tolerancia a fallos.
- La resolución de nombres forma parte de la conectividad; no es un detalle
  decorativo.
- Los límites, precios y nombres de proveedor son material vivo y deben
  revisarse cuando se usen ejemplos fechados.

## Requisitos para `src/networking.rs`

El módulo Rust mínimo deberá modelar, sin dependencias externas:

- redes virtuales con nombre y rango de direcciones educativo;
- subredes con rol, zona lógica y relación con la red virtual;
- rutas con destino, siguiente salto y propósito;
- reglas de tráfico con origen, destino, protocolo y puerto;
- exposición pública o privada como decisión explícita;
- validaciones legibles cuando falte origen, destino, intención o ruta.

El módulo no debe intentar simular paquetes reales ni reemplazar herramientas de
red. Su función es pedagógica: hacer ejecutables los conceptos del capítulo y
permitir pruebas unitarias claras.

## Decisiones pendientes

- Definir si el rango de direcciones se modela como texto validado o como tipo
  educativo de CIDR.
- Definir la granularidad inicial de rutas y reglas de tráfico.
- Nombrar los errores públicos del módulo antes de escribir ejemplos.

## Estado editorial

Este capítulo queda en `draft`. No está marcado como `reviewed` ni `published`.
