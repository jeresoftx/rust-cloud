# Identidad y accesos

- **Curso:** rust-cloud
- **Semestre:** 5
- **Estado:** implemented
- **Milestone:** 05. Identidad y accesos
- **Issues:** #17, #18, #19, #20
- **Módulo Rust:** `src/iam.rs`
- **Diagrama:** `diagrams/05-identidad-y-accesos.mmd`
- **Ejemplo:** `examples/iam.rs`
- **Ejercicios:** `docs/ejercicios/05-identidad-y-accesos.md`
- **Costos:** `docs/costos/05-identidad-y-accesos.md`

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

## Imagen mental

Piensa en un edificio técnico con credenciales distintas para cada persona y
sistema.

- La **identidad** es la credencial: dice quién o qué intenta operar.
- La **autenticación** revisa si la credencial es válida.
- La **autorización** decide qué puerta puede abrir esa credencial.
- El **alcance** evita que una llave de una sala abra todo el edificio.
- La **duración** define si la llave vive minutos, horas o indefinidamente.
- La **auditoría** registra qué puerta se abrió, cuándo y con qué justificación.

La metáfora ayuda a separar preguntas que suelen mezclarse. Saber quién es un
sujeto no significa que deba poder administrar todo. Tener un rol no significa
que el permiso sea correcto. Tener una sesión temporal reduce riesgo, pero no
reemplaza propósito, condición y trazabilidad.

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
acceso como relación entre sujeto, acción, recurso, alcance, duración,
condición, propósito y evidencia. Después se podrán comparar roles, políticas,
grupos, service accounts, llaves y mecanismos concretos de AWS, GCP u otros
proveedores.

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

## Modelo Rust mínimo

El módulo Rust mínimo vive en `src/iam.rs` y modela, sin dependencias externas:

- sujetos de acceso: humano, cuenta de servicio, workload, automatización e
  identidad externa;
- acciones sobre recursos cloud;
- recursos con alcance educativo;
- duración de credenciales: permanente, sesión temporal o acceso emergente;
- grants con propósito explícito;
- planes de acceso con hallazgos educativos de riesgo;
- errores legibles cuando falte sujeto, acción, recurso, propósito o alcance;
- señales de riesgo cuando aparezcan comodines, permisos administrativos,
  identidades externas permanentes, fronteras cruzadas sin condición o permisos
  privilegiados sin auditoría.

El módulo no debe intentar implementar un motor IAM real ni reemplazar políticas
de AWS, GCP u otro proveedor. Su función es pedagógica: hacer visibles las
preguntas de autorización y permitir pruebas unitarias claras.

## Comparación educativa

| Elemento | Qué decide | Qué no decide por sí solo | Riesgo común |
|----------|------------|----------------------------|--------------|
| Principal | Quién o qué pide acceso | Si el permiso es correcto | Usar cuentas compartidas |
| Acción | Qué operación se permite | El alcance real del daño | Confundir lectura con administración |
| Recurso | Sobre qué objeto aplica el permiso | La intención humana | Autorizar comodines por comodidad |
| Alcance | Qué tan amplio es el permiso | La duración del acceso | Dar acceso de cuenta completa para tareas puntuales |
| Condición | Cuándo o bajo qué contexto aplica | Auditoría posterior | Creer que toda federación ya es segura |
| Duración | Cuánto vive la credencial | Mínimo privilegio completo | Dejar llaves permanentes en automatizaciones |
| Auditoría | Qué evidencia queda | Prevención automática | Registrar eventos sin revisarlos |

## Cómo leer el módulo Rust

El módulo `iam` empieza declarando una identidad. Una identidad no es solo un
nombre: también tiene tipo, frontera de confianza y duración de credenciales.

```rust
use rust_cloud::iam::{
    CredentialDuration, Principal, PrincipalKind, TrustBoundary,
};

let workload = Principal::new(
    "academy-web",
    PrincipalKind::Workload,
    TrustBoundary::SameAccount,
    CredentialDuration::TemporarySession { hours: 1 },
)
.unwrap();

assert_eq!(workload.name(), "academy-web");
```

Después se declara un grant. El grant obliga a nombrar recurso, tipo de recurso,
acción, alcance y propósito:

