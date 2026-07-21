use rust_cloud::managed_services::{
    Criticality, ManagedCapability, ManagedServiceFinding, ManagedServiceProfile,
    ManagedServiceRequirements, RecoveryStrategy,
};

fn main() {
    let queue = ManagedServiceProfile::new(
        "publish-events",
        ManagedServiceRequirements {
            capability: ManagedCapability::Queue,
            durable_state: false,
            criticality: Criticality::Medium,
            recovery: RecoveryStrategy::None,
            has_owner: true,
            purpose: "procesar eventos temporales de publicación",
        },
    )
    .unwrap();

    assert!(queue.evaluate().is_low_risk());

    let database = ManagedServiceProfile::new(
        "academy-db",
        ManagedServiceRequirements {
            capability: ManagedCapability::Database,
            durable_state: true,
            criticality: Criticality::High,
            recovery: RecoveryStrategy::BackupOnly,
            has_owner: true,
            purpose: "guardar progreso y evaluaciones del estudiante",
        },
    )
    .unwrap();

    assert!(
        database
            .evaluate()
            .findings()
            .contains(&ManagedServiceFinding::CriticalServiceWithoutTestedRecovery("academy-db"),)
    );
}
