//! Servicios manejados como frontera explícita de delegación.
//!
//! Este módulo no cataloga productos de proveedor. Hace visibles decisiones
//! educativas: qué capacidad se consume, qué operación se delega, qué conserva
//! el equipo, qué estado existe, cómo se recupera y qué riesgos quedan.

/// Capacidad principal entregada por un servicio manejado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedCapability {
    /// Base de datos relacional o documental.
    Database,
    /// Cola, bus o mensajería asíncrona.
    Queue,
    /// Caché administrado.
    Cache,
    /// Gestión de secretos o llaves.
    Secrets,
    /// Métricas, logs, trazas o alertas.
    Observability,
    /// Búsqueda o indexación administrada.
    Search,
}

/// Responsabilidad que normalmente puede delegarse al proveedor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelegatedResponsibility {
    /// Aprovisionar capacidad base.
    Provisioning,
    /// Aplicar parches o upgrades administrados.
    Patching,
    /// Ejecutar respaldos administrados.
    BackupAutomation,
    /// Replicar datos o nodos.
    Replication,
    /// Monitorear salud del servicio base.
    HealthMonitoring,
    /// Escalar capacidad dentro de límites configurados.
    Scaling,
}

/// Responsabilidad que el equipo no debe olvidar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetainedResponsibility {
    /// Definir esquema, índices, particiones o contratos de datos.
    DataModeling,
    /// Configurar permisos y acceso.
    AccessControl,
    /// Definir cuotas, límites y capacidad esperada.
    CapacityPlanning,
    /// Probar restauración y recuperación.
    RecoveryTesting,
    /// Observar uso, errores y saturación.
    UsageObservability,
    /// Controlar costo y crecimiento.
    CostControl,
}

/// Criticidad educativa del servicio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Criticality {
    /// Puede fallar sin romper el producto principal.
    Low,
    /// Afecta una parte importante del flujo.
    Medium,
    /// Su falla degrada producción o datos críticos.
    High,
}

/// Estrategia de recuperación declarada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Sin recuperación declarada.
    None,
    /// Backup existe, pero no hay restore probado.
    BackupOnly,
    /// Restore probado en entorno controlado.
    TestedRestore,
    /// Replica/failover con prueba periódica.
    TestedFailover,
}

/// Requisitos para decidir o evaluar un servicio manejado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedServiceRequirements {
    /// Capacidad que se busca.
    pub capability: ManagedCapability,
    /// El servicio conserva estado durable.
    pub durable_state: bool,
    /// Criticidad para el sistema.
    pub criticality: Criticality,
    /// Estrategia de recuperación.
    pub recovery: RecoveryStrategy,
    /// Tiene dueño operativo claro.
    pub has_owner: bool,
    /// Propósito humano del servicio.
    pub purpose: &'static str,
}

/// Perfil educativo de un servicio manejado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedServiceProfile {
    name: &'static str,
    capability: ManagedCapability,
    delegated: Vec<DelegatedResponsibility>,
    retained: Vec<RetainedResponsibility>,
    requirements: ManagedServiceRequirements,
}

/// Error al construir un perfil incompleto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedServiceDecisionError {
    /// Falta nombre del perfil.
    MissingName,
    /// Falta propósito humano.
    MissingPurpose,
}

