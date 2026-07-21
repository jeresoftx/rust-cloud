//! Almacenamiento cloud como promesa explícita del dato.
//!
//! Este módulo no simula un sistema de almacenamiento. Hace visibles las
//! preguntas educativas: cómo se accede al dato, si es fuente de verdad, qué
//! durabilidad espera y qué costo aparece por retenerlo o recuperarlo.

/// Forma educativa de almacenamiento.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageMode {
    /// Datos direccionados por clave, como archivos, assets o respaldos.
    Object,
    /// Volumen persistente montado por una máquina o workload.
    Block,
    /// Sistema de archivos compartido entre varias cargas.
    File,
    /// Dato temporal que no es fuente de verdad.
    Ephemeral,
    /// Dato histórico con baja frecuencia de lectura.
    Archive,
}

/// Patrón principal de acceso al dato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessPattern {
    /// Acceso por clave u objeto completo.
    KeyAddressed,
    /// Acceso como volumen montado por una carga.
    MountedVolume,
    /// Acceso por jerarquía y rutas compartidas.
    SharedFilesystem,
    /// Acceso temporal local a la carga.
    TemporaryScratch,
    /// Acceso raro, con recuperación más lenta aceptable.
    ColdRetrieval,
}

/// Promesa educativa de consistencia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsistencyModel {
    /// Lecturas posteriores observan la última escritura aceptada.
    Strong,
    /// Las réplicas pueden converger después de un intervalo.
    Eventual,
    /// La consistencia depende de sesión, región o frontera documentada.
    Bounded,
}

/// Expectativa de durabilidad del dato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityExpectation {
    /// El dato puede desaparecer sin romper la fuente de verdad.
    Ephemeral,
    /// El dato debe sobrevivir a fallas comunes.
    Durable,
    /// El dato se replica para tolerar más fallas.
    Replicated,
    /// El dato se conserva para recuperación histórica.
    Archived,
}

/// Ciclo de vida esperado del dato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataLifecycle {
    /// Dato temporal o derivable.
    Temporary,
    /// Dato activo y consultado con frecuencia.
    Active,
    /// Dato retenido por negocio, auditoría o recuperación.
    Retained,
    /// Dato histórico con recuperación esporádica.
    Archived,
}

/// Frecuencia educativa de lectura.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessFrequency {
    /// Lecturas frecuentes o interactivas.
    Hot,
    /// Lecturas ocasionales.
    Warm,
    /// Lecturas raras.
    Cold,
    /// Lecturas excepcionales o de recuperación.
    Archive,
}

/// Intensidad educativa para comparar costos y operación.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageScore {
    /// Bajo para el equipo o para el costo representado.
    Low,
    /// Medio: existe tradeoff visible.
    Medium,
    /// Alto: el tradeoff domina la decisión.
    High,
}

/// Perfil pedagógico de una forma de almacenamiento.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageProfile {
    /// Forma descrita por el perfil.
    pub mode: StorageMode,
    /// Patrón principal de acceso.
    pub access_pattern: AccessPattern,
    /// Promesa de durabilidad.
    pub durability: DurabilityExpectation,
    /// Consistencia esperada por defecto en el modelo educativo.
    pub consistency: ConsistencyModel,
    /// Ciclo de vida típico.
    pub lifecycle: DataLifecycle,
    /// Costo relativo de recuperar o mover datos.
    pub retrieval_cost: StorageScore,
    /// Carga operativa que conserva el equipo.
    pub team_operational_load: StorageScore,
}

/// Requisitos mínimos para elegir una forma de almacenamiento.
///
/// # Examples
///
/// ```
/// use rust_cloud::storage::{
///     recommend_storage, AccessFrequency, DataRequirements, StorageMode,
/// };
///
/// let data = DataRequirements {
///     size_gib: Some(10),
///     source_of_truth: true,
///     accessed_by_key: true,
///     read_frequency: AccessFrequency::Warm,
///     ..DataRequirements::default()
/// };
///
/// assert_eq!(recommend_storage(data), Ok(StorageMode::Object));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataRequirements {
    /// Tamaño aproximado del dato en GiB.
    pub size_gib: Option<u32>,
    /// El dato es fuente de verdad.
    pub source_of_truth: bool,
    /// El dato es temporal o derivable.
    pub temporary_data: bool,
    /// El dato se accede por clave.
    pub accessed_by_key: bool,
    /// La carga necesita montar el dato como volumen.
    pub requires_mount: bool,
    /// Varias cargas necesitan ver rutas o archivos compartidos.
    pub shared_between_workloads: bool,
    /// Frecuencia esperada de lectura.
    pub read_frequency: AccessFrequency,
    /// Consistencia requerida por el consumidor.
    pub consistency: ConsistencyModel,
    /// Ciclo de vida esperado.
    pub lifecycle: DataLifecycle,
}

