//! Cómputo cloud como capacidad de ejecución con límites explícitos.
//!
//! Este módulo no intenta simular un scheduler real. Su propósito es hacer
//! visibles las decisiones pequeñas: qué forma de cómputo ejecuta una carga,
//! qué recursos pide, cómo se aísla y cómo escala.

/// Forma educativa de ejecutar una carga en cloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeMode {
    /// Máquina virtual con control amplio sobre sistema operativo y runtime.
    VirtualMachine,
    /// Contenedor ejecutado por una plataforma que aún requiere operación.
    Container,
    /// Contenedor donde la plataforma administra despliegue y escalado base.
    ManagedContainer,
    /// Función o handler orientado a eventos.
    Function,
    /// Trabajo batch, programado o activado por cola.
    BatchJob,
}

/// Frontera principal de aislamiento de una carga.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationBoundary {
    /// La frontera principal es la máquina virtual.
    Machine,
    /// La frontera principal es un proceso administrado por el equipo.
    Process,
    /// La frontera principal es la imagen o runtime de contenedor.
    Container,
    /// El proveedor administra runtime, ciclo de vida y aislamiento detallado.
    ProviderManagedRuntime,
}

/// Ciclo de vida esperado para una carga.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePolicy {
    /// Servicio siempre encendido o de larga duración.
    LongRunning,
    /// Servicio que responde solicitudes mientras la plataforma lo mantiene.
    RequestResponse,
    /// Carga activada por eventos.
    EventDriven,
    /// Carga ejecutada por horario, lote o cola.
    ScheduledBatch,
}

/// Señal principal usada para escalar una carga.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingSignal {
    /// El equipo decide capacidad manualmente.
    Manual,
    /// CPU o memoria empujan el escalado.
    CpuOrMemory,
    /// La profundidad de una cola empuja concurrencia o réplicas.
    QueueDepth,
    /// La tasa de eventos empuja ejecuciones.
    EventRate,
    /// El horario o ventana de trabajo define la capacidad.
    Schedule,
}

/// Intensidad educativa para comparar modos de cómputo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComputeScore {
    /// Bajo para el equipo que consume la plataforma.
    Low,
    /// Medio: la plataforma ayuda, pero quedan decisiones relevantes.
    Medium,
    /// Alto: el equipo conserva mucha responsabilidad o control.
    High,
}

/// Recursos mínimos solicitados por una carga.
///
/// # Examples
///
/// ```
/// use rust_cloud::compute::ResourceRequest;
///
/// let request = ResourceRequest::new(2, 1024).expect("recursos válidos");
/// assert_eq!(request.vcpu, 2);
/// assert_eq!(request.memory_mib, 1024);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceRequest {
    /// vCPU solicitadas.
    pub vcpu: u16,
    /// Memoria solicitada en MiB.
    pub memory_mib: u32,
}

/// Límite educativo para validar si una carga cabe en una frontera.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimit {
    /// vCPU máximas permitidas.
    pub max_vcpu: u16,
    /// Memoria máxima permitida en MiB.
    pub max_memory_mib: u32,
}

/// Perfil pedagógico de un modo de cómputo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComputeProfile {
    /// Modo descrito por el perfil.
    pub mode: ComputeMode,
    /// Frontera principal de aislamiento.
    pub isolation: IsolationBoundary,
    /// Ciclo de vida esperado.
    pub lifecycle: LifecyclePolicy,
    /// Señal principal de escalado.
    pub scaling: ScalingSignal,
    /// Nivel de control que conserva el equipo.
    pub team_control: ComputeScore,
    /// Carga operativa que conserva el equipo.
    pub team_operational_load: ComputeScore,
    /// Elasticidad natural del modo, sin prometer costo menor.
    pub elasticity: ComputeScore,
}

