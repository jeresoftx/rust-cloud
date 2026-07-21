//! GCP como aterrizaje práctico de conceptos cloud.
//!
//! Este módulo no usa SDKs, `gcloud` ni credenciales reales. Modela decisiones
//! educativas: capacidad base, servicio candidato, proyecto, región, ambiente,
//! identidad, red, límites, observabilidad, costo y ciclo de vida.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudCapability {
    /// Formas de ejecutar trabajo.
    Compute,
    /// Datos, objetos, discos, archivos o consultas analíticas.
    Storage,
    /// Aislamiento, rutas y exposición.
    Networking,
    /// Identidad, permisos y service accounts.
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
pub enum GcpService {
    /// Servicio administrado para contenedores HTTP.
    CloudRun,
    /// Máquinas virtuales.
    ComputeEngine,
    /// Kubernetes administrado.
    Gke,
    /// Funciones event-driven.
    CloudFunctions,
    /// Almacenamiento de objetos.
    CloudStorage,
    /// Red virtual privada.
    Vpc,
    /// Identidad para cargas y permisos.
    ServiceAccount,
    /// Base de datos relacional administrada.
    CloudSql,
    /// Mensajería pub/sub administrada.
    PubSub,
    /// Logs, métricas y trazas.
    CloudOperations,
    /// Presupuestos y alertas de facturación.
    Budgets,
    /// Analítica administrada.
    BigQuery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcpEnvironment {
    /// Producción visible para usuarios.
    Production,
    /// Staging o preproducción.
    Staging,
    /// Desarrollo.
    Development,
    /// Ambiente temporal.
    Sandbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcpNetworkExposure {
    /// Solo tráfico interno.
    InternalOnly,
    /// Entrada pública controlada.
    PublicEntrypoint,
    /// Exposición pública sin frontera clara.
    PublicUnbounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GcpPracticeRequirements {
    /// Capacidad del curso que se aterriza.
    pub capability: CloudCapability,
    /// Servicio GCP candidato.
    pub service: GcpService,
    /// Proyecto GCP que agrupa recursos, IAM y facturación.
    pub project: &'static str,
    /// Región donde vive la carga.
    pub region: &'static str,
    /// Ambiente operativo.
    pub environment: GcpEnvironment,
    /// Dueño humano o equipo responsable.
    pub owner: &'static str,
    /// Propósito humano.
    pub purpose: &'static str,
    /// Roles mínimos declarados.
    pub least_privilege: bool,
    /// Identidad administrada o service account sin llave exportada.
    pub managed_identity: bool,
    /// Exposición de red.
    pub network_exposure: GcpNetworkExposure,
    /// Límite explícito de capacidad, concurrencia, retención o consumo.
    pub has_limit: bool,
    /// Logs, métricas, auditoría o trazas suficientes para el caso.
    pub observability: bool,
    /// Labels para costo, ambiente y dueño.
    pub cost_labels: bool,
    /// Ciclo de vida de datos o recursos.
    pub lifecycle_policy: bool,
    /// Si el ejemplo requiere credenciales reales.
    pub uses_real_credentials: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GcpWorkload {
    name: &'static str,
    requirements: GcpPracticeRequirements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcpPracticeDecisionError {
    /// Falta nombre.
    MissingName,
    /// Falta proyecto.
    MissingProject,
    /// Falta región.
    MissingRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcpFinding {
    /// El servicio no representa bien la capacidad declarada.
    ServiceDoesNotMatchCapability(&'static str),
    /// Falta dueño.
    MissingOwner(&'static str),
    /// Falta propósito.
    MissingPurpose(&'static str),
    /// Permisos demasiado amplios.
    BroadPermissions(&'static str),
    /// Credenciales permanentes o llaves exportadas.
    PermanentOrExportedCredentials(&'static str),
    /// El ejemplo requiere credenciales reales.
    RealCredentialsInExample(&'static str),
    /// Exposición pública sin frontera.
    PublicNetworkWithoutBoundary(&'static str),
    /// Falta límite explícito.
    MissingLimit(&'static str),
    /// Falta observabilidad.
    MissingObservability(&'static str),
    /// Falta atribución de costo.
    MissingCostLabels(&'static str),
    /// Falta ciclo de vida.
    MissingLifecyclePolicy(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcpEvaluation {
    findings: Vec<GcpFinding>,
}

impl GcpWorkload {
    /// Crea un workload educativo aterrizado en GCP.
    pub fn new(
        name: &'static str,
        requirements: GcpPracticeRequirements,
    ) -> Result<Self, GcpPracticeDecisionError> {
        if name.is_empty() {
            return Err(GcpPracticeDecisionError::MissingName);
        }

        if requirements.project.is_empty() {
            return Err(GcpPracticeDecisionError::MissingProject);
        }

        if requirements.region.is_empty() {
            return Err(GcpPracticeDecisionError::MissingRegion);
        }

        Ok(Self { name, requirements })
    }

    /// Evalúa señales educativas de riesgo.
    pub fn evaluate(&self) -> GcpEvaluation {
        let mut findings = Vec::new();

        if !service_matches_capability(self.requirements.service, self.requirements.capability) {
            findings.push(GcpFinding::ServiceDoesNotMatchCapability(self.name));
        }

        if self.requirements.owner.is_empty() {
            findings.push(GcpFinding::MissingOwner(self.name));
        }

        if self.requirements.purpose.is_empty() {
            findings.push(GcpFinding::MissingPurpose(self.name));
        }

        if !self.requirements.least_privilege {
            findings.push(GcpFinding::BroadPermissions(self.name));
        }

        if !self.requirements.managed_identity {
            findings.push(GcpFinding::PermanentOrExportedCredentials(self.name));
        }

        if self.requirements.uses_real_credentials {
            findings.push(GcpFinding::RealCredentialsInExample(self.name));
        }

        if self.requirements.network_exposure == GcpNetworkExposure::PublicUnbounded {
            findings.push(GcpFinding::PublicNetworkWithoutBoundary(self.name));
        }

        if !self.requirements.has_limit {
            findings.push(GcpFinding::MissingLimit(self.name));
        }

        if !self.requirements.observability {
            findings.push(GcpFinding::MissingObservability(self.name));
        }

        if !self.requirements.cost_labels {
            findings.push(GcpFinding::MissingCostLabels(self.name));
        }

        if !self.requirements.lifecycle_policy {
            findings.push(GcpFinding::MissingLifecyclePolicy(self.name));
        }

        GcpEvaluation { findings }
    }

    /// Nombre del workload.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Requisitos declarados.
    pub const fn requirements(&self) -> GcpPracticeRequirements {
        self.requirements
    }
}

impl GcpEvaluation {
    /// Indica si no hay hallazgos educativos.
    pub fn is_low_risk(&self) -> bool {
        self.findings.is_empty()
    }

    /// Hallazgos detectados.
    pub fn findings(&self) -> &[GcpFinding] {
        &self.findings
    }
}

const fn service_matches_capability(service: GcpService, capability: CloudCapability) -> bool {
    matches!(
        (service, capability),
        (GcpService::CloudRun, CloudCapability::Compute)
            | (GcpService::ComputeEngine, CloudCapability::Compute)
            | (GcpService::Gke, CloudCapability::Compute)
            | (GcpService::CloudFunctions, CloudCapability::Compute)
            | (GcpService::CloudFunctions, CloudCapability::Serverless)
            | (GcpService::CloudStorage, CloudCapability::Storage)
            | (GcpService::BigQuery, CloudCapability::Storage)
            | (GcpService::Vpc, CloudCapability::Networking)
            | (GcpService::ServiceAccount, CloudCapability::Identity)
            | (GcpService::CloudSql, CloudCapability::ManagedService)
            | (GcpService::PubSub, CloudCapability::ManagedService)
            | (GcpService::PubSub, CloudCapability::Serverless)
            | (GcpService::CloudOperations, CloudCapability::Observability)
            | (GcpService::Budgets, CloudCapability::FinOps)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workload_requires_project() {
        let requirements = GcpPracticeRequirements {
            capability: CloudCapability::Storage,
            service: GcpService::CloudStorage,
            project: "",
            region: "us-central1",
            environment: GcpEnvironment::Development,
            owner: "equipo academy",
            purpose: "guardar assets temporales",
            least_privilege: true,
            managed_identity: true,
            network_exposure: GcpNetworkExposure::InternalOnly,
            has_limit: true,
            observability: true,
            cost_labels: true,
            lifecycle_policy: true,
            uses_real_credentials: false,
        };

        assert_eq!(
            GcpWorkload::new("assets", requirements),
            Err(GcpPracticeDecisionError::MissingProject)
        );
    }
}