/// Error educativo cuando la decisión no declara supuestos suficientes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageDecisionError {
    /// El dato no declaró tamaño.
    MissingSize,
    /// El tamaño declarado no puede ser cero.
    InvalidSize(&'static str),
    /// El dato no declara si es fuente de verdad o temporal.
    MissingTruthDeclaration,
    /// No hay patrón mínimo de acceso.
    MissingAccessPattern,
    /// Los supuestos empujan decisiones incompatibles.
    ConflictingRequirements(&'static str),
}

impl Default for DataRequirements {
    fn default() -> Self {
        Self {
            size_gib: None,
            source_of_truth: false,
            temporary_data: false,
            accessed_by_key: false,
            requires_mount: false,
            shared_between_workloads: false,
            read_frequency: AccessFrequency::Hot,
            consistency: ConsistencyModel::Strong,
            lifecycle: DataLifecycle::Active,
        }
    }
}

impl StorageMode {
    /// Devuelve el perfil educativo de la forma de almacenamiento.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_cloud::storage::{AccessPattern, StorageMode};
    ///
    /// let profile = StorageMode::Object.profile();
    /// assert_eq!(profile.access_pattern, AccessPattern::KeyAddressed);
    /// ```
    pub fn profile(self) -> StorageProfile {
        match self {
            Self::Object => OBJECT_PROFILE,
            Self::Block => BLOCK_PROFILE,
            Self::File => FILE_PROFILE,
            Self::Ephemeral => EPHEMERAL_PROFILE,
            Self::Archive => ARCHIVE_PROFILE,
        }
    }

    /// Nivel relativo de recuperación lenta o histórica.
    pub const fn recovery_delay_level(self) -> u8 {
        match self {
            Self::Ephemeral => 0,
            Self::Block => 1,
            Self::File => 1,
            Self::Object => 2,
            Self::Archive => 3,
        }
    }
}

/// Devuelve las formas de almacenamiento del capítulo.
pub const fn storage_modes() -> &'static [StorageMode] {
    &STORAGE_MODES
}

/// Recomienda una forma de almacenamiento a partir de supuestos explícitos.
///
/// La función ordena tradeoffs; no reemplaza una evaluación real de proveedor.
pub fn recommend_storage(data: DataRequirements) -> Result<StorageMode, StorageDecisionError> {
    let size_gib = data.size_gib.ok_or(StorageDecisionError::MissingSize)?;
    if size_gib == 0 {
        return Err(StorageDecisionError::InvalidSize(
            "size_gib debe ser mayor que cero",
        ));
    }

    if !data.source_of_truth && !data.temporary_data {
        return Err(StorageDecisionError::MissingTruthDeclaration);
    }

    if data.source_of_truth && data.temporary_data {
        return Err(StorageDecisionError::ConflictingRequirements(
            "un dato no puede ser fuente de verdad y temporal al mismo tiempo",
        ));
    }

    if data.temporary_data {
        return Ok(StorageMode::Ephemeral);
    }

    if data.lifecycle == DataLifecycle::Archived || data.read_frequency == AccessFrequency::Archive
    {
        return Ok(StorageMode::Archive);
    }

    if data.shared_between_workloads && data.requires_mount {
        return Ok(StorageMode::File);
    }

    if data.shared_between_workloads && !data.requires_mount {
        return Err(StorageDecisionError::ConflictingRequirements(
            "compartir archivos entre cargas requiere declarar una frontera de acceso",
        ));
    }

    if data.requires_mount {
        return Ok(StorageMode::Block);
    }

    if data.accessed_by_key {
        return Ok(StorageMode::Object);
    }

    Err(StorageDecisionError::MissingAccessPattern)
}

const STORAGE_MODES: [StorageMode; 5] = [
    StorageMode::Object,
    StorageMode::Block,
    StorageMode::File,
    StorageMode::Ephemeral,
    StorageMode::Archive,
];

const OBJECT_PROFILE: StorageProfile = StorageProfile {
    mode: StorageMode::Object,
    access_pattern: AccessPattern::KeyAddressed,
    durability: DurabilityExpectation::Replicated,
    consistency: ConsistencyModel::Bounded,
    lifecycle: DataLifecycle::Active,
    retrieval_cost: StorageScore::Low,
    team_operational_load: StorageScore::Low,
};

const BLOCK_PROFILE: StorageProfile = StorageProfile {
    mode: StorageMode::Block,
    access_pattern: AccessPattern::MountedVolume,
    durability: DurabilityExpectation::Durable,
    consistency: ConsistencyModel::Strong,
    lifecycle: DataLifecycle::Active,
    retrieval_cost: StorageScore::Low,
    team_operational_load: StorageScore::Medium,
};

const FILE_PROFILE: StorageProfile = StorageProfile {
    mode: StorageMode::File,
    access_pattern: AccessPattern::SharedFilesystem,
    durability: DurabilityExpectation::Replicated,
    consistency: ConsistencyModel::Strong,
    lifecycle: DataLifecycle::Active,
    retrieval_cost: StorageScore::Medium,
    team_operational_load: StorageScore::Medium,
};

const EPHEMERAL_PROFILE: StorageProfile = StorageProfile {
    mode: StorageMode::Ephemeral,
    access_pattern: AccessPattern::TemporaryScratch,
    durability: DurabilityExpectation::Ephemeral,
    consistency: ConsistencyModel::Strong,
    lifecycle: DataLifecycle::Temporary,
    retrieval_cost: StorageScore::Low,
    team_operational_load: StorageScore::Low,
};

const ARCHIVE_PROFILE: StorageProfile = StorageProfile {
    mode: StorageMode::Archive,
    access_pattern: AccessPattern::ColdRetrieval,
    durability: DurabilityExpectation::Archived,
    consistency: ConsistencyModel::Eventual,
    lifecycle: DataLifecycle::Archived,
    retrieval_cost: StorageScore::High,
    team_operational_load: StorageScore::Low,
};
