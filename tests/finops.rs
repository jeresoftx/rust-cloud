use rust_cloud::finops::{
    BudgetControl, CostCategory, CostVisibility, ElasticityLimit, Environment, FinOpsCriticality,
    FinOpsDecisionError, FinOpsFinding, FinOpsProfile, FinOpsRequirements, OptimizationIntent,
    UsagePattern,
};

#[test]
fn attributed_production_cost_with_budget_is_low_risk() {
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

    assert_eq!(profile.name(), "academy-api");
    assert!(profile.evaluate().is_low_risk());
}

#[test]
fn unbounded_cost_without_attribution_is_visible_risk() {
    let profile = FinOpsProfile::new(
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
    let evaluation = profile.evaluate();

    assert!(
        evaluation
            .findings()
            .contains(&FinOpsFinding::MissingOwner("preview-workers"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&FinOpsFinding::MissingUnitEconomics("preview-workers"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&FinOpsFinding::MissingBudgetControl("preview-workers"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&FinOpsFinding::UnboundedElasticity("preview-workers"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&FinOpsFinding::LowVisibility("preview-workers"))
    );
}

#[test]
fn idle_production_resource_is_visible_risk() {
    let profile = FinOpsProfile::new(
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

    assert!(
        profile
            .evaluate()
            .findings()
            .contains(&FinOpsFinding::IdleProductionResource(
                "legacy-search-index"
            ),)
    );
}

#[test]
fn profile_requires_name_and_positive_elasticity_limit() {
    let requirements = FinOpsRequirements {
        category: CostCategory::Storage,
        environment: Environment::Staging,
        usage_pattern: UsagePattern::Bursty,
        criticality: FinOpsCriticality::Medium,
        elasticity: ElasticityLimit::Bounded { max_units: 1 },
        visibility: CostVisibility::Attributed,
        budget_control: BudgetControl::AlertOnly,
        optimization_intent: OptimizationIntent::BuySpeed,
        owner: "equipo academy",
        purpose: "probar artefactos antes de producción",
        unit_economics: "costo por artefacto retenido",
    };

    assert_eq!(
        FinOpsProfile::new("", requirements),
        Err(FinOpsDecisionError::MissingName)
    );

    assert_eq!(
        FinOpsProfile::new(
            "staging-storage",
            FinOpsRequirements {
                elasticity: ElasticityLimit::Bounded { max_units: 0 },
                ..requirements
            },
        ),
        Err(FinOpsDecisionError::InvalidLimit(
            "max_units debe ser mayor que cero",
        ))
    );
}