/// Requisitos mínimos para seleccionar una forma de cómputo.
///
/// # Examples
///
/// ```
/// use rust_cloud::compute::{
///     recommend_compute, ComputeMode, ResourceRequest, WorkloadRequirements,
/// };
///
/// let workload = WorkloadRequirements {
///     resources: Some(ResourceRequest::new(1, 512).unwrap()),
///     event_driven: true,
///     wants_low_operational_load: true,
///     ..WorkloadRequirements::default()
/// };
///
/// assert_eq!(recommend_compute(workload), Ok(ComputeMode::Function));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WorkloadRequirements {
    /// Recursos mínimos que la carga necesita para ejecutarse.
    pub resources: Option<ResourceRequest>,
    /// La carga requiere administrar sistema operativo.
    pub requires_operating_system_control: bool,
    /// La carga ya se entrega como imagen de contenedor.
    pub packaged_as_container: bool,
    /// La carga debe vivir como servicio de larga duración.
    pub long_running_service: bool,
    /// La carga se activa principalmente por eventos.
    pub event_driven: bool,
    /// La carga corre por horario, lote o cola.
    pub scheduled_or_batch: bool,
    /// El equipo quiere reducir operación directa.
    pub wants_low_operational_load: bool,
}

/// Error educativo cuando falta contexto o los supuestos chocan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeDecisionError {
    /// La carga no declaró recursos mínimos.
    MissingResourceRequest,
    /// CPU o memoria fueron declaradas como cero.
    InvalidResourceRequest(&'static str),
    /// La carga no cabe dentro del límite elegido.
    ExceedsResourceLimit,
    /// Los supuestos empujan decisiones incompatibles.
    ConflictingRequirements(&'static str),
}

impl ComputeMode {
    /// Devuelve el perfil educativo del modo.
    pub fn profile(self) -> ComputeProfile {
        match self {
            Self::VirtualMachine => VIRTUAL_MACHINE_PROFILE,
            Self::Container => CONTAINER_PROFILE,
            Self::ManagedContainer => MANAGED_CONTAINER_PROFILE,
            Self::Function => FUNCTION_PROFILE,
            Self::BatchJob => BATCH_JOB_PROFILE,
        }
    }

    /// Nivel relativo de delegación de plataforma.
    pub const fn platform_delegation_level(self) -> u8 {
        match self {
            Self::VirtualMachine => 1,
            Self::Container => 2,
            Self::ManagedContainer => 3,
            Self::BatchJob => 4,
            Self::Function => 5,
        }
    }
}

impl ResourceRequest {
    /// Construye una solicitud de recursos con validación mínima.
    pub fn new(vcpu: u16, memory_mib: u32) -> Result<Self, ComputeDecisionError> {
        if vcpu == 0 {
            return Err(ComputeDecisionError::InvalidResourceRequest(
                "vcpu debe ser mayor que cero",
            ));
        }

        if memory_mib == 0 {
            return Err(ComputeDecisionError::InvalidResourceRequest(
                "memory_mib debe ser mayor que cero",
            ));
        }

        Ok(Self { vcpu, memory_mib })
    }

    /// Indica si la solicitud cabe dentro de un límite educativo.
    pub fn fits_within(self, limit: ResourceLimit) -> bool {
        self.vcpu <= limit.max_vcpu && self.memory_mib <= limit.max_memory_mib
    }
}

impl ResourceLimit {
    /// Construye un límite con validación mínima.
    pub fn new(max_vcpu: u16, max_memory_mib: u32) -> Result<Self, ComputeDecisionError> {
        if max_vcpu == 0 {
            return Err(ComputeDecisionError::InvalidResourceRequest(
                "max_vcpu debe ser mayor que cero",
            ));
        }

        if max_memory_mib == 0 {
            return Err(ComputeDecisionError::InvalidResourceRequest(
                "max_memory_mib debe ser mayor que cero",
            ));
        }

        Ok(Self {
            max_vcpu,
            max_memory_mib,
        })
    }
}

/// Devuelve los modos de cómputo en orden de mayor delegación de plataforma.
pub const fn compute_modes() -> &'static [ComputeMode] {
    &COMPUTE_MODES
}

