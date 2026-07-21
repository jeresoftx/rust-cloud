# ROADMAP

Estado de avance de `rust-cloud`, repositorio del camino troncal de Jeresoft
Academy para Cloud en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado actual

El repositorio acaba de entrar en desarrollo. La estructura inicial declara la
frontera del curso, el mapa de capítulos, el contrato para `academy-web`, las
licencias, el crate Rust mínimo y el flujo de trabajo con GitHub.

El plan de trabajo vive en GitHub como milestones, issues, labels y un GitHub
Project asociado al repositorio. Cada paso accionable queda asignado a
`jeresoftx`, asociado al milestone correspondiente y etiquetado para conservar
la regla del repositorio: un issue, un commit y un PR.

Ningún capítulo está marcado como `reviewed` ni `published`, porque la revisión
humana de Joel sigue siendo obligatoria según RFC-0001 §20.

## Progresión del Semestre 5

Cloud cierra el Semestre 5 junto con arquitectura de software. La progresión
esperada es:

1. **Plataforma como concepto:** modelos de servicio y cómputo.
2. **Fundamentos persistentes:** almacenamiento, red e identidad.
3. **Servicios de plataforma:** servicios manejados y serverless.
4. **Operación económica:** costos y FinOps.
5. **Aterrizaje por proveedor:** AWS y GCP en la práctica, sin convertir el
   curso en catálogo de consola.

## Capítulos planeados

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Modelos de servicio | planned |
| 02 | Cómputo | planned |
| 03 | Almacenamiento | planned |
| 04 | Redes y VPC | planned |
| 05 | Identidad y accesos | planned |
| 06 | Servicios manejados | planned |
| 07 | Serverless | planned |
| 08 | Costos y FinOps | planned |
| 09 | AWS en la práctica | planned |
| 10 | GCP en la práctica | planned |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.
- El orden del curso sigue RFC-0001 §10: concepto primero, proveedor después.

## Fuera de alcance por ahora

- Enseñar Kubernetes como capítulo canónico: ese canon vive en `rust-devops`.
- Reemplazar fundamentos por tutoriales de consola de proveedor.
- Publicar recomendaciones dependientes de precios actuales sin revisión
  humana y fecha explícita.
- Agregar SDKs de AWS, GCP u otros proveedores antes de justificar su valor
  educativo.
- Publicar capítulos parciales como si estuvieran completos.

## Siguiente paso natural

Trabajar el primer milestone, `01. Modelos de servicio`, con el flujo completo:
especificación conceptual, modelo Rust mínimo, capítulo/diagrama/ejemplos,
ejercicios, soluciones y análisis de costos.
