use rust_cloud::managed_services::{
    Criticality, ManagedCapability, ManagedServiceDecisionError, ManagedServiceFinding,
    ManagedServiceProfile, ManagedServiceRequirements, RecoveryStrategy, RetainedResponsibility,
};

#[test]
fn managed_database_with_tested_restore_is_low_risk() {
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

    assert_eq!(profile.capability(), ManagedCapability::Database);
    assert!(
        profile
            .retained()
            .contains(&RetainedResponsibility::DataModeling)
    );
    assert!(profile.evaluate().is_low_risk());
}

#[test]
fn critical_stateful_service_without_restore_is_visible_risk() {
    let profile = ManagedServiceProfile::new(
        "academy-db",
        ManagedServiceRequirements {
            capability: ManagedCapability::Database,
            durable_state: true,
            criticality: Criticality::High,
            recovery: RecoveryStrategy::BackupOnly,
            has_owner: false,
            purpose: "guardar progreso y evaluaciones del estudiante",
        },
    )
    .unwrap();
    let evaluation = profile.evaluate();

    assert!(
        evaluation
            .findings()
            .contains(&ManagedServiceFinding::MissingOwner("academy-db"))
    );
    assert!(
        evaluation
            .findings()
            .contains(&ManagedServiceFinding::CriticalServiceWithoutTestedRecovery("academy-db"),)
    );
}

#[test]
fn durable_state_without_recovery_is_visible_risk() {
    let profile = ManagedServiceProfile::new(
        "search-index",
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
        &ManagedServiceFinding::DurableStateWithoutRecovery("search-index"),
    ));
}

#[test]
fn profile_requires_name() {
    let requirements = ManagedServiceRequirements {
        capability: ManagedCapability::Queue,
        durable_state: false,
        criticality: Criticality::Medium,
        recovery: RecoveryStrategy::None,
        has_owner: true,
        purpose: "procesar eventos de publicación",
    };

    assert_eq!(
        ManagedServiceProfile::new("", requirements),
        Err(ManagedServiceDecisionError::MissingName)
    );
}
