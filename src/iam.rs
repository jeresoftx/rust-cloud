//! Identidad y accesos como contrato explícito de confianza.
//!
//! Este módulo no implementa un motor IAM real. Hace visibles las preguntas
//! educativas: quién o qué pide acceso, qué acción necesita, sobre qué recurso,
//! con qué alcance, durante cuánto tiempo, bajo qué condición y con qué rastro.

/// Tipo de identidad que pide o recibe acceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrincipalKind {
    /// Persona operando la plataforma.
    HumanUser,
    /// Cuenta de servicio usada por una aplicación o proceso.
    ServiceAccount,
    /// Workload que obtiene identidad al ejecutarse.
    Workload,
    /// Automatización de despliegue, respaldo o mantenimiento.
    Automation,
    /// Identidad federada o externa a la frontera principal.
    ExternalIdentity,
}

/// Frontera de confianza donde vive la identidad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustBoundary {
    /// Misma cuenta, proyecto o tenant educativo.
    SameAccount,
    /// Otra cuenta o proyecto bajo control organizacional.
    CrossAccount,
    /// Proveedor externo o federación fuera del control directo.
    ExternalProvider,
}

/// Acción educativa sobre un recurso cloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessAction {
    /// Leer metadatos o contenido.
    Read,
    /// Escribir, crear o modificar contenido.
    Write,
    /// Ejecutar una operación o workload.
    Execute,
    /// Administrar configuración, permisos o ciclo de vida.
    Administer,
    /// Asumir otro rol o identidad.
    AssumeRole,
}

/// Tipo de recurso protegido.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    /// Bucket, objeto o almacenamiento por clave.
    ObjectStorage,
    /// Máquina, contenedor, función o workload.
    ComputeWorkload,
    /// Red virtual, subred, ruta o regla de tráfico.
    Network,
    /// Secreto, llave o material sensible.
    Secret,
    /// Base de datos o servicio persistente.
    Database,
    /// Cuenta, proyecto, suscripción o tenant.
    PlatformAccount,
}

/// Alcance educativo del permiso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceScope {
    /// Un recurso específico.
    SingleResource,
    /// Un grupo acotado de recursos.
    ResourceGroup,
    /// Toda una cuenta, proyecto o suscripción.
    AccountWide,
    /// Toda una organización.
    OrganizationWide,
    /// Comodín explícito.
    Wildcard,
}

/// Duración de credenciales o permiso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialDuration {
    /// Acceso persistente hasta que alguien lo rote o revoque.
    Permanent,
    /// Sesión temporal con expiración declarada.
    TemporarySession { hours: u16 },
    /// Acceso emergente para incidente o recuperación.
    BreakGlass { hours: u16 },
}

/// Identidad educativa que participa en una decisión de acceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Principal {
    name: &'static str,
    kind: PrincipalKind,
    trust_boundary: TrustBoundary,
    credential_duration: CredentialDuration,
    mfa_required: bool,
}

/// Grant educativo: una decisión explícita de acceso.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessGrant {
    principal: Principal,
    resource: &'static str,
    resource_kind: ResourceKind,
    action: AccessAction,
    scope: ResourceScope,
    purpose: &'static str,
    condition: Option<&'static str>,
    audit_event: Option<&'static str>,
}

/// Conjunto de grants que se evalúan juntos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPlan {
    name: &'static str,
    grants: Vec<AccessGrant>,
}

/// Error al construir una decisión de acceso incompleta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessDecisionError {
    /// Falta nombre de identidad o plan.
    MissingPrincipal,
    /// Falta acción.
    MissingAction,
    /// Falta recurso.
    MissingResource,
    /// Falta alcance.
    MissingScope,
    /// Falta propósito humano.
    MissingPurpose,
    /// La duración temporal declarada no puede ser cero.
    InvalidDuration(&'static str),
}

