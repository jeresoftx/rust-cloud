use rust_cloud::managed_services::{
    Criticality, ManagedCapability, ManagedServiceProfile, ManagedServiceRequirements,
    RecoveryStrategy,
};

fn main() {
    let profile = ManagedServiceProfile::new(
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

    assert!(profile.evaluate().is_low_risk());
}
