# Identidad y accesos

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** draft
- **Milestone:** 05. Identidad y accesos
- **Issue:** #17
- **Módulo Rust:** `src/iam.rs`

## Concepto

Identidad y accesos es la disciplina que decide quién o qué existe dentro de
una plataforma cloud, qué puede hacer, bajo qué frontera de confianza y con qué
evidencia. No se limita a usuarios humanos: también incluye servicios,
workloads, automatizaciones, llaves, sesiones, roles, políticas, tokens y
procesos temporales.

En este curso, IAM se estudia como el contrato entre identidad, intención,
permiso y auditoría. La pregunta central no es "¿qué botón concede acceso?",
sino "¿qué sujeto necesita qué acción sobre qué recurso, por cuánto tiempo y
con qué trazabilidad?".

## Problema

Cloud vuelve fácil crear recursos y delegar operación, pero también vuelve fácil
abrir privilegios de más. Un permiso amplio puede parecer cómodo durante un
incidente, una demo o una automatización, pero termina escondiendo una deuda:
nadie sabe quién puede hacer qué, qué acceso sigue vivo, qué token se filtró o
qué acción no quedó auditable.

La enseñanza de IAM suele caer en tres errores:

- memorizar nombres de servicios de proveedor sin explicar el modelo de
  autorización;
- tratar roles, grupos y políticas como trámites administrativos;
- enseñar seguridad como una lista de prohibiciones, no como diseño de
  responsabilidades.

Sin una base clara, el estudiante puede creer que autenticación y autorización
son lo mismo, que una cuenta compartida es suficiente para operar, o que mínimo
privilegio significa bloquear todo sin entender el flujo real del sistema.

## Alternativas consideradas

1. **Empezar por usuarios y contraseñas.** Es familiar, pero reduce IAM a login
   humano y deja fuera identidades de servicio, automatización y permisos
   temporales.
2. **Empezar por políticas de proveedor.** Aterriza rápido, pero ata el
   criterio a sintaxis comercial y puede convertir el capítulo en catálogo.
3. **Empezar por sujetos, acciones, recursos y evidencia.** Permite razonar
   sobre proveedores después, sin perder la estructura transferible de una
   decisión de acceso.

## Justificación

Este capítulo adopta la tercera alternativa. Primero se modela una decisión de
acceso como relación entre sujeto, acción, recurso, alcance, duración y
evidencia. Después se podrán comparar roles, políticas, grupos, service
accounts, llaves y mecanismos concretos de AWS, GCP u otros proveedores.

La decisión conserva RFC-0001 §2: concepto antes de implementación, problema
antes de herramienta, alternativas antes de elección. También respeta RFC-0001
§10: el curso enseña fundamentos cloud antes de aterrizarlos por proveedor.

## Invariantes del capítulo

- Autenticación responde "quién eres"; autorización responde "qué puedes hacer".
- Todo permiso debe tener sujeto, acción, recurso, alcance y propósito.
- El permiso amplio es una decisión explícita de riesgo, no un atajo neutro.
- Las identidades de servicio son identidades reales y deben auditarse.
- Los accesos temporales reducen superficie permanente, pero requieren
  expiración, rotación y observabilidad.
- Un rol no debe mezclar operación humana, automatización y ejecución de
  workloads sin una razón documentada.
- Mínimo privilegio no significa impedir trabajo: significa conceder lo
  necesario, en el alcance correcto y por el tiempo correcto.
- Separación de responsabilidades reduce daño accidental y abuso.
- Toda decisión de acceso debe poder revisarse después: quién, qué, cuándo, por
  qué y desde dónde.
- Los nombres, límites y servicios de proveedor son material vivo y deben
  revisarse cuando se usen ejemplos fechados.

## Requisitos para `src/iam.rs`

El módulo Rust mínimo deberá modelar, sin dependencias externas:

- sujetos de acceso: humano, servicio, workload y automatización;
- acciones sobre recursos cloud;
- recursos con alcance educativo;
- duración del permiso: permanente, temporal o emergente;
- políticas o grants con propósito explícito;
- evaluación educativa de una solicitud de acceso;
- errores legibles cuando falte sujeto, acción, recurso, propósito o alcance;
- señales de riesgo cuando aparezcan comodines o permisos administrativos.

El módulo no debe intentar implementar un motor IAM real ni reemplazar políticas
de AWS, GCP u otro proveedor. Su función es pedagógica: hacer visibles las
preguntas de autorización y permitir pruebas unitarias claras.

## Decisiones pendientes

- Definir si una acción se modela como enum cerrado o como texto educativo
  validado.
- Definir la granularidad inicial de recursos y alcance.
- Nombrar los errores públicos del módulo antes de escribir ejemplos.
- Decidir cómo representar permisos amplios sin normalizarlos como buena
  práctica.

## Estado editorial

Este capítulo queda en `draft`. No está marcado como `reviewed` ni `published`.