```rust
use rust_cloud::iam::{
    AccessAction, AccessGrant, ResourceKind, ResourceScope,
};

# let workload = rust_cloud::iam::Principal::new(
#     "academy-web",
#     rust_cloud::iam::PrincipalKind::Workload,
#     rust_cloud::iam::TrustBoundary::SameAccount,
#     rust_cloud::iam::CredentialDuration::TemporarySession { hours: 1 },
# )
# .unwrap();
let grant = AccessGrant::new(
    Some(workload),
    "assets-publicos",
    ResourceKind::ObjectStorage,
    Some(AccessAction::Write),
    Some(ResourceScope::SingleResource),
    "publicar assets generados por el pipeline",
)
.unwrap()
.with_audit_event("object.write");

assert_eq!(grant.purpose(), "publicar assets generados por el pipeline");
```

Un plan agrupa grants y permite detectar señales de riesgo:

```rust
use rust_cloud::iam::AccessPlan;

# let workload = rust_cloud::iam::Principal::new(
#     "academy-web",
#     rust_cloud::iam::PrincipalKind::Workload,
#     rust_cloud::iam::TrustBoundary::SameAccount,
#     rust_cloud::iam::CredentialDuration::TemporarySession { hours: 1 },
# )
# .unwrap();
# let grant = rust_cloud::iam::AccessGrant::new(
#     Some(workload),
#     "assets-publicos",
#     rust_cloud::iam::ResourceKind::ObjectStorage,
#     Some(rust_cloud::iam::AccessAction::Write),
#     Some(rust_cloud::iam::ResourceScope::SingleResource),
#     "publicar assets generados por el pipeline",
# )
# .unwrap()
# .with_audit_event("object.write");
let mut plan = AccessPlan::new("academy-prod").unwrap();
plan.add_grant(grant);

assert!(plan.evaluate().is_low_risk());
```

El objetivo del modelo no es decir "esta política es producción segura". El
objetivo es forzar preguntas que deben existir antes de tocar una consola:
quién, qué, sobre qué, con qué alcance, durante cuánto tiempo, bajo qué
condición y con qué evidencia.

## Diagrama

El diagrama del capítulo vive en `diagrams/05-identidad-y-accesos.mmd`. Resume
la lectura principal:

```text
principal -> grant -> plan -> evaluación -> hallazgos de riesgo
```

## Ejemplo ejecutable

El ejemplo `examples/iam.rs` construye dos planes educativos: uno de bajo
riesgo para publicar assets de la academia y otro que muestra riesgos comunes
de administración amplia sin MFA.

```bash
cargo run --example iam
```

El ejemplo no contacta proveedores ni valida políticas reales. Su intención es
mostrar qué información mínima debe estar presente antes de convertir una
decisión de acceso en infraestructura.

## Práctica sugerida

Antes de conceder un permiso cloud, escribe:

1. Sujeto: qué persona, servicio, workload o automatización pide acceso.
2. Acción: qué operación exacta necesita.
3. Recurso: sobre qué recurso o grupo de recursos aplica.
4. Alcance: si es un recurso único, grupo, cuenta, organización o comodín.
5. Duración: si el permiso es temporal, emergente o permanente.
6. Condición: desde dónde, cuándo o bajo qué contexto puede usarse.
7. Propósito: qué trabajo legítimo habilita.
8. Auditoría: qué evento permitirá revisarlo después.

Si un permiso no puede explicar su propósito en una frase, todavía no debería
existir. Si necesita comodín, administración amplia o credenciales permanentes,
debe quedar escrito como riesgo explícito y no como normalidad.

Los ejercicios graduados viven en
`docs/ejercicios/05-identidad-y-accesos.md`. El análisis de costos del capítulo
vive en `docs/costos/05-identidad-y-accesos.md`.

## Decisiones registradas

- Las acciones se modelan como enum cerrado (`AccessAction`) para mantener el
  foco en intención educativa, no en sintaxis de proveedor.
- Las identidades se modelan como `Principal`: nombre, tipo (`PrincipalKind`),
  frontera de confianza (`TrustBoundary`), duración de credenciales y señal de
  MFA.
- Los recursos se modelan dentro de cada `AccessGrant` con nombre,
  `ResourceKind` y `ResourceScope`.
- Los permisos se representan como `AccessGrant`: identidad, recurso, acción,
  alcance, propósito, condición opcional y evento auditable opcional.
- Un `AccessPlan` agrupa grants y produce `AccessEvaluation` con hallazgos
  educativos (`AccessFinding`).
- Los hallazgos hacen visibles permisos administrativos con comodín, humanos
  administradores sin MFA, acceso externo permanente, fronteras cruzadas sin
  condición y permisos privilegiados sin auditoría.
- Los errores públicos viven en `AccessDecisionError` para que pruebas y
  ejemplos expliquen cada rechazo.

## Estado editorial

Este capítulo queda en `implemented`. No está marcado como `reviewed` ni
`published`.
