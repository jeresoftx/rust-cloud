use rust_cloud::managed_services::{
    Criticality, ManagedCapability, ManagedServiceFinding, ManagedServiceProfile,
    ManagedServiceRequirements, RecoveryStrategy,
};

fn main() {
    let database = ManagedServiceProfile::new(
        "academy-db",
        ManagedServiceRequirements {
            capability: ManagedCapability::Database,
            durable_state: true,
            criticality: Criticality::High,
            recovery: RecoveryStrategy::TestedRestore,
            has_owner: true,
            purpose: "guardar progreso y evaluaciones del estudiante",
        },
    )
    .unwrap();

    assert!(database.evaluate().is_low_risk());
    println!("{}: servicio crítico con restore probado", database.name());

    let search = ManagedServiceProfile::new(
        "academy-search",
        ManagedServiceRequirements {
            capability: ManagedCapability::Search,
            durable_state: true,
            criticality: Criticality::Medium,
            recovery: RecoveryStrategy::None,
            has_owner: true,
            purpose: "buscar contenidos publicados del curso",
        },
    )
    .unwrap();

    let findings = search.evaluate().findings().to_vec();
    assert!(
        findings.contains(&ManagedServiceFinding::DurableStateWithoutRecovery(
            "academy-search",
        ))
    );
    println!("{}: {} hallazgos educativos", search.name(), findings.len());
}