/// Hallazgo educativo de operación o riesgo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedServiceFinding {
    /// El servicio no tiene dueño operativo.
    MissingOwner(&'static str),
    /// Estado durable sin backup o restore.
    DurableStateWithoutRecovery(&'static str),
    /// Servicio crítico sin restauración probada.
    CriticalServiceWithoutTestedRecovery(&'static str),
    /// No se declaró responsabilidad retenida clave.
    MissingRetainedResponsibility(RetainedResponsibility),
}

/// Resultado de evaluar un servicio manejado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedServiceEvaluation {
    findings: Vec<ManagedServiceFinding>,
}

impl ManagedServiceProfile {
    /// Crea un perfil educativo de servicio manejado.
    pub fn new(
        name: &'static str,
        requirements: ManagedServiceRequirements,
    ) -> Result<Self, ManagedServiceDecisionError> {
        if name.is_empty() {
            return Err(ManagedServiceDecisionError::MissingName);
        }

        if requirements.purpose.is_empty() {
            return Err(ManagedServiceDecisionError::MissingPurpose);
        }

        Ok(Self {
            name,
            capability: requirements.capability,
            delegated: default_delegated(requirements.capability),
            retained: default_retained(requirements.capability),
            requirements,
        })
    }

    /// Evalúa riesgos educativos visibles.
    pub fn evaluate(&self) -> ManagedServiceEvaluation {
        let mut findings = Vec::new();

        if !self.requirements.has_owner {
            findings.push(ManagedServiceFinding::MissingOwner(self.name));
        }

        if self.requirements.durable_state && self.requirements.recovery == RecoveryStrategy::None {
            findings.push(ManagedServiceFinding::DurableStateWithoutRecovery(
                self.name,
            ));
        }

        if self.requirements.criticality == Criticality::High
            && !matches!(
                self.requirements.recovery,
                RecoveryStrategy::TestedRestore | RecoveryStrategy::TestedFailover
            )
        {
            findings.push(ManagedServiceFinding::CriticalServiceWithoutTestedRecovery(
                self.name,
            ));
        }

        for required in [
            RetainedResponsibility::AccessControl,
            RetainedResponsibility::UsageObservability,
            RetainedResponsibility::CostControl,
        ] {
            if !self.retained.contains(&required) {
                findings.push(ManagedServiceFinding::MissingRetainedResponsibility(
                    required,
                ));
            }
        }

        ManagedServiceEvaluation { findings }
    }

    /// Nombre del perfil.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Capacidad principal.
    pub const fn capability(&self) -> ManagedCapability {
        self.capability
    }

    /// Responsabilidades delegadas.
    pub fn delegated(&self) -> &[DelegatedResponsibility] {
        &self.delegated
    }

    /// Responsabilidades retenidas.
    pub fn retained(&self) -> &[RetainedResponsibility] {
        &self.retained
    }
}

impl ManagedServiceEvaluation {
    /// Indica si no hay hallazgos educativos.
    pub fn is_low_risk(&self) -> bool {
        self.findings.is_empty()
    }

    /// Hallazgos detectados.
    pub fn findings(&self) -> &[ManagedServiceFinding] {
        &self.findings
    }
}

fn default_delegated(capability: ManagedCapability) -> Vec<DelegatedResponsibility> {
    match capability {
        ManagedCapability::Database => vec![
            DelegatedResponsibility::Provisioning,
            DelegatedResponsibility::Patching,
            DelegatedResponsibility::BackupAutomation,
            DelegatedResponsibility::Replication,
            DelegatedResponsibility::HealthMonitoring,
        ],
        ManagedCapability::Queue | ManagedCapability::Cache => vec![
            DelegatedResponsibility::Provisioning,
            DelegatedResponsibility::Scaling,
            DelegatedResponsibility::HealthMonitoring,
        ],
        ManagedCapability::Secrets
        | ManagedCapability::Observability
        | ManagedCapability::Search => {
            vec![
                DelegatedResponsibility::Provisioning,
                DelegatedResponsibility::Patching,
                DelegatedResponsibility::HealthMonitoring,
            ]
        }
    }
}

fn default_retained(capability: ManagedCapability) -> Vec<RetainedResponsibility> {
    let mut retained = vec![
        RetainedResponsibility::AccessControl,
        RetainedResponsibility::CapacityPlanning,
        RetainedResponsibility::UsageObservability,
        RetainedResponsibility::CostControl,
    ];

    if matches!(
        capability,
        ManagedCapability::Database | ManagedCapability::Search
    ) {
        retained.push(RetainedResponsibility::DataModeling);
        retained.push(RetainedResponsibility::RecoveryTesting);
    }

    retained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_requires_visible_purpose() {
        let requirements = ManagedServiceRequirements {
            capability: ManagedCapability::Database,
            durable_state: true,
            criticality: Criticality::High,
            recovery: RecoveryStrategy::TestedRestore,
            has_owner: true,
            purpose: "",
        };

        assert_eq!(
            ManagedServiceProfile::new("academy-db", requirements),
            Err(ManagedServiceDecisionError::MissingPurpose)
        );
    }
}
