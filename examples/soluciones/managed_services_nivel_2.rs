use rust_cloud::managed_services::{
    Criticality, ManagedCapability, ManagedServiceFinding, ManagedServiceProfile,
    ManagedServiceRequirements, RecoveryStrategy,
};

fn main() {
    let profile = ManagedServiceProfile::new(
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

    assert!(profile.evaluate().findings().contains(
        &ManagedServiceFinding::DurableStateWithoutRecovery("academy-search",)
    ));
}
