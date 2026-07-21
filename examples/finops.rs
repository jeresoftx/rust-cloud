use rust_cloud::finops::{
    BudgetControl, CostCategory, CostVisibility, ElasticityLimit, Environment, FinOpsCriticality,
    FinOpsFinding, FinOpsProfile, FinOpsRequirements, OptimizationIntent, UsagePattern,
};

fn main() {
    let academy_api = FinOpsProfile::new(
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

    let preview_workers = FinOpsProfile::new(
        "preview-workers",
        FinOpsRequirements {
            category: CostCategory::Invocations,
            environment: Environment::Development,
            usage_pattern: UsagePattern::Growing,
            criticality: FinOpsCriticality::Medium,
            elasticity: ElasticityLimit::Unbounded,
            visibility: CostVisibility::Aggregate,
            budget_control: BudgetControl::None,
            optimization_intent: OptimizationIntent::None,
            owner: "",
            purpose: "ejecutar previews automáticos",
            unit_economics: "",
        },
    )
    .unwrap();

    let legacy_worker = FinOpsProfile::new(
        "legacy-worker",
        FinOpsRequirements {
            category: CostCategory::Compute,
            environment: Environment::Production,
            usage_pattern: UsagePattern::Idle,
            criticality: FinOpsCriticality::Low,
            elasticity: ElasticityLimit::NotElastic,
            visibility: CostVisibility::Attributed,
            budget_control: BudgetControl::BudgetWithOwner,
            optimization_intent: OptimizationIntent::ReduceWaste,
            owner: "equipo academy",
            purpose: "mantener compatibilidad temporal con cargas antiguas",
            unit_economics: "costo por ejecución heredada",
        },
    )
    .unwrap();

    print_evaluation(&academy_api);
    let preview_evaluation = preview_workers.evaluate();
    assert!(
        preview_evaluation
            .findings()
            .contains(&FinOpsFinding::UnboundedElasticity("preview-workers"),)
    );
    print_evaluation(&preview_workers);

    let legacy_evaluation = legacy_worker.evaluate();
    assert!(
        legacy_evaluation
            .findings()
            .contains(&FinOpsFinding::IdleProductionResource("legacy-worker"),)
    );
    print_evaluation(&legacy_worker);
}

fn print_evaluation(profile: &FinOpsProfile) {
    let evaluation = profile.evaluate();

    if evaluation.is_low_risk() {
        println!("{}: costo atribuido y gobernable", profile.name());
    } else {
        println!(
            "{}: {} hallazgos educativos",
            profile.name(),
            evaluation.findings().len()
        );
    }
}
