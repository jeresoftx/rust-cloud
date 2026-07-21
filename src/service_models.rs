//! Modelos de servicio cloud como contratos de responsabilidad.
//!
//! El objetivo de este módulo no es simular un proveedor. Es hacer visible qué
//! delega un equipo cuando elige IaaS, PaaS, SaaS o Serverless.

/// Modelo de servicio cloud explicado por reparto de responsabilidades.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceModel {
    /// Infraestructura como servicio: alto control y mayor carga operativa.
    Iaas,
    /// Plataforma como servicio: runtime y plataforma delegados al proveedor.
    Paas,
    /// Software como servicio: la aplicación completa se consume como producto.
    Saas,
    /// Ejecución orientada a eventos con capacidad y escalado delegados.
    Serverless,
}

/// Capa educativa usada para ubicar una responsabilidad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsibilityLayer {
    /// Centro de datos, energía, enfriamiento e instalaciones físicas.
    PhysicalFacilities,
    /// Red física operada por el proveedor.
    PhysicalNetwork,
    /// Servidores físicos y capacidad base.
    PhysicalCompute,
    /// Hipervisor o capa de virtualización.
    Virtualization,
    /// Sistema operativo que ejecuta la carga.
    OperatingSystem,
    /// Runtime, framework o plataforma de ejecución.
    Runtime,
    /// Código de aplicación escrito o configurado por el equipo.
    ApplicationCode,
    /// Datos de negocio que la aplicación produce o consume.
    ApplicationData,
    /// Política de escalado y administración de capacidad.
    ScalingPolicy,
}

/// Actor responsable de operar o gobernar una capa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsibilityOwner {
    /// La responsabilidad se delega al proveedor cloud o SaaS.
    Provider,
    /// La responsabilidad queda principalmente en el equipo.
    Team,
    /// La responsabilidad se comparte y requiere frontera explícita.
    Shared,
}

/// Intensidad educativa para comparar modelos sin convertirla en métrica real.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EducationalScore {
    /// Bajo para el equipo que consume el servicio.
    Low,
    /// Medio: existe delegación, pero todavía hay decisiones relevantes.
    Medium,
    /// Alto: el equipo conserva mucha responsabilidad o control.
    High,
}

/// Relación entre una capa y su responsable principal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResponsibilityAssignment {
    /// Capa evaluada.
    pub layer: ResponsibilityLayer,
    /// Responsable principal de la capa.
    pub owner: ResponsibilityOwner,
}

/// Perfil pedagógico de un modelo de servicio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceModelProfile {
    /// Modelo descrito por el perfil.
    pub model: ServiceModel,
    /// Reparto de responsabilidades por capa.
    pub responsibilities: &'static [ResponsibilityAssignment],
    /// Nivel de control que conserva el equipo.
    pub team_control: EducationalScore,
    /// Carga operativa que conserva el equipo.
    pub team_operational_load: EducationalScore,
    /// Facilidad relativa para cambiar de proveedor o estrategia.
    pub portability: EducationalScore,
}

/// Supuestos mínimos para recomendar un modelo de servicio.
///
/// # Examples
///
/// ```
/// use rust_cloud::service_models::{recommend_model, DecisionContext, ServiceModel};
///
/// let context = DecisionContext {
///     wants_low_operational_load: true,
///     event_driven_workload: true,
///     ..DecisionContext::default()
/// };
///
/// assert_eq!(recommend_model(context), Ok(ServiceModel::Serverless));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DecisionContext {
    /// La carga requiere administrar el sistema operativo.
    pub requires_operating_system_control: bool,
    /// El equipo quiere reducir carga operativa.
    pub wants_low_operational_load: bool,
    /// El equipo necesita controlar el código de aplicación.
    pub requires_application_control: bool,
    /// La carga se activa principalmente por eventos.
    pub event_driven_workload: bool,
}

/// Error educativo cuando una recomendación no declara supuestos suficientes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionError {
    /// No hay datos mínimos para justificar una recomendación.
    MissingAssumptions,
    /// Los supuestos declarados empujan en direcciones incompatibles.
    ConflictingAssumptions(&'static str),
}