/// Señal educativa de riesgo o deuda de acceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessFinding {
    /// Administración con comodín o recurso global.
    WildcardAdministration(&'static str),
    /// Usuario humano con privilegio administrativo sin MFA.
    HumanAdminWithoutMfa(&'static str),
    /// Identidad externa con credenciales permanentes.
    LongLivedExternalAccess(&'static str),
    /// Frontera cruzada sin condición explícita.
    CrossBoundaryWithoutCondition(&'static str),
    /// Permiso privilegiado sin evento auditable.
    PrivilegedAccessWithoutAudit(&'static str),
}

/// Resultado de evaluar un plan de acceso.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessEvaluation {
    findings: Vec<AccessFinding>,
}

impl CredentialDuration {
    /// Construye una sesión temporal.
    pub const fn temporary(hours: u16) -> Result<Self, AccessDecisionError> {
        if hours == 0 {
            return Err(AccessDecisionError::InvalidDuration(
                "hours debe ser mayor que cero",
            ));
        }

        Ok(Self::TemporarySession { hours })
    }

    /// Construye un acceso emergente.
    pub const fn break_glass(hours: u16) -> Result<Self, AccessDecisionError> {
        if hours == 0 {
            return Err(AccessDecisionError::InvalidDuration(
                "hours debe ser mayor que cero",
            ));
        }

        Ok(Self::BreakGlass { hours })
    }

    /// Indica si el permiso expira por diseño.
    pub const fn expires(self) -> bool {
        matches!(
            self,
            Self::TemporarySession { .. } | Self::BreakGlass { .. }
        )
    }
}

impl Principal {
    /// Crea una identidad educativa.
    ///
    /// ```
    /// use rust_cloud::iam::{
    ///     CredentialDuration, Principal, PrincipalKind, TrustBoundary,
    /// };
    ///
    /// let principal = Principal::new(
    ///     "api-orders",
    ///     PrincipalKind::Workload,
    ///     TrustBoundary::SameAccount,
    ///     CredentialDuration::TemporarySession { hours: 1 },
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(principal.name(), "api-orders");
    /// ```
    pub fn new(
        name: &'static str,
        kind: PrincipalKind,
        trust_boundary: TrustBoundary,
        credential_duration: CredentialDuration,
    ) -> Result<Self, AccessDecisionError> {
        if name.is_empty() {
            return Err(AccessDecisionError::MissingPrincipal);
        }

        Ok(Self {
            name,
            kind,
            trust_boundary,
            credential_duration,
            mfa_required: false,
        })
    }

    /// Marca que la identidad requiere MFA para operar.
    pub const fn with_mfa(mut self) -> Self {
        self.mfa_required = true;
        self
    }

    /// Nombre estable de la identidad.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Tipo de identidad.
    pub const fn kind(&self) -> PrincipalKind {
        self.kind
    }

    /// Frontera de confianza declarada.
    pub const fn trust_boundary(&self) -> TrustBoundary {
        self.trust_boundary
    }

    /// Duración de credenciales o permiso.
    pub const fn credential_duration(&self) -> CredentialDuration {
        self.credential_duration
    }

    /// Indica si MFA es obligatorio.
    pub const fn mfa_required(&self) -> bool {
        self.mfa_required
    }
}

impl AccessGrant {
    /// Crea un grant con validación de campos obligatorios.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        principal: Option<Principal>,
        resource: &'static str,
        resource_kind: ResourceKind,
        action: Option<AccessAction>,
        scope: Option<ResourceScope>,
        purpose: &'static str,
    ) -> Result<Self, AccessDecisionError> {
        let principal = principal.ok_or(AccessDecisionError::MissingPrincipal)?;
        let action = action.ok_or(AccessDecisionError::MissingAction)?;
        let scope = scope.ok_or(AccessDecisionError::MissingScope)?;

        if resource.is_empty() {
            return Err(AccessDecisionError::MissingResource);
        }

        if purpose.is_empty() {
            return Err(AccessDecisionError::MissingPurpose);
        }

        Ok(Self {
            principal,
            resource,
            resource_kind,
            action,
            scope,
            purpose,
            condition: None,
            audit_event: None,
        })
    }

    /// Agrega condición explícita para el permiso.
    pub fn with_condition(mut self, condition: &'static str) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Agrega el nombre educativo del evento auditable.
    pub fn with_audit_event(mut self, audit_event: &'static str) -> Self {
        self.audit_event = Some(audit_event);
        self
    }

    /// Identidad que recibe el grant.
    pub const fn principal(&self) -> Principal {
        self.principal
    }

    /// Recurso protegido.
    pub const fn resource(&self) -> &'static str {
        self.resource
    }

    /// Tipo de recurso.
    pub const fn resource_kind(&self) -> ResourceKind {
        self.resource_kind
    }

    /// Acción permitida.
    pub const fn action(&self) -> AccessAction {
        self.action
    }

    /// Alcance del permiso.
    pub const fn scope(&self) -> ResourceScope {
        self.scope
    }

    /// Propósito humano.
    pub const fn purpose(&self) -> &'static str {
        self.purpose
    }

    /// Condición explícita del permiso.
    pub const fn condition(&self) -> Option<&'static str> {
        self.condition
    }
}