/// Recomienda un modo de cómputo a partir de supuestos explícitos.
///
/// La función enseña criterio; no reemplaza una evaluación real de plataforma.
pub fn recommend_compute(
    workload: WorkloadRequirements,
) -> Result<ComputeMode, ComputeDecisionError> {
    let resources = workload
        .resources
        .ok_or(ComputeDecisionError::MissingResourceRequest)?;

    let default_limit = ResourceLimit::new(64, 262_144)?;
    if !resources.fits_within(default_limit) {
        return Err(ComputeDecisionError::ExceedsResourceLimit);
    }

    if workload.requires_operating_system_control && workload.wants_low_operational_load {
        return Err(ComputeDecisionError::ConflictingRequirements(
            "control de sistema operativo y baja carga operativa requieren decidir el tradeoff",
        ));
    }

    if workload.event_driven && workload.long_running_service {
        return Err(ComputeDecisionError::ConflictingRequirements(
            "una carga no puede ser principalmente evento y servicio siempre encendido a la vez",
        ));
    }

    if workload.requires_operating_system_control {
        return Ok(ComputeMode::VirtualMachine);
    }

    if workload.scheduled_or_batch {
        return Ok(ComputeMode::BatchJob);
    }

    if workload.event_driven {
        return Ok(ComputeMode::Function);
    }

    if workload.packaged_as_container && workload.wants_low_operational_load {
        return Ok(ComputeMode::ManagedContainer);
    }

    if workload.packaged_as_container {
        return Ok(ComputeMode::Container);
    }

    if workload.long_running_service && workload.wants_low_operational_load {
        return Ok(ComputeMode::ManagedContainer);
    }

    Ok(ComputeMode::VirtualMachine)
}

const COMPUTE_MODES: [ComputeMode; 5] = [
    ComputeMode::VirtualMachine,
    ComputeMode::Container,
    ComputeMode::ManagedContainer,
    ComputeMode::BatchJob,
    ComputeMode::Function,
];

const VIRTUAL_MACHINE_PROFILE: ComputeProfile = ComputeProfile {
    mode: ComputeMode::VirtualMachine,
    isolation: IsolationBoundary::Machine,
    lifecycle: LifecyclePolicy::LongRunning,
    scaling: ScalingSignal::Manual,
    team_control: ComputeScore::High,
    team_operational_load: ComputeScore::High,
    elasticity: ComputeScore::Low,
};

const CONTAINER_PROFILE: ComputeProfile = ComputeProfile {
    mode: ComputeMode::Container,
    isolation: IsolationBoundary::Container,
    lifecycle: LifecyclePolicy::LongRunning,
    scaling: ScalingSignal::CpuOrMemory,
    team_control: ComputeScore::High,
    team_operational_load: ComputeScore::Medium,
    elasticity: ComputeScore::Medium,
};

const MANAGED_CONTAINER_PROFILE: ComputeProfile = ComputeProfile {
    mode: ComputeMode::ManagedContainer,
    isolation: IsolationBoundary::Container,
    lifecycle: LifecyclePolicy::RequestResponse,
    scaling: ScalingSignal::CpuOrMemory,
    team_control: ComputeScore::Medium,
    team_operational_load: ComputeScore::Medium,
    elasticity: ComputeScore::High,
};

const FUNCTION_PROFILE: ComputeProfile = ComputeProfile {
    mode: ComputeMode::Function,
    isolation: IsolationBoundary::ProviderManagedRuntime,
    lifecycle: LifecyclePolicy::EventDriven,
    scaling: ScalingSignal::EventRate,
    team_control: ComputeScore::Low,
    team_operational_load: ComputeScore::Low,
    elasticity: ComputeScore::High,
};

const BATCH_JOB_PROFILE: ComputeProfile = ComputeProfile {
    mode: ComputeMode::BatchJob,
    isolation: IsolationBoundary::Process,
    lifecycle: LifecyclePolicy::ScheduledBatch,
    scaling: ScalingSignal::QueueDepth,
    team_control: ComputeScore::Medium,
    team_operational_load: ComputeScore::Medium,
    elasticity: ComputeScore::Medium,
};