impl ServiceModel {
    /// Devuelve el perfil educativo del modelo.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_cloud::service_models::{
    ///     ResponsibilityLayer, ResponsibilityOwner, ServiceModel,
    /// };
    ///
    /// let profile = ServiceModel::Iaas.profile();
    /// assert_eq!(
    ///     profile.owner_of(ResponsibilityLayer::OperatingSystem),
    ///     Some(ResponsibilityOwner::Team),
    /// );
    /// ```
    pub fn profile(self) -> ServiceModelProfile {
        match self {
            Self::Iaas => IAAS_PROFILE,
            Self::Paas => PAAS_PROFILE,
            Self::Saas => SAAS_PROFILE,
            Self::Serverless => SERVERLESS_PROFILE,
        }
    }

    /// Nivel de abstracción relativo del modelo.
    ///
    /// IaaS queda abajo porque conserva más operación en el equipo; SaaS queda
    /// arriba porque entrega una capacidad completa.
    pub const fn abstraction_level(self) -> u8 {
        match self {
            Self::Iaas => 1,
            Self::Paas => 2,
            Self::Serverless => 3,
            Self::Saas => 4,
        }
    }
}

impl ServiceModelProfile {
    /// Devuelve el responsable principal de una capa.
    pub fn owner_of(self, layer: ResponsibilityLayer) -> Option<ResponsibilityOwner> {
        self.responsibilities
            .iter()
            .find(|assignment| assignment.layer == layer)
            .map(|assignment| assignment.owner)
    }

    /// Itera las capas que siguen principalmente en manos del equipo.
    pub fn team_owned_layers(self) -> impl Iterator<Item = ResponsibilityLayer> {
        self.responsibilities
            .iter()
            .filter(|assignment| assignment.owner == ResponsibilityOwner::Team)
            .map(|assignment| assignment.layer)
    }
}

/// Devuelve los modelos en orden de abstracción creciente.
pub const fn service_models() -> &'static [ServiceModel] {
    &SERVICE_MODELS
}

/// Devuelve las capas usadas por todos los perfiles.
pub const fn responsibility_layers() -> &'static [ResponsibilityLayer] {
    &RESPONSIBILITY_LAYERS
}

/// Recomienda un modelo a partir de supuestos explícitos.
///
/// La función es deliberadamente pequeña: enseña tradeoffs, no sustituye una
/// evaluación de arquitectura.
pub fn recommend_model(context: DecisionContext) -> Result<ServiceModel, DecisionError> {
    if context == DecisionContext::default() {
        return Err(DecisionError::MissingAssumptions);
    }

    if context.requires_operating_system_control && context.wants_low_operational_load {
        return Err(DecisionError::ConflictingAssumptions(
            "control de sistema operativo y baja carga operativa requieren decidir el tradeoff",
        ));
    }

    if context.requires_operating_system_control {
        return Ok(ServiceModel::Iaas);
    }

    if context.event_driven_workload && context.wants_low_operational_load {
        return Ok(ServiceModel::Serverless);
    }

    if context.wants_low_operational_load && !context.requires_application_control {
        return Ok(ServiceModel::Saas);
    }

    if context.event_driven_workload {
        return Ok(ServiceModel::Serverless);
    }

    Ok(ServiceModel::Paas)
}

const SERVICE_MODELS: [ServiceModel; 4] = [
    ServiceModel::Iaas,
    ServiceModel::Paas,
    ServiceModel::Serverless,
    ServiceModel::Saas,
];

const RESPONSIBILITY_LAYERS: [ResponsibilityLayer; 9] = [
    ResponsibilityLayer::PhysicalFacilities,
    ResponsibilityLayer::PhysicalNetwork,
    ResponsibilityLayer::PhysicalCompute,
    ResponsibilityLayer::Virtualization,
    ResponsibilityLayer::OperatingSystem,
    ResponsibilityLayer::Runtime,
    ResponsibilityLayer::ApplicationCode,
    ResponsibilityLayer::ApplicationData,
    ResponsibilityLayer::ScalingPolicy,
];

