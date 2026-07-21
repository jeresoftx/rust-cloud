use rust_cloud::finops::{
    BudgetControl, CostCategory, CostVisibility, ElasticityLimit, Environment, FinOpsCriticality,
    FinOpsProfile, FinOpsRequirements, OptimizationIntent, UsagePattern,
};

fn main() {
    let profile = FinOpsProfile::new(
        "academy-api",
        FinOpsRequirements {
            category: CostCategory::Compute,
            environment: Environment::Production,
            usage_pattern: UsagePattern::Steady,
            criticality: FinOpsCriticality::High,
            elasticity: ElasticityLimit::Bounded { max_units: 20 },
            visibility: CostVisibility::UnitEconomics,
            budget_control: BudgetControl::ForecastAndReview,
            optimization_intent: OptimizationIntent::ReduceWaste,
            owner: "equipo academy",
            purpose: "servir rutas de aprendizaje a estudiantes",
            unit_economics: "costo por estudiante activo",
        },
    )
    .unwrap();

    assert!(profile.evaluate().is_low_risk());
}
