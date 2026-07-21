use rust_cloud::finops::{
    BudgetControl, CostCategory, CostVisibility, ElasticityLimit, Environment, FinOpsCriticality,
    FinOpsFinding, FinOpsProfile, FinOpsRequirements, OptimizationIntent, UsagePattern,
};

fn main() {
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
    let findings = profile.evaluate();

    assert!(
        findings
            .findings()
            .contains(&FinOpsFinding::MissingOwner("preview-workers"))
    );
    assert!(
        findings
            .findings()
            .contains(&FinOpsFinding::MissingUnitEconomics("preview-workers"))
    );
    assert!(
        findings
            .findings()
            .contains(&FinOpsFinding::MissingBudgetControl("preview-workers"))
    );
    assert!(
        findings
            .findings()
            .contains(&FinOpsFinding::UnboundedElasticity("preview-workers"))
    );
    assert!(
        findings
            .findings()
            .contains(&FinOpsFinding::LowVisibility("preview-workers"))
    );
}