const IAAS_RESPONSIBILITIES: [ResponsibilityAssignment; 9] = [
    provider(ResponsibilityLayer::PhysicalFacilities),
    provider(ResponsibilityLayer::PhysicalNetwork),
    provider(ResponsibilityLayer::PhysicalCompute),
    provider(ResponsibilityLayer::Virtualization),
    team(ResponsibilityLayer::OperatingSystem),
    team(ResponsibilityLayer::Runtime),
    team(ResponsibilityLayer::ApplicationCode),
    team(ResponsibilityLayer::ApplicationData),
    team(ResponsibilityLayer::ScalingPolicy),
];

const PAAS_RESPONSIBILITIES: [ResponsibilityAssignment; 9] = [
    provider(ResponsibilityLayer::PhysicalFacilities),
    provider(ResponsibilityLayer::PhysicalNetwork),
    provider(ResponsibilityLayer::PhysicalCompute),
    provider(ResponsibilityLayer::Virtualization),
    provider(ResponsibilityLayer::OperatingSystem),
    provider(ResponsibilityLayer::Runtime),
    team(ResponsibilityLayer::ApplicationCode),
    team(ResponsibilityLayer::ApplicationData),
    provider(ResponsibilityLayer::ScalingPolicy),
];

const SERVERLESS_RESPONSIBILITIES: [ResponsibilityAssignment; 9] = [
    provider(ResponsibilityLayer::PhysicalFacilities),
    provider(ResponsibilityLayer::PhysicalNetwork),
    provider(ResponsibilityLayer::PhysicalCompute),
    provider(ResponsibilityLayer::Virtualization),
    provider(ResponsibilityLayer::OperatingSystem),
    provider(ResponsibilityLayer::Runtime),
    team(ResponsibilityLayer::ApplicationCode),
    shared(ResponsibilityLayer::ApplicationData),
    provider(ResponsibilityLayer::ScalingPolicy),
];

const SAAS_RESPONSIBILITIES: [ResponsibilityAssignment; 9] = [
    provider(ResponsibilityLayer::PhysicalFacilities),
    provider(ResponsibilityLayer::PhysicalNetwork),
    provider(ResponsibilityLayer::PhysicalCompute),
    provider(ResponsibilityLayer::Virtualization),
    provider(ResponsibilityLayer::OperatingSystem),
    provider(ResponsibilityLayer::Runtime),
    provider(ResponsibilityLayer::ApplicationCode),
    shared(ResponsibilityLayer::ApplicationData),
    provider(ResponsibilityLayer::ScalingPolicy),
];

const IAAS_PROFILE: ServiceModelProfile = ServiceModelProfile {
    model: ServiceModel::Iaas,
    responsibilities: &IAAS_RESPONSIBILITIES,
    team_control: EducationalScore::High,
    team_operational_load: EducationalScore::High,
    portability: EducationalScore::Medium,
};

const PAAS_PROFILE: ServiceModelProfile = ServiceModelProfile {
    model: ServiceModel::Paas,
    responsibilities: &PAAS_RESPONSIBILITIES,
    team_control: EducationalScore::Medium,
    team_operational_load: EducationalScore::Medium,
    portability: EducationalScore::Medium,
};

const SERVERLESS_PROFILE: ServiceModelProfile = ServiceModelProfile {
    model: ServiceModel::Serverless,
    responsibilities: &SERVERLESS_RESPONSIBILITIES,
    team_control: EducationalScore::Medium,
    team_operational_load: EducationalScore::Low,
    portability: EducationalScore::Low,
};

const SAAS_PROFILE: ServiceModelProfile = ServiceModelProfile {
    model: ServiceModel::Saas,
    responsibilities: &SAAS_RESPONSIBILITIES,
    team_control: EducationalScore::Low,
    team_operational_load: EducationalScore::Low,
    portability: EducationalScore::Low,
};

const fn provider(layer: ResponsibilityLayer) -> ResponsibilityAssignment {
    ResponsibilityAssignment {
        layer,
        owner: ResponsibilityOwner::Provider,
    }
}

const fn team(layer: ResponsibilityLayer) -> ResponsibilityAssignment {
    ResponsibilityAssignment {
        layer,
        owner: ResponsibilityOwner::Team,
    }
}

const fn shared(layer: ResponsibilityLayer) -> ResponsibilityAssignment {
    ResponsibilityAssignment {
        layer,
        owner: ResponsibilityOwner::Shared,
    }
}
