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

El GitHub Project del curso vive en
`https://github.com/users/jeresoftx/projects/10`. Debe permanecer asociado al
repositorio, contener todos los issues accionables y tener la vista principal
agrupada por `Milestone`. Esta agrupación es requisito de aceptación del
andamiaje de GitHub, no una recomendación visual.

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
| 01 | Modelos de servicio | implemented |
| 02 | Cómputo | implemented |
| 03 | Almacenamiento | implemented |
| 04 | Redes y VPC | implemented |
| 05 | Identidad y accesos | implemented |
| 06 | Servicios manejados | implemented |
| 07 | Serverless | implemented |
| 08 | Costos y FinOps | implemented |
| 09 | AWS en la práctica | implemented |
| 10 | GCP en la práctica | draft |

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

Completar el milestone `10. GCP en la práctica` con el flujo restante: modelo
Rust mínimo, tests, capítulo narrativo expandido, diagrama Mermaid, ejemplo
ejecutable, ejercicios, soluciones y análisis de costos.
