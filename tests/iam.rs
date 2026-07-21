use rust_cloud::iam::{
    AccessAction, AccessDecisionError, AccessFinding, AccessGrant, AccessPlan, CredentialDuration,
    Principal, PrincipalKind, ResourceKind, ResourceScope, TrustBoundary,
};

#[test]
fn low_risk_workload_access_has_no_findings() {
    let workload = Principal::new(
        "academy-web",
        PrincipalKind::Workload,
        TrustBoundary::SameAccount,
        CredentialDuration::TemporarySession { hours: 1 },
    )
    .unwrap();
    let grant = AccessGrant::new(
        Some(workload),
        "assets-publicos",
        ResourceKind::ObjectStorage,
        Some(AccessAction::Write),
        Some(ResourceScope::SingleResource),
        "publicar assets generados por el pipeline",
    )
    .unwrap()
    .with_audit_event("object.write");
    let mut plan = AccessPlan::new("academy-prod").unwrap();

    plan.add_grant(grant);

    assert!(plan.evaluate().is_low_risk());
}

#[test]
fn human_admin_without_mfa_is_flagged() {
    let human = Principal::new(
        "joel",
        PrincipalKind::HumanUser,
        TrustBoundary::SameAccount,
        CredentialDuration::BreakGlass { hours: 2 },
    )
    .unwrap();
    let grant = AccessGrant::new(
        Some(human),
        "academy-prod",
        ResourceKind::PlatformAccount,
        Some(AccessAction::Administer),
        Some(ResourceScope::AccountWide),
        "recuperar producción durante incidente",
    )
    .unwrap();
    let mut plan = AccessPlan::new("incidente-prod").unwrap();

    plan.add_grant(grant);

    assert!(
        plan.evaluate()
            .findings()
            .contains(&AccessFinding::HumanAdminWithoutMfa("joel"))
    );
}

#[test]
fn external_permanent_access_without_condition_is_flagged() {
    let external = Principal::new(
        "proveedor-analitica",
        PrincipalKind::ExternalIdentity,
        TrustBoundary::ExternalProvider,
        CredentialDuration::Permanent,
    )
    .unwrap();
    let grant = AccessGrant::new(
        Some(external),
        "eventos-curso",
        ResourceKind::Database,
        Some(AccessAction::Read),
        Some(ResourceScope::ResourceGroup),
        "integración externa para analítica agregada",
    )
    .unwrap();
    let mut plan = AccessPlan::new("analytics-prod").unwrap();

    plan.add_grant(grant);
    let evaluation = plan.evaluate();

    assert!(
        evaluation
            .findings()
            .contains(&AccessFinding::LongLivedExternalAccess(
                "proveedor-analitica"
            ))
    );
    assert!(
        evaluation
            .findings()
            .contains(&AccessFinding::CrossBoundaryWithoutCondition(
                "proveedor-analitica"
            ))
    );
}

#[test]
fn grant_requires_visible_purpose() {
    let automation = Principal::new(
        "terraform",
        PrincipalKind::Automation,
        TrustBoundary::SameAccount,
        CredentialDuration::TemporarySession { hours: 1 },
    )
    .unwrap();

    assert_eq!(
        AccessGrant::new(
            Some(automation),
            "academy-prod",
            ResourceKind::PlatformAccount,
            Some(AccessAction::Administer),
            Some(ResourceScope::AccountWide),
            "",
        ),
        Err(AccessDecisionError::MissingPurpose)
    );
}

#[test]
fn zero_hour_temporary_credentials_are_rejected() {
    assert_eq!(
        CredentialDuration::temporary(0),
        Err(AccessDecisionError::InvalidDuration(
            "hours debe ser mayor que cero",
        ))
    );
}