impl AccessPlan {
    /// Crea un plan de acceso.
    pub fn new(name: &'static str) -> Result<Self, AccessDecisionError> {
        if name.is_empty() {
            return Err(AccessDecisionError::MissingPrincipal);
        }

        Ok(Self {
            name,
            grants: Vec::new(),
        })
    }

    /// Agrega un grant al plan.
    pub fn add_grant(&mut self, grant: AccessGrant) {
        self.grants.push(grant);
    }

    /// Evalúa las señales de riesgo del plan.
    pub fn evaluate(&self) -> AccessEvaluation {
        AccessEvaluation {
            findings: self.grants.iter().flat_map(evaluate_grant).collect(),
        }
    }

    /// Nombre del plan.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Grants declarados.
    pub fn grants(&self) -> &[AccessGrant] {
        &self.grants
    }
}

impl AccessEvaluation {
    /// Indica si el plan no tiene hallazgos educativos.
    pub fn is_low_risk(&self) -> bool {
        self.findings.is_empty()
    }

    /// Hallazgos detectados.
    pub fn findings(&self) -> &[AccessFinding] {
        &self.findings
    }
}

fn evaluate_grant(grant: &AccessGrant) -> Vec<AccessFinding> {
    let mut findings = Vec::new();
    let principal = grant.principal();

    if grant.action == AccessAction::Administer
        && (grant.resource == "*" || grant.scope == ResourceScope::Wildcard)
    {
        findings.push(AccessFinding::WildcardAdministration(grant.resource));
    }

    if principal.kind == PrincipalKind::HumanUser
        && grant.action == AccessAction::Administer
        && !principal.mfa_required
    {
        findings.push(AccessFinding::HumanAdminWithoutMfa(principal.name));
    }

    if principal.trust_boundary == TrustBoundary::ExternalProvider
        && !principal.credential_duration.expires()
    {
        findings.push(AccessFinding::LongLivedExternalAccess(principal.name));
    }

    if matches!(
        principal.trust_boundary,
        TrustBoundary::CrossAccount | TrustBoundary::ExternalProvider
    ) && grant.condition.is_none()
    {
        findings.push(AccessFinding::CrossBoundaryWithoutCondition(principal.name));
    }

    if matches!(
        grant.scope,
        ResourceScope::AccountWide | ResourceScope::OrganizationWide | ResourceScope::Wildcard
    ) && matches!(
        grant.action,
        AccessAction::Administer | AccessAction::AssumeRole
    ) && grant.audit_event.is_none()
    {
        findings.push(AccessFinding::PrivilegedAccessWithoutAudit(grant.resource));
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporary_duration_must_be_positive() {
        assert_eq!(
            CredentialDuration::temporary(0),
            Err(AccessDecisionError::InvalidDuration(
                "hours debe ser mayor que cero",
            ))
        );
    }

    #[test]
    fn low_risk_workload_access_has_no_findings() {
        let workload = Principal::new(
            "orders-api",
            PrincipalKind::Workload,
            TrustBoundary::SameAccount,
            CredentialDuration::TemporarySession { hours: 1 },
        )
        .unwrap();
        let grant = AccessGrant::new(
            Some(workload),
            "bucket-orders-read",
            ResourceKind::ObjectStorage,
            Some(AccessAction::Read),
            Some(ResourceScope::SingleResource),
            "leer órdenes confirmadas para exponer la API",
        )
        .unwrap()
        .with_audit_event("object.read");
        let mut plan = AccessPlan::new("orders-prod").unwrap();

        plan.add_grant(grant);

        assert!(plan.evaluate().is_low_risk());
    }
}
