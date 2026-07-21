use rust_cloud::finops::{
    BudgetControl, CostCategory, CostVisibility, ElasticityLimit, Environment, FinOpsCriticality,
    FinOpsFinding, FinOpsProfile, FinOpsRequirements, OptimizationIntent, UsagePattern,
};

fn main() {
    let observability = FinOpsProfile::new(
        "production-observability",
        FinOpsRequirements {
            category: CostCategory::Observability,
            environment: Environment::Production,
            usage_pattern: UsagePattern::Steady,
            criticality: FinOpsCriticality::High,
            elasticity: ElasticityLimit::Bounded { max_units: 12 },
            visibility: CostVisibility::Attributed,
            budget_control: BudgetControl::BudgetWithOwner,
            optimization_intent: OptimizationIntent::IncreaseReliability,
            owner: "equipo plataforma",
            purpose: "diagnosticar incidentes y proteger aprendizaje activo",
            unit_economics: "costo por curso observado",
        },
    )
    .unwrap();

    let legacy_index = FinOpsProfile::new(
        "legacy-search-index",
        FinOpsRequirements {
            category: CostCategory::ManagedService,
            environment: Environment::Production,
            usage_pattern: UsagePattern::Idle,
            criticality: FinOpsCriticality::Low,
            elasticity: ElasticityLimit::NotElastic,
            visibility: CostVisibility::Attributed,
            budget_control: BudgetControl::BudgetWithOwner,
            optimization_intent: OptimizationIntent::ReduceWaste,
            owner: "equipo academy",
            purpose: "mantener búsquedas heredadas",
            unit_economics: "costo por búsqueda activa",
        },
    )
    .unwrap();

    let preview_env = FinOpsProfile::new(
        "preview-environment",
        FinOpsRequirements {
            category: CostCategory::Compute,
            environment: Environment::Ephemeral,
            usage_pattern: UsagePattern::Bursty,
            criticality: FinOpsCriticality::Low,
            elasticity: ElasticityLimit::Bounded { max_units: 3 },
            visibility: CostVisibility::Attributed,
            budget_control: BudgetControl::AlertOnly,
            optimization_intent: OptimizationIntent::BuySpeed,
            owner: "equipo academy",
            purpose: "validar cambios antes de revisión humana",
            unit_economics: "costo por preview activo",
        },
    )
    .unwrap();

    assert!(observability.evaluate().is_low_risk());
    assert!(
        legacy_index
            .evaluate()
            .findings()
            .contains(&FinOpsFinding::IdleProductionResource(
                "legacy-search-index"
            ),)
    );
    assert!(preview_env.evaluate().is_low_risk());
}
