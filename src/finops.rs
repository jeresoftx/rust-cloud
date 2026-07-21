//! Costos y FinOps como señales explícitas de arquitectura.
//!
//! Este módulo no calcula precios reales ni reemplaza una factura de proveedor.
//! Hace visibles decisiones educativas: unidad de costo, ambiente, dueño,
//! propósito, visibilidad, límites y tradeoffs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostCategory {
    /// Capacidad de CPU y memoria.
    Compute,
    /// Datos retenidos en disco, objetos, backups o índices.
    Storage,
    /// Transferencia entre zonas, regiones o internet.
    Network,
    /// Logs, métricas, trazas y retención operativa.
    Observability,
    /// Ejecuciones por evento, función o workflow.
    Invocations,
    /// Servicio de plataforma administrado.
    ManagedService,
    /// Operación humana necesaria para sostener el sistema.
    HumanOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Producción visible para usuarios.
    Production,
    /// Ambiente estable previo a producción.
    Staging,
    /// Ambiente de desarrollo compartido o personal.
    Development,
    /// Ambiente temporal que debe tener ciclo de vida corto.
    Ephemeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsagePattern {
    /// Uso constante y predecible.
    Steady,
    /// Uso por picos o eventos.
    Bursty,
    /// Uso que crece con el producto.
    Growing,
    /// Recurso casi sin uso visible.
    Idle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinOpsCriticality {
    /// Costo menor o experimental.
    Low,
    /// Costo relevante para operación normal.
    Medium,
    /// Costo crítico para producto, margen o disponibilidad.
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElasticityLimit {
    /// No aplica o no escala automáticamente.
    NotElastic,
    /// Tiene límite explícito.
    Bounded { max_units: u32 },
    /// Puede crecer sin límite educativo visible.
    Unbounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostVisibility {
    /// Sin señal declarada.
    None,
    /// La señal existe solo como total agregado.
    Aggregate,
    /// El costo puede atribuirse por etiquetas, equipo o producto.
    Attributed,
    /// El costo se conecta con una unidad económica.
    UnitEconomics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetControl {
    /// Sin control de presupuesto.
    None,
    /// Alerta básica.
    AlertOnly,
    /// Presupuesto con dueño explícito.
    BudgetWithOwner,
    /// Presupuesto, forecast y revisión periódica.
    ForecastAndReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationIntent {
    /// Todavía no se declaró intención.
    None,
    /// Reducir consumo que no compra valor.
    ReduceWaste,
    /// Comprar mayor resiliencia o disponibilidad.
    IncreaseReliability,
    /// Comprar velocidad de entrega o aprendizaje.
    BuySpeed,
    /// Aceptar un riesgo explícito para bajar costo.
    AcceptRisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinOpsRequirements {
    /// Categoría educativa del costo.
    pub category: CostCategory,
    /// Ambiente donde aparece el gasto.
    pub environment: Environment,
    /// Patrón de uso observado.
    pub usage_pattern: UsagePattern,
    /// Criticidad económica u operativa.
    pub criticality: FinOpsCriticality,
    /// Límite de elasticidad.
    pub elasticity: ElasticityLimit,
    /// Visibilidad disponible.
    pub visibility: CostVisibility,
    /// Control de presupuesto.
    pub budget_control: BudgetControl,
    /// Intención del cambio económico.
    pub optimization_intent: OptimizationIntent,
    /// Dueño humano o equipo responsable.
    pub owner: &'static str,
    /// Propósito humano del gasto.
    pub purpose: &'static str,
    /// Unidad económica: estudiante, curso, evento, petición, documento, etc.
    pub unit_economics: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinOpsProfile {
    name: &'static str,
    requirements: FinOpsRequirements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinOpsDecisionError {
    /// Falta nombre.
    MissingName,
    /// Límite inválido.
    InvalidLimit(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinOpsFinding {
    /// El costo no tiene dueño.
    MissingOwner(&'static str),
    /// El costo no tiene propósito visible.
    MissingPurpose(&'static str),
    /// El costo no se conecta con unidad económica.
    MissingUnitEconomics(&'static str),
    /// Falta presupuesto o alerta.
    MissingBudgetControl(&'static str),
    /// Elasticidad sin límite explícito.
    UnboundedElasticity(&'static str),
    /// La visibilidad no permite atribuir costo.
    LowVisibility(&'static str),
    /// Recurso de producción casi sin uso.
    IdleProductionResource(&'static str),
    /// Observabilidad sin intención económica clara.
    ObservabilityWithoutIntent(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinOpsEvaluation {
    findings: Vec<FinOpsFinding>,
}

impl FinOpsProfile {
    /// Crea un perfil FinOps educativo.
    pub fn new(
        name: &'static str,
        requirements: FinOpsRequirements,
    ) -> Result<Self, FinOpsDecisionError> {
        if name.is_empty() {
            return Err(FinOpsDecisionError::MissingName);
        }

        if let ElasticityLimit::Bounded { max_units } = requirements.elasticity
            && max_units == 0
        {
            return Err(FinOpsDecisionError::InvalidLimit(
                "max_units debe ser mayor que cero",
            ));
        }

        Ok(Self { name, requirements })
    }

    /// Evalúa señales educativas de costo.
    pub fn evaluate(&self) -> FinOpsEvaluation {
        let mut findings = Vec::new();

        if self.requirements.owner.is_empty() {
            findings.push(FinOpsFinding::MissingOwner(self.name));
        }

        if self.requirements.purpose.is_empty() {
            findings.push(FinOpsFinding::MissingPurpose(self.name));
        }

        if self.requirements.unit_economics.is_empty() {
            findings.push(FinOpsFinding::MissingUnitEconomics(self.name));
        }

        if self.requirements.budget_control == BudgetControl::None {
            findings.push(FinOpsFinding::MissingBudgetControl(self.name));
        }

        if self.requirements.elasticity == ElasticityLimit::Unbounded {
            findings.push(FinOpsFinding::UnboundedElasticity(self.name));
        }

        if matches!(
            self.requirements.visibility,
            CostVisibility::None | CostVisibility::Aggregate
        ) {
            findings.push(FinOpsFinding::LowVisibility(self.name));
        }

        if self.requirements.environment == Environment::Production
            && self.requirements.usage_pattern == UsagePattern::Idle
        {
            findings.push(FinOpsFinding::IdleProductionResource(self.name));
        }

        if self.requirements.category == CostCategory::Observability
            && self.requirements.optimization_intent == OptimizationIntent::None
        {
            findings.push(FinOpsFinding::ObservabilityWithoutIntent(self.name));
        }

        FinOpsEvaluation { findings }
    }

    /// Nombre del perfil.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Requisitos declarados.
    pub const fn requirements(&self) -> FinOpsRequirements {
        self.requirements
    }
}

impl FinOpsEvaluation {
    /// Indica si no hay hallazgos educativos.
    pub fn is_low_risk(&self) -> bool {
        self.findings.is_empty()
    }

    /// Hallazgos detectados.
    pub fn findings(&self) -> &[FinOpsFinding] {
        &self.findings
    }
}
