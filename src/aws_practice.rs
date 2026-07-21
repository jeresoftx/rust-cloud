//! AWS como aterrizaje práctico de conceptos cloud.
//!
//! Este módulo no usa SDKs ni credenciales de AWS. Modela decisiones
//! educativas: concepto base, servicio candidato, región, ambiente, permisos,
//! red, límites, observabilidad, costo y responsabilidades retenidas.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudConcept {
    /// Formas de ejecutar trabajo.
    Compute,
    /// Datos, objetos, volúmenes o archivos.
    Storage,
    /// Aislamiento, rutas y exposición.
    Networking,
    /// Identidad, permisos y credenciales.
    Identity,
    /// Servicio administrado por la plataforma.
    ManagedService,
    /// Ejecución orientada a eventos.
    Serverless,
    /// Señales económicas y gobierno de costo.
    FinOps,
    /// Señales para investigar operación.
    Observability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwsService {
    /// Máquinas virtuales.
    Ec2,
    /// Contenedores administrados sin manejar hosts directamente.
    EcsFargate,
    /// Funciones event-driven.
    Lambda,
    /// Almacenamiento de objetos.
    S3,
    /// Distribución de contenido.
    CloudFront,
    /// Red virtual privada.
    Vpc,
    /// Roles y permisos.
    IamRole,
    /// Base de datos relacional administrada.
    Rds,
    /// Cola administrada.
    Sqs,
    /// Bus o scheduler de eventos.
    EventBridge,
    /// Logs, métricas y alarmas.
    CloudWatch,
    /// Presupuestos y alertas económicas.
    Budgets,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwsEnvironment {
    /// Producción.
    Production,
    /// Staging o preproducción.
    Staging,
    /// Desarrollo.
    Development,
    /// Ambiente temporal.
    Sandbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkExposure {
    /// Sin exposición pública.
    Private,
    /// Entrada pública controlada.
    PublicEntrypoint,
    /// Exposición pública sin frontera clara.
    PublicUnbounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataLifecycle {
    /// No aplica.
    NotApplicable,
    /// Datos temporales.
    Temporary,
    /// Retención explícita.
    Retained,
    /// Retención indefinida sin justificación.
    Indefinite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AwsPracticeRequirements {
    /// Concepto del curso que se aterriza.
    pub concept: CloudConcept,
    /// Servicio AWS candidato.
    pub service: AwsService,
    /// Ambiente.
    pub environment: AwsEnvironment,
    /// Región declarada.
    pub region: &'static str,
    /// Dueño humano o equipo responsable.
    pub owner: &'static str,
    /// Propósito humano.
    pub purpose: &'static str,
    /// Permisos mínimos declarados.
    pub least_privilege: bool,
    /// Credenciales temporales o rol asumible.
    pub temporary_credentials: bool,
    /// Exposición de red.
    pub network_exposure: NetworkExposure,
    /// Límite explícito de capacidad, concurrencia, retención o consumo.
    pub has_limit: bool,
    /// Logs, métricas, auditoría o trazas suficientes para el caso.
    pub observability: bool,
    /// Tags para costo, ambiente y dueño.
    pub cost_tags: bool,
    /// Ciclo de vida de datos o recursos.
    pub data_lifecycle: DataLifecycle,
    /// Si el ejemplo requiere credenciales reales.
    pub uses_real_credentials: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AwsWorkload {
    name: &'static str,
    requirements: AwsPracticeRequirements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwsPracticeDecisionError {
    /// Falta nombre.
    MissingName,
    /// Falta región.
    MissingRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwsPracticeFinding {
    /// El servicio no representa bien el concepto declarado.
    ServiceDoesNotMatchConcept(&'static str),
    /// Falta dueño.
    MissingOwner(&'static str),
    /// Falta propósito.
    MissingPurpose(&'static str),
    /// Permisos demasiado amplios.
    BroadPermissions(&'static str),
    /// Credenciales permanentes.
    PermanentCredentials(&'static str),
    /// El ejemplo requiere credenciales reales.
    RealCredentialsInExample(&'static str),
    /// Exposición pública sin frontera.
    PublicNetworkWithoutBoundary(&'static str),
    /// Falta límite explícito.
    MissingLimit(&'static str),
    /// Falta observabilidad.
    MissingObservability(&'static str),
    /// Falta atribución de costo.
    MissingCostTags(&'static str),
    /// Datos retenidos sin ciclo de vida claro.
    IndefiniteDataLifecycle(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsPracticeEvaluation {
    findings: Vec<AwsPracticeFinding>,
}

impl AwsWorkload {
    /// Crea un workload educativo aterrizado en AWS.
    pub fn new(
        name: &'static str,
        requirements: AwsPracticeRequirements,
    ) -> Result<Self, AwsPracticeDecisionError> {
        if name.is_empty() {
            return Err(AwsPracticeDecisionError::MissingName);
        }

        if requirements.region.is_empty() {
            return Err(AwsPracticeDecisionError::MissingRegion);
        }

        Ok(Self { name, requirements })
    }

    /// Evalúa señales educativas de riesgo.
    pub fn evaluate(&self) -> AwsPracticeEvaluation {
        let mut findings = Vec::new();

        if !service_matches_concept(self.requirements.service, self.requirements.concept) {
            findings.push(AwsPracticeFinding::ServiceDoesNotMatchConcept(self.name));
        }

        if self.requirements.owner.is_empty() {
            findings.push(AwsPracticeFinding::MissingOwner(self.name));
        }

        if self.requirements.purpose.is_empty() {
            findings.push(AwsPracticeFinding::MissingPurpose(self.name));
        }

        if !self.requirements.least_privilege {
            findings.push(AwsPracticeFinding::BroadPermissions(self.name));
        }

        if !self.requirements.temporary_credentials {
            findings.push(AwsPracticeFinding::PermanentCredentials(self.name));
        }

        if self.requirements.uses_real_credentials {
            findings.push(AwsPracticeFinding::RealCredentialsInExample(self.name));
        }

        if self.requirements.network_exposure == NetworkExposure::PublicUnbounded {
            findings.push(AwsPracticeFinding::PublicNetworkWithoutBoundary(self.name));
        }

        if !self.requirements.has_limit {
            findings.push(AwsPracticeFinding::MissingLimit(self.name));
        }

        if !self.requirements.observability {
            findings.push(AwsPracticeFinding::MissingObservability(self.name));
        }

        if !self.requirements.cost_tags {
            findings.push(AwsPracticeFinding::MissingCostTags(self.name));
        }

        if self.requirements.data_lifecycle == DataLifecycle::Indefinite {
            findings.push(AwsPracticeFinding::IndefiniteDataLifecycle(self.name));
        }

        AwsPracticeEvaluation { findings }
    }

    /// Nombre del workload.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Requisitos declarados.
    pub const fn requirements(&self) -> AwsPracticeRequirements {
        self.requirements
    }
}

impl AwsPracticeEvaluation {
    /// Indica si no hay hallazgos educativos.
    pub fn is_low_risk(&self) -> bool {
        self.findings.is_empty()
    }

    /// Hallazgos detectados.
    pub fn findings(&self) -> &[AwsPracticeFinding] {
        &self.findings
    }
}

const fn service_matches_concept(service: AwsService, concept: CloudConcept) -> bool {
    matches!(
        (service, concept),
        (AwsService::Ec2, CloudConcept::Compute)
            | (AwsService::EcsFargate, CloudConcept::Compute)
            | (AwsService::Lambda, CloudConcept::Compute)
            | (AwsService::Lambda, CloudConcept::Serverless)
            | (AwsService::S3, CloudConcept::Storage)
            | (AwsService::CloudFront, CloudConcept::Networking)
            | (AwsService::Vpc, CloudConcept::Networking)
            | (AwsService::IamRole, CloudConcept::Identity)
            | (AwsService::Rds, CloudConcept::ManagedService)
            | (AwsService::Sqs, CloudConcept::ManagedService)
            | (AwsService::Sqs, CloudConcept::Serverless)
            | (AwsService::EventBridge, CloudConcept::Serverless)
            | (AwsService::CloudWatch, CloudConcept::Observability)
            | (AwsService::Budgets, CloudConcept::FinOps)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workload_requires_region() {
        let requirements = AwsPracticeRequirements {
            concept: CloudConcept::Storage,
            service: AwsService::S3,
            environment: AwsEnvironment::Development,
            region: "",
            owner: "equipo academy",
            purpose: "guardar assets temporales",
            least_privilege: true,
            temporary_credentials: true,
            network_exposure: NetworkExposure::Private,
            has_limit: true,
            observability: true,
            cost_tags: true,
            data_lifecycle: DataLifecycle::Temporary,
            uses_real_credentials: false,
        };

        assert_eq!(
            AwsWorkload::new("assets", requirements),
            Err(AwsPracticeDecisionError::MissingRegion)
        );
    }
}
